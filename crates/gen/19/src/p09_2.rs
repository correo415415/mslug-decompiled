// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p09_2) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $0981E6: rts
pub fn i_0981e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x981E8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
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
// ===== LogoScene_Tpl_098720  ($098720, 272 B, frontend/hiscore_memcard_options_09773c.s)
/// $098720: move.w #$2c, d0
pub fn i_098720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98724;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $098724: jsr $2352.l
pub fn i_098724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9872A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9872A);
    cpu.pc = 0x2352;
    16
}
/// $09872A: move.w #$d, d0
pub fn i_09872a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9872E;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09872E: jsr $43568.l
pub fn i_09872e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98734;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98734);
    cpu.pc = 0x43568;
    16
}
/// $098734: moveq #$0, d0
pub fn i_098734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98736;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $098736: moveq #$0, d1
pub fn i_098736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98738;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $098738: jsr $437da.l
pub fn i_098738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9873E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9873E);
    cpu.pc = 0x437DA;
    16
}
/// $09873E: move.w #$c8, $82(a6)
pub fn i_09873e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98744;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $098744: lea.l $98838(pc), a1
pub fn i_098744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98748;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x98838;
    16
}
/// $098748: jsr $4ae.l
pub fn i_098748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9874E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9874E);
    cpu.pc = 0x4AE;
    16
}
/// $09874E: jsr $5dd02.l
pub fn i_09874e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98754;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98754);
    cpu.pc = 0x5DD02;
    16
}
/// $098754: move.w #$0, $80(a0)
pub fn i_098754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9875A;
    cpu.ir = 0x317C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09875A: lea.l $98838(pc), a1
pub fn i_09875a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9875E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x98838;
    16
}
/// $09875E: jsr $4ae.l
pub fn i_09875e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98764;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98764);
    cpu.pc = 0x4AE;
    16
}
/// $098764: jsr $5dd02.l
pub fn i_098764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9876A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9876A);
    cpu.pc = 0x5DD02;
    16
}
/// $09876A: move.w #$1, $80(a0)
pub fn i_09876a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98770;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $098770: lea.l $98838(pc), a1
pub fn i_098770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98774;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x98838;
    16
}
/// $098774: jsr $4ae.l
pub fn i_098774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9877A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9877A);
    cpu.pc = 0x4AE;
    16
}
/// $09877A: jsr $5dd02.l
pub fn i_09877a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98780;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98780);
    cpu.pc = 0x5DD02;
    16
}
/// $098780: move.w #$2, $80(a0)
pub fn i_098780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98786;
    cpu.ir = 0x317C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $098786: lea.l $98838(pc), a1
pub fn i_098786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9878A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x98838;
    16
}
/// $09878A: jsr $4ae.l
pub fn i_09878a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98790;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98790);
    cpu.pc = 0x4AE;
    16
}
/// $098790: jsr $5dd02.l
pub fn i_098790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98796;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98796);
    cpu.pc = 0x5DD02;
    16
}
/// $098796: move.w #$3, $80(a0)
pub fn i_098796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9879C;
    cpu.ir = 0x317C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09879C: lea.l $98838(pc), a1
pub fn i_09879c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x987A0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x98838;
    16
}
/// $0987A0: jsr $4ae.l
pub fn i_0987a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x987A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x987A6);
    cpu.pc = 0x4AE;
    16
}
/// $0987A6: jsr $5dd02.l
pub fn i_0987a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x987AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x987AC);
    cpu.pc = 0x5DD02;
    16
}
/// $0987AC: move.w #$4, $80(a0)
pub fn i_0987ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x987B2;
    cpu.ir = 0x317C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0987B2: lea.l $98838(pc), a1
pub fn i_0987b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x987B6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x98838;
    16
}
/// $0987B6: jsr $4ae.l
pub fn i_0987b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x987BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x987BC);
    cpu.pc = 0x4AE;
    16
}
/// $0987BC: jsr $5dd02.l
pub fn i_0987bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x987C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x987C2);
    cpu.pc = 0x5DD02;
    16
}
/// $0987C2: move.w #$5, $80(a0)
pub fn i_0987c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x987C8;
    cpu.ir = 0x317C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0987C8: lea.l $98838(pc), a1
pub fn i_0987c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x987CC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x98838;
    16
}
/// $0987CC: jsr $4ae.l
pub fn i_0987cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x987D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x987D2);
    cpu.pc = 0x4AE;
    16
}
/// $0987D2: jsr $5dd02.l
pub fn i_0987d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x987D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x987D8);
    cpu.pc = 0x5DD02;
    16
}
/// $0987D8: move.w #$6, $80(a0)
pub fn i_0987d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x987DE;
    cpu.ir = 0x317C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0987DE: lea.l $98838(pc), a1
pub fn i_0987de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x987E2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x98838;
    16
}
/// $0987E2: jsr $4ae.l
pub fn i_0987e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x987E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x987E8);
    cpu.pc = 0x4AE;
    16
}
/// $0987E8: jsr $5dd02.l
pub fn i_0987e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x987EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x987EE);
    cpu.pc = 0x5DD02;
    16
}
/// $0987EE: move.w #$7, $80(a0)
pub fn i_0987ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x987F4;
    cpu.ir = 0x317C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0987F4: lea.l $98838(pc), a1
pub fn i_0987f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x987F8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x98838;
    16
}
/// $0987F8: jsr $4ae.l
pub fn i_0987f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x987FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x987FE);
    cpu.pc = 0x4AE;
    16
}
/// $0987FE: jsr $5dd02.l
pub fn i_0987fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98804;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98804);
    cpu.pc = 0x5DD02;
    16
}
/// $098804: move.w #$8, $80(a0)
pub fn i_098804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9880A;
    cpu.ir = 0x317C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09880A: lea.l $988b2(pc), a1
pub fn i_09880a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9880E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x988B2;
    16
}
/// $09880E: jsr $4ae.l
pub fn i_09880e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98814;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98814);
    cpu.pc = 0x4AE;
    16
}
/// $098814: jsr $5dd02.l
pub fn i_098814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9881A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9881A);
    cpu.pc = 0x5DD02;
    16
}
/// $09881A: lea.l $98820(pc), a1
pub fn i_09881a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9881E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x98820;
    16
}
/// $09881E: move.l a1, (a6)
pub fn i_09881e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98820;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $098820: subq.w #$1, $82(a6)
pub fn i_098820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98824;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $098824: beq.w $9882a
pub fn i_098824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98828;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9882A; return 10; }
    8
}
/// $098828: rts
pub fn i_098828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9882A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $09882A: clr.b $106ed2.l
pub fn i_09882a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98830;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106ED2) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106ED2, r2 as u8);
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
/// $098BD0: move.w #$1, $70(a6)
pub fn i_098bd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98BD6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $098BD6: move.w #$10, $72(a6)
pub fn i_098bd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98BDC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
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
/// $098C32: jsr $8f010.l
pub fn i_098c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98C38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98C38);
    cpu.pc = 0x8F010;
    16
}
/// $098C38: bra.w $98c5a
pub fn i_098c38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98C3C;
    cpu.ir = 0x6000;
    cpu.pc = 0x98C5A;
    10
}
/// $098C3C: eori.b #$1, $3a(a6)
pub fn i_098c3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98C42;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $098C42: neg.w $28(a6)
pub fn i_098c42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98C46;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $098C46: move.w #$8, d0
pub fn i_098c46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98C4A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $098C4A: btst.b #$0, $3a(a6)
pub fn i_098c4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98C50;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $098C50: bne.w $98c56
pub fn i_098c50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98C54;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x98C56; return 10; }
    8
}
/// $098C54: neg.w d0
pub fn i_098c54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98C56;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $098C56: add.w d0, $22(a6)
pub fn i_098c56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98C5A;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
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
/// $098D96: move.w #$1, $70(a6)
pub fn i_098d96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98D9C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $098D9C: move.w #$3, $72(a6)
pub fn i_098d9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98DA2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
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
/// $098F4E: move.w #$127, d1
pub fn i_098f4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98F52;
    cpu.ir = 0x323C;
    let v1: u32 = 0x127;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $098F52: jsr $236e.l
pub fn i_098f52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98F58;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98F58);
    cpu.pc = 0x236E;
    16
}
/// $098F58: bra.w $98f66
pub fn i_098f58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98F5C;
    cpu.ir = 0x6000;
    cpu.pc = 0x98F66;
    10
}
/// $098F5C: move.w #$128, d1
pub fn i_098f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98F60;
    cpu.ir = 0x323C;
    let v1: u32 = 0x128;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $098F60: jsr $236e.l
pub fn i_098f60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98F66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98F66);
    cpu.pc = 0x236E;
    16
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
/// $098F94: bclr.b #$1, $12(a6)
pub fn i_098f94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98F9A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== SetTaskHandler_098f9a  ($098F9A, 8 B, kernel/task_handlers.c)
/// $098F9A: lea.l $99004(pc), a1
pub fn i_098f9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98F9E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99004;
    16
}
/// $098F9E: move.l a1, (a6)
pub fn i_098f9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98FA0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $098FA0: rts
pub fn i_098fa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98FA2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_Tmpl196_Init_098fa2  ($098FA2, 90 B, frontend/hiscore_memcard_options_09773c.s)
/// $098FA2: move.w #$126, d1
pub fn i_098fa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98FA6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x126;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $098FA6: jsr $236e.l
pub fn i_098fa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98FAC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98FAC);
    cpu.pc = 0x236E;
    16
}
/// $098FAC: bra.w $98fc8
pub fn i_098fac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98FB0;
    cpu.ir = 0x6000;
    cpu.pc = 0x98FC8;
    10
}
/// $098FB0: move.w #$127, d1
pub fn i_098fb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98FB4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x127;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $098FB4: jsr $236e.l
pub fn i_098fb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98FBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98FBA);
    cpu.pc = 0x236E;
    16
}
/// $098FBA: bra.w $98fc8
pub fn i_098fba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98FBE;
    cpu.ir = 0x6000;
    cpu.pc = 0x98FC8;
    10
}
/// $098FBE: move.w #$128, d1
pub fn i_098fbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98FC2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x128;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $098FC2: jsr $236e.l
pub fn i_098fc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98FC8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98FC8);
    cpu.pc = 0x236E;
    16
}
/// $098FC8: jsr $267e2.l
pub fn i_098fc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98FCE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98FCE);
    cpu.pc = 0x267E2;
    16
}
/// $098FCE: jsr $5e7c0.l
pub fn i_098fce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98FD4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98FD4);
    cpu.pc = 0x5E7C0;
    16
}
/// $098FD4: jsr $8f002.l
pub fn i_098fd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98FDA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98FDA);
    cpu.pc = 0x8F002;
    16
}
/// $098FDA: move.w #$0, $70(a6)
pub fn i_098fda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98FE0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $098FE0: move.w #$8000, d0
pub fn i_098fe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98FE4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $098FE4: jsr $28134.l
pub fn i_098fe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98FEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x98FEA);
    cpu.pc = 0x28134;
    16
}
/// $098FEA: andi.w #$ffe3, $38(a6)
pub fn i_098fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98FF0;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $098FF0: ori.w #$4, $38(a6)
pub fn i_098ff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98FF6;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x4) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $098FF6: bclr.b #$1, $12(a6)
pub fn i_098ff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x98FFC;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== SetTaskHandler_098ffc  ($098FFC, 8 B, kernel/task_handlers.c)
/// $098FFC: lea.l $99004(pc), a1
pub fn i_098ffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99000;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99004;
    16
}
/// $099000: move.l a1, (a6)
pub fn i_099000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99002;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $099002: rts
pub fn i_099002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99004;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_Stand_099004  ($099004, 100 B, frontend/hiscore_memcard_options_09773c.s)
/// $099004: jsr $8f010.l
pub fn i_099004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9900A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9900A);
    cpu.pc = 0x8F010;
    16
}
/// $09900A: move.w #$1, $66(a6)
pub fn i_09900a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99010;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $099010: move.w $70(a6), d0
pub fn i_099010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99014;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $099014: movea.l #$2f55ae, a0
pub fn i_099014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9901A;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2F55AE;
    cpu.a[0] = v1;
    4
}
/// $09901A: lsl.w #$2, d0
pub fn i_09901a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9901C;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $09901C: movea.l (a0, d0.w), a0
pub fn i_09901c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99020;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $099020: cmpa.l #$ffffffff, a0
pub fn i_099020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99026;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $099026: beq.w $99030
pub fn i_099026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9902A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x99030; return 10; }
    8
}
/// $09902A: jsr $28cd4.l
pub fn i_09902a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99030;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99030);
    cpu.pc = 0x28CD4;
    16
}
/// $099030: lea.l $99036(pc), a1
pub fn i_099030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99034;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99036;
    16
}
/// $099034: move.l a1, (a6)
pub fn i_099034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99036;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $099036: jsr $27a92.l
pub fn i_099036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9903C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9903C);
    cpu.pc = 0x27A92;
    16
}
/// $09903C: jsr $27eba.l
pub fn i_09903c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99042;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99042);
    cpu.pc = 0x27EBA;
    16
}
/// $099042: bcc.w $9904c
pub fn i_099042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99046;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9904C; return 10; }
    8
}
/// $099046: lea.l $99070(pc), a1
pub fn i_099046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9904A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99070;
    16
}
/// $09904A: move.l a1, (a6)
pub fn i_09904a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9904C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09904C: jsr $28d70.l
pub fn i_09904c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99052;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99052);
    cpu.pc = 0x28D70;
    16
}
/// $099052: movea.l #$ffffffff, a0
pub fn i_099052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99058;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $099058: lea.l $2f61ac.l, a0
pub fn i_099058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9905E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F61AC;
    20
}
/// $09905E: jsr $5dd56.l
pub fn i_09905e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99064;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99064);
    cpu.pc = 0x5DD56;
    16
}
/// $099064: bcc.w $9906e
pub fn i_099064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99068;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9906E; return 10; }
    8
}
// ===== SetTaskHandler_099068  ($099068, 8 B, kernel/task_handlers.c)
/// $099068: lea.l $99794(pc), a1
pub fn i_099068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9906C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99794;
    16
}
/// $09906C: move.l a1, (a6)
pub fn i_09906c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9906E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09906E: rts
pub fn i_09906e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99070;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_Flee2_099070  ($099070, 68 B, frontend/hiscore_memcard_options_09773c.s)
/// $099070: jsr $267e2.l
pub fn i_099070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99076;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99076);
    cpu.pc = 0x267E2;
    16
}
/// $099076: move.w #$ffe0, $2a(a6)
pub fn i_099076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9907C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09907C: lea.l $99082(pc), a1
pub fn i_09907c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99080;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99082;
    16
}
/// $099080: move.l a1, (a6)
pub fn i_099080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99082;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $099082: jsr $27c8c.l
pub fn i_099082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99088;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99088);
    cpu.pc = 0x27C8C;
    16
}
/// $099088: jsr $27eba.l
pub fn i_099088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9908E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9908E);
    cpu.pc = 0x27EBA;
    16
}
/// $09908E: bcs.w $99098
pub fn i_09908e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99092;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x99098; return 10; }
    8
}
/// $099092: lea.l $99004(pc), a1
pub fn i_099092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99096;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99004;
    16
}
/// $099096: move.l a1, (a6)
pub fn i_099096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99098;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $099098: jsr $28d70.l
pub fn i_099098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9909E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9909E);
    cpu.pc = 0x28D70;
    16
}
/// $09909E: movea.l #$ffffffff, a0
pub fn i_09909e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x990A4;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0990A4: lea.l $2f61ac.l, a0
pub fn i_0990a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x990AA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F61AC;
    20
}
/// $0990AA: jsr $5dd56.l
pub fn i_0990aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x990B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x990B0);
    cpu.pc = 0x5DD56;
    16
}
/// $0990B0: bcc.w $990ba
pub fn i_0990b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x990B4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x990BA; return 10; }
    8
}
// ===== SetTaskHandler_0990b4  ($0990B4, 8 B, kernel/task_handlers.c)
/// $0990B4: lea.l $99794(pc), a1
pub fn i_0990b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x990B8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99794;
    16
}
/// $0990B8: move.l a1, (a6)
pub fn i_0990b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x990BA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0990BA: rts
pub fn i_0990ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x990BC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_Tmpl201_Init_0990bc  ($0990BC, 90 B, frontend/hiscore_memcard_options_09773c.s)
/// $0990BC: move.w #$126, d1
pub fn i_0990bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x990C0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x126;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0990C0: jsr $236e.l
pub fn i_0990c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x990C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x990C6);
    cpu.pc = 0x236E;
    16
}
/// $0990C6: bra.w $990e2
pub fn i_0990c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x990CA;
    cpu.ir = 0x6000;
    cpu.pc = 0x990E2;
    10
}
/// $0990CA: move.w #$127, d1
pub fn i_0990ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x990CE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x127;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0990CE: jsr $236e.l
pub fn i_0990ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x990D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x990D4);
    cpu.pc = 0x236E;
    16
}
/// $0990D4: bra.w $990e2
pub fn i_0990d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x990D8;
    cpu.ir = 0x6000;
    cpu.pc = 0x990E2;
    10
}
/// $0990D8: move.w #$128, d1
pub fn i_0990d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x990DC;
    cpu.ir = 0x323C;
    let v1: u32 = 0x128;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0990DC: jsr $236e.l
pub fn i_0990dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x990E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x990E2);
    cpu.pc = 0x236E;
    16
}
/// $0990E2: jsr $267e2.l
pub fn i_0990e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x990E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x990E8);
    cpu.pc = 0x267E2;
    16
}
/// $0990E8: jsr $5e7c0.l
pub fn i_0990e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x990EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x990EE);
    cpu.pc = 0x5E7C0;
    16
}
/// $0990EE: jsr $8f002.l
pub fn i_0990ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x990F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x990F4);
    cpu.pc = 0x8F002;
    16
}
/// $0990F4: move.w #$1, $70(a6)
pub fn i_0990f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x990FA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0990FA: move.w #$8000, d0
pub fn i_0990fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x990FE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0990FE: jsr $28134.l
pub fn i_0990fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99104;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99104);
    cpu.pc = 0x28134;
    16
}
/// $099104: andi.w #$ffe3, $38(a6)
pub fn i_099104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9910A;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09910A: ori.w #$4, $38(a6)
pub fn i_09910a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99110;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x4) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $099110: bclr.b #$1, $12(a6)
pub fn i_099110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99116;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== SetTaskHandler_099116  ($099116, 8 B, kernel/task_handlers.c)
/// $099116: lea.l $99180(pc), a1
pub fn i_099116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9911A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99180;
    16
}
/// $09911A: move.l a1, (a6)
pub fn i_09911a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9911C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09911C: rts
pub fn i_09911c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9911E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_Tmpl202_Init_09911e  ($09911E, 90 B, frontend/hiscore_memcard_options_09773c.s)
/// $09911E: move.w #$126, d1
pub fn i_09911e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99122;
    cpu.ir = 0x323C;
    let v1: u32 = 0x126;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $099122: jsr $236e.l
pub fn i_099122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99128;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99128);
    cpu.pc = 0x236E;
    16
}
/// $099128: bra.w $99144
pub fn i_099128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9912C;
    cpu.ir = 0x6000;
    cpu.pc = 0x99144;
    10
}
/// $09912C: move.w #$127, d1
pub fn i_09912c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99130;
    cpu.ir = 0x323C;
    let v1: u32 = 0x127;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $099130: jsr $236e.l
pub fn i_099130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99136;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99136);
    cpu.pc = 0x236E;
    16
}
/// $099136: bra.w $99144
pub fn i_099136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9913A;
    cpu.ir = 0x6000;
    cpu.pc = 0x99144;
    10
}
/// $09913A: move.w #$128, d1
pub fn i_09913a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9913E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x128;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09913E: jsr $236e.l
pub fn i_09913e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99144;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99144);
    cpu.pc = 0x236E;
    16
}
/// $099144: jsr $267e2.l
pub fn i_099144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9914A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9914A);
    cpu.pc = 0x267E2;
    16
}
/// $09914A: jsr $5e7c0.l
pub fn i_09914a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99150;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99150);
    cpu.pc = 0x5E7C0;
    16
}
/// $099150: jsr $8f002.l
pub fn i_099150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99156;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99156);
    cpu.pc = 0x8F002;
    16
}
/// $099156: move.w #$0, $70(a6)
pub fn i_099156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9915C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09915C: move.w #$8000, d0
pub fn i_09915c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99160;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $099160: jsr $28134.l
pub fn i_099160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99166;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99166);
    cpu.pc = 0x28134;
    16
}
/// $099166: andi.w #$ffe3, $38(a6)
pub fn i_099166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9916C;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09916C: ori.w #$4, $38(a6)
pub fn i_09916c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99172;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x4) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $099172: bclr.b #$1, $12(a6)
pub fn i_099172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99178;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== SetTaskHandler_099178  ($099178, 8 B, kernel/task_handlers.c)
/// $099178: lea.l $99180(pc), a1
pub fn i_099178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9917C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99180;
    16
}
/// $09917C: move.l a1, (a6)
pub fn i_09917c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9917E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09917E: rts
pub fn i_09917e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99180;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_Appear_099180  ($099180, 82 B, frontend/hiscore_memcard_options_09773c.s)
/// $099180: move.w $70(a6), d0
pub fn i_099180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99184;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $099184: movea.l #$2f55b6, a0
pub fn i_099184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9918A;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2F55B6;
    cpu.a[0] = v1;
    4
}
/// $09918A: lsl.w #$2, d0
pub fn i_09918a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9918C;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $09918C: movea.l (a0, d0.w), a0
pub fn i_09918c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99190;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $099190: cmpa.l #$ffffffff, a0
pub fn i_099190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99196;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $099196: beq.w $991a0
pub fn i_099196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9919A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x991A0; return 10; }
    8
}
/// $09919A: jsr $28cd4.l
pub fn i_09919a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x991A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x991A0);
    cpu.pc = 0x28CD4;
    16
}
/// $0991A0: lea.l $991a6(pc), a1
pub fn i_0991a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x991A4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x991A6;
    16
}
/// $0991A4: move.l a1, (a6)
pub fn i_0991a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x991A6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0991A6: jsr $2783a.l
pub fn i_0991a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x991AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x991AC);
    cpu.pc = 0x2783A;
    16
}
/// $0991AC: jsr $28d70.l
pub fn i_0991ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x991B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x991B2);
    cpu.pc = 0x28D70;
    16
}
/// $0991B2: bcc.w $991bc
pub fn i_0991b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x991B6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x991BC; return 10; }
    8
}
/// $0991B6: lea.l $99004(pc), a1
pub fn i_0991b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x991BA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99004;
    16
}
/// $0991BA: move.l a1, (a6)
pub fn i_0991ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x991BC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0991BC: movea.l #$ffffffff, a0
pub fn i_0991bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x991C2;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0991C2: lea.l $2f61ac.l, a0
pub fn i_0991c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x991C8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F61AC;
    20
}
/// $0991C8: jsr $5dd56.l
pub fn i_0991c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x991CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x991CE);
    cpu.pc = 0x5DD56;
    16
}
/// $0991CE: bcc.w $991d8
pub fn i_0991ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x991D2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x991D8; return 10; }
    8
}
// ===== SetTaskHandler_0991d2  ($0991D2, 8 B, kernel/task_handlers.c)
/// $0991D2: lea.l $99794(pc), a1
pub fn i_0991d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x991D6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99794;
    16
}
/// $0991D6: move.l a1, (a6)
pub fn i_0991d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x991D8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0991D8: rts
pub fn i_0991d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x991DA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_Tmpl207_Init_0991da  ($0991DA, 56 B, frontend/hiscore_memcard_options_09773c.s)
/// $0991DA: move.w #$125, d1
pub fn i_0991da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x991DE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x125;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0991DE: jsr $236e.l
pub fn i_0991de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x991E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x991E4);
    cpu.pc = 0x236E;
    16
}
/// $0991E4: jsr $5e7c0.l
pub fn i_0991e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x991EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x991EA);
    cpu.pc = 0x5E7C0;
    16
}
/// $0991EA: jsr $267e2.l
pub fn i_0991ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x991F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x991F0);
    cpu.pc = 0x267E2;
    16
}
/// $0991F0: jsr $8f002.l
pub fn i_0991f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x991F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x991F6);
    cpu.pc = 0x8F002;
    16
}
/// $0991F6: move.w #$0, d0
pub fn i_0991f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x991FA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0991FA: jsr $28134.l
pub fn i_0991fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99200;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99200);
    cpu.pc = 0x28134;
    16
}
/// $099200: andi.w #$ffe3, $38(a6)
pub fn i_099200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99206;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $099206: ori.w #$4, $38(a6)
pub fn i_099206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9920C;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x4) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09920C: bclr.b #$1, $12(a6)
pub fn i_09920c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99212;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== SetTaskHandler_099212  ($099212, 8 B, kernel/task_handlers.c)
/// $099212: lea.l $9921a(pc), a1
pub fn i_099212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99216;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9921A;
    16
}
/// $099216: move.l a1, (a6)
pub fn i_099216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99218;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $099218: rts
pub fn i_099218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9921A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_Sit_09921a  ($09921A, 102 B, frontend/hiscore_memcard_options_09773c.s)
/// $09921A: move.w #$3c, $74(a6)
pub fn i_09921a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99220;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $099220: move.w #$1, $66(a6)
pub fn i_099220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99226;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $099226: lea.l $2f5ed8.l, a0
pub fn i_099226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9922C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F5ED8;
    20
}
/// $09922C: jsr $28cd4.l
pub fn i_09922c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99232;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99232);
    cpu.pc = 0x28CD4;
    16
}
/// $099232: lea.l $99238(pc), a1
pub fn i_099232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99236;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99238;
    16
}
/// $099236: move.l a1, (a6)
pub fn i_099236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99238;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $099238: jsr $2783a.l
pub fn i_099238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9923E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9923E);
    cpu.pc = 0x2783A;
    16
}
/// $09923E: jsr $28d70.l
pub fn i_09923e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99244;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99244);
    cpu.pc = 0x28D70;
    16
}
/// $099244: tst.w $74(a6)
pub fn i_099244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99248;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $099248: beq.w $99254
pub fn i_099248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9924C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x99254; return 10; }
    8
}
/// $09924C: subq.w #$1, $74(a6)
pub fn i_09924c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99250;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $099250: bne.w $99264
pub fn i_099250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99254;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x99264; return 10; }
    8
}
/// $099254: jsr $2870a.l
pub fn i_099254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9925A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9925A);
    cpu.pc = 0x2870A;
    16
}
/// $09925A: bcc.w $99264
pub fn i_09925a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9925E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x99264; return 10; }
    8
}
/// $09925E: lea.l $99288(pc), a1
pub fn i_09925e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99262;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99288;
    16
}
/// $099262: move.l a1, (a6)
pub fn i_099262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99264;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $099264: bclr.b #$3, $13(a6)
pub fn i_099264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9926A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $09926A: movea.l #$ffffffff, a0
pub fn i_09926a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99270;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $099270: lea.l $2f61ac.l, a0
pub fn i_099270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99276;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F61AC;
    20
}
/// $099276: jsr $5dd56.l
pub fn i_099276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9927C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9927C);
    cpu.pc = 0x5DD56;
    16
}
/// $09927C: bcc.w $99286
pub fn i_09927c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99280;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x99286; return 10; }
    8
}
// ===== SetTaskHandler_099280  ($099280, 8 B, kernel/task_handlers.c)
/// $099280: lea.l $99794(pc), a1
pub fn i_099280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99284;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99794;
    16
}
/// $099284: move.l a1, (a6)
pub fn i_099284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99286;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $099286: rts
pub fn i_099286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99288;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_Sit_Hop_099288  ($099288, 72 B, frontend/hiscore_memcard_options_09773c.s)
/// $099288: move.w #$28, $74(a6)
pub fn i_099288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9928E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09928E: lea.l $2f5ef8.l, a0
pub fn i_09928e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99294;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F5EF8;
    20
}
/// $099294: jsr $28cd4.l
pub fn i_099294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9929A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9929A);
    cpu.pc = 0x28CD4;
    16
}
/// $09929A: lea.l $992a0(pc), a1
pub fn i_09929a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9929E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x992A0;
    16
}
/// $09929E: move.l a1, (a6)
pub fn i_09929e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x992A0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0992A0: jsr $2783a.l
pub fn i_0992a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x992A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x992A6);
    cpu.pc = 0x2783A;
    16
}
/// $0992A6: jsr $28d70.l
pub fn i_0992a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x992AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x992AC);
    cpu.pc = 0x28D70;
    16
}
/// $0992AC: subq.w #$1, $74(a6)
pub fn i_0992ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x992B0;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0992B0: bne.w $992ba
pub fn i_0992b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x992B4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x992BA; return 10; }
    8
}
/// $0992B4: lea.l $9921a(pc), a1
pub fn i_0992b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x992B8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9921A;
    16
}
/// $0992B8: move.l a1, (a6)
pub fn i_0992b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x992BA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0992BA: movea.l #$ffffffff, a0
pub fn i_0992ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x992C0;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0992C0: lea.l $2f61ac.l, a0
pub fn i_0992c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x992C6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F61AC;
    20
}
/// $0992C6: jsr $5dd56.l
pub fn i_0992c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x992CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x992CC);
    cpu.pc = 0x5DD56;
    16
}
/// $0992CC: bcc.w $992d6
pub fn i_0992cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x992D0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x992D6; return 10; }
    8
}
// ===== SetTaskHandler_0992d0  ($0992D0, 8 B, kernel/task_handlers.c)
/// $0992D0: lea.l $99794(pc), a1
pub fn i_0992d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x992D4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99794;
    16
}
/// $0992D4: move.l a1, (a6)
pub fn i_0992d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x992D6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0992D6: rts
pub fn i_0992d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x992D8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_Tmpl208_Init_0992d8  ($0992D8, 102 B, frontend/hiscore_memcard_options_09773c.s)
/// $0992D8: move.w #$126, d1
pub fn i_0992d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x992DC;
    cpu.ir = 0x323C;
    let v1: u32 = 0x126;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0992DC: jsr $236e.l
pub fn i_0992dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x992E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x992E2);
    cpu.pc = 0x236E;
    16
}
/// $0992E2: bra.w $992fe
pub fn i_0992e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x992E6;
    cpu.ir = 0x6000;
    cpu.pc = 0x992FE;
    10
}
/// $0992E6: move.w #$127, d1
pub fn i_0992e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x992EA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x127;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0992EA: jsr $236e.l
pub fn i_0992ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x992F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x992F0);
    cpu.pc = 0x236E;
    16
}
/// $0992F0: bra.w $992fe
pub fn i_0992f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x992F4;
    cpu.ir = 0x6000;
    cpu.pc = 0x992FE;
    10
}
/// $0992F4: move.w #$128, d1
pub fn i_0992f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x992F8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x128;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0992F8: jsr $236e.l
pub fn i_0992f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x992FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x992FE);
    cpu.pc = 0x236E;
    16
}
/// $0992FE: jsr $267e2.l
pub fn i_0992fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99304;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99304);
    cpu.pc = 0x267E2;
    16
}
/// $099304: jsr $5e7c0.l
pub fn i_099304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9930A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9930A);
    cpu.pc = 0x5E7C0;
    16
}
/// $09930A: jsr $8f002.l
pub fn i_09930a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99310;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99310);
    cpu.pc = 0x8F002;
    16
}
/// $099310: move.w #$8000, d0
pub fn i_099310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99314;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $099314: jsr $28134.l
pub fn i_099314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9931A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9931A);
    cpu.pc = 0x28134;
    16
}
/// $09931A: andi.w #$ffe3, $38(a6)
pub fn i_09931a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99320;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $099320: ori.w #$4, $38(a6)
pub fn i_099320(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99326;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x4) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $099326: move.w #$1, $66(a6)
pub fn i_099326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9932C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09932C: lea.l $2f5f18.l, a0
pub fn i_09932c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99332;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F5F18;
    20
}
/// $099332: jsr $28cd4.l
pub fn i_099332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99338;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99338);
    cpu.pc = 0x28CD4;
    16
}
/// $099338: bclr.b #$1, $12(a6)
pub fn i_099338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9933E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== SetTaskHandler_09933e  ($09933E, 8 B, kernel/task_handlers.c)
/// $09933E: lea.l $9976a(pc), a1
pub fn i_09933e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99342;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9976A;
    16
}
/// $099342: move.l a1, (a6)
pub fn i_099342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99344;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $099344: rts
pub fn i_099344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99346;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_Tmpl209_Init_099346  ($099346, 84 B, frontend/hiscore_memcard_options_09773c.s)
/// $099346: move.w #$126, d1
pub fn i_099346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9934A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x126;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09934A: jsr $236e.l
pub fn i_09934a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99350;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99350);
    cpu.pc = 0x236E;
    16
}
/// $099350: bra.w $9936c
pub fn i_099350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99354;
    cpu.ir = 0x6000;
    cpu.pc = 0x9936C;
    10
}
/// $099354: move.w #$127, d1
pub fn i_099354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99358;
    cpu.ir = 0x323C;
    let v1: u32 = 0x127;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $099358: jsr $236e.l
pub fn i_099358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9935E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9935E);
    cpu.pc = 0x236E;
    16
}
/// $09935E: bra.w $9936c
pub fn i_09935e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99362;
    cpu.ir = 0x6000;
    cpu.pc = 0x9936C;
    10
}
/// $099362: move.w #$128, d1
pub fn i_099362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99366;
    cpu.ir = 0x323C;
    let v1: u32 = 0x128;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $099366: jsr $236e.l
pub fn i_099366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9936C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9936C);
    cpu.pc = 0x236E;
    16
}
/// $09936C: lea.l $2f5f38.l, a0
pub fn i_09936c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99372;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F5F38;
    20
}
/// $099372: jsr $28cd4.l
pub fn i_099372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99378;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99378);
    cpu.pc = 0x28CD4;
    16
}
/// $099378: move.w #$8000, d0
pub fn i_099378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9937C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09937C: jsr $28134.l
pub fn i_09937c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99382;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99382);
    cpu.pc = 0x28134;
    16
}
/// $099382: andi.w #$ffe3, $38(a6)
pub fn i_099382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99388;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $099388: ori.w #$4, $38(a6)
pub fn i_099388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9938E;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x4) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09938E: jsr $5e7c0.l
pub fn i_09938e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99394;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99394);
    cpu.pc = 0x5E7C0;
    16
}
/// $099394: bclr.b #$1, $12(a6)
pub fn i_099394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9939A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== SetTaskHandler_09939a  ($09939A, 8 B, kernel/task_handlers.c)
/// $09939A: lea.l $993a2(pc), a1
pub fn i_09939a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9939E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x993A2;
    16
}
/// $09939E: move.l a1, (a6)
pub fn i_09939e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x993A0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0993A0: rts
pub fn i_0993a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x993A2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_Tmpl209_Body_0993a2  ($0993A2, 80 B, frontend/hiscore_memcard_options_09773c.s)
/// $0993A2: jsr $267e2.l
pub fn i_0993a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x993A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x993A8);
    cpu.pc = 0x267E2;
    16
}
/// $0993A8: jsr $8f002.l
pub fn i_0993a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x993AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x993AE);
    cpu.pc = 0x8F002;
    16
}
/// $0993AE: jsr $8f010.l
pub fn i_0993ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x993B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x993B4);
    cpu.pc = 0x8F010;
    16
}
/// $0993B4: move.w #$1, $66(a6)
pub fn i_0993b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x993BA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0993BA: lea.l $993c0(pc), a1
pub fn i_0993ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x993BE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x993C0;
    16
}
/// $0993BE: move.l a1, (a6)
pub fn i_0993be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x993C0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0993C0: jsr $27a92.l
pub fn i_0993c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x993C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x993C6);
    cpu.pc = 0x27A92;
    16
}
/// $0993C6: jsr $27eba.l
pub fn i_0993c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x993CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x993CC);
    cpu.pc = 0x27EBA;
    16
}
/// $0993CC: bcc.w $993d6
pub fn i_0993cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x993D0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x993D6; return 10; }
    8
}
/// $0993D0: lea.l $9936c(pc), a1
pub fn i_0993d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x993D4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9936C;
    16
}
/// $0993D4: move.l a1, (a6)
pub fn i_0993d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x993D6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0993D6: jsr $28d70.l
pub fn i_0993d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x993DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x993DC);
    cpu.pc = 0x28D70;
    16
}
/// $0993DC: movea.l #$ffffffff, a0
pub fn i_0993dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x993E2;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0993E2: lea.l $2f61ac.l, a0
pub fn i_0993e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x993E8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F61AC;
    20
}
/// $0993E8: jsr $5dd56.l
pub fn i_0993e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x993EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x993EE);
    cpu.pc = 0x5DD56;
    16
}
/// $0993EE: bcc.w $993f8
pub fn i_0993ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x993F2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x993F8; return 10; }
    8
}
// ===== SetTaskHandler_0993f2  ($0993F2, 8 B, kernel/task_handlers.c)
/// $0993F2: lea.l $99794(pc), a1
pub fn i_0993f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x993F6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99794;
    16
}
/// $0993F6: move.l a1, (a6)
pub fn i_0993f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x993F8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0993F8: rts
pub fn i_0993f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x993FA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_Flee3_0993fa  ($0993FA, 68 B, frontend/hiscore_memcard_options_09773c.s)
/// $0993FA: jsr $267e2.l
pub fn i_0993fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99400;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99400);
    cpu.pc = 0x267E2;
    16
}
/// $099400: move.w #$ffe0, $2a(a6)
pub fn i_099400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99406;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $099406: lea.l $9940c(pc), a1
pub fn i_099406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9940A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9940C;
    16
}
/// $09940A: move.l a1, (a6)
pub fn i_09940a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9940C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09940C: jsr $27c8c.l
pub fn i_09940c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99412;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99412);
    cpu.pc = 0x27C8C;
    16
}
/// $099412: jsr $27eba.l
pub fn i_099412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99418;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99418);
    cpu.pc = 0x27EBA;
    16
}
/// $099418: bcs.w $99422
pub fn i_099418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9941C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x99422; return 10; }
    8
}
/// $09941C: lea.l $9936c(pc), a1
pub fn i_09941c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99420;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9936C;
    16
}
/// $099420: move.l a1, (a6)
pub fn i_099420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99422;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $099422: jsr $28d70.l
pub fn i_099422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99428;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99428);
    cpu.pc = 0x28D70;
    16
}
/// $099428: movea.l #$ffffffff, a0
pub fn i_099428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9942E;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $09942E: lea.l $2f61ac.l, a0
pub fn i_09942e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99434;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F61AC;
    20
}
/// $099434: jsr $5dd56.l
pub fn i_099434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9943A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9943A);
    cpu.pc = 0x5DD56;
    16
}
/// $09943A: bcc.w $99444
pub fn i_09943a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9943E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x99444; return 10; }
    8
}
// ===== SetTaskHandler_09943e  ($09943E, 8 B, kernel/task_handlers.c)
/// $09943E: lea.l $99794(pc), a1
pub fn i_09943e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99442;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99794;
    16
}
/// $099442: move.l a1, (a6)
pub fn i_099442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99444;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $099444: rts
pub fn i_099444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99446;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_Tmpl210_Init_099446  ($099446, 84 B, frontend/hiscore_memcard_options_09773c.s)
/// $099446: move.w #$126, d1
pub fn i_099446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9944A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x126;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09944A: jsr $236e.l
pub fn i_09944a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99450;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99450);
    cpu.pc = 0x236E;
    16
}
/// $099450: bra.w $9946c
pub fn i_099450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99454;
    cpu.ir = 0x6000;
    cpu.pc = 0x9946C;
    10
}
/// $099454: move.w #$127, d1
pub fn i_099454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99458;
    cpu.ir = 0x323C;
    let v1: u32 = 0x127;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $099458: jsr $236e.l
pub fn i_099458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9945E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9945E);
    cpu.pc = 0x236E;
    16
}
/// $09945E: bra.w $9946c
pub fn i_09945e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99462;
    cpu.ir = 0x6000;
    cpu.pc = 0x9946C;
    10
}
/// $099462: move.w #$128, d1
pub fn i_099462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99466;
    cpu.ir = 0x323C;
    let v1: u32 = 0x128;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $099466: jsr $236e.l
pub fn i_099466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9946C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9946C);
    cpu.pc = 0x236E;
    16
}
/// $09946C: jsr $5e7c0.l
pub fn i_09946c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99472;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99472);
    cpu.pc = 0x5E7C0;
    16
}
/// $099472: move.w #$8000, d0
pub fn i_099472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99476;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $099476: jsr $28134.l
pub fn i_099476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9947C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9947C);
    cpu.pc = 0x28134;
    16
}
/// $09947C: andi.w #$ffe3, $38(a6)
pub fn i_09947c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99482;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $099482: ori.w #$4, $38(a6)
pub fn i_099482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99488;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x4) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $099488: lea.l $2f5f80.l, a0
pub fn i_099488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9948E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F5F80;
    20
}
/// $09948E: jsr $28cd4.l
pub fn i_09948e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99494;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99494);
    cpu.pc = 0x28CD4;
    16
}
/// $099494: bclr.b #$1, $12(a6)
pub fn i_099494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9949A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== SetTaskHandler_09949a  ($09949A, 8 B, kernel/task_handlers.c)
/// $09949A: lea.l $993a2(pc), a1
pub fn i_09949a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9949E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x993A2;
    16
}
/// $09949E: move.l a1, (a6)
pub fn i_09949e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x994A0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0994A0: rts
pub fn i_0994a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x994A2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_Tmpl211_Init_0994a2  ($0994A2, 84 B, frontend/hiscore_memcard_options_09773c.s)
/// $0994A2: move.w #$126, d1
pub fn i_0994a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x994A6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x126;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0994A6: jsr $236e.l
pub fn i_0994a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x994AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x994AC);
    cpu.pc = 0x236E;
    16
}
/// $0994AC: bra.w $994c8
pub fn i_0994ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x994B0;
    cpu.ir = 0x6000;
    cpu.pc = 0x994C8;
    10
}
/// $0994B0: move.w #$127, d1
pub fn i_0994b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x994B4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x127;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0994B4: jsr $236e.l
pub fn i_0994b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x994BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x994BA);
    cpu.pc = 0x236E;
    16
}
/// $0994BA: bra.w $994c8
pub fn i_0994ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x994BE;
    cpu.ir = 0x6000;
    cpu.pc = 0x994C8;
    10
}
/// $0994BE: move.w #$128, d1
pub fn i_0994be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x994C2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x128;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0994C2: jsr $236e.l
pub fn i_0994c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x994C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x994C8);
    cpu.pc = 0x236E;
    16
}
/// $0994C8: jsr $5e7c0.l
pub fn i_0994c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x994CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x994CE);
    cpu.pc = 0x5E7C0;
    16
}
/// $0994CE: move.w #$8000, d0
pub fn i_0994ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x994D2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0994D2: jsr $28134.l
pub fn i_0994d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x994D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x994D8);
    cpu.pc = 0x28134;
    16
}
/// $0994D8: andi.w #$ffe3, $38(a6)
pub fn i_0994d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x994DE;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0994DE: ori.w #$4, $38(a6)
pub fn i_0994de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x994E4;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x4) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0994E4: lea.l $2f5fc8.l, a0
pub fn i_0994e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x994EA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F5FC8;
    20
}
/// $0994EA: jsr $28cd4.l
pub fn i_0994ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x994F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x994F0);
    cpu.pc = 0x28CD4;
    16
}
/// $0994F0: bclr.b #$1, $12(a6)
pub fn i_0994f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x994F6;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== SetTaskHandler_0994f6  ($0994F6, 8 B, kernel/task_handlers.c)
/// $0994F6: lea.l $993a2(pc), a1
pub fn i_0994f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x994FA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x993A2;
    16
}
/// $0994FA: move.l a1, (a6)
pub fn i_0994fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x994FC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0994FC: rts
pub fn i_0994fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x994FE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_Tmpl220_Init_0994fe  ($0994FE, 56 B, frontend/hiscore_memcard_options_09773c.s)
/// $0994FE: move.w #$125, d1
pub fn i_0994fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99502;
    cpu.ir = 0x323C;
    let v1: u32 = 0x125;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $099502: jsr $236e.l
pub fn i_099502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99508;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99508);
    cpu.pc = 0x236E;
    16
}
/// $099508: jsr $267e2.l
pub fn i_099508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9950E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9950E);
    cpu.pc = 0x267E2;
    16
}
/// $09950E: jsr $5e7c0.l
pub fn i_09950e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99514;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99514);
    cpu.pc = 0x5E7C0;
    16
}
/// $099514: jsr $8f002.l
pub fn i_099514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9951A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9951A);
    cpu.pc = 0x8F002;
    16
}
/// $09951A: move.w #$8000, d0
pub fn i_09951a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9951E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09951E: jsr $28134.l
pub fn i_09951e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99524;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99524);
    cpu.pc = 0x28134;
    16
}
/// $099524: andi.w #$ffe3, $38(a6)
pub fn i_099524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9952A;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09952A: ori.w #$4, $38(a6)
pub fn i_09952a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99530;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x4) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $099530: bclr.b #$1, $12(a6)
pub fn i_099530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99536;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== SetTaskHandler_099536  ($099536, 8 B, kernel/task_handlers.c)
/// $099536: lea.l $9953e(pc), a1
pub fn i_099536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9953A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9953E;
    16
}
/// $09953A: move.l a1, (a6)
pub fn i_09953a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9953C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09953C: rts
pub fn i_09953c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9953E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_RunLeft_09953e  ($09953E, 74 B, frontend/hiscore_memcard_options_09773c.s)
/// $09953E: move.w #$1, $66(a6)
pub fn i_09953e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99544;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $099544: lea.l $2f6010.l, a0
pub fn i_099544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9954A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F6010;
    20
}
/// $09954A: jsr $28cd4.l
pub fn i_09954a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99550;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99550);
    cpu.pc = 0x28CD4;
    16
}
/// $099550: lea.l $99556(pc), a1
pub fn i_099550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99554;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99556;
    16
}
/// $099554: move.l a1, (a6)
pub fn i_099554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99556;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $099556: jsr $2783a.l
pub fn i_099556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9955C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9955C);
    cpu.pc = 0x2783A;
    16
}
/// $09955C: jsr $28d70.l
pub fn i_09955c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99562;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99562);
    cpu.pc = 0x28D70;
    16
}
/// $099562: cmpi.w #$b0, $22(a6)
pub fn i_099562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99568;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xB0, Size::Word, false);
    16
}
/// $099568: bgt.w $99572
pub fn i_099568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9956C;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x99572; return 10; }
    8
}
/// $09956C: lea.l $99590(pc), a1
pub fn i_09956c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99570;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99590;
    16
}
/// $099570: move.l a1, (a6)
pub fn i_099570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99572;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $099572: movea.l #$ffffffff, a0
pub fn i_099572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99578;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $099578: lea.l $2f61ac.l, a0
pub fn i_099578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9957E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F61AC;
    20
}
/// $09957E: jsr $5dd56.l
pub fn i_09957e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99584;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99584);
    cpu.pc = 0x5DD56;
    16
}
/// $099584: bcc.w $9958e
pub fn i_099584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99588;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9958E; return 10; }
    8
}
// ===== SetTaskHandler_099588  ($099588, 8 B, kernel/task_handlers.c)
/// $099588: lea.l $99794(pc), a1
pub fn i_099588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9958C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99794;
    16
}
/// $09958C: move.l a1, (a6)
pub fn i_09958c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9958E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09958E: rts
pub fn i_09958e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99590;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_RunLeft_Stop_099590  ($099590, 52 B, frontend/hiscore_memcard_options_09773c.s)
/// $099590: lea.l $2f6022.l, a0
pub fn i_099590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99596;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F6022;
    20
}
/// $099596: jsr $28cd4.l
pub fn i_099596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9959C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9959C);
    cpu.pc = 0x28CD4;
    16
}
/// $09959C: lea.l $995a2(pc), a1
pub fn i_09959c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x995A0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x995A2;
    16
}
/// $0995A0: move.l a1, (a6)
pub fn i_0995a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x995A2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0995A2: jsr $2783a.l
pub fn i_0995a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x995A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x995A8);
    cpu.pc = 0x2783A;
    16
}
/// $0995A8: jsr $28d70.l
pub fn i_0995a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x995AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x995AE);
    cpu.pc = 0x28D70;
    16
}
/// $0995AE: movea.l #$ffffffff, a0
pub fn i_0995ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x995B4;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0995B4: lea.l $2f61ac.l, a0
pub fn i_0995b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x995BA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F61AC;
    20
}
/// $0995BA: jsr $5dd56.l
pub fn i_0995ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x995C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x995C0);
    cpu.pc = 0x5DD56;
    16
}
/// $0995C0: bcc.w $995ca
pub fn i_0995c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x995C4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x995CA; return 10; }
    8
}
// ===== SetTaskHandler_0995c4  ($0995C4, 8 B, kernel/task_handlers.c)
/// $0995C4: lea.l $99794(pc), a1
pub fn i_0995c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x995C8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99794;
    16
}
/// $0995C8: move.l a1, (a6)
pub fn i_0995c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x995CA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0995CA: rts
pub fn i_0995ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x995CC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_Tmpl221_Init_0995cc  ($0995CC, 60 B, frontend/hiscore_memcard_options_09773c.s)
/// $0995CC: move.w #$127, d1
pub fn i_0995cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x995D0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x127;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0995D0: jsr $236e.l
pub fn i_0995d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x995D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x995D6);
    cpu.pc = 0x236E;
    16
}
/// $0995D6: jsr $267e2.l
pub fn i_0995d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x995DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x995DC);
    cpu.pc = 0x267E2;
    16
}
/// $0995DC: jsr $5e7c0.l
pub fn i_0995dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x995E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x995E2);
    cpu.pc = 0x5E7C0;
    16
}
/// $0995E2: jsr $8f002.l
pub fn i_0995e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x995E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x995E8);
    cpu.pc = 0x8F002;
    16
}
/// $0995E8: clr.b $21(a6)
pub fn i_0995e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x995EC;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0995EC: move.w #$8000, d0
pub fn i_0995ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x995F0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0995F0: jsr $28134.l
pub fn i_0995f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x995F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x995F6);
    cpu.pc = 0x28134;
    16
}
/// $0995F6: andi.w #$ffe3, $38(a6)
pub fn i_0995f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x995FC;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0995FC: ori.w #$4, $38(a6)
pub fn i_0995fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99602;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x4) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $099602: bclr.b #$1, $12(a6)
pub fn i_099602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99608;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== SetTaskHandler_099608  ($099608, 8 B, kernel/task_handlers.c)
/// $099608: lea.l $99610(pc), a1
pub fn i_099608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9960C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99610;
    16
}
/// $09960C: move.l a1, (a6)
pub fn i_09960c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9960E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09960E: rts
pub fn i_09960e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99610;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_RunLeft2_099610  ($099610, 88 B, frontend/hiscore_memcard_options_09773c.s)
/// $099610: move.w #$1, $66(a6)
pub fn i_099610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99616;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $099616: lea.l $2f6048.l, a0
pub fn i_099616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9961C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F6048;
    20
}
/// $09961C: jsr $28cd4.l
pub fn i_09961c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99622;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99622);
    cpu.pc = 0x28CD4;
    16
}
/// $099622: lea.l $99628(pc), a1
pub fn i_099622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99626;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99628;
    16
}
/// $099626: move.l a1, (a6)
pub fn i_099626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99628;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $099628: jsr $2783a.l
pub fn i_099628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9962E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9962E);
    cpu.pc = 0x2783A;
    16
}
/// $09962E: jsr $28d70.l
pub fn i_09962e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99634;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99634);
    cpu.pc = 0x28D70;
    16
}
/// $099634: tst.b $21(a6)
pub fn i_099634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99638;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $099638: bne.w $99652
pub fn i_099638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9963C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x99652; return 10; }
    8
}
/// $09963C: cmpi.w #$b0, $22(a6)
pub fn i_09963c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99642;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xB0, Size::Word, false);
    16
}
/// $099642: bgt.w $99652
pub fn i_099642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99646;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x99652; return 10; }
    8
}
/// $099646: move.b #$ff, $21(a6)
pub fn i_099646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9964C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09964C: lea.l $99670(pc), a1
pub fn i_09964c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99650;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99670;
    16
}
/// $099650: move.l a1, (a6)
pub fn i_099650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99652;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $099652: movea.l #$ffffffff, a0
pub fn i_099652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99658;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $099658: lea.l $2f61ac.l, a0
pub fn i_099658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9965E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F61AC;
    20
}
/// $09965E: jsr $5dd56.l
pub fn i_09965e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99664;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99664);
    cpu.pc = 0x5DD56;
    16
}
/// $099664: bcc.w $9966e
pub fn i_099664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99668;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9966E; return 10; }
    8
}
// ===== SetTaskHandler_099668  ($099668, 8 B, kernel/task_handlers.c)
/// $099668: lea.l $99794(pc), a1
pub fn i_099668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9966C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99794;
    16
}
/// $09966C: move.l a1, (a6)
pub fn i_09966c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9966E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09966E: rts
pub fn i_09966e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99670;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_RunLeft2_Wait_099670  ($099670, 62 B, frontend/hiscore_memcard_options_09773c.s)
/// $099670: lea.l $2f6068.l, a0
pub fn i_099670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99676;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F6068;
    20
}
/// $099676: jsr $28cd4.l
pub fn i_099676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9967C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9967C);
    cpu.pc = 0x28CD4;
    16
}
/// $09967C: lea.l $99682(pc), a1
pub fn i_09967c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99680;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99682;
    16
}
/// $099680: move.l a1, (a6)
pub fn i_099680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99682;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $099682: jsr $2783a.l
pub fn i_099682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99688;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99688);
    cpu.pc = 0x2783A;
    16
}
/// $099688: jsr $28d70.l
pub fn i_099688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9968E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9968E);
    cpu.pc = 0x28D70;
    16
}
/// $09968E: bcc.w $99698
pub fn i_09968e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99692;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x99698; return 10; }
    8
}
/// $099692: lea.l $99610(pc), a1
pub fn i_099692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99696;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99610;
    16
}
/// $099696: move.l a1, (a6)
pub fn i_099696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99698;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $099698: movea.l #$ffffffff, a0
pub fn i_099698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9969E;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $09969E: lea.l $2f61ac.l, a0
pub fn i_09969e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x996A4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F61AC;
    20
}
/// $0996A4: jsr $5dd56.l
pub fn i_0996a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x996AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x996AA);
    cpu.pc = 0x5DD56;
    16
}
/// $0996AA: bcc.w $996b4
pub fn i_0996aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x996AE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x996B4; return 10; }
    8
}
// ===== SetTaskHandler_0996ae  ($0996AE, 8 B, kernel/task_handlers.c)
/// $0996AE: lea.l $99794(pc), a1
pub fn i_0996ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x996B2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99794;
    16
}
/// $0996B2: move.l a1, (a6)
pub fn i_0996b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x996B4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0996B4: rts
pub fn i_0996b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x996B6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_Tmpl222_Init_0996b6  ($0996B6, 172 B, frontend/hiscore_memcard_options_09773c.s)
/// $0996B6: move.w #$126, d1
pub fn i_0996b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x996BA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x126;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0996BA: jsr $236e.l
pub fn i_0996ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x996C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x996C0);
    cpu.pc = 0x236E;
    16
}
/// $0996C0: lea.l $2f607a.l, a0
pub fn i_0996c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x996C6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F607A;
    20
}
/// $0996C6: jsr $28cd4.l
pub fn i_0996c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x996CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x996CC);
    cpu.pc = 0x28CD4;
    16
}
/// $0996CC: bra.w $99734
pub fn i_0996cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x996D0;
    cpu.ir = 0x6000;
    cpu.pc = 0x99734;
    10
}
/// $0996D0: move.w #$127, d1
pub fn i_0996d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x996D4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x127;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0996D4: jsr $236e.l
pub fn i_0996d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x996DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x996DA);
    cpu.pc = 0x236E;
    16
}
/// $0996DA: lea.l $2f607a.l, a0
pub fn i_0996da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x996E0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F607A;
    20
}
/// $0996E0: jsr $28cd4.l
pub fn i_0996e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x996E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x996E6);
    cpu.pc = 0x28CD4;
    16
}
/// $0996E6: bra.w $99734
pub fn i_0996e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x996EA;
    cpu.ir = 0x6000;
    cpu.pc = 0x99734;
    10
}
/// $0996EA: move.w #$128, d1
pub fn i_0996ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x996EE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x128;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0996EE: jsr $236e.l
pub fn i_0996ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x996F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x996F4);
    cpu.pc = 0x236E;
    16
}
/// $0996F4: lea.l $2f607a.l, a0
pub fn i_0996f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x996FA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F607A;
    20
}
/// $0996FA: jsr $28cd4.l
pub fn i_0996fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99700;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99700);
    cpu.pc = 0x28CD4;
    16
}
/// $099700: bra.w $99734
pub fn i_099700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99704;
    cpu.ir = 0x6000;
    cpu.pc = 0x99734;
    10
}
/// $099704: move.w #$129, d1
pub fn i_099704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99708;
    cpu.ir = 0x323C;
    let v1: u32 = 0x129;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $099708: jsr $236e.l
pub fn i_099708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9970E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9970E);
    cpu.pc = 0x236E;
    16
}
/// $09970E: lea.l $2f608c.l, a0
pub fn i_09970e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99714;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F608C;
    20
}
/// $099714: jsr $28cd4.l
pub fn i_099714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9971A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9971A);
    cpu.pc = 0x28CD4;
    16
}
/// $09971A: bra.w $99734
pub fn i_09971a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9971E;
    cpu.ir = 0x6000;
    cpu.pc = 0x99734;
    10
}
/// $09971E: move.w #$129, d1
pub fn i_09971e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99722;
    cpu.ir = 0x323C;
    let v1: u32 = 0x129;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $099722: jsr $236e.l
pub fn i_099722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99728;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99728);
    cpu.pc = 0x236E;
    16
}
/// $099728: lea.l $2f609e.l, a0
pub fn i_099728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9972E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F609E;
    20
}
/// $09972E: jsr $28cd4.l
pub fn i_09972e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99734;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99734);
    cpu.pc = 0x28CD4;
    16
}
/// $099734: jsr $267e2.l
pub fn i_099734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9973A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9973A);
    cpu.pc = 0x267E2;
    16
}
/// $09973A: jsr $5e7c0.l
pub fn i_09973a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99740;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99740);
    cpu.pc = 0x5E7C0;
    16
}
/// $099740: jsr $8f002.l
pub fn i_099740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99746;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99746);
    cpu.pc = 0x8F002;
    16
}
/// $099746: move.w #$8000, d0
pub fn i_099746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9974A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09974A: jsr $28134.l
pub fn i_09974a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99750;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99750);
    cpu.pc = 0x28134;
    16
}
/// $099750: andi.w #$ffe3, $38(a6)
pub fn i_099750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99756;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $099756: ori.w #$4, $38(a6)
pub fn i_099756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9975C;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x4) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09975C: bclr.b #$1, $12(a6)
pub fn i_09975c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99762;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== SetTaskHandler_099762  ($099762, 8 B, kernel/task_handlers.c)
/// $099762: lea.l $9976a(pc), a1
pub fn i_099762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99766;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9976A;
    16
}
/// $099766: move.l a1, (a6)
pub fn i_099766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99768;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $099768: rts
pub fn i_099768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9976A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_PhysLoop_09976a  ($09976A, 34 B, frontend/hiscore_memcard_options_09773c.s)
/// $09976A: jsr $2783a.l
pub fn i_09976a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99770;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99770);
    cpu.pc = 0x2783A;
    16
}
/// $099770: jsr $28d70.l
pub fn i_099770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99776;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99776);
    cpu.pc = 0x28D70;
    16
}
/// $099776: movea.l #$ffffffff, a0
pub fn i_099776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9977C;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $09977C: lea.l $2f61ac.l, a0
pub fn i_09977c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99782;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F61AC;
    20
}
/// $099782: jsr $5dd56.l
pub fn i_099782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99788;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99788);
    cpu.pc = 0x5DD56;
    16
}
/// $099788: bcc.w $99792
pub fn i_099788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9978C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x99792; return 10; }
    8
}
// ===== SetTaskHandler_09978c  ($09978C, 8 B, kernel/task_handlers.c)
/// $09978C: lea.l $99794(pc), a1
pub fn i_09978c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99790;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99794;
    16
}
/// $099790: move.l a1, (a6)
pub fn i_099790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99792;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $099792: rts
pub fn i_099792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99794;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_099794  ($099794, 8 B, kernel/jmp_to_scheduler.c)
/// $099794: jmp $518.l
pub fn i_099794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9979A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $09979A: rts
pub fn i_09979a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9979C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CmpPrioWithSibling_09979c  ($09979C, 16 B, frontend/hiscore_memcard_options_09773c.s)
/// $09979C: movea.l $8(a6), a1
pub fn i_09979c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x997A0;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0997A0: move.b $10(a6), d0
pub fn i_0997a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x997A4;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0997A4: cmp.b $10(a1), d0
pub fn i_0997a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x997A8;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $0997A8: bcs.w $997b2
pub fn i_0997a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x997AC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x997B2; return 10; }
    8
}
// ===== ClearXN_0997ac  ($0997AC, 6 B, thunks/ccr_helpers.c)
/// $0997AC: andi.b #$ee, ccr
pub fn i_0997ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x997B0;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0997B0: rts
pub fn i_0997b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x997B2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_0997b2  ($0997B2, 6 B, thunks/ccr_helpers.c)
/// $0997B2: ori.b #$11, ccr
pub fn i_0997b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x997B6;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0997B6: rts
pub fn i_0997b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x997B8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Trail_RingReset_0997b8  ($0997B8, 20 B, frontend/hiscore_memcard_options_09773c.s)
/// $0997B8: clr.w $10e47e.l
pub fn i_0997b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x997BE;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x10E47E) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x10E47E, r2 as u16);
    16
}
/// $0997BE: clr.w $10e480.l
pub fn i_0997be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x997C4;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x10E480) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x10E480, r2 as u16);
    16
}
/// $0997C4: clr.w $10e482.l
pub fn i_0997c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x997CA;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x10E482) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x10E482, r2 as u16);
    16
}
/// $0997CA: rts
pub fn i_0997ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x997CC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Trail_RingAdvance_0997cc  ($0997CC, 22 B, frontend/hiscore_memcard_options_09773c.s)
/// $0997CC: move.w $10e480.l, $10e47e.l
pub fn i_0997cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x997D6;
    cpu.ir = 0x33F9;
    let v2 = bus.read16(0x10E480) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10E47E, v2 as u16);
    28
}
/// $0997D6: move.w $10e482.l, $10e480.l
pub fn i_0997d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x997E0;
    cpu.ir = 0x33F9;
    let v2 = bus.read16(0x10E482) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10E480, v2 as u16);
    28
}
/// $0997E0: rts
pub fn i_0997e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x997E2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_TrailRecord_Alt_0997e2  ($0997E2, 48 B, frontend/hiscore_memcard_options_09773c.s)
/// $0997E2: lea.l $10e3be.l, a0
pub fn i_0997e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x997E8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E3BE;
    20
}
/// $0997E8: move.b $6c(a6), d7
pub fn i_0997e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x997EC;
    cpu.ir = 0x1E2E;
    let ad2 = cpu.a[6].wrapping_add(0x6C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0997EC: lsr.b #$4, d7
pub fn i_0997ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x997EE;
    cpu.ir = 0xE80F;
    let h1 = cpu.d[7] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, false);
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r & 0xFF);
    14
}
/// $0997EE: cmpi.b #$e, d7
pub fn i_0997ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x997F2;
    cpu.ir = 0xC07;
    let v1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, v1, 0xE, Size::Byte, false);
    8
}
/// $0997F2: beq.w $9980e
pub fn i_0997f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x997F6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9980E; return 10; }
    8
}
/// $0997F6: move.b $6c(a6), d5
pub fn i_0997f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x997FA;
    cpu.ir = 0x1A2E;
    let ad2 = cpu.a[6].wrapping_add(0x6C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0997FA: andi.b #$f, d5
pub fn i_0997fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x997FE;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0997FE: ori.b #$e0, d5
pub fn i_0997fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99802;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0xE0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $099802: move.b d5, $6c(a6)
pub fn i_099802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99806;
    cpu.ir = 0x1D45;
    let v1 = cpu.d[5] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x6C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $099806: clr.w d5
pub fn i_099806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99808;
    cpu.ir = 0x4245;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $099808: clr.w d6
pub fn i_099808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9980A;
    cpu.ir = 0x4246;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $09980A: bra.w $99896
pub fn i_09980a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9980E;
    cpu.ir = 0x6000;
    cpu.pc = 0x99896;
    10
}
/// $09980E: bra.w $9985a
pub fn i_09980e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99812;
    cpu.ir = 0x6000;
    cpu.pc = 0x9985A;
    10
}
// ===== Entity_TrailRecord_099812  ($099812, 184 B, entity/trail_record_099812.s)
/// $099812: lea.l $10e3be.l, a0
pub fn i_099812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99818;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E3BE;
    20
}
/// $099818: move.b $6c(a6), d7
pub fn i_099818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9981C;
    cpu.ir = 0x1E2E;
    let ad2 = cpu.a[6].wrapping_add(0x6C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09981C: lsr.b #$4, d7
pub fn i_09981c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9981E;
    cpu.ir = 0xE80F;
    let h1 = cpu.d[7] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, false);
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r & 0xFF);
    14
}
/// $09981E: cmpi.b #$f, d7
pub fn i_09981e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99822;
    cpu.ir = 0xC07;
    let v1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, v1, 0xF, Size::Byte, false);
    8
}
/// $099822: bne.w $9985a
pub fn i_099822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99826;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9985A; return 10; }
    8
}
/// $099826: move.b $10e484.l, d7
pub fn i_099826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9982C;
    cpu.ir = 0x1E39;
    let v2 = bus.read8(0x10E484) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v2 & 0xFF);
    16
}
/// $09982C: addq.b #$1, d7
pub fn i_09982c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9982E;
    cpu.ir = 0x5207;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    4
}
/// $09982E: cmpi.b #$e, d7
pub fn i_09982e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99832;
    cpu.ir = 0xC07;
    let v1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, v1, 0xE, Size::Byte, false);
    8
}
/// $099832: bcs.w $9983a
pub fn i_099832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99836;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x9983A; return 10; }
    8
}
/// $099836: move.b #$0, d7
pub fn i_099836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9983A;
    cpu.ir = 0x1E3C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v1 & 0xFF);
    4
}
/// $09983A: move.b d7, $10e484.l
pub fn i_09983a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99840;
    cpu.ir = 0x13C7;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E484, v1 as u8);
    16
}
/// $099840: move.b d7, d6
pub fn i_099840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99842;
    cpu.ir = 0x1C07;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v1 & 0xFF);
    4
}
/// $099842: lsl.b #$4, d6
pub fn i_099842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99844;
    cpu.ir = 0xE90E;
    let h1 = cpu.d[6] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r & 0xFF);
    14
}
/// $099844: move.b $6c(a6), d5
pub fn i_099844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99848;
    cpu.ir = 0x1A2E;
    let ad2 = cpu.a[6].wrapping_add(0x6C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    12
}
/// $099848: andi.b #$f, d5
pub fn i_099848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9984C;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $09984C: or.b d6, d5
pub fn i_09984c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9984E;
    cpu.ir = 0x8A06;
    let v1 = cpu.d[6] & 0xFF;
    let r = (v1 | cpu.d[5]) & 0xFF;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $09984E: move.b d5, $6c(a6)
pub fn i_09984e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99852;
    cpu.ir = 0x1D45;
    let v1 = cpu.d[5] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x6C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $099852: clr.w d5
pub fn i_099852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99854;
    cpu.ir = 0x4245;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $099854: clr.w d6
pub fn i_099854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99856;
    cpu.ir = 0x4246;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $099856: bra.w $99896
pub fn i_099856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9985A;
    cpu.ir = 0x6000;
    cpu.pc = 0x99896;
    10
}
/// $09985A: move.w $10e47e.l, d3
pub fn i_09985a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99860;
    cpu.ir = 0x3639;
    let v2 = bus.read16(0x10E47E) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $099860: cmp.w $10e480.l, d3
pub fn i_099860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99866;
    cpu.ir = 0xB679;
    let v2 = bus.read16(0x10E480) as u32;
    let h1 = cpu.d[3] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    16
}
/// $099866: bne.w $99872
pub fn i_099866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9986A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x99872; return 10; }
    8
}
/// $09986A: clr.w d5
pub fn i_09986a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9986C;
    cpu.ir = 0x4245;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $09986C: clr.w d6
pub fn i_09986c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9986E;
    cpu.ir = 0x4246;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $09986E: bra.w $99896
pub fn i_09986e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99872;
    cpu.ir = 0x6000;
    cpu.pc = 0x99896;
    10
}
/// $099872: cmp.b (a0, d3.w), d7
pub fn i_099872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99876;
    cpu.ir = 0xBE30;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    14
}
/// $099876: beq.w $9988a
pub fn i_099876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9987A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9988A; return 10; }
    8
}
/// $09987A: addi.w #$c, d3
pub fn i_09987a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9987E;
    cpu.ir = 0x643;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xC, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09987E: cmpi.w #$c0, d3
pub fn i_09987e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99882;
    cpu.ir = 0xC43;
    let v1 = cpu.d[3] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xC0, Size::Word, false);
    8
}
/// $099882: bcs.w $99888
pub fn i_099882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99886;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x99888; return 10; }
    8
}
/// $099886: clr.w d3
pub fn i_099886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99888;
    cpu.ir = 0x4243;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $099888: bra.b $99860
pub fn i_099888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9988A;
    cpu.ir = 0x60D6;
    cpu.pc = 0x99860;
    10
}
/// $09988A: move.w d0, d5
pub fn i_09988a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9988C;
    cpu.ir = 0x3A00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09988C: sub.w $2(a0, d3.w), d5
pub fn i_09988c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99890;
    cpu.ir = 0x9A70;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[5] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $099890: move.w d1, d6
pub fn i_099890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99892;
    cpu.ir = 0x3C01;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $099892: sub.w $4(a0, d3.w), d6
pub fn i_099892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99896;
    cpu.ir = 0x9C70;
    let ad2 = cpu.a[0].wrapping_add(0x4).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $099896: move.w $10e482.l, d3
pub fn i_099896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9989C;
    cpu.ir = 0x3639;
    let v2 = bus.read16(0x10E482) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $09989C: move.b d7, (a0, d3.w)
pub fn i_09989c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998A0;
    cpu.ir = 0x1187;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $0998A0: move.w d0, $2(a0, d3.w)
pub fn i_0998a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998A4;
    cpu.ir = 0x3180;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $0998A4: move.w d1, $4(a0, d3.w)
pub fn i_0998a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998A8;
    cpu.ir = 0x3181;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x4).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $0998A8: move.w d2, $6(a0, d3.w)
pub fn i_0998a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998AC;
    cpu.ir = 0x3182;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x6).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $0998AC: move.w d5, $8(a0, d3.w)
pub fn i_0998ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998B0;
    cpu.ir = 0x3185;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x8).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $0998B0: move.w d6, $a(a0, d3.w)
pub fn i_0998b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998B4;
    cpu.ir = 0x3186;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0xA).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $0998B4: addi.w #$c, d3
pub fn i_0998b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998B8;
    cpu.ir = 0x643;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xC, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0998B8: cmpi.w #$c0, d3
pub fn i_0998b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998BC;
    cpu.ir = 0xC43;
    let v1 = cpu.d[3] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xC0, Size::Word, false);
    8
}
/// $0998BC: bcs.w $998c2
pub fn i_0998bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998C0;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x998C2; return 10; }
    8
}
/// $0998C0: clr.w d3
pub fn i_0998c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998C2;
    cpu.ir = 0x4243;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0998C2: move.w d3, $10e482.l
pub fn i_0998c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998C8;
    cpu.ir = 0x33C3;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10E482, v1 as u16);
    16
}
/// $0998C8: rts
pub fn i_0998c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998CA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Trail_FindByKeyRange_0998ca  ($0998CA, 30 B, frontend/hiscore_memcard_options_09773c.s)
/// $0998CA: movem.l d7/a0, -(a7)
pub fn i_0998ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998CE;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[7]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $0998CE: lea.l $10e3be.l, a0
pub fn i_0998ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998D4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E3BE;
    20
}
/// $0998D4: move.w $10e480.l, d7
pub fn i_0998d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998DA;
    cpu.ir = 0x3E39;
    let v2 = bus.read16(0x10E480) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $0998DA: cmp.w $10e47e.l, d7
pub fn i_0998da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998E0;
    cpu.ir = 0xBE79;
    let v2 = bus.read16(0x10E47E) as u32;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    16
}
/// $0998E0: bne.w $998ee
pub fn i_0998e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998E4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x998EE; return 10; }
    8
}
/// $0998E4: movem.l (a7)+, d7/a0
pub fn i_0998e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998E8;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[7] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
// ===== ClearC_0998e8  ($0998E8, 6 B, thunks/ccr_helpers.c)
/// $0998E8: andi.b #$fe, ccr
pub fn i_0998e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998EC;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $0998EC: rts
pub fn i_0998ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998EE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Trail_FindByKeyRange_Step_0998ee  ($0998EE, 70 B, frontend/hiscore_memcard_options_09773c.s)
/// $0998EE: subi.w #$c, d7
pub fn i_0998ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998F2;
    cpu.ir = 0x447;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0xC, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0998F2: bcc.w $998fa
pub fn i_0998f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998F6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x998FA; return 10; }
    8
}
/// $0998F6: addi.w #$c0, d7
pub fn i_0998f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998FA;
    cpu.ir = 0x647;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xC0, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0998FA: cmp.w $4(a0, d7.w), d2
pub fn i_0998fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998FE;
    cpu.ir = 0xB470;
    let ad2 = cpu.a[0].wrapping_add(0x4).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    14
}
/// $0998FE: bne.w $9993a
pub fn i_0998fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99902;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9993A; return 10; }
    8
}
/// $099902: move.w d1, d5
pub fn i_099902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99904;
    cpu.ir = 0x3A01;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $099904: sub.w $2(a0, d7.w), d5
pub fn i_099904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99908;
    cpu.ir = 0x9A70;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[5] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $099908: cmp.w $6(a0, d7.w), d5
pub fn i_099908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9990C;
    cpu.ir = 0xBA70;
    let ad2 = cpu.a[0].wrapping_add(0x6).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[5] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    14
}
/// $09990C: bcc.w $9993a
pub fn i_09990c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99910;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9993A; return 10; }
    8
}
/// $099910: move.b $6c(a6), d5
pub fn i_099910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99914;
    cpu.ir = 0x1A2E;
    let ad2 = cpu.a[6].wrapping_add(0x6C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    12
}
/// $099914: lsr.b #$4, d5
pub fn i_099914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99916;
    cpu.ir = 0xE80D;
    let h1 = cpu.d[5] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, false);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r & 0xFF);
    14
}
/// $099916: cmp.b (a0, d7.w), d5
pub fn i_099916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9991A;
    cpu.ir = 0xBA30;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[5] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    14
}
/// $09991A: beq.w $9993a
pub fn i_09991a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9991E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9993A; return 10; }
    8
}
/// $09991E: andi.b #$f0, $6c(a6)
pub fn i_09991e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99924;
    cpu.ir = 0x22E;
    let ad1 = cpu.a[6].wrapping_add(0x6C);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0xF0) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $099924: move.b (a0, d7.w), d5
pub fn i_099924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99928;
    cpu.ir = 0x1A30;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    14
}
/// $099928: andi.b #$f, d5
pub fn i_099928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9992C;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $09992C: or.b d5, $6c(a6)
pub fn i_09992c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99930;
    cpu.ir = 0x8B2E;
    let ad1 = cpu.a[6].wrapping_add(0x6C);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | cpu.d[5]) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $099930: movem.l (a7)+, d7/a0
pub fn i_099930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99934;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[7] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
// ===== SetC_099934  ($099934, 6 B, thunks/ccr_helpers.c)
/// $099934: ori.b #$1, ccr
pub fn i_099934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99938;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $099938: rts
pub fn i_099938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9993A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Trail_FindByKeyRange_Next_09993a  ($09993A, 2 B, frontend/hiscore_memcard_options_09773c.s)
/// $09993A: bra.b $998da
pub fn i_09993a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9993C;
    cpu.ir = 0x609E;
    cpu.pc = 0x998DA;
    10
}
// ===== Trail_FindNearest_09993c  ($09993C, 120 B, frontend/hiscore_memcard_options_09773c.s)
/// $09993C: movem.l d7/a0, -(a7)
pub fn i_09993c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99940;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[7]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $099940: lea.l $10e3be.l, a0
pub fn i_099940(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99946;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E3BE;
    20
}
/// $099946: move.w $10e47e.l, d7
pub fn i_099946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9994C;
    cpu.ir = 0x3E39;
    let v2 = bus.read16(0x10E47E) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $09994C: cmp.w $10e480.l, d7
pub fn i_09994c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99952;
    cpu.ir = 0xBE79;
    let v2 = bus.read16(0x10E480) as u32;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    16
}
/// $099952: bne.w $99976
pub fn i_099952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99956;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x99976; return 10; }
    8
}
/// $099956: move.b $6c(a6), d7
pub fn i_099956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9995A;
    cpu.ir = 0x1E2E;
    let ad2 = cpu.a[6].wrapping_add(0x6C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09995A: andi.b #$f, d7
pub fn i_09995a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9995E;
    cpu.ir = 0x207;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $09995E: cmpi.b #$e, d7
pub fn i_09995e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99962;
    cpu.ir = 0xC07;
    let v1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, v1, 0xE, Size::Byte, false);
    8
}
/// $099962: bne.w $9996c
pub fn i_099962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99966;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9996C; return 10; }
    8
}
/// $099966: ori.b #$f, $6c(a6)
pub fn i_099966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9996C;
    cpu.ir = 0x2E;
    let ad1 = cpu.a[6].wrapping_add(0x6C);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0xF) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $09996C: move.b #$f, d6
pub fn i_09996c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99970;
    cpu.ir = 0x1C3C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v1 & 0xFF);
    4
}
/// $099970: movem.l (a7)+, d7/a0
pub fn i_099970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99974;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[7] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $099974: rts
pub fn i_099974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99976;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $099976: move.w $4(a0, d7.w), d5
pub fn i_099976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9997A;
    cpu.ir = 0x3A30;
    let ad2 = cpu.a[0].wrapping_add(0x4).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $09997A: cmp.w d5, d2
pub fn i_09997a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9997C;
    cpu.ir = 0xB445;
    let v1 = cpu.d[5] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $09997C: bgt.w $999ba
pub fn i_09997c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99980;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x999BA; return 10; }
    8
}
/// $099980: subi.w #$14, d5
pub fn i_099980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99984;
    cpu.ir = 0x445;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x14, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $099984: cmp.w d5, d2
pub fn i_099984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99986;
    cpu.ir = 0xB445;
    let v1 = cpu.d[5] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $099986: ble.w $999ba
pub fn i_099986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9998A;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x999BA; return 10; }
    8
}
/// $09998A: move.w d1, d5
pub fn i_09998a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9998C;
    cpu.ir = 0x3A01;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09998C: sub.w $2(a0, d7.w), d5
pub fn i_09998c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99990;
    cpu.ir = 0x9A70;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[5] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $099990: cmp.w $6(a0, d7.w), d5
pub fn i_099990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99994;
    cpu.ir = 0xBA70;
    let ad2 = cpu.a[0].wrapping_add(0x6).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[5] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    14
}
/// $099994: bcc.w $999ba
pub fn i_099994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99998;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x999BA; return 10; }
    8
}
/// $099998: move.b $6c(a6), d6
pub fn i_099998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9999C;
    cpu.ir = 0x1C2E;
    let ad2 = cpu.a[6].wrapping_add(0x6C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09999C: lsr.b #$4, d6
pub fn i_09999c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9999E;
    cpu.ir = 0xE80E;
    let h1 = cpu.d[6] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, false);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r & 0xFF);
    14
}
/// $09999E: cmp.b (a0, d7.w), d6
pub fn i_09999e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999A2;
    cpu.ir = 0xBC30;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[6] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    14
}
/// $0999A2: beq.w $999ba
pub fn i_0999a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999A6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x999BA; return 10; }
    8
}
/// $0999A6: move.w $4(a0, d7.w), d5
pub fn i_0999a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999AA;
    cpu.ir = 0x3A30;
    let ad2 = cpu.a[0].wrapping_add(0x4).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $0999AA: sub.w d2, d5
pub fn i_0999aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999AC;
    cpu.ir = 0x9A42;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[5] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0999AC: move.b (a0, d7.w), d6
pub fn i_0999ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999B0;
    cpu.ir = 0x1C30;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    14
}
/// $0999B0: movem.l (a7)+, d7/a0
pub fn i_0999b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999B4;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[7] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
// ===== SetC_0999b4  ($0999B4, 6 B, thunks/ccr_helpers.c)
/// $0999B4: ori.b #$1, ccr
pub fn i_0999b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999B8;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $0999B8: rts
pub fn i_0999b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999BA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Trail_FindNearest_Next_0999ba  ($0999BA, 16 B, frontend/hiscore_memcard_options_09773c.s)
/// $0999BA: addi.w #$c, d7
pub fn i_0999ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999BE;
    cpu.ir = 0x647;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xC, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0999BE: cmpi.w #$c0, d7
pub fn i_0999be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999C2;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xC0, Size::Word, false);
    8
}
/// $0999C2: bcs.w $999c8
pub fn i_0999c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999C6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x999C8; return 10; }
    8
}
/// $0999C6: clr.w d7
pub fn i_0999c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999C8;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0999C8: bra.b $9994c
pub fn i_0999c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999CA;
    cpu.ir = 0x6082;
    cpu.pc = 0x9994C;
    10
}
// ===== Trail_MergeIdNibble_0999ca  ($0999CA, 14 B, frontend/hiscore_memcard_options_09773c.s)
/// $0999CA: move.b $6c(a6), d7
pub fn i_0999ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999CE;
    cpu.ir = 0x1E2E;
    let ad2 = cpu.a[6].wrapping_add(0x6C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0999CE: andi.w #$f0, d7
pub fn i_0999ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999D2;
    cpu.ir = 0x247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = (v1 & 0xF0) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0999D2: andi.w #$f, d0
pub fn i_0999d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999D6;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0999D6: or.w d7, d0
pub fn i_0999d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999D8;
    cpu.ir = 0x8047;
    let v1 = cpu.d[7] & 0xFFFF;
    let r = (v1 | cpu.d[0]) & 0xFFFF;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
// ===== SetTaskB_0999d8  ($0999D8, 6 B, kernel/task_setters_b.c)
/// $0999D8: move.b d0, $6c(a6)
pub fn i_0999d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999DC;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x6C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0999DC: rts
pub fn i_0999dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999DE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Clipping_Test_0999DE  ($0999DE, 24 B, video/clipping_test_0999de.s)
/// $0999DE: move.b $6c(a6), d5
pub fn i_0999de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999E2;
    cpu.ir = 0x1A2E;
    let ad2 = cpu.a[6].wrapping_add(0x6C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0999E2: move.b d5, d6
pub fn i_0999e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999E4;
    cpu.ir = 0x1C05;
    let v1 = cpu.d[5] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0999E4: andi.b #$f, d5
pub fn i_0999e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999E8;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0999E8: cmpi.b #$f, d5
pub fn i_0999e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999EC;
    cpu.ir = 0xC05;
    let v1 = cpu.d[5] & 0xFF;
    sub_with_flags(cpu, v1, 0xF, Size::Byte, false);
    8
}
/// $0999EC: bne.w $999f6
pub fn i_0999ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999F0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x999F6; return 10; }
    8
}
/// $0999F0: andi.b #$fe, ccr
pub fn i_0999f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999F4;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $0999F4: rts
pub fn i_0999f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999F6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Trail_LookupById_0999f6  ($0999F6, 38 B, frontend/hiscore_memcard_options_09773c.s)
/// $0999F6: lsr.b #$4, d6
pub fn i_0999f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999F8;
    cpu.ir = 0xE80E;
    let h1 = cpu.d[6] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, false);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r & 0xFF);
    14
}
/// $0999F8: cmp.b d6, d5
pub fn i_0999f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999FA;
    cpu.ir = 0xBA06;
    let v1 = cpu.d[6] & 0xFF;
    let h1 = cpu.d[5] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $0999FA: bne.w $99a00
pub fn i_0999fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999FE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x99A00; return 10; }
    8
}
/// $0999FE: trap #$f
pub fn i_0999fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A00;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $099A00: lea.l $10e3be.l, a0
pub fn i_099a00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A06;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E3BE;
    20
}
/// $099A06: move.w $10e47e.l, d7
pub fn i_099a06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A0C;
    cpu.ir = 0x3E39;
    let v2 = bus.read16(0x10E47E) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $099A0C: cmp.w $10e480.l, d7
pub fn i_099a0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A12;
    cpu.ir = 0xBE79;
    let v2 = bus.read16(0x10E480) as u32;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    16
}
/// $099A12: bne.w $99a22
pub fn i_099a12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A16;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x99A22; return 10; }
    8
}
/// $099A16: ori.b #$f, $6c(a6)
pub fn i_099a16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A1C;
    cpu.ir = 0x2E;
    let ad1 = cpu.a[6].wrapping_add(0x6C);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0xF) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== ClearC_099a1c  ($099A1C, 6 B, thunks/ccr_helpers.c)
/// $099A1C: andi.b #$fe, ccr
pub fn i_099a1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A20;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $099A20: rts
pub fn i_099a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A22;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Trail_LookupById_Step_099a22  ($099A22, 34 B, frontend/hiscore_memcard_options_09773c.s)
/// $099A22: cmp.b (a0, d7.w), d5
pub fn i_099a22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A26;
    cpu.ir = 0xBA30;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[5] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    14
}
/// $099A26: beq.w $99a3a
pub fn i_099a26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A2A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x99A3A; return 10; }
    8
}
/// $099A2A: addi.w #$c, d7
pub fn i_099a2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A2E;
    cpu.ir = 0x647;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xC, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $099A2E: cmpi.w #$c0, d7
pub fn i_099a2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A32;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xC0, Size::Word, false);
    8
}
/// $099A32: bcs.w $99a38
pub fn i_099a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A36;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x99A38; return 10; }
    8
}
/// $099A36: clr.w d7
pub fn i_099a36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A38;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $099A38: bra.b $99a0c
pub fn i_099a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A3A;
    cpu.ir = 0x60D2;
    cpu.pc = 0x99A0C;
    10
}
/// $099A3A: adda.w d7, a0
pub fn i_099a3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A3C;
    cpu.ir = 0xD0C7;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $099A3C: move.w $8(a0), d2
pub fn i_099a3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A40;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $099A40: move.w $a(a0), d3
pub fn i_099a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A44;
    cpu.ir = 0x3628;
    let ad2 = cpu.a[0].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
// ===== SetC_099a44  ($099A44, 6 B, thunks/ccr_helpers.c)
/// $099A44: ori.b #$1, ccr
pub fn i_099a44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A48;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $099A48: rts
pub fn i_099a48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A4A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== DebugCursor_Init_099a4a  ($099A4A, 18 B, frontend/hiscore_memcard_options_09773c.s)
/// $099A4A: move.w #$e, d1
pub fn i_099a4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A4E;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $099A4E: jsr $236e.l
pub fn i_099a4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99A54);
    cpu.pc = 0x236E;
    16
}
/// $099A54: move.l #$2507fa, $3c(a6)
pub fn i_099a54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A5C;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2507FA;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
// ===== SetTaskHandler_099a5c  ($099A5C, 8 B, kernel/task_handlers.c)
/// $099A5C: lea.l $99a64(pc), a1
pub fn i_099a5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A60;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99A64;
    16
}
/// $099A60: move.l a1, (a6)
pub fn i_099a60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A62;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $099A62: rts
pub fn i_099a62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A64;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== DebugCursor_Run_099a64  ($099A64, 90 B, frontend/hiscore_memcard_options_09773c.s)
/// $099A64: move.b $10fd9c.l, d0
pub fn i_099a64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A6A;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10FD9C) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $099A6A: btst.b #$0, d0
pub fn i_099a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A6E;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $099A6E: beq.w $99a76
pub fn i_099a6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A72;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x99A76; return 10; }
    8
}
/// $099A72: addq.w #$8, $2a(a6)
pub fn i_099a72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A76;
    cpu.ir = 0x506E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $099A76: btst.b #$1, d0
pub fn i_099a76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A7A;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((1) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $099A7A: beq.w $99a82
pub fn i_099a7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A7E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x99A82; return 10; }
    8
}
/// $099A7E: subq.w #$8, $2a(a6)
pub fn i_099a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A82;
    cpu.ir = 0x516E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $099A82: btst.b #$2, d0
pub fn i_099a82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A86;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((2) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $099A86: beq.w $99a8e
pub fn i_099a86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A8A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x99A8E; return 10; }
    8
}
/// $099A8A: subq.w #$8, $28(a6)
pub fn i_099a8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A8E;
    cpu.ir = 0x516E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $099A8E: btst.b #$3, d0
pub fn i_099a8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A92;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((3) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $099A92: beq.w $99a9a
pub fn i_099a92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A96;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x99A9A; return 10; }
    8
}
/// $099A96: addq.w #$8, $28(a6)
pub fn i_099a96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A9A;
    cpu.ir = 0x506E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $099A9A: jsr $27cee.l
pub fn i_099a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99AA0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99AA0);
    cpu.pc = 0x27CEE;
    16
}
/// $099AA0: move.w $22(a6), d0
pub fn i_099aa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99AA4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $099AA4: move.w $24(a6), d1
pub fn i_099aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99AA8;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $099AA8: subi.w #$10, d0
pub fn i_099aa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99AAC;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $099AAC: addq.w #$7, d1
pub fn i_099aac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99AAE;
    cpu.ir = 0x5E41;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 7, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $099AAE: move.w #$20, d2
pub fn i_099aae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99AB2;
    cpu.ir = 0x343C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $099AB2: andi.w #$1ff, d0
pub fn i_099ab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99AB6;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1FF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $099AB6: andi.w #$1ff, d1
pub fn i_099ab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99ABA;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1FF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $099ABA: jsr $99812(pc)
pub fn i_099aba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99ABE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x99ABE);
    cpu.pc = 0x99812;
    16
}
// ===== JsrAbsThunk_099abe  ($099ABE, 8 B, thunks/jsr_abs_thunks.c)
/// $099ABE: jsr $5ca2a.l
pub fn i_099abe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99AC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99AC4);
    cpu.pc = 0x5CA2A;
    16
}
/// $099AC4: rts
pub fn i_099ac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99AC6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CmpPrioWithSibling_099ac6  ($099AC6, 16 B, frontend/hiscore_memcard_options_09773c.s)
/// $099AC6: movea.l $8(a6), a1
pub fn i_099ac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99ACA;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $099ACA: move.b $10(a6), d0
pub fn i_099aca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99ACE;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $099ACE: cmp.b $10(a1), d0
pub fn i_099ace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99AD2;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $099AD2: bcs.w $99adc
pub fn i_099ad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99AD6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x99ADC; return 10; }
    8
}
// ===== ClearXN_099ad6  ($099AD6, 6 B, thunks/ccr_helpers.c)
/// $099AD6: andi.b #$ee, ccr
pub fn i_099ad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99ADA;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $099ADA: rts
pub fn i_099ada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99ADC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_099adc  ($099ADC, 6 B, thunks/ccr_helpers.c)
/// $099ADC: ori.b #$11, ccr
pub fn i_099adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99AE0;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $099AE0: rts
pub fn i_099ae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99AE2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AttractInit_Single_099AE2  ($099AE2, 26 B, frontend/attract_init_single_099ae2.s)
/// $099AE2: move.b #$3, $10fd88.l
pub fn i_099ae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99AEA;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10FD88, v1 as u8);
    16
}
/// $099AEA: move.b #$2, $10fd8b.l
pub fn i_099aea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99AF2;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10FD8B, v1 as u8);
    16
}
/// $099AF2: move.b #$0, $10e486.l
pub fn i_099af2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99AFA;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E486, v1 as u8);
    16
}
/// $099AFA: rts
pub fn i_099afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99AFC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Global_Clear10E486_099AFC  ($099AFC, 10 B, kernel/global_clear_10e486_099afc.s)
/// $099AFC: move.b #$0, $10e486.l
pub fn i_099afc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99B04;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E486, v1 as u8);
    16
}
/// $099B04: rts
pub fn i_099b04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99B06;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== OptionSelect2_Tpl_099b06  ($099B06, 120 B, frontend/hiscore_memcard_options_09773c.s)
/// $099B06: movea.w #$7258, a1
pub fn i_099b06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99B0A;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7258;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $099B0A: move.w #$2300, d0
pub fn i_099b0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99B0E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $099B0E: lea.l $2f61b4.l, a2
pub fn i_099b0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99B14;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2F61B4;
    20
}
/// $099B14: jsr $5dad8.l
pub fn i_099b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99B1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99B1A);
    cpu.pc = 0x5DAD8;
    16
}
/// $099B1A: movea.w #$725a, a1
pub fn i_099b1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99B1E;
    cpu.ir = 0x327C;
    let v1: u32 = 0x725A;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $099B1E: move.w #$2300, d0
pub fn i_099b1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99B22;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $099B22: lea.l $2f61ba.l, a2
pub fn i_099b22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99B28;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2F61BA;
    20
}
/// $099B28: jsr $5dad8.l
pub fn i_099b28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99B2E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99B2E);
    cpu.pc = 0x5DAD8;
    16
}
/// $099B2E: clr.w $78(a6)
pub fn i_099b2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99B32;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $099B32: jsr $99ee4(pc)
pub fn i_099b32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99B36;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x99B36);
    cpu.pc = 0x99EE4;
    16
}
/// $099B36: lea.l $99b3c(pc), a1
pub fn i_099b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99B3A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99B3C;
    16
}
/// $099B3A: move.l a1, (a6)
pub fn i_099b3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99B3C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $099B3C: jsr $99ed2(pc)
pub fn i_099b3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99B40;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x99B40);
    cpu.pc = 0x99ED2;
    16
}
/// $099B40: btst.b #$4, d0
pub fn i_099b40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99B44;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((4) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $099B44: beq.w $99b64
pub fn i_099b44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99B48;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x99B64; return 10; }
    8
}
/// $099B48: move.w $78(a6), d0
pub fn i_099b48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99B4C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $099B4C: andi.w #$1, d0
pub fn i_099b4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99B50;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $099B50: move.b d0, $106ed5.l
pub fn i_099b50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99B56;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106ED5, v1 as u8);
    16
}
/// $099B56: clr.b $106ed2.l
pub fn i_099b56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99B5C;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106ED2) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106ED2, r2 as u8);
    16
}
/// $099B5C: jmp $518.l
pub fn i_099b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99B62;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $099B62: rts
pub fn i_099b62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99B64;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $099B64: jsr $99ed2(pc)
pub fn i_099b64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99B68;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x99B68);
    cpu.pc = 0x99ED2;
    16
}
/// $099B68: btst.b #$1, d0
pub fn i_099b68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99B6C;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((1) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $099B6C: beq.w $99b84
pub fn i_099b6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99B70;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x99B84; return 10; }
    8
}
/// $099B70: jsr $99f10(pc)
pub fn i_099b70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99B74;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x99B74);
    cpu.pc = 0x99F10;
    16
}
/// $099B74: addq.w #$1, $78(a6)
pub fn i_099b74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99B78;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $099B78: andi.w #$1, $78(a6)
pub fn i_099b78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99B7E;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
// ===== JsrPcThunk_099b7e  ($099B7E, 6 B, thunks/jsr_pc_thunks.c)
/// $099B7E: jsr $99ee4(pc)
pub fn i_099b7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99B82;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x99B82);
    cpu.pc = 0x99EE4;
    16
}
/// $099B82: rts
pub fn i_099b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99B84;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== OptionSelect2_Left_099b84  ($099B84, 26 B, frontend/hiscore_memcard_options_09773c.s)
/// $099B84: jsr $99ed2(pc)
pub fn i_099b84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99B88;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x99B88);
    cpu.pc = 0x99ED2;
    16
}
/// $099B88: btst.b #$0, d0
pub fn i_099b88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99B8C;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $099B8C: beq.w $99ba4
pub fn i_099b8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99B90;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x99BA4; return 10; }
    8
}
/// $099B90: jsr $99f10(pc)
pub fn i_099b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99B94;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x99B94);
    cpu.pc = 0x99F10;
    16
}
/// $099B94: subq.w #$1, $78(a6)
pub fn i_099b94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99B98;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $099B98: andi.w #$1, $78(a6)
pub fn i_099b98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99B9E;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
// ===== JsrPcThunk_099b9e  ($099B9E, 6 B, thunks/jsr_pc_thunks.c)
/// $099B9E: jsr $99ee4(pc)
pub fn i_099b9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99BA2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x99BA2);
    cpu.pc = 0x99EE4;
    16
}
/// $099BA2: rts
pub fn i_099ba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99BA4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== OptionSelect2_Rts_099ba4  ($099BA4, 2 B, frontend/hiscore_memcard_options_09773c.s)
/// $099BA4: rts
pub fn i_099ba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99BA6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== OptionsMenu_Tpl_099ba6  ($099BA6, 444 B, frontend/hiscore_memcard_options_09773c.s)
/// $099BA6: lea.l $59a86.l, a1
pub fn i_099ba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99BAC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x59A86;
    20
}
/// $099BAC: jsr $4ae.l
pub fn i_099bac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99BB2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99BB2);
    cpu.pc = 0x4AE;
    16
}
/// $099BB2: cmpi.b #$2, $10fd83.l
pub fn i_099bb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99BBA;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10FD83) as u32;
    sub_with_flags(cpu, v2, 0x2, Size::Byte, false);
    20
}
/// $099BBA: bne.w $99bd2
pub fn i_099bba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99BBE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x99BD2; return 10; }
    8
}
/// $099BBE: lea.l $2f623c.l, a1
pub fn i_099bbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99BC4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2F623C;
    20
}
/// $099BC4: move.l a1, $80(a6)
pub fn i_099bc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99BC8;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $099BC8: move.w #$3, $7a(a6)
pub fn i_099bc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99BCE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $099BCE: bra.w $99be2
pub fn i_099bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99BD2;
    cpu.ir = 0x6000;
    cpu.pc = 0x99BE2;
    10
}
/// $099BD2: lea.l $2f6236.l, a1
pub fn i_099bd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99BD8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2F6236;
    20
}
/// $099BD8: move.l a1, $80(a6)
pub fn i_099bd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99BDC;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $099BDC: move.w #$2, $7a(a6)
pub fn i_099bdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99BE2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $099BE2: movea.w #$71c7, a1
pub fn i_099be2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99BE6;
    cpu.ir = 0x327C;
    let v1: u32 = 0x71C7;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $099BE6: lea.l $2f61c2.l, a2
pub fn i_099be6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99BEC;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2F61C2;
    20
}
/// $099BEC: move.w #$4, d1
pub fn i_099bec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99BF0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $099BF0: jsr $4784c.l
pub fn i_099bf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99BF6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99BF6);
    cpu.pc = 0x4784C;
    16
}
/// $099BF6: movea.w #$716b, a1
pub fn i_099bf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99BFA;
    cpu.ir = 0x327C;
    let v1: u32 = 0x716B;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $099BFA: lea.l $2f61d8.l, a2
pub fn i_099bfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99C00;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2F61D8;
    20
}
/// $099C00: move.w #$4, d1
pub fn i_099c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99C04;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $099C04: jsr $4784c.l
pub fn i_099c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99C0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99C0A);
    cpu.pc = 0x4784C;
    16
}
/// $099C0A: movea.w #$716f, a1
pub fn i_099c0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99C0E;
    cpu.ir = 0x327C;
    let v1: u32 = 0x716F;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $099C0E: lea.l $2f61de.l, a2
pub fn i_099c0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99C14;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2F61DE;
    20
}
/// $099C14: move.w #$4, d1
pub fn i_099c14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99C18;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $099C18: jsr $4784c.l
pub fn i_099c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99C1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99C1E);
    cpu.pc = 0x4784C;
    16
}
/// $099C1E: cmpi.b #$2, $10fd83.l
pub fn i_099c1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99C26;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10FD83) as u32;
    sub_with_flags(cpu, v2, 0x2, Size::Byte, false);
    20
}
/// $099C26: bne.w $99c3e
pub fn i_099c26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99C2A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x99C3E; return 10; }
    8
}
/// $099C2A: movea.w #$7173, a1
pub fn i_099c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99C2E;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7173;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $099C2E: lea.l $2f6210.l, a2
pub fn i_099c2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99C34;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2F6210;
    20
}
/// $099C34: move.w #$4, d1
pub fn i_099c34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99C38;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $099C38: jsr $477fc.l
pub fn i_099c38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99C3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99C3E);
    cpu.pc = 0x477FC;
    16
}
/// $099C3E: movea.w #$7177, a1
pub fn i_099c3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99C42;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7177;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $099C42: lea.l $2f61ea.l, a2
pub fn i_099c42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99C48;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2F61EA;
    20
}
/// $099C48: move.w #$4, d1
pub fn i_099c48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99C4C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $099C4C: jsr $4784c.l
pub fn i_099c4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99C52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99C52);
    cpu.pc = 0x4784C;
    16
}
/// $099C52: clr.w $78(a6)
pub fn i_099c52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99C56;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $099C56: clr.w d0
pub fn i_099c56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99C58;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $099C58: move.b $10fd8b.l, d0
pub fn i_099c58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99C5E;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10FD8B) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $099C5E: andi.b #$7, d0
pub fn i_099c5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99C62;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x7) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $099C62: lea.l $2f6244.l, a1
pub fn i_099c62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99C68;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2F6244;
    20
}
/// $099C68: move.b (a1, d0.w), $72(a6)
pub fn i_099c68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99C6E;
    cpu.ir = 0x1D71;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad4, v3 as u8);
    22
}
/// $099C6E: move.b $10fd88.l, $73(a6)
pub fn i_099c6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99C76;
    cpu.ir = 0x1D79;
    let v2 = bus.read8(0x10FD88) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[6].wrapping_add(0x73);
    bus.write8(ad3, v2 as u8);
    24
}
/// $099C76: cmpi.b #$1, $73(a6)
pub fn i_099c76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99C7C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $099C7C: bcc.w $99c86
pub fn i_099c7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99C80;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x99C86; return 10; }
    8
}
/// $099C80: move.b #$1, $73(a6)
pub fn i_099c80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99C86;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    bus.write8(ad2, v1 as u8);
    12
}
/// $099C86: cmpi.b #$5, $73(a6)
pub fn i_099c86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99C8C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x5, Size::Byte, false);
    16
}
/// $099C8C: bls.w $99c96
pub fn i_099c8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99C90;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x99C96; return 10; }
    8
}
/// $099C90: move.b #$5, $73(a6)
pub fn i_099c90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99C96;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    bus.write8(ad2, v1 as u8);
    12
}
/// $099C96: move.b $10e486.l, $74(a6)
pub fn i_099c96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99C9E;
    cpu.ir = 0x1D79;
    let v2 = bus.read8(0x10E486) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[6].wrapping_add(0x74);
    bus.write8(ad3, v2 as u8);
    24
}
/// $099C9E: addq.b #$1, $74(a6)
pub fn i_099c9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99CA2;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $099CA2: cmpi.b #$1, $74(a6)
pub fn i_099ca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99CA8;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $099CA8: bcc.w $99cb2
pub fn i_099ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99CAC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x99CB2; return 10; }
    8
}
/// $099CAC: move.b #$1, $74(a6)
pub fn i_099cac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99CB2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write8(ad2, v1 as u8);
    12
}
/// $099CB2: cmpi.b #$5, $74(a6)
pub fn i_099cb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99CB8;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x5, Size::Byte, false);
    16
}
/// $099CB8: bls.w $99cc2
pub fn i_099cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99CBC;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x99CC2; return 10; }
    8
}
/// $099CBC: move.b #$5, $74(a6)
pub fn i_099cbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99CC2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write8(ad2, v1 as u8);
    12
}
/// $099CC2: move.b $10fd92.l, $76(a6)
pub fn i_099cc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99CCA;
    cpu.ir = 0x1D79;
    let v2 = bus.read8(0x10FD92) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[6].wrapping_add(0x76);
    bus.write8(ad3, v2 as u8);
    24
}
/// $099CCA: cmpi.b #$1, $76(a6)
pub fn i_099cca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99CD0;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $099CD0: beq.w $99cd8
pub fn i_099cd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99CD4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x99CD8; return 10; }
    8
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x0981E6, i_0981e6),
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
    (0x098720, i_098720),
    (0x098724, i_098724),
    (0x09872A, i_09872a),
    (0x09872E, i_09872e),
    (0x098734, i_098734),
    (0x098736, i_098736),
    (0x098738, i_098738),
    (0x09873E, i_09873e),
    (0x098744, i_098744),
    (0x098748, i_098748),
    (0x09874E, i_09874e),
    (0x098754, i_098754),
    (0x09875A, i_09875a),
    (0x09875E, i_09875e),
    (0x098764, i_098764),
    (0x09876A, i_09876a),
    (0x098770, i_098770),
    (0x098774, i_098774),
    (0x09877A, i_09877a),
    (0x098780, i_098780),
    (0x098786, i_098786),
    (0x09878A, i_09878a),
    (0x098790, i_098790),
    (0x098796, i_098796),
    (0x09879C, i_09879c),
    (0x0987A0, i_0987a0),
    (0x0987A6, i_0987a6),
    (0x0987AC, i_0987ac),
    (0x0987B2, i_0987b2),
    (0x0987B6, i_0987b6),
    (0x0987BC, i_0987bc),
    (0x0987C2, i_0987c2),
    (0x0987C8, i_0987c8),
    (0x0987CC, i_0987cc),
    (0x0987D2, i_0987d2),
    (0x0987D8, i_0987d8),
    (0x0987DE, i_0987de),
    (0x0987E2, i_0987e2),
    (0x0987E8, i_0987e8),
    (0x0987EE, i_0987ee),
    (0x0987F4, i_0987f4),
    (0x0987F8, i_0987f8),
    (0x0987FE, i_0987fe),
    (0x098804, i_098804),
    (0x09880A, i_09880a),
    (0x09880E, i_09880e),
    (0x098814, i_098814),
    (0x09881A, i_09881a),
    (0x09881E, i_09881e),
    (0x098820, i_098820),
    (0x098824, i_098824),
    (0x098828, i_098828),
    (0x09882A, i_09882a),
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
    (0x098BD0, i_098bd0),
    (0x098BD6, i_098bd6),
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
    (0x098C32, i_098c32),
    (0x098C38, i_098c38),
    (0x098C3C, i_098c3c),
    (0x098C42, i_098c42),
    (0x098C46, i_098c46),
    (0x098C4A, i_098c4a),
    (0x098C50, i_098c50),
    (0x098C54, i_098c54),
    (0x098C56, i_098c56),
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
    (0x098D96, i_098d96),
    (0x098D9C, i_098d9c),
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
    (0x098F4E, i_098f4e),
    (0x098F52, i_098f52),
    (0x098F58, i_098f58),
    (0x098F5C, i_098f5c),
    (0x098F60, i_098f60),
    (0x098F66, i_098f66),
    (0x098F6C, i_098f6c),
    (0x098F72, i_098f72),
    (0x098F78, i_098f78),
    (0x098F7E, i_098f7e),
    (0x098F82, i_098f82),
    (0x098F88, i_098f88),
    (0x098F8E, i_098f8e),
    (0x098F94, i_098f94),
    (0x098F9A, i_098f9a),
    (0x098F9E, i_098f9e),
    (0x098FA0, i_098fa0),
    (0x098FA2, i_098fa2),
    (0x098FA6, i_098fa6),
    (0x098FAC, i_098fac),
    (0x098FB0, i_098fb0),
    (0x098FB4, i_098fb4),
    (0x098FBA, i_098fba),
    (0x098FBE, i_098fbe),
    (0x098FC2, i_098fc2),
    (0x098FC8, i_098fc8),
    (0x098FCE, i_098fce),
    (0x098FD4, i_098fd4),
    (0x098FDA, i_098fda),
    (0x098FE0, i_098fe0),
    (0x098FE4, i_098fe4),
    (0x098FEA, i_098fea),
    (0x098FF0, i_098ff0),
    (0x098FF6, i_098ff6),
    (0x098FFC, i_098ffc),
    (0x099000, i_099000),
    (0x099002, i_099002),
    (0x099004, i_099004),
    (0x09900A, i_09900a),
    (0x099010, i_099010),
    (0x099014, i_099014),
    (0x09901A, i_09901a),
    (0x09901C, i_09901c),
    (0x099020, i_099020),
    (0x099026, i_099026),
    (0x09902A, i_09902a),
    (0x099030, i_099030),
    (0x099034, i_099034),
    (0x099036, i_099036),
    (0x09903C, i_09903c),
    (0x099042, i_099042),
    (0x099046, i_099046),
    (0x09904A, i_09904a),
    (0x09904C, i_09904c),
    (0x099052, i_099052),
    (0x099058, i_099058),
    (0x09905E, i_09905e),
    (0x099064, i_099064),
    (0x099068, i_099068),
    (0x09906C, i_09906c),
    (0x09906E, i_09906e),
    (0x099070, i_099070),
    (0x099076, i_099076),
    (0x09907C, i_09907c),
    (0x099080, i_099080),
    (0x099082, i_099082),
    (0x099088, i_099088),
    (0x09908E, i_09908e),
    (0x099092, i_099092),
    (0x099096, i_099096),
    (0x099098, i_099098),
    (0x09909E, i_09909e),
    (0x0990A4, i_0990a4),
    (0x0990AA, i_0990aa),
    (0x0990B0, i_0990b0),
    (0x0990B4, i_0990b4),
    (0x0990B8, i_0990b8),
    (0x0990BA, i_0990ba),
    (0x0990BC, i_0990bc),
    (0x0990C0, i_0990c0),
    (0x0990C6, i_0990c6),
    (0x0990CA, i_0990ca),
    (0x0990CE, i_0990ce),
    (0x0990D4, i_0990d4),
    (0x0990D8, i_0990d8),
    (0x0990DC, i_0990dc),
    (0x0990E2, i_0990e2),
    (0x0990E8, i_0990e8),
    (0x0990EE, i_0990ee),
    (0x0990F4, i_0990f4),
    (0x0990FA, i_0990fa),
    (0x0990FE, i_0990fe),
    (0x099104, i_099104),
    (0x09910A, i_09910a),
    (0x099110, i_099110),
    (0x099116, i_099116),
    (0x09911A, i_09911a),
    (0x09911C, i_09911c),
    (0x09911E, i_09911e),
    (0x099122, i_099122),
    (0x099128, i_099128),
    (0x09912C, i_09912c),
    (0x099130, i_099130),
    (0x099136, i_099136),
    (0x09913A, i_09913a),
    (0x09913E, i_09913e),
    (0x099144, i_099144),
    (0x09914A, i_09914a),
    (0x099150, i_099150),
    (0x099156, i_099156),
    (0x09915C, i_09915c),
    (0x099160, i_099160),
    (0x099166, i_099166),
    (0x09916C, i_09916c),
    (0x099172, i_099172),
    (0x099178, i_099178),
    (0x09917C, i_09917c),
    (0x09917E, i_09917e),
    (0x099180, i_099180),
    (0x099184, i_099184),
    (0x09918A, i_09918a),
    (0x09918C, i_09918c),
    (0x099190, i_099190),
    (0x099196, i_099196),
    (0x09919A, i_09919a),
    (0x0991A0, i_0991a0),
    (0x0991A4, i_0991a4),
    (0x0991A6, i_0991a6),
    (0x0991AC, i_0991ac),
    (0x0991B2, i_0991b2),
    (0x0991B6, i_0991b6),
    (0x0991BA, i_0991ba),
    (0x0991BC, i_0991bc),
    (0x0991C2, i_0991c2),
    (0x0991C8, i_0991c8),
    (0x0991CE, i_0991ce),
    (0x0991D2, i_0991d2),
    (0x0991D6, i_0991d6),
    (0x0991D8, i_0991d8),
    (0x0991DA, i_0991da),
    (0x0991DE, i_0991de),
    (0x0991E4, i_0991e4),
    (0x0991EA, i_0991ea),
    (0x0991F0, i_0991f0),
    (0x0991F6, i_0991f6),
    (0x0991FA, i_0991fa),
    (0x099200, i_099200),
    (0x099206, i_099206),
    (0x09920C, i_09920c),
    (0x099212, i_099212),
    (0x099216, i_099216),
    (0x099218, i_099218),
    (0x09921A, i_09921a),
    (0x099220, i_099220),
    (0x099226, i_099226),
    (0x09922C, i_09922c),
    (0x099232, i_099232),
    (0x099236, i_099236),
    (0x099238, i_099238),
    (0x09923E, i_09923e),
    (0x099244, i_099244),
    (0x099248, i_099248),
    (0x09924C, i_09924c),
    (0x099250, i_099250),
    (0x099254, i_099254),
    (0x09925A, i_09925a),
    (0x09925E, i_09925e),
    (0x099262, i_099262),
    (0x099264, i_099264),
    (0x09926A, i_09926a),
    (0x099270, i_099270),
    (0x099276, i_099276),
    (0x09927C, i_09927c),
    (0x099280, i_099280),
    (0x099284, i_099284),
    (0x099286, i_099286),
    (0x099288, i_099288),
    (0x09928E, i_09928e),
    (0x099294, i_099294),
    (0x09929A, i_09929a),
    (0x09929E, i_09929e),
    (0x0992A0, i_0992a0),
    (0x0992A6, i_0992a6),
    (0x0992AC, i_0992ac),
    (0x0992B0, i_0992b0),
    (0x0992B4, i_0992b4),
    (0x0992B8, i_0992b8),
    (0x0992BA, i_0992ba),
    (0x0992C0, i_0992c0),
    (0x0992C6, i_0992c6),
    (0x0992CC, i_0992cc),
    (0x0992D0, i_0992d0),
    (0x0992D4, i_0992d4),
    (0x0992D6, i_0992d6),
    (0x0992D8, i_0992d8),
    (0x0992DC, i_0992dc),
    (0x0992E2, i_0992e2),
    (0x0992E6, i_0992e6),
    (0x0992EA, i_0992ea),
    (0x0992F0, i_0992f0),
    (0x0992F4, i_0992f4),
    (0x0992F8, i_0992f8),
    (0x0992FE, i_0992fe),
    (0x099304, i_099304),
    (0x09930A, i_09930a),
    (0x099310, i_099310),
    (0x099314, i_099314),
    (0x09931A, i_09931a),
    (0x099320, i_099320),
    (0x099326, i_099326),
    (0x09932C, i_09932c),
    (0x099332, i_099332),
    (0x099338, i_099338),
    (0x09933E, i_09933e),
    (0x099342, i_099342),
    (0x099344, i_099344),
    (0x099346, i_099346),
    (0x09934A, i_09934a),
    (0x099350, i_099350),
    (0x099354, i_099354),
    (0x099358, i_099358),
    (0x09935E, i_09935e),
    (0x099362, i_099362),
    (0x099366, i_099366),
    (0x09936C, i_09936c),
    (0x099372, i_099372),
    (0x099378, i_099378),
    (0x09937C, i_09937c),
    (0x099382, i_099382),
    (0x099388, i_099388),
    (0x09938E, i_09938e),
    (0x099394, i_099394),
    (0x09939A, i_09939a),
    (0x09939E, i_09939e),
    (0x0993A0, i_0993a0),
    (0x0993A2, i_0993a2),
    (0x0993A8, i_0993a8),
    (0x0993AE, i_0993ae),
    (0x0993B4, i_0993b4),
    (0x0993BA, i_0993ba),
    (0x0993BE, i_0993be),
    (0x0993C0, i_0993c0),
    (0x0993C6, i_0993c6),
    (0x0993CC, i_0993cc),
    (0x0993D0, i_0993d0),
    (0x0993D4, i_0993d4),
    (0x0993D6, i_0993d6),
    (0x0993DC, i_0993dc),
    (0x0993E2, i_0993e2),
    (0x0993E8, i_0993e8),
    (0x0993EE, i_0993ee),
    (0x0993F2, i_0993f2),
    (0x0993F6, i_0993f6),
    (0x0993F8, i_0993f8),
    (0x0993FA, i_0993fa),
    (0x099400, i_099400),
    (0x099406, i_099406),
    (0x09940A, i_09940a),
    (0x09940C, i_09940c),
    (0x099412, i_099412),
    (0x099418, i_099418),
    (0x09941C, i_09941c),
    (0x099420, i_099420),
    (0x099422, i_099422),
    (0x099428, i_099428),
    (0x09942E, i_09942e),
    (0x099434, i_099434),
    (0x09943A, i_09943a),
    (0x09943E, i_09943e),
    (0x099442, i_099442),
    (0x099444, i_099444),
    (0x099446, i_099446),
    (0x09944A, i_09944a),
    (0x099450, i_099450),
    (0x099454, i_099454),
    (0x099458, i_099458),
    (0x09945E, i_09945e),
    (0x099462, i_099462),
    (0x099466, i_099466),
    (0x09946C, i_09946c),
    (0x099472, i_099472),
    (0x099476, i_099476),
    (0x09947C, i_09947c),
    (0x099482, i_099482),
    (0x099488, i_099488),
    (0x09948E, i_09948e),
    (0x099494, i_099494),
    (0x09949A, i_09949a),
    (0x09949E, i_09949e),
    (0x0994A0, i_0994a0),
    (0x0994A2, i_0994a2),
    (0x0994A6, i_0994a6),
    (0x0994AC, i_0994ac),
    (0x0994B0, i_0994b0),
    (0x0994B4, i_0994b4),
    (0x0994BA, i_0994ba),
    (0x0994BE, i_0994be),
    (0x0994C2, i_0994c2),
    (0x0994C8, i_0994c8),
    (0x0994CE, i_0994ce),
    (0x0994D2, i_0994d2),
    (0x0994D8, i_0994d8),
    (0x0994DE, i_0994de),
    (0x0994E4, i_0994e4),
    (0x0994EA, i_0994ea),
    (0x0994F0, i_0994f0),
    (0x0994F6, i_0994f6),
    (0x0994FA, i_0994fa),
    (0x0994FC, i_0994fc),
    (0x0994FE, i_0994fe),
    (0x099502, i_099502),
    (0x099508, i_099508),
    (0x09950E, i_09950e),
    (0x099514, i_099514),
    (0x09951A, i_09951a),
    (0x09951E, i_09951e),
    (0x099524, i_099524),
    (0x09952A, i_09952a),
    (0x099530, i_099530),
    (0x099536, i_099536),
    (0x09953A, i_09953a),
    (0x09953C, i_09953c),
    (0x09953E, i_09953e),
    (0x099544, i_099544),
    (0x09954A, i_09954a),
    (0x099550, i_099550),
    (0x099554, i_099554),
    (0x099556, i_099556),
    (0x09955C, i_09955c),
    (0x099562, i_099562),
    (0x099568, i_099568),
    (0x09956C, i_09956c),
    (0x099570, i_099570),
    (0x099572, i_099572),
    (0x099578, i_099578),
    (0x09957E, i_09957e),
    (0x099584, i_099584),
    (0x099588, i_099588),
    (0x09958C, i_09958c),
    (0x09958E, i_09958e),
    (0x099590, i_099590),
    (0x099596, i_099596),
    (0x09959C, i_09959c),
    (0x0995A0, i_0995a0),
    (0x0995A2, i_0995a2),
    (0x0995A8, i_0995a8),
    (0x0995AE, i_0995ae),
    (0x0995B4, i_0995b4),
    (0x0995BA, i_0995ba),
    (0x0995C0, i_0995c0),
    (0x0995C4, i_0995c4),
    (0x0995C8, i_0995c8),
    (0x0995CA, i_0995ca),
    (0x0995CC, i_0995cc),
    (0x0995D0, i_0995d0),
    (0x0995D6, i_0995d6),
    (0x0995DC, i_0995dc),
    (0x0995E2, i_0995e2),
    (0x0995E8, i_0995e8),
    (0x0995EC, i_0995ec),
    (0x0995F0, i_0995f0),
    (0x0995F6, i_0995f6),
    (0x0995FC, i_0995fc),
    (0x099602, i_099602),
    (0x099608, i_099608),
    (0x09960C, i_09960c),
    (0x09960E, i_09960e),
    (0x099610, i_099610),
    (0x099616, i_099616),
    (0x09961C, i_09961c),
    (0x099622, i_099622),
    (0x099626, i_099626),
    (0x099628, i_099628),
    (0x09962E, i_09962e),
    (0x099634, i_099634),
    (0x099638, i_099638),
    (0x09963C, i_09963c),
    (0x099642, i_099642),
    (0x099646, i_099646),
    (0x09964C, i_09964c),
    (0x099650, i_099650),
    (0x099652, i_099652),
    (0x099658, i_099658),
    (0x09965E, i_09965e),
    (0x099664, i_099664),
    (0x099668, i_099668),
    (0x09966C, i_09966c),
    (0x09966E, i_09966e),
    (0x099670, i_099670),
    (0x099676, i_099676),
    (0x09967C, i_09967c),
    (0x099680, i_099680),
    (0x099682, i_099682),
    (0x099688, i_099688),
    (0x09968E, i_09968e),
    (0x099692, i_099692),
    (0x099696, i_099696),
    (0x099698, i_099698),
    (0x09969E, i_09969e),
    (0x0996A4, i_0996a4),
    (0x0996AA, i_0996aa),
    (0x0996AE, i_0996ae),
    (0x0996B2, i_0996b2),
    (0x0996B4, i_0996b4),
    (0x0996B6, i_0996b6),
    (0x0996BA, i_0996ba),
    (0x0996C0, i_0996c0),
    (0x0996C6, i_0996c6),
    (0x0996CC, i_0996cc),
    (0x0996D0, i_0996d0),
    (0x0996D4, i_0996d4),
    (0x0996DA, i_0996da),
    (0x0996E0, i_0996e0),
    (0x0996E6, i_0996e6),
    (0x0996EA, i_0996ea),
    (0x0996EE, i_0996ee),
    (0x0996F4, i_0996f4),
    (0x0996FA, i_0996fa),
    (0x099700, i_099700),
    (0x099704, i_099704),
    (0x099708, i_099708),
    (0x09970E, i_09970e),
    (0x099714, i_099714),
    (0x09971A, i_09971a),
    (0x09971E, i_09971e),
    (0x099722, i_099722),
    (0x099728, i_099728),
    (0x09972E, i_09972e),
    (0x099734, i_099734),
    (0x09973A, i_09973a),
    (0x099740, i_099740),
    (0x099746, i_099746),
    (0x09974A, i_09974a),
    (0x099750, i_099750),
    (0x099756, i_099756),
    (0x09975C, i_09975c),
    (0x099762, i_099762),
    (0x099766, i_099766),
    (0x099768, i_099768),
    (0x09976A, i_09976a),
    (0x099770, i_099770),
    (0x099776, i_099776),
    (0x09977C, i_09977c),
    (0x099782, i_099782),
    (0x099788, i_099788),
    (0x09978C, i_09978c),
    (0x099790, i_099790),
    (0x099792, i_099792),
    (0x099794, i_099794),
    (0x09979A, i_09979a),
    (0x09979C, i_09979c),
    (0x0997A0, i_0997a0),
    (0x0997A4, i_0997a4),
    (0x0997A8, i_0997a8),
    (0x0997AC, i_0997ac),
    (0x0997B0, i_0997b0),
    (0x0997B2, i_0997b2),
    (0x0997B6, i_0997b6),
    (0x0997B8, i_0997b8),
    (0x0997BE, i_0997be),
    (0x0997C4, i_0997c4),
    (0x0997CA, i_0997ca),
    (0x0997CC, i_0997cc),
    (0x0997D6, i_0997d6),
    (0x0997E0, i_0997e0),
    (0x0997E2, i_0997e2),
    (0x0997E8, i_0997e8),
    (0x0997EC, i_0997ec),
    (0x0997EE, i_0997ee),
    (0x0997F2, i_0997f2),
    (0x0997F6, i_0997f6),
    (0x0997FA, i_0997fa),
    (0x0997FE, i_0997fe),
    (0x099802, i_099802),
    (0x099806, i_099806),
    (0x099808, i_099808),
    (0x09980A, i_09980a),
    (0x09980E, i_09980e),
    (0x099812, i_099812),
    (0x099818, i_099818),
    (0x09981C, i_09981c),
    (0x09981E, i_09981e),
    (0x099822, i_099822),
    (0x099826, i_099826),
    (0x09982C, i_09982c),
    (0x09982E, i_09982e),
    (0x099832, i_099832),
    (0x099836, i_099836),
    (0x09983A, i_09983a),
    (0x099840, i_099840),
    (0x099842, i_099842),
    (0x099844, i_099844),
    (0x099848, i_099848),
    (0x09984C, i_09984c),
    (0x09984E, i_09984e),
    (0x099852, i_099852),
    (0x099854, i_099854),
    (0x099856, i_099856),
    (0x09985A, i_09985a),
    (0x099860, i_099860),
    (0x099866, i_099866),
    (0x09986A, i_09986a),
    (0x09986C, i_09986c),
    (0x09986E, i_09986e),
    (0x099872, i_099872),
    (0x099876, i_099876),
    (0x09987A, i_09987a),
    (0x09987E, i_09987e),
    (0x099882, i_099882),
    (0x099886, i_099886),
    (0x099888, i_099888),
    (0x09988A, i_09988a),
    (0x09988C, i_09988c),
    (0x099890, i_099890),
    (0x099892, i_099892),
    (0x099896, i_099896),
    (0x09989C, i_09989c),
    (0x0998A0, i_0998a0),
    (0x0998A4, i_0998a4),
    (0x0998A8, i_0998a8),
    (0x0998AC, i_0998ac),
    (0x0998B0, i_0998b0),
    (0x0998B4, i_0998b4),
    (0x0998B8, i_0998b8),
    (0x0998BC, i_0998bc),
    (0x0998C0, i_0998c0),
    (0x0998C2, i_0998c2),
    (0x0998C8, i_0998c8),
    (0x0998CA, i_0998ca),
    (0x0998CE, i_0998ce),
    (0x0998D4, i_0998d4),
    (0x0998DA, i_0998da),
    (0x0998E0, i_0998e0),
    (0x0998E4, i_0998e4),
    (0x0998E8, i_0998e8),
    (0x0998EC, i_0998ec),
    (0x0998EE, i_0998ee),
    (0x0998F2, i_0998f2),
    (0x0998F6, i_0998f6),
    (0x0998FA, i_0998fa),
    (0x0998FE, i_0998fe),
    (0x099902, i_099902),
    (0x099904, i_099904),
    (0x099908, i_099908),
    (0x09990C, i_09990c),
    (0x099910, i_099910),
    (0x099914, i_099914),
    (0x099916, i_099916),
    (0x09991A, i_09991a),
    (0x09991E, i_09991e),
    (0x099924, i_099924),
    (0x099928, i_099928),
    (0x09992C, i_09992c),
    (0x099930, i_099930),
    (0x099934, i_099934),
    (0x099938, i_099938),
    (0x09993A, i_09993a),
    (0x09993C, i_09993c),
    (0x099940, i_099940),
    (0x099946, i_099946),
    (0x09994C, i_09994c),
    (0x099952, i_099952),
    (0x099956, i_099956),
    (0x09995A, i_09995a),
    (0x09995E, i_09995e),
    (0x099962, i_099962),
    (0x099966, i_099966),
    (0x09996C, i_09996c),
    (0x099970, i_099970),
    (0x099974, i_099974),
    (0x099976, i_099976),
    (0x09997A, i_09997a),
    (0x09997C, i_09997c),
    (0x099980, i_099980),
    (0x099984, i_099984),
    (0x099986, i_099986),
    (0x09998A, i_09998a),
    (0x09998C, i_09998c),
    (0x099990, i_099990),
    (0x099994, i_099994),
    (0x099998, i_099998),
    (0x09999C, i_09999c),
    (0x09999E, i_09999e),
    (0x0999A2, i_0999a2),
    (0x0999A6, i_0999a6),
    (0x0999AA, i_0999aa),
    (0x0999AC, i_0999ac),
    (0x0999B0, i_0999b0),
    (0x0999B4, i_0999b4),
    (0x0999B8, i_0999b8),
    (0x0999BA, i_0999ba),
    (0x0999BE, i_0999be),
    (0x0999C2, i_0999c2),
    (0x0999C6, i_0999c6),
    (0x0999C8, i_0999c8),
    (0x0999CA, i_0999ca),
    (0x0999CE, i_0999ce),
    (0x0999D2, i_0999d2),
    (0x0999D6, i_0999d6),
    (0x0999D8, i_0999d8),
    (0x0999DC, i_0999dc),
    (0x0999DE, i_0999de),
    (0x0999E2, i_0999e2),
    (0x0999E4, i_0999e4),
    (0x0999E8, i_0999e8),
    (0x0999EC, i_0999ec),
    (0x0999F0, i_0999f0),
    (0x0999F4, i_0999f4),
    (0x0999F6, i_0999f6),
    (0x0999F8, i_0999f8),
    (0x0999FA, i_0999fa),
    (0x0999FE, i_0999fe),
    (0x099A00, i_099a00),
    (0x099A06, i_099a06),
    (0x099A0C, i_099a0c),
    (0x099A12, i_099a12),
    (0x099A16, i_099a16),
    (0x099A1C, i_099a1c),
    (0x099A20, i_099a20),
    (0x099A22, i_099a22),
    (0x099A26, i_099a26),
    (0x099A2A, i_099a2a),
    (0x099A2E, i_099a2e),
    (0x099A32, i_099a32),
    (0x099A36, i_099a36),
    (0x099A38, i_099a38),
    (0x099A3A, i_099a3a),
    (0x099A3C, i_099a3c),
    (0x099A40, i_099a40),
    (0x099A44, i_099a44),
    (0x099A48, i_099a48),
    (0x099A4A, i_099a4a),
    (0x099A4E, i_099a4e),
    (0x099A54, i_099a54),
    (0x099A5C, i_099a5c),
    (0x099A60, i_099a60),
    (0x099A62, i_099a62),
    (0x099A64, i_099a64),
    (0x099A6A, i_099a6a),
    (0x099A6E, i_099a6e),
    (0x099A72, i_099a72),
    (0x099A76, i_099a76),
    (0x099A7A, i_099a7a),
    (0x099A7E, i_099a7e),
    (0x099A82, i_099a82),
    (0x099A86, i_099a86),
    (0x099A8A, i_099a8a),
    (0x099A8E, i_099a8e),
    (0x099A92, i_099a92),
    (0x099A96, i_099a96),
    (0x099A9A, i_099a9a),
    (0x099AA0, i_099aa0),
    (0x099AA4, i_099aa4),
    (0x099AA8, i_099aa8),
    (0x099AAC, i_099aac),
    (0x099AAE, i_099aae),
    (0x099AB2, i_099ab2),
    (0x099AB6, i_099ab6),
    (0x099ABA, i_099aba),
    (0x099ABE, i_099abe),
    (0x099AC4, i_099ac4),
    (0x099AC6, i_099ac6),
    (0x099ACA, i_099aca),
    (0x099ACE, i_099ace),
    (0x099AD2, i_099ad2),
    (0x099AD6, i_099ad6),
    (0x099ADA, i_099ada),
    (0x099ADC, i_099adc),
    (0x099AE0, i_099ae0),
    (0x099AE2, i_099ae2),
    (0x099AEA, i_099aea),
    (0x099AF2, i_099af2),
    (0x099AFA, i_099afa),
    (0x099AFC, i_099afc),
    (0x099B04, i_099b04),
    (0x099B06, i_099b06),
    (0x099B0A, i_099b0a),
    (0x099B0E, i_099b0e),
    (0x099B14, i_099b14),
    (0x099B1A, i_099b1a),
    (0x099B1E, i_099b1e),
    (0x099B22, i_099b22),
    (0x099B28, i_099b28),
    (0x099B2E, i_099b2e),
    (0x099B32, i_099b32),
    (0x099B36, i_099b36),
    (0x099B3A, i_099b3a),
    (0x099B3C, i_099b3c),
    (0x099B40, i_099b40),
    (0x099B44, i_099b44),
    (0x099B48, i_099b48),
    (0x099B4C, i_099b4c),
    (0x099B50, i_099b50),
    (0x099B56, i_099b56),
    (0x099B5C, i_099b5c),
    (0x099B62, i_099b62),
    (0x099B64, i_099b64),
    (0x099B68, i_099b68),
    (0x099B6C, i_099b6c),
    (0x099B70, i_099b70),
    (0x099B74, i_099b74),
    (0x099B78, i_099b78),
    (0x099B7E, i_099b7e),
    (0x099B82, i_099b82),
    (0x099B84, i_099b84),
    (0x099B88, i_099b88),
    (0x099B8C, i_099b8c),
    (0x099B90, i_099b90),
    (0x099B94, i_099b94),
    (0x099B98, i_099b98),
    (0x099B9E, i_099b9e),
    (0x099BA2, i_099ba2),
    (0x099BA4, i_099ba4),
    (0x099BA6, i_099ba6),
    (0x099BAC, i_099bac),
    (0x099BB2, i_099bb2),
    (0x099BBA, i_099bba),
    (0x099BBE, i_099bbe),
    (0x099BC4, i_099bc4),
    (0x099BC8, i_099bc8),
    (0x099BCE, i_099bce),
    (0x099BD2, i_099bd2),
    (0x099BD8, i_099bd8),
    (0x099BDC, i_099bdc),
    (0x099BE2, i_099be2),
    (0x099BE6, i_099be6),
    (0x099BEC, i_099bec),
    (0x099BF0, i_099bf0),
    (0x099BF6, i_099bf6),
    (0x099BFA, i_099bfa),
    (0x099C00, i_099c00),
    (0x099C04, i_099c04),
    (0x099C0A, i_099c0a),
    (0x099C0E, i_099c0e),
    (0x099C14, i_099c14),
    (0x099C18, i_099c18),
    (0x099C1E, i_099c1e),
    (0x099C26, i_099c26),
    (0x099C2A, i_099c2a),
    (0x099C2E, i_099c2e),
    (0x099C34, i_099c34),
    (0x099C38, i_099c38),
    (0x099C3E, i_099c3e),
    (0x099C42, i_099c42),
    (0x099C48, i_099c48),
    (0x099C4C, i_099c4c),
    (0x099C52, i_099c52),
    (0x099C56, i_099c56),
    (0x099C58, i_099c58),
    (0x099C5E, i_099c5e),
    (0x099C62, i_099c62),
    (0x099C68, i_099c68),
    (0x099C6E, i_099c6e),
    (0x099C76, i_099c76),
    (0x099C7C, i_099c7c),
    (0x099C80, i_099c80),
    (0x099C86, i_099c86),
    (0x099C8C, i_099c8c),
    (0x099C90, i_099c90),
    (0x099C96, i_099c96),
    (0x099C9E, i_099c9e),
    (0x099CA2, i_099ca2),
    (0x099CA8, i_099ca8),
    (0x099CAC, i_099cac),
    (0x099CB2, i_099cb2),
    (0x099CB8, i_099cb8),
    (0x099CBC, i_099cbc),
    (0x099CC2, i_099cc2),
    (0x099CCA, i_099cca),
    (0x099CD0, i_099cd0),
];
