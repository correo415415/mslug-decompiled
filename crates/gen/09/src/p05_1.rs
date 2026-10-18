// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p05_1) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $051BA6: rts
pub fn i_051ba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BA8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== CellApply_BidirScan_051BA8  ($051BA8, 96 B, entity/collision_cell_apply_051ba8.s)
/// $051BA8: movem.l d2-d3/a1, -(a7)
pub fn i_051ba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BAC;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $051BAC: movea.l $12(a0), a1
pub fn i_051bac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BB0;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0x12);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $051BB0: move.l a1, d2
pub fn i_051bb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BB2;
    cpu.ir = 0x2409;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.d[2] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $051BB2: beq.w $51bfe
pub fn i_051bb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BB6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x51BFE; return 10; }
    8
}
/// $051BB6: move.w (a1), d2
pub fn i_051bb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BB8;
    cpu.ir = 0x3411;
    let v2 = bus.read16(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $051BB8: move.w $4(a1), d3
pub fn i_051bb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BBC;
    cpu.ir = 0x3629;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $051BBC: cmp.w d2, d0
pub fn i_051bbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BBE;
    cpu.ir = 0xB042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $051BBE: bcc.w $51bd0
pub fn i_051bbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BC2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x51BD0; return 10; }
    8
}
/// $051BC2: cmpi.w #$0, d2
pub fn i_051bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BC6;
    cpu.ir = 0xC42;
    let v1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $051BC6: beq.w $51bfe
pub fn i_051bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BCA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x51BFE; return 10; }
    8
}
/// $051BCA: lea.l -$4(a1), a1
pub fn i_051bca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BCE;
    cpu.ir = 0x43E9;
    let ad1 = cpu.a[1].wrapping_add(0xFFFFFFFC);
    cpu.a[1] = ad1;
    16
}
/// $051BCE: bra.b $51bb6
pub fn i_051bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BD0;
    cpu.ir = 0x60E6;
    cpu.pc = 0x51BB6;
    10
}
/// $051BD0: cmp.w d3, d0
pub fn i_051bd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BD2;
    cpu.ir = 0xB043;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $051BD2: bcs.w $51be6
pub fn i_051bd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BD6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x51BE6; return 10; }
    8
}
/// $051BD6: cmpi.w #$ffff, $4(a1)
pub fn i_051bd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BDC;
    cpu.ir = 0xC69;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFFF, Size::Word, false);
    16
}
/// $051BDC: beq.w $51bfe
pub fn i_051bdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BE0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x51BFE; return 10; }
    8
}
/// $051BE0: lea.l $4(a1), a1
pub fn i_051be0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BE4;
    cpu.ir = 0x43E9;
    let ad1 = cpu.a[1].wrapping_add(0x4);
    cpu.a[1] = ad1;
    16
}
/// $051BE4: bra.b $51bb6
pub fn i_051be4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BE6;
    cpu.ir = 0x60D0;
    cpu.pc = 0x51BB6;
    10
}
/// $051BE6: move.b $2(a1), $27(a0)
pub fn i_051be6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BEC;
    cpu.ir = 0x1169;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x27);
    bus.write8(ad4, v3 as u8);
    20
}
/// $051BEC: move.b $3(a1), d2
pub fn i_051bec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BF0;
    cpu.ir = 0x1429;
    let ad2 = cpu.a[1].wrapping_add(0x3);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $051BF0: add.w $24(a0), d2
pub fn i_051bf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BF4;
    cpu.ir = 0xD468;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $051BF4: neg.w d2
pub fn i_051bf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BF6;
    cpu.ir = 0x4442;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $051BF6: andi.w #$1f, d2
pub fn i_051bf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BFA;
    cpu.ir = 0x242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $051BFA: move.b d2, $26(a0)
pub fn i_051bfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BFE;
    cpu.ir = 0x1142;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x26);
    bus.write8(ad2, v1 as u8);
    12
}
/// $051BFE: move.l a1, $12(a0)
pub fn i_051bfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C02;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x12);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $051C02: movem.l (a7)+, d2-d3/a1
pub fn i_051c02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C06;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $051C06: rts
pub fn i_051c06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C08;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Collision_ProbeRange_051C08  ($051C08, 122 B, entity/collision_probes_051c08.s)
/// $051C08: movea.l $e(a0), a1
pub fn i_051c08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C0C;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0xE);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $051C0C: move.w $4(a0), d0
pub fn i_051c0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C10;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $051C10: move.w $8(a0), d1
pub fn i_051c10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C14;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $051C14: add.w $16(a0), d0
pub fn i_051c14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C18;
    cpu.ir = 0xD068;
    let ad2 = cpu.a[0].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $051C18: asr.w #$4, d0
pub fn i_051c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C1A;
    cpu.ir = 0xE840;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $051C1A: add.w $18(a0), d1
pub fn i_051c1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C1E;
    cpu.ir = 0xD268;
    let ad2 = cpu.a[0].wrapping_add(0x18);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $051C1E: asr.w #$4, d1
pub fn i_051c1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C20;
    cpu.ir = 0xE841;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $051C20: move.w d0, $1e(a0)
pub fn i_051c20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C24;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x1E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $051C24: move.w d1, $20(a0)
pub fn i_051c24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C28;
    cpu.ir = 0x3141;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write16(ad2, v1 as u16);
    12
}
/// $051C28: move.w d0, d2
pub fn i_051c28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C2A;
    cpu.ir = 0x3400;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $051C2A: move.w d1, d3
pub fn i_051c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C2C;
    cpu.ir = 0x3601;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $051C2C: move.w $1a(a0), d4
pub fn i_051c2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C30;
    cpu.ir = 0x3828;
    let ad2 = cpu.a[0].wrapping_add(0x1A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $051C30: move.w $1c(a0), d5
pub fn i_051c30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C34;
    cpu.ir = 0x3A28;
    let ad2 = cpu.a[0].wrapping_add(0x1C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $051C34: bsr.w $51d84
pub fn i_051c34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C38;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x51C38);
    cpu.pc = 0x51D84;
    18
}
/// $051C38: bcc.w $51c80
pub fn i_051c38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C3C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x51C80; return 10; }
    8
}
/// $051C3C: movem.w d0-d6, -(a7)
pub fn i_051c3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C40;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[6] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[5] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[4] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[3] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[2] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[1] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $051C40: movem.w d0/d4, -(a7)
pub fn i_051c40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C44;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[4] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $051C44: subq.w #$1, d4
pub fn i_051c44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C46;
    cpu.ir = 0x5344;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $051C46: lea.l $52(a0), a2
pub fn i_051c46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C4A;
    cpu.ir = 0x45E8;
    let ad1 = cpu.a[0].wrapping_add(0x52);
    cpu.a[2] = ad1;
    16
}
/// $051C4A: lea.l $32(a0), a3
pub fn i_051c4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C4E;
    cpu.ir = 0x47E8;
    let ad1 = cpu.a[0].wrapping_add(0x32);
    cpu.a[3] = ad1;
    16
}
/// $051C4E: bsr.w $51ba8
pub fn i_051c4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C52;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x51C52);
    cpu.pc = 0x51BA8;
    18
}
/// $051C52: move.w d0, d7
pub fn i_051c52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C54;
    cpu.ir = 0x3E00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $051C54: add.w $22(a0), d7
pub fn i_051c54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C58;
    cpu.ir = 0xDE68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $051C58: andi.w #$1f, d7
pub fn i_051c58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C5C;
    cpu.ir = 0x247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $051C5C: move.b $26(a0), (a2, d7.w)
pub fn i_051c5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C62;
    cpu.ir = 0x15A8;
    let ad2 = cpu.a[0].wrapping_add(0x26);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    bus.write8(ad4, v3 as u8);
    22
}
/// $051C62: move.b $27(a0), (a3, d7.w)
pub fn i_051c62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C68;
    cpu.ir = 0x17A8;
    let ad2 = cpu.a[0].wrapping_add(0x27);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[3].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    bus.write8(ad4, v3 as u8);
    22
}
/// $051C68: addq.w #$1, d0
pub fn i_051c68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C6A;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $051C6A: dbra d4, $51c4e
pub fn i_051c6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C6E;
    cpu.ir = 0x51CC;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x51C6E; return 12; }
    let n = (cpu.d[4] as u16).wrapping_sub(1);
    cpu.d[4] = (cpu.d[4] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x51C6E; return 14; }
    cpu.pc = 0x51C4E;
    10
}
/// $051C6E: movem.w (a7)+, d0/d4
pub fn i_051c6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C72;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[4] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $051C72: bsr.w $51de2
pub fn i_051c72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C76;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x51C76);
    cpu.pc = 0x51DE2;
    18
}
/// $051C76: movem.w (a7)+, d0-d6
pub fn i_051c76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C7A;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[1] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[2] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[3] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[4] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[5] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[6] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $051C7A: ori.b #$1, ccr
pub fn i_051c7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C7E;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $051C7E: rts
pub fn i_051c7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C80;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $051C80: rts
pub fn i_051c80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C82;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Collision_ProbeX_051C82  ($051C82, 116 B, entity/collision_probes_051c08.s)
/// $051C82: movea.l $e(a0), a1
pub fn i_051c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C86;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0xE);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $051C86: move.w $4(a0), d0
pub fn i_051c86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C8A;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $051C8A: move.w $8(a0), d1
pub fn i_051c8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C8E;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $051C8E: add.w $16(a0), d0
pub fn i_051c8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C92;
    cpu.ir = 0xD068;
    let ad2 = cpu.a[0].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $051C92: asr.w #$4, d0
pub fn i_051c92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C94;
    cpu.ir = 0xE840;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $051C94: add.w $18(a0), d1
pub fn i_051c94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C98;
    cpu.ir = 0xD268;
    let ad2 = cpu.a[0].wrapping_add(0x18);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $051C98: asr.w #$4, d1
pub fn i_051c98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C9A;
    cpu.ir = 0xE841;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $051C9A: cmp.w $1e(a0), d0
pub fn i_051c9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51C9E;
    cpu.ir = 0xB068;
    let ad2 = cpu.a[0].wrapping_add(0x1E);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $051C9E: beq.w $51cf0
pub fn i_051c9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CA2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x51CF0; return 10; }
    8
}
/// $051CA2: move.w d0, $1e(a0)
pub fn i_051ca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CA6;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x1E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $051CA6: move.w (a0), d2
pub fn i_051ca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CA8;
    cpu.ir = 0x3410;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $051CA8: bmi.w $51cb2
pub fn i_051ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CAC;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x51CB2; return 10; }
    8
}
/// $051CAC: add.w $1a(a0), d0
pub fn i_051cac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CB0;
    cpu.ir = 0xD068;
    let ad2 = cpu.a[0].wrapping_add(0x1A);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $051CB0: subq.w #$1, d0
pub fn i_051cb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CB2;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $051CB2: move.w d0, d2
pub fn i_051cb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CB4;
    cpu.ir = 0x3400;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $051CB4: move.w d1, d3
pub fn i_051cb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CB6;
    cpu.ir = 0x3601;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $051CB6: moveq #$1, d4
pub fn i_051cb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CB8;
    cpu.ir = 0x7801;
    cpu.d[4] = 0x1;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $051CB8: move.w $1c(a0), d5
pub fn i_051cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CBC;
    cpu.ir = 0x3A28;
    let ad2 = cpu.a[0].wrapping_add(0x1C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $051CBC: bsr.w $51d84
pub fn i_051cbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CC0;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x51CC0);
    cpu.pc = 0x51D84;
    18
}
/// $051CC0: bcc.w $51cf0
pub fn i_051cc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CC4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x51CF0; return 10; }
    8
}
/// $051CC4: movem.w d0-d6, -(a7)
pub fn i_051cc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CC8;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[6] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[5] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[4] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[3] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[2] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[1] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $051CC8: bsr.w $51ba8
pub fn i_051cc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CCC;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x51CCC);
    cpu.pc = 0x51BA8;
    18
}
/// $051CCC: move.w d0, d7
pub fn i_051ccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CCE;
    cpu.ir = 0x3E00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $051CCE: add.w $22(a0), d7
pub fn i_051cce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CD2;
    cpu.ir = 0xDE68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $051CD2: andi.w #$1f, d7
pub fn i_051cd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CD6;
    cpu.ir = 0x247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $051CD6: move.b $26(a0), $52(a0, d7.w)
pub fn i_051cd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CDC;
    cpu.ir = 0x11A8;
    let ad2 = cpu.a[0].wrapping_add(0x26);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x52).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    bus.write8(ad4, v3 as u8);
    22
}
/// $051CDC: move.b $27(a0), $32(a0, d7.w)
pub fn i_051cdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CE2;
    cpu.ir = 0x11A8;
    let ad2 = cpu.a[0].wrapping_add(0x27);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x32).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    bus.write8(ad4, v3 as u8);
    22
}
/// $051CE2: bsr.w $51de2
pub fn i_051ce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CE6;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x51CE6);
    cpu.pc = 0x51DE2;
    18
}
/// $051CE6: movem.w (a7)+, d0-d6
pub fn i_051ce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CEA;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[1] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[2] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[3] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[4] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[5] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[6] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $051CEA: ori.b #$1, ccr
pub fn i_051cea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CEE;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $051CEE: rts
pub fn i_051cee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CF0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $051CF0: andi.b #$fe, ccr
pub fn i_051cf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CF4;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $051CF4: rts
pub fn i_051cf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CF6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Collision_ProbeY_051CF6  ($051CF6, 142 B, entity/collision_probes_051c08.s)
/// $051CF6: movea.l $e(a0), a1
pub fn i_051cf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CFA;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0xE);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $051CFA: move.w $4(a0), d0
pub fn i_051cfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51CFE;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $051CFE: move.w $8(a0), d1
pub fn i_051cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D02;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $051D02: add.w $16(a0), d0
pub fn i_051d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D06;
    cpu.ir = 0xD068;
    let ad2 = cpu.a[0].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $051D06: asr.w #$4, d0
pub fn i_051d06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D08;
    cpu.ir = 0xE840;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $051D08: add.w $18(a0), d1
pub fn i_051d08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D0C;
    cpu.ir = 0xD268;
    let ad2 = cpu.a[0].wrapping_add(0x18);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $051D0C: asr.w #$4, d1
pub fn i_051d0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D0E;
    cpu.ir = 0xE841;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $051D0E: cmp.w $20(a0), d1
pub fn i_051d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D12;
    cpu.ir = 0xB268;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $051D12: beq.w $51d7e
pub fn i_051d12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D16;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x51D7E; return 10; }
    8
}
/// $051D16: move.w d1, $20(a0)
pub fn i_051d16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D1A;
    cpu.ir = 0x3141;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write16(ad2, v1 as u16);
    12
}
/// $051D1A: move.w $2(a0), d2
pub fn i_051d1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D1E;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $051D1E: bmi.w $51d28
pub fn i_051d1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D22;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x51D28; return 10; }
    8
}
/// $051D22: add.w $1c(a0), d1
pub fn i_051d22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D26;
    cpu.ir = 0xD268;
    let ad2 = cpu.a[0].wrapping_add(0x1C);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $051D26: subq.w #$1, d1
pub fn i_051d26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D28;
    cpu.ir = 0x5341;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $051D28: move.w d0, d2
pub fn i_051d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D2A;
    cpu.ir = 0x3400;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $051D2A: move.w d1, d3
pub fn i_051d2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D2C;
    cpu.ir = 0x3601;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $051D2C: move.w $1a(a0), d4
pub fn i_051d2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D30;
    cpu.ir = 0x3828;
    let ad2 = cpu.a[0].wrapping_add(0x1A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $051D30: moveq #$1, d5
pub fn i_051d30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D32;
    cpu.ir = 0x7A01;
    cpu.d[5] = 0x1;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $051D32: bsr.w $51d84
pub fn i_051d32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D36;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x51D36);
    cpu.pc = 0x51D84;
    18
}
/// $051D36: bcc.w $51d7e
pub fn i_051d36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D3A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x51D7E; return 10; }
    8
}
/// $051D3A: movem.w d0-d6, -(a7)
pub fn i_051d3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D3E;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[6] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[5] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[4] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[3] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[2] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[1] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $051D3E: movem.w d0/d4, -(a7)
pub fn i_051d3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D42;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[4] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $051D42: subq.w #$1, d4
pub fn i_051d42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D44;
    cpu.ir = 0x5344;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $051D44: lea.l $52(a0), a2
pub fn i_051d44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D48;
    cpu.ir = 0x45E8;
    let ad1 = cpu.a[0].wrapping_add(0x52);
    cpu.a[2] = ad1;
    16
}
/// $051D48: lea.l $32(a0), a3
pub fn i_051d48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D4C;
    cpu.ir = 0x47E8;
    let ad1 = cpu.a[0].wrapping_add(0x32);
    cpu.a[3] = ad1;
    16
}
/// $051D4C: bsr.w $51ba8
pub fn i_051d4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D50;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x51D50);
    cpu.pc = 0x51BA8;
    18
}
/// $051D50: move.w d0, d7
pub fn i_051d50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D52;
    cpu.ir = 0x3E00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $051D52: add.w $22(a0), d7
pub fn i_051d52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D56;
    cpu.ir = 0xDE68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $051D56: andi.w #$1f, d7
pub fn i_051d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D5A;
    cpu.ir = 0x247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $051D5A: move.b $26(a0), (a2, d7.w)
pub fn i_051d5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D60;
    cpu.ir = 0x15A8;
    let ad2 = cpu.a[0].wrapping_add(0x26);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    bus.write8(ad4, v3 as u8);
    22
}
/// $051D60: move.b $27(a0), (a3, d7.w)
pub fn i_051d60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D66;
    cpu.ir = 0x17A8;
    let ad2 = cpu.a[0].wrapping_add(0x27);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[3].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    bus.write8(ad4, v3 as u8);
    22
}
/// $051D66: addq.w #$1, d0
pub fn i_051d66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D68;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $051D68: dbra d4, $51d4c
pub fn i_051d68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D6C;
    cpu.ir = 0x51CC;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x51D6C; return 12; }
    let n = (cpu.d[4] as u16).wrapping_sub(1);
    cpu.d[4] = (cpu.d[4] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x51D6C; return 14; }
    cpu.pc = 0x51D4C;
    10
}
/// $051D6C: movem.w (a7)+, d0/d4
pub fn i_051d6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D70;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[4] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $051D70: bsr.w $51de2
pub fn i_051d70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D74;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x51D74);
    cpu.pc = 0x51DE2;
    18
}
/// $051D74: movem.w (a7)+, d0-d6
pub fn i_051d74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D78;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[1] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[2] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[3] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[4] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[5] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[6] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $051D78: ori.b #$1, ccr
pub fn i_051d78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D7C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $051D7C: rts
pub fn i_051d7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D7E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $051D7E: andi.b #$fe, ccr
pub fn i_051d7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D82;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $051D82: rts
pub fn i_051d82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D84;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== CellMap_ClipRectToWindow_051d84  ($051D84, 82 B, video/cellmap_fix_overlay_palfade_051aa4.s)
/// $051D84: add.w d2, d4
pub fn i_051d84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D86;
    cpu.ir = 0xD842;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $051D86: move.w (a1), d6
pub fn i_051d86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D88;
    cpu.ir = 0x3C11;
    let v2 = bus.read16(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $051D88: cmp.w d2, d6
pub fn i_051d88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D8A;
    cpu.ir = 0xBC42;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $051D8A: bcc.w $51d98
pub fn i_051d8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D8E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x51D98; return 10; }
    8
}
/// $051D8E: cmp.w d2, d4
pub fn i_051d8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D90;
    cpu.ir = 0xB842;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $051D90: bhi.w $51ddc
pub fn i_051d90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D94;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x51DDC; return 10; }
    8
}
/// $051D94: sub.w d2, d0
pub fn i_051d94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D96;
    cpu.ir = 0x9042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $051D96: clr.w d2
pub fn i_051d96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D98;
    cpu.ir = 0x4242;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $051D98: cmp.w d4, d6
pub fn i_051d98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D9A;
    cpu.ir = 0xBC44;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $051D9A: bcc.w $51da0
pub fn i_051d9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51D9E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x51DA0; return 10; }
    8
}
/// $051D9E: move.w d6, d4
pub fn i_051d9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DA0;
    cpu.ir = 0x3806;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $051DA0: sub.w d2, d4
pub fn i_051da0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DA2;
    cpu.ir = 0x9842;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $051DA2: beq.w $51ddc
pub fn i_051da2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DA6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x51DDC; return 10; }
    8
}
/// $051DA6: add.w d3, d5
pub fn i_051da6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DA8;
    cpu.ir = 0xDA43;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[5] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $051DA8: move.w $2(a1), d6
pub fn i_051da8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DAC;
    cpu.ir = 0x3C29;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $051DAC: cmp.w d3, d6
pub fn i_051dac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DAE;
    cpu.ir = 0xBC43;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $051DAE: bcc.w $51dbc
pub fn i_051dae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DB2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x51DBC; return 10; }
    8
}
/// $051DB2: cmp.w d3, d5
pub fn i_051db2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DB4;
    cpu.ir = 0xBA43;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[5] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $051DB4: bhi.w $51ddc
pub fn i_051db4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DB8;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x51DDC; return 10; }
    8
}
/// $051DB8: sub.w d3, d1
pub fn i_051db8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DBA;
    cpu.ir = 0x9243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $051DBA: clr.w d3
pub fn i_051dba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DBC;
    cpu.ir = 0x4243;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $051DBC: cmp.w d5, d6
pub fn i_051dbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DBE;
    cpu.ir = 0xBC45;
    let v1 = cpu.d[5] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $051DBE: bcc.w $51dc4
pub fn i_051dbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DC2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x51DC4; return 10; }
    8
}
/// $051DC2: move.w d6, d5
pub fn i_051dc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DC4;
    cpu.ir = 0x3A06;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $051DC4: sub.w d3, d5
pub fn i_051dc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DC6;
    cpu.ir = 0x9A43;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[5] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $051DC6: beq.w $51ddc
pub fn i_051dc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DCA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x51DDC; return 10; }
    8
}
/// $051DCA: andi.l #$ffff, d4
pub fn i_051dca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DD0;
    cpu.ir = 0x284;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[4] = r2;
    8
}
/// $051DD0: andi.l #$ffff, d5
pub fn i_051dd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DD6;
    cpu.ir = 0x285;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[5] = r2;
    8
}
// ===== SetC_051dd6  ($051DD6, 6 B, thunks/ccr_helpers.c)
/// $051DD6: ori.b #$1, ccr
pub fn i_051dd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DDA;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $051DDA: rts
pub fn i_051dda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DDC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearC_051ddc  ($051DDC, 6 B, thunks/ccr_helpers.c)
/// $051DDC: andi.b #$fe, ccr
pub fn i_051ddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DE0;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $051DE0: rts
pub fn i_051de0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DE2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== CellCommit_MMIO_051DE2  ($051DE2, 146 B, entity/collision_cell_apply_051ba8.s)
/// $051DE2: andi.l #$ffff, d4
pub fn i_051de2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DE8;
    cpu.ir = 0x284;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[4] = r2;
    8
}
/// $051DE8: andi.l #$ffff, d5
pub fn i_051de8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DEE;
    cpu.ir = 0x285;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[5] = r2;
    8
}
/// $051DEE: move.w $2(a1), d6
pub fn i_051dee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DF2;
    cpu.ir = 0x3C29;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $051DF2: mulu.w d6, d2
pub fn i_051df2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DF4;
    cpu.ir = 0xC4C6;
    let v1 = cpu.d[6] & 0xFFFF;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[2] & 0xFFFF);
    cpu.d[2] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $051DF4: add.w d3, d2
pub fn i_051df4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DF6;
    cpu.ir = 0xD443;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $051DF6: add.l d2, d2
pub fn i_051df6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DF8;
    cpu.ir = 0xD482;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[2] = r;
    4
}
/// $051DF8: add.l d2, d2
pub fn i_051df8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DFA;
    cpu.ir = 0xD482;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[2] = r;
    4
}
/// $051DFA: movea.l $4(a1), a3
pub fn i_051dfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51DFE;
    cpu.ir = 0x2669;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read32(ad2);
    cpu.a[3] = v3;
    16
}
/// $051DFE: lea.l (a3, d2.l), a3
pub fn i_051dfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E02;
    cpu.ir = 0x47F3;
    let ad1 = cpu.a[3].wrapping_add(0x0).wrapping_add(cpu.d[2]);
    cpu.a[3] = ad1;
    18
}
/// $051E02: sub.w d5, d6
pub fn i_051e02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E04;
    cpu.ir = 0x9C45;
    let v1 = cpu.d[5] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $051E04: add.w d6, d6
pub fn i_051e04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E06;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $051E06: add.w d6, d6
pub fn i_051e06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E08;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $051E08: subq.w #$1, d4
pub fn i_051e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E0A;
    cpu.ir = 0x5344;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $051E0A: subq.w #$1, d5
pub fn i_051e0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E0C;
    cpu.ir = 0x5345;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $051E0C: add.w $22(a0), d0
pub fn i_051e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E10;
    cpu.ir = 0xD068;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $051E10: add.w $24(a0), d1
pub fn i_051e10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E14;
    cpu.ir = 0xD268;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $051E14: movem.w d0-d1/d4-d5, -(a7)
pub fn i_051e14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E18;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[5] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[4] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[1] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $051E18: andi.w #$1f, d0
pub fn i_051e18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E1C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $051E1C: add.b $52(a0, d0.w), d1
pub fn i_051e1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E20;
    cpu.ir = 0xD230;
    let ad2 = cpu.a[0].wrapping_add(0x52).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[1] & 0xFF;
    let r = add_with_flags(cpu, h1, v3, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    14
}
/// $051E20: andi.w #$1f, d1
pub fn i_051e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E24;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $051E24: add.w $28(a0), d0
pub fn i_051e24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E28;
    cpu.ir = 0xD068;
    let ad2 = cpu.a[0].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $051E28: lsl.w #$6, d0
pub fn i_051e28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E2A;
    cpu.ir = 0xED48;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 6, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    18
}
/// $051E2A: addi.w #$40, d0
pub fn i_051e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E2E;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x40, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $051E2E: movea.w d0, a1
pub fn i_051e2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E30;
    cpu.ir = 0x3240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $051E30: add.w d1, d1
pub fn i_051e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E32;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $051E32: lea.l (a1, d1.w), a2
pub fn i_051e32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E36;
    cpu.ir = 0x45F1;
    let ad1 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    cpu.a[2] = ad1;
    18
}
/// $051E36: move.w a2, $106ee4.l
pub fn i_051e36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E3C;
    cpu.ir = 0x33CA;
    let v1 = cpu.a[2] as i16 as i32 as u32;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x106EE4, v1 as u16);
    16
}
/// $051E3C: move.w a2, $3c0000.l
pub fn i_051e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E42;
    cpu.ir = 0x33CA;
    let v1 = cpu.a[2] as i16 as i32 as u32;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0000, v1 as u16);
    16
}
/// $051E42: move.w (a3)+, $3c0002.l
pub fn i_051e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E48;
    cpu.ir = 0x33DB;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0002, v3 as u16);
    20
}
/// $051E48: addq.w #$1, a2
pub fn i_051e48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E4A;
    cpu.ir = 0x524A;
    cpu.a[2] = cpu.a[2].wrapping_add(1);
    8
}
/// $051E4A: move.w a2, $106ee4.l
pub fn i_051e4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E50;
    cpu.ir = 0x33CA;
    let v1 = cpu.a[2] as i16 as i32 as u32;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x106EE4, v1 as u16);
    16
}
/// $051E50: move.w a2, $3c0000.l
pub fn i_051e50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E56;
    cpu.ir = 0x33CA;
    let v1 = cpu.a[2] as i16 as i32 as u32;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0000, v1 as u16);
    16
}
/// $051E56: move.w (a3)+, $3c0002.l
pub fn i_051e56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E5C;
    cpu.ir = 0x33DB;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0002, v3 as u16);
    20
}
/// $051E5C: addq.w #$2, d1
pub fn i_051e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E5E;
    cpu.ir = 0x5441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $051E5E: andi.w #$3f, d1
pub fn i_051e5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E62;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x3F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $051E62: dbra d5, $51e32
pub fn i_051e62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E66;
    cpu.ir = 0x51CD;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x51E66; return 12; }
    let n = (cpu.d[5] as u16).wrapping_sub(1);
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x51E66; return 14; }
    cpu.pc = 0x51E32;
    10
}
/// $051E66: movem.w (a7)+, d0-d1/d4-d5
pub fn i_051e66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E6A;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[1] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[4] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[5] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $051E6A: addq.w #$1, d0
pub fn i_051e6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E6C;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $051E6C: adda.w d6, a3
pub fn i_051e6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E6E;
    cpu.ir = 0xD6C6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.a[3] = cpu.a[3].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $051E6E: dbra d4, $51e14
pub fn i_051e6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E72;
    cpu.ir = 0x51CC;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x51E72; return 12; }
    let n = (cpu.d[4] as u16).wrapping_sub(1);
    cpu.d[4] = (cpu.d[4] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x51E72; return 14; }
    cpu.pc = 0x51E14;
    10
}
/// $051E72: rts
pub fn i_051e72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E74;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== CellMap_BlitRectToLSPC_051e74  ($051E74, 90 B, video/cellmap_fix_overlay_palfade_051aa4.s)
/// $051E74: andi.l #$ffff, d4
pub fn i_051e74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E7A;
    cpu.ir = 0x284;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[4] = r2;
    8
}
/// $051E7A: andi.l #$ffff, d5
pub fn i_051e7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E80;
    cpu.ir = 0x285;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[5] = r2;
    8
}
/// $051E80: subq.w #$1, d4
pub fn i_051e80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E82;
    cpu.ir = 0x5344;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $051E82: subq.w #$1, d5
pub fn i_051e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E84;
    cpu.ir = 0x5345;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $051E84: add.w $22(a0), d0
pub fn i_051e84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E88;
    cpu.ir = 0xD068;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $051E88: add.w $24(a0), d1
pub fn i_051e88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E8C;
    cpu.ir = 0xD268;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $051E8C: clr.w d7
pub fn i_051e8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E8E;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $051E8E: movem.w d0-d1/d4-d5, -(a7)
pub fn i_051e8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E92;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[5] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[4] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[1] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $051E92: andi.w #$1f, d0
pub fn i_051e92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E96;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $051E96: add.b $52(a0, d0.w), d1
pub fn i_051e96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E9A;
    cpu.ir = 0xD230;
    let ad2 = cpu.a[0].wrapping_add(0x52).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[1] & 0xFF;
    let r = add_with_flags(cpu, h1, v3, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    14
}
/// $051E9A: andi.w #$1f, d1
pub fn i_051e9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51E9E;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $051E9E: add.w $28(a0), d0
pub fn i_051e9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51EA2;
    cpu.ir = 0xD068;
    let ad2 = cpu.a[0].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $051EA2: lsl.w #$6, d0
pub fn i_051ea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51EA4;
    cpu.ir = 0xED48;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 6, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    18
}
/// $051EA4: addi.w #$40, d0
pub fn i_051ea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51EA8;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x40, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $051EA8: add.w d1, d1
pub fn i_051ea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51EAA;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $051EAA: move.w d0, d6
pub fn i_051eaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51EAC;
    cpu.ir = 0x3C00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $051EAC: add.w d1, d6
pub fn i_051eac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51EAE;
    cpu.ir = 0xDC41;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $051EAE: movem.w d6-d7, $3c0000.l
pub fn i_051eae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51EB6;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[6] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[7] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $051EB6: addq.w #$2, d1
pub fn i_051eb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51EB8;
    cpu.ir = 0x5441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $051EB8: andi.w #$3f, d1
pub fn i_051eb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51EBC;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x3F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $051EBC: dbra d5, $51eaa
pub fn i_051ebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51EC0;
    cpu.ir = 0x51CD;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x51EC0; return 12; }
    let n = (cpu.d[5] as u16).wrapping_sub(1);
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x51EC0; return 14; }
    cpu.pc = 0x51EAA;
    10
}
/// $051EC0: movem.w (a7)+, d0-d1/d4-d5
pub fn i_051ec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51EC4;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[1] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[4] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[5] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $051EC4: addq.w #$1, d0
pub fn i_051ec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51EC6;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $051EC6: adda.w d6, a3
pub fn i_051ec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51EC8;
    cpu.ir = 0xD6C6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.a[3] = cpu.a[3].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $051EC8: dbra d4, $51e8e
pub fn i_051ec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51ECC;
    cpu.ir = 0x51CC;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x51ECC; return 12; }
    let n = (cpu.d[4] as u16).wrapping_sub(1);
    cpu.d[4] = (cpu.d[4] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x51ECC; return 14; }
    cpu.pc = 0x51E8E;
    10
}
/// $051ECC: rts
pub fn i_051ecc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51ECE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== CellMap_SetDirty_051ece  ($051ECE, 8 B, video/cellmap_fix_overlay_palfade_051aa4.s)
/// $051ECE: bset.b #$0, $c(a0)
pub fn i_051ece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51ED4;
    cpu.ir = 0x8E8;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[0].wrapping_add(0xC);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $051ED4: rts
pub fn i_051ed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51ED6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== CellMap_ClearSprites32_051ed6  ($051ED6, 44 B, video/cellmap_fix_overlay_palfade_051aa4.s)
/// $051ED6: bclr.b #$0, $c(a0)
pub fn i_051ed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51EDC;
    cpu.ir = 0x8A8;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[0].wrapping_add(0xC);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $051EDC: moveq #$1f, d1
pub fn i_051edc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51EDE;
    cpu.ir = 0x721F;
    cpu.d[1] = 0x1F;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $051EDE: move.w $28(a0), d0
pub fn i_051ede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51EE2;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $051EE2: addi.w #$8201, d0
pub fn i_051ee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51EE6;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x8201, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $051EE6: move.w d0, $106ee4.l
pub fn i_051ee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51EEC;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x106EE4, v1 as u16);
    16
}
/// $051EEC: move.w d0, $3c0000.l
pub fn i_051eec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51EF2;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0000, v1 as u16);
    16
}
/// $051EF2: move.w #$0, $3c0002.l
pub fn i_051ef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51EFA;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0002, v1 as u16);
    16
}
/// $051EFA: addq.w #$1, d0
pub fn i_051efa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51EFC;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $051EFC: dbra d1, $51ee6
pub fn i_051efc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F00;
    cpu.ir = 0x51C9;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x51F00; return 12; }
    let n = (cpu.d[1] as u16).wrapping_sub(1);
    cpu.d[1] = (cpu.d[1] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x51F00; return 14; }
    cpu.pc = 0x51EE6;
    10
}
/// $051F00: rts
pub fn i_051f00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F02;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== CellMap_ClearVramBlock_051f02  ($051F02, 46 B, video/cellmap_fix_overlay_palfade_051aa4.s)
/// $051F02: move.l #$3ff, d7
pub fn i_051f02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F08;
    cpu.ir = 0x2E3C;
    let v1: u32 = 0x3FF;
    cpu.d[7] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $051F08: move.w $28(a0), d0
pub fn i_051f08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F0C;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $051F0C: lsl.w #$6, d0
pub fn i_051f0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F0E;
    cpu.ir = 0xED48;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 6, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    18
}
/// $051F0E: addi.w #$40, d0
pub fn i_051f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F12;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x40, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $051F12: swap d0
pub fn i_051f12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F14;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $051F14: clr.w d0
pub fn i_051f14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F16;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $051F16: moveq #$1, d1
pub fn i_051f16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F18;
    cpu.ir = 0x7201;
    cpu.d[1] = 0x1;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $051F18: swap d1
pub fn i_051f18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F1A;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $051F1A: move.l d0, $3c0000.l
pub fn i_051f1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F20;
    cpu.ir = 0x23C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    bus.write32(0x3C0000, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $051F20: add.l d1, d0
pub fn i_051f20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F22;
    cpu.ir = 0xD081;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[0] = r;
    4
}
/// $051F22: move.l d0, $3c0000.l
pub fn i_051f22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F28;
    cpu.ir = 0x23C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    bus.write32(0x3C0000, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $051F28: add.l d1, d0
pub fn i_051f28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F2A;
    cpu.ir = 0xD081;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[0] = r;
    4
}
/// $051F2A: dbra d7, $51f1a
pub fn i_051f2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F2E;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x51F2E; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x51F2E; return 14; }
    cpu.pc = 0x51F1A;
    10
}
/// $051F2E: rts
pub fn i_051f2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F30;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TransformCommit_MMIO_051F30  ($051F30, 100 B, video/camera_sprite_helpers_043e8c.s)
/// $051F30: btst.b #$0, $c(a0)
pub fn i_051f30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F36;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $051F36: bne.w $51f3c
pub fn i_051f36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F3A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x51F3C; return 10; }
    8
}
/// $051F3A: rts
pub fn i_051f3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F3C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $051F3C: move.w $4(a0), d0
pub fn i_051f3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F40;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $051F40: move.w $22(a0), d1
pub fn i_051f40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F44;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $051F44: lsl.w #$4, d1
pub fn i_051f44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F46;
    cpu.ir = 0xE949;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $051F46: add.w d1, d0
pub fn i_051f46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F48;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $051F48: move.w d0, d1
pub fn i_051f48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F4A;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $051F4A: lsr.w #$4, d1
pub fn i_051f4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F4C;
    cpu.ir = 0xE849;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $051F4C: subq.b #$1, d1
pub fn i_051f4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F4E;
    cpu.ir = 0x5301;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $051F4E: andi.w #$1f, d1
pub fn i_051f4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F52;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $051F52: move.w d1, $2a(a0)
pub fn i_051f52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F56;
    cpu.ir = 0x3141;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $051F56: addi.w #$16, d1
pub fn i_051f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F5A;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x16, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $051F5A: andi.w #$1f, d1
pub fn i_051f5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F5E;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $051F5E: move.w d1, $2c(a0)
pub fn i_051f5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F62;
    cpu.ir = 0x3141;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $051F62: neg.w d0
pub fn i_051f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F64;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $051F64: lsl.w #$7, d0
pub fn i_051f64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F66;
    cpu.ir = 0xEF48;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 7, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    20
}
/// $051F66: move.w d0, $2e(a0)
pub fn i_051f66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F6A;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $051F6A: move.w $8(a0), d0
pub fn i_051f6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F6E;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $051F6E: move.w $24(a0), d1
pub fn i_051f6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F72;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $051F72: lsl.w #$4, d1
pub fn i_051f72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F74;
    cpu.ir = 0xE949;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $051F74: add.w d1, d0
pub fn i_051f74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F76;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $051F76: lsl.w #$7, d0
pub fn i_051f76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F78;
    cpu.ir = 0xEF48;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 7, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    20
}
/// $051F78: move.w d0, $30(a0)
pub fn i_051f78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F7C;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $051F7C: movem.l a0/a6, -(a7)
pub fn i_051f7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F80;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $051F80: movea.l a0, a6
pub fn i_051f80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F82;
    cpu.ir = 0x2C48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[6] = v1;
    4
}
/// $051F82: lea.l $51f94(pc), a0
pub fn i_051f82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F86;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x51F94;
    16
}
/// $051F86: jsr $1f4a.l
pub fn i_051f86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F8C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x51F8C);
    cpu.pc = 0x1F4A;
    16
}
/// $051F8C: movea.l a6, a0
pub fn i_051f8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F8E;
    cpu.ir = 0x204E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $051F8E: movem.l (a7)+, a0/a6
pub fn i_051f8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F92;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $051F92: rts
pub fn i_051f92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F94;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TileMap_HandlerInline_051F94  ($051F94, 158 B, entity/collision_probes_051c08.s)
/// $051F94: move.w #$8201, d0
pub fn i_051f94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F98;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8201;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $051F98: add.w $28(a6), d0
pub fn i_051f98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51F9C;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $051F9C: move.w $2a(a6), d1
pub fn i_051f9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51FA0;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $051FA0: move.w $30(a6), d2
pub fn i_051fa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51FA4;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $051FA4: move.w d0, d5
pub fn i_051fa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51FA6;
    cpu.ir = 0x3A00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $051FA6: add.w d1, d5
pub fn i_051fa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51FA8;
    cpu.ir = 0xDA41;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[5] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $051FA8: move.w d5, $106ee4.l
pub fn i_051fa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51FAE;
    cpu.ir = 0x33C5;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x106EE4, v1 as u16);
    16
}
/// $051FAE: move.w d5, $3c0000.l
pub fn i_051fae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51FB4;
    cpu.ir = 0x33C5;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0000, v1 as u16);
    16
}
/// $051FB4: move.w #$0, $3c0002.l
pub fn i_051fb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51FBC;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0002, v1 as u16);
    16
}
/// $051FBC: addq.b #$1, d1
pub fn i_051fbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51FBE;
    cpu.ir = 0x5201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $051FBE: andi.b #$1f, d1
pub fn i_051fbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51FC2;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0x1F) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $051FC2: moveq #$b, d4
pub fn i_051fc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51FC4;
    cpu.ir = 0x780B;
    cpu.d[4] = 0xB;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $051FC4: move.w d1, d3
pub fn i_051fc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51FC6;
    cpu.ir = 0x3601;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $051FC6: lsl.w d4, d3
pub fn i_051fc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51FC8;
    cpu.ir = 0xE96B;
    let cnt = cpu.d[4] & 0x3F;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, cnt, Size::Word, 1, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    6 + 2 * cnt
}
/// $051FC8: add.w $2e(a6), d3
pub fn i_051fc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51FCC;
    cpu.ir = 0xD66E;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $051FCC: cmp.w $2c(a6), d1
pub fn i_051fcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51FD0;
    cpu.ir = 0xB26E;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $051FD0: beq.w $52018
pub fn i_051fd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51FD4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x52018; return 10; }
    8
}
/// $051FD4: move.w d0, d5
pub fn i_051fd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51FD6;
    cpu.ir = 0x3A00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $051FD6: add.w d1, d5
pub fn i_051fd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51FD8;
    cpu.ir = 0xDA41;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[5] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $051FD8: move.w d5, $106ee4.l
pub fn i_051fd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51FDE;
    cpu.ir = 0x33C5;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x106EE4, v1 as u16);
    16
}
/// $051FDE: move.w d5, $3c0000.l
pub fn i_051fde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51FE4;
    cpu.ir = 0x33C5;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0000, v1 as u16);
    16
}
/// $051FE4: move.b $52(a6, d1.w), d6
pub fn i_051fe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51FE8;
    cpu.ir = 0x1C36;
    let ad2 = cpu.a[6].wrapping_add(0x52).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    14
}
/// $051FE8: lsl.w d4, d6
pub fn i_051fe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51FEA;
    cpu.ir = 0xE96E;
    let cnt = cpu.d[4] & 0x3F;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, cnt, Size::Word, 1, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    6 + 2 * cnt
}
/// $051FEA: add.w d2, d6
pub fn i_051fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51FEC;
    cpu.ir = 0xDC42;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $051FEC: or.b $32(a6, d1.w), d6
pub fn i_051fec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51FF0;
    cpu.ir = 0x8C36;
    let ad2 = cpu.a[6].wrapping_add(0x32).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    let r = (v3 | cpu.d[6]) & 0xFF;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    14
}
/// $051FF0: move.w d6, $3c0002.l
pub fn i_051ff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51FF6;
    cpu.ir = 0x33C6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0002, v1 as u16);
    16
}
/// $051FF6: addi.w #$200, d5
pub fn i_051ff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51FFA;
    cpu.ir = 0x645;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x200, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $051FFA: move.w d5, $106ee4.l
pub fn i_051ffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52000;
    cpu.ir = 0x33C5;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x106EE4, v1 as u16);
    16
}
/// $052000: move.w d5, $3c0000.l
pub fn i_052000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52006;
    cpu.ir = 0x33C5;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0000, v1 as u16);
    16
}
/// $052006: addq.b #$1, d1
pub fn i_052006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52008;
    cpu.ir = 0x5201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $052008: andi.w #$1f, d1
pub fn i_052008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5200C;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05200C: move.w d3, $3c0002.l
pub fn i_05200c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52012;
    cpu.ir = 0x33C3;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0002, v1 as u16);
    16
}
/// $052012: addi.w #$800, d3
pub fn i_052012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52016;
    cpu.ir = 0x643;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x800, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $052016: bra.b $51fcc
pub fn i_052016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52018;
    cpu.ir = 0x60B4;
    cpu.pc = 0x51FCC;
    10
}
/// $052018: move.w d0, d4
pub fn i_052018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5201A;
    cpu.ir = 0x3800;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05201A: add.w d1, d4
pub fn i_05201a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5201C;
    cpu.ir = 0xD841;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05201C: move.w d4, $106ee4.l
pub fn i_05201c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52022;
    cpu.ir = 0x33C4;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x106EE4, v1 as u16);
    16
}
/// $052022: move.w d4, $3c0000.l
pub fn i_052022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52028;
    cpu.ir = 0x33C4;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0000, v1 as u16);
    16
}
/// $052028: move.w #$0, $3c0002.l
pub fn i_052028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52030;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0002, v1 as u16);
    16
}
/// $052030: rts
pub fn i_052030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52032;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Scheduler_CompareField10_052032  ($052032, 16 B, video/cellmap_fix_overlay_palfade_051aa4.s)
/// $052032: movea.l $8(a6), a1
pub fn i_052032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52036;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $052036: move.b $10(a6), d0
pub fn i_052036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5203A;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05203A: cmp.b $10(a1), d0
pub fn i_05203a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5203E;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $05203E: bcs.w $52048
pub fn i_05203e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52042;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x52048; return 10; }
    8
}
// ===== ClearXN_052042  ($052042, 6 B, thunks/ccr_helpers.c)
/// $052042: andi.b #$ee, ccr
pub fn i_052042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52046;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $052046: rts
pub fn i_052046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52048;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_052048  ($052048, 6 B, thunks/ccr_helpers.c)
/// $052048: ori.b #$11, ccr
pub fn i_052048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5204C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05204C: rts
pub fn i_05204c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5204E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Stub_0005204E  ($05204E, 2 B, thunks/trivial_rts.c)
/// $05204E: rts
pub fn i_05204e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52050;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FixOverlay_DrawCreditsOrFree_052050  ($052050, 146 B, video/cellmap_fix_overlay_palfade_051aa4.s)
/// $052050: tst.b $10fd82.l
pub fn i_052050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52056;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FD82) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $052056: beq.w $52068
pub fn i_052056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5205A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x52068; return 10; }
    8
}
/// $05205A: move.b $10fd91.l, d0
pub fn i_05205a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52060;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10FD91) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $052060: andi.b #$2, d0
pub fn i_052060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52064;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $052064: bne.w $52142
pub fn i_052064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52068;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x52142; return 10; }
    8
}
/// $052068: btst.b #$1, $1081be.l
pub fn i_052068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52070;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((1) % 8);
    let v2 = bus.read8(0x1081BE) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $052070: bne.w $52144
pub fn i_052070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52074;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x52144; return 10; }
    8
}
/// $052074: btst.b #$0, $1081be.l
pub fn i_052074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5207C;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((0) % 8);
    let v2 = bus.read8(0x1081BE) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $05207C: beq.w $52142
pub fn i_05207c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52080;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x52142; return 10; }
    8
}
/// $052080: bsr.w $521b2
pub fn i_052080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52084;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x52084);
    cpu.pc = 0x521B2;
    18
}
/// $052084: tst.b $10fd82.l
pub fn i_052084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5208A;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FD82) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $05208A: beq.w $520a6
pub fn i_05208a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5208E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x520A6; return 10; }
    8
}
/// $05208E: lea.l $10fe00.l, a1
pub fn i_05208e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52094;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x10FE00;
    20
}
/// $052094: tst.w $10fe80.l
pub fn i_052094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5209A;
    cpu.ir = 0x4A79;
    let v2 = bus.read16(0x10FE80) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    16
}
/// $05209A: bne.b $520ac
pub fn i_05209a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5209C;
    cpu.ir = 0x6610;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x520AC; return 10; }
    8
}
/// $05209C: lea.l $d00034.l, a1
pub fn i_05209c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x520A2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xD00034;
    20
}
/// $0520A2: bra.w $520ac
pub fn i_0520a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x520A6;
    cpu.ir = 0x6000;
    cpu.pc = 0x520AC;
    10
}
/// $0520A6: lea.l $1081bf.l, a1
pub fn i_0520a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x520AC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1081BF;
    20
}
/// $0520AC: move.w #$2300, d1
pub fn i_0520ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x520B0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0520B0: btst.b #$4, $1081be.l
pub fn i_0520b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x520B8;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((4) % 8);
    let v2 = bus.read8(0x1081BE) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $0520B8: beq.w $520c0
pub fn i_0520b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x520BC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x520C0; return 10; }
    8
}
/// $0520BC: move.w #$2300, d1
pub fn i_0520bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x520C0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0520C0: tst.b $10fd82.l
pub fn i_0520c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x520C6;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FD82) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $0520C6: beq.w $520d4
pub fn i_0520c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x520CA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x520D4; return 10; }
    8
}
/// $0520CA: cmpi.b #$1, $10fd83.l
pub fn i_0520ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x520D2;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10FD83) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $0520D2: bne.b $520de
pub fn i_0520d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x520D4;
    cpu.ir = 0x660A;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x520DE; return 10; }
    8
}
/// $0520D4: move.w #$707d, d0
pub fn i_0520d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x520D8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x707D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0520D8: bsr.w $520e2
pub fn i_0520d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x520DC;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x520DC);
    cpu.pc = 0x520E2;
    18
}
/// $0520DC: addq.l #$1, a1
pub fn i_0520dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x520DE;
    cpu.ir = 0x5289;
    cpu.a[1] = cpu.a[1].wrapping_add(1);
    8
}
/// $0520DE: move.w #$737d, d0
pub fn i_0520de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x520E2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x737D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== FixOverlay_PutCreditsLine_0520e2  ($0520E2, 80 B, video/cellmap_fix_overlay_palfade_051aa4.s)
/// $0520E2: lea.l $52230(pc), a2
pub fn i_0520e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x520E6;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0x52230;
    16
}
/// $0520E6: moveq #$5, d7
pub fn i_0520e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x520E8;
    cpu.ir = 0x7E05;
    cpu.d[7] = 0x5;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0520E8: move.b (a2)+, d1
pub fn i_0520e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x520EA;
    cpu.ir = 0x121A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    8
}
/// $0520EA: movem.w d0-d1, $3c0000.l
pub fn i_0520ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x520F2;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0520F2: addi.w #$20, d0
pub fn i_0520f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x520F6;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0520F6: dbra d7, $520e8
pub fn i_0520f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x520FA;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x520FA; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x520FA; return 14; }
    cpu.pc = 0x520E8;
    10
}
/// $0520FA: move.b (a2)+, d1
pub fn i_0520fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x520FC;
    cpu.ir = 0x121A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    8
}
/// $0520FC: move.b (a1), d2
pub fn i_0520fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x520FE;
    cpu.ir = 0x1411;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $0520FE: cmpi.b #$2, d2
pub fn i_0520fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52102;
    cpu.ir = 0xC02;
    let v1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $052102: bcc.w $5210a
pub fn i_052102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52106;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5210A; return 10; }
    8
}
/// $052106: move.b #$20, d1
pub fn i_052106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5210A;
    cpu.ir = 0x123C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05210A: movem.w d0-d1, $3c0000.l
pub fn i_05210a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52112;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $052112: addi.w #$20, d0
pub fn i_052112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52116;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $052116: move.b (a2)+, d1
pub fn i_052116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52118;
    cpu.ir = 0x121A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    8
}
/// $052118: movem.w d0-d1, $3c0000.l
pub fn i_052118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52120;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $052120: addi.w #$20, d0
pub fn i_052120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52124;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $052124: move.b d2, d1
pub fn i_052124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52126;
    cpu.ir = 0x1202;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $052126: lsr.b #$4, d1
pub fn i_052126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52128;
    cpu.ir = 0xE809;
    let h1 = cpu.d[1] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, false);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    14
}
/// $052128: bsr.w $52132
pub fn i_052128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5212C;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5212C);
    cpu.pc = 0x52132;
    18
}
/// $05212C: andi.b #$f, d2
pub fn i_05212c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52130;
    cpu.ir = 0x202;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $052130: move.b d2, d1
pub fn i_052130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52132;
    cpu.ir = 0x1202;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
// ===== FixOverlay_PutDigit_052132  ($052132, 128 B, video/cellmap_fix_overlay_palfade_051aa4.s)
/// $052132: addi.b #$30, d1
pub fn i_052132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52136;
    cpu.ir = 0x601;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x30, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $052136: movem.w d0-d1, $3c0000.l
pub fn i_052136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5213E;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $05213E: addi.w #$20, d0
pub fn i_05213e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52142;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $052142: rts
pub fn i_052142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52144;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $052144: move.w #$701d, d0
pub fn i_052144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52148;
    cpu.ir = 0x303C;
    let v1: u32 = 0x701D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $052148: moveq #$f, d7
pub fn i_052148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5214A;
    cpu.ir = 0x7E0F;
    cpu.d[7] = 0xF;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05214A: move.w #$20, d1
pub fn i_05214a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5214E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05214E: ori.w #$2300, d1
pub fn i_05214e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52152;
    cpu.ir = 0x41;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 | 0x2300) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $052152: btst.b #$4, $1081be.l
pub fn i_052152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5215A;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((4) % 8);
    let v2 = bus.read8(0x1081BE) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $05215A: beq.w $52166
pub fn i_05215a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5215E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x52166; return 10; }
    8
}
/// $05215E: move.w #$20, d1
pub fn i_05215e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52162;
    cpu.ir = 0x323C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $052162: ori.w #$2300, d1
pub fn i_052162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52166;
    cpu.ir = 0x41;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 | 0x2300) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $052166: movem.w d0-d1, $3c0000.l
pub fn i_052166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5216E;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $05216E: addi.w #$20, d0
pub fn i_05216e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52172;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $052172: dbra d7, $52166
pub fn i_052172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52176;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x52176; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x52176; return 14; }
    cpu.pc = 0x52166;
    10
}
/// $052176: move.w #$72fd, d0
pub fn i_052176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5217A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x72FD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05217A: moveq #$10, d7
pub fn i_05217a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5217C;
    cpu.ir = 0x7E10;
    cpu.d[7] = 0x10;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05217C: move.w #$20, d1
pub fn i_05217c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52180;
    cpu.ir = 0x323C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $052180: ori.w #$2300, d1
pub fn i_052180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52184;
    cpu.ir = 0x41;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 | 0x2300) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $052184: btst.b #$4, $1081be.l
pub fn i_052184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5218C;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((4) % 8);
    let v2 = bus.read8(0x1081BE) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $05218C: beq.w $52198
pub fn i_05218c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52190;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x52198; return 10; }
    8
}
/// $052190: move.w #$20, d1
pub fn i_052190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52194;
    cpu.ir = 0x323C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $052194: ori.w #$2300, d1
pub fn i_052194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52198;
    cpu.ir = 0x41;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 | 0x2300) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $052198: movem.w d0-d1, $3c0000.l
pub fn i_052198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x521A0;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0521A0: addi.w #$20, d0
pub fn i_0521a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x521A4;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0521A4: dbra d7, $52198
pub fn i_0521a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x521A8;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x521A8; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x521A8; return 14; }
    cpu.pc = 0x52198;
    10
}
/// $0521A8: andi.b #$fc, $1081be.l
pub fn i_0521a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x521B0;
    cpu.ir = 0x239;
    let v1 = bus.read8(0x1081BE) as u32;
    let r2 = (v1 & 0xFC) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x1081BE, r2 as u8);
    20
}
/// $0521B0: rts
pub fn i_0521b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x521B2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FixOverlay_ClearCreditsArea_0521b2  ($0521B2, 126 B, video/cellmap_fix_overlay_palfade_051aa4.s)
/// $0521B2: move.w #$701d, d0
pub fn i_0521b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x521B6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x701D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0521B6: move.w #$2, d7
pub fn i_0521b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x521BA;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0521BA: move.w #$20, d1
pub fn i_0521ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x521BE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0521BE: ori.w #$2300, d1
pub fn i_0521be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x521C2;
    cpu.ir = 0x41;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 | 0x2300) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0521C2: btst.b #$4, $1081be.l
pub fn i_0521c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x521CA;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((4) % 8);
    let v2 = bus.read8(0x1081BE) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $0521CA: beq.w $521d6
pub fn i_0521ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x521CE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x521D6; return 10; }
    8
}
/// $0521CE: move.w #$20, d1
pub fn i_0521ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x521D2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0521D2: ori.w #$2300, d1
pub fn i_0521d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x521D6;
    cpu.ir = 0x41;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 | 0x2300) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0521D6: movem.w d0-d1, $3c0000.l
pub fn i_0521d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x521DE;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0521DE: addi.w #$20, d0
pub fn i_0521de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x521E2;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0521E2: dbra d7, $521d6
pub fn i_0521e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x521E6;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x521E6; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x521E6; return 14; }
    cpu.pc = 0x521D6;
    10
}
/// $0521E6: move.w #$719d, d0
pub fn i_0521e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x521EA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x719D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0521EA: move.w #$3, d7
pub fn i_0521ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x521EE;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0521EE: movem.w d0-d1, $3c0000.l
pub fn i_0521ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x521F6;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0521F6: addi.w #$20, d0
pub fn i_0521f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x521FA;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0521FA: dbra d7, $521ee
pub fn i_0521fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x521FE;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x521FE; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x521FE; return 14; }
    cpu.pc = 0x521EE;
    10
}
/// $0521FE: move.w #$72fd, d0
pub fn i_0521fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52202;
    cpu.ir = 0x303C;
    let v1: u32 = 0x72FD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $052202: move.w #$3, d7
pub fn i_052202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52206;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $052206: movem.w d0-d1, $3c0000.l
pub fn i_052206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5220E;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $05220E: addi.w #$20, d0
pub fn i_05220e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52212;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $052212: dbra d7, $52206
pub fn i_052212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52216;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x52216; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x52216; return 14; }
    cpu.pc = 0x52206;
    10
}
/// $052216: move.w #$749d, d0
pub fn i_052216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5221A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x749D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05221A: move.w #$3, d7
pub fn i_05221a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5221E;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05221E: movem.w d0-d1, $3c0000.l
pub fn i_05221e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52226;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $052226: addi.w #$20, d0
pub fn i_052226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5222A;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05222A: dbra d7, $5221e
pub fn i_05222a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5222E;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5222E; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5222E; return 14; }
    cpu.pc = 0x5221E;
    10
}
/// $05222E: rts
pub fn i_05222e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52230;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $052238: move.l -(a0), d0
pub fn i_052238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5223A;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $05223A: tst.b $10fd82.l
pub fn i_05223a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52240;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FD82) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $052240: beq.w $522ac
pub fn i_052240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52244;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x522AC; return 10; }
    8
}
/// $052244: move.b $10fd91.l, d0
pub fn i_052244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5224A;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10FD91) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $05224A: andi.b #$1, d0
pub fn i_05224a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5224E;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05224E: bne.w $522ac
pub fn i_05224e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52252;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x522AC; return 10; }
    8
}
/// $052252: btst.b #$3, $1081be.l
pub fn i_052252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5225A;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((3) % 8);
    let v2 = bus.read8(0x1081BE) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $05225A: bne.w $522ae
pub fn i_05225a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5225E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x522AE; return 10; }
    8
}
/// $05225E: btst.b #$2, $1081be.l
pub fn i_05225e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52266;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((2) % 8);
    let v2 = bus.read8(0x1081BE) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $052266: beq.w $522ac
pub fn i_052266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5226A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x522AC; return 10; }
    8
}
/// $05226A: move.w #$721d, d0
pub fn i_05226a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5226E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x721D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05226E: lea.l $522ea(pc), a2
pub fn i_05226e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52272;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0x522EA;
    16
}
/// $052272: moveq #$5, d7
pub fn i_052272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52274;
    cpu.ir = 0x7E05;
    cpu.d[7] = 0x5;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $052274: move.w #$2300, d1
pub fn i_052274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52278;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $052278: btst.b #$4, $1081be.l
pub fn i_052278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52280;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((4) % 8);
    let v2 = bus.read8(0x1081BE) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $052280: beq.w $52288
pub fn i_052280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52284;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x52288; return 10; }
    8
}
/// $052284: move.w #$2300, d1
pub fn i_052284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52288;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $052288: move.b (a2)+, d1
pub fn i_052288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5228A;
    cpu.ir = 0x121A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05228A: movem.w d0-d1, $3c0000.l
pub fn i_05228a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52292;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $052292: addi.w #$20, d0
pub fn i_052292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52296;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $052296: dbra d7, $52288
pub fn i_052296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5229A;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5229A; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5229A; return 14; }
    cpu.pc = 0x52288;
    10
}
/// $05229A: move.b $10fd8b.l, d1
pub fn i_05229a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x522A0;
    cpu.ir = 0x1239;
    let v2 = bus.read8(0x10FD8B) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    16
}
/// $0522A0: addi.b #$31, d1
pub fn i_0522a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x522A4;
    cpu.ir = 0x601;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x31, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0522A4: movem.w d0-d1, $3c0000.l
pub fn i_0522a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x522AC;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0522AC: rts
pub fn i_0522ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x522AE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0522AE: move.w #$721d, d0
pub fn i_0522ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x522B2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x721D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0522B2: moveq #$7, d7
pub fn i_0522b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x522B4;
    cpu.ir = 0x7E07;
    cpu.d[7] = 0x7;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0522B4: move.w #$20, d1
pub fn i_0522b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x522B8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0522B8: ori.w #$2300, d1
pub fn i_0522b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x522BC;
    cpu.ir = 0x41;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 | 0x2300) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0522BC: btst.b #$4, $1081be.l
pub fn i_0522bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x522C4;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((4) % 8);
    let v2 = bus.read8(0x1081BE) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $0522C4: beq.w $522d0
pub fn i_0522c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x522C8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x522D0; return 10; }
    8
}
/// $0522C8: move.w #$20, d1
pub fn i_0522c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x522CC;
    cpu.ir = 0x323C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0522CC: ori.w #$2300, d1
pub fn i_0522cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x522D0;
    cpu.ir = 0x41;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 | 0x2300) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0522D0: movem.w d0-d1, $3c0000.l
pub fn i_0522d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x522D8;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0522D8: addi.w #$20, d0
pub fn i_0522d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x522DC;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0522DC: dbra d7, $522d0
pub fn i_0522dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x522E0;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x522E0; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x522E0; return 14; }
    cpu.pc = 0x522D0;
    10
}
/// $0522E0: andi.b #$f3, $1081be.l
pub fn i_0522e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x522E8;
    cpu.ir = 0x239;
    let v1 = bus.read8(0x1081BE) as u32;
    let r2 = (v1 & 0xF3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x1081BE, r2 as u8);
    20
}
/// $0522E8: rts
pub fn i_0522e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x522EA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FixOverlay_PutPauseTail_0522ea  ($0522EA, 2 B, video/cellmap_fix_overlay_palfade_051aa4.s)
/// $0522EA: move.l a3, (a0)
pub fn i_0522ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x522EC;
    cpu.ir = 0x208B;
    let v1 = cpu.a[3] & 0xFFFFFFFF;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Str_PauseTiles_0522ec  ($0522EC, 18 B, video/cellmap_fix_overlay_palfade_051aa4.s)
/// $0522EC: ?
pub fn i_0522ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x522EE;
    cpu.ir = 0x8C8D;
    let v1 = cpu.a[5] & 0xFFFFFFFF;
    let r = (v1 | cpu.d[6]) & 0xFFFFFFFF;
    cpu.d[6] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0522EE: ?
pub fn i_0522ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x522F0;
    cpu.ir = 0x8E8F;
    let v1 = cpu.a[7] & 0xFFFFFFFF;
    let r = (v1 | cpu.d[7]) & 0xFFFFFFFF;
    cpu.d[7] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0522F0: movea.l #$7226, a1
pub fn i_0522f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x522F6;
    cpu.ir = 0x227C;
    let v1: u32 = 0x7226;
    cpu.a[1] = v1;
    4
}
/// $0522F6: lea.l $5231c(pc), a2
pub fn i_0522f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x522FA;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0x5231C;
    16
}
/// $0522FA: move.w #$300, d0
pub fn i_0522fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x522FE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_0522fe  ($0522FE, 8 B, thunks/jsr_abs_thunks.c)
/// $0522FE: jsr $5dad8.l
pub fn i_0522fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52304;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52304);
    cpu.pc = 0x5DAD8;
    16
}
/// $052304: rts
pub fn i_052304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52306;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FixOverlay_ClearPause_052306  ($052306, 14 B, video/cellmap_fix_overlay_palfade_051aa4.s)
/// $052306: movea.l #$7226, a1
pub fn i_052306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5230C;
    cpu.ir = 0x227C;
    let v1: u32 = 0x7226;
    cpu.a[1] = v1;
    4
}
/// $05230C: lea.l $52322(pc), a2
pub fn i_05230c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52310;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0x52322;
    16
}
/// $052310: move.w #$0, d0
pub fn i_052310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52314;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_052314  ($052314, 8 B, thunks/jsr_abs_thunks.c)
/// $052314: jsr $5dad8.l
pub fn i_052314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5231A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5231A);
    cpu.pc = 0x5DAD8;
    16
}
/// $05231A: rts
pub fn i_05231a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5231C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $052328: moveq #$0, d0
pub fn i_052328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5232A;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05232A: tst.b $10fd82.l
pub fn i_05232a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52330;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FD82) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $052330: bne.w $5233a
pub fn i_052330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52334;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5233A; return 10; }
    8
}
/// $052334: bset.b #$2, d0
pub fn i_052334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52338;
    cpu.ir = 0x8C0;
    let mk: u32 = 1u32 << ((2) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 | mk;
    cpu.d[0] = r2;
    8
}
/// $052338: rts
pub fn i_052338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5233A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05233A: move.b #$1, $10fdb0.l
pub fn i_05233a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52342;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10FDB0, v1 as u8);
    16
}
/// $052342: move.b #$1, $10fdb1.l
pub fn i_052342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5234A;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10FDB1, v1 as u8);
    16
}
/// $05234A: move.b #$0, $10fdb2.l
pub fn i_05234a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52352;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10FDB2, v1 as u8);
    16
}
/// $052352: move.b #$0, $10fdb3.l
pub fn i_052352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5235A;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10FDB3, v1 as u8);
    16
}
/// $05235A: move.l d0, -(a7)
pub fn i_05235a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5235C;
    cpu.ir = 0x2F00;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $05235C: jsr $c00450.l
pub fn i_05235c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52362;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52362);
    cpu.pc = 0xC00450;
    16
}
/// $052362: move.l (a7)+, d0
pub fn i_052362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52364;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $052364: tst.b $10fdb0.l
pub fn i_052364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5236A;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FDB0) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $05236A: beq.w $52372
pub fn i_05236a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5236E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x52372; return 10; }
    8
}
/// $05236E: bset.b #$0, d0
pub fn i_05236e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52372;
    cpu.ir = 0x8C0;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 | mk;
    cpu.d[0] = r2;
    8
}
/// $052372: tst.b $10fdb1.l
pub fn i_052372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52378;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FDB1) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $052378: beq.w $52380
pub fn i_052378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5237C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x52380; return 10; }
    8
}
/// $05237C: bset.b #$1, d0
pub fn i_05237c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52380;
    cpu.ir = 0x8C0;
    let mk: u32 = 1u32 << ((1) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 | mk;
    cpu.d[0] = r2;
    8
}
/// $052380: rts
pub fn i_052380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52382;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Scheduler_CompareField10_052382  ($052382, 16 B, video/cellmap_fix_overlay_palfade_051aa4.s)
/// $052382: movea.l $8(a6), a1
pub fn i_052382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52386;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $052386: move.b $10(a6), d0
pub fn i_052386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5238A;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05238A: cmp.b $10(a1), d0
pub fn i_05238a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5238E;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $05238E: bcs.w $52398
pub fn i_05238e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52392;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x52398; return 10; }
    8
}
// ===== ClearXN_052392  ($052392, 6 B, thunks/ccr_helpers.c)
/// $052392: andi.b #$ee, ccr
pub fn i_052392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52396;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $052396: rts
pub fn i_052396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52398;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_052398  ($052398, 6 B, thunks/ccr_helpers.c)
/// $052398: ori.b #$11, ccr
pub fn i_052398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5239C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05239C: rts
pub fn i_05239c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5239E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_SpawnAndPublishD0At70_05239E  ($05239E, 20 B, entity/spawn_publish_d0_at70_05239e.s)
/// $05239E: move.l d0, -(a7)
pub fn i_05239e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x523A0;
    cpu.ir = 0x2F00;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $0523A0: lea.l $523ee(pc), a1
pub fn i_0523a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x523A4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x523EE;
    16
}
/// $0523A4: jsr $4ae.l
pub fn i_0523a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x523AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x523AA);
    cpu.pc = 0x4AE;
    16
}
/// $0523AA: move.l (a7)+, d0
pub fn i_0523aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x523AC;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0523AC: move.w d0, $70(a0)
pub fn i_0523ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x523B0;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0523B0: rts
pub fn i_0523b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x523B2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_SpawnAndPublishD0At70_0523B2  ($0523B2, 20 B, entity/spawn_publish_d0_at70_0523b2.s)
/// $0523B2: move.l d0, -(a7)
pub fn i_0523b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x523B4;
    cpu.ir = 0x2F00;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $0523B4: lea.l $524aa(pc), a1
pub fn i_0523b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x523B8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x524AA;
    16
}
/// $0523B8: jsr $4ae.l
pub fn i_0523b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x523BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x523BE);
    cpu.pc = 0x4AE;
    16
}
/// $0523BE: move.l (a7)+, d0
pub fn i_0523be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x523C0;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0523C0: move.w d0, $70(a0)
pub fn i_0523c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x523C4;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0523C4: rts
pub fn i_0523c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x523C6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PalFade_SpawnIn_0523c6  ($0523C6, 20 B, video/cellmap_fix_overlay_palfade_051aa4.s)
/// $0523C6: move.l d0, -(a7)
pub fn i_0523c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x523C8;
    cpu.ir = 0x2F00;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $0523C8: lea.l $523fa(pc), a1
pub fn i_0523c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x523CC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x523FA;
    16
}
/// $0523CC: jsr $4ae.l
pub fn i_0523cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x523D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x523D2);
    cpu.pc = 0x4AE;
    16
}
/// $0523D2: move.l (a7)+, d0
pub fn i_0523d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x523D4;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0523D4: move.w d0, $70(a0)
pub fn i_0523d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x523D8;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0523D8: rts
pub fn i_0523d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x523DA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PalFade_SpawnOut_0523da  ($0523DA, 20 B, video/cellmap_fix_overlay_palfade_051aa4.s)
/// $0523DA: move.l d0, -(a7)
pub fn i_0523da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x523DC;
    cpu.ir = 0x2F00;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $0523DC: lea.l $524b6(pc), a1
pub fn i_0523dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x523E0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x524B6;
    16
}
/// $0523E0: jsr $4ae.l
pub fn i_0523e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x523E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x523E6);
    cpu.pc = 0x4AE;
    16
}
/// $0523E6: move.l (a7)+, d0
pub fn i_0523e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x523E8;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0523E8: move.w d0, $70(a0)
pub fn i_0523e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x523EC;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0523EC: rts
pub fn i_0523ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x523EE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Template_0523EE  ($0523EE, 12 B, video/cellmap_fix_overlay_palfade_051aa4.s)
/// $0523EE: move.b #$1, $10a2c8.l
pub fn i_0523ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x523F6;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2C8, v1 as u8);
    16
}
/// $0523F6: bra.w $52402
pub fn i_0523f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x523FA;
    cpu.ir = 0x6000;
    cpu.pc = 0x52402;
    10
}
// ===== PalFade_In_Task_0523fa  ($0523FA, 176 B, video/cellmap_fix_overlay_palfade_051aa4.s)
/// $0523FA: move.b #$ff, $10a2c8.l
pub fn i_0523fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52402;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2C8, v1 as u8);
    16
}
/// $052402: moveq #$0, d0
pub fn i_052402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52404;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $052404: move.b d0, $10a2ca.l
pub fn i_052404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5240A;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CA, v1 as u8);
    16
}
/// $05240A: move.b d0, $10a2cb.l
pub fn i_05240a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52410;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CB, v1 as u8);
    16
}
/// $052410: move.b d0, $10a2cc.l
pub fn i_052410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52416;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CC, v1 as u8);
    16
}
/// $052416: move.b d0, $10a2cf.l
pub fn i_052416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5241C;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CF, v1 as u8);
    16
}
/// $05241C: moveq #$1, d1
pub fn i_05241c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5241E;
    cpu.ir = 0x7201;
    cpu.d[1] = 0x1;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05241E: cmpi.w #$3, $70(a6)
pub fn i_05241e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52424;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Word, false);
    16
}
/// $052424: bhi.w $5243e
pub fn i_052424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52428;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x5243E; return 10; }
    8
}
/// $052428: move.w $70(a6), d0
pub fn i_052428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5242C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05242C: add.w d0, d0
pub fn i_05242c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5242E;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05242E: add.w d0, d0
pub fn i_05242e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52430;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $052430: lea.l $52570(pc), a0
pub fn i_052430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52434;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x52570;
    16
}
/// $052434: move.w (a0, d0.w), d1
pub fn i_052434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52438;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $052438: move.w $2(a0, d0.w), $70(a6)
pub fn i_052438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5243E;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad4, v3 as u16);
    22
}
/// $05243E: move.w $70(a6), $72(a6)
pub fn i_05243e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52444;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad4, v3 as u16);
    20
}
/// $052444: move.w #$0, $74(a6)
pub fn i_052444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5244A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05244A: move.w d1, $8c(a6)
pub fn i_05244a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5244E;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05244E: lea.l $52454(pc), a1
pub fn i_05244e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52452;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x52454;
    16
}
/// $052452: move.l a1, (a6)
pub fn i_052452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52454;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $052454: subq.w #$1, $72(a6)
pub fn i_052454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52458;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $052458: bne.w $524a8
pub fn i_052458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5245C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x524A8; return 10; }
    8
}
/// $05245C: move.w $70(a6), $72(a6)
pub fn i_05245c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52462;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad4, v3 as u16);
    20
}
/// $052462: move.w $8c(a6), d0
pub fn i_052462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52466;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $052466: add.b d0, $10a2ca.l
pub fn i_052466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5246C;
    cpu.ir = 0xD139;
    let v1 = bus.read8(0x10A2CA) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, h1, Size::Byte, true);
    bus.write8(0x10A2CA, r2 as u8);
    16
}
/// $05246C: add.b d0, $10a2cb.l
pub fn i_05246c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52472;
    cpu.ir = 0xD139;
    let v1 = bus.read8(0x10A2CB) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, h1, Size::Byte, true);
    bus.write8(0x10A2CB, r2 as u8);
    16
}
/// $052472: add.b d0, $10a2cc.l
pub fn i_052472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52478;
    cpu.ir = 0xD139;
    let v1 = bus.read8(0x10A2CC) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, h1, Size::Byte, true);
    bus.write8(0x10A2CC, r2 as u8);
    16
}
/// $052478: add.w d0, $74(a6)
pub fn i_052478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5247C;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $05247C: cmpi.w #$1f, $74(a6)
pub fn i_05247c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52482;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1F, Size::Word, false);
    16
}
/// $052482: blt.w $524a8
pub fn i_052482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52486;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x524A8; return 10; }
    8
}
/// $052486: moveq #$1f, d0
pub fn i_052486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52488;
    cpu.ir = 0x701F;
    cpu.d[0] = 0x1F;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $052488: move.b d0, $10a2ca.l
pub fn i_052488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5248E;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CA, v1 as u8);
    16
}
/// $05248E: move.b d0, $10a2cb.l
pub fn i_05248e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52494;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CB, v1 as u8);
    16
}
/// $052494: move.b d0, $10a2cc.l
pub fn i_052494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5249A;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CC, v1 as u8);
    16
}
/// $05249A: move.b #$ff, $10a2cf.l
pub fn i_05249a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x524A2;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CF, v1 as u8);
    16
}
/// $0524A2: jmp $518.l
pub fn i_0524a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x524A8;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0524A8: rts
pub fn i_0524a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x524AA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Template_0524AA  ($0524AA, 12 B, video/cellmap_fix_overlay_palfade_051aa4.s)
/// $0524AA: move.b #$1, $10a2c8.l
pub fn i_0524aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x524B2;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2C8, v1 as u8);
    16
}
/// $0524B2: bra.w $524be
pub fn i_0524b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x524B6;
    cpu.ir = 0x6000;
    cpu.pc = 0x524BE;
    10
}
// ===== PalFade_Out_Task_0524b6  ($0524B6, 86 B, video/cellmap_fix_overlay_palfade_051aa4.s)
/// $0524B6: move.b #$ff, $10a2c8.l
pub fn i_0524b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x524BE;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2C8, v1 as u8);
    16
}
/// $0524BE: moveq #$1f, d0
pub fn i_0524be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x524C0;
    cpu.ir = 0x701F;
    cpu.d[0] = 0x1F;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0524C0: move.b d0, $10a2ca.l
pub fn i_0524c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x524C6;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CA, v1 as u8);
    16
}
/// $0524C6: move.b d0, $10a2cb.l
pub fn i_0524c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x524CC;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CB, v1 as u8);
    16
}
/// $0524CC: move.b d0, $10a2cc.l
pub fn i_0524cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x524D2;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CC, v1 as u8);
    16
}
/// $0524D2: move.b #$0, $10a2cf.l
pub fn i_0524d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x524DA;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CF, v1 as u8);
    16
}
/// $0524DA: moveq #$1, d1
pub fn i_0524da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x524DC;
    cpu.ir = 0x7201;
    cpu.d[1] = 0x1;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0524DC: cmpi.w #$3, $70(a6)
pub fn i_0524dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x524E2;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Word, false);
    16
}
/// $0524E2: bhi.w $524fc
pub fn i_0524e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x524E6;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x524FC; return 10; }
    8
}
/// $0524E6: move.w $70(a6), d0
pub fn i_0524e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x524EA;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0524EA: add.w d0, d0
pub fn i_0524ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x524EC;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0524EC: add.w d0, d0
pub fn i_0524ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x524EE;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0524EE: lea.l $52570(pc), a0
pub fn i_0524ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x524F2;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x52570;
    16
}
/// $0524F2: move.w (a0, d0.w), d1
pub fn i_0524f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x524F6;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $0524F6: move.w $2(a0, d0.w), $70(a6)
pub fn i_0524f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x524FC;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad4, v3 as u16);
    22
}
/// $0524FC: move.w $70(a6), $72(a6)
pub fn i_0524fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52502;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad4, v3 as u16);
    20
}
/// $052502: move.w #$0, $74(a6)
pub fn i_052502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52508;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $052508: move.w d1, $8c(a6)
pub fn i_052508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5250C;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_05250c  ($05250C, 8 B, kernel/task_handlers.c)
/// $05250C: lea.l $52514(pc), a1
pub fn i_05250c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52510;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x52514;
    16
}
/// $052510: move.l a1, (a6)
pub fn i_052510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52512;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $052512: rts
pub fn i_052512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52514;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PalFade_Out_Step_052514  ($052514, 92 B, video/cellmap_fix_overlay_palfade_051aa4.s)
/// $052514: subq.w #$1, $72(a6)
pub fn i_052514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52518;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $052518: bne.w $5256e
pub fn i_052518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5251C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5256E; return 10; }
    8
}
/// $05251C: move.w $70(a6), $72(a6)
pub fn i_05251c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52522;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad4, v3 as u16);
    20
}
/// $052522: move.w $8c(a6), d0
pub fn i_052522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52526;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $052526: sub.b d0, $10a2ca.l
pub fn i_052526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5252C;
    cpu.ir = 0x9139;
    let v1 = bus.read8(0x10A2CA) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, h1, Size::Byte, true);
    bus.write8(0x10A2CA, r2 as u8);
    16
}
/// $05252C: sub.b d0, $10a2cb.l
pub fn i_05252c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52532;
    cpu.ir = 0x9139;
    let v1 = bus.read8(0x10A2CB) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, h1, Size::Byte, true);
    bus.write8(0x10A2CB, r2 as u8);
    16
}
/// $052532: sub.b d0, $10a2cc.l
pub fn i_052532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52538;
    cpu.ir = 0x9139;
    let v1 = bus.read8(0x10A2CC) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, h1, Size::Byte, true);
    bus.write8(0x10A2CC, r2 as u8);
    16
}
/// $052538: add.w d0, $74(a6)
pub fn i_052538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5253C;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $05253C: cmpi.w #$1f, $74(a6)
pub fn i_05253c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52542;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1F, Size::Word, false);
    16
}
/// $052542: blt.w $5256e
pub fn i_052542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52546;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x5256E; return 10; }
    8
}
/// $052546: moveq #$0, d0
pub fn i_052546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52548;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $052548: move.b d0, $10a2c8.l
pub fn i_052548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5254E;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2C8, v1 as u8);
    16
}
/// $05254E: move.b d0, $10a2ca.l
pub fn i_05254e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52554;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CA, v1 as u8);
    16
}
/// $052554: move.b d0, $10a2cb.l
pub fn i_052554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5255A;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CB, v1 as u8);
    16
}
/// $05255A: move.b d0, $10a2cc.l
pub fn i_05255a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52560;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CC, v1 as u8);
    16
}
/// $052560: move.b #$ff, $10a2cf.l
pub fn i_052560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52568;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CF, v1 as u8);
    16
}
/// $052568: jmp $518.l
pub fn i_052568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5256E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $05256E: rts
pub fn i_05256e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52570;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PalFade_SpeedTable_052570  ($052570, 52 B, video/cellmap_fix_overlay_palfade_051aa4.s)
/// $052570: ori.b #$1, (a7)+
pub fn i_052570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52574;
    cpu.ir = 0x1F;
    let ad1 = cpu.a[7]; cpu.a[7] = ad1.wrapping_add(2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $052574: ori.b #$1, d4
pub fn i_052574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52578;
    cpu.ir = 0x4;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $052578: ori.b #$1, d2
pub fn i_052578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5257C;
    cpu.ir = 0x2;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05257C: ori.b #$1, d1
pub fn i_05257c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52580;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $052580: movem.l d0-d3, -(a7)
pub fn i_052580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52584;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $052584: lea.l $525a4(pc), a1
pub fn i_052584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52588;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x525A4;
    16
}
/// $052588: jsr $4ae.l
pub fn i_052588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5258E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5258E);
    cpu.pc = 0x4AE;
    16
}
/// $05258E: movem.l (a7)+, d0-d3
pub fn i_05258e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52592;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $052592: move.w d0, $7c(a0)
pub fn i_052592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52596;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $052596: move.w d1, $7e(a0)
pub fn i_052596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5259A;
    cpu.ir = 0x3141;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05259A: move.w d2, $80(a0)
pub fn i_05259a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5259E;
    cpu.ir = 0x3142;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05259E: move.w d3, $70(a0)
pub fn i_05259e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x525A2;
    cpu.ir = 0x3143;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0525A2: rts
pub fn i_0525a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x525A4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PalFade_ToColor_Task_0525a4  ($0525A4, 254 B, video/cellmap_fix_overlay_palfade_051aa4.s)
/// $0525A4: move.b #$80, $10a2c8.l
pub fn i_0525a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x525AC;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2C8, v1 as u8);
    16
}
/// $0525AC: move.b $10a2ca.l, $88(a6)
pub fn i_0525ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x525B4;
    cpu.ir = 0x1D79;
    let v2 = bus.read8(0x10A2CA) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[6].wrapping_add(0x88);
    bus.write8(ad3, v2 as u8);
    24
}
/// $0525B4: move.b $10a2cb.l, $89(a6)
pub fn i_0525b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x525BC;
    cpu.ir = 0x1D79;
    let v2 = bus.read8(0x10A2CB) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[6].wrapping_add(0x89);
    bus.write8(ad3, v2 as u8);
    24
}
/// $0525BC: move.b $10a2cc.l, $8a(a6)
pub fn i_0525bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x525C4;
    cpu.ir = 0x1D79;
    let v2 = bus.read8(0x10A2CC) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[6].wrapping_add(0x8A);
    bus.write8(ad3, v2 as u8);
    24
}
/// $0525C4: moveq #$0, d0
pub fn i_0525c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x525C6;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0525C6: move.w d0, $82(a6)
pub fn i_0525c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x525CA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0525CA: move.w d0, $84(a6)
pub fn i_0525ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x525CE;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0525CE: move.w d0, $86(a6)
pub fn i_0525ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x525D2;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0525D2: moveq #$0, d0
pub fn i_0525d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x525D4;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0525D4: move.b $10a2ca.l, d0
pub fn i_0525d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x525DA;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10A2CA) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $0525DA: asl.w #$8, d0
pub fn i_0525da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x525DC;
    cpu.ir = 0xE140;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $0525DC: move.w $7c(a6), d1
pub fn i_0525dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x525E0;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0525E0: asl.w #$8, d1
pub fn i_0525e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x525E2;
    cpu.ir = 0xE141;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $0525E2: sub.w d1, d0
pub fn i_0525e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x525E4;
    cpu.ir = 0x9041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0525E4: ext.l d0
pub fn i_0525e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x525E6;
    cpu.ir = 0x48C0;
    cpu.d[0] = cpu.d[0] as i16 as i32 as u32;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0525E6: divs.w $70(a6), d0
pub fn i_0525e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x525EA;
    cpu.ir = 0x81EE;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set(StatusRegister::C, false);
    let dv = v3 as u16 as i16 as i32;
    if dv == 0 { cpu.enter_group2_exception(bus, Exception::DivideByZero); return 38; }
    let dd = cpu.d[0] as i32; let q = dd.wrapping_div(dv); let rm = dd.wrapping_rem(dv);
    if !(-32768..=32767).contains(&q) { cpu.sr.set(StatusRegister::V, true); cpu.sr.set(StatusRegister::N, true); cpu.sr.set(StatusRegister::Z, false); return 166; }
    cpu.d[0] = ((rm as u32 & 0xFFFF) << 16) | (q as u32 & 0xFFFF);
    cpu.sr.set_nz(q as u32 & 0xFFFF, Size::Word); cpu.sr.set(StatusRegister::V, false);
    166
}
/// $0525EA: move.w d0, $76(a6)
pub fn i_0525ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x525EE;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0525EE: moveq #$0, d0
pub fn i_0525ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x525F0;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0525F0: move.b $10a2cb.l, d0
pub fn i_0525f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x525F6;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10A2CB) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $0525F6: asl.w #$8, d0
pub fn i_0525f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x525F8;
    cpu.ir = 0xE140;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $0525F8: move.w $7e(a6), d1
pub fn i_0525f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x525FC;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0525FC: asl.w #$8, d1
pub fn i_0525fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x525FE;
    cpu.ir = 0xE141;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $0525FE: sub.w d1, d0
pub fn i_0525fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52600;
    cpu.ir = 0x9041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $052600: ext.l d0
pub fn i_052600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52602;
    cpu.ir = 0x48C0;
    cpu.d[0] = cpu.d[0] as i16 as i32 as u32;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $052602: divs.w $70(a6), d0
pub fn i_052602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52606;
    cpu.ir = 0x81EE;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set(StatusRegister::C, false);
    let dv = v3 as u16 as i16 as i32;
    if dv == 0 { cpu.enter_group2_exception(bus, Exception::DivideByZero); return 38; }
    let dd = cpu.d[0] as i32; let q = dd.wrapping_div(dv); let rm = dd.wrapping_rem(dv);
    if !(-32768..=32767).contains(&q) { cpu.sr.set(StatusRegister::V, true); cpu.sr.set(StatusRegister::N, true); cpu.sr.set(StatusRegister::Z, false); return 166; }
    cpu.d[0] = ((rm as u32 & 0xFFFF) << 16) | (q as u32 & 0xFFFF);
    cpu.sr.set_nz(q as u32 & 0xFFFF, Size::Word); cpu.sr.set(StatusRegister::V, false);
    166
}
/// $052606: move.w d0, $78(a6)
pub fn i_052606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5260A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05260A: moveq #$0, d0
pub fn i_05260a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5260C;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05260C: move.b $10a2cc.l, d0
pub fn i_05260c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52612;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10A2CC) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $052612: asl.w #$8, d0
pub fn i_052612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52614;
    cpu.ir = 0xE140;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $052614: move.w $80(a6), d1
pub fn i_052614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52618;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $052618: asl.w #$8, d1
pub fn i_052618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5261A;
    cpu.ir = 0xE141;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $05261A: sub.w d1, d0
pub fn i_05261a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5261C;
    cpu.ir = 0x9041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05261C: ext.l d0
pub fn i_05261c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5261E;
    cpu.ir = 0x48C0;
    cpu.d[0] = cpu.d[0] as i16 as i32 as u32;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05261E: divs.w $70(a6), d0
pub fn i_05261e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52622;
    cpu.ir = 0x81EE;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set(StatusRegister::C, false);
    let dv = v3 as u16 as i16 as i32;
    if dv == 0 { cpu.enter_group2_exception(bus, Exception::DivideByZero); return 38; }
    let dd = cpu.d[0] as i32; let q = dd.wrapping_div(dv); let rm = dd.wrapping_rem(dv);
    if !(-32768..=32767).contains(&q) { cpu.sr.set(StatusRegister::V, true); cpu.sr.set(StatusRegister::N, true); cpu.sr.set(StatusRegister::Z, false); return 166; }
    cpu.d[0] = ((rm as u32 & 0xFFFF) << 16) | (q as u32 & 0xFFFF);
    cpu.sr.set_nz(q as u32 & 0xFFFF, Size::Word); cpu.sr.set(StatusRegister::V, false);
    166
}
/// $052622: move.w d0, $7a(a6)
pub fn i_052622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52626;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $052626: lea.l $5262c(pc), a1
pub fn i_052626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5262A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5262C;
    16
}
/// $05262A: move.l a1, (a6)
pub fn i_05262a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5262C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05262C: subq.w #$1, $70(a6)
pub fn i_05262c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52630;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $052630: beq.w $52684
pub fn i_052630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52634;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x52684; return 10; }
    8
}
/// $052634: move.w $76(a6), d0
pub fn i_052634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52638;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $052638: add.w d0, $82(a6)
pub fn i_052638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5263C;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $05263C: move.w $78(a6), d0
pub fn i_05263c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52640;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $052640: add.w d0, $84(a6)
pub fn i_052640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52644;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x84);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $052644: move.w $7a(a6), d0
pub fn i_052644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52648;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $052648: add.w d0, $86(a6)
pub fn i_052648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5264C;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $05264C: move.w $82(a6), d0
pub fn i_05264c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52650;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $052650: asr.w #$8, d0
pub fn i_052650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52652;
    cpu.ir = 0xE040;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $052652: move.b $88(a6), d1
pub fn i_052652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52656;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $052656: sub.b d0, d1
pub fn i_052656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52658;
    cpu.ir = 0x9200;
    let v1 = cpu.d[0] & 0xFF;
    let h1 = cpu.d[1] & 0xFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    4
}
/// $052658: move.b d1, $10a2ca.l
pub fn i_052658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5265E;
    cpu.ir = 0x13C1;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CA, v1 as u8);
    16
}
/// $05265E: move.w $84(a6), d0
pub fn i_05265e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52662;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $052662: asr.w #$8, d0
pub fn i_052662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52664;
    cpu.ir = 0xE040;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $052664: move.b $89(a6), d1
pub fn i_052664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52668;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x89);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $052668: sub.b d0, d1
pub fn i_052668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5266A;
    cpu.ir = 0x9200;
    let v1 = cpu.d[0] & 0xFF;
    let h1 = cpu.d[1] & 0xFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    4
}
/// $05266A: move.b d1, $10a2cb.l
pub fn i_05266a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52670;
    cpu.ir = 0x13C1;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CB, v1 as u8);
    16
}
/// $052670: move.w $86(a6), d0
pub fn i_052670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52674;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $052674: asr.w #$8, d0
pub fn i_052674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52676;
    cpu.ir = 0xE040;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $052676: move.b $8a(a6), d1
pub fn i_052676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5267A;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05267A: sub.b d0, d1
pub fn i_05267a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5267C;
    cpu.ir = 0x9200;
    let v1 = cpu.d[0] & 0xFF;
    let h1 = cpu.d[1] & 0xFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    4
}
/// $05267C: move.b d1, $10a2cc.l
pub fn i_05267c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52682;
    cpu.ir = 0x13C1;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CC, v1 as u8);
    16
}
/// $052682: rts
pub fn i_052682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52684;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $052684: move.w $7c(a6), d0
pub fn i_052684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52688;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $052688: move.b d0, $10a2ca.l
pub fn i_052688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5268E;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CA, v1 as u8);
    16
}
/// $05268E: move.w $7e(a6), d0
pub fn i_05268e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52692;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $052692: move.b d0, $10a2cb.l
pub fn i_052692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52698;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CB, v1 as u8);
    16
}
/// $052698: move.w $80(a6), d0
pub fn i_052698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5269C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05269C: move.b d0, $10a2cc.l
pub fn i_05269c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526A2;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CC, v1 as u8);
    16
}
// ===== SetTaskHandler_0526a2  ($0526A2, 8 B, kernel/task_handlers.c)
/// $0526A2: lea.l $526aa(pc), a1
pub fn i_0526a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526A6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x526AA;
    16
}
/// $0526A6: move.l a1, (a6)
pub fn i_0526a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526A8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0526A8: rts
pub fn i_0526a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526AA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearGlobalFlag_0526aa  ($0526AA, 14 B, kernel/scheduler_flag_setters.c)
/// $0526AA: clr.b $10a2c8.l
pub fn i_0526aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526B0;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10A2C8) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10A2C8, r2 as u8);
    16
}
/// $0526B0: jmp $518.l
pub fn i_0526b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526B6;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0526B6: rts
pub fn i_0526b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526B8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SpriteTable_Init256_0526b8  ($0526B8, 90 B, video/cellmap_fix_overlay_palfade_051aa4.s)
/// $0526B8: lea.l $1082c8.l, a1
pub fn i_0526b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526BE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1082C8;
    20
}
/// $0526BE: move.w #$ff, d2
pub fn i_0526be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526C2;
    cpu.ir = 0x343C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0526C2: move.w #$8800, d0
pub fn i_0526c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526C6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8800;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0526C6: move.w d0, (a1)+
pub fn i_0526c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526C8;
    cpu.ir = 0x32C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $0526C8: clr.w (a1)+
pub fn i_0526c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526CA;
    cpu.ir = 0x4259;
    let ad1 = cpu.a[1]; cpu.a[1] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $0526CA: clr.l (a1)+
pub fn i_0526ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526CC;
    cpu.ir = 0x4299;
    let ad1 = cpu.a[1]; cpu.a[1] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $0526CC: clr.l (a1)+
pub fn i_0526cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526CE;
    cpu.ir = 0x4299;
    let ad1 = cpu.a[1]; cpu.a[1] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $0526CE: clr.l (a1)+
pub fn i_0526ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526D0;
    cpu.ir = 0x4299;
    let ad1 = cpu.a[1]; cpu.a[1] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $0526D0: clr.l (a1)+
pub fn i_0526d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526D2;
    cpu.ir = 0x4299;
    let ad1 = cpu.a[1]; cpu.a[1] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $0526D2: clr.l (a1)+
pub fn i_0526d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526D4;
    cpu.ir = 0x4299;
    let ad1 = cpu.a[1]; cpu.a[1] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $0526D4: clr.l (a1)+
pub fn i_0526d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526D6;
    cpu.ir = 0x4299;
    let ad1 = cpu.a[1]; cpu.a[1] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $0526D6: clr.l (a1)+
pub fn i_0526d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526D8;
    cpu.ir = 0x4299;
    let ad1 = cpu.a[1]; cpu.a[1] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $0526D8: dbra d2, $526c6
pub fn i_0526d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526DC;
    cpu.ir = 0x51CA;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x526DC; return 12; }
    let n = (cpu.d[2] as u16).wrapping_sub(1);
    cpu.d[2] = (cpu.d[2] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x526DC; return 14; }
    cpu.pc = 0x526C6;
    10
}
/// $0526DC: lea.l $1081c2.l, a1
pub fn i_0526dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526E2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1081C2;
    20
}
/// $0526E2: move.w #$30, d1
pub fn i_0526e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526E6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0526E6: move.w d1, $1082c2.l
pub fn i_0526e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526EC;
    cpu.ir = 0x33C1;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x1082C2, v1 as u16);
    16
}
/// $0526EC: subq.w #$1, d1
pub fn i_0526ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526EE;
    cpu.ir = 0x5341;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0526EE: clr.b d2
pub fn i_0526ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526F0;
    cpu.ir = 0x4202;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    4
}
/// $0526F0: move.b d2, (a1)+
pub fn i_0526f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526F2;
    cpu.ir = 0x12C2;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $0526F2: addq.b #$1, d2
pub fn i_0526f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526F4;
    cpu.ir = 0x5202;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    4
}
/// $0526F4: dbra d1, $526f0
pub fn i_0526f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526F8;
    cpu.ir = 0x51C9;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x526F8; return 12; }
    let n = (cpu.d[1] as u16).wrapping_sub(1);
    cpu.d[1] = (cpu.d[1] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x526F8; return 14; }
    cpu.pc = 0x526F0;
    10
}
/// $0526F8: moveq #$0, d0
pub fn i_0526f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x526FA;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0526FA: move.w d0, $1082c4.l
pub fn i_0526fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52700;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x1082C4, v1 as u16);
    16
}
/// $052700: move.w d0, $1082c6.l
pub fn i_052700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52706;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x1082C6, v1 as u16);
    16
}
/// $052706: move.b d0, $10a2c9.l
pub fn i_052706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5270C;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2C9, v1 as u8);
    16
}
/// $05270C: move.b d0, $10a2d1.l
pub fn i_05270c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52712;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2D1, v1 as u8);
    16
}
// ===== Pubcleaner_10A2Cx_052712  ($052712, 40 B, kernel/publish_cleaner_052712.s)
/// $052712: moveq #$0, d0
pub fn i_052712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52714;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $052714: move.b d0, $10a2c8.l
pub fn i_052714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5271A;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2C8, v1 as u8);
    16
}
/// $05271A: move.b d0, $10a2ca.l
pub fn i_05271a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52720;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CA, v1 as u8);
    16
}
/// $052720: move.b d0, $10a2cb.l
pub fn i_052720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52726;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CB, v1 as u8);
    16
}
/// $052726: move.b d0, $10a2cc.l
pub fn i_052726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5272C;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CC, v1 as u8);
    16
}
/// $05272C: move.b d0, $10a2cf.l
pub fn i_05272c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52732;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CF, v1 as u8);
    16
}
/// $052732: move.b d0, $10a2d0.l
pub fn i_052732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52738;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2D0, v1 as u8);
    16
}
/// $052738: rts
pub fn i_052738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5273A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Scheduler_CompareField10_05273a  ($05273A, 16 B, video/cellmap_fix_overlay_palfade_051aa4.s)
/// $05273A: movea.l $8(a6), a1
pub fn i_05273a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5273E;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $05273E: move.b $10(a6), d0
pub fn i_05273e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52742;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $052742: cmp.b $10(a1), d0
pub fn i_052742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52746;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $052746: bcs.w $52750
pub fn i_052746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5274A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x52750; return 10; }
    8
}
// ===== ClearXN_05274a  ($05274A, 6 B, thunks/ccr_helpers.c)
/// $05274A: andi.b #$ee, ccr
pub fn i_05274a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5274E;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05274E: rts
pub fn i_05274e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52750;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_052750  ($052750, 6 B, thunks/ccr_helpers.c)
/// $052750: ori.b #$11, ccr
pub fn i_052750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52754;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $052754: rts
pub fn i_052754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52756;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sprite_SetupSlotFromTableA_Thunk_052756  ($052756, 14 B, video/cellmap_fix_overlay_palfade_051aa4.s)
/// $052756: move.w (a0), d1
pub fn i_052756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52758;
    cpu.ir = 0x3210;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $052758: move.w $2(a0), d2
pub fn i_052758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5275C;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05275C: move.w $4(a0), d3
pub fn i_05275c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52760;
    cpu.ir = 0x3628;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $052760: move.w #$1, d4
pub fn i_052760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52764;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_052764  ($052764, 8 B, thunks/jsr_abs_thunks.c)
/// $052764: jsr $2c26.l
pub fn i_052764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5276A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5276A);
    cpu.pc = 0x2C26;
    16
}
/// $05276A: rts
pub fn i_05276a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5276C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_SpawnAndPublishD0At70_Thunk_05276c  ($05276C, 2 B, video/cellmap_fix_overlay_palfade_051aa4.s)
/// $05276C: move.w (a0), d0
pub fn i_05276c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5276E;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
// ===== JsrAbsThunk_05276e  ($05276E, 8 B, thunks/jsr_abs_thunks.c)
/// $05276E: jsr $5239e.l
pub fn i_05276e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52774;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52774);
    cpu.pc = 0x5239E;
    16
}
/// $052774: rts
pub fn i_052774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52776;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_SpawnAndPublishD0At70_Thunk_052776  ($052776, 2 B, video/cellmap_fix_overlay_palfade_051aa4.s)
/// $052776: move.w (a0), d0
pub fn i_052776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52778;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
// ===== JsrAbsThunk_052778  ($052778, 8 B, thunks/jsr_abs_thunks.c)
/// $052778: jsr $523b2.l
pub fn i_052778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5277E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5277E);
    cpu.pc = 0x523B2;
    16
}
/// $05277E: rts
pub fn i_05277e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52780;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrAbsThunk_052780  ($052780, 8 B, thunks/jsr_abs_thunks.c)
/// $052780: jsr $5dd2a.l
pub fn i_052780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52786;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52786);
    cpu.pc = 0x5DD2A;
    16
}
/// $052786: rts
pub fn i_052786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52788;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Task_AllocProp86586_052788  ($052788, 6 B, video/cellmap_fix_overlay_palfade_051aa4.s)
/// $052788: lea.l $86586.l, a1
pub fn i_052788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5278E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x86586;
    20
}
// ===== JsrAbsThunk_05278e  ($05278E, 8 B, thunks/jsr_abs_thunks.c)
/// $05278E: jsr $4ae.l
pub fn i_05278e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52794;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52794);
    cpu.pc = 0x4AE;
    16
}
/// $052794: rts
pub fn i_052794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52796;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrAbsThunk_052796  ($052796, 8 B, thunks/jsr_abs_thunks.c)
/// $052796: jsr $5026c.l
pub fn i_052796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5279C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5279C);
    cpu.pc = 0x5026C;
    16
}
/// $05279C: rts
pub fn i_05279c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5279E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Scheduler_CompareField10_05279e  ($05279E, 16 B, video/cellmap_fix_overlay_palfade_051aa4.s)
/// $05279E: movea.l $8(a6), a1
pub fn i_05279e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x527A2;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0527A2: move.b $10(a6), d0
pub fn i_0527a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x527A6;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0527A6: cmp.b $10(a1), d0
pub fn i_0527a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x527AA;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $0527AA: bcs.w $527b4
pub fn i_0527aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x527AE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x527B4; return 10; }
    8
}
// ===== ClearXN_0527ae  ($0527AE, 6 B, thunks/ccr_helpers.c)
/// $0527AE: andi.b #$ee, ccr
pub fn i_0527ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x527B2;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0527B2: rts
pub fn i_0527b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x527B4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_0527b4  ($0527B4, 6 B, thunks/ccr_helpers.c)
/// $0527B4: ori.b #$11, ccr
pub fn i_0527b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x527B8;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0527B8: rts
pub fn i_0527b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x527BA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Breakable2Stage_0527ba  ($0527BA, 258 B, props/destructible_0527ba.s)
/// $0527BA: movea.l $3c(a6), a1
pub fn i_0527ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x527BE;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0527BE: jsr $2942a.l
pub fn i_0527be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x527C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x527C4);
    cpu.pc = 0x2942A;
    16
}
/// $0527C4: move.w #$40, $70(a6)
pub fn i_0527c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x527CA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0527CA: move.w #$a, $66(a6)
pub fn i_0527ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x527D0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0527D0: move.b #$0, $21(a6)
pub fn i_0527d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x527D6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0527D6: lea.l $297100.l, a0
pub fn i_0527d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x527DC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x297100;
    20
}
/// $0527DC: jsr $28cd4.l
pub fn i_0527dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x527E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x527E2);
    cpu.pc = 0x28CD4;
    16
}
/// $0527E2: cmpi.w #$0, $38(a6)
pub fn i_0527e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x527E8;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0527E8: beq.w $52802
pub fn i_0527e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x527EC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x52802; return 10; }
    8
}
/// $0527EC: cmpi.w #$6, $38(a6)
pub fn i_0527ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x527F2;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x6, Size::Word, false);
    16
}
/// $0527F2: beq.w $52802
pub fn i_0527f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x527F6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x52802; return 10; }
    8
}
/// $0527F6: lea.l $297116.l, a0
pub fn i_0527f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x527FC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x297116;
    20
}
/// $0527FC: jsr $28cd4.l
pub fn i_0527fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52802;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52802);
    cpu.pc = 0x28CD4;
    16
}
/// $052802: lea.l $52808(pc), a1
pub fn i_052802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52806;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x52808;
    16
}
/// $052806: move.l a1, (a6)
pub fn i_052806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52808;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $052808: jsr $2783a.l
pub fn i_052808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5280E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5280E);
    cpu.pc = 0x2783A;
    16
}
/// $05280E: jsr $28d70.l
pub fn i_05280e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52814;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52814);
    cpu.pc = 0x28D70;
    16
}
/// $052814: jsr $2870a.l
pub fn i_052814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5281A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5281A);
    cpu.pc = 0x2870A;
    16
}
/// $05281A: bcc.w $52844
pub fn i_05281a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5281E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x52844; return 10; }
    8
}
/// $05281E: lea.l $5e766.l, a0
pub fn i_05281e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52824;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $052824: jsr $5e770.l
pub fn i_052824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5282A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5282A);
    cpu.pc = 0x5E770;
    16
}
/// $05282A: bclr.b #$3, $13(a6)
pub fn i_05282a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52830;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $052830: cmpi.b #$2, $58(a6)
pub fn i_052830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52836;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $052836: bne.w $52844
pub fn i_052836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5283A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x52844; return 10; }
    8
}
/// $05283A: move.b #$1, $21(a6)
pub fn i_05283a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52840;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $052840: bra.w $5284e
pub fn i_052840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52844;
    cpu.ir = 0x6000;
    cpu.pc = 0x5284E;
    10
}
/// $052844: jsr $28758.l
pub fn i_052844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5284A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5284A);
    cpu.pc = 0x28758;
    16
}
/// $05284A: bcc.w $528aa
pub fn i_05284a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5284E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x528AA; return 10; }
    8
}
/// $05284E: bclr.b #$0, $13(a6)
pub fn i_05284e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52854;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $052854: move.w #$1e, $66(a6)
pub fn i_052854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5285A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05285A: cmpi.b #$1, $21(a6)
pub fn i_05285a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52860;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $052860: bne.w $52894
pub fn i_052860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52864;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x52894; return 10; }
    8
}
/// $052864: move.w #$1027, d0
pub fn i_052864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52868;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1027;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $052868: jsr $2352.l
pub fn i_052868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5286E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5286E);
    cpu.pc = 0x2352;
    16
}
/// $05286E: move.w #$3, d0
pub fn i_05286e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52872;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $052872: jsr $5ea1c.l
pub fn i_052872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52878;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52878);
    cpu.pc = 0x5EA1C;
    16
}
/// $052878: addq.w #$1, d0
pub fn i_052878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5287A;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05287A: move.w d0, $72(a6)
pub fn i_05287a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5287E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05287E: lea.l $53768(pc), a1
pub fn i_05287e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52882;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x53768;
    16
}
/// $052882: jsr $4ae.l
pub fn i_052882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52888;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52888);
    cpu.pc = 0x4AE;
    16
}
/// $052888: jsr $5dd22.l
pub fn i_052888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5288E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5288E);
    cpu.pc = 0x5DD22;
    16
}
/// $05288E: subq.w #$1, $72(a6)
pub fn i_05288e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52892;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $052892: bne.b $5287e
pub fn i_052892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52894;
    cpu.ir = 0x66EA;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5287E; return 10; }
    8
}
/// $052894: jsr $53eba(pc)
pub fn i_052894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52898;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x52898);
    cpu.pc = 0x53EBA;
    16
}
/// $052898: jsr $53e78(pc)
pub fn i_052898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5289C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5289C);
    cpu.pc = 0x53E78;
    16
}
/// $05289C: addq.b #$1, $21(a6)
pub fn i_05289c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x528A0;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $0528A0: cmpi.b #$2, $21(a6)
pub fn i_0528a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x528A6;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $0528A6: beq.w $528b4
pub fn i_0528a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x528AA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x528B4; return 10; }
    8
}
/// $0528AA: jsr $4fa70.l
pub fn i_0528aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x528B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x528B0);
    cpu.pc = 0x4FA70;
    16
}
/// $0528B0: bcc.w $528ba
pub fn i_0528b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x528B4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x528BA; return 10; }
    8
}
/// $0528B4: jmp $518.l
pub fn i_0528b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x528BA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0528BA: rts
pub fn i_0528ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x528BC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_OneShotAnim_0528bc  ($0528BC, 70 B, props/destructible_0527ba.s)
/// $0528BC: move.w #$6d, d1
pub fn i_0528bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x528C0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x6D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0528C0: jsr $236e.l
pub fn i_0528c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x528C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x528C6);
    cpu.pc = 0x236E;
    16
}
/// $0528C6: move.b #$ff, $32(a6)
pub fn i_0528c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x528CC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0528CC: move.b #$ff, $33(a6)
pub fn i_0528cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x528D2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0528D2: move.b #$0, $3a(a6)
pub fn i_0528d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x528D8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0528D8: lea.l $29712c.l, a0
pub fn i_0528d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x528DE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29712C;
    20
}
/// $0528DE: jsr $28cd4.l
pub fn i_0528de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x528E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x528E4);
    cpu.pc = 0x28CD4;
    16
}
/// $0528E4: lea.l $528ea(pc), a1
pub fn i_0528e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x528E8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x528EA;
    16
}
/// $0528E8: move.l a1, (a6)
pub fn i_0528e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x528EA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0528EA: jsr $2783a.l
pub fn i_0528ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x528F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x528F0);
    cpu.pc = 0x2783A;
    16
}
/// $0528F0: jsr $28d70.l
pub fn i_0528f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x528F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x528F6);
    cpu.pc = 0x28D70;
    16
}
/// $0528F6: bcc.w $52900
pub fn i_0528f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x528FA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x52900; return 10; }
    8
}
/// $0528FA: jmp $518.l
pub fn i_0528fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52900;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $052900: rts
pub fn i_052900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52902;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Sign_052902  ($052902, 132 B, props/destructible_0527ba.s)
/// $052902: movea.l $3c(a6), a1
pub fn i_052902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52906;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $052906: jsr $2942a.l
pub fn i_052906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5290C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5290C);
    cpu.pc = 0x2942A;
    16
}
/// $05290C: move.w #$33, d1
pub fn i_05290c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52910;
    cpu.ir = 0x323C;
    let v1: u32 = 0x33;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $052910: jsr $236e.l
pub fn i_052910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52916;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52916);
    cpu.pc = 0x236E;
    16
}
/// $052916: move.w #$20, $70(a6)
pub fn i_052916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5291C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05291C: move.b #$ff, $32(a6)
pub fn i_05291c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52922;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $052922: move.b #$ff, $33(a6)
pub fn i_052922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52928;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $052928: move.b #$0, $3a(a6)
pub fn i_052928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5292E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05292E: move.w #$1, $66(a6)
pub fn i_05292e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52934;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $052934: lea.l $297174.l, a0
pub fn i_052934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5293A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x297174;
    20
}
/// $05293A: jsr $28cd4.l
pub fn i_05293a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52940;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52940);
    cpu.pc = 0x28CD4;
    16
}
/// $052940: lea.l $52946(pc), a1
pub fn i_052940(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52944;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x52946;
    16
}
/// $052944: move.l a1, (a6)
pub fn i_052944(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52946;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $052946: jsr $2783a.l
pub fn i_052946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5294C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5294C);
    cpu.pc = 0x2783A;
    16
}
/// $05294C: jsr $28d70.l
pub fn i_05294c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52952;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52952);
    cpu.pc = 0x28D70;
    16
}
/// $052952: jsr $2870a.l
pub fn i_052952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52958;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52958);
    cpu.pc = 0x2870A;
    16
}
/// $052958: bcc.w $52974
pub fn i_052958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5295C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x52974; return 10; }
    8
}
/// $05295C: bclr.b #$3, $13(a6)
pub fn i_05295c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52962;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $052962: move.l #$500, d0
pub fn i_052962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52968;
    cpu.ir = 0x203C;
    let v1: u32 = 0x500;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $052968: jsr $51a28.l
pub fn i_052968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5296E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5296E);
    cpu.pc = 0x51A28;
    16
}
/// $05296E: lea.l $52986(pc), a1
pub fn i_05296e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52972;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x52986;
    16
}
/// $052972: move.l a1, (a6)
pub fn i_052972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52974;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $052974: jsr $4fa70.l
pub fn i_052974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5297A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5297A);
    cpu.pc = 0x4FA70;
    16
}
/// $05297A: bcc.w $52984
pub fn i_05297a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5297E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x52984; return 10; }
    8
}
/// $05297E: jmp $518.l
pub fn i_05297e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52984;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $052984: rts
pub fn i_052984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52986;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_SignBroken_052986  ($052986, 68 B, props/destructible_0527ba.s)
/// $052986: move.w #$1029, d0
pub fn i_052986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5298A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1029;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05298A: jsr $2352.l
pub fn i_05298a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52990;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52990);
    cpu.pc = 0x2352;
    16
}
/// $052990: lea.l $29718a.l, a0
pub fn i_052990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52996;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29718A;
    20
}
/// $052996: jsr $28cd4.l
pub fn i_052996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5299C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5299C);
    cpu.pc = 0x28CD4;
    16
}
/// $05299C: lea.l $529a2(pc), a1
pub fn i_05299c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x529A0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x529A2;
    16
}
/// $0529A0: move.l a1, (a6)
pub fn i_0529a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x529A2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0529A2: jsr $2783a.l
pub fn i_0529a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x529A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x529A8);
    cpu.pc = 0x2783A;
    16
}
/// $0529A8: jsr $28d70.l
pub fn i_0529a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x529AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x529AE);
    cpu.pc = 0x28D70;
    16
}
/// $0529AE: bcc.w $529b8
pub fn i_0529ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x529B2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x529B8; return 10; }
    8
}
/// $0529B2: lea.l $529ca(pc), a1
pub fn i_0529b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x529B6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x529CA;
    16
}
/// $0529B6: move.l a1, (a6)
pub fn i_0529b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x529B8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0529B8: jsr $4fa70.l
pub fn i_0529b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x529BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x529BE);
    cpu.pc = 0x4FA70;
    16
}
/// $0529BE: bcc.w $529c8
pub fn i_0529be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x529C2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x529C8; return 10; }
    8
}
/// $0529C2: jmp $518.l
pub fn i_0529c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x529C8;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0529C8: rts
pub fn i_0529c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x529CA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_SignWreck_0529ca  ($0529CA, 92 B, props/destructible_0527ba.s)
/// $0529CA: bclr.b #$3, $13(a6)
pub fn i_0529ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x529D0;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0529D0: lea.l $2971ee.l, a0
pub fn i_0529d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x529D6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2971EE;
    20
}
/// $0529D6: jsr $28cd4.l
pub fn i_0529d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x529DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x529DC);
    cpu.pc = 0x28CD4;
    16
}
/// $0529DC: lea.l $529e2(pc), a1
pub fn i_0529dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x529E0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x529E2;
    16
}
/// $0529E0: move.l a1, (a6)
pub fn i_0529e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x529E2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0529E2: jsr $2783a.l
pub fn i_0529e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x529E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x529E8);
    cpu.pc = 0x2783A;
    16
}
/// $0529E8: jsr $28d70.l
pub fn i_0529e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x529EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x529EE);
    cpu.pc = 0x28D70;
    16
}
/// $0529EE: jsr $2870a.l
pub fn i_0529ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x529F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x529F4);
    cpu.pc = 0x2870A;
    16
}
/// $0529F4: bcc.w $52a0e
pub fn i_0529f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x529F8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x52A0E; return 10; }
    8
}
/// $0529F8: move.w #$1029, d0
pub fn i_0529f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x529FC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1029;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0529FC: jsr $2352.l
pub fn i_0529fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52A02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52A02);
    cpu.pc = 0x2352;
    16
}
/// $052A02: lea.l $2971ee.l, a0
pub fn i_052a02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52A08;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2971EE;
    20
}
/// $052A08: jsr $28cd4.l
pub fn i_052a08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52A0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52A0E);
    cpu.pc = 0x28CD4;
    16
}
/// $052A0E: bclr.b #$3, $13(a6)
pub fn i_052a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52A14;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $052A14: jsr $4fa70.l
pub fn i_052a14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52A1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52A1A);
    cpu.pc = 0x4FA70;
    16
}
/// $052A1A: bcc.w $52a24
pub fn i_052a1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52A1E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x52A24; return 10; }
    8
}
/// $052A1E: jmp $518.l
pub fn i_052a1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52A24;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $052A24: rts
pub fn i_052a24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52A26;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Wall_052a26  ($052A26, 206 B, props/destructible_0527ba.s)
/// $052A26: movea.l $3c(a6), a1
pub fn i_052a26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52A2A;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $052A2A: jsr $2942a.l
pub fn i_052a2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52A30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52A30);
    cpu.pc = 0x2942A;
    16
}
/// $052A30: move.w #$32, d1
pub fn i_052a30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52A34;
    cpu.ir = 0x323C;
    let v1: u32 = 0x32;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $052A34: jsr $236e.l
pub fn i_052a34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52A3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52A3A);
    cpu.pc = 0x236E;
    16
}
/// $052A3A: move.w #$20, $70(a6)
pub fn i_052a3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52A40;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $052A40: move.b #$ff, $32(a6)
pub fn i_052a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52A46;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $052A46: move.b #$ff, $33(a6)
pub fn i_052a46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52A4C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $052A4C: move.b #$0, $3a(a6)
pub fn i_052a4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52A52;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $052A52: move.w #$3c, $66(a6)
pub fn i_052a52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52A58;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $052A58: lea.l $2973b6.l, a0
pub fn i_052a58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52A5E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2973B6;
    20
}
/// $052A5E: jsr $28cd4.l
pub fn i_052a5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52A64;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52A64);
    cpu.pc = 0x28CD4;
    16
}
/// $052A64: lea.l $52a6a(pc), a1
pub fn i_052a64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52A68;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x52A6A;
    16
}
/// $052A68: move.l a1, (a6)
pub fn i_052a68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52A6A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $052A6A: jsr $2783a.l
pub fn i_052a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52A70;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52A70);
    cpu.pc = 0x2783A;
    16
}
/// $052A70: jsr $28d70.l
pub fn i_052a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52A76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52A76);
    cpu.pc = 0x28D70;
    16
}
/// $052A76: jsr $2870a.l
pub fn i_052a76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52A7C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52A7C);
    cpu.pc = 0x2870A;
    16
}
/// $052A7C: bcc.w $52acc
pub fn i_052a7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52A80;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x52ACC; return 10; }
    8
}
/// $052A80: bclr.b #$3, $13(a6)
pub fn i_052a80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52A86;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $052A86: move.w #$10bb, d0
pub fn i_052a86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52A8A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10BB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $052A8A: jsr $2352.l
pub fn i_052a8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52A90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52A90);
    cpu.pc = 0x2352;
    16
}
/// $052A90: addq.b #$1, $47(a6)
pub fn i_052a90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52A94;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x47);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $052A94: andi.b #$3, $47(a6)
pub fn i_052a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52A9A;
    cpu.ir = 0x22E;
    let ad1 = cpu.a[6].wrapping_add(0x47);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0x3) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $052A9A: lea.l $537ea(pc), a1
pub fn i_052a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52A9E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x537EA;
    16
}
/// $052A9E: jsr $6fe.l
pub fn i_052a9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52AA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52AA4);
    cpu.pc = 0x6FE;
    16
}
/// $052AA4: jsr $5dd02.l
pub fn i_052aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52AAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52AAA);
    cpu.pc = 0x5DD02;
    16
}
/// $052AAA: move.l #$10, d0
pub fn i_052aaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52AB0;
    cpu.ir = 0x203C;
    let v1: u32 = 0x10;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $052AB0: jsr $51a28.l
pub fn i_052ab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52AB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52AB6);
    cpu.pc = 0x51A28;
    16
}
/// $052AB6: btst.b #$3, $13(a6)
pub fn i_052ab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52ABC;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $052ABC: bne.w $52acc
pub fn i_052abc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52AC0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x52ACC; return 10; }
    8
}
/// $052AC0: lea.l $2973cc.l, a0
pub fn i_052ac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52AC6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2973CC;
    20
}
/// $052AC6: jsr $28cd4.l
pub fn i_052ac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52ACC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52ACC);
    cpu.pc = 0x28CD4;
    16
}
/// $052ACC: jsr $28758.l
pub fn i_052acc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52AD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52AD2);
    cpu.pc = 0x28758;
    16
}
/// $052AD2: bcc.w $52ae2
pub fn i_052ad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52AD6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x52AE2; return 10; }
    8
}
/// $052AD6: bclr.b #$0, $13(a6)
pub fn i_052ad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52ADC;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $052ADC: lea.l $52af4(pc), a1
pub fn i_052adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52AE0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x52AF4;
    16
}
/// $052AE0: move.l a1, (a6)
pub fn i_052ae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52AE2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $052AE2: jsr $4fa70.l
pub fn i_052ae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52AE8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52AE8);
    cpu.pc = 0x4FA70;
    16
}
/// $052AE8: bcc.w $52af2
pub fn i_052ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52AEC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x52AF2; return 10; }
    8
}
/// $052AEC: jmp $518.l
pub fn i_052aec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52AF2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $052AF2: rts
pub fn i_052af2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52AF4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_WallStage2_052af4  ($052AF4, 150 B, props/destructible_0527ba.s)
/// $052AF4: move.w #$3c, $66(a6)
pub fn i_052af4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52AFA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $052AFA: lea.l $297458.l, a0
pub fn i_052afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52B00;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x297458;
    20
}
/// $052B00: jsr $28cd4.l
pub fn i_052b00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52B06;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52B06);
    cpu.pc = 0x28CD4;
    16
}
/// $052B06: lea.l $52b0c(pc), a1
pub fn i_052b06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52B0A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x52B0C;
    16
}
/// $052B0A: move.l a1, (a6)
pub fn i_052b0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52B0C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $052B0C: jsr $2783a.l
pub fn i_052b0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52B12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52B12);
    cpu.pc = 0x2783A;
    16
}
/// $052B12: jsr $28d70.l
pub fn i_052b12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52B18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52B18);
    cpu.pc = 0x28D70;
    16
}
/// $052B18: btst.b #$0, $13(a6)
pub fn i_052b18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52B1E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $052B1E: bne.w $52b78
pub fn i_052b1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52B22;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x52B78; return 10; }
    8
}
/// $052B22: jsr $2870a.l
pub fn i_052b22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52B28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52B28);
    cpu.pc = 0x2870A;
    16
}
/// $052B28: bcc.w $52b78
pub fn i_052b28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52B2C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x52B78; return 10; }
    8
}
/// $052B2C: bclr.b #$3, $13(a6)
pub fn i_052b2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52B32;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $052B32: move.w #$10bb, d0
pub fn i_052b32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52B36;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10BB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $052B36: jsr $2352.l
pub fn i_052b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52B3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52B3C);
    cpu.pc = 0x2352;
    16
}
/// $052B3C: addq.b #$1, $47(a6)
pub fn i_052b3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52B40;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x47);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $052B40: andi.b #$3, $47(a6)
pub fn i_052b40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52B46;
    cpu.ir = 0x22E;
    let ad1 = cpu.a[6].wrapping_add(0x47);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0x3) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $052B46: lea.l $537ea(pc), a1
pub fn i_052b46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52B4A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x537EA;
    16
}
/// $052B4A: jsr $6fe.l
pub fn i_052b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52B50;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52B50);
    cpu.pc = 0x6FE;
    16
}
/// $052B50: jsr $5dd02.l
pub fn i_052b50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52B56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52B56);
    cpu.pc = 0x5DD02;
    16
}
/// $052B56: move.l #$10, d0
pub fn i_052b56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52B5C;
    cpu.ir = 0x203C;
    let v1: u32 = 0x10;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $052B5C: jsr $51a28.l
pub fn i_052b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52B62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52B62);
    cpu.pc = 0x51A28;
    16
}
/// $052B62: btst.b #$3, $13(a6)
pub fn i_052b62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52B68;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $052B68: bne.w $52b78
pub fn i_052b68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52B6C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x52B78; return 10; }
    8
}
/// $052B6C: lea.l $297474.l, a0
pub fn i_052b6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52B72;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x297474;
    20
}
/// $052B72: jsr $28cd4.l
pub fn i_052b72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52B78;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52B78);
    cpu.pc = 0x28CD4;
    16
}
/// $052B78: jsr $4fa70.l
pub fn i_052b78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52B7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52B7E);
    cpu.pc = 0x4FA70;
    16
}
/// $052B7E: bcc.w $52b88
pub fn i_052b7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52B82;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x52B88; return 10; }
    8
}
/// $052B82: jmp $518.l
pub fn i_052b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52B88;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $052B88: rts
pub fn i_052b88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52B8A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Large_052b8a  ($052B8A, 278 B, props/destructible_0527ba.s)
/// $052B8A: movea.l $3c(a6), a1
pub fn i_052b8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52B8E;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $052B8E: jsr $2942a.l
pub fn i_052b8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52B94;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52B94);
    cpu.pc = 0x2942A;
    16
}
/// $052B94: move.w #$31, d1
pub fn i_052b94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52B98;
    cpu.ir = 0x323C;
    let v1: u32 = 0x31;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $052B98: jsr $236e.l
pub fn i_052b98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52B9E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52B9E);
    cpu.pc = 0x236E;
    16
}
/// $052B9E: move.w #$10, $70(a6)
pub fn i_052b9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52BA4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $052BA4: move.b #$ff, $32(a6)
pub fn i_052ba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52BAA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $052BAA: move.b #$ff, $33(a6)
pub fn i_052baa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52BB0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $052BB0: move.b #$0, $3a(a6)
pub fn i_052bb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52BB6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $052BB6: move.w #$64, $66(a6)
pub fn i_052bb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52BBC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x64;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $052BBC: move.b #$0, $75(a6)
pub fn i_052bbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52BC2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $052BC2: lea.l $77228.l, a1
pub fn i_052bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52BC8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77228;
    20
}
/// $052BC8: jsr $4ae.l
pub fn i_052bc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52BCE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52BCE);
    cpu.pc = 0x4AE;
    16
}
/// $052BCE: jsr $5dd22.l
pub fn i_052bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52BD4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52BD4);
    cpu.pc = 0x5DD22;
    16
}
/// $052BD4: addi.w #$8, $22(a0)
pub fn i_052bd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52BDA;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $052BDA: move.b #$81, $98(a0)
pub fn i_052bda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52BE0;
    cpu.ir = 0x117C;
    let v1: u32 = 0x81;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $052BE0: move.b #$75, $99(a0)
pub fn i_052be0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52BE6;
    cpu.ir = 0x117C;
    let v1: u32 = 0x75;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $052BE6: lea.l $297500.l, a0
pub fn i_052be6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52BEC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x297500;
    20
}
/// $052BEC: jsr $28cd4.l
pub fn i_052bec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52BF2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52BF2);
    cpu.pc = 0x28CD4;
    16
}
/// $052BF2: lea.l $52bf8(pc), a1
pub fn i_052bf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52BF6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x52BF8;
    16
}
/// $052BF6: move.l a1, (a6)
pub fn i_052bf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52BF8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $052BF8: jsr $2783a.l
pub fn i_052bf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52BFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52BFE);
    cpu.pc = 0x2783A;
    16
}
/// $052BFE: jsr $28d70.l
pub fn i_052bfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52C04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52C04);
    cpu.pc = 0x28D70;
    16
}
/// $052C04: jsr $2870a.l
pub fn i_052c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52C0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52C0A);
    cpu.pc = 0x2870A;
    16
}
/// $052C0A: bcc.w $52c5c
pub fn i_052c0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52C0E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x52C5C; return 10; }
    8
}
/// $052C0E: lea.l $5e766.l, a0
pub fn i_052c0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52C14;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $052C14: jsr $5e770.l
pub fn i_052c14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52C1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52C1A);
    cpu.pc = 0x5E770;
    16
}
/// $052C1A: bclr.b #$3, $13(a6)
pub fn i_052c1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52C20;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $052C20: lea.l $77e10.l, a1
pub fn i_052c20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52C26;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77E10;
    20
}
/// $052C26: jsr $4ae.l
pub fn i_052c26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52C2C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52C2C);
    cpu.pc = 0x4AE;
    16
}
/// $052C2C: jsr $5dd22.l
pub fn i_052c2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52C32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52C32);
    cpu.pc = 0x5DD22;
    16
}
/// $052C32: move.w $54(a6), $22(a0)
pub fn i_052c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52C38;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x54);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $052C38: move.w $56(a6), $24(a0)
pub fn i_052c38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52C3E;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x56);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $052C3E: move.b $106f28.l, d0
pub fn i_052c3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52C44;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $052C44: btst.b #$0, d0
pub fn i_052c44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52C48;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $052C48: beq.w $52c5c
pub fn i_052c48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52C4C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x52C5C; return 10; }
    8
}
/// $052C4C: lea.l $53894(pc), a1
pub fn i_052c4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52C50;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x53894;
    16
}
/// $052C50: jsr $4ae.l
pub fn i_052c50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52C56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52C56);
    cpu.pc = 0x4AE;
    16
}
/// $052C56: jsr $5dd22.l
pub fn i_052c56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52C5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52C5C);
    cpu.pc = 0x5DD22;
    16
}
/// $052C5C: jsr $28758.l
pub fn i_052c5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52C62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52C62);
    cpu.pc = 0x28758;
    16
}
/// $052C62: bcc.w $52c88
pub fn i_052c62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52C66;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x52C88; return 10; }
    8
}
/// $052C66: bclr.b #$0, $13(a6)
pub fn i_052c66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52C6C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $052C6C: move.w #$1024, d0
pub fn i_052c6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52C70;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1024;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $052C70: jsr $2352.l
pub fn i_052c70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52C76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52C76);
    cpu.pc = 0x2352;
    16
}
/// $052C76: lea.l $297fe4.l, a1
pub fn i_052c76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52C7C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x297FE4;
    20
}
/// $052C7C: jsr $77c7e.l
pub fn i_052c7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52C82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52C82);
    cpu.pc = 0x77C7E;
    16
}
/// $052C82: lea.l $52ca0(pc), a1
pub fn i_052c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52C86;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x52CA0;
    16
}
/// $052C86: move.l a1, (a6)
pub fn i_052c86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52C88;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $052C88: jsr $4fa70.l
pub fn i_052c88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52C8E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52C8E);
    cpu.pc = 0x4FA70;
    16
}
/// $052C8E: bcc.w $52c9e
pub fn i_052c8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52C92;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x52C9E; return 10; }
    8
}
/// $052C92: move.b #$ff, $75(a6)
pub fn i_052c92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52C98;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $052C98: jmp $518.l
pub fn i_052c98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52C9E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $052C9E: rts
pub fn i_052c9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52CA0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_LargeWreck_052ca0  ($052CA0, 74 B, props/destructible_0527ba.s)
/// $052CA0: move.l #$1000, d0
pub fn i_052ca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52CA6;
    cpu.ir = 0x203C;
    let v1: u32 = 0x1000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $052CA6: jsr $51a28.l
pub fn i_052ca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52CAC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52CAC);
    cpu.pc = 0x51A28;
    16
}
/// $052CAC: lea.l $ffff.w, a0
pub fn i_052cac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52CB0;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $052CB0: move.l a0, $48(a6)
pub fn i_052cb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52CB4;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $052CB4: move.b #$ff, $75(a6)
pub fn i_052cb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52CBA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $052CBA: lea.l $297516.l, a0
pub fn i_052cba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52CC0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x297516;
    20
}
/// $052CC0: jsr $28cd4.l
pub fn i_052cc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52CC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52CC6);
    cpu.pc = 0x28CD4;
    16
}
/// $052CC6: lea.l $52ccc(pc), a1
pub fn i_052cc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52CCA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x52CCC;
    16
}
/// $052CCA: move.l a1, (a6)
pub fn i_052cca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52CCC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $052CCC: jsr $2783a.l
pub fn i_052ccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52CD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52CD2);
    cpu.pc = 0x2783A;
    16
}
/// $052CD2: jsr $28d70.l
pub fn i_052cd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52CD8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52CD8);
    cpu.pc = 0x28D70;
    16
}
/// $052CD8: jsr $4fa70.l
pub fn i_052cd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52CDE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52CDE);
    cpu.pc = 0x4FA70;
    16
}
/// $052CDE: bcc.w $52ce8
pub fn i_052cde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52CE2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x52CE8; return 10; }
    8
}
/// $052CE2: jmp $518.l
pub fn i_052ce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52CE8;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $052CE8: rts
pub fn i_052ce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52CEA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Explosive_052cea  ($052CEA, 228 B, props/destructible_0527ba.s)
/// $052CEA: movea.l $3c(a6), a1
pub fn i_052cea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52CEE;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $052CEE: jsr $2942a.l
pub fn i_052cee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52CF4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52CF4);
    cpu.pc = 0x2942A;
    16
}
/// $052CF4: move.w #$30, d1
pub fn i_052cf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52CF8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $052CF8: jsr $236e.l
pub fn i_052cf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52CFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52CFE);
    cpu.pc = 0x236E;
    16
}
/// $052CFE: move.w #$10, $70(a6)
pub fn i_052cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52D04;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $052D04: move.b #$ff, $32(a6)
pub fn i_052d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52D0A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $052D0A: move.b #$ff, $33(a6)
pub fn i_052d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52D10;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $052D10: move.b #$0, $3a(a6)
pub fn i_052d10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52D16;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $052D16: move.w #$14, $66(a6)
pub fn i_052d16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52D1C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $052D1C: lea.l $297526.l, a0
pub fn i_052d1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52D22;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x297526;
    20
}
/// $052D22: jsr $28cd4.l
pub fn i_052d22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52D28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52D28);
    cpu.pc = 0x28CD4;
    16
}
/// $052D28: lea.l $52d2e(pc), a1
pub fn i_052d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52D2C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x52D2E;
    16
}
/// $052D2C: move.l a1, (a6)
pub fn i_052d2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52D2E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $052D2E: jsr $2783a.l
pub fn i_052d2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52D34;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52D34);
    cpu.pc = 0x2783A;
    16
}
/// $052D34: jsr $28d70.l
pub fn i_052d34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52D3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52D3A);
    cpu.pc = 0x28D70;
    16
}
/// $052D3A: jsr $2870a.l
pub fn i_052d3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52D40;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52D40);
    cpu.pc = 0x2870A;
    16
}
/// $052D40: bcc.w $52d74
pub fn i_052d40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52D44;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x52D74; return 10; }
    8
}
/// $052D44: lea.l $5e766.l, a0
pub fn i_052d44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52D4A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $052D4A: jsr $5e770.l
pub fn i_052d4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52D50;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52D50);
    cpu.pc = 0x5E770;
    16
}
/// $052D50: bclr.b #$3, $13(a6)
pub fn i_052d50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52D56;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $052D56: lea.l $77e10.l, a1
pub fn i_052d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52D5C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77E10;
    20
}
/// $052D5C: jsr $4ae.l
pub fn i_052d5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52D62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52D62);
    cpu.pc = 0x4AE;
    16
}
/// $052D62: jsr $5dd22.l
pub fn i_052d62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52D68;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52D68);
    cpu.pc = 0x5DD22;
    16
}
/// $052D68: move.w $54(a6), $22(a0)
pub fn i_052d68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52D6E;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x54);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $052D6E: move.w $56(a6), $24(a0)
pub fn i_052d6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52D74;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x56);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $052D74: jsr $28758.l
pub fn i_052d74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52D7A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52D7A);
    cpu.pc = 0x28758;
    16
}
/// $052D7A: bcc.w $52dbc
pub fn i_052d7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52D7E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x52DBC; return 10; }
    8
}
/// $052D7E: bclr.b #$0, $13(a6)
pub fn i_052d7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52D84;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $052D84: move.w #$1076, d0
pub fn i_052d84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52D88;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1076;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $052D88: jsr $2352.l
pub fn i_052d88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52D8E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52D8E);
    cpu.pc = 0x2352;
    16
}
/// $052D8E: lea.l $297fe4.l, a1
pub fn i_052d8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52D94;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x297FE4;
    20
}
/// $052D94: jsr $77c7e.l
pub fn i_052d94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52D9A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52D9A);
    cpu.pc = 0x77C7E;
    16
}
/// $052D9A: lea.l $29874a.l, a0
pub fn i_052d9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52DA0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29874A;
    20
}
/// $052DA0: move.l a0, $4c(a6)
pub fn i_052da0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52DA4;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $052DA4: jsr $283ca.l
pub fn i_052da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52DAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52DAA);
    cpu.pc = 0x283CA;
    16
}
/// $052DAA: jsr $283ca.l
pub fn i_052daa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52DB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52DB0);
    cpu.pc = 0x283CA;
    16
}
/// $052DB0: jsr $283d8.l
pub fn i_052db0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52DB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52DB6);
    cpu.pc = 0x283D8;
    16
}
/// $052DB6: lea.l $52dce(pc), a1
pub fn i_052db6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52DBA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x52DCE;
    16
}
/// $052DBA: move.l a1, (a6)
pub fn i_052dba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52DBC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $052DBC: jsr $4fa70.l
pub fn i_052dbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52DC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52DC2);
    cpu.pc = 0x4FA70;
    16
}
/// $052DC2: bcc.w $52dcc
pub fn i_052dc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52DC6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x52DCC; return 10; }
    8
}
/// $052DC6: jmp $518.l
pub fn i_052dc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52DCC;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $052DCC: rts
pub fn i_052dcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52DCE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_ExplosiveWreck_052dce  ($052DCE, 82 B, props/destructible_0527ba.s)
/// $052DCE: move.l #$500, d0
pub fn i_052dce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52DD4;
    cpu.ir = 0x203C;
    let v1: u32 = 0x500;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $052DD4: jsr $51a28.l
pub fn i_052dd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52DDA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52DDA);
    cpu.pc = 0x51A28;
    16
}
/// $052DDA: lea.l $ffff.w, a0
pub fn i_052dda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52DDE;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $052DDE: move.l a0, $4c(a6)
pub fn i_052dde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52DE2;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $052DE2: jsr $283ca.l
pub fn i_052de2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52DE8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52DE8);
    cpu.pc = 0x283CA;
    16
}
/// $052DE8: lea.l $ffff.w, a0
pub fn i_052de8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52DEC;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $052DEC: move.l a0, $48(a6)
pub fn i_052dec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52DF0;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $052DF0: lea.l $29753c.l, a0
pub fn i_052df0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52DF6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29753C;
    20
}
/// $052DF6: jsr $28cd4.l
pub fn i_052df6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52DFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52DFC);
    cpu.pc = 0x28CD4;
    16
}
/// $052DFC: lea.l $52e02(pc), a1
pub fn i_052dfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52E00;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x52E02;
    16
}
/// $052E00: move.l a1, (a6)
pub fn i_052e00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52E02;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $052E02: jsr $2783a.l
pub fn i_052e02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52E08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52E08);
    cpu.pc = 0x2783A;
    16
}
/// $052E08: jsr $28d70.l
pub fn i_052e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52E0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52E0E);
    cpu.pc = 0x28D70;
    16
}
/// $052E0E: jsr $4fa70.l
pub fn i_052e0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52E14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52E14);
    cpu.pc = 0x4FA70;
    16
}
/// $052E14: bcc.w $52e1e
pub fn i_052e14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52E18;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x52E1E; return 10; }
    8
}
/// $052E18: jmp $518.l
pub fn i_052e18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52E1E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $052E1E: rts
pub fn i_052e1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52E20;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_HouseVariants_052e20  ($052E20, 684 B, props/destructible_0527ba.s)
/// $052E20: movea.l $3c(a6), a1
pub fn i_052e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52E24;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $052E24: jsr $2942a.l
pub fn i_052e24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52E2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52E2A);
    cpu.pc = 0x2942A;
    16
}
/// $052E2A: lea.l $2983f0.l, a2
pub fn i_052e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52E30;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2983F0;
    20
}
/// $052E30: move.l a2, $80(a6)
pub fn i_052e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52E34;
    cpu.ir = 0x2D4A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $052E34: lea.l $29754c.l, a0
pub fn i_052e34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52E3A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29754C;
    20
}
/// $052E3A: jsr $28cd4.l
pub fn i_052e3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52E40;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52E40);
    cpu.pc = 0x28CD4;
    16
}
/// $052E40: bra.w $52fc0
pub fn i_052e40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52E44;
    cpu.ir = 0x6000;
    cpu.pc = 0x52FC0;
    10
}
/// $052E44: movea.l $3c(a6), a1
pub fn i_052e44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52E48;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $052E48: jsr $2942a.l
pub fn i_052e48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52E4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52E4E);
    cpu.pc = 0x2942A;
    16
}
/// $052E4E: lea.l $298404.l, a2
pub fn i_052e4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52E54;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x298404;
    20
}
/// $052E54: move.l a2, $80(a6)
pub fn i_052e54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52E58;
    cpu.ir = 0x2D4A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $052E58: lea.l $29754c.l, a0
pub fn i_052e58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52E5E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29754C;
    20
}
/// $052E5E: jsr $28cd4.l
pub fn i_052e5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52E64;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52E64);
    cpu.pc = 0x28CD4;
    16
}
/// $052E64: bra.w $52fc0
pub fn i_052e64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52E68;
    cpu.ir = 0x6000;
    cpu.pc = 0x52FC0;
    10
}
/// $052E68: movea.l $3c(a6), a1
pub fn i_052e68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52E6C;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $052E6C: jsr $2942a.l
pub fn i_052e6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52E72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52E72);
    cpu.pc = 0x2942A;
    16
}
/// $052E72: lea.l $298418.l, a2
pub fn i_052e72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52E78;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x298418;
    20
}
/// $052E78: move.l a2, $80(a6)
pub fn i_052e78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52E7C;
    cpu.ir = 0x2D4A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $052E7C: lea.l $297562.l, a0
pub fn i_052e7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52E82;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x297562;
    20
}
/// $052E82: jsr $28cd4.l
pub fn i_052e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52E88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52E88);
    cpu.pc = 0x28CD4;
    16
}
/// $052E88: bra.w $52fc0
pub fn i_052e88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52E8C;
    cpu.ir = 0x6000;
    cpu.pc = 0x52FC0;
    10
}
/// $052E8C: movea.l $3c(a6), a1
pub fn i_052e8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52E90;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $052E90: jsr $2942a.l
pub fn i_052e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52E96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52E96);
    cpu.pc = 0x2942A;
    16
}
/// $052E96: lea.l $29842c.l, a2
pub fn i_052e96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52E9C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29842C;
    20
}
/// $052E9C: move.l a2, $80(a6)
pub fn i_052e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52EA0;
    cpu.ir = 0x2D4A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $052EA0: lea.l $38f14.l, a1
pub fn i_052ea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52EA6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x38F14;
    20
}
/// $052EA6: jsr $4ae.l
pub fn i_052ea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52EAC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52EAC);
    cpu.pc = 0x4AE;
    16
}
/// $052EAC: jsr $5dd02.l
pub fn i_052eac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52EB2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52EB2);
    cpu.pc = 0x5DD02;
    16
}
/// $052EB2: addi.w #$7, $22(a0)
pub fn i_052eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52EB8;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x7, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $052EB8: addi.w #$20, $24(a0)
pub fn i_052eb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52EBE;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $052EBE: move.b #$1, $98(a0)
pub fn i_052ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52EC4;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $052EC4: lea.l $38f48.l, a1
pub fn i_052ec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52ECA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x38F48;
    20
}
/// $052ECA: jsr $4ae.l
pub fn i_052eca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52ED0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52ED0);
    cpu.pc = 0x4AE;
    16
}
/// $052ED0: jsr $5dd02.l
pub fn i_052ed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52ED6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52ED6);
    cpu.pc = 0x5DD02;
    16
}
/// $052ED6: addi.w #$30, $22(a0)
pub fn i_052ed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52EDC;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x30, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $052EDC: addi.w #$20, $24(a0)
pub fn i_052edc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52EE2;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $052EE2: move.b #$1, $98(a0)
pub fn i_052ee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52EE8;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $052EE8: lea.l $2986fc.l, a1
pub fn i_052ee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52EEE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2986FC;
    20
}
/// $052EEE: jsr $43fac.l
pub fn i_052eee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52EF4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52EF4);
    cpu.pc = 0x43FAC;
    16
}
/// $052EF4: movea.l $80(a6), a2
pub fn i_052ef4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52EF8;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $052EF8: jsr $5022a.l
pub fn i_052ef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52EFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52EFE);
    cpu.pc = 0x5022A;
    16
}
/// $052EFE: jmp $518.l
pub fn i_052efe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52F04;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $052F04: rts
pub fn i_052f04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52F06;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $052F06: lea.l $297562.l, a0
pub fn i_052f06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52F0C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x297562;
    20
}
/// $052F0C: jsr $28cd4.l
pub fn i_052f0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52F12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52F12);
    cpu.pc = 0x28CD4;
    16
}
/// $052F12: bra.w $52fc0
pub fn i_052f12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52F16;
    cpu.ir = 0x6000;
    cpu.pc = 0x52FC0;
    10
}
/// $052F16: movea.l $3c(a6), a1
pub fn i_052f16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52F1A;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $052F1A: jsr $2942a.l
pub fn i_052f1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52F20;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52F20);
    cpu.pc = 0x2942A;
    16
}
/// $052F20: lea.l $298440.l, a2
pub fn i_052f20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52F26;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x298440;
    20
}
/// $052F26: move.l a2, $80(a6)
pub fn i_052f26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52F2A;
    cpu.ir = 0x2D4A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $052F2A: lea.l $297578.l, a0
pub fn i_052f2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52F30;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x297578;
    20
}
/// $052F30: jsr $28cd4.l
pub fn i_052f30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52F36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52F36);
    cpu.pc = 0x28CD4;
    16
}
/// $052F36: bra.w $52fc0
pub fn i_052f36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52F3A;
    cpu.ir = 0x6000;
    cpu.pc = 0x52FC0;
    10
}
/// $052F3A: movea.l $3c(a6), a1
pub fn i_052f3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52F3E;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $052F3E: jsr $2942a.l
pub fn i_052f3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52F44;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52F44);
    cpu.pc = 0x2942A;
    16
}
/// $052F44: lea.l $298454.l, a2
pub fn i_052f44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52F4A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x298454;
    20
}
/// $052F4A: move.l a2, $80(a6)
pub fn i_052f4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52F4E;
    cpu.ir = 0x2D4A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $052F4E: lea.l $38f14.l, a1
pub fn i_052f4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52F54;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x38F14;
    20
}
/// $052F54: jsr $4ae.l
pub fn i_052f54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52F5A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52F5A);
    cpu.pc = 0x4AE;
    16
}
/// $052F5A: jsr $5dd02.l
pub fn i_052f5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52F60;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52F60);
    cpu.pc = 0x5DD02;
    16
}
/// $052F60: addi.w #$7, $22(a0)
pub fn i_052f60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52F66;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x7, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $052F66: addi.w #$20, $24(a0)
pub fn i_052f66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52F6C;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $052F6C: move.b #$1, $98(a0)
pub fn i_052f6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52F72;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $052F72: lea.l $38f48.l, a1
pub fn i_052f72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52F78;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x38F48;
    20
}
/// $052F78: jsr $4ae.l
pub fn i_052f78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52F7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52F7E);
    cpu.pc = 0x4AE;
    16
}
/// $052F7E: jsr $5dd02.l
pub fn i_052f7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52F84;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52F84);
    cpu.pc = 0x5DD02;
    16
}
/// $052F84: addi.w #$30, $22(a0)
pub fn i_052f84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52F8A;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x30, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $052F8A: addi.w #$20, $24(a0)
pub fn i_052f8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52F90;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $052F90: move.b #$1, $98(a0)
pub fn i_052f90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52F96;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $052F96: lea.l $2986fc.l, a1
pub fn i_052f96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52F9C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2986FC;
    20
}
/// $052F9C: jsr $43fac.l
pub fn i_052f9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52FA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52FA2);
    cpu.pc = 0x43FAC;
    16
}
/// $052FA2: movea.l $80(a6), a2
pub fn i_052fa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52FA6;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $052FA6: jsr $5022a.l
pub fn i_052fa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52FAC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52FAC);
    cpu.pc = 0x5022A;
    16
}
/// $052FAC: jmp $518.l
pub fn i_052fac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52FB2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $052FB2: rts
pub fn i_052fb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52FB4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $052FB4: lea.l $297578.l, a0
pub fn i_052fb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52FBA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x297578;
    20
}
/// $052FBA: jsr $28cd4.l
pub fn i_052fba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52FC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52FC0);
    cpu.pc = 0x28CD4;
    16
}
/// $052FC0: move.w #$40, $70(a6)
pub fn i_052fc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52FC6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $052FC6: move.w #$28, $66(a6)
pub fn i_052fc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52FCC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $052FCC: move.b #$0, $21(a6)
pub fn i_052fcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52FD2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $052FD2: lea.l $52fd8(pc), a1
pub fn i_052fd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52FD6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x52FD8;
    16
}
/// $052FD6: move.l a1, (a6)
pub fn i_052fd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52FD8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $052FD8: jsr $2783a.l
pub fn i_052fd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52FDE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52FDE);
    cpu.pc = 0x2783A;
    16
}
/// $052FDE: jsr $28d70.l
pub fn i_052fde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52FE4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52FE4);
    cpu.pc = 0x28D70;
    16
}
/// $052FE4: jsr $2870a.l
pub fn i_052fe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52FEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52FEA);
    cpu.pc = 0x2870A;
    16
}
/// $052FEA: bcc.w $53000
pub fn i_052fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52FEE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x53000; return 10; }
    8
}
/// $052FEE: lea.l $5e766.l, a0
pub fn i_052fee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52FF4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $052FF4: jsr $5e770.l
pub fn i_052ff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x52FFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x52FFA);
    cpu.pc = 0x5E770;
    16
}
/// $052FFA: bclr.b #$3, $13(a6)
pub fn i_052ffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53000;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $053000: jsr $28758.l
pub fn i_053000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53006;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53006);
    cpu.pc = 0x28758;
    16
}
/// $053006: bcc.w $530ba
pub fn i_053006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5300A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x530BA; return 10; }
    8
}
/// $05300A: move.w #$1035, d0
pub fn i_05300a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5300E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1035;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05300E: jsr $2352.l
pub fn i_05300e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53014;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53014);
    cpu.pc = 0x2352;
    16
}
/// $053014: lea.l $ffff.w, a0
pub fn i_053014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53018;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $053018: move.l a0, $48(a6)
pub fn i_053018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5301C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $05301C: move.l #$300, d0
pub fn i_05301c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53022;
    cpu.ir = 0x203C;
    let v1: u32 = 0x300;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $053022: jsr $51a28.l
pub fn i_053022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53028;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53028);
    cpu.pc = 0x51A28;
    16
}
/// $053028: lea.l $38f14.l, a1
pub fn i_053028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5302E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x38F14;
    20
}
/// $05302E: jsr $4ae.l
pub fn i_05302e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53034;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53034);
    cpu.pc = 0x4AE;
    16
}
/// $053034: jsr $5dd02.l
pub fn i_053034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5303A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5303A);
    cpu.pc = 0x5DD02;
    16
}
/// $05303A: addi.w #$7, $22(a0)
pub fn i_05303a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53040;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x7, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $053040: addi.w #$20, $24(a0)
pub fn i_053040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53046;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $053046: move.b #$1, $98(a0)
pub fn i_053046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5304C;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05304C: lea.l $38f48.l, a1
pub fn i_05304c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53052;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x38F48;
    20
}
/// $053052: jsr $4ae.l
pub fn i_053052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53058;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53058);
    cpu.pc = 0x4AE;
    16
}
/// $053058: jsr $5dd02.l
pub fn i_053058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5305E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5305E);
    cpu.pc = 0x5DD02;
    16
}
/// $05305E: addi.w #$30, $22(a0)
pub fn i_05305e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53064;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x30, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $053064: addi.w #$20, $24(a0)
pub fn i_053064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5306A;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $05306A: move.b #$1, $98(a0)
pub fn i_05306a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53070;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $053070: jsr $434dc.l
pub fn i_053070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53076;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53076);
    cpu.pc = 0x434DC;
    16
}
/// $053076: lea.l $2986fc.l, a1
pub fn i_053076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5307C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2986FC;
    20
}
/// $05307C: jsr $43fac.l
pub fn i_05307c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53082;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53082);
    cpu.pc = 0x43FAC;
    16
}
/// $053082: subi.w #$10, $24(a6)
pub fn i_053082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53088;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $053088: lea.l $297fc0.l, a1
pub fn i_053088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5308E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x297FC0;
    20
}
/// $05308E: jsr $77c7e.l
pub fn i_05308e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53094;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53094);
    cpu.pc = 0x77C7E;
    16
}
/// $053094: lea.l $297fd2.l, a1
pub fn i_053094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5309A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x297FD2;
    20
}
/// $05309A: jsr $77c7e.l
pub fn i_05309a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x530A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x530A0);
    cpu.pc = 0x77C7E;
    16
}
/// $0530A0: lea.l $298086.l, a1
pub fn i_0530a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x530A6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x298086;
    20
}
/// $0530A6: jsr $77c7e.l
pub fn i_0530a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x530AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x530AC);
    cpu.pc = 0x77C7E;
    16
}
/// $0530AC: movea.l $80(a6), a2
pub fn i_0530ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x530B0;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $0530B0: jsr $5022a.l
pub fn i_0530b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x530B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x530B6);
    cpu.pc = 0x5022A;
    16
}
/// $0530B6: bra.w $530c4
pub fn i_0530b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x530BA;
    cpu.ir = 0x6000;
    cpu.pc = 0x530C4;
    10
}
/// $0530BA: jsr $4fa70.l
pub fn i_0530ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x530C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x530C0);
    cpu.pc = 0x4FA70;
    16
}
/// $0530C0: bcc.w $530ca
pub fn i_0530c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x530C4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x530CA; return 10; }
    8
}
/// $0530C4: jmp $518.l
pub fn i_0530c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x530CA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0530CA: rts
pub fn i_0530ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x530CC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_HutVariants_0530cc  ($0530CC, 570 B, props/destructible_0527ba.s)
/// $0530CC: movea.l $3c(a6), a1
pub fn i_0530cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x530D0;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0530D0: jsr $2942a.l
pub fn i_0530d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x530D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x530D6);
    cpu.pc = 0x2942A;
    16
}
/// $0530D6: move.b #$0, $21(a6)
pub fn i_0530d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x530DC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0530DC: lea.l $298468.l, a2
pub fn i_0530dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x530E2;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x298468;
    20
}
/// $0530E2: move.l a2, $80(a6)
pub fn i_0530e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x530E6;
    cpu.ir = 0x2D4A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0530E6: lea.l $29758e.l, a0
pub fn i_0530e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x530EC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29758E;
    20
}
/// $0530EC: jsr $28cd4.l
pub fn i_0530ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x530F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x530F2);
    cpu.pc = 0x28CD4;
    16
}
/// $0530F2: bra.w $5326c
pub fn i_0530f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x530F6;
    cpu.ir = 0x6000;
    cpu.pc = 0x5326C;
    10
}
/// $0530F6: movea.l $3c(a6), a1
pub fn i_0530f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x530FA;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0530FA: jsr $2942a.l
pub fn i_0530fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53100;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53100);
    cpu.pc = 0x2942A;
    16
}
/// $053100: move.b #$0, $21(a6)
pub fn i_053100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53106;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $053106: lea.l $29847c.l, a2
pub fn i_053106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5310C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29847C;
    20
}
/// $05310C: move.l a2, $80(a6)
pub fn i_05310c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53110;
    cpu.ir = 0x2D4A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $053110: lea.l $29758e.l, a0
pub fn i_053110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53116;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29758E;
    20
}
/// $053116: jsr $28cd4.l
pub fn i_053116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5311C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5311C);
    cpu.pc = 0x28CD4;
    16
}
/// $05311C: bra.w $5326c
pub fn i_05311c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53120;
    cpu.ir = 0x6000;
    cpu.pc = 0x5326C;
    10
}
/// $053120: movea.l $3c(a6), a1
pub fn i_053120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53124;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $053124: jsr $2942a.l
pub fn i_053124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5312A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5312A);
    cpu.pc = 0x2942A;
    16
}
/// $05312A: move.b #$1, $21(a6)
pub fn i_05312a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53130;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $053130: lea.l $298490.l, a2
pub fn i_053130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53136;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x298490;
    20
}
/// $053136: move.l a2, $80(a6)
pub fn i_053136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5313A;
    cpu.ir = 0x2D4A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $05313A: lea.l $2975a4.l, a0
pub fn i_05313a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53140;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2975A4;
    20
}
/// $053140: jsr $28cd4.l
pub fn i_053140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53146;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53146);
    cpu.pc = 0x28CD4;
    16
}
/// $053146: bra.w $5326c
pub fn i_053146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5314A;
    cpu.ir = 0x6000;
    cpu.pc = 0x5326C;
    10
}
/// $05314A: movea.l $3c(a6), a1
pub fn i_05314a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5314E;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $05314E: jsr $2942a.l
pub fn i_05314e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53154;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53154);
    cpu.pc = 0x2942A;
    16
}
/// $053154: move.b #$1, $21(a6)
pub fn i_053154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5315A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05315A: lea.l $2984a4.l, a2
pub fn i_05315a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53160;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2984A4;
    20
}
/// $053160: move.l a2, $80(a6)
pub fn i_053160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53164;
    cpu.ir = 0x2D4A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $053164: lea.l $2975a4.l, a0
pub fn i_053164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5316A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2975A4;
    20
}
/// $05316A: jsr $28cd4.l
pub fn i_05316a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53170;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53170);
    cpu.pc = 0x28CD4;
    16
}
/// $053170: bra.w $5326c
pub fn i_053170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53174;
    cpu.ir = 0x6000;
    cpu.pc = 0x5326C;
    10
}
/// $053174: movea.l $3c(a6), a1
pub fn i_053174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53178;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $053178: jsr $2942a.l
pub fn i_053178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5317E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5317E);
    cpu.pc = 0x2942A;
    16
}
/// $05317E: move.b #$2, $21(a6)
pub fn i_05317e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53184;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $053184: lea.l $2984b8.l, a2
pub fn i_053184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5318A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2984B8;
    20
}
/// $05318A: move.l a2, $80(a6)
pub fn i_05318a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5318E;
    cpu.ir = 0x2D4A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $05318E: lea.l $2975ba.l, a0
pub fn i_05318e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53194;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2975BA;
    20
}
/// $053194: jsr $28cd4.l
pub fn i_053194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5319A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5319A);
    cpu.pc = 0x28CD4;
    16
}
/// $05319A: bra.w $5326c
pub fn i_05319a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5319E;
    cpu.ir = 0x6000;
    cpu.pc = 0x5326C;
    10
}
/// $05319E: movea.l $3c(a6), a1
pub fn i_05319e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x531A2;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0531A2: jsr $2942a.l
pub fn i_0531a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x531A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x531A8);
    cpu.pc = 0x2942A;
    16
}
/// $0531A8: move.b #$2, $21(a6)
pub fn i_0531a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x531AE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0531AE: lea.l $2984cc.l, a2
pub fn i_0531ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x531B4;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2984CC;
    20
}
/// $0531B4: move.l a2, $80(a6)
pub fn i_0531b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x531B8;
    cpu.ir = 0x2D4A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0531B8: lea.l $2975ba.l, a0
pub fn i_0531b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x531BE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2975BA;
    20
}
/// $0531BE: jsr $28cd4.l
pub fn i_0531be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x531C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x531C4);
    cpu.pc = 0x28CD4;
    16
}
/// $0531C4: bra.w $5326c
pub fn i_0531c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x531C8;
    cpu.ir = 0x6000;
    cpu.pc = 0x5326C;
    10
}
/// $0531C8: movea.l $3c(a6), a1
pub fn i_0531c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x531CC;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0531CC: jsr $2942a.l
pub fn i_0531cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x531D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x531D2);
    cpu.pc = 0x2942A;
    16
}
/// $0531D2: move.b #$3, $21(a6)
pub fn i_0531d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x531D8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0531D8: lea.l $2984e0.l, a2
pub fn i_0531d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x531DE;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2984E0;
    20
}
/// $0531DE: move.l a2, $80(a6)
pub fn i_0531de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x531E2;
    cpu.ir = 0x2D4A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0531E2: lea.l $2975d0.l, a0
pub fn i_0531e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x531E8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2975D0;
    20
}
/// $0531E8: jsr $28cd4.l
pub fn i_0531e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x531EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x531EE);
    cpu.pc = 0x28CD4;
    16
}
/// $0531EE: bra.w $5326c
pub fn i_0531ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x531F2;
    cpu.ir = 0x6000;
    cpu.pc = 0x5326C;
    10
}
/// $0531F2: movea.l $3c(a6), a1
pub fn i_0531f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x531F6;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0531F6: jsr $2942a.l
pub fn i_0531f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x531FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x531FC);
    cpu.pc = 0x2942A;
    16
}
/// $0531FC: move.b #$3, $21(a6)
pub fn i_0531fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53202;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $053202: lea.l $2984f4.l, a2
pub fn i_053202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53208;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2984F4;
    20
}
/// $053208: move.l a2, $80(a6)
pub fn i_053208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5320C;
    cpu.ir = 0x2D4A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $05320C: lea.l $2975d0.l, a0
pub fn i_05320c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53212;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2975D0;
    20
}
/// $053212: jsr $28cd4.l
pub fn i_053212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53218;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53218);
    cpu.pc = 0x28CD4;
    16
}
/// $053218: bra.w $5326c
pub fn i_053218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5321C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5326C;
    10
}
/// $05321C: movea.l $3c(a6), a1
pub fn i_05321c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53220;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $053220: jsr $2942a.l
pub fn i_053220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53226;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53226);
    cpu.pc = 0x2942A;
    16
}
/// $053226: move.b #$4, $21(a6)
pub fn i_053226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5322C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05322C: lea.l $298508.l, a2
pub fn i_05322c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53232;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x298508;
    20
}
/// $053232: move.l a2, $80(a6)
pub fn i_053232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53236;
    cpu.ir = 0x2D4A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $053236: lea.l $2975e6.l, a0
pub fn i_053236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5323C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2975E6;
    20
}
/// $05323C: jsr $28cd4.l
pub fn i_05323c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53242;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53242);
    cpu.pc = 0x28CD4;
    16
}
/// $053242: bra.w $5326c
pub fn i_053242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53246;
    cpu.ir = 0x6000;
    cpu.pc = 0x5326C;
    10
}
/// $053246: movea.l $3c(a6), a1
pub fn i_053246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5324A;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $05324A: jsr $2942a.l
pub fn i_05324a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53250;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53250);
    cpu.pc = 0x2942A;
    16
}
/// $053250: move.b #$4, $21(a6)
pub fn i_053250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53256;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $053256: lea.l $29851c.l, a2
pub fn i_053256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5325C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29851C;
    20
}
/// $05325C: move.l a2, $80(a6)
pub fn i_05325c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53260;
    cpu.ir = 0x2D4A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $053260: lea.l $2975e6.l, a0
pub fn i_053260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53266;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2975E6;
    20
}
/// $053266: jsr $28cd4.l
pub fn i_053266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5326C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5326C);
    cpu.pc = 0x28CD4;
    16
}
/// $05326C: move.w #$40, $70(a6)
pub fn i_05326c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53272;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $053272: move.w #$28, $66(a6)
pub fn i_053272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53278;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $053278: lea.l $5327e(pc), a1
pub fn i_053278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5327C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5327E;
    16
}
/// $05327C: move.l a1, (a6)
pub fn i_05327c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5327E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05327E: jsr $2783a.l
pub fn i_05327e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53284;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53284);
    cpu.pc = 0x2783A;
    16
}
/// $053284: jsr $28d70.l
pub fn i_053284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5328A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5328A);
    cpu.pc = 0x28D70;
    16
}
/// $05328A: jsr $2870a.l
pub fn i_05328a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53290;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53290);
    cpu.pc = 0x2870A;
    16
}
/// $053290: bcc.w $532a6
pub fn i_053290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53294;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x532A6; return 10; }
    8
}
/// $053294: lea.l $5e766.l, a0
pub fn i_053294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5329A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $05329A: jsr $5e770.l
pub fn i_05329a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x532A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x532A0);
    cpu.pc = 0x5E770;
    16
}
/// $0532A0: bclr.b #$3, $13(a6)
pub fn i_0532a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x532A6;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0532A6: jsr $28758.l
pub fn i_0532a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x532AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x532AC);
    cpu.pc = 0x28758;
    16
}
/// $0532AC: bcc.w $532f4
pub fn i_0532ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x532B0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x532F4; return 10; }
    8
}
/// $0532B0: jsr $53e0c(pc)
pub fn i_0532b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x532B4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x532B4);
    cpu.pc = 0x53E0C;
    16
}
/// $0532B4: lea.l $ffff.w, a0
pub fn i_0532b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x532B8;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $0532B8: move.l a0, $48(a6)
pub fn i_0532b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x532BC;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0532BC: move.l #$500, d0
pub fn i_0532bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x532C2;
    cpu.ir = 0x203C;
    let v1: u32 = 0x500;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0532C2: jsr $51a28.l
pub fn i_0532c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x532C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x532C8);
    cpu.pc = 0x51A28;
    16
}
/// $0532C8: addi.w #$10, $22(a6)
pub fn i_0532c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x532CE;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0532CE: lea.l $297ff6.l, a1
pub fn i_0532ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x532D4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x297FF6;
    20
}
/// $0532D4: jsr $77c7e.l
pub fn i_0532d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x532DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x532DA);
    cpu.pc = 0x77C7E;
    16
}
/// $0532DA: lea.l $298008.l, a1
pub fn i_0532da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x532E0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x298008;
    20
}
/// $0532E0: jsr $77c7e.l
pub fn i_0532e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x532E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x532E6);
    cpu.pc = 0x77C7E;
    16
}
/// $0532E6: movea.l $80(a6), a2
pub fn i_0532e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x532EA;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $0532EA: jsr $5022a.l
pub fn i_0532ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x532F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x532F0);
    cpu.pc = 0x5022A;
    16
}
/// $0532F0: bra.w $532fe
pub fn i_0532f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x532F4;
    cpu.ir = 0x6000;
    cpu.pc = 0x532FE;
    10
}
/// $0532F4: jsr $4fa70.l
pub fn i_0532f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x532FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x532FA);
    cpu.pc = 0x4FA70;
    16
}
/// $0532FA: bcc.w $53304
pub fn i_0532fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x532FE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x53304; return 10; }
    8
}
/// $0532FE: jmp $518.l
pub fn i_0532fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53304;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $053304: rts
pub fn i_053304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53306;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Tower_053306  ($053306, 212 B, props/destructible_0527ba.s)
/// $053306: movea.l $3c(a6), a1
pub fn i_053306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5330A;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $05330A: jsr $2942a.l
pub fn i_05330a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53310;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53310);
    cpu.pc = 0x2942A;
    16
}
/// $053310: move.w #$36, d1
pub fn i_053310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53314;
    cpu.ir = 0x323C;
    let v1: u32 = 0x36;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $053314: jsr $236e.l
pub fn i_053314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5331A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5331A);
    cpu.pc = 0x236E;
    16
}
/// $05331A: move.w #$40, $70(a6)
pub fn i_05331a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53320;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $053320: move.w #$64, $66(a6)
pub fn i_053320(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53326;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x64;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $053326: lea.l $2975fc.l, a0
pub fn i_053326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5332C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2975FC;
    20
}
/// $05332C: jsr $28cd4.l
pub fn i_05332c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53332;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53332);
    cpu.pc = 0x28CD4;
    16
}
/// $053332: jsr $2783a.l
pub fn i_053332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53338;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53338);
    cpu.pc = 0x2783A;
    16
}
/// $053338: move.l #$2988ea, $60(a6)
pub fn i_053338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53340;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2988EA;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $053340: lea.l $ea3fa.l, a1
pub fn i_053340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53346;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEA3FA;
    20
}
/// $053346: move.w #$74, d0
pub fn i_053346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5334A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x74;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05334A: move.b #$0, $74(a6)
pub fn i_05334a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53350;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write8(ad2, v1 as u8);
    12
}
/// $053350: jsr $4429e.l
pub fn i_053350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53356;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53356);
    cpu.pc = 0x4429E;
    16
}
/// $053356: lea.l $5335c(pc), a1
pub fn i_053356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5335A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5335C;
    16
}
/// $05335A: move.l a1, (a6)
pub fn i_05335a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5335C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05335C: clr.b $10e39a.l
pub fn i_05335c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53362;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $053362: jsr $2783a.l
pub fn i_053362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53368;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53368);
    cpu.pc = 0x2783A;
    16
}
/// $053368: jsr $28998.l
pub fn i_053368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5336E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5336E);
    cpu.pc = 0x28998;
    16
}
/// $05336E: jsr $28d70.l
pub fn i_05336e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53374;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53374);
    cpu.pc = 0x28D70;
    16
}
/// $053374: jsr $2870a.l
pub fn i_053374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5337A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5337A);
    cpu.pc = 0x2870A;
    16
}
/// $05337A: bcc.w $53390
pub fn i_05337a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5337E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x53390; return 10; }
    8
}
/// $05337E: lea.l $5e766.l, a0
pub fn i_05337e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53384;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $053384: jsr $5e770.l
pub fn i_053384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5338A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5338A);
    cpu.pc = 0x5E770;
    16
}
/// $05338A: bclr.b #$3, $13(a6)
pub fn i_05338a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53390;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $053390: jsr $28758.l
pub fn i_053390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53396;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53396);
    cpu.pc = 0x28758;
    16
}
/// $053396: bcc.w $533c8
pub fn i_053396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5339A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x533C8; return 10; }
    8
}
/// $05339A: bclr.b #$0, $13(a6)
pub fn i_05339a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x533A0;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0533A0: move.w #$1023, d0
pub fn i_0533a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x533A4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1023;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0533A4: jsr $2352.l
pub fn i_0533a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x533AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x533AA);
    cpu.pc = 0x2352;
    16
}
/// $0533AA: lea.l $29801a.l, a1
pub fn i_0533aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x533B0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29801A;
    20
}
/// $0533B0: jsr $77c7e.l
pub fn i_0533b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x533B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x533B6);
    cpu.pc = 0x77C7E;
    16
}
/// $0533B6: lea.l $29802c.l, a1
pub fn i_0533b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x533BC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29802C;
    20
}
/// $0533BC: jsr $77c7e.l
pub fn i_0533bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x533C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x533C2);
    cpu.pc = 0x77C7E;
    16
}
/// $0533C2: lea.l $533da(pc), a1
pub fn i_0533c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x533C6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x533DA;
    16
}
/// $0533C6: move.l a1, (a6)
pub fn i_0533c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x533C8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0533C8: jsr $4fa70.l
pub fn i_0533c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x533CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x533CE);
    cpu.pc = 0x4FA70;
    16
}
/// $0533CE: bcc.w $533d8
pub fn i_0533ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x533D2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x533D8; return 10; }
    8
}
/// $0533D2: jmp $518.l
pub fn i_0533d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x533D8;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0533D8: rts
pub fn i_0533d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x533DA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_TowerStage2_0533da  ($0533DA, 168 B, props/destructible_0527ba.s)
/// $0533DA: lea.l $298530.l, a2
pub fn i_0533da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x533E0;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x298530;
    20
}
/// $0533E0: jsr $5022a.l
pub fn i_0533e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x533E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x533E6);
    cpu.pc = 0x5022A;
    16
}
/// $0533E6: move.w #$64, $66(a6)
pub fn i_0533e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x533EC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x64;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0533EC: lea.l $297612.l, a0
pub fn i_0533ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x533F2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x297612;
    20
}
/// $0533F2: jsr $28cd4.l
pub fn i_0533f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x533F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x533F8);
    cpu.pc = 0x28CD4;
    16
}
/// $0533F8: lea.l $533fe(pc), a1
pub fn i_0533f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x533FC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x533FE;
    16
}
/// $0533FC: move.l a1, (a6)
pub fn i_0533fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x533FE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0533FE: clr.b $10e39a.l
pub fn i_0533fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53404;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $053404: jsr $2783a.l
pub fn i_053404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5340A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5340A);
    cpu.pc = 0x2783A;
    16
}
/// $05340A: jsr $28998.l
pub fn i_05340a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53410;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53410);
    cpu.pc = 0x28998;
    16
}
/// $053410: jsr $28d70.l
pub fn i_053410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53416;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53416);
    cpu.pc = 0x28D70;
    16
}
/// $053416: jsr $2870a.l
pub fn i_053416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5341C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5341C);
    cpu.pc = 0x2870A;
    16
}
/// $05341C: bcc.w $53432
pub fn i_05341c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53420;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x53432; return 10; }
    8
}
/// $053420: lea.l $5e766.l, a0
pub fn i_053420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53426;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $053426: jsr $5e770.l
pub fn i_053426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5342C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5342C);
    cpu.pc = 0x5E770;
    16
}
/// $05342C: bclr.b #$3, $13(a6)
pub fn i_05342c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53432;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $053432: jsr $28758.l
pub fn i_053432(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53438;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53438);
    cpu.pc = 0x28758;
    16
}
/// $053438: bcc.w $53470
pub fn i_053438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5343C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x53470; return 10; }
    8
}
/// $05343C: move.w #$1030, d0
pub fn i_05343c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53440;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1030;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $053440: jsr $2352.l
pub fn i_053440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53446;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53446);
    cpu.pc = 0x2352;
    16
}
/// $053446: lea.l $29801a.l, a1
pub fn i_053446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5344C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29801A;
    20
}
/// $05344C: jsr $77c7e.l
pub fn i_05344c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53452;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53452);
    cpu.pc = 0x77C7E;
    16
}
/// $053452: lea.l $29802c.l, a1
pub fn i_053452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53458;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29802C;
    20
}
/// $053458: jsr $77c7e.l
pub fn i_053458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5345E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5345E);
    cpu.pc = 0x77C7E;
    16
}
/// $05345E: lea.l $2980d8.l, a1
pub fn i_05345e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53464;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2980D8;
    20
}
/// $053464: jsr $77c7e.l
pub fn i_053464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5346A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5346A);
    cpu.pc = 0x77C7E;
    16
}
/// $05346A: lea.l $53482(pc), a1
pub fn i_05346a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5346E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x53482;
    16
}
/// $05346E: move.l a1, (a6)
pub fn i_05346e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53470;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $053470: jsr $4fa70.l
pub fn i_053470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53476;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53476);
    cpu.pc = 0x4FA70;
    16
}
/// $053476: bcc.w $53480
pub fn i_053476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5347A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x53480; return 10; }
    8
}
/// $05347A: jmp $518.l
pub fn i_05347a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53480;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $053480: rts
pub fn i_053480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53482;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_TowerWreck_053482  ($053482, 86 B, props/destructible_0527ba.s)
/// $053482: move.l #$2000, d0
pub fn i_053482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53488;
    cpu.ir = 0x203C;
    let v1: u32 = 0x2000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $053488: jsr $51a28.l
pub fn i_053488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5348E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5348E);
    cpu.pc = 0x51A28;
    16
}
/// $05348E: move.b #$ff, $74(a6)
pub fn i_05348e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53494;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write8(ad2, v1 as u8);
    12
}
/// $053494: lea.l $298544.l, a2
pub fn i_053494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5349A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x298544;
    20
}
/// $05349A: jsr $5022a.l
pub fn i_05349a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x534A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x534A0);
    cpu.pc = 0x5022A;
    16
}
/// $0534A0: lea.l $ffff.w, a0
pub fn i_0534a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x534A4;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $0534A4: move.l a0, $48(a6)
pub fn i_0534a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x534A8;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0534A8: lea.l $297622.l, a0
pub fn i_0534a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x534AE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x297622;
    20
}
/// $0534AE: jsr $28cd4.l
pub fn i_0534ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x534B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x534B4);
    cpu.pc = 0x28CD4;
    16
}
/// $0534B4: lea.l $534ba(pc), a1
pub fn i_0534b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x534B8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x534BA;
    16
}
/// $0534B8: move.l a1, (a6)
pub fn i_0534b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x534BA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0534BA: jsr $2783a.l
pub fn i_0534ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x534C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x534C0);
    cpu.pc = 0x2783A;
    16
}
/// $0534C0: jsr $28d70.l
pub fn i_0534c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x534C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x534C6);
    cpu.pc = 0x28D70;
    16
}
/// $0534C6: jsr $4fa70.l
pub fn i_0534c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x534CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x534CC);
    cpu.pc = 0x4FA70;
    16
}
/// $0534CC: bcc.w $534d6
pub fn i_0534cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x534D0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x534D6; return 10; }
    8
}
/// $0534D0: jmp $518.l
pub fn i_0534d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x534D6;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0534D6: rts
pub fn i_0534d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x534D8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Gate_0534d8  ($0534D8, 198 B, props/destructible_0527ba.s)
/// $0534D8: movea.l $3c(a6), a1
pub fn i_0534d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x534DC;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0534DC: jsr $2942a.l
pub fn i_0534dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x534E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x534E2);
    cpu.pc = 0x2942A;
    16
}
/// $0534E2: move.w #$36, d1
pub fn i_0534e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x534E6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x36;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0534E6: jsr $236e.l
pub fn i_0534e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x534EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x534EC);
    cpu.pc = 0x236E;
    16
}
/// $0534EC: move.w #$c0, $70(a6)
pub fn i_0534ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x534F2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0534F2: move.w #$28, $66(a6)
pub fn i_0534f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x534F8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0534F8: lea.l $297632.l, a0
pub fn i_0534f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x534FE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x297632;
    20
}
/// $0534FE: jsr $28cd4.l
pub fn i_0534fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53504;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53504);
    cpu.pc = 0x28CD4;
    16
}
/// $053504: lea.l $5350a(pc), a1
pub fn i_053504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53508;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5350A;
    16
}
/// $053508: move.l a1, (a6)
pub fn i_053508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5350A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05350A: jsr $2783a.l
pub fn i_05350a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53510;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53510);
    cpu.pc = 0x2783A;
    16
}
/// $053510: jsr $28d70.l
pub fn i_053510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53516;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53516);
    cpu.pc = 0x28D70;
    16
}
/// $053516: cmpi.w #$90, $22(a6)
pub fn i_053516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5351C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x90, Size::Word, false);
    16
}
/// $05351C: bgt.w $53590
pub fn i_05351c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53520;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x53590; return 10; }
    8
}
/// $053520: cmpi.w #$ffe0, $22(a6)
pub fn i_053520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53526;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFE0, Size::Word, false);
    16
}
/// $053526: blt.w $5357e
pub fn i_053526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5352A;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x5357E; return 10; }
    8
}
/// $05352A: jsr $2870a.l
pub fn i_05352a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53530;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53530);
    cpu.pc = 0x2870A;
    16
}
/// $053530: bcc.w $53546
pub fn i_053530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53534;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x53546; return 10; }
    8
}
/// $053534: lea.l $5e766.l, a0
pub fn i_053534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5353A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $05353A: jsr $5e770.l
pub fn i_05353a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53540;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53540);
    cpu.pc = 0x5E770;
    16
}
/// $053540: bclr.b #$3, $13(a6)
pub fn i_053540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53546;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $053546: jsr $28758.l
pub fn i_053546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5354C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5354C);
    cpu.pc = 0x28758;
    16
}
/// $05354C: bcc.w $5357e
pub fn i_05354c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53550;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5357E; return 10; }
    8
}
/// $053550: bclr.b #$0, $13(a6)
pub fn i_053550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53556;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $053556: move.w #$1023, d0
pub fn i_053556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5355A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1023;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05355A: jsr $2352.l
pub fn i_05355a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53560;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53560);
    cpu.pc = 0x2352;
    16
}
/// $053560: lea.l $29803e.l, a1
pub fn i_053560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53566;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29803E;
    20
}
/// $053566: jsr $77c7e.l
pub fn i_053566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5356C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5356C);
    cpu.pc = 0x77C7E;
    16
}
/// $05356C: lea.l $298050.l, a1
pub fn i_05356c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53572;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x298050;
    20
}
/// $053572: jsr $77c7e.l
pub fn i_053572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53578;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53578);
    cpu.pc = 0x77C7E;
    16
}
/// $053578: lea.l $5359e(pc), a1
pub fn i_053578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5357C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5359E;
    16
}
/// $05357C: move.l a1, (a6)
pub fn i_05357c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5357E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05357E: jsr $4fa70.l
pub fn i_05357e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53584;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53584);
    cpu.pc = 0x4FA70;
    16
}
/// $053584: bcc.w $5358e
pub fn i_053584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53588;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5358E; return 10; }
    8
}
/// $053588: jmp $518.l
pub fn i_053588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5358E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $05358E: rts
pub fn i_05358e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53590;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $053590: bclr.b #$3, $13(a6)
pub fn i_053590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53596;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $053596: bclr.b #$0, $13(a6)
pub fn i_053596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5359C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05359C: rts
pub fn i_05359c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5359E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_GateStage2_05359e  ($05359E, 180 B, props/destructible_0527ba.s)
/// $05359E: cmpi.w #$0, $38(a6)
pub fn i_05359e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x535A4;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0535A4: beq.w $535b0
pub fn i_0535a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x535A8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x535B0; return 10; }
    8
}
/// $0535A8: jsr $53f2e(pc)
pub fn i_0535a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x535AC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x535AC);
    cpu.pc = 0x53F2E;
    16
}
/// $0535AC: bra.w $535b4
pub fn i_0535ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x535B0;
    cpu.ir = 0x6000;
    cpu.pc = 0x535B4;
    10
}
/// $0535B0: jsr $53ee2(pc)
pub fn i_0535b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x535B4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x535B4);
    cpu.pc = 0x53EE2;
    16
}
/// $0535B4: move.w #$28, $66(a6)
pub fn i_0535b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x535BA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0535BA: lea.l $297648.l, a0
pub fn i_0535ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x535C0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x297648;
    20
}
/// $0535C0: jsr $28cd4.l
pub fn i_0535c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x535C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x535C6);
    cpu.pc = 0x28CD4;
    16
}
/// $0535C6: lea.l $535cc(pc), a1
pub fn i_0535c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x535CA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x535CC;
    16
}
/// $0535CA: move.l a1, (a6)
pub fn i_0535ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x535CC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0535CC: jsr $2783a.l
pub fn i_0535cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x535D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x535D2);
    cpu.pc = 0x2783A;
    16
}
/// $0535D2: jsr $28d70.l
pub fn i_0535d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x535D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x535D8);
    cpu.pc = 0x28D70;
    16
}
/// $0535D8: cmpi.w #$ffe0, $22(a6)
pub fn i_0535d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x535DE;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFE0, Size::Word, false);
    16
}
/// $0535DE: blt.b $5357e
pub fn i_0535de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x535E0;
    cpu.ir = 0x6D9E;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x5357E; return 10; }
    8
}
/// $0535E0: jsr $2870a.l
pub fn i_0535e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x535E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x535E6);
    cpu.pc = 0x2870A;
    16
}
/// $0535E6: bcc.w $535fc
pub fn i_0535e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x535EA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x535FC; return 10; }
    8
}
/// $0535EA: lea.l $5e766.l, a0
pub fn i_0535ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x535F0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $0535F0: jsr $5e770.l
pub fn i_0535f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x535F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x535F6);
    cpu.pc = 0x5E770;
    16
}
/// $0535F6: bclr.b #$3, $13(a6)
pub fn i_0535f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x535FC;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0535FC: jsr $28758.l
pub fn i_0535fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53602;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53602);
    cpu.pc = 0x28758;
    16
}
/// $053602: bcc.w $53640
pub fn i_053602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53606;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x53640; return 10; }
    8
}
/// $053606: bclr.b #$0, $13(a6)
pub fn i_053606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5360C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05360C: move.w #$1030, d0
pub fn i_05360c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53610;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1030;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $053610: jsr $2352.l
pub fn i_053610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53616;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53616);
    cpu.pc = 0x2352;
    16
}
/// $053616: lea.l $29803e.l, a1
pub fn i_053616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5361C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29803E;
    20
}
/// $05361C: jsr $77c7e.l
pub fn i_05361c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53622;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53622);
    cpu.pc = 0x77C7E;
    16
}
/// $053622: lea.l $298050.l, a1
pub fn i_053622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53628;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x298050;
    20
}
/// $053628: jsr $77c7e.l
pub fn i_053628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5362E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5362E);
    cpu.pc = 0x77C7E;
    16
}
/// $05362E: lea.l $2980ea.l, a1
pub fn i_05362e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53634;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2980EA;
    20
}
/// $053634: jsr $77c7e.l
pub fn i_053634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5363A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5363A);
    cpu.pc = 0x77C7E;
    16
}
/// $05363A: lea.l $53652(pc), a1
pub fn i_05363a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5363E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x53652;
    16
}
/// $05363E: move.l a1, (a6)
pub fn i_05363e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53640;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $053640: jsr $4fa70.l
pub fn i_053640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53646;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53646);
    cpu.pc = 0x4FA70;
    16
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x051BA6, i_051ba6),
    (0x051BA8, i_051ba8),
    (0x051BAC, i_051bac),
    (0x051BB0, i_051bb0),
    (0x051BB2, i_051bb2),
    (0x051BB6, i_051bb6),
    (0x051BB8, i_051bb8),
    (0x051BBC, i_051bbc),
    (0x051BBE, i_051bbe),
    (0x051BC2, i_051bc2),
    (0x051BC6, i_051bc6),
    (0x051BCA, i_051bca),
    (0x051BCE, i_051bce),
    (0x051BD0, i_051bd0),
    (0x051BD2, i_051bd2),
    (0x051BD6, i_051bd6),
    (0x051BDC, i_051bdc),
    (0x051BE0, i_051be0),
    (0x051BE4, i_051be4),
    (0x051BE6, i_051be6),
    (0x051BEC, i_051bec),
    (0x051BF0, i_051bf0),
    (0x051BF4, i_051bf4),
    (0x051BF6, i_051bf6),
    (0x051BFA, i_051bfa),
    (0x051BFE, i_051bfe),
    (0x051C02, i_051c02),
    (0x051C06, i_051c06),
    (0x051C08, i_051c08),
    (0x051C0C, i_051c0c),
    (0x051C10, i_051c10),
    (0x051C14, i_051c14),
    (0x051C18, i_051c18),
    (0x051C1A, i_051c1a),
    (0x051C1E, i_051c1e),
    (0x051C20, i_051c20),
    (0x051C24, i_051c24),
    (0x051C28, i_051c28),
    (0x051C2A, i_051c2a),
    (0x051C2C, i_051c2c),
    (0x051C30, i_051c30),
    (0x051C34, i_051c34),
    (0x051C38, i_051c38),
    (0x051C3C, i_051c3c),
    (0x051C40, i_051c40),
    (0x051C44, i_051c44),
    (0x051C46, i_051c46),
    (0x051C4A, i_051c4a),
    (0x051C4E, i_051c4e),
    (0x051C52, i_051c52),
    (0x051C54, i_051c54),
    (0x051C58, i_051c58),
    (0x051C5C, i_051c5c),
    (0x051C62, i_051c62),
    (0x051C68, i_051c68),
    (0x051C6A, i_051c6a),
    (0x051C6E, i_051c6e),
    (0x051C72, i_051c72),
    (0x051C76, i_051c76),
    (0x051C7A, i_051c7a),
    (0x051C7E, i_051c7e),
    (0x051C80, i_051c80),
    (0x051C82, i_051c82),
    (0x051C86, i_051c86),
    (0x051C8A, i_051c8a),
    (0x051C8E, i_051c8e),
    (0x051C92, i_051c92),
    (0x051C94, i_051c94),
    (0x051C98, i_051c98),
    (0x051C9A, i_051c9a),
    (0x051C9E, i_051c9e),
    (0x051CA2, i_051ca2),
    (0x051CA6, i_051ca6),
    (0x051CA8, i_051ca8),
    (0x051CAC, i_051cac),
    (0x051CB0, i_051cb0),
    (0x051CB2, i_051cb2),
    (0x051CB4, i_051cb4),
    (0x051CB6, i_051cb6),
    (0x051CB8, i_051cb8),
    (0x051CBC, i_051cbc),
    (0x051CC0, i_051cc0),
    (0x051CC4, i_051cc4),
    (0x051CC8, i_051cc8),
    (0x051CCC, i_051ccc),
    (0x051CCE, i_051cce),
    (0x051CD2, i_051cd2),
    (0x051CD6, i_051cd6),
    (0x051CDC, i_051cdc),
    (0x051CE2, i_051ce2),
    (0x051CE6, i_051ce6),
    (0x051CEA, i_051cea),
    (0x051CEE, i_051cee),
    (0x051CF0, i_051cf0),
    (0x051CF4, i_051cf4),
    (0x051CF6, i_051cf6),
    (0x051CFA, i_051cfa),
    (0x051CFE, i_051cfe),
    (0x051D02, i_051d02),
    (0x051D06, i_051d06),
    (0x051D08, i_051d08),
    (0x051D0C, i_051d0c),
    (0x051D0E, i_051d0e),
    (0x051D12, i_051d12),
    (0x051D16, i_051d16),
    (0x051D1A, i_051d1a),
    (0x051D1E, i_051d1e),
    (0x051D22, i_051d22),
    (0x051D26, i_051d26),
    (0x051D28, i_051d28),
    (0x051D2A, i_051d2a),
    (0x051D2C, i_051d2c),
    (0x051D30, i_051d30),
    (0x051D32, i_051d32),
    (0x051D36, i_051d36),
    (0x051D3A, i_051d3a),
    (0x051D3E, i_051d3e),
    (0x051D42, i_051d42),
    (0x051D44, i_051d44),
    (0x051D48, i_051d48),
    (0x051D4C, i_051d4c),
    (0x051D50, i_051d50),
    (0x051D52, i_051d52),
    (0x051D56, i_051d56),
    (0x051D5A, i_051d5a),
    (0x051D60, i_051d60),
    (0x051D66, i_051d66),
    (0x051D68, i_051d68),
    (0x051D6C, i_051d6c),
    (0x051D70, i_051d70),
    (0x051D74, i_051d74),
    (0x051D78, i_051d78),
    (0x051D7C, i_051d7c),
    (0x051D7E, i_051d7e),
    (0x051D82, i_051d82),
    (0x051D84, i_051d84),
    (0x051D86, i_051d86),
    (0x051D88, i_051d88),
    (0x051D8A, i_051d8a),
    (0x051D8E, i_051d8e),
    (0x051D90, i_051d90),
    (0x051D94, i_051d94),
    (0x051D96, i_051d96),
    (0x051D98, i_051d98),
    (0x051D9A, i_051d9a),
    (0x051D9E, i_051d9e),
    (0x051DA0, i_051da0),
    (0x051DA2, i_051da2),
    (0x051DA6, i_051da6),
    (0x051DA8, i_051da8),
    (0x051DAC, i_051dac),
    (0x051DAE, i_051dae),
    (0x051DB2, i_051db2),
    (0x051DB4, i_051db4),
    (0x051DB8, i_051db8),
    (0x051DBA, i_051dba),
    (0x051DBC, i_051dbc),
    (0x051DBE, i_051dbe),
    (0x051DC2, i_051dc2),
    (0x051DC4, i_051dc4),
    (0x051DC6, i_051dc6),
    (0x051DCA, i_051dca),
    (0x051DD0, i_051dd0),
    (0x051DD6, i_051dd6),
    (0x051DDA, i_051dda),
    (0x051DDC, i_051ddc),
    (0x051DE0, i_051de0),
    (0x051DE2, i_051de2),
    (0x051DE8, i_051de8),
    (0x051DEE, i_051dee),
    (0x051DF2, i_051df2),
    (0x051DF4, i_051df4),
    (0x051DF6, i_051df6),
    (0x051DF8, i_051df8),
    (0x051DFA, i_051dfa),
    (0x051DFE, i_051dfe),
    (0x051E02, i_051e02),
    (0x051E04, i_051e04),
    (0x051E06, i_051e06),
    (0x051E08, i_051e08),
    (0x051E0A, i_051e0a),
    (0x051E0C, i_051e0c),
    (0x051E10, i_051e10),
    (0x051E14, i_051e14),
    (0x051E18, i_051e18),
    (0x051E1C, i_051e1c),
    (0x051E20, i_051e20),
    (0x051E24, i_051e24),
    (0x051E28, i_051e28),
    (0x051E2A, i_051e2a),
    (0x051E2E, i_051e2e),
    (0x051E30, i_051e30),
    (0x051E32, i_051e32),
    (0x051E36, i_051e36),
    (0x051E3C, i_051e3c),
    (0x051E42, i_051e42),
    (0x051E48, i_051e48),
    (0x051E4A, i_051e4a),
    (0x051E50, i_051e50),
    (0x051E56, i_051e56),
    (0x051E5C, i_051e5c),
    (0x051E5E, i_051e5e),
    (0x051E62, i_051e62),
    (0x051E66, i_051e66),
    (0x051E6A, i_051e6a),
    (0x051E6C, i_051e6c),
    (0x051E6E, i_051e6e),
    (0x051E72, i_051e72),
    (0x051E74, i_051e74),
    (0x051E7A, i_051e7a),
    (0x051E80, i_051e80),
    (0x051E82, i_051e82),
    (0x051E84, i_051e84),
    (0x051E88, i_051e88),
    (0x051E8C, i_051e8c),
    (0x051E8E, i_051e8e),
    (0x051E92, i_051e92),
    (0x051E96, i_051e96),
    (0x051E9A, i_051e9a),
    (0x051E9E, i_051e9e),
    (0x051EA2, i_051ea2),
    (0x051EA4, i_051ea4),
    (0x051EA8, i_051ea8),
    (0x051EAA, i_051eaa),
    (0x051EAC, i_051eac),
    (0x051EAE, i_051eae),
    (0x051EB6, i_051eb6),
    (0x051EB8, i_051eb8),
    (0x051EBC, i_051ebc),
    (0x051EC0, i_051ec0),
    (0x051EC4, i_051ec4),
    (0x051EC6, i_051ec6),
    (0x051EC8, i_051ec8),
    (0x051ECC, i_051ecc),
    (0x051ECE, i_051ece),
    (0x051ED4, i_051ed4),
    (0x051ED6, i_051ed6),
    (0x051EDC, i_051edc),
    (0x051EDE, i_051ede),
    (0x051EE2, i_051ee2),
    (0x051EE6, i_051ee6),
    (0x051EEC, i_051eec),
    (0x051EF2, i_051ef2),
    (0x051EFA, i_051efa),
    (0x051EFC, i_051efc),
    (0x051F00, i_051f00),
    (0x051F02, i_051f02),
    (0x051F08, i_051f08),
    (0x051F0C, i_051f0c),
    (0x051F0E, i_051f0e),
    (0x051F12, i_051f12),
    (0x051F14, i_051f14),
    (0x051F16, i_051f16),
    (0x051F18, i_051f18),
    (0x051F1A, i_051f1a),
    (0x051F20, i_051f20),
    (0x051F22, i_051f22),
    (0x051F28, i_051f28),
    (0x051F2A, i_051f2a),
    (0x051F2E, i_051f2e),
    (0x051F30, i_051f30),
    (0x051F36, i_051f36),
    (0x051F3A, i_051f3a),
    (0x051F3C, i_051f3c),
    (0x051F40, i_051f40),
    (0x051F44, i_051f44),
    (0x051F46, i_051f46),
    (0x051F48, i_051f48),
    (0x051F4A, i_051f4a),
    (0x051F4C, i_051f4c),
    (0x051F4E, i_051f4e),
    (0x051F52, i_051f52),
    (0x051F56, i_051f56),
    (0x051F5A, i_051f5a),
    (0x051F5E, i_051f5e),
    (0x051F62, i_051f62),
    (0x051F64, i_051f64),
    (0x051F66, i_051f66),
    (0x051F6A, i_051f6a),
    (0x051F6E, i_051f6e),
    (0x051F72, i_051f72),
    (0x051F74, i_051f74),
    (0x051F76, i_051f76),
    (0x051F78, i_051f78),
    (0x051F7C, i_051f7c),
    (0x051F80, i_051f80),
    (0x051F82, i_051f82),
    (0x051F86, i_051f86),
    (0x051F8C, i_051f8c),
    (0x051F8E, i_051f8e),
    (0x051F92, i_051f92),
    (0x051F94, i_051f94),
    (0x051F98, i_051f98),
    (0x051F9C, i_051f9c),
    (0x051FA0, i_051fa0),
    (0x051FA4, i_051fa4),
    (0x051FA6, i_051fa6),
    (0x051FA8, i_051fa8),
    (0x051FAE, i_051fae),
    (0x051FB4, i_051fb4),
    (0x051FBC, i_051fbc),
    (0x051FBE, i_051fbe),
    (0x051FC2, i_051fc2),
    (0x051FC4, i_051fc4),
    (0x051FC6, i_051fc6),
    (0x051FC8, i_051fc8),
    (0x051FCC, i_051fcc),
    (0x051FD0, i_051fd0),
    (0x051FD4, i_051fd4),
    (0x051FD6, i_051fd6),
    (0x051FD8, i_051fd8),
    (0x051FDE, i_051fde),
    (0x051FE4, i_051fe4),
    (0x051FE8, i_051fe8),
    (0x051FEA, i_051fea),
    (0x051FEC, i_051fec),
    (0x051FF0, i_051ff0),
    (0x051FF6, i_051ff6),
    (0x051FFA, i_051ffa),
    (0x052000, i_052000),
    (0x052006, i_052006),
    (0x052008, i_052008),
    (0x05200C, i_05200c),
    (0x052012, i_052012),
    (0x052016, i_052016),
    (0x052018, i_052018),
    (0x05201A, i_05201a),
    (0x05201C, i_05201c),
    (0x052022, i_052022),
    (0x052028, i_052028),
    (0x052030, i_052030),
    (0x052032, i_052032),
    (0x052036, i_052036),
    (0x05203A, i_05203a),
    (0x05203E, i_05203e),
    (0x052042, i_052042),
    (0x052046, i_052046),
    (0x052048, i_052048),
    (0x05204C, i_05204c),
    (0x05204E, i_05204e),
    (0x052050, i_052050),
    (0x052056, i_052056),
    (0x05205A, i_05205a),
    (0x052060, i_052060),
    (0x052064, i_052064),
    (0x052068, i_052068),
    (0x052070, i_052070),
    (0x052074, i_052074),
    (0x05207C, i_05207c),
    (0x052080, i_052080),
    (0x052084, i_052084),
    (0x05208A, i_05208a),
    (0x05208E, i_05208e),
    (0x052094, i_052094),
    (0x05209A, i_05209a),
    (0x05209C, i_05209c),
    (0x0520A2, i_0520a2),
    (0x0520A6, i_0520a6),
    (0x0520AC, i_0520ac),
    (0x0520B0, i_0520b0),
    (0x0520B8, i_0520b8),
    (0x0520BC, i_0520bc),
    (0x0520C0, i_0520c0),
    (0x0520C6, i_0520c6),
    (0x0520CA, i_0520ca),
    (0x0520D2, i_0520d2),
    (0x0520D4, i_0520d4),
    (0x0520D8, i_0520d8),
    (0x0520DC, i_0520dc),
    (0x0520DE, i_0520de),
    (0x0520E2, i_0520e2),
    (0x0520E6, i_0520e6),
    (0x0520E8, i_0520e8),
    (0x0520EA, i_0520ea),
    (0x0520F2, i_0520f2),
    (0x0520F6, i_0520f6),
    (0x0520FA, i_0520fa),
    (0x0520FC, i_0520fc),
    (0x0520FE, i_0520fe),
    (0x052102, i_052102),
    (0x052106, i_052106),
    (0x05210A, i_05210a),
    (0x052112, i_052112),
    (0x052116, i_052116),
    (0x052118, i_052118),
    (0x052120, i_052120),
    (0x052124, i_052124),
    (0x052126, i_052126),
    (0x052128, i_052128),
    (0x05212C, i_05212c),
    (0x052130, i_052130),
    (0x052132, i_052132),
    (0x052136, i_052136),
    (0x05213E, i_05213e),
    (0x052142, i_052142),
    (0x052144, i_052144),
    (0x052148, i_052148),
    (0x05214A, i_05214a),
    (0x05214E, i_05214e),
    (0x052152, i_052152),
    (0x05215A, i_05215a),
    (0x05215E, i_05215e),
    (0x052162, i_052162),
    (0x052166, i_052166),
    (0x05216E, i_05216e),
    (0x052172, i_052172),
    (0x052176, i_052176),
    (0x05217A, i_05217a),
    (0x05217C, i_05217c),
    (0x052180, i_052180),
    (0x052184, i_052184),
    (0x05218C, i_05218c),
    (0x052190, i_052190),
    (0x052194, i_052194),
    (0x052198, i_052198),
    (0x0521A0, i_0521a0),
    (0x0521A4, i_0521a4),
    (0x0521A8, i_0521a8),
    (0x0521B0, i_0521b0),
    (0x0521B2, i_0521b2),
    (0x0521B6, i_0521b6),
    (0x0521BA, i_0521ba),
    (0x0521BE, i_0521be),
    (0x0521C2, i_0521c2),
    (0x0521CA, i_0521ca),
    (0x0521CE, i_0521ce),
    (0x0521D2, i_0521d2),
    (0x0521D6, i_0521d6),
    (0x0521DE, i_0521de),
    (0x0521E2, i_0521e2),
    (0x0521E6, i_0521e6),
    (0x0521EA, i_0521ea),
    (0x0521EE, i_0521ee),
    (0x0521F6, i_0521f6),
    (0x0521FA, i_0521fa),
    (0x0521FE, i_0521fe),
    (0x052202, i_052202),
    (0x052206, i_052206),
    (0x05220E, i_05220e),
    (0x052212, i_052212),
    (0x052216, i_052216),
    (0x05221A, i_05221a),
    (0x05221E, i_05221e),
    (0x052226, i_052226),
    (0x05222A, i_05222a),
    (0x05222E, i_05222e),
    (0x052238, i_052238),
    (0x05223A, i_05223a),
    (0x052240, i_052240),
    (0x052244, i_052244),
    (0x05224A, i_05224a),
    (0x05224E, i_05224e),
    (0x052252, i_052252),
    (0x05225A, i_05225a),
    (0x05225E, i_05225e),
    (0x052266, i_052266),
    (0x05226A, i_05226a),
    (0x05226E, i_05226e),
    (0x052272, i_052272),
    (0x052274, i_052274),
    (0x052278, i_052278),
    (0x052280, i_052280),
    (0x052284, i_052284),
    (0x052288, i_052288),
    (0x05228A, i_05228a),
    (0x052292, i_052292),
    (0x052296, i_052296),
    (0x05229A, i_05229a),
    (0x0522A0, i_0522a0),
    (0x0522A4, i_0522a4),
    (0x0522AC, i_0522ac),
    (0x0522AE, i_0522ae),
    (0x0522B2, i_0522b2),
    (0x0522B4, i_0522b4),
    (0x0522B8, i_0522b8),
    (0x0522BC, i_0522bc),
    (0x0522C4, i_0522c4),
    (0x0522C8, i_0522c8),
    (0x0522CC, i_0522cc),
    (0x0522D0, i_0522d0),
    (0x0522D8, i_0522d8),
    (0x0522DC, i_0522dc),
    (0x0522E0, i_0522e0),
    (0x0522E8, i_0522e8),
    (0x0522EA, i_0522ea),
    (0x0522EC, i_0522ec),
    (0x0522EE, i_0522ee),
    (0x0522F0, i_0522f0),
    (0x0522F6, i_0522f6),
    (0x0522FA, i_0522fa),
    (0x0522FE, i_0522fe),
    (0x052304, i_052304),
    (0x052306, i_052306),
    (0x05230C, i_05230c),
    (0x052310, i_052310),
    (0x052314, i_052314),
    (0x05231A, i_05231a),
    (0x052328, i_052328),
    (0x05232A, i_05232a),
    (0x052330, i_052330),
    (0x052334, i_052334),
    (0x052338, i_052338),
    (0x05233A, i_05233a),
    (0x052342, i_052342),
    (0x05234A, i_05234a),
    (0x052352, i_052352),
    (0x05235A, i_05235a),
    (0x05235C, i_05235c),
    (0x052362, i_052362),
    (0x052364, i_052364),
    (0x05236A, i_05236a),
    (0x05236E, i_05236e),
    (0x052372, i_052372),
    (0x052378, i_052378),
    (0x05237C, i_05237c),
    (0x052380, i_052380),
    (0x052382, i_052382),
    (0x052386, i_052386),
    (0x05238A, i_05238a),
    (0x05238E, i_05238e),
    (0x052392, i_052392),
    (0x052396, i_052396),
    (0x052398, i_052398),
    (0x05239C, i_05239c),
    (0x05239E, i_05239e),
    (0x0523A0, i_0523a0),
    (0x0523A4, i_0523a4),
    (0x0523AA, i_0523aa),
    (0x0523AC, i_0523ac),
    (0x0523B0, i_0523b0),
    (0x0523B2, i_0523b2),
    (0x0523B4, i_0523b4),
    (0x0523B8, i_0523b8),
    (0x0523BE, i_0523be),
    (0x0523C0, i_0523c0),
    (0x0523C4, i_0523c4),
    (0x0523C6, i_0523c6),
    (0x0523C8, i_0523c8),
    (0x0523CC, i_0523cc),
    (0x0523D2, i_0523d2),
    (0x0523D4, i_0523d4),
    (0x0523D8, i_0523d8),
    (0x0523DA, i_0523da),
    (0x0523DC, i_0523dc),
    (0x0523E0, i_0523e0),
    (0x0523E6, i_0523e6),
    (0x0523E8, i_0523e8),
    (0x0523EC, i_0523ec),
    (0x0523EE, i_0523ee),
    (0x0523F6, i_0523f6),
    (0x0523FA, i_0523fa),
    (0x052402, i_052402),
    (0x052404, i_052404),
    (0x05240A, i_05240a),
    (0x052410, i_052410),
    (0x052416, i_052416),
    (0x05241C, i_05241c),
    (0x05241E, i_05241e),
    (0x052424, i_052424),
    (0x052428, i_052428),
    (0x05242C, i_05242c),
    (0x05242E, i_05242e),
    (0x052430, i_052430),
    (0x052434, i_052434),
    (0x052438, i_052438),
    (0x05243E, i_05243e),
    (0x052444, i_052444),
    (0x05244A, i_05244a),
    (0x05244E, i_05244e),
    (0x052452, i_052452),
    (0x052454, i_052454),
    (0x052458, i_052458),
    (0x05245C, i_05245c),
    (0x052462, i_052462),
    (0x052466, i_052466),
    (0x05246C, i_05246c),
    (0x052472, i_052472),
    (0x052478, i_052478),
    (0x05247C, i_05247c),
    (0x052482, i_052482),
    (0x052486, i_052486),
    (0x052488, i_052488),
    (0x05248E, i_05248e),
    (0x052494, i_052494),
    (0x05249A, i_05249a),
    (0x0524A2, i_0524a2),
    (0x0524A8, i_0524a8),
    (0x0524AA, i_0524aa),
    (0x0524B2, i_0524b2),
    (0x0524B6, i_0524b6),
    (0x0524BE, i_0524be),
    (0x0524C0, i_0524c0),
    (0x0524C6, i_0524c6),
    (0x0524CC, i_0524cc),
    (0x0524D2, i_0524d2),
    (0x0524DA, i_0524da),
    (0x0524DC, i_0524dc),
    (0x0524E2, i_0524e2),
    (0x0524E6, i_0524e6),
    (0x0524EA, i_0524ea),
    (0x0524EC, i_0524ec),
    (0x0524EE, i_0524ee),
    (0x0524F2, i_0524f2),
    (0x0524F6, i_0524f6),
    (0x0524FC, i_0524fc),
    (0x052502, i_052502),
    (0x052508, i_052508),
    (0x05250C, i_05250c),
    (0x052510, i_052510),
    (0x052512, i_052512),
    (0x052514, i_052514),
    (0x052518, i_052518),
    (0x05251C, i_05251c),
    (0x052522, i_052522),
    (0x052526, i_052526),
    (0x05252C, i_05252c),
    (0x052532, i_052532),
    (0x052538, i_052538),
    (0x05253C, i_05253c),
    (0x052542, i_052542),
    (0x052546, i_052546),
    (0x052548, i_052548),
    (0x05254E, i_05254e),
    (0x052554, i_052554),
    (0x05255A, i_05255a),
    (0x052560, i_052560),
    (0x052568, i_052568),
    (0x05256E, i_05256e),
    (0x052570, i_052570),
    (0x052574, i_052574),
    (0x052578, i_052578),
    (0x05257C, i_05257c),
    (0x052580, i_052580),
    (0x052584, i_052584),
    (0x052588, i_052588),
    (0x05258E, i_05258e),
    (0x052592, i_052592),
    (0x052596, i_052596),
    (0x05259A, i_05259a),
    (0x05259E, i_05259e),
    (0x0525A2, i_0525a2),
    (0x0525A4, i_0525a4),
    (0x0525AC, i_0525ac),
    (0x0525B4, i_0525b4),
    (0x0525BC, i_0525bc),
    (0x0525C4, i_0525c4),
    (0x0525C6, i_0525c6),
    (0x0525CA, i_0525ca),
    (0x0525CE, i_0525ce),
    (0x0525D2, i_0525d2),
    (0x0525D4, i_0525d4),
    (0x0525DA, i_0525da),
    (0x0525DC, i_0525dc),
    (0x0525E0, i_0525e0),
    (0x0525E2, i_0525e2),
    (0x0525E4, i_0525e4),
    (0x0525E6, i_0525e6),
    (0x0525EA, i_0525ea),
    (0x0525EE, i_0525ee),
    (0x0525F0, i_0525f0),
    (0x0525F6, i_0525f6),
    (0x0525F8, i_0525f8),
    (0x0525FC, i_0525fc),
    (0x0525FE, i_0525fe),
    (0x052600, i_052600),
    (0x052602, i_052602),
    (0x052606, i_052606),
    (0x05260A, i_05260a),
    (0x05260C, i_05260c),
    (0x052612, i_052612),
    (0x052614, i_052614),
    (0x052618, i_052618),
    (0x05261A, i_05261a),
    (0x05261C, i_05261c),
    (0x05261E, i_05261e),
    (0x052622, i_052622),
    (0x052626, i_052626),
    (0x05262A, i_05262a),
    (0x05262C, i_05262c),
    (0x052630, i_052630),
    (0x052634, i_052634),
    (0x052638, i_052638),
    (0x05263C, i_05263c),
    (0x052640, i_052640),
    (0x052644, i_052644),
    (0x052648, i_052648),
    (0x05264C, i_05264c),
    (0x052650, i_052650),
    (0x052652, i_052652),
    (0x052656, i_052656),
    (0x052658, i_052658),
    (0x05265E, i_05265e),
    (0x052662, i_052662),
    (0x052664, i_052664),
    (0x052668, i_052668),
    (0x05266A, i_05266a),
    (0x052670, i_052670),
    (0x052674, i_052674),
    (0x052676, i_052676),
    (0x05267A, i_05267a),
    (0x05267C, i_05267c),
    (0x052682, i_052682),
    (0x052684, i_052684),
    (0x052688, i_052688),
    (0x05268E, i_05268e),
    (0x052692, i_052692),
    (0x052698, i_052698),
    (0x05269C, i_05269c),
    (0x0526A2, i_0526a2),
    (0x0526A6, i_0526a6),
    (0x0526A8, i_0526a8),
    (0x0526AA, i_0526aa),
    (0x0526B0, i_0526b0),
    (0x0526B6, i_0526b6),
    (0x0526B8, i_0526b8),
    (0x0526BE, i_0526be),
    (0x0526C2, i_0526c2),
    (0x0526C6, i_0526c6),
    (0x0526C8, i_0526c8),
    (0x0526CA, i_0526ca),
    (0x0526CC, i_0526cc),
    (0x0526CE, i_0526ce),
    (0x0526D0, i_0526d0),
    (0x0526D2, i_0526d2),
    (0x0526D4, i_0526d4),
    (0x0526D6, i_0526d6),
    (0x0526D8, i_0526d8),
    (0x0526DC, i_0526dc),
    (0x0526E2, i_0526e2),
    (0x0526E6, i_0526e6),
    (0x0526EC, i_0526ec),
    (0x0526EE, i_0526ee),
    (0x0526F0, i_0526f0),
    (0x0526F2, i_0526f2),
    (0x0526F4, i_0526f4),
    (0x0526F8, i_0526f8),
    (0x0526FA, i_0526fa),
    (0x052700, i_052700),
    (0x052706, i_052706),
    (0x05270C, i_05270c),
    (0x052712, i_052712),
    (0x052714, i_052714),
    (0x05271A, i_05271a),
    (0x052720, i_052720),
    (0x052726, i_052726),
    (0x05272C, i_05272c),
    (0x052732, i_052732),
    (0x052738, i_052738),
    (0x05273A, i_05273a),
    (0x05273E, i_05273e),
    (0x052742, i_052742),
    (0x052746, i_052746),
    (0x05274A, i_05274a),
    (0x05274E, i_05274e),
    (0x052750, i_052750),
    (0x052754, i_052754),
    (0x052756, i_052756),
    (0x052758, i_052758),
    (0x05275C, i_05275c),
    (0x052760, i_052760),
    (0x052764, i_052764),
    (0x05276A, i_05276a),
    (0x05276C, i_05276c),
    (0x05276E, i_05276e),
    (0x052774, i_052774),
    (0x052776, i_052776),
    (0x052778, i_052778),
    (0x05277E, i_05277e),
    (0x052780, i_052780),
    (0x052786, i_052786),
    (0x052788, i_052788),
    (0x05278E, i_05278e),
    (0x052794, i_052794),
    (0x052796, i_052796),
    (0x05279C, i_05279c),
    (0x05279E, i_05279e),
    (0x0527A2, i_0527a2),
    (0x0527A6, i_0527a6),
    (0x0527AA, i_0527aa),
    (0x0527AE, i_0527ae),
    (0x0527B2, i_0527b2),
    (0x0527B4, i_0527b4),
    (0x0527B8, i_0527b8),
    (0x0527BA, i_0527ba),
    (0x0527BE, i_0527be),
    (0x0527C4, i_0527c4),
    (0x0527CA, i_0527ca),
    (0x0527D0, i_0527d0),
    (0x0527D6, i_0527d6),
    (0x0527DC, i_0527dc),
    (0x0527E2, i_0527e2),
    (0x0527E8, i_0527e8),
    (0x0527EC, i_0527ec),
    (0x0527F2, i_0527f2),
    (0x0527F6, i_0527f6),
    (0x0527FC, i_0527fc),
    (0x052802, i_052802),
    (0x052806, i_052806),
    (0x052808, i_052808),
    (0x05280E, i_05280e),
    (0x052814, i_052814),
    (0x05281A, i_05281a),
    (0x05281E, i_05281e),
    (0x052824, i_052824),
    (0x05282A, i_05282a),
    (0x052830, i_052830),
    (0x052836, i_052836),
    (0x05283A, i_05283a),
    (0x052840, i_052840),
    (0x052844, i_052844),
    (0x05284A, i_05284a),
    (0x05284E, i_05284e),
    (0x052854, i_052854),
    (0x05285A, i_05285a),
    (0x052860, i_052860),
    (0x052864, i_052864),
    (0x052868, i_052868),
    (0x05286E, i_05286e),
    (0x052872, i_052872),
    (0x052878, i_052878),
    (0x05287A, i_05287a),
    (0x05287E, i_05287e),
    (0x052882, i_052882),
    (0x052888, i_052888),
    (0x05288E, i_05288e),
    (0x052892, i_052892),
    (0x052894, i_052894),
    (0x052898, i_052898),
    (0x05289C, i_05289c),
    (0x0528A0, i_0528a0),
    (0x0528A6, i_0528a6),
    (0x0528AA, i_0528aa),
    (0x0528B0, i_0528b0),
    (0x0528B4, i_0528b4),
    (0x0528BA, i_0528ba),
    (0x0528BC, i_0528bc),
    (0x0528C0, i_0528c0),
    (0x0528C6, i_0528c6),
    (0x0528CC, i_0528cc),
    (0x0528D2, i_0528d2),
    (0x0528D8, i_0528d8),
    (0x0528DE, i_0528de),
    (0x0528E4, i_0528e4),
    (0x0528E8, i_0528e8),
    (0x0528EA, i_0528ea),
    (0x0528F0, i_0528f0),
    (0x0528F6, i_0528f6),
    (0x0528FA, i_0528fa),
    (0x052900, i_052900),
    (0x052902, i_052902),
    (0x052906, i_052906),
    (0x05290C, i_05290c),
    (0x052910, i_052910),
    (0x052916, i_052916),
    (0x05291C, i_05291c),
    (0x052922, i_052922),
    (0x052928, i_052928),
    (0x05292E, i_05292e),
    (0x052934, i_052934),
    (0x05293A, i_05293a),
    (0x052940, i_052940),
    (0x052944, i_052944),
    (0x052946, i_052946),
    (0x05294C, i_05294c),
    (0x052952, i_052952),
    (0x052958, i_052958),
    (0x05295C, i_05295c),
    (0x052962, i_052962),
    (0x052968, i_052968),
    (0x05296E, i_05296e),
    (0x052972, i_052972),
    (0x052974, i_052974),
    (0x05297A, i_05297a),
    (0x05297E, i_05297e),
    (0x052984, i_052984),
    (0x052986, i_052986),
    (0x05298A, i_05298a),
    (0x052990, i_052990),
    (0x052996, i_052996),
    (0x05299C, i_05299c),
    (0x0529A0, i_0529a0),
    (0x0529A2, i_0529a2),
    (0x0529A8, i_0529a8),
    (0x0529AE, i_0529ae),
    (0x0529B2, i_0529b2),
    (0x0529B6, i_0529b6),
    (0x0529B8, i_0529b8),
    (0x0529BE, i_0529be),
    (0x0529C2, i_0529c2),
    (0x0529C8, i_0529c8),
    (0x0529CA, i_0529ca),
    (0x0529D0, i_0529d0),
    (0x0529D6, i_0529d6),
    (0x0529DC, i_0529dc),
    (0x0529E0, i_0529e0),
    (0x0529E2, i_0529e2),
    (0x0529E8, i_0529e8),
    (0x0529EE, i_0529ee),
    (0x0529F4, i_0529f4),
    (0x0529F8, i_0529f8),
    (0x0529FC, i_0529fc),
    (0x052A02, i_052a02),
    (0x052A08, i_052a08),
    (0x052A0E, i_052a0e),
    (0x052A14, i_052a14),
    (0x052A1A, i_052a1a),
    (0x052A1E, i_052a1e),
    (0x052A24, i_052a24),
    (0x052A26, i_052a26),
    (0x052A2A, i_052a2a),
    (0x052A30, i_052a30),
    (0x052A34, i_052a34),
    (0x052A3A, i_052a3a),
    (0x052A40, i_052a40),
    (0x052A46, i_052a46),
    (0x052A4C, i_052a4c),
    (0x052A52, i_052a52),
    (0x052A58, i_052a58),
    (0x052A5E, i_052a5e),
    (0x052A64, i_052a64),
    (0x052A68, i_052a68),
    (0x052A6A, i_052a6a),
    (0x052A70, i_052a70),
    (0x052A76, i_052a76),
    (0x052A7C, i_052a7c),
    (0x052A80, i_052a80),
    (0x052A86, i_052a86),
    (0x052A8A, i_052a8a),
    (0x052A90, i_052a90),
    (0x052A94, i_052a94),
    (0x052A9A, i_052a9a),
    (0x052A9E, i_052a9e),
    (0x052AA4, i_052aa4),
    (0x052AAA, i_052aaa),
    (0x052AB0, i_052ab0),
    (0x052AB6, i_052ab6),
    (0x052ABC, i_052abc),
    (0x052AC0, i_052ac0),
    (0x052AC6, i_052ac6),
    (0x052ACC, i_052acc),
    (0x052AD2, i_052ad2),
    (0x052AD6, i_052ad6),
    (0x052ADC, i_052adc),
    (0x052AE0, i_052ae0),
    (0x052AE2, i_052ae2),
    (0x052AE8, i_052ae8),
    (0x052AEC, i_052aec),
    (0x052AF2, i_052af2),
    (0x052AF4, i_052af4),
    (0x052AFA, i_052afa),
    (0x052B00, i_052b00),
    (0x052B06, i_052b06),
    (0x052B0A, i_052b0a),
    (0x052B0C, i_052b0c),
    (0x052B12, i_052b12),
    (0x052B18, i_052b18),
    (0x052B1E, i_052b1e),
    (0x052B22, i_052b22),
    (0x052B28, i_052b28),
    (0x052B2C, i_052b2c),
    (0x052B32, i_052b32),
    (0x052B36, i_052b36),
    (0x052B3C, i_052b3c),
    (0x052B40, i_052b40),
    (0x052B46, i_052b46),
    (0x052B4A, i_052b4a),
    (0x052B50, i_052b50),
    (0x052B56, i_052b56),
    (0x052B5C, i_052b5c),
    (0x052B62, i_052b62),
    (0x052B68, i_052b68),
    (0x052B6C, i_052b6c),
    (0x052B72, i_052b72),
    (0x052B78, i_052b78),
    (0x052B7E, i_052b7e),
    (0x052B82, i_052b82),
    (0x052B88, i_052b88),
    (0x052B8A, i_052b8a),
    (0x052B8E, i_052b8e),
    (0x052B94, i_052b94),
    (0x052B98, i_052b98),
    (0x052B9E, i_052b9e),
    (0x052BA4, i_052ba4),
    (0x052BAA, i_052baa),
    (0x052BB0, i_052bb0),
    (0x052BB6, i_052bb6),
    (0x052BBC, i_052bbc),
    (0x052BC2, i_052bc2),
    (0x052BC8, i_052bc8),
    (0x052BCE, i_052bce),
    (0x052BD4, i_052bd4),
    (0x052BDA, i_052bda),
    (0x052BE0, i_052be0),
    (0x052BE6, i_052be6),
    (0x052BEC, i_052bec),
    (0x052BF2, i_052bf2),
    (0x052BF6, i_052bf6),
    (0x052BF8, i_052bf8),
    (0x052BFE, i_052bfe),
    (0x052C04, i_052c04),
    (0x052C0A, i_052c0a),
    (0x052C0E, i_052c0e),
    (0x052C14, i_052c14),
    (0x052C1A, i_052c1a),
    (0x052C20, i_052c20),
    (0x052C26, i_052c26),
    (0x052C2C, i_052c2c),
    (0x052C32, i_052c32),
    (0x052C38, i_052c38),
    (0x052C3E, i_052c3e),
    (0x052C44, i_052c44),
    (0x052C48, i_052c48),
    (0x052C4C, i_052c4c),
    (0x052C50, i_052c50),
    (0x052C56, i_052c56),
    (0x052C5C, i_052c5c),
    (0x052C62, i_052c62),
    (0x052C66, i_052c66),
    (0x052C6C, i_052c6c),
    (0x052C70, i_052c70),
    (0x052C76, i_052c76),
    (0x052C7C, i_052c7c),
    (0x052C82, i_052c82),
    (0x052C86, i_052c86),
    (0x052C88, i_052c88),
    (0x052C8E, i_052c8e),
    (0x052C92, i_052c92),
    (0x052C98, i_052c98),
    (0x052C9E, i_052c9e),
    (0x052CA0, i_052ca0),
    (0x052CA6, i_052ca6),
    (0x052CAC, i_052cac),
    (0x052CB0, i_052cb0),
    (0x052CB4, i_052cb4),
    (0x052CBA, i_052cba),
    (0x052CC0, i_052cc0),
    (0x052CC6, i_052cc6),
    (0x052CCA, i_052cca),
    (0x052CCC, i_052ccc),
    (0x052CD2, i_052cd2),
    (0x052CD8, i_052cd8),
    (0x052CDE, i_052cde),
    (0x052CE2, i_052ce2),
    (0x052CE8, i_052ce8),
    (0x052CEA, i_052cea),
    (0x052CEE, i_052cee),
    (0x052CF4, i_052cf4),
    (0x052CF8, i_052cf8),
    (0x052CFE, i_052cfe),
    (0x052D04, i_052d04),
    (0x052D0A, i_052d0a),
    (0x052D10, i_052d10),
    (0x052D16, i_052d16),
    (0x052D1C, i_052d1c),
    (0x052D22, i_052d22),
    (0x052D28, i_052d28),
    (0x052D2C, i_052d2c),
    (0x052D2E, i_052d2e),
    (0x052D34, i_052d34),
    (0x052D3A, i_052d3a),
    (0x052D40, i_052d40),
    (0x052D44, i_052d44),
    (0x052D4A, i_052d4a),
    (0x052D50, i_052d50),
    (0x052D56, i_052d56),
    (0x052D5C, i_052d5c),
    (0x052D62, i_052d62),
    (0x052D68, i_052d68),
    (0x052D6E, i_052d6e),
    (0x052D74, i_052d74),
    (0x052D7A, i_052d7a),
    (0x052D7E, i_052d7e),
    (0x052D84, i_052d84),
    (0x052D88, i_052d88),
    (0x052D8E, i_052d8e),
    (0x052D94, i_052d94),
    (0x052D9A, i_052d9a),
    (0x052DA0, i_052da0),
    (0x052DA4, i_052da4),
    (0x052DAA, i_052daa),
    (0x052DB0, i_052db0),
    (0x052DB6, i_052db6),
    (0x052DBA, i_052dba),
    (0x052DBC, i_052dbc),
    (0x052DC2, i_052dc2),
    (0x052DC6, i_052dc6),
    (0x052DCC, i_052dcc),
    (0x052DCE, i_052dce),
    (0x052DD4, i_052dd4),
    (0x052DDA, i_052dda),
    (0x052DDE, i_052dde),
    (0x052DE2, i_052de2),
    (0x052DE8, i_052de8),
    (0x052DEC, i_052dec),
    (0x052DF0, i_052df0),
    (0x052DF6, i_052df6),
    (0x052DFC, i_052dfc),
    (0x052E00, i_052e00),
    (0x052E02, i_052e02),
    (0x052E08, i_052e08),
    (0x052E0E, i_052e0e),
    (0x052E14, i_052e14),
    (0x052E18, i_052e18),
    (0x052E1E, i_052e1e),
    (0x052E20, i_052e20),
    (0x052E24, i_052e24),
    (0x052E2A, i_052e2a),
    (0x052E30, i_052e30),
    (0x052E34, i_052e34),
    (0x052E3A, i_052e3a),
    (0x052E40, i_052e40),
    (0x052E44, i_052e44),
    (0x052E48, i_052e48),
    (0x052E4E, i_052e4e),
    (0x052E54, i_052e54),
    (0x052E58, i_052e58),
    (0x052E5E, i_052e5e),
    (0x052E64, i_052e64),
    (0x052E68, i_052e68),
    (0x052E6C, i_052e6c),
    (0x052E72, i_052e72),
    (0x052E78, i_052e78),
    (0x052E7C, i_052e7c),
    (0x052E82, i_052e82),
    (0x052E88, i_052e88),
    (0x052E8C, i_052e8c),
    (0x052E90, i_052e90),
    (0x052E96, i_052e96),
    (0x052E9C, i_052e9c),
    (0x052EA0, i_052ea0),
    (0x052EA6, i_052ea6),
    (0x052EAC, i_052eac),
    (0x052EB2, i_052eb2),
    (0x052EB8, i_052eb8),
    (0x052EBE, i_052ebe),
    (0x052EC4, i_052ec4),
    (0x052ECA, i_052eca),
    (0x052ED0, i_052ed0),
    (0x052ED6, i_052ed6),
    (0x052EDC, i_052edc),
    (0x052EE2, i_052ee2),
    (0x052EE8, i_052ee8),
    (0x052EEE, i_052eee),
    (0x052EF4, i_052ef4),
    (0x052EF8, i_052ef8),
    (0x052EFE, i_052efe),
    (0x052F04, i_052f04),
    (0x052F06, i_052f06),
    (0x052F0C, i_052f0c),
    (0x052F12, i_052f12),
    (0x052F16, i_052f16),
    (0x052F1A, i_052f1a),
    (0x052F20, i_052f20),
    (0x052F26, i_052f26),
    (0x052F2A, i_052f2a),
    (0x052F30, i_052f30),
    (0x052F36, i_052f36),
    (0x052F3A, i_052f3a),
    (0x052F3E, i_052f3e),
    (0x052F44, i_052f44),
    (0x052F4A, i_052f4a),
    (0x052F4E, i_052f4e),
    (0x052F54, i_052f54),
    (0x052F5A, i_052f5a),
    (0x052F60, i_052f60),
    (0x052F66, i_052f66),
    (0x052F6C, i_052f6c),
    (0x052F72, i_052f72),
    (0x052F78, i_052f78),
    (0x052F7E, i_052f7e),
    (0x052F84, i_052f84),
    (0x052F8A, i_052f8a),
    (0x052F90, i_052f90),
    (0x052F96, i_052f96),
    (0x052F9C, i_052f9c),
    (0x052FA2, i_052fa2),
    (0x052FA6, i_052fa6),
    (0x052FAC, i_052fac),
    (0x052FB2, i_052fb2),
    (0x052FB4, i_052fb4),
    (0x052FBA, i_052fba),
    (0x052FC0, i_052fc0),
    (0x052FC6, i_052fc6),
    (0x052FCC, i_052fcc),
    (0x052FD2, i_052fd2),
    (0x052FD6, i_052fd6),
    (0x052FD8, i_052fd8),
    (0x052FDE, i_052fde),
    (0x052FE4, i_052fe4),
    (0x052FEA, i_052fea),
    (0x052FEE, i_052fee),
    (0x052FF4, i_052ff4),
    (0x052FFA, i_052ffa),
    (0x053000, i_053000),
    (0x053006, i_053006),
    (0x05300A, i_05300a),
    (0x05300E, i_05300e),
    (0x053014, i_053014),
    (0x053018, i_053018),
    (0x05301C, i_05301c),
    (0x053022, i_053022),
    (0x053028, i_053028),
    (0x05302E, i_05302e),
    (0x053034, i_053034),
    (0x05303A, i_05303a),
    (0x053040, i_053040),
    (0x053046, i_053046),
    (0x05304C, i_05304c),
    (0x053052, i_053052),
    (0x053058, i_053058),
    (0x05305E, i_05305e),
    (0x053064, i_053064),
    (0x05306A, i_05306a),
    (0x053070, i_053070),
    (0x053076, i_053076),
    (0x05307C, i_05307c),
    (0x053082, i_053082),
    (0x053088, i_053088),
    (0x05308E, i_05308e),
    (0x053094, i_053094),
    (0x05309A, i_05309a),
    (0x0530A0, i_0530a0),
    (0x0530A6, i_0530a6),
    (0x0530AC, i_0530ac),
    (0x0530B0, i_0530b0),
    (0x0530B6, i_0530b6),
    (0x0530BA, i_0530ba),
    (0x0530C0, i_0530c0),
    (0x0530C4, i_0530c4),
    (0x0530CA, i_0530ca),
    (0x0530CC, i_0530cc),
    (0x0530D0, i_0530d0),
    (0x0530D6, i_0530d6),
    (0x0530DC, i_0530dc),
    (0x0530E2, i_0530e2),
    (0x0530E6, i_0530e6),
    (0x0530EC, i_0530ec),
    (0x0530F2, i_0530f2),
    (0x0530F6, i_0530f6),
    (0x0530FA, i_0530fa),
    (0x053100, i_053100),
    (0x053106, i_053106),
    (0x05310C, i_05310c),
    (0x053110, i_053110),
    (0x053116, i_053116),
    (0x05311C, i_05311c),
    (0x053120, i_053120),
    (0x053124, i_053124),
    (0x05312A, i_05312a),
    (0x053130, i_053130),
    (0x053136, i_053136),
    (0x05313A, i_05313a),
    (0x053140, i_053140),
    (0x053146, i_053146),
    (0x05314A, i_05314a),
    (0x05314E, i_05314e),
    (0x053154, i_053154),
    (0x05315A, i_05315a),
    (0x053160, i_053160),
    (0x053164, i_053164),
    (0x05316A, i_05316a),
    (0x053170, i_053170),
    (0x053174, i_053174),
    (0x053178, i_053178),
    (0x05317E, i_05317e),
    (0x053184, i_053184),
    (0x05318A, i_05318a),
    (0x05318E, i_05318e),
    (0x053194, i_053194),
    (0x05319A, i_05319a),
    (0x05319E, i_05319e),
    (0x0531A2, i_0531a2),
    (0x0531A8, i_0531a8),
    (0x0531AE, i_0531ae),
    (0x0531B4, i_0531b4),
    (0x0531B8, i_0531b8),
    (0x0531BE, i_0531be),
    (0x0531C4, i_0531c4),
    (0x0531C8, i_0531c8),
    (0x0531CC, i_0531cc),
    (0x0531D2, i_0531d2),
    (0x0531D8, i_0531d8),
    (0x0531DE, i_0531de),
    (0x0531E2, i_0531e2),
    (0x0531E8, i_0531e8),
    (0x0531EE, i_0531ee),
    (0x0531F2, i_0531f2),
    (0x0531F6, i_0531f6),
    (0x0531FC, i_0531fc),
    (0x053202, i_053202),
    (0x053208, i_053208),
    (0x05320C, i_05320c),
    (0x053212, i_053212),
    (0x053218, i_053218),
    (0x05321C, i_05321c),
    (0x053220, i_053220),
    (0x053226, i_053226),
    (0x05322C, i_05322c),
    (0x053232, i_053232),
    (0x053236, i_053236),
    (0x05323C, i_05323c),
    (0x053242, i_053242),
    (0x053246, i_053246),
    (0x05324A, i_05324a),
    (0x053250, i_053250),
    (0x053256, i_053256),
    (0x05325C, i_05325c),
    (0x053260, i_053260),
    (0x053266, i_053266),
    (0x05326C, i_05326c),
    (0x053272, i_053272),
    (0x053278, i_053278),
    (0x05327C, i_05327c),
    (0x05327E, i_05327e),
    (0x053284, i_053284),
    (0x05328A, i_05328a),
    (0x053290, i_053290),
    (0x053294, i_053294),
    (0x05329A, i_05329a),
    (0x0532A0, i_0532a0),
    (0x0532A6, i_0532a6),
    (0x0532AC, i_0532ac),
    (0x0532B0, i_0532b0),
    (0x0532B4, i_0532b4),
    (0x0532B8, i_0532b8),
    (0x0532BC, i_0532bc),
    (0x0532C2, i_0532c2),
    (0x0532C8, i_0532c8),
    (0x0532CE, i_0532ce),
    (0x0532D4, i_0532d4),
    (0x0532DA, i_0532da),
    (0x0532E0, i_0532e0),
    (0x0532E6, i_0532e6),
    (0x0532EA, i_0532ea),
    (0x0532F0, i_0532f0),
    (0x0532F4, i_0532f4),
    (0x0532FA, i_0532fa),
    (0x0532FE, i_0532fe),
    (0x053304, i_053304),
    (0x053306, i_053306),
    (0x05330A, i_05330a),
    (0x053310, i_053310),
    (0x053314, i_053314),
    (0x05331A, i_05331a),
    (0x053320, i_053320),
    (0x053326, i_053326),
    (0x05332C, i_05332c),
    (0x053332, i_053332),
    (0x053338, i_053338),
    (0x053340, i_053340),
    (0x053346, i_053346),
    (0x05334A, i_05334a),
    (0x053350, i_053350),
    (0x053356, i_053356),
    (0x05335A, i_05335a),
    (0x05335C, i_05335c),
    (0x053362, i_053362),
    (0x053368, i_053368),
    (0x05336E, i_05336e),
    (0x053374, i_053374),
    (0x05337A, i_05337a),
    (0x05337E, i_05337e),
    (0x053384, i_053384),
    (0x05338A, i_05338a),
    (0x053390, i_053390),
    (0x053396, i_053396),
    (0x05339A, i_05339a),
    (0x0533A0, i_0533a0),
    (0x0533A4, i_0533a4),
    (0x0533AA, i_0533aa),
    (0x0533B0, i_0533b0),
    (0x0533B6, i_0533b6),
    (0x0533BC, i_0533bc),
    (0x0533C2, i_0533c2),
    (0x0533C6, i_0533c6),
    (0x0533C8, i_0533c8),
    (0x0533CE, i_0533ce),
    (0x0533D2, i_0533d2),
    (0x0533D8, i_0533d8),
    (0x0533DA, i_0533da),
    (0x0533E0, i_0533e0),
    (0x0533E6, i_0533e6),
    (0x0533EC, i_0533ec),
    (0x0533F2, i_0533f2),
    (0x0533F8, i_0533f8),
    (0x0533FC, i_0533fc),
    (0x0533FE, i_0533fe),
    (0x053404, i_053404),
    (0x05340A, i_05340a),
    (0x053410, i_053410),
    (0x053416, i_053416),
    (0x05341C, i_05341c),
    (0x053420, i_053420),
    (0x053426, i_053426),
    (0x05342C, i_05342c),
    (0x053432, i_053432),
    (0x053438, i_053438),
    (0x05343C, i_05343c),
    (0x053440, i_053440),
    (0x053446, i_053446),
    (0x05344C, i_05344c),
    (0x053452, i_053452),
    (0x053458, i_053458),
    (0x05345E, i_05345e),
    (0x053464, i_053464),
    (0x05346A, i_05346a),
    (0x05346E, i_05346e),
    (0x053470, i_053470),
    (0x053476, i_053476),
    (0x05347A, i_05347a),
    (0x053480, i_053480),
    (0x053482, i_053482),
    (0x053488, i_053488),
    (0x05348E, i_05348e),
    (0x053494, i_053494),
    (0x05349A, i_05349a),
    (0x0534A0, i_0534a0),
    (0x0534A4, i_0534a4),
    (0x0534A8, i_0534a8),
    (0x0534AE, i_0534ae),
    (0x0534B4, i_0534b4),
    (0x0534B8, i_0534b8),
    (0x0534BA, i_0534ba),
    (0x0534C0, i_0534c0),
    (0x0534C6, i_0534c6),
    (0x0534CC, i_0534cc),
    (0x0534D0, i_0534d0),
    (0x0534D6, i_0534d6),
    (0x0534D8, i_0534d8),
    (0x0534DC, i_0534dc),
    (0x0534E2, i_0534e2),
    (0x0534E6, i_0534e6),
    (0x0534EC, i_0534ec),
    (0x0534F2, i_0534f2),
    (0x0534F8, i_0534f8),
    (0x0534FE, i_0534fe),
    (0x053504, i_053504),
    (0x053508, i_053508),
    (0x05350A, i_05350a),
    (0x053510, i_053510),
    (0x053516, i_053516),
    (0x05351C, i_05351c),
    (0x053520, i_053520),
    (0x053526, i_053526),
    (0x05352A, i_05352a),
    (0x053530, i_053530),
    (0x053534, i_053534),
    (0x05353A, i_05353a),
    (0x053540, i_053540),
    (0x053546, i_053546),
    (0x05354C, i_05354c),
    (0x053550, i_053550),
    (0x053556, i_053556),
    (0x05355A, i_05355a),
    (0x053560, i_053560),
    (0x053566, i_053566),
    (0x05356C, i_05356c),
    (0x053572, i_053572),
    (0x053578, i_053578),
    (0x05357C, i_05357c),
    (0x05357E, i_05357e),
    (0x053584, i_053584),
    (0x053588, i_053588),
    (0x05358E, i_05358e),
    (0x053590, i_053590),
    (0x053596, i_053596),
    (0x05359C, i_05359c),
    (0x05359E, i_05359e),
    (0x0535A4, i_0535a4),
    (0x0535A8, i_0535a8),
    (0x0535AC, i_0535ac),
    (0x0535B0, i_0535b0),
    (0x0535B4, i_0535b4),
    (0x0535BA, i_0535ba),
    (0x0535C0, i_0535c0),
    (0x0535C6, i_0535c6),
    (0x0535CA, i_0535ca),
    (0x0535CC, i_0535cc),
    (0x0535D2, i_0535d2),
    (0x0535D8, i_0535d8),
    (0x0535DE, i_0535de),
    (0x0535E0, i_0535e0),
    (0x0535E6, i_0535e6),
    (0x0535EA, i_0535ea),
    (0x0535F0, i_0535f0),
    (0x0535F6, i_0535f6),
    (0x0535FC, i_0535fc),
    (0x053602, i_053602),
    (0x053606, i_053606),
    (0x05360C, i_05360c),
    (0x053610, i_053610),
    (0x053616, i_053616),
    (0x05361C, i_05361c),
    (0x053622, i_053622),
    (0x053628, i_053628),
    (0x05362E, i_05362e),
    (0x053634, i_053634),
    (0x05363A, i_05363a),
    (0x05363E, i_05363e),
    (0x053640, i_053640),
];
