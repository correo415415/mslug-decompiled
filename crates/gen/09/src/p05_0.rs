// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p05_0) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $050004: addi.w #$70, $24(a0)
pub fn i_050004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5000A;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x70, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $05000A: movem.l a6, -(a7)
pub fn i_05000a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5000E;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $05000E: movem.l a0, -(a7)
pub fn i_05000e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50012;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $050012: movea.l a0, a6
pub fn i_050012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50014;
    cpu.ir = 0x2C48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[6] = v1;
    4
}
/// $050014: lea.l $2946a2.l, a0
pub fn i_050014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5001A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2946A2;
    20
}
/// $05001A: jsr $28cd4.l
pub fn i_05001a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50020;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50020);
    cpu.pc = 0x28CD4;
    16
}
/// $050020: movem.l (a7)+, a0
pub fn i_050020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50024;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $050024: movem.l (a7)+, a6
pub fn i_050024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50028;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $050028: rts
pub fn i_050028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5002A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Fortress_SpawnPiece01_05002a  ($05002A, 84 B, enemies/bosses/allen_oneil_04fa58.s)
/// $05002A: lea.l $4f1f4(pc), a1
pub fn i_05002a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5002E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F1F4;
    16
}
/// $05002E: jsr $4ae.l
pub fn i_05002e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50034;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50034);
    cpu.pc = 0x4AE;
    16
}
/// $050034: jsr $5dd22.l
pub fn i_050034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5003A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5003A);
    cpu.pc = 0x5DD22;
    16
}
/// $05003A: move.b #$1, $21(a0)
pub fn i_05003a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50040;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $050040: lea.l $295a6a.l, a2
pub fn i_050040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50046;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x295A6A;
    20
}
/// $050046: move.l a2, $70(a0)
pub fn i_050046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5004A;
    cpu.ir = 0x214A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $05004A: move.l #$ffffffff, $74(a0)
pub fn i_05004a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50052;
    cpu.ir = 0x217C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $050052: addi.w #$0, $22(a0)
pub fn i_050052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50058;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $050058: addi.w #$90, $24(a0)
pub fn i_050058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5005E;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x90, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $05005E: movem.l a6, -(a7)
pub fn i_05005e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50062;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $050062: movem.l a0, -(a7)
pub fn i_050062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50066;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $050066: movea.l a0, a6
pub fn i_050066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50068;
    cpu.ir = 0x2C48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[6] = v1;
    4
}
/// $050068: lea.l $2946fa.l, a0
pub fn i_050068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5006E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2946FA;
    20
}
/// $05006E: jsr $28cd4.l
pub fn i_05006e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50074;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50074);
    cpu.pc = 0x28CD4;
    16
}
/// $050074: movem.l (a7)+, a0
pub fn i_050074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50078;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $050078: movem.l (a7)+, a6
pub fn i_050078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5007C;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $05007C: rts
pub fn i_05007c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5007E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Fortress_SpawnPiece02_05007e  ($05007E, 84 B, enemies/bosses/allen_oneil_04fa58.s)
/// $05007E: lea.l $4f1f4(pc), a1
pub fn i_05007e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50082;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F1F4;
    16
}
/// $050082: jsr $4ae.l
pub fn i_050082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50088;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50088);
    cpu.pc = 0x4AE;
    16
}
/// $050088: jsr $5dd22.l
pub fn i_050088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5008E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5008E);
    cpu.pc = 0x5DD22;
    16
}
/// $05008E: move.b #$2, $21(a0)
pub fn i_05008e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50094;
    cpu.ir = 0x117C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $050094: lea.l $295a7e.l, a2
pub fn i_050094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5009A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x295A7E;
    20
}
/// $05009A: move.l a2, $70(a0)
pub fn i_05009a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5009E;
    cpu.ir = 0x214A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $05009E: move.l #$ffffffff, $74(a0)
pub fn i_05009e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x500A6;
    cpu.ir = 0x217C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0500A6: addi.w #$20, $22(a0)
pub fn i_0500a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x500AC;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0500AC: addi.w #$90, $24(a0)
pub fn i_0500ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x500B2;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x90, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0500B2: movem.l a6, -(a7)
pub fn i_0500b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x500B6;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $0500B6: movem.l a0, -(a7)
pub fn i_0500b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x500BA;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $0500BA: movea.l a0, a6
pub fn i_0500ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x500BC;
    cpu.ir = 0x2C48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[6] = v1;
    4
}
/// $0500BC: lea.l $294710.l, a0
pub fn i_0500bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x500C2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x294710;
    20
}
/// $0500C2: jsr $28cd4.l
pub fn i_0500c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x500C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x500C8);
    cpu.pc = 0x28CD4;
    16
}
/// $0500C8: movem.l (a7)+, a0
pub fn i_0500c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x500CC;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $0500CC: movem.l (a7)+, a6
pub fn i_0500cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x500D0;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $0500D0: rts
pub fn i_0500d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x500D2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Fortress_SpawnPiece04_0500d2  ($0500D2, 84 B, enemies/bosses/allen_oneil_04fa58.s)
/// $0500D2: lea.l $4f1f4(pc), a1
pub fn i_0500d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x500D6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F1F4;
    16
}
/// $0500D6: jsr $4ae.l
pub fn i_0500d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x500DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x500DC);
    cpu.pc = 0x4AE;
    16
}
/// $0500DC: jsr $5dd22.l
pub fn i_0500dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x500E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x500E2);
    cpu.pc = 0x5DD22;
    16
}
/// $0500E2: move.b #$4, $21(a0)
pub fn i_0500e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x500E8;
    cpu.ir = 0x117C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0500E8: lea.l $295a92.l, a2
pub fn i_0500e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x500EE;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x295A92;
    20
}
/// $0500EE: move.l a2, $70(a0)
pub fn i_0500ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x500F2;
    cpu.ir = 0x214A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0500F2: move.l #$ffffffff, $74(a0)
pub fn i_0500f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x500FA;
    cpu.ir = 0x217C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0500FA: addi.w #$20, $22(a0)
pub fn i_0500fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50100;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $050100: addi.w #$70, $24(a0)
pub fn i_050100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50106;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x70, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $050106: movem.l a6, -(a7)
pub fn i_050106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5010A;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $05010A: movem.l a0, -(a7)
pub fn i_05010a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5010E;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $05010E: movea.l a0, a6
pub fn i_05010e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50110;
    cpu.ir = 0x2C48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[6] = v1;
    4
}
/// $050110: lea.l $294726.l, a0
pub fn i_050110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50116;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x294726;
    20
}
/// $050116: jsr $28cd4.l
pub fn i_050116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5011C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5011C);
    cpu.pc = 0x28CD4;
    16
}
/// $05011C: movem.l (a7)+, a0
pub fn i_05011c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50120;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $050120: movem.l (a7)+, a6
pub fn i_050120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50124;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $050124: rts
pub fn i_050124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50126;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Fortress_SpawnPiece08_050126  ($050126, 84 B, enemies/bosses/allen_oneil_04fa58.s)
/// $050126: lea.l $4f1f4(pc), a1
pub fn i_050126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5012A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F1F4;
    16
}
/// $05012A: jsr $4ae.l
pub fn i_05012a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50130;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50130);
    cpu.pc = 0x4AE;
    16
}
/// $050130: jsr $5dd22.l
pub fn i_050130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50136;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50136);
    cpu.pc = 0x5DD22;
    16
}
/// $050136: move.b #$8, $21(a0)
pub fn i_050136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5013C;
    cpu.ir = 0x117C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05013C: lea.l $295aa6.l, a2
pub fn i_05013c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50142;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x295AA6;
    20
}
/// $050142: move.l a2, $70(a0)
pub fn i_050142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50146;
    cpu.ir = 0x214A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $050146: move.l #$ffffffff, $74(a0)
pub fn i_050146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5014E;
    cpu.ir = 0x217C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $05014E: addi.w #$40, $22(a0)
pub fn i_05014e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50154;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $050154: addi.w #$70, $24(a0)
pub fn i_050154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5015A;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x70, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $05015A: movem.l a6, -(a7)
pub fn i_05015a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5015E;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $05015E: movem.l a0, -(a7)
pub fn i_05015e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50162;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $050162: movea.l a0, a6
pub fn i_050162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50164;
    cpu.ir = 0x2C48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[6] = v1;
    4
}
/// $050164: lea.l $29473c.l, a0
pub fn i_050164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5016A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29473C;
    20
}
/// $05016A: jsr $28cd4.l
pub fn i_05016a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50170;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50170);
    cpu.pc = 0x28CD4;
    16
}
/// $050170: movem.l (a7)+, a0
pub fn i_050170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50174;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $050174: movem.l (a7)+, a6
pub fn i_050174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50178;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $050178: rts
pub fn i_050178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5017A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Fortress_SpawnPiece10_05017a  ($05017A, 94 B, enemies/bosses/allen_oneil_04fa58.s)
/// $05017A: lea.l $4f1f4(pc), a1
pub fn i_05017a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5017E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4F1F4;
    16
}
/// $05017E: jsr $4ae.l
pub fn i_05017e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50184;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50184);
    cpu.pc = 0x4AE;
    16
}
/// $050184: jsr $5dd22.l
pub fn i_050184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5018A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5018A);
    cpu.pc = 0x5DD22;
    16
}
/// $05018A: move.b #$10, $21(a0)
pub fn i_05018a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50190;
    cpu.ir = 0x117C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $050190: lea.l $295aba.l, a2
pub fn i_050190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50196;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x295ABA;
    20
}
/// $050196: move.l a2, $70(a0)
pub fn i_050196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5019A;
    cpu.ir = 0x214A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $05019A: lea.l $295ace.l, a2
pub fn i_05019a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x501A0;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x295ACE;
    20
}
/// $0501A0: move.l a2, $74(a0)
pub fn i_0501a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x501A4;
    cpu.ir = 0x214A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0501A4: move.l #$ffffffff, $78(a0)
pub fn i_0501a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x501AC;
    cpu.ir = 0x217C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x78);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0501AC: addi.w #$60, $22(a0)
pub fn i_0501ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x501B2;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x60, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0501B2: addi.w #$10, $24(a0)
pub fn i_0501b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x501B8;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0501B8: movem.l a6, -(a7)
pub fn i_0501b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x501BC;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $0501BC: movem.l a0, -(a7)
pub fn i_0501bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x501C0;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $0501C0: movea.l a0, a6
pub fn i_0501c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x501C2;
    cpu.ir = 0x2C48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[6] = v1;
    4
}
/// $0501C2: lea.l $2946b8.l, a0
pub fn i_0501c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x501C8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2946B8;
    20
}
/// $0501C8: jsr $28cd4.l
pub fn i_0501c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x501CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x501CE);
    cpu.pc = 0x28CD4;
    16
}
/// $0501CE: movem.l (a7)+, a0
pub fn i_0501ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x501D2;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $0501D2: movem.l (a7)+, a6
pub fn i_0501d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x501D6;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $0501D6: rts
pub fn i_0501d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x501D8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gatehouse_BlitDamaged_0501d8  ($0501D8, 76 B, enemies/bosses/allen_oneil_04fa58.s)
/// $0501D8: lea.l $295902.l, a2
pub fn i_0501d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x501DE;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x295902;
    20
}
/// $0501DE: jsr $5022a(pc)
pub fn i_0501de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x501E2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x501E2);
    cpu.pc = 0x5022A;
    16
}
/// $0501E2: lea.l $29592a.l, a2
pub fn i_0501e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x501E8;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29592A;
    20
}
/// $0501E8: jsr $5022a(pc)
pub fn i_0501e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x501EC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x501EC);
    cpu.pc = 0x5022A;
    16
}
/// $0501EC: lea.l $295952.l, a2
pub fn i_0501ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x501F2;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x295952;
    20
}
/// $0501F2: jsr $5022a(pc)
pub fn i_0501f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x501F6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x501F6);
    cpu.pc = 0x5022A;
    16
}
/// $0501F6: lea.l $29597a.l, a2
pub fn i_0501f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x501FC;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29597A;
    20
}
/// $0501FC: jsr $5022a(pc)
pub fn i_0501fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50200;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x50200);
    cpu.pc = 0x5022A;
    16
}
/// $050200: lea.l $2959a2.l, a2
pub fn i_050200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50206;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2959A2;
    20
}
/// $050206: jsr $5022a(pc)
pub fn i_050206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5020A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5020A);
    cpu.pc = 0x5022A;
    16
}
/// $05020A: lea.l $2959ca.l, a2
pub fn i_05020a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50210;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2959CA;
    20
}
/// $050210: jsr $5022a(pc)
pub fn i_050210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50214;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x50214);
    cpu.pc = 0x5022A;
    16
}
/// $050214: lea.l $2959f2.l, a2
pub fn i_050214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5021A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2959F2;
    20
}
/// $05021A: jsr $5022a(pc)
pub fn i_05021a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5021E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5021E);
    cpu.pc = 0x5022A;
    16
}
/// $05021E: lea.l $295a1a.l, a2
pub fn i_05021e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50224;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x295A1A;
    20
}
// ===== JsrPcThunk_050224  ($050224, 6 B, thunks/jsr_pc_thunks.c)
/// $050224: jsr $5022a(pc)
pub fn i_050224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50228;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x50228);
    cpu.pc = 0x5022A;
    16
}
/// $050228: rts
pub fn i_050228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5022A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sprite_InvokeBlit8Params  ($05022A, 38 B, video/sprite_invoke_05022a.s)
/// $05022A: movea.l (a2), a0
pub fn i_05022a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5022C;
    cpu.ir = 0x2052;
    let v2 = bus.read32(cpu.a[2]);
    cpu.a[0] = v2;
    12
}
/// $05022C: movea.l $4(a2), a1
pub fn i_05022c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50230;
    cpu.ir = 0x226A;
    let ad2 = cpu.a[2].wrapping_add(0x4);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $050230: move.w $8(a2), d0
pub fn i_050230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50234;
    cpu.ir = 0x302A;
    let ad2 = cpu.a[2].wrapping_add(0x8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $050234: move.w $a(a2), d1
pub fn i_050234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50238;
    cpu.ir = 0x322A;
    let ad2 = cpu.a[2].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $050238: move.w $c(a2), d2
pub fn i_050238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5023C;
    cpu.ir = 0x342A;
    let ad2 = cpu.a[2].wrapping_add(0xC);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05023C: move.w $e(a2), d3
pub fn i_05023c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50240;
    cpu.ir = 0x362A;
    let ad2 = cpu.a[2].wrapping_add(0xE);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $050240: move.w $10(a2), d4
pub fn i_050240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50244;
    cpu.ir = 0x382A;
    let ad2 = cpu.a[2].wrapping_add(0x10);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $050244: move.w $12(a2), d5
pub fn i_050244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50248;
    cpu.ir = 0x3A2A;
    let ad2 = cpu.a[2].wrapping_add(0x12);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $050248: jsr $51de2.l
pub fn i_050248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5024E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5024E);
    cpu.pc = 0x51DE2;
    16
}
/// $05024E: rts
pub fn i_05024e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50250;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SlotPrioCheck_050250  ($050250, 16 B, enemies/bosses/allen_oneil_04fa58.s)
/// $050250: movea.l $8(a6), a1
pub fn i_050250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50254;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $050254: move.b $10(a6), d0
pub fn i_050254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50258;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $050258: cmp.b $10(a1), d0
pub fn i_050258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5025C;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $05025C: bcs.w $50266
pub fn i_05025c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50260;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x50266; return 10; }
    8
}
// ===== ClearXN_050260  ($050260, 6 B, thunks/ccr_helpers.c)
/// $050260: andi.b #$ee, ccr
pub fn i_050260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50264;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $050264: rts
pub fn i_050264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50266;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_050266  ($050266, 6 B, thunks/ccr_helpers.c)
/// $050266: ori.b #$11, ccr
pub fn i_050266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5026A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05026A: rts
pub fn i_05026a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5026C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Blit_Loop48Iters_05026C  ($05026C, 48 B, video/blit_loop_48_iters_05026c.s)
/// $05026C: move.w #$0, d0
pub fn i_05026c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50270;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $050270: move.w #$f0, d1
pub fn i_050270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50274;
    cpu.ir = 0x323C;
    let v1: u32 = 0xF0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $050274: jsr $44022.l
pub fn i_050274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5027A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5027A);
    cpu.pc = 0x44022;
    16
}
/// $05027A: move.w #$30, -(a7)
pub fn i_05027a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5027E;
    cpu.ir = 0x3F3C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $05027E: move.w d0, -(a7)
pub fn i_05027e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50280;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $050280: move.w d1, -(a7)
pub fn i_050280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50282;
    cpu.ir = 0x3F01;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $050282: move.b #$1, d7
pub fn i_050282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50286;
    cpu.ir = 0x1E3C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v1 & 0xFF);
    4
}
/// $050286: jsr $43f5e.l
pub fn i_050286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5028C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5028C);
    cpu.pc = 0x43F5E;
    16
}
/// $05028C: move.w (a7)+, d1
pub fn i_05028c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5028E;
    cpu.ir = 0x321F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05028E: move.w (a7)+, d0
pub fn i_05028e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50290;
    cpu.ir = 0x301F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $050290: addq.w #$8, d0
pub fn i_050290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50292;
    cpu.ir = 0x5040;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $050292: subq.w #$1, (a7)
pub fn i_050292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50294;
    cpu.ir = 0x5357;
    let ad1 = cpu.a[7];
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $050294: bne.b $5027e
pub fn i_050294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50296;
    cpu.ir = 0x66E8;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5027E; return 10; }
    8
}
/// $050296: adda.w #$2, a7
pub fn i_050296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5029A;
    cpu.ir = 0xDEFC;
    let v1: u32 = 0x2;
    cpu.a[7] = cpu.a[7].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05029A: rts
pub fn i_05029a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5029C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_StepMoveWithProbe_05029c  ($05029C, 196 B, enemies/bosses/allen_oneil_04fa58.s)
/// $05029C: move.l #$27bc8, -(a7)
pub fn i_05029c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x502A2;
    cpu.ir = 0x2F3C;
    let v1: u32 = 0x27BC8;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $0502A2: bra.w $502ac
pub fn i_0502a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x502A6;
    cpu.ir = 0x6000;
    cpu.pc = 0x502AC;
    10
}
/// $0502A6: move.l #$27d50, -(a7)
pub fn i_0502a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x502AC;
    cpu.ir = 0x2F3C;
    let v1: u32 = 0x27D50;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $0502AC: move.w $2a(a6), -(a7)
pub fn i_0502ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x502B0;
    cpu.ir = 0x3F2E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write16(ad4, v3 as u16);
    18
}
/// $0502B0: move.w $28(a6), -(a7)
pub fn i_0502b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x502B4;
    cpu.ir = 0x3F2E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write16(ad4, v3 as u16);
    18
}
/// $0502B4: move.w $2e(a6), -(a7)
pub fn i_0502b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x502B8;
    cpu.ir = 0x3F2E;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write16(ad4, v3 as u16);
    18
}
/// $0502B8: move.w $2c(a6), -(a7)
pub fn i_0502b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x502BC;
    cpu.ir = 0x3F2E;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write16(ad4, v3 as u16);
    18
}
/// $0502BC: clr.w $2c(a6)
pub fn i_0502bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x502C0;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0502C0: clr.w $2e(a6)
pub fn i_0502c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x502C4;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0502C4: move.w $28(a6), -(a7)
pub fn i_0502c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x502C8;
    cpu.ir = 0x3F2E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write16(ad4, v3 as u16);
    18
}
/// $0502C8: bmi.w $502d6
pub fn i_0502c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x502CC;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x502D6; return 10; }
    8
}
/// $0502CC: move.w #$800, $28(a6)
pub fn i_0502cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x502D2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x800;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0502D2: bra.w $502dc
pub fn i_0502d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x502D6;
    cpu.ir = 0x6000;
    cpu.pc = 0x502DC;
    10
}
/// $0502D6: move.w #$f800, $28(a6)
pub fn i_0502d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x502DC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF800;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0502DC: move.w $2a(a6), -(a7)
pub fn i_0502dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x502E0;
    cpu.ir = 0x3F2E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write16(ad4, v3 as u16);
    18
}
/// $0502E0: bmi.w $502ee
pub fn i_0502e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x502E4;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x502EE; return 10; }
    8
}
/// $0502E4: move.w #$800, $2a(a6)
pub fn i_0502e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x502EA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x800;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0502EA: bra.w $502f4
pub fn i_0502ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x502EE;
    cpu.ir = 0x6000;
    cpu.pc = 0x502F4;
    10
}
/// $0502EE: move.w #$f800, $2a(a6)
pub fn i_0502ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x502F4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF800;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0502F4: move.w $2(a7), d0
pub fn i_0502f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x502F8;
    cpu.ir = 0x302F;
    let ad2 = cpu.a[7].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0502F8: smi.b d1
pub fn i_0502f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x502FA;
    cpu.ir = 0x5BC1;
    let c1: u32 = if cpu.sr.evaluate(Condition::Mi) { 0xFF } else { 0 };
    let v2 = cpu.d[1] & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (c1 & 0xFF);
    4
}
/// $0502FA: sub.w $28(a6), d0
pub fn i_0502fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x502FE;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0502FE: smi.b d2
pub fn i_0502fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50300;
    cpu.ir = 0x5BC2;
    let c1: u32 = if cpu.sr.evaluate(Condition::Mi) { 0xFF } else { 0 };
    let v2 = cpu.d[2] & 0xFF;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (c1 & 0xFF);
    4
}
/// $050300: eor.b d1, d2
pub fn i_050300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50302;
    cpu.ir = 0xB302;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 ^ cpu.d[1]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    4
}
/// $050302: beq.w $5030c
pub fn i_050302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50306;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5030C; return 10; }
    8
}
/// $050306: add.w d0, $28(a6)
pub fn i_050306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5030A;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $05030A: clr.w d0
pub fn i_05030a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5030C;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05030C: move.w d0, $2(a7)
pub fn i_05030c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50310;
    cpu.ir = 0x3F40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $050310: move.w (a7), d0
pub fn i_050310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50312;
    cpu.ir = 0x3017;
    let v2 = bus.read16(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $050312: smi.b d1
pub fn i_050312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50314;
    cpu.ir = 0x5BC1;
    let c1: u32 = if cpu.sr.evaluate(Condition::Mi) { 0xFF } else { 0 };
    let v2 = cpu.d[1] & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (c1 & 0xFF);
    4
}
/// $050314: sub.w $2a(a6), d0
pub fn i_050314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50318;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $050318: smi.b d2
pub fn i_050318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5031A;
    cpu.ir = 0x5BC2;
    let c1: u32 = if cpu.sr.evaluate(Condition::Mi) { 0xFF } else { 0 };
    let v2 = cpu.d[2] & 0xFF;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (c1 & 0xFF);
    4
}
/// $05031A: eor.b d1, d2
pub fn i_05031a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5031C;
    cpu.ir = 0xB302;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 ^ cpu.d[1]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05031C: beq.w $50326
pub fn i_05031c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50320;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x50326; return 10; }
    8
}
/// $050320: add.w d0, $2a(a6)
pub fn i_050320(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50324;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $050324: clr.w d0
pub fn i_050324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50326;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $050326: move.w d0, (a7)
pub fn i_050326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50328;
    cpu.ir = 0x3E80;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[7], v1 as u16);
    8
}
/// $050328: movea.l $c(a7), a0
pub fn i_050328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5032C;
    cpu.ir = 0x206F;
    let ad2 = cpu.a[7].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $05032C: jsr (a0)
pub fn i_05032c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5032E;
    cpu.ir = 0x4E90;
    cpu.push32(bus, 0x5032E);
    cpu.jump_to(cpu.a[0]);
    16
}
/// $05032E: move.w sr, d7
pub fn i_05032e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50330;
    cpu.ir = 0x40C7;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | ((cpu.sr.0 as u32) & 0xFFFF);
    6
}
/// $050330: bcs.w $5033c
pub fn i_050330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50334;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5033C; return 10; }
    8
}
/// $050334: move.w (a7), d0
pub fn i_050334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50336;
    cpu.ir = 0x3017;
    let v2 = bus.read16(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $050336: and.w $2(a7), d0
pub fn i_050336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5033A;
    cpu.ir = 0xC06F;
    let ad2 = cpu.a[7].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    let r = (v3 & cpu.d[0]) & 0xFFFF;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $05033A: bne.b $502f4
pub fn i_05033a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5033C;
    cpu.ir = 0x66B8;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x502F4; return 10; }
    8
}
/// $05033C: adda.w #$4, a7
pub fn i_05033c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50340;
    cpu.ir = 0xDEFC;
    let v1: u32 = 0x4;
    cpu.a[7] = cpu.a[7].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $050340: move.w (a7)+, d0
pub fn i_050340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50342;
    cpu.ir = 0x301F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $050342: move.w d0, $2c(a6)
pub fn i_050342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50346;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $050346: move.w (a7)+, d1
pub fn i_050346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50348;
    cpu.ir = 0x321F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $050348: move.w d1, $2e(a6)
pub fn i_050348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5034C;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05034C: add.w (a7)+, d0
pub fn i_05034c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5034E;
    cpu.ir = 0xD05F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $05034E: move.w d0, $28(a6)
pub fn i_05034e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50352;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $050352: add.w (a7)+, d1
pub fn i_050352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50354;
    cpu.ir = 0xD25F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $050354: move.w d1, $2a(a6)
pub fn i_050354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50358;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $050358: adda.w #$4, a7
pub fn i_050358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5035C;
    cpu.ir = 0xDEFC;
    let v1: u32 = 0x4;
    cpu.a[7] = cpu.a[7].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05035C: move.w d7, sr
pub fn i_05035c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5035E;
    cpu.ir = 0x46C7;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let v1 = cpu.d[7] & 0xFFFF;
    write_sr(cpu, v1 as u16);
    12
}
/// $05035E: rts
pub fn i_05035e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50360;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Allen_Physics_050360  ($050360, 138 B, enemies/bosses/allen_oneil_04fa58.s)
/// $050360: tst.b $70(a6)
pub fn i_050360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50364;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $050364: beq.w $503ce
pub fn i_050364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50368;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x503CE; return 10; }
    8
}
/// $050368: move.w $24(a6), d1
pub fn i_050368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5036C;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05036C: jsr $4400e.l
pub fn i_05036c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50372;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50372);
    cpu.pc = 0x4400E;
    16
}
/// $050372: cmpi.w #$ef, d1
pub fn i_050372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50376;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xEF, Size::Word, false);
    8
}
/// $050376: blt.w $50380
pub fn i_050376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5037A;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x50380; return 10; }
    8
}
/// $05037A: move.b #$ff, $71(a6)
pub fn i_05037a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50380;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    bus.write8(ad2, v1 as u8);
    12
}
/// $050380: tst.b $71(a6)
pub fn i_050380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50384;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $050384: beq.w $50394
pub fn i_050384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50388;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x50394; return 10; }
    8
}
/// $050388: jsr $5029c(pc)
pub fn i_050388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5038C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5038C);
    cpu.pc = 0x5029C;
    16
}
/// $05038C: scc.b $70(a6)
pub fn i_05038c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50390;
    cpu.ir = 0x54EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cc) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $050390: bra.w $503ca
pub fn i_050390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50394;
    cpu.ir = 0x6000;
    cpu.pc = 0x503CA;
    10
}
/// $050394: move.w $22(a6), d1
pub fn i_050394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50398;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $050398: move.w $24(a6), d2
pub fn i_050398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5039C;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05039C: jsr $280c6.l
pub fn i_05039c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x503A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x503A2);
    cpu.pc = 0x280C6;
    16
}
/// $0503A2: bcc.w $503c0
pub fn i_0503a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x503A6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x503C0; return 10; }
    8
}
/// $0503A6: move.w $22(a6), d1
pub fn i_0503a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x503AA;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0503AA: move.w $24(a6), d2
pub fn i_0503aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x503AE;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0503AE: subq.w #$8, d2
pub fn i_0503ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x503B0;
    cpu.ir = 0x5142;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0503B0: jsr $280c6.l
pub fn i_0503b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x503B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x503B6);
    cpu.pc = 0x280C6;
    16
}
/// $0503B6: bcs.w $503c0
pub fn i_0503b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x503BA;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x503C0; return 10; }
    8
}
/// $0503BA: move.b #$ff, $71(a6)
pub fn i_0503ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x503C0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0503C0: jsr $27d50.l
pub fn i_0503c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x503C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x503C6);
    cpu.pc = 0x27D50;
    16
}
/// $0503C6: scc.b $70(a6)
pub fn i_0503c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x503CA;
    cpu.ir = 0x54EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cc) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $0503CA: bra.w $503e8
pub fn i_0503ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x503CE;
    cpu.ir = 0x6000;
    cpu.pc = 0x503E8;
    10
}
/// $0503CE: jsr $27a92.l
pub fn i_0503ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x503D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x503D4);
    cpu.pc = 0x27A92;
    16
}
/// $0503D4: jsr $27eba.l
pub fn i_0503d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x503DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x503DA);
    cpu.pc = 0x27EBA;
    16
}
/// $0503DA: bcc.w $503e8
pub fn i_0503da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x503DE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x503E8; return 10; }
    8
}
/// $0503DE: move.w #$ffc0, $2e(a6)
pub fn i_0503de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x503E4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFC0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0503E4: st.b $70(a6)
pub fn i_0503e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x503E8;
    cpu.ir = 0x50EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::True) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $0503E8: rts
pub fn i_0503e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x503EA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0503F2: move.b $7b(a6), d0
pub fn i_0503f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x503F6;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0503F6: beq.w $50400
pub fn i_0503f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x503FA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x50400; return 10; }
    8
}
/// $0503FA: subq.b #$1, d0
pub fn i_0503fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x503FC;
    cpu.ir = 0x5300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $0503FC: move.b d0, $7b(a6)
pub fn i_0503fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50400;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $050400: move.l $72(a6), d0
pub fn i_050400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50404;
    cpu.ir = 0x202E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $050404: bmi.w $5043a
pub fn i_050404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50408;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x5043A; return 10; }
    8
}
/// $050408: movea.l d0, a0
pub fn i_050408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5040A;
    cpu.ir = 0x2040;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $05040A: jsr $5e338.l
pub fn i_05040a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50410;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50410);
    cpu.pc = 0x5E338;
    16
}
/// $050410: bcc.w $50420
pub fn i_050410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50414;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x50420; return 10; }
    8
}
/// $050414: move.l #$ffffffff, $72(a6)
pub fn i_050414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5041C;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $05041C: bra.w $5043a
pub fn i_05041c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50420;
    cpu.ir = 0x6000;
    cpu.pc = 0x5043A;
    10
}
/// $050420: lea.l $503ea(pc), a0
pub fn i_050420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50424;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x503EA;
    16
}
/// $050424: jsr $5e086.l
pub fn i_050424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5042A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5042A);
    cpu.pc = 0x5E086;
    16
}
/// $05042A: scc.b $79(a6)
pub fn i_05042a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5042E;
    cpu.ir = 0x54EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cc) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x79);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $05042E: bcs.w $50436
pub fn i_05042e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50432;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x50436; return 10; }
    8
}
/// $050432: move.l a0, $72(a6)
pub fn i_050432(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50436;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $050436: bra.w $50462
pub fn i_050436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5043A;
    cpu.ir = 0x6000;
    cpu.pc = 0x50462;
    10
}
/// $05043A: lea.l $503ea(pc), a0
pub fn i_05043a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5043E;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x503EA;
    16
}
/// $05043E: jsr $5e086.l
pub fn i_05043e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50444;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50444);
    cpu.pc = 0x5E086;
    16
}
/// $050444: scc.b $79(a6)
pub fn i_050444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50448;
    cpu.ir = 0x54EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cc) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x79);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $050448: bcs.w $50454
pub fn i_050448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5044C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x50454; return 10; }
    8
}
/// $05044C: move.l a0, $72(a6)
pub fn i_05044c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50450;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $050450: bra.w $50462
pub fn i_050450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50454;
    cpu.ir = 0x6000;
    cpu.pc = 0x50462;
    10
}
/// $050454: jsr $5e0d4.l
pub fn i_050454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5045A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5045A);
    cpu.pc = 0x5E0D4;
    16
}
/// $05045A: bcs.w $50462
pub fn i_05045a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5045E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x50462; return 10; }
    8
}
/// $05045E: move.l a0, $72(a6)
pub fn i_05045e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50462;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $050462: move.l $72(a6), d0
pub fn i_050462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50466;
    cpu.ir = 0x202E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $050466: bmi.w $50488
pub fn i_050466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5046A;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x50488; return 10; }
    8
}
/// $05046A: movea.l d0, a0
pub fn i_05046a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5046C;
    cpu.ir = 0x2040;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $05046C: move.w $22(a0), d0
pub fn i_05046c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50470;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $050470: sub.w $22(a6), d0
pub fn i_050470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50474;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $050474: smi.b d0
pub fn i_050474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50476;
    cpu.ir = 0x5BC0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Mi) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $050476: btst.b #$0, $3a(a6)
pub fn i_050476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5047C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05047C: sne.b d1
pub fn i_05047c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5047E;
    cpu.ir = 0x56C1;
    let c1: u32 = if cpu.sr.evaluate(Condition::Ne) { 0xFF } else { 0 };
    let v2 = cpu.d[1] & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (c1 & 0xFF);
    4
}
/// $05047E: eor.b d0, d1
pub fn i_05047e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50480;
    cpu.ir = 0xB101;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 ^ cpu.d[0]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $050480: move.b d1, $77(a6)
pub fn i_050480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50484;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x77);
    bus.write8(ad2, v1 as u8);
    12
}
/// $050484: bra.w $5048e
pub fn i_050484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50488;
    cpu.ir = 0x6000;
    cpu.pc = 0x5048E;
    10
}
/// $050488: move.b #$ff, $77(a6)
pub fn i_050488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5048E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x77);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05048E: rts
pub fn i_05048e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50490;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Allen_CheckTargetAbove_050490  ($050490, 90 B, enemies/bosses/allen_oneil_04fa58.s)
/// $050490: move.w $22(a6), d0
pub fn i_050490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50494;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $050494: subi.w #$20, d0
pub fn i_050494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50498;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $050498: cmpi.w #$100, d0
pub fn i_050498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5049C;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x100, Size::Word, false);
    8
}
/// $05049C: bcc.w $504ea
pub fn i_05049c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x504A0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x504EA; return 10; }
    8
}
/// $0504A0: tst.b $7b(a6)
pub fn i_0504a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x504A4;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0504A4: bne.w $504ea
pub fn i_0504a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x504A8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x504EA; return 10; }
    8
}
/// $0504A8: move.l $72(a6), d0
pub fn i_0504a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x504AC;
    cpu.ir = 0x202E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0504AC: bmi.w $504ea
pub fn i_0504ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x504B0;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x504EA; return 10; }
    8
}
/// $0504B0: movea.l d0, a0
pub fn i_0504b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x504B2;
    cpu.ir = 0x2040;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0504B2: move.w $22(a0), d0
pub fn i_0504b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x504B6;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0504B6: sub.w $22(a6), d0
pub fn i_0504b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x504BA;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0504BA: smi.b d1
pub fn i_0504ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x504BC;
    cpu.ir = 0x5BC1;
    let c1: u32 = if cpu.sr.evaluate(Condition::Mi) { 0xFF } else { 0 };
    let v2 = cpu.d[1] & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (c1 & 0xFF);
    4
}
/// $0504BC: move.b $3a(a6), d2
pub fn i_0504bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x504C0;
    cpu.ir = 0x142E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0504C0: eor.b d2, d1
pub fn i_0504c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x504C2;
    cpu.ir = 0xB501;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 ^ cpu.d[2]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $0504C2: btst.b #$0, d1
pub fn i_0504c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x504C6;
    cpu.ir = 0x801;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $0504C6: beq.w $504ea
pub fn i_0504c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x504CA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x504EA; return 10; }
    8
}
/// $0504CA: tst.w d0
pub fn i_0504ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x504CC;
    cpu.ir = 0x4A40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $0504CC: bpl.w $504d2
pub fn i_0504cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x504D0;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x504D2; return 10; }
    8
}
/// $0504D0: neg.w d0
pub fn i_0504d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x504D2;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0504D2: move.w $24(a0), d1
pub fn i_0504d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x504D6;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0504D6: sub.w $24(a6), d1
pub fn i_0504d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x504DA;
    cpu.ir = 0x926E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0504DA: bmi.w $504ea
pub fn i_0504da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x504DE;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x504EA; return 10; }
    8
}
/// $0504DE: sub.w d0, d1
pub fn i_0504de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x504E0;
    cpu.ir = 0x9240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0504E0: subi.w #$10, d1
pub fn i_0504e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x504E4;
    cpu.ir = 0x441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0504E4: cmpi.w #$20, d1
pub fn i_0504e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x504E8;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x20, Size::Word, false);
    8
}
/// $0504E8: rts
pub fn i_0504e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x504EA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearC_0504ea  ($0504EA, 6 B, thunks/ccr_helpers.c)
/// $0504EA: andi.b #$fe, ccr
pub fn i_0504ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x504EE;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $0504EE: rts
pub fn i_0504ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x504F0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Allen_CheckTargetFront_0504f0  ($0504F0, 36 B, enemies/bosses/allen_oneil_04fa58.s)
/// $0504F0: move.w $22(a6), d0
pub fn i_0504f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x504F4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0504F4: subi.w #$20, d0
pub fn i_0504f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x504F8;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0504F8: cmpi.w #$100, d0
pub fn i_0504f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x504FC;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x100, Size::Word, false);
    8
}
/// $0504FC: bcc.w $5051a
pub fn i_0504fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50500;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5051A; return 10; }
    8
}
/// $050500: move.b $7b(a6), d0
pub fn i_050500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50504;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $050504: bne.w $5051a
pub fn i_050504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50508;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5051A; return 10; }
    8
}
/// $050508: move.b $77(a6), d0
pub fn i_050508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5050C;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x77);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05050C: and.b $79(a6), d0
pub fn i_05050c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50510;
    cpu.ir = 0xC02E;
    let ad2 = cpu.a[6].wrapping_add(0x79);
    let v3 = bus.read8(ad2) as u32;
    let r = (v3 & cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $050510: beq.w $5051a
pub fn i_050510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50514;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5051A; return 10; }
    8
}
// ===== SetC_050514  ($050514, 6 B, thunks/ccr_helpers.c)
/// $050514: ori.b #$1, ccr
pub fn i_050514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50518;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $050518: rts
pub fn i_050518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5051A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearC_05051a  ($05051A, 6 B, thunks/ccr_helpers.c)
/// $05051A: andi.b #$fe, ccr
pub fn i_05051a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5051E;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $05051E: rts
pub fn i_05051e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50520;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Allen_CheckScreenEdge_050520  ($050520, 32 B, enemies/bosses/allen_oneil_04fa58.s)
/// $050520: move.w $28(a6), d0
pub fn i_050520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50524;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $050524: bmi.w $50536
pub fn i_050524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50528;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x50536; return 10; }
    8
}
/// $050528: cmpi.w #$110, $22(a6)
pub fn i_050528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5052E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x110, Size::Word, false);
    16
}
/// $05052E: bpl.w $50546
pub fn i_05052e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50532;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x50546; return 10; }
    8
}
/// $050532: bra.w $50540
pub fn i_050532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50536;
    cpu.ir = 0x6000;
    cpu.pc = 0x50540;
    10
}
/// $050536: cmpi.w #$30, $22(a6)
pub fn i_050536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5053C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x30, Size::Word, false);
    16
}
/// $05053C: bmi.w $50546
pub fn i_05053c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50540;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x50546; return 10; }
    8
}
// ===== ClearC_050540  ($050540, 6 B, thunks/ccr_helpers.c)
/// $050540: andi.b #$fe, ccr
pub fn i_050540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50544;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $050544: rts
pub fn i_050544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50546;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetC_050546  ($050546, 6 B, thunks/ccr_helpers.c)
/// $050546: ori.b #$1, ccr
pub fn i_050546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5054A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $05054A: rts
pub fn i_05054a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5054C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Allen_CheckJumpRandom_05054c  ($05054C, 72 B, enemies/bosses/allen_oneil_04fa58.s)
/// $05054C: move.l $72(a6), d0
pub fn i_05054c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50550;
    cpu.ir = 0x202E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $050550: bmi.w $50594
pub fn i_050550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50554;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x50594; return 10; }
    8
}
/// $050554: move.w $24(a6), d1
pub fn i_050554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50558;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $050558: jsr $4400e.l
pub fn i_050558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5055E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5055E);
    cpu.pc = 0x4400E;
    16
}
/// $05055E: cmpi.w #$8f, d1
pub fn i_05055e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50562;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x8F, Size::Word, false);
    8
}
/// $050562: ble.w $50594
pub fn i_050562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50566;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x50594; return 10; }
    8
}
/// $050566: tst.b $7e(a6)
pub fn i_050566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5056A;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $05056A: bne.w $50594
pub fn i_05056a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5056E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x50594; return 10; }
    8
}
/// $05056E: tst.b $79(a6)
pub fn i_05056e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50572;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x79);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $050572: beq.w $50582
pub fn i_050572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50576;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x50582; return 10; }
    8
}
/// $050576: move.b $7c(a6), d0
pub fn i_050576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5057A;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05057A: or.b $7d(a6), d0
pub fn i_05057a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5057E;
    cpu.ir = 0x802E;
    let ad2 = cpu.a[6].wrapping_add(0x7D);
    let v3 = bus.read8(ad2) as u32;
    let r = (v3 | cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $05057E: bne.w $5059a
pub fn i_05057e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50582;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5059A; return 10; }
    8
}
/// $050582: jsr $5e9b6.l
pub fn i_050582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50588;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50588);
    cpu.pc = 0x5E9B6;
    16
}
/// $050588: andi.b #$f, d0
pub fn i_050588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5058C;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05058C: cmp.b $7a(a6), d0
pub fn i_05058c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50590;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $050590: bcs.w $5059a
pub fn i_050590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50594;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5059A; return 10; }
    8
}
// ===== ClearC_050594  ($050594, 6 B, thunks/ccr_helpers.c)
/// $050594: andi.b #$fe, ccr
pub fn i_050594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50598;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $050598: rts
pub fn i_050598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5059A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetC_05059a  ($05059A, 6 B, thunks/ccr_helpers.c)
/// $05059A: ori.b #$1, ccr
pub fn i_05059a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5059E;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $05059E: rts
pub fn i_05059e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x505A0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Allen_CheckTargetFar_0505a0  ($0505A0, 68 B, enemies/bosses/allen_oneil_04fa58.s)
/// $0505A0: tst.b $86(a6)
pub fn i_0505a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x505A4;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0505A4: beq.w $505ac
pub fn i_0505a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x505A8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x505AC; return 10; }
    8
}
/// $0505A8: subq.b #$1, $86(a6)
pub fn i_0505a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x505AC;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $0505AC: move.l $72(a6), d0
pub fn i_0505ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x505B0;
    cpu.ir = 0x202E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0505B0: bmi.w $505ea
pub fn i_0505b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x505B4;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x505EA; return 10; }
    8
}
/// $0505B4: tst.b $77(a6)
pub fn i_0505b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x505B8;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x77);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0505B8: beq.w $505ea
pub fn i_0505b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x505BC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x505EA; return 10; }
    8
}
/// $0505BC: movea.l d0, a0
pub fn i_0505bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x505BE;
    cpu.ir = 0x2040;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0505BE: move.w $24(a6), d0
pub fn i_0505be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x505C2;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0505C2: sub.w $24(a0), d0
pub fn i_0505c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x505C6;
    cpu.ir = 0x9068;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0505C6: beq.w $505ea
pub fn i_0505c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x505CA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x505EA; return 10; }
    8
}
/// $0505CA: bmi.w $505ea
pub fn i_0505ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x505CE;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x505EA; return 10; }
    8
}
/// $0505CE: move.w $22(a6), d1
pub fn i_0505ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x505D2;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0505D2: sub.w $22(a0), d1
pub fn i_0505d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x505D6;
    cpu.ir = 0x9268;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0505D6: bpl.w $505dc
pub fn i_0505d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x505DA;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x505DC; return 10; }
    8
}
/// $0505DA: neg.w d1
pub fn i_0505da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x505DC;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0505DC: add.w d1, d1
pub fn i_0505dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x505DE;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0505DE: cmp.w d0, d1
pub fn i_0505de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x505E0;
    cpu.ir = 0xB240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $0505E0: bcs.w $505ea
pub fn i_0505e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x505E4;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x505EA; return 10; }
    8
}
// ===== SetC_0505e4  ($0505E4, 6 B, thunks/ccr_helpers.c)
/// $0505E4: ori.b #$1, ccr
pub fn i_0505e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x505E8;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $0505E8: rts
pub fn i_0505e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x505EA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearC_0505ea  ($0505EA, 6 B, thunks/ccr_helpers.c)
/// $0505EA: andi.b #$fe, ccr
pub fn i_0505ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x505EE;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $0505EE: rts
pub fn i_0505ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x505F0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Allen_KnifeHitCheck_0505f0  ($0505F0, 66 B, enemies/bosses/allen_oneil_04fa58.s)
/// $0505F0: tst.b $45(a6)
pub fn i_0505f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x505F4;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0505F4: bne.w $50638
pub fn i_0505f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x505F8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x50638; return 10; }
    8
}
/// $0505F8: move.w $22(a6), d0
pub fn i_0505f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x505FC;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0505FC: cmpi.w #$20, d0
pub fn i_0505fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50600;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x20, Size::Word, false);
    8
}
/// $050600: bmi.w $50638
pub fn i_050600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50604;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x50638; return 10; }
    8
}
/// $050604: cmpi.w #$300, d0
pub fn i_050604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50608;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x300, Size::Word, false);
    8
}
/// $050608: bpl.w $50638
pub fn i_050608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5060C;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x50638; return 10; }
    8
}
/// $05060C: lea.l $296dca.l, a0
pub fn i_05060c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50612;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x296DCA;
    20
}
/// $050612: move.l a0, $4c(a6)
pub fn i_050612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50616;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $050616: jsr $283ca.l
pub fn i_050616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5061C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5061C);
    cpu.pc = 0x283CA;
    16
}
/// $05061C: jsr $283ca.l
pub fn i_05061c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50622;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50622);
    cpu.pc = 0x283CA;
    16
}
/// $050622: jsr $283d8.l
pub fn i_050622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50628;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50628);
    cpu.pc = 0x283D8;
    16
}
/// $050628: btst.b #$1, $13(a6)
pub fn i_050628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5062E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05062E: beq.w $50638
pub fn i_05062e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50632;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x50638; return 10; }
    8
}
// ===== SetC_050632  ($050632, 6 B, thunks/ccr_helpers.c)
/// $050632: ori.b #$1, ccr
pub fn i_050632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50636;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $050636: rts
pub fn i_050636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50638;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearC_050638  ($050638, 6 B, thunks/ccr_helpers.c)
/// $050638: andi.b #$fe, ccr
pub fn i_050638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5063C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $05063C: rts
pub fn i_05063c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5063E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Allen_ThrowGrenade_05063e  ($05063E, 100 B, enemies/bosses/allen_oneil_04fa58.s)
/// $05063E: move.l $72(a6), d0
pub fn i_05063e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50642;
    cpu.ir = 0x202E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $050642: bmi.w $506a0
pub fn i_050642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50646;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x506A0; return 10; }
    8
}
/// $050646: move.l a6, -(a7)
pub fn i_050646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50648;
    cpu.ir = 0x2F0E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $050648: lea.l $100800.l, a6
pub fn i_050648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5064E;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0x100800;
    20
}
/// $05064E: lea.l $512b4(pc), a1
pub fn i_05064e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50652;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x512B4;
    16
}
/// $050652: jsr $4ae.l
pub fn i_050652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50658;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50658);
    cpu.pc = 0x4AE;
    16
}
/// $050658: movea.l (a7)+, a6
pub fn i_050658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5065A;
    cpu.ir = 0x2C5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    12
}
/// $05065A: move.w $22(a6), $22(a0)
pub fn i_05065a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50660;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $050660: move.w $24(a6), d0
pub fn i_050660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50664;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $050664: addi.w #$10, d0
pub fn i_050664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50668;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $050668: move.w d0, $24(a0)
pub fn i_050668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5066C;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05066C: move.l a0, -(a7)
pub fn i_05066c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5066E;
    cpu.ir = 0x2F08;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $05066E: jsr $5e9b6.l
pub fn i_05066e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50674;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50674);
    cpu.pc = 0x5E9B6;
    16
}
/// $050674: andi.w #$1f, d0
pub fn i_050674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50678;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $050678: subi.w #$10, d0
pub fn i_050678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5067C;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05067C: movea.l (a7)+, a0
pub fn i_05067c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5067E;
    cpu.ir = 0x205F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    12
}
/// $05067E: movea.l $72(a6), a1
pub fn i_05067e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50682;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $050682: add.w $22(a1), d0
pub fn i_050682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50686;
    cpu.ir = 0xD069;
    let ad2 = cpu.a[1].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $050686: sub.w $22(a0), d0
pub fn i_050686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5068A;
    cpu.ir = 0x9068;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $05068A: move.w $24(a1), d1
pub fn i_05068a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5068E;
    cpu.ir = 0x3229;
    let ad2 = cpu.a[1].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05068E: sub.w $24(a0), d1
pub fn i_05068e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50692;
    cpu.ir = 0x9268;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $050692: move.l a0, -(a7)
pub fn i_050692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50694;
    cpu.ir = 0x2F08;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $050694: jsr $5e018.l
pub fn i_050694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5069A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5069A);
    cpu.pc = 0x5E018;
    16
}
/// $05069A: movea.l (a7)+, a0
pub fn i_05069a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5069C;
    cpu.ir = 0x205F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    12
}
/// $05069C: move.w d0, $34(a0)
pub fn i_05069c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x506A0;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0506A0: rts
pub fn i_0506a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x506A2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Allen_CheckJumpHigh_0506a2  ($0506A2, 72 B, enemies/bosses/allen_oneil_04fa58.s)
/// $0506A2: move.l $72(a6), d0
pub fn i_0506a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x506A6;
    cpu.ir = 0x202E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0506A6: bmi.w $50594
pub fn i_0506a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x506AA;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x50594; return 10; }
    8
}
/// $0506AA: move.w $24(a6), d1
pub fn i_0506aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x506AE;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0506AE: jsr $4400e.l
pub fn i_0506ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x506B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x506B4);
    cpu.pc = 0x4400E;
    16
}
/// $0506B4: cmpi.w #$ef, d1
pub fn i_0506b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x506B8;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xEF, Size::Word, false);
    8
}
/// $0506B8: bge.w $506ea
pub fn i_0506b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x506BC;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x506EA; return 10; }
    8
}
/// $0506BC: tst.b $7f(a6)
pub fn i_0506bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x506C0;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x7F);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0506C0: bne.w $506ea
pub fn i_0506c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x506C4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x506EA; return 10; }
    8
}
/// $0506C4: tst.b $79(a6)
pub fn i_0506c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x506C8;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x79);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0506C8: beq.w $506d8
pub fn i_0506c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x506CC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x506D8; return 10; }
    8
}
/// $0506CC: move.b $7c(a6), d0
pub fn i_0506cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x506D0;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0506D0: or.b $7d(a6), d0
pub fn i_0506d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x506D4;
    cpu.ir = 0x802E;
    let ad2 = cpu.a[6].wrapping_add(0x7D);
    let v3 = bus.read8(ad2) as u32;
    let r = (v3 | cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0506D4: bne.w $506f0
pub fn i_0506d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x506D8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x506F0; return 10; }
    8
}
/// $0506D8: jsr $5e9b6.l
pub fn i_0506d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x506DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x506DE);
    cpu.pc = 0x5E9B6;
    16
}
/// $0506DE: andi.b #$f, d0
pub fn i_0506de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x506E2;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0506E2: cmp.b $7a(a6), d0
pub fn i_0506e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x506E6;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $0506E6: bcs.w $506f0
pub fn i_0506e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x506EA;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x506F0; return 10; }
    8
}
// ===== ClearC_0506ea  ($0506EA, 6 B, thunks/ccr_helpers.c)
/// $0506EA: andi.b #$fe, ccr
pub fn i_0506ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x506EE;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $0506EE: rts
pub fn i_0506ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x506F0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetC_0506f0  ($0506F0, 6 B, thunks/ccr_helpers.c)
/// $0506F0: ori.b #$1, ccr
pub fn i_0506f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x506F4;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $0506F4: rts
pub fn i_0506f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x506F6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Allen_CheckTouchKnife_0506f6  ($0506F6, 36 B, enemies/bosses/allen_oneil_04fa58.s)
/// $0506F6: tst.b $45(a6)
pub fn i_0506f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x506FA;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0506FA: bne.w $5071a
pub fn i_0506fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x506FE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5071A; return 10; }
    8
}
/// $0506FE: tst.b $78(a6)
pub fn i_0506fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50702;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $050702: beq.w $50712
pub fn i_050702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50706;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x50712; return 10; }
    8
}
/// $050706: tst.b $7c(a6)
pub fn i_050706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5070A;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $05070A: bne.w $50720
pub fn i_05070a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5070E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x50720; return 10; }
    8
}
/// $05070E: bra.w $5071a
pub fn i_05070e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50712;
    cpu.ir = 0x6000;
    cpu.pc = 0x5071A;
    10
}
/// $050712: tst.b $7d(a6)
pub fn i_050712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50716;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x7D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $050716: bne.w $50720
pub fn i_050716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5071A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x50720; return 10; }
    8
}
// ===== ClearC_05071a  ($05071A, 6 B, thunks/ccr_helpers.c)
/// $05071A: andi.b #$fe, ccr
pub fn i_05071a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5071E;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $05071E: rts
pub fn i_05071e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50720;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetC_050720  ($050720, 6 B, thunks/ccr_helpers.c)
/// $050720: ori.b #$1, ccr
pub fn i_050720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50724;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $050724: rts
pub fn i_050724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50726;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05072E: tst.b $87(a6)
pub fn i_05072e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50732;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x87);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $050732: beq.w $5073a
pub fn i_050732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50736;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5073A; return 10; }
    8
}
/// $050736: subq.b #$1, $87(a6)
pub fn i_050736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5073A;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x87);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $05073A: jsr $2870a.l
pub fn i_05073a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50740;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50740);
    cpu.pc = 0x2870A;
    16
}
/// $050740: bcc.w $507de
pub fn i_050740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50744;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x507DE; return 10; }
    8
}
/// $050744: bclr.b #$3, $13(a6)
pub fn i_050744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5074A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05074A: tst.b $87(a6)
pub fn i_05074a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5074E;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x87);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $05074E: bne.w $5075e
pub fn i_05074e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50752;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5075E; return 10; }
    8
}
/// $050752: jsr $4aae0.l
pub fn i_050752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50758;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50758);
    cpu.pc = 0x4AAE0;
    16
}
/// $050758: move.b #$c, $87(a6)
pub fn i_050758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5075E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x87);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05075E: move.w $66(a6), d0
pub fn i_05075e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50762;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $050762: move.w $84(a6), d1
pub fn i_050762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50766;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $050766: subi.w #$32, d1
pub fn i_050766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5076A;
    cpu.ir = 0x441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x32, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05076A: cmp.w d1, d0
pub fn i_05076a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5076C;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05076C: bge.w $50776
pub fn i_05076c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50770;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x50776; return 10; }
    8
}
/// $050770: move.w d1, d0
pub fn i_050770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50772;
    cpu.ir = 0x3001;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $050772: move.w d1, $66(a6)
pub fn i_050772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50776;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $050776: cmp.w $80(a6), d0
pub fn i_050776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5077A;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $05077A: bcc.w $507c2
pub fn i_05077a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5077E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x507C2; return 10; }
    8
}
/// $05077E: move.w $80(a6), d0
pub fn i_05077e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50782;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $050782: sub.w $82(a6), d0
pub fn i_050782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50786;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $050786: move.w d0, $80(a6)
pub fn i_050786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5078A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05078A: move.b $76(a6), d0
pub fn i_05078a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5078E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05078E: andi.w #$ff, d0
pub fn i_05078e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50792;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $050792: add.w d0, d0
pub fn i_050792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50794;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $050794: lea.l $296e72.l, a0
pub fn i_050794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5079A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x296E72;
    20
}
/// $05079A: move.w (a0, d0.w), d2
pub fn i_05079a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5079E;
    cpu.ir = 0x3430;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $05079E: move.w $14(a6), d1
pub fn i_05079e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x507A2;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0507A2: move.w #$ffff, d3
pub fn i_0507a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x507A6;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0507A6: move.w #$ffff, d4
pub fn i_0507a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x507AA;
    cpu.ir = 0x383C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0507AA: jsr $2c30.l
pub fn i_0507aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x507B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x507B0);
    cpu.pc = 0x2C30;
    16
}
/// $0507B0: move.b $76(a6), d0
pub fn i_0507b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x507B4;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0507B4: cmpi.b #$4, d0
pub fn i_0507b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x507B8;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x4, Size::Byte, false);
    8
}
/// $0507B8: bcc.w $507c2
pub fn i_0507b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x507BC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x507C2; return 10; }
    8
}
/// $0507BC: addq.b #$1, d0
pub fn i_0507bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x507BE;
    cpu.ir = 0x5200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $0507BE: move.b d0, $76(a6)
pub fn i_0507be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x507C2;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0507C2: cmpi.w #$0, $66(a6)
pub fn i_0507c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x507C8;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0507C8: ble.w $507d2
pub fn i_0507c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x507CC;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x507D2; return 10; }
    8
}
/// $0507CC: bclr.b #$0, $13(a6)
pub fn i_0507cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x507D2;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0507D2: lea.l $5e766.l, a0
pub fn i_0507d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x507D8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $0507D8: jsr $5e770.l
pub fn i_0507d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x507DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x507DE);
    cpu.pc = 0x5E770;
    16
}
/// $0507DE: jsr $28758.l
pub fn i_0507de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x507E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x507E4);
    cpu.pc = 0x28758;
    16
}
/// $0507E4: bcc.w $507ee
pub fn i_0507e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x507E8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x507EE; return 10; }
    8
}
/// $0507E8: lea.l $50e40(pc), a1
pub fn i_0507e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x507EC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x50E40;
    16
}
/// $0507EC: move.l a1, (a6)
pub fn i_0507ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x507EE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0507EE: move.w $66(a6), $84(a6)
pub fn i_0507ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x507F4;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x84);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0507F4: lea.l $50726(pc), a0
pub fn i_0507f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x507F8;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x50726;
    16
}
/// $0507F8: jsr $5dd56.l
pub fn i_0507f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x507FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x507FE);
    cpu.pc = 0x5DD56;
    16
}
/// $0507FE: bcc.w $50808
pub fn i_0507fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50802;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x50808; return 10; }
    8
}
/// $050802: jmp $518.l
pub fn i_050802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50808;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $050808: rts
pub fn i_050808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5080A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Allen_SpawnMuzzleFlashA_05080a  ($05080A, 4 B, enemies/bosses/allen_oneil_04fa58.s)
/// $05080A: lea.l $5100e(pc), a1
pub fn i_05080a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5080E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5100E;
    16
}
// ===== JsrAbsThunk_05080e  ($05080E, 8 B, thunks/jsr_abs_thunks.c)
/// $05080E: jsr $4ae.l
pub fn i_05080e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50814;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50814);
    cpu.pc = 0x4AE;
    16
}
/// $050814: rts
pub fn i_050814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50816;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Allen_SpawnMuzzleFlashB_050816  ($050816, 4 B, enemies/bosses/allen_oneil_04fa58.s)
/// $050816: lea.l $5101e(pc), a1
pub fn i_050816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5081A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5101E;
    16
}
// ===== JsrAbsThunk_05081a  ($05081A, 8 B, thunks/jsr_abs_thunks.c)
/// $05081A: jsr $4ae.l
pub fn i_05081a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50820;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50820);
    cpu.pc = 0x4AE;
    16
}
/// $050820: rts
pub fn i_050820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50822;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Allen_SpawnBulletLevel_050822  ($050822, 90 B, enemies/bosses/allen_oneil_04fa58.s)
/// $050822: move.w #$10f4, d0
pub fn i_050822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50826;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10F4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $050826: jsr $2352.l
pub fn i_050826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5082C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5082C);
    cpu.pc = 0x2352;
    16
}
/// $05082C: move.l a6, -(a7)
pub fn i_05082c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5082E;
    cpu.ir = 0x2F0E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $05082E: lea.l $100800.l, a6
pub fn i_05082e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50834;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0x100800;
    20
}
/// $050834: lea.l $51530(pc), a1
pub fn i_050834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50838;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x51530;
    16
}
/// $050838: jsr $4ae.l
pub fn i_050838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5083E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5083E);
    cpu.pc = 0x4AE;
    16
}
/// $05083E: movea.l (a7)+, a6
pub fn i_05083e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50840;
    cpu.ir = 0x2C5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    12
}
/// $050840: move.b $3a(a6), $3a(a0)
pub fn i_050840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50846;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x3A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $050846: move.w #$38, d0
pub fn i_050846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5084A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x38;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05084A: btst.b #$0, $3a(a6)
pub fn i_05084a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50850;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $050850: bne.w $50860
pub fn i_050850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50854;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x50860; return 10; }
    8
}
/// $050854: neg.w d0
pub fn i_050854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50856;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $050856: move.w #$80, $34(a0)
pub fn i_050856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5085C;
    cpu.ir = 0x317C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05085C: bra.w $50866
pub fn i_05085c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50860;
    cpu.ir = 0x6000;
    cpu.pc = 0x50866;
    10
}
/// $050860: move.w #$0, $34(a0)
pub fn i_050860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50866;
    cpu.ir = 0x317C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $050866: add.w $22(a6), d0
pub fn i_050866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5086A;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $05086A: move.w d0, $22(a0)
pub fn i_05086a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5086E;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05086E: move.w $24(a6), d0
pub fn i_05086e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50872;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $050872: addi.w #$18, d0
pub fn i_050872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50876;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x18, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $050876: move.w d0, $24(a0)
pub fn i_050876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5087A;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05087A: rts
pub fn i_05087a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5087C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Allen_SpawnBulletDiag_05087c  ($05087C, 90 B, enemies/bosses/allen_oneil_04fa58.s)
/// $05087C: move.w #$10f4, d0
pub fn i_05087c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50880;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10F4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $050880: jsr $2352.l
pub fn i_050880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50886;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50886);
    cpu.pc = 0x2352;
    16
}
/// $050886: move.l a6, -(a7)
pub fn i_050886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50888;
    cpu.ir = 0x2F0E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $050888: lea.l $100800.l, a6
pub fn i_050888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5088E;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0x100800;
    20
}
/// $05088E: lea.l $51530(pc), a1
pub fn i_05088e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50892;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x51530;
    16
}
/// $050892: jsr $4ae.l
pub fn i_050892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50898;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50898);
    cpu.pc = 0x4AE;
    16
}
/// $050898: movea.l (a7)+, a6
pub fn i_050898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5089A;
    cpu.ir = 0x2C5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    12
}
/// $05089A: move.b $3a(a6), $3a(a0)
pub fn i_05089a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x508A0;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x3A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $0508A0: move.w #$28, d0
pub fn i_0508a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x508A4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0508A4: btst.b #$0, $3a(a6)
pub fn i_0508a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x508AA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0508AA: bne.w $508ba
pub fn i_0508aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x508AE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x508BA; return 10; }
    8
}
/// $0508AE: neg.w d0
pub fn i_0508ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x508B0;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0508B0: move.w #$60, $34(a0)
pub fn i_0508b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x508B6;
    cpu.ir = 0x317C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0508B6: bra.w $508c0
pub fn i_0508b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x508BA;
    cpu.ir = 0x6000;
    cpu.pc = 0x508C0;
    10
}
/// $0508BA: move.w #$20, $34(a0)
pub fn i_0508ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x508C0;
    cpu.ir = 0x317C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0508C0: add.w $22(a6), d0
pub fn i_0508c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x508C4;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0508C4: move.w d0, $22(a0)
pub fn i_0508c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x508C8;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0508C8: move.w $24(a6), d0
pub fn i_0508c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x508CC;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0508CC: addi.w #$40, d0
pub fn i_0508cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x508D0;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x40, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0508D0: move.w d0, $24(a0)
pub fn i_0508d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x508D4;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0508D4: rts
pub fn i_0508d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x508D6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Allen_Init_0508d6  ($0508D6, 152 B, enemies/bosses/allen_oneil_04fa58.s)
/// $0508D6: lea.l $296e7a.l, a0
pub fn i_0508d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x508DC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x296E7A;
    20
}
/// $0508DC: jsr $799de.l
pub fn i_0508dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x508E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x508E2);
    cpu.pc = 0x799DE;
    16
}
/// $0508E2: move.w d0, $66(a6)
pub fn i_0508e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x508E6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0508E6: move.w d0, $84(a6)
pub fn i_0508e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x508EA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0508EA: move.w d0, d1
pub fn i_0508ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x508EC;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0508EC: tst.w d0
pub fn i_0508ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x508EE;
    cpu.ir = 0x4A40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $0508EE: bne.w $508fa
pub fn i_0508ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x508F2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x508FA; return 10; }
    8
}
/// $0508F2: move.w d6, d6
pub fn i_0508f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x508F4;
    cpu.ir = 0x3C06;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0508F4: move.w d6, d6
pub fn i_0508f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x508F6;
    cpu.ir = 0x3C06;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0508F6: move.w d6, d6
pub fn i_0508f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x508F8;
    cpu.ir = 0x3C06;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0508F8: trap #$f
pub fn i_0508f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x508FA;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $0508FA: andi.l #$ffff, d0
pub fn i_0508fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50900;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $050900: divu.w #$5, d0
pub fn i_050900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50904;
    cpu.ir = 0x80FC;
    let v1: u32 = 0x5;
    cpu.sr.set(StatusRegister::C, false);
    let dv = v1 & 0xFFFF;
    if dv == 0 { cpu.enter_group2_exception(bus, Exception::DivideByZero); return 38; }
    let q = cpu.d[0] / dv; let rm = cpu.d[0] % dv;
    if q > 0xFFFF { cpu.sr.set(StatusRegister::V, true); cpu.sr.set(StatusRegister::N, true); cpu.sr.set(StatusRegister::Z, false); return 76; }
    cpu.d[0] = (rm << 16) | (q & 0xFFFF);
    cpu.sr.set_nz(q, Size::Word); cpu.sr.set(StatusRegister::V, false);
    140
}
/// $050904: move.w d0, $82(a6)
pub fn i_050904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50908;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $050908: sub.w d0, d1
pub fn i_050908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5090A;
    cpu.ir = 0x9240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05090A: move.w d1, $80(a6)
pub fn i_05090a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5090E;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05090E: move.w #$18e, d1
pub fn i_05090e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50912;
    cpu.ir = 0x323C;
    let v1: u32 = 0x18E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $050912: jsr $29f2.l
pub fn i_050912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50918;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50918);
    cpu.pc = 0x29F2;
    16
}
/// $050918: clr.b $76(a6)
pub fn i_050918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5091C;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $05091C: lea.l $50fa6(pc), a1
pub fn i_05091c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50920;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x50FA6;
    16
}
/// $050920: jsr $4ae.l
pub fn i_050920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50926;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50926);
    cpu.pc = 0x4AE;
    16
}
/// $050926: lea.l $50fba(pc), a1
pub fn i_050926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5092A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x50FBA;
    16
}
/// $05092A: jsr $4ae.l
pub fn i_05092a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50930;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50930);
    cpu.pc = 0x4AE;
    16
}
/// $050930: lea.l $50f7e(pc), a1
pub fn i_050930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50934;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x50F7E;
    16
}
/// $050934: jsr $4ae.l
pub fn i_050934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5093A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5093A);
    cpu.pc = 0x4AE;
    16
}
/// $05093A: lea.l $50f92(pc), a1
pub fn i_05093a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5093E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x50F92;
    16
}
/// $05093E: jsr $4ae.l
pub fn i_05093e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50944;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50944);
    cpu.pc = 0x4AE;
    16
}
/// $050944: clr.b $7b(a6)
pub fn i_050944(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50948;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x7B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $050948: clr.b $7a(a6)
pub fn i_050948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5094C;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x7A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $05094C: clr.b $86(a6)
pub fn i_05094c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50950;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $050950: clr.b $87(a6)
pub fn i_050950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50954;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x87);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $050954: move.w $22(a6), d0
pub fn i_050954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50958;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $050958: cmpi.w #$a0, d0
pub fn i_050958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5095C;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA0, Size::Word, false);
    8
}
/// $05095C: bpl.w $5096a
pub fn i_05095c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50960;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x5096A; return 10; }
    8
}
/// $050960: bset.b #$0, $3a(a6)
pub fn i_050960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50966;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $050966: clr.b $78(a6)
pub fn i_050966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5096A;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $05096A: bsr.w $503f2
pub fn i_05096a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5096E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5096E);
    cpu.pc = 0x503F2;
    18
}
// ===== SetTaskHandler_05096e  ($05096E, 8 B, kernel/task_handlers.c)
/// $05096E: lea.l $50976(pc), a1
pub fn i_05096e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50972;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x50976;
    16
}
/// $050972: move.l a1, (a6)
pub fn i_050972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50974;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $050974: rts
pub fn i_050974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50976;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Allen_Spawn_050976  ($050976, 96 B, enemies/bosses/allen_oneil_04fa58.s)
/// $050976: lea.l $ffff.w, a0
pub fn i_050976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5097A;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $05097A: move.l a0, $48(a6)
pub fn i_05097a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5097E;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $05097E: lea.l $296894.l, a0
pub fn i_05097e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50984;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x296894;
    20
}
/// $050984: jsr $28cd4.l
pub fn i_050984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5098A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5098A);
    cpu.pc = 0x28CD4;
    16
}
/// $05098A: tst.b $78(a6)
pub fn i_05098a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5098E;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $05098E: beq.w $5099c
pub fn i_05098e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50992;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5099C; return 10; }
    8
}
/// $050992: move.w #$fa00, $28(a6)
pub fn i_050992(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50998;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFA00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $050998: bra.w $509a2
pub fn i_050998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5099C;
    cpu.ir = 0x6000;
    cpu.pc = 0x509A2;
    10
}
/// $05099C: move.w #$600, $28(a6)
pub fn i_05099c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x509A2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x600;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0509A2: lea.l $509a8(pc), a1
pub fn i_0509a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x509A6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x509A8;
    16
}
/// $0509A6: move.l a1, (a6)
pub fn i_0509a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x509A8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0509A8: move.b #$1e, $45(a6)
pub fn i_0509a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x509AE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0509AE: bsr.w $503f2
pub fn i_0509ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x509B2;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x509B2);
    cpu.pc = 0x503F2;
    18
}
/// $0509B2: bsr.w $50360
pub fn i_0509b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x509B6;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x509B6);
    cpu.pc = 0x50360;
    18
}
/// $0509B6: jsr $28d70.l
pub fn i_0509b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x509BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x509BC);
    cpu.pc = 0x28D70;
    16
}
/// $0509BC: move.w $22(a6), d0
pub fn i_0509bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x509C0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0509C0: subi.w #$30, d0
pub fn i_0509c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x509C4;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x30, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0509C4: cmpi.w #$e0, d0
pub fn i_0509c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x509C8;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xE0, Size::Word, false);
    8
}
/// $0509C8: bcc.w $509d2
pub fn i_0509c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x509CC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x509D2; return 10; }
    8
}
/// $0509CC: lea.l $509de(pc), a1
pub fn i_0509cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x509D0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x509DE;
    16
}
/// $0509D0: move.l a1, (a6)
pub fn i_0509d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x509D2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0509D2: bra.w $5072e
pub fn i_0509d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x509D6;
    cpu.ir = 0x6000;
    cpu.pc = 0x5072E;
    10
}
// ===== Allen_Intro_0509de  ($0509DE, 76 B, enemies/bosses/allen_oneil_04fa58.s)
/// $0509DE: lea.l $296bd8.l, a0
pub fn i_0509de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x509E4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x296BD8;
    20
}
/// $0509E4: jsr $28cd4.l
pub fn i_0509e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x509EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x509EA);
    cpu.pc = 0x28CD4;
    16
}
/// $0509EA: move.w #$5a, $30(a6)
pub fn i_0509ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x509F0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x5A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0509F0: clr.w $28(a6)
pub fn i_0509f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x509F4;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0509F4: lea.l $509fa(pc), a1
pub fn i_0509f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x509F8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x509FA;
    16
}
/// $0509F8: move.l a1, (a6)
pub fn i_0509f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x509FA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0509FA: move.b #$1e, $45(a6)
pub fn i_0509fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50A00;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $050A00: bsr.w $503f2
pub fn i_050a00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50A04;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x50A04);
    cpu.pc = 0x503F2;
    18
}
/// $050A04: bsr.w $50360
pub fn i_050a04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50A08;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x50A08);
    cpu.pc = 0x50360;
    18
}
/// $050A08: jsr $28d70.l
pub fn i_050a08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50A0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50A0E);
    cpu.pc = 0x28D70;
    16
}
/// $050A0E: subq.w #$1, $30(a6)
pub fn i_050a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50A12;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x30);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $050A12: bne.w $50a26
pub fn i_050a12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50A16;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x50A26; return 10; }
    8
}
/// $050A16: lea.l $2963c4.l, a0
pub fn i_050a16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50A1C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2963C4;
    20
}
/// $050A1C: move.l a0, $48(a6)
pub fn i_050a1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50A20;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $050A20: lea.l $50a2a(pc), a1
pub fn i_050a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50A24;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x50A2A;
    16
}
/// $050A24: move.l a1, (a6)
pub fn i_050a24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50A26;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $050A26: bra.w $5072e
pub fn i_050a26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50A2A;
    cpu.ir = 0x6000;
    cpu.pc = 0x5072E;
    10
}
// ===== Allen_Main_050a2a  ($050A2A, 248 B, enemies/bosses/allen_oneil_04fa58.s)
/// $050A2A: tst.b $78(a6)
pub fn i_050a2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50A2E;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $050A2E: beq.w $50a3c
pub fn i_050a2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50A32;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x50A3C; return 10; }
    8
}
/// $050A32: move.w #$fa00, $28(a6)
pub fn i_050a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50A38;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFA00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $050A38: bra.w $50a42
pub fn i_050a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50A3C;
    cpu.ir = 0x6000;
    cpu.pc = 0x50A42;
    10
}
/// $050A3C: move.w #$600, $28(a6)
pub fn i_050a3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50A42;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x600;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $050A42: move.b $78(a6), d0
pub fn i_050a42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50A46;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $050A46: move.b $3a(a6), d1
pub fn i_050a46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50A4A;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $050A4A: eor.b d1, d0
pub fn i_050a4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50A4C;
    cpu.ir = 0xB300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 ^ cpu.d[1]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $050A4C: btst.b #$0, d0
pub fn i_050a4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50A50;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $050A50: beq.w $50a64
pub fn i_050a50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50A54;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x50A64; return 10; }
    8
}
/// $050A54: lea.l $296894.l, a0
pub fn i_050a54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50A5A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x296894;
    20
}
/// $050A5A: jsr $28cd4.l
pub fn i_050a5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50A60;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50A60);
    cpu.pc = 0x28CD4;
    16
}
/// $050A60: bra.w $50a70
pub fn i_050a60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50A64;
    cpu.ir = 0x6000;
    cpu.pc = 0x50A70;
    10
}
/// $050A64: lea.l $296908.l, a0
pub fn i_050a64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50A6A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x296908;
    20
}
/// $050A6A: jsr $28cd4.l
pub fn i_050a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50A70;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50A70);
    cpu.pc = 0x28CD4;
    16
}
/// $050A70: lea.l $50a76(pc), a1
pub fn i_050a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50A74;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x50A76;
    16
}
/// $050A74: move.l a1, (a6)
pub fn i_050a74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50A76;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $050A76: bsr.w $503f2
pub fn i_050a76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50A7A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x50A7A);
    cpu.pc = 0x503F2;
    18
}
/// $050A7A: bsr.w $50360
pub fn i_050a7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50A7E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x50A7E);
    cpu.pc = 0x50360;
    18
}
/// $050A7E: jsr $28d70.l
pub fn i_050a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50A84;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50A84);
    cpu.pc = 0x28D70;
    16
}
/// $050A84: move.l $72(a6), d0
pub fn i_050a84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50A88;
    cpu.ir = 0x202E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $050A88: bpl.w $50a92
pub fn i_050a88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50A8C;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x50A92; return 10; }
    8
}
/// $050A8C: lea.l $50c18(pc), a1
pub fn i_050a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50A90;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x50C18;
    16
}
/// $050A90: move.l a1, (a6)
pub fn i_050a90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50A92;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $050A92: tst.b $77(a6)
pub fn i_050a92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50A96;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x77);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $050A96: bne.w $50aa0
pub fn i_050a96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50A9A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x50AA0; return 10; }
    8
}
/// $050A9A: lea.l $50b22(pc), a1
pub fn i_050a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50A9E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x50B22;
    16
}
/// $050A9E: move.l a1, (a6)
pub fn i_050a9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50AA0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $050AA0: jsr $50490(pc)
pub fn i_050aa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50AA4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x50AA4);
    cpu.pc = 0x50490;
    16
}
/// $050AA4: bcc.w $50aae
pub fn i_050aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50AA8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x50AAE; return 10; }
    8
}
/// $050AA8: lea.l $50bd2(pc), a1
pub fn i_050aa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50AAC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x50BD2;
    16
}
/// $050AAC: move.l a1, (a6)
pub fn i_050aac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50AAE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $050AAE: jsr $504f0(pc)
pub fn i_050aae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50AB2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x50AB2);
    cpu.pc = 0x504F0;
    16
}
/// $050AB2: bcc.w $50abc
pub fn i_050ab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50AB6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x50ABC; return 10; }
    8
}
/// $050AB6: lea.l $50bc2(pc), a1
pub fn i_050ab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50ABA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x50BC2;
    16
}
/// $050ABA: move.l a1, (a6)
pub fn i_050aba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50ABC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $050ABC: jsr $505f0(pc)
pub fn i_050abc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50AC0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x50AC0);
    cpu.pc = 0x505F0;
    16
}
/// $050AC0: bcc.w $50aca
pub fn i_050ac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50AC4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x50ACA; return 10; }
    8
}
/// $050AC4: lea.l $50dfc(pc), a1
pub fn i_050ac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50AC8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x50DFC;
    16
}
/// $050AC8: move.l a1, (a6)
pub fn i_050ac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50ACA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $050ACA: jsr $505a0(pc)
pub fn i_050aca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50ACE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x50ACE);
    cpu.pc = 0x505A0;
    16
}
/// $050ACE: bcc.w $50ad8
pub fn i_050ace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50AD2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x50AD8; return 10; }
    8
}
/// $050AD2: lea.l $50b6a(pc), a1
pub fn i_050ad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50AD6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x50B6A;
    16
}
/// $050AD6: move.l a1, (a6)
pub fn i_050ad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50AD8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $050AD8: jsr $50520(pc)
pub fn i_050ad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50ADC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x50ADC);
    cpu.pc = 0x50520;
    16
}
/// $050ADC: bcc.w $50ae6
pub fn i_050adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50AE0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x50AE6; return 10; }
    8
}
/// $050AE0: lea.l $50b5e(pc), a1
pub fn i_050ae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50AE4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x50B5E;
    16
}
/// $050AE4: move.l a1, (a6)
pub fn i_050ae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50AE6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $050AE6: jsr $5054c(pc)
pub fn i_050ae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50AEA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x50AEA);
    cpu.pc = 0x5054C;
    16
}
/// $050AEA: bcc.w $50af4
pub fn i_050aea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50AEE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x50AF4; return 10; }
    8
}
/// $050AEE: lea.l $50d1a(pc), a1
pub fn i_050aee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50AF2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x50D1A;
    16
}
/// $050AF2: move.l a1, (a6)
pub fn i_050af2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50AF4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $050AF4: jsr $506a2(pc)
pub fn i_050af4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50AF8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x50AF8);
    cpu.pc = 0x506A2;
    16
}
/// $050AF8: bcc.w $50b02
pub fn i_050af8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50AFC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x50B02; return 10; }
    8
}
/// $050AFC: lea.l $50dbe(pc), a1
pub fn i_050afc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50B00;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x50DBE;
    16
}
/// $050B00: move.l a1, (a6)
pub fn i_050b00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50B02;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $050B02: jsr $506f6(pc)
pub fn i_050b02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50B06;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x50B06);
    cpu.pc = 0x506F6;
    16
}
/// $050B06: bcc.w $50b10
pub fn i_050b06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50B0A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x50B10; return 10; }
    8
}
/// $050B0A: lea.l $50b5e(pc), a1
pub fn i_050b0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50B0E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x50B5E;
    16
}
/// $050B0E: move.l a1, (a6)
pub fn i_050b0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50B10;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $050B10: tst.b $70(a6)
pub fn i_050b10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50B14;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $050B14: beq.w $50b1e
pub fn i_050b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50B18;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x50B1E; return 10; }
    8
}
/// $050B18: lea.l $50c88(pc), a1
pub fn i_050b18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50B1C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x50C88;
    16
}
/// $050B1C: move.l a1, (a6)
pub fn i_050b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50B1E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $050B1E: bra.w $5072e
pub fn i_050b1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50B22;
    cpu.ir = 0x6000;
    cpu.pc = 0x5072E;
    10
}
// ===== Allen_Turn_050b22  ($050B22, 60 B, enemies/bosses/allen_oneil_04fa58.s)
/// $050B22: clr.b $86(a6)
pub fn i_050b22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50B26;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $050B26: lea.l $296954.l, a0
pub fn i_050b26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50B2C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x296954;
    20
}
/// $050B2C: jsr $28cd4.l
pub fn i_050b2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50B32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50B32);
    cpu.pc = 0x28CD4;
    16
}
/// $050B32: clr.w $28(a6)
pub fn i_050b32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50B36;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $050B36: lea.l $50b3c(pc), a1
pub fn i_050b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50B3A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x50B3C;
    16
}
/// $050B3A: move.l a1, (a6)
pub fn i_050b3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50B3C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $050B3C: bsr.w $503f2
pub fn i_050b3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50B40;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x50B40);
    cpu.pc = 0x503F2;
    18
}
/// $050B40: bsr.w $50360
pub fn i_050b40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50B44;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x50B44);
    cpu.pc = 0x50360;
    18
}
/// $050B44: jsr $28d70.l
pub fn i_050b44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50B4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50B4A);
    cpu.pc = 0x28D70;
    16
}
/// $050B4A: bcc.w $50b5a
pub fn i_050b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50B4E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x50B5A; return 10; }
    8
}
/// $050B4E: bchg.b #$0, $3a(a6)
pub fn i_050b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50B54;
    cpu.ir = 0x86E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 ^ mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $050B54: lea.l $50a2a(pc), a1
pub fn i_050b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50B58;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x50A2A;
    16
}
/// $050B58: move.l a1, (a6)
pub fn i_050b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50B5A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $050B5A: bra.w $5072e
pub fn i_050b5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50B5E;
    cpu.ir = 0x6000;
    cpu.pc = 0x5072E;
    10
}
// ===== Allen_ToggleRetreat_050b5e  ($050B5E, 12 B, enemies/bosses/allen_oneil_04fa58.s)
/// $050B5E: addq.b #$1, $7a(a6)
pub fn i_050b5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50B62;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x7A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $050B62: not.b $78(a6)
pub fn i_050b62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50B66;
    cpu.ir = 0x462E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read8(ad1) as u32;
    let r3 = !v2 & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $050B66: bra.w $50a2a
pub fn i_050b66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50B6A;
    cpu.ir = 0x6000;
    cpu.pc = 0x50A2A;
    10
}
// ===== Allen_Taunt_050b6a  ($050B6A, 88 B, enemies/bosses/allen_oneil_04fa58.s)
/// $050B6A: tst.b $86(a6)
pub fn i_050b6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50B6E;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $050B6E: bne.w $50b90
pub fn i_050b6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50B72;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x50B90; return 10; }
    8
}
/// $050B72: move.b #$1e, $86(a6)
pub fn i_050b72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50B78;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write8(ad2, v1 as u8);
    12
}
/// $050B78: jsr $5e9b6.l
pub fn i_050b78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50B7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50B7E);
    cpu.pc = 0x5E9B6;
    16
}
/// $050B7E: andi.w #$6, d0
pub fn i_050b7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50B82;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x6) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $050B82: lea.l $509d6(pc), a0
pub fn i_050b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50B86;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x509D6;
    16
}
/// $050B86: move.w (a0, d0.w), d0
pub fn i_050b86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50B8A;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $050B8A: jsr $2352.l
pub fn i_050b8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50B90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50B90);
    cpu.pc = 0x2352;
    16
}
/// $050B90: lea.l $296b76.l, a0
pub fn i_050b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50B96;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x296B76;
    20
}
/// $050B96: jsr $28cd4.l
pub fn i_050b96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50B9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50B9C);
    cpu.pc = 0x28CD4;
    16
}
/// $050B9C: clr.w $28(a6)
pub fn i_050b9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50BA0;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $050BA0: lea.l $50ba6(pc), a1
pub fn i_050ba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50BA4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x50BA6;
    16
}
/// $050BA4: move.l a1, (a6)
pub fn i_050ba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50BA6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $050BA6: bsr.w $503f2
pub fn i_050ba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50BAA;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x50BAA);
    cpu.pc = 0x503F2;
    18
}
/// $050BAA: bsr.w $50360
pub fn i_050baa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50BAE;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x50BAE);
    cpu.pc = 0x50360;
    18
}
/// $050BAE: jsr $28d70.l
pub fn i_050bae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50BB4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50BB4);
    cpu.pc = 0x28D70;
    16
}
/// $050BB4: bcc.w $50bbe
pub fn i_050bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50BB8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x50BBE; return 10; }
    8
}
/// $050BB8: lea.l $50a2a(pc), a1
pub fn i_050bb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50BBC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x50A2A;
    16
}
/// $050BBC: move.l a1, (a6)
pub fn i_050bbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50BBE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $050BBE: bra.w $5072e
pub fn i_050bbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50BC2;
    cpu.ir = 0x6000;
    cpu.pc = 0x5072E;
    10
}
// ===== Allen_FireMGHigh_050bc2  ($050BC2, 16 B, enemies/bosses/allen_oneil_04fa58.s)
/// $050BC2: lea.l $29658a.l, a0
pub fn i_050bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50BC8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29658A;
    20
}
/// $050BC8: jsr $28cd4.l
pub fn i_050bc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50BCE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50BCE);
    cpu.pc = 0x28CD4;
    16
}
/// $050BCE: bra.w $50bde
pub fn i_050bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50BD2;
    cpu.ir = 0x6000;
    cpu.pc = 0x50BDE;
    10
}
// ===== Allen_FireMGLevel_050bd2  ($050BD2, 70 B, enemies/bosses/allen_oneil_04fa58.s)
/// $050BD2: lea.l $29668c.l, a0
pub fn i_050bd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50BD8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29668C;
    20
}
/// $050BD8: jsr $28cd4.l
pub fn i_050bd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50BDE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50BDE);
    cpu.pc = 0x28CD4;
    16
}
/// $050BDE: clr.b $86(a6)
pub fn i_050bde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50BE2;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $050BE2: clr.w $28(a6)
pub fn i_050be2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50BE6;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $050BE6: lea.l $50bec(pc), a1
pub fn i_050be6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50BEA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x50BEC;
    16
}
/// $050BEA: move.l a1, (a6)
pub fn i_050bea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50BEC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $050BEC: bsr.w $503f2
pub fn i_050bec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50BF0;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x50BF0);
    cpu.pc = 0x503F2;
    18
}
/// $050BF0: bsr.w $50360
pub fn i_050bf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50BF4;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x50BF4);
    cpu.pc = 0x50360;
    18
}
/// $050BF4: jsr $28d70.l
pub fn i_050bf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50BFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50BFA);
    cpu.pc = 0x28D70;
    16
}
/// $050BFA: bcc.w $50c14
pub fn i_050bfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50BFE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x50C14; return 10; }
    8
}
/// $050BFE: lea.l $296efc.l, a0
pub fn i_050bfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50C04;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x296EFC;
    20
}
/// $050C04: jsr $799de.l
pub fn i_050c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50C0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50C0A);
    cpu.pc = 0x799DE;
    16
}
/// $050C0A: move.b d0, $7b(a6)
pub fn i_050c0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50C0E;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $050C0E: lea.l $50a2a(pc), a1
pub fn i_050c0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50C12;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x50A2A;
    16
}
/// $050C12: move.l a1, (a6)
pub fn i_050c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50C14;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $050C14: bra.w $5072e
pub fn i_050c14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50C18;
    cpu.ir = 0x6000;
    cpu.pc = 0x5072E;
    10
}
// ===== Allen_IdleNoTarget_050c18  ($050C18, 54 B, enemies/bosses/allen_oneil_04fa58.s)
/// $050C18: clr.w $28(a6)
pub fn i_050c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50C1C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $050C1C: lea.l $296bd8.l, a0
pub fn i_050c1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50C22;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x296BD8;
    20
}
/// $050C22: jsr $28cd4.l
pub fn i_050c22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50C28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50C28);
    cpu.pc = 0x28CD4;
    16
}
/// $050C28: lea.l $50c2e(pc), a1
pub fn i_050c28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50C2C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x50C2E;
    16
}
/// $050C2C: move.l a1, (a6)
pub fn i_050c2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50C2E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $050C2E: bsr.w $503f2
pub fn i_050c2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50C32;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x50C32);
    cpu.pc = 0x503F2;
    18
}
/// $050C32: bsr.w $50360
pub fn i_050c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50C36;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x50C36);
    cpu.pc = 0x50360;
    18
}
/// $050C36: jsr $28d70.l
pub fn i_050c36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50C3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50C3C);
    cpu.pc = 0x28D70;
    16
}
/// $050C3C: move.l $72(a6), d0
pub fn i_050c3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50C40;
    cpu.ir = 0x202E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $050C40: bmi.w $50c4a
pub fn i_050c40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50C44;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x50C4A; return 10; }
    8
}
/// $050C44: lea.l $50a2a(pc), a1
pub fn i_050c44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50C48;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x50A2A;
    16
}
/// $050C48: move.l a1, (a6)
pub fn i_050c48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50C4A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $050C4A: bra.w $5072e
pub fn i_050c4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50C4E;
    cpu.ir = 0x6000;
    cpu.pc = 0x5072E;
    10
}
// ===== Allen_Land_050c4e  ($050C4E, 58 B, enemies/bosses/allen_oneil_04fa58.s)
/// $050C4E: clr.b $86(a6)
pub fn i_050c4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50C52;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $050C52: lea.l $296a04.l, a0
pub fn i_050c52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50C58;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x296A04;
    20
}
/// $050C58: jsr $28cd4.l
pub fn i_050c58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50C5E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50C5E);
    cpu.pc = 0x28CD4;
    16
}
/// $050C5E: clr.b $7a(a6)
pub fn i_050c5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50C62;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x7A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $050C62: clr.w $28(a6)
pub fn i_050c62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50C66;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $050C66: lea.l $50c6c(pc), a1
pub fn i_050c66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50C6A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x50C6C;
    16
}
/// $050C6A: move.l a1, (a6)
pub fn i_050c6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50C6C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $050C6C: bsr.w $503f2
pub fn i_050c6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50C70;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x50C70);
    cpu.pc = 0x503F2;
    18
}
/// $050C70: bsr.w $50360
pub fn i_050c70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50C74;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x50C74);
    cpu.pc = 0x50360;
    18
}
/// $050C74: jsr $28d70.l
pub fn i_050c74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50C7A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50C7A);
    cpu.pc = 0x28D70;
    16
}
/// $050C7A: bcc.w $50c84
pub fn i_050c7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50C7E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x50C84; return 10; }
    8
}
/// $050C7E: lea.l $50a2a(pc), a1
pub fn i_050c7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50C82;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x50A2A;
    16
}
/// $050C82: move.l a1, (a6)
pub fn i_050c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50C84;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $050C84: bra.w $5072e
pub fn i_050c84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50C88;
    cpu.ir = 0x6000;
    cpu.pc = 0x5072E;
    10
}
// ===== Allen_JumpDecide_050c88  ($050C88, 146 B, enemies/bosses/allen_oneil_04fa58.s)
/// $050C88: move.w $24(a6), d1
pub fn i_050c88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50C8C;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $050C8C: jsr $4400e.l
pub fn i_050c8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50C92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50C92);
    cpu.pc = 0x4400E;
    16
}
/// $050C92: cmpi.w #$8f, d1
pub fn i_050c92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50C96;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x8F, Size::Word, false);
    8
}
/// $050C96: ble.w $50cb4
pub fn i_050c96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50C9A;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x50CB4; return 10; }
    8
}
/// $050C9A: move.l $72(a6), d0
pub fn i_050c9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50C9E;
    cpu.ir = 0x202E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $050C9E: bmi.w $50cb4
pub fn i_050c9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50CA2;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x50CB4; return 10; }
    8
}
/// $050CA2: movea.l d0, a0
pub fn i_050ca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50CA4;
    cpu.ir = 0x2040;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $050CA4: move.w $24(a0), d0
pub fn i_050ca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50CA8;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $050CA8: cmp.w $24(a6), d0
pub fn i_050ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50CAC;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $050CAC: bcs.w $50d1a
pub fn i_050cac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50CB0;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x50D1A; return 10; }
    8
}
/// $050CB0: bra.w $50cf6
pub fn i_050cb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50CB4;
    cpu.ir = 0x6000;
    cpu.pc = 0x50CF6;
    10
}
/// $050CB4: tst.b $79(a6)
pub fn i_050cb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50CB8;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x79);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $050CB8: bne.w $50cf6
pub fn i_050cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50CBC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x50CF6; return 10; }
    8
}
/// $050CBC: lea.l $2969bc.l, a0
pub fn i_050cbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50CC2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2969BC;
    20
}
/// $050CC2: jsr $28cd4.l
pub fn i_050cc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50CC8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50CC8);
    cpu.pc = 0x28CD4;
    16
}
/// $050CC8: move.w #$886, $2a(a6)
pub fn i_050cc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50CCE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x886;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $050CCE: move.w #$fedd, $2e(a6)
pub fn i_050cce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50CD4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFEDD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $050CD4: move.w #$444, d0
pub fn i_050cd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50CD8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x444;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $050CD8: tst.b $78(a6)
pub fn i_050cd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50CDC;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $050CDC: beq.w $50ce2
pub fn i_050cdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50CE0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x50CE2; return 10; }
    8
}
/// $050CE0: neg.w d0
pub fn i_050ce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50CE2;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $050CE2: move.w d0, $28(a6)
pub fn i_050ce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50CE6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $050CE6: move.b #$ff, $70(a6)
pub fn i_050ce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50CEC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $050CEC: lea.l $50d90(pc), a1
pub fn i_050cec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50CF0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x50D90;
    16
}
/// $050CF0: move.l a1, (a6)
pub fn i_050cf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50CF2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $050CF2: bra.w $50d90
pub fn i_050cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50CF6;
    cpu.ir = 0x6000;
    cpu.pc = 0x50D90;
    10
}
/// $050CF6: lea.l $2969bc.l, a0
pub fn i_050cf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50CFC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2969BC;
    20
}
/// $050CFC: jsr $28cd4.l
pub fn i_050cfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50D02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50D02);
    cpu.pc = 0x28CD4;
    16
}
/// $050D02: clr.w $28(a6)
pub fn i_050d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50D06;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $050D06: clr.w $2a(a6)
pub fn i_050d06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50D0A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $050D0A: move.w #$ffc0, $2e(a6)
pub fn i_050d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50D10;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFC0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $050D10: lea.l $50d90(pc), a1
pub fn i_050d10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50D14;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x50D90;
    16
}
/// $050D14: move.l a1, (a6)
pub fn i_050d14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50D16;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $050D16: bra.w $50d90
pub fn i_050d16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50D1A;
    cpu.ir = 0x6000;
    cpu.pc = 0x50D90;
    10
}
// ===== Allen_JumpToward_050d1a  ($050D1A, 118 B, enemies/bosses/allen_oneil_04fa58.s)
/// $050D1A: lea.l $2969bc.l, a0
pub fn i_050d1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50D20;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2969BC;
    20
}
/// $050D20: jsr $28cd4.l
pub fn i_050d20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50D26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50D26);
    cpu.pc = 0x28CD4;
    16
}
/// $050D26: move.l #$1111, d0
pub fn i_050d26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50D2C;
    cpu.ir = 0x203C;
    let v1: u32 = 0x1111;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $050D2C: move.w #$180, d1
pub fn i_050d2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50D30;
    cpu.ir = 0x323C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $050D30: mulu.w d0, d1
pub fn i_050d30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50D32;
    cpu.ir = 0xC2C0;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[1] & 0xFFFF);
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $050D32: lsr.l #$8, d1
pub fn i_050d32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50D34;
    cpu.ir = 0xE089;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 1, false);
    cpu.d[1] = r;
    22
}
/// $050D34: mulu.w d0, d1
pub fn i_050d34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50D36;
    cpu.ir = 0xC2C0;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[1] & 0xFFFF);
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $050D36: swap d1
pub fn i_050d36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50D38;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $050D38: neg.w d1
pub fn i_050d38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50D3A;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $050D3A: move.w d1, $2e(a6)
pub fn i_050d3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50D3E;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $050D3E: move.w #$f0, d1
pub fn i_050d3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50D42;
    cpu.ir = 0x323C;
    let v1: u32 = 0xF0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $050D42: mulu.w d0, d1
pub fn i_050d42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50D44;
    cpu.ir = 0xC2C0;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[1] & 0xFFFF);
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $050D44: lsr.l #$8, d1
pub fn i_050d44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50D46;
    cpu.ir = 0xE089;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 1, false);
    cpu.d[1] = r;
    22
}
/// $050D46: move.w d1, $2a(a6)
pub fn i_050d46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50D4A;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $050D4A: move.w $24(a6), d1
pub fn i_050d4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50D4E;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $050D4E: jsr $4400e.l
pub fn i_050d4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50D54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50D54);
    cpu.pc = 0x4400E;
    16
}
/// $050D54: cmpi.w #$bf, d1
pub fn i_050d54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50D58;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xBF, Size::Word, false);
    8
}
/// $050D58: ble.w $50d64
pub fn i_050d58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50D5C;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x50D64; return 10; }
    8
}
/// $050D5C: move.w #$30, d0
pub fn i_050d5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50D60;
    cpu.ir = 0x303C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $050D60: bra.w $50d68
pub fn i_050d60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50D64;
    cpu.ir = 0x6000;
    cpu.pc = 0x50D68;
    10
}
/// $050D64: move.w #$60, d0
pub fn i_050d64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50D68;
    cpu.ir = 0x303C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $050D68: cmpi.w #$a0, $22(a6)
pub fn i_050d68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50D6E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xA0, Size::Word, false);
    16
}
/// $050D6E: bmi.w $50d78
pub fn i_050d6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50D72;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x50D78; return 10; }
    8
}
/// $050D72: neg.w d0
pub fn i_050d72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50D74;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $050D74: addi.w #$140, d0
pub fn i_050d74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50D78;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x140, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $050D78: sub.w $22(a6), d0
pub fn i_050d78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50D7C;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $050D7C: muls.w #$11, d0
pub fn i_050d7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50D80;
    cpu.ir = 0xC1FC;
    let v1: u32 = 0x11;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[0] as u16 as i16 as i32)) as u32;
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $050D80: move.w d0, $28(a6)
pub fn i_050d80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50D84;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $050D84: move.b #$ff, $70(a6)
pub fn i_050d84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50D8A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $050D8A: lea.l $50d90(pc), a1
pub fn i_050d8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50D8E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x50D90;
    16
}
/// $050D8E: move.l a1, (a6)
pub fn i_050d8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50D90;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Allen_Airborne_050d90  ($050D90, 46 B, enemies/bosses/allen_oneil_04fa58.s)
/// $050D90: bsr.w $503f2
pub fn i_050d90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50D94;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x50D94);
    cpu.pc = 0x503F2;
    18
}
/// $050D94: bsr.w $50360
pub fn i_050d94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50D98;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x50D98);
    cpu.pc = 0x50360;
    18
}
/// $050D98: jsr $28d70.l
pub fn i_050d98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50D9E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50D9E);
    cpu.pc = 0x28D70;
    16
}
/// $050D9E: tst.b $70(a6)
pub fn i_050d9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50DA2;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $050DA2: bne.w $50dac
pub fn i_050da2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50DA6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x50DAC; return 10; }
    8
}
/// $050DA6: lea.l $50c4e(pc), a1
pub fn i_050da6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50DAA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x50C4E;
    16
}
/// $050DAA: move.l a1, (a6)
pub fn i_050daa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50DAC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $050DAC: jsr $505f0(pc)
pub fn i_050dac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50DB0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x50DB0);
    cpu.pc = 0x505F0;
    16
}
/// $050DB0: bcc.w $50dba
pub fn i_050db0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50DB4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x50DBA; return 10; }
    8
}
/// $050DB4: lea.l $50dfc(pc), a1
pub fn i_050db4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50DB8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x50DFC;
    16
}
/// $050DB8: move.l a1, (a6)
pub fn i_050db8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50DBA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $050DBA: bra.w $5072e
pub fn i_050dba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50DBE;
    cpu.ir = 0x6000;
    cpu.pc = 0x5072E;
    10
}
// ===== Allen_JumpHigh_050dbe  ($050DBE, 62 B, enemies/bosses/allen_oneil_04fa58.s)
/// $050DBE: lea.l $2969bc.l, a0
pub fn i_050dbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50DC4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2969BC;
    20
}
/// $050DC4: jsr $28cd4.l
pub fn i_050dc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50DCA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50DCA);
    cpu.pc = 0x28CD4;
    16
}
/// $050DCA: move.w #$0, d0
pub fn i_050dca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50DCE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $050DCE: jsr $5dca4.l
pub fn i_050dce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50DD4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50DD4);
    cpu.pc = 0x5DCA4;
    16
}
/// $050DD4: move.w d0, $28(a6)
pub fn i_050dd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50DD8;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $050DD8: move.w #$922, $2a(a6)
pub fn i_050dd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50DDE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x922;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $050DDE: move.w #$fd64, $2e(a6)
pub fn i_050dde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50DE4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFD64;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $050DE4: move.w #$0, $2c(a6)
pub fn i_050de4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50DEA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $050DEA: move.b #$ff, $70(a6)
pub fn i_050dea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50DF0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $050DF0: clr.b $71(a6)
pub fn i_050df0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50DF4;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x71);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $050DF4: lea.l $50d90(pc), a1
pub fn i_050df4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50DF8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x50D90;
    16
}
/// $050DF8: move.l a1, (a6)
pub fn i_050df8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50DFA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $050DFA: bra.b $50d90
pub fn i_050dfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50DFC;
    cpu.ir = 0x6094;
    cpu.pc = 0x50D90;
    10
}
// ===== Allen_KnifeAttack_050dfc  ($050DFC, 68 B, enemies/bosses/allen_oneil_04fa58.s)
/// $050DFC: clr.b $86(a6)
pub fn i_050dfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50E00;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $050E00: lea.l $296c82.l, a0
pub fn i_050e00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50E06;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x296C82;
    20
}
/// $050E06: jsr $28cd4.l
pub fn i_050e06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50E0C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50E0C);
    cpu.pc = 0x28CD4;
    16
}
/// $050E0C: jsr $283ca.l
pub fn i_050e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50E12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50E12);
    cpu.pc = 0x283CA;
    16
}
/// $050E12: lea.l $50e18(pc), a1
pub fn i_050e12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50E16;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x50E18;
    16
}
/// $050E16: move.l a1, (a6)
pub fn i_050e16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50E18;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $050E18: bsr.w $503f2
pub fn i_050e18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50E1C;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x50E1C);
    cpu.pc = 0x503F2;
    18
}
/// $050E1C: bsr.w $50360
pub fn i_050e1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50E20;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x50E20);
    cpu.pc = 0x50360;
    18
}
/// $050E20: jsr $28d70.l
pub fn i_050e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50E26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50E26);
    cpu.pc = 0x28D70;
    16
}
/// $050E26: bcc.w $50e30
pub fn i_050e26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50E2A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x50E30; return 10; }
    8
}
/// $050E2A: lea.l $50a2a(pc), a1
pub fn i_050e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50E2E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x50A2A;
    16
}
/// $050E2E: move.l a1, (a6)
pub fn i_050e2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50E30;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $050E30: tst.b $70(a6)
pub fn i_050e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50E34;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $050E34: bne.w $50e3c
pub fn i_050e34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50E38;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x50E3C; return 10; }
    8
}
/// $050E38: clr.w $28(a6)
pub fn i_050e38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50E3C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $050E3C: bra.w $5072e
pub fn i_050e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50E40;
    cpu.ir = 0x6000;
    cpu.pc = 0x5072E;
    10
}
// ===== Allen_Death_050e40  ($050E40, 68 B, enemies/bosses/allen_oneil_04fa58.s)
/// $050E40: jsr $4aad2.l
pub fn i_050e40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50E46;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50E46);
    cpu.pc = 0x4AAD2;
    16
}
/// $050E46: move.w #$1140, d0
pub fn i_050e46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50E4A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1140;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $050E4A: jsr $2352.l
pub fn i_050e4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50E50;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50E50);
    cpu.pc = 0x2352;
    16
}
/// $050E50: move.w $14(a6), d1
pub fn i_050e50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50E54;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $050E54: move.w #$18e, d2
pub fn i_050e54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50E58;
    cpu.ir = 0x343C;
    let v1: u32 = 0x18E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $050E58: move.w #$1, d3
pub fn i_050e58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50E5C;
    cpu.ir = 0x363C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $050E5C: move.w #$1, d4
pub fn i_050e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50E60;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $050E60: jsr $2c30.l
pub fn i_050e60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50E66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50E66);
    cpu.pc = 0x2C30;
    16
}
/// $050E66: tst.b $70(a6)
pub fn i_050e66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50E6A;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $050E6A: bne.w $50ea2
pub fn i_050e6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50E6E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x50EA2; return 10; }
    8
}
/// $050E6E: lea.l $296a4c.l, a0
pub fn i_050e6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50E74;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x296A4C;
    20
}
/// $050E74: jsr $28cd4.l
pub fn i_050e74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50E7A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50E7A);
    cpu.pc = 0x28CD4;
    16
}
/// $050E7A: clr.w $28(a6)
pub fn i_050e7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50E7E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $050E7E: lea.l $50e84(pc), a1
pub fn i_050e7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50E82;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x50E84;
    16
}
/// $050E82: move.l a1, (a6)
pub fn i_050e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50E84;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Allen_DeathFall_050e84  ($050E84, 114 B, enemies/bosses/allen_oneil_04fa58.s)
/// $050E84: move.b #$2, $45(a6)
pub fn i_050e84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50E8A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $050E8A: bsr.w $50360
pub fn i_050e8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50E8E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x50E8E);
    cpu.pc = 0x50360;
    18
}
/// $050E8E: jsr $28d70.l
pub fn i_050e8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50E94;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50E94);
    cpu.pc = 0x28D70;
    16
}
/// $050E94: bcc.w $50e9e
pub fn i_050e94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50E98;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x50E9E; return 10; }
    8
}
/// $050E98: lea.l $50f56(pc), a1
pub fn i_050e98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50E9C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x50F56;
    16
}
/// $050E9C: move.l a1, (a6)
pub fn i_050e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50E9E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $050E9E: bra.w $507ee
pub fn i_050e9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50EA2;
    cpu.ir = 0x6000;
    cpu.pc = 0x507EE;
    10
}
/// $050EA2: lea.l $296abc.l, a0
pub fn i_050ea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50EA8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x296ABC;
    20
}
/// $050EA8: jsr $28cd4.l
pub fn i_050ea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50EAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50EAE);
    cpu.pc = 0x28CD4;
    16
}
/// $050EAE: move.w #$0, d0
pub fn i_050eae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50EB2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $050EB2: jsr $5dca4.l
pub fn i_050eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50EB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50EB8);
    cpu.pc = 0x5DCA4;
    16
}
/// $050EB8: move.w d0, $28(a6)
pub fn i_050eb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50EBC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $050EBC: move.w #$400, $2a(a6)
pub fn i_050ebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50EC2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $050EC2: move.w #$ff00, $2e(a6)
pub fn i_050ec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50EC8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $050EC8: move.w #$0, $2c(a6)
pub fn i_050ec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50ECE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $050ECE: lea.l $50ed4(pc), a1
pub fn i_050ece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50ED2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x50ED4;
    16
}
/// $050ED2: move.l a1, (a6)
pub fn i_050ed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50ED4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $050ED4: move.b #$2, $45(a6)
pub fn i_050ed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50EDA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $050EDA: jsr $50360(pc)
pub fn i_050eda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50EDE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x50EDE);
    cpu.pc = 0x50360;
    16
}
/// $050EDE: jsr $28d70.l
pub fn i_050ede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50EE4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50EE4);
    cpu.pc = 0x28D70;
    16
}
/// $050EE4: tst.b $70(a6)
pub fn i_050ee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50EE8;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $050EE8: bne.w $50ef2
pub fn i_050ee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50EEC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x50EF2; return 10; }
    8
}
/// $050EEC: lea.l $50ef6(pc), a1
pub fn i_050eec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50EF0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x50EF6;
    16
}
/// $050EF0: move.l a1, (a6)
pub fn i_050ef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50EF2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $050EF2: bra.w $507ee
pub fn i_050ef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50EF6;
    cpu.ir = 0x6000;
    cpu.pc = 0x507EE;
    10
}
// ===== Allen_DeathLand_050ef6  ($050EF6, 26 B, enemies/bosses/allen_oneil_04fa58.s)
/// $050EF6: lea.l $296adc.l, a0
pub fn i_050ef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50EFC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x296ADC;
    20
}
/// $050EFC: jsr $28cd4.l
pub fn i_050efc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50F02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50F02);
    cpu.pc = 0x28CD4;
    16
}
/// $050F02: clr.w $28(a6)
pub fn i_050f02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50F06;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $050F06: lea.l $50e84(pc), a1
pub fn i_050f06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50F0A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x50E84;
    16
}
/// $050F0A: move.l a1, (a6)
pub fn i_050f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50F0C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $050F0C: bra.w $50e84
pub fn i_050f0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50F10;
    cpu.ir = 0x6000;
    cpu.pc = 0x50E84;
    10
}
// ===== Allen_Stagger_050f10  ($050F10, 70 B, enemies/bosses/allen_oneil_04fa58.s)
/// $050F10: lea.l $296b24.l, a0
pub fn i_050f10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50F16;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x296B24;
    20
}
/// $050F16: jsr $28cd4.l
pub fn i_050f16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50F1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50F1C);
    cpu.pc = 0x28CD4;
    16
}
/// $050F1C: clr.w $28(a6)
pub fn i_050f1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50F20;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $050F20: lea.l $50f26(pc), a1
pub fn i_050f20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50F24;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x50F26;
    16
}
/// $050F24: move.l a1, (a6)
pub fn i_050f24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50F26;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $050F26: move.b #$2, $45(a6)
pub fn i_050f26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50F2C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $050F2C: bsr.w $50360
pub fn i_050f2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50F30;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x50F30);
    cpu.pc = 0x50360;
    18
}
/// $050F30: jsr $28d70.l
pub fn i_050f30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50F36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50F36);
    cpu.pc = 0x28D70;
    16
}
/// $050F36: bcc.w $50f52
pub fn i_050f36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50F3A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x50F52; return 10; }
    8
}
/// $050F3A: move.b #$f, $45(a6)
pub fn i_050f3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50F40;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $050F40: move.b $45(a6), $59(a6)
pub fn i_050f40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50F46;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x59);
    bus.write8(ad4, v3 as u8);
    20
}
/// $050F46: bclr.b #$3, $13(a6)
pub fn i_050f46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50F4C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $050F4C: lea.l $50a2a(pc), a1
pub fn i_050f4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50F50;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x50A2A;
    16
}
/// $050F50: move.l a1, (a6)
pub fn i_050f50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50F52;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $050F52: bra.w $507ee
pub fn i_050f52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50F56;
    cpu.ir = 0x6000;
    cpu.pc = 0x507EE;
    10
}
// ===== Allen_DeathFade_050f56  ($050F56, 40 B, enemies/bosses/allen_oneil_04fa58.s)
/// $050F56: move.b #$1e, $59(a6)
pub fn i_050f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50F5C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x59);
    bus.write8(ad2, v1 as u8);
    12
}
/// $050F5C: lea.l $50f62(pc), a1
pub fn i_050f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50F60;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x50F62;
    16
}
/// $050F60: move.l a1, (a6)
pub fn i_050f60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50F62;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $050F62: bsr.w $50360
pub fn i_050f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50F66;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x50F66);
    cpu.pc = 0x50360;
    18
}
/// $050F66: jsr $28d70.l
pub fn i_050f66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50F6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50F6C);
    cpu.pc = 0x28D70;
    16
}
/// $050F6C: tst.b $59(a6)
pub fn i_050f6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50F70;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x59);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $050F70: bne.w $50f7a
pub fn i_050f70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50F74;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x50F7A; return 10; }
    8
}
/// $050F74: jmp $518.l
pub fn i_050f74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50F7A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $050F7A: bra.w $507ee
pub fn i_050f7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50F7E;
    cpu.ir = 0x6000;
    cpu.pc = 0x507EE;
    10
}
// ===== Allen_TouchSensor_050f7e  ($050F7E, 144 B, enemies/bosses/allen_oneil_04fa58.s)
/// $050F7E: lea.l $29643a.l, a0
pub fn i_050f7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50F84;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29643A;
    20
}
/// $050F84: move.l a0, $48(a6)
pub fn i_050f84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50F88;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $050F88: move.w #$7e, $5c(a6)
pub fn i_050f88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50F8E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $050F8E: bra.w $50fca
pub fn i_050f8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50F92;
    cpu.ir = 0x6000;
    cpu.pc = 0x50FCA;
    10
}
/// $050F92: lea.l $29648e.l, a0
pub fn i_050f92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50F98;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29648E;
    20
}
/// $050F98: move.l a0, $48(a6)
pub fn i_050f98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50F9C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $050F9C: move.w #$7f, $5c(a6)
pub fn i_050f9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50FA2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $050FA2: bra.w $50fca
pub fn i_050fa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50FA6;
    cpu.ir = 0x6000;
    cpu.pc = 0x50FCA;
    10
}
/// $050FA6: lea.l $2964e2.l, a0
pub fn i_050fa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50FAC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2964E2;
    20
}
/// $050FAC: move.l a0, $48(a6)
pub fn i_050fac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50FB0;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $050FB0: move.w #$7c, $5c(a6)
pub fn i_050fb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50FB6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $050FB6: bra.w $50fca
pub fn i_050fb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50FBA;
    cpu.ir = 0x6000;
    cpu.pc = 0x50FCA;
    10
}
/// $050FBA: lea.l $296536.l, a0
pub fn i_050fba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50FC0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x296536;
    20
}
/// $050FC0: move.l a0, $48(a6)
pub fn i_050fc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50FC4;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $050FC4: move.w #$7d, $5c(a6)
pub fn i_050fc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50FCA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $050FCA: lea.l $50fd0(pc), a1
pub fn i_050fca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50FCE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x50FD0;
    16
}
/// $050FCE: move.l a1, (a6)
pub fn i_050fce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50FD0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $050FD0: movea.l $c(a6), a0
pub fn i_050fd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50FD4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $050FD4: move.w $22(a0), $22(a6)
pub fn i_050fd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50FDA;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $050FDA: move.w $24(a0), $24(a6)
pub fn i_050fda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50FE0;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $050FE0: tst.b $87(a0)
pub fn i_050fe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50FE4;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x87);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $050FE4: bne.w $50ffc
pub fn i_050fe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50FE8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x50FFC; return 10; }
    8
}
/// $050FE8: jsr $2870a.l
pub fn i_050fe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50FEE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x50FEE);
    cpu.pc = 0x2870A;
    16
}
/// $050FEE: movea.l $c(a6), a0
pub fn i_050fee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50FF2;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $050FF2: adda.w $5c(a6), a0
pub fn i_050ff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50FF6;
    cpu.ir = 0xD0EE;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.a[0] = cpu.a[0].wrapping_add((v3 as u16 as i16 as i32 as u32));
    16
}
/// $050FF6: scs.b (a0)
pub fn i_050ff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50FF8;
    cpu.ir = 0x55D0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let ad2 = cpu.a[0];
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    8
}
/// $050FF8: bra.w $51006
pub fn i_050ff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x50FFC;
    cpu.ir = 0x6000;
    cpu.pc = 0x51006;
    10
}
/// $050FFC: movea.l $c(a6), a0
pub fn i_050ffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51000;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $051000: adda.w $5c(a6), a0
pub fn i_051000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51004;
    cpu.ir = 0xD0EE;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.a[0] = cpu.a[0].wrapping_add((v3 as u16 as i16 as i32 as u32));
    16
}
/// $051004: clr.b (a0)
pub fn i_051004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51006;
    cpu.ir = 0x4210;
    let ad1 = cpu.a[0];
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    8
}
/// $051006: bclr.b #$3, $13(a6)
pub fn i_051006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5100C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05100C: rts
pub fn i_05100c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5100E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Allen_MuzzleFlash_05100e  ($05100E, 90 B, enemies/bosses/allen_oneil_04fa58.s)
/// $05100E: lea.l $29663a.l, a0
pub fn i_05100e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51014;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29663A;
    20
}
/// $051014: jsr $28cd4.l
pub fn i_051014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5101A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5101A);
    cpu.pc = 0x28CD4;
    16
}
/// $05101A: bra.w $5102a
pub fn i_05101a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5101E;
    cpu.ir = 0x6000;
    cpu.pc = 0x5102A;
    10
}
/// $05101E: lea.l $296784.l, a0
pub fn i_05101e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51024;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x296784;
    20
}
/// $051024: jsr $28cd4.l
pub fn i_051024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5102A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5102A);
    cpu.pc = 0x28CD4;
    16
}
/// $05102A: move.w #$18f, d1
pub fn i_05102a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5102E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x18F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05102E: jsr $236e.l
pub fn i_05102e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51034;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x51034);
    cpu.pc = 0x236E;
    16
}
/// $051034: lea.l $5103a(pc), a1
pub fn i_051034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51038;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5103A;
    16
}
/// $051038: move.l a1, (a6)
pub fn i_051038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5103A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05103A: movea.l $c(a6), a0
pub fn i_05103a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5103E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $05103E: move.b $3a(a0), $3a(a6)
pub fn i_05103e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51044;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $051044: move.w $38(a0), $38(a6)
pub fn i_051044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5104A;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad4, v3 as u16);
    20
}
/// $05104A: move.w $22(a0), $22(a6)
pub fn i_05104a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51050;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $051050: move.w $24(a0), $24(a6)
pub fn i_051050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51056;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $051056: jsr $28d70.l
pub fn i_051056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5105C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5105C);
    cpu.pc = 0x28D70;
    16
}
/// $05105C: bcc.w $51066
pub fn i_05105c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51060;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x51066; return 10; }
    8
}
/// $051060: jmp $518.l
pub fn i_051060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51066;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $051066: rts
pub fn i_051066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51068;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0512B4: bset.b #$4, $6b(a6)
pub fn i_0512b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x512BA;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0512BA: move.w #$d000, $38(a6)
pub fn i_0512ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x512C0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0512C0: lea.l $296f7e.l, a0
pub fn i_0512c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x512C6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x296F7E;
    20
}
/// $0512C6: jsr $799de.l
pub fn i_0512c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x512CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x512CC);
    cpu.pc = 0x799DE;
    16
}
/// $0512CC: move.w d0, d1
pub fn i_0512cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x512CE;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0512CE: move.w $34(a6), d0
pub fn i_0512ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x512D2;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0512D2: jsr $13c0e.l
pub fn i_0512d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x512D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x512D8);
    cpu.pc = 0x13C0E;
    16
}
/// $0512D8: move.w d1, $28(a6)
pub fn i_0512d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x512DC;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0512DC: move.w d2, $2a(a6)
pub fn i_0512dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x512E0;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0512E0: clr.w $2c(a6)
pub fn i_0512e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x512E4;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0512E4: clr.w $2e(a6)
pub fn i_0512e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x512E8;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0512E8: move.w #$15d, d1
pub fn i_0512e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x512EC;
    cpu.ir = 0x323C;
    let v1: u32 = 0x15D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0512EC: jsr $236e.l
pub fn i_0512ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x512F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x512F2);
    cpu.pc = 0x236E;
    16
}
/// $0512F2: move.w #$164, d1
pub fn i_0512f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x512F6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x164;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0512F6: jsr $236e.l
pub fn i_0512f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x512FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x512FC);
    cpu.pc = 0x236E;
    16
}
/// $0512FC: move.w #$165, d1
pub fn i_0512fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51300;
    cpu.ir = 0x323C;
    let v1: u32 = 0x165;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $051300: jsr $236e.l
pub fn i_051300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51306;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x51306);
    cpu.pc = 0x236E;
    16
}
/// $051306: lea.l $51208(pc), a0
pub fn i_051306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5130A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x51208;
    16
}
/// $05130A: jsr $28cd4.l
pub fn i_05130a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51310;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x51310);
    cpu.pc = 0x28CD4;
    16
}
/// $051310: lea.l $51070(pc), a0
pub fn i_051310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51314;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x51070;
    16
}
/// $051314: move.l a0, $4c(a6)
pub fn i_051314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51318;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $051318: jsr $283ca.l
pub fn i_051318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5131E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5131E);
    cpu.pc = 0x283CA;
    16
}
/// $05131E: lea.l $51324(pc), a1
pub fn i_05131e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51322;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x51324;
    16
}
/// $051322: move.l a1, (a6)
pub fn i_051322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51324;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $051324: jsr $502a6(pc)
pub fn i_051324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51328;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x51328);
    cpu.pc = 0x502A6;
    16
}
/// $051328: bcc.w $51332
pub fn i_051328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5132C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x51332; return 10; }
    8
}
/// $05132C: lea.l $51392(pc), a1
pub fn i_05132c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51330;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x51392;
    16
}
/// $051330: move.l a1, (a6)
pub fn i_051330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51332;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $051332: moveq #$0, d0
pub fn i_051332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51334;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $051334: move.b $106f28.l, d0
pub fn i_051334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5133A;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $05133A: andi.w #$3, d0
pub fn i_05133a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5133E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05133E: lsl.w #$1, d0
pub fn i_05133e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51340;
    cpu.ir = 0xE348;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $051340: lea.l $51068(pc), a0
pub fn i_051340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51344;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x51068;
    16
}
/// $051344: move.w (a0, d0.w), d1
pub fn i_051344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51348;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $051348: move.w (a6, d1.w), $14(a6)
pub fn i_051348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5134E;
    cpu.ir = 0x3D76;
    let ad2 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    22
}
/// $05134E: jsr $28d70.l
pub fn i_05134e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51354;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x51354);
    cpu.pc = 0x28D70;
    16
}
/// $051354: btst.b #$1, $13(a6)
pub fn i_051354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5135A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05135A: beq.w $51364
pub fn i_05135a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5135E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x51364; return 10; }
    8
}
/// $05135E: lea.l $513da(pc), a1
pub fn i_05135e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51362;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x513DA;
    16
}
/// $051362: move.l a1, (a6)
pub fn i_051362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51364;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $051364: jsr $2870a.l
pub fn i_051364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5136A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5136A);
    cpu.pc = 0x2870A;
    16
}
/// $05136A: bcc.w $51374
pub fn i_05136a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5136E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x51374; return 10; }
    8
}
/// $05136E: lea.l $513b6(pc), a1
pub fn i_05136e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51372;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x513B6;
    16
}
/// $051372: move.l a1, (a6)
pub fn i_051372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51374;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $051374: movea.l #$ffffffff, a0
pub fn i_051374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5137A;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $05137A: lea.l $55cc8.l, a0
pub fn i_05137a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51380;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x55CC8;
    20
}
/// $051380: jsr $5dd56.l
pub fn i_051380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51386;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x51386);
    cpu.pc = 0x5DD56;
    16
}
/// $051386: bcc.w $51390
pub fn i_051386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5138A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x51390; return 10; }
    8
}
// ===== SetTaskHandler_05138a  ($05138A, 8 B, kernel/task_handlers.c)
/// $05138A: lea.l $5147e(pc), a1
pub fn i_05138a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5138E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5147E;
    16
}
/// $05138E: move.l a1, (a6)
pub fn i_05138e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51390;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $051390: rts
pub fn i_051390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51392;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Allen_GrenadeExplode_051392  ($051392, 184 B, enemies/bosses/allen_oneil_04fa58.s)
/// $051392: move.w #$2000, d0
pub fn i_051392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51396;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $051396: jsr $28134.l
pub fn i_051396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5139C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5139C);
    cpu.pc = 0x28134;
    16
}
/// $05139C: andi.w #$ffe3, $38(a6)
pub fn i_05139c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x513A2;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0513A2: ori.w #$10, $38(a6)
pub fn i_0513a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x513A8;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x10) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0513A8: move.w #$1027, d0
pub fn i_0513a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x513AC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1027;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0513AC: jsr $2352.l
pub fn i_0513ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x513B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x513B2);
    cpu.pc = 0x2352;
    16
}
/// $0513B2: bra.w $513fa
pub fn i_0513b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x513B6;
    cpu.ir = 0x6000;
    cpu.pc = 0x513FA;
    10
}
/// $0513B6: move.w #$8000, d0
pub fn i_0513b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x513BA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0513BA: jsr $28134.l
pub fn i_0513ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x513C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x513C0);
    cpu.pc = 0x28134;
    16
}
/// $0513C0: andi.w #$ffe3, $38(a6)
pub fn i_0513c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x513C6;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0513C6: ori.w #$4, $38(a6)
pub fn i_0513c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x513CC;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x4) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0513CC: move.w #$1027, d0
pub fn i_0513cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x513D0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1027;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0513D0: jsr $2352.l
pub fn i_0513d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x513D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x513D6);
    cpu.pc = 0x2352;
    16
}
/// $0513D6: bra.w $513fa
pub fn i_0513d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x513DA;
    cpu.ir = 0x6000;
    cpu.pc = 0x513FA;
    10
}
/// $0513DA: move.w #$8000, d0
pub fn i_0513da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x513DE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0513DE: jsr $28134.l
pub fn i_0513de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x513E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x513E4);
    cpu.pc = 0x28134;
    16
}
/// $0513E4: andi.w #$ffe3, $38(a6)
pub fn i_0513e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x513EA;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0513EA: ori.w #$10, $38(a6)
pub fn i_0513ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x513F0;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x10) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0513F0: move.w #$ffff, d0
pub fn i_0513f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x513F4;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0513F4: jsr $2352.l
pub fn i_0513f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x513FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x513FA);
    cpu.pc = 0x2352;
    16
}
/// $0513FA: move.l #$ffffffff, $48(a6)
pub fn i_0513fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51402;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $051402: bclr.b #$1, $13(a6)
pub fn i_051402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51408;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $051408: bclr.b #$3, $13(a6)
pub fn i_051408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5140E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05140E: bclr.b #$0, $13(a6)
pub fn i_05140e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51414;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $051414: jsr $13600.l
pub fn i_051414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5141A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5141A);
    cpu.pc = 0x13600;
    16
}
/// $05141A: move.w #$1e0, d1
pub fn i_05141a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5141E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1E0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05141E: jsr $236e.l
pub fn i_05141e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51424;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x51424);
    cpu.pc = 0x236E;
    16
}
/// $051424: lea.l $29e76c.l, a0
pub fn i_051424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5142A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29E76C;
    20
}
/// $05142A: jsr $28cd4.l
pub fn i_05142a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51430;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x51430);
    cpu.pc = 0x28CD4;
    16
}
/// $051430: lea.l $51114(pc), a0
pub fn i_051430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51434;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x51114;
    16
}
/// $051434: move.l a0, $4c(a6)
pub fn i_051434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51438;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $051438: jsr $283ca.l
pub fn i_051438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5143E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5143E);
    cpu.pc = 0x283CA;
    16
}
/// $05143E: jsr $283ca.l
pub fn i_05143e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51444;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x51444);
    cpu.pc = 0x283CA;
    16
}
/// $051444: jsr $283d8.l
pub fn i_051444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5144A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5144A);
    cpu.pc = 0x283D8;
    16
}
// ===== SetTaskHandler_05144a  ($05144A, 8 B, kernel/task_handlers.c)
/// $05144A: lea.l $51452(pc), a1
pub fn i_05144a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5144E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x51452;
    16
}
/// $05144E: move.l a1, (a6)
pub fn i_05144e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51450;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $051450: rts
pub fn i_051450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51452;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Allen_GrenadeExplodeTick_051452  ($051452, 36 B, enemies/bosses/allen_oneil_04fa58.s)
/// $051452: jsr $2783a.l
pub fn i_051452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51458;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x51458);
    cpu.pc = 0x2783A;
    16
}
/// $051458: jsr $28d70.l
pub fn i_051458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5145E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5145E);
    cpu.pc = 0x28D70;
    16
}
/// $05145E: bcc.w $5146e
pub fn i_05145e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51462;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5146E; return 10; }
    8
}
/// $051462: jsr $5b6.l
pub fn i_051462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51468;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x51468);
    cpu.pc = 0x5B6;
    16
}
/// $051468: jmp $518.l
pub fn i_051468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5146E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $05146E: lea.l $ffff.w, a0
pub fn i_05146e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51472;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $051472: move.l a0, $4c(a6)
pub fn i_051472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51476;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
// ===== JsrAbsThunk_051476  ($051476, 8 B, thunks/jsr_abs_thunks.c)
/// $051476: jsr $283ca.l
pub fn i_051476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5147C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5147C);
    cpu.pc = 0x283CA;
    16
}
/// $05147C: rts
pub fn i_05147c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5147E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Jsr5B6ThenJmpScheduler_05147e  ($05147E, 14 B, kernel/jsr5b6_jmp518.c)
/// $05147E: jsr $5b6.l
pub fn i_05147e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51484;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x51484);
    cpu.pc = 0x5B6;
    16
}
/// $051484: jmp $518.l
pub fn i_051484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5148A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $05148A: rts
pub fn i_05148a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5148C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $051530: move.w #$18f, d1
pub fn i_051530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51534;
    cpu.ir = 0x323C;
    let v1: u32 = 0x18F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $051534: jsr $236e.l
pub fn i_051534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5153A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5153A);
    cpu.pc = 0x236E;
    16
}
/// $05153A: lea.l $2967d6.l, a0
pub fn i_05153a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51540;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2967D6;
    20
}
/// $051540: jsr $28cd4.l
pub fn i_051540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51546;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x51546);
    cpu.pc = 0x28CD4;
    16
}
/// $051546: lea.l $297000.l, a0
pub fn i_051546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5154C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x297000;
    20
}
/// $05154C: jsr $799de.l
pub fn i_05154c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51552;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x51552);
    cpu.pc = 0x799DE;
    16
}
/// $051552: move.w d0, d1
pub fn i_051552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51554;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $051554: move.w $34(a6), d0
pub fn i_051554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51558;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $051558: jsr $13c0e.l
pub fn i_051558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5155E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5155E);
    cpu.pc = 0x13C0E;
    16
}
/// $05155E: move.w d1, $28(a6)
pub fn i_05155e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51562;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $051562: move.w d2, $2a(a6)
pub fn i_051562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51566;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $051566: lea.l $5148c(pc), a0
pub fn i_051566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5156A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x5148C;
    16
}
/// $05156A: move.l a0, $4c(a6)
pub fn i_05156a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5156E;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $05156E: jsr $283ca.l
pub fn i_05156e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51574;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x51574);
    cpu.pc = 0x283CA;
    16
}
/// $051574: lea.l $5157a(pc), a1
pub fn i_051574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51578;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5157A;
    16
}
/// $051578: move.l a1, (a6)
pub fn i_051578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5157A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05157A: jsr $502a6(pc)
pub fn i_05157a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5157E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5157E);
    cpu.pc = 0x502A6;
    16
}
/// $05157E: jsr $28d70.l
pub fn i_05157e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51584;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x51584);
    cpu.pc = 0x28D70;
    16
}
/// $051584: jsr $283d8.l
pub fn i_051584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5158A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5158A);
    cpu.pc = 0x283D8;
    16
}
/// $05158A: bcs.w $515b0
pub fn i_05158a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5158E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x515B0; return 10; }
    8
}
/// $05158E: move.w $22(a6), d0
pub fn i_05158e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51592;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $051592: addi.w #$10, d0
pub fn i_051592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51596;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $051596: cmpi.w #$160, d0
pub fn i_051596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5159A;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x160, Size::Word, false);
    8
}
/// $05159A: bcc.w $515f4
pub fn i_05159a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5159E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x515F4; return 10; }
    8
}
/// $05159E: move.w $24(a6), d0
pub fn i_05159e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x515A2;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0515A2: subi.w #$f0, d0
pub fn i_0515a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x515A6;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0xF0, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0515A6: cmpi.w #$120, d0
pub fn i_0515a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x515AA;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x120, Size::Word, false);
    8
}
/// $0515AA: bcc.w $515f4
pub fn i_0515aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x515AE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x515F4; return 10; }
    8
}
/// $0515AE: rts
pub fn i_0515ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x515B0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0515B0: jsr $236e.l
pub fn i_0515b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x515B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x515B6);
    cpu.pc = 0x236E;
    16
}
/// $0515B6: move.w $2a(a6), d0
pub fn i_0515b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x515BA;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0515BA: bne.w $515ce
pub fn i_0515ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x515BE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x515CE; return 10; }
    8
}
/// $0515BE: lea.l $296d26.l, a0
pub fn i_0515be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x515C4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x296D26;
    20
}
/// $0515C4: jsr $28cd4.l
pub fn i_0515c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x515CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x515CA);
    cpu.pc = 0x28CD4;
    16
}
/// $0515CA: bra.w $515da
pub fn i_0515ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x515CE;
    cpu.ir = 0x6000;
    cpu.pc = 0x515DA;
    10
}
/// $0515CE: lea.l $296d78.l, a0
pub fn i_0515ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x515D4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x296D78;
    20
}
/// $0515D4: jsr $28cd4.l
pub fn i_0515d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x515DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x515DA);
    cpu.pc = 0x28CD4;
    16
}
/// $0515DA: clr.w $28(a6)
pub fn i_0515da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x515DE;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0515DE: clr.w $2a(a6)
pub fn i_0515de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x515E2;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0515E2: lea.l $515e8(pc), a1
pub fn i_0515e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x515E6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x515E8;
    16
}
/// $0515E6: move.l a1, (a6)
pub fn i_0515e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x515E8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0515E8: jsr $28d70.l
pub fn i_0515e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x515EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x515EE);
    cpu.pc = 0x28D70;
    16
}
/// $0515EE: bcs.w $515f4
pub fn i_0515ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x515F2;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x515F4; return 10; }
    8
}
/// $0515F2: bra.b $5158e
pub fn i_0515f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x515F4;
    cpu.ir = 0x609A;
    cpu.pc = 0x5158E;
    10
}
/// $0515F4: jmp $518.l
pub fn i_0515f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x515FA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Rts_0515fa  ($0515FA, 2 B, enemies/bosses/allen_oneil_04fa58.s)
/// $0515FA: rts
pub fn i_0515fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x515FC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SlotPrioCheck_0515fc  ($0515FC, 16 B, enemies/bosses/allen_oneil_04fa58.s)
/// $0515FC: movea.l $8(a6), a1
pub fn i_0515fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51600;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $051600: move.b $10(a6), d0
pub fn i_051600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51604;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $051604: cmp.b $10(a1), d0
pub fn i_051604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51608;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $051608: bcs.w $51612
pub fn i_051608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5160C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x51612; return 10; }
    8
}
// ===== ClearXN_05160c  ($05160C, 6 B, thunks/ccr_helpers.c)
/// $05160C: andi.b #$ee, ccr
pub fn i_05160c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51610;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $051610: rts
pub fn i_051610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51612;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_051612  ($051612, 6 B, thunks/ccr_helpers.c)
/// $051612: ori.b #$11, ccr
pub fn i_051612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51616;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $051616: rts
pub fn i_051616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51618;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05162C: move.b #$1, $106ed2.l
pub fn i_05162c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51634;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106ED2, v1 as u8);
    16
}
/// $051634: lea.l $98288.l, a1
pub fn i_051634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5163A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x98288;
    20
}
/// $05163A: jsr $4ae.l
pub fn i_05163a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51640;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x51640);
    cpu.pc = 0x4AE;
    16
}
/// $051640: move.l #$5164a, $98(a0)
pub fn i_051640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51648;
    cpu.ir = 0x217C;
    let v1: u32 = 0x5164A;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $051648: rts
pub fn i_051648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5164A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MemCard_LoadScene_05164a  ($05164A, 22 B, enemies/bosses/allen_oneil_04fa58.s)
/// $05164A: lea.l $2970f8.l, a0
pub fn i_05164a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51650;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2970F8;
    20
}
/// $051650: jsr $43562.l
pub fn i_051650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51656;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x51656);
    cpu.pc = 0x43562;
    16
}
/// $051656: moveq #$0, d0
pub fn i_051656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51658;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $051658: moveq #$0, d1
pub fn i_051658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5165A;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05165A: jmp $437da.l
pub fn i_05165a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51660;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x437DA;
    8
}
// ===== MemCard_DetectAndSave_051660  ($051660, 46 B, enemies/bosses/allen_oneil_04fa58.s)
/// $051660: jsr $9826e.l
pub fn i_051660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51666;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x51666);
    cpu.pc = 0x9826E;
    16
}
/// $051666: bcs.w $51674
pub fn i_051666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5166A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x51674; return 10; }
    8
}
/// $05166A: clr.b $106ed2.l
pub fn i_05166a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51670;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106ED2) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106ED2, r2 as u8);
    16
}
/// $051670: bra.w $51694
pub fn i_051670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51674;
    cpu.ir = 0x6000;
    cpu.pc = 0x51694;
    10
}
/// $051674: move.b #$1, $106ed2.l
pub fn i_051674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5167C;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106ED2, v1 as u8);
    16
}
/// $05167C: jsr $5164a(pc)
pub fn i_05167c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51680;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x51680);
    cpu.pc = 0x5164A;
    16
}
/// $051680: move.b #$ff, $10e3b6.l
pub fn i_051680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51688;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E3B6, v1 as u8);
    16
}
/// $051688: lea.l $9840c.l, a1
pub fn i_051688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5168E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x9840C;
    20
}
// ===== JsrAbsThunk_05168e  ($05168E, 8 B, thunks/jsr_abs_thunks.c)
/// $05168E: jsr $4ae.l
pub fn i_05168e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51694;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x51694);
    cpu.pc = 0x4AE;
    16
}
/// $051694: rts
pub fn i_051694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51696;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MemCard_CopySlotFlag_051696  ($051696, 14 B, enemies/bosses/allen_oneil_04fa58.s)
/// $051696: move.b $10e3b7.l, d0
pub fn i_051696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5169C;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10E3B7) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $05169C: move.b d0, $106ed0.l
pub fn i_05169c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x516A2;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106ED0, v1 as u8);
    16
}
/// $0516A2: rts
pub fn i_0516a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x516A4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MemCard_IsPending_0516a4  ($0516A4, 10 B, enemies/bosses/allen_oneil_04fa58.s)
/// $0516A4: tst.b $10e3b6.l
pub fn i_0516a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x516AA;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10E3B6) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $0516AA: bne.w $516b4
pub fn i_0516aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x516AE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x516B4; return 10; }
    8
}
// ===== ClearC_0516ae  ($0516AE, 6 B, thunks/ccr_helpers.c)
/// $0516AE: andi.b #$fe, ccr
pub fn i_0516ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x516B2;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $0516B2: rts
pub fn i_0516b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x516B4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetC_0516b4  ($0516B4, 6 B, thunks/ccr_helpers.c)
/// $0516B4: ori.b #$1, ccr
pub fn i_0516b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x516B8;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $0516B8: rts
pub fn i_0516b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x516BA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Players_SnapshotMods_0516ba  ($0516BA, 32 B, enemies/bosses/allen_oneil_04fa58.s)
/// $0516BA: lea.l $10e3a2.l, a4
pub fn i_0516ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x516C0;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0x10E3A2;
    20
}
/// $0516C0: move.b $106ed0.l, $15(a4)
pub fn i_0516c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x516C8;
    cpu.ir = 0x1979;
    let v2 = bus.read8(0x106ED0) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[4].wrapping_add(0x15);
    bus.write8(ad3, v2 as u8);
    24
}
/// $0516C8: move.b $10fdb6.l, $16(a4)
pub fn i_0516c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x516D0;
    cpu.ir = 0x1979;
    let v2 = bus.read8(0x10FDB6) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[4].wrapping_add(0x16);
    bus.write8(ad3, v2 as u8);
    24
}
/// $0516D0: move.b $10fdb7.l, $17(a4)
pub fn i_0516d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x516D8;
    cpu.ir = 0x1979;
    let v2 = bus.read8(0x10FDB7) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[4].wrapping_add(0x17);
    bus.write8(ad3, v2 as u8);
    24
}
/// $0516D8: rts
pub fn i_0516d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x516DA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Counters_AddSaturate_0516da  ($0516DA, 50 B, enemies/bosses/allen_oneil_04fa58.s)
/// $0516DA: movem.w d1-d2, -(a7)
pub fn i_0516da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x516DE;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[2] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[1] as u16);
    cpu.a[7] = a;
    20
}
/// $0516DE: add.b $10e3bc.l, d1
pub fn i_0516de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x516E4;
    cpu.ir = 0xD239;
    let v2 = bus.read8(0x10E3BC) as u32;
    let h1 = cpu.d[1] & 0xFF;
    let r = add_with_flags(cpu, h1, v2, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    16
}
/// $0516E4: bcc.w $516ec
pub fn i_0516e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x516E8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x516EC; return 10; }
    8
}
/// $0516E8: move.b #$ff, d1
pub fn i_0516e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x516EC;
    cpu.ir = 0x123C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0516EC: move.b d1, $10e3bc.l
pub fn i_0516ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x516F2;
    cpu.ir = 0x13C1;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E3BC, v1 as u8);
    16
}
/// $0516F2: add.b $10e3bd.l, d2
pub fn i_0516f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x516F8;
    cpu.ir = 0xD439;
    let v2 = bus.read8(0x10E3BD) as u32;
    let h1 = cpu.d[2] & 0xFF;
    let r = add_with_flags(cpu, h1, v2, Size::Byte, true);
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r & 0xFF);
    16
}
/// $0516F8: bcc.w $51700
pub fn i_0516f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x516FC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x51700; return 10; }
    8
}
/// $0516FC: move.b #$ff, d2
pub fn i_0516fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51700;
    cpu.ir = 0x143C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $051700: move.b d2, $10e3bd.l
pub fn i_051700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51706;
    cpu.ir = 0x13C2;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E3BD, v1 as u8);
    16
}
/// $051706: movem.w (a7)+, d1-d2
pub fn i_051706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5170A;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[1] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[2] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $05170A: rts
pub fn i_05170a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5170C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_CounterSaturateByte_05170C  ($05170C, 64 B, player/counter_saturate_05170c.s)
/// $05170C: lea.l $10e3a2.l, a4
pub fn i_05170c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51712;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0x10E3A2;
    20
}
/// $051712: cmpa.l #$106e94, a1
pub fn i_051712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51718;
    cpu.ir = 0xB3FC;
    let v1: u32 = 0x106E94;
    let h1 = cpu.a[1];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $051718: bne.w $51726
pub fn i_051718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5171C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x51726; return 10; }
    8
}
/// $05171C: adda.l #$18, a4
pub fn i_05171c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51722;
    cpu.ir = 0xD9FC;
    let v1: u32 = 0x18;
    cpu.a[4] = cpu.a[4].wrapping_add(v1);
    8
}
/// $051722: bra.w $5173c
pub fn i_051722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51726;
    cpu.ir = 0x6000;
    cpu.pc = 0x5173C;
    10
}
/// $051726: cmpa.l #$106e9c, a1
pub fn i_051726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5172C;
    cpu.ir = 0xB3FC;
    let v1: u32 = 0x106E9C;
    let h1 = cpu.a[1];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05172C: bne.w $5173a
pub fn i_05172c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51730;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5173A; return 10; }
    8
}
/// $051730: adda.l #$19, a4
pub fn i_051730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51736;
    cpu.ir = 0xD9FC;
    let v1: u32 = 0x19;
    cpu.a[4] = cpu.a[4].wrapping_add(v1);
    8
}
/// $051736: bra.w $5173c
pub fn i_051736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5173A;
    cpu.ir = 0x6000;
    cpu.pc = 0x5173C;
    10
}
/// $05173A: rts
pub fn i_05173a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5173C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05173C: move.b (a4), d0
pub fn i_05173c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5173E;
    cpu.ir = 0x1014;
    let v2 = bus.read8(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $05173E: addq.b #$1, d0
pub fn i_05173e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51740;
    cpu.ir = 0x5200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $051740: bcc.w $51748
pub fn i_051740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51744;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x51748; return 10; }
    8
}
/// $051744: move.b #$ff, d0
pub fn i_051744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51748;
    cpu.ir = 0x103C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $051748: move.b d0, (a4)
pub fn i_051748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5174A;
    cpu.ir = 0x1880;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[4], v1 as u8);
    8
}
/// $05174A: rts
pub fn i_05174a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5174C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Players_ActiveMask_05174c  ($05174C, 54 B, enemies/bosses/allen_oneil_04fa58.s)
/// $05174C: lea.l $10e3a2.l, a0
pub fn i_05174c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51752;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E3A2;
    20
}
/// $051752: clr.b d0
pub fn i_051752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51754;
    cpu.ir = 0x4200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $051754: clr.b d1
pub fn i_051754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51756;
    cpu.ir = 0x4201;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $051756: clr.b d2
pub fn i_051756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51758;
    cpu.ir = 0x4202;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    4
}
/// $051758: cmpi.b #$1, $16(a0)
pub fn i_051758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5175E;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x16);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $05175E: bne.w $5176a
pub fn i_05175e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51762;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5176A; return 10; }
    8
}
/// $051762: move.b #$ff, d0
pub fn i_051762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51766;
    cpu.ir = 0x103C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $051766: move.b $18(a0), d2
pub fn i_051766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5176A;
    cpu.ir = 0x1428;
    let ad2 = cpu.a[0].wrapping_add(0x18);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05176A: cmpi.b #$1, $17(a0)
pub fn i_05176a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51770;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x17);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $051770: bne.w $5177c
pub fn i_051770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51774;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5177C; return 10; }
    8
}
/// $051774: move.b #$ff, d1
pub fn i_051774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51778;
    cpu.ir = 0x123C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $051778: add.b $19(a0), d2
pub fn i_051778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5177C;
    cpu.ir = 0xD428;
    let ad2 = cpu.a[0].wrapping_add(0x19);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[2] & 0xFF;
    let r = add_with_flags(cpu, h1, v3, Size::Byte, true);
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r & 0xFF);
    12
}
/// $05177C: and.b d0, d1
pub fn i_05177c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5177E;
    cpu.ir = 0xC200;
    let v1 = cpu.d[0] & 0xFF;
    let r = (v1 & cpu.d[1]) & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05177E: bne.w $51788
pub fn i_05177e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51782;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x51788; return 10; }
    8
}
// ===== SetC_051782  ($051782, 6 B, thunks/ccr_helpers.c)
/// $051782: ori.b #$1, ccr
pub fn i_051782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51786;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $051786: rts
pub fn i_051786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51788;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearC_051788  ($051788, 6 B, thunks/ccr_helpers.c)
/// $051788: andi.b #$fe, ccr
pub fn i_051788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5178C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $05178C: rts
pub fn i_05178c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5178E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SlotPrioCheck_05178e  ($05178E, 16 B, enemies/bosses/allen_oneil_04fa58.s)
/// $05178E: movea.l $8(a6), a1
pub fn i_05178e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51792;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $051792: move.b $10(a6), d0
pub fn i_051792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51796;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $051796: cmp.b $10(a1), d0
pub fn i_051796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5179A;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $05179A: bcs.w $517a4
pub fn i_05179a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5179E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x517A4; return 10; }
    8
}
// ===== ClearXN_05179e  ($05179E, 6 B, thunks/ccr_helpers.c)
/// $05179E: andi.b #$ee, ccr
pub fn i_05179e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x517A2;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0517A2: rts
pub fn i_0517a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x517A4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_0517a4  ($0517A4, 6 B, thunks/ccr_helpers.c)
/// $0517A4: ori.b #$11, ccr
pub fn i_0517a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x517A8;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0517A8: rts
pub fn i_0517a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x517AA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_SetIndexFromParent_0517aa  ($0517AA, 84 B, enemies/bosses/allen_oneil_04fa58.s)
/// $0517AA: movea.l $c(a6), a0
pub fn i_0517aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x517AE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0517AE: cmpa.l #$100300, a0
pub fn i_0517ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x517B4;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0x100300;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $0517B4: bne.w $517c6
pub fn i_0517b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x517B8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x517C6; return 10; }
    8
}
/// $0517B8: move.b #$0, $68(a6)
pub fn i_0517b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x517BE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0517BE: move.b #$1, $6d(a6)
pub fn i_0517be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x517C4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x6D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0517C4: rts
pub fn i_0517c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x517C6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0517C6: cmpa.l #$1003a0, a0
pub fn i_0517c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x517CC;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0x1003A0;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $0517CC: bne.w $517de
pub fn i_0517cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x517D0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x517DE; return 10; }
    8
}
/// $0517D0: move.b #$1, $68(a6)
pub fn i_0517d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x517D6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0517D6: move.b #$2, $6d(a6)
pub fn i_0517d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x517DC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x6D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0517DC: rts
pub fn i_0517dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x517DE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0517DE: move.b #$ff, $6d(a6)
pub fn i_0517de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x517E4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x6D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0517E4: move.b $68(a6), d0
pub fn i_0517e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x517E8;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0517E8: cmpi.b #$ff, d0
pub fn i_0517e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x517EC;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $0517EC: beq.w $517fc
pub fn i_0517ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x517F0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x517FC; return 10; }
    8
}
/// $0517F0: nop
pub fn i_0517f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x517F2;
    cpu.ir = 0x4E71;
    4
}
/// $0517F2: nop
pub fn i_0517f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x517F4;
    cpu.ir = 0x4E71;
    4
}
/// $0517F4: cmpi.b #$ff, d0
pub fn i_0517f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x517F8;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $0517F8: nop
pub fn i_0517f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x517FA;
    cpu.ir = 0x4E71;
    4
}
/// $0517FA: trap #$f
pub fn i_0517fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x517FC;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $0517FC: rts
pub fn i_0517fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x517FE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CopyField68AndCall_0517FE  ($0517FE, 14 B, entity/copy_field68_call_0517fe.s)
/// $0517FE: move.b $68(a6), $68(a0)
pub fn i_0517fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51804;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x68);
    bus.write8(ad4, v3 as u8);
    20
}
/// $051804: jsr $5ccc8.l
pub fn i_051804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5180A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5180A);
    cpu.pc = 0x5CCC8;
    16
}
/// $05180A: rts
pub fn i_05180a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5180C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Clear8Bytes_05180c  ($05180C, 8 B, enemies/bosses/allen_oneil_04fa58.s)
/// $05180C: clr.l (a1)
pub fn i_05180c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5180E;
    cpu.ir = 0x4291;
    let ad1 = cpu.a[1];
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $05180E: clr.l $4(a1)
pub fn i_05180e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51812;
    cpu.ir = 0x42A9;
    let ad1 = cpu.a[1].wrapping_add(0x4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $051812: rts
pub fn i_051812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51814;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Nibbles_Unpack4Store_051814  ($051814, 20 B, enemies/bosses/allen_oneil_04fa58.s)
/// $051814: move.l a1, -(a7)
pub fn i_051814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51816;
    cpu.ir = 0x2F09;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $051816: movea.l a1, a0
pub fn i_051816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51818;
    cpu.ir = 0x2049;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $051818: lea.l $1081b6.l, a1
pub fn i_051818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5181E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1081B6;
    20
}
/// $05181E: move.l d0, (a1)
pub fn i_05181e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51820;
    cpu.ir = 0x2280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    bus.write32(cpu.a[1], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $051820: bsr.w $51828
pub fn i_051820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51824;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x51824);
    cpu.pc = 0x51828;
    18
}
/// $051824: movea.l (a7)+, a1
pub fn i_051824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51826;
    cpu.ir = 0x225F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    12
}
/// $051826: rts
pub fn i_051826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51828;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Nibbles_Unpack4_051828  ($051828, 58 B, enemies/bosses/allen_oneil_04fa58.s)
/// $051828: move.b (a1)+, d0
pub fn i_051828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5182A;
    cpu.ir = 0x1019;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05182A: move.b d0, d1
pub fn i_05182a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5182C;
    cpu.ir = 0x1200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05182C: lsr.b #$4, d1
pub fn i_05182c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5182E;
    cpu.ir = 0xE809;
    let h1 = cpu.d[1] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, false);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    14
}
/// $05182E: andi.b #$f, d0
pub fn i_05182e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51832;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $051832: move.b d1, (a0)+
pub fn i_051832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51834;
    cpu.ir = 0x10C1;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $051834: move.b d0, (a0)+
pub fn i_051834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51836;
    cpu.ir = 0x10C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $051836: move.b (a1)+, d0
pub fn i_051836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51838;
    cpu.ir = 0x1019;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    8
}
/// $051838: move.b d0, d1
pub fn i_051838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5183A;
    cpu.ir = 0x1200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05183A: lsr.b #$4, d1
pub fn i_05183a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5183C;
    cpu.ir = 0xE809;
    let h1 = cpu.d[1] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, false);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    14
}
/// $05183C: andi.b #$f, d0
pub fn i_05183c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51840;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $051840: move.b d1, (a0)+
pub fn i_051840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51842;
    cpu.ir = 0x10C1;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $051842: move.b d0, (a0)+
pub fn i_051842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51844;
    cpu.ir = 0x10C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $051844: move.b (a1)+, d0
pub fn i_051844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51846;
    cpu.ir = 0x1019;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    8
}
/// $051846: move.b d0, d1
pub fn i_051846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51848;
    cpu.ir = 0x1200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $051848: lsr.b #$4, d1
pub fn i_051848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5184A;
    cpu.ir = 0xE809;
    let h1 = cpu.d[1] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, false);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    14
}
/// $05184A: andi.b #$f, d0
pub fn i_05184a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5184E;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05184E: move.b d1, (a0)+
pub fn i_05184e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51850;
    cpu.ir = 0x10C1;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $051850: move.b d0, (a0)+
pub fn i_051850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51852;
    cpu.ir = 0x10C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $051852: move.b (a1)+, d0
pub fn i_051852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51854;
    cpu.ir = 0x1019;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    8
}
/// $051854: move.b d0, d1
pub fn i_051854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51856;
    cpu.ir = 0x1200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $051856: lsr.b #$4, d1
pub fn i_051856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51858;
    cpu.ir = 0xE809;
    let h1 = cpu.d[1] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, false);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    14
}
/// $051858: andi.b #$f, d0
pub fn i_051858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5185C;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05185C: move.b d1, (a0)+
pub fn i_05185c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5185E;
    cpu.ir = 0x10C1;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $05185E: move.b d0, (a0)+
pub fn i_05185e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51860;
    cpu.ir = 0x10C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $051860: rts
pub fn i_051860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51862;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Nibbles_Pack8_051862  ($051862, 42 B, enemies/bosses/allen_oneil_04fa58.s)
/// $051862: move.b (a0)+, d0
pub fn i_051862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51864;
    cpu.ir = 0x1018;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    8
}
/// $051864: move.b (a0)+, d1
pub fn i_051864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51866;
    cpu.ir = 0x1218;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    8
}
/// $051866: lsl.b #$4, d0
pub fn i_051866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51868;
    cpu.ir = 0xE908;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    14
}
/// $051868: or.b d1, d0
pub fn i_051868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5186A;
    cpu.ir = 0x8001;
    let v1 = cpu.d[1] & 0xFF;
    let r = (v1 | cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05186A: move.b d0, (a1)+
pub fn i_05186a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5186C;
    cpu.ir = 0x12C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $05186C: move.b (a0)+, d0
pub fn i_05186c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5186E;
    cpu.ir = 0x1018;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05186E: move.b (a0)+, d1
pub fn i_05186e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51870;
    cpu.ir = 0x1218;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    8
}
/// $051870: lsl.b #$4, d0
pub fn i_051870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51872;
    cpu.ir = 0xE908;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    14
}
/// $051872: or.b d1, d0
pub fn i_051872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51874;
    cpu.ir = 0x8001;
    let v1 = cpu.d[1] & 0xFF;
    let r = (v1 | cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $051874: move.b d0, (a1)+
pub fn i_051874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51876;
    cpu.ir = 0x12C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $051876: move.b (a0)+, d0
pub fn i_051876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51878;
    cpu.ir = 0x1018;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    8
}
/// $051878: move.b (a0)+, d1
pub fn i_051878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5187A;
    cpu.ir = 0x1218;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05187A: lsl.b #$4, d0
pub fn i_05187a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5187C;
    cpu.ir = 0xE908;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    14
}
/// $05187C: or.b d1, d0
pub fn i_05187c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5187E;
    cpu.ir = 0x8001;
    let v1 = cpu.d[1] & 0xFF;
    let r = (v1 | cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05187E: move.b d0, (a1)+
pub fn i_05187e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51880;
    cpu.ir = 0x12C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $051880: move.b (a0)+, d0
pub fn i_051880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51882;
    cpu.ir = 0x1018;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    8
}
/// $051882: move.b (a0)+, d1
pub fn i_051882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51884;
    cpu.ir = 0x1218;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    8
}
/// $051884: lsl.b #$4, d0
pub fn i_051884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51886;
    cpu.ir = 0xE908;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    14
}
/// $051886: or.b d1, d0
pub fn i_051886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51888;
    cpu.ir = 0x8001;
    let v1 = cpu.d[1] & 0xFF;
    let r = (v1 | cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $051888: move.b d0, (a1)+
pub fn i_051888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5188A;
    cpu.ir = 0x12C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $05188A: rts
pub fn i_05188a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5188C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_DispatchStateBySlot_051914  ($051914, 134 B, player/dispatch_cluster_051914.s)
/// $051914: cmpa.l #$100440, a1
pub fn i_051914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5191A;
    cpu.ir = 0xB3FC;
    let v1: u32 = 0x100440;
    let h1 = cpu.a[1];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05191A: beq.w $51998
pub fn i_05191a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5191E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x51998; return 10; }
    8
}
/// $05191E: cmpa.l #$1004e0, a1
pub fn i_05191e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51924;
    cpu.ir = 0xB3FC;
    let v1: u32 = 0x1004E0;
    let h1 = cpu.a[1];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $051924: beq.w $51998
pub fn i_051924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51928;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x51998; return 10; }
    8
}
/// $051928: cmpa.l #$100580, a1
pub fn i_051928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5192E;
    cpu.ir = 0xB3FC;
    let v1: u32 = 0x100580;
    let h1 = cpu.a[1];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05192E: beq.w $51998
pub fn i_05192e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51932;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x51998; return 10; }
    8
}
/// $051932: move.b $58(a1), d1
pub fn i_051932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51936;
    cpu.ir = 0x1229;
    let ad2 = cpu.a[1].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $051936: beq.w $51998
pub fn i_051936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5193A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x51998; return 10; }
    8
}
/// $05193A: move.b $68(a6), d0
pub fn i_05193a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5193E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05193E: bmi.w $51998
pub fn i_05193e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51942;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x51998; return 10; }
    8
}
/// $051942: move.b d0, $6e(a1)
pub fn i_051942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51946;
    cpu.ir = 0x1340;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x6E);
    bus.write8(ad2, v1 as u8);
    12
}
/// $051946: btst.b #$1, $12(a1)
pub fn i_051946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5194C;
    cpu.ir = 0x829;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x12);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05194C: beq.w $51998
pub fn i_05194c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51950;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x51998; return 10; }
    8
}
/// $051950: movem.l a0-a2, -(a7)
pub fn i_051950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51954;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $051954: tst.b d0
pub fn i_051954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51956;
    cpu.ir = 0x4A00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $051956: bne.w $51964
pub fn i_051956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5195A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x51964; return 10; }
    8
}
/// $05195A: lea.l $106e94.l, a0
pub fn i_05195a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51960;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x106E94;
    20
}
/// $051960: bra.w $5196a
pub fn i_051960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51964;
    cpu.ir = 0x6000;
    cpu.pc = 0x5196A;
    10
}
/// $051964: lea.l $106e9c.l, a0
pub fn i_051964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5196A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x106E9C;
    20
}
/// $05196A: lea.l $1081b6.l, a1
pub fn i_05196a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51970;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1081B6;
    20
}
/// $051970: move.w d1, -(a7)
pub fn i_051970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51972;
    cpu.ir = 0x3F01;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $051972: bsr.w $51862
pub fn i_051972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51976;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x51976);
    cpu.pc = 0x51862;
    18
}
/// $051976: move.w (a7)+, d1
pub fn i_051976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51978;
    cpu.ir = 0x321F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $051978: lea.l $5188c(pc), a2
pub fn i_051978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5197C;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0x5188C;
    16
}
/// $05197C: andi.w #$1f, d1
pub fn i_05197c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51980;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $051980: add.w d1, d1
pub fn i_051980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51982;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $051982: add.w d1, d1
pub fn i_051982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51984;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $051984: addq.w #$4, d1
pub fn i_051984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51986;
    cpu.ir = 0x5841;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $051986: adda.w d1, a2
pub fn i_051986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51988;
    cpu.ir = 0xD4C1;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.a[2] = cpu.a[2].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $051988: bsr.w $51a10
pub fn i_051988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5198C;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5198C);
    cpu.pc = 0x51A10;
    18
}
/// $05198C: suba.w #$8, a0
pub fn i_05198c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51990;
    cpu.ir = 0x90FC;
    let v1: u32 = 0x8;
    cpu.a[0] = cpu.a[0].wrapping_sub((v1 as u16 as i16 as i32 as u32));
    8
}
/// $051990: bsr.w $51828
pub fn i_051990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51994;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x51994);
    cpu.pc = 0x51828;
    18
}
/// $051994: movem.l (a7)+, a0-a2
pub fn i_051994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51998;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $051998: rts
pub fn i_051998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5199A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_BuildTableAddrOnly_05199A  ($05199A, 36 B, player/dispatch_cluster_051914.s)
/// $05199A: move.b $58(a1), d1
pub fn i_05199a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5199E;
    cpu.ir = 0x1229;
    let ad2 = cpu.a[1].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05199E: move.b $68(a6), d0
pub fn i_05199e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x519A2;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0519A2: movem.l a0-a2, -(a7)
pub fn i_0519a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x519A6;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $0519A6: lea.l $5188c(pc), a2
pub fn i_0519a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x519AA;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0x5188C;
    16
}
/// $0519AA: andi.w #$1f, d1
pub fn i_0519aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x519AE;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0519AE: add.w d1, d1
pub fn i_0519ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x519B0;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0519B0: add.w d1, d1
pub fn i_0519b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x519B2;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0519B2: addq.w #$4, d1
pub fn i_0519b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x519B4;
    cpu.ir = 0x5841;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0519B4: adda.w d1, a2
pub fn i_0519b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x519B6;
    cpu.ir = 0xD4C1;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.a[2] = cpu.a[2].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $0519B6: move.l a2, d6
pub fn i_0519b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x519B8;
    cpu.ir = 0x2C0A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    cpu.d[6] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0519B8: movem.l (a7)+, a0-a2
pub fn i_0519b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x519BC;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $0519BC: rts
pub fn i_0519bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x519BE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_StateDispatch_0519BE  ($0519BE, 82 B, player/state_dispatch_0519be.s)
/// $0519BE: btst.b #$1, $12(a6)
pub fn i_0519be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x519C4;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x12);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0519C4: bne.w $51a0e
pub fn i_0519c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x519C8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x51A0E; return 10; }
    8
}
/// $0519C8: move.b $6e(a6), d0
pub fn i_0519c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x519CC;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x6E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0519CC: bmi.w $51a0e
pub fn i_0519cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x519D0;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x51A0E; return 10; }
    8
}
/// $0519D0: bne.w $519de
pub fn i_0519d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x519D4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x519DE; return 10; }
    8
}
/// $0519D4: lea.l $106e94.l, a0
pub fn i_0519d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x519DA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x106E94;
    20
}
/// $0519DA: bra.w $519e4
pub fn i_0519da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x519DE;
    cpu.ir = 0x6000;
    cpu.pc = 0x519E4;
    10
}
/// $0519DE: lea.l $106e9c.l, a0
pub fn i_0519de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x519E4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x106E9C;
    20
}
/// $0519E4: lea.l $1081b6.l, a1
pub fn i_0519e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x519EA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1081B6;
    20
}
/// $0519EA: bsr.w $51862
pub fn i_0519ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x519EE;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x519EE);
    cpu.pc = 0x51862;
    18
}
/// $0519EE: lea.l $5188c(pc), a2
pub fn i_0519ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x519F2;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0x5188C;
    16
}
/// $0519F2: move.b $58(a6), d1
pub fn i_0519f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x519F6;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0519F6: andi.w #$1f, d1
pub fn i_0519f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x519FA;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0519FA: add.w d1, d1
pub fn i_0519fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x519FC;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0519FC: add.w d1, d1
pub fn i_0519fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x519FE;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0519FE: addq.w #$4, d1
pub fn i_0519fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A00;
    cpu.ir = 0x5841;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $051A00: adda.w d1, a2
pub fn i_051a00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A02;
    cpu.ir = 0xD4C1;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.a[2] = cpu.a[2].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $051A02: bsr.w $51a10
pub fn i_051a02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A06;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x51A06);
    cpu.pc = 0x51A10;
    18
}
/// $051A06: suba.w #$8, a0
pub fn i_051a06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A0A;
    cpu.ir = 0x90FC;
    let v1: u32 = 0x8;
    cpu.a[0] = cpu.a[0].wrapping_sub((v1 as u16 as i16 as i32 as u32));
    8
}
/// $051A0A: bsr.w $51828
pub fn i_051a0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A0E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x51A0E);
    cpu.pc = 0x51828;
    18
}
/// $051A0E: rts
pub fn i_051a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A10;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== BCD_AddClamp99999999_051A10  ($051A10, 24 B, kernel/math_bcd_add_clamp_051a10.s)
/// $051A10: andi.b #$ef, ccr
pub fn i_051a10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A14;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xF);
    20
}
/// $051A14: abcd.b -(a2), -(a1)
pub fn i_051a14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A16;
    cpu.ir = 0xC30A;
    cpu.a[2] = cpu.a[2].wrapping_sub(1);
    let s = bus.read8(cpu.a[2]);
    cpu.a[1] = cpu.a[1].wrapping_sub(1);
    let d = bus.read8(cpu.a[1]);
    let x = cpu.sr.get(StatusRegister::X) as u8;
    let (r, carry, vf) = abcd_calc(d, s, x);
    if r != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::N, r & 0x80 != 0); cpu.sr.set(StatusRegister::V, vf);
    cpu.sr.set(StatusRegister::C, carry); cpu.sr.set(StatusRegister::X, carry);
    bus.write8(cpu.a[1], r);
    18
}
/// $051A16: abcd.b -(a2), -(a1)
pub fn i_051a16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A18;
    cpu.ir = 0xC30A;
    cpu.a[2] = cpu.a[2].wrapping_sub(1);
    let s = bus.read8(cpu.a[2]);
    cpu.a[1] = cpu.a[1].wrapping_sub(1);
    let d = bus.read8(cpu.a[1]);
    let x = cpu.sr.get(StatusRegister::X) as u8;
    let (r, carry, vf) = abcd_calc(d, s, x);
    if r != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::N, r & 0x80 != 0); cpu.sr.set(StatusRegister::V, vf);
    cpu.sr.set(StatusRegister::C, carry); cpu.sr.set(StatusRegister::X, carry);
    bus.write8(cpu.a[1], r);
    18
}
/// $051A18: abcd.b -(a2), -(a1)
pub fn i_051a18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A1A;
    cpu.ir = 0xC30A;
    cpu.a[2] = cpu.a[2].wrapping_sub(1);
    let s = bus.read8(cpu.a[2]);
    cpu.a[1] = cpu.a[1].wrapping_sub(1);
    let d = bus.read8(cpu.a[1]);
    let x = cpu.sr.get(StatusRegister::X) as u8;
    let (r, carry, vf) = abcd_calc(d, s, x);
    if r != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::N, r & 0x80 != 0); cpu.sr.set(StatusRegister::V, vf);
    cpu.sr.set(StatusRegister::C, carry); cpu.sr.set(StatusRegister::X, carry);
    bus.write8(cpu.a[1], r);
    18
}
/// $051A1A: abcd.b -(a2), -(a1)
pub fn i_051a1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A1C;
    cpu.ir = 0xC30A;
    cpu.a[2] = cpu.a[2].wrapping_sub(1);
    let s = bus.read8(cpu.a[2]);
    cpu.a[1] = cpu.a[1].wrapping_sub(1);
    let d = bus.read8(cpu.a[1]);
    let x = cpu.sr.get(StatusRegister::X) as u8;
    let (r, carry, vf) = abcd_calc(d, s, x);
    if r != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::N, r & 0x80 != 0); cpu.sr.set(StatusRegister::V, vf);
    cpu.sr.set(StatusRegister::C, carry); cpu.sr.set(StatusRegister::X, carry);
    bus.write8(cpu.a[1], r);
    18
}
/// $051A1C: bcc.w $51a26
pub fn i_051a1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A20;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x51A26; return 10; }
    8
}
/// $051A20: move.l #$99999999, (a1)
pub fn i_051a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A26;
    cpu.ir = 0x22BC;
    let v1: u32 = 0x99999999;
    bus.write32(cpu.a[1], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $051A26: rts
pub fn i_051a26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A28;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_DispatchOrLoadFromSlot50_051A28  ($051A28, 94 B, player/dispatch_cluster_051914.s)
/// $051A28: movem.l d0/a0-a2, -(a7)
pub fn i_051a28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A2C;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $051A2C: movea.l $50(a6), a0
pub fn i_051a2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A30;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x50);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $051A30: cmpa.l #$ffffffff, a0
pub fn i_051a30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A36;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $051A36: beq.w $51a80
pub fn i_051a36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A3A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x51A80; return 10; }
    8
}
/// $051A3A: move.b $68(a0), $6e(a6)
pub fn i_051a3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A40;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x68);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x6E);
    bus.write8(ad4, v3 as u8);
    20
}
/// $051A40: bra.w $51a48
pub fn i_051a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A44;
    cpu.ir = 0x6000;
    cpu.pc = 0x51A48;
    10
}
/// $051A44: movem.l d0/a0-a2, -(a7)
pub fn i_051a44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A48;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $051A48: tst.b $6e(a6)
pub fn i_051a48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A4C;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x6E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $051A4C: bmi.w $51a80
pub fn i_051a4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A50;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x51A80; return 10; }
    8
}
/// $051A50: bne.w $51a5e
pub fn i_051a50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A54;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x51A5E; return 10; }
    8
}
/// $051A54: lea.l $106e94.l, a0
pub fn i_051a54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A5A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x106E94;
    20
}
/// $051A5A: bra.w $51a64
pub fn i_051a5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A5E;
    cpu.ir = 0x6000;
    cpu.pc = 0x51A64;
    10
}
/// $051A5E: lea.l $106e9c.l, a0
pub fn i_051a5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A64;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x106E9C;
    20
}
/// $051A64: lea.l $1081b6.l, a1
pub fn i_051a64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A6A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1081B6;
    20
}
/// $051A6A: lea.l $1081ba.l, a2
pub fn i_051a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A70;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x1081BA;
    20
}
/// $051A70: move.l d0, (a2)
pub fn i_051a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A72;
    cpu.ir = 0x2480;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    bus.write32(cpu.a[2], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $051A72: bsr.w $51862
pub fn i_051a72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A76;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x51A76);
    cpu.pc = 0x51862;
    18
}
/// $051A76: addq.w #$4, a2
pub fn i_051a76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A78;
    cpu.ir = 0x584A;
    cpu.a[2] = cpu.a[2].wrapping_add(4);
    8
}
/// $051A78: bsr.b $51a10
pub fn i_051a78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A7A;
    cpu.ir = 0x6196;
    cpu.push32(bus, 0x51A7A);
    cpu.pc = 0x51A10;
    18
}
/// $051A7A: subq.w #$8, a0
pub fn i_051a7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A7C;
    cpu.ir = 0x5148;
    cpu.a[0] = cpu.a[0].wrapping_sub(8);
    8
}
/// $051A7C: bsr.w $51828
pub fn i_051a7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A80;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x51A80);
    cpu.pc = 0x51828;
    18
}
/// $051A80: movem.l (a7)+, d0/a0-a2
pub fn i_051a80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A84;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $051A84: rts
pub fn i_051a84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A86;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_IncCounterAt7_051A86  ($051A86, 30 B, player/dispatch_cluster_051914.s)
/// $051A86: move.b $7(a1), d0
pub fn i_051a86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A8A;
    cpu.ir = 0x1029;
    let ad2 = cpu.a[1].wrapping_add(0x7);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $051A8A: addq.b #$1, d0
pub fn i_051a8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A8C;
    cpu.ir = 0x5200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $051A8C: cmpi.b #$a, d0
pub fn i_051a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A90;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xA, Size::Byte, false);
    8
}
/// $051A90: bcs.w $51a98
pub fn i_051a90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A94;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x51A98; return 10; }
    8
}
/// $051A94: move.b #$9, d0
pub fn i_051a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A98;
    cpu.ir = 0x103C;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $051A98: move.b d0, $7(a1)
pub fn i_051a98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51A9C;
    cpu.ir = 0x1340;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x7);
    bus.write8(ad2, v1 as u8);
    12
}
/// $051A9C: jsr $5170c.l
pub fn i_051a9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51AA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x51AA2);
    cpu.pc = 0x5170C;
    16
}
/// $051AA2: rts
pub fn i_051aa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51AA4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== CellMap_ReadPacked32_051aa4  ($051AA4, 26 B, video/cellmap_fix_overlay_palfade_051aa4.s)
/// $051AA4: movem.l a0-a1, -(a7)
pub fn i_051aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51AA8;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $051AA8: lea.l $1081b6.l, a1
pub fn i_051aa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51AAE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1081B6;
    20
}
/// $051AAE: bsr.w $51862
pub fn i_051aae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51AB2;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x51AB2);
    cpu.pc = 0x51862;
    18
}
/// $051AB2: suba.w #$4, a1
pub fn i_051ab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51AB6;
    cpu.ir = 0x92FC;
    let v1: u32 = 0x4;
    cpu.a[1] = cpu.a[1].wrapping_sub((v1 as u16 as i16 as i32 as u32));
    8
}
/// $051AB6: move.l (a1), d0
pub fn i_051ab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51AB8;
    cpu.ir = 0x2011;
    let v2 = bus.read32(cpu.a[1]);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $051AB8: movem.l (a7)+, a0-a1
pub fn i_051ab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51ABC;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $051ABC: rts
pub fn i_051abc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51ABE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_AllocAndInit_051ABE  ($051ABE, 94 B, video/cellmap_fix_overlay_palfade_051aa4.s)
/// $051ABE: move.w d0, $28(a0)
pub fn i_051abe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51AC2;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $051AC2: clr.l $e(a0)
pub fn i_051ac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51AC6;
    cpu.ir = 0x42A8;
    let ad1 = cpu.a[0].wrapping_add(0xE);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $051AC6: move.w (a1), $16(a0)
pub fn i_051ac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51ACA;
    cpu.ir = 0x3151;
    let v2 = bus.read16(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_add(0x16);
    bus.write16(ad3, v2 as u16);
    16
}
/// $051ACA: move.w $2(a1), $18(a0)
pub fn i_051aca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51AD0;
    cpu.ir = 0x3169;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x18);
    bus.write16(ad4, v3 as u16);
    20
}
/// $051AD0: move.w $4(a1), $1a(a0)
pub fn i_051ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51AD6;
    cpu.ir = 0x3169;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x1A);
    bus.write16(ad4, v3 as u16);
    20
}
/// $051AD6: move.w $6(a1), $1c(a0)
pub fn i_051ad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51ADC;
    cpu.ir = 0x3169;
    let ad2 = cpu.a[1].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x1C);
    bus.write16(ad4, v3 as u16);
    20
}
/// $051ADC: clr.w $2a(a0)
pub fn i_051adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51AE0;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $051AE0: clr.w $2c(a0)
pub fn i_051ae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51AE4;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $051AE4: clr.w $22(a0)
pub fn i_051ae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51AE8;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $051AE8: clr.w $24(a0)
pub fn i_051ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51AEC;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $051AEC: clr.w $c(a0)
pub fn i_051aec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51AF0;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0xC);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $051AF0: clr.l $4(a0)
pub fn i_051af0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51AF4;
    cpu.ir = 0x42A8;
    let ad1 = cpu.a[0].wrapping_add(0x4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $051AF4: clr.l $8(a0)
pub fn i_051af4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51AF8;
    cpu.ir = 0x42A8;
    let ad1 = cpu.a[0].wrapping_add(0x8);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $051AF8: clr.w (a0)
pub fn i_051af8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51AFA;
    cpu.ir = 0x4250;
    let ad1 = cpu.a[0];
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $051AFA: clr.w $2(a0)
pub fn i_051afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51AFE;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $051AFE: clr.w $1e(a0)
pub fn i_051afe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B02;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x1E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $051B02: clr.w $20(a0)
pub fn i_051b02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B06;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x20);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $051B06: bsr.w $51ed6
pub fn i_051b06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B0A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x51B0A);
    cpu.pc = 0x51ED6;
    18
}
/// $051B0A: bsr.w $51f02
pub fn i_051b0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B0E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x51B0E);
    cpu.pc = 0x51F02;
    18
}
/// $051B0E: lea.l $32(a0), a1
pub fn i_051b0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B12;
    cpu.ir = 0x43E8;
    let ad1 = cpu.a[0].wrapping_add(0x32);
    cpu.a[1] = ad1;
    16
}
/// $051B12: moveq #$1f, d7
pub fn i_051b12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B14;
    cpu.ir = 0x7E1F;
    cpu.d[7] = 0x1F;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $051B14: clr.b (a1)+
pub fn i_051b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B16;
    cpu.ir = 0x4219;
    let ad1 = cpu.a[1]; cpu.a[1] = ad1.wrapping_add(1);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    8
}
/// $051B16: dbra d7, $51b14
pub fn i_051b16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B1A;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x51B1A; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x51B1A; return 14; }
    cpu.pc = 0x51B14;
    10
}
/// $051B1A: rts
pub fn i_051b1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B1C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== CellMap_SetCursorAndClear_051b1c  ($051B1C, 100 B, video/cellmap_fix_overlay_palfade_051aa4.s)
/// $051B1C: andi.b #$3f, d0
pub fn i_051b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B20;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3F) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $051B20: move.b d0, $27(a0)
pub fn i_051b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B24;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x27);
    bus.write8(ad2, v1 as u8);
    12
}
/// $051B24: add.w $24(a0), d1
pub fn i_051b24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B28;
    cpu.ir = 0xD268;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $051B28: neg.w d1
pub fn i_051b28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B2A;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $051B2A: andi.w #$1f, d1
pub fn i_051b2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B2E;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $051B2E: move.b d1, $26(a0)
pub fn i_051b2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B32;
    cpu.ir = 0x1141;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x26);
    bus.write8(ad2, v1 as u8);
    12
}
/// $051B32: clr.l $12(a0)
pub fn i_051b32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B36;
    cpu.ir = 0x42A8;
    let ad1 = cpu.a[0].wrapping_add(0x12);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $051B36: rts
pub fn i_051b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B38;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $051B38: move.l a1, $12(a0)
pub fn i_051b38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B3C;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x12);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $051B3C: rts
pub fn i_051b3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B3E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $051B3E: move.l a1, $e(a0)
pub fn i_051b3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B42;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0xE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $051B42: move.w $4(a0), d2
pub fn i_051b42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B46;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $051B46: sub.w d0, d2
pub fn i_051b46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B48;
    cpu.ir = 0x9440;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $051B48: lsr.w #$4, d2
pub fn i_051b48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B4A;
    cpu.ir = 0xE84A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, false);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $051B4A: add.w $22(a0), d2
pub fn i_051b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B4E;
    cpu.ir = 0xD468;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $051B4E: andi.w #$1f, d2
pub fn i_051b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B52;
    cpu.ir = 0x242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $051B52: move.w d2, $22(a0)
pub fn i_051b52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B56;
    cpu.ir = 0x3142;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $051B56: move.w $8(a0), d2
pub fn i_051b56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B5A;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $051B5A: sub.w d1, d2
pub fn i_051b5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B5C;
    cpu.ir = 0x9441;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $051B5C: lsr.w #$4, d2
pub fn i_051b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B5E;
    cpu.ir = 0xE84A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, false);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $051B5E: add.w $24(a0), d2
pub fn i_051b5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B62;
    cpu.ir = 0xD468;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $051B62: andi.w #$1f, d2
pub fn i_051b62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B66;
    cpu.ir = 0x242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $051B66: move.w d2, $24(a0)
pub fn i_051b66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B6A;
    cpu.ir = 0x3142;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $051B6A: move.w d0, $4(a0)
pub fn i_051b6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B6E;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    bus.write16(ad2, v1 as u16);
    12
}
/// $051B6E: move.w d1, $8(a0)
pub fn i_051b6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B72;
    cpu.ir = 0x3141;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x8);
    bus.write16(ad2, v1 as u16);
    12
}
/// $051B72: clr.w $1e(a0)
pub fn i_051b72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B76;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x1E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $051B76: clr.w $20(a0)
pub fn i_051b76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B7A;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x20);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $051B7A: moveq #$20, d0
pub fn i_051b7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B7C;
    cpu.ir = 0x7020;
    cpu.d[0] = 0x20;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $051B7C: moveq #$0, d1
pub fn i_051b7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B7E;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $051B7E: bra.b $51b1c
pub fn i_051b7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B80;
    cpu.ir = 0x609C;
    cpu.pc = 0x51B1C;
    10
}
// ===== Integrator_XY_051B80  ($051B80, 40 B, video/camera_sprite_helpers_043e8c.s)
/// $051B80: move.l $4(a0), d2
pub fn i_051b80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B84;
    cpu.ir = 0x2428;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $051B84: add.l d2, d0
pub fn i_051b84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B86;
    cpu.ir = 0xD082;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[0] = r;
    4
}
/// $051B86: move.l d0, $4(a0)
pub fn i_051b86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B8A;
    cpu.ir = 0x2140;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $051B8A: swap d0
pub fn i_051b8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B8C;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $051B8C: swap d2
pub fn i_051b8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B8E;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $051B8E: sub.w d2, d0
pub fn i_051b8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B90;
    cpu.ir = 0x9042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $051B90: move.w d0, (a0)
pub fn i_051b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B92;
    cpu.ir = 0x3080;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v1 as u16);
    8
}
/// $051B92: move.l $8(a0), d2
pub fn i_051b92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B96;
    cpu.ir = 0x2428;
    let ad2 = cpu.a[0].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $051B96: add.l d2, d1
pub fn i_051b96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B98;
    cpu.ir = 0xD282;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[1] = r;
    4
}
/// $051B98: move.l d1, $8(a0)
pub fn i_051b98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B9C;
    cpu.ir = 0x2141;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x8);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $051B9C: swap d1
pub fn i_051b9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51B9E;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $051B9E: swap d2
pub fn i_051b9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BA0;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $051BA0: sub.w d2, d1
pub fn i_051ba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BA2;
    cpu.ir = 0x9242;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $051BA2: move.w d1, $2(a0)
pub fn i_051ba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x51BA6;
    cpu.ir = 0x3141;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x050004, i_050004),
    (0x05000A, i_05000a),
    (0x05000E, i_05000e),
    (0x050012, i_050012),
    (0x050014, i_050014),
    (0x05001A, i_05001a),
    (0x050020, i_050020),
    (0x050024, i_050024),
    (0x050028, i_050028),
    (0x05002A, i_05002a),
    (0x05002E, i_05002e),
    (0x050034, i_050034),
    (0x05003A, i_05003a),
    (0x050040, i_050040),
    (0x050046, i_050046),
    (0x05004A, i_05004a),
    (0x050052, i_050052),
    (0x050058, i_050058),
    (0x05005E, i_05005e),
    (0x050062, i_050062),
    (0x050066, i_050066),
    (0x050068, i_050068),
    (0x05006E, i_05006e),
    (0x050074, i_050074),
    (0x050078, i_050078),
    (0x05007C, i_05007c),
    (0x05007E, i_05007e),
    (0x050082, i_050082),
    (0x050088, i_050088),
    (0x05008E, i_05008e),
    (0x050094, i_050094),
    (0x05009A, i_05009a),
    (0x05009E, i_05009e),
    (0x0500A6, i_0500a6),
    (0x0500AC, i_0500ac),
    (0x0500B2, i_0500b2),
    (0x0500B6, i_0500b6),
    (0x0500BA, i_0500ba),
    (0x0500BC, i_0500bc),
    (0x0500C2, i_0500c2),
    (0x0500C8, i_0500c8),
    (0x0500CC, i_0500cc),
    (0x0500D0, i_0500d0),
    (0x0500D2, i_0500d2),
    (0x0500D6, i_0500d6),
    (0x0500DC, i_0500dc),
    (0x0500E2, i_0500e2),
    (0x0500E8, i_0500e8),
    (0x0500EE, i_0500ee),
    (0x0500F2, i_0500f2),
    (0x0500FA, i_0500fa),
    (0x050100, i_050100),
    (0x050106, i_050106),
    (0x05010A, i_05010a),
    (0x05010E, i_05010e),
    (0x050110, i_050110),
    (0x050116, i_050116),
    (0x05011C, i_05011c),
    (0x050120, i_050120),
    (0x050124, i_050124),
    (0x050126, i_050126),
    (0x05012A, i_05012a),
    (0x050130, i_050130),
    (0x050136, i_050136),
    (0x05013C, i_05013c),
    (0x050142, i_050142),
    (0x050146, i_050146),
    (0x05014E, i_05014e),
    (0x050154, i_050154),
    (0x05015A, i_05015a),
    (0x05015E, i_05015e),
    (0x050162, i_050162),
    (0x050164, i_050164),
    (0x05016A, i_05016a),
    (0x050170, i_050170),
    (0x050174, i_050174),
    (0x050178, i_050178),
    (0x05017A, i_05017a),
    (0x05017E, i_05017e),
    (0x050184, i_050184),
    (0x05018A, i_05018a),
    (0x050190, i_050190),
    (0x050196, i_050196),
    (0x05019A, i_05019a),
    (0x0501A0, i_0501a0),
    (0x0501A4, i_0501a4),
    (0x0501AC, i_0501ac),
    (0x0501B2, i_0501b2),
    (0x0501B8, i_0501b8),
    (0x0501BC, i_0501bc),
    (0x0501C0, i_0501c0),
    (0x0501C2, i_0501c2),
    (0x0501C8, i_0501c8),
    (0x0501CE, i_0501ce),
    (0x0501D2, i_0501d2),
    (0x0501D6, i_0501d6),
    (0x0501D8, i_0501d8),
    (0x0501DE, i_0501de),
    (0x0501E2, i_0501e2),
    (0x0501E8, i_0501e8),
    (0x0501EC, i_0501ec),
    (0x0501F2, i_0501f2),
    (0x0501F6, i_0501f6),
    (0x0501FC, i_0501fc),
    (0x050200, i_050200),
    (0x050206, i_050206),
    (0x05020A, i_05020a),
    (0x050210, i_050210),
    (0x050214, i_050214),
    (0x05021A, i_05021a),
    (0x05021E, i_05021e),
    (0x050224, i_050224),
    (0x050228, i_050228),
    (0x05022A, i_05022a),
    (0x05022C, i_05022c),
    (0x050230, i_050230),
    (0x050234, i_050234),
    (0x050238, i_050238),
    (0x05023C, i_05023c),
    (0x050240, i_050240),
    (0x050244, i_050244),
    (0x050248, i_050248),
    (0x05024E, i_05024e),
    (0x050250, i_050250),
    (0x050254, i_050254),
    (0x050258, i_050258),
    (0x05025C, i_05025c),
    (0x050260, i_050260),
    (0x050264, i_050264),
    (0x050266, i_050266),
    (0x05026A, i_05026a),
    (0x05026C, i_05026c),
    (0x050270, i_050270),
    (0x050274, i_050274),
    (0x05027A, i_05027a),
    (0x05027E, i_05027e),
    (0x050280, i_050280),
    (0x050282, i_050282),
    (0x050286, i_050286),
    (0x05028C, i_05028c),
    (0x05028E, i_05028e),
    (0x050290, i_050290),
    (0x050292, i_050292),
    (0x050294, i_050294),
    (0x050296, i_050296),
    (0x05029A, i_05029a),
    (0x05029C, i_05029c),
    (0x0502A2, i_0502a2),
    (0x0502A6, i_0502a6),
    (0x0502AC, i_0502ac),
    (0x0502B0, i_0502b0),
    (0x0502B4, i_0502b4),
    (0x0502B8, i_0502b8),
    (0x0502BC, i_0502bc),
    (0x0502C0, i_0502c0),
    (0x0502C4, i_0502c4),
    (0x0502C8, i_0502c8),
    (0x0502CC, i_0502cc),
    (0x0502D2, i_0502d2),
    (0x0502D6, i_0502d6),
    (0x0502DC, i_0502dc),
    (0x0502E0, i_0502e0),
    (0x0502E4, i_0502e4),
    (0x0502EA, i_0502ea),
    (0x0502EE, i_0502ee),
    (0x0502F4, i_0502f4),
    (0x0502F8, i_0502f8),
    (0x0502FA, i_0502fa),
    (0x0502FE, i_0502fe),
    (0x050300, i_050300),
    (0x050302, i_050302),
    (0x050306, i_050306),
    (0x05030A, i_05030a),
    (0x05030C, i_05030c),
    (0x050310, i_050310),
    (0x050312, i_050312),
    (0x050314, i_050314),
    (0x050318, i_050318),
    (0x05031A, i_05031a),
    (0x05031C, i_05031c),
    (0x050320, i_050320),
    (0x050324, i_050324),
    (0x050326, i_050326),
    (0x050328, i_050328),
    (0x05032C, i_05032c),
    (0x05032E, i_05032e),
    (0x050330, i_050330),
    (0x050334, i_050334),
    (0x050336, i_050336),
    (0x05033A, i_05033a),
    (0x05033C, i_05033c),
    (0x050340, i_050340),
    (0x050342, i_050342),
    (0x050346, i_050346),
    (0x050348, i_050348),
    (0x05034C, i_05034c),
    (0x05034E, i_05034e),
    (0x050352, i_050352),
    (0x050354, i_050354),
    (0x050358, i_050358),
    (0x05035C, i_05035c),
    (0x05035E, i_05035e),
    (0x050360, i_050360),
    (0x050364, i_050364),
    (0x050368, i_050368),
    (0x05036C, i_05036c),
    (0x050372, i_050372),
    (0x050376, i_050376),
    (0x05037A, i_05037a),
    (0x050380, i_050380),
    (0x050384, i_050384),
    (0x050388, i_050388),
    (0x05038C, i_05038c),
    (0x050390, i_050390),
    (0x050394, i_050394),
    (0x050398, i_050398),
    (0x05039C, i_05039c),
    (0x0503A2, i_0503a2),
    (0x0503A6, i_0503a6),
    (0x0503AA, i_0503aa),
    (0x0503AE, i_0503ae),
    (0x0503B0, i_0503b0),
    (0x0503B6, i_0503b6),
    (0x0503BA, i_0503ba),
    (0x0503C0, i_0503c0),
    (0x0503C6, i_0503c6),
    (0x0503CA, i_0503ca),
    (0x0503CE, i_0503ce),
    (0x0503D4, i_0503d4),
    (0x0503DA, i_0503da),
    (0x0503DE, i_0503de),
    (0x0503E4, i_0503e4),
    (0x0503E8, i_0503e8),
    (0x0503F2, i_0503f2),
    (0x0503F6, i_0503f6),
    (0x0503FA, i_0503fa),
    (0x0503FC, i_0503fc),
    (0x050400, i_050400),
    (0x050404, i_050404),
    (0x050408, i_050408),
    (0x05040A, i_05040a),
    (0x050410, i_050410),
    (0x050414, i_050414),
    (0x05041C, i_05041c),
    (0x050420, i_050420),
    (0x050424, i_050424),
    (0x05042A, i_05042a),
    (0x05042E, i_05042e),
    (0x050432, i_050432),
    (0x050436, i_050436),
    (0x05043A, i_05043a),
    (0x05043E, i_05043e),
    (0x050444, i_050444),
    (0x050448, i_050448),
    (0x05044C, i_05044c),
    (0x050450, i_050450),
    (0x050454, i_050454),
    (0x05045A, i_05045a),
    (0x05045E, i_05045e),
    (0x050462, i_050462),
    (0x050466, i_050466),
    (0x05046A, i_05046a),
    (0x05046C, i_05046c),
    (0x050470, i_050470),
    (0x050474, i_050474),
    (0x050476, i_050476),
    (0x05047C, i_05047c),
    (0x05047E, i_05047e),
    (0x050480, i_050480),
    (0x050484, i_050484),
    (0x050488, i_050488),
    (0x05048E, i_05048e),
    (0x050490, i_050490),
    (0x050494, i_050494),
    (0x050498, i_050498),
    (0x05049C, i_05049c),
    (0x0504A0, i_0504a0),
    (0x0504A4, i_0504a4),
    (0x0504A8, i_0504a8),
    (0x0504AC, i_0504ac),
    (0x0504B0, i_0504b0),
    (0x0504B2, i_0504b2),
    (0x0504B6, i_0504b6),
    (0x0504BA, i_0504ba),
    (0x0504BC, i_0504bc),
    (0x0504C0, i_0504c0),
    (0x0504C2, i_0504c2),
    (0x0504C6, i_0504c6),
    (0x0504CA, i_0504ca),
    (0x0504CC, i_0504cc),
    (0x0504D0, i_0504d0),
    (0x0504D2, i_0504d2),
    (0x0504D6, i_0504d6),
    (0x0504DA, i_0504da),
    (0x0504DE, i_0504de),
    (0x0504E0, i_0504e0),
    (0x0504E4, i_0504e4),
    (0x0504E8, i_0504e8),
    (0x0504EA, i_0504ea),
    (0x0504EE, i_0504ee),
    (0x0504F0, i_0504f0),
    (0x0504F4, i_0504f4),
    (0x0504F8, i_0504f8),
    (0x0504FC, i_0504fc),
    (0x050500, i_050500),
    (0x050504, i_050504),
    (0x050508, i_050508),
    (0x05050C, i_05050c),
    (0x050510, i_050510),
    (0x050514, i_050514),
    (0x050518, i_050518),
    (0x05051A, i_05051a),
    (0x05051E, i_05051e),
    (0x050520, i_050520),
    (0x050524, i_050524),
    (0x050528, i_050528),
    (0x05052E, i_05052e),
    (0x050532, i_050532),
    (0x050536, i_050536),
    (0x05053C, i_05053c),
    (0x050540, i_050540),
    (0x050544, i_050544),
    (0x050546, i_050546),
    (0x05054A, i_05054a),
    (0x05054C, i_05054c),
    (0x050550, i_050550),
    (0x050554, i_050554),
    (0x050558, i_050558),
    (0x05055E, i_05055e),
    (0x050562, i_050562),
    (0x050566, i_050566),
    (0x05056A, i_05056a),
    (0x05056E, i_05056e),
    (0x050572, i_050572),
    (0x050576, i_050576),
    (0x05057A, i_05057a),
    (0x05057E, i_05057e),
    (0x050582, i_050582),
    (0x050588, i_050588),
    (0x05058C, i_05058c),
    (0x050590, i_050590),
    (0x050594, i_050594),
    (0x050598, i_050598),
    (0x05059A, i_05059a),
    (0x05059E, i_05059e),
    (0x0505A0, i_0505a0),
    (0x0505A4, i_0505a4),
    (0x0505A8, i_0505a8),
    (0x0505AC, i_0505ac),
    (0x0505B0, i_0505b0),
    (0x0505B4, i_0505b4),
    (0x0505B8, i_0505b8),
    (0x0505BC, i_0505bc),
    (0x0505BE, i_0505be),
    (0x0505C2, i_0505c2),
    (0x0505C6, i_0505c6),
    (0x0505CA, i_0505ca),
    (0x0505CE, i_0505ce),
    (0x0505D2, i_0505d2),
    (0x0505D6, i_0505d6),
    (0x0505DA, i_0505da),
    (0x0505DC, i_0505dc),
    (0x0505DE, i_0505de),
    (0x0505E0, i_0505e0),
    (0x0505E4, i_0505e4),
    (0x0505E8, i_0505e8),
    (0x0505EA, i_0505ea),
    (0x0505EE, i_0505ee),
    (0x0505F0, i_0505f0),
    (0x0505F4, i_0505f4),
    (0x0505F8, i_0505f8),
    (0x0505FC, i_0505fc),
    (0x050600, i_050600),
    (0x050604, i_050604),
    (0x050608, i_050608),
    (0x05060C, i_05060c),
    (0x050612, i_050612),
    (0x050616, i_050616),
    (0x05061C, i_05061c),
    (0x050622, i_050622),
    (0x050628, i_050628),
    (0x05062E, i_05062e),
    (0x050632, i_050632),
    (0x050636, i_050636),
    (0x050638, i_050638),
    (0x05063C, i_05063c),
    (0x05063E, i_05063e),
    (0x050642, i_050642),
    (0x050646, i_050646),
    (0x050648, i_050648),
    (0x05064E, i_05064e),
    (0x050652, i_050652),
    (0x050658, i_050658),
    (0x05065A, i_05065a),
    (0x050660, i_050660),
    (0x050664, i_050664),
    (0x050668, i_050668),
    (0x05066C, i_05066c),
    (0x05066E, i_05066e),
    (0x050674, i_050674),
    (0x050678, i_050678),
    (0x05067C, i_05067c),
    (0x05067E, i_05067e),
    (0x050682, i_050682),
    (0x050686, i_050686),
    (0x05068A, i_05068a),
    (0x05068E, i_05068e),
    (0x050692, i_050692),
    (0x050694, i_050694),
    (0x05069A, i_05069a),
    (0x05069C, i_05069c),
    (0x0506A0, i_0506a0),
    (0x0506A2, i_0506a2),
    (0x0506A6, i_0506a6),
    (0x0506AA, i_0506aa),
    (0x0506AE, i_0506ae),
    (0x0506B4, i_0506b4),
    (0x0506B8, i_0506b8),
    (0x0506BC, i_0506bc),
    (0x0506C0, i_0506c0),
    (0x0506C4, i_0506c4),
    (0x0506C8, i_0506c8),
    (0x0506CC, i_0506cc),
    (0x0506D0, i_0506d0),
    (0x0506D4, i_0506d4),
    (0x0506D8, i_0506d8),
    (0x0506DE, i_0506de),
    (0x0506E2, i_0506e2),
    (0x0506E6, i_0506e6),
    (0x0506EA, i_0506ea),
    (0x0506EE, i_0506ee),
    (0x0506F0, i_0506f0),
    (0x0506F4, i_0506f4),
    (0x0506F6, i_0506f6),
    (0x0506FA, i_0506fa),
    (0x0506FE, i_0506fe),
    (0x050702, i_050702),
    (0x050706, i_050706),
    (0x05070A, i_05070a),
    (0x05070E, i_05070e),
    (0x050712, i_050712),
    (0x050716, i_050716),
    (0x05071A, i_05071a),
    (0x05071E, i_05071e),
    (0x050720, i_050720),
    (0x050724, i_050724),
    (0x05072E, i_05072e),
    (0x050732, i_050732),
    (0x050736, i_050736),
    (0x05073A, i_05073a),
    (0x050740, i_050740),
    (0x050744, i_050744),
    (0x05074A, i_05074a),
    (0x05074E, i_05074e),
    (0x050752, i_050752),
    (0x050758, i_050758),
    (0x05075E, i_05075e),
    (0x050762, i_050762),
    (0x050766, i_050766),
    (0x05076A, i_05076a),
    (0x05076C, i_05076c),
    (0x050770, i_050770),
    (0x050772, i_050772),
    (0x050776, i_050776),
    (0x05077A, i_05077a),
    (0x05077E, i_05077e),
    (0x050782, i_050782),
    (0x050786, i_050786),
    (0x05078A, i_05078a),
    (0x05078E, i_05078e),
    (0x050792, i_050792),
    (0x050794, i_050794),
    (0x05079A, i_05079a),
    (0x05079E, i_05079e),
    (0x0507A2, i_0507a2),
    (0x0507A6, i_0507a6),
    (0x0507AA, i_0507aa),
    (0x0507B0, i_0507b0),
    (0x0507B4, i_0507b4),
    (0x0507B8, i_0507b8),
    (0x0507BC, i_0507bc),
    (0x0507BE, i_0507be),
    (0x0507C2, i_0507c2),
    (0x0507C8, i_0507c8),
    (0x0507CC, i_0507cc),
    (0x0507D2, i_0507d2),
    (0x0507D8, i_0507d8),
    (0x0507DE, i_0507de),
    (0x0507E4, i_0507e4),
    (0x0507E8, i_0507e8),
    (0x0507EC, i_0507ec),
    (0x0507EE, i_0507ee),
    (0x0507F4, i_0507f4),
    (0x0507F8, i_0507f8),
    (0x0507FE, i_0507fe),
    (0x050802, i_050802),
    (0x050808, i_050808),
    (0x05080A, i_05080a),
    (0x05080E, i_05080e),
    (0x050814, i_050814),
    (0x050816, i_050816),
    (0x05081A, i_05081a),
    (0x050820, i_050820),
    (0x050822, i_050822),
    (0x050826, i_050826),
    (0x05082C, i_05082c),
    (0x05082E, i_05082e),
    (0x050834, i_050834),
    (0x050838, i_050838),
    (0x05083E, i_05083e),
    (0x050840, i_050840),
    (0x050846, i_050846),
    (0x05084A, i_05084a),
    (0x050850, i_050850),
    (0x050854, i_050854),
    (0x050856, i_050856),
    (0x05085C, i_05085c),
    (0x050860, i_050860),
    (0x050866, i_050866),
    (0x05086A, i_05086a),
    (0x05086E, i_05086e),
    (0x050872, i_050872),
    (0x050876, i_050876),
    (0x05087A, i_05087a),
    (0x05087C, i_05087c),
    (0x050880, i_050880),
    (0x050886, i_050886),
    (0x050888, i_050888),
    (0x05088E, i_05088e),
    (0x050892, i_050892),
    (0x050898, i_050898),
    (0x05089A, i_05089a),
    (0x0508A0, i_0508a0),
    (0x0508A4, i_0508a4),
    (0x0508AA, i_0508aa),
    (0x0508AE, i_0508ae),
    (0x0508B0, i_0508b0),
    (0x0508B6, i_0508b6),
    (0x0508BA, i_0508ba),
    (0x0508C0, i_0508c0),
    (0x0508C4, i_0508c4),
    (0x0508C8, i_0508c8),
    (0x0508CC, i_0508cc),
    (0x0508D0, i_0508d0),
    (0x0508D4, i_0508d4),
    (0x0508D6, i_0508d6),
    (0x0508DC, i_0508dc),
    (0x0508E2, i_0508e2),
    (0x0508E6, i_0508e6),
    (0x0508EA, i_0508ea),
    (0x0508EC, i_0508ec),
    (0x0508EE, i_0508ee),
    (0x0508F2, i_0508f2),
    (0x0508F4, i_0508f4),
    (0x0508F6, i_0508f6),
    (0x0508F8, i_0508f8),
    (0x0508FA, i_0508fa),
    (0x050900, i_050900),
    (0x050904, i_050904),
    (0x050908, i_050908),
    (0x05090A, i_05090a),
    (0x05090E, i_05090e),
    (0x050912, i_050912),
    (0x050918, i_050918),
    (0x05091C, i_05091c),
    (0x050920, i_050920),
    (0x050926, i_050926),
    (0x05092A, i_05092a),
    (0x050930, i_050930),
    (0x050934, i_050934),
    (0x05093A, i_05093a),
    (0x05093E, i_05093e),
    (0x050944, i_050944),
    (0x050948, i_050948),
    (0x05094C, i_05094c),
    (0x050950, i_050950),
    (0x050954, i_050954),
    (0x050958, i_050958),
    (0x05095C, i_05095c),
    (0x050960, i_050960),
    (0x050966, i_050966),
    (0x05096A, i_05096a),
    (0x05096E, i_05096e),
    (0x050972, i_050972),
    (0x050974, i_050974),
    (0x050976, i_050976),
    (0x05097A, i_05097a),
    (0x05097E, i_05097e),
    (0x050984, i_050984),
    (0x05098A, i_05098a),
    (0x05098E, i_05098e),
    (0x050992, i_050992),
    (0x050998, i_050998),
    (0x05099C, i_05099c),
    (0x0509A2, i_0509a2),
    (0x0509A6, i_0509a6),
    (0x0509A8, i_0509a8),
    (0x0509AE, i_0509ae),
    (0x0509B2, i_0509b2),
    (0x0509B6, i_0509b6),
    (0x0509BC, i_0509bc),
    (0x0509C0, i_0509c0),
    (0x0509C4, i_0509c4),
    (0x0509C8, i_0509c8),
    (0x0509CC, i_0509cc),
    (0x0509D0, i_0509d0),
    (0x0509D2, i_0509d2),
    (0x0509DE, i_0509de),
    (0x0509E4, i_0509e4),
    (0x0509EA, i_0509ea),
    (0x0509F0, i_0509f0),
    (0x0509F4, i_0509f4),
    (0x0509F8, i_0509f8),
    (0x0509FA, i_0509fa),
    (0x050A00, i_050a00),
    (0x050A04, i_050a04),
    (0x050A08, i_050a08),
    (0x050A0E, i_050a0e),
    (0x050A12, i_050a12),
    (0x050A16, i_050a16),
    (0x050A1C, i_050a1c),
    (0x050A20, i_050a20),
    (0x050A24, i_050a24),
    (0x050A26, i_050a26),
    (0x050A2A, i_050a2a),
    (0x050A2E, i_050a2e),
    (0x050A32, i_050a32),
    (0x050A38, i_050a38),
    (0x050A3C, i_050a3c),
    (0x050A42, i_050a42),
    (0x050A46, i_050a46),
    (0x050A4A, i_050a4a),
    (0x050A4C, i_050a4c),
    (0x050A50, i_050a50),
    (0x050A54, i_050a54),
    (0x050A5A, i_050a5a),
    (0x050A60, i_050a60),
    (0x050A64, i_050a64),
    (0x050A6A, i_050a6a),
    (0x050A70, i_050a70),
    (0x050A74, i_050a74),
    (0x050A76, i_050a76),
    (0x050A7A, i_050a7a),
    (0x050A7E, i_050a7e),
    (0x050A84, i_050a84),
    (0x050A88, i_050a88),
    (0x050A8C, i_050a8c),
    (0x050A90, i_050a90),
    (0x050A92, i_050a92),
    (0x050A96, i_050a96),
    (0x050A9A, i_050a9a),
    (0x050A9E, i_050a9e),
    (0x050AA0, i_050aa0),
    (0x050AA4, i_050aa4),
    (0x050AA8, i_050aa8),
    (0x050AAC, i_050aac),
    (0x050AAE, i_050aae),
    (0x050AB2, i_050ab2),
    (0x050AB6, i_050ab6),
    (0x050ABA, i_050aba),
    (0x050ABC, i_050abc),
    (0x050AC0, i_050ac0),
    (0x050AC4, i_050ac4),
    (0x050AC8, i_050ac8),
    (0x050ACA, i_050aca),
    (0x050ACE, i_050ace),
    (0x050AD2, i_050ad2),
    (0x050AD6, i_050ad6),
    (0x050AD8, i_050ad8),
    (0x050ADC, i_050adc),
    (0x050AE0, i_050ae0),
    (0x050AE4, i_050ae4),
    (0x050AE6, i_050ae6),
    (0x050AEA, i_050aea),
    (0x050AEE, i_050aee),
    (0x050AF2, i_050af2),
    (0x050AF4, i_050af4),
    (0x050AF8, i_050af8),
    (0x050AFC, i_050afc),
    (0x050B00, i_050b00),
    (0x050B02, i_050b02),
    (0x050B06, i_050b06),
    (0x050B0A, i_050b0a),
    (0x050B0E, i_050b0e),
    (0x050B10, i_050b10),
    (0x050B14, i_050b14),
    (0x050B18, i_050b18),
    (0x050B1C, i_050b1c),
    (0x050B1E, i_050b1e),
    (0x050B22, i_050b22),
    (0x050B26, i_050b26),
    (0x050B2C, i_050b2c),
    (0x050B32, i_050b32),
    (0x050B36, i_050b36),
    (0x050B3A, i_050b3a),
    (0x050B3C, i_050b3c),
    (0x050B40, i_050b40),
    (0x050B44, i_050b44),
    (0x050B4A, i_050b4a),
    (0x050B4E, i_050b4e),
    (0x050B54, i_050b54),
    (0x050B58, i_050b58),
    (0x050B5A, i_050b5a),
    (0x050B5E, i_050b5e),
    (0x050B62, i_050b62),
    (0x050B66, i_050b66),
    (0x050B6A, i_050b6a),
    (0x050B6E, i_050b6e),
    (0x050B72, i_050b72),
    (0x050B78, i_050b78),
    (0x050B7E, i_050b7e),
    (0x050B82, i_050b82),
    (0x050B86, i_050b86),
    (0x050B8A, i_050b8a),
    (0x050B90, i_050b90),
    (0x050B96, i_050b96),
    (0x050B9C, i_050b9c),
    (0x050BA0, i_050ba0),
    (0x050BA4, i_050ba4),
    (0x050BA6, i_050ba6),
    (0x050BAA, i_050baa),
    (0x050BAE, i_050bae),
    (0x050BB4, i_050bb4),
    (0x050BB8, i_050bb8),
    (0x050BBC, i_050bbc),
    (0x050BBE, i_050bbe),
    (0x050BC2, i_050bc2),
    (0x050BC8, i_050bc8),
    (0x050BCE, i_050bce),
    (0x050BD2, i_050bd2),
    (0x050BD8, i_050bd8),
    (0x050BDE, i_050bde),
    (0x050BE2, i_050be2),
    (0x050BE6, i_050be6),
    (0x050BEA, i_050bea),
    (0x050BEC, i_050bec),
    (0x050BF0, i_050bf0),
    (0x050BF4, i_050bf4),
    (0x050BFA, i_050bfa),
    (0x050BFE, i_050bfe),
    (0x050C04, i_050c04),
    (0x050C0A, i_050c0a),
    (0x050C0E, i_050c0e),
    (0x050C12, i_050c12),
    (0x050C14, i_050c14),
    (0x050C18, i_050c18),
    (0x050C1C, i_050c1c),
    (0x050C22, i_050c22),
    (0x050C28, i_050c28),
    (0x050C2C, i_050c2c),
    (0x050C2E, i_050c2e),
    (0x050C32, i_050c32),
    (0x050C36, i_050c36),
    (0x050C3C, i_050c3c),
    (0x050C40, i_050c40),
    (0x050C44, i_050c44),
    (0x050C48, i_050c48),
    (0x050C4A, i_050c4a),
    (0x050C4E, i_050c4e),
    (0x050C52, i_050c52),
    (0x050C58, i_050c58),
    (0x050C5E, i_050c5e),
    (0x050C62, i_050c62),
    (0x050C66, i_050c66),
    (0x050C6A, i_050c6a),
    (0x050C6C, i_050c6c),
    (0x050C70, i_050c70),
    (0x050C74, i_050c74),
    (0x050C7A, i_050c7a),
    (0x050C7E, i_050c7e),
    (0x050C82, i_050c82),
    (0x050C84, i_050c84),
    (0x050C88, i_050c88),
    (0x050C8C, i_050c8c),
    (0x050C92, i_050c92),
    (0x050C96, i_050c96),
    (0x050C9A, i_050c9a),
    (0x050C9E, i_050c9e),
    (0x050CA2, i_050ca2),
    (0x050CA4, i_050ca4),
    (0x050CA8, i_050ca8),
    (0x050CAC, i_050cac),
    (0x050CB0, i_050cb0),
    (0x050CB4, i_050cb4),
    (0x050CB8, i_050cb8),
    (0x050CBC, i_050cbc),
    (0x050CC2, i_050cc2),
    (0x050CC8, i_050cc8),
    (0x050CCE, i_050cce),
    (0x050CD4, i_050cd4),
    (0x050CD8, i_050cd8),
    (0x050CDC, i_050cdc),
    (0x050CE0, i_050ce0),
    (0x050CE2, i_050ce2),
    (0x050CE6, i_050ce6),
    (0x050CEC, i_050cec),
    (0x050CF0, i_050cf0),
    (0x050CF2, i_050cf2),
    (0x050CF6, i_050cf6),
    (0x050CFC, i_050cfc),
    (0x050D02, i_050d02),
    (0x050D06, i_050d06),
    (0x050D0A, i_050d0a),
    (0x050D10, i_050d10),
    (0x050D14, i_050d14),
    (0x050D16, i_050d16),
    (0x050D1A, i_050d1a),
    (0x050D20, i_050d20),
    (0x050D26, i_050d26),
    (0x050D2C, i_050d2c),
    (0x050D30, i_050d30),
    (0x050D32, i_050d32),
    (0x050D34, i_050d34),
    (0x050D36, i_050d36),
    (0x050D38, i_050d38),
    (0x050D3A, i_050d3a),
    (0x050D3E, i_050d3e),
    (0x050D42, i_050d42),
    (0x050D44, i_050d44),
    (0x050D46, i_050d46),
    (0x050D4A, i_050d4a),
    (0x050D4E, i_050d4e),
    (0x050D54, i_050d54),
    (0x050D58, i_050d58),
    (0x050D5C, i_050d5c),
    (0x050D60, i_050d60),
    (0x050D64, i_050d64),
    (0x050D68, i_050d68),
    (0x050D6E, i_050d6e),
    (0x050D72, i_050d72),
    (0x050D74, i_050d74),
    (0x050D78, i_050d78),
    (0x050D7C, i_050d7c),
    (0x050D80, i_050d80),
    (0x050D84, i_050d84),
    (0x050D8A, i_050d8a),
    (0x050D8E, i_050d8e),
    (0x050D90, i_050d90),
    (0x050D94, i_050d94),
    (0x050D98, i_050d98),
    (0x050D9E, i_050d9e),
    (0x050DA2, i_050da2),
    (0x050DA6, i_050da6),
    (0x050DAA, i_050daa),
    (0x050DAC, i_050dac),
    (0x050DB0, i_050db0),
    (0x050DB4, i_050db4),
    (0x050DB8, i_050db8),
    (0x050DBA, i_050dba),
    (0x050DBE, i_050dbe),
    (0x050DC4, i_050dc4),
    (0x050DCA, i_050dca),
    (0x050DCE, i_050dce),
    (0x050DD4, i_050dd4),
    (0x050DD8, i_050dd8),
    (0x050DDE, i_050dde),
    (0x050DE4, i_050de4),
    (0x050DEA, i_050dea),
    (0x050DF0, i_050df0),
    (0x050DF4, i_050df4),
    (0x050DF8, i_050df8),
    (0x050DFA, i_050dfa),
    (0x050DFC, i_050dfc),
    (0x050E00, i_050e00),
    (0x050E06, i_050e06),
    (0x050E0C, i_050e0c),
    (0x050E12, i_050e12),
    (0x050E16, i_050e16),
    (0x050E18, i_050e18),
    (0x050E1C, i_050e1c),
    (0x050E20, i_050e20),
    (0x050E26, i_050e26),
    (0x050E2A, i_050e2a),
    (0x050E2E, i_050e2e),
    (0x050E30, i_050e30),
    (0x050E34, i_050e34),
    (0x050E38, i_050e38),
    (0x050E3C, i_050e3c),
    (0x050E40, i_050e40),
    (0x050E46, i_050e46),
    (0x050E4A, i_050e4a),
    (0x050E50, i_050e50),
    (0x050E54, i_050e54),
    (0x050E58, i_050e58),
    (0x050E5C, i_050e5c),
    (0x050E60, i_050e60),
    (0x050E66, i_050e66),
    (0x050E6A, i_050e6a),
    (0x050E6E, i_050e6e),
    (0x050E74, i_050e74),
    (0x050E7A, i_050e7a),
    (0x050E7E, i_050e7e),
    (0x050E82, i_050e82),
    (0x050E84, i_050e84),
    (0x050E8A, i_050e8a),
    (0x050E8E, i_050e8e),
    (0x050E94, i_050e94),
    (0x050E98, i_050e98),
    (0x050E9C, i_050e9c),
    (0x050E9E, i_050e9e),
    (0x050EA2, i_050ea2),
    (0x050EA8, i_050ea8),
    (0x050EAE, i_050eae),
    (0x050EB2, i_050eb2),
    (0x050EB8, i_050eb8),
    (0x050EBC, i_050ebc),
    (0x050EC2, i_050ec2),
    (0x050EC8, i_050ec8),
    (0x050ECE, i_050ece),
    (0x050ED2, i_050ed2),
    (0x050ED4, i_050ed4),
    (0x050EDA, i_050eda),
    (0x050EDE, i_050ede),
    (0x050EE4, i_050ee4),
    (0x050EE8, i_050ee8),
    (0x050EEC, i_050eec),
    (0x050EF0, i_050ef0),
    (0x050EF2, i_050ef2),
    (0x050EF6, i_050ef6),
    (0x050EFC, i_050efc),
    (0x050F02, i_050f02),
    (0x050F06, i_050f06),
    (0x050F0A, i_050f0a),
    (0x050F0C, i_050f0c),
    (0x050F10, i_050f10),
    (0x050F16, i_050f16),
    (0x050F1C, i_050f1c),
    (0x050F20, i_050f20),
    (0x050F24, i_050f24),
    (0x050F26, i_050f26),
    (0x050F2C, i_050f2c),
    (0x050F30, i_050f30),
    (0x050F36, i_050f36),
    (0x050F3A, i_050f3a),
    (0x050F40, i_050f40),
    (0x050F46, i_050f46),
    (0x050F4C, i_050f4c),
    (0x050F50, i_050f50),
    (0x050F52, i_050f52),
    (0x050F56, i_050f56),
    (0x050F5C, i_050f5c),
    (0x050F60, i_050f60),
    (0x050F62, i_050f62),
    (0x050F66, i_050f66),
    (0x050F6C, i_050f6c),
    (0x050F70, i_050f70),
    (0x050F74, i_050f74),
    (0x050F7A, i_050f7a),
    (0x050F7E, i_050f7e),
    (0x050F84, i_050f84),
    (0x050F88, i_050f88),
    (0x050F8E, i_050f8e),
    (0x050F92, i_050f92),
    (0x050F98, i_050f98),
    (0x050F9C, i_050f9c),
    (0x050FA2, i_050fa2),
    (0x050FA6, i_050fa6),
    (0x050FAC, i_050fac),
    (0x050FB0, i_050fb0),
    (0x050FB6, i_050fb6),
    (0x050FBA, i_050fba),
    (0x050FC0, i_050fc0),
    (0x050FC4, i_050fc4),
    (0x050FCA, i_050fca),
    (0x050FCE, i_050fce),
    (0x050FD0, i_050fd0),
    (0x050FD4, i_050fd4),
    (0x050FDA, i_050fda),
    (0x050FE0, i_050fe0),
    (0x050FE4, i_050fe4),
    (0x050FE8, i_050fe8),
    (0x050FEE, i_050fee),
    (0x050FF2, i_050ff2),
    (0x050FF6, i_050ff6),
    (0x050FF8, i_050ff8),
    (0x050FFC, i_050ffc),
    (0x051000, i_051000),
    (0x051004, i_051004),
    (0x051006, i_051006),
    (0x05100C, i_05100c),
    (0x05100E, i_05100e),
    (0x051014, i_051014),
    (0x05101A, i_05101a),
    (0x05101E, i_05101e),
    (0x051024, i_051024),
    (0x05102A, i_05102a),
    (0x05102E, i_05102e),
    (0x051034, i_051034),
    (0x051038, i_051038),
    (0x05103A, i_05103a),
    (0x05103E, i_05103e),
    (0x051044, i_051044),
    (0x05104A, i_05104a),
    (0x051050, i_051050),
    (0x051056, i_051056),
    (0x05105C, i_05105c),
    (0x051060, i_051060),
    (0x051066, i_051066),
    (0x0512B4, i_0512b4),
    (0x0512BA, i_0512ba),
    (0x0512C0, i_0512c0),
    (0x0512C6, i_0512c6),
    (0x0512CC, i_0512cc),
    (0x0512CE, i_0512ce),
    (0x0512D2, i_0512d2),
    (0x0512D8, i_0512d8),
    (0x0512DC, i_0512dc),
    (0x0512E0, i_0512e0),
    (0x0512E4, i_0512e4),
    (0x0512E8, i_0512e8),
    (0x0512EC, i_0512ec),
    (0x0512F2, i_0512f2),
    (0x0512F6, i_0512f6),
    (0x0512FC, i_0512fc),
    (0x051300, i_051300),
    (0x051306, i_051306),
    (0x05130A, i_05130a),
    (0x051310, i_051310),
    (0x051314, i_051314),
    (0x051318, i_051318),
    (0x05131E, i_05131e),
    (0x051322, i_051322),
    (0x051324, i_051324),
    (0x051328, i_051328),
    (0x05132C, i_05132c),
    (0x051330, i_051330),
    (0x051332, i_051332),
    (0x051334, i_051334),
    (0x05133A, i_05133a),
    (0x05133E, i_05133e),
    (0x051340, i_051340),
    (0x051344, i_051344),
    (0x051348, i_051348),
    (0x05134E, i_05134e),
    (0x051354, i_051354),
    (0x05135A, i_05135a),
    (0x05135E, i_05135e),
    (0x051362, i_051362),
    (0x051364, i_051364),
    (0x05136A, i_05136a),
    (0x05136E, i_05136e),
    (0x051372, i_051372),
    (0x051374, i_051374),
    (0x05137A, i_05137a),
    (0x051380, i_051380),
    (0x051386, i_051386),
    (0x05138A, i_05138a),
    (0x05138E, i_05138e),
    (0x051390, i_051390),
    (0x051392, i_051392),
    (0x051396, i_051396),
    (0x05139C, i_05139c),
    (0x0513A2, i_0513a2),
    (0x0513A8, i_0513a8),
    (0x0513AC, i_0513ac),
    (0x0513B2, i_0513b2),
    (0x0513B6, i_0513b6),
    (0x0513BA, i_0513ba),
    (0x0513C0, i_0513c0),
    (0x0513C6, i_0513c6),
    (0x0513CC, i_0513cc),
    (0x0513D0, i_0513d0),
    (0x0513D6, i_0513d6),
    (0x0513DA, i_0513da),
    (0x0513DE, i_0513de),
    (0x0513E4, i_0513e4),
    (0x0513EA, i_0513ea),
    (0x0513F0, i_0513f0),
    (0x0513F4, i_0513f4),
    (0x0513FA, i_0513fa),
    (0x051402, i_051402),
    (0x051408, i_051408),
    (0x05140E, i_05140e),
    (0x051414, i_051414),
    (0x05141A, i_05141a),
    (0x05141E, i_05141e),
    (0x051424, i_051424),
    (0x05142A, i_05142a),
    (0x051430, i_051430),
    (0x051434, i_051434),
    (0x051438, i_051438),
    (0x05143E, i_05143e),
    (0x051444, i_051444),
    (0x05144A, i_05144a),
    (0x05144E, i_05144e),
    (0x051450, i_051450),
    (0x051452, i_051452),
    (0x051458, i_051458),
    (0x05145E, i_05145e),
    (0x051462, i_051462),
    (0x051468, i_051468),
    (0x05146E, i_05146e),
    (0x051472, i_051472),
    (0x051476, i_051476),
    (0x05147C, i_05147c),
    (0x05147E, i_05147e),
    (0x051484, i_051484),
    (0x05148A, i_05148a),
    (0x051530, i_051530),
    (0x051534, i_051534),
    (0x05153A, i_05153a),
    (0x051540, i_051540),
    (0x051546, i_051546),
    (0x05154C, i_05154c),
    (0x051552, i_051552),
    (0x051554, i_051554),
    (0x051558, i_051558),
    (0x05155E, i_05155e),
    (0x051562, i_051562),
    (0x051566, i_051566),
    (0x05156A, i_05156a),
    (0x05156E, i_05156e),
    (0x051574, i_051574),
    (0x051578, i_051578),
    (0x05157A, i_05157a),
    (0x05157E, i_05157e),
    (0x051584, i_051584),
    (0x05158A, i_05158a),
    (0x05158E, i_05158e),
    (0x051592, i_051592),
    (0x051596, i_051596),
    (0x05159A, i_05159a),
    (0x05159E, i_05159e),
    (0x0515A2, i_0515a2),
    (0x0515A6, i_0515a6),
    (0x0515AA, i_0515aa),
    (0x0515AE, i_0515ae),
    (0x0515B0, i_0515b0),
    (0x0515B6, i_0515b6),
    (0x0515BA, i_0515ba),
    (0x0515BE, i_0515be),
    (0x0515C4, i_0515c4),
    (0x0515CA, i_0515ca),
    (0x0515CE, i_0515ce),
    (0x0515D4, i_0515d4),
    (0x0515DA, i_0515da),
    (0x0515DE, i_0515de),
    (0x0515E2, i_0515e2),
    (0x0515E6, i_0515e6),
    (0x0515E8, i_0515e8),
    (0x0515EE, i_0515ee),
    (0x0515F2, i_0515f2),
    (0x0515F4, i_0515f4),
    (0x0515FA, i_0515fa),
    (0x0515FC, i_0515fc),
    (0x051600, i_051600),
    (0x051604, i_051604),
    (0x051608, i_051608),
    (0x05160C, i_05160c),
    (0x051610, i_051610),
    (0x051612, i_051612),
    (0x051616, i_051616),
    (0x05162C, i_05162c),
    (0x051634, i_051634),
    (0x05163A, i_05163a),
    (0x051640, i_051640),
    (0x051648, i_051648),
    (0x05164A, i_05164a),
    (0x051650, i_051650),
    (0x051656, i_051656),
    (0x051658, i_051658),
    (0x05165A, i_05165a),
    (0x051660, i_051660),
    (0x051666, i_051666),
    (0x05166A, i_05166a),
    (0x051670, i_051670),
    (0x051674, i_051674),
    (0x05167C, i_05167c),
    (0x051680, i_051680),
    (0x051688, i_051688),
    (0x05168E, i_05168e),
    (0x051694, i_051694),
    (0x051696, i_051696),
    (0x05169C, i_05169c),
    (0x0516A2, i_0516a2),
    (0x0516A4, i_0516a4),
    (0x0516AA, i_0516aa),
    (0x0516AE, i_0516ae),
    (0x0516B2, i_0516b2),
    (0x0516B4, i_0516b4),
    (0x0516B8, i_0516b8),
    (0x0516BA, i_0516ba),
    (0x0516C0, i_0516c0),
    (0x0516C8, i_0516c8),
    (0x0516D0, i_0516d0),
    (0x0516D8, i_0516d8),
    (0x0516DA, i_0516da),
    (0x0516DE, i_0516de),
    (0x0516E4, i_0516e4),
    (0x0516E8, i_0516e8),
    (0x0516EC, i_0516ec),
    (0x0516F2, i_0516f2),
    (0x0516F8, i_0516f8),
    (0x0516FC, i_0516fc),
    (0x051700, i_051700),
    (0x051706, i_051706),
    (0x05170A, i_05170a),
    (0x05170C, i_05170c),
    (0x051712, i_051712),
    (0x051718, i_051718),
    (0x05171C, i_05171c),
    (0x051722, i_051722),
    (0x051726, i_051726),
    (0x05172C, i_05172c),
    (0x051730, i_051730),
    (0x051736, i_051736),
    (0x05173A, i_05173a),
    (0x05173C, i_05173c),
    (0x05173E, i_05173e),
    (0x051740, i_051740),
    (0x051744, i_051744),
    (0x051748, i_051748),
    (0x05174A, i_05174a),
    (0x05174C, i_05174c),
    (0x051752, i_051752),
    (0x051754, i_051754),
    (0x051756, i_051756),
    (0x051758, i_051758),
    (0x05175E, i_05175e),
    (0x051762, i_051762),
    (0x051766, i_051766),
    (0x05176A, i_05176a),
    (0x051770, i_051770),
    (0x051774, i_051774),
    (0x051778, i_051778),
    (0x05177C, i_05177c),
    (0x05177E, i_05177e),
    (0x051782, i_051782),
    (0x051786, i_051786),
    (0x051788, i_051788),
    (0x05178C, i_05178c),
    (0x05178E, i_05178e),
    (0x051792, i_051792),
    (0x051796, i_051796),
    (0x05179A, i_05179a),
    (0x05179E, i_05179e),
    (0x0517A2, i_0517a2),
    (0x0517A4, i_0517a4),
    (0x0517A8, i_0517a8),
    (0x0517AA, i_0517aa),
    (0x0517AE, i_0517ae),
    (0x0517B4, i_0517b4),
    (0x0517B8, i_0517b8),
    (0x0517BE, i_0517be),
    (0x0517C4, i_0517c4),
    (0x0517C6, i_0517c6),
    (0x0517CC, i_0517cc),
    (0x0517D0, i_0517d0),
    (0x0517D6, i_0517d6),
    (0x0517DC, i_0517dc),
    (0x0517DE, i_0517de),
    (0x0517E4, i_0517e4),
    (0x0517E8, i_0517e8),
    (0x0517EC, i_0517ec),
    (0x0517F0, i_0517f0),
    (0x0517F2, i_0517f2),
    (0x0517F4, i_0517f4),
    (0x0517F8, i_0517f8),
    (0x0517FA, i_0517fa),
    (0x0517FC, i_0517fc),
    (0x0517FE, i_0517fe),
    (0x051804, i_051804),
    (0x05180A, i_05180a),
    (0x05180C, i_05180c),
    (0x05180E, i_05180e),
    (0x051812, i_051812),
    (0x051814, i_051814),
    (0x051816, i_051816),
    (0x051818, i_051818),
    (0x05181E, i_05181e),
    (0x051820, i_051820),
    (0x051824, i_051824),
    (0x051826, i_051826),
    (0x051828, i_051828),
    (0x05182A, i_05182a),
    (0x05182C, i_05182c),
    (0x05182E, i_05182e),
    (0x051832, i_051832),
    (0x051834, i_051834),
    (0x051836, i_051836),
    (0x051838, i_051838),
    (0x05183A, i_05183a),
    (0x05183C, i_05183c),
    (0x051840, i_051840),
    (0x051842, i_051842),
    (0x051844, i_051844),
    (0x051846, i_051846),
    (0x051848, i_051848),
    (0x05184A, i_05184a),
    (0x05184E, i_05184e),
    (0x051850, i_051850),
    (0x051852, i_051852),
    (0x051854, i_051854),
    (0x051856, i_051856),
    (0x051858, i_051858),
    (0x05185C, i_05185c),
    (0x05185E, i_05185e),
    (0x051860, i_051860),
    (0x051862, i_051862),
    (0x051864, i_051864),
    (0x051866, i_051866),
    (0x051868, i_051868),
    (0x05186A, i_05186a),
    (0x05186C, i_05186c),
    (0x05186E, i_05186e),
    (0x051870, i_051870),
    (0x051872, i_051872),
    (0x051874, i_051874),
    (0x051876, i_051876),
    (0x051878, i_051878),
    (0x05187A, i_05187a),
    (0x05187C, i_05187c),
    (0x05187E, i_05187e),
    (0x051880, i_051880),
    (0x051882, i_051882),
    (0x051884, i_051884),
    (0x051886, i_051886),
    (0x051888, i_051888),
    (0x05188A, i_05188a),
    (0x051914, i_051914),
    (0x05191A, i_05191a),
    (0x05191E, i_05191e),
    (0x051924, i_051924),
    (0x051928, i_051928),
    (0x05192E, i_05192e),
    (0x051932, i_051932),
    (0x051936, i_051936),
    (0x05193A, i_05193a),
    (0x05193E, i_05193e),
    (0x051942, i_051942),
    (0x051946, i_051946),
    (0x05194C, i_05194c),
    (0x051950, i_051950),
    (0x051954, i_051954),
    (0x051956, i_051956),
    (0x05195A, i_05195a),
    (0x051960, i_051960),
    (0x051964, i_051964),
    (0x05196A, i_05196a),
    (0x051970, i_051970),
    (0x051972, i_051972),
    (0x051976, i_051976),
    (0x051978, i_051978),
    (0x05197C, i_05197c),
    (0x051980, i_051980),
    (0x051982, i_051982),
    (0x051984, i_051984),
    (0x051986, i_051986),
    (0x051988, i_051988),
    (0x05198C, i_05198c),
    (0x051990, i_051990),
    (0x051994, i_051994),
    (0x051998, i_051998),
    (0x05199A, i_05199a),
    (0x05199E, i_05199e),
    (0x0519A2, i_0519a2),
    (0x0519A6, i_0519a6),
    (0x0519AA, i_0519aa),
    (0x0519AE, i_0519ae),
    (0x0519B0, i_0519b0),
    (0x0519B2, i_0519b2),
    (0x0519B4, i_0519b4),
    (0x0519B6, i_0519b6),
    (0x0519B8, i_0519b8),
    (0x0519BC, i_0519bc),
    (0x0519BE, i_0519be),
    (0x0519C4, i_0519c4),
    (0x0519C8, i_0519c8),
    (0x0519CC, i_0519cc),
    (0x0519D0, i_0519d0),
    (0x0519D4, i_0519d4),
    (0x0519DA, i_0519da),
    (0x0519DE, i_0519de),
    (0x0519E4, i_0519e4),
    (0x0519EA, i_0519ea),
    (0x0519EE, i_0519ee),
    (0x0519F2, i_0519f2),
    (0x0519F6, i_0519f6),
    (0x0519FA, i_0519fa),
    (0x0519FC, i_0519fc),
    (0x0519FE, i_0519fe),
    (0x051A00, i_051a00),
    (0x051A02, i_051a02),
    (0x051A06, i_051a06),
    (0x051A0A, i_051a0a),
    (0x051A0E, i_051a0e),
    (0x051A10, i_051a10),
    (0x051A14, i_051a14),
    (0x051A16, i_051a16),
    (0x051A18, i_051a18),
    (0x051A1A, i_051a1a),
    (0x051A1C, i_051a1c),
    (0x051A20, i_051a20),
    (0x051A26, i_051a26),
    (0x051A28, i_051a28),
    (0x051A2C, i_051a2c),
    (0x051A30, i_051a30),
    (0x051A36, i_051a36),
    (0x051A3A, i_051a3a),
    (0x051A40, i_051a40),
    (0x051A44, i_051a44),
    (0x051A48, i_051a48),
    (0x051A4C, i_051a4c),
    (0x051A50, i_051a50),
    (0x051A54, i_051a54),
    (0x051A5A, i_051a5a),
    (0x051A5E, i_051a5e),
    (0x051A64, i_051a64),
    (0x051A6A, i_051a6a),
    (0x051A70, i_051a70),
    (0x051A72, i_051a72),
    (0x051A76, i_051a76),
    (0x051A78, i_051a78),
    (0x051A7A, i_051a7a),
    (0x051A7C, i_051a7c),
    (0x051A80, i_051a80),
    (0x051A84, i_051a84),
    (0x051A86, i_051a86),
    (0x051A8A, i_051a8a),
    (0x051A8C, i_051a8c),
    (0x051A90, i_051a90),
    (0x051A94, i_051a94),
    (0x051A98, i_051a98),
    (0x051A9C, i_051a9c),
    (0x051AA2, i_051aa2),
    (0x051AA4, i_051aa4),
    (0x051AA8, i_051aa8),
    (0x051AAE, i_051aae),
    (0x051AB2, i_051ab2),
    (0x051AB6, i_051ab6),
    (0x051AB8, i_051ab8),
    (0x051ABC, i_051abc),
    (0x051ABE, i_051abe),
    (0x051AC2, i_051ac2),
    (0x051AC6, i_051ac6),
    (0x051ACA, i_051aca),
    (0x051AD0, i_051ad0),
    (0x051AD6, i_051ad6),
    (0x051ADC, i_051adc),
    (0x051AE0, i_051ae0),
    (0x051AE4, i_051ae4),
    (0x051AE8, i_051ae8),
    (0x051AEC, i_051aec),
    (0x051AF0, i_051af0),
    (0x051AF4, i_051af4),
    (0x051AF8, i_051af8),
    (0x051AFA, i_051afa),
    (0x051AFE, i_051afe),
    (0x051B02, i_051b02),
    (0x051B06, i_051b06),
    (0x051B0A, i_051b0a),
    (0x051B0E, i_051b0e),
    (0x051B12, i_051b12),
    (0x051B14, i_051b14),
    (0x051B16, i_051b16),
    (0x051B1A, i_051b1a),
    (0x051B1C, i_051b1c),
    (0x051B20, i_051b20),
    (0x051B24, i_051b24),
    (0x051B28, i_051b28),
    (0x051B2A, i_051b2a),
    (0x051B2E, i_051b2e),
    (0x051B32, i_051b32),
    (0x051B36, i_051b36),
    (0x051B38, i_051b38),
    (0x051B3C, i_051b3c),
    (0x051B3E, i_051b3e),
    (0x051B42, i_051b42),
    (0x051B46, i_051b46),
    (0x051B48, i_051b48),
    (0x051B4A, i_051b4a),
    (0x051B4E, i_051b4e),
    (0x051B52, i_051b52),
    (0x051B56, i_051b56),
    (0x051B5A, i_051b5a),
    (0x051B5C, i_051b5c),
    (0x051B5E, i_051b5e),
    (0x051B62, i_051b62),
    (0x051B66, i_051b66),
    (0x051B6A, i_051b6a),
    (0x051B6E, i_051b6e),
    (0x051B72, i_051b72),
    (0x051B76, i_051b76),
    (0x051B7A, i_051b7a),
    (0x051B7C, i_051b7c),
    (0x051B7E, i_051b7e),
    (0x051B80, i_051b80),
    (0x051B84, i_051b84),
    (0x051B86, i_051b86),
    (0x051B8A, i_051b8a),
    (0x051B8C, i_051b8c),
    (0x051B8E, i_051b8e),
    (0x051B90, i_051b90),
    (0x051B92, i_051b92),
    (0x051B96, i_051b96),
    (0x051B98, i_051b98),
    (0x051B9C, i_051b9c),
    (0x051B9E, i_051b9e),
    (0x051BA0, i_051ba0),
    (0x051BA2, i_051ba2),
];
