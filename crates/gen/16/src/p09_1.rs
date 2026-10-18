// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p09_1) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $096992: jsr $43568.l
pub fn i_096992(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96998;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x96998);
    cpu.pc = 0x43568;
    16
}
/// $096998: move.b #$0, $10e39e.l
pub fn i_096998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x969A0;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E39E, v1 as u8);
    16
}
/// $0969A0: lea.l $969a6(pc), a1
pub fn i_0969a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x969A4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x969A6;
    16
}
/// $0969A4: move.l a1, (a6)
pub fn i_0969a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x969A6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0969A6: jsr $967c0(pc)
pub fn i_0969a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x969AA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x969AA);
    cpu.pc = 0x967C0;
    16
}
/// $0969AA: jsr $436de.l
pub fn i_0969aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x969B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x969B0);
    cpu.pc = 0x436DE;
    16
}
/// $0969B0: jsr $96a80(pc)
pub fn i_0969b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x969B4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x969B4);
    cpu.pc = 0x96A80;
    16
}
/// $0969B4: jsr $96b24(pc)
pub fn i_0969b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x969B8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x969B8);
    cpu.pc = 0x96B24;
    16
}
/// $0969B8: move.b #$1, $10e39a.l
pub fn i_0969b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x969C0;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E39A, v1 as u8);
    16
}
/// $0969C0: rts
pub fn i_0969c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x969C2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AttractCuller_Cam0_0969C2  ($0969C2, 76 B, frontend/attract_sub_helpers_0967c0.s)
/// $0969C2: move.b $21(a6), d0
pub fn i_0969c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x969C6;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0969C6: jsr $96b7e(pc)
pub fn i_0969c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x969CA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x969CA);
    cpu.pc = 0x96B7E;
    16
}
/// $0969CA: clr.l d4
pub fn i_0969ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x969CC;
    cpu.ir = 0x4284;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[4] = r2;
    4
}
/// $0969CC: move.w $70(a6), d4
pub fn i_0969cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x969D0;
    cpu.ir = 0x382E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0969D0: cmpi.w #$ffff, (a0, d4.w)
pub fn i_0969d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x969D6;
    cpu.ir = 0xC70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFFF, Size::Word, false);
    18
}
/// $0969D6: beq.w $96a0c
pub fn i_0969d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x969DA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x96A0C; return 10; }
    8
}
/// $0969DA: move.w $2(a0, d4.w), d0
pub fn i_0969da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x969DE;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $0969DE: move.l $106f50.l, d1
pub fn i_0969de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x969E4;
    cpu.ir = 0x2239;
    let v2 = bus.read32(0x106F50);
    cpu.d[1] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $0969E4: swap d1
pub fn i_0969e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x969E6;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0969E6: addi.w #$140, d1
pub fn i_0969e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x969EA;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x140, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0969EA: sub.w d1, d0
pub fn i_0969ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x969EC;
    cpu.ir = 0x9041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0969EC: cmpi.w #$0, d0
pub fn i_0969ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x969F0;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $0969F0: bge.w $96a0c
pub fn i_0969f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x969F4;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x96A0C; return 10; }
    8
}
/// $0969F4: movea.l a0, a1
pub fn i_0969f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x969F6;
    cpu.ir = 0x2248;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $0969F6: andi.l #$ffff, d4
pub fn i_0969f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x969FC;
    cpu.ir = 0x284;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[4] = r2;
    8
}
/// $0969FC: adda.l d4, a1
pub fn i_0969fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x969FE;
    cpu.ir = 0xD3C4;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $0969FE: jsr $5dcce.l
pub fn i_0969fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x96A04);
    cpu.pc = 0x5DCCE;
    16
}
/// $096A04: addi.w #$14, $70(a6)
pub fn i_096a04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A0A;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x14, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $096A0A: bra.b $969c2
pub fn i_096a0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A0C;
    cpu.ir = 0x60B6;
    cpu.pc = 0x969C2;
    10
}
/// $096A0C: rts
pub fn i_096a0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A0E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AttractCuller_Cam1_096A0E  ($096A0E, 76 B, frontend/attract_sub_helpers_0967c0.s)
/// $096A0E: move.b $21(a6), d0
pub fn i_096a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A12;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $096A12: jsr $96b7e(pc)
pub fn i_096a12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A16;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x96A16);
    cpu.pc = 0x96B7E;
    16
}
/// $096A16: clr.l d4
pub fn i_096a16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A18;
    cpu.ir = 0x4284;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[4] = r2;
    4
}
/// $096A18: move.w $70(a6), d4
pub fn i_096a18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A1C;
    cpu.ir = 0x382E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $096A1C: cmpi.w #$ffff, (a0, d4.w)
pub fn i_096a1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A22;
    cpu.ir = 0xC70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFFF, Size::Word, false);
    18
}
/// $096A22: beq.w $96a58
pub fn i_096a22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A26;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x96A58; return 10; }
    8
}
/// $096A26: move.w $2(a0, d4.w), d0
pub fn i_096a26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A2A;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $096A2A: move.l $106f5c.l, d1
pub fn i_096a2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A30;
    cpu.ir = 0x2239;
    let v2 = bus.read32(0x106F5C);
    cpu.d[1] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $096A30: swap d1
pub fn i_096a30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A32;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $096A32: addi.w #$140, d1
pub fn i_096a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A36;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x140, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $096A36: sub.w d1, d0
pub fn i_096a36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A38;
    cpu.ir = 0x9041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $096A38: cmpi.w #$0, d0
pub fn i_096a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A3C;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $096A3C: bge.w $96a58
pub fn i_096a3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A40;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x96A58; return 10; }
    8
}
/// $096A40: movea.l a0, a1
pub fn i_096a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A42;
    cpu.ir = 0x2248;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $096A42: andi.l #$ffff, d4
pub fn i_096a42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A48;
    cpu.ir = 0x284;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[4] = r2;
    8
}
/// $096A48: adda.l d4, a1
pub fn i_096a48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A4A;
    cpu.ir = 0xD3C4;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $096A4A: jsr $5dcce.l
pub fn i_096a4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A50;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x96A50);
    cpu.pc = 0x5DCCE;
    16
}
/// $096A50: addi.w #$14, $70(a6)
pub fn i_096a50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A56;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x14, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $096A56: bra.b $96a0e
pub fn i_096a56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A58;
    cpu.ir = 0x60B6;
    cpu.pc = 0x96A0E;
    10
}
/// $096A58: rts
pub fn i_096a58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A5A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Viewport_CoordToScreen_096A5A  ($096A5A, 38 B, frontend/attract_sub_helpers_0967c0.s)
/// $096A5A: move.l $106f50.l, d2
pub fn i_096a5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A60;
    cpu.ir = 0x2439;
    let v2 = bus.read32(0x106F50);
    cpu.d[2] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $096A60: asr.l #$8, d2
pub fn i_096a60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A62;
    cpu.ir = 0xE082;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[2] = r;
    22
}
/// $096A62: asr.l #$8, d2
pub fn i_096a62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A64;
    cpu.ir = 0xE082;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[2] = r;
    22
}
/// $096A64: sub.w d2, d0
pub fn i_096a64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A66;
    cpu.ir = 0x9042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $096A66: addq.w #$1, d0
pub fn i_096a66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A68;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $096A68: move.l $106f54.l, d3
pub fn i_096a68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A6E;
    cpu.ir = 0x2639;
    let v2 = bus.read32(0x106F54);
    cpu.d[3] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $096A6E: asr.l #$8, d3
pub fn i_096a6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A70;
    cpu.ir = 0xE083;
    let h1 = cpu.d[3] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[3] = r;
    22
}
/// $096A70: asr.l #$8, d3
pub fn i_096a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A72;
    cpu.ir = 0xE083;
    let h1 = cpu.d[3] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[3] = r;
    22
}
/// $096A72: sub.w d3, d1
pub fn i_096a72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A74;
    cpu.ir = 0x9243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $096A74: move.w #$180, d4
pub fn i_096a74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A78;
    cpu.ir = 0x383C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $096A78: sub.w d1, d4
pub fn i_096a78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A7A;
    cpu.ir = 0x9841;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $096A7A: move.w d4, d1
pub fn i_096a7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A7C;
    cpu.ir = 0x3204;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $096A7C: addq.w #$1, d1
pub fn i_096a7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A7E;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $096A7E: rts
pub fn i_096a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A80;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Debug_DrawHUDVars_096A80  ($096A80, 164 B, video/debug_draw_hud_vars_096a80.s)
/// $096A80: btst.b #$0, $100001.l
pub fn i_096a80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A88;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((0) % 8);
    let v2 = bus.read8(0x100001) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $096A88: beq.w $96b22
pub fn i_096a88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A8C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x96B22; return 10; }
    8
}
/// $096A8C: movea.l #$7146, a1
pub fn i_096a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A92;
    cpu.ir = 0x227C;
    let v1: u32 = 0x7146;
    cpu.a[1] = v1;
    4
}
/// $096A92: move.w $106e88.l, d0
pub fn i_096a92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A98;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106E88) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $096A98: jsr $5d6c2.l
pub fn i_096a98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96A9E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x96A9E);
    cpu.pc = 0x5D6C2;
    16
}
/// $096A9E: movea.w #$72e3, a1
pub fn i_096a9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96AA2;
    cpu.ir = 0x327C;
    let v1: u32 = 0x72E3;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $096AA2: move.w $106f5c.l, d0
pub fn i_096aa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96AA8;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106F5C) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $096AA8: jsr $5d6c2.l
pub fn i_096aa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96AAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x96AAE);
    cpu.pc = 0x5D6C2;
    16
}
/// $096AAE: movea.w #$7383, a1
pub fn i_096aae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96AB2;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7383;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $096AB2: move.l $106f50.l, d0
pub fn i_096ab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96AB8;
    cpu.ir = 0x2039;
    let v2 = bus.read32(0x106F50);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $096AB8: swap d0
pub fn i_096ab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96ABA;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $096ABA: jsr $5d6c2.l
pub fn i_096aba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96AC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x96AC0);
    cpu.pc = 0x5D6C2;
    16
}
/// $096AC0: movea.w #$7423, a1
pub fn i_096ac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96AC4;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7423;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $096AC4: move.l $106f54.l, d0
pub fn i_096ac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96ACA;
    cpu.ir = 0x2039;
    let v2 = bus.read32(0x106F54);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $096ACA: swap d0
pub fn i_096aca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96ACC;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $096ACC: jsr $5d6c2.l
pub fn i_096acc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96AD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x96AD2);
    cpu.pc = 0x5D6C2;
    16
}
/// $096AD2: movea.w #$7384, a1
pub fn i_096ad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96AD6;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7384;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $096AD6: move.l $106f50.l, d0
pub fn i_096ad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96ADC;
    cpu.ir = 0x2039;
    let v2 = bus.read32(0x106F50);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $096ADC: swap d0
pub fn i_096adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96ADE;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $096ADE: andi.l #$ffff, d0
pub fn i_096ade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96AE4;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $096AE4: jsr $5d8f2.l
pub fn i_096ae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96AEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x96AEA);
    cpu.pc = 0x5D8F2;
    16
}
/// $096AEA: jsr $5d6c2.l
pub fn i_096aea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96AF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x96AF0);
    cpu.pc = 0x5D6C2;
    16
}
/// $096AF0: movea.w #$7424, a1
pub fn i_096af0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96AF4;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7424;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $096AF4: move.l $106f54.l, d0
pub fn i_096af4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96AFA;
    cpu.ir = 0x2039;
    let v2 = bus.read32(0x106F54);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $096AFA: swap d0
pub fn i_096afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96AFC;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $096AFC: andi.l #$ffff, d0
pub fn i_096afc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B02;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $096B02: jsr $5d8f2.l
pub fn i_096b02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x96B08);
    cpu.pc = 0x5D8F2;
    16
}
/// $096B08: jsr $5d6c2.l
pub fn i_096b08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x96B0E);
    cpu.pc = 0x5D6C2;
    16
}
/// $096B0E: movea.w #$7385, a1
pub fn i_096b0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B12;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7385;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $096B12: move.b $10e39c.l, d0
pub fn i_096b12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B18;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10E39C) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $096B18: andi.w #$ff, d0
pub fn i_096b18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B1C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $096B1C: jsr $5d6c2.l
pub fn i_096b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B22;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x96B22);
    cpu.pc = 0x5D6C2;
    16
}
/// $096B22: rts
pub fn i_096b22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B24;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== DebugTriggers_TwoBits_096B24  ($096B24, 90 B, frontend/attract_sub_helpers_0967c0.s)
/// $096B24: btst.b #$2, $100001.l
pub fn i_096b24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B2C;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((2) % 8);
    let v2 = bus.read8(0x100001) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $096B2C: beq.w $96b50
pub fn i_096b2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B30;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x96B50; return 10; }
    8
}
/// $096B30: jsr $5cd18.l
pub fn i_096b30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x96B36);
    cpu.pc = 0x5CD18;
    16
}
/// $096B36: bcc.w $96b50
pub fn i_096b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B3A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x96B50; return 10; }
    8
}
/// $096B3A: jsr $5d11c.l
pub fn i_096b3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B40;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x96B40);
    cpu.pc = 0x5D11C;
    16
}
/// $096B40: bcc.w $96b50
pub fn i_096b40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B44;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x96B50; return 10; }
    8
}
/// $096B44: lea.l $86586.l, a1
pub fn i_096b44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B4A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x86586;
    20
}
/// $096B4A: jsr $4ae.l
pub fn i_096b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B50;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x96B50);
    cpu.pc = 0x4AE;
    16
}
/// $096B50: btst.b #$7, $100001.l
pub fn i_096b50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B58;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((7) % 8);
    let v2 = bus.read8(0x100001) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $096B58: beq.w $96b7c
pub fn i_096b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B5C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x96B7C; return 10; }
    8
}
/// $096B5C: jsr $5cd18.l
pub fn i_096b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x96B62);
    cpu.pc = 0x5CD18;
    16
}
/// $096B62: bcc.w $96b7c
pub fn i_096b62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B66;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x96B7C; return 10; }
    8
}
/// $096B66: jsr $5d11c.l
pub fn i_096b66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x96B6C);
    cpu.pc = 0x5D11C;
    16
}
/// $096B6C: bcc.w $96b7c
pub fn i_096b6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B70;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x96B7C; return 10; }
    8
}
/// $096B70: lea.l $86582.l, a1
pub fn i_096b70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B76;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x86582;
    20
}
/// $096B76: jsr $4ae.l
pub fn i_096b76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B7C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x96B7C);
    cpu.pc = 0x4AE;
    16
}
/// $096B7C: rts
pub fn i_096b7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B7E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClampAndLookup8_096B7E  ($096B7E, 62 B, frontend/attract_sub_helpers_0967c0.s)
/// $096B7E: cmpi.b #$7, d0
pub fn i_096b7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B82;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x7, Size::Byte, false);
    8
}
/// $096B82: ble.w $96b8a
pub fn i_096b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B86;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x96B8A; return 10; }
    8
}
/// $096B86: move.b #$7, d0
pub fn i_096b86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B8A;
    cpu.ir = 0x103C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $096B8A: andi.l #$ff, d0
pub fn i_096b8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B90;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $096B90: asl.l #$2, d0
pub fn i_096b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B92;
    cpu.ir = 0xE580;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Long, 0, true);
    cpu.d[0] = r;
    10
}
/// $096B92: lea.l $96b9c(pc), a0
pub fn i_096b92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B96;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x96B9C;
    16
}
/// $096B96: movea.l (a0, d0.w), a0
pub fn i_096b96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B9A;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $096B9A: rts
pub fn i_096b9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96B9C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ChildRank_CmpByte10_097720  ($097720, 16 B, frontend/attract_sprite_lists_096bbc.s)
/// $097720: movea.l $8(a6), a1
pub fn i_097720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97724;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $097724: move.b $10(a6), d0
pub fn i_097724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97728;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $097728: cmp.b $10(a1), d0
pub fn i_097728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9772C;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $09772C: bcs.w $97736
pub fn i_09772c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97730;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x97736; return 10; }
    8
}
// ===== ClearXN_097730  ($097730, 6 B, thunks/ccr_helpers.c)
/// $097730: andi.b #$ee, ccr
pub fn i_097730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97734;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $097734: rts
pub fn i_097734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97736;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_097736  ($097736, 6 B, thunks/ccr_helpers.c)
/// $097736: ori.b #$11, ccr
pub fn i_097736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9773A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $09773A: rts
pub fn i_09773a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9773C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HiScore_LoadDefaults_09773c  ($09773C, 134 B, frontend/hiscore_memcard_options_09773c.s)
/// $09773C: lea.l $2f53b2.l, a0
pub fn i_09773c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97742;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F53B2;
    20
}
/// $097742: lea.l $100002.l, a1
pub fn i_097742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97748;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x100002;
    20
}
/// $097748: move.b (a0)+, (a1)+
pub fn i_097748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9774A;
    cpu.ir = 0x12D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $09774A: move.b (a0)+, (a1)+
pub fn i_09774a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9774C;
    cpu.ir = 0x12D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $09774C: move.l (a0)+, (a1)+
pub fn i_09774c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9774E;
    cpu.ir = 0x22D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $09774E: move.w (a0)+, (a1)+
pub fn i_09774e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97750;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $097750: move.w (a0)+, (a1)+
pub fn i_097750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97752;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $097752: move.w (a0)+, (a1)+
pub fn i_097752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97754;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $097754: move.b (a0)+, (a1)+
pub fn i_097754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97756;
    cpu.ir = 0x12D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $097756: move.b (a0)+, (a1)+
pub fn i_097756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97758;
    cpu.ir = 0x12D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $097758: move.l (a0)+, (a1)+
pub fn i_097758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9775A;
    cpu.ir = 0x22D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $09775A: move.w (a0)+, (a1)+
pub fn i_09775a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9775C;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $09775C: move.w (a0)+, (a1)+
pub fn i_09775c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9775E;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $09775E: move.w (a0)+, (a1)+
pub fn i_09775e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97760;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $097760: move.b (a0)+, (a1)+
pub fn i_097760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97762;
    cpu.ir = 0x12D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $097762: move.b (a0)+, (a1)+
pub fn i_097762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97764;
    cpu.ir = 0x12D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $097764: move.l (a0)+, (a1)+
pub fn i_097764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97766;
    cpu.ir = 0x22D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $097766: move.w (a0)+, (a1)+
pub fn i_097766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97768;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $097768: move.w (a0)+, (a1)+
pub fn i_097768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9776A;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $09776A: move.w (a0)+, (a1)+
pub fn i_09776a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9776C;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $09776C: move.b (a0)+, (a1)+
pub fn i_09776c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9776E;
    cpu.ir = 0x12D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $09776E: move.b (a0)+, (a1)+
pub fn i_09776e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97770;
    cpu.ir = 0x12D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $097770: move.l (a0)+, (a1)+
pub fn i_097770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97772;
    cpu.ir = 0x22D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $097772: move.w (a0)+, (a1)+
pub fn i_097772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97774;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $097774: move.w (a0)+, (a1)+
pub fn i_097774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97776;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $097776: move.w (a0)+, (a1)+
pub fn i_097776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97778;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $097778: move.b (a0)+, (a1)+
pub fn i_097778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9777A;
    cpu.ir = 0x12D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $09777A: move.b (a0)+, (a1)+
pub fn i_09777a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9777C;
    cpu.ir = 0x12D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $09777C: move.l (a0)+, (a1)+
pub fn i_09777c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9777E;
    cpu.ir = 0x22D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $09777E: move.w (a0)+, (a1)+
pub fn i_09777e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97780;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $097780: move.w (a0)+, (a1)+
pub fn i_097780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97782;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $097782: move.w (a0)+, (a1)+
pub fn i_097782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97784;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $097784: move.b (a0)+, (a1)+
pub fn i_097784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97786;
    cpu.ir = 0x12D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $097786: move.b (a0)+, (a1)+
pub fn i_097786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97788;
    cpu.ir = 0x12D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $097788: move.l (a0)+, (a1)+
pub fn i_097788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9778A;
    cpu.ir = 0x22D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $09778A: move.w (a0)+, (a1)+
pub fn i_09778a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9778C;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $09778C: move.w (a0)+, (a1)+
pub fn i_09778c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9778E;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $09778E: move.w (a0)+, (a1)+
pub fn i_09778e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97790;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $097790: move.b (a0)+, (a1)+
pub fn i_097790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97792;
    cpu.ir = 0x12D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $097792: move.b (a0)+, (a1)+
pub fn i_097792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97794;
    cpu.ir = 0x12D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $097794: move.l (a0)+, (a1)+
pub fn i_097794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97796;
    cpu.ir = 0x22D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $097796: move.w (a0)+, (a1)+
pub fn i_097796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97798;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $097798: move.w (a0)+, (a1)+
pub fn i_097798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9779A;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $09779A: move.w (a0)+, (a1)+
pub fn i_09779a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9779C;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $09779C: move.b (a0)+, (a1)+
pub fn i_09779c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9779E;
    cpu.ir = 0x12D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $09779E: move.b (a0)+, (a1)+
pub fn i_09779e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x977A0;
    cpu.ir = 0x12D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $0977A0: move.l (a0)+, (a1)+
pub fn i_0977a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x977A2;
    cpu.ir = 0x22D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $0977A2: move.w (a0)+, (a1)+
pub fn i_0977a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x977A4;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $0977A4: move.w (a0)+, (a1)+
pub fn i_0977a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x977A6;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $0977A6: move.w (a0)+, (a1)+
pub fn i_0977a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x977A8;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $0977A8: move.b (a0)+, (a1)+
pub fn i_0977a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x977AA;
    cpu.ir = 0x12D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $0977AA: move.b (a0)+, (a1)+
pub fn i_0977aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x977AC;
    cpu.ir = 0x12D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $0977AC: move.l (a0)+, (a1)+
pub fn i_0977ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x977AE;
    cpu.ir = 0x22D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $0977AE: move.w (a0)+, (a1)+
pub fn i_0977ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x977B0;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $0977B0: move.w (a0)+, (a1)+
pub fn i_0977b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x977B2;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $0977B2: move.w (a0)+, (a1)+
pub fn i_0977b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x977B4;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $0977B4: move.b (a0)+, (a1)+
pub fn i_0977b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x977B6;
    cpu.ir = 0x12D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $0977B6: move.b (a0)+, (a1)+
pub fn i_0977b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x977B8;
    cpu.ir = 0x12D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $0977B8: move.l (a0)+, (a1)+
pub fn i_0977b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x977BA;
    cpu.ir = 0x22D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $0977BA: move.w (a0)+, (a1)+
pub fn i_0977ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x977BC;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $0977BC: move.w (a0)+, (a1)+
pub fn i_0977bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x977BE;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $0977BE: move.w (a0)+, (a1)+
pub fn i_0977be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x977C0;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $0977C0: rts
pub fn i_0977c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x977C2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HiScore_Tpl_Frame_0977d6  ($0977D6, 20 B, frontend/hiscore_memcard_options_09773c.s)
/// $0977D6: lea.l $9794c(pc), a1
pub fn i_0977d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x977DA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9794C;
    16
}
/// $0977DA: jsr $4ae.l
pub fn i_0977da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x977E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x977E0);
    cpu.pc = 0x4AE;
    16
}
/// $0977E0: move.w #$3c, $80(a6)
pub fn i_0977e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x977E6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0977E6: bra.w $977fa
pub fn i_0977e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x977EA;
    cpu.ir = 0x6000;
    cpu.pc = 0x977FA;
    10
}
// ===== HiScore_Tpl_Loader_0977ea  ($0977EA, 96 B, frontend/hiscore_memcard_options_09773c.s)
/// $0977EA: lea.l $9794c(pc), a1
pub fn i_0977ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x977EE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9794C;
    16
}
/// $0977EE: jsr $4ae.l
pub fn i_0977ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x977F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x977F4);
    cpu.pc = 0x4AE;
    16
}
/// $0977F4: move.w #$3c, $80(a6)
pub fn i_0977f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x977FA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0977FA: lea.l $100002.l, a1
pub fn i_0977fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97800;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x100002;
    20
}
/// $097800: moveq #$0, d1
pub fn i_097800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97802;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $097802: move.b d1, (a1)
pub fn i_097802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97804;
    cpu.ir = 0x1281;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[1], v1 as u8);
    8
}
/// $097804: move.b d1, (a1)
pub fn i_097804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97806;
    cpu.ir = 0x1281;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[1], v1 as u8);
    8
}
/// $097806: move.b d1, (a1)
pub fn i_097806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97808;
    cpu.ir = 0x1281;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[1], v1 as u8);
    8
}
/// $097808: move.b d1, (a1)
pub fn i_097808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9780A;
    cpu.ir = 0x1281;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[1], v1 as u8);
    8
}
/// $09780A: move.b d1, (a1)
pub fn i_09780a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9780C;
    cpu.ir = 0x1281;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[1], v1 as u8);
    8
}
/// $09780C: move.b d1, (a1)
pub fn i_09780c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9780E;
    cpu.ir = 0x1281;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[1], v1 as u8);
    8
}
/// $09780E: move.b d1, (a1)
pub fn i_09780e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97810;
    cpu.ir = 0x1281;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[1], v1 as u8);
    8
}
/// $097810: move.b d1, (a1)
pub fn i_097810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97812;
    cpu.ir = 0x1281;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[1], v1 as u8);
    8
}
/// $097812: move.b d1, (a1)
pub fn i_097812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97814;
    cpu.ir = 0x1281;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[1], v1 as u8);
    8
}
/// $097814: move.b d1, (a1)
pub fn i_097814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97816;
    cpu.ir = 0x1281;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[1], v1 as u8);
    8
}
/// $097816: move.b #$b, d0
pub fn i_097816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9781A;
    cpu.ir = 0x103C;
    let v1: u32 = 0xB;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $09781A: jsr $43568.l
pub fn i_09781a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97820;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x97820);
    cpu.pc = 0x43568;
    16
}
/// $097820: moveq #$0, d0
pub fn i_097820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97822;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $097822: moveq #$0, d1
pub fn i_097822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97824;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $097824: jsr $437da.l
pub fn i_097824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9782A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9782A);
    cpu.pc = 0x437DA;
    16
}
/// $09782A: clr.b $21(a6)
pub fn i_09782a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9782E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $09782E: move.w #$0, $70(a6)
pub fn i_09782e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97834;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $097834: movea.l #$719b, a1
pub fn i_097834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9783A;
    cpu.ir = 0x227C;
    let v1: u32 = 0x719B;
    cpu.a[1] = v1;
    4
}
/// $09783A: lea.l $2f54a0.l, a2
pub fn i_09783a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97840;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2F54A0;
    20
}
/// $097840: move.w #$4300, d0
pub fn i_097840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97844;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $097844: jsr $5dad8.l
pub fn i_097844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9784A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9784A);
    cpu.pc = 0x5DAD8;
    16
}
// ===== SetTaskHandler_09784a  ($09784A, 8 B, kernel/task_handlers.c)
/// $09784A: lea.l $97852(pc), a1
pub fn i_09784a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9784E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x97852;
    16
}
/// $09784E: move.l a1, (a6)
pub fn i_09784e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97850;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $097850: rts
pub fn i_097850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97852;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HiScore_WaitLogo_097852  ($097852, 42 B, frontend/hiscore_memcard_options_09773c.s)
/// $097852: tst.b $21(a6)
pub fn i_097852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97856;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $097856: bne.w $9785c
pub fn i_097856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9785A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9785C; return 10; }
    8
}
/// $09785A: rts
pub fn i_09785a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9785C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $09785C: clr.b $20(a6)
pub fn i_09785c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97860;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $097860: movea.l #$70a6, a1
pub fn i_097860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97866;
    cpu.ir = 0x227C;
    let v1: u32 = 0x70A6;
    cpu.a[1] = v1;
    4
}
/// $097866: move.l a1, $76(a6)
pub fn i_097866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9786A;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $09786A: move.l #$100002, $72(a6)
pub fn i_09786a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97872;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x100002;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $097872: cmpi.b #$ff, $21(a6)
pub fn i_097872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97878;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $097878: bne.w $97884
pub fn i_097878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9787C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x97884; return 10; }
    8
}
// ===== SetTaskHandler_09787c  ($09787C, 8 B, kernel/task_handlers.c)
/// $09787C: lea.l $9788c(pc), a1
pub fn i_09787c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97880;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9788C;
    16
}
/// $097880: move.l a1, (a6)
pub fn i_097880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97882;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $097882: rts
pub fn i_097882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97884;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetTaskHandler_097884  ($097884, 8 B, kernel/task_handlers.c)
/// $097884: lea.l $978fa(pc), a1
pub fn i_097884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97888;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x978FA;
    16
}
/// $097888: move.l a1, (a6)
pub fn i_097888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9788A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09788A: rts
pub fn i_09788a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9788C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HiScore_HeaderDelay_09788c  ($09788C, 24 B, frontend/hiscore_memcard_options_09773c.s)
/// $09788C: addq.w #$1, $70(a6)
pub fn i_09788c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97890;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $097890: cmpi.w #$10, $70(a6)
pub fn i_097890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97896;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x10, Size::Word, false);
    16
}
/// $097896: blt.w $978aa
pub fn i_097896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9789A;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x978AA; return 10; }
    8
}
/// $09789A: move.w #$0, $70(a6)
pub fn i_09789a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x978A0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0978A0: jsr $979f8(pc)
pub fn i_0978a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x978A4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x978A4);
    cpu.pc = 0x979F8;
    16
}
// ===== SetTaskHandler_0978a4  ($0978A4, 8 B, kernel/task_handlers.c)
/// $0978A4: lea.l $978ac(pc), a1
pub fn i_0978a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x978A8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x978AC;
    16
}
/// $0978A8: move.l a1, (a6)
pub fn i_0978a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x978AA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0978AA: rts
pub fn i_0978aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x978AC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HiScore_DrawRowsStep_0978ac  ($0978AC, 78 B, frontend/hiscore_memcard_options_09773c.s)
/// $0978AC: addq.w #$1, $70(a6)
pub fn i_0978ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x978B0;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0978B0: cmpi.w #$2, $70(a6)
pub fn i_0978b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x978B6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Word, false);
    16
}
/// $0978B6: blt.w $978f8
pub fn i_0978b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x978BA;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x978F8; return 10; }
    8
}
/// $0978BA: movea.l $76(a6), a1
pub fn i_0978ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x978BE;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0978BE: movea.l $72(a6), a0
pub fn i_0978be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x978C2;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0978C2: movem.l a0-a1, -(a7)
pub fn i_0978c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x978C6;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $0978C6: jsr $97b74(pc)
pub fn i_0978c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x978CA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x978CA);
    cpu.pc = 0x97B74;
    16
}
/// $0978CA: movem.l (a7)+, a0-a1
pub fn i_0978ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x978CE;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $0978CE: adda.l #$2, a1
pub fn i_0978ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x978D4;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x2;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $0978D4: adda.l #$c, a0
pub fn i_0978d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x978DA;
    cpu.ir = 0xD1FC;
    let v1: u32 = 0xC;
    cpu.a[0] = cpu.a[0].wrapping_add(v1);
    8
}
/// $0978DA: cmpa.l #$10007a, a0
pub fn i_0978da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x978E0;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0x10007A;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $0978E0: bne.w $978ea
pub fn i_0978e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x978E4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x978EA; return 10; }
    8
}
/// $0978E4: lea.l $9792e(pc), a1
pub fn i_0978e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x978E8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9792E;
    16
}
/// $0978E8: move.l a1, (a6)
pub fn i_0978e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x978EA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0978EA: move.w #$0, $70(a6)
pub fn i_0978ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x978F0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0978F0: move.l a0, $72(a6)
pub fn i_0978f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x978F4;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0978F4: move.l a1, $76(a6)
pub fn i_0978f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x978F8;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0978F8: rts
pub fn i_0978f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x978FA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HiScore_DrawAllRows_0978fa  ($0978FA, 44 B, frontend/hiscore_memcard_options_09773c.s)
/// $0978FA: jsr $979f8(pc)
pub fn i_0978fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x978FE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x978FE);
    cpu.pc = 0x979F8;
    16
}
/// $0978FE: movea.l $76(a6), a1
pub fn i_0978fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97902;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $097902: movea.l $72(a6), a0
pub fn i_097902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97906;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $097906: move.w #$9, d5
pub fn i_097906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9790A;
    cpu.ir = 0x3A3C;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09790A: movem.l d5/a0-a1, -(a7)
pub fn i_09790a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9790E;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $09790E: jsr $97b74(pc)
pub fn i_09790e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97912;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x97912);
    cpu.pc = 0x97B74;
    16
}
/// $097912: movem.l (a7)+, d5/a0-a1
pub fn i_097912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97916;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $097916: adda.l #$2, a1
pub fn i_097916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9791C;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x2;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $09791C: adda.l #$c, a0
pub fn i_09791c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97922;
    cpu.ir = 0xD1FC;
    let v1: u32 = 0xC;
    cpu.a[0] = cpu.a[0].wrapping_add(v1);
    8
}
/// $097922: dbra d5, $9790a
pub fn i_097922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97926;
    cpu.ir = 0x51CD;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x97926; return 12; }
    let n = (cpu.d[5] as u16).wrapping_sub(1);
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x97926; return 14; }
    cpu.pc = 0x9790A;
    10
}
// ===== SetTaskHandler_097926  ($097926, 8 B, kernel/task_handlers.c)
/// $097926: lea.l $9792e(pc), a1
pub fn i_097926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9792A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9792E;
    16
}
/// $09792A: move.l a1, (a6)
pub fn i_09792a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9792C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09792C: rts
pub fn i_09792c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9792E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HiScore_WaitExit_09792e  ($09792E, 22 B, frontend/hiscore_memcard_options_09773c.s)
/// $09792E: tst.b $20(a6)
pub fn i_09792e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97932;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $097932: bne.w $9794a
pub fn i_097932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97936;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9794A; return 10; }
    8
}
/// $097936: subq.w #$1, $80(a6)
pub fn i_097936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9793A;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x80);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09793A: bne.w $9794a
pub fn i_09793a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9793E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9794A; return 10; }
    8
}
/// $09793E: clr.b $106ed2.l
pub fn i_09793e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97944;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106ED2) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106ED2, r2 as u8);
    16
}
// ===== SetTaskHandler_097944  ($097944, 8 B, kernel/task_handlers.c)
/// $097944: lea.l $9794a(pc), a1
pub fn i_097944(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97948;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9794A;
    16
}
/// $097948: move.l a1, (a6)
pub fn i_097948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9794A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09794A: rts
pub fn i_09794a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9794C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HiScore_Logo_FadeIn_09794c  ($09794C, 164 B, frontend/hiscore_memcard_options_09773c.s)
/// $09794C: move.b #$ff, $21(a6)
pub fn i_09794c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97952;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $097952: move.b #$0, $32(a6)
pub fn i_097952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97958;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $097958: move.w #$0, $70(a6)
pub fn i_097958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9795E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09795E: bra.w $97974
pub fn i_09795e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97962;
    cpu.ir = 0x6000;
    cpu.pc = 0x97974;
    10
}
/// $097974: move.w #$15f, d1
pub fn i_097974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97978;
    cpu.ir = 0x323C;
    let v1: u32 = 0x15F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $097978: jsr $236e.l
pub fn i_097978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9797E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9797E);
    cpu.pc = 0x236E;
    16
}
/// $09797E: move.w #$a0, $22(a6)
pub fn i_09797e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97984;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $097984: move.w #$180, $24(a6)
pub fn i_097984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9798A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09798A: move.b #$0, $20(a6)
pub fn i_09798a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97990;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $097990: jsr $5e9b6.l
pub fn i_097990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97996;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x97996);
    cpu.pc = 0x5E9B6;
    16
}
/// $097996: andi.w #$1, d0
pub fn i_097996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9799A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09799A: movea.l #$2f5504, a0
pub fn i_09799a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x979A0;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2F5504;
    cpu.a[0] = v1;
    4
}
/// $0979A0: lsl.w #$2, d0
pub fn i_0979a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x979A2;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $0979A2: movea.l (a0, d0.w), a0
pub fn i_0979a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x979A6;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $0979A6: cmpa.l #$ffffffff, a0
pub fn i_0979a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x979AC;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $0979AC: beq.w $979b6
pub fn i_0979ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x979B0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x979B6; return 10; }
    8
}
/// $0979B0: jsr $28cd4.l
pub fn i_0979b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x979B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x979B6);
    cpu.pc = 0x28CD4;
    16
}
/// $0979B6: lea.l $979bc(pc), a1
pub fn i_0979b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x979BA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x979BC;
    16
}
/// $0979BA: move.l a1, (a6)
pub fn i_0979ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x979BC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0979BC: tst.b $20(a6)
pub fn i_0979bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x979C0;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0979C0: bne.w $979f0
pub fn i_0979c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x979C4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x979F0; return 10; }
    8
}
/// $0979C4: addi.b #$20, $32(a6)
pub fn i_0979c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x979CA;
    cpu.ir = 0x62E;
    let ad1 = cpu.a[6].wrapping_add(0x32);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $0979CA: addi.w #$20, $70(a6)
pub fn i_0979ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x979D0;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0979D0: cmpi.w #$ff, $70(a6)
pub fn i_0979d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x979D6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Word, false);
    16
}
/// $0979D6: bcs.w $979f0
pub fn i_0979d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x979DA;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x979F0; return 10; }
    8
}
/// $0979DA: move.b #$ff, $32(a6)
pub fn i_0979da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x979E0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0979E0: move.b #$ff, $20(a6)
pub fn i_0979e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x979E6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0979E6: movea.l $c(a6), a0
pub fn i_0979e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x979EA;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0979EA: move.b $21(a6), $21(a0)
pub fn i_0979ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x979F0;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad4, v3 as u8);
    20
}
// ===== JsrAbsThunk_0979f0  ($0979F0, 8 B, thunks/jsr_abs_thunks.c)
/// $0979F0: jsr $28d70.l
pub fn i_0979f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x979F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x979F6);
    cpu.pc = 0x28D70;
    16
}
/// $0979F6: rts
pub fn i_0979f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x979F8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HiScore_DrawHeaders_0979f8  ($0979F8, 72 B, frontend/hiscore_memcard_options_09773c.s)
/// $0979F8: movea.l #$7083, a1
pub fn i_0979f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x979FE;
    cpu.ir = 0x227C;
    let v1: u32 = 0x7083;
    cpu.a[1] = v1;
    4
}
/// $0979FE: lea.l $2f547c.l, a2
pub fn i_0979fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97A04;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2F547C;
    20
}
/// $097A04: move.w #$4000, d0
pub fn i_097a04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97A08;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $097A08: moveq #$2, d1
pub fn i_097a08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97A0A;
    cpu.ir = 0x7202;
    cpu.d[1] = 0x2;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $097A0A: moveq #$2, d2
pub fn i_097a0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97A0C;
    cpu.ir = 0x7402;
    cpu.d[2] = 0x2;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $097A0C: jsr $5db1a.l
pub fn i_097a0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97A12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x97A12);
    cpu.pc = 0x5DB1A;
    16
}
/// $097A12: movea.l #$71a3, a1
pub fn i_097a12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97A18;
    cpu.ir = 0x227C;
    let v1: u32 = 0x71A3;
    cpu.a[1] = v1;
    4
}
/// $097A18: lea.l $2f5486.l, a2
pub fn i_097a18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97A1E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2F5486;
    20
}
/// $097A1E: move.w #$4000, d0
pub fn i_097a1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97A22;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $097A22: moveq #$2, d1
pub fn i_097a22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97A24;
    cpu.ir = 0x7202;
    cpu.d[1] = 0x2;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $097A24: moveq #$2, d2
pub fn i_097a24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97A26;
    cpu.ir = 0x7402;
    cpu.d[2] = 0x2;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $097A26: jsr $5db1a.l
pub fn i_097a26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97A2C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x97A2C);
    cpu.pc = 0x5DB1A;
    16
}
/// $097A2C: movea.l #$72c3, a1
pub fn i_097a2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97A32;
    cpu.ir = 0x227C;
    let v1: u32 = 0x72C3;
    cpu.a[1] = v1;
    4
}
/// $097A32: lea.l $2f5490.l, a2
pub fn i_097a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97A38;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2F5490;
    20
}
/// $097A38: move.w #$4000, d0
pub fn i_097a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97A3C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $097A3C: moveq #$2, d1
pub fn i_097a3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97A3E;
    cpu.ir = 0x7202;
    cpu.d[1] = 0x2;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $097A3E: moveq #$2, d2
pub fn i_097a3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97A40;
    cpu.ir = 0x7402;
    cpu.d[2] = 0x2;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
// ===== JsrAbsThunk_097a40  ($097A40, 8 B, thunks/jsr_abs_thunks.c)
/// $097A40: jsr $5db1a.l
pub fn i_097a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97A46;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x97A46);
    cpu.pc = 0x5DB1A;
    16
}
/// $097A46: rts
pub fn i_097a46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97A48;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HiScore_DrawHeaderA_097a48  ($097A48, 6 B, frontend/hiscore_memcard_options_09773c.s)
/// $097A48: movea.l #$7063, a1
pub fn i_097a48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97A4E;
    cpu.ir = 0x227C;
    let v1: u32 = 0x7063;
    cpu.a[1] = v1;
    4
}
// ===== JsrPcThunk_097a4e  ($097A4E, 6 B, thunks/jsr_pc_thunks.c)
/// $097A4E: jsr $97a60(pc)
pub fn i_097a4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97A52;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x97A52);
    cpu.pc = 0x97A60;
    16
}
/// $097A52: rts
pub fn i_097a52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97A54;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HiScore_DrawHeaderB_097a54  ($097A54, 6 B, frontend/hiscore_memcard_options_09773c.s)
/// $097A54: movea.l #$7323, a1
pub fn i_097a54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97A5A;
    cpu.ir = 0x227C;
    let v1: u32 = 0x7323;
    cpu.a[1] = v1;
    4
}
// ===== JsrPcThunk_097a5a  ($097A5A, 6 B, thunks/jsr_pc_thunks.c)
/// $097A5A: jsr $97a72(pc)
pub fn i_097a5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97A5E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x97A5E);
    cpu.pc = 0x97A72;
    16
}
/// $097A5E: rts
pub fn i_097a5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97A60;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HiScore_TryEnter_P1_097a60  ($097A60, 18 B, frontend/hiscore_memcard_options_09773c.s)
/// $097A60: lea.l $106e94.l, a0
pub fn i_097a60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97A66;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x106E94;
    20
}
/// $097A66: jsr $51aa4.l
pub fn i_097a66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97A6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x97A6C);
    cpu.pc = 0x51AA4;
    16
}
/// $097A6C: moveq #$1, d1
pub fn i_097a6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97A6E;
    cpu.ir = 0x7201;
    cpu.d[1] = 0x1;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $097A6E: bra.w $97a80
pub fn i_097a6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97A72;
    cpu.ir = 0x6000;
    cpu.pc = 0x97A80;
    10
}
// ===== HiScore_TryEnter_P2_097a72  ($097A72, 56 B, frontend/hiscore_memcard_options_09773c.s)
/// $097A72: lea.l $106e9c.l, a0
pub fn i_097a72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97A78;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x106E9C;
    20
}
/// $097A78: jsr $51aa4.l
pub fn i_097a78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97A7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x97A7E);
    cpu.pc = 0x51AA4;
    16
}
/// $097A7E: moveq #$2, d1
pub fn i_097a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97A80;
    cpu.ir = 0x7202;
    cpu.d[1] = 0x2;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $097A80: jsr $97aaa(pc)
pub fn i_097a80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97A84;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x97A84);
    cpu.pc = 0x97AAA;
    16
}
/// $097A84: bcc.w $97aa4
pub fn i_097a84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97A88;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x97AA4; return 10; }
    8
}
/// $097A88: movem.l d0-d1/a1, -(a7)
pub fn i_097a88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97A8C;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $097A8C: lea.l $97d24(pc), a1
pub fn i_097a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97A90;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x97D24;
    16
}
/// $097A90: jsr $4ae.l
pub fn i_097a90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97A96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x97A96);
    cpu.pc = 0x4AE;
    16
}
/// $097A96: movem.l (a7)+, d0-d1/a1
pub fn i_097a96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97A9A;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $097A9A: move.l a1, $80(a0)
pub fn i_097a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97A9E;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $097A9E: move.w d1, $88(a0)
pub fn i_097a9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97AA2;
    cpu.ir = 0x3141;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x88);
    bus.write16(ad2, v1 as u16);
    12
}
/// $097AA2: rts
pub fn i_097aa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97AA4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $097AA4: clr.b $21(a6)
pub fn i_097aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97AA8;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $097AA8: rts
pub fn i_097aa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97AAA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HiScore_InsertScore_097aaa  ($097AAA, 186 B, frontend/hiscore_memcard_options_09773c.s)
/// $097AAA: movem.l d0-d1/a1, -(a7)
pub fn i_097aaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97AAE;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $097AAE: lea.l $100002.l, a2
pub fn i_097aae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97AB4;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x100002;
    20
}
/// $097AB4: moveq #$9, d2
pub fn i_097ab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97AB6;
    cpu.ir = 0x7409;
    cpu.d[2] = 0x9;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $097AB6: cmp.b (a2), d1
pub fn i_097ab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97AB8;
    cpu.ir = 0xB212;
    let v2 = bus.read8(cpu.a[2]) as u32;
    let h1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, h1, v2, Size::Byte, false);
    8
}
/// $097AB8: bne.w $97abe
pub fn i_097ab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97ABC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x97ABE; return 10; }
    8
}
/// $097ABC: clr.b (a6)
pub fn i_097abc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97ABE;
    cpu.ir = 0x4216;
    let ad1 = cpu.a[6];
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    8
}
/// $097ABE: adda.l #$c, a2
pub fn i_097abe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97AC4;
    cpu.ir = 0xD5FC;
    let v1: u32 = 0xC;
    cpu.a[2] = cpu.a[2].wrapping_add(v1);
    8
}
/// $097AC4: dbra d2, $97ab6
pub fn i_097ac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97AC8;
    cpu.ir = 0x51CA;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x97AC8; return 12; }
    let n = (cpu.d[2] as u16).wrapping_sub(1);
    cpu.d[2] = (cpu.d[2] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x97AC8; return 14; }
    cpu.pc = 0x97AB6;
    10
}
/// $097AC8: moveq #$0, d3
pub fn i_097ac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97ACA;
    cpu.ir = 0x7600;
    cpu.d[3] = 0x0;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $097ACA: lea.l $100002.l, a2
pub fn i_097aca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97AD0;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x100002;
    20
}
/// $097AD0: move.w #$6c, d2
pub fn i_097ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97AD4;
    cpu.ir = 0x343C;
    let v1: u32 = 0x6C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $097AD4: cmp.l $2(a2, d2.w), d0
pub fn i_097ad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97AD8;
    cpu.ir = 0xB0B2;
    let ad2 = cpu.a[2].wrapping_add(0x2).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    18
}
/// $097AD8: blt.w $97ae8
pub fn i_097ad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97ADC;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x97AE8; return 10; }
    8
}
/// $097ADC: addq.w #$1, d3
pub fn i_097adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97ADE;
    cpu.ir = 0x5243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $097ADE: subi.w #$c, d2
pub fn i_097ade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97AE2;
    cpu.ir = 0x442;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0xC, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $097AE2: bmi.w $97ae8
pub fn i_097ae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97AE6;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x97AE8; return 10; }
    8
}
/// $097AE6: bra.b $97ad4
pub fn i_097ae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97AE8;
    cpu.ir = 0x60EC;
    cpu.pc = 0x97AD4;
    10
}
/// $097AE8: tst.w d3
pub fn i_097ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97AEA;
    cpu.ir = 0x4A43;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $097AEA: beq.w $97b6a
pub fn i_097aea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97AEE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x97B6A; return 10; }
    8
}
/// $097AEE: subq.w #$1, d3
pub fn i_097aee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97AF0;
    cpu.ir = 0x5343;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $097AF0: beq.w $97b38
pub fn i_097af0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97AF4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x97B38; return 10; }
    8
}
/// $097AF4: move.w #$6c, d4
pub fn i_097af4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97AF8;
    cpu.ir = 0x383C;
    let v1: u32 = 0x6C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $097AF8: move.w #$60, d5
pub fn i_097af8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97AFC;
    cpu.ir = 0x3A3C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $097AFC: move.b (a2, d5.w), (a2, d4.w)
pub fn i_097afc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B02;
    cpu.ir = 0x15B2;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    bus.write8(ad4, v3 as u8);
    24
}
/// $097B02: move.b $1(a2, d5.w), d6
pub fn i_097b02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B06;
    cpu.ir = 0x1C32;
    let ad2 = cpu.a[2].wrapping_add(0x1).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    14
}
/// $097B06: cmp.l $2(a2, d5.w), d0
pub fn i_097b06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B0A;
    cpu.ir = 0xB0B2;
    let ad2 = cpu.a[2].wrapping_add(0x2).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    18
}
/// $097B0A: beq.w $97b10
pub fn i_097b0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B0E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x97B10; return 10; }
    8
}
/// $097B0E: addq.b #$1, d6
pub fn i_097b0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B10;
    cpu.ir = 0x5206;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    4
}
/// $097B10: move.b d6, $1(a2, d4.w)
pub fn i_097b10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B14;
    cpu.ir = 0x1586;
    let v1 = cpu.d[6] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[2].wrapping_add(0x1).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $097B14: move.l $2(a2, d5.w), $2(a2, d4.w)
pub fn i_097b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B1A;
    cpu.ir = 0x25B2;
    let ad2 = cpu.a[2].wrapping_add(0x2).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[2].wrapping_add(0x2).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    32
}
/// $097B1A: move.w $6(a2, d5.w), $6(a2, d4.w)
pub fn i_097b1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B20;
    cpu.ir = 0x35B2;
    let ad2 = cpu.a[2].wrapping_add(0x6).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2].wrapping_add(0x6).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    bus.write16(ad4, v3 as u16);
    24
}
/// $097B20: move.w $8(a2, d5.w), $8(a2, d4.w)
pub fn i_097b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B26;
    cpu.ir = 0x35B2;
    let ad2 = cpu.a[2].wrapping_add(0x8).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2].wrapping_add(0x8).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    bus.write16(ad4, v3 as u16);
    24
}
/// $097B26: move.w $a(a2, d5.w), $a(a2, d4.w)
pub fn i_097b26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B2C;
    cpu.ir = 0x35B2;
    let ad2 = cpu.a[2].wrapping_add(0xA).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2].wrapping_add(0xA).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    bus.write16(ad4, v3 as u16);
    24
}
/// $097B2C: subi.w #$c, d4
pub fn i_097b2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B30;
    cpu.ir = 0x444;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0xC, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $097B30: subi.w #$c, d5
pub fn i_097b30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B34;
    cpu.ir = 0x445;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0xC, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $097B34: cmp.w d5, d2
pub fn i_097b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B36;
    cpu.ir = 0xB445;
    let v1 = cpu.d[5] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $097B36: bne.b $97afc
pub fn i_097b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B38;
    cpu.ir = 0x66C4;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x97AFC; return 10; }
    8
}
/// $097B38: addi.w #$c, d2
pub fn i_097b38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B3C;
    cpu.ir = 0x642;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xC, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $097B3C: move.b d1, (a2, d2.w)
pub fn i_097b3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B40;
    cpu.ir = 0x1581;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $097B40: move.l d0, $2(a2, d2.w)
pub fn i_097b40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B44;
    cpu.ir = 0x2580;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[2].wrapping_add(0x2).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $097B44: move.w d1, d0
pub fn i_097b44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B46;
    cpu.ir = 0x3001;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $097B46: jsr $98176(pc)
pub fn i_097b46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B4A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x97B4A);
    cpu.pc = 0x98176;
    16
}
/// $097B4A: move.b d1, $1(a2, d2.w)
pub fn i_097b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B4E;
    cpu.ir = 0x1581;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[2].wrapping_add(0x1).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $097B4E: move.w #$b40, $6(a2, d2.w)
pub fn i_097b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B54;
    cpu.ir = 0x35BC;
    let v1: u32 = 0xB40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[2].wrapping_add(0x6).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $097B54: move.w #$b40, $8(a2, d2.w)
pub fn i_097b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B5A;
    cpu.ir = 0x35BC;
    let v1: u32 = 0xB40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[2].wrapping_add(0x8).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $097B5A: move.w #$b40, $a(a2, d2.w)
pub fn i_097b5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B60;
    cpu.ir = 0x35BC;
    let v1: u32 = 0xB40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[2].wrapping_add(0xA).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $097B60: movem.l (a7)+, d0-d1/a1
pub fn i_097b60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B64;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
// ===== SetXN_097b64  ($097B64, 6 B, thunks/ccr_helpers.c)
/// $097B64: ori.b #$11, ccr
pub fn i_097b64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B68;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $097B68: rts
pub fn i_097b68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B6A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HiScore_InsertScore_NoRank_097b6a  ($097B6A, 4 B, frontend/hiscore_memcard_options_09773c.s)
/// $097B6A: movem.l (a7)+, d0-d1/a1
pub fn i_097b6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B6E;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
// ===== ClearXN_097b6e  ($097B6E, 6 B, thunks/ccr_helpers.c)
/// $097B6E: andi.b #$ee, ccr
pub fn i_097b6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B72;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $097B72: rts
pub fn i_097b72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B74;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HiScore_DrawRow_097b74  ($097B74, 134 B, frontend/hiscore_memcard_options_09773c.s)
/// $097B74: moveq #$0, d0
pub fn i_097b74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B76;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $097B76: move.b (a0)+, d0
pub fn i_097b76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B78;
    cpu.ir = 0x1018;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    8
}
/// $097B78: tst.b d0
pub fn i_097b78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B7A;
    cpu.ir = 0x4A00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $097B7A: beq.w $97b9c
pub fn i_097b7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B7E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x97B9C; return 10; }
    8
}
/// $097B7E: movem.l a0-a1, -(a7)
pub fn i_097b7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B82;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $097B82: move.l d0, -(a7)
pub fn i_097b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B84;
    cpu.ir = 0x2F00;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $097B84: lea.l $97cc4(pc), a1
pub fn i_097b84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B88;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x97CC4;
    16
}
/// $097B88: jsr $4ae.l
pub fn i_097b88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B8E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x97B8E);
    cpu.pc = 0x4AE;
    16
}
/// $097B8E: move.l (a7)+, d0
pub fn i_097b8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B90;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $097B90: move.w d0, $88(a0)
pub fn i_097b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B94;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x88);
    bus.write16(ad2, v1 as u16);
    12
}
/// $097B94: movem.l (a7)+, a0-a1
pub fn i_097b94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B98;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $097B98: addq.b #$1, $20(a6)
pub fn i_097b98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B9C;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $097B9C: move.b (a0)+, d0
pub fn i_097b9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97B9E;
    cpu.ir = 0x1018;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    8
}
/// $097B9E: jsr $97c02(pc)
pub fn i_097b9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97BA2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x97BA2);
    cpu.pc = 0x97C02;
    16
}
/// $097BA2: adda.l #$c0, a1
pub fn i_097ba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97BA8;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0xC0;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $097BA8: move.l a1, -(a7)
pub fn i_097ba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97BAA;
    cpu.ir = 0x2F09;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $097BAA: move.l (a0)+, d0
pub fn i_097baa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97BAC;
    cpu.ir = 0x2018;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $097BAC: moveq #$4, d1
pub fn i_097bac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97BAE;
    cpu.ir = 0x7204;
    cpu.d[1] = 0x4;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $097BAE: jsr $5d7be.l
pub fn i_097bae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97BB4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x97BB4);
    cpu.pc = 0x5D7BE;
    16
}
/// $097BB4: movea.l (a7)+, a1
pub fn i_097bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97BB6;
    cpu.ir = 0x225F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    12
}
/// $097BB6: adda.l #$240, a1
pub fn i_097bb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97BBC;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x240;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $097BBC: move.w (a0)+, d0
pub fn i_097bbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97BBE;
    cpu.ir = 0x3018;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $097BBE: ori.w #$4000, d0
pub fn i_097bbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97BC2;
    cpu.ir = 0x40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 | 0x4000) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $097BC2: moveq #$2, d1
pub fn i_097bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97BC4;
    cpu.ir = 0x7202;
    cpu.d[1] = 0x2;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $097BC4: moveq #$2, d2
pub fn i_097bc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97BC6;
    cpu.ir = 0x7402;
    cpu.d[2] = 0x2;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $097BC6: move.l a1, -(a7)
pub fn i_097bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97BC8;
    cpu.ir = 0x2F09;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $097BC8: jsr $5da56.l
pub fn i_097bc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97BCE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x97BCE);
    cpu.pc = 0x5DA56;
    16
}
/// $097BCE: movea.l (a7)+, a1
pub fn i_097bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97BD0;
    cpu.ir = 0x225F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    12
}
/// $097BD0: adda.l #$40, a1
pub fn i_097bd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97BD6;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x40;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $097BD6: move.w (a0)+, d0
pub fn i_097bd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97BD8;
    cpu.ir = 0x3018;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $097BD8: ori.w #$4000, d0
pub fn i_097bd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97BDC;
    cpu.ir = 0x40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 | 0x4000) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $097BDC: moveq #$2, d1
pub fn i_097bdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97BDE;
    cpu.ir = 0x7202;
    cpu.d[1] = 0x2;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $097BDE: moveq #$2, d2
pub fn i_097bde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97BE0;
    cpu.ir = 0x7402;
    cpu.d[2] = 0x2;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $097BE0: move.l a1, -(a7)
pub fn i_097be0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97BE2;
    cpu.ir = 0x2F09;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $097BE2: jsr $5da56.l
pub fn i_097be2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97BE8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x97BE8);
    cpu.pc = 0x5DA56;
    16
}
/// $097BE8: movea.l (a7)+, a1
pub fn i_097be8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97BEA;
    cpu.ir = 0x225F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    12
}
/// $097BEA: adda.l #$40, a1
pub fn i_097bea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97BF0;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x40;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $097BF0: move.w (a0)+, d0
pub fn i_097bf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97BF2;
    cpu.ir = 0x3018;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $097BF2: ori.w #$4000, d0
pub fn i_097bf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97BF6;
    cpu.ir = 0x40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 | 0x4000) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $097BF6: moveq #$2, d1
pub fn i_097bf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97BF8;
    cpu.ir = 0x7202;
    cpu.d[1] = 0x2;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $097BF8: moveq #$2, d2
pub fn i_097bf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97BFA;
    cpu.ir = 0x7402;
    cpu.d[2] = 0x2;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
// ===== JsrAbsThunk_097bfa  ($097BFA, 8 B, thunks/jsr_abs_thunks.c)
/// $097BFA: jsr $5da56.l
pub fn i_097bfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97C00;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x97C00);
    cpu.pc = 0x5DA56;
    16
}
/// $097C00: rts
pub fn i_097c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97C02;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HiScore_DrawRank_097c02  ($097C02, 90 B, frontend/hiscore_memcard_options_09773c.s)
/// $097C02: move.l a1, -(a7)
pub fn i_097c02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97C04;
    cpu.ir = 0x2F09;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $097C04: andi.w #$ff, d0
pub fn i_097c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97C08;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $097C08: subq.w #$1, d0
pub fn i_097c08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97C0A;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $097C0A: bmi.w $97c16
pub fn i_097c0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97C0E;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x97C16; return 10; }
    8
}
/// $097C0E: cmpi.w #$a, d0
pub fn i_097c0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97C12;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA, Size::Word, false);
    8
}
/// $097C12: ble.w $97c18
pub fn i_097c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97C16;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x97C18; return 10; }
    8
}
/// $097C16: moveq #$0, d0
pub fn i_097c16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97C18;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $097C18: move.l d0, -(a7)
pub fn i_097c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97C1A;
    cpu.ir = 0x2F00;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $097C1A: lea.l $2f542a.l, a2
pub fn i_097c1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97C20;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2F542A;
    20
}
/// $097C20: add.w d0, d0
pub fn i_097c20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97C22;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $097C22: move.w (a2, d0.w), d0
pub fn i_097c22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97C26;
    cpu.ir = 0x3032;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $097C26: ori.w #$4000, d0
pub fn i_097c26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97C2A;
    cpu.ir = 0x40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 | 0x4000) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $097C2A: moveq #$2, d1
pub fn i_097c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97C2C;
    cpu.ir = 0x7202;
    cpu.d[1] = 0x2;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $097C2C: moveq #$2, d2
pub fn i_097c2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97C2E;
    cpu.ir = 0x7402;
    cpu.d[2] = 0x2;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $097C2E: move.l a1, -(a7)
pub fn i_097c2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97C30;
    cpu.ir = 0x2F09;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $097C30: jsr $5da56.l
pub fn i_097c30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97C36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x97C36);
    cpu.pc = 0x5DA56;
    16
}
/// $097C36: movea.l (a7)+, a1
pub fn i_097c36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97C38;
    cpu.ir = 0x225F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    12
}
/// $097C38: move.l (a7)+, d0
pub fn i_097c38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97C3A;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $097C3A: adda.l #$41, a1
pub fn i_097c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97C40;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x41;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $097C40: lea.l $2f5440.l, a2
pub fn i_097c40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97C46;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2F5440;
    20
}
/// $097C46: add.w d0, d0
pub fn i_097c46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97C48;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $097C48: add.w d0, d0
pub fn i_097c48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97C4A;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $097C4A: movea.l (a2, d0.w), a2
pub fn i_097c4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97C4E;
    cpu.ir = 0x2472;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    18
}
/// $097C4E: move.w #$4300, d0
pub fn i_097c4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97C52;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $097C52: jsr $5dad8.l
pub fn i_097c52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97C58;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x97C58);
    cpu.pc = 0x5DAD8;
    16
}
/// $097C58: movea.l (a7)+, a1
pub fn i_097c58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97C5A;
    cpu.ir = 0x225F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    12
}
/// $097C5A: rts
pub fn i_097c5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97C5C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HiScore_DrawNameChars_097c5c  ($097C5C, 96 B, frontend/hiscore_memcard_options_09773c.s)
/// $097C5C: tst.w d0
pub fn i_097c5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97C5E;
    cpu.ir = 0x4A40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $097C5E: beq.w $97cc2
pub fn i_097c5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97C62;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x97CC2; return 10; }
    8
}
/// $097C62: move.l d0, -(a7)
pub fn i_097c62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97C64;
    cpu.ir = 0x2F00;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $097C64: move.w (a0), d0
pub fn i_097c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97C66;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $097C66: ori.w #$4000, d0
pub fn i_097c66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97C6A;
    cpu.ir = 0x40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 | 0x4000) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $097C6A: moveq #$2, d1
pub fn i_097c6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97C6C;
    cpu.ir = 0x7202;
    cpu.d[1] = 0x2;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $097C6C: moveq #$2, d2
pub fn i_097c6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97C6E;
    cpu.ir = 0x7402;
    cpu.d[2] = 0x2;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $097C6E: move.l a1, -(a7)
pub fn i_097c6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97C70;
    cpu.ir = 0x2F09;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $097C70: jsr $5da56.l
pub fn i_097c70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97C76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x97C76);
    cpu.pc = 0x5DA56;
    16
}
/// $097C76: movea.l (a7)+, a1
pub fn i_097c76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97C78;
    cpu.ir = 0x225F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    12
}
/// $097C78: move.l (a7)+, d0
pub fn i_097c78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97C7A;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $097C7A: cmpi.w #$1, d0
pub fn i_097c7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97C7E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x1, Size::Word, false);
    8
}
/// $097C7E: beq.w $97cc2
pub fn i_097c7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97C82;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x97CC2; return 10; }
    8
}
/// $097C82: move.l d0, -(a7)
pub fn i_097c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97C84;
    cpu.ir = 0x2F00;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $097C84: adda.l #$40, a1
pub fn i_097c84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97C8A;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x40;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $097C8A: move.w $2(a0), d0
pub fn i_097c8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97C8E;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $097C8E: ori.w #$4000, d0
pub fn i_097c8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97C92;
    cpu.ir = 0x40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 | 0x4000) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $097C92: moveq #$2, d1
pub fn i_097c92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97C94;
    cpu.ir = 0x7202;
    cpu.d[1] = 0x2;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $097C94: moveq #$2, d2
pub fn i_097c94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97C96;
    cpu.ir = 0x7402;
    cpu.d[2] = 0x2;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $097C96: move.l a1, -(a7)
pub fn i_097c96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97C98;
    cpu.ir = 0x2F09;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $097C98: jsr $5da56.l
pub fn i_097c98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97C9E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x97C9E);
    cpu.pc = 0x5DA56;
    16
}
/// $097C9E: movea.l (a7)+, a1
pub fn i_097c9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97CA0;
    cpu.ir = 0x225F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    12
}
/// $097CA0: move.l (a7)+, d0
pub fn i_097ca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97CA2;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $097CA2: cmpi.w #$2, d0
pub fn i_097ca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97CA6;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x2, Size::Word, false);
    8
}
/// $097CA6: beq.w $97cc2
pub fn i_097ca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97CAA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x97CC2; return 10; }
    8
}
/// $097CAA: adda.l #$40, a1
pub fn i_097caa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97CB0;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x40;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $097CB0: move.w $4(a0), d0
pub fn i_097cb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97CB4;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $097CB4: ori.w #$4000, d0
pub fn i_097cb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97CB8;
    cpu.ir = 0x40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 | 0x4000) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $097CB8: moveq #$2, d1
pub fn i_097cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97CBA;
    cpu.ir = 0x7202;
    cpu.d[1] = 0x2;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $097CBA: moveq #$2, d2
pub fn i_097cba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97CBC;
    cpu.ir = 0x7402;
    cpu.d[2] = 0x2;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
// ===== JsrAbsThunk_097cbc  ($097CBC, 8 B, thunks/jsr_abs_thunks.c)
/// $097CBC: jsr $5da56.l
pub fn i_097cbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97CC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x97CC2);
    cpu.pc = 0x5DA56;
    16
}
/// $097CC2: rts
pub fn i_097cc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97CC4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== NameEntry_InitRow_097cc4  ($097CC4, 180 B, frontend/hiscore_memcard_options_09773c.s)
/// $097CC4: move.b #$ff, $21(a6)
pub fn i_097cc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97CCA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $097CCA: moveq #$0, d0
pub fn i_097cca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97CCC;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $097CCC: move.w d0, $7a(a6)
pub fn i_097ccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97CD0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $097CD0: move.w d0, $7c(a6)
pub fn i_097cd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97CD4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $097CD4: move.w d0, $7e(a6)
pub fn i_097cd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97CD8;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $097CD8: move.w d0, $72(a6)
pub fn i_097cd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97CDC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $097CDC: move.w d0, $70(a6)
pub fn i_097cdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97CE0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $097CE0: move.b d0, $8c(a6)
pub fn i_097ce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97CE4;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $097CE4: move.b d0, $8d(a6)
pub fn i_097ce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97CE8;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $097CE8: move.w #$b40, d1
pub fn i_097ce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97CEC;
    cpu.ir = 0x323C;
    let v1: u32 = 0xB40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $097CEC: move.w d1, $74(a6)
pub fn i_097cec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97CF0;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $097CF0: move.w d1, $76(a6)
pub fn i_097cf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97CF4;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $097CF4: move.w d1, $78(a6)
pub fn i_097cf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97CF8;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad2, v1 as u16);
    12
}
/// $097CF8: move.w $88(a6), d0
pub fn i_097cf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97CFC;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $097CFC: jsr $981ae(pc)
pub fn i_097cfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97D00;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x97D00);
    cpu.pc = 0x981AE;
    16
}
/// $097D00: movea.l #$70a6, a1
pub fn i_097d00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97D06;
    cpu.ir = 0x227C;
    let v1: u32 = 0x70A6;
    cpu.a[1] = v1;
    4
}
/// $097D06: subq.w #$1, d1
pub fn i_097d06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97D08;
    cpu.ir = 0x5341;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $097D08: mulu.w #$2, d1
pub fn i_097d08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97D0C;
    cpu.ir = 0xC2FC;
    let v1: u32 = 0x2;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[1] & 0xFFFF);
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $097D0C: adda.l d1, a1
pub fn i_097d0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97D0E;
    cpu.ir = 0xD3C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $097D0E: move.l a1, $80(a6)
pub fn i_097d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97D12;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $097D12: move.l #$300, $84(a6)
pub fn i_097d12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97D1A;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x300;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $097D1A: lea.l $97d78(pc), a1
pub fn i_097d1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97D1E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x97D78;
    16
}
/// $097D1E: move.l a1, (a6)
pub fn i_097d1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97D20;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $097D20: bra.w $97d78
pub fn i_097d20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97D24;
    cpu.ir = 0x6000;
    cpu.pc = 0x97D78;
    10
}
// ===== NameEntry_Run_097d78  ($097D78, 722 B, frontend/hiscore_memcard_options_09773c.s)
/// $097D78: move.w $88(a6), d0
pub fn i_097d78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97D7C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $097D7C: jsr $98176(pc)
pub fn i_097d7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97D80;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x97D80);
    cpu.pc = 0x98176;
    16
}
/// $097D80: movea.l $80(a6), a1
pub fn i_097d80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97D84;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $097D84: move.w d1, d0
pub fn i_097d84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97D86;
    cpu.ir = 0x3001;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $097D86: jsr $97c02(pc)
pub fn i_097d86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97D8A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x97D8A);
    cpu.pc = 0x97C02;
    16
}
/// $097D8A: tst.b $21(a6)
pub fn i_097d8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97D8E;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $097D8E: beq.w $97dec
pub fn i_097d8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97D92;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x97DEC; return 10; }
    8
}
/// $097D92: move.w $7a(a6), d0
pub fn i_097d92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97D96;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $097D96: lsr.w #$4, d0
pub fn i_097d96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97D98;
    cpu.ir = 0xE848;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $097D98: andi.w #$3, d0
pub fn i_097d98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97D9C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $097D9C: cmpi.w #$3, d0
pub fn i_097d9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97DA0;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x3, Size::Word, false);
    8
}
/// $097DA0: beq.w $97dd2
pub fn i_097da0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97DA4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x97DD2; return 10; }
    8
}
/// $097DA4: move.w $88(a6), d0
pub fn i_097da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97DA8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $097DA8: jsr $981ae(pc)
pub fn i_097da8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97DAC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x97DAC);
    cpu.pc = 0x981AE;
    16
}
/// $097DAC: lea.l $100002.l, a1
pub fn i_097dac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97DB2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x100002;
    20
}
/// $097DB2: subq.w #$1, d1
pub fn i_097db2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97DB4;
    cpu.ir = 0x5341;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $097DB4: mulu.w #$c, d1
pub fn i_097db4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97DB8;
    cpu.ir = 0xC2FC;
    let v1: u32 = 0xC;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[1] & 0xFFFF);
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $097DB8: move.l $2(a1, d1.w), d0
pub fn i_097db8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97DBC;
    cpu.ir = 0x2031;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $097DBC: movea.l $80(a6), a1
pub fn i_097dbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97DC0;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $097DC0: adda.l #$c0, a1
pub fn i_097dc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97DC6;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0xC0;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $097DC6: moveq #$4, d1
pub fn i_097dc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97DC8;
    cpu.ir = 0x7204;
    cpu.d[1] = 0x4;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $097DC8: jsr $5d7be.l
pub fn i_097dc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97DCE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x97DCE);
    cpu.pc = 0x5D7BE;
    16
}
/// $097DCE: bra.w $97dec
pub fn i_097dce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97DD2;
    cpu.ir = 0x6000;
    cpu.pc = 0x97DEC;
    10
}
/// $097DD2: movea.l $80(a6), a1
pub fn i_097dd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97DD6;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $097DD6: adda.l #$c0, a1
pub fn i_097dd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97DDC;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0xC0;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $097DDC: move.w #$b40, d0
pub fn i_097ddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97DE0;
    cpu.ir = 0x303C;
    let v1: u32 = 0xB40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $097DE0: move.w #$10, d1
pub fn i_097de0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97DE4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $097DE4: moveq #$2, d2
pub fn i_097de4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97DE6;
    cpu.ir = 0x7402;
    cpu.d[2] = 0x2;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $097DE6: jsr $5da9c.l
pub fn i_097de6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97DEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x97DEC);
    cpu.pc = 0x5DA9C;
    16
}
/// $097DEC: movea.l $80(a6), a1
pub fn i_097dec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97DF0;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $097DF0: adda.l $84(a6), a1
pub fn i_097df0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97DF4;
    cpu.ir = 0xD3EE;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read32(ad2);
    cpu.a[1] = cpu.a[1].wrapping_add(v3);
    20
}
/// $097DF4: movea.l a6, a0
pub fn i_097df4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97DF6;
    cpu.ir = 0x204E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $097DF6: adda.l #$74, a0
pub fn i_097df6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97DFC;
    cpu.ir = 0xD1FC;
    let v1: u32 = 0x74;
    cpu.a[0] = cpu.a[0].wrapping_add(v1);
    8
}
/// $097DFC: move.w $70(a6), d0
pub fn i_097dfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97E00;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $097E00: jsr $97c5c(pc)
pub fn i_097e00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97E04;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x97E04);
    cpu.pc = 0x97C5C;
    16
}
/// $097E04: movea.l $80(a6), a1
pub fn i_097e04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97E08;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $097E08: adda.l $84(a6), a1
pub fn i_097e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97E0C;
    cpu.ir = 0xD3EE;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read32(ad2);
    cpu.a[1] = cpu.a[1].wrapping_add(v3);
    20
}
/// $097E0C: move.w $70(a6), d1
pub fn i_097e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97E10;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $097E10: mulu.w #$40, d1
pub fn i_097e10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97E14;
    cpu.ir = 0xC2FC;
    let v1: u32 = 0x40;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[1] & 0xFFFF);
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $097E14: adda.l d1, a1
pub fn i_097e14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97E16;
    cpu.ir = 0xD3C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $097E16: addq.w #$1, $7e(a6)
pub fn i_097e16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97E1A;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x7E);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $097E1A: move.w $7e(a6), d0
pub fn i_097e1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97E1E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $097E1E: btst.b #$4, d0
pub fn i_097e1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97E22;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((4) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $097E22: beq.w $97e3c
pub fn i_097e22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97E26;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x97E3C; return 10; }
    8
}
/// $097E26: move.w #$b40, d0
pub fn i_097e26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97E2A;
    cpu.ir = 0x303C;
    let v1: u32 = 0xB40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $097E2A: ori.w #$4000, d0
pub fn i_097e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97E2E;
    cpu.ir = 0x40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 | 0x4000) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $097E2E: moveq #$2, d1
pub fn i_097e2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97E30;
    cpu.ir = 0x7202;
    cpu.d[1] = 0x2;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $097E30: moveq #$2, d2
pub fn i_097e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97E32;
    cpu.ir = 0x7402;
    cpu.d[2] = 0x2;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $097E32: jsr $5da56.l
pub fn i_097e32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97E38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x97E38);
    cpu.pc = 0x5DA56;
    16
}
/// $097E38: bra.w $97e5a
pub fn i_097e38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97E3C;
    cpu.ir = 0x6000;
    cpu.pc = 0x97E5A;
    10
}
/// $097E3C: lea.l $2f54b2.l, a0
pub fn i_097e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97E42;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F54B2;
    20
}
/// $097E42: move.w $72(a6), d0
pub fn i_097e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97E46;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $097E46: add.w d0, d0
pub fn i_097e46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97E48;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $097E48: move.w (a0, d0.w), d0
pub fn i_097e48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97E4C;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $097E4C: ori.w #$4000, d0
pub fn i_097e4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97E50;
    cpu.ir = 0x40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 | 0x4000) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $097E50: moveq #$2, d1
pub fn i_097e50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97E52;
    cpu.ir = 0x7202;
    cpu.d[1] = 0x2;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $097E52: moveq #$2, d2
pub fn i_097e52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97E54;
    cpu.ir = 0x7402;
    cpu.d[2] = 0x2;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $097E54: jsr $5da56.l
pub fn i_097e54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97E5A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x97E5A);
    cpu.pc = 0x5DA56;
    16
}
/// $097E5A: move.w $88(a6), d1
pub fn i_097e5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97E5E;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $097E5E: subq.w #$1, d1
pub fn i_097e5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97E60;
    cpu.ir = 0x5341;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $097E60: add.w d1, d1
pub fn i_097e60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97E62;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $097E62: add.w d1, d1
pub fn i_097e62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97E64;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $097E64: lea.l $2f552c.l, a0
pub fn i_097e64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97E6A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F552C;
    20
}
/// $097E6A: movea.l (a0, d1.w), a0
pub fn i_097e6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97E6E;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $097E6E: move.b (a0), d0
pub fn i_097e6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97E70;
    cpu.ir = 0x1010;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $097E70: andi.b #$3c, d0
pub fn i_097e70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97E74;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3C) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $097E74: bne.w $97e9e
pub fn i_097e74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97E78;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x97E9E; return 10; }
    8
}
/// $097E78: lea.l $2f5534.l, a0
pub fn i_097e78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97E7E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F5534;
    20
}
/// $097E7E: movea.l (a0, d1.w), a0
pub fn i_097e7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97E82;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $097E82: move.b (a0), d0
pub fn i_097e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97E84;
    cpu.ir = 0x1010;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $097E84: andi.b #$c, d0
pub fn i_097e84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97E88;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xC) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $097E88: bne.w $97e9e
pub fn i_097e88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97E8C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x97E9E; return 10; }
    8
}
/// $097E8C: addq.w #$1, $7c(a6)
pub fn i_097e8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97E90;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x7C);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $097E90: cmpi.w #$12c, $7c(a6)
pub fn i_097e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97E96;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x12C, Size::Word, false);
    16
}
/// $097E96: bcc.w $97f96
pub fn i_097e96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97E9A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x97F96; return 10; }
    8
}
/// $097E9A: bra.w $97f72
pub fn i_097e9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97E9E;
    cpu.ir = 0x6000;
    cpu.pc = 0x97F72;
    10
}
/// $097E9E: move.w #$0, $7c(a6)
pub fn i_097e9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97EA4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $097EA4: move.w #$0, $7e(a6)
pub fn i_097ea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97EAA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $097EAA: move.b #$0, $8d(a6)
pub fn i_097eaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97EB0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $097EB0: move.b d0, $8c(a6)
pub fn i_097eb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97EB4;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $097EB4: andi.b #$c, $8c(a6)
pub fn i_097eb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97EBA;
    cpu.ir = 0x22E;
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0xC) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $097EBA: btst.b #$2, d0
pub fn i_097eba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97EBE;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((2) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $097EBE: beq.w $97ed0
pub fn i_097ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97EC2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x97ED0; return 10; }
    8
}
/// $097EC2: subq.w #$1, $72(a6)
pub fn i_097ec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97EC6;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $097EC6: bpl.w $97ed0
pub fn i_097ec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97ECA;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x97ED0; return 10; }
    8
}
/// $097ECA: move.w #$28, $72(a6)
pub fn i_097eca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97ED0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $097ED0: btst.b #$3, d0
pub fn i_097ed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97ED4;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((3) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $097ED4: beq.w $97eea
pub fn i_097ed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97ED8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x97EEA; return 10; }
    8
}
/// $097ED8: addq.w #$1, $72(a6)
pub fn i_097ed8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97EDC;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $097EDC: cmpi.w #$29, $72(a6)
pub fn i_097edc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97EE2;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x29, Size::Word, false);
    16
}
/// $097EE2: blt.w $97eea
pub fn i_097ee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97EE6;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x97EEA; return 10; }
    8
}
/// $097EE6: clr.w $72(a6)
pub fn i_097ee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97EEA;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $097EEA: btst.b #$4, d0
pub fn i_097eea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97EEE;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((4) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $097EEE: beq.w $97f2c
pub fn i_097eee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97EF2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x97F2C; return 10; }
    8
}
/// $097EF2: lea.l $2f54b2.l, a1
pub fn i_097ef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97EF8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2F54B2;
    20
}
/// $097EF8: move.w $72(a6), d1
pub fn i_097ef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97EFC;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $097EFC: move.w $70(a6), d2
pub fn i_097efc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97F00;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $097F00: add.w d1, d1
pub fn i_097f00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97F02;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $097F02: add.w d2, d2
pub fn i_097f02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97F04;
    cpu.ir = 0xD442;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $097F04: move.w (a1, d1.w), d3
pub fn i_097f04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97F08;
    cpu.ir = 0x3631;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $097F08: cmpi.w #$ce6, d3
pub fn i_097f08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97F0C;
    cpu.ir = 0xC43;
    let v1 = cpu.d[3] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xCE6, Size::Word, false);
    8
}
/// $097F0C: bne.w $97f1a
pub fn i_097f0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97F10;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x97F1A; return 10; }
    8
}
/// $097F10: move.w #$b40, $74(a6, d2.w)
pub fn i_097f10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97F16;
    cpu.ir = 0x3DBC;
    let v1: u32 = 0xB40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $097F16: bra.w $97fc0
pub fn i_097f16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97F1A;
    cpu.ir = 0x6000;
    cpu.pc = 0x97FC0;
    10
}
/// $097F1A: move.w d3, $74(a6, d2.w)
pub fn i_097f1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97F1E;
    cpu.ir = 0x3D83;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $097F1E: addq.w #$1, $70(a6)
pub fn i_097f1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97F22;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $097F22: cmpi.w #$3, $70(a6)
pub fn i_097f22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97F28;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Word, false);
    16
}
/// $097F28: beq.w $97fc0
pub fn i_097f28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97F2C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x97FC0; return 10; }
    8
}
/// $097F2C: btst.b #$5, d0
pub fn i_097f2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97F30;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((5) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $097F30: beq.w $97f72
pub fn i_097f30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97F34;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x97F72; return 10; }
    8
}
/// $097F34: movea.l $80(a6), a1
pub fn i_097f34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97F38;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $097F38: adda.l $84(a6), a1
pub fn i_097f38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97F3C;
    cpu.ir = 0xD3EE;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read32(ad2);
    cpu.a[1] = cpu.a[1].wrapping_add(v3);
    20
}
/// $097F3C: move.w $70(a6), d1
pub fn i_097f3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97F40;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $097F40: mulu.w #$40, d1
pub fn i_097f40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97F44;
    cpu.ir = 0xC2FC;
    let v1: u32 = 0x40;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[1] & 0xFFFF);
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $097F44: adda.l d1, a1
pub fn i_097f44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97F46;
    cpu.ir = 0xD3C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $097F46: move.w #$b40, d0
pub fn i_097f46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97F4A;
    cpu.ir = 0x303C;
    let v1: u32 = 0xB40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $097F4A: ori.w #$4000, d0
pub fn i_097f4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97F4E;
    cpu.ir = 0x40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 | 0x4000) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $097F4E: moveq #$2, d1
pub fn i_097f4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97F50;
    cpu.ir = 0x7202;
    cpu.d[1] = 0x2;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $097F50: moveq #$2, d2
pub fn i_097f50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97F52;
    cpu.ir = 0x7402;
    cpu.d[2] = 0x2;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $097F52: jsr $5da56.l
pub fn i_097f52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97F58;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x97F58);
    cpu.pc = 0x5DA56;
    16
}
/// $097F58: subq.w #$1, $70(a6)
pub fn i_097f58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97F5C;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $097F5C: bpl.w $97f66
pub fn i_097f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97F60;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x97F66; return 10; }
    8
}
/// $097F60: move.w #$0, $70(a6)
pub fn i_097f60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97F66;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $097F66: move.w $70(a6), d2
pub fn i_097f66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97F6A;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $097F6A: add.w d2, d2
pub fn i_097f6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97F6C;
    cpu.ir = 0xD442;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $097F6C: move.w #$b40, $74(a6, d2.w)
pub fn i_097f6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97F72;
    cpu.ir = 0x3DBC;
    let v1: u32 = 0xB40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $097F72: addq.w #$1, $7a(a6)
pub fn i_097f72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97F76;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x7A);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $097F76: cmpi.w #$384, $7a(a6)
pub fn i_097f76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97F7C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x384, Size::Word, false);
    16
}
/// $097F7C: bcc.w $97f96
pub fn i_097f7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97F80;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x97F96; return 10; }
    8
}
/// $097F80: tst.b $21(a6)
pub fn i_097f80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97F84;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $097F84: bne.w $97f94
pub fn i_097f84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97F88;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x97F94; return 10; }
    8
}
/// $097F88: movea.l $c(a6), a0
pub fn i_097f88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97F8C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $097F8C: tst.b $20(a0)
pub fn i_097f8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97F90;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $097F90: beq.w $97f96
pub fn i_097f90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97F94;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x97F96; return 10; }
    8
}
/// $097F94: rts
pub fn i_097f94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97F96;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $097F96: lea.l $2f54b2.l, a1
pub fn i_097f96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97F9C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2F54B2;
    20
}
/// $097F9C: move.w $72(a6), d1
pub fn i_097f9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97FA0;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $097FA0: move.w $70(a6), d2
pub fn i_097fa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97FA4;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $097FA4: add.w d1, d1
pub fn i_097fa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97FA6;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $097FA6: add.w d2, d2
pub fn i_097fa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97FA8;
    cpu.ir = 0xD442;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $097FA8: move.w (a1, d1.w), d3
pub fn i_097fa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97FAC;
    cpu.ir = 0x3631;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $097FAC: cmpi.w #$ce6, d3
pub fn i_097fac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97FB0;
    cpu.ir = 0xC43;
    let v1 = cpu.d[3] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xCE6, Size::Word, false);
    8
}
/// $097FB0: bne.w $97fb8
pub fn i_097fb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97FB4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x97FB8; return 10; }
    8
}
/// $097FB4: move.w #$b40, d3
pub fn i_097fb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97FB8;
    cpu.ir = 0x363C;
    let v1: u32 = 0xB40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $097FB8: move.w d3, $74(a6, d2.w)
pub fn i_097fb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97FBC;
    cpu.ir = 0x3D83;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $097FBC: bra.w $97fc0
pub fn i_097fbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97FC0;
    cpu.ir = 0x6000;
    cpu.pc = 0x97FC0;
    10
}
/// $097FC0: jsr $98144(pc)
pub fn i_097fc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97FC4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x97FC4);
    cpu.pc = 0x98144;
    16
}
/// $097FC4: move.w $88(a6), d0
pub fn i_097fc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97FC8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $097FC8: jsr $981ae(pc)
pub fn i_097fc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97FCC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x97FCC);
    cpu.pc = 0x981AE;
    16
}
/// $097FCC: cmpi.w #$b, d1
pub fn i_097fcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97FD0;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xB, Size::Word, false);
    8
}
/// $097FD0: beq.w $98002
pub fn i_097fd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97FD4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x98002; return 10; }
    8
}
/// $097FD4: lea.l $100002.l, a2
pub fn i_097fd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97FDA;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x100002;
    20
}
/// $097FDA: subq.w #$1, d1
pub fn i_097fda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97FDC;
    cpu.ir = 0x5341;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $097FDC: mulu.w #$c, d1
pub fn i_097fdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97FE0;
    cpu.ir = 0xC2FC;
    let v1: u32 = 0xC;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[1] & 0xFFFF);
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $097FE0: adda.l d1, a2
pub fn i_097fe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97FE2;
    cpu.ir = 0xD5C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.a[2] = cpu.a[2].wrapping_add(v1);
    8
}
/// $097FE2: move.w $88(a6), d0
pub fn i_097fe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97FE6;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $097FE6: jsr $98176(pc)
pub fn i_097fe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97FEA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x97FEA);
    cpu.pc = 0x98176;
    16
}
/// $097FEA: move.b d1, $1(a2)
pub fn i_097fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97FEE;
    cpu.ir = 0x1541;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[2].wrapping_add(0x1);
    bus.write8(ad2, v1 as u8);
    12
}
/// $097FEE: clr.b (a2)
pub fn i_097fee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97FF0;
    cpu.ir = 0x4212;
    let ad1 = cpu.a[2];
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    8
}
/// $097FF0: move.w $74(a6), $6(a2)
pub fn i_097ff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97FF6;
    cpu.ir = 0x356E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2].wrapping_add(0x6);
    bus.write16(ad4, v3 as u16);
    20
}
/// $097FF6: move.w $76(a6), $8(a2)
pub fn i_097ff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97FFC;
    cpu.ir = 0x356E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2].wrapping_add(0x8);
    bus.write16(ad4, v3 as u16);
    20
}
/// $097FFC: move.w $78(a6), $a(a2)
pub fn i_097ffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98002;
    cpu.ir = 0x356E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2].wrapping_add(0xA);
    bus.write16(ad4, v3 as u16);
    20
}
/// $098002: move.w d1, $8a(a6)
pub fn i_098002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98006;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $098006: move.w d1, d0
pub fn i_098006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98008;
    cpu.ir = 0x3001;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $098008: movea.l $80(a6), a1
pub fn i_098008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9800C;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $09800C: jsr $97c02(pc)
pub fn i_09800c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98010;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x98010);
    cpu.pc = 0x97C02;
    16
}
/// $098010: movea.l $80(a6), a1
pub fn i_098010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98014;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $098014: adda.l $84(a6), a1
pub fn i_098014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98018;
    cpu.ir = 0xD3EE;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read32(ad2);
    cpu.a[1] = cpu.a[1].wrapping_add(v3);
    20
}
/// $098018: movea.l a6, a0
pub fn i_098018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9801A;
    cpu.ir = 0x204E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $09801A: adda.l #$74, a0
pub fn i_09801a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98020;
    cpu.ir = 0xD1FC;
    let v1: u32 = 0x74;
    cpu.a[0] = cpu.a[0].wrapping_add(v1);
    8
}
/// $098020: jsr $97c5c(pc)
pub fn i_098020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98024;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x98024);
    cpu.pc = 0x97C5C;
    16
}
/// $098024: tst.b $21(a6)
pub fn i_098024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98028;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $098028: beq.w $98038
pub fn i_098028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9802C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x98038; return 10; }
    8
}
/// $09802C: movea.l $c(a6), a0
pub fn i_09802c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98030;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $098030: subq.b #$1, $20(a0)
pub fn i_098030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98034;
    cpu.ir = 0x5328;
    let ad1 = cpu.a[0].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $098034: bra.w $98044
pub fn i_098034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98038;
    cpu.ir = 0x6000;
    cpu.pc = 0x98044;
    10
}
/// $098038: movea.l $c(a6), a0
pub fn i_098038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9803C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $09803C: tst.b $20(a0)
pub fn i_09803c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98040;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $098040: beq.w $98052
pub fn i_098040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98044;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x98052; return 10; }
    8
}
/// $098044: move.w #$0, $7a(a6)
pub fn i_098044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9804A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_09804a  ($09804A, 8 B, kernel/task_handlers.c)
/// $09804A: lea.l $9806a(pc), a1
pub fn i_09804a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9804E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9806A;
    16
}
/// $09804E: move.l a1, (a6)
pub fn i_09804e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98050;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $098050: rts
pub fn i_098050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98052;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== NameEntry_Finish_098052  ($098052, 24 B, frontend/hiscore_memcard_options_09773c.s)
/// $098052: tst.b $21(a6)
pub fn i_098052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98056;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $098056: bne.w $98062
pub fn i_098056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9805A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x98062; return 10; }
    8
}
/// $09805A: movea.l $c(a6), a0
pub fn i_09805a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9805E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $09805E: clr.b $21(a0)
pub fn i_09805e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98062;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $098062: jmp $518.l
pub fn i_098062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98068;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== NameEntry_Blink_09806a  ($09806A, 124 B, frontend/hiscore_memcard_options_09773c.s)
/// $09806A: tst.b $21(a6)
pub fn i_09806a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9806E;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $09806E: bne.w $9807c
pub fn i_09806e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98072;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9807C; return 10; }
    8
}
/// $098072: movea.l $c(a6), a0
pub fn i_098072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98076;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $098076: tst.b $20(a0)
pub fn i_098076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9807A;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $09807A: beq.b $98052
pub fn i_09807a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9807C;
    cpu.ir = 0x67D6;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x98052; return 10; }
    8
}
/// $09807C: addq.w #$1, $7a(a6)
pub fn i_09807c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98080;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x7A);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $098080: move.w $7a(a6), d0
pub fn i_098080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98084;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $098084: cmpi.w #$60, d0
pub fn i_098084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98088;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x60, Size::Word, false);
    8
}
/// $098088: bge.b $98052
pub fn i_098088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9808A;
    cpu.ir = 0x6CC8;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x98052; return 10; }
    8
}
/// $09808A: cmpi.w #$48, d0
pub fn i_09808a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9808E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x48, Size::Word, false);
    8
}
/// $09808E: bge.w $9809a
pub fn i_09808e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98092;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x9809A; return 10; }
    8
}
/// $098092: andi.w #$8, d0
pub fn i_098092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98096;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x8) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $098096: bne.w $980ec
pub fn i_098096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9809A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x980EC; return 10; }
    8
}
/// $09809A: movea.l $80(a6), a1
pub fn i_09809a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9809E;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $09809E: move.w $8a(a6), d0
pub fn i_09809e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x980A2;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0980A2: jsr $97c02(pc)
pub fn i_0980a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x980A6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x980A6);
    cpu.pc = 0x97C02;
    16
}
/// $0980A6: tst.b $21(a6)
pub fn i_0980a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x980AA;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0980AA: beq.w $980d4
pub fn i_0980aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x980AE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x980D4; return 10; }
    8
}
/// $0980AE: lea.l $100002.l, a1
pub fn i_0980ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x980B4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x100002;
    20
}
/// $0980B4: move.w $8a(a6), d1
pub fn i_0980b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x980B8;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0980B8: subq.w #$1, d1
pub fn i_0980b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x980BA;
    cpu.ir = 0x5341;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0980BA: mulu.w #$c, d1
pub fn i_0980ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x980BE;
    cpu.ir = 0xC2FC;
    let v1: u32 = 0xC;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[1] & 0xFFFF);
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $0980BE: move.l $2(a1, d1.w), d0
pub fn i_0980be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x980C2;
    cpu.ir = 0x2031;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $0980C2: movea.l $80(a6), a1
pub fn i_0980c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x980C6;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0980C6: adda.l #$c0, a1
pub fn i_0980c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x980CC;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0xC0;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $0980CC: moveq #$4, d1
pub fn i_0980cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x980CE;
    cpu.ir = 0x7204;
    cpu.d[1] = 0x4;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0980CE: jsr $5d7be.l
pub fn i_0980ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x980D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x980D4);
    cpu.pc = 0x5D7BE;
    16
}
/// $0980D4: movea.l $80(a6), a1
pub fn i_0980d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x980D8;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0980D8: adda.l $84(a6), a1
pub fn i_0980d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x980DC;
    cpu.ir = 0xD3EE;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read32(ad2);
    cpu.a[1] = cpu.a[1].wrapping_add(v3);
    20
}
/// $0980DC: movea.l a6, a0
pub fn i_0980dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x980DE;
    cpu.ir = 0x204E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0980DE: adda.l #$74, a0
pub fn i_0980de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x980E4;
    cpu.ir = 0xD1FC;
    let v1: u32 = 0x74;
    cpu.a[0] = cpu.a[0].wrapping_add(v1);
    8
}
/// $0980E4: moveq #$3, d0
pub fn i_0980e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x980E6;
    cpu.ir = 0x7003;
    cpu.d[0] = 0x3;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
// ===== JsrPcThunk_0980e6  ($0980E6, 6 B, thunks/jsr_pc_thunks.c)
/// $0980E6: jsr $97c5c(pc)
pub fn i_0980e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x980EA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x980EA);
    cpu.pc = 0x97C5C;
    16
}
/// $0980EA: rts
pub fn i_0980ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x980EC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== NameEntry_ClearRow_0980ec  ($0980EC, 80 B, frontend/hiscore_memcard_options_09773c.s)
/// $0980EC: movea.l $80(a6), a1
pub fn i_0980ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x980F0;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0980F0: move.w #$b40, d0
pub fn i_0980f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x980F4;
    cpu.ir = 0x303C;
    let v1: u32 = 0xB40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0980F4: ori.w #$4000, d0
pub fn i_0980f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x980F8;
    cpu.ir = 0x40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 | 0x4000) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0980F8: moveq #$6, d1
pub fn i_0980f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x980FA;
    cpu.ir = 0x7206;
    cpu.d[1] = 0x6;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0980FA: moveq #$2, d2
pub fn i_0980fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x980FC;
    cpu.ir = 0x7402;
    cpu.d[2] = 0x2;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0980FC: jsr $5da9c.l
pub fn i_0980fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98102;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98102);
    cpu.pc = 0x5DA9C;
    16
}
/// $098102: tst.b $21(a6)
pub fn i_098102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98106;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $098106: beq.w $98128
pub fn i_098106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9810A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x98128; return 10; }
    8
}
/// $09810A: movea.l $80(a6), a1
pub fn i_09810a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9810E;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $09810E: adda.l #$c0, a1
pub fn i_09810e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98114;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0xC0;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $098114: move.w #$b40, d0
pub fn i_098114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98118;
    cpu.ir = 0x303C;
    let v1: u32 = 0xB40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $098118: ori.w #$4000, d0
pub fn i_098118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9811C;
    cpu.ir = 0x40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 | 0x4000) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09811C: move.w #$10, d1
pub fn i_09811c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98120;
    cpu.ir = 0x323C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $098120: moveq #$2, d2
pub fn i_098120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98122;
    cpu.ir = 0x7402;
    cpu.d[2] = 0x2;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $098122: jsr $5da9c.l
pub fn i_098122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98128;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98128);
    cpu.pc = 0x5DA9C;
    16
}
/// $098128: movea.l $80(a6), a1
pub fn i_098128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9812C;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $09812C: adda.l $84(a6), a1
pub fn i_09812c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98130;
    cpu.ir = 0xD3EE;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read32(ad2);
    cpu.a[1] = cpu.a[1].wrapping_add(v3);
    20
}
/// $098130: move.w #$b40, d0
pub fn i_098130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98134;
    cpu.ir = 0x303C;
    let v1: u32 = 0xB40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $098134: ori.w #$4000, d0
pub fn i_098134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98138;
    cpu.ir = 0x40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 | 0x4000) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $098138: moveq #$6, d1
pub fn i_098138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9813A;
    cpu.ir = 0x7206;
    cpu.d[1] = 0x6;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $09813A: moveq #$2, d2
pub fn i_09813a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9813C;
    cpu.ir = 0x7402;
    cpu.d[2] = 0x2;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
// ===== JsrAbsThunk_09813c  ($09813C, 8 B, thunks/jsr_abs_thunks.c)
/// $09813C: jsr $5da9c.l
pub fn i_09813c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98142;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98142);
    cpu.pc = 0x5DA9C;
    16
}
/// $098142: rts
pub fn i_098142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98144;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== NameEntry_CensorName_098144  ($098144, 50 B, frontend/hiscore_memcard_options_09773c.s)
/// $098144: cmpi.w #$ba6, $74(a6)
pub fn i_098144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9814A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xBA6, Size::Word, false);
    16
}
/// $09814A: bne.w $98174
pub fn i_09814a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9814E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x98174; return 10; }
    8
}
/// $09814E: cmpi.w #$b8a, $76(a6)
pub fn i_09814e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98154;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xB8A, Size::Word, false);
    16
}
/// $098154: bne.w $98174
pub fn i_098154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98158;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x98174; return 10; }
    8
}
/// $098158: cmpi.w #$ca0, $78(a6)
pub fn i_098158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9815E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xCA0, Size::Word, false);
    16
}
/// $09815E: bne.w $98174
pub fn i_09815e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98162;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x98174; return 10; }
    8
}
/// $098162: move.w #$c8e, $74(a6)
pub fn i_098162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98168;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC8E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $098168: move.w #$c80, $76(a6)
pub fn i_098168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9816E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09816E: move.w #$b42, $78(a6)
pub fn i_09816e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98174;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xB42;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad2, v1 as u16);
    12
}
/// $098174: rts
pub fn i_098174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98176;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HiScore_RankOfPlayer_098176  ($098176, 56 B, frontend/hiscore_memcard_options_09773c.s)
/// $098176: lea.l $100002.l, a1
pub fn i_098176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9817C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x100002;
    20
}
/// $09817C: moveq #$1, d1
pub fn i_09817c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9817E;
    cpu.ir = 0x7201;
    cpu.d[1] = 0x1;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $09817E: cmp.b (a1), d0
pub fn i_09817e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98180;
    cpu.ir = 0xB011;
    let v2 = bus.read8(cpu.a[1]) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v2, Size::Byte, false);
    8
}
/// $098180: beq.w $981a4
pub fn i_098180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98184;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x981A4; return 10; }
    8
}
/// $098184: adda.l #$c, a1
pub fn i_098184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9818A;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0xC;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $09818A: addq.w #$1, d1
pub fn i_09818a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9818C;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $09818C: cmpi.w #$b, d1
pub fn i_09818c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98190;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xB, Size::Word, false);
    8
}
/// $098190: bne.b $9817e
pub fn i_098190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98192;
    cpu.ir = 0x66EC;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9817E; return 10; }
    8
}
/// $098192: suba.l #$c, a1
pub fn i_098192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98198;
    cpu.ir = 0x93FC;
    let v1: u32 = 0xC;
    cpu.a[1] = cpu.a[1].wrapping_sub(v1);
    8
}
/// $098198: move.b $1(a1), d1
pub fn i_098198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9819C;
    cpu.ir = 0x1229;
    let ad2 = cpu.a[1].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09819C: addq.b #$1, d1
pub fn i_09819c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9819E;
    cpu.ir = 0x5201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $09819E: andi.w #$ff, d1
pub fn i_09819e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x981A2;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0981A2: rts
pub fn i_0981a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x981A4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0981A4: move.b $1(a1), d1
pub fn i_0981a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x981A8;
    cpu.ir = 0x1229;
    let ad2 = cpu.a[1].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0981A8: andi.w #$ff, d1
pub fn i_0981a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x981AC;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0981AC: rts
pub fn i_0981ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x981AE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HiScore_FindPlayerRow_0981ae  ($0981AE, 30 B, frontend/hiscore_memcard_options_09773c.s)
/// $0981AE: lea.l $100002.l, a1
pub fn i_0981ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x981B4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x100002;
    20
}
/// $0981B4: moveq #$1, d1
pub fn i_0981b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x981B6;
    cpu.ir = 0x7201;
    cpu.d[1] = 0x1;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0981B6: cmp.b (a1), d0
pub fn i_0981b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x981B8;
    cpu.ir = 0xB011;
    let v2 = bus.read8(cpu.a[1]) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v2, Size::Byte, false);
    8
}
/// $0981B8: beq.w $981ca
pub fn i_0981b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x981BC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x981CA; return 10; }
    8
}
/// $0981BC: adda.l #$c, a1
pub fn i_0981bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x981C2;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0xC;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $0981C2: addq.w #$1, d1
pub fn i_0981c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x981C4;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0981C4: cmpi.w #$b, d1
pub fn i_0981c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x981C8;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xB, Size::Word, false);
    8
}
/// $0981C8: bne.b $981b6
pub fn i_0981c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x981CA;
    cpu.ir = 0x66EC;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x981B6; return 10; }
    8
}
/// $0981CA: rts
pub fn i_0981ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x981CC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CmpPrioWithSibling_0981cc  ($0981CC, 16 B, frontend/hiscore_memcard_options_09773c.s)
/// $0981CC: movea.l $8(a6), a1
pub fn i_0981cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x981D0;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0981D0: move.b $10(a6), d0
pub fn i_0981d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x981D4;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0981D4: cmp.b $10(a1), d0
pub fn i_0981d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x981D8;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $0981D8: bcs.w $981e2
pub fn i_0981d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x981DC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x981E2; return 10; }
    8
}
// ===== ClearXN_0981dc  ($0981DC, 6 B, thunks/ccr_helpers.c)
/// $0981DC: andi.b #$ee, ccr
pub fn i_0981dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x981E0;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0981E0: rts
pub fn i_0981e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x981E2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_0981e2  ($0981E2, 6 B, thunks/ccr_helpers.c)
/// $0981E2: ori.b #$11, ccr
pub fn i_0981e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x981E6;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0981E6: rts
pub fn i_0981e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x981E8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MemCard_FileName_0981e8  ($0981E8, 20 B, frontend/hiscore_memcard_options_09773c.s)
/// $0981E8: ?
pub fn i_0981e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x981EA;
    cpu.ir = 0x4D45;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
// ===== Copy2Bytes_10FDB6to10E3A0_0981FC  ($0981FC, 18 B, kernel/copy_start_masks_0981fc.s)
/// $0981FC: lea.l $10fdb6.l, a0
pub fn i_0981fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98202;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10FDB6;
    20
}
/// $098202: lea.l $10e3a0.l, a1
pub fn i_098202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98208;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x10E3A0;
    20
}
/// $098208: move.b (a0)+, (a1)+
pub fn i_098208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9820A;
    cpu.ir = 0x12D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $09820A: move.b (a0), (a1)
pub fn i_09820a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9820C;
    cpu.ir = 0x1290;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[1], v2 as u8);
    12
}
/// $09820C: rts
pub fn i_09820c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9820E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Fix_DrawGlyphList_09820e  ($09820E, 36 B, frontend/hiscore_memcard_options_09773c.s)
/// $09820E: move.w (a0)+, d0
pub fn i_09820e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98210;
    cpu.ir = 0x3018;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $098210: cmpi.w #$ffff, d0
pub fn i_098210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98214;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFFF, Size::Word, false);
    8
}
/// $098214: beq.w $98230
pub fn i_098214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98218;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x98230; return 10; }
    8
}
/// $098218: move.w #$2, d1
pub fn i_098218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9821C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09821C: move.w #$2, d2
pub fn i_09821c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98220;
    cpu.ir = 0x343C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $098220: move.l a1, -(a7)
pub fn i_098220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98222;
    cpu.ir = 0x2F09;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $098222: jsr $5da56.l
pub fn i_098222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98228;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98228);
    cpu.pc = 0x5DA56;
    16
}
/// $098228: movea.l (a7)+, a1
pub fn i_098228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9822A;
    cpu.ir = 0x225F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    12
}
/// $09822A: adda.w #$40, a1
pub fn i_09822a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9822E;
    cpu.ir = 0xD2FC;
    let v1: u32 = 0x40;
    cpu.a[1] = cpu.a[1].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $09822E: bra.b $9820e
pub fn i_09822e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98230;
    cpu.ir = 0x60DE;
    cpu.pc = 0x9820E;
    10
}
/// $098230: rts
pub fn i_098230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98232;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Input_EdgePressedPlayers_098232  ($098232, 60 B, frontend/hiscore_memcard_options_09773c.s)
/// $098232: moveq #$0, d0
pub fn i_098232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98234;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $098234: move.b $10e3a0.l, d7
pub fn i_098234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9823A;
    cpu.ir = 0x1E39;
    let v2 = bus.read8(0x10E3A0) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v2 & 0xFF);
    16
}
/// $09823A: subq.b #$1, d7
pub fn i_09823a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9823C;
    cpu.ir = 0x5307;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    4
}
/// $09823C: cmpi.b #$2, d7
pub fn i_09823c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98240;
    cpu.ir = 0xC07;
    let v1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $098240: bcc.w $9824a
pub fn i_098240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98244;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9824A; return 10; }
    8
}
/// $098244: or.b $10fd96.l, d0
pub fn i_098244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9824A;
    cpu.ir = 0x8039;
    let v2 = bus.read8(0x10FD96) as u32;
    let r = (v2 | cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $09824A: move.b $10e3a1.l, d7
pub fn i_09824a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98250;
    cpu.ir = 0x1E39;
    let v2 = bus.read8(0x10E3A1) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v2 & 0xFF);
    16
}
/// $098250: subq.b #$1, d7
pub fn i_098250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98252;
    cpu.ir = 0x5307;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    4
}
/// $098252: cmpi.b #$2, d7
pub fn i_098252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98256;
    cpu.ir = 0xC07;
    let v1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $098256: bcc.w $98260
pub fn i_098256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9825A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x98260; return 10; }
    8
}
/// $09825A: or.b $10fd9c.l, d0
pub fn i_09825a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98260;
    cpu.ir = 0x8039;
    let v2 = bus.read8(0x10FD9C) as u32;
    let r = (v2 | cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $098260: move.b $74(a6), d1
pub fn i_098260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98264;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $098264: move.b d0, $74(a6)
pub fn i_098264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98268;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write8(ad2, v1 as u8);
    12
}
/// $098268: eor.b d0, d1
pub fn i_098268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9826A;
    cpu.ir = 0xB101;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 ^ cpu.d[0]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $09826A: and.b d1, d0
pub fn i_09826a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9826C;
    cpu.ir = 0xC001;
    let v1 = cpu.d[1] & 0xFF;
    let r = (v1 & cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $09826C: rts
pub fn i_09826c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9826E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MemCard_Detect_09826e  ($09826E, 14 B, frontend/hiscore_memcard_options_09773c.s)
/// $09826E: move.b $380000.l, d0
pub fn i_09826e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98274;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x380000) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $098274: andi.b #$30, d0
pub fn i_098274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98278;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x30) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $098278: beq.w $98282
pub fn i_098278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9827C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x98282; return 10; }
    8
}
// ===== ClearC_09827c  ($09827C, 6 B, thunks/ccr_helpers.c)
/// $09827C: andi.b #$fe, ccr
pub fn i_09827c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98280;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $098280: rts
pub fn i_098280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98282;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetC_098282  ($098282, 6 B, thunks/ccr_helpers.c)
/// $098282: ori.b #$1, ccr
pub fn i_098282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98286;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $098286: rts
pub fn i_098286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98288;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MemCard_LoadDialog_Init_098288  ($098288, 120 B, frontend/hiscore_memcard_options_09773c.s)
/// $098288: jsr $98578(pc)
pub fn i_098288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9828C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9828C);
    cpu.pc = 0x98578;
    16
}
/// $09828C: bcs.w $983b0
pub fn i_09828c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98290;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x983B0; return 10; }
    8
}
/// $098290: movea.l $98(a6), a0
pub fn i_098290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98294;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $098294: cmpa.l #$0, a0
pub fn i_098294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9829A;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0x0;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $09829A: beq.w $982a0
pub fn i_09829a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9829E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x982A0; return 10; }
    8
}
/// $09829E: jsr (a0)
pub fn i_09829e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x982A0;
    cpu.ir = 0x4E90;
    cpu.push32(bus, 0x982A0);
    cpu.jump_to(cpu.a[0]);
    16
}
/// $0982A0: movea.l #$7000, a1
pub fn i_0982a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x982A6;
    cpu.ir = 0x227C;
    let v1: u32 = 0x7000;
    cpu.a[1] = v1;
    4
}
/// $0982A6: move.w #$ff, d0
pub fn i_0982a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x982AA;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0982AA: move.w #$26, d1
pub fn i_0982aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x982AE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x26;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0982AE: move.w #$1b, d2
pub fn i_0982ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x982B2;
    cpu.ir = 0x343C;
    let v1: u32 = 0x1B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0982B2: jsr $5da9c.l
pub fn i_0982b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x982B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x982B8);
    cpu.pc = 0x5DA9C;
    16
}
/// $0982B8: movea.l #$7204, a1
pub fn i_0982b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x982BE;
    cpu.ir = 0x227C;
    let v1: u32 = 0x7204;
    cpu.a[1] = v1;
    4
}
/// $0982BE: lea.l $98692(pc), a0
pub fn i_0982be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x982C2;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x98692;
    16
}
/// $0982C2: bsr.w $9820e
pub fn i_0982c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x982C6;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x982C6);
    cpu.pc = 0x9820E;
    18
}
/// $0982C6: movea.l #$724a, a1
pub fn i_0982c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x982CC;
    cpu.ir = 0x227C;
    let v1: u32 = 0x724A;
    cpu.a[1] = v1;
    4
}
/// $0982CC: lea.l $9869e(pc), a0
pub fn i_0982cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x982D0;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x9869E;
    16
}
/// $0982D0: bsr.w $9820e
pub fn i_0982d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x982D4;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x982D4);
    cpu.pc = 0x9820E;
    18
}
/// $0982D4: movea.l #$724e, a1
pub fn i_0982d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x982DA;
    cpu.ir = 0x227C;
    let v1: u32 = 0x724E;
    cpu.a[1] = v1;
    4
}
/// $0982DA: lea.l $986a6(pc), a0
pub fn i_0982da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x982DE;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x986A6;
    16
}
/// $0982DE: bsr.w $9820e
pub fn i_0982de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x982E2;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x982E2);
    cpu.pc = 0x9820E;
    18
}
/// $0982E2: movea.l #$7116, a1
pub fn i_0982e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x982E8;
    cpu.ir = 0x227C;
    let v1: u32 = 0x7116;
    cpu.a[1] = v1;
    4
}
/// $0982E8: lea.l $986ac(pc), a0
pub fn i_0982e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x982EC;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x986AC;
    16
}
/// $0982EC: bsr.w $9820e
pub fn i_0982ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x982F0;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x982F0);
    cpu.pc = 0x9820E;
    18
}
/// $0982F0: move.w #$0, $72(a6)
pub fn i_0982f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x982F6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0982F6: jsr $98232(pc)
pub fn i_0982f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x982FA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x982FA);
    cpu.pc = 0x98232;
    16
}
/// $0982FA: move.w #$12c, $70(a6)
pub fn i_0982fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98300;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x12C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_098300  ($098300, 8 B, kernel/task_handlers.c)
/// $098300: lea.l $98308(pc), a1
pub fn i_098300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98304;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x98308;
    16
}
/// $098304: move.l a1, (a6)
pub fn i_098304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98306;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $098306: rts
pub fn i_098306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98308;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MemCard_LoadDialog_Run_098308  ($098308, 230 B, frontend/hiscore_memcard_options_09773c.s)
/// $098308: lea.l $98700(pc), a0
pub fn i_098308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9830C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x98700;
    16
}
/// $09830C: move.w $72(a6), d0
pub fn i_09830c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98310;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $098310: andi.w #$1, d0
pub fn i_098310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98314;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $098314: add.w d0, d0
pub fn i_098314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98316;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $098316: move.w (a0, d0.w), d0
pub fn i_098316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9831A;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $09831A: movea.l #$720a, a1
pub fn i_09831a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98320;
    cpu.ir = 0x227C;
    let v1: u32 = 0x720A;
    cpu.a[1] = v1;
    4
}
/// $098320: move.w #$2, d1
pub fn i_098320(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98324;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $098324: move.w #$2, d2
pub fn i_098324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98328;
    cpu.ir = 0x343C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $098328: jsr $5da56.l
pub fn i_098328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9832E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9832E);
    cpu.pc = 0x5DA56;
    16
}
/// $09832E: lea.l $98700(pc), a0
pub fn i_09832e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98332;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x98700;
    16
}
/// $098332: move.w $72(a6), d0
pub fn i_098332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98336;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $098336: addq.w #$1, d0
pub fn i_098336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98338;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $098338: andi.w #$1, d0
pub fn i_098338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9833C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09833C: add.w d0, d0
pub fn i_09833c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9833E;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09833E: move.w (a0, d0.w), d0
pub fn i_09833e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98342;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $098342: movea.l #$720e, a1
pub fn i_098342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98348;
    cpu.ir = 0x227C;
    let v1: u32 = 0x720E;
    cpu.a[1] = v1;
    4
}
/// $098348: move.w #$2, d1
pub fn i_098348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9834C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09834C: move.w #$2, d2
pub fn i_09834c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98350;
    cpu.ir = 0x343C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $098350: jsr $5da56.l
pub fn i_098350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98356;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98356);
    cpu.pc = 0x5DA56;
    16
}
/// $098356: subq.w #$1, $70(a6)
pub fn i_098356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9835A;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09835A: beq.w $98388
pub fn i_09835a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9835E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x98388; return 10; }
    8
}
/// $09835E: bsr.w $98232
pub fn i_09835e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98362;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x98362);
    cpu.pc = 0x98232;
    18
}
/// $098362: btst.b #$4, d0
pub fn i_098362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98366;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((4) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $098366: bne.w $98388
pub fn i_098366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9836A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x98388; return 10; }
    8
}
/// $09836A: btst.b #$0, d0
pub fn i_09836a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9836E;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $09836E: beq.w $98378
pub fn i_09836e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98372;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x98378; return 10; }
    8
}
/// $098372: move.w #$0, $72(a6)
pub fn i_098372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98378;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $098378: btst.b #$1, d0
pub fn i_098378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9837C;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((1) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $09837C: beq.w $98386
pub fn i_09837c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98380;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x98386; return 10; }
    8
}
/// $098380: move.w #$1, $72(a6)
pub fn i_098380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98386;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $098386: rts
pub fn i_098386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98388;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $098388: move.w #$ff, d0
pub fn i_098388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9838C;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09838C: movea.l #$7116, a1
pub fn i_09838c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98392;
    cpu.ir = 0x227C;
    let v1: u32 = 0x7116;
    cpu.a[1] = v1;
    4
}
/// $098392: move.w #$1a, d1
pub fn i_098392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98396;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $098396: move.w #$2, d2
pub fn i_098396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9839A;
    cpu.ir = 0x343C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09839A: jsr $5da9c.l
pub fn i_09839a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x983A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x983A0);
    cpu.pc = 0x5DA9C;
    16
}
/// $0983A0: tst.w $72(a6)
pub fn i_0983a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x983A4;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $0983A4: bne.w $983b0
pub fn i_0983a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x983A8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x983B0; return 10; }
    8
}
/// $0983A8: jsr $98578(pc)
pub fn i_0983a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x983AC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x983AC);
    cpu.pc = 0x98578;
    16
}
/// $0983AC: bcc.w $983da
pub fn i_0983ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x983B0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x983DA; return 10; }
    8
}
/// $0983B0: lea.l $10e3a2.l, a0
pub fn i_0983b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x983B6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E3A2;
    20
}
/// $0983B6: clr.b $14(a0)
pub fn i_0983b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x983BA;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x14);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0983BA: clr.b $15(a0)
pub fn i_0983ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x983BE;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x15);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0983BE: clr.b $16(a0)
pub fn i_0983be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x983C2;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x16);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0983C2: clr.b $17(a0)
pub fn i_0983c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x983C6;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x17);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0983C6: clr.b $18(a0)
pub fn i_0983c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x983CA;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x18);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0983CA: clr.b $19(a0)
pub fn i_0983ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x983CE;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x19);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0983CE: clr.b $1a(a0)
pub fn i_0983ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x983D2;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x1A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0983D2: clr.b $1b(a0)
pub fn i_0983d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x983D6;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x1B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0983D6: bra.w $983fe
pub fn i_0983d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x983DA;
    cpu.ir = 0x6000;
    cpu.pc = 0x983FE;
    10
}
/// $0983DA: movea.l #$7152, a1
pub fn i_0983da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x983E0;
    cpu.ir = 0x227C;
    let v1: u32 = 0x7152;
    cpu.a[1] = v1;
    4
}
/// $0983E0: lea.l $986c8(pc), a0
pub fn i_0983e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x983E4;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x986C8;
    16
}
/// $0983E4: bsr.w $9820e
pub fn i_0983e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x983E8;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x983E8);
    cpu.pc = 0x9820E;
    18
}
/// $0983E8: move.w #$1e, $70(a6)
pub fn i_0983e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x983EE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_0983ee  ($0983EE, 8 B, kernel/task_handlers.c)
/// $0983EE: lea.l $983f6(pc), a1
pub fn i_0983ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x983F2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x983F6;
    16
}
/// $0983F2: move.l a1, (a6)
pub fn i_0983f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x983F4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0983F4: rts
pub fn i_0983f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x983F6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MemCard_Dialog_ExitDelay_0983f6  ($0983F6, 22 B, frontend/hiscore_memcard_options_09773c.s)
/// $0983F6: subq.w #$1, $70(a6)
pub fn i_0983f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x983FA;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0983FA: bne.w $9840a
pub fn i_0983fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x983FE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9840A; return 10; }
    8
}
/// $0983FE: clr.b $106ed2.l
pub fn i_0983fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98404;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106ED2) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106ED2, r2 as u8);
    16
}
/// $098404: jmp $518.l
pub fn i_098404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9840A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $09840A: rts
pub fn i_09840a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9840C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MemCard_SaveDialog_Init_09840c  ($09840C, 110 B, frontend/hiscore_memcard_options_09773c.s)
/// $09840C: move.b $380000.l, d0
pub fn i_09840c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98412;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x380000) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $098412: andi.b #$30, d0
pub fn i_098412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98416;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x30) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $098416: beq.w $9841e
pub fn i_098416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9841A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9841E; return 10; }
    8
}
/// $09841A: bra.w $9856a
pub fn i_09841a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9841E;
    cpu.ir = 0x6000;
    cpu.pc = 0x9856A;
    10
}
/// $09841E: movea.l #$7000, a1
pub fn i_09841e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98424;
    cpu.ir = 0x227C;
    let v1: u32 = 0x7000;
    cpu.a[1] = v1;
    4
}
/// $098424: move.w #$ff, d0
pub fn i_098424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98428;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $098428: move.w #$26, d1
pub fn i_098428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9842C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x26;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09842C: move.w #$1b, d2
pub fn i_09842c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98430;
    cpu.ir = 0x343C;
    let v1: u32 = 0x1B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $098430: jsr $5da9c.l
pub fn i_098430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98436;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98436);
    cpu.pc = 0x5DA9C;
    16
}
/// $098436: movea.l #$7204, a1
pub fn i_098436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9843C;
    cpu.ir = 0x227C;
    let v1: u32 = 0x7204;
    cpu.a[1] = v1;
    4
}
/// $09843C: lea.l $986de(pc), a0
pub fn i_09843c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98440;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x986DE;
    16
}
/// $098440: bsr.w $9820e
pub fn i_098440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98444;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x98444);
    cpu.pc = 0x9820E;
    18
}
/// $098444: movea.l #$724a, a1
pub fn i_098444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9844A;
    cpu.ir = 0x227C;
    let v1: u32 = 0x724A;
    cpu.a[1] = v1;
    4
}
/// $09844A: lea.l $9869e(pc), a0
pub fn i_09844a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9844E;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x9869E;
    16
}
/// $09844E: bsr.w $9820e
pub fn i_09844e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98452;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x98452);
    cpu.pc = 0x9820E;
    18
}
/// $098452: movea.l #$724e, a1
pub fn i_098452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98458;
    cpu.ir = 0x227C;
    let v1: u32 = 0x724E;
    cpu.a[1] = v1;
    4
}
/// $098458: lea.l $986a6(pc), a0
pub fn i_098458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9845C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x986A6;
    16
}
/// $09845C: bsr.w $9820e
pub fn i_09845c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98460;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x98460);
    cpu.pc = 0x9820E;
    18
}
/// $098460: movea.l #$7116, a1
pub fn i_098460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98466;
    cpu.ir = 0x227C;
    let v1: u32 = 0x7116;
    cpu.a[1] = v1;
    4
}
/// $098466: lea.l $986ac(pc), a0
pub fn i_098466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9846A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x986AC;
    16
}
/// $09846A: bsr.w $9820e
pub fn i_09846a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9846E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x9846E);
    cpu.pc = 0x9820E;
    18
}
/// $09846E: move.w #$1, $72(a6)
pub fn i_09846e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98474;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $098474: move.w #$12c, $70(a6)
pub fn i_098474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9847A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x12C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_09847a  ($09847A, 8 B, kernel/task_handlers.c)
/// $09847A: lea.l $98482(pc), a1
pub fn i_09847a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9847E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x98482;
    16
}
/// $09847E: move.l a1, (a6)
pub fn i_09847e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98480;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $098480: rts
pub fn i_098480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98482;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MemCard_SaveDialog_Run_098482  ($098482, 246 B, frontend/hiscore_memcard_options_09773c.s)
/// $098482: lea.l $98700(pc), a0
pub fn i_098482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98486;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x98700;
    16
}
/// $098486: move.w $72(a6), d0
pub fn i_098486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9848A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09848A: add.w d0, d0
pub fn i_09848a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9848C;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09848C: move.w (a0, d0.w), d0
pub fn i_09848c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98490;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $098490: movea.l #$720a, a1
pub fn i_098490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98496;
    cpu.ir = 0x227C;
    let v1: u32 = 0x720A;
    cpu.a[1] = v1;
    4
}
/// $098496: move.w #$2, d1
pub fn i_098496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9849A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09849A: move.w #$2, d2
pub fn i_09849a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9849E;
    cpu.ir = 0x343C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09849E: jsr $5da56.l
pub fn i_09849e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x984A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x984A4);
    cpu.pc = 0x5DA56;
    16
}
/// $0984A4: lea.l $98700(pc), a0
pub fn i_0984a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x984A8;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x98700;
    16
}
/// $0984A8: move.w $72(a6), d0
pub fn i_0984a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x984AC;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0984AC: eori.w #$1, d0
pub fn i_0984ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x984B0;
    cpu.ir = 0xA40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 ^ 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0984B0: add.w d0, d0
pub fn i_0984b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x984B2;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0984B2: move.w (a0, d0.w), d0
pub fn i_0984b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x984B6;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $0984B6: movea.l #$720e, a1
pub fn i_0984b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x984BC;
    cpu.ir = 0x227C;
    let v1: u32 = 0x720E;
    cpu.a[1] = v1;
    4
}
/// $0984BC: move.w #$2, d1
pub fn i_0984bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x984C0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0984C0: move.w #$2, d2
pub fn i_0984c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x984C4;
    cpu.ir = 0x343C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0984C4: jsr $5da56.l
pub fn i_0984c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x984CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x984CA);
    cpu.pc = 0x5DA56;
    16
}
/// $0984CA: subq.w #$1, $70(a6)
pub fn i_0984ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x984CE;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0984CE: beq.w $984fc
pub fn i_0984ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x984D2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x984FC; return 10; }
    8
}
/// $0984D2: bsr.w $98232
pub fn i_0984d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x984D6;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x984D6);
    cpu.pc = 0x98232;
    18
}
/// $0984D6: btst.b #$4, d0
pub fn i_0984d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x984DA;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((4) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $0984DA: bne.w $984fc
pub fn i_0984da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x984DE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x984FC; return 10; }
    8
}
/// $0984DE: btst.b #$0, d0
pub fn i_0984de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x984E2;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $0984E2: beq.w $984ec
pub fn i_0984e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x984E6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x984EC; return 10; }
    8
}
/// $0984E6: move.w #$0, $72(a6)
pub fn i_0984e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x984EC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0984EC: btst.b #$1, d0
pub fn i_0984ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x984F0;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((1) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $0984F0: beq.w $984fa
pub fn i_0984f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x984F4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x984FA; return 10; }
    8
}
/// $0984F4: move.w #$1, $72(a6)
pub fn i_0984f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x984FA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0984FA: rts
pub fn i_0984fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x984FC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0984FC: move.w #$ff, d0
pub fn i_0984fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98500;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $098500: movea.l #$7116, a1
pub fn i_098500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98506;
    cpu.ir = 0x227C;
    let v1: u32 = 0x7116;
    cpu.a[1] = v1;
    4
}
/// $098506: move.w #$1a, d1
pub fn i_098506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9850A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09850A: move.w #$2, d2
pub fn i_09850a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9850E;
    cpu.ir = 0x343C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09850E: jsr $5da9c.l
pub fn i_09850e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98514;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98514);
    cpu.pc = 0x5DA9C;
    16
}
/// $098514: tst.w $72(a6)
pub fn i_098514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98518;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $098518: bne.w $9856a
pub fn i_098518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9851C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9856A; return 10; }
    8
}
/// $09851C: move.b #$0, $3a0005.l
pub fn i_09851c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98524;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0005, v1 as u8);
    16
}
/// $098524: move.b #$0, $3a0017.l
pub fn i_098524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9852C;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0017, v1 as u8);
    16
}
/// $09852C: move.b #$0, $3a0019.l
pub fn i_09852c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98534;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0019, v1 as u8);
    16
}
/// $098534: move.b #$0, $380011.l
pub fn i_098534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9853C;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x380011, v1 as u8);
    16
}
/// $09853C: jsr $9866a(pc)
pub fn i_09853c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98540;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x98540);
    cpu.pc = 0x9866A;
    16
}
/// $098540: jsr $985ca(pc)
pub fn i_098540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98544;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x98544);
    cpu.pc = 0x985CA;
    16
}
/// $098544: bcs.w $9856a
pub fn i_098544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98548;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x9856A; return 10; }
    8
}
/// $098548: movea.l #$7152, a1
pub fn i_098548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9854E;
    cpu.ir = 0x227C;
    let v1: u32 = 0x7152;
    cpu.a[1] = v1;
    4
}
/// $09854E: lea.l $986ea(pc), a0
pub fn i_09854e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98552;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x986EA;
    16
}
/// $098552: bsr.w $9820e
pub fn i_098552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98556;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x98556);
    cpu.pc = 0x9820E;
    18
}
/// $098556: move.w #$1e, $70(a6)
pub fn i_098556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9855C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09855C: lea.l $98562(pc), a1
pub fn i_09855c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98560;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x98562;
    16
}
/// $098560: move.l a1, (a6)
pub fn i_098560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98562;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $098562: subq.w #$1, $70(a6)
pub fn i_098562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98566;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $098566: bne.w $98576
pub fn i_098566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9856A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x98576; return 10; }
    8
}
/// $09856A: clr.b $106ed2.l
pub fn i_09856a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98570;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106ED2) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106ED2, r2 as u8);
    16
}
/// $098570: jmp $518.l
pub fn i_098570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98576;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $098576: rts
pub fn i_098576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98578;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MemCard_BiosRead_098578  ($098578, 66 B, frontend/hiscore_memcard_options_09773c.s)
/// $098578: movem.l d0-d7/a0-a6, -(a7)
pub fn i_098578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9857C;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[7]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $09857C: move.b #$2, $10fdc4.l
pub fn i_09857c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98584;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10FDC4, v1 as u8);
    16
}
/// $098584: move.w #$201, $10fdce.l
pub fn i_098584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9858C;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x201;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10FDCE, v1 as u16);
    16
}
/// $09858C: move.b #$0, $10fdd0.l
pub fn i_09858c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98594;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10FDD0, v1 as u8);
    16
}
/// $098594: move.l #$10e3a2, $10fdc8.l
pub fn i_098594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9859E;
    cpu.ir = 0x23FC;
    let v1: u32 = 0x10E3A2;
    bus.write32(0x10FDC8, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $09859E: move.w #$1c, $10fdcc.l
pub fn i_09859e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x985A6;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x1C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10FDCC, v1 as u16);
    16
}
/// $0985A6: jsr $c00468.l
pub fn i_0985a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x985AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x985AC);
    cpu.pc = 0xC00468;
    16
}
/// $0985AC: tst.b $10fdc6.l
pub fn i_0985ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x985B2;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FDC6) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $0985B2: beq.w $985c0
pub fn i_0985b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x985B6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x985C0; return 10; }
    8
}
/// $0985B6: movem.l (a7)+, d0-d7/a0-a6
pub fn i_0985b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x985BA;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[7] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
// ===== SetXN_0985ba  ($0985BA, 6 B, thunks/ccr_helpers.c)
/// $0985BA: ori.b #$11, ccr
pub fn i_0985ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x985BE;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0985BE: rts
pub fn i_0985be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x985C0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MemCard_BiosRead_Ok_0985c0  ($0985C0, 4 B, frontend/hiscore_memcard_options_09773c.s)
/// $0985C0: movem.l (a7)+, d0-d7/a0-a6
pub fn i_0985c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x985C4;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[7] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
// ===== ClearXN_0985c4  ($0985C4, 6 B, thunks/ccr_helpers.c)
/// $0985C4: andi.b #$ee, ccr
pub fn i_0985c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x985C8;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0985C8: rts
pub fn i_0985c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x985CA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MemCard_BiosWrite_0985ca  ($0985CA, 144 B, frontend/hiscore_memcard_options_09773c.s)
/// $0985CA: movem.l d0-d7/a0-a6, -(a7)
pub fn i_0985ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x985CE;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[7]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $0985CE: move.b #$3, $10fdc4.l
pub fn i_0985ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x985D6;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10FDC4, v1 as u8);
    16
}
/// $0985D6: move.w #$201, $10fdce.l
pub fn i_0985d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x985DE;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x201;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10FDCE, v1 as u16);
    16
}
/// $0985DE: move.b #$0, $10fdd0.l
pub fn i_0985de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x985E6;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10FDD0, v1 as u8);
    16
}
/// $0985E6: lea.l $10e3a2.l, a0
pub fn i_0985e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x985EC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E3A2;
    20
}
/// $0985EC: lea.l $981e8(pc), a1
pub fn i_0985ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x985F0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x981E8;
    16
}
/// $0985F0: move.l (a1)+, (a0)+
pub fn i_0985f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x985F2;
    cpu.ir = 0x20D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $0985F2: move.l (a1)+, (a0)+
pub fn i_0985f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x985F4;
    cpu.ir = 0x20D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $0985F4: move.l (a1)+, (a0)+
pub fn i_0985f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x985F6;
    cpu.ir = 0x20D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $0985F6: move.l (a1)+, (a0)+
pub fn i_0985f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x985F8;
    cpu.ir = 0x20D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $0985F8: move.l (a1)+, (a0)+
pub fn i_0985f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x985FA;
    cpu.ir = 0x20D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $0985FA: move.l #$10e3a2, $10fdc8.l
pub fn i_0985fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98604;
    cpu.ir = 0x23FC;
    let v1: u32 = 0x10E3A2;
    bus.write32(0x10FDC8, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $098604: move.w #$1c, $10fdcc.l
pub fn i_098604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9860C;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x1C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10FDCC, v1 as u16);
    16
}
/// $09860C: jsr $c00468.l
pub fn i_09860c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98612;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98612);
    cpu.pc = 0xC00468;
    16
}
/// $098612: tst.b $10fdc6.l
pub fn i_098612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98618;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FDC6) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $098618: beq.w $98660
pub fn i_098618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9861C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x98660; return 10; }
    8
}
/// $09861C: movea.l #$7000, a1
pub fn i_09861c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98622;
    cpu.ir = 0x227C;
    let v1: u32 = 0x7000;
    cpu.a[1] = v1;
    4
}
/// $098622: move.w #$ff, d0
pub fn i_098622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98626;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $098626: move.w #$26, d1
pub fn i_098626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9862A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x26;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09862A: move.w #$1b, d2
pub fn i_09862a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9862E;
    cpu.ir = 0x343C;
    let v1: u32 = 0x1B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09862E: jsr $5da9c.l
pub fn i_09862e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98634;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98634);
    cpu.pc = 0x5DA9C;
    16
}
/// $098634: move.b $10fdc2.l, -(a7)
pub fn i_098634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9863A;
    cpu.ir = 0x1F39;
    let v2 = bus.read8(0x10FDC2) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad3;
    bus.write8(ad3, v2 as u8);
    22
}
/// $09863A: clr.b $10fdc2.l
pub fn i_09863a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98640;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10FDC2) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10FDC2, r2 as u8);
    16
}
/// $098640: jsr $c0046e.l
pub fn i_098640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98646;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98646);
    cpu.pc = 0xC0046E;
    16
}
/// $098646: move.b (a7)+, $10fdc2.l
pub fn i_098646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9864C;
    cpu.ir = 0x13DF;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10FDC2, v3 as u8);
    20
}
/// $09864C: tst.b $10fdc6.l
pub fn i_09864c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98652;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FDC6) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $098652: beq.w $98660
pub fn i_098652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98656;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x98660; return 10; }
    8
}
/// $098656: movem.l (a7)+, d0-d7/a0-a6
pub fn i_098656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9865A;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[7] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
// ===== SetXN_09865a  ($09865A, 6 B, thunks/ccr_helpers.c)
/// $09865A: ori.b #$11, ccr
pub fn i_09865a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9865E;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $09865E: rts
pub fn i_09865e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98660;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MemCard_BiosWrite_Ok_098660  ($098660, 4 B, frontend/hiscore_memcard_options_09773c.s)
/// $098660: movem.l (a7)+, d0-d7/a0-a6
pub fn i_098660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98664;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[7] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
// ===== ClearXN_098664  ($098664, 6 B, thunks/ccr_helpers.c)
/// $098664: andi.b #$ee, ccr
pub fn i_098664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98668;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $098668: rts
pub fn i_098668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9866A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MemCard_BiosCreate_09866a  ($09866A, 40 B, frontend/hiscore_memcard_options_09773c.s)
/// $09866A: movem.l d0-d7/a0-a6, -(a7)
pub fn i_09866a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9866E;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[7]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $09866E: move.b #$4, $10fdc4.l
pub fn i_09866e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98676;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10FDC4, v1 as u8);
    16
}
/// $098676: move.w #$201, $10fdce.l
pub fn i_098676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9867E;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x201;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10FDCE, v1 as u16);
    16
}
/// $09867E: move.b #$0, $10fdd0.l
pub fn i_09867e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98686;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10FDD0, v1 as u8);
    16
}
/// $098686: jsr $c00468.l
pub fn i_098686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9868C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9868C);
    cpu.pc = 0xC00468;
    16
}
/// $09868C: movem.l (a7)+, d0-d7/a0-a6
pub fn i_09868c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98690;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[7] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $098690: rts
pub fn i_098690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98692;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CmpPrioWithSibling_098704  ($098704, 16 B, frontend/hiscore_memcard_options_09773c.s)
/// $098704: movea.l $8(a6), a1
pub fn i_098704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98708;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $098708: move.b $10(a6), d0
pub fn i_098708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9870C;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09870C: cmp.b $10(a1), d0
pub fn i_09870c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98710;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $098710: bcs.w $9871a
pub fn i_098710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98714;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x9871A; return 10; }
    8
}
// ===== ClearXN_098714  ($098714, 6 B, thunks/ccr_helpers.c)
/// $098714: andi.b #$ee, ccr
pub fn i_098714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98718;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $098718: rts
pub fn i_098718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9871A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_09871a  ($09871A, 6 B, thunks/ccr_helpers.c)
/// $09871A: ori.b #$11, ccr
pub fn i_09871a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9871E;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $09871E: rts
pub fn i_09871e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98720;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetTaskHandler_098830  ($098830, 8 B, kernel/task_handlers.c)
/// $098830: lea.l $98836(pc), a1
pub fn i_098830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98834;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x98836;
    16
}
/// $098834: move.l a1, (a6)
pub fn i_098834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98836;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $098836: rts
pub fn i_098836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98838;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== LogoScene_Piece_Init_098838  ($098838, 70 B, frontend/hiscore_memcard_options_09773c.s)
/// $098838: move.w #$11d, d1
pub fn i_098838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9883C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x11D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09883C: jsr $236e.l
pub fn i_09883c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98842;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98842);
    cpu.pc = 0x236E;
    16
}
/// $098842: move.w $14(a6), d1
pub fn i_098842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98846;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $098846: jsr $2c66.l
pub fn i_098846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9884C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9884C);
    cpu.pc = 0x2C66;
    16
}
/// $09884C: lea.l $2f553c.l, a0
pub fn i_09884c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98852;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F553C;
    20
}
/// $098852: move.w $80(a6), d0
pub fn i_098852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98856;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $098856: add.w d0, d0
pub fn i_098856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98858;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $098858: add.w d0, d0
pub fn i_098858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9885A;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09885A: move.w (a0, d0.w), $22(a6)
pub fn i_09885a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98860;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    22
}
/// $098860: move.w #$180, $24(a6)
pub fn i_098860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98866;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $098866: move.w $2(a0, d0.w), $84(a6)
pub fn i_098866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9886C;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x84);
    bus.write16(ad4, v3 as u16);
    22
}
/// $09886C: move.w #$0, $82(a6)
pub fn i_09886c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98872;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $098872: lea.l $2f5576.l, a0
pub fn i_098872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98878;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F5576;
    20
}
/// $098878: jsr $28cd4.l
pub fn i_098878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9887E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9887E);
    cpu.pc = 0x28CD4;
    16
}
// ===== SetTaskHandler_09887e  ($09887E, 8 B, kernel/task_handlers.c)
/// $09887E: lea.l $98886(pc), a1
pub fn i_09887e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98882;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x98886;
    16
}
/// $098882: move.l a1, (a6)
pub fn i_098882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98884;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $098884: rts
pub fn i_098884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98886;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== LogoScene_Piece_Run_098886  ($098886, 44 B, frontend/hiscore_memcard_options_09773c.s)
/// $098886: tst.w $84(a6)
pub fn i_098886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9888A;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $09888A: beq.w $98896
pub fn i_09888a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9888E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x98896; return 10; }
    8
}
/// $09888E: subq.w #$1, $84(a6)
pub fn i_09888e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98892;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x84);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $098892: bne.w $9889c
pub fn i_098892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98896;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9889C; return 10; }
    8
}
/// $098896: jsr $28d70.l
pub fn i_098896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9889C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9889C);
    cpu.pc = 0x28D70;
    16
}
/// $09889C: addq.w #$1, $82(a6)
pub fn i_09889c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x988A0;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0988A0: cmpi.w #$c, $82(a6)
pub fn i_0988a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x988A6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xC, Size::Word, false);
    16
}
/// $0988A6: blt.w $988b0
pub fn i_0988a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x988AA;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x988B0; return 10; }
    8
}
/// $0988AA: jmp $518.l
pub fn i_0988aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x988B0;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0988B0: rts
pub fn i_0988b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x988B2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== LogoScene_Center_Init_0988b2  ($0988B2, 82 B, frontend/hiscore_memcard_options_09773c.s)
/// $0988B2: move.w #$11e, d1
pub fn i_0988b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x988B6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x11E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0988B6: jsr $236e.l
pub fn i_0988b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x988BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x988BC);
    cpu.pc = 0x236E;
    16
}
/// $0988BC: move.w $14(a6), d1
pub fn i_0988bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x988C0;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0988C0: jsr $2c66.l
pub fn i_0988c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x988C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x988C6);
    cpu.pc = 0x2C66;
    16
}
/// $0988C6: move.w #$a0, $22(a6)
pub fn i_0988c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x988CC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0988CC: move.w #$180, $24(a6)
pub fn i_0988cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x988D2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0988D2: moveq #$0, d0
pub fn i_0988d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x988D4;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0988D4: move.b d0, $20(a6)
pub fn i_0988d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x988D8;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0988D8: move.b d0, $21(a6)
pub fn i_0988d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x988DC;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0988DC: move.b #$ff, $32(a6)
pub fn i_0988dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x988E2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0988E2: move.b #$2a, $33(a6)
pub fn i_0988e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x988E8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2A;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0988E8: move.w d0, $86(a6)
pub fn i_0988e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x988EC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0988EC: move.w #$d, $84(a6)
pub fn i_0988ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x988F2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0988F2: move.w #$0, $82(a6)
pub fn i_0988f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x988F8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0988F8: lea.l $2f5582.l, a0
pub fn i_0988f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x988FE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F5582;
    20
}
/// $0988FE: jsr $28cd4.l
pub fn i_0988fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98904;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98904);
    cpu.pc = 0x28CD4;
    16
}
// ===== SetTaskHandler_098904  ($098904, 8 B, kernel/task_handlers.c)
/// $098904: lea.l $9890c(pc), a1
pub fn i_098904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98908;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9890C;
    16
}
/// $098908: move.l a1, (a6)
pub fn i_098908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9890A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09890A: rts
pub fn i_09890a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9890C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== LogoScene_Center_Run_09890c  ($09890C, 118 B, frontend/hiscore_memcard_options_09773c.s)
/// $09890C: tst.w $84(a6)
pub fn i_09890c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98910;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $098910: beq.w $9892e
pub fn i_098910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98914;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9892E; return 10; }
    8
}
/// $098914: subq.w #$1, $84(a6)
pub fn i_098914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98918;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x84);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $098918: bne.w $98988
pub fn i_098918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9891C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x98988; return 10; }
    8
}
/// $09891C: move.w $14(a6), d1
pub fn i_09891c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98920;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $098920: move.w #$11f, d2
pub fn i_098920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98924;
    cpu.ir = 0x343C;
    let v1: u32 = 0x11F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $098924: moveq #$1, d3
pub fn i_098924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98926;
    cpu.ir = 0x7601;
    cpu.d[3] = 0x1;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $098926: moveq #$1, d4
pub fn i_098926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98928;
    cpu.ir = 0x7801;
    cpu.d[4] = 0x1;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $098928: jsr $2c30.l
pub fn i_098928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9892E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9892E);
    cpu.pc = 0x2C30;
    16
}
/// $09892E: jsr $28d70.l
pub fn i_09892e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98934;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98934);
    cpu.pc = 0x28D70;
    16
}
/// $098934: cmpi.b #$ff, $33(a6)
pub fn i_098934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9893A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $09893A: bne.w $9898a
pub fn i_09893a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9893E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9898A; return 10; }
    8
}
/// $09893E: addq.w #$1, $82(a6)
pub fn i_09893e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98942;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $098942: cmpi.w #$10, $82(a6)
pub fn i_098942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98948;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x10, Size::Word, false);
    16
}
/// $098948: blt.w $98988
pub fn i_098948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9894C;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x98988; return 10; }
    8
}
/// $09894C: tst.b $20(a6)
pub fn i_09894c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98950;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $098950: bne.w $98988
pub fn i_098950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98954;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x98988; return 10; }
    8
}
/// $098954: move.b #$ff, $20(a6)
pub fn i_098954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9895A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09895A: move.w $14(a6), d1
pub fn i_09895a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9895E;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09895E: move.w #$120, d2
pub fn i_09895e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98962;
    cpu.ir = 0x343C;
    let v1: u32 = 0x120;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $098962: moveq #$1, d3
pub fn i_098962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98964;
    cpu.ir = 0x7601;
    cpu.d[3] = 0x1;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $098964: moveq #$1, d4
pub fn i_098964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98966;
    cpu.ir = 0x7801;
    cpu.d[4] = 0x1;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $098966: jsr $2c30.l
pub fn i_098966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9896C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9896C);
    cpu.pc = 0x2C30;
    16
}
/// $09896C: move.w $14(a6), d1
pub fn i_09896c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98970;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $098970: jsr $2c66.l
pub fn i_098970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98976;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98976);
    cpu.pc = 0x2C66;
    16
}
/// $098976: move.w #$ff, d1
pub fn i_098976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9897A;
    cpu.ir = 0x323C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09897A: move.w #$120, d2
pub fn i_09897a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9897E;
    cpu.ir = 0x343C;
    let v1: u32 = 0x120;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09897E: moveq #$1, d3
pub fn i_09897e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98980;
    cpu.ir = 0x7601;
    cpu.d[3] = 0x1;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $098980: moveq #$1, d4
pub fn i_098980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98982;
    cpu.ir = 0x7801;
    cpu.d[4] = 0x1;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
// ===== JsrAbsThunk_098982  ($098982, 8 B, thunks/jsr_abs_thunks.c)
/// $098982: jsr $2c30.l
pub fn i_098982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98988;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98988);
    cpu.pc = 0x2C30;
    16
}
/// $098988: rts
pub fn i_098988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9898A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== LogoScene_Center_PalStep_09898a  ($09898A, 22 B, frontend/hiscore_memcard_options_09773c.s)
/// $09898A: lea.l $2f5560.l, a0
pub fn i_09898a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98990;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F5560;
    20
}
/// $098990: move.w $86(a6), d0
pub fn i_098990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98994;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $098994: move.b (a0, d0.w), $33(a6)
pub fn i_098994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9899A;
    cpu.ir = 0x1D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad4, v3 as u8);
    22
}
/// $09899A: addq.w #$1, $86(a6)
pub fn i_09899a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9899E;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09899E: rts
pub fn i_09899e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x989A0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CmpPrioWithSibling_0989a0  ($0989A0, 16 B, frontend/hiscore_memcard_options_09773c.s)
/// $0989A0: movea.l $8(a6), a1
pub fn i_0989a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x989A4;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0989A4: move.b $10(a6), d0
pub fn i_0989a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x989A8;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0989A8: cmp.b $10(a1), d0
pub fn i_0989a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x989AC;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $0989AC: bcs.w $989b6
pub fn i_0989ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x989B0;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x989B6; return 10; }
    8
}
// ===== ClearXN_0989b0  ($0989B0, 6 B, thunks/ccr_helpers.c)
/// $0989B0: andi.b #$ee, ccr
pub fn i_0989b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x989B4;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0989B4: rts
pub fn i_0989b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x989B6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_0989b6  ($0989B6, 6 B, thunks/ccr_helpers.c)
/// $0989B6: ori.b #$11, ccr
pub fn i_0989b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x989BA;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0989BA: rts
pub fn i_0989ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x989BC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_Tmpl174_Init_0989bc  ($0989BC, 28 B, frontend/hiscore_memcard_options_09773c.s)
/// $0989BC: move.w #$125, d1
pub fn i_0989bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x989C0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x125;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0989C0: jsr $236e.l
pub fn i_0989c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x989C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x989C6);
    cpu.pc = 0x236E;
    16
}
/// $0989C6: move.w #$0, $80(a6)
pub fn i_0989c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x989CC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0989CC: jsr $8f002.l
pub fn i_0989cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x989D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x989D2);
    cpu.pc = 0x8F002;
    16
}
/// $0989D2: bclr.b #$1, $12(a6)
pub fn i_0989d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x989D8;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== SetTaskHandler_0989d8  ($0989D8, 8 B, kernel/task_handlers.c)
/// $0989D8: lea.l $989e0(pc), a1
pub fn i_0989d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x989DC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x989E0;
    16
}
/// $0989DC: move.l a1, (a6)
pub fn i_0989dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x989DE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0989DE: rts
pub fn i_0989de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x989E0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_Walk_0989e0  ($0989E0, 160 B, frontend/hiscore_memcard_options_09773c.s)
/// $0989E0: move.b #$0, $76(a6)
pub fn i_0989e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x989E6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0989E6: jsr $267e2.l
pub fn i_0989e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x989EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x989EC);
    cpu.pc = 0x267E2;
    16
}
/// $0989EC: jsr $5e7c0.l
pub fn i_0989ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x989F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x989F2);
    cpu.pc = 0x5E7C0;
    16
}
/// $0989F2: move.w #$8000, d0
pub fn i_0989f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x989F6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0989F6: jsr $28134.l
pub fn i_0989f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x989FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x989FC);
    cpu.pc = 0x28134;
    16
}
/// $0989FC: andi.w #$ffe3, $38(a6)
pub fn i_0989fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98A02;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $098A02: ori.w #$4, $38(a6)
pub fn i_098a02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98A08;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x4) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $098A08: move.w #$1, $66(a6)
pub fn i_098a08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98A0E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $098A0E: jsr $8f010.l
pub fn i_098a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98A14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98A14);
    cpu.pc = 0x8F010;
    16
}
/// $098A14: bclr.b #$3, $13(a6)
pub fn i_098a14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98A1A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $098A1A: lea.l $2f55be.l, a0
pub fn i_098a1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98A20;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F55BE;
    20
}
/// $098A20: jsr $28cd4.l
pub fn i_098a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98A26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98A26);
    cpu.pc = 0x28CD4;
    16
}
/// $098A26: lea.l $98a2c(pc), a1
pub fn i_098a26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98A2A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x98A2C;
    16
}
/// $098A2A: move.l a1, (a6)
pub fn i_098a2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98A2C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $098A2C: jsr $27a92.l
pub fn i_098a2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98A32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98A32);
    cpu.pc = 0x27A92;
    16
}
/// $098A32: jsr $27eba.l
pub fn i_098a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98A38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98A38);
    cpu.pc = 0x27EBA;
    16
}
/// $098A38: bcc.w $98a42
pub fn i_098a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98A3C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x98A42; return 10; }
    8
}
/// $098A3C: lea.l $98b66(pc), a1
pub fn i_098a3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98A40;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x98B66;
    16
}
/// $098A40: move.l a1, (a6)
pub fn i_098a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98A42;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $098A42: jsr $28d70.l
pub fn i_098a42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98A48;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98A48);
    cpu.pc = 0x28D70;
    16
}
/// $098A48: tst.b $76(a6)
pub fn i_098a48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98A4C;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $098A4C: beq.w $98a5a
pub fn i_098a4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98A50;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x98A5A; return 10; }
    8
}
/// $098A50: bclr.b #$3, $13(a6)
pub fn i_098a50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98A56;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $098A56: bra.w $98a6a
pub fn i_098a56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98A5A;
    cpu.ir = 0x6000;
    cpu.pc = 0x98A6A;
    10
}
/// $098A5A: jsr $2870a.l
pub fn i_098a5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98A60;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98A60);
    cpu.pc = 0x2870A;
    16
}
/// $098A60: bcc.w $98a6a
pub fn i_098a60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98A64;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x98A6A; return 10; }
    8
}
/// $098A64: lea.l $98a88(pc), a1
pub fn i_098a64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98A68;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x98A88;
    16
}
/// $098A68: move.l a1, (a6)
pub fn i_098a68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98A6A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $098A6A: movea.l #$ffffffff, a0
pub fn i_098a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98A70;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $098A70: lea.l $2f61ac.l, a0
pub fn i_098a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98A76;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F61AC;
    20
}
/// $098A76: jsr $5dd56.l
pub fn i_098a76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98A7C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98A7C);
    cpu.pc = 0x5DD56;
    16
}
/// $098A7C: bcc.w $98a86
pub fn i_098a7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98A80;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x98A86; return 10; }
    8
}
// ===== SetTaskHandler_098a80  ($098A80, 8 B, kernel/task_handlers.c)
/// $098A80: lea.l $99794(pc), a1
pub fn i_098a80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98A84;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99794;
    16
}
/// $098A84: move.l a1, (a6)
pub fn i_098a84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98A86;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $098A86: rts
pub fn i_098a86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98A88;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_Hit_098a88  ($098A88, 68 B, frontend/hiscore_memcard_options_09773c.s)
/// $098A88: move.b #$ff, $76(a6)
pub fn i_098a88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98A8E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write8(ad2, v1 as u8);
    12
}
/// $098A8E: lea.l $2f5642.l, a0
pub fn i_098a8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98A94;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F5642;
    20
}
/// $098A94: jsr $28cd4.l
pub fn i_098a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98A9A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98A9A);
    cpu.pc = 0x28CD4;
    16
}
/// $098A9A: lea.l $98aa0(pc), a1
pub fn i_098a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98A9E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x98AA0;
    16
}
/// $098A9E: move.l a1, (a6)
pub fn i_098a9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98AA0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $098AA0: jsr $2783a.l
pub fn i_098aa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98AA6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98AA6);
    cpu.pc = 0x2783A;
    16
}
/// $098AA6: jsr $28d70.l
pub fn i_098aa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98AAC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98AAC);
    cpu.pc = 0x28D70;
    16
}
/// $098AAC: bcc.w $98ab6
pub fn i_098aac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98AB0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x98AB6; return 10; }
    8
}
/// $098AB0: lea.l $98a08(pc), a1
pub fn i_098ab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98AB4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x98A08;
    16
}
/// $098AB4: move.l a1, (a6)
pub fn i_098ab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98AB6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $098AB6: movea.l #$ffffffff, a0
pub fn i_098ab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98ABC;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $098ABC: lea.l $2f61ac.l, a0
pub fn i_098abc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98AC2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F61AC;
    20
}
/// $098AC2: jsr $5dd56.l
pub fn i_098ac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98AC8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98AC8);
    cpu.pc = 0x5DD56;
    16
}
/// $098AC8: bcc.w $98ad2
pub fn i_098ac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98ACC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x98AD2; return 10; }
    8
}
// ===== SetTaskHandler_098acc  ($098ACC, 8 B, kernel/task_handlers.c)
/// $098ACC: lea.l $99794(pc), a1
pub fn i_098acc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98AD0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99794;
    16
}
/// $098AD0: move.l a1, (a6)
pub fn i_098ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98AD2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $098AD2: rts
pub fn i_098ad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98AD4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_Tmpl175_Init_098ad4  ($098AD4, 34 B, frontend/hiscore_memcard_options_09773c.s)
/// $098AD4: move.w #$125, d1
pub fn i_098ad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98AD8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x125;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $098AD8: jsr $236e.l
pub fn i_098ad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98ADE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98ADE);
    cpu.pc = 0x236E;
    16
}
/// $098ADE: jsr $267e2.l
pub fn i_098ade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98AE4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98AE4);
    cpu.pc = 0x267E2;
    16
}
/// $098AE4: jsr $8f002.l
pub fn i_098ae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98AEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98AEA);
    cpu.pc = 0x8F002;
    16
}
/// $098AEA: move.w #$0, $80(a6)
pub fn i_098aea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98AF0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $098AF0: bclr.b #$1, $12(a6)
pub fn i_098af0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98AF6;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== SetTaskHandler_098af6  ($098AF6, 8 B, kernel/task_handlers.c)
/// $098AF6: lea.l $98afe(pc), a1
pub fn i_098af6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98AFA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x98AFE;
    16
}
/// $098AFA: move.l a1, (a6)
pub fn i_098afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98AFC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $098AFC: rts
pub fn i_098afc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98AFE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_Tmpl175_Idle_098afe  ($098AFE, 96 B, frontend/hiscore_memcard_options_09773c.s)
/// $098AFE: jsr $5e7c0.l
pub fn i_098afe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98B04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98B04);
    cpu.pc = 0x5E7C0;
    16
}
/// $098B04: move.w #$8000, d0
pub fn i_098b04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98B08;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $098B08: jsr $28134.l
pub fn i_098b08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98B0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98B0E);
    cpu.pc = 0x28134;
    16
}
/// $098B0E: andi.w #$ffe3, $38(a6)
pub fn i_098b0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98B14;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $098B14: ori.w #$4, $38(a6)
pub fn i_098b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98B1A;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x4) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $098B1A: move.w #$1, $66(a6)
pub fn i_098b1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98B20;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $098B20: lea.l $2f5780.l, a0
pub fn i_098b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98B26;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F5780;
    20
}
/// $098B26: jsr $28cd4.l
pub fn i_098b26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98B2C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98B2C);
    cpu.pc = 0x28CD4;
    16
}
/// $098B2C: lea.l $98b32(pc), a1
pub fn i_098b2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98B30;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x98B32;
    16
}
/// $098B30: move.l a1, (a6)
pub fn i_098b30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98B32;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $098B32: jsr $2783a.l
pub fn i_098b32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98B38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98B38);
    cpu.pc = 0x2783A;
    16
}
/// $098B38: jsr $28d70.l
pub fn i_098b38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98B3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98B3E);
    cpu.pc = 0x28D70;
    16
}
/// $098B3E: bcc.w $98b48
pub fn i_098b3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98B42;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x98B48; return 10; }
    8
}
/// $098B42: lea.l $989e0(pc), a1
pub fn i_098b42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98B46;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x989E0;
    16
}
/// $098B46: move.l a1, (a6)
pub fn i_098b46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98B48;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $098B48: movea.l #$ffffffff, a0
pub fn i_098b48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98B4E;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $098B4E: lea.l $2f61ac.l, a0
pub fn i_098b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98B54;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F61AC;
    20
}
/// $098B54: jsr $5dd56.l
pub fn i_098b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98B5A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98B5A);
    cpu.pc = 0x5DD56;
    16
}
/// $098B5A: bcc.w $98b64
pub fn i_098b5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98B5E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x98B64; return 10; }
    8
}
// ===== SetTaskHandler_098b5e  ($098B5E, 8 B, kernel/task_handlers.c)
/// $098B5E: lea.l $99794(pc), a1
pub fn i_098b5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98B62;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99794;
    16
}
/// $098B62: move.l a1, (a6)
pub fn i_098b62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98B64;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $098B64: rts
pub fn i_098b64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98B66;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_Flee_098b66  ($098B66, 68 B, frontend/hiscore_memcard_options_09773c.s)
/// $098B66: jsr $267e2.l
pub fn i_098b66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98B6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98B6C);
    cpu.pc = 0x267E2;
    16
}
/// $098B6C: move.w #$ffe0, $2a(a6)
pub fn i_098b6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98B72;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $098B72: lea.l $98b78(pc), a1
pub fn i_098b72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98B76;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x98B78;
    16
}
/// $098B76: move.l a1, (a6)
pub fn i_098b76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98B78;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $098B78: jsr $27c8c.l
pub fn i_098b78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98B7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98B7E);
    cpu.pc = 0x27C8C;
    16
}
/// $098B7E: jsr $27eba.l
pub fn i_098b7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98B84;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98B84);
    cpu.pc = 0x27EBA;
    16
}
/// $098B84: bcs.w $98b8e
pub fn i_098b84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98B88;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x98B8E; return 10; }
    8
}
/// $098B88: lea.l $98bb2(pc), a1
pub fn i_098b88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98B8C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x98BB2;
    16
}
/// $098B8C: move.l a1, (a6)
pub fn i_098b8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98B8E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $098B8E: jsr $28d70.l
pub fn i_098b8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98B94;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98B94);
    cpu.pc = 0x28D70;
    16
}
/// $098B94: movea.l #$ffffffff, a0
pub fn i_098b94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98B9A;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $098B9A: lea.l $2f61ac.l, a0
pub fn i_098b9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98BA0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F61AC;
    20
}
/// $098BA0: jsr $5dd56.l
pub fn i_098ba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98BA6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98BA6);
    cpu.pc = 0x5DD56;
    16
}
/// $098BA6: bcc.w $98bb0
pub fn i_098ba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98BAA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x98BB0; return 10; }
    8
}
// ===== SetTaskHandler_098baa  ($098BAA, 8 B, kernel/task_handlers.c)
/// $098BAA: lea.l $99794(pc), a1
pub fn i_098baa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98BAE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99794;
    16
}
/// $098BAE: move.l a1, (a6)
pub fn i_098bae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98BB0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $098BB0: rts
pub fn i_098bb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98BB2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_DispatchBy80_098bb2  ($098BB2, 20 B, frontend/hiscore_memcard_options_09773c.s)
/// $098BB2: move.w $80(a6), d0
pub fn i_098bb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98BB6;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $098BB6: lea.l $2f55a6.l, a0
pub fn i_098bb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98BBC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F55A6;
    20
}
/// $098BBC: add.w d0, d0
pub fn i_098bbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98BBE;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $098BBE: add.w d0, d0
pub fn i_098bbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98BC0;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $098BC0: movea.l (a0, d0.w), a0
pub fn i_098bc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98BC4;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $098BC4: jmp (a0)
pub fn i_098bc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98BC6;
    cpu.ir = 0x4ED0;
    cpu.jump_to(cpu.a[0]);
    8
}
// ===== Mob_Tmpl176_Init_098bc6  ($098BC6, 50 B, frontend/hiscore_memcard_options_09773c.s)
/// $098BC6: move.w #$0, $70(a6)
pub fn i_098bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98BCC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $098BCC: bra.w $98bdc
pub fn i_098bcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98BD0;
    cpu.ir = 0x6000;
    cpu.pc = 0x98BDC;
    10
}
/// $098BDC: move.w #$124, d1
pub fn i_098bdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98BE0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x124;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $098BE0: jsr $236e.l
pub fn i_098be0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98BE6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98BE6);
    cpu.pc = 0x236E;
    16
}
/// $098BE6: move.w #$1, $80(a6)
pub fn i_098be6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98BEC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $098BEC: move.b #$0, $76(a6)
pub fn i_098bec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98BF2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write8(ad2, v1 as u8);
    12
}
/// $098BF2: bclr.b #$1, $12(a6)
pub fn i_098bf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98BF8;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== SetTaskHandler_098bf8  ($098BF8, 8 B, kernel/task_handlers.c)
/// $098BF8: lea.l $98c00(pc), a1
pub fn i_098bf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98BFC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x98C00;
    16
}
/// $098BFC: move.l a1, (a6)
pub fn i_098bfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98BFE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $098BFE: rts
pub fn i_098bfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98C00;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_Tmpl176_Body_098c00  ($098C00, 282 B, frontend/hiscore_memcard_options_09773c.s)
/// $098C00: jsr $267e2.l
pub fn i_098c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98C06;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98C06);
    cpu.pc = 0x267E2;
    16
}
/// $098C06: jsr $5e7c0.l
pub fn i_098c06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98C0C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98C0C);
    cpu.pc = 0x5E7C0;
    16
}
/// $098C0C: jsr $8f002.l
pub fn i_098c0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98C12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98C12);
    cpu.pc = 0x8F002;
    16
}
/// $098C12: jsr $8f010.l
pub fn i_098c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98C18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98C18);
    cpu.pc = 0x8F010;
    16
}
/// $098C18: move.w #$8000, d0
pub fn i_098c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98C1C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $098C1C: jsr $28134.l
pub fn i_098c1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98C22;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98C22);
    cpu.pc = 0x28134;
    16
}
/// $098C22: andi.w #$ffe3, $38(a6)
pub fn i_098c22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98C28;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $098C28: ori.w #$4, $38(a6)
pub fn i_098c28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98C2E;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x4) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $098C2E: bra.w $98c5a
pub fn i_098c2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98C32;
    cpu.ir = 0x6000;
    cpu.pc = 0x98C5A;
    10
}
/// $098C5A: move.w #$1, $66(a6)
pub fn i_098c5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98C60;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $098C60: move.w $70(a6), d0
pub fn i_098c60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98C64;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $098C64: movea.l #$2f558e, a0
pub fn i_098c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98C6A;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2F558E;
    cpu.a[0] = v1;
    4
}
/// $098C6A: lsl.w #$2, d0
pub fn i_098c6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98C6C;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $098C6C: movea.l (a0, d0.w), a0
pub fn i_098c6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98C70;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $098C70: cmpa.l #$ffffffff, a0
pub fn i_098c70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98C76;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $098C76: beq.w $98c80
pub fn i_098c76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98C7A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x98C80; return 10; }
    8
}
/// $098C7A: jsr $28cd4.l
pub fn i_098c7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98C80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98C80);
    cpu.pc = 0x28CD4;
    16
}
/// $098C80: bclr.b #$3, $13(a6)
pub fn i_098c80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98C86;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $098C86: lea.l $98c8c(pc), a1
pub fn i_098c86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98C8A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x98C8C;
    16
}
/// $098C8A: move.l a1, (a6)
pub fn i_098c8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98C8C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $098C8C: jsr $27a92.l
pub fn i_098c8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98C92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98C92);
    cpu.pc = 0x27A92;
    16
}
/// $098C92: btst.b #$5, $5a(a6)
pub fn i_098c92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98C98;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $098C98: beq.w $98ca2
pub fn i_098c98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98C9C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x98CA2; return 10; }
    8
}
/// $098C9C: lea.l $98d22(pc), a1
pub fn i_098c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98CA0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x98D22;
    16
}
/// $098CA0: move.l a1, (a6)
pub fn i_098ca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98CA2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $098CA2: jsr $27eba.l
pub fn i_098ca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98CA8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98CA8);
    cpu.pc = 0x27EBA;
    16
}
/// $098CA8: bcc.w $98cb2
pub fn i_098ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98CAC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x98CB2; return 10; }
    8
}
/// $098CAC: lea.l $98b66(pc), a1
pub fn i_098cac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98CB0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x98B66;
    16
}
/// $098CB0: move.l a1, (a6)
pub fn i_098cb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98CB2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $098CB2: jsr $28d70.l
pub fn i_098cb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98CB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98CB8);
    cpu.pc = 0x28D70;
    16
}
/// $098CB8: tst.b $76(a6)
pub fn i_098cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98CBC;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $098CBC: beq.w $98cca
pub fn i_098cbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98CC0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x98CCA; return 10; }
    8
}
/// $098CC0: bclr.b #$3, $13(a6)
pub fn i_098cc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98CC6;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $098CC6: bra.w $98d04
pub fn i_098cc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98CCA;
    cpu.ir = 0x6000;
    cpu.pc = 0x98D04;
    10
}
/// $098CCA: jsr $2870a.l
pub fn i_098cca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98CD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98CD0);
    cpu.pc = 0x2870A;
    16
}
/// $098CD0: bcc.w $98d04
pub fn i_098cd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98CD4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x98D04; return 10; }
    8
}
/// $098CD4: tst.w $70(a6)
pub fn i_098cd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98CD8;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $098CD8: beq.w $98cfe
pub fn i_098cd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98CDC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x98CFE; return 10; }
    8
}
/// $098CDC: subq.w #$1, d0
pub fn i_098cdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98CDE;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $098CDE: move.b $3a(a6), d1
pub fn i_098cde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98CE2;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $098CE2: andi.w #$1, d1
pub fn i_098ce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98CE6;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $098CE6: cmp.w d1, d0
pub fn i_098ce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98CE8;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $098CE8: bne.w $98cfe
pub fn i_098ce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98CEC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x98CFE; return 10; }
    8
}
/// $098CEC: subq.w #$1, $72(a6)
pub fn i_098cec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98CF0;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $098CF0: beq.w $98cfe
pub fn i_098cf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98CF4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x98CFE; return 10; }
    8
}
/// $098CF4: lea.l $98e4e(pc), a1
pub fn i_098cf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98CF8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x98E4E;
    16
}
/// $098CF8: move.l a1, (a6)
pub fn i_098cf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98CFA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $098CFA: bra.w $98d04
pub fn i_098cfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98CFE;
    cpu.ir = 0x6000;
    cpu.pc = 0x98D04;
    10
}
/// $098CFE: lea.l $98d22(pc), a1
pub fn i_098cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98D02;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x98D22;
    16
}
/// $098D02: move.l a1, (a6)
pub fn i_098d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98D04;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $098D04: movea.l #$ffffffff, a0
pub fn i_098d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98D0A;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $098D0A: lea.l $2f61ac.l, a0
pub fn i_098d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98D10;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F61AC;
    20
}
/// $098D10: jsr $5dd56.l
pub fn i_098d10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98D16;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98D16);
    cpu.pc = 0x5DD56;
    16
}
/// $098D16: bcc.w $98d20
pub fn i_098d16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98D1A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x98D20; return 10; }
    8
}
// ===== SetTaskHandler_098d1a  ($098D1A, 8 B, kernel/task_handlers.c)
/// $098D1A: lea.l $99794(pc), a1
pub fn i_098d1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98D1E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99794;
    16
}
/// $098D1E: move.l a1, (a6)
pub fn i_098d1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98D20;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $098D20: rts
pub fn i_098d20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98D22;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_Hit2_098d22  ($098D22, 98 B, frontend/hiscore_memcard_options_09773c.s)
/// $098D22: move.b #$ff, $76(a6)
pub fn i_098d22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98D28;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write8(ad2, v1 as u8);
    12
}
/// $098D28: tst.w $70(a6)
pub fn i_098d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98D2C;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $098D2C: beq.w $98d46
pub fn i_098d2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98D30;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x98D46; return 10; }
    8
}
/// $098D30: lea.l $98e9a(pc), a1
pub fn i_098d30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98D34;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x98E9A;
    16
}
/// $098D34: jsr $4ae.l
pub fn i_098d34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98D3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98D3A);
    cpu.pc = 0x4AE;
    16
}
/// $098D3A: jsr $5dd02.l
pub fn i_098d3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98D40;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98D40);
    cpu.pc = 0x5DD02;
    16
}
/// $098D40: move.w #$0, $70(a6)
pub fn i_098d40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98D46;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $098D46: lea.l $2f5b50.l, a0
pub fn i_098d46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98D4C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F5B50;
    20
}
/// $098D4C: jsr $28cd4.l
pub fn i_098d4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98D52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98D52);
    cpu.pc = 0x28CD4;
    16
}
/// $098D52: lea.l $98d58(pc), a1
pub fn i_098d52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98D56;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x98D58;
    16
}
/// $098D56: move.l a1, (a6)
pub fn i_098d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98D58;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $098D58: jsr $2783a.l
pub fn i_098d58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98D5E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98D5E);
    cpu.pc = 0x2783A;
    16
}
/// $098D5E: jsr $28d70.l
pub fn i_098d5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98D64;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98D64);
    cpu.pc = 0x28D70;
    16
}
/// $098D64: bcc.w $98d6e
pub fn i_098d64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98D68;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x98D6E; return 10; }
    8
}
/// $098D68: lea.l $98c3c(pc), a1
pub fn i_098d68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98D6C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x98C3C;
    16
}
/// $098D6C: move.l a1, (a6)
pub fn i_098d6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98D6E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $098D6E: movea.l #$ffffffff, a0
pub fn i_098d6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98D74;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $098D74: lea.l $2f61ac.l, a0
pub fn i_098d74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98D7A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F61AC;
    20
}
/// $098D7A: jsr $5dd56.l
pub fn i_098d7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98D80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98D80);
    cpu.pc = 0x5DD56;
    16
}
/// $098D80: bcc.w $98d8a
pub fn i_098d80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98D84;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x98D8A; return 10; }
    8
}
// ===== SetTaskHandler_098d84  ($098D84, 8 B, kernel/task_handlers.c)
/// $098D84: lea.l $99794(pc), a1
pub fn i_098d84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98D88;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99794;
    16
}
/// $098D88: move.l a1, (a6)
pub fn i_098d88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98D8A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $098D8A: rts
pub fn i_098d8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98D8C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_Tmpl178_Init_098d8c  ($098D8C, 50 B, frontend/hiscore_memcard_options_09773c.s)
/// $098D8C: move.w #$0, $70(a6)
pub fn i_098d8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98D92;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $098D92: bra.w $98da2
pub fn i_098d92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98D96;
    cpu.ir = 0x6000;
    cpu.pc = 0x98DA2;
    10
}
/// $098DA2: move.w #$124, d1
pub fn i_098da2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98DA6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x124;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $098DA6: jsr $236e.l
pub fn i_098da6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98DAC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98DAC);
    cpu.pc = 0x236E;
    16
}
/// $098DAC: move.w #$1, $80(a6)
pub fn i_098dac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98DB2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $098DB2: move.b #$0, $76(a6)
pub fn i_098db2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98DB8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write8(ad2, v1 as u8);
    12
}
/// $098DB8: bclr.b #$1, $12(a6)
pub fn i_098db8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98DBE;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== SetTaskHandler_098dbe  ($098DBE, 8 B, kernel/task_handlers.c)
/// $098DBE: lea.l $98dc6(pc), a1
pub fn i_098dbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98DC2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x98DC6;
    16
}
/// $098DC2: move.l a1, (a6)
pub fn i_098dc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98DC4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $098DC4: rts
pub fn i_098dc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98DC6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_Tmpl178_Body_098dc6  ($098DC6, 128 B, frontend/hiscore_memcard_options_09773c.s)
/// $098DC6: jsr $267e2.l
pub fn i_098dc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98DCC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98DCC);
    cpu.pc = 0x267E2;
    16
}
/// $098DCC: jsr $5e7c0.l
pub fn i_098dcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98DD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98DD2);
    cpu.pc = 0x5E7C0;
    16
}
/// $098DD2: jsr $8f002.l
pub fn i_098dd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98DD8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98DD8);
    cpu.pc = 0x8F002;
    16
}
/// $098DD8: jsr $8f010.l
pub fn i_098dd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98DDE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98DDE);
    cpu.pc = 0x8F010;
    16
}
/// $098DDE: move.w #$8000, d0
pub fn i_098dde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98DE2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $098DE2: jsr $28134.l
pub fn i_098de2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98DE8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98DE8);
    cpu.pc = 0x28134;
    16
}
/// $098DE8: andi.w #$ffe3, $38(a6)
pub fn i_098de8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98DEE;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $098DEE: ori.w #$4, $38(a6)
pub fn i_098dee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98DF4;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x4) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $098DF4: move.w $70(a6), d0
pub fn i_098df4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98DF8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $098DF8: movea.l #$2f5596, a0
pub fn i_098df8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98DFE;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2F5596;
    cpu.a[0] = v1;
    4
}
/// $098DFE: lsl.w #$2, d0
pub fn i_098dfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98E00;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $098E00: movea.l (a0, d0.w), a0
pub fn i_098e00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98E04;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $098E04: cmpa.l #$ffffffff, a0
pub fn i_098e04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98E0A;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $098E0A: beq.w $98e14
pub fn i_098e0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98E0E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x98E14; return 10; }
    8
}
/// $098E0E: jsr $28cd4.l
pub fn i_098e0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98E14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98E14);
    cpu.pc = 0x28CD4;
    16
}
/// $098E14: lea.l $98e1a(pc), a1
pub fn i_098e14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98E18;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x98E1A;
    16
}
/// $098E18: move.l a1, (a6)
pub fn i_098e18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98E1A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $098E1A: jsr $2783a.l
pub fn i_098e1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98E20;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98E20);
    cpu.pc = 0x2783A;
    16
}
/// $098E20: jsr $28d70.l
pub fn i_098e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98E26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98E26);
    cpu.pc = 0x28D70;
    16
}
/// $098E26: bcc.w $98e30
pub fn i_098e26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98E2A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x98E30; return 10; }
    8
}
/// $098E2A: lea.l $98c5a(pc), a1
pub fn i_098e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98E2E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x98C5A;
    16
}
/// $098E2E: move.l a1, (a6)
pub fn i_098e2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98E30;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $098E30: movea.l #$ffffffff, a0
pub fn i_098e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98E36;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $098E36: lea.l $2f61ac.l, a0
pub fn i_098e36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98E3C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F61AC;
    20
}
/// $098E3C: jsr $5dd56.l
pub fn i_098e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98E42;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98E42);
    cpu.pc = 0x5DD56;
    16
}
/// $098E42: bcc.w $98e4c
pub fn i_098e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98E46;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x98E4C; return 10; }
    8
}
// ===== SetTaskHandler_098e46  ($098E46, 8 B, kernel/task_handlers.c)
/// $098E46: lea.l $99794(pc), a1
pub fn i_098e46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98E4A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99794;
    16
}
/// $098E4A: move.l a1, (a6)
pub fn i_098e4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98E4C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $098E4C: rts
pub fn i_098e4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98E4E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_Pause_098e4e  ($098E4E, 68 B, frontend/hiscore_memcard_options_09773c.s)
/// $098E4E: lea.l $2f5c80.l, a0
pub fn i_098e4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98E54;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F5C80;
    20
}
/// $098E54: jsr $28cd4.l
pub fn i_098e54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98E5A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98E5A);
    cpu.pc = 0x28CD4;
    16
}
/// $098E5A: lea.l $98e60(pc), a1
pub fn i_098e5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98E5E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x98E60;
    16
}
/// $098E5E: move.l a1, (a6)
pub fn i_098e5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98E60;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $098E60: jsr $2783a.l
pub fn i_098e60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98E66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98E66);
    cpu.pc = 0x2783A;
    16
}
/// $098E66: bclr.b #$3, $13(a6)
pub fn i_098e66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98E6C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $098E6C: jsr $28d70.l
pub fn i_098e6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98E72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98E72);
    cpu.pc = 0x28D70;
    16
}
/// $098E72: bcc.w $98e7c
pub fn i_098e72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98E76;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x98E7C; return 10; }
    8
}
/// $098E76: lea.l $98c5a(pc), a1
pub fn i_098e76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98E7A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x98C5A;
    16
}
/// $098E7A: move.l a1, (a6)
pub fn i_098e7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98E7C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $098E7C: movea.l #$ffffffff, a0
pub fn i_098e7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98E82;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $098E82: lea.l $2f61ac.l, a0
pub fn i_098e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98E88;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F61AC;
    20
}
/// $098E88: jsr $5dd56.l
pub fn i_098e88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98E8E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98E8E);
    cpu.pc = 0x5DD56;
    16
}
/// $098E8E: bcc.w $98e98
pub fn i_098e8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98E92;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x98E98; return 10; }
    8
}
// ===== SetTaskHandler_098e92  ($098E92, 8 B, kernel/task_handlers.c)
/// $098E92: lea.l $99794(pc), a1
pub fn i_098e92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98E96;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99794;
    16
}
/// $098E96: move.l a1, (a6)
pub fn i_098e96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98E98;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $098E98: rts
pub fn i_098e98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98E9A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_Drop_098e9a  ($098E9A, 158 B, frontend/hiscore_memcard_options_09773c.s)
/// $098E9A: move.w #$124, d1
pub fn i_098e9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98E9E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x124;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $098E9E: jsr $236e.l
pub fn i_098e9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98EA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98EA4);
    cpu.pc = 0x236E;
    16
}
/// $098EA4: move.w #$8000, d0
pub fn i_098ea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98EA8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $098EA8: jsr $28134.l
pub fn i_098ea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98EAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98EAE);
    cpu.pc = 0x28134;
    16
}
/// $098EAE: andi.w #$ffe3, $38(a6)
pub fn i_098eae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98EB4;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $098EB4: ori.w #$4, $38(a6)
pub fn i_098eb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98EBA;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x4) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $098EBA: jsr $267e2.l
pub fn i_098eba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98EC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98EC0);
    cpu.pc = 0x267E2;
    16
}
/// $098EC0: move.w #$355, d0
pub fn i_098ec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98EC4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x355;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $098EC4: jsr $5dca4.l
pub fn i_098ec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98ECA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98ECA);
    cpu.pc = 0x5DCA4;
    16
}
/// $098ECA: move.w d0, $28(a6)
pub fn i_098eca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98ECE;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $098ECE: move.w #$690, $2a(a6)
pub fn i_098ece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98ED4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x690;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $098ED4: move.w #$ffc8, $2e(a6)
pub fn i_098ed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98EDA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFC8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $098EDA: move.w #$0, $2c(a6)
pub fn i_098eda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98EE0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $098EE0: jsr $5e9b6.l
pub fn i_098ee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98EE6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98EE6);
    cpu.pc = 0x5E9B6;
    16
}
/// $098EE6: move.w #$7f, d0
pub fn i_098ee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98EEA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $098EEA: subi.w #$3f, d0
pub fn i_098eea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98EEE;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x3F, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $098EEE: add.w $28(a6), d0
pub fn i_098eee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98EF2;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $098EF2: move.w d0, $28(a6)
pub fn i_098ef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98EF6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $098EF6: btst.b #$0, $3a(a6)
pub fn i_098ef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98EFC;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $098EFC: beq.w $98f04
pub fn i_098efc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98F00;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x98F04; return 10; }
    8
}
/// $098F00: neg.w $28(a6)
pub fn i_098f00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98F04;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $098F04: lea.l $2f5c9c.l, a0
pub fn i_098f04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98F0A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F5C9C;
    20
}
/// $098F0A: jsr $28cd4.l
pub fn i_098f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98F10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98F10);
    cpu.pc = 0x28CD4;
    16
}
/// $098F10: lea.l $98f16(pc), a1
pub fn i_098f10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98F14;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x98F16;
    16
}
/// $098F14: move.l a1, (a6)
pub fn i_098f14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98F16;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $098F16: jsr $27d50.l
pub fn i_098f16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98F1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98F1C);
    cpu.pc = 0x27D50;
    16
}
/// $098F1C: jsr $28d70.l
pub fn i_098f1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98F22;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98F22);
    cpu.pc = 0x28D70;
    16
}
/// $098F22: movea.l #$ffffffff, a0
pub fn i_098f22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98F28;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $098F28: lea.l $2f61ac.l, a0
pub fn i_098f28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98F2E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F61AC;
    20
}
/// $098F2E: jsr $5dd56.l
pub fn i_098f2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98F34;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98F34);
    cpu.pc = 0x5DD56;
    16
}
/// $098F34: bcc.w $98f3e
pub fn i_098f34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98F38;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x98F3E; return 10; }
    8
}
// ===== SetTaskHandler_098f38  ($098F38, 8 B, kernel/task_handlers.c)
/// $098F38: lea.l $99794(pc), a1
pub fn i_098f38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98F3C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99794;
    16
}
/// $098F3C: move.l a1, (a6)
pub fn i_098f3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98F3E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $098F3E: rts
pub fn i_098f3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98F40;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_Tmpl195_Init_098f40  ($098F40, 90 B, frontend/hiscore_memcard_options_09773c.s)
/// $098F40: move.w #$126, d1
pub fn i_098f40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98F44;
    cpu.ir = 0x323C;
    let v1: u32 = 0x126;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $098F44: jsr $236e.l
pub fn i_098f44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98F4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98F4A);
    cpu.pc = 0x236E;
    16
}
/// $098F4A: bra.w $98f66
pub fn i_098f4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98F4E;
    cpu.ir = 0x6000;
    cpu.pc = 0x98F66;
    10
}
/// $098F66: jsr $267e2.l
pub fn i_098f66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98F6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98F6C);
    cpu.pc = 0x267E2;
    16
}
/// $098F6C: jsr $5e7c0.l
pub fn i_098f6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98F72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98F72);
    cpu.pc = 0x5E7C0;
    16
}
/// $098F72: jsr $8f002.l
pub fn i_098f72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98F78;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98F78);
    cpu.pc = 0x8F002;
    16
}
/// $098F78: move.w #$1, $70(a6)
pub fn i_098f78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98F7E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $098F7E: move.w #$8000, d0
pub fn i_098f7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98F82;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $098F82: jsr $28134.l
pub fn i_098f82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98F88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98F88);
    cpu.pc = 0x28134;
    16
}
/// $098F88: andi.w #$ffe3, $38(a6)
pub fn i_098f88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98F8E;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $098F8E: ori.w #$4, $38(a6)
pub fn i_098f8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98F94;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x4) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x096992, i_096992),
    (0x096998, i_096998),
    (0x0969A0, i_0969a0),
    (0x0969A4, i_0969a4),
    (0x0969A6, i_0969a6),
    (0x0969AA, i_0969aa),
    (0x0969B0, i_0969b0),
    (0x0969B4, i_0969b4),
    (0x0969B8, i_0969b8),
    (0x0969C0, i_0969c0),
    (0x0969C2, i_0969c2),
    (0x0969C6, i_0969c6),
    (0x0969CA, i_0969ca),
    (0x0969CC, i_0969cc),
    (0x0969D0, i_0969d0),
    (0x0969D6, i_0969d6),
    (0x0969DA, i_0969da),
    (0x0969DE, i_0969de),
    (0x0969E4, i_0969e4),
    (0x0969E6, i_0969e6),
    (0x0969EA, i_0969ea),
    (0x0969EC, i_0969ec),
    (0x0969F0, i_0969f0),
    (0x0969F4, i_0969f4),
    (0x0969F6, i_0969f6),
    (0x0969FC, i_0969fc),
    (0x0969FE, i_0969fe),
    (0x096A04, i_096a04),
    (0x096A0A, i_096a0a),
    (0x096A0C, i_096a0c),
    (0x096A0E, i_096a0e),
    (0x096A12, i_096a12),
    (0x096A16, i_096a16),
    (0x096A18, i_096a18),
    (0x096A1C, i_096a1c),
    (0x096A22, i_096a22),
    (0x096A26, i_096a26),
    (0x096A2A, i_096a2a),
    (0x096A30, i_096a30),
    (0x096A32, i_096a32),
    (0x096A36, i_096a36),
    (0x096A38, i_096a38),
    (0x096A3C, i_096a3c),
    (0x096A40, i_096a40),
    (0x096A42, i_096a42),
    (0x096A48, i_096a48),
    (0x096A4A, i_096a4a),
    (0x096A50, i_096a50),
    (0x096A56, i_096a56),
    (0x096A58, i_096a58),
    (0x096A5A, i_096a5a),
    (0x096A60, i_096a60),
    (0x096A62, i_096a62),
    (0x096A64, i_096a64),
    (0x096A66, i_096a66),
    (0x096A68, i_096a68),
    (0x096A6E, i_096a6e),
    (0x096A70, i_096a70),
    (0x096A72, i_096a72),
    (0x096A74, i_096a74),
    (0x096A78, i_096a78),
    (0x096A7A, i_096a7a),
    (0x096A7C, i_096a7c),
    (0x096A7E, i_096a7e),
    (0x096A80, i_096a80),
    (0x096A88, i_096a88),
    (0x096A8C, i_096a8c),
    (0x096A92, i_096a92),
    (0x096A98, i_096a98),
    (0x096A9E, i_096a9e),
    (0x096AA2, i_096aa2),
    (0x096AA8, i_096aa8),
    (0x096AAE, i_096aae),
    (0x096AB2, i_096ab2),
    (0x096AB8, i_096ab8),
    (0x096ABA, i_096aba),
    (0x096AC0, i_096ac0),
    (0x096AC4, i_096ac4),
    (0x096ACA, i_096aca),
    (0x096ACC, i_096acc),
    (0x096AD2, i_096ad2),
    (0x096AD6, i_096ad6),
    (0x096ADC, i_096adc),
    (0x096ADE, i_096ade),
    (0x096AE4, i_096ae4),
    (0x096AEA, i_096aea),
    (0x096AF0, i_096af0),
    (0x096AF4, i_096af4),
    (0x096AFA, i_096afa),
    (0x096AFC, i_096afc),
    (0x096B02, i_096b02),
    (0x096B08, i_096b08),
    (0x096B0E, i_096b0e),
    (0x096B12, i_096b12),
    (0x096B18, i_096b18),
    (0x096B1C, i_096b1c),
    (0x096B22, i_096b22),
    (0x096B24, i_096b24),
    (0x096B2C, i_096b2c),
    (0x096B30, i_096b30),
    (0x096B36, i_096b36),
    (0x096B3A, i_096b3a),
    (0x096B40, i_096b40),
    (0x096B44, i_096b44),
    (0x096B4A, i_096b4a),
    (0x096B50, i_096b50),
    (0x096B58, i_096b58),
    (0x096B5C, i_096b5c),
    (0x096B62, i_096b62),
    (0x096B66, i_096b66),
    (0x096B6C, i_096b6c),
    (0x096B70, i_096b70),
    (0x096B76, i_096b76),
    (0x096B7C, i_096b7c),
    (0x096B7E, i_096b7e),
    (0x096B82, i_096b82),
    (0x096B86, i_096b86),
    (0x096B8A, i_096b8a),
    (0x096B90, i_096b90),
    (0x096B92, i_096b92),
    (0x096B96, i_096b96),
    (0x096B9A, i_096b9a),
    (0x097720, i_097720),
    (0x097724, i_097724),
    (0x097728, i_097728),
    (0x09772C, i_09772c),
    (0x097730, i_097730),
    (0x097734, i_097734),
    (0x097736, i_097736),
    (0x09773A, i_09773a),
    (0x09773C, i_09773c),
    (0x097742, i_097742),
    (0x097748, i_097748),
    (0x09774A, i_09774a),
    (0x09774C, i_09774c),
    (0x09774E, i_09774e),
    (0x097750, i_097750),
    (0x097752, i_097752),
    (0x097754, i_097754),
    (0x097756, i_097756),
    (0x097758, i_097758),
    (0x09775A, i_09775a),
    (0x09775C, i_09775c),
    (0x09775E, i_09775e),
    (0x097760, i_097760),
    (0x097762, i_097762),
    (0x097764, i_097764),
    (0x097766, i_097766),
    (0x097768, i_097768),
    (0x09776A, i_09776a),
    (0x09776C, i_09776c),
    (0x09776E, i_09776e),
    (0x097770, i_097770),
    (0x097772, i_097772),
    (0x097774, i_097774),
    (0x097776, i_097776),
    (0x097778, i_097778),
    (0x09777A, i_09777a),
    (0x09777C, i_09777c),
    (0x09777E, i_09777e),
    (0x097780, i_097780),
    (0x097782, i_097782),
    (0x097784, i_097784),
    (0x097786, i_097786),
    (0x097788, i_097788),
    (0x09778A, i_09778a),
    (0x09778C, i_09778c),
    (0x09778E, i_09778e),
    (0x097790, i_097790),
    (0x097792, i_097792),
    (0x097794, i_097794),
    (0x097796, i_097796),
    (0x097798, i_097798),
    (0x09779A, i_09779a),
    (0x09779C, i_09779c),
    (0x09779E, i_09779e),
    (0x0977A0, i_0977a0),
    (0x0977A2, i_0977a2),
    (0x0977A4, i_0977a4),
    (0x0977A6, i_0977a6),
    (0x0977A8, i_0977a8),
    (0x0977AA, i_0977aa),
    (0x0977AC, i_0977ac),
    (0x0977AE, i_0977ae),
    (0x0977B0, i_0977b0),
    (0x0977B2, i_0977b2),
    (0x0977B4, i_0977b4),
    (0x0977B6, i_0977b6),
    (0x0977B8, i_0977b8),
    (0x0977BA, i_0977ba),
    (0x0977BC, i_0977bc),
    (0x0977BE, i_0977be),
    (0x0977C0, i_0977c0),
    (0x0977D6, i_0977d6),
    (0x0977DA, i_0977da),
    (0x0977E0, i_0977e0),
    (0x0977E6, i_0977e6),
    (0x0977EA, i_0977ea),
    (0x0977EE, i_0977ee),
    (0x0977F4, i_0977f4),
    (0x0977FA, i_0977fa),
    (0x097800, i_097800),
    (0x097802, i_097802),
    (0x097804, i_097804),
    (0x097806, i_097806),
    (0x097808, i_097808),
    (0x09780A, i_09780a),
    (0x09780C, i_09780c),
    (0x09780E, i_09780e),
    (0x097810, i_097810),
    (0x097812, i_097812),
    (0x097814, i_097814),
    (0x097816, i_097816),
    (0x09781A, i_09781a),
    (0x097820, i_097820),
    (0x097822, i_097822),
    (0x097824, i_097824),
    (0x09782A, i_09782a),
    (0x09782E, i_09782e),
    (0x097834, i_097834),
    (0x09783A, i_09783a),
    (0x097840, i_097840),
    (0x097844, i_097844),
    (0x09784A, i_09784a),
    (0x09784E, i_09784e),
    (0x097850, i_097850),
    (0x097852, i_097852),
    (0x097856, i_097856),
    (0x09785A, i_09785a),
    (0x09785C, i_09785c),
    (0x097860, i_097860),
    (0x097866, i_097866),
    (0x09786A, i_09786a),
    (0x097872, i_097872),
    (0x097878, i_097878),
    (0x09787C, i_09787c),
    (0x097880, i_097880),
    (0x097882, i_097882),
    (0x097884, i_097884),
    (0x097888, i_097888),
    (0x09788A, i_09788a),
    (0x09788C, i_09788c),
    (0x097890, i_097890),
    (0x097896, i_097896),
    (0x09789A, i_09789a),
    (0x0978A0, i_0978a0),
    (0x0978A4, i_0978a4),
    (0x0978A8, i_0978a8),
    (0x0978AA, i_0978aa),
    (0x0978AC, i_0978ac),
    (0x0978B0, i_0978b0),
    (0x0978B6, i_0978b6),
    (0x0978BA, i_0978ba),
    (0x0978BE, i_0978be),
    (0x0978C2, i_0978c2),
    (0x0978C6, i_0978c6),
    (0x0978CA, i_0978ca),
    (0x0978CE, i_0978ce),
    (0x0978D4, i_0978d4),
    (0x0978DA, i_0978da),
    (0x0978E0, i_0978e0),
    (0x0978E4, i_0978e4),
    (0x0978E8, i_0978e8),
    (0x0978EA, i_0978ea),
    (0x0978F0, i_0978f0),
    (0x0978F4, i_0978f4),
    (0x0978F8, i_0978f8),
    (0x0978FA, i_0978fa),
    (0x0978FE, i_0978fe),
    (0x097902, i_097902),
    (0x097906, i_097906),
    (0x09790A, i_09790a),
    (0x09790E, i_09790e),
    (0x097912, i_097912),
    (0x097916, i_097916),
    (0x09791C, i_09791c),
    (0x097922, i_097922),
    (0x097926, i_097926),
    (0x09792A, i_09792a),
    (0x09792C, i_09792c),
    (0x09792E, i_09792e),
    (0x097932, i_097932),
    (0x097936, i_097936),
    (0x09793A, i_09793a),
    (0x09793E, i_09793e),
    (0x097944, i_097944),
    (0x097948, i_097948),
    (0x09794A, i_09794a),
    (0x09794C, i_09794c),
    (0x097952, i_097952),
    (0x097958, i_097958),
    (0x09795E, i_09795e),
    (0x097974, i_097974),
    (0x097978, i_097978),
    (0x09797E, i_09797e),
    (0x097984, i_097984),
    (0x09798A, i_09798a),
    (0x097990, i_097990),
    (0x097996, i_097996),
    (0x09799A, i_09799a),
    (0x0979A0, i_0979a0),
    (0x0979A2, i_0979a2),
    (0x0979A6, i_0979a6),
    (0x0979AC, i_0979ac),
    (0x0979B0, i_0979b0),
    (0x0979B6, i_0979b6),
    (0x0979BA, i_0979ba),
    (0x0979BC, i_0979bc),
    (0x0979C0, i_0979c0),
    (0x0979C4, i_0979c4),
    (0x0979CA, i_0979ca),
    (0x0979D0, i_0979d0),
    (0x0979D6, i_0979d6),
    (0x0979DA, i_0979da),
    (0x0979E0, i_0979e0),
    (0x0979E6, i_0979e6),
    (0x0979EA, i_0979ea),
    (0x0979F0, i_0979f0),
    (0x0979F6, i_0979f6),
    (0x0979F8, i_0979f8),
    (0x0979FE, i_0979fe),
    (0x097A04, i_097a04),
    (0x097A08, i_097a08),
    (0x097A0A, i_097a0a),
    (0x097A0C, i_097a0c),
    (0x097A12, i_097a12),
    (0x097A18, i_097a18),
    (0x097A1E, i_097a1e),
    (0x097A22, i_097a22),
    (0x097A24, i_097a24),
    (0x097A26, i_097a26),
    (0x097A2C, i_097a2c),
    (0x097A32, i_097a32),
    (0x097A38, i_097a38),
    (0x097A3C, i_097a3c),
    (0x097A3E, i_097a3e),
    (0x097A40, i_097a40),
    (0x097A46, i_097a46),
    (0x097A48, i_097a48),
    (0x097A4E, i_097a4e),
    (0x097A52, i_097a52),
    (0x097A54, i_097a54),
    (0x097A5A, i_097a5a),
    (0x097A5E, i_097a5e),
    (0x097A60, i_097a60),
    (0x097A66, i_097a66),
    (0x097A6C, i_097a6c),
    (0x097A6E, i_097a6e),
    (0x097A72, i_097a72),
    (0x097A78, i_097a78),
    (0x097A7E, i_097a7e),
    (0x097A80, i_097a80),
    (0x097A84, i_097a84),
    (0x097A88, i_097a88),
    (0x097A8C, i_097a8c),
    (0x097A90, i_097a90),
    (0x097A96, i_097a96),
    (0x097A9A, i_097a9a),
    (0x097A9E, i_097a9e),
    (0x097AA2, i_097aa2),
    (0x097AA4, i_097aa4),
    (0x097AA8, i_097aa8),
    (0x097AAA, i_097aaa),
    (0x097AAE, i_097aae),
    (0x097AB4, i_097ab4),
    (0x097AB6, i_097ab6),
    (0x097AB8, i_097ab8),
    (0x097ABC, i_097abc),
    (0x097ABE, i_097abe),
    (0x097AC4, i_097ac4),
    (0x097AC8, i_097ac8),
    (0x097ACA, i_097aca),
    (0x097AD0, i_097ad0),
    (0x097AD4, i_097ad4),
    (0x097AD8, i_097ad8),
    (0x097ADC, i_097adc),
    (0x097ADE, i_097ade),
    (0x097AE2, i_097ae2),
    (0x097AE6, i_097ae6),
    (0x097AE8, i_097ae8),
    (0x097AEA, i_097aea),
    (0x097AEE, i_097aee),
    (0x097AF0, i_097af0),
    (0x097AF4, i_097af4),
    (0x097AF8, i_097af8),
    (0x097AFC, i_097afc),
    (0x097B02, i_097b02),
    (0x097B06, i_097b06),
    (0x097B0A, i_097b0a),
    (0x097B0E, i_097b0e),
    (0x097B10, i_097b10),
    (0x097B14, i_097b14),
    (0x097B1A, i_097b1a),
    (0x097B20, i_097b20),
    (0x097B26, i_097b26),
    (0x097B2C, i_097b2c),
    (0x097B30, i_097b30),
    (0x097B34, i_097b34),
    (0x097B36, i_097b36),
    (0x097B38, i_097b38),
    (0x097B3C, i_097b3c),
    (0x097B40, i_097b40),
    (0x097B44, i_097b44),
    (0x097B46, i_097b46),
    (0x097B4A, i_097b4a),
    (0x097B4E, i_097b4e),
    (0x097B54, i_097b54),
    (0x097B5A, i_097b5a),
    (0x097B60, i_097b60),
    (0x097B64, i_097b64),
    (0x097B68, i_097b68),
    (0x097B6A, i_097b6a),
    (0x097B6E, i_097b6e),
    (0x097B72, i_097b72),
    (0x097B74, i_097b74),
    (0x097B76, i_097b76),
    (0x097B78, i_097b78),
    (0x097B7A, i_097b7a),
    (0x097B7E, i_097b7e),
    (0x097B82, i_097b82),
    (0x097B84, i_097b84),
    (0x097B88, i_097b88),
    (0x097B8E, i_097b8e),
    (0x097B90, i_097b90),
    (0x097B94, i_097b94),
    (0x097B98, i_097b98),
    (0x097B9C, i_097b9c),
    (0x097B9E, i_097b9e),
    (0x097BA2, i_097ba2),
    (0x097BA8, i_097ba8),
    (0x097BAA, i_097baa),
    (0x097BAC, i_097bac),
    (0x097BAE, i_097bae),
    (0x097BB4, i_097bb4),
    (0x097BB6, i_097bb6),
    (0x097BBC, i_097bbc),
    (0x097BBE, i_097bbe),
    (0x097BC2, i_097bc2),
    (0x097BC4, i_097bc4),
    (0x097BC6, i_097bc6),
    (0x097BC8, i_097bc8),
    (0x097BCE, i_097bce),
    (0x097BD0, i_097bd0),
    (0x097BD6, i_097bd6),
    (0x097BD8, i_097bd8),
    (0x097BDC, i_097bdc),
    (0x097BDE, i_097bde),
    (0x097BE0, i_097be0),
    (0x097BE2, i_097be2),
    (0x097BE8, i_097be8),
    (0x097BEA, i_097bea),
    (0x097BF0, i_097bf0),
    (0x097BF2, i_097bf2),
    (0x097BF6, i_097bf6),
    (0x097BF8, i_097bf8),
    (0x097BFA, i_097bfa),
    (0x097C00, i_097c00),
    (0x097C02, i_097c02),
    (0x097C04, i_097c04),
    (0x097C08, i_097c08),
    (0x097C0A, i_097c0a),
    (0x097C0E, i_097c0e),
    (0x097C12, i_097c12),
    (0x097C16, i_097c16),
    (0x097C18, i_097c18),
    (0x097C1A, i_097c1a),
    (0x097C20, i_097c20),
    (0x097C22, i_097c22),
    (0x097C26, i_097c26),
    (0x097C2A, i_097c2a),
    (0x097C2C, i_097c2c),
    (0x097C2E, i_097c2e),
    (0x097C30, i_097c30),
    (0x097C36, i_097c36),
    (0x097C38, i_097c38),
    (0x097C3A, i_097c3a),
    (0x097C40, i_097c40),
    (0x097C46, i_097c46),
    (0x097C48, i_097c48),
    (0x097C4A, i_097c4a),
    (0x097C4E, i_097c4e),
    (0x097C52, i_097c52),
    (0x097C58, i_097c58),
    (0x097C5A, i_097c5a),
    (0x097C5C, i_097c5c),
    (0x097C5E, i_097c5e),
    (0x097C62, i_097c62),
    (0x097C64, i_097c64),
    (0x097C66, i_097c66),
    (0x097C6A, i_097c6a),
    (0x097C6C, i_097c6c),
    (0x097C6E, i_097c6e),
    (0x097C70, i_097c70),
    (0x097C76, i_097c76),
    (0x097C78, i_097c78),
    (0x097C7A, i_097c7a),
    (0x097C7E, i_097c7e),
    (0x097C82, i_097c82),
    (0x097C84, i_097c84),
    (0x097C8A, i_097c8a),
    (0x097C8E, i_097c8e),
    (0x097C92, i_097c92),
    (0x097C94, i_097c94),
    (0x097C96, i_097c96),
    (0x097C98, i_097c98),
    (0x097C9E, i_097c9e),
    (0x097CA0, i_097ca0),
    (0x097CA2, i_097ca2),
    (0x097CA6, i_097ca6),
    (0x097CAA, i_097caa),
    (0x097CB0, i_097cb0),
    (0x097CB4, i_097cb4),
    (0x097CB8, i_097cb8),
    (0x097CBA, i_097cba),
    (0x097CBC, i_097cbc),
    (0x097CC2, i_097cc2),
    (0x097CC4, i_097cc4),
    (0x097CCA, i_097cca),
    (0x097CCC, i_097ccc),
    (0x097CD0, i_097cd0),
    (0x097CD4, i_097cd4),
    (0x097CD8, i_097cd8),
    (0x097CDC, i_097cdc),
    (0x097CE0, i_097ce0),
    (0x097CE4, i_097ce4),
    (0x097CE8, i_097ce8),
    (0x097CEC, i_097cec),
    (0x097CF0, i_097cf0),
    (0x097CF4, i_097cf4),
    (0x097CF8, i_097cf8),
    (0x097CFC, i_097cfc),
    (0x097D00, i_097d00),
    (0x097D06, i_097d06),
    (0x097D08, i_097d08),
    (0x097D0C, i_097d0c),
    (0x097D0E, i_097d0e),
    (0x097D12, i_097d12),
    (0x097D1A, i_097d1a),
    (0x097D1E, i_097d1e),
    (0x097D20, i_097d20),
    (0x097D78, i_097d78),
    (0x097D7C, i_097d7c),
    (0x097D80, i_097d80),
    (0x097D84, i_097d84),
    (0x097D86, i_097d86),
    (0x097D8A, i_097d8a),
    (0x097D8E, i_097d8e),
    (0x097D92, i_097d92),
    (0x097D96, i_097d96),
    (0x097D98, i_097d98),
    (0x097D9C, i_097d9c),
    (0x097DA0, i_097da0),
    (0x097DA4, i_097da4),
    (0x097DA8, i_097da8),
    (0x097DAC, i_097dac),
    (0x097DB2, i_097db2),
    (0x097DB4, i_097db4),
    (0x097DB8, i_097db8),
    (0x097DBC, i_097dbc),
    (0x097DC0, i_097dc0),
    (0x097DC6, i_097dc6),
    (0x097DC8, i_097dc8),
    (0x097DCE, i_097dce),
    (0x097DD2, i_097dd2),
    (0x097DD6, i_097dd6),
    (0x097DDC, i_097ddc),
    (0x097DE0, i_097de0),
    (0x097DE4, i_097de4),
    (0x097DE6, i_097de6),
    (0x097DEC, i_097dec),
    (0x097DF0, i_097df0),
    (0x097DF4, i_097df4),
    (0x097DF6, i_097df6),
    (0x097DFC, i_097dfc),
    (0x097E00, i_097e00),
    (0x097E04, i_097e04),
    (0x097E08, i_097e08),
    (0x097E0C, i_097e0c),
    (0x097E10, i_097e10),
    (0x097E14, i_097e14),
    (0x097E16, i_097e16),
    (0x097E1A, i_097e1a),
    (0x097E1E, i_097e1e),
    (0x097E22, i_097e22),
    (0x097E26, i_097e26),
    (0x097E2A, i_097e2a),
    (0x097E2E, i_097e2e),
    (0x097E30, i_097e30),
    (0x097E32, i_097e32),
    (0x097E38, i_097e38),
    (0x097E3C, i_097e3c),
    (0x097E42, i_097e42),
    (0x097E46, i_097e46),
    (0x097E48, i_097e48),
    (0x097E4C, i_097e4c),
    (0x097E50, i_097e50),
    (0x097E52, i_097e52),
    (0x097E54, i_097e54),
    (0x097E5A, i_097e5a),
    (0x097E5E, i_097e5e),
    (0x097E60, i_097e60),
    (0x097E62, i_097e62),
    (0x097E64, i_097e64),
    (0x097E6A, i_097e6a),
    (0x097E6E, i_097e6e),
    (0x097E70, i_097e70),
    (0x097E74, i_097e74),
    (0x097E78, i_097e78),
    (0x097E7E, i_097e7e),
    (0x097E82, i_097e82),
    (0x097E84, i_097e84),
    (0x097E88, i_097e88),
    (0x097E8C, i_097e8c),
    (0x097E90, i_097e90),
    (0x097E96, i_097e96),
    (0x097E9A, i_097e9a),
    (0x097E9E, i_097e9e),
    (0x097EA4, i_097ea4),
    (0x097EAA, i_097eaa),
    (0x097EB0, i_097eb0),
    (0x097EB4, i_097eb4),
    (0x097EBA, i_097eba),
    (0x097EBE, i_097ebe),
    (0x097EC2, i_097ec2),
    (0x097EC6, i_097ec6),
    (0x097ECA, i_097eca),
    (0x097ED0, i_097ed0),
    (0x097ED4, i_097ed4),
    (0x097ED8, i_097ed8),
    (0x097EDC, i_097edc),
    (0x097EE2, i_097ee2),
    (0x097EE6, i_097ee6),
    (0x097EEA, i_097eea),
    (0x097EEE, i_097eee),
    (0x097EF2, i_097ef2),
    (0x097EF8, i_097ef8),
    (0x097EFC, i_097efc),
    (0x097F00, i_097f00),
    (0x097F02, i_097f02),
    (0x097F04, i_097f04),
    (0x097F08, i_097f08),
    (0x097F0C, i_097f0c),
    (0x097F10, i_097f10),
    (0x097F16, i_097f16),
    (0x097F1A, i_097f1a),
    (0x097F1E, i_097f1e),
    (0x097F22, i_097f22),
    (0x097F28, i_097f28),
    (0x097F2C, i_097f2c),
    (0x097F30, i_097f30),
    (0x097F34, i_097f34),
    (0x097F38, i_097f38),
    (0x097F3C, i_097f3c),
    (0x097F40, i_097f40),
    (0x097F44, i_097f44),
    (0x097F46, i_097f46),
    (0x097F4A, i_097f4a),
    (0x097F4E, i_097f4e),
    (0x097F50, i_097f50),
    (0x097F52, i_097f52),
    (0x097F58, i_097f58),
    (0x097F5C, i_097f5c),
    (0x097F60, i_097f60),
    (0x097F66, i_097f66),
    (0x097F6A, i_097f6a),
    (0x097F6C, i_097f6c),
    (0x097F72, i_097f72),
    (0x097F76, i_097f76),
    (0x097F7C, i_097f7c),
    (0x097F80, i_097f80),
    (0x097F84, i_097f84),
    (0x097F88, i_097f88),
    (0x097F8C, i_097f8c),
    (0x097F90, i_097f90),
    (0x097F94, i_097f94),
    (0x097F96, i_097f96),
    (0x097F9C, i_097f9c),
    (0x097FA0, i_097fa0),
    (0x097FA4, i_097fa4),
    (0x097FA6, i_097fa6),
    (0x097FA8, i_097fa8),
    (0x097FAC, i_097fac),
    (0x097FB0, i_097fb0),
    (0x097FB4, i_097fb4),
    (0x097FB8, i_097fb8),
    (0x097FBC, i_097fbc),
    (0x097FC0, i_097fc0),
    (0x097FC4, i_097fc4),
    (0x097FC8, i_097fc8),
    (0x097FCC, i_097fcc),
    (0x097FD0, i_097fd0),
    (0x097FD4, i_097fd4),
    (0x097FDA, i_097fda),
    (0x097FDC, i_097fdc),
    (0x097FE0, i_097fe0),
    (0x097FE2, i_097fe2),
    (0x097FE6, i_097fe6),
    (0x097FEA, i_097fea),
    (0x097FEE, i_097fee),
    (0x097FF0, i_097ff0),
    (0x097FF6, i_097ff6),
    (0x097FFC, i_097ffc),
    (0x098002, i_098002),
    (0x098006, i_098006),
    (0x098008, i_098008),
    (0x09800C, i_09800c),
    (0x098010, i_098010),
    (0x098014, i_098014),
    (0x098018, i_098018),
    (0x09801A, i_09801a),
    (0x098020, i_098020),
    (0x098024, i_098024),
    (0x098028, i_098028),
    (0x09802C, i_09802c),
    (0x098030, i_098030),
    (0x098034, i_098034),
    (0x098038, i_098038),
    (0x09803C, i_09803c),
    (0x098040, i_098040),
    (0x098044, i_098044),
    (0x09804A, i_09804a),
    (0x09804E, i_09804e),
    (0x098050, i_098050),
    (0x098052, i_098052),
    (0x098056, i_098056),
    (0x09805A, i_09805a),
    (0x09805E, i_09805e),
    (0x098062, i_098062),
    (0x09806A, i_09806a),
    (0x09806E, i_09806e),
    (0x098072, i_098072),
    (0x098076, i_098076),
    (0x09807A, i_09807a),
    (0x09807C, i_09807c),
    (0x098080, i_098080),
    (0x098084, i_098084),
    (0x098088, i_098088),
    (0x09808A, i_09808a),
    (0x09808E, i_09808e),
    (0x098092, i_098092),
    (0x098096, i_098096),
    (0x09809A, i_09809a),
    (0x09809E, i_09809e),
    (0x0980A2, i_0980a2),
    (0x0980A6, i_0980a6),
    (0x0980AA, i_0980aa),
    (0x0980AE, i_0980ae),
    (0x0980B4, i_0980b4),
    (0x0980B8, i_0980b8),
    (0x0980BA, i_0980ba),
    (0x0980BE, i_0980be),
    (0x0980C2, i_0980c2),
    (0x0980C6, i_0980c6),
    (0x0980CC, i_0980cc),
    (0x0980CE, i_0980ce),
    (0x0980D4, i_0980d4),
    (0x0980D8, i_0980d8),
    (0x0980DC, i_0980dc),
    (0x0980DE, i_0980de),
    (0x0980E4, i_0980e4),
    (0x0980E6, i_0980e6),
    (0x0980EA, i_0980ea),
    (0x0980EC, i_0980ec),
    (0x0980F0, i_0980f0),
    (0x0980F4, i_0980f4),
    (0x0980F8, i_0980f8),
    (0x0980FA, i_0980fa),
    (0x0980FC, i_0980fc),
    (0x098102, i_098102),
    (0x098106, i_098106),
    (0x09810A, i_09810a),
    (0x09810E, i_09810e),
    (0x098114, i_098114),
    (0x098118, i_098118),
    (0x09811C, i_09811c),
    (0x098120, i_098120),
    (0x098122, i_098122),
    (0x098128, i_098128),
    (0x09812C, i_09812c),
    (0x098130, i_098130),
    (0x098134, i_098134),
    (0x098138, i_098138),
    (0x09813A, i_09813a),
    (0x09813C, i_09813c),
    (0x098142, i_098142),
    (0x098144, i_098144),
    (0x09814A, i_09814a),
    (0x09814E, i_09814e),
    (0x098154, i_098154),
    (0x098158, i_098158),
    (0x09815E, i_09815e),
    (0x098162, i_098162),
    (0x098168, i_098168),
    (0x09816E, i_09816e),
    (0x098174, i_098174),
    (0x098176, i_098176),
    (0x09817C, i_09817c),
    (0x09817E, i_09817e),
    (0x098180, i_098180),
    (0x098184, i_098184),
    (0x09818A, i_09818a),
    (0x09818C, i_09818c),
    (0x098190, i_098190),
    (0x098192, i_098192),
    (0x098198, i_098198),
    (0x09819C, i_09819c),
    (0x09819E, i_09819e),
    (0x0981A2, i_0981a2),
    (0x0981A4, i_0981a4),
    (0x0981A8, i_0981a8),
    (0x0981AC, i_0981ac),
    (0x0981AE, i_0981ae),
    (0x0981B4, i_0981b4),
    (0x0981B6, i_0981b6),
    (0x0981B8, i_0981b8),
    (0x0981BC, i_0981bc),
    (0x0981C2, i_0981c2),
    (0x0981C4, i_0981c4),
    (0x0981C8, i_0981c8),
    (0x0981CA, i_0981ca),
    (0x0981CC, i_0981cc),
    (0x0981D0, i_0981d0),
    (0x0981D4, i_0981d4),
    (0x0981D8, i_0981d8),
    (0x0981DC, i_0981dc),
    (0x0981E0, i_0981e0),
    (0x0981E2, i_0981e2),
    (0x0981E6, i_0981e6),
    (0x0981E8, i_0981e8),
    (0x0981FC, i_0981fc),
    (0x098202, i_098202),
    (0x098208, i_098208),
    (0x09820A, i_09820a),
    (0x09820C, i_09820c),
    (0x09820E, i_09820e),
    (0x098210, i_098210),
    (0x098214, i_098214),
    (0x098218, i_098218),
    (0x09821C, i_09821c),
    (0x098220, i_098220),
    (0x098222, i_098222),
    (0x098228, i_098228),
    (0x09822A, i_09822a),
    (0x09822E, i_09822e),
    (0x098230, i_098230),
    (0x098232, i_098232),
    (0x098234, i_098234),
    (0x09823A, i_09823a),
    (0x09823C, i_09823c),
    (0x098240, i_098240),
    (0x098244, i_098244),
    (0x09824A, i_09824a),
    (0x098250, i_098250),
    (0x098252, i_098252),
    (0x098256, i_098256),
    (0x09825A, i_09825a),
    (0x098260, i_098260),
    (0x098264, i_098264),
    (0x098268, i_098268),
    (0x09826A, i_09826a),
    (0x09826C, i_09826c),
    (0x09826E, i_09826e),
    (0x098274, i_098274),
    (0x098278, i_098278),
    (0x09827C, i_09827c),
    (0x098280, i_098280),
    (0x098282, i_098282),
    (0x098286, i_098286),
    (0x098288, i_098288),
    (0x09828C, i_09828c),
    (0x098290, i_098290),
    (0x098294, i_098294),
    (0x09829A, i_09829a),
    (0x09829E, i_09829e),
    (0x0982A0, i_0982a0),
    (0x0982A6, i_0982a6),
    (0x0982AA, i_0982aa),
    (0x0982AE, i_0982ae),
    (0x0982B2, i_0982b2),
    (0x0982B8, i_0982b8),
    (0x0982BE, i_0982be),
    (0x0982C2, i_0982c2),
    (0x0982C6, i_0982c6),
    (0x0982CC, i_0982cc),
    (0x0982D0, i_0982d0),
    (0x0982D4, i_0982d4),
    (0x0982DA, i_0982da),
    (0x0982DE, i_0982de),
    (0x0982E2, i_0982e2),
    (0x0982E8, i_0982e8),
    (0x0982EC, i_0982ec),
    (0x0982F0, i_0982f0),
    (0x0982F6, i_0982f6),
    (0x0982FA, i_0982fa),
    (0x098300, i_098300),
    (0x098304, i_098304),
    (0x098306, i_098306),
    (0x098308, i_098308),
    (0x09830C, i_09830c),
    (0x098310, i_098310),
    (0x098314, i_098314),
    (0x098316, i_098316),
    (0x09831A, i_09831a),
    (0x098320, i_098320),
    (0x098324, i_098324),
    (0x098328, i_098328),
    (0x09832E, i_09832e),
    (0x098332, i_098332),
    (0x098336, i_098336),
    (0x098338, i_098338),
    (0x09833C, i_09833c),
    (0x09833E, i_09833e),
    (0x098342, i_098342),
    (0x098348, i_098348),
    (0x09834C, i_09834c),
    (0x098350, i_098350),
    (0x098356, i_098356),
    (0x09835A, i_09835a),
    (0x09835E, i_09835e),
    (0x098362, i_098362),
    (0x098366, i_098366),
    (0x09836A, i_09836a),
    (0x09836E, i_09836e),
    (0x098372, i_098372),
    (0x098378, i_098378),
    (0x09837C, i_09837c),
    (0x098380, i_098380),
    (0x098386, i_098386),
    (0x098388, i_098388),
    (0x09838C, i_09838c),
    (0x098392, i_098392),
    (0x098396, i_098396),
    (0x09839A, i_09839a),
    (0x0983A0, i_0983a0),
    (0x0983A4, i_0983a4),
    (0x0983A8, i_0983a8),
    (0x0983AC, i_0983ac),
    (0x0983B0, i_0983b0),
    (0x0983B6, i_0983b6),
    (0x0983BA, i_0983ba),
    (0x0983BE, i_0983be),
    (0x0983C2, i_0983c2),
    (0x0983C6, i_0983c6),
    (0x0983CA, i_0983ca),
    (0x0983CE, i_0983ce),
    (0x0983D2, i_0983d2),
    (0x0983D6, i_0983d6),
    (0x0983DA, i_0983da),
    (0x0983E0, i_0983e0),
    (0x0983E4, i_0983e4),
    (0x0983E8, i_0983e8),
    (0x0983EE, i_0983ee),
    (0x0983F2, i_0983f2),
    (0x0983F4, i_0983f4),
    (0x0983F6, i_0983f6),
    (0x0983FA, i_0983fa),
    (0x0983FE, i_0983fe),
    (0x098404, i_098404),
    (0x09840A, i_09840a),
    (0x09840C, i_09840c),
    (0x098412, i_098412),
    (0x098416, i_098416),
    (0x09841A, i_09841a),
    (0x09841E, i_09841e),
    (0x098424, i_098424),
    (0x098428, i_098428),
    (0x09842C, i_09842c),
    (0x098430, i_098430),
    (0x098436, i_098436),
    (0x09843C, i_09843c),
    (0x098440, i_098440),
    (0x098444, i_098444),
    (0x09844A, i_09844a),
    (0x09844E, i_09844e),
    (0x098452, i_098452),
    (0x098458, i_098458),
    (0x09845C, i_09845c),
    (0x098460, i_098460),
    (0x098466, i_098466),
    (0x09846A, i_09846a),
    (0x09846E, i_09846e),
    (0x098474, i_098474),
    (0x09847A, i_09847a),
    (0x09847E, i_09847e),
    (0x098480, i_098480),
    (0x098482, i_098482),
    (0x098486, i_098486),
    (0x09848A, i_09848a),
    (0x09848C, i_09848c),
    (0x098490, i_098490),
    (0x098496, i_098496),
    (0x09849A, i_09849a),
    (0x09849E, i_09849e),
    (0x0984A4, i_0984a4),
    (0x0984A8, i_0984a8),
    (0x0984AC, i_0984ac),
    (0x0984B0, i_0984b0),
    (0x0984B2, i_0984b2),
    (0x0984B6, i_0984b6),
    (0x0984BC, i_0984bc),
    (0x0984C0, i_0984c0),
    (0x0984C4, i_0984c4),
    (0x0984CA, i_0984ca),
    (0x0984CE, i_0984ce),
    (0x0984D2, i_0984d2),
    (0x0984D6, i_0984d6),
    (0x0984DA, i_0984da),
    (0x0984DE, i_0984de),
    (0x0984E2, i_0984e2),
    (0x0984E6, i_0984e6),
    (0x0984EC, i_0984ec),
    (0x0984F0, i_0984f0),
    (0x0984F4, i_0984f4),
    (0x0984FA, i_0984fa),
    (0x0984FC, i_0984fc),
    (0x098500, i_098500),
    (0x098506, i_098506),
    (0x09850A, i_09850a),
    (0x09850E, i_09850e),
    (0x098514, i_098514),
    (0x098518, i_098518),
    (0x09851C, i_09851c),
    (0x098524, i_098524),
    (0x09852C, i_09852c),
    (0x098534, i_098534),
    (0x09853C, i_09853c),
    (0x098540, i_098540),
    (0x098544, i_098544),
    (0x098548, i_098548),
    (0x09854E, i_09854e),
    (0x098552, i_098552),
    (0x098556, i_098556),
    (0x09855C, i_09855c),
    (0x098560, i_098560),
    (0x098562, i_098562),
    (0x098566, i_098566),
    (0x09856A, i_09856a),
    (0x098570, i_098570),
    (0x098576, i_098576),
    (0x098578, i_098578),
    (0x09857C, i_09857c),
    (0x098584, i_098584),
    (0x09858C, i_09858c),
    (0x098594, i_098594),
    (0x09859E, i_09859e),
    (0x0985A6, i_0985a6),
    (0x0985AC, i_0985ac),
    (0x0985B2, i_0985b2),
    (0x0985B6, i_0985b6),
    (0x0985BA, i_0985ba),
    (0x0985BE, i_0985be),
    (0x0985C0, i_0985c0),
    (0x0985C4, i_0985c4),
    (0x0985C8, i_0985c8),
    (0x0985CA, i_0985ca),
    (0x0985CE, i_0985ce),
    (0x0985D6, i_0985d6),
    (0x0985DE, i_0985de),
    (0x0985E6, i_0985e6),
    (0x0985EC, i_0985ec),
    (0x0985F0, i_0985f0),
    (0x0985F2, i_0985f2),
    (0x0985F4, i_0985f4),
    (0x0985F6, i_0985f6),
    (0x0985F8, i_0985f8),
    (0x0985FA, i_0985fa),
    (0x098604, i_098604),
    (0x09860C, i_09860c),
    (0x098612, i_098612),
    (0x098618, i_098618),
    (0x09861C, i_09861c),
    (0x098622, i_098622),
    (0x098626, i_098626),
    (0x09862A, i_09862a),
    (0x09862E, i_09862e),
    (0x098634, i_098634),
    (0x09863A, i_09863a),
    (0x098640, i_098640),
    (0x098646, i_098646),
    (0x09864C, i_09864c),
    (0x098652, i_098652),
    (0x098656, i_098656),
    (0x09865A, i_09865a),
    (0x09865E, i_09865e),
    (0x098660, i_098660),
    (0x098664, i_098664),
    (0x098668, i_098668),
    (0x09866A, i_09866a),
    (0x09866E, i_09866e),
    (0x098676, i_098676),
    (0x09867E, i_09867e),
    (0x098686, i_098686),
    (0x09868C, i_09868c),
    (0x098690, i_098690),
    (0x098704, i_098704),
    (0x098708, i_098708),
    (0x09870C, i_09870c),
    (0x098710, i_098710),
    (0x098714, i_098714),
    (0x098718, i_098718),
    (0x09871A, i_09871a),
    (0x09871E, i_09871e),
    (0x098830, i_098830),
    (0x098834, i_098834),
    (0x098836, i_098836),
    (0x098838, i_098838),
    (0x09883C, i_09883c),
    (0x098842, i_098842),
    (0x098846, i_098846),
    (0x09884C, i_09884c),
    (0x098852, i_098852),
    (0x098856, i_098856),
    (0x098858, i_098858),
    (0x09885A, i_09885a),
    (0x098860, i_098860),
    (0x098866, i_098866),
    (0x09886C, i_09886c),
    (0x098872, i_098872),
    (0x098878, i_098878),
    (0x09887E, i_09887e),
    (0x098882, i_098882),
    (0x098884, i_098884),
    (0x098886, i_098886),
    (0x09888A, i_09888a),
    (0x09888E, i_09888e),
    (0x098892, i_098892),
    (0x098896, i_098896),
    (0x09889C, i_09889c),
    (0x0988A0, i_0988a0),
    (0x0988A6, i_0988a6),
    (0x0988AA, i_0988aa),
    (0x0988B0, i_0988b0),
    (0x0988B2, i_0988b2),
    (0x0988B6, i_0988b6),
    (0x0988BC, i_0988bc),
    (0x0988C0, i_0988c0),
    (0x0988C6, i_0988c6),
    (0x0988CC, i_0988cc),
    (0x0988D2, i_0988d2),
    (0x0988D4, i_0988d4),
    (0x0988D8, i_0988d8),
    (0x0988DC, i_0988dc),
    (0x0988E2, i_0988e2),
    (0x0988E8, i_0988e8),
    (0x0988EC, i_0988ec),
    (0x0988F2, i_0988f2),
    (0x0988F8, i_0988f8),
    (0x0988FE, i_0988fe),
    (0x098904, i_098904),
    (0x098908, i_098908),
    (0x09890A, i_09890a),
    (0x09890C, i_09890c),
    (0x098910, i_098910),
    (0x098914, i_098914),
    (0x098918, i_098918),
    (0x09891C, i_09891c),
    (0x098920, i_098920),
    (0x098924, i_098924),
    (0x098926, i_098926),
    (0x098928, i_098928),
    (0x09892E, i_09892e),
    (0x098934, i_098934),
    (0x09893A, i_09893a),
    (0x09893E, i_09893e),
    (0x098942, i_098942),
    (0x098948, i_098948),
    (0x09894C, i_09894c),
    (0x098950, i_098950),
    (0x098954, i_098954),
    (0x09895A, i_09895a),
    (0x09895E, i_09895e),
    (0x098962, i_098962),
    (0x098964, i_098964),
    (0x098966, i_098966),
    (0x09896C, i_09896c),
    (0x098970, i_098970),
    (0x098976, i_098976),
    (0x09897A, i_09897a),
    (0x09897E, i_09897e),
    (0x098980, i_098980),
    (0x098982, i_098982),
    (0x098988, i_098988),
    (0x09898A, i_09898a),
    (0x098990, i_098990),
    (0x098994, i_098994),
    (0x09899A, i_09899a),
    (0x09899E, i_09899e),
    (0x0989A0, i_0989a0),
    (0x0989A4, i_0989a4),
    (0x0989A8, i_0989a8),
    (0x0989AC, i_0989ac),
    (0x0989B0, i_0989b0),
    (0x0989B4, i_0989b4),
    (0x0989B6, i_0989b6),
    (0x0989BA, i_0989ba),
    (0x0989BC, i_0989bc),
    (0x0989C0, i_0989c0),
    (0x0989C6, i_0989c6),
    (0x0989CC, i_0989cc),
    (0x0989D2, i_0989d2),
    (0x0989D8, i_0989d8),
    (0x0989DC, i_0989dc),
    (0x0989DE, i_0989de),
    (0x0989E0, i_0989e0),
    (0x0989E6, i_0989e6),
    (0x0989EC, i_0989ec),
    (0x0989F2, i_0989f2),
    (0x0989F6, i_0989f6),
    (0x0989FC, i_0989fc),
    (0x098A02, i_098a02),
    (0x098A08, i_098a08),
    (0x098A0E, i_098a0e),
    (0x098A14, i_098a14),
    (0x098A1A, i_098a1a),
    (0x098A20, i_098a20),
    (0x098A26, i_098a26),
    (0x098A2A, i_098a2a),
    (0x098A2C, i_098a2c),
    (0x098A32, i_098a32),
    (0x098A38, i_098a38),
    (0x098A3C, i_098a3c),
    (0x098A40, i_098a40),
    (0x098A42, i_098a42),
    (0x098A48, i_098a48),
    (0x098A4C, i_098a4c),
    (0x098A50, i_098a50),
    (0x098A56, i_098a56),
    (0x098A5A, i_098a5a),
    (0x098A60, i_098a60),
    (0x098A64, i_098a64),
    (0x098A68, i_098a68),
    (0x098A6A, i_098a6a),
    (0x098A70, i_098a70),
    (0x098A76, i_098a76),
    (0x098A7C, i_098a7c),
    (0x098A80, i_098a80),
    (0x098A84, i_098a84),
    (0x098A86, i_098a86),
    (0x098A88, i_098a88),
    (0x098A8E, i_098a8e),
    (0x098A94, i_098a94),
    (0x098A9A, i_098a9a),
    (0x098A9E, i_098a9e),
    (0x098AA0, i_098aa0),
    (0x098AA6, i_098aa6),
    (0x098AAC, i_098aac),
    (0x098AB0, i_098ab0),
    (0x098AB4, i_098ab4),
    (0x098AB6, i_098ab6),
    (0x098ABC, i_098abc),
    (0x098AC2, i_098ac2),
    (0x098AC8, i_098ac8),
    (0x098ACC, i_098acc),
    (0x098AD0, i_098ad0),
    (0x098AD2, i_098ad2),
    (0x098AD4, i_098ad4),
    (0x098AD8, i_098ad8),
    (0x098ADE, i_098ade),
    (0x098AE4, i_098ae4),
    (0x098AEA, i_098aea),
    (0x098AF0, i_098af0),
    (0x098AF6, i_098af6),
    (0x098AFA, i_098afa),
    (0x098AFC, i_098afc),
    (0x098AFE, i_098afe),
    (0x098B04, i_098b04),
    (0x098B08, i_098b08),
    (0x098B0E, i_098b0e),
    (0x098B14, i_098b14),
    (0x098B1A, i_098b1a),
    (0x098B20, i_098b20),
    (0x098B26, i_098b26),
    (0x098B2C, i_098b2c),
    (0x098B30, i_098b30),
    (0x098B32, i_098b32),
    (0x098B38, i_098b38),
    (0x098B3E, i_098b3e),
    (0x098B42, i_098b42),
    (0x098B46, i_098b46),
    (0x098B48, i_098b48),
    (0x098B4E, i_098b4e),
    (0x098B54, i_098b54),
    (0x098B5A, i_098b5a),
    (0x098B5E, i_098b5e),
    (0x098B62, i_098b62),
    (0x098B64, i_098b64),
    (0x098B66, i_098b66),
    (0x098B6C, i_098b6c),
    (0x098B72, i_098b72),
    (0x098B76, i_098b76),
    (0x098B78, i_098b78),
    (0x098B7E, i_098b7e),
    (0x098B84, i_098b84),
    (0x098B88, i_098b88),
    (0x098B8C, i_098b8c),
    (0x098B8E, i_098b8e),
    (0x098B94, i_098b94),
    (0x098B9A, i_098b9a),
    (0x098BA0, i_098ba0),
    (0x098BA6, i_098ba6),
    (0x098BAA, i_098baa),
    (0x098BAE, i_098bae),
    (0x098BB0, i_098bb0),
    (0x098BB2, i_098bb2),
    (0x098BB6, i_098bb6),
    (0x098BBC, i_098bbc),
    (0x098BBE, i_098bbe),
    (0x098BC0, i_098bc0),
    (0x098BC4, i_098bc4),
    (0x098BC6, i_098bc6),
    (0x098BCC, i_098bcc),
    (0x098BDC, i_098bdc),
    (0x098BE0, i_098be0),
    (0x098BE6, i_098be6),
    (0x098BEC, i_098bec),
    (0x098BF2, i_098bf2),
    (0x098BF8, i_098bf8),
    (0x098BFC, i_098bfc),
    (0x098BFE, i_098bfe),
    (0x098C00, i_098c00),
    (0x098C06, i_098c06),
    (0x098C0C, i_098c0c),
    (0x098C12, i_098c12),
    (0x098C18, i_098c18),
    (0x098C1C, i_098c1c),
    (0x098C22, i_098c22),
    (0x098C28, i_098c28),
    (0x098C2E, i_098c2e),
    (0x098C5A, i_098c5a),
    (0x098C60, i_098c60),
    (0x098C64, i_098c64),
    (0x098C6A, i_098c6a),
    (0x098C6C, i_098c6c),
    (0x098C70, i_098c70),
    (0x098C76, i_098c76),
    (0x098C7A, i_098c7a),
    (0x098C80, i_098c80),
    (0x098C86, i_098c86),
    (0x098C8A, i_098c8a),
    (0x098C8C, i_098c8c),
    (0x098C92, i_098c92),
    (0x098C98, i_098c98),
    (0x098C9C, i_098c9c),
    (0x098CA0, i_098ca0),
    (0x098CA2, i_098ca2),
    (0x098CA8, i_098ca8),
    (0x098CAC, i_098cac),
    (0x098CB0, i_098cb0),
    (0x098CB2, i_098cb2),
    (0x098CB8, i_098cb8),
    (0x098CBC, i_098cbc),
    (0x098CC0, i_098cc0),
    (0x098CC6, i_098cc6),
    (0x098CCA, i_098cca),
    (0x098CD0, i_098cd0),
    (0x098CD4, i_098cd4),
    (0x098CD8, i_098cd8),
    (0x098CDC, i_098cdc),
    (0x098CDE, i_098cde),
    (0x098CE2, i_098ce2),
    (0x098CE6, i_098ce6),
    (0x098CE8, i_098ce8),
    (0x098CEC, i_098cec),
    (0x098CF0, i_098cf0),
    (0x098CF4, i_098cf4),
    (0x098CF8, i_098cf8),
    (0x098CFA, i_098cfa),
    (0x098CFE, i_098cfe),
    (0x098D02, i_098d02),
    (0x098D04, i_098d04),
    (0x098D0A, i_098d0a),
    (0x098D10, i_098d10),
    (0x098D16, i_098d16),
    (0x098D1A, i_098d1a),
    (0x098D1E, i_098d1e),
    (0x098D20, i_098d20),
    (0x098D22, i_098d22),
    (0x098D28, i_098d28),
    (0x098D2C, i_098d2c),
    (0x098D30, i_098d30),
    (0x098D34, i_098d34),
    (0x098D3A, i_098d3a),
    (0x098D40, i_098d40),
    (0x098D46, i_098d46),
    (0x098D4C, i_098d4c),
    (0x098D52, i_098d52),
    (0x098D56, i_098d56),
    (0x098D58, i_098d58),
    (0x098D5E, i_098d5e),
    (0x098D64, i_098d64),
    (0x098D68, i_098d68),
    (0x098D6C, i_098d6c),
    (0x098D6E, i_098d6e),
    (0x098D74, i_098d74),
    (0x098D7A, i_098d7a),
    (0x098D80, i_098d80),
    (0x098D84, i_098d84),
    (0x098D88, i_098d88),
    (0x098D8A, i_098d8a),
    (0x098D8C, i_098d8c),
    (0x098D92, i_098d92),
    (0x098DA2, i_098da2),
    (0x098DA6, i_098da6),
    (0x098DAC, i_098dac),
    (0x098DB2, i_098db2),
    (0x098DB8, i_098db8),
    (0x098DBE, i_098dbe),
    (0x098DC2, i_098dc2),
    (0x098DC4, i_098dc4),
    (0x098DC6, i_098dc6),
    (0x098DCC, i_098dcc),
    (0x098DD2, i_098dd2),
    (0x098DD8, i_098dd8),
    (0x098DDE, i_098dde),
    (0x098DE2, i_098de2),
    (0x098DE8, i_098de8),
    (0x098DEE, i_098dee),
    (0x098DF4, i_098df4),
    (0x098DF8, i_098df8),
    (0x098DFE, i_098dfe),
    (0x098E00, i_098e00),
    (0x098E04, i_098e04),
    (0x098E0A, i_098e0a),
    (0x098E0E, i_098e0e),
    (0x098E14, i_098e14),
    (0x098E18, i_098e18),
    (0x098E1A, i_098e1a),
    (0x098E20, i_098e20),
    (0x098E26, i_098e26),
    (0x098E2A, i_098e2a),
    (0x098E2E, i_098e2e),
    (0x098E30, i_098e30),
    (0x098E36, i_098e36),
    (0x098E3C, i_098e3c),
    (0x098E42, i_098e42),
    (0x098E46, i_098e46),
    (0x098E4A, i_098e4a),
    (0x098E4C, i_098e4c),
    (0x098E4E, i_098e4e),
    (0x098E54, i_098e54),
    (0x098E5A, i_098e5a),
    (0x098E5E, i_098e5e),
    (0x098E60, i_098e60),
    (0x098E66, i_098e66),
    (0x098E6C, i_098e6c),
    (0x098E72, i_098e72),
    (0x098E76, i_098e76),
    (0x098E7A, i_098e7a),
    (0x098E7C, i_098e7c),
    (0x098E82, i_098e82),
    (0x098E88, i_098e88),
    (0x098E8E, i_098e8e),
    (0x098E92, i_098e92),
    (0x098E96, i_098e96),
    (0x098E98, i_098e98),
    (0x098E9A, i_098e9a),
    (0x098E9E, i_098e9e),
    (0x098EA4, i_098ea4),
    (0x098EA8, i_098ea8),
    (0x098EAE, i_098eae),
    (0x098EB4, i_098eb4),
    (0x098EBA, i_098eba),
    (0x098EC0, i_098ec0),
    (0x098EC4, i_098ec4),
    (0x098ECA, i_098eca),
    (0x098ECE, i_098ece),
    (0x098ED4, i_098ed4),
    (0x098EDA, i_098eda),
    (0x098EE0, i_098ee0),
    (0x098EE6, i_098ee6),
    (0x098EEA, i_098eea),
    (0x098EEE, i_098eee),
    (0x098EF2, i_098ef2),
    (0x098EF6, i_098ef6),
    (0x098EFC, i_098efc),
    (0x098F00, i_098f00),
    (0x098F04, i_098f04),
    (0x098F0A, i_098f0a),
    (0x098F10, i_098f10),
    (0x098F14, i_098f14),
    (0x098F16, i_098f16),
    (0x098F1C, i_098f1c),
    (0x098F22, i_098f22),
    (0x098F28, i_098f28),
    (0x098F2E, i_098f2e),
    (0x098F34, i_098f34),
    (0x098F38, i_098f38),
    (0x098F3C, i_098f3c),
    (0x098F3E, i_098f3e),
    (0x098F40, i_098f40),
    (0x098F44, i_098f44),
    (0x098F4A, i_098f4a),
    (0x098F66, i_098f66),
    (0x098F6C, i_098f6c),
    (0x098F72, i_098f72),
    (0x098F78, i_098f78),
    (0x098F7E, i_098f7e),
    (0x098F82, i_098f82),
    (0x098F88, i_098f88),
    (0x098F8E, i_098f8e),
];
