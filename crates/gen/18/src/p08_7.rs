// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p08_7) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


// ===== Icon_Slot3_08c3da  ($08C3DA, 84 B, mission/cutscene_anim_08ba0c.s)
/// $08C3DA: move.w #$e000, $38(a6)
pub fn i_08c3da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C3E0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xE000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C3E0: bset.b #$6, $12(a6)
pub fn i_08c3e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C3E6;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08C3E6: move.w #$b0, d1
pub fn i_08c3e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C3EA;
    cpu.ir = 0x323C;
    let v1: u32 = 0xB0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08C3EA: jsr $236e.l
pub fn i_08c3ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C3F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C3F0);
    cpu.pc = 0x236E;
    16
}
/// $08C3F0: lea.l $2f2ed6.l, a0
pub fn i_08c3f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C3F6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F2ED6;
    20
}
/// $08C3F6: jsr $28cd4.l
pub fn i_08c3f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C3FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C3FC);
    cpu.pc = 0x28CD4;
    16
}
/// $08C3FC: move.w #$d0, $22(a6)
pub fn i_08c3fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C402;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xD0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C402: move.w #$12b, $24(a6)
pub fn i_08c402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C408;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x12B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C408: clr.w $26(a6)
pub fn i_08c408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C40C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x26);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08C40C: lea.l $8c552(pc), a1
pub fn i_08c40c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C410;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C552;
    16
}
/// $08C410: jsr $4ae.l
pub fn i_08c410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C416;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C416);
    cpu.pc = 0x4AE;
    16
}
/// $08C416: jsr $5dd02.l
pub fn i_08c416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C41C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C41C);
    cpu.pc = 0x5DD02;
    16
}
/// $08C41C: move.b #$40, $8a(a6)
pub fn i_08c41c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C422;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08C422: move.w #$0, $76(a6)
pub fn i_08c422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C428;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C428: lea.l $8c4a0(pc), a1
pub fn i_08c428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C42C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C4A0;
    16
}
/// $08C42C: move.l a1, (a6)
pub fn i_08c42c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C42E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrAbsThunk_08c42e  ($08C42E, 8 B, thunks/jsr_abs_thunks.c)
/// $08C42E: jsr $28d70.l
pub fn i_08c42e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C434;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C434);
    cpu.pc = 0x28D70;
    16
}
/// $08C434: rts
pub fn i_08c434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C436;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Icon_Slot4_08c436  ($08C436, 140 B, mission/cutscene_anim_08ba0c.s)
/// $08C436: movea.w #$73d7, a1
pub fn i_08c436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C43A;
    cpu.ir = 0x327C;
    let v1: u32 = 0x73D7;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $08C43A: move.w #$2e80, d0
pub fn i_08c43a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C43E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2E80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08C43E: move.w #$4, d1
pub fn i_08c43e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C442;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08C442: move.w #$4, d2
pub fn i_08c442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C446;
    cpu.ir = 0x343C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08C446: jsr $5da56.l
pub fn i_08c446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C44C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C44C);
    cpu.pc = 0x5DA56;
    16
}
/// $08C44C: move.w #$e000, $38(a6)
pub fn i_08c44c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C452;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xE000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C452: bset.b #$6, $12(a6)
pub fn i_08c452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C458;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08C458: move.w #$b2, d1
pub fn i_08c458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C45C;
    cpu.ir = 0x323C;
    let v1: u32 = 0xB2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08C45C: jsr $236e.l
pub fn i_08c45c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C462;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C462);
    cpu.pc = 0x236E;
    16
}
/// $08C462: lea.l $2f2ed6.l, a0
pub fn i_08c462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C468;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F2ED6;
    20
}
/// $08C468: jsr $28cd4.l
pub fn i_08c468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C46E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C46E);
    cpu.pc = 0x28CD4;
    16
}
/// $08C46E: move.w #$100, $22(a6)
pub fn i_08c46e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C474;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C474: move.w #$12b, $24(a6)
pub fn i_08c474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C47A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x12B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C47A: clr.w $26(a6)
pub fn i_08c47a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C47E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x26);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08C47E: lea.l $8c57e(pc), a1
pub fn i_08c47e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C482;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C57E;
    16
}
/// $08C482: jsr $4ae.l
pub fn i_08c482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C488;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C488);
    cpu.pc = 0x4AE;
    16
}
/// $08C488: jsr $5dd02.l
pub fn i_08c488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C48E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C48E);
    cpu.pc = 0x5DD02;
    16
}
/// $08C48E: move.b #$80, $8a(a6)
pub fn i_08c48e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C494;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08C494: move.w #$0, $76(a6)
pub fn i_08c494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C49A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C49A: lea.l $8c4a0(pc), a1
pub fn i_08c49a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C49E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C4A0;
    16
}
/// $08C49E: move.l a1, (a6)
pub fn i_08c49e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C4A0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08C4A0: movea.l $c(a6), a0
pub fn i_08c4a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C4A4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $08C4A4: move.b $93(a0), d0
pub fn i_08c4a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C4A8;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x93);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08C4A8: and.b $8a(a6), d0
pub fn i_08c4a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C4AC;
    cpu.ir = 0xC02E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read8(ad2) as u32;
    let r = (v3 & cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $08C4AC: beq.w $8c4ca
pub fn i_08c4ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C4B0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8C4CA; return 10; }
    8
}
/// $08C4B0: move.w #$1, $76(a6)
pub fn i_08c4b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C4B6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C4B6: lea.l $2f2ee2.l, a0
pub fn i_08c4b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C4BC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F2EE2;
    20
}
/// $08C4BC: jsr $28cd4.l
pub fn i_08c4bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C4C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C4C2);
    cpu.pc = 0x28CD4;
    16
}
// ===== JsrAbsThunk_08c4c2  ($08C4C2, 8 B, thunks/jsr_abs_thunks.c)
/// $08C4C2: jsr $28d70.l
pub fn i_08c4c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C4C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C4C8);
    cpu.pc = 0x28D70;
    16
}
/// $08C4C8: rts
pub fn i_08c4c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C4CA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Icon_Slot_Tail_08c4ca  ($08C4CA, 40 B, mission/cutscene_anim_08ba0c.s)
/// $08C4CA: move.b $92(a0), d0
pub fn i_08c4ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C4CE;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x92);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08C4CE: and.b $8a(a6), d0
pub fn i_08c4ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C4D2;
    cpu.ir = 0xC02E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read8(ad2) as u32;
    let r = (v3 & cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $08C4D2: bne.w $8c4f2
pub fn i_08c4d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C4D6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8C4F2; return 10; }
    8
}
/// $08C4D6: cmpi.w #$1, $76(a6)
pub fn i_08c4d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C4DC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $08C4DC: bne.w $8c4f2
pub fn i_08c4dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C4E0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8C4F2; return 10; }
    8
}
/// $08C4E0: move.w #$0, $76(a6)
pub fn i_08c4e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C4E6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C4E6: lea.l $2f2f02.l, a0
pub fn i_08c4e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C4EC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F2F02;
    20
}
/// $08C4EC: jsr $28cd4.l
pub fn i_08c4ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C4F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C4F2);
    cpu.pc = 0x28CD4;
    16
}
// ===== JsrAbsThunk_08c4f2  ($08C4F2, 8 B, thunks/jsr_abs_thunks.c)
/// $08C4F2: jsr $28d70.l
pub fn i_08c4f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C4F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C4F8);
    cpu.pc = 0x28D70;
    16
}
/// $08C4F8: rts
pub fn i_08c4f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C4FA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Icon_Slot1_Lit_08c4fa  ($08C4FA, 44 B, mission/cutscene_anim_08ba0c.s)
/// $08C4FA: move.w #$e000, $38(a6)
pub fn i_08c4fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C500;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xE000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C500: bset.b #$6, $12(a6)
pub fn i_08c500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C506;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08C506: move.w #$ae, d1
pub fn i_08c506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C50A;
    cpu.ir = 0x323C;
    let v1: u32 = 0xAE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08C50A: jsr $236e.l
pub fn i_08c50a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C510;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C510);
    cpu.pc = 0x236E;
    16
}
/// $08C510: lea.l $2f2f22.l, a0
pub fn i_08c510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C516;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F2F22;
    20
}
/// $08C516: jsr $28cd4.l
pub fn i_08c516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C51C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C51C);
    cpu.pc = 0x28CD4;
    16
}
/// $08C51C: lea.l $8c5aa(pc), a1
pub fn i_08c51c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C520;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C5AA;
    16
}
/// $08C520: move.l a1, (a6)
pub fn i_08c520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C522;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08C522: bra.w $8c5aa
pub fn i_08c522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C526;
    cpu.ir = 0x6000;
    cpu.pc = 0x8C5AA;
    10
}
// ===== Icon_Slot2_Lit_08c526  ($08C526, 44 B, mission/cutscene_anim_08ba0c.s)
/// $08C526: move.w #$e000, $38(a6)
pub fn i_08c526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C52C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xE000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C52C: bset.b #$6, $12(a6)
pub fn i_08c52c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C532;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08C532: move.w #$af, d1
pub fn i_08c532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C536;
    cpu.ir = 0x323C;
    let v1: u32 = 0xAF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08C536: jsr $236e.l
pub fn i_08c536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C53C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C53C);
    cpu.pc = 0x236E;
    16
}
/// $08C53C: lea.l $2f2f2e.l, a0
pub fn i_08c53c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C542;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F2F2E;
    20
}
/// $08C542: jsr $28cd4.l
pub fn i_08c542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C548;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C548);
    cpu.pc = 0x28CD4;
    16
}
/// $08C548: lea.l $8c5aa(pc), a1
pub fn i_08c548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C54C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C5AA;
    16
}
/// $08C54C: move.l a1, (a6)
pub fn i_08c54c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C54E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08C54E: bra.w $8c5aa
pub fn i_08c54e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C552;
    cpu.ir = 0x6000;
    cpu.pc = 0x8C5AA;
    10
}
// ===== Icon_Slot3_Lit_08c552  ($08C552, 44 B, mission/cutscene_anim_08ba0c.s)
/// $08C552: move.w #$e000, $38(a6)
pub fn i_08c552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C558;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xE000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C558: bset.b #$6, $12(a6)
pub fn i_08c558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C55E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08C55E: move.w #$b0, d1
pub fn i_08c55e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C562;
    cpu.ir = 0x323C;
    let v1: u32 = 0xB0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08C562: jsr $236e.l
pub fn i_08c562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C568;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C568);
    cpu.pc = 0x236E;
    16
}
/// $08C568: lea.l $2f2f3a.l, a0
pub fn i_08c568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C56E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F2F3A;
    20
}
/// $08C56E: jsr $28cd4.l
pub fn i_08c56e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C574;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C574);
    cpu.pc = 0x28CD4;
    16
}
/// $08C574: lea.l $8c5aa(pc), a1
pub fn i_08c574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C578;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C5AA;
    16
}
/// $08C578: move.l a1, (a6)
pub fn i_08c578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C57A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08C57A: bra.w $8c5aa
pub fn i_08c57a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C57E;
    cpu.ir = 0x6000;
    cpu.pc = 0x8C5AA;
    10
}
// ===== Icon_Slot4_Lit_08c57e  ($08C57E, 44 B, mission/cutscene_anim_08ba0c.s)
/// $08C57E: move.w #$e000, $38(a6)
pub fn i_08c57e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C584;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xE000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C584: bset.b #$6, $12(a6)
pub fn i_08c584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C58A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08C58A: move.w #$b2, d1
pub fn i_08c58a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C58E;
    cpu.ir = 0x323C;
    let v1: u32 = 0xB2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08C58E: jsr $236e.l
pub fn i_08c58e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C594;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C594);
    cpu.pc = 0x236E;
    16
}
/// $08C594: lea.l $2f2f46.l, a0
pub fn i_08c594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C59A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F2F46;
    20
}
/// $08C59A: jsr $28cd4.l
pub fn i_08c59a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C5A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C5A0);
    cpu.pc = 0x28CD4;
    16
}
/// $08C5A0: lea.l $8c5aa(pc), a1
pub fn i_08c5a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C5A4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C5AA;
    16
}
/// $08C5A4: move.l a1, (a6)
pub fn i_08c5a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C5A6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08C5A6: bra.w $8c5aa
pub fn i_08c5a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C5AA;
    cpu.ir = 0x6000;
    cpu.pc = 0x8C5AA;
    10
}
// ===== JsrAbsThunk_08c5aa  ($08C5AA, 8 B, thunks/jsr_abs_thunks.c)
/// $08C5AA: jsr $28d70.l
pub fn i_08c5aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C5B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C5B0);
    cpu.pc = 0x28D70;
    16
}
/// $08C5B0: rts
pub fn i_08c5b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C5B2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Icon_Anchor_Init_08c5b2  ($08C5B2, 190 B, mission/cutscene_anim_08ba0c.s)
/// $08C5B2: move.w #$8000, $38(a6)
pub fn i_08c5b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C5B8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C5B8: cmpi.b #$0, $10fd83.l
pub fn i_08c5b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C5C0;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10FD83) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    20
}
/// $08C5C0: bne.w $8c5ce
pub fn i_08c5c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C5C4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8C5CE; return 10; }
    8
}
/// $08C5C4: move.b #$0, $8a(a6)
pub fn i_08c5c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C5CA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08C5CA: bra.w $8c614
pub fn i_08c5ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C5CE;
    cpu.ir = 0x6000;
    cpu.pc = 0x8C614;
    10
}
/// $08C5CE: cmpi.b #$2, $10fd83.l
pub fn i_08c5ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C5D6;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10FD83) as u32;
    sub_with_flags(cpu, v2, 0x2, Size::Byte, false);
    20
}
/// $08C5D6: bne.w $8c60e
pub fn i_08c5d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C5DA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8C60E; return 10; }
    8
}
/// $08C5DA: move.b $10fd92.l, d0
pub fn i_08c5da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C5E0;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10FD92) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $08C5E0: cmpi.b #$2, d0
pub fn i_08c5e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C5E4;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $08C5E4: bne.w $8c5ee
pub fn i_08c5e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C5E8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8C5EE; return 10; }
    8
}
/// $08C5E8: move.b #$3, $8a(a6)
pub fn i_08c5e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C5EE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08C5EE: cmpi.b #$1, d0
pub fn i_08c5ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C5F2;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $08C5F2: bne.w $8c5fc
pub fn i_08c5f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C5F6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8C5FC; return 10; }
    8
}
/// $08C5F6: move.b #$2, $8a(a6)
pub fn i_08c5f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C5FC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08C5FC: cmpi.b #$0, d0
pub fn i_08c5fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C600;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $08C600: bne.w $8c60a
pub fn i_08c600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C604;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8C60A; return 10; }
    8
}
/// $08C604: move.b #$1, $8a(a6)
pub fn i_08c604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C60A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08C60A: bra.w $8c614
pub fn i_08c60a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C60E;
    cpu.ir = 0x6000;
    cpu.pc = 0x8C614;
    10
}
/// $08C60E: move.b #$1, $8a(a6)
pub fn i_08c60e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C614;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08C614: move.w #$97, d1
pub fn i_08c614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C618;
    cpu.ir = 0x323C;
    let v1: u32 = 0x97;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08C618: jsr $236e.l
pub fn i_08c618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C61E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C61E);
    cpu.pc = 0x236E;
    16
}
/// $08C61E: lea.l $2f2f52.l, a0
pub fn i_08c61e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C624;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F2F52;
    20
}
/// $08C624: jsr $28cd4.l
pub fn i_08c624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C62A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C62A);
    cpu.pc = 0x28CD4;
    16
}
/// $08C62A: move.w #$e0, $22(a6)
pub fn i_08c62a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C630;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C630: move.w #$160, $24(a6)
pub fn i_08c630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C636;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x160;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C636: clr.w $26(a6)
pub fn i_08c636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C63A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x26);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08C63A: jsr $267e2.l
pub fn i_08c63a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C640;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C640);
    cpu.pc = 0x267E2;
    16
}
/// $08C640: clr.l $2c(a6)
pub fn i_08c640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C644;
    cpu.ir = 0x42AE;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $08C644: move.l #$7084, $86(a6)
pub fn i_08c644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C64C;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x7084;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08C64C: move.w #$0, $70(a6)
pub fn i_08c64c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C652;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C652: move.w #$0, $80(a6)
pub fn i_08c652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C658;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C658: move.l #$0, $82(a6)
pub fn i_08c658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C660;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x0;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08C660: lea.l $2f0056.l, a1
pub fn i_08c660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C666;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2F0056;
    20
}
/// $08C666: move.l a1, $7c(a6)
pub fn i_08c666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C66A;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08C66A: move.w #$0, $76(a6)
pub fn i_08c66a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C670;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_08c670  ($08C670, 8 B, kernel/task_handlers.c)
/// $08C670: lea.l $8c678(pc), a1
pub fn i_08c670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C674;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C678;
    16
}
/// $08C674: move.l a1, (a6)
pub fn i_08c674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C676;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08C676: rts
pub fn i_08c676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C678;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Icon_Anchor_Run_08c678  ($08C678, 170 B, mission/cutscene_anim_08ba0c.s)
/// $08C678: movea.l $c(a6), a0
pub fn i_08c678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C67C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $08C67C: cmpi.w #$0, $76(a0)
pub fn i_08c67c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C682;
    cpu.ir = 0xC68;
    let ad2 = cpu.a[0].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08C682: beq.w $8c70c
pub fn i_08c682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C686;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8C70C; return 10; }
    8
}
/// $08C686: clr.l d0
pub fn i_08c686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C688;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $08C688: move.w $76(a0), d0
pub fn i_08c688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C68C;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08C68C: cmp.w $76(a6), d0
pub fn i_08c68c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C690;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $08C690: beq.w $8c6f6
pub fn i_08c690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C694;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8C6F6; return 10; }
    8
}
/// $08C694: move.w d0, $76(a6)
pub fn i_08c694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C698;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C698: cmpi.b #$0, $8a(a6)
pub fn i_08c698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C69E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08C69E: bne.w $8c6ac
pub fn i_08c69e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C6A2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8C6AC; return 10; }
    8
}
/// $08C6A2: lea.l $2efacc.l, a1
pub fn i_08c6a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C6A8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EFACC;
    20
}
/// $08C6A8: bra.w $8c6dc
pub fn i_08c6a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C6AC;
    cpu.ir = 0x6000;
    cpu.pc = 0x8C6DC;
    10
}
/// $08C6AC: cmpi.b #$3, $8a(a6)
pub fn i_08c6ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C6B2;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $08C6B2: bne.w $8c6bc
pub fn i_08c6b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C6B6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8C6BC; return 10; }
    8
}
/// $08C6B6: lea.l $2efeec.l, a1
pub fn i_08c6b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C6BC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EFEEC;
    20
}
/// $08C6BC: cmpi.b #$2, $8a(a6)
pub fn i_08c6bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C6C2;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $08C6C2: bne.w $8c6cc
pub fn i_08c6c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C6C6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8C6CC; return 10; }
    8
}
/// $08C6C6: lea.l $2efd8c.l, a1
pub fn i_08c6c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C6CC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EFD8C;
    20
}
/// $08C6CC: cmpi.b #$1, $8a(a6)
pub fn i_08c6cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C6D2;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $08C6D2: bne.w $8c6dc
pub fn i_08c6d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C6D6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8C6DC; return 10; }
    8
}
/// $08C6D6: lea.l $2efc2c.l, a1
pub fn i_08c6d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C6DC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EFC2C;
    20
}
/// $08C6DC: lsl.l #$3, d0
pub fn i_08c6dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C6DE;
    cpu.ir = 0xE788;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Long, 1, true);
    cpu.d[0] = r;
    12
}
/// $08C6DE: adda.l d0, a1
pub fn i_08c6de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C6E0;
    cpu.ir = 0xD3C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $08C6E0: move.l (a1)+, $7c(a6)
pub fn i_08c6e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C6E4;
    cpu.ir = 0x2D59;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $08C6E4: move.l (a1), $86(a6)
pub fn i_08c6e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C6E8;
    cpu.ir = 0x2D51;
    let v2 = bus.read32(cpu.a[1]);
    let ad3 = cpu.a[6].wrapping_add(0x86);
    bus.write32(ad3, v2);
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $08C6E8: move.l #$0, $82(a6)
pub fn i_08c6e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C6F0;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x0;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08C6F0: move.w #$0, $70(a6)
pub fn i_08c6f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C6F6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C6F6: cmpi.b #$0, $8a(a6)
pub fn i_08c6f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C6FC;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08C6FC: bne.w $8c708
pub fn i_08c6fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C700;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8C708; return 10; }
    8
}
/// $08C700: jsr $8bce6(pc)
pub fn i_08c700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C704;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8C704);
    cpu.pc = 0x8BCE6;
    16
}
/// $08C704: bra.w $8c70c
pub fn i_08c704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C708;
    cpu.ir = 0x6000;
    cpu.pc = 0x8C70C;
    10
}
/// $08C708: jsr $8be28(pc)
pub fn i_08c708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C70C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8C70C);
    cpu.pc = 0x8BE28;
    16
}
/// $08C70C: jsr $27cee.l
pub fn i_08c70c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C712;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C712);
    cpu.pc = 0x27CEE;
    16
}
/// $08C712: jsr $28d70.l
pub fn i_08c712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C718;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C718);
    cpu.pc = 0x28D70;
    16
}
/// $08C718: cmpi.w #$150, $22(a6)
pub fn i_08c718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C71E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x150, Size::Word, false);
    16
}
/// $08C71E: bls.w $8c72e
pub fn i_08c71e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C722;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x8C72E; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_08c722  ($08C722, 14 B, kernel/jsr5b6_jmp518.c)
/// $08C722: jsr $5b6.l
pub fn i_08c722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C728;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C728);
    cpu.pc = 0x5B6;
    16
}
/// $08C728: jmp $518.l
pub fn i_08c728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C72E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08C72E: rts
pub fn i_08c72e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C730;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Icon_Anchor_Drop_08c730  ($08C730, 150 B, mission/cutscene_anim_08ba0c.s)
/// $08C730: move.w #$140, $22(a6)
pub fn i_08c730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C736;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x140;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C736: move.w #$180, $24(a6)
pub fn i_08c736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C73C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C73C: move.w #$179, d1
pub fn i_08c73c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C740;
    cpu.ir = 0x323C;
    let v1: u32 = 0x179;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08C740: jsr $236e.l
pub fn i_08c740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C746;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C746);
    cpu.pc = 0x236E;
    16
}
/// $08C746: move.w #$8000, $38(a6)
pub fn i_08c746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C74C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C74C: jsr $267e2.l
pub fn i_08c74c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C752;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C752);
    cpu.pc = 0x267E2;
    16
}
/// $08C752: move.w #$f800, $28(a6)
pub fn i_08c752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C758;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF800;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C758: lea.l $2ef80e.l, a0
pub fn i_08c758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C75E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EF80E;
    20
}
/// $08C75E: jsr $28cd4.l
pub fn i_08c75e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C764;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C764);
    cpu.pc = 0x28CD4;
    16
}
/// $08C764: lea.l $2ef84c.l, a0
pub fn i_08c764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C76A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EF84C;
    20
}
/// $08C76A: move.l a0, $4c(a6)
pub fn i_08c76a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C76E;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08C76E: jsr $283ca.l
pub fn i_08c76e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C774;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C774);
    cpu.pc = 0x283CA;
    16
}
/// $08C774: lea.l $8c77a(pc), a1
pub fn i_08c774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C778;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C77A;
    16
}
/// $08C778: move.l a1, (a6)
pub fn i_08c778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C77A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08C77A: jsr $28d70.l
pub fn i_08c77a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C780;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C780);
    cpu.pc = 0x28D70;
    16
}
/// $08C780: jsr $27cee.l
pub fn i_08c780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C786;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C786);
    cpu.pc = 0x27CEE;
    16
}
/// $08C786: jsr $283d8.l
pub fn i_08c786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C78C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C78C);
    cpu.pc = 0x283D8;
    16
}
/// $08C78C: btst.b #$1, $13(a6)
pub fn i_08c78c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C792;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $08C792: bne.w $8c7ac
pub fn i_08c792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C796;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8C7AC; return 10; }
    8
}
/// $08C796: movea.l #$ffffffff, a0
pub fn i_08c796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C79C;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $08C79C: lea.l $2ef8a0.l, a0
pub fn i_08c79c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C7A2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EF8A0;
    20
}
/// $08C7A2: jsr $5dd56.l
pub fn i_08c7a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C7A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C7A8);
    cpu.pc = 0x5DD56;
    16
}
/// $08C7A8: bcc.w $8c7c4
pub fn i_08c7a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C7AC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8C7C4; return 10; }
    8
}
/// $08C7AC: lea.l $77f6a.l, a1
pub fn i_08c7ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C7B2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77F6A;
    20
}
/// $08C7B2: jsr $4ae.l
pub fn i_08c7b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C7B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C7B8);
    cpu.pc = 0x4AE;
    16
}
/// $08C7B8: jsr $5dd02.l
pub fn i_08c7b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C7BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C7BE);
    cpu.pc = 0x5DD02;
    16
}
/// $08C7BE: jmp $518.l
pub fn i_08c7be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C7C4;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08C7C4: rts
pub fn i_08c7c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C7C6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CmpPrioWithSibling_08c7c6  ($08C7C6, 16 B, mission/cutscene_anim_08ba0c.s)
/// $08C7C6: movea.l $8(a6), a1
pub fn i_08c7c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C7CA;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08C7CA: move.b $10(a6), d0
pub fn i_08c7ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C7CE;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08C7CE: cmp.b $10(a1), d0
pub fn i_08c7ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C7D2;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $08C7D2: bcs.w $8c7dc
pub fn i_08c7d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C7D6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8C7DC; return 10; }
    8
}
// ===== ClearXN_08c7d6  ($08C7D6, 6 B, thunks/ccr_helpers.c)
/// $08C7D6: andi.b #$ee, ccr
pub fn i_08c7d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C7DA;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $08C7DA: rts
pub fn i_08c7da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C7DC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_08c7dc  ($08C7DC, 6 B, thunks/ccr_helpers.c)
/// $08C7DC: ori.b #$11, ccr
pub fn i_08c7dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C7E0;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $08C7E0: rts
pub fn i_08c7e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C7E2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Cut_SetMode2_08c7e2  ($08C7E2, 14 B, mission/cutscene_anim_08ba0c.s)
/// $08C7E2: move.b #$2, $10e2ef.l
pub fn i_08c7e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C7EA;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2EF, v1 as u8);
    16
}
/// $08C7EA: jmp $518.l
pub fn i_08c7ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C7F0;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Rts_08c7f0  ($08C7F0, 2 B, mission/cutscene_anim_08ba0c.s)
/// $08C7F0: rts
pub fn i_08c7f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C7F2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearGlobalFlag_08c7f2  ($08C7F2, 14 B, kernel/scheduler_flag_setters.c)
/// $08C7F2: clr.b $10e2ee.l
pub fn i_08c7f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C7F8;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E2EE) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E2EE, r2 as u8);
    16
}
/// $08C7F8: jmp $518.l
pub fn i_08c7f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C7FE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08C7FE: rts
pub fn i_08c7fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C800;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Cut_SetVariant1_08c800  ($08C800, 14 B, mission/cutscene_anim_08ba0c.s)
/// $08C800: move.b #$1, $10e2ee.l
pub fn i_08c800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C808;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2EE, v1 as u8);
    16
}
/// $08C808: jmp $518.l
pub fn i_08c808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C80E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Rts_08c80e  ($08C80E, 2 B, mission/cutscene_anim_08ba0c.s)
/// $08C80E: rts
pub fn i_08c80e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C810;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Cut_SetVariant2_08c810  ($08C810, 14 B, mission/cutscene_anim_08ba0c.s)
/// $08C810: move.b #$2, $10e2ee.l
pub fn i_08c810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C818;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2EE, v1 as u8);
    16
}
/// $08C818: jmp $518.l
pub fn i_08c818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C81E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Rts_08c81e  ($08C81E, 2 B, mission/cutscene_anim_08ba0c.s)
/// $08C81E: rts
pub fn i_08c81e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C820;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Cut_InstallListByVariant_08c820  ($08C820, 60 B, mission/cutscene_anim_08ba0c.s)
/// $08C820: cmpi.b #$2, $10e2ee.l
pub fn i_08c820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C828;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10E2EE) as u32;
    sub_with_flags(cpu, v2, 0x2, Size::Byte, false);
    20
}
/// $08C828: bne.w $8c838
pub fn i_08c828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C82C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8C838; return 10; }
    8
}
/// $08C82C: lea.l $2f3688.l, a0
pub fn i_08c82c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C832;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3688;
    20
}
/// $08C832: jmp $5db58.l
pub fn i_08c832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C838;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x5DB58;
    8
}
/// $08C838: cmpi.b #$1, $10e2ee.l
pub fn i_08c838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C840;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10E2EE) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $08C840: bne.w $8c850
pub fn i_08c840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C844;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8C850; return 10; }
    8
}
/// $08C844: lea.l $2f35fe.l, a0
pub fn i_08c844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C84A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F35FE;
    20
}
/// $08C84A: jmp $5db58.l
pub fn i_08c84a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C850;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x5DB58;
    8
}
/// $08C850: lea.l $2f3574.l, a0
pub fn i_08c850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C856;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3574;
    20
}
/// $08C856: jmp $5db58.l
pub fn i_08c856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C85C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x5DB58;
    8
}
// ===== ClrRamWord_08c85c  ($08C85C, 8 B, thunks/clr_ram_word.c)
/// $08C85C: clr.b $10e2ee.l
pub fn i_08c85c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C862;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E2EE) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E2EE, r2 as u8);
    16
}
/// $08C862: rts
pub fn i_08c862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C864;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Cut_Watcher_Init_08c864  ($08C864, 28 B, mission/cutscene_anim_08ba0c.s)
/// $08C864: move.w #$7408, $22(a6)
pub fn i_08c864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C86A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7408;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C86A: clr.b $10e2ef.l
pub fn i_08c86a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C870;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E2EF) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E2EF, r2 as u8);
    16
}
/// $08C870: move.w #$ffff, $70(a6)
pub fn i_08c870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C876;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C876: clr.w $30(a6)
pub fn i_08c876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C87A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x30);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08C87A: lea.l $8c880(pc), a1
pub fn i_08c87a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C87E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C880;
    16
}
/// $08C87E: move.l a1, (a6)
pub fn i_08c87e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C880;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Cut_Watcher_Run_08c880  ($08C880, 122 B, mission/cutscene_anim_08ba0c.s)
/// $08C880: move.w $106f5c.l, d0
pub fn i_08c880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C886;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106F5C) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $08C886: cmp.w $70(a6), d0
pub fn i_08c886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C88A;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $08C88A: beq.w $8c89c
pub fn i_08c88a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C88E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8C89C; return 10; }
    8
}
/// $08C88E: move.w d0, $70(a6)
pub fn i_08c88e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C892;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C892: move.w #$78, $30(a6)
pub fn i_08c892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C898;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x78;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C898: bra.w $8c8e8
pub fn i_08c898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C89C;
    cpu.ir = 0x6000;
    cpu.pc = 0x8C8E8;
    10
}
/// $08C89C: tst.w $30(a6)
pub fn i_08c89c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C8A0;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $08C8A0: beq.w $8c8ac
pub fn i_08c8a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C8A4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8C8AC; return 10; }
    8
}
/// $08C8A4: subq.w #$1, $30(a6)
pub fn i_08c8a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C8A8;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x30);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08C8A8: bra.w $8c8e8
pub fn i_08c8a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C8AC;
    cpu.ir = 0x6000;
    cpu.pc = 0x8C8E8;
    10
}
/// $08C8AC: move.w $106e88.l, d0
pub fn i_08c8ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C8B2;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106E88) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $08C8B2: or.w $106e8a.l, d0
pub fn i_08c8b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C8B8;
    cpu.ir = 0x8079;
    let v2 = bus.read16(0x106E8A) as u32;
    let r = (v2 | cpu.d[0]) & 0xFFFF;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    16
}
/// $08C8B8: bne.w $8c8e8
pub fn i_08c8b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C8BC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8C8E8; return 10; }
    8
}
/// $08C8BC: jsr $5e1aa.l
pub fn i_08c8bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C8C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C8C2);
    cpu.pc = 0x5E1AA;
    16
}
/// $08C8C2: bcs.w $8c8e8
pub fn i_08c8c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C8C6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8C8E8; return 10; }
    8
}
/// $08C8C6: jsr $43ce2.l
pub fn i_08c8c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C8CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C8CC);
    cpu.pc = 0x43CE2;
    16
}
/// $08C8CC: bcs.w $8c8e8
pub fn i_08c8cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C8D0;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8C8E8; return 10; }
    8
}
/// $08C8D0: tst.b $106ed3.l
pub fn i_08c8d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C8D6;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x106ED3) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $08C8D6: beq.w $8c8e8
pub fn i_08c8d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C8DA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8C8E8; return 10; }
    8
}
/// $08C8DA: move.w #$78, $30(a6)
pub fn i_08c8da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C8E0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x78;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C8E0: move.b #$2, $10e2ef.l
pub fn i_08c8e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C8E8;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2EF, v1 as u8);
    16
}
/// $08C8E8: tst.b $10e2ef.l
pub fn i_08c8e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C8EE;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10E2EF) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $08C8EE: bne.w $8c8f4
pub fn i_08c8ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C8F2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8C8F4; return 10; }
    8
}
/// $08C8F2: rts
pub fn i_08c8f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C8F4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $08C8F4: lea.l $8c8fa(pc), a1
pub fn i_08c8f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C8F8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C8FA;
    16
}
/// $08C8F8: move.l a1, (a6)
pub fn i_08c8f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C8FA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Cut_Watcher_Play_08c8fa  ($08C8FA, 56 B, mission/cutscene_anim_08ba0c.s)
/// $08C8FA: bsr.w $8c820
pub fn i_08c8fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C8FE;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x8C8FE);
    cpu.pc = 0x8C820;
    18
}
/// $08C8FE: move.w #$10d5, d0
pub fn i_08c8fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C902;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10D5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08C902: jsr $2352.l
pub fn i_08c902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C908;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C908);
    cpu.pc = 0x2352;
    16
}
/// $08C908: lea.l $8c90e(pc), a1
pub fn i_08c908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C90C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C90E;
    16
}
/// $08C90C: move.l a1, (a6)
pub fn i_08c90c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C90E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08C90E: jsr $5db6a.l
pub fn i_08c90e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C914;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C914);
    cpu.pc = 0x5DB6A;
    16
}
/// $08C914: bcc.w $8c938
pub fn i_08c914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C918;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8C938; return 10; }
    8
}
/// $08C918: subq.b #$1, $10e2ef.l
pub fn i_08c918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C91E;
    cpu.ir = 0x5339;
    let v1 = bus.read8(0x10E2EF) as u32;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    bus.write8(0x10E2EF, r2 as u8);
    16
}
/// $08C91E: bne.w $8c932
pub fn i_08c91e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C922;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8C932; return 10; }
    8
}
/// $08C922: jsr $5dbdc.l
pub fn i_08c922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C928;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C928);
    cpu.pc = 0x5DBDC;
    16
}
/// $08C928: lea.l $8c880(pc), a1
pub fn i_08c928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C92C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C880;
    16
}
/// $08C92C: move.l a1, (a6)
pub fn i_08c92c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C92E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08C92E: bra.w $8c938
pub fn i_08c92e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C932;
    cpu.ir = 0x6000;
    cpu.pc = 0x8C938;
    10
}
// ===== SetTaskHandler_08c932  ($08C932, 8 B, kernel/task_handlers.c)
/// $08C932: lea.l $8c8fa(pc), a1
pub fn i_08c932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C936;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C8FA;
    16
}
/// $08C936: move.l a1, (a6)
pub fn i_08c936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C938;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08C938: rts
pub fn i_08c938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C93A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CmpPrioWithSibling_08c93a  ($08C93A, 16 B, mission/cutscene_anim_08ba0c.s)
/// $08C93A: movea.l $8(a6), a1
pub fn i_08c93a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C93E;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08C93E: move.b $10(a6), d0
pub fn i_08c93e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C942;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08C942: cmp.b $10(a1), d0
pub fn i_08c942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C946;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $08C946: bcs.w $8c950
pub fn i_08c946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C94A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8C950; return 10; }
    8
}
// ===== ClearXN_08c94a  ($08C94A, 6 B, thunks/ccr_helpers.c)
/// $08C94A: andi.b #$ee, ccr
pub fn i_08c94a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C94E;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $08C94E: rts
pub fn i_08c94e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C950;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_08c950  ($08C950, 6 B, thunks/ccr_helpers.c)
/// $08C950: ori.b #$11, ccr
pub fn i_08c950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C954;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $08C954: rts
pub fn i_08c954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C956;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Cut_Fade_08c956  ($08C956, 80 B, mission/cutscene_anim_08ba0c.s)
/// $08C956: move.w #$78, $70(a6)
pub fn i_08c956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C95C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x78;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C95C: movea.w #$70ae, a1
pub fn i_08c95c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C960;
    cpu.ir = 0x327C;
    let v1: u32 = 0x70AE;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $08C960: lea.l $2f4a12.l, a2
pub fn i_08c960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C966;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2F4A12;
    20
}
/// $08C966: move.w #$4, d1
pub fn i_08c966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C96A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08C96A: jsr $4784c.l
pub fn i_08c96a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C970;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C970);
    cpu.pc = 0x4784C;
    16
}
/// $08C970: lea.l $8c976(pc), a1
pub fn i_08c970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C974;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C976;
    16
}
/// $08C974: move.l a1, (a6)
pub fn i_08c974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C976;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08C976: subq.w #$1, $70(a6)
pub fn i_08c976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C97A;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08C97A: cmpi.w #$0, $70(a6)
pub fn i_08c97a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C980;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08C980: bge.w $8c9a4
pub fn i_08c980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C984;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x8C9A4; return 10; }
    8
}
/// $08C984: clr.b $106ed2.l
pub fn i_08c984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C98A;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106ED2) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106ED2, r2 as u8);
    16
}
/// $08C98A: movea.w #$70ae, a1
pub fn i_08c98a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C98E;
    cpu.ir = 0x327C;
    let v1: u32 = 0x70AE;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $08C98E: lea.l $2f4a22.l, a2
pub fn i_08c98e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C994;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2F4A22;
    20
}
/// $08C994: move.w #$4, d1
pub fn i_08c994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C998;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08C998: jsr $4784c.l
pub fn i_08c998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C99E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C99E);
    cpu.pc = 0x4784C;
    16
}
/// $08C99E: jmp $518.l
pub fn i_08c99e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C9A4;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08C9A4: rts
pub fn i_08c9a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C9A6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Fix_TextRow_Draw_08c9a6  ($08C9A6, 182 B, mission/cutscene_anim_08ba0c.s)
/// $08C9A6: move.w #$709a, d0
pub fn i_08c9a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C9AA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x709A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08C9AA: movea.l $80(a6), a1
pub fn i_08c9aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C9AE;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08C9AE: move.l a1, $84(a6)
pub fn i_08c9ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C9B2;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08C9B2: cmpa.l #$0, a1
pub fn i_08c9b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C9B8;
    cpu.ir = 0xB3FC;
    let v1: u32 = 0x0;
    let h1 = cpu.a[1];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $08C9B8: bne.w $8c9be
pub fn i_08c9b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C9BC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8C9BE; return 10; }
    8
}
/// $08C9BC: rts
pub fn i_08c9bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C9BE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $08C9BE: move.w (a1)+, d1
pub fn i_08c9be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C9C0;
    cpu.ir = 0x3219;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $08C9C0: cmpi.w #$fffe, d1
pub fn i_08c9c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C9C4;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFFE, Size::Word, false);
    8
}
/// $08C9C4: bne.w $8c9ce
pub fn i_08c9c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C9C8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8C9CE; return 10; }
    8
}
/// $08C9C8: move.w #$709c, d0
pub fn i_08c9c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C9CC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x709C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08C9CC: move.w (a1)+, d1
pub fn i_08c9cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C9CE;
    cpu.ir = 0x3219;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $08C9CE: cmpi.w #$0, d1
pub fn i_08c9ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C9D2;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $08C9D2: bne.w $8c9dc
pub fn i_08c9d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C9D6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8C9DC; return 10; }
    8
}
/// $08C9D6: move.l a1, $80(a6)
pub fn i_08c9d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C9DA;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08C9DA: rts
pub fn i_08c9da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C9DC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $08C9DC: cmpi.w #$ffff, d1
pub fn i_08c9dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C9E0;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFFF, Size::Word, false);
    8
}
/// $08C9E0: bne.w $8c9ee
pub fn i_08c9e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C9E4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8C9EE; return 10; }
    8
}
/// $08C9E4: clr.l $80(a6)
pub fn i_08c9e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C9E8;
    cpu.ir = 0x42AE;
    let ad1 = cpu.a[6].wrapping_add(0x80);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $08C9E8: clr.l $84(a6)
pub fn i_08c9e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C9EC;
    cpu.ir = 0x42AE;
    let ad1 = cpu.a[6].wrapping_add(0x84);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $08C9EC: rts
pub fn i_08c9ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C9EE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $08C9EE: cmpi.w #$fffd, d1
pub fn i_08c9ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C9F2;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFFD, Size::Word, false);
    8
}
/// $08C9F2: bne.w $8c9f8
pub fn i_08c9f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C9F6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8C9F8; return 10; }
    8
}
/// $08C9F6: bra.b $8c9be
pub fn i_08c9f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C9F8;
    cpu.ir = 0x60C6;
    cpu.pc = 0x8C9BE;
    10
}
/// $08C9F8: move.w d1, d2
pub fn i_08c9f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C9FA;
    cpu.ir = 0x3401;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08C9FA: andi.w #$f00, d2
pub fn i_08c9fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C9FE;
    cpu.ir = 0x242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = (v1 & 0xF00) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08C9FE: move.w #$9000, d3
pub fn i_08c9fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CA02;
    cpu.ir = 0x363C;
    let v1: u32 = 0x9000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08CA02: cmpi.w #$b00, d2
pub fn i_08ca02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CA06;
    cpu.ir = 0xC42;
    let v1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xB00, Size::Word, false);
    8
}
/// $08CA06: bne.w $8ca0e
pub fn i_08ca06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CA0A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8CA0E; return 10; }
    8
}
/// $08CA0A: move.w #$4000, d3
pub fn i_08ca0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CA0E;
    cpu.ir = 0x363C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08CA0E: cmpi.w #$c00, d2
pub fn i_08ca0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CA12;
    cpu.ir = 0xC42;
    let v1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xC00, Size::Word, false);
    8
}
/// $08CA12: bne.w $8ca1a
pub fn i_08ca12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CA16;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8CA1A; return 10; }
    8
}
/// $08CA16: move.w #$4000, d3
pub fn i_08ca16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CA1A;
    cpu.ir = 0x363C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08CA1A: or.w d3, d1
pub fn i_08ca1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CA1C;
    cpu.ir = 0x8243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r = (v1 | cpu.d[1]) & 0xFFFF;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $08CA1C: movem.w d0-d1, $3c0000.l
pub fn i_08ca1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CA24;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $08CA24: addi.w #$20, d0
pub fn i_08ca24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CA28;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08CA28: addq.w #$1, d1
pub fn i_08ca28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CA2A;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08CA2A: movem.w d0-d1, $3c0000.l
pub fn i_08ca2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CA32;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $08CA32: subi.w #$1f, d0
pub fn i_08ca32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CA36;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x1F, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08CA36: addi.w #$f, d1
pub fn i_08ca36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CA3A;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xF, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08CA3A: movem.w d0-d1, $3c0000.l
pub fn i_08ca3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CA42;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $08CA42: addi.w #$20, d0
pub fn i_08ca42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CA46;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08CA46: addq.w #$1, d1
pub fn i_08ca46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CA48;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08CA48: movem.w d0-d1, $3c0000.l
pub fn i_08ca48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CA50;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $08CA50: subi.w #$21, d0
pub fn i_08ca50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CA54;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x21, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08CA54: addi.w #$40, d0
pub fn i_08ca54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CA58;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x40, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08CA58: bra.w $8c9be
pub fn i_08ca58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CA5C;
    cpu.ir = 0x6000;
    cpu.pc = 0x8C9BE;
    10
}
// ===== Rts_08ca5c  ($08CA5C, 2 B, mission/cutscene_anim_08ba0c.s)
/// $08CA5C: rts
pub fn i_08ca5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CA5E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Fix_TextRow_Clear_08ca5e  ($08CA5E, 184 B, mission/cutscene_anim_08ba0c.s)
/// $08CA5E: movea.w #$701a, a1
pub fn i_08ca5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CA62;
    cpu.ir = 0x327C;
    let v1: u32 = 0x701A;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $08CA62: move.w #$20, d0
pub fn i_08ca62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CA66;
    cpu.ir = 0x303C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08CA66: move.w #$28, d1
pub fn i_08ca66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CA6A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08CA6A: move.w #$2, d2
pub fn i_08ca6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CA6E;
    cpu.ir = 0x343C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08CA6E: jsr $5da9c.l
pub fn i_08ca6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CA74;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8CA74);
    cpu.pc = 0x5DA9C;
    16
}
/// $08CA74: move.w #$709a, d0
pub fn i_08ca74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CA78;
    cpu.ir = 0x303C;
    let v1: u32 = 0x709A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08CA78: movea.l $80(a6), a1
pub fn i_08ca78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CA7C;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08CA7C: cmpa.l #$0, a1
pub fn i_08ca7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CA82;
    cpu.ir = 0xB3FC;
    let v1: u32 = 0x0;
    let h1 = cpu.a[1];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $08CA82: bne.w $8ca88
pub fn i_08ca82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CA86;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8CA88; return 10; }
    8
}
/// $08CA86: rts
pub fn i_08ca86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CA88;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $08CA88: move.w (a1)+, d1
pub fn i_08ca88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CA8A;
    cpu.ir = 0x3219;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $08CA8A: cmpi.w #$fffe, d1
pub fn i_08ca8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CA8E;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFFE, Size::Word, false);
    8
}
/// $08CA8E: bne.w $8ca94
pub fn i_08ca8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CA92;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8CA94; return 10; }
    8
}
/// $08CA92: rts
pub fn i_08ca92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CA94;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $08CA94: cmpi.w #$0, d1
pub fn i_08ca94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CA98;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $08CA98: bne.w $8ca9e
pub fn i_08ca98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CA9C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8CA9E; return 10; }
    8
}
/// $08CA9C: rts
pub fn i_08ca9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CA9E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $08CA9E: cmpi.w #$ffff, d1
pub fn i_08ca9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CAA2;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFFF, Size::Word, false);
    8
}
/// $08CAA2: bne.w $8caa8
pub fn i_08caa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CAA6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8CAA8; return 10; }
    8
}
/// $08CAA6: rts
pub fn i_08caa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CAA8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $08CAA8: cmpi.w #$fffd, d1
pub fn i_08caa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CAAC;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFFD, Size::Word, false);
    8
}
/// $08CAAC: bne.w $8cab2
pub fn i_08caac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CAB0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8CAB2; return 10; }
    8
}
/// $08CAB0: rts
pub fn i_08cab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CAB2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $08CAB2: move.w d1, d2
pub fn i_08cab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CAB4;
    cpu.ir = 0x3401;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08CAB4: andi.w #$f00, d2
pub fn i_08cab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CAB8;
    cpu.ir = 0x242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = (v1 & 0xF00) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08CAB8: move.w #$9000, d3
pub fn i_08cab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CABC;
    cpu.ir = 0x363C;
    let v1: u32 = 0x9000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08CABC: cmpi.w #$b00, d2
pub fn i_08cabc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CAC0;
    cpu.ir = 0xC42;
    let v1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xB00, Size::Word, false);
    8
}
/// $08CAC0: bne.w $8cac8
pub fn i_08cac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CAC4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8CAC8; return 10; }
    8
}
/// $08CAC4: move.w #$4000, d3
pub fn i_08cac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CAC8;
    cpu.ir = 0x363C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08CAC8: cmpi.w #$c00, d2
pub fn i_08cac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CACC;
    cpu.ir = 0xC42;
    let v1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xC00, Size::Word, false);
    8
}
/// $08CACC: bne.w $8cad4
pub fn i_08cacc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CAD0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8CAD4; return 10; }
    8
}
/// $08CAD0: move.w #$4000, d3
pub fn i_08cad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CAD4;
    cpu.ir = 0x363C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08CAD4: or.w d3, d1
pub fn i_08cad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CAD6;
    cpu.ir = 0x8243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r = (v1 | cpu.d[1]) & 0xFFFF;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $08CAD6: movem.w d0-d1, $3c0000.l
pub fn i_08cad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CADE;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $08CADE: addi.w #$20, d0
pub fn i_08cade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CAE2;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08CAE2: addq.w #$1, d1
pub fn i_08cae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CAE4;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08CAE4: movem.w d0-d1, $3c0000.l
pub fn i_08cae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CAEC;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $08CAEC: subi.w #$1f, d0
pub fn i_08caec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CAF0;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x1F, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08CAF0: addi.w #$f, d1
pub fn i_08caf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CAF4;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xF, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08CAF4: movem.w d0-d1, $3c0000.l
pub fn i_08caf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CAFC;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $08CAFC: addi.w #$20, d0
pub fn i_08cafc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CB00;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08CB00: addq.w #$1, d1
pub fn i_08cb00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CB02;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08CB02: movem.w d0-d1, $3c0000.l
pub fn i_08cb02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CB0A;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $08CB0A: subi.w #$21, d0
pub fn i_08cb0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CB0E;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x21, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08CB0E: addi.w #$40, d0
pub fn i_08cb0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CB12;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x40, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08CB12: bra.w $8ca88
pub fn i_08cb12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CB16;
    cpu.ir = 0x6000;
    cpu.pc = 0x8CA88;
    10
}
// ===== Rts_08cb16  ($08CB16, 2 B, mission/cutscene_anim_08ba0c.s)
/// $08CB16: rts
pub fn i_08cb16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CB18;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Fix_TextRow_DrawOne_08cb18  ($08CB18, 174 B, mission/cutscene_anim_08ba0c.s)
/// $08CB18: cmpi.l #$0, $84(a6)
pub fn i_08cb18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CB20;
    cpu.ir = 0xCAE;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read32(ad2);
    sub_with_flags(cpu, v3, 0x0, Size::Long, false);
    20
}
/// $08CB20: bne.w $8cb26
pub fn i_08cb20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CB24;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8CB26; return 10; }
    8
}
/// $08CB24: rts
pub fn i_08cb24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CB26;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $08CB26: movea.l $84(a6), a1
pub fn i_08cb26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CB2A;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08CB2A: move.w (a1), d1
pub fn i_08cb2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CB2C;
    cpu.ir = 0x3211;
    let v2 = bus.read16(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $08CB2C: cmpi.w #$0, d1
pub fn i_08cb2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CB30;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $08CB30: bne.w $8cb36
pub fn i_08cb30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CB34;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8CB36; return 10; }
    8
}
/// $08CB34: rts
pub fn i_08cb34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CB36;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $08CB36: cmpi.w #$ffff, d1
pub fn i_08cb36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CB3A;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFFF, Size::Word, false);
    8
}
/// $08CB3A: bne.w $8cb40
pub fn i_08cb3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CB3E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8CB40; return 10; }
    8
}
/// $08CB3E: rts
pub fn i_08cb3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CB40;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $08CB40: cmpi.w #$fffe, d1
pub fn i_08cb40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CB44;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFFE, Size::Word, false);
    8
}
/// $08CB44: bne.w $8cb5a
pub fn i_08cb44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CB48;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8CB5A; return 10; }
    8
}
/// $08CB48: move.w #$709c, d0
pub fn i_08cb48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CB4C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x709C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08CB4C: move.w d0, $90(a6)
pub fn i_08cb4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CB50;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08CB50: addi.l #$2, $84(a6)
pub fn i_08cb50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CB58;
    cpu.ir = 0x6AE;
    let ad1 = cpu.a[6].wrapping_add(0x84);
    let v2 = bus.read32(ad1);
    let r3 = add_with_flags(cpu, v2, 0x2, Size::Long, true);
    bus.write32(ad1, r3);
    20
}
/// $08CB58: bra.b $8cb18
pub fn i_08cb58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CB5A;
    cpu.ir = 0x60BE;
    cpu.pc = 0x8CB18;
    10
}
/// $08CB5A: cmpi.w #$fffd, d1
pub fn i_08cb5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CB5E;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFFD, Size::Word, false);
    8
}
/// $08CB5E: bne.w $8cb70
pub fn i_08cb5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CB62;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8CB70; return 10; }
    8
}
/// $08CB62: jsr $8ca5e(pc)
pub fn i_08cb62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CB66;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8CB66);
    cpu.pc = 0x8CA5E;
    16
}
/// $08CB66: addi.l #$2, $84(a6)
pub fn i_08cb66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CB6E;
    cpu.ir = 0x6AE;
    let ad1 = cpu.a[6].wrapping_add(0x84);
    let v2 = bus.read32(ad1);
    let r3 = add_with_flags(cpu, v2, 0x2, Size::Long, true);
    bus.write32(ad1, r3);
    20
}
/// $08CB6E: bra.b $8cb18
pub fn i_08cb6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CB70;
    cpu.ir = 0x60A8;
    cpu.pc = 0x8CB18;
    10
}
/// $08CB70: movea.l $84(a6), a1
pub fn i_08cb70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CB74;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08CB74: move.w (a1), d1
pub fn i_08cb74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CB76;
    cpu.ir = 0x3211;
    let v2 = bus.read16(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $08CB76: ori.w #$b000, d1
pub fn i_08cb76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CB7A;
    cpu.ir = 0x41;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 | 0xB000) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08CB7A: move.w $90(a6), d0
pub fn i_08cb7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CB7E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08CB7E: movem.w d0-d1, $3c0000.l
pub fn i_08cb7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CB86;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $08CB86: addi.w #$20, d0
pub fn i_08cb86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CB8A;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08CB8A: addq.w #$1, d1
pub fn i_08cb8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CB8C;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08CB8C: movem.w d0-d1, $3c0000.l
pub fn i_08cb8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CB94;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $08CB94: subi.w #$1f, d0
pub fn i_08cb94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CB98;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x1F, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08CB98: addi.w #$f, d1
pub fn i_08cb98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CB9C;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xF, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08CB9C: movem.w d0-d1, $3c0000.l
pub fn i_08cb9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CBA4;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $08CBA4: addi.w #$20, d0
pub fn i_08cba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CBA8;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08CBA8: addq.w #$1, d1
pub fn i_08cba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CBAA;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08CBAA: movem.w d0-d1, $3c0000.l
pub fn i_08cbaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CBB2;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $08CBB2: subi.w #$21, d0
pub fn i_08cbb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CBB6;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x21, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08CBB6: addi.w #$40, $90(a6)
pub fn i_08cbb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CBBC;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x90);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08CBBC: addi.l #$2, $84(a6)
pub fn i_08cbbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CBC4;
    cpu.ir = 0x6AE;
    let ad1 = cpu.a[6].wrapping_add(0x84);
    let v2 = bus.read32(ad1);
    let r3 = add_with_flags(cpu, v2, 0x2, Size::Long, true);
    bus.write32(ad1, r3);
    20
}
/// $08CBC4: rts
pub fn i_08cbc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CBC6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Cut_TextPanel_08cbc6  ($08CBC6, 120 B, mission/cutscene_anim_08ba0c.s)
/// $08CBC6: lea.l $2f4712.l, a1
pub fn i_08cbc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CBCC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2F4712;
    20
}
/// $08CBCC: move.l a1, $80(a6)
pub fn i_08cbcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CBD0;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08CBD0: move.w #$ffff, $38(a6)
pub fn i_08cbd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CBD6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08CBD6: bset.b #$6, $12(a6)
pub fn i_08cbd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CBDC;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08CBDC: move.w #$15e, d1
pub fn i_08cbdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CBE0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x15E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08CBE0: jsr $236e.l
pub fn i_08cbe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CBE6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8CBE6);
    cpu.pc = 0x236E;
    16
}
/// $08CBE6: lea.l $2f3732.l, a0
pub fn i_08cbe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CBEC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3732;
    20
}
/// $08CBEC: jsr $28cd4.l
pub fn i_08cbec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CBF2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8CBF2);
    cpu.pc = 0x28CD4;
    16
}
/// $08CBF2: move.w #$a0, $22(a6)
pub fn i_08cbf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CBF8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08CBF8: move.w #$121, $24(a6)
pub fn i_08cbf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CBFE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x121;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08CBFE: clr.w $26(a6)
pub fn i_08cbfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CC02;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x26);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08CC02: move.w #$709a, d0
pub fn i_08cc02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CC06;
    cpu.ir = 0x303C;
    let v1: u32 = 0x709A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08CC06: move.w d0, $90(a6)
pub fn i_08cc06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CC0A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08CC0A: lea.l $8cc10(pc), a1
pub fn i_08cc0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CC0E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8CC10;
    16
}
/// $08CC0E: move.l a1, (a6)
pub fn i_08cc0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CC10;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08CC10: jsr $28d70.l
pub fn i_08cc10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CC16;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8CC16);
    cpu.pc = 0x28D70;
    16
}
/// $08CC16: move.b $106f29.l, d0
pub fn i_08cc16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CC1C;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F29) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $08CC1C: bne.w $8cc44
pub fn i_08cc1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CC20;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8CC44; return 10; }
    8
}
/// $08CC20: movea.w #$701a, a1
pub fn i_08cc20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CC24;
    cpu.ir = 0x327C;
    let v1: u32 = 0x701A;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $08CC24: move.w #$20, d0
pub fn i_08cc24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CC28;
    cpu.ir = 0x303C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08CC28: move.w #$28, d1
pub fn i_08cc28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CC2C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08CC2C: move.w #$4, d2
pub fn i_08cc2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CC30;
    cpu.ir = 0x343C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08CC30: jsr $5da9c.l
pub fn i_08cc30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CC36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8CC36);
    cpu.pc = 0x5DA9C;
    16
}
/// $08CC36: jsr $8c9a6(pc)
pub fn i_08cc36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CC3A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8CC3A);
    cpu.pc = 0x8C9A6;
    16
}
/// $08CC3A: move.w #$709a, d0
pub fn i_08cc3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CC3E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x709A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== SetTaskW_08cc3e  ($08CC3E, 6 B, kernel/task_setters_w.c)
/// $08CC3E: move.w d0, $90(a6)
pub fn i_08cc3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CC42;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08CC42: rts
pub fn i_08cc42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CC44;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Cut_TextPanel_Type_08cc44  ($08CC44, 32 B, mission/cutscene_anim_08ba0c.s)
/// $08CC44: cmpi.b #$20, d0
pub fn i_08cc44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CC48;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x20, Size::Byte, false);
    8
}
/// $08CC48: bhi.w $8cc64
pub fn i_08cc48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CC4C;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x8CC64; return 10; }
    8
}
/// $08CC4C: move.w d0, d4
pub fn i_08cc4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CC4E;
    cpu.ir = 0x3800;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08CC4E: move.w #$1, d5
pub fn i_08cc4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CC52;
    cpu.ir = 0x3A3C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08CC52: bra.b $8cc56
pub fn i_08cc52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CC54;
    cpu.ir = 0x6002;
    cpu.pc = 0x8CC56;
    10
}
/// $08CC54: addq.w #$1, d5
pub fn i_08cc54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CC56;
    cpu.ir = 0x5245;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08CC56: cmp.w d4, d5
pub fn i_08cc56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CC58;
    cpu.ir = 0xBA44;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[5] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $08CC58: bgt.w $8cc62
pub fn i_08cc58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CC5C;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8CC62; return 10; }
    8
}
/// $08CC5C: jsr $8cb18(pc)
pub fn i_08cc5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CC60;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8CC60);
    cpu.pc = 0x8CB18;
    16
}
/// $08CC60: bra.b $8cc54
pub fn i_08cc60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CC62;
    cpu.ir = 0x60F2;
    cpu.pc = 0x8CC54;
    10
}
/// $08CC62: rts
pub fn i_08cc62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CC64;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Stub_0008CC64  ($08CC64, 2 B, thunks/trivial_rts.c)
/// $08CC64: rts
pub fn i_08cc64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CC66;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Cut_Banner_08cc66  ($08CC66, 88 B, mission/cutscene_anim_08ba0c.s)
/// $08CC66: move.w #$f0, $70(a6)
pub fn i_08cc66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CC6C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08CC6C: move.w #$ffff, $38(a6)
pub fn i_08cc6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CC72;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08CC72: bset.b #$6, $12(a6)
pub fn i_08cc72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CC78;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08CC78: move.w #$15e, d1
pub fn i_08cc78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CC7C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x15E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08CC7C: jsr $236e.l
pub fn i_08cc7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CC82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8CC82);
    cpu.pc = 0x236E;
    16
}
/// $08CC82: lea.l $2f373e.l, a0
pub fn i_08cc82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CC88;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F373E;
    20
}
/// $08CC88: jsr $28cd4.l
pub fn i_08cc88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CC8E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8CC8E);
    cpu.pc = 0x28CD4;
    16
}
/// $08CC8E: move.w #$a0, $22(a6)
pub fn i_08cc8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CC94;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08CC94: move.w #$f1, $24(a6)
pub fn i_08cc94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CC9A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08CC9A: clr.w $26(a6)
pub fn i_08cc9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CC9E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x26);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08CC9E: lea.l $8cca4(pc), a1
pub fn i_08cc9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CCA2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8CCA4;
    16
}
/// $08CCA2: move.l a1, (a6)
pub fn i_08cca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CCA4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08CCA4: subq.w #$1, $70(a6)
pub fn i_08cca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CCA8;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08CCA8: cmpi.w #$0, $70(a6)
pub fn i_08cca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CCAE;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08CCAE: bgt.w $8ccbe
pub fn i_08ccae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CCB2;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8CCBE; return 10; }
    8
}
/// $08CCB2: jsr $46a96.l
pub fn i_08ccb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CCB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8CCB8);
    cpu.pc = 0x46A96;
    16
}
/// $08CCB8: jmp $518.l
pub fn i_08ccb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CCBE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== JsrAbsThunk_08ccbe  ($08CCBE, 8 B, thunks/jsr_abs_thunks.c)
/// $08CCBE: jsr $28d70.l
pub fn i_08ccbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CCC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8CCC4);
    cpu.pc = 0x28D70;
    16
}
/// $08CCC4: rts
pub fn i_08ccc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CCC6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Scroll_StepVelX_08ccc6  ($08CCC6, 18 B, mission/cutscene_anim_08ba0c.s)
/// $08CCC6: move.l $80(a6), $106f60.l
pub fn i_08ccc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CCCE;
    cpu.ir = 0x23EE;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read32(ad2);
    bus.write32(0x106F60, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    32
}
/// $08CCCE: move.l $84(a6), d1
pub fn i_08ccce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CCD2;
    cpu.ir = 0x222E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08CCD2: add.l d1, $80(a6)
pub fn i_08ccd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CCD6;
    cpu.ir = 0xD3AE;
    let ad1 = cpu.a[6].wrapping_add(0x80);
    let v2 = bus.read32(ad1);
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Long, true);
    bus.write32(ad1, r3);
    16
}
/// $08CCD6: rts
pub fn i_08ccd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CCD8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Scroll_StepVelY_08ccd8  ($08CCD8, 18 B, mission/cutscene_anim_08ba0c.s)
/// $08CCD8: move.l $88(a6), $106f64.l
pub fn i_08ccd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CCE0;
    cpu.ir = 0x23EE;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read32(ad2);
    bus.write32(0x106F64, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    32
}
/// $08CCE0: move.l $8c(a6), d1
pub fn i_08cce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CCE4;
    cpu.ir = 0x222E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08CCE4: add.l d1, $88(a6)
pub fn i_08cce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CCE8;
    cpu.ir = 0xD3AE;
    let ad1 = cpu.a[6].wrapping_add(0x88);
    let v2 = bus.read32(ad1);
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Long, true);
    bus.write32(ad1, r3);
    16
}
/// $08CCE8: rts
pub fn i_08cce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CCEA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Cut_Dropper_08ccea  ($08CCEA, 174 B, mission/cutscene_anim_08ba0c.s)
/// $08CCEA: move.w #$fffe, $38(a6)
pub fn i_08ccea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CCF0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08CCF0: bset.b #$6, $12(a6)
pub fn i_08ccf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CCF6;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08CCF6: clr.w d0
pub fn i_08ccf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CCF8;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08CCF8: move.b $99(a6), d0
pub fn i_08ccf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CCFC;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08CCFC: lsl.w #$5, d0
pub fn i_08ccfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CCFE;
    cpu.ir = 0xEB48;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $08CCFE: move.w d0, $2a(a6)
pub fn i_08ccfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CD02;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08CD02: move.w #$136, d1
pub fn i_08cd02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CD06;
    cpu.ir = 0x323C;
    let v1: u32 = 0x136;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08CD06: jsr $236e.l
pub fn i_08cd06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CD0C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8CD0C);
    cpu.pc = 0x236E;
    16
}
/// $08CD0C: clr.l d0
pub fn i_08cd0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CD0E;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $08CD0E: move.b $98(a6), d0
pub fn i_08cd0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CD12;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08CD12: cmpi.l #$a, d0
pub fn i_08cd12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CD18;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0xA, Size::Long, false);
    8
}
/// $08CD18: bls.w $8cd1e
pub fn i_08cd18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CD1C;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x8CD1E; return 10; }
    8
}
/// $08CD1C: clr.l d0
pub fn i_08cd1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CD1E;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $08CD1E: movea.l #$2f455c, a0
pub fn i_08cd1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CD24;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2F455C;
    cpu.a[0] = v1;
    4
}
/// $08CD24: lsl.w #$2, d0
pub fn i_08cd24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CD26;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $08CD26: movea.l (a0, d0.w), a0
pub fn i_08cd26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CD2A;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $08CD2A: cmpa.l #$ffffffff, a0
pub fn i_08cd2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CD30;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $08CD30: beq.w $8cd3a
pub fn i_08cd30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CD34;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8CD3A; return 10; }
    8
}
/// $08CD34: jsr $28cd4.l
pub fn i_08cd34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CD3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8CD3A);
    cpu.pc = 0x28CD4;
    16
}
/// $08CD3A: lea.l $8cd40(pc), a1
pub fn i_08cd3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CD3E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8CD40;
    16
}
/// $08CD3E: move.l a1, (a6)
pub fn i_08cd3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CD40;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08CD40: jsr $8d2d4(pc)
pub fn i_08cd40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CD44;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8CD44);
    cpu.pc = 0x8D2D4;
    16
}
/// $08CD44: cmpi.b #$0, $98(a6)
pub fn i_08cd44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CD4A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08CD4A: bne.w $8cd6a
pub fn i_08cd4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CD4E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8CD6A; return 10; }
    8
}
/// $08CD4E: cmpi.b #$b, $106ece.l
pub fn i_08cd4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CD56;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0xB, Size::Byte, false);
    20
}
/// $08CD56: bne.w $8cd6a
pub fn i_08cd56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CD5A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8CD6A; return 10; }
    8
}
/// $08CD5A: cmpi.w #$170, $24(a6)
pub fn i_08cd5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CD60;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x170, Size::Word, false);
    16
}
/// $08CD60: blt.w $8cd6a
pub fn i_08cd60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CD64;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x8CD6A; return 10; }
    8
}
/// $08CD64: lea.l $8cda0(pc), a1
pub fn i_08cd64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CD68;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8CDA0;
    16
}
/// $08CD68: move.l a1, (a6)
pub fn i_08cd68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CD6A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08CD6A: cmpi.b #$a, $98(a6)
pub fn i_08cd6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CD70;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xA, Size::Byte, false);
    16
}
/// $08CD70: bne.w $8cd8a
pub fn i_08cd70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CD74;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8CD8A; return 10; }
    8
}
/// $08CD74: cmpi.w #$170, $24(a6)
pub fn i_08cd74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CD7A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x170, Size::Word, false);
    16
}
/// $08CD7A: blt.w $8cd8a
pub fn i_08cd7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CD7E;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x8CD8A; return 10; }
    8
}
/// $08CD7E: move.w #$78, $70(a6)
pub fn i_08cd7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CD84;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x78;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08CD84: lea.l $8cdba(pc), a1
pub fn i_08cd84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CD88;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8CDBA;
    16
}
/// $08CD88: move.l a1, (a6)
pub fn i_08cd88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CD8A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08CD8A: jsr $8d24c(pc)
pub fn i_08cd8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CD8E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8CD8E);
    cpu.pc = 0x8D24C;
    16
}
/// $08CD8E: bcc.w $8cd98
pub fn i_08cd8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CD92;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8CD98; return 10; }
    8
}
/// $08CD92: jmp $518.l
pub fn i_08cd92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CD98;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== JsrAbsThunk_08cd98  ($08CD98, 8 B, thunks/jsr_abs_thunks.c)
/// $08CD98: jsr $28d70.l
pub fn i_08cd98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CD9E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8CD9E);
    cpu.pc = 0x28D70;
    16
}
/// $08CD9E: rts
pub fn i_08cd9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CDA0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Cut_Dropper_WaitScroll_08cda0  ($08CDA0, 26 B, mission/cutscene_anim_08ba0c.s)
/// $08CDA0: jsr $28d70.l
pub fn i_08cda0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CDA6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8CDA6);
    cpu.pc = 0x28D70;
    16
}
/// $08CDA6: cmpi.w #$1000, $106f5c.l
pub fn i_08cda6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CDAE;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F5C) as u32;
    sub_with_flags(cpu, v2, 0x1000, Size::Word, false);
    20
}
/// $08CDAE: bcs.w $8cdb8
pub fn i_08cdae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CDB2;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8CDB8; return 10; }
    8
}
/// $08CDB2: jmp $518.l
pub fn i_08cdb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CDB8;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08CDB8: rts
pub fn i_08cdb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CDBA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Cut_Dropper_Finish_08cdba  ($08CDBA, 58 B, mission/cutscene_anim_08ba0c.s)
/// $08CDBA: subq.w #$1, $70(a6)
pub fn i_08cdba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CDBE;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08CDBE: cmpi.w #$0, $70(a6)
pub fn i_08cdbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CDC4;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08CDC4: bgt.w $8cdf4
pub fn i_08cdc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CDC8;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8CDF4; return 10; }
    8
}
/// $08CDC8: cmpi.b #$c, $106ece.l
pub fn i_08cdc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CDD0;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0xC, Size::Byte, false);
    20
}
/// $08CDD0: bne.w $8cdde
pub fn i_08cdd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CDD4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8CDDE; return 10; }
    8
}
/// $08CDD4: move.b #$60, d0
pub fn i_08cdd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CDD8;
    cpu.ir = 0x103C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $08CDD8: jsr $2308.l
pub fn i_08cdd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CDDE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8CDDE);
    cpu.pc = 0x2308;
    16
}
/// $08CDDE: move.w #$4, d0
pub fn i_08cdde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CDE2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08CDE2: jsr $5239e.l
pub fn i_08cde2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CDE8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8CDE8);
    cpu.pc = 0x5239E;
    16
}
/// $08CDE8: move.w #$a0, $70(a6)
pub fn i_08cde8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CDEE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08CDEE: lea.l $8cdfc(pc), a1
pub fn i_08cdee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CDF2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8CDFC;
    16
}
/// $08CDF2: move.l a1, (a6)
pub fn i_08cdf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CDF4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrAbsThunk_08cdf4  ($08CDF4, 8 B, thunks/jsr_abs_thunks.c)
/// $08CDF4: jsr $28d70.l
pub fn i_08cdf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CDFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8CDFA);
    cpu.pc = 0x28D70;
    16
}
/// $08CDFA: rts
pub fn i_08cdfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CDFC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Cut_Dropper_Exit_08cdfc  ($08CDFC, 26 B, mission/cutscene_anim_08ba0c.s)
/// $08CDFC: subq.w #$1, $70(a6)
pub fn i_08cdfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CE00;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08CE00: cmpi.w #$0, $70(a6)
pub fn i_08ce00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CE06;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08CE06: bgt.w $8ce16
pub fn i_08ce06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CE0A;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8CE16; return 10; }
    8
}
/// $08CE0A: clr.b $106ed2.l
pub fn i_08ce0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CE10;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106ED2) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106ED2, r2 as u8);
    16
}
/// $08CE10: jmp $518.l
pub fn i_08ce10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CE16;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== JsrAbsThunk_08ce16  ($08CE16, 8 B, thunks/jsr_abs_thunks.c)
/// $08CE16: jsr $28d70.l
pub fn i_08ce16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CE1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8CE1C);
    cpu.pc = 0x28D70;
    16
}
/// $08CE1C: rts
pub fn i_08ce1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CE1E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Cut_Item_08ce1e  ($08CE1E, 62 B, mission/cutscene_anim_08ba0c.s)
/// $08CE1E: move.w #$ffff, $38(a6)
pub fn i_08ce1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CE24;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08CE24: bset.b #$6, $12(a6)
pub fn i_08ce24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CE2A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08CE2A: move.w #$5f, d1
pub fn i_08ce2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CE2E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x5F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08CE2E: jsr $236e.l
pub fn i_08ce2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CE34;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8CE34);
    cpu.pc = 0x236E;
    16
}
/// $08CE34: lea.l $2f3b2c.l, a0
pub fn i_08ce34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CE3A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3B2C;
    20
}
/// $08CE3A: jsr $28cd4.l
pub fn i_08ce3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CE40;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8CE40);
    cpu.pc = 0x28CD4;
    16
}
/// $08CE40: lea.l $8ce46(pc), a1
pub fn i_08ce40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CE44;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8CE46;
    16
}
/// $08CE44: move.l a1, (a6)
pub fn i_08ce44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CE46;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08CE46: jsr $8d2d4(pc)
pub fn i_08ce46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CE4A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8CE4A);
    cpu.pc = 0x8D2D4;
    16
}
/// $08CE4A: subq.b #$1, $99(a6)
pub fn i_08ce4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CE4E;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x99);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $08CE4E: bne.w $8ce58
pub fn i_08ce4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CE52;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8CE58; return 10; }
    8
}
/// $08CE52: jmp $518.l
pub fn i_08ce52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CE58;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08CE58: jsr $8efb0(pc)
pub fn i_08ce58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CE5C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8CE5C);
    cpu.pc = 0x8EFB0;
    16
}
// ===== JsrAbsThunk_08ce5c  ($08CE5C, 8 B, thunks/jsr_abs_thunks.c)
/// $08CE5C: jsr $28d70.l
pub fn i_08ce5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CE62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8CE62);
    cpu.pc = 0x28D70;
    16
}
/// $08CE62: rts
pub fn i_08ce62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CE64;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SceneB_Init_08ce64  ($08CE64, 182 B, mission/cutscene_anim_08ba0c.s)
/// $08CE64: move.w #$29, d0
pub fn i_08ce64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CE68;
    cpu.ir = 0x303C;
    let v1: u32 = 0x29;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08CE68: jsr $2352.l
pub fn i_08ce68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CE6E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8CE6E);
    cpu.pc = 0x2352;
    16
}
/// $08CE6E: clr.w $70(a6)
pub fn i_08ce6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CE72;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08CE72: move.b #$6, $21(a6)
pub fn i_08ce72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CE78;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08CE78: move.b #$e, d0
pub fn i_08ce78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CE7C;
    cpu.ir = 0x103C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $08CE7C: jsr $43568.l
pub fn i_08ce7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CE82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8CE82);
    cpu.pc = 0x43568;
    16
}
/// $08CE82: move.w #$ffff, $106f5e.l
pub fn i_08ce82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CE8A;
    cpu.ir = 0x33FC;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x106F5E, v1 as u16);
    16
}
/// $08CE8A: cmpi.b #$1, $106eae.l
pub fn i_08ce8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CE92;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106EAE) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $08CE92: bne.w $8cea4
pub fn i_08ce92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CE96;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8CEA4; return 10; }
    8
}
/// $08CE96: move.l #$1c000, $106f60.l
pub fn i_08ce96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CEA0;
    cpu.ir = 0x23FC;
    let v1: u32 = 0x1C000;
    bus.write32(0x106F60, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $08CEA0: bra.w $8ceae
pub fn i_08cea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CEA4;
    cpu.ir = 0x6000;
    cpu.pc = 0x8CEAE;
    10
}
/// $08CEA4: move.l #$18000, $106f60.l
pub fn i_08cea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CEAE;
    cpu.ir = 0x23FC;
    let v1: u32 = 0x18000;
    bus.write32(0x106F60, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $08CEAE: move.l #$0, $106f64.l
pub fn i_08ceae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CEB8;
    cpu.ir = 0x23FC;
    let v1: u32 = 0x0;
    bus.write32(0x106F64, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $08CEB8: jsr $46a96.l
pub fn i_08ceb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CEBE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8CEBE);
    cpu.pc = 0x46A96;
    16
}
/// $08CEBE: jsr $2230.l
pub fn i_08cebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CEC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8CEC4);
    cpu.pc = 0x2230;
    16
}
/// $08CEC4: lea.l $8d3b4(pc), a1
pub fn i_08cec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CEC8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8D3B4;
    16
}
/// $08CEC8: jsr $4ae.l
pub fn i_08cec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CECE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8CECE);
    cpu.pc = 0x4AE;
    16
}
/// $08CECE: clr.b $20(a6)
pub fn i_08cece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CED2;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $08CED2: lea.l $8ced8(pc), a1
pub fn i_08ced2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CED6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8CED8;
    16
}
/// $08CED6: move.l a1, (a6)
pub fn i_08ced6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CED8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08CED8: jsr $437da.l
pub fn i_08ced8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CEDE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8CEDE);
    cpu.pc = 0x437DA;
    16
}
/// $08CEDE: jsr $96a0e.l
pub fn i_08cede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CEE4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8CEE4);
    cpu.pc = 0x96A0E;
    16
}
/// $08CEE4: jsr $96a80.l
pub fn i_08cee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CEEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8CEEA);
    cpu.pc = 0x96A80;
    16
}
/// $08CEEA: cmpi.w #$100, $106f5c.l
pub fn i_08ceea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CEF2;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F5C) as u32;
    sub_with_flags(cpu, v2, 0x100, Size::Word, false);
    20
}
/// $08CEF2: bcs.w $8cf20
pub fn i_08cef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CEF6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8CF20; return 10; }
    8
}
/// $08CEF6: cmpi.b #$1, $106eae.l
pub fn i_08cef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CEFE;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106EAE) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $08CEFE: bne.w $8cf10
pub fn i_08cefe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CF02;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8CF10; return 10; }
    8
}
/// $08CF02: move.l #$1a000, $106f60.l
pub fn i_08cf02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CF0C;
    cpu.ir = 0x23FC;
    let v1: u32 = 0x1A000;
    bus.write32(0x106F60, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $08CF0C: bra.w $8cf1a
pub fn i_08cf0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CF10;
    cpu.ir = 0x6000;
    cpu.pc = 0x8CF1A;
    10
}
/// $08CF10: move.l #$16000, $106f60.l
pub fn i_08cf10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CF1A;
    cpu.ir = 0x23FC;
    let v1: u32 = 0x16000;
    bus.write32(0x106F60, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
// ===== SetTaskHandler_08cf1a  ($08CF1A, 8 B, kernel/task_handlers.c)
/// $08CF1A: lea.l $8cf22(pc), a1
pub fn i_08cf1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CF1E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8CF22;
    16
}
/// $08CF1E: move.l a1, (a6)
pub fn i_08cf1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CF20;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08CF20: rts
pub fn i_08cf20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CF22;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SceneB_Stage2_08cf22  ($08CF22, 66 B, mission/cutscene_anim_08ba0c.s)
/// $08CF22: jsr $437da.l
pub fn i_08cf22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CF28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8CF28);
    cpu.pc = 0x437DA;
    16
}
/// $08CF28: jsr $96a0e.l
pub fn i_08cf28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CF2E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8CF2E);
    cpu.pc = 0x96A0E;
    16
}
/// $08CF2E: jsr $96a80.l
pub fn i_08cf2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CF34;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8CF34);
    cpu.pc = 0x96A80;
    16
}
/// $08CF34: cmpi.w #$1000, $106f5c.l
pub fn i_08cf34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CF3C;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F5C) as u32;
    sub_with_flags(cpu, v2, 0x1000, Size::Word, false);
    20
}
/// $08CF3C: bcs.w $8cf6a
pub fn i_08cf3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CF40;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8CF6A; return 10; }
    8
}
/// $08CF40: cmpi.b #$1, $106eae.l
pub fn i_08cf40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CF48;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106EAE) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $08CF48: bne.w $8cf5a
pub fn i_08cf48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CF4C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8CF5A; return 10; }
    8
}
/// $08CF4C: move.l #$16000, $106f60.l
pub fn i_08cf4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CF56;
    cpu.ir = 0x23FC;
    let v1: u32 = 0x16000;
    bus.write32(0x106F60, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $08CF56: bra.w $8cf64
pub fn i_08cf56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CF5A;
    cpu.ir = 0x6000;
    cpu.pc = 0x8CF64;
    10
}
/// $08CF5A: move.l #$12000, $106f60.l
pub fn i_08cf5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CF64;
    cpu.ir = 0x23FC;
    let v1: u32 = 0x12000;
    bus.write32(0x106F60, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
// ===== SetTaskHandler_08cf64  ($08CF64, 8 B, kernel/task_handlers.c)
/// $08CF64: lea.l $8cf6c(pc), a1
pub fn i_08cf64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CF68;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8CF6C;
    16
}
/// $08CF68: move.l a1, (a6)
pub fn i_08cf68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CF6A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08CF6A: rts
pub fn i_08cf6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CF6C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SceneB_Stage3_08cf6c  ($08CF6C, 66 B, mission/cutscene_anim_08ba0c.s)
/// $08CF6C: jsr $437da.l
pub fn i_08cf6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CF72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8CF72);
    cpu.pc = 0x437DA;
    16
}
/// $08CF72: jsr $96a0e.l
pub fn i_08cf72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CF78;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8CF78);
    cpu.pc = 0x96A0E;
    16
}
/// $08CF78: jsr $96a80.l
pub fn i_08cf78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CF7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8CF7E);
    cpu.pc = 0x96A80;
    16
}
/// $08CF7E: cmpi.w #$2b94, $106f5c.l
pub fn i_08cf7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CF86;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F5C) as u32;
    sub_with_flags(cpu, v2, 0x2B94, Size::Word, false);
    20
}
/// $08CF86: bcs.w $8cfb4
pub fn i_08cf86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CF8A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8CFB4; return 10; }
    8
}
/// $08CF8A: cmpi.b #$1, $106eae.l
pub fn i_08cf8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CF92;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106EAE) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $08CF92: bne.w $8cfa4
pub fn i_08cf92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CF96;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8CFA4; return 10; }
    8
}
/// $08CF96: move.l #$16000, $106f60.l
pub fn i_08cf96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CFA0;
    cpu.ir = 0x23FC;
    let v1: u32 = 0x16000;
    bus.write32(0x106F60, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $08CFA0: bra.w $8cfae
pub fn i_08cfa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CFA4;
    cpu.ir = 0x6000;
    cpu.pc = 0x8CFAE;
    10
}
/// $08CFA4: move.l #$12000, $106f60.l
pub fn i_08cfa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CFAE;
    cpu.ir = 0x23FC;
    let v1: u32 = 0x12000;
    bus.write32(0x106F60, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
// ===== SetTaskHandler_08cfae  ($08CFAE, 8 B, kernel/task_handlers.c)
/// $08CFAE: lea.l $8cfb6(pc), a1
pub fn i_08cfae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CFB2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8CFB6;
    16
}
/// $08CFB2: move.l a1, (a6)
pub fn i_08cfb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CFB4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08CFB4: rts
pub fn i_08cfb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CFB6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SceneB_Stage4_08cfb6  ($08CFB6, 66 B, mission/cutscene_anim_08ba0c.s)
/// $08CFB6: jsr $437da.l
pub fn i_08cfb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CFBC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8CFBC);
    cpu.pc = 0x437DA;
    16
}
/// $08CFBC: jsr $96a0e.l
pub fn i_08cfbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CFC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8CFC2);
    cpu.pc = 0x96A0E;
    16
}
/// $08CFC2: jsr $96a80.l
pub fn i_08cfc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CFC8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8CFC8);
    cpu.pc = 0x96A80;
    16
}
/// $08CFC8: cmpi.w #$2d00, $106f5c.l
pub fn i_08cfc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CFD0;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F5C) as u32;
    sub_with_flags(cpu, v2, 0x2D00, Size::Word, false);
    20
}
/// $08CFD0: bcs.w $8cffe
pub fn i_08cfd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CFD4;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8CFFE; return 10; }
    8
}
/// $08CFD4: cmpi.b #$1, $106eae.l
pub fn i_08cfd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CFDC;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106EAE) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $08CFDC: bne.w $8cfee
pub fn i_08cfdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CFE0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8CFEE; return 10; }
    8
}
/// $08CFE0: move.l #$14000, $106f60.l
pub fn i_08cfe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CFEA;
    cpu.ir = 0x23FC;
    let v1: u32 = 0x14000;
    bus.write32(0x106F60, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $08CFEA: bra.w $8cff8
pub fn i_08cfea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CFEE;
    cpu.ir = 0x6000;
    cpu.pc = 0x8CFF8;
    10
}
/// $08CFEE: move.l #$10000, $106f60.l
pub fn i_08cfee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CFF8;
    cpu.ir = 0x23FC;
    let v1: u32 = 0x10000;
    bus.write32(0x106F60, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
// ===== SetTaskHandler_08cff8  ($08CFF8, 8 B, kernel/task_handlers.c)
/// $08CFF8: lea.l $8d000(pc), a1
pub fn i_08cff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CFFC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8D000;
    16
}
/// $08CFFC: move.l a1, (a6)
pub fn i_08cffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8CFFE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08CFFE: rts
pub fn i_08cffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D000;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SceneB_Stage5_08d000  ($08D000, 66 B, mission/cutscene_anim_08ba0c.s)
/// $08D000: jsr $437da.l
pub fn i_08d000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D006;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D006);
    cpu.pc = 0x437DA;
    16
}
/// $08D006: jsr $96a0e.l
pub fn i_08d006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D00C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D00C);
    cpu.pc = 0x96A0E;
    16
}
/// $08D00C: jsr $96a80.l
pub fn i_08d00c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D012;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D012);
    cpu.pc = 0x96A80;
    16
}
/// $08D012: cmpi.w #$3000, $106f5c.l
pub fn i_08d012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D01A;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F5C) as u32;
    sub_with_flags(cpu, v2, 0x3000, Size::Word, false);
    20
}
/// $08D01A: bcs.w $8d048
pub fn i_08d01a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D01E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8D048; return 10; }
    8
}
/// $08D01E: cmpi.b #$1, $106eae.l
pub fn i_08d01e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D026;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106EAE) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $08D026: bne.w $8d038
pub fn i_08d026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D02A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8D038; return 10; }
    8
}
/// $08D02A: move.l #$26000, $106f60.l
pub fn i_08d02a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D034;
    cpu.ir = 0x23FC;
    let v1: u32 = 0x26000;
    bus.write32(0x106F60, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $08D034: bra.w $8d042
pub fn i_08d034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D038;
    cpu.ir = 0x6000;
    cpu.pc = 0x8D042;
    10
}
/// $08D038: move.l #$20000, $106f60.l
pub fn i_08d038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D042;
    cpu.ir = 0x23FC;
    let v1: u32 = 0x20000;
    bus.write32(0x106F60, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
// ===== SetTaskHandler_08d042  ($08D042, 8 B, kernel/task_handlers.c)
/// $08D042: lea.l $8d04a(pc), a1
pub fn i_08d042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D046;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8D04A;
    16
}
/// $08D046: move.l a1, (a6)
pub fn i_08d046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D048;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08D048: rts
pub fn i_08d048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D04A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SceneB_Stage6_08d04a  ($08D04A, 66 B, mission/cutscene_anim_08ba0c.s)
/// $08D04A: jsr $437da.l
pub fn i_08d04a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D050;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D050);
    cpu.pc = 0x437DA;
    16
}
/// $08D050: jsr $96a0e.l
pub fn i_08d050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D056;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D056);
    cpu.pc = 0x96A0E;
    16
}
/// $08D056: jsr $96a80.l
pub fn i_08d056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D05C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D05C);
    cpu.pc = 0x96A80;
    16
}
/// $08D05C: cmpi.w #$4000, $106f5c.l
pub fn i_08d05c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D064;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F5C) as u32;
    sub_with_flags(cpu, v2, 0x4000, Size::Word, false);
    20
}
/// $08D064: bcs.w $8d092
pub fn i_08d064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D068;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8D092; return 10; }
    8
}
/// $08D068: cmpi.b #$1, $106eae.l
pub fn i_08d068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D070;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106EAE) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $08D070: bne.w $8d082
pub fn i_08d070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D074;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8D082; return 10; }
    8
}
/// $08D074: move.l #$26000, $106f60.l
pub fn i_08d074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D07E;
    cpu.ir = 0x23FC;
    let v1: u32 = 0x26000;
    bus.write32(0x106F60, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $08D07E: bra.w $8d08c
pub fn i_08d07e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D082;
    cpu.ir = 0x6000;
    cpu.pc = 0x8D08C;
    10
}
/// $08D082: move.l #$20000, $106f60.l
pub fn i_08d082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D08C;
    cpu.ir = 0x23FC;
    let v1: u32 = 0x20000;
    bus.write32(0x106F60, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
// ===== SetTaskHandler_08d08c  ($08D08C, 8 B, kernel/task_handlers.c)
/// $08D08C: lea.l $8d094(pc), a1
pub fn i_08d08c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D090;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8D094;
    16
}
/// $08D090: move.l a1, (a6)
pub fn i_08d090(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D092;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08D092: rts
pub fn i_08d092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D094;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SceneB_Tail_08d094  ($08D094, 12 B, mission/cutscene_anim_08ba0c.s)
/// $08D094: jsr $437da.l
pub fn i_08d094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D09A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D09A);
    cpu.pc = 0x437DA;
    16
}
/// $08D09A: jsr $96a0e.l
pub fn i_08d09a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D0A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D0A0);
    cpu.pc = 0x96A0E;
    16
}
// ===== JsrAbsThunk_08d0a0  ($08D0A0, 8 B, thunks/jsr_abs_thunks.c)
/// $08D0A0: jsr $96a80.l
pub fn i_08d0a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D0A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D0A6);
    cpu.pc = 0x96A80;
    16
}
/// $08D0A6: rts
pub fn i_08d0a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D0A8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SceneC_Init_08d0a8  ($08D0A8, 210 B, mission/cutscene_anim_08ba0c.s)
/// $08D0A8: lea.l $8cc66(pc), a1
pub fn i_08d0a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D0AC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8CC66;
    16
}
/// $08D0AC: jsr $4ae.l
pub fn i_08d0ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D0B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D0B2);
    cpu.pc = 0x4AE;
    16
}
/// $08D0B2: lea.l $8cbc6(pc), a1
pub fn i_08d0b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D0B6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8CBC6;
    16
}
/// $08D0B6: jsr $4ae.l
pub fn i_08d0b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D0BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D0BC);
    cpu.pc = 0x4AE;
    16
}
/// $08D0BC: move.w #$2a, d0
pub fn i_08d0bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D0C0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08D0C0: jsr $2352.l
pub fn i_08d0c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D0C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D0C6);
    cpu.pc = 0x2352;
    16
}
/// $08D0C6: clr.w $70(a6)
pub fn i_08d0c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D0CA;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08D0CA: move.b #$7, $21(a6)
pub fn i_08d0ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D0D0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08D0D0: move.b #$f, d0
pub fn i_08d0d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D0D4;
    cpu.ir = 0x103C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $08D0D4: jsr $43568.l
pub fn i_08d0d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D0DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D0DA);
    cpu.pc = 0x43568;
    16
}
/// $08D0DA: move.w #$ffff, $106f5e.l
pub fn i_08d0da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D0E2;
    cpu.ir = 0x33FC;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x106F5E, v1 as u16);
    16
}
/// $08D0E2: move.l #$18000, $106f60.l
pub fn i_08d0e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D0EC;
    cpu.ir = 0x23FC;
    let v1: u32 = 0x18000;
    bus.write32(0x106F60, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $08D0EC: move.l #$0, $106f64.l
pub fn i_08d0ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D0F6;
    cpu.ir = 0x23FC;
    let v1: u32 = 0x0;
    bus.write32(0x106F64, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $08D0F6: bra.w $8d154
pub fn i_08d0f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D0FA;
    cpu.ir = 0x6000;
    cpu.pc = 0x8D154;
    10
}
/// $08D0FA: jsr $46a96.l
pub fn i_08d0fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D100;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D100);
    cpu.pc = 0x46A96;
    16
}
/// $08D100: move.w #$2a, d0
pub fn i_08d100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D104;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08D104: jsr $2352.l
pub fn i_08d104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D10A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D10A);
    cpu.pc = 0x2352;
    16
}
/// $08D10A: clr.w $70(a6)
pub fn i_08d10a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D10E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08D10E: move.b #$7, $21(a6)
pub fn i_08d10e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D114;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08D114: move.b #$f, d0
pub fn i_08d114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D118;
    cpu.ir = 0x103C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $08D118: jsr $43568.l
pub fn i_08d118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D11E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D11E);
    cpu.pc = 0x43568;
    16
}
/// $08D11E: move.w #$ffff, $106f5e.l
pub fn i_08d11e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D126;
    cpu.ir = 0x33FC;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x106F5E, v1 as u16);
    16
}
/// $08D126: move.l #$0, $106f64.l
pub fn i_08d126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D130;
    cpu.ir = 0x23FC;
    let v1: u32 = 0x0;
    bus.write32(0x106F64, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $08D130: cmpi.b #$1, $106eae.l
pub fn i_08d130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D138;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106EAE) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $08D138: bne.w $8d14a
pub fn i_08d138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D13C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8D14A; return 10; }
    8
}
/// $08D13C: move.l #$1c000, $106f60.l
pub fn i_08d13c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D146;
    cpu.ir = 0x23FC;
    let v1: u32 = 0x1C000;
    bus.write32(0x106F60, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $08D146: bra.w $8d154
pub fn i_08d146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D14A;
    cpu.ir = 0x6000;
    cpu.pc = 0x8D154;
    10
}
/// $08D14A: move.l #$18000, $106f60.l
pub fn i_08d14a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D154;
    cpu.ir = 0x23FC;
    let v1: u32 = 0x18000;
    bus.write32(0x106F60, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $08D154: jsr $2230.l
pub fn i_08d154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D15A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D15A);
    cpu.pc = 0x2230;
    16
}
/// $08D15A: lea.l $8d3b4(pc), a1
pub fn i_08d15a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D15E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8D3B4;
    16
}
/// $08D15E: jsr $4ae.l
pub fn i_08d15e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D164;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D164);
    cpu.pc = 0x4AE;
    16
}
/// $08D164: clr.b $20(a6)
pub fn i_08d164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D168;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $08D168: lea.l $8d16e(pc), a1
pub fn i_08d168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D16C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8D16E;
    16
}
/// $08D16C: move.l a1, (a6)
pub fn i_08d16c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D16E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08D16E: jsr $437da.l
pub fn i_08d16e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D174;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D174);
    cpu.pc = 0x437DA;
    16
}
/// $08D174: jsr $96a0e.l
pub fn i_08d174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D17A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D17A);
    cpu.pc = 0x96A0E;
    16
}
// ===== JsrAbsThunk_08d17a  ($08D17A, 8 B, thunks/jsr_abs_thunks.c)
/// $08D17A: jsr $96a80.l
pub fn i_08d17a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D180;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D180);
    cpu.pc = 0x96A80;
    16
}
/// $08D180: rts
pub fn i_08d180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D182;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Stub_0008D182  ($08D182, 2 B, thunks/trivial_rts.c)
/// $08D182: rts
pub fn i_08d182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D184;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Screen_InBoundsX_Latched_08d184  ($08D184, 30 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08D184: btst.b #$7, $13(a6)
pub fn i_08d184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D18A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $08D18A: beq.w $8d1ae
pub fn i_08d18a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D18E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8D1AE; return 10; }
    8
}
/// $08D18E: cmpi.w #$0, $22(a6)
pub fn i_08d18e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D194;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08D194: blt.w $8d1a8
pub fn i_08d194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D198;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x8D1A8; return 10; }
    8
}
/// $08D198: cmpi.w #$140, $22(a6)
pub fn i_08d198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D19E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x140, Size::Word, false);
    16
}
/// $08D19E: bgt.w $8d1a8
pub fn i_08d19e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D1A2;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8D1A8; return 10; }
    8
}
// ===== ClearXN_08d1a2  ($08D1A2, 6 B, thunks/ccr_helpers.c)
/// $08D1A2: andi.b #$ee, ccr
pub fn i_08d1a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D1A6;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $08D1A6: rts
pub fn i_08d1a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D1A8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_08d1a8  ($08D1A8, 6 B, thunks/ccr_helpers.c)
/// $08D1A8: ori.b #$11, ccr
pub fn i_08d1a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D1AC;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $08D1AC: rts
pub fn i_08d1ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D1AE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Screen_InBoundsX_Latch_08d1ae  ($08D1AE, 46 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08D1AE: cmpi.w #$140, $22(a6)
pub fn i_08d1ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D1B4;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x140, Size::Word, false);
    16
}
/// $08D1B4: bge.w $8d1e2
pub fn i_08d1b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D1B8;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x8D1E2; return 10; }
    8
}
/// $08D1B8: cmpi.w #$0, $22(a6)
pub fn i_08d1b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D1BE;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08D1BE: ble.w $8d1e2
pub fn i_08d1be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D1C2;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x8D1E2; return 10; }
    8
}
/// $08D1C2: cmpi.w #$1f0, $24(a6)
pub fn i_08d1c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D1C8;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1F0, Size::Word, false);
    16
}
/// $08D1C8: bge.w $8d1e2
pub fn i_08d1c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D1CC;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x8D1E2; return 10; }
    8
}
/// $08D1CC: cmpi.w #$100, $24(a6)
pub fn i_08d1cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D1D2;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x100, Size::Word, false);
    16
}
/// $08D1D2: ble.w $8d1e2
pub fn i_08d1d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D1D6;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x8D1E2; return 10; }
    8
}
/// $08D1D6: bset.b #$7, $13(a6)
pub fn i_08d1d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D1DC;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== ClearC_08d1dc  ($08D1DC, 6 B, thunks/ccr_helpers.c)
/// $08D1DC: andi.b #$fe, ccr
pub fn i_08d1dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D1E0;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $08D1E0: rts
pub fn i_08d1e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D1E2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearC_08d1e2  ($08D1E2, 6 B, thunks/ccr_helpers.c)
/// $08D1E2: andi.b #$fe, ccr
pub fn i_08d1e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D1E6;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $08D1E6: rts
pub fn i_08d1e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D1E8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Screen_InBoundsXWide_Latched_08d1e8  ($08D1E8, 30 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08D1E8: btst.b #$7, $13(a6)
pub fn i_08d1e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D1EE;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $08D1EE: beq.w $8d212
pub fn i_08d1ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D1F2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8D212; return 10; }
    8
}
/// $08D1F2: cmpi.w #$ffe0, $22(a6)
pub fn i_08d1f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D1F8;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFE0, Size::Word, false);
    16
}
/// $08D1F8: blt.w $8d20c
pub fn i_08d1f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D1FC;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x8D20C; return 10; }
    8
}
/// $08D1FC: cmpi.w #$140, $22(a6)
pub fn i_08d1fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D202;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x140, Size::Word, false);
    16
}
/// $08D202: bgt.w $8d20c
pub fn i_08d202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D206;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8D20C; return 10; }
    8
}
// ===== ClearXN_08d206  ($08D206, 6 B, thunks/ccr_helpers.c)
/// $08D206: andi.b #$ee, ccr
pub fn i_08d206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D20A;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $08D20A: rts
pub fn i_08d20a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D20C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_08d20c  ($08D20C, 6 B, thunks/ccr_helpers.c)
/// $08D20C: ori.b #$11, ccr
pub fn i_08d20c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D210;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $08D210: rts
pub fn i_08d210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D212;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Screen_InBoundsXWide_Latch_08d212  ($08D212, 46 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08D212: cmpi.w #$140, $22(a6)
pub fn i_08d212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D218;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x140, Size::Word, false);
    16
}
/// $08D218: bge.w $8d246
pub fn i_08d218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D21C;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x8D246; return 10; }
    8
}
/// $08D21C: cmpi.w #$0, $22(a6)
pub fn i_08d21c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D222;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08D222: ble.w $8d246
pub fn i_08d222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D226;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x8D246; return 10; }
    8
}
/// $08D226: cmpi.w #$1f0, $24(a6)
pub fn i_08d226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D22C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1F0, Size::Word, false);
    16
}
/// $08D22C: bge.w $8d246
pub fn i_08d22c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D230;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x8D246; return 10; }
    8
}
/// $08D230: cmpi.w #$100, $24(a6)
pub fn i_08d230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D236;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x100, Size::Word, false);
    16
}
/// $08D236: ble.w $8d246
pub fn i_08d236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D23A;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x8D246; return 10; }
    8
}
/// $08D23A: bset.b #$7, $13(a6)
pub fn i_08d23a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D240;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== ClearC_08d240  ($08D240, 6 B, thunks/ccr_helpers.c)
/// $08D240: andi.b #$fe, ccr
pub fn i_08d240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D244;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $08D244: rts
pub fn i_08d244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D246;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearC_08d246  ($08D246, 6 B, thunks/ccr_helpers.c)
/// $08D246: andi.b #$fe, ccr
pub fn i_08d246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D24A;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $08D24A: rts
pub fn i_08d24a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D24C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Screen_InBoundsY_Latched_08d24c  ($08D24C, 30 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08D24C: btst.b #$7, $13(a6)
pub fn i_08d24c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D252;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $08D252: beq.w $8d276
pub fn i_08d252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D256;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8D276; return 10; }
    8
}
/// $08D256: cmpi.w #$1ff, $24(a6)
pub fn i_08d256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D25C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1FF, Size::Word, false);
    16
}
/// $08D25C: bgt.w $8d270
pub fn i_08d25c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D260;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8D270; return 10; }
    8
}
/// $08D260: cmpi.w #$100, $24(a6)
pub fn i_08d260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D266;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x100, Size::Word, false);
    16
}
/// $08D266: blt.w $8d270
pub fn i_08d266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D26A;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x8D270; return 10; }
    8
}
// ===== ClearXN_08d26a  ($08D26A, 6 B, thunks/ccr_helpers.c)
/// $08D26A: andi.b #$ee, ccr
pub fn i_08d26a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D26E;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $08D26E: rts
pub fn i_08d26e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D270;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_08d270  ($08D270, 6 B, thunks/ccr_helpers.c)
/// $08D270: ori.b #$11, ccr
pub fn i_08d270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D274;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $08D274: rts
pub fn i_08d274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D276;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Screen_InBoundsY_Latch_08d276  ($08D276, 46 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08D276: cmpi.w #$140, $22(a6)
pub fn i_08d276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D27C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x140, Size::Word, false);
    16
}
/// $08D27C: bge.w $8d2aa
pub fn i_08d27c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D280;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x8D2AA; return 10; }
    8
}
/// $08D280: cmpi.w #$0, $22(a6)
pub fn i_08d280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D286;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08D286: ble.w $8d2aa
pub fn i_08d286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D28A;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x8D2AA; return 10; }
    8
}
/// $08D28A: cmpi.w #$1ff, $24(a6)
pub fn i_08d28a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D290;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1FF, Size::Word, false);
    16
}
/// $08D290: bge.w $8d2aa
pub fn i_08d290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D294;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x8D2AA; return 10; }
    8
}
/// $08D294: cmpi.w #$100, $24(a6)
pub fn i_08d294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D29A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x100, Size::Word, false);
    16
}
/// $08D29A: ble.w $8d2aa
pub fn i_08d29a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D29E;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x8D2AA; return 10; }
    8
}
/// $08D29E: bset.b #$7, $13(a6)
pub fn i_08d29e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D2A4;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== ClearC_08d2a4  ($08D2A4, 6 B, thunks/ccr_helpers.c)
/// $08D2A4: andi.b #$fe, ccr
pub fn i_08d2a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D2A8;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $08D2A8: rts
pub fn i_08d2a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D2AA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearC_08d2aa  ($08D2AA, 6 B, thunks/ccr_helpers.c)
/// $08D2AA: andi.b #$fe, ccr
pub fn i_08d2aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D2AE;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $08D2AE: rts
pub fn i_08d2ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D2B0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pos_IntegrateX88_08d2b0  ($08D2B0, 30 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08D2B0: move.w $22(a6), d0
pub fn i_08d2b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D2B4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08D2B4: swap d0
pub fn i_08d2b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D2B6;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08D2B6: move.w $26(a6), d0
pub fn i_08d2b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D2BA;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x26);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08D2BA: clr.b d0
pub fn i_08d2ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D2BC;
    cpu.ir = 0x4200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $08D2BC: move.w $28(a6), d1
pub fn i_08d2bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D2C0;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08D2C0: ext.l d1
pub fn i_08d2c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D2C2;
    cpu.ir = 0x48C1;
    cpu.d[1] = cpu.d[1] as i16 as i32 as u32;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08D2C2: asl.l #$8, d1
pub fn i_08d2c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D2C4;
    cpu.ir = 0xE181;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, true);
    cpu.d[1] = r;
    22
}
/// $08D2C4: add.l d1, d0
pub fn i_08d2c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D2C6;
    cpu.ir = 0xD081;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[0] = r;
    4
}
/// $08D2C6: asr.w #$8, d0
pub fn i_08d2c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D2C8;
    cpu.ir = 0xE040;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $08D2C8: move.b d0, $26(a6)
pub fn i_08d2c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D2CC;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x26);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08D2CC: swap d0
pub fn i_08d2cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D2CE;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
// ===== SetTaskW_08d2ce  ($08D2CE, 6 B, kernel/task_setters_w.c)
/// $08D2CE: move.w d0, $22(a6)
pub fn i_08d2ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D2D2;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08D2D2: rts
pub fn i_08d2d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D2D4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pos_IntegrateY88_08d2d4  ($08D2D4, 30 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08D2D4: move.w $24(a6), d0
pub fn i_08d2d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D2D8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08D2D8: swap d0
pub fn i_08d2d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D2DA;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08D2DA: move.b $27(a6), d0
pub fn i_08d2da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D2DE;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x27);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08D2DE: asl.w #$8, d0
pub fn i_08d2de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D2E0;
    cpu.ir = 0xE140;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $08D2E0: move.w $2a(a6), d1
pub fn i_08d2e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D2E4;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08D2E4: ext.l d1
pub fn i_08d2e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D2E6;
    cpu.ir = 0x48C1;
    cpu.d[1] = cpu.d[1] as i16 as i32 as u32;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08D2E6: asl.l #$8, d1
pub fn i_08d2e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D2E8;
    cpu.ir = 0xE181;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, true);
    cpu.d[1] = r;
    22
}
/// $08D2E8: add.l d1, d0
pub fn i_08d2e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D2EA;
    cpu.ir = 0xD081;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[0] = r;
    4
}
/// $08D2EA: asr.w #$8, d0
pub fn i_08d2ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D2EC;
    cpu.ir = 0xE040;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $08D2EC: move.b d0, $27(a6)
pub fn i_08d2ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D2F0;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x27);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08D2F0: swap d0
pub fn i_08d2f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D2F2;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
// ===== SetTaskW_08d2f2  ($08D2F2, 6 B, kernel/task_setters_w.c)
/// $08D2F2: move.w d0, $24(a6)
pub fn i_08d2f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D2F6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08D2F6: rts
pub fn i_08d2f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D2F8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pos_IntegrateXY88_08d2f8  ($08D2F8, 80 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08D2F8: tst.w $28(a6)
pub fn i_08d2f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D2FC;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $08D2FC: beq.w $8d322
pub fn i_08d2fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D300;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8D322; return 10; }
    8
}
/// $08D300: move.w $22(a6), d0
pub fn i_08d300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D304;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08D304: swap d0
pub fn i_08d304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D306;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08D306: move.w $26(a6), d0
pub fn i_08d306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D30A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x26);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08D30A: clr.b d0
pub fn i_08d30a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D30C;
    cpu.ir = 0x4200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $08D30C: move.w $28(a6), d1
pub fn i_08d30c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D310;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08D310: ext.l d1
pub fn i_08d310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D312;
    cpu.ir = 0x48C1;
    cpu.d[1] = cpu.d[1] as i16 as i32 as u32;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08D312: asl.l #$8, d1
pub fn i_08d312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D314;
    cpu.ir = 0xE181;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, true);
    cpu.d[1] = r;
    22
}
/// $08D314: add.l d1, d0
pub fn i_08d314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D316;
    cpu.ir = 0xD081;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[0] = r;
    4
}
/// $08D316: asr.w #$8, d0
pub fn i_08d316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D318;
    cpu.ir = 0xE040;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $08D318: move.b d0, $26(a6)
pub fn i_08d318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D31C;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x26);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08D31C: swap d0
pub fn i_08d31c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D31E;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08D31E: move.w d0, $22(a6)
pub fn i_08d31e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D322;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08D322: tst.w $2a(a6)
pub fn i_08d322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D326;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $08D326: beq.w $8d34c
pub fn i_08d326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D32A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8D34C; return 10; }
    8
}
/// $08D32A: move.w $24(a6), d0
pub fn i_08d32a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D32E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08D32E: swap d0
pub fn i_08d32e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D330;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08D330: move.b $27(a6), d0
pub fn i_08d330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D334;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x27);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08D334: asl.w #$8, d0
pub fn i_08d334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D336;
    cpu.ir = 0xE140;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $08D336: move.w $2a(a6), d1
pub fn i_08d336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D33A;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08D33A: ext.l d1
pub fn i_08d33a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D33C;
    cpu.ir = 0x48C1;
    cpu.d[1] = cpu.d[1] as i16 as i32 as u32;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08D33C: asl.l #$8, d1
pub fn i_08d33c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D33E;
    cpu.ir = 0xE181;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, true);
    cpu.d[1] = r;
    22
}
/// $08D33E: add.l d1, d0
pub fn i_08d33e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D340;
    cpu.ir = 0xD081;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[0] = r;
    4
}
/// $08D340: asr.w #$8, d0
pub fn i_08d340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D342;
    cpu.ir = 0xE040;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $08D342: move.b d0, $27(a6)
pub fn i_08d342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D346;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x27);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08D346: swap d0
pub fn i_08d346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D348;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
// ===== SetTaskW_08d348  ($08D348, 6 B, kernel/task_setters_w.c)
/// $08D348: move.w d0, $24(a6)
pub fn i_08d348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D34C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08D34C: rts
pub fn i_08d34c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D34E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pos_IntegrateXY88_Accel_08d34e  ($08D34E, 102 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08D34E: tst.w $28(a6)
pub fn i_08d34e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D352;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $08D352: beq.w $8d378
pub fn i_08d352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D356;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8D378; return 10; }
    8
}
/// $08D356: move.w $22(a6), d0
pub fn i_08d356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D35A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08D35A: swap d0
pub fn i_08d35a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D35C;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08D35C: move.w $26(a6), d0
pub fn i_08d35c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D360;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x26);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08D360: clr.b d0
pub fn i_08d360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D362;
    cpu.ir = 0x4200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $08D362: move.w $28(a6), d1
pub fn i_08d362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D366;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08D366: ext.l d1
pub fn i_08d366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D368;
    cpu.ir = 0x48C1;
    cpu.d[1] = cpu.d[1] as i16 as i32 as u32;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08D368: asl.l #$8, d1
pub fn i_08d368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D36A;
    cpu.ir = 0xE181;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, true);
    cpu.d[1] = r;
    22
}
/// $08D36A: add.l d1, d0
pub fn i_08d36a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D36C;
    cpu.ir = 0xD081;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[0] = r;
    4
}
/// $08D36C: asr.w #$8, d0
pub fn i_08d36c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D36E;
    cpu.ir = 0xE040;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $08D36E: move.b d0, $26(a6)
pub fn i_08d36e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D372;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x26);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08D372: swap d0
pub fn i_08d372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D374;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08D374: move.w d0, $22(a6)
pub fn i_08d374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D378;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08D378: tst.w $2a(a6)
pub fn i_08d378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D37C;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $08D37C: beq.w $8d3a2
pub fn i_08d37c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D380;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8D3A2; return 10; }
    8
}
/// $08D380: move.w $24(a6), d0
pub fn i_08d380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D384;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08D384: swap d0
pub fn i_08d384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D386;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08D386: move.b $27(a6), d0
pub fn i_08d386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D38A;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x27);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08D38A: asl.w #$8, d0
pub fn i_08d38a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D38C;
    cpu.ir = 0xE140;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $08D38C: move.w $2a(a6), d1
pub fn i_08d38c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D390;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08D390: ext.l d1
pub fn i_08d390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D392;
    cpu.ir = 0x48C1;
    cpu.d[1] = cpu.d[1] as i16 as i32 as u32;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08D392: asl.l #$8, d1
pub fn i_08d392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D394;
    cpu.ir = 0xE181;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, true);
    cpu.d[1] = r;
    22
}
/// $08D394: add.l d1, d0
pub fn i_08d394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D396;
    cpu.ir = 0xD081;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[0] = r;
    4
}
/// $08D396: asr.w #$8, d0
pub fn i_08d396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D398;
    cpu.ir = 0xE040;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $08D398: move.b d0, $27(a6)
pub fn i_08d398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D39C;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x27);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08D39C: swap d0
pub fn i_08d39c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D39E;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08D39E: move.w d0, $24(a6)
pub fn i_08d39e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D3A2;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08D3A2: move.w $2c(a6), d0
pub fn i_08d3a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D3A6;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08D3A6: add.w d0, $28(a6)
pub fn i_08d3a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D3AA;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08D3AA: move.w $2e(a6), d0
pub fn i_08d3aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D3AE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08D3AE: add.w d0, $2a(a6)
pub fn i_08d3ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D3B2;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08D3B2: rts
pub fn i_08d3b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D3B4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Capsule_Fly_08d3b4  ($08D3B4, 96 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08D3B4: move.w #$8000, $38(a6)
pub fn i_08d3b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D3BA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08D3BA: move.w #$e, d1
pub fn i_08d3ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D3BE;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08D3BE: jsr $236e.l
pub fn i_08d3be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D3C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D3C4);
    cpu.pc = 0x236E;
    16
}
/// $08D3C4: lea.l $2f460c.l, a0
pub fn i_08d3c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D3CA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F460C;
    20
}
/// $08D3CA: jsr $28cd4.l
pub fn i_08d3ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D3D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D3D0);
    cpu.pc = 0x28CD4;
    16
}
/// $08D3D0: move.b #$1, $3a(a6)
pub fn i_08d3d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D3D6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08D3D6: move.w #$120, $22(a6)
pub fn i_08d3d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D3DC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x120;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08D3DC: move.w #$190, $24(a6)
pub fn i_08d3dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D3E2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x190;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08D3E2: clr.w $26(a6)
pub fn i_08d3e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D3E6;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x26);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08D3E6: jsr $5e7c0.l
pub fn i_08d3e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D3EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D3EC);
    cpu.pc = 0x5E7C0;
    16
}
/// $08D3EC: lea.l $8d3f2(pc), a1
pub fn i_08d3ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D3F0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8D3F2;
    16
}
/// $08D3F0: move.l a1, (a6)
pub fn i_08d3f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D3F2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08D3F2: jsr $2783a.l
pub fn i_08d3f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D3F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D3F8);
    cpu.pc = 0x2783A;
    16
}
/// $08D3F8: jsr $28d70.l
pub fn i_08d3f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D3FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D3FE);
    cpu.pc = 0x28D70;
    16
}
/// $08D3FE: cmpi.w #$100, $22(a6)
pub fn i_08d3fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D404;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x100, Size::Word, false);
    16
}
/// $08D404: bgt.w $8d41a
pub fn i_08d404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D408;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8D41A; return 10; }
    8
}
/// $08D408: lea.l $2f4628.l, a0
pub fn i_08d408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D40E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4628;
    20
}
/// $08D40E: jsr $28cd4.l
pub fn i_08d40e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D414;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D414);
    cpu.pc = 0x28CD4;
    16
}
// ===== SetTaskHandler_08d414  ($08D414, 8 B, kernel/task_handlers.c)
/// $08D414: lea.l $8d41c(pc), a1
pub fn i_08d414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D418;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8D41C;
    16
}
/// $08D418: move.l a1, (a6)
pub fn i_08d418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D41A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08D41A: rts
pub fn i_08d41a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D41C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Capsule_Descend_08d41c  ($08D41C, 44 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08D41C: jsr $2783a.l
pub fn i_08d41c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D422;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D422);
    cpu.pc = 0x2783A;
    16
}
/// $08D422: jsr $28d70.l
pub fn i_08d422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D428;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D428);
    cpu.pc = 0x28D70;
    16
}
/// $08D428: bcc.w $8d44e
pub fn i_08d428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D42C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8D44E; return 10; }
    8
}
/// $08D42C: lea.l $8d474(pc), a1
pub fn i_08d42c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D430;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8D474;
    16
}
/// $08D430: jsr $4ae.l
pub fn i_08d430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D436;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D436);
    cpu.pc = 0x4AE;
    16
}
/// $08D436: jsr $5dd02.l
pub fn i_08d436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D43C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D43C);
    cpu.pc = 0x5DD02;
    16
}
/// $08D43C: lea.l $2f46ca.l, a0
pub fn i_08d43c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D442;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F46CA;
    20
}
/// $08D442: jsr $28cd4.l
pub fn i_08d442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D448;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D448);
    cpu.pc = 0x28CD4;
    16
}
// ===== SetTaskHandler_08d448  ($08D448, 8 B, kernel/task_handlers.c)
/// $08D448: lea.l $8d450(pc), a1
pub fn i_08d448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D44C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8D450;
    16
}
/// $08D44C: move.l a1, (a6)
pub fn i_08d44c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D44E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08D44E: rts
pub fn i_08d44e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D450;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Capsule_WaitGround_08d450  ($08D450, 28 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08D450: jsr $2783a.l
pub fn i_08d450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D456;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D456);
    cpu.pc = 0x2783A;
    16
}
/// $08D456: jsr $28d70.l
pub fn i_08d456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D45C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D45C);
    cpu.pc = 0x28D70;
    16
}
/// $08D45C: movea.l #$ffffffff, a0
pub fn i_08d45c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D462;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $08D462: jsr $5dd5c.l
pub fn i_08d462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D468;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D468);
    cpu.pc = 0x5DD5C;
    16
}
/// $08D468: bcc.w $8d472
pub fn i_08d468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D46C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8D472; return 10; }
    8
}
// ===== SetTaskHandler_08d46c  ($08D46C, 8 B, kernel/task_handlers.c)
/// $08D46C: lea.l $8d472(pc), a1
pub fn i_08d46c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D470;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8D472;
    16
}
/// $08D470: move.l a1, (a6)
pub fn i_08d470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D472;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08D472: rts
pub fn i_08d472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D474;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Capsule_Open_08d474  ($08D474, 74 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08D474: clr.b $3a(a6)
pub fn i_08d474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D478;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $08D478: move.w #$f000, $38(a6)
pub fn i_08d478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D47E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08D47E: move.w #$f5, d1
pub fn i_08d47e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D482;
    cpu.ir = 0x323C;
    let v1: u32 = 0xF5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08D482: jsr $236e.l
pub fn i_08d482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D488;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D488);
    cpu.pc = 0x236E;
    16
}
/// $08D488: lea.l $2f3904.l, a0
pub fn i_08d488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D48E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3904;
    20
}
/// $08D48E: jsr $28cd4.l
pub fn i_08d48e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D494;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D494);
    cpu.pc = 0x28CD4;
    16
}
/// $08D494: addi.w #$4, $22(a6)
pub fn i_08d494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D49A;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x4, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08D49A: addi.w #$20, $24(a6)
pub fn i_08d49a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D4A0;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08D4A0: move.b #$ff, $32(a6)
pub fn i_08d4a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D4A6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08D4A6: move.b #$ff, $33(a6)
pub fn i_08d4a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D4AC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08D4AC: move.b #$0, $5c(a6)
pub fn i_08d4ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D4B2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08D4B2: move.w #$200, $28(a6)
pub fn i_08d4b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D4B8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x200;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08D4B8: move.w #$4, $70(a6)
pub fn i_08d4b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D4BE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_08d4be  ($08D4BE, 8 B, kernel/task_handlers.c)
/// $08D4BE: lea.l $8d4c6(pc), a1
pub fn i_08d4be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D4C2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8D4C6;
    16
}
/// $08D4C2: move.l a1, (a6)
pub fn i_08d4c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D4C4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08D4C4: rts
pub fn i_08d4c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D4C6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Capsule_Flash_08d4c6  ($08D4C6, 48 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08D4C6: jsr $8d2b0(pc)
pub fn i_08d4c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D4CA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8D4CA);
    cpu.pc = 0x8D2B0;
    16
}
/// $08D4CA: jsr $28d70.l
pub fn i_08d4ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D4D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D4D0);
    cpu.pc = 0x28D70;
    16
}
/// $08D4D0: subq.w #$1, $70(a6)
pub fn i_08d4d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D4D4;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08D4D4: cmpi.w #$0, $70(a6)
pub fn i_08d4d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D4DA;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08D4DA: bgt.w $8d4fc
pub fn i_08d4da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D4DE;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8D4FC; return 10; }
    8
}
/// $08D4DE: move.b #$38, $32(a6)
pub fn i_08d4de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D4E4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x38;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08D4E4: move.b #$38, $33(a6)
pub fn i_08d4e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D4EA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x38;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08D4EA: lea.l $2f38f8.l, a0
pub fn i_08d4ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D4F0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F38F8;
    20
}
/// $08D4F0: jsr $28cd4.l
pub fn i_08d4f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D4F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D4F6);
    cpu.pc = 0x28CD4;
    16
}
// ===== SetTaskHandler_08d4f6  ($08D4F6, 8 B, kernel/task_handlers.c)
/// $08D4F6: lea.l $8d4fe(pc), a1
pub fn i_08d4f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D4FA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8D4FE;
    16
}
/// $08D4FA: move.l a1, (a6)
pub fn i_08d4fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D4FC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08D4FC: rts
pub fn i_08d4fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D4FE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Capsule_FadeIn_08d4fe  ($08D4FE, 50 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08D4FE: addq.b #$2, $32(a6)
pub fn i_08d4fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D502;
    cpu.ir = 0x542E;
    let ad1 = cpu.a[6].wrapping_add(0x32);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $08D502: addq.b #$2, $33(a6)
pub fn i_08d502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D506;
    cpu.ir = 0x542E;
    let ad1 = cpu.a[6].wrapping_add(0x33);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $08D506: cmpi.b #$7f, $32(a6)
pub fn i_08d506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D50C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x7F, Size::Byte, false);
    16
}
/// $08D50C: bcs.w $8d51c
pub fn i_08d50c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D510;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8D51C; return 10; }
    8
}
/// $08D510: move.b #$7f, $32(a6)
pub fn i_08d510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D516;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x7F;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08D516: move.b #$7f, $33(a6)
pub fn i_08d516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D51C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x7F;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08D51C: jsr $8d2b0(pc)
pub fn i_08d51c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D520;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8D520);
    cpu.pc = 0x8D2B0;
    16
}
/// $08D520: jsr $28d70.l
pub fn i_08d520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D526;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D526);
    cpu.pc = 0x28D70;
    16
}
/// $08D526: cmpi.w #$a0, $22(a6)
pub fn i_08d526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D52C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xA0, Size::Word, false);
    16
}
/// $08D52C: blt.w $8d536
pub fn i_08d52c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D530;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x8D536; return 10; }
    8
}
// ===== SetTaskHandler_08d530  ($08D530, 8 B, kernel/task_handlers.c)
/// $08D530: lea.l $8d5a8(pc), a1
pub fn i_08d530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D534;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8D5A8;
    16
}
/// $08D534: move.l a1, (a6)
pub fn i_08d534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D536;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08D536: rts
pub fn i_08d536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D538;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Capsule_Glow_08d538  ($08D538, 28 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08D538: lea.l $2f377e.l, a0
pub fn i_08d538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D53E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F377E;
    20
}
/// $08D53E: jsr $28cd4.l
pub fn i_08d53e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D544;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D544);
    cpu.pc = 0x28CD4;
    16
}
/// $08D544: lea.l $8d54a(pc), a1
pub fn i_08d544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D548;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8D54A;
    16
}
/// $08D548: move.l a1, (a6)
pub fn i_08d548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D54A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08D54A: jsr $28d70.l
pub fn i_08d54a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D550;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D550);
    cpu.pc = 0x28D70;
    16
}
/// $08D550: bcc.w $8d55a
pub fn i_08d550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D554;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8D55A; return 10; }
    8
}
// ===== SetTaskHandler_08d554  ($08D554, 8 B, kernel/task_handlers.c)
/// $08D554: lea.l $8d55c(pc), a1
pub fn i_08d554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D558;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8D55C;
    16
}
/// $08D558: move.l a1, (a6)
pub fn i_08d558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D55A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08D55A: rts
pub fn i_08d55a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D55C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Capsule_GlowDown_08d55c  ($08D55C, 28 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08D55C: subq.b #$2, $32(a6)
pub fn i_08d55c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D560;
    cpu.ir = 0x552E;
    let ad1 = cpu.a[6].wrapping_add(0x32);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 2, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $08D560: subq.b #$2, $33(a6)
pub fn i_08d560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D564;
    cpu.ir = 0x552E;
    let ad1 = cpu.a[6].wrapping_add(0x33);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 2, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $08D564: cmpi.b #$7f, $32(a6)
pub fn i_08d564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D56A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x7F, Size::Byte, false);
    16
}
/// $08D56A: bhi.w $8d574
pub fn i_08d56a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D56E;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x8D574; return 10; }
    8
}
/// $08D56E: lea.l $8d580(pc), a1
pub fn i_08d56e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D572;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8D580;
    16
}
/// $08D572: move.l a1, (a6)
pub fn i_08d572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D574;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08D574: jsr $8d804(pc)
pub fn i_08d574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D578;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8D578);
    cpu.pc = 0x8D804;
    16
}
// ===== JsrAbsThunk_08d578  ($08D578, 8 B, thunks/jsr_abs_thunks.c)
/// $08D578: jsr $28d70.l
pub fn i_08d578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D57E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D57E);
    cpu.pc = 0x28D70;
    16
}
/// $08D57E: rts
pub fn i_08d57e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D580;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Capsule_Glow2_08d580  ($08D580, 34 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08D580: lea.l $2f37da.l, a0
pub fn i_08d580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D586;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F37DA;
    20
}
/// $08D586: jsr $28cd4.l
pub fn i_08d586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D58C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D58C);
    cpu.pc = 0x28CD4;
    16
}
/// $08D58C: lea.l $8d592(pc), a1
pub fn i_08d58c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D590;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8D592;
    16
}
/// $08D590: move.l a1, (a6)
pub fn i_08d590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D592;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08D592: jsr $28d70.l
pub fn i_08d592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D598;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D598);
    cpu.pc = 0x28D70;
    16
}
/// $08D598: bcc.w $8d5a2
pub fn i_08d598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D59C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8D5A2; return 10; }
    8
}
/// $08D59C: lea.l $8d5a8(pc), a1
pub fn i_08d59c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D5A0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8D5A8;
    16
}
/// $08D5A0: move.l a1, (a6)
pub fn i_08d5a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D5A2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrPcThunk_08d5a2  ($08D5A2, 6 B, thunks/jsr_pc_thunks.c)
/// $08D5A2: jsr $8d804(pc)
pub fn i_08d5a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D5A6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8D5A6);
    cpu.pc = 0x8D804;
    16
}
/// $08D5A6: rts
pub fn i_08d5a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D5A8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Capsule_GlowUp_08d5a8  ($08D5A8, 30 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08D5A8: addq.b #$2, $32(a6)
pub fn i_08d5a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D5AC;
    cpu.ir = 0x542E;
    let ad1 = cpu.a[6].wrapping_add(0x32);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $08D5AC: addq.b #$2, $33(a6)
pub fn i_08d5ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D5B0;
    cpu.ir = 0x542E;
    let ad1 = cpu.a[6].wrapping_add(0x33);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $08D5B0: cmpi.b #$d0, $32(a6)
pub fn i_08d5b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D5B6;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xD0, Size::Byte, false);
    16
}
/// $08D5B6: bcs.w $8d5c0
pub fn i_08d5b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D5BA;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8D5C0; return 10; }
    8
}
/// $08D5BA: lea.l $8d538(pc), a1
pub fn i_08d5ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D5BE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8D538;
    16
}
/// $08D5BE: move.l a1, (a6)
pub fn i_08d5be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D5C0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08D5C0: jsr $28d70.l
pub fn i_08d5c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D5C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D5C6);
    cpu.pc = 0x28D70;
    16
}
// ===== JsrPcThunk_08d5c6  ($08D5C6, 6 B, thunks/jsr_pc_thunks.c)
/// $08D5C6: jsr $8d804(pc)
pub fn i_08d5c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D5CA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8D5CA);
    cpu.pc = 0x8D804;
    16
}
/// $08D5CA: rts
pub fn i_08d5ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D5CC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Capsule_Rise_08d5cc  ($08D5CC, 90 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08D5CC: move.w $24(a6), $5c(a6)
pub fn i_08d5cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D5D2;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08D5D2: move.w #$ffc0, $2a(a6)
pub fn i_08d5d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D5D8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFC0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08D5D8: move.w #$2000, $38(a6)
pub fn i_08d5d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D5DE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08D5DE: move.w #$c8, $70(a6)
pub fn i_08d5de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D5E4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08D5E4: lea.l $8d5ea(pc), a1
pub fn i_08d5e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D5E8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8D5EA;
    16
}
/// $08D5E8: move.l a1, (a6)
pub fn i_08d5e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D5EA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08D5EA: subq.b #$2, $32(a6)
pub fn i_08d5ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D5EE;
    cpu.ir = 0x552E;
    let ad1 = cpu.a[6].wrapping_add(0x32);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 2, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $08D5EE: subq.b #$2, $33(a6)
pub fn i_08d5ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D5F2;
    cpu.ir = 0x552E;
    let ad1 = cpu.a[6].wrapping_add(0x33);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 2, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $08D5F2: cmpi.b #$34, $32(a6)
pub fn i_08d5f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D5F8;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x34, Size::Byte, false);
    16
}
/// $08D5F8: bhi.w $8d608
pub fn i_08d5f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D5FC;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x8D608; return 10; }
    8
}
/// $08D5FC: move.b #$34, $32(a6)
pub fn i_08d5fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D602;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x34;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08D602: move.b #$34, $33(a6)
pub fn i_08d602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D608;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x34;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08D608: jsr $8d2d4(pc)
pub fn i_08d608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D60C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8D60C);
    cpu.pc = 0x8D2D4;
    16
}
/// $08D60C: jsr $28d70.l
pub fn i_08d60c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D612;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D612);
    cpu.pc = 0x28D70;
    16
}
/// $08D612: subq.w #$1, $70(a6)
pub fn i_08d612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D616;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08D616: cmpi.w #$0, $70(a6)
pub fn i_08d616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D61C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08D61C: bgt.w $8d62c
pub fn i_08d61c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D620;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8D62C; return 10; }
    8
}
/// $08D620: move.w #$80, $2a(a6)
pub fn i_08d620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D626;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_08d626  ($08D626, 8 B, kernel/task_handlers.c)
/// $08D626: lea.l $8d62e(pc), a1
pub fn i_08d626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D62A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8D62E;
    16
}
/// $08D62A: move.l a1, (a6)
pub fn i_08d62a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D62C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08D62C: rts
pub fn i_08d62c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D62E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Capsule_Fall_08d62e  ($08D62E, 58 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08D62E: addq.b #$2, $32(a6)
pub fn i_08d62e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D632;
    cpu.ir = 0x542E;
    let ad1 = cpu.a[6].wrapping_add(0x32);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $08D632: addq.b #$2, $33(a6)
pub fn i_08d632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D636;
    cpu.ir = 0x542E;
    let ad1 = cpu.a[6].wrapping_add(0x33);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $08D636: cmpi.b #$7f, $32(a6)
pub fn i_08d636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D63C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x7F, Size::Byte, false);
    16
}
/// $08D63C: bcs.w $8d64c
pub fn i_08d63c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D640;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8D64C; return 10; }
    8
}
/// $08D640: move.b #$7f, $32(a6)
pub fn i_08d640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D646;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x7F;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08D646: move.b #$7f, $33(a6)
pub fn i_08d646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D64C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x7F;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08D64C: jsr $8d2d4(pc)
pub fn i_08d64c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D650;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8D650);
    cpu.pc = 0x8D2D4;
    16
}
/// $08D650: jsr $28d70.l
pub fn i_08d650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D656;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D656);
    cpu.pc = 0x28D70;
    16
}
/// $08D656: move.w $5c(a6), d0
pub fn i_08d656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D65A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08D65A: cmp.w $24(a6), d0
pub fn i_08d65a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D65E;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $08D65E: bge.w $8d66e
pub fn i_08d65e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D662;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x8D66E; return 10; }
    8
}
/// $08D662: move.w #$f000, $38(a6)
pub fn i_08d662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D668;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_08d668  ($08D668, 8 B, kernel/task_handlers.c)
/// $08D668: lea.l $8d580(pc), a1
pub fn i_08d668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D66C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8D580;
    16
}
/// $08D66C: move.l a1, (a6)
pub fn i_08d66c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D66E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08D66E: rts
pub fn i_08d66e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D670;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionEnd_ScrollOut_08d670  ($08D670, 116 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08D670: move.b #$0, $5c(a6)
pub fn i_08d670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D676;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08D676: move.l #$20000, $80(a6)
pub fn i_08d676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D67E;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x20000;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08D67E: move.l #$0, $84(a6)
pub fn i_08d67e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D686;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x0;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08D686: move.l #$ffff0000, $88(a6)
pub fn i_08d686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D68E;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFF0000;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08D68E: move.l #$0, $8c(a6)
pub fn i_08d68e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D696;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x0;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08D696: lea.l $2f3910.l, a0
pub fn i_08d696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D69C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3910;
    20
}
/// $08D69C: jsr $28cd4.l
pub fn i_08d69c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D6A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D6A2);
    cpu.pc = 0x28CD4;
    16
}
/// $08D6A2: lea.l $8d6a8(pc), a1
pub fn i_08d6a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D6A6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8D6A8;
    16
}
/// $08D6A6: move.l a1, (a6)
pub fn i_08d6a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D6A8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08D6A8: jsr $8ccc6(pc)
pub fn i_08d6a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D6AC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8D6AC);
    cpu.pc = 0x8CCC6;
    16
}
/// $08D6AC: jsr $8ccd8(pc)
pub fn i_08d6ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D6B0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8D6B0);
    cpu.pc = 0x8CCD8;
    16
}
/// $08D6B0: jsr $8efb0(pc)
pub fn i_08d6b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D6B4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8D6B4);
    cpu.pc = 0x8EFB0;
    16
}
/// $08D6B4: cmpi.b #$2, $32(a6)
pub fn i_08d6b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D6BA;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $08D6BA: bcs.w $8d6ce
pub fn i_08d6ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D6BE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8D6CE; return 10; }
    8
}
/// $08D6BE: move.b $5c(a6), d0
pub fn i_08d6be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D6C2;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08D6C2: add.b d0, $32(a6)
pub fn i_08d6c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D6C6;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x32);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $08D6C6: add.b d0, $33(a6)
pub fn i_08d6c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D6CA;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x33);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $08D6CA: bra.w $8d6e4
pub fn i_08d6ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D6CE;
    cpu.ir = 0x6000;
    cpu.pc = 0x8D6E4;
    10
}
/// $08D6CE: lea.l $8d6ec(pc), a1
pub fn i_08d6ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D6D2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8D6EC;
    16
}
/// $08D6D2: jsr $4ae.l
pub fn i_08d6d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D6D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D6D8);
    cpu.pc = 0x4AE;
    16
}
/// $08D6D8: jsr $5dd02.l
pub fn i_08d6d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D6DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D6DE);
    cpu.pc = 0x5DD02;
    16
}
/// $08D6DE: jmp $518.l
pub fn i_08d6de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D6E4;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== JsrAbsThunk_08d6e4  ($08D6E4, 8 B, thunks/jsr_abs_thunks.c)
/// $08D6E4: jsr $28d70.l
pub fn i_08d6e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D6EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D6EA);
    cpu.pc = 0x28D70;
    16
}
/// $08D6EA: rts
pub fn i_08d6ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D6EC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionEnd_Flash_08d6ec  ($08D6EC, 54 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08D6EC: move.w #$f4, d1
pub fn i_08d6ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D6F0;
    cpu.ir = 0x323C;
    let v1: u32 = 0xF4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08D6F0: jsr $236e.l
pub fn i_08d6f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D6F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D6F6);
    cpu.pc = 0x236E;
    16
}
/// $08D6F6: lea.l $2f44f6.l, a0
pub fn i_08d6f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D6FC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F44F6;
    20
}
/// $08D6FC: jsr $28cd4.l
pub fn i_08d6fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D702;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D702);
    cpu.pc = 0x28CD4;
    16
}
/// $08D702: move.w #$14, $70(a6)
pub fn i_08d702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D708;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08D708: lea.l $8d70e(pc), a1
pub fn i_08d708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D70C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8D70E;
    16
}
/// $08D70C: move.l a1, (a6)
pub fn i_08d70c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D70E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08D70E: jsr $4407a.l
pub fn i_08d70e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D714;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D714);
    cpu.pc = 0x4407A;
    16
}
/// $08D714: subq.w #$1, $70(a6)
pub fn i_08d714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D718;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08D718: cmpi.w #$0, $70(a6)
pub fn i_08d718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D71E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08D71E: bgt.w $8d728
pub fn i_08d71e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D722;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8D728; return 10; }
    8
}
// ===== SetTaskHandler_08d722  ($08D722, 8 B, kernel/task_handlers.c)
/// $08D722: lea.l $8d72a(pc), a1
pub fn i_08d722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D726;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8D72A;
    16
}
/// $08D726: move.l a1, (a6)
pub fn i_08d726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D728;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08D728: rts
pub fn i_08d728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D72A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionEnd_SpawnDropper_08d72a  ($08D72A, 74 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08D72A: jsr $4407a.l
pub fn i_08d72a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D730;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D730);
    cpu.pc = 0x4407A;
    16
}
/// $08D730: jsr $28d70.l
pub fn i_08d730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D736;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D736);
    cpu.pc = 0x28D70;
    16
}
/// $08D736: bcc.w $8d772
pub fn i_08d736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D73A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8D772; return 10; }
    8
}
/// $08D73A: lea.l $8edc6(pc), a1
pub fn i_08d73a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D73E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8EDC6;
    16
}
/// $08D73E: jsr $4ae.l
pub fn i_08d73e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D744;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D744);
    cpu.pc = 0x4AE;
    16
}
/// $08D744: jsr $5dd02.l
pub fn i_08d744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D74A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D74A);
    cpu.pc = 0x5DD02;
    16
}
/// $08D74A: lea.l $8ccea(pc), a1
pub fn i_08d74a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D74E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8CCEA;
    16
}
/// $08D74E: jsr $4ae.l
pub fn i_08d74e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D754;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D754);
    cpu.pc = 0x4AE;
    16
}
/// $08D754: move.b #$a, $98(a0)
pub fn i_08d754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D75A;
    cpu.ir = 0x117C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08D75A: move.b #$8, $99(a0)
pub fn i_08d75a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D760;
    cpu.ir = 0x117C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08D760: move.w #$a0, $22(a0)
pub fn i_08d760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D766;
    cpu.ir = 0x317C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08D766: move.w #$100, $24(a0)
pub fn i_08d766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D76C;
    cpu.ir = 0x317C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08D76C: jmp $518.l
pub fn i_08d76c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D772;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08D772: rts
pub fn i_08d772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D774;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Capsule_DimWaitScroll_08d774  ($08D774, 66 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08D774: subq.b #$2, $32(a6)
pub fn i_08d774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D778;
    cpu.ir = 0x552E;
    let ad1 = cpu.a[6].wrapping_add(0x32);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 2, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $08D778: subq.b #$2, $33(a6)
pub fn i_08d778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D77C;
    cpu.ir = 0x552E;
    let ad1 = cpu.a[6].wrapping_add(0x33);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 2, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $08D77C: cmpi.b #$34, $32(a6)
pub fn i_08d77c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D782;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x34, Size::Byte, false);
    16
}
/// $08D782: bcc.w $8d792
pub fn i_08d782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D786;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8D792; return 10; }
    8
}
/// $08D786: move.b #$34, $32(a6)
pub fn i_08d786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D78C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x34;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08D78C: move.b #$34, $33(a6)
pub fn i_08d78c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D792;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x34;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08D792: jsr $28d70.l
pub fn i_08d792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D798;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D798);
    cpu.pc = 0x28D70;
    16
}
/// $08D798: cmpi.w #$433f, $106f5c.l
pub fn i_08d798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D7A0;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F5C) as u32;
    sub_with_flags(cpu, v2, 0x433F, Size::Word, false);
    20
}
/// $08D7A0: bcs.w $8d7bc
pub fn i_08d7a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D7A4;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8D7BC; return 10; }
    8
}
/// $08D7A4: move.w #$0, $28(a6)
pub fn i_08d7a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D7AA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08D7AA: move.w #$ff80, $2a(a6)
pub fn i_08d7aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D7B0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08D7B0: move.w #$ffc0, $2e(a6)
pub fn i_08d7b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D7B6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFC0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_08d7b6  ($08D7B6, 8 B, kernel/task_handlers.c)
/// $08D7B6: lea.l $8d7be(pc), a1
pub fn i_08d7b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D7BA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8D7BE;
    16
}
/// $08D7BA: move.l a1, (a6)
pub fn i_08d7ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D7BC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08D7BC: rts
pub fn i_08d7bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D7BE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Capsule_FallToGround_08d7be  ($08D7BE, 28 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08D7BE: jsr $2783a.l
pub fn i_08d7be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D7C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D7C4);
    cpu.pc = 0x2783A;
    16
}
/// $08D7C4: jsr $27bc8.l
pub fn i_08d7c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D7CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D7CA);
    cpu.pc = 0x27BC8;
    16
}
/// $08D7CA: bcc.w $8d7da
pub fn i_08d7ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D7CE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8D7DA; return 10; }
    8
}
/// $08D7CE: move.w #$75, $70(a6)
pub fn i_08d7ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D7D4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x75;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08D7D4: lea.l $8d7e2(pc), a1
pub fn i_08d7d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D7D8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8D7E2;
    16
}
/// $08D7D8: move.l a1, (a6)
pub fn i_08d7d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D7DA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrAbsThunk_08d7da  ($08D7DA, 8 B, thunks/jsr_abs_thunks.c)
/// $08D7DA: jsr $28d70.l
pub fn i_08d7da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D7E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D7E0);
    cpu.pc = 0x28D70;
    16
}
/// $08D7E0: rts
pub fn i_08d7e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D7E2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Capsule_Vanish_08d7e2  ($08D7E2, 26 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08D7E2: subq.w #$1, $70(a6)
pub fn i_08d7e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D7E6;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08D7E6: cmpi.w #$0, $70(a6)
pub fn i_08d7e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D7EC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08D7EC: bgt.w $8d7f6
pub fn i_08d7ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D7F0;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8D7F6; return 10; }
    8
}
/// $08D7F0: jmp $518.l
pub fn i_08d7f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D7F6;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08D7F6: jsr $2783a.l
pub fn i_08d7f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D7FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D7FC);
    cpu.pc = 0x2783A;
    16
}
// ===== JsrAbsThunk_08d7fc  ($08D7FC, 8 B, thunks/jsr_abs_thunks.c)
/// $08D7FC: jsr $28d70.l
pub fn i_08d7fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D802;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D802);
    cpu.pc = 0x28D70;
    16
}
/// $08D802: rts
pub fn i_08d802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D804;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Capsule_CheckMissionEnd_08d804  ($08D804, 54 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08D804: cmpi.b #$b, $106ece.l
pub fn i_08d804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D80C;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0xB, Size::Byte, false);
    20
}
/// $08D80C: bne.w $8d822
pub fn i_08d80c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D810;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8D822; return 10; }
    8
}
/// $08D810: cmpi.w #$42c0, $106f5c.l
pub fn i_08d810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D818;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F5C) as u32;
    sub_with_flags(cpu, v2, 0x42C0, Size::Word, false);
    20
}
/// $08D818: bcs.w $8d822
pub fn i_08d818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D81C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8D822; return 10; }
    8
}
/// $08D81C: lea.l $8d670(pc), a1
pub fn i_08d81c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D820;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8D670;
    16
}
/// $08D820: move.l a1, (a6)
pub fn i_08d820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D822;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08D822: cmpi.b #$c, $106ece.l
pub fn i_08d822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D82A;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0xC, Size::Byte, false);
    20
}
/// $08D82A: bne.w $8d840
pub fn i_08d82a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D82E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8D840; return 10; }
    8
}
/// $08D82E: cmpi.w #$4280, $106f5c.l
pub fn i_08d82e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D836;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F5C) as u32;
    sub_with_flags(cpu, v2, 0x4280, Size::Word, false);
    20
}
/// $08D836: bcs.w $8d840
pub fn i_08d836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D83A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8D840; return 10; }
    8
}
// ===== SetTaskHandler_08d83a  ($08D83A, 8 B, kernel/task_handlers.c)
/// $08D83A: lea.l $8d774(pc), a1
pub fn i_08d83a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D83E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8D774;
    16
}
/// $08D83E: move.l a1, (a6)
pub fn i_08d83e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D840;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08D840: rts
pub fn i_08d840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D842;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionEnd_Marker_08d842  ($08D842, 80 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08D842: move.w #$f0, $22(a6)
pub fn i_08d842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D848;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08D848: move.w #$1a0, $24(a6)
pub fn i_08d848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D84E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1A0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08D84E: clr.w $26(a6)
pub fn i_08d84e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D852;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x26);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08D852: move.w #$98, d1
pub fn i_08d852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D856;
    cpu.ir = 0x323C;
    let v1: u32 = 0x98;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08D856: jsr $236e.l
pub fn i_08d856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D85C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D85C);
    cpu.pc = 0x236E;
    16
}
/// $08D85C: lea.l $2f3960.l, a0
pub fn i_08d85c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D862;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3960;
    20
}
/// $08D862: jsr $28cd4.l
pub fn i_08d862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D868;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D868);
    cpu.pc = 0x28CD4;
    16
}
/// $08D868: lea.l $8d86e(pc), a1
pub fn i_08d868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D86C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8D86E;
    16
}
/// $08D86C: move.l a1, (a6)
pub fn i_08d86c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D86E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08D86E: cmpi.w #$4300, $106f5c.l
pub fn i_08d86e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D876;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F5C) as u32;
    sub_with_flags(cpu, v2, 0x4300, Size::Word, false);
    20
}
/// $08D876: bcs.w $8d88c
pub fn i_08d876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D87A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8D88C; return 10; }
    8
}
/// $08D87A: lea.l $2f39dc.l, a0
pub fn i_08d87a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D880;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F39DC;
    20
}
/// $08D880: jsr $28cd4.l
pub fn i_08d880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D886;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D886);
    cpu.pc = 0x28CD4;
    16
}
/// $08D886: lea.l $8d89a(pc), a1
pub fn i_08d886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D88A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8D89A;
    16
}
/// $08D88A: move.l a1, (a6)
pub fn i_08d88a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D88C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08D88C: jsr $2783a.l
pub fn i_08d88c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D892;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D892);
    cpu.pc = 0x2783A;
    16
}
// ===== JsrAbsThunk_08d892  ($08D892, 8 B, thunks/jsr_abs_thunks.c)
/// $08D892: jsr $28d70.l
pub fn i_08d892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D898;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D898);
    cpu.pc = 0x28D70;
    16
}
/// $08D898: rts
pub fn i_08d898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D89A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionEnd_Marker2_08d89a  ($08D89A, 36 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08D89A: cmpi.w #$433f, $106f5c.l
pub fn i_08d89a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D8A2;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F5C) as u32;
    sub_with_flags(cpu, v2, 0x433F, Size::Word, false);
    20
}
/// $08D8A2: bcs.w $8d8b8
pub fn i_08d8a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D8A6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8D8B8; return 10; }
    8
}
/// $08D8A6: lea.l $2f39e8.l, a0
pub fn i_08d8a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D8AC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F39E8;
    20
}
/// $08D8AC: jsr $28cd4.l
pub fn i_08d8ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D8B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D8B2);
    cpu.pc = 0x28CD4;
    16
}
/// $08D8B2: lea.l $8d8c6(pc), a1
pub fn i_08d8b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D8B6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8D8C6;
    16
}
/// $08D8B6: move.l a1, (a6)
pub fn i_08d8b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D8B8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08D8B8: jsr $2783a.l
pub fn i_08d8b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D8BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D8BE);
    cpu.pc = 0x2783A;
    16
}
// ===== JsrAbsThunk_08d8be  ($08D8BE, 8 B, thunks/jsr_abs_thunks.c)
/// $08D8BE: jsr $28d70.l
pub fn i_08d8be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D8C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D8C4);
    cpu.pc = 0x28D70;
    16
}
/// $08D8C4: rts
pub fn i_08d8c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D8C6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionEnd_Marker3_08d8c6  ($08D8C6, 38 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08D8C6: jsr $2783a.l
pub fn i_08d8c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D8CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D8CC);
    cpu.pc = 0x2783A;
    16
}
/// $08D8CC: jsr $28d70.l
pub fn i_08d8cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D8D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D8D2);
    cpu.pc = 0x28D70;
    16
}
/// $08D8D2: bcc.w $8d8f2
pub fn i_08d8d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D8D6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8D8F2; return 10; }
    8
}
/// $08D8D6: move.w #$3c, $70(a6)
pub fn i_08d8d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D8DC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08D8DC: move.l #$ffff8000, $88(a6)
pub fn i_08d8dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D8E4;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFF8000;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08D8E4: move.l #$0, $8c(a6)
pub fn i_08d8e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D8EC;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x0;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
// ===== SetTaskHandler_08d8ec  ($08D8EC, 8 B, kernel/task_handlers.c)
/// $08D8EC: lea.l $8d8f4(pc), a1
pub fn i_08d8ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D8F0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8D8F4;
    16
}
/// $08D8F0: move.l a1, (a6)
pub fn i_08d8f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D8F2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08D8F2: rts
pub fn i_08d8f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D8F4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionEnd_ScrollUp_08d8f4  ($08D8F4, 60 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08D8F4: cmpi.w #$d8, $106f54.l
pub fn i_08d8f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D8FC;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F54) as u32;
    sub_with_flags(cpu, v2, 0xD8, Size::Word, false);
    20
}
/// $08D8FC: bhi.w $8d926
pub fn i_08d8fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D900;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x8D926; return 10; }
    8
}
/// $08D900: move.w #$b4, $70(a6)
pub fn i_08d900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D906;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xB4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08D906: move.l #$0, $106f64.l
pub fn i_08d906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D910;
    cpu.ir = 0x23FC;
    let v1: u32 = 0x0;
    bus.write32(0x106F64, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $08D910: move.l #$0, $88(a6)
pub fn i_08d910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D918;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x0;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08D918: move.l #$0, $8c(a6)
pub fn i_08d918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D920;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x0;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08D920: lea.l $8d938(pc), a1
pub fn i_08d920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D924;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8D938;
    16
}
/// $08D924: move.l a1, (a6)
pub fn i_08d924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D926;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08D926: jsr $8ccd8(pc)
pub fn i_08d926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D92A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8D92A);
    cpu.pc = 0x8CCD8;
    16
}
/// $08D92A: jsr $2783a.l
pub fn i_08d92a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D930;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D930);
    cpu.pc = 0x2783A;
    16
}
// ===== JsrAbsThunk_08d930  ($08D930, 8 B, thunks/jsr_abs_thunks.c)
/// $08D930: jsr $28d70.l
pub fn i_08d930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D936;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D936);
    cpu.pc = 0x28D70;
    16
}
/// $08D936: rts
pub fn i_08d936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D938;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionEnd_Wait_08d938  ($08D938, 70 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08D938: subq.w #$1, $70(a6)
pub fn i_08d938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D93C;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08D93C: cmpi.w #$0, $70(a6)
pub fn i_08d93c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D942;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08D942: bgt.w $8d978
pub fn i_08d942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D946;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8D978; return 10; }
    8
}
/// $08D946: lea.l $8ccea(pc), a1
pub fn i_08d946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D94A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8CCEA;
    16
}
/// $08D94A: jsr $4ae.l
pub fn i_08d94a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D950;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D950);
    cpu.pc = 0x4AE;
    16
}
/// $08D950: move.b #$a, $98(a0)
pub fn i_08d950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D956;
    cpu.ir = 0x117C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08D956: move.b #$8, $99(a0)
pub fn i_08d956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D95C;
    cpu.ir = 0x117C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08D95C: move.w #$a0, $22(a0)
pub fn i_08d95c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D962;
    cpu.ir = 0x317C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08D962: move.w #$100, $24(a0)
pub fn i_08d962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D968;
    cpu.ir = 0x317C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08D968: move.l #$0, $106f64.l
pub fn i_08d968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D972;
    cpu.ir = 0x23FC;
    let v1: u32 = 0x0;
    bus.write32(0x106F64, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $08D972: lea.l $8d986(pc), a1
pub fn i_08d972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D976;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8D986;
    16
}
/// $08D976: move.l a1, (a6)
pub fn i_08d976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D978;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08D978: jsr $2783a.l
pub fn i_08d978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D97E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D97E);
    cpu.pc = 0x2783A;
    16
}
// ===== JsrAbsThunk_08d97e  ($08D97E, 8 B, thunks/jsr_abs_thunks.c)
/// $08D97E: jsr $28d70.l
pub fn i_08d97e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D984;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D984);
    cpu.pc = 0x28D70;
    16
}
/// $08D984: rts
pub fn i_08d984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D986;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionEnd_Idle_08d986  ($08D986, 6 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08D986: jsr $2783a.l
pub fn i_08d986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D98C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D98C);
    cpu.pc = 0x2783A;
    16
}
// ===== JsrAbsThunk_08d98c  ($08D98C, 8 B, thunks/jsr_abs_thunks.c)
/// $08D98C: jsr $28d70.l
pub fn i_08d98c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D992;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D992);
    cpu.pc = 0x28D70;
    16
}
/// $08D992: rts
pub fn i_08d992(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D994;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_Tmpl157_08d994  ($08D994, 70 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08D994: jsr $8f108(pc)
pub fn i_08d994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D998;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8D998);
    cpu.pc = 0x8F108;
    16
}
/// $08D998: jsr $8f002(pc)
pub fn i_08d998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D99C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8D99C);
    cpu.pc = 0x8F002;
    16
}
/// $08D99C: jsr $8f02c(pc)
pub fn i_08d99c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D9A0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8D9A0);
    cpu.pc = 0x8F02C;
    16
}
/// $08D9A0: jsr $8f084(pc)
pub fn i_08d9a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D9A4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8D9A4);
    cpu.pc = 0x8F084;
    16
}
/// $08D9A4: lea.l $2f3ab4.l, a0
pub fn i_08d9a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D9AA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3AB4;
    20
}
/// $08D9AA: jsr $28cd4.l
pub fn i_08d9aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D9B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D9B0);
    cpu.pc = 0x28CD4;
    16
}
/// $08D9B0: lea.l $8d9b6(pc), a1
pub fn i_08d9b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D9B4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8D9B6;
    16
}
/// $08D9B4: move.l a1, (a6)
pub fn i_08d9b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D9B6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08D9B6: jsr $8f040(pc)
pub fn i_08d9b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D9BA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8D9BA);
    cpu.pc = 0x8F040;
    16
}
/// $08D9BA: bcc.w $8d9d0
pub fn i_08d9ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D9BE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8D9D0; return 10; }
    8
}
/// $08D9BE: lea.l $2f3ac0.l, a0
pub fn i_08d9be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D9C4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3AC0;
    20
}
/// $08D9C4: jsr $28cd4.l
pub fn i_08d9c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D9CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D9CA);
    cpu.pc = 0x28CD4;
    16
}
/// $08D9CA: lea.l $8d9d0(pc), a1
pub fn i_08d9ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D9CE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8D9D0;
    16
}
/// $08D9CE: move.l a1, (a6)
pub fn i_08d9ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D9D0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08D9D0: jsr $2783a.l
pub fn i_08d9d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D9D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D9D6);
    cpu.pc = 0x2783A;
    16
}
/// $08D9D6: jsr $8efb0(pc)
pub fn i_08d9d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D9DA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8D9DA);
    cpu.pc = 0x8EFB0;
    16
}
// ===== JsrAbsThunk_08d9da  ($08D9DA, 8 B, thunks/jsr_abs_thunks.c)
/// $08D9DA: jsr $28d70.l
pub fn i_08d9da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D9E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D9E0);
    cpu.pc = 0x28D70;
    16
}
/// $08D9E0: rts
pub fn i_08d9e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D9E2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_Tmpl158_08d9e2  ($08D9E2, 70 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08D9E2: jsr $8f108(pc)
pub fn i_08d9e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D9E6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8D9E6);
    cpu.pc = 0x8F108;
    16
}
/// $08D9E6: jsr $8f002(pc)
pub fn i_08d9e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D9EA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8D9EA);
    cpu.pc = 0x8F002;
    16
}
/// $08D9EA: jsr $8f02c(pc)
pub fn i_08d9ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D9EE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8D9EE);
    cpu.pc = 0x8F02C;
    16
}
/// $08D9EE: jsr $8f084(pc)
pub fn i_08d9ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D9F2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8D9F2);
    cpu.pc = 0x8F084;
    16
}
/// $08D9F2: lea.l $2f3af0.l, a0
pub fn i_08d9f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D9F8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3AF0;
    20
}
/// $08D9F8: jsr $28cd4.l
pub fn i_08d9f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D9FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8D9FE);
    cpu.pc = 0x28CD4;
    16
}
/// $08D9FE: lea.l $8da04(pc), a1
pub fn i_08d9fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA02;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DA04;
    16
}
/// $08DA02: move.l a1, (a6)
pub fn i_08da02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA04;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08DA04: jsr $8f040(pc)
pub fn i_08da04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA08;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DA08);
    cpu.pc = 0x8F040;
    16
}
/// $08DA08: bcc.w $8da1e
pub fn i_08da08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA0C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8DA1E; return 10; }
    8
}
/// $08DA0C: lea.l $2f3afc.l, a0
pub fn i_08da0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA12;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3AFC;
    20
}
/// $08DA12: jsr $28cd4.l
pub fn i_08da12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DA18);
    cpu.pc = 0x28CD4;
    16
}
/// $08DA18: lea.l $8da1e(pc), a1
pub fn i_08da18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA1C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DA1E;
    16
}
/// $08DA1C: move.l a1, (a6)
pub fn i_08da1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA1E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08DA1E: jsr $2783a.l
pub fn i_08da1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA24;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DA24);
    cpu.pc = 0x2783A;
    16
}
/// $08DA24: jsr $8efb0(pc)
pub fn i_08da24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA28;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DA28);
    cpu.pc = 0x8EFB0;
    16
}
// ===== JsrAbsThunk_08da28  ($08DA28, 8 B, thunks/jsr_abs_thunks.c)
/// $08DA28: jsr $28d70.l
pub fn i_08da28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA2E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DA2E);
    cpu.pc = 0x28D70;
    16
}
/// $08DA2E: rts
pub fn i_08da2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA30;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_Carrier_08da30  ($08DA30, 102 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08DA30: move.b #$0, $98(a6)
pub fn i_08da30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA36;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08DA36: move.b #$0, $99(a6)
pub fn i_08da36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA3C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08DA3C: move.w #$40, $22(a6)
pub fn i_08da3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA42;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08DA42: move.w #$150, $24(a6)
pub fn i_08da42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA48;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x150;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08DA48: clr.w $26(a6)
pub fn i_08da48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA4C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x26);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08DA4C: jsr $8f108(pc)
pub fn i_08da4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA50;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DA50);
    cpu.pc = 0x8F108;
    16
}
/// $08DA50: jsr $8f002(pc)
pub fn i_08da50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA54;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DA54);
    cpu.pc = 0x8F002;
    16
}
/// $08DA54: jsr $8f02c(pc)
pub fn i_08da54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA58;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DA58);
    cpu.pc = 0x8F02C;
    16
}
/// $08DA58: jsr $8f0d0(pc)
pub fn i_08da58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA5C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DA5C);
    cpu.pc = 0x8F0D0;
    16
}
/// $08DA5C: clr.w $5c(a6)
pub fn i_08da5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA60;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x5C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08DA60: lea.l $2f3b2c.l, a0
pub fn i_08da60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA66;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3B2C;
    20
}
/// $08DA66: jsr $28cd4.l
pub fn i_08da66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DA6C);
    cpu.pc = 0x28CD4;
    16
}
/// $08DA6C: lea.l $8da72(pc), a1
pub fn i_08da6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA70;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DA72;
    16
}
/// $08DA70: move.l a1, (a6)
pub fn i_08da70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA72;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08DA72: jsr $8f040(pc)
pub fn i_08da72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA76;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DA76);
    cpu.pc = 0x8F040;
    16
}
/// $08DA76: bcc.w $8da8c
pub fn i_08da76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA7A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8DA8C; return 10; }
    8
}
/// $08DA7A: lea.l $2f3b70.l, a0
pub fn i_08da7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA80;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3B70;
    20
}
/// $08DA80: jsr $28cd4.l
pub fn i_08da80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DA86);
    cpu.pc = 0x28CD4;
    16
}
/// $08DA86: lea.l $8da9e(pc), a1
pub fn i_08da86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA8A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DA9E;
    16
}
/// $08DA8A: move.l a1, (a6)
pub fn i_08da8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA8C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08DA8C: jsr $2783a.l
pub fn i_08da8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DA92);
    cpu.pc = 0x2783A;
    16
}
/// $08DA92: jsr $8efb0(pc)
pub fn i_08da92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA96;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DA96);
    cpu.pc = 0x8EFB0;
    16
}
// ===== JsrAbsThunk_08da96  ($08DA96, 8 B, thunks/jsr_abs_thunks.c)
/// $08DA96: jsr $28d70.l
pub fn i_08da96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DA9C);
    cpu.pc = 0x28D70;
    16
}
/// $08DA9C: rts
pub fn i_08da9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DA9E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_Carrier_Drop_08da9e  ($08DA9E, 60 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08DA9E: cmpi.w #$0, $5c(a6)
pub fn i_08da9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DAA4;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08DAA4: beq.w $8dad0
pub fn i_08daa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DAA8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8DAD0; return 10; }
    8
}
/// $08DAA8: lea.l $8dae2(pc), a1
pub fn i_08daa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DAAC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DAE2;
    16
}
/// $08DAAC: jsr $4ae.l
pub fn i_08daac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DAB2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DAB2);
    cpu.pc = 0x4AE;
    16
}
/// $08DAB2: jsr $5dd02.l
pub fn i_08dab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DAB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DAB8);
    cpu.pc = 0x5DD02;
    16
}
/// $08DAB8: move.b $3a(a6), $98(a0)
pub fn i_08dab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DABE;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad4, v3 as u8);
    20
}
/// $08DABE: move.b #$30, $99(a0)
pub fn i_08dabe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DAC4;
    cpu.ir = 0x117C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08DAC4: move.b $9a(a6), $9a(a0)
pub fn i_08dac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DACA;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $08DACA: lea.l $8dad0(pc), a1
pub fn i_08daca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DACE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DAD0;
    16
}
/// $08DACE: move.l a1, (a6)
pub fn i_08dace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DAD0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08DAD0: jsr $2783a.l
pub fn i_08dad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DAD6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DAD6);
    cpu.pc = 0x2783A;
    16
}
/// $08DAD6: jsr $8efb0(pc)
pub fn i_08dad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DADA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DADA);
    cpu.pc = 0x8EFB0;
    16
}
// ===== JsrAbsThunk_08dada  ($08DADA, 8 B, thunks/jsr_abs_thunks.c)
/// $08DADA: jsr $28d70.l
pub fn i_08dada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DAE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DAE0);
    cpu.pc = 0x28D70;
    16
}
/// $08DAE0: rts
pub fn i_08dae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DAE2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_Dropped_08dae2  ($08DAE2, 102 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08DAE2: move.w #$c000, $38(a6)
pub fn i_08dae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DAE8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08DAE8: andi.w #$ffe3, $38(a6)
pub fn i_08dae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DAEE;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $08DAEE: ori.w #$1c, $38(a6)
pub fn i_08daee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DAF4;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $08DAF4: jsr $8f002(pc)
pub fn i_08daf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DAF8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DAF8);
    cpu.pc = 0x8F002;
    16
}
/// $08DAF8: jsr $8f010(pc)
pub fn i_08daf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DAFC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DAFC);
    cpu.pc = 0x8F010;
    16
}
/// $08DAFC: neg.w $28(a6)
pub fn i_08dafc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB00;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08DB00: move.w #$10, $2e(a6)
pub fn i_08db00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB06;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08DB06: addi.w #$20, $24(a6)
pub fn i_08db06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB0C;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08DB0C: jsr $8f0d0(pc)
pub fn i_08db0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB10;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DB10);
    cpu.pc = 0x8F0D0;
    16
}
/// $08DB10: lea.l $2f3c04.l, a0
pub fn i_08db10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB16;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3C04;
    20
}
/// $08DB16: jsr $28cd4.l
pub fn i_08db16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DB1C);
    cpu.pc = 0x28CD4;
    16
}
/// $08DB1C: lea.l $8db22(pc), a1
pub fn i_08db1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB20;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DB22;
    16
}
/// $08DB20: move.l a1, (a6)
pub fn i_08db20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB22;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08DB22: jsr $2783a.l
pub fn i_08db22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DB28);
    cpu.pc = 0x2783A;
    16
}
/// $08DB28: jsr $8d34e(pc)
pub fn i_08db28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB2C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DB2C);
    cpu.pc = 0x8D34E;
    16
}
/// $08DB2C: movea.l #$ffffffff, a0
pub fn i_08db2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB32;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $08DB32: jsr $5dd56.l
pub fn i_08db32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DB38);
    cpu.pc = 0x5DD56;
    16
}
/// $08DB38: bcc.w $8db48
pub fn i_08db38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB3C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8DB48; return 10; }
    8
}
/// $08DB3C: jsr $5b6.l
pub fn i_08db3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB42;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DB42);
    cpu.pc = 0x5B6;
    16
}
/// $08DB42: jmp $518.l
pub fn i_08db42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB48;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== JsrAbsThunk_08db48  ($08DB48, 8 B, thunks/jsr_abs_thunks.c)
/// $08DB48: jsr $28d70.l
pub fn i_08db48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DB4E);
    cpu.pc = 0x28D70;
    16
}
/// $08DB4E: rts
pub fn i_08db4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB50;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_Tmpl162_08db50  ($08DB50, 42 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08DB50: move.w #$d000, $38(a6)
pub fn i_08db50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB56;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08DB56: jsr $8f002(pc)
pub fn i_08db56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB5A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DB5A);
    cpu.pc = 0x8F002;
    16
}
/// $08DB5A: jsr $8f010(pc)
pub fn i_08db5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB5E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DB5E);
    cpu.pc = 0x8F010;
    16
}
/// $08DB5E: move.w #$f8, d1
pub fn i_08db5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB62;
    cpu.ir = 0x323C;
    let v1: u32 = 0xF8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08DB62: jsr $236e.l
pub fn i_08db62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB68;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DB68);
    cpu.pc = 0x236E;
    16
}
/// $08DB68: lea.l $2f3c84.l, a0
pub fn i_08db68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB6E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3C84;
    20
}
/// $08DB6E: jsr $28cd4.l
pub fn i_08db6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB74;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DB74);
    cpu.pc = 0x28CD4;
    16
}
/// $08DB74: lea.l $8db7a(pc), a1
pub fn i_08db74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB78;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DB7A;
    16
}
/// $08DB78: move.l a1, (a6)
pub fn i_08db78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB7A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Grunt_HopDown_08db7a  ($08DB7A, 70 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08DB7A: move.w #$e, $2e(a6)
pub fn i_08db7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB80;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08DB80: move.w #$8, $70(a6)
pub fn i_08db80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB86;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08DB86: move.w #$7, d0
pub fn i_08db86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB8A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08DB8A: jsr $5ea1c.l
pub fn i_08db8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DB90);
    cpu.pc = 0x5EA1C;
    16
}
/// $08DB90: add.w d0, $70(a6)
pub fn i_08db90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB94;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08DB94: lea.l $2f3c84.l, a0
pub fn i_08db94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DB9A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3C84;
    20
}
/// $08DB9A: jsr $28cd4.l
pub fn i_08db9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DBA0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DBA0);
    cpu.pc = 0x28CD4;
    16
}
/// $08DBA0: lea.l $8dba6(pc), a1
pub fn i_08dba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DBA4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DBA6;
    16
}
/// $08DBA4: move.l a1, (a6)
pub fn i_08dba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DBA6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08DBA6: jsr $2783a.l
pub fn i_08dba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DBAC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DBAC);
    cpu.pc = 0x2783A;
    16
}
/// $08DBAC: jsr $8d34e(pc)
pub fn i_08dbac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DBB0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DBB0);
    cpu.pc = 0x8D34E;
    16
}
/// $08DBB0: jsr $28d70.l
pub fn i_08dbb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DBB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DBB6);
    cpu.pc = 0x28D70;
    16
}
/// $08DBB6: cmpi.w #$1d0, $24(a6)
pub fn i_08dbb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DBBC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1D0, Size::Word, false);
    16
}
/// $08DBBC: blt.w $8dbc8
pub fn i_08dbbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DBC0;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x8DBC8; return 10; }
    8
}
// ===== SetTaskHandler_08dbc0  ($08DBC0, 8 B, kernel/task_handlers.c)
/// $08DBC0: lea.l $8dbe2(pc), a1
pub fn i_08dbc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DBC4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DBE2;
    16
}
/// $08DBC4: move.l a1, (a6)
pub fn i_08dbc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DBC6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08DBC6: rts
pub fn i_08dbc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DBC8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_HopDown_Wait_08dbc8  ($08DBC8, 20 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08DBC8: subq.w #$1, $70(a6)
pub fn i_08dbc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DBCC;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08DBCC: cmpi.w #$0, $70(a6)
pub fn i_08dbcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DBD2;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08DBD2: bhi.w $8dbdc
pub fn i_08dbd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DBD6;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x8DBDC; return 10; }
    8
}
/// $08DBD6: lea.l $8dbe2(pc), a1
pub fn i_08dbd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DBDA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DBE2;
    16
}
/// $08DBDA: move.l a1, (a6)
pub fn i_08dbda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DBDC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrPcThunk_08dbdc  ($08DBDC, 6 B, thunks/jsr_pc_thunks.c)
/// $08DBDC: jsr $8efb0(pc)
pub fn i_08dbdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DBE0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DBE0);
    cpu.pc = 0x8EFB0;
    16
}
/// $08DBE0: rts
pub fn i_08dbe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DBE2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_HopUp_08dbe2  ($08DBE2, 70 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08DBE2: move.w #$fff2, $2e(a6)
pub fn i_08dbe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DBE8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFF2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08DBE8: move.w #$8, $70(a6)
pub fn i_08dbe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DBEE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08DBEE: move.w #$7, d0
pub fn i_08dbee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DBF2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08DBF2: jsr $5ea1c.l
pub fn i_08dbf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DBF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DBF8);
    cpu.pc = 0x5EA1C;
    16
}
/// $08DBF8: add.w d0, $70(a6)
pub fn i_08dbf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DBFC;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08DBFC: lea.l $2f3cb4.l, a0
pub fn i_08dbfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC02;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3CB4;
    20
}
/// $08DC02: jsr $28cd4.l
pub fn i_08dc02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DC08);
    cpu.pc = 0x28CD4;
    16
}
/// $08DC08: lea.l $8dc0e(pc), a1
pub fn i_08dc08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC0C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DC0E;
    16
}
/// $08DC0C: move.l a1, (a6)
pub fn i_08dc0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC0E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08DC0E: jsr $2783a.l
pub fn i_08dc0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DC14);
    cpu.pc = 0x2783A;
    16
}
/// $08DC14: jsr $8d34e(pc)
pub fn i_08dc14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC18;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DC18);
    cpu.pc = 0x8D34E;
    16
}
/// $08DC18: jsr $28d70.l
pub fn i_08dc18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DC1E);
    cpu.pc = 0x28D70;
    16
}
/// $08DC1E: cmpi.w #$160, $24(a6)
pub fn i_08dc1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC24;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x160, Size::Word, false);
    16
}
/// $08DC24: bge.w $8dc30
pub fn i_08dc24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC28;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x8DC30; return 10; }
    8
}
// ===== SetTaskHandler_08dc28  ($08DC28, 8 B, kernel/task_handlers.c)
/// $08DC28: lea.l $8db7a(pc), a1
pub fn i_08dc28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC2C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DB7A;
    16
}
/// $08DC2C: move.l a1, (a6)
pub fn i_08dc2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC2E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08DC2E: rts
pub fn i_08dc2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC30;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_HopUp_Wait_08dc30  ($08DC30, 20 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08DC30: subq.w #$1, $70(a6)
pub fn i_08dc30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC34;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08DC34: cmpi.w #$0, $70(a6)
pub fn i_08dc34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC3A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08DC3A: bhi.w $8dc44
pub fn i_08dc3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC3E;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x8DC44; return 10; }
    8
}
/// $08DC3E: lea.l $8db7a(pc), a1
pub fn i_08dc3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC42;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DB7A;
    16
}
/// $08DC42: move.l a1, (a6)
pub fn i_08dc42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC44;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrPcThunk_08dc44  ($08DC44, 6 B, thunks/jsr_pc_thunks.c)
/// $08DC44: jsr $8efb0(pc)
pub fn i_08dc44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC48;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DC48);
    cpu.pc = 0x8EFB0;
    16
}
/// $08DC48: rts
pub fn i_08dc48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC4A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_Rand_Idle_08dc4a  ($08DC4A, 68 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08DC4A: move.w #$40, $22(a6)
pub fn i_08dc4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC50;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08DC50: move.w #$150, $24(a6)
pub fn i_08dc50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC56;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x150;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08DC56: clr.w $26(a6)
pub fn i_08dc56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC5A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x26);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08DC5A: jsr $8f108(pc)
pub fn i_08dc5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC5E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DC5E);
    cpu.pc = 0x8F108;
    16
}
/// $08DC5E: jsr $8f002(pc)
pub fn i_08dc5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC62;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DC62);
    cpu.pc = 0x8F002;
    16
}
/// $08DC62: jsr $8f02c(pc)
pub fn i_08dc62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC66;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DC66);
    cpu.pc = 0x8F02C;
    16
}
/// $08DC66: move.w #$f8, d1
pub fn i_08dc66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC6A;
    cpu.ir = 0x323C;
    let v1: u32 = 0xF8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08DC6A: jsr $236e.l
pub fn i_08dc6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC70;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DC70);
    cpu.pc = 0x236E;
    16
}
/// $08DC70: lea.l $2f3c58.l, a0
pub fn i_08dc70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC76;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3C58;
    20
}
/// $08DC76: jsr $28cd4.l
pub fn i_08dc76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC7C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DC7C);
    cpu.pc = 0x28CD4;
    16
}
/// $08DC7C: jsr $27cee.l
pub fn i_08dc7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DC82);
    cpu.pc = 0x27CEE;
    16
}
/// $08DC82: jsr $28d70.l
pub fn i_08dc82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DC88);
    cpu.pc = 0x28D70;
    16
}
/// $08DC88: lea.l $8dc94(pc), a1
pub fn i_08dc88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC8C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DC94;
    16
}
/// $08DC8C: move.l a1, (a6)
pub fn i_08dc8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC8E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrPcThunk_08dc8e  ($08DC8E, 6 B, thunks/jsr_pc_thunks.c)
/// $08DC8E: jsr $8efb0(pc)
pub fn i_08dc8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC92;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DC92);
    cpu.pc = 0x8EFB0;
    16
}
/// $08DC92: rts
pub fn i_08dc92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC94;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_Rand_Stand_08dc94  ($08DC94, 96 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08DC94: lea.l $2f3c58.l, a0
pub fn i_08dc94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DC9A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3C58;
    20
}
/// $08DC9A: jsr $28cd4.l
pub fn i_08dc9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DCA0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DCA0);
    cpu.pc = 0x28CD4;
    16
}
/// $08DCA0: lea.l $8dca6(pc), a1
pub fn i_08dca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DCA4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DCA6;
    16
}
/// $08DCA4: move.l a1, (a6)
pub fn i_08dca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DCA6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08DCA6: jsr $2783a.l
pub fn i_08dca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DCAC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DCAC);
    cpu.pc = 0x2783A;
    16
}
/// $08DCAC: jsr $28d70.l
pub fn i_08dcac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DCB2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DCB2);
    cpu.pc = 0x28D70;
    16
}
/// $08DCB2: bcc.w $8dcf4
pub fn i_08dcb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DCB6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8DCF4; return 10; }
    8
}
/// $08DCB6: cmpi.b #$0, $99(a6)
pub fn i_08dcb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DCBC;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08DCBC: bne.w $8dcde
pub fn i_08dcbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DCC0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8DCDE; return 10; }
    8
}
/// $08DCC0: move.w #$60, d0
pub fn i_08dcc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DCC4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08DCC4: jsr $8efce(pc)
pub fn i_08dcc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DCC8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DCC8);
    cpu.pc = 0x8EFCE;
    16
}
/// $08DCC8: bcc.w $8dcd6
pub fn i_08dcc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DCCC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8DCD6; return 10; }
    8
}
/// $08DCCC: lea.l $8dd7e(pc), a1
pub fn i_08dccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DCD0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DD7E;
    16
}
/// $08DCD0: move.l a1, (a6)
pub fn i_08dcd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DCD2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08DCD2: bra.w $8dcda
pub fn i_08dcd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DCD6;
    cpu.ir = 0x6000;
    cpu.pc = 0x8DCDA;
    10
}
/// $08DCD6: jsr $8dec2(pc)
pub fn i_08dcd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DCDA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DCDA);
    cpu.pc = 0x8DEC2;
    16
}
/// $08DCDA: bra.w $8dcf4
pub fn i_08dcda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DCDE;
    cpu.ir = 0x6000;
    cpu.pc = 0x8DCF4;
    10
}
/// $08DCDE: jsr $8f040(pc)
pub fn i_08dcde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DCE2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DCE2);
    cpu.pc = 0x8F040;
    16
}
/// $08DCE2: bcc.w $8dcf0
pub fn i_08dce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DCE6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8DCF0; return 10; }
    8
}
/// $08DCE6: lea.l $8dd7e(pc), a1
pub fn i_08dce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DCEA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DD7E;
    16
}
/// $08DCEA: move.l a1, (a6)
pub fn i_08dcea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DCEC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08DCEC: bra.w $8dcf4
pub fn i_08dcec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DCF0;
    cpu.ir = 0x6000;
    cpu.pc = 0x8DCF4;
    10
}
/// $08DCF0: jsr $8dec2(pc)
pub fn i_08dcf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DCF4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DCF4);
    cpu.pc = 0x8DEC2;
    16
}
// ===== JsrPcThunk_08dcf4  ($08DCF4, 6 B, thunks/jsr_pc_thunks.c)
/// $08DCF4: jsr $8efb0(pc)
pub fn i_08dcf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DCF8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DCF8);
    cpu.pc = 0x8EFB0;
    16
}
/// $08DCF8: rts
pub fn i_08dcf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DCFA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_Rand_Stand2_08dcfa  ($08DCFA, 38 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08DCFA: lea.l $2f3c64.l, a0
pub fn i_08dcfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD00;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3C64;
    20
}
/// $08DD00: jsr $28cd4.l
pub fn i_08dd00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD06;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DD06);
    cpu.pc = 0x28CD4;
    16
}
/// $08DD06: lea.l $8dd0c(pc), a1
pub fn i_08dd06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD0A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DD0C;
    16
}
/// $08DD0A: move.l a1, (a6)
pub fn i_08dd0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD0C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08DD0C: jsr $2783a.l
pub fn i_08dd0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DD12);
    cpu.pc = 0x2783A;
    16
}
/// $08DD12: jsr $28d70.l
pub fn i_08dd12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DD18);
    cpu.pc = 0x28D70;
    16
}
/// $08DD18: bcc.w $8dd20
pub fn i_08dd18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD1C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8DD20; return 10; }
    8
}
/// $08DD1C: jsr $8dec2(pc)
pub fn i_08dd1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD20;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DD20);
    cpu.pc = 0x8DEC2;
    16
}
// ===== JsrPcThunk_08dd20  ($08DD20, 6 B, thunks/jsr_pc_thunks.c)
/// $08DD20: jsr $8efb0(pc)
pub fn i_08dd20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD24;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DD24);
    cpu.pc = 0x8EFB0;
    16
}
/// $08DD24: rts
pub fn i_08dd24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD26;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_Rand_Walk_08dd26  ($08DD26, 82 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08DD26: move.w #$1, d0
pub fn i_08dd26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD2A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08DD2A: jsr $5ea1c.l
pub fn i_08dd2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DD30);
    cpu.pc = 0x5EA1C;
    16
}
/// $08DD30: move.b d0, $98(a6)
pub fn i_08dd30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD34;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08DD34: bne.w $8dd42
pub fn i_08dd34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD38;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8DD42; return 10; }
    8
}
/// $08DD38: move.w #$ff80, $28(a6)
pub fn i_08dd38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD3E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08DD3E: bra.w $8dd48
pub fn i_08dd3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD42;
    cpu.ir = 0x6000;
    cpu.pc = 0x8DD48;
    10
}
/// $08DD42: move.w #$80, $28(a6)
pub fn i_08dd42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD48;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08DD48: jsr $8f002(pc)
pub fn i_08dd48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD4C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DD4C);
    cpu.pc = 0x8F002;
    16
}
/// $08DD4C: lea.l $2f3c10.l, a0
pub fn i_08dd4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD52;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3C10;
    20
}
/// $08DD52: jsr $28cd4.l
pub fn i_08dd52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD58;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DD58);
    cpu.pc = 0x28CD4;
    16
}
/// $08DD58: lea.l $8dd5e(pc), a1
pub fn i_08dd58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD5C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DD5E;
    16
}
/// $08DD5C: move.l a1, (a6)
pub fn i_08dd5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD5E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08DD5E: jsr $2783a.l
pub fn i_08dd5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD64;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DD64);
    cpu.pc = 0x2783A;
    16
}
/// $08DD64: jsr $8d2f8(pc)
pub fn i_08dd64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD68;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DD68);
    cpu.pc = 0x8D2F8;
    16
}
/// $08DD68: jsr $28d70.l
pub fn i_08dd68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD6E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DD6E);
    cpu.pc = 0x28D70;
    16
}
/// $08DD6E: bcc.w $8dd78
pub fn i_08dd6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD72;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8DD78; return 10; }
    8
}
/// $08DD72: lea.l $8dc94(pc), a1
pub fn i_08dd72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD76;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DC94;
    16
}
/// $08DD76: move.l a1, (a6)
pub fn i_08dd76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD78;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrPcThunk_08dd78  ($08DD78, 6 B, thunks/jsr_pc_thunks.c)
/// $08DD78: jsr $8efb0(pc)
pub fn i_08dd78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD7C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DD7C);
    cpu.pc = 0x8EFB0;
    16
}
/// $08DD7C: rts
pub fn i_08dd7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD7E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_Hit_08dd7e  ($08DD7E, 44 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08DD7E: addi.w #$4, $24(a6)
pub fn i_08dd7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD84;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x4, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08DD84: move.w #$1e, $70(a6)
pub fn i_08dd84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD8A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08DD8A: move.w #$f, d0
pub fn i_08dd8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD8E;
    cpu.ir = 0x303C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08DD8E: jsr $5ea1c.l
pub fn i_08dd8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD94;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DD94);
    cpu.pc = 0x5EA1C;
    16
}
/// $08DD94: add.w d0, $70(a6)
pub fn i_08dd94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD98;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08DD98: jsr $2783a.l
pub fn i_08dd98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DD9E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DD9E);
    cpu.pc = 0x2783A;
    16
}
/// $08DD9E: jsr $28d70.l
pub fn i_08dd9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DDA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DDA4);
    cpu.pc = 0x28D70;
    16
}
/// $08DDA4: lea.l $8ddb0(pc), a1
pub fn i_08dda4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DDA8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DDB0;
    16
}
/// $08DDA8: move.l a1, (a6)
pub fn i_08dda8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DDAA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrPcThunk_08ddaa  ($08DDAA, 6 B, thunks/jsr_pc_thunks.c)
/// $08DDAA: jsr $8efb0(pc)
pub fn i_08ddaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DDAE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DDAE);
    cpu.pc = 0x8EFB0;
    16
}
/// $08DDAE: rts
pub fn i_08ddae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DDB0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_Hit_Angle_08ddb0  ($08DDB0, 36 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08DDB0: move.w #$1f, d0
pub fn i_08ddb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DDB4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08DDB4: jsr $5ea1c.l
pub fn i_08ddb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DDBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DDBA);
    cpu.pc = 0x5EA1C;
    16
}
/// $08DDBA: addi.w #$10, d0
pub fn i_08ddba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DDBE;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08DDBE: move.w d0, $34(a6)
pub fn i_08ddbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DDC2;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08DDC2: jsr $2783a.l
pub fn i_08ddc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DDC8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DDC8);
    cpu.pc = 0x2783A;
    16
}
/// $08DDC8: jsr $28d70.l
pub fn i_08ddc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DDCE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DDCE);
    cpu.pc = 0x28D70;
    16
}
/// $08DDCE: lea.l $8ddda(pc), a1
pub fn i_08ddce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DDD2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DDDA;
    16
}
/// $08DDD2: move.l a1, (a6)
pub fn i_08ddd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DDD4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrPcThunk_08ddd4  ($08DDD4, 6 B, thunks/jsr_pc_thunks.c)
/// $08DDD4: jsr $8efb0(pc)
pub fn i_08ddd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DDD8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DDD8);
    cpu.pc = 0x8EFB0;
    16
}
/// $08DDD8: rts
pub fn i_08ddd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DDDA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_Hit_Launch_08ddda  ($08DDDA, 74 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08DDDA: move.w #$ff, d0
pub fn i_08ddda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DDDE;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08DDDE: jsr $5ea1c.l
pub fn i_08ddde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DDE4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DDE4);
    cpu.pc = 0x5EA1C;
    16
}
/// $08DDE4: move.w #$200, d1
pub fn i_08dde4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DDE8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x200;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08DDE8: add.w d0, d1
pub fn i_08dde8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DDEA;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $08DDEA: move.w d1, $36(a6)
pub fn i_08ddea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DDEE;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08DDEE: move.w $34(a6), d0
pub fn i_08ddee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DDF2;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08DDF2: jsr $13c0e.l
pub fn i_08ddf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DDF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DDF8);
    cpu.pc = 0x13C0E;
    16
}
/// $08DDF8: move.w d1, $28(a6)
pub fn i_08ddf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DDFC;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08DDFC: move.w d2, $2a(a6)
pub fn i_08ddfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE00;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08DE00: move.b #$1, $3a(a6)
pub fn i_08de00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE06;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08DE06: lea.l $2f3c84.l, a0
pub fn i_08de06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE0C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3C84;
    20
}
/// $08DE0C: jsr $28cd4.l
pub fn i_08de0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DE12);
    cpu.pc = 0x28CD4;
    16
}
/// $08DE12: jsr $2783a.l
pub fn i_08de12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DE18);
    cpu.pc = 0x2783A;
    16
}
/// $08DE18: jsr $28d70.l
pub fn i_08de18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DE1E);
    cpu.pc = 0x28D70;
    16
}
/// $08DE1E: lea.l $8de2a(pc), a1
pub fn i_08de1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE22;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DE2A;
    16
}
/// $08DE22: move.l a1, (a6)
pub fn i_08de22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE24;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrPcThunk_08de24  ($08DE24, 6 B, thunks/jsr_pc_thunks.c)
/// $08DE24: jsr $8efb0(pc)
pub fn i_08de24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE28;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DE28);
    cpu.pc = 0x8EFB0;
    16
}
/// $08DE28: rts
pub fn i_08de28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE2A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_Hit_Fly_08de2a  ($08DE2A, 52 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08DE2A: jsr $2783a.l
pub fn i_08de2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DE30);
    cpu.pc = 0x2783A;
    16
}
/// $08DE30: jsr $8d2f8(pc)
pub fn i_08de30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE34;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DE34);
    cpu.pc = 0x8D2F8;
    16
}
/// $08DE34: jsr $28d70.l
pub fn i_08de34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DE3A);
    cpu.pc = 0x28D70;
    16
}
/// $08DE3A: subq.w #$1, $70(a6)
pub fn i_08de3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE3E;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08DE3E: cmpi.w #$0, $70(a6)
pub fn i_08de3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE44;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08DE44: bhi.w $8de5e
pub fn i_08de44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE48;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x8DE5E; return 10; }
    8
}
/// $08DE48: move.w #$d000, $38(a6)
pub fn i_08de48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE4E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08DE4E: move.w $36(a6), $28(a6)
pub fn i_08de4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE54;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08DE54: clr.w $2a(a6)
pub fn i_08de54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE58;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08DE58: lea.l $8db7a(pc), a1
pub fn i_08de58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE5C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DB7A;
    16
}
/// $08DE5C: move.l a1, (a6)
pub fn i_08de5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE5E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrPcThunk_08de5e  ($08DE5E, 6 B, thunks/jsr_pc_thunks.c)
/// $08DE5E: jsr $8efb0(pc)
pub fn i_08de5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE62;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DE62);
    cpu.pc = 0x8EFB0;
    16
}
/// $08DE62: rts
pub fn i_08de62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE64;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Grunt_Tmpl_Standing_08de64  ($08DE64, 88 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08DE64: move.b #$8, $99(a6)
pub fn i_08de64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE6A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08DE6A: move.b #$1, $98(a6)
pub fn i_08de6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE70;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08DE70: jsr $8f108(pc)
pub fn i_08de70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE74;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DE74);
    cpu.pc = 0x8F108;
    16
}
/// $08DE74: jsr $8f002(pc)
pub fn i_08de74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE78;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DE78);
    cpu.pc = 0x8F002;
    16
}
/// $08DE78: jsr $8f010(pc)
pub fn i_08de78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE7C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DE7C);
    cpu.pc = 0x8F010;
    16
}
/// $08DE7C: move.w #$f8, d1
pub fn i_08de7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE80;
    cpu.ir = 0x323C;
    let v1: u32 = 0xF8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08DE80: jsr $236e.l
pub fn i_08de80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DE86);
    cpu.pc = 0x236E;
    16
}
/// $08DE86: lea.l $8de8c(pc), a1
pub fn i_08de86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE8A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DE8C;
    16
}
/// $08DE8A: move.l a1, (a6)
pub fn i_08de8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE8C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08DE8C: lea.l $2f3c58.l, a0
pub fn i_08de8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE92;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3C58;
    20
}
/// $08DE92: jsr $28cd4.l
pub fn i_08de92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DE98);
    cpu.pc = 0x28CD4;
    16
}
/// $08DE98: lea.l $8de9e(pc), a1
pub fn i_08de98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE9C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DE9E;
    16
}
/// $08DE9C: move.l a1, (a6)
pub fn i_08de9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DE9E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08DE9E: jsr $2783a.l
pub fn i_08de9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DEA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DEA4);
    cpu.pc = 0x2783A;
    16
}
/// $08DEA4: jsr $28d70.l
pub fn i_08dea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DEAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8DEAA);
    cpu.pc = 0x28D70;
    16
}
/// $08DEAA: move.w #$60, d0
pub fn i_08deaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DEAE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08DEAE: jsr $8efce(pc)
pub fn i_08deae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DEB2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DEB2);
    cpu.pc = 0x8EFCE;
    16
}
/// $08DEB2: bcc.w $8debc
pub fn i_08deb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DEB6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8DEBC; return 10; }
    8
}
/// $08DEB6: lea.l $8dd7e(pc), a1
pub fn i_08deb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DEBA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8DD7E;
    16
}
/// $08DEBA: move.l a1, (a6)
pub fn i_08deba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DEBC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrPcThunk_08debc  ($08DEBC, 6 B, thunks/jsr_pc_thunks.c)
/// $08DEBC: jsr $8efb0(pc)
pub fn i_08debc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DEC0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8DEC0);
    cpu.pc = 0x8EFB0;
    16
}
/// $08DEC0: rts
pub fn i_08dec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8DEC2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x08C3DA, i_08c3da),
    (0x08C3E0, i_08c3e0),
    (0x08C3E6, i_08c3e6),
    (0x08C3EA, i_08c3ea),
    (0x08C3F0, i_08c3f0),
    (0x08C3F6, i_08c3f6),
    (0x08C3FC, i_08c3fc),
    (0x08C402, i_08c402),
    (0x08C408, i_08c408),
    (0x08C40C, i_08c40c),
    (0x08C410, i_08c410),
    (0x08C416, i_08c416),
    (0x08C41C, i_08c41c),
    (0x08C422, i_08c422),
    (0x08C428, i_08c428),
    (0x08C42C, i_08c42c),
    (0x08C42E, i_08c42e),
    (0x08C434, i_08c434),
    (0x08C436, i_08c436),
    (0x08C43A, i_08c43a),
    (0x08C43E, i_08c43e),
    (0x08C442, i_08c442),
    (0x08C446, i_08c446),
    (0x08C44C, i_08c44c),
    (0x08C452, i_08c452),
    (0x08C458, i_08c458),
    (0x08C45C, i_08c45c),
    (0x08C462, i_08c462),
    (0x08C468, i_08c468),
    (0x08C46E, i_08c46e),
    (0x08C474, i_08c474),
    (0x08C47A, i_08c47a),
    (0x08C47E, i_08c47e),
    (0x08C482, i_08c482),
    (0x08C488, i_08c488),
    (0x08C48E, i_08c48e),
    (0x08C494, i_08c494),
    (0x08C49A, i_08c49a),
    (0x08C49E, i_08c49e),
    (0x08C4A0, i_08c4a0),
    (0x08C4A4, i_08c4a4),
    (0x08C4A8, i_08c4a8),
    (0x08C4AC, i_08c4ac),
    (0x08C4B0, i_08c4b0),
    (0x08C4B6, i_08c4b6),
    (0x08C4BC, i_08c4bc),
    (0x08C4C2, i_08c4c2),
    (0x08C4C8, i_08c4c8),
    (0x08C4CA, i_08c4ca),
    (0x08C4CE, i_08c4ce),
    (0x08C4D2, i_08c4d2),
    (0x08C4D6, i_08c4d6),
    (0x08C4DC, i_08c4dc),
    (0x08C4E0, i_08c4e0),
    (0x08C4E6, i_08c4e6),
    (0x08C4EC, i_08c4ec),
    (0x08C4F2, i_08c4f2),
    (0x08C4F8, i_08c4f8),
    (0x08C4FA, i_08c4fa),
    (0x08C500, i_08c500),
    (0x08C506, i_08c506),
    (0x08C50A, i_08c50a),
    (0x08C510, i_08c510),
    (0x08C516, i_08c516),
    (0x08C51C, i_08c51c),
    (0x08C520, i_08c520),
    (0x08C522, i_08c522),
    (0x08C526, i_08c526),
    (0x08C52C, i_08c52c),
    (0x08C532, i_08c532),
    (0x08C536, i_08c536),
    (0x08C53C, i_08c53c),
    (0x08C542, i_08c542),
    (0x08C548, i_08c548),
    (0x08C54C, i_08c54c),
    (0x08C54E, i_08c54e),
    (0x08C552, i_08c552),
    (0x08C558, i_08c558),
    (0x08C55E, i_08c55e),
    (0x08C562, i_08c562),
    (0x08C568, i_08c568),
    (0x08C56E, i_08c56e),
    (0x08C574, i_08c574),
    (0x08C578, i_08c578),
    (0x08C57A, i_08c57a),
    (0x08C57E, i_08c57e),
    (0x08C584, i_08c584),
    (0x08C58A, i_08c58a),
    (0x08C58E, i_08c58e),
    (0x08C594, i_08c594),
    (0x08C59A, i_08c59a),
    (0x08C5A0, i_08c5a0),
    (0x08C5A4, i_08c5a4),
    (0x08C5A6, i_08c5a6),
    (0x08C5AA, i_08c5aa),
    (0x08C5B0, i_08c5b0),
    (0x08C5B2, i_08c5b2),
    (0x08C5B8, i_08c5b8),
    (0x08C5C0, i_08c5c0),
    (0x08C5C4, i_08c5c4),
    (0x08C5CA, i_08c5ca),
    (0x08C5CE, i_08c5ce),
    (0x08C5D6, i_08c5d6),
    (0x08C5DA, i_08c5da),
    (0x08C5E0, i_08c5e0),
    (0x08C5E4, i_08c5e4),
    (0x08C5E8, i_08c5e8),
    (0x08C5EE, i_08c5ee),
    (0x08C5F2, i_08c5f2),
    (0x08C5F6, i_08c5f6),
    (0x08C5FC, i_08c5fc),
    (0x08C600, i_08c600),
    (0x08C604, i_08c604),
    (0x08C60A, i_08c60a),
    (0x08C60E, i_08c60e),
    (0x08C614, i_08c614),
    (0x08C618, i_08c618),
    (0x08C61E, i_08c61e),
    (0x08C624, i_08c624),
    (0x08C62A, i_08c62a),
    (0x08C630, i_08c630),
    (0x08C636, i_08c636),
    (0x08C63A, i_08c63a),
    (0x08C640, i_08c640),
    (0x08C644, i_08c644),
    (0x08C64C, i_08c64c),
    (0x08C652, i_08c652),
    (0x08C658, i_08c658),
    (0x08C660, i_08c660),
    (0x08C666, i_08c666),
    (0x08C66A, i_08c66a),
    (0x08C670, i_08c670),
    (0x08C674, i_08c674),
    (0x08C676, i_08c676),
    (0x08C678, i_08c678),
    (0x08C67C, i_08c67c),
    (0x08C682, i_08c682),
    (0x08C686, i_08c686),
    (0x08C688, i_08c688),
    (0x08C68C, i_08c68c),
    (0x08C690, i_08c690),
    (0x08C694, i_08c694),
    (0x08C698, i_08c698),
    (0x08C69E, i_08c69e),
    (0x08C6A2, i_08c6a2),
    (0x08C6A8, i_08c6a8),
    (0x08C6AC, i_08c6ac),
    (0x08C6B2, i_08c6b2),
    (0x08C6B6, i_08c6b6),
    (0x08C6BC, i_08c6bc),
    (0x08C6C2, i_08c6c2),
    (0x08C6C6, i_08c6c6),
    (0x08C6CC, i_08c6cc),
    (0x08C6D2, i_08c6d2),
    (0x08C6D6, i_08c6d6),
    (0x08C6DC, i_08c6dc),
    (0x08C6DE, i_08c6de),
    (0x08C6E0, i_08c6e0),
    (0x08C6E4, i_08c6e4),
    (0x08C6E8, i_08c6e8),
    (0x08C6F0, i_08c6f0),
    (0x08C6F6, i_08c6f6),
    (0x08C6FC, i_08c6fc),
    (0x08C700, i_08c700),
    (0x08C704, i_08c704),
    (0x08C708, i_08c708),
    (0x08C70C, i_08c70c),
    (0x08C712, i_08c712),
    (0x08C718, i_08c718),
    (0x08C71E, i_08c71e),
    (0x08C722, i_08c722),
    (0x08C728, i_08c728),
    (0x08C72E, i_08c72e),
    (0x08C730, i_08c730),
    (0x08C736, i_08c736),
    (0x08C73C, i_08c73c),
    (0x08C740, i_08c740),
    (0x08C746, i_08c746),
    (0x08C74C, i_08c74c),
    (0x08C752, i_08c752),
    (0x08C758, i_08c758),
    (0x08C75E, i_08c75e),
    (0x08C764, i_08c764),
    (0x08C76A, i_08c76a),
    (0x08C76E, i_08c76e),
    (0x08C774, i_08c774),
    (0x08C778, i_08c778),
    (0x08C77A, i_08c77a),
    (0x08C780, i_08c780),
    (0x08C786, i_08c786),
    (0x08C78C, i_08c78c),
    (0x08C792, i_08c792),
    (0x08C796, i_08c796),
    (0x08C79C, i_08c79c),
    (0x08C7A2, i_08c7a2),
    (0x08C7A8, i_08c7a8),
    (0x08C7AC, i_08c7ac),
    (0x08C7B2, i_08c7b2),
    (0x08C7B8, i_08c7b8),
    (0x08C7BE, i_08c7be),
    (0x08C7C4, i_08c7c4),
    (0x08C7C6, i_08c7c6),
    (0x08C7CA, i_08c7ca),
    (0x08C7CE, i_08c7ce),
    (0x08C7D2, i_08c7d2),
    (0x08C7D6, i_08c7d6),
    (0x08C7DA, i_08c7da),
    (0x08C7DC, i_08c7dc),
    (0x08C7E0, i_08c7e0),
    (0x08C7E2, i_08c7e2),
    (0x08C7EA, i_08c7ea),
    (0x08C7F0, i_08c7f0),
    (0x08C7F2, i_08c7f2),
    (0x08C7F8, i_08c7f8),
    (0x08C7FE, i_08c7fe),
    (0x08C800, i_08c800),
    (0x08C808, i_08c808),
    (0x08C80E, i_08c80e),
    (0x08C810, i_08c810),
    (0x08C818, i_08c818),
    (0x08C81E, i_08c81e),
    (0x08C820, i_08c820),
    (0x08C828, i_08c828),
    (0x08C82C, i_08c82c),
    (0x08C832, i_08c832),
    (0x08C838, i_08c838),
    (0x08C840, i_08c840),
    (0x08C844, i_08c844),
    (0x08C84A, i_08c84a),
    (0x08C850, i_08c850),
    (0x08C856, i_08c856),
    (0x08C85C, i_08c85c),
    (0x08C862, i_08c862),
    (0x08C864, i_08c864),
    (0x08C86A, i_08c86a),
    (0x08C870, i_08c870),
    (0x08C876, i_08c876),
    (0x08C87A, i_08c87a),
    (0x08C87E, i_08c87e),
    (0x08C880, i_08c880),
    (0x08C886, i_08c886),
    (0x08C88A, i_08c88a),
    (0x08C88E, i_08c88e),
    (0x08C892, i_08c892),
    (0x08C898, i_08c898),
    (0x08C89C, i_08c89c),
    (0x08C8A0, i_08c8a0),
    (0x08C8A4, i_08c8a4),
    (0x08C8A8, i_08c8a8),
    (0x08C8AC, i_08c8ac),
    (0x08C8B2, i_08c8b2),
    (0x08C8B8, i_08c8b8),
    (0x08C8BC, i_08c8bc),
    (0x08C8C2, i_08c8c2),
    (0x08C8C6, i_08c8c6),
    (0x08C8CC, i_08c8cc),
    (0x08C8D0, i_08c8d0),
    (0x08C8D6, i_08c8d6),
    (0x08C8DA, i_08c8da),
    (0x08C8E0, i_08c8e0),
    (0x08C8E8, i_08c8e8),
    (0x08C8EE, i_08c8ee),
    (0x08C8F2, i_08c8f2),
    (0x08C8F4, i_08c8f4),
    (0x08C8F8, i_08c8f8),
    (0x08C8FA, i_08c8fa),
    (0x08C8FE, i_08c8fe),
    (0x08C902, i_08c902),
    (0x08C908, i_08c908),
    (0x08C90C, i_08c90c),
    (0x08C90E, i_08c90e),
    (0x08C914, i_08c914),
    (0x08C918, i_08c918),
    (0x08C91E, i_08c91e),
    (0x08C922, i_08c922),
    (0x08C928, i_08c928),
    (0x08C92C, i_08c92c),
    (0x08C92E, i_08c92e),
    (0x08C932, i_08c932),
    (0x08C936, i_08c936),
    (0x08C938, i_08c938),
    (0x08C93A, i_08c93a),
    (0x08C93E, i_08c93e),
    (0x08C942, i_08c942),
    (0x08C946, i_08c946),
    (0x08C94A, i_08c94a),
    (0x08C94E, i_08c94e),
    (0x08C950, i_08c950),
    (0x08C954, i_08c954),
    (0x08C956, i_08c956),
    (0x08C95C, i_08c95c),
    (0x08C960, i_08c960),
    (0x08C966, i_08c966),
    (0x08C96A, i_08c96a),
    (0x08C970, i_08c970),
    (0x08C974, i_08c974),
    (0x08C976, i_08c976),
    (0x08C97A, i_08c97a),
    (0x08C980, i_08c980),
    (0x08C984, i_08c984),
    (0x08C98A, i_08c98a),
    (0x08C98E, i_08c98e),
    (0x08C994, i_08c994),
    (0x08C998, i_08c998),
    (0x08C99E, i_08c99e),
    (0x08C9A4, i_08c9a4),
    (0x08C9A6, i_08c9a6),
    (0x08C9AA, i_08c9aa),
    (0x08C9AE, i_08c9ae),
    (0x08C9B2, i_08c9b2),
    (0x08C9B8, i_08c9b8),
    (0x08C9BC, i_08c9bc),
    (0x08C9BE, i_08c9be),
    (0x08C9C0, i_08c9c0),
    (0x08C9C4, i_08c9c4),
    (0x08C9C8, i_08c9c8),
    (0x08C9CC, i_08c9cc),
    (0x08C9CE, i_08c9ce),
    (0x08C9D2, i_08c9d2),
    (0x08C9D6, i_08c9d6),
    (0x08C9DA, i_08c9da),
    (0x08C9DC, i_08c9dc),
    (0x08C9E0, i_08c9e0),
    (0x08C9E4, i_08c9e4),
    (0x08C9E8, i_08c9e8),
    (0x08C9EC, i_08c9ec),
    (0x08C9EE, i_08c9ee),
    (0x08C9F2, i_08c9f2),
    (0x08C9F6, i_08c9f6),
    (0x08C9F8, i_08c9f8),
    (0x08C9FA, i_08c9fa),
    (0x08C9FE, i_08c9fe),
    (0x08CA02, i_08ca02),
    (0x08CA06, i_08ca06),
    (0x08CA0A, i_08ca0a),
    (0x08CA0E, i_08ca0e),
    (0x08CA12, i_08ca12),
    (0x08CA16, i_08ca16),
    (0x08CA1A, i_08ca1a),
    (0x08CA1C, i_08ca1c),
    (0x08CA24, i_08ca24),
    (0x08CA28, i_08ca28),
    (0x08CA2A, i_08ca2a),
    (0x08CA32, i_08ca32),
    (0x08CA36, i_08ca36),
    (0x08CA3A, i_08ca3a),
    (0x08CA42, i_08ca42),
    (0x08CA46, i_08ca46),
    (0x08CA48, i_08ca48),
    (0x08CA50, i_08ca50),
    (0x08CA54, i_08ca54),
    (0x08CA58, i_08ca58),
    (0x08CA5C, i_08ca5c),
    (0x08CA5E, i_08ca5e),
    (0x08CA62, i_08ca62),
    (0x08CA66, i_08ca66),
    (0x08CA6A, i_08ca6a),
    (0x08CA6E, i_08ca6e),
    (0x08CA74, i_08ca74),
    (0x08CA78, i_08ca78),
    (0x08CA7C, i_08ca7c),
    (0x08CA82, i_08ca82),
    (0x08CA86, i_08ca86),
    (0x08CA88, i_08ca88),
    (0x08CA8A, i_08ca8a),
    (0x08CA8E, i_08ca8e),
    (0x08CA92, i_08ca92),
    (0x08CA94, i_08ca94),
    (0x08CA98, i_08ca98),
    (0x08CA9C, i_08ca9c),
    (0x08CA9E, i_08ca9e),
    (0x08CAA2, i_08caa2),
    (0x08CAA6, i_08caa6),
    (0x08CAA8, i_08caa8),
    (0x08CAAC, i_08caac),
    (0x08CAB0, i_08cab0),
    (0x08CAB2, i_08cab2),
    (0x08CAB4, i_08cab4),
    (0x08CAB8, i_08cab8),
    (0x08CABC, i_08cabc),
    (0x08CAC0, i_08cac0),
    (0x08CAC4, i_08cac4),
    (0x08CAC8, i_08cac8),
    (0x08CACC, i_08cacc),
    (0x08CAD0, i_08cad0),
    (0x08CAD4, i_08cad4),
    (0x08CAD6, i_08cad6),
    (0x08CADE, i_08cade),
    (0x08CAE2, i_08cae2),
    (0x08CAE4, i_08cae4),
    (0x08CAEC, i_08caec),
    (0x08CAF0, i_08caf0),
    (0x08CAF4, i_08caf4),
    (0x08CAFC, i_08cafc),
    (0x08CB00, i_08cb00),
    (0x08CB02, i_08cb02),
    (0x08CB0A, i_08cb0a),
    (0x08CB0E, i_08cb0e),
    (0x08CB12, i_08cb12),
    (0x08CB16, i_08cb16),
    (0x08CB18, i_08cb18),
    (0x08CB20, i_08cb20),
    (0x08CB24, i_08cb24),
    (0x08CB26, i_08cb26),
    (0x08CB2A, i_08cb2a),
    (0x08CB2C, i_08cb2c),
    (0x08CB30, i_08cb30),
    (0x08CB34, i_08cb34),
    (0x08CB36, i_08cb36),
    (0x08CB3A, i_08cb3a),
    (0x08CB3E, i_08cb3e),
    (0x08CB40, i_08cb40),
    (0x08CB44, i_08cb44),
    (0x08CB48, i_08cb48),
    (0x08CB4C, i_08cb4c),
    (0x08CB50, i_08cb50),
    (0x08CB58, i_08cb58),
    (0x08CB5A, i_08cb5a),
    (0x08CB5E, i_08cb5e),
    (0x08CB62, i_08cb62),
    (0x08CB66, i_08cb66),
    (0x08CB6E, i_08cb6e),
    (0x08CB70, i_08cb70),
    (0x08CB74, i_08cb74),
    (0x08CB76, i_08cb76),
    (0x08CB7A, i_08cb7a),
    (0x08CB7E, i_08cb7e),
    (0x08CB86, i_08cb86),
    (0x08CB8A, i_08cb8a),
    (0x08CB8C, i_08cb8c),
    (0x08CB94, i_08cb94),
    (0x08CB98, i_08cb98),
    (0x08CB9C, i_08cb9c),
    (0x08CBA4, i_08cba4),
    (0x08CBA8, i_08cba8),
    (0x08CBAA, i_08cbaa),
    (0x08CBB2, i_08cbb2),
    (0x08CBB6, i_08cbb6),
    (0x08CBBC, i_08cbbc),
    (0x08CBC4, i_08cbc4),
    (0x08CBC6, i_08cbc6),
    (0x08CBCC, i_08cbcc),
    (0x08CBD0, i_08cbd0),
    (0x08CBD6, i_08cbd6),
    (0x08CBDC, i_08cbdc),
    (0x08CBE0, i_08cbe0),
    (0x08CBE6, i_08cbe6),
    (0x08CBEC, i_08cbec),
    (0x08CBF2, i_08cbf2),
    (0x08CBF8, i_08cbf8),
    (0x08CBFE, i_08cbfe),
    (0x08CC02, i_08cc02),
    (0x08CC06, i_08cc06),
    (0x08CC0A, i_08cc0a),
    (0x08CC0E, i_08cc0e),
    (0x08CC10, i_08cc10),
    (0x08CC16, i_08cc16),
    (0x08CC1C, i_08cc1c),
    (0x08CC20, i_08cc20),
    (0x08CC24, i_08cc24),
    (0x08CC28, i_08cc28),
    (0x08CC2C, i_08cc2c),
    (0x08CC30, i_08cc30),
    (0x08CC36, i_08cc36),
    (0x08CC3A, i_08cc3a),
    (0x08CC3E, i_08cc3e),
    (0x08CC42, i_08cc42),
    (0x08CC44, i_08cc44),
    (0x08CC48, i_08cc48),
    (0x08CC4C, i_08cc4c),
    (0x08CC4E, i_08cc4e),
    (0x08CC52, i_08cc52),
    (0x08CC54, i_08cc54),
    (0x08CC56, i_08cc56),
    (0x08CC58, i_08cc58),
    (0x08CC5C, i_08cc5c),
    (0x08CC60, i_08cc60),
    (0x08CC62, i_08cc62),
    (0x08CC64, i_08cc64),
    (0x08CC66, i_08cc66),
    (0x08CC6C, i_08cc6c),
    (0x08CC72, i_08cc72),
    (0x08CC78, i_08cc78),
    (0x08CC7C, i_08cc7c),
    (0x08CC82, i_08cc82),
    (0x08CC88, i_08cc88),
    (0x08CC8E, i_08cc8e),
    (0x08CC94, i_08cc94),
    (0x08CC9A, i_08cc9a),
    (0x08CC9E, i_08cc9e),
    (0x08CCA2, i_08cca2),
    (0x08CCA4, i_08cca4),
    (0x08CCA8, i_08cca8),
    (0x08CCAE, i_08ccae),
    (0x08CCB2, i_08ccb2),
    (0x08CCB8, i_08ccb8),
    (0x08CCBE, i_08ccbe),
    (0x08CCC4, i_08ccc4),
    (0x08CCC6, i_08ccc6),
    (0x08CCCE, i_08ccce),
    (0x08CCD2, i_08ccd2),
    (0x08CCD6, i_08ccd6),
    (0x08CCD8, i_08ccd8),
    (0x08CCE0, i_08cce0),
    (0x08CCE4, i_08cce4),
    (0x08CCE8, i_08cce8),
    (0x08CCEA, i_08ccea),
    (0x08CCF0, i_08ccf0),
    (0x08CCF6, i_08ccf6),
    (0x08CCF8, i_08ccf8),
    (0x08CCFC, i_08ccfc),
    (0x08CCFE, i_08ccfe),
    (0x08CD02, i_08cd02),
    (0x08CD06, i_08cd06),
    (0x08CD0C, i_08cd0c),
    (0x08CD0E, i_08cd0e),
    (0x08CD12, i_08cd12),
    (0x08CD18, i_08cd18),
    (0x08CD1C, i_08cd1c),
    (0x08CD1E, i_08cd1e),
    (0x08CD24, i_08cd24),
    (0x08CD26, i_08cd26),
    (0x08CD2A, i_08cd2a),
    (0x08CD30, i_08cd30),
    (0x08CD34, i_08cd34),
    (0x08CD3A, i_08cd3a),
    (0x08CD3E, i_08cd3e),
    (0x08CD40, i_08cd40),
    (0x08CD44, i_08cd44),
    (0x08CD4A, i_08cd4a),
    (0x08CD4E, i_08cd4e),
    (0x08CD56, i_08cd56),
    (0x08CD5A, i_08cd5a),
    (0x08CD60, i_08cd60),
    (0x08CD64, i_08cd64),
    (0x08CD68, i_08cd68),
    (0x08CD6A, i_08cd6a),
    (0x08CD70, i_08cd70),
    (0x08CD74, i_08cd74),
    (0x08CD7A, i_08cd7a),
    (0x08CD7E, i_08cd7e),
    (0x08CD84, i_08cd84),
    (0x08CD88, i_08cd88),
    (0x08CD8A, i_08cd8a),
    (0x08CD8E, i_08cd8e),
    (0x08CD92, i_08cd92),
    (0x08CD98, i_08cd98),
    (0x08CD9E, i_08cd9e),
    (0x08CDA0, i_08cda0),
    (0x08CDA6, i_08cda6),
    (0x08CDAE, i_08cdae),
    (0x08CDB2, i_08cdb2),
    (0x08CDB8, i_08cdb8),
    (0x08CDBA, i_08cdba),
    (0x08CDBE, i_08cdbe),
    (0x08CDC4, i_08cdc4),
    (0x08CDC8, i_08cdc8),
    (0x08CDD0, i_08cdd0),
    (0x08CDD4, i_08cdd4),
    (0x08CDD8, i_08cdd8),
    (0x08CDDE, i_08cdde),
    (0x08CDE2, i_08cde2),
    (0x08CDE8, i_08cde8),
    (0x08CDEE, i_08cdee),
    (0x08CDF2, i_08cdf2),
    (0x08CDF4, i_08cdf4),
    (0x08CDFA, i_08cdfa),
    (0x08CDFC, i_08cdfc),
    (0x08CE00, i_08ce00),
    (0x08CE06, i_08ce06),
    (0x08CE0A, i_08ce0a),
    (0x08CE10, i_08ce10),
    (0x08CE16, i_08ce16),
    (0x08CE1C, i_08ce1c),
    (0x08CE1E, i_08ce1e),
    (0x08CE24, i_08ce24),
    (0x08CE2A, i_08ce2a),
    (0x08CE2E, i_08ce2e),
    (0x08CE34, i_08ce34),
    (0x08CE3A, i_08ce3a),
    (0x08CE40, i_08ce40),
    (0x08CE44, i_08ce44),
    (0x08CE46, i_08ce46),
    (0x08CE4A, i_08ce4a),
    (0x08CE4E, i_08ce4e),
    (0x08CE52, i_08ce52),
    (0x08CE58, i_08ce58),
    (0x08CE5C, i_08ce5c),
    (0x08CE62, i_08ce62),
    (0x08CE64, i_08ce64),
    (0x08CE68, i_08ce68),
    (0x08CE6E, i_08ce6e),
    (0x08CE72, i_08ce72),
    (0x08CE78, i_08ce78),
    (0x08CE7C, i_08ce7c),
    (0x08CE82, i_08ce82),
    (0x08CE8A, i_08ce8a),
    (0x08CE92, i_08ce92),
    (0x08CE96, i_08ce96),
    (0x08CEA0, i_08cea0),
    (0x08CEA4, i_08cea4),
    (0x08CEAE, i_08ceae),
    (0x08CEB8, i_08ceb8),
    (0x08CEBE, i_08cebe),
    (0x08CEC4, i_08cec4),
    (0x08CEC8, i_08cec8),
    (0x08CECE, i_08cece),
    (0x08CED2, i_08ced2),
    (0x08CED6, i_08ced6),
    (0x08CED8, i_08ced8),
    (0x08CEDE, i_08cede),
    (0x08CEE4, i_08cee4),
    (0x08CEEA, i_08ceea),
    (0x08CEF2, i_08cef2),
    (0x08CEF6, i_08cef6),
    (0x08CEFE, i_08cefe),
    (0x08CF02, i_08cf02),
    (0x08CF0C, i_08cf0c),
    (0x08CF10, i_08cf10),
    (0x08CF1A, i_08cf1a),
    (0x08CF1E, i_08cf1e),
    (0x08CF20, i_08cf20),
    (0x08CF22, i_08cf22),
    (0x08CF28, i_08cf28),
    (0x08CF2E, i_08cf2e),
    (0x08CF34, i_08cf34),
    (0x08CF3C, i_08cf3c),
    (0x08CF40, i_08cf40),
    (0x08CF48, i_08cf48),
    (0x08CF4C, i_08cf4c),
    (0x08CF56, i_08cf56),
    (0x08CF5A, i_08cf5a),
    (0x08CF64, i_08cf64),
    (0x08CF68, i_08cf68),
    (0x08CF6A, i_08cf6a),
    (0x08CF6C, i_08cf6c),
    (0x08CF72, i_08cf72),
    (0x08CF78, i_08cf78),
    (0x08CF7E, i_08cf7e),
    (0x08CF86, i_08cf86),
    (0x08CF8A, i_08cf8a),
    (0x08CF92, i_08cf92),
    (0x08CF96, i_08cf96),
    (0x08CFA0, i_08cfa0),
    (0x08CFA4, i_08cfa4),
    (0x08CFAE, i_08cfae),
    (0x08CFB2, i_08cfb2),
    (0x08CFB4, i_08cfb4),
    (0x08CFB6, i_08cfb6),
    (0x08CFBC, i_08cfbc),
    (0x08CFC2, i_08cfc2),
    (0x08CFC8, i_08cfc8),
    (0x08CFD0, i_08cfd0),
    (0x08CFD4, i_08cfd4),
    (0x08CFDC, i_08cfdc),
    (0x08CFE0, i_08cfe0),
    (0x08CFEA, i_08cfea),
    (0x08CFEE, i_08cfee),
    (0x08CFF8, i_08cff8),
    (0x08CFFC, i_08cffc),
    (0x08CFFE, i_08cffe),
    (0x08D000, i_08d000),
    (0x08D006, i_08d006),
    (0x08D00C, i_08d00c),
    (0x08D012, i_08d012),
    (0x08D01A, i_08d01a),
    (0x08D01E, i_08d01e),
    (0x08D026, i_08d026),
    (0x08D02A, i_08d02a),
    (0x08D034, i_08d034),
    (0x08D038, i_08d038),
    (0x08D042, i_08d042),
    (0x08D046, i_08d046),
    (0x08D048, i_08d048),
    (0x08D04A, i_08d04a),
    (0x08D050, i_08d050),
    (0x08D056, i_08d056),
    (0x08D05C, i_08d05c),
    (0x08D064, i_08d064),
    (0x08D068, i_08d068),
    (0x08D070, i_08d070),
    (0x08D074, i_08d074),
    (0x08D07E, i_08d07e),
    (0x08D082, i_08d082),
    (0x08D08C, i_08d08c),
    (0x08D090, i_08d090),
    (0x08D092, i_08d092),
    (0x08D094, i_08d094),
    (0x08D09A, i_08d09a),
    (0x08D0A0, i_08d0a0),
    (0x08D0A6, i_08d0a6),
    (0x08D0A8, i_08d0a8),
    (0x08D0AC, i_08d0ac),
    (0x08D0B2, i_08d0b2),
    (0x08D0B6, i_08d0b6),
    (0x08D0BC, i_08d0bc),
    (0x08D0C0, i_08d0c0),
    (0x08D0C6, i_08d0c6),
    (0x08D0CA, i_08d0ca),
    (0x08D0D0, i_08d0d0),
    (0x08D0D4, i_08d0d4),
    (0x08D0DA, i_08d0da),
    (0x08D0E2, i_08d0e2),
    (0x08D0EC, i_08d0ec),
    (0x08D0F6, i_08d0f6),
    (0x08D0FA, i_08d0fa),
    (0x08D100, i_08d100),
    (0x08D104, i_08d104),
    (0x08D10A, i_08d10a),
    (0x08D10E, i_08d10e),
    (0x08D114, i_08d114),
    (0x08D118, i_08d118),
    (0x08D11E, i_08d11e),
    (0x08D126, i_08d126),
    (0x08D130, i_08d130),
    (0x08D138, i_08d138),
    (0x08D13C, i_08d13c),
    (0x08D146, i_08d146),
    (0x08D14A, i_08d14a),
    (0x08D154, i_08d154),
    (0x08D15A, i_08d15a),
    (0x08D15E, i_08d15e),
    (0x08D164, i_08d164),
    (0x08D168, i_08d168),
    (0x08D16C, i_08d16c),
    (0x08D16E, i_08d16e),
    (0x08D174, i_08d174),
    (0x08D17A, i_08d17a),
    (0x08D180, i_08d180),
    (0x08D182, i_08d182),
    (0x08D184, i_08d184),
    (0x08D18A, i_08d18a),
    (0x08D18E, i_08d18e),
    (0x08D194, i_08d194),
    (0x08D198, i_08d198),
    (0x08D19E, i_08d19e),
    (0x08D1A2, i_08d1a2),
    (0x08D1A6, i_08d1a6),
    (0x08D1A8, i_08d1a8),
    (0x08D1AC, i_08d1ac),
    (0x08D1AE, i_08d1ae),
    (0x08D1B4, i_08d1b4),
    (0x08D1B8, i_08d1b8),
    (0x08D1BE, i_08d1be),
    (0x08D1C2, i_08d1c2),
    (0x08D1C8, i_08d1c8),
    (0x08D1CC, i_08d1cc),
    (0x08D1D2, i_08d1d2),
    (0x08D1D6, i_08d1d6),
    (0x08D1DC, i_08d1dc),
    (0x08D1E0, i_08d1e0),
    (0x08D1E2, i_08d1e2),
    (0x08D1E6, i_08d1e6),
    (0x08D1E8, i_08d1e8),
    (0x08D1EE, i_08d1ee),
    (0x08D1F2, i_08d1f2),
    (0x08D1F8, i_08d1f8),
    (0x08D1FC, i_08d1fc),
    (0x08D202, i_08d202),
    (0x08D206, i_08d206),
    (0x08D20A, i_08d20a),
    (0x08D20C, i_08d20c),
    (0x08D210, i_08d210),
    (0x08D212, i_08d212),
    (0x08D218, i_08d218),
    (0x08D21C, i_08d21c),
    (0x08D222, i_08d222),
    (0x08D226, i_08d226),
    (0x08D22C, i_08d22c),
    (0x08D230, i_08d230),
    (0x08D236, i_08d236),
    (0x08D23A, i_08d23a),
    (0x08D240, i_08d240),
    (0x08D244, i_08d244),
    (0x08D246, i_08d246),
    (0x08D24A, i_08d24a),
    (0x08D24C, i_08d24c),
    (0x08D252, i_08d252),
    (0x08D256, i_08d256),
    (0x08D25C, i_08d25c),
    (0x08D260, i_08d260),
    (0x08D266, i_08d266),
    (0x08D26A, i_08d26a),
    (0x08D26E, i_08d26e),
    (0x08D270, i_08d270),
    (0x08D274, i_08d274),
    (0x08D276, i_08d276),
    (0x08D27C, i_08d27c),
    (0x08D280, i_08d280),
    (0x08D286, i_08d286),
    (0x08D28A, i_08d28a),
    (0x08D290, i_08d290),
    (0x08D294, i_08d294),
    (0x08D29A, i_08d29a),
    (0x08D29E, i_08d29e),
    (0x08D2A4, i_08d2a4),
    (0x08D2A8, i_08d2a8),
    (0x08D2AA, i_08d2aa),
    (0x08D2AE, i_08d2ae),
    (0x08D2B0, i_08d2b0),
    (0x08D2B4, i_08d2b4),
    (0x08D2B6, i_08d2b6),
    (0x08D2BA, i_08d2ba),
    (0x08D2BC, i_08d2bc),
    (0x08D2C0, i_08d2c0),
    (0x08D2C2, i_08d2c2),
    (0x08D2C4, i_08d2c4),
    (0x08D2C6, i_08d2c6),
    (0x08D2C8, i_08d2c8),
    (0x08D2CC, i_08d2cc),
    (0x08D2CE, i_08d2ce),
    (0x08D2D2, i_08d2d2),
    (0x08D2D4, i_08d2d4),
    (0x08D2D8, i_08d2d8),
    (0x08D2DA, i_08d2da),
    (0x08D2DE, i_08d2de),
    (0x08D2E0, i_08d2e0),
    (0x08D2E4, i_08d2e4),
    (0x08D2E6, i_08d2e6),
    (0x08D2E8, i_08d2e8),
    (0x08D2EA, i_08d2ea),
    (0x08D2EC, i_08d2ec),
    (0x08D2F0, i_08d2f0),
    (0x08D2F2, i_08d2f2),
    (0x08D2F6, i_08d2f6),
    (0x08D2F8, i_08d2f8),
    (0x08D2FC, i_08d2fc),
    (0x08D300, i_08d300),
    (0x08D304, i_08d304),
    (0x08D306, i_08d306),
    (0x08D30A, i_08d30a),
    (0x08D30C, i_08d30c),
    (0x08D310, i_08d310),
    (0x08D312, i_08d312),
    (0x08D314, i_08d314),
    (0x08D316, i_08d316),
    (0x08D318, i_08d318),
    (0x08D31C, i_08d31c),
    (0x08D31E, i_08d31e),
    (0x08D322, i_08d322),
    (0x08D326, i_08d326),
    (0x08D32A, i_08d32a),
    (0x08D32E, i_08d32e),
    (0x08D330, i_08d330),
    (0x08D334, i_08d334),
    (0x08D336, i_08d336),
    (0x08D33A, i_08d33a),
    (0x08D33C, i_08d33c),
    (0x08D33E, i_08d33e),
    (0x08D340, i_08d340),
    (0x08D342, i_08d342),
    (0x08D346, i_08d346),
    (0x08D348, i_08d348),
    (0x08D34C, i_08d34c),
    (0x08D34E, i_08d34e),
    (0x08D352, i_08d352),
    (0x08D356, i_08d356),
    (0x08D35A, i_08d35a),
    (0x08D35C, i_08d35c),
    (0x08D360, i_08d360),
    (0x08D362, i_08d362),
    (0x08D366, i_08d366),
    (0x08D368, i_08d368),
    (0x08D36A, i_08d36a),
    (0x08D36C, i_08d36c),
    (0x08D36E, i_08d36e),
    (0x08D372, i_08d372),
    (0x08D374, i_08d374),
    (0x08D378, i_08d378),
    (0x08D37C, i_08d37c),
    (0x08D380, i_08d380),
    (0x08D384, i_08d384),
    (0x08D386, i_08d386),
    (0x08D38A, i_08d38a),
    (0x08D38C, i_08d38c),
    (0x08D390, i_08d390),
    (0x08D392, i_08d392),
    (0x08D394, i_08d394),
    (0x08D396, i_08d396),
    (0x08D398, i_08d398),
    (0x08D39C, i_08d39c),
    (0x08D39E, i_08d39e),
    (0x08D3A2, i_08d3a2),
    (0x08D3A6, i_08d3a6),
    (0x08D3AA, i_08d3aa),
    (0x08D3AE, i_08d3ae),
    (0x08D3B2, i_08d3b2),
    (0x08D3B4, i_08d3b4),
    (0x08D3BA, i_08d3ba),
    (0x08D3BE, i_08d3be),
    (0x08D3C4, i_08d3c4),
    (0x08D3CA, i_08d3ca),
    (0x08D3D0, i_08d3d0),
    (0x08D3D6, i_08d3d6),
    (0x08D3DC, i_08d3dc),
    (0x08D3E2, i_08d3e2),
    (0x08D3E6, i_08d3e6),
    (0x08D3EC, i_08d3ec),
    (0x08D3F0, i_08d3f0),
    (0x08D3F2, i_08d3f2),
    (0x08D3F8, i_08d3f8),
    (0x08D3FE, i_08d3fe),
    (0x08D404, i_08d404),
    (0x08D408, i_08d408),
    (0x08D40E, i_08d40e),
    (0x08D414, i_08d414),
    (0x08D418, i_08d418),
    (0x08D41A, i_08d41a),
    (0x08D41C, i_08d41c),
    (0x08D422, i_08d422),
    (0x08D428, i_08d428),
    (0x08D42C, i_08d42c),
    (0x08D430, i_08d430),
    (0x08D436, i_08d436),
    (0x08D43C, i_08d43c),
    (0x08D442, i_08d442),
    (0x08D448, i_08d448),
    (0x08D44C, i_08d44c),
    (0x08D44E, i_08d44e),
    (0x08D450, i_08d450),
    (0x08D456, i_08d456),
    (0x08D45C, i_08d45c),
    (0x08D462, i_08d462),
    (0x08D468, i_08d468),
    (0x08D46C, i_08d46c),
    (0x08D470, i_08d470),
    (0x08D472, i_08d472),
    (0x08D474, i_08d474),
    (0x08D478, i_08d478),
    (0x08D47E, i_08d47e),
    (0x08D482, i_08d482),
    (0x08D488, i_08d488),
    (0x08D48E, i_08d48e),
    (0x08D494, i_08d494),
    (0x08D49A, i_08d49a),
    (0x08D4A0, i_08d4a0),
    (0x08D4A6, i_08d4a6),
    (0x08D4AC, i_08d4ac),
    (0x08D4B2, i_08d4b2),
    (0x08D4B8, i_08d4b8),
    (0x08D4BE, i_08d4be),
    (0x08D4C2, i_08d4c2),
    (0x08D4C4, i_08d4c4),
    (0x08D4C6, i_08d4c6),
    (0x08D4CA, i_08d4ca),
    (0x08D4D0, i_08d4d0),
    (0x08D4D4, i_08d4d4),
    (0x08D4DA, i_08d4da),
    (0x08D4DE, i_08d4de),
    (0x08D4E4, i_08d4e4),
    (0x08D4EA, i_08d4ea),
    (0x08D4F0, i_08d4f0),
    (0x08D4F6, i_08d4f6),
    (0x08D4FA, i_08d4fa),
    (0x08D4FC, i_08d4fc),
    (0x08D4FE, i_08d4fe),
    (0x08D502, i_08d502),
    (0x08D506, i_08d506),
    (0x08D50C, i_08d50c),
    (0x08D510, i_08d510),
    (0x08D516, i_08d516),
    (0x08D51C, i_08d51c),
    (0x08D520, i_08d520),
    (0x08D526, i_08d526),
    (0x08D52C, i_08d52c),
    (0x08D530, i_08d530),
    (0x08D534, i_08d534),
    (0x08D536, i_08d536),
    (0x08D538, i_08d538),
    (0x08D53E, i_08d53e),
    (0x08D544, i_08d544),
    (0x08D548, i_08d548),
    (0x08D54A, i_08d54a),
    (0x08D550, i_08d550),
    (0x08D554, i_08d554),
    (0x08D558, i_08d558),
    (0x08D55A, i_08d55a),
    (0x08D55C, i_08d55c),
    (0x08D560, i_08d560),
    (0x08D564, i_08d564),
    (0x08D56A, i_08d56a),
    (0x08D56E, i_08d56e),
    (0x08D572, i_08d572),
    (0x08D574, i_08d574),
    (0x08D578, i_08d578),
    (0x08D57E, i_08d57e),
    (0x08D580, i_08d580),
    (0x08D586, i_08d586),
    (0x08D58C, i_08d58c),
    (0x08D590, i_08d590),
    (0x08D592, i_08d592),
    (0x08D598, i_08d598),
    (0x08D59C, i_08d59c),
    (0x08D5A0, i_08d5a0),
    (0x08D5A2, i_08d5a2),
    (0x08D5A6, i_08d5a6),
    (0x08D5A8, i_08d5a8),
    (0x08D5AC, i_08d5ac),
    (0x08D5B0, i_08d5b0),
    (0x08D5B6, i_08d5b6),
    (0x08D5BA, i_08d5ba),
    (0x08D5BE, i_08d5be),
    (0x08D5C0, i_08d5c0),
    (0x08D5C6, i_08d5c6),
    (0x08D5CA, i_08d5ca),
    (0x08D5CC, i_08d5cc),
    (0x08D5D2, i_08d5d2),
    (0x08D5D8, i_08d5d8),
    (0x08D5DE, i_08d5de),
    (0x08D5E4, i_08d5e4),
    (0x08D5E8, i_08d5e8),
    (0x08D5EA, i_08d5ea),
    (0x08D5EE, i_08d5ee),
    (0x08D5F2, i_08d5f2),
    (0x08D5F8, i_08d5f8),
    (0x08D5FC, i_08d5fc),
    (0x08D602, i_08d602),
    (0x08D608, i_08d608),
    (0x08D60C, i_08d60c),
    (0x08D612, i_08d612),
    (0x08D616, i_08d616),
    (0x08D61C, i_08d61c),
    (0x08D620, i_08d620),
    (0x08D626, i_08d626),
    (0x08D62A, i_08d62a),
    (0x08D62C, i_08d62c),
    (0x08D62E, i_08d62e),
    (0x08D632, i_08d632),
    (0x08D636, i_08d636),
    (0x08D63C, i_08d63c),
    (0x08D640, i_08d640),
    (0x08D646, i_08d646),
    (0x08D64C, i_08d64c),
    (0x08D650, i_08d650),
    (0x08D656, i_08d656),
    (0x08D65A, i_08d65a),
    (0x08D65E, i_08d65e),
    (0x08D662, i_08d662),
    (0x08D668, i_08d668),
    (0x08D66C, i_08d66c),
    (0x08D66E, i_08d66e),
    (0x08D670, i_08d670),
    (0x08D676, i_08d676),
    (0x08D67E, i_08d67e),
    (0x08D686, i_08d686),
    (0x08D68E, i_08d68e),
    (0x08D696, i_08d696),
    (0x08D69C, i_08d69c),
    (0x08D6A2, i_08d6a2),
    (0x08D6A6, i_08d6a6),
    (0x08D6A8, i_08d6a8),
    (0x08D6AC, i_08d6ac),
    (0x08D6B0, i_08d6b0),
    (0x08D6B4, i_08d6b4),
    (0x08D6BA, i_08d6ba),
    (0x08D6BE, i_08d6be),
    (0x08D6C2, i_08d6c2),
    (0x08D6C6, i_08d6c6),
    (0x08D6CA, i_08d6ca),
    (0x08D6CE, i_08d6ce),
    (0x08D6D2, i_08d6d2),
    (0x08D6D8, i_08d6d8),
    (0x08D6DE, i_08d6de),
    (0x08D6E4, i_08d6e4),
    (0x08D6EA, i_08d6ea),
    (0x08D6EC, i_08d6ec),
    (0x08D6F0, i_08d6f0),
    (0x08D6F6, i_08d6f6),
    (0x08D6FC, i_08d6fc),
    (0x08D702, i_08d702),
    (0x08D708, i_08d708),
    (0x08D70C, i_08d70c),
    (0x08D70E, i_08d70e),
    (0x08D714, i_08d714),
    (0x08D718, i_08d718),
    (0x08D71E, i_08d71e),
    (0x08D722, i_08d722),
    (0x08D726, i_08d726),
    (0x08D728, i_08d728),
    (0x08D72A, i_08d72a),
    (0x08D730, i_08d730),
    (0x08D736, i_08d736),
    (0x08D73A, i_08d73a),
    (0x08D73E, i_08d73e),
    (0x08D744, i_08d744),
    (0x08D74A, i_08d74a),
    (0x08D74E, i_08d74e),
    (0x08D754, i_08d754),
    (0x08D75A, i_08d75a),
    (0x08D760, i_08d760),
    (0x08D766, i_08d766),
    (0x08D76C, i_08d76c),
    (0x08D772, i_08d772),
    (0x08D774, i_08d774),
    (0x08D778, i_08d778),
    (0x08D77C, i_08d77c),
    (0x08D782, i_08d782),
    (0x08D786, i_08d786),
    (0x08D78C, i_08d78c),
    (0x08D792, i_08d792),
    (0x08D798, i_08d798),
    (0x08D7A0, i_08d7a0),
    (0x08D7A4, i_08d7a4),
    (0x08D7AA, i_08d7aa),
    (0x08D7B0, i_08d7b0),
    (0x08D7B6, i_08d7b6),
    (0x08D7BA, i_08d7ba),
    (0x08D7BC, i_08d7bc),
    (0x08D7BE, i_08d7be),
    (0x08D7C4, i_08d7c4),
    (0x08D7CA, i_08d7ca),
    (0x08D7CE, i_08d7ce),
    (0x08D7D4, i_08d7d4),
    (0x08D7D8, i_08d7d8),
    (0x08D7DA, i_08d7da),
    (0x08D7E0, i_08d7e0),
    (0x08D7E2, i_08d7e2),
    (0x08D7E6, i_08d7e6),
    (0x08D7EC, i_08d7ec),
    (0x08D7F0, i_08d7f0),
    (0x08D7F6, i_08d7f6),
    (0x08D7FC, i_08d7fc),
    (0x08D802, i_08d802),
    (0x08D804, i_08d804),
    (0x08D80C, i_08d80c),
    (0x08D810, i_08d810),
    (0x08D818, i_08d818),
    (0x08D81C, i_08d81c),
    (0x08D820, i_08d820),
    (0x08D822, i_08d822),
    (0x08D82A, i_08d82a),
    (0x08D82E, i_08d82e),
    (0x08D836, i_08d836),
    (0x08D83A, i_08d83a),
    (0x08D83E, i_08d83e),
    (0x08D840, i_08d840),
    (0x08D842, i_08d842),
    (0x08D848, i_08d848),
    (0x08D84E, i_08d84e),
    (0x08D852, i_08d852),
    (0x08D856, i_08d856),
    (0x08D85C, i_08d85c),
    (0x08D862, i_08d862),
    (0x08D868, i_08d868),
    (0x08D86C, i_08d86c),
    (0x08D86E, i_08d86e),
    (0x08D876, i_08d876),
    (0x08D87A, i_08d87a),
    (0x08D880, i_08d880),
    (0x08D886, i_08d886),
    (0x08D88A, i_08d88a),
    (0x08D88C, i_08d88c),
    (0x08D892, i_08d892),
    (0x08D898, i_08d898),
    (0x08D89A, i_08d89a),
    (0x08D8A2, i_08d8a2),
    (0x08D8A6, i_08d8a6),
    (0x08D8AC, i_08d8ac),
    (0x08D8B2, i_08d8b2),
    (0x08D8B6, i_08d8b6),
    (0x08D8B8, i_08d8b8),
    (0x08D8BE, i_08d8be),
    (0x08D8C4, i_08d8c4),
    (0x08D8C6, i_08d8c6),
    (0x08D8CC, i_08d8cc),
    (0x08D8D2, i_08d8d2),
    (0x08D8D6, i_08d8d6),
    (0x08D8DC, i_08d8dc),
    (0x08D8E4, i_08d8e4),
    (0x08D8EC, i_08d8ec),
    (0x08D8F0, i_08d8f0),
    (0x08D8F2, i_08d8f2),
    (0x08D8F4, i_08d8f4),
    (0x08D8FC, i_08d8fc),
    (0x08D900, i_08d900),
    (0x08D906, i_08d906),
    (0x08D910, i_08d910),
    (0x08D918, i_08d918),
    (0x08D920, i_08d920),
    (0x08D924, i_08d924),
    (0x08D926, i_08d926),
    (0x08D92A, i_08d92a),
    (0x08D930, i_08d930),
    (0x08D936, i_08d936),
    (0x08D938, i_08d938),
    (0x08D93C, i_08d93c),
    (0x08D942, i_08d942),
    (0x08D946, i_08d946),
    (0x08D94A, i_08d94a),
    (0x08D950, i_08d950),
    (0x08D956, i_08d956),
    (0x08D95C, i_08d95c),
    (0x08D962, i_08d962),
    (0x08D968, i_08d968),
    (0x08D972, i_08d972),
    (0x08D976, i_08d976),
    (0x08D978, i_08d978),
    (0x08D97E, i_08d97e),
    (0x08D984, i_08d984),
    (0x08D986, i_08d986),
    (0x08D98C, i_08d98c),
    (0x08D992, i_08d992),
    (0x08D994, i_08d994),
    (0x08D998, i_08d998),
    (0x08D99C, i_08d99c),
    (0x08D9A0, i_08d9a0),
    (0x08D9A4, i_08d9a4),
    (0x08D9AA, i_08d9aa),
    (0x08D9B0, i_08d9b0),
    (0x08D9B4, i_08d9b4),
    (0x08D9B6, i_08d9b6),
    (0x08D9BA, i_08d9ba),
    (0x08D9BE, i_08d9be),
    (0x08D9C4, i_08d9c4),
    (0x08D9CA, i_08d9ca),
    (0x08D9CE, i_08d9ce),
    (0x08D9D0, i_08d9d0),
    (0x08D9D6, i_08d9d6),
    (0x08D9DA, i_08d9da),
    (0x08D9E0, i_08d9e0),
    (0x08D9E2, i_08d9e2),
    (0x08D9E6, i_08d9e6),
    (0x08D9EA, i_08d9ea),
    (0x08D9EE, i_08d9ee),
    (0x08D9F2, i_08d9f2),
    (0x08D9F8, i_08d9f8),
    (0x08D9FE, i_08d9fe),
    (0x08DA02, i_08da02),
    (0x08DA04, i_08da04),
    (0x08DA08, i_08da08),
    (0x08DA0C, i_08da0c),
    (0x08DA12, i_08da12),
    (0x08DA18, i_08da18),
    (0x08DA1C, i_08da1c),
    (0x08DA1E, i_08da1e),
    (0x08DA24, i_08da24),
    (0x08DA28, i_08da28),
    (0x08DA2E, i_08da2e),
    (0x08DA30, i_08da30),
    (0x08DA36, i_08da36),
    (0x08DA3C, i_08da3c),
    (0x08DA42, i_08da42),
    (0x08DA48, i_08da48),
    (0x08DA4C, i_08da4c),
    (0x08DA50, i_08da50),
    (0x08DA54, i_08da54),
    (0x08DA58, i_08da58),
    (0x08DA5C, i_08da5c),
    (0x08DA60, i_08da60),
    (0x08DA66, i_08da66),
    (0x08DA6C, i_08da6c),
    (0x08DA70, i_08da70),
    (0x08DA72, i_08da72),
    (0x08DA76, i_08da76),
    (0x08DA7A, i_08da7a),
    (0x08DA80, i_08da80),
    (0x08DA86, i_08da86),
    (0x08DA8A, i_08da8a),
    (0x08DA8C, i_08da8c),
    (0x08DA92, i_08da92),
    (0x08DA96, i_08da96),
    (0x08DA9C, i_08da9c),
    (0x08DA9E, i_08da9e),
    (0x08DAA4, i_08daa4),
    (0x08DAA8, i_08daa8),
    (0x08DAAC, i_08daac),
    (0x08DAB2, i_08dab2),
    (0x08DAB8, i_08dab8),
    (0x08DABE, i_08dabe),
    (0x08DAC4, i_08dac4),
    (0x08DACA, i_08daca),
    (0x08DACE, i_08dace),
    (0x08DAD0, i_08dad0),
    (0x08DAD6, i_08dad6),
    (0x08DADA, i_08dada),
    (0x08DAE0, i_08dae0),
    (0x08DAE2, i_08dae2),
    (0x08DAE8, i_08dae8),
    (0x08DAEE, i_08daee),
    (0x08DAF4, i_08daf4),
    (0x08DAF8, i_08daf8),
    (0x08DAFC, i_08dafc),
    (0x08DB00, i_08db00),
    (0x08DB06, i_08db06),
    (0x08DB0C, i_08db0c),
    (0x08DB10, i_08db10),
    (0x08DB16, i_08db16),
    (0x08DB1C, i_08db1c),
    (0x08DB20, i_08db20),
    (0x08DB22, i_08db22),
    (0x08DB28, i_08db28),
    (0x08DB2C, i_08db2c),
    (0x08DB32, i_08db32),
    (0x08DB38, i_08db38),
    (0x08DB3C, i_08db3c),
    (0x08DB42, i_08db42),
    (0x08DB48, i_08db48),
    (0x08DB4E, i_08db4e),
    (0x08DB50, i_08db50),
    (0x08DB56, i_08db56),
    (0x08DB5A, i_08db5a),
    (0x08DB5E, i_08db5e),
    (0x08DB62, i_08db62),
    (0x08DB68, i_08db68),
    (0x08DB6E, i_08db6e),
    (0x08DB74, i_08db74),
    (0x08DB78, i_08db78),
    (0x08DB7A, i_08db7a),
    (0x08DB80, i_08db80),
    (0x08DB86, i_08db86),
    (0x08DB8A, i_08db8a),
    (0x08DB90, i_08db90),
    (0x08DB94, i_08db94),
    (0x08DB9A, i_08db9a),
    (0x08DBA0, i_08dba0),
    (0x08DBA4, i_08dba4),
    (0x08DBA6, i_08dba6),
    (0x08DBAC, i_08dbac),
    (0x08DBB0, i_08dbb0),
    (0x08DBB6, i_08dbb6),
    (0x08DBBC, i_08dbbc),
    (0x08DBC0, i_08dbc0),
    (0x08DBC4, i_08dbc4),
    (0x08DBC6, i_08dbc6),
    (0x08DBC8, i_08dbc8),
    (0x08DBCC, i_08dbcc),
    (0x08DBD2, i_08dbd2),
    (0x08DBD6, i_08dbd6),
    (0x08DBDA, i_08dbda),
    (0x08DBDC, i_08dbdc),
    (0x08DBE0, i_08dbe0),
    (0x08DBE2, i_08dbe2),
    (0x08DBE8, i_08dbe8),
    (0x08DBEE, i_08dbee),
    (0x08DBF2, i_08dbf2),
    (0x08DBF8, i_08dbf8),
    (0x08DBFC, i_08dbfc),
    (0x08DC02, i_08dc02),
    (0x08DC08, i_08dc08),
    (0x08DC0C, i_08dc0c),
    (0x08DC0E, i_08dc0e),
    (0x08DC14, i_08dc14),
    (0x08DC18, i_08dc18),
    (0x08DC1E, i_08dc1e),
    (0x08DC24, i_08dc24),
    (0x08DC28, i_08dc28),
    (0x08DC2C, i_08dc2c),
    (0x08DC2E, i_08dc2e),
    (0x08DC30, i_08dc30),
    (0x08DC34, i_08dc34),
    (0x08DC3A, i_08dc3a),
    (0x08DC3E, i_08dc3e),
    (0x08DC42, i_08dc42),
    (0x08DC44, i_08dc44),
    (0x08DC48, i_08dc48),
    (0x08DC4A, i_08dc4a),
    (0x08DC50, i_08dc50),
    (0x08DC56, i_08dc56),
    (0x08DC5A, i_08dc5a),
    (0x08DC5E, i_08dc5e),
    (0x08DC62, i_08dc62),
    (0x08DC66, i_08dc66),
    (0x08DC6A, i_08dc6a),
    (0x08DC70, i_08dc70),
    (0x08DC76, i_08dc76),
    (0x08DC7C, i_08dc7c),
    (0x08DC82, i_08dc82),
    (0x08DC88, i_08dc88),
    (0x08DC8C, i_08dc8c),
    (0x08DC8E, i_08dc8e),
    (0x08DC92, i_08dc92),
    (0x08DC94, i_08dc94),
    (0x08DC9A, i_08dc9a),
    (0x08DCA0, i_08dca0),
    (0x08DCA4, i_08dca4),
    (0x08DCA6, i_08dca6),
    (0x08DCAC, i_08dcac),
    (0x08DCB2, i_08dcb2),
    (0x08DCB6, i_08dcb6),
    (0x08DCBC, i_08dcbc),
    (0x08DCC0, i_08dcc0),
    (0x08DCC4, i_08dcc4),
    (0x08DCC8, i_08dcc8),
    (0x08DCCC, i_08dccc),
    (0x08DCD0, i_08dcd0),
    (0x08DCD2, i_08dcd2),
    (0x08DCD6, i_08dcd6),
    (0x08DCDA, i_08dcda),
    (0x08DCDE, i_08dcde),
    (0x08DCE2, i_08dce2),
    (0x08DCE6, i_08dce6),
    (0x08DCEA, i_08dcea),
    (0x08DCEC, i_08dcec),
    (0x08DCF0, i_08dcf0),
    (0x08DCF4, i_08dcf4),
    (0x08DCF8, i_08dcf8),
    (0x08DCFA, i_08dcfa),
    (0x08DD00, i_08dd00),
    (0x08DD06, i_08dd06),
    (0x08DD0A, i_08dd0a),
    (0x08DD0C, i_08dd0c),
    (0x08DD12, i_08dd12),
    (0x08DD18, i_08dd18),
    (0x08DD1C, i_08dd1c),
    (0x08DD20, i_08dd20),
    (0x08DD24, i_08dd24),
    (0x08DD26, i_08dd26),
    (0x08DD2A, i_08dd2a),
    (0x08DD30, i_08dd30),
    (0x08DD34, i_08dd34),
    (0x08DD38, i_08dd38),
    (0x08DD3E, i_08dd3e),
    (0x08DD42, i_08dd42),
    (0x08DD48, i_08dd48),
    (0x08DD4C, i_08dd4c),
    (0x08DD52, i_08dd52),
    (0x08DD58, i_08dd58),
    (0x08DD5C, i_08dd5c),
    (0x08DD5E, i_08dd5e),
    (0x08DD64, i_08dd64),
    (0x08DD68, i_08dd68),
    (0x08DD6E, i_08dd6e),
    (0x08DD72, i_08dd72),
    (0x08DD76, i_08dd76),
    (0x08DD78, i_08dd78),
    (0x08DD7C, i_08dd7c),
    (0x08DD7E, i_08dd7e),
    (0x08DD84, i_08dd84),
    (0x08DD8A, i_08dd8a),
    (0x08DD8E, i_08dd8e),
    (0x08DD94, i_08dd94),
    (0x08DD98, i_08dd98),
    (0x08DD9E, i_08dd9e),
    (0x08DDA4, i_08dda4),
    (0x08DDA8, i_08dda8),
    (0x08DDAA, i_08ddaa),
    (0x08DDAE, i_08ddae),
    (0x08DDB0, i_08ddb0),
    (0x08DDB4, i_08ddb4),
    (0x08DDBA, i_08ddba),
    (0x08DDBE, i_08ddbe),
    (0x08DDC2, i_08ddc2),
    (0x08DDC8, i_08ddc8),
    (0x08DDCE, i_08ddce),
    (0x08DDD2, i_08ddd2),
    (0x08DDD4, i_08ddd4),
    (0x08DDD8, i_08ddd8),
    (0x08DDDA, i_08ddda),
    (0x08DDDE, i_08ddde),
    (0x08DDE4, i_08dde4),
    (0x08DDE8, i_08dde8),
    (0x08DDEA, i_08ddea),
    (0x08DDEE, i_08ddee),
    (0x08DDF2, i_08ddf2),
    (0x08DDF8, i_08ddf8),
    (0x08DDFC, i_08ddfc),
    (0x08DE00, i_08de00),
    (0x08DE06, i_08de06),
    (0x08DE0C, i_08de0c),
    (0x08DE12, i_08de12),
    (0x08DE18, i_08de18),
    (0x08DE1E, i_08de1e),
    (0x08DE22, i_08de22),
    (0x08DE24, i_08de24),
    (0x08DE28, i_08de28),
    (0x08DE2A, i_08de2a),
    (0x08DE30, i_08de30),
    (0x08DE34, i_08de34),
    (0x08DE3A, i_08de3a),
    (0x08DE3E, i_08de3e),
    (0x08DE44, i_08de44),
    (0x08DE48, i_08de48),
    (0x08DE4E, i_08de4e),
    (0x08DE54, i_08de54),
    (0x08DE58, i_08de58),
    (0x08DE5C, i_08de5c),
    (0x08DE5E, i_08de5e),
    (0x08DE62, i_08de62),
    (0x08DE64, i_08de64),
    (0x08DE6A, i_08de6a),
    (0x08DE70, i_08de70),
    (0x08DE74, i_08de74),
    (0x08DE78, i_08de78),
    (0x08DE7C, i_08de7c),
    (0x08DE80, i_08de80),
    (0x08DE86, i_08de86),
    (0x08DE8A, i_08de8a),
    (0x08DE8C, i_08de8c),
    (0x08DE92, i_08de92),
    (0x08DE98, i_08de98),
    (0x08DE9C, i_08de9c),
    (0x08DE9E, i_08de9e),
    (0x08DEA4, i_08dea4),
    (0x08DEAA, i_08deaa),
    (0x08DEAE, i_08deae),
    (0x08DEB2, i_08deb2),
    (0x08DEB6, i_08deb6),
    (0x08DEBA, i_08deba),
    (0x08DEBC, i_08debc),
    (0x08DEC0, i_08dec0),
];
