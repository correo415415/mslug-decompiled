// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p01_0) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


// ===== Pal_LoadRaw16_0133b0  ($0133B0, 54 B, video/palette_engine_sprite_grid_0133b0.s)
/// $0133B0: movea.l a3, a4
pub fn i_0133b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x133B2;
    cpu.ir = 0x284B;
    let v1 = cpu.a[3] & 0xFFFFFFFF;
    cpu.a[4] = v1;
    4
}
/// $0133B2: addq.w #$2, a3
pub fn i_0133b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x133B4;
    cpu.ir = 0x544B;
    cpu.a[3] = cpu.a[3].wrapping_add(2);
    8
}
/// $0133B4: addq.l #$4, a2
pub fn i_0133b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x133B6;
    cpu.ir = 0x588A;
    cpu.a[2] = cpu.a[2].wrapping_add(4);
    8
}
/// $0133B6: move.w #$e, d5
pub fn i_0133b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x133BA;
    cpu.ir = 0x3A3C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0133BA: moveq #$0, d4
pub fn i_0133ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x133BC;
    cpu.ir = 0x7800;
    cpu.d[4] = 0x0;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0133BC: move.b (a2)+, d4
pub fn i_0133bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x133BE;
    cpu.ir = 0x181A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    8
}
/// $0133BE: lsl.w #$5, d4
pub fn i_0133be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x133C0;
    cpu.ir = 0xEB4C;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 1, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $0133C0: or.b (a2)+, d4
pub fn i_0133c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x133C2;
    cpu.ir = 0x881A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    let r = (v3 | cpu.d[4]) & 0xFF;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    8
}
/// $0133C2: lsl.w #$5, d4
pub fn i_0133c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x133C4;
    cpu.ir = 0xEB4C;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 1, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $0133C4: or.b (a2)+, d4
pub fn i_0133c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x133C6;
    cpu.ir = 0x881A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    let r = (v3 | cpu.d[4]) & 0xFF;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    8
}
/// $0133C6: addq.l #$1, a2
pub fn i_0133c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x133C8;
    cpu.ir = 0x528A;
    cpu.a[2] = cpu.a[2].wrapping_add(1);
    8
}
/// $0133C8: lea.l $2f30.l, a0
pub fn i_0133c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x133CE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F30;
    20
}
/// $0133CE: add.w d4, d4
pub fn i_0133ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x133D0;
    cpu.ir = 0xD844;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0133D0: adda.l d4, a0
pub fn i_0133d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x133D2;
    cpu.ir = 0xD1C4;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add(v1);
    8
}
/// $0133D2: move.w (a0), d4
pub fn i_0133d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x133D4;
    cpu.ir = 0x3810;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $0133D4: move.w d4, (a3)+
pub fn i_0133d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x133D6;
    cpu.ir = 0x36C4;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $0133D6: dbra d5, $133ba
pub fn i_0133d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x133DA;
    cpu.ir = 0x51CD;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x133DA; return 12; }
    let n = (cpu.d[5] as u16).wrapping_sub(1);
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x133DA; return 14; }
    cpu.pc = 0x133BA;
    10
}
/// $0133DA: andi.b #$fe, (a1, d1.w)
pub fn i_0133da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x133E0;
    cpu.ir = 0x231;
    let ad1 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0xFE) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    18
}
/// $0133E0: move.w #$1, (a4)
pub fn i_0133e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x133E4;
    cpu.ir = 0x38BC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[4], v1 as u16);
    8
}
/// $0133E4: rts
pub fn i_0133e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x133E6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pal_ClearSlot16_0133e6  ($0133E6, 34 B, video/palette_engine_sprite_grid_0133b0.s)
/// $0133E6: movea.l a3, a4
pub fn i_0133e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x133E8;
    cpu.ir = 0x284B;
    let v1 = cpu.a[3] & 0xFFFFFFFF;
    cpu.a[4] = v1;
    4
}
/// $0133E8: addq.w #$2, a3
pub fn i_0133e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x133EA;
    cpu.ir = 0x544B;
    cpu.a[3] = cpu.a[3].wrapping_add(2);
    8
}
/// $0133EA: moveq #$0, d4
pub fn i_0133ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x133EC;
    cpu.ir = 0x7800;
    cpu.d[4] = 0x0;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0133EC: move.w d4, (a3)+
pub fn i_0133ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x133EE;
    cpu.ir = 0x36C4;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $0133EE: move.l d4, (a3)+
pub fn i_0133ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x133F0;
    cpu.ir = 0x26C4;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0133F0: move.l d4, (a3)+
pub fn i_0133f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x133F2;
    cpu.ir = 0x26C4;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0133F2: move.l d4, (a3)+
pub fn i_0133f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x133F4;
    cpu.ir = 0x26C4;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0133F4: move.l d4, (a3)+
pub fn i_0133f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x133F6;
    cpu.ir = 0x26C4;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0133F6: move.l d4, (a3)+
pub fn i_0133f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x133F8;
    cpu.ir = 0x26C4;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0133F8: move.l d4, (a3)+
pub fn i_0133f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x133FA;
    cpu.ir = 0x26C4;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0133FA: move.l d4, (a3)+
pub fn i_0133fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x133FC;
    cpu.ir = 0x26C4;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0133FC: move.b #$80, (a1, d1.w)
pub fn i_0133fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13402;
    cpu.ir = 0x13BC;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $013402: move.w #$1, (a4)
pub fn i_013402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13406;
    cpu.ir = 0x38BC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[4], v1 as u16);
    8
}
/// $013406: rts
pub fn i_013406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13408;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PalAnim_StepSlot_013408  ($013408, 120 B, video/palette_engine_sprite_grid_0133b0.s)
/// $013408: movem.l d0-d7/a0-a6, -(a7)
pub fn i_013408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1340C;
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
/// $01340C: move.w $8(a1, d1.w), d2
pub fn i_01340c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13410;
    cpu.ir = 0x3431;
    let ad2 = cpu.a[1].wrapping_add(0x8).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $013410: cmpi.w #$ffff, d2
pub fn i_013410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13414;
    cpu.ir = 0xC42;
    let v1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFFF, Size::Word, false);
    8
}
/// $013414: beq.w $13462
pub fn i_013414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13418;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x13462; return 10; }
    8
}
/// $013418: subq.w #$1, $10(a1, d1.w)
pub fn i_013418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1341C;
    cpu.ir = 0x5371;
    let ad1 = cpu.a[1].wrapping_add(0x10).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    14
}
/// $01341C: tst.w $10(a1, d1.w)
pub fn i_01341c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13420;
    cpu.ir = 0x4A71;
    let ad2 = cpu.a[1].wrapping_add(0x10).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    14
}
/// $013420: bne.w $1345c
pub fn i_013420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13424;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x1345C; return 10; }
    8
}
/// $013424: move.w $8(a1, d1.w), d2
pub fn i_013424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13428;
    cpu.ir = 0x3431;
    let ad2 = cpu.a[1].wrapping_add(0x8).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $013428: move.w d2, $10(a1, d1.w)
pub fn i_013428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1342C;
    cpu.ir = 0x3382;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x10).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $01342C: move.b $f(a1, d1.w), d2
pub fn i_01342c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13430;
    cpu.ir = 0x1431;
    let ad2 = cpu.a[1].wrapping_add(0xF).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    14
}
/// $013430: bne.w $13438
pub fn i_013430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13434;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x13438; return 10; }
    8
}
/// $013434: move.b #$1, d2
pub fn i_013434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13438;
    cpu.ir = 0x143C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $013438: add.b d2, $e(a1, d1.w)
pub fn i_013438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1343C;
    cpu.ir = 0xD531;
    let ad1 = cpu.a[1].wrapping_add(0xE).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[2] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    14
}
/// $01343C: cmpi.b #$20, $e(a1, d1.w)
pub fn i_01343c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13442;
    cpu.ir = 0xC31;
    let ad2 = cpu.a[1].wrapping_add(0xE).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x20, Size::Byte, false);
    18
}
/// $013442: bge.w $13462
pub fn i_013442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13446;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x13462; return 10; }
    8
}
/// $013446: movea.l $2(a1, d1.w), a2
pub fn i_013446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1344A;
    cpu.ir = 0x2471;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    18
}
/// $01344A: movea.l $a(a1, d1.w), a3
pub fn i_01344a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1344E;
    cpu.ir = 0x2671;
    let ad2 = cpu.a[1].wrapping_add(0xA).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[3] = v3;
    18
}
/// $01344E: move.b $e(a1, d1.w), d5
pub fn i_01344e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13452;
    cpu.ir = 0x1A31;
    let ad2 = cpu.a[1].wrapping_add(0xE).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    14
}
/// $013452: jsr $13480(pc)
pub fn i_013452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13456;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x13456);
    cpu.pc = 0x13480;
    16
}
/// $013456: and.w d7, d7
pub fn i_013456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13458;
    cpu.ir = 0xCE47;
    let v1 = cpu.d[7] & 0xFFFF;
    let r = (v1 & cpu.d[7]) & 0xFFFF;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $013458: bne.w $13462
pub fn i_013458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1345C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x13462; return 10; }
    8
}
/// $01345C: movem.l (a7)+, d0-d7/a0-a6
pub fn i_01345c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13460;
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
/// $013460: rts
pub fn i_013460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13462;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $013462: move.l $a(a1, d1.w), d2
pub fn i_013462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13466;
    cpu.ir = 0x2431;
    let ad2 = cpu.a[1].wrapping_add(0xA).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $013466: move.l d2, $2(a1, d1.w)
pub fn i_013466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1346A;
    cpu.ir = 0x2382;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $01346A: andi.b #$fd, (a1, d1.w)
pub fn i_01346a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13470;
    cpu.ir = 0x231;
    let ad1 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0xFD) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    18
}
/// $013470: ori.b #$1, (a1, d1.w)
pub fn i_013470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13476;
    cpu.ir = 0x31;
    let ad1 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    18
}
/// $013476: clr.w $8(a1, d1.w)
pub fn i_013476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1347A;
    cpu.ir = 0x4271;
    let ad1 = cpu.a[1].wrapping_add(0x8).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    14
}
/// $01347A: movem.l (a7)+, d0-d7/a0-a6
pub fn i_01347a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1347E;
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
/// $01347E: rts
pub fn i_01347e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13480;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PalAnim_Blend16_013480  ($013480, 132 B, video/palette_engine_sprite_grid_0133b0.s)
/// $013480: movem.l d1/a1, -(a7)
pub fn i_013480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13484;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $013484: lea.l $10a2d4.l, a4
pub fn i_013484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1348A;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0x10A2D4;
    20
}
/// $01348A: move.w #$f, d6
pub fn i_01348a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1348E;
    cpu.ir = 0x3C3C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $01348E: moveq #$0, d0
pub fn i_01348e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13490;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $013490: moveq #$0, d4
pub fn i_013490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13492;
    cpu.ir = 0x7800;
    cpu.d[4] = 0x0;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $013492: move.w #$ffff, d7
pub fn i_013492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13496;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $013496: jsr $13504(pc)
pub fn i_013496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1349A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x1349A);
    cpu.pc = 0x13504;
    16
}
/// $01349A: andi.w #$1f, d2
pub fn i_01349a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1349E;
    cpu.ir = 0x242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $01349E: andi.w #$1f, d3
pub fn i_01349e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x134A2;
    cpu.ir = 0x243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0134A2: andi.w #$1f, d4
pub fn i_0134a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x134A6;
    cpu.ir = 0x244;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0134A6: tst.b $10a2c8.l
pub fn i_0134a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x134AC;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10A2C8) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $0134AC: beq.w $134cc
pub fn i_0134ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x134B0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x134CC; return 10; }
    8
}
/// $0134B0: cmpi.b #$ff, $10a2c8.l
pub fn i_0134b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x134B8;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10A2C8) as u32;
    sub_with_flags(cpu, v2, 0xFF, Size::Byte, false);
    20
}
/// $0134B8: beq.w $134c4
pub fn i_0134b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x134BC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x134C4; return 10; }
    8
}
/// $0134BC: jsr $13624(pc)
pub fn i_0134bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x134C0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x134C0);
    cpu.pc = 0x13624;
    16
}
/// $0134C0: bra.w $134d4
pub fn i_0134c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x134C4;
    cpu.ir = 0x6000;
    cpu.pc = 0x134D4;
    10
}
/// $0134C4: jsr $13694(pc)
pub fn i_0134c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x134C8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x134C8);
    cpu.pc = 0x13694;
    16
}
/// $0134C8: bra.w $134d4
pub fn i_0134c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x134CC;
    cpu.ir = 0x6000;
    cpu.pc = 0x134D4;
    10
}
/// $0134CC: lsl.w #$5, d2
pub fn i_0134cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x134CE;
    cpu.ir = 0xEB4A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 1, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $0134CE: or.w d3, d2
pub fn i_0134ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x134D0;
    cpu.ir = 0x8443;
    let v1 = cpu.d[3] & 0xFFFF;
    let r = (v1 | cpu.d[2]) & 0xFFFF;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $0134D0: lsl.w #$5, d2
pub fn i_0134d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x134D2;
    cpu.ir = 0xEB4A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 1, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $0134D2: or.w d2, d4
pub fn i_0134d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x134D4;
    cpu.ir = 0x8842;
    let v1 = cpu.d[2] & 0xFFFF;
    let r = (v1 | cpu.d[4]) & 0xFFFF;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $0134D4: lea.l $2f30.l, a0
pub fn i_0134d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x134DA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F30;
    20
}
/// $0134DA: andi.l #$ffff, d4
pub fn i_0134da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x134E0;
    cpu.ir = 0x284;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[4] = r2;
    8
}
/// $0134E0: add.l d4, d4
pub fn i_0134e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x134E2;
    cpu.ir = 0xD884;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[4] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[4] = r;
    4
}
/// $0134E2: adda.l d4, a0
pub fn i_0134e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x134E4;
    cpu.ir = 0xD1C4;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add(v1);
    8
}
/// $0134E4: move.w (a0), d4
pub fn i_0134e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x134E6;
    cpu.ir = 0x3810;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $0134E6: move.w d4, (a4, d1.w)
pub fn i_0134e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x134EA;
    cpu.ir = 0x3984;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $0134EA: and.w d0, d7
pub fn i_0134ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x134EC;
    cpu.ir = 0xCE40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = (v1 & cpu.d[7]) & 0xFFFF;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $0134EC: addq.w #$2, d1
pub fn i_0134ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x134EE;
    cpu.ir = 0x5441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0134EE: dbra d6, $13496
pub fn i_0134ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x134F2;
    cpu.ir = 0x51CE;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x134F2; return 12; }
    let n = (cpu.d[6] as u16).wrapping_sub(1);
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x134F2; return 14; }
    cpu.pc = 0x13496;
    10
}
/// $0134F2: movem.l (a7)+, d1/a1
pub fn i_0134f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x134F6;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $0134F6: lea.l $10a2d4.l, a4
pub fn i_0134f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x134FC;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0x10A2D4;
    20
}
/// $0134FC: move.w #$1, (a4, d1.w)
pub fn i_0134fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13502;
    cpu.ir = 0x39BC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $013502: rts
pub fn i_013502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13504;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PalAnim_StepRGB_013504  ($013504, 252 B, video/palette_engine_sprite_grid_0133b0.s)
/// $013504: move.w d1, -(a7)
pub fn i_013504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13506;
    cpu.ir = 0x3F01;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $013506: movem.l d5-d7/a0, -(a7)
pub fn i_013506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1350A;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[7]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $01350A: tst.b d5
pub fn i_01350a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1350C;
    cpu.ir = 0x4A05;
    let v1 = cpu.d[5] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $01350C: beq.w $135f6
pub fn i_01350c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13510;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x135F6; return 10; }
    8
}
/// $013510: andi.w #$1f, d5
pub fn i_013510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13514;
    cpu.ir = 0x245;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $013514: subq.b #$1, d5
pub fn i_013514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13516;
    cpu.ir = 0x5305;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    4
}
/// $013516: moveq #$0, d7
pub fn i_013516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13518;
    cpu.ir = 0x7E00;
    cpu.d[7] = 0x0;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $013518: clr.w d0
pub fn i_013518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1351A;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $01351A: clr.w d4
pub fn i_01351a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1351C;
    cpu.ir = 0x4244;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $01351C: move.b (a2)+, d0
pub fn i_01351c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1351E;
    cpu.ir = 0x101A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    8
}
/// $01351E: move.b (a3)+, d4
pub fn i_01351e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13520;
    cpu.ir = 0x181B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    8
}
/// $013520: move.b d4, d6
pub fn i_013520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13522;
    cpu.ir = 0x1C04;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v1 & 0xFF);
    4
}
/// $013522: clr.b d2
pub fn i_013522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13524;
    cpu.ir = 0x4202;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    4
}
/// $013524: sub.w d0, d4
pub fn i_013524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13526;
    cpu.ir = 0x9840;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $013526: beq.w $13554
pub fn i_013526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1352A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x13554; return 10; }
    8
}
/// $01352A: bpl.w $13534
pub fn i_01352a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1352E;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x13534; return 10; }
    8
}
/// $01352E: neg.w d4
pub fn i_01352e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13530;
    cpu.ir = 0x4444;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $013530: move.b #$ff, d2
pub fn i_013530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13534;
    cpu.ir = 0x143C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $013534: add.w d4, d4
pub fn i_013534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13536;
    cpu.ir = 0xD844;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $013536: add.w d4, d4
pub fn i_013536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13538;
    cpu.ir = 0xD844;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $013538: lea.l $12f30(pc), a0
pub fn i_013538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1353C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x12F30;
    16
}
/// $01353C: movea.l (a0, d4.w), a0
pub fn i_01353c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13540;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $013540: move.b (a0, d5.w), d4
pub fn i_013540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13544;
    cpu.ir = 0x1830;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    14
}
/// $013544: tst.b d2
pub fn i_013544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13546;
    cpu.ir = 0x4A02;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $013546: beq.w $1354c
pub fn i_013546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1354A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x1354C; return 10; }
    8
}
/// $01354A: neg.b d4
pub fn i_01354a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1354C;
    cpu.ir = 0x4404;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.set(StatusRegister::V, v1 == 0x80); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    4
}
/// $01354C: add.b d4, d0
pub fn i_01354c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1354E;
    cpu.ir = 0xD004;
    let v1 = cpu.d[4] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    let r = add_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    4
}
/// $01354E: cmp.b d0, d6
pub fn i_01354e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13550;
    cpu.ir = 0xBC00;
    let v1 = cpu.d[0] & 0xFF;
    let h1 = cpu.d[6] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $013550: bne.w $13558
pub fn i_013550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13554;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x13558; return 10; }
    8
}
/// $013554: addq.b #$1, d7
pub fn i_013554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13556;
    cpu.ir = 0x5207;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    4
}
/// $013556: move.b d6, d0
pub fn i_013556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13558;
    cpu.ir = 0x1006;
    let v1 = cpu.d[6] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $013558: move.b d0, d2
pub fn i_013558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1355A;
    cpu.ir = 0x1400;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $01355A: clr.w d0
pub fn i_01355a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1355C;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $01355C: clr.w d4
pub fn i_01355c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1355E;
    cpu.ir = 0x4244;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $01355E: move.b (a2)+, d0
pub fn i_01355e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13560;
    cpu.ir = 0x101A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    8
}
/// $013560: move.b (a3)+, d4
pub fn i_013560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13562;
    cpu.ir = 0x181B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    8
}
/// $013562: move.b d4, d6
pub fn i_013562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13564;
    cpu.ir = 0x1C04;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v1 & 0xFF);
    4
}
/// $013564: clr.b d3
pub fn i_013564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13566;
    cpu.ir = 0x4203;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    4
}
/// $013566: sub.w d0, d4
pub fn i_013566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13568;
    cpu.ir = 0x9840;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $013568: beq.w $13596
pub fn i_013568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1356C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x13596; return 10; }
    8
}
/// $01356C: bpl.w $13576
pub fn i_01356c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13570;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x13576; return 10; }
    8
}
/// $013570: neg.w d4
pub fn i_013570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13572;
    cpu.ir = 0x4444;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $013572: move.b #$ff, d3
pub fn i_013572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13576;
    cpu.ir = 0x163C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v1 & 0xFF);
    4
}
/// $013576: add.w d4, d4
pub fn i_013576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13578;
    cpu.ir = 0xD844;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $013578: add.w d4, d4
pub fn i_013578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1357A;
    cpu.ir = 0xD844;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $01357A: lea.l $12f30(pc), a0
pub fn i_01357a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1357E;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x12F30;
    16
}
/// $01357E: movea.l (a0, d4.w), a0
pub fn i_01357e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13582;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $013582: move.b (a0, d5.w), d4
pub fn i_013582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13586;
    cpu.ir = 0x1830;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    14
}
/// $013586: tst.b d3
pub fn i_013586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13588;
    cpu.ir = 0x4A03;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $013588: beq.w $1358e
pub fn i_013588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1358C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x1358E; return 10; }
    8
}
/// $01358C: neg.b d4
pub fn i_01358c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1358E;
    cpu.ir = 0x4404;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.set(StatusRegister::V, v1 == 0x80); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    4
}
/// $01358E: add.b d4, d0
pub fn i_01358e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13590;
    cpu.ir = 0xD004;
    let v1 = cpu.d[4] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    let r = add_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    4
}
/// $013590: cmp.b d0, d6
pub fn i_013590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13592;
    cpu.ir = 0xBC00;
    let v1 = cpu.d[0] & 0xFF;
    let h1 = cpu.d[6] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $013592: bne.w $1359a
pub fn i_013592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13596;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x1359A; return 10; }
    8
}
/// $013596: addq.b #$1, d7
pub fn i_013596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13598;
    cpu.ir = 0x5207;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    4
}
/// $013598: move.b d6, d0
pub fn i_013598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1359A;
    cpu.ir = 0x1006;
    let v1 = cpu.d[6] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $01359A: move.b d0, d3
pub fn i_01359a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1359C;
    cpu.ir = 0x1600;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v1 & 0xFF);
    4
}
/// $01359C: clr.w d0
pub fn i_01359c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1359E;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $01359E: clr.w d4
pub fn i_01359e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x135A0;
    cpu.ir = 0x4244;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0135A0: move.b (a2)+, d0
pub fn i_0135a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x135A2;
    cpu.ir = 0x101A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    8
}
/// $0135A2: move.b (a3)+, d4
pub fn i_0135a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x135A4;
    cpu.ir = 0x181B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    8
}
/// $0135A4: move.b d4, d6
pub fn i_0135a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x135A6;
    cpu.ir = 0x1C04;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0135A6: clr.b d1
pub fn i_0135a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x135A8;
    cpu.ir = 0x4201;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $0135A8: sub.w d0, d4
pub fn i_0135a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x135AA;
    cpu.ir = 0x9840;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0135AA: beq.w $135d8
pub fn i_0135aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x135AE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x135D8; return 10; }
    8
}
/// $0135AE: bpl.w $135b8
pub fn i_0135ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x135B2;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x135B8; return 10; }
    8
}
/// $0135B2: neg.w d4
pub fn i_0135b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x135B4;
    cpu.ir = 0x4444;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0135B4: move.b #$ff, d1
pub fn i_0135b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x135B8;
    cpu.ir = 0x123C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0135B8: add.w d4, d4
pub fn i_0135b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x135BA;
    cpu.ir = 0xD844;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0135BA: add.w d4, d4
pub fn i_0135ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x135BC;
    cpu.ir = 0xD844;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0135BC: lea.l $12f30(pc), a0
pub fn i_0135bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x135C0;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x12F30;
    16
}
/// $0135C0: movea.l (a0, d4.w), a0
pub fn i_0135c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x135C4;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $0135C4: move.b (a0, d5.w), d4
pub fn i_0135c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x135C8;
    cpu.ir = 0x1830;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    14
}
/// $0135C8: tst.b d1
pub fn i_0135c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x135CA;
    cpu.ir = 0x4A01;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $0135CA: beq.w $135d0
pub fn i_0135ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x135CE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x135D0; return 10; }
    8
}
/// $0135CE: neg.b d4
pub fn i_0135ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x135D0;
    cpu.ir = 0x4404;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.set(StatusRegister::V, v1 == 0x80); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    4
}
/// $0135D0: add.b d4, d0
pub fn i_0135d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x135D2;
    cpu.ir = 0xD004;
    let v1 = cpu.d[4] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    let r = add_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    4
}
/// $0135D2: cmp.b d0, d6
pub fn i_0135d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x135D4;
    cpu.ir = 0xBC00;
    let v1 = cpu.d[0] & 0xFF;
    let h1 = cpu.d[6] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $0135D4: bne.w $135dc
pub fn i_0135d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x135D8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x135DC; return 10; }
    8
}
/// $0135D8: addq.b #$1, d7
pub fn i_0135d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x135DA;
    cpu.ir = 0x5207;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    4
}
/// $0135DA: move.b d6, d0
pub fn i_0135da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x135DC;
    cpu.ir = 0x1006;
    let v1 = cpu.d[6] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0135DC: move.b d0, d4
pub fn i_0135dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x135DE;
    cpu.ir = 0x1800;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0135DE: addq.l #$1, a2
pub fn i_0135de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x135E0;
    cpu.ir = 0x528A;
    cpu.a[2] = cpu.a[2].wrapping_add(1);
    8
}
/// $0135E0: addq.l #$1, a3
pub fn i_0135e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x135E2;
    cpu.ir = 0x528B;
    cpu.a[3] = cpu.a[3].wrapping_add(1);
    8
}
/// $0135E2: cmpi.b #$3, d7
pub fn i_0135e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x135E6;
    cpu.ir = 0xC07;
    let v1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $0135E6: bne.w $135f6
pub fn i_0135e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x135EA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x135F6; return 10; }
    8
}
/// $0135EA: move.w #$ffff, d0
pub fn i_0135ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x135EE;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0135EE: movem.l (a7)+, d5-d7/a0
pub fn i_0135ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x135F2;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[7] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $0135F2: move.w (a7)+, d1
pub fn i_0135f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x135F4;
    cpu.ir = 0x321F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $0135F4: rts
pub fn i_0135f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x135F6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0135F6: clr.w d0
pub fn i_0135f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x135F8;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0135F8: movem.l (a7)+, d5-d7/a0
pub fn i_0135f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x135FC;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[7] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $0135FC: move.w (a7)+, d1
pub fn i_0135fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x135FE;
    cpu.ir = 0x321F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $0135FE: rts
pub fn i_0135fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13600;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_FlushSlotHistory_013600  ($013600, 36 B, entity/flush_slots_013600.s)
/// $013600: tst.w $1e(a6)
pub fn i_013600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13604;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x1E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $013604: beq.w $13622
pub fn i_013604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13608;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x13622; return 10; }
    8
}
/// $013608: subq.w #$1, $1e(a6)
pub fn i_013608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1360C;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x1E);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $01360C: move.w $1e(a6), d2
pub fn i_01360c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13610;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x1E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $013610: add.w d2, d2
pub fn i_013610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13612;
    cpu.ir = 0xD442;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $013612: move.w $16(a6, d2.w), d1
pub fn i_013612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13616;
    cpu.ir = 0x3236;
    let ad2 = cpu.a[6].wrapping_add(0x16).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $013616: clr.w $16(a6, d2.w)
pub fn i_013616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1361A;
    cpu.ir = 0x4276;
    let ad1 = cpu.a[6].wrapping_add(0x16).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    14
}
/// $01361A: jsr $2bc4.l
pub fn i_01361a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13620;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x13620);
    cpu.pc = 0x2BC4;
    16
}
/// $013620: bra.b $13600
pub fn i_013620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13622;
    cpu.ir = 0x60DE;
    cpu.pc = 0x13600;
    10
}
/// $013622: rts
pub fn i_013622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13624;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pal_ApplyFadeDarken_013624  ($013624, 112 B, video/palette_engine_sprite_grid_0133b0.s)
/// $013624: move.l d0, -(a7)
pub fn i_013624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13626;
    cpu.ir = 0x2F00;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $013626: movem.l d5-d6/a0-a1, -(a7)
pub fn i_013626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1362A;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $01362A: clr.w d5
pub fn i_01362a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1362C;
    cpu.ir = 0x4245;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $01362C: lea.l $12f30(pc), a1
pub fn i_01362c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13630;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x12F30;
    16
}
/// $013630: clr.w d0
pub fn i_013630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13632;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $013632: move.b d2, d0
pub fn i_013632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13634;
    cpu.ir = 0x1002;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $013634: add.w d0, d0
pub fn i_013634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13636;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $013636: add.w d0, d0
pub fn i_013636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13638;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $013638: movea.l (a1, d0.w), a0
pub fn i_013638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1363C;
    cpu.ir = 0x2071;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $01363C: clr.w d6
pub fn i_01363c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1363E;
    cpu.ir = 0x4246;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $01363E: move.b $10a2ca.l, d6
pub fn i_01363e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13644;
    cpu.ir = 0x1C39;
    let v2 = bus.read8(0x10A2CA) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v2 & 0xFF);
    16
}
/// $013644: move.b (a0, d6.w), d6
pub fn i_013644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13648;
    cpu.ir = 0x1C30;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    14
}
/// $013648: sub.b d6, d2
pub fn i_013648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1364A;
    cpu.ir = 0x9406;
    let v1 = cpu.d[6] & 0xFF;
    let h1 = cpu.d[2] & 0xFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r & 0xFF);
    4
}
/// $01364A: move.b d2, d5
pub fn i_01364a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1364C;
    cpu.ir = 0x1A02;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v1 & 0xFF);
    4
}
/// $01364C: lsl.w #$5, d5
pub fn i_01364c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1364E;
    cpu.ir = 0xEB4D;
    let h1 = cpu.d[5] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 1, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $01364E: clr.w d0
pub fn i_01364e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13650;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $013650: move.b d3, d0
pub fn i_013650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13652;
    cpu.ir = 0x1003;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $013652: add.w d0, d0
pub fn i_013652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13654;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $013654: add.w d0, d0
pub fn i_013654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13656;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $013656: movea.l (a1, d0.w), a0
pub fn i_013656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1365A;
    cpu.ir = 0x2071;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $01365A: clr.w d6
pub fn i_01365a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1365C;
    cpu.ir = 0x4246;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $01365C: move.b $10a2cb.l, d6
pub fn i_01365c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13662;
    cpu.ir = 0x1C39;
    let v2 = bus.read8(0x10A2CB) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v2 & 0xFF);
    16
}
/// $013662: move.b (a0, d6.w), d6
pub fn i_013662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13666;
    cpu.ir = 0x1C30;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    14
}
/// $013666: sub.b d6, d3
pub fn i_013666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13668;
    cpu.ir = 0x9606;
    let v1 = cpu.d[6] & 0xFF;
    let h1 = cpu.d[3] & 0xFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r & 0xFF);
    4
}
/// $013668: or.b d3, d5
pub fn i_013668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1366A;
    cpu.ir = 0x8A03;
    let v1 = cpu.d[3] & 0xFF;
    let r = (v1 | cpu.d[5]) & 0xFF;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $01366A: lsl.w #$5, d5
pub fn i_01366a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1366C;
    cpu.ir = 0xEB4D;
    let h1 = cpu.d[5] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 1, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $01366C: clr.w d0
pub fn i_01366c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1366E;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $01366E: move.b d4, d0
pub fn i_01366e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13670;
    cpu.ir = 0x1004;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $013670: add.w d0, d0
pub fn i_013670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13672;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $013672: add.w d0, d0
pub fn i_013672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13674;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $013674: movea.l (a1, d0.w), a0
pub fn i_013674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13678;
    cpu.ir = 0x2071;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $013678: clr.w d6
pub fn i_013678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1367A;
    cpu.ir = 0x4246;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $01367A: move.b $10a2cc.l, d6
pub fn i_01367a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13680;
    cpu.ir = 0x1C39;
    let v2 = bus.read8(0x10A2CC) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v2 & 0xFF);
    16
}
/// $013680: move.b (a0, d6.w), d6
pub fn i_013680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13684;
    cpu.ir = 0x1C30;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    14
}
/// $013684: sub.b d6, d4
pub fn i_013684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13686;
    cpu.ir = 0x9806;
    let v1 = cpu.d[6] & 0xFF;
    let h1 = cpu.d[4] & 0xFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r & 0xFF);
    4
}
/// $013686: andi.w #$1f, d4
pub fn i_013686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1368A;
    cpu.ir = 0x244;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $01368A: or.w d5, d4
pub fn i_01368a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1368C;
    cpu.ir = 0x8845;
    let v1 = cpu.d[5] & 0xFFFF;
    let r = (v1 | cpu.d[4]) & 0xFFFF;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $01368C: movem.l (a7)+, d5-d6/a0-a1
pub fn i_01368c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13690;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $013690: move.l (a7)+, d0
pub fn i_013690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13692;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $013692: rts
pub fn i_013692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13694;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pal_ApplyFadeLighten_013694  ($013694, 118 B, video/palette_engine_sprite_grid_0133b0.s)
/// $013694: move.l d0, -(a7)
pub fn i_013694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13696;
    cpu.ir = 0x2F00;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $013696: movem.l d5-d6/a0-a1, -(a7)
pub fn i_013696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1369A;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $01369A: lea.l $12f30(pc), a1
pub fn i_01369a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1369E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x12F30;
    16
}
/// $01369E: clr.w d5
pub fn i_01369e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x136A0;
    cpu.ir = 0x4245;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0136A0: move.w #$1f, d0
pub fn i_0136a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x136A4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0136A4: sub.b d2, d0
pub fn i_0136a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x136A6;
    cpu.ir = 0x9002;
    let v1 = cpu.d[2] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    4
}
/// $0136A6: add.w d0, d0
pub fn i_0136a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x136A8;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0136A8: add.w d0, d0
pub fn i_0136a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x136AA;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0136AA: movea.l (a1, d0.w), a0
pub fn i_0136aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x136AE;
    cpu.ir = 0x2071;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $0136AE: clr.w d6
pub fn i_0136ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x136B0;
    cpu.ir = 0x4246;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0136B0: move.b $10a2ca.l, d6
pub fn i_0136b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x136B6;
    cpu.ir = 0x1C39;
    let v2 = bus.read8(0x10A2CA) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v2 & 0xFF);
    16
}
/// $0136B6: move.b (a0, d6.w), d6
pub fn i_0136b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x136BA;
    cpu.ir = 0x1C30;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    14
}
/// $0136BA: add.b d6, d2
pub fn i_0136ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x136BC;
    cpu.ir = 0xD406;
    let v1 = cpu.d[6] & 0xFF;
    let h1 = cpu.d[2] & 0xFF;
    let r = add_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r & 0xFF);
    4
}
/// $0136BC: move.b d2, d5
pub fn i_0136bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x136BE;
    cpu.ir = 0x1A02;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0136BE: lsl.w #$5, d5
pub fn i_0136be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x136C0;
    cpu.ir = 0xEB4D;
    let h1 = cpu.d[5] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 1, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $0136C0: move.w #$1f, d0
pub fn i_0136c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x136C4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0136C4: sub.b d3, d0
pub fn i_0136c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x136C6;
    cpu.ir = 0x9003;
    let v1 = cpu.d[3] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    4
}
/// $0136C6: add.w d0, d0
pub fn i_0136c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x136C8;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0136C8: add.w d0, d0
pub fn i_0136c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x136CA;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0136CA: movea.l (a1, d0.w), a0
pub fn i_0136ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x136CE;
    cpu.ir = 0x2071;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $0136CE: clr.w d6
pub fn i_0136ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x136D0;
    cpu.ir = 0x4246;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0136D0: move.b $10a2cb.l, d6
pub fn i_0136d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x136D6;
    cpu.ir = 0x1C39;
    let v2 = bus.read8(0x10A2CB) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v2 & 0xFF);
    16
}
/// $0136D6: move.b (a0, d6.w), d6
pub fn i_0136d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x136DA;
    cpu.ir = 0x1C30;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    14
}
/// $0136DA: add.b d6, d3
pub fn i_0136da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x136DC;
    cpu.ir = 0xD606;
    let v1 = cpu.d[6] & 0xFF;
    let h1 = cpu.d[3] & 0xFF;
    let r = add_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r & 0xFF);
    4
}
/// $0136DC: or.b d3, d5
pub fn i_0136dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x136DE;
    cpu.ir = 0x8A03;
    let v1 = cpu.d[3] & 0xFF;
    let r = (v1 | cpu.d[5]) & 0xFF;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $0136DE: lsl.w #$5, d5
pub fn i_0136de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x136E0;
    cpu.ir = 0xEB4D;
    let h1 = cpu.d[5] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 1, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $0136E0: move.w #$1f, d0
pub fn i_0136e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x136E4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0136E4: sub.b d4, d0
pub fn i_0136e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x136E6;
    cpu.ir = 0x9004;
    let v1 = cpu.d[4] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    4
}
/// $0136E6: add.w d0, d0
pub fn i_0136e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x136E8;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0136E8: add.w d0, d0
pub fn i_0136e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x136EA;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0136EA: movea.l (a1, d0.w), a0
pub fn i_0136ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x136EE;
    cpu.ir = 0x2071;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $0136EE: clr.w d6
pub fn i_0136ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x136F0;
    cpu.ir = 0x4246;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0136F0: move.b $10a2cc.l, d6
pub fn i_0136f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x136F6;
    cpu.ir = 0x1C39;
    let v2 = bus.read8(0x10A2CC) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v2 & 0xFF);
    16
}
/// $0136F6: move.b (a0, d6.w), d6
pub fn i_0136f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x136FA;
    cpu.ir = 0x1C30;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    14
}
/// $0136FA: add.b d6, d4
pub fn i_0136fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x136FC;
    cpu.ir = 0xD806;
    let v1 = cpu.d[6] & 0xFF;
    let h1 = cpu.d[4] & 0xFF;
    let r = add_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r & 0xFF);
    4
}
/// $0136FC: andi.w #$1f, d4
pub fn i_0136fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13700;
    cpu.ir = 0x244;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $013700: or.w d5, d4
pub fn i_013700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13702;
    cpu.ir = 0x8845;
    let v1 = cpu.d[5] & 0xFFFF;
    let r = (v1 | cpu.d[4]) & 0xFFFF;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $013702: movem.l (a7)+, d5-d6/a0-a1
pub fn i_013702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13706;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $013706: move.l (a7)+, d0
pub fn i_013706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13708;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $013708: rts
pub fn i_013708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1370A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pal_ShadowClearAll_01379a  ($01379A, 44 B, video/palette_engine_sprite_grid_0133b0.s)
/// $01379A: lea.l $10a2d4.l, a0
pub fn i_01379a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x137A0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10A2D4;
    20
}
/// $0137A0: moveq #$0, d1
pub fn i_0137a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x137A2;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0137A2: move.w #$ff, d0
pub fn i_0137a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x137A6;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0137A6: move.l d1, (a0)+
pub fn i_0137a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x137A8;
    cpu.ir = 0x20C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0137A8: move.l d1, (a0)+
pub fn i_0137a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x137AA;
    cpu.ir = 0x20C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0137AA: move.l d1, (a0)+
pub fn i_0137aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x137AC;
    cpu.ir = 0x20C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0137AC: move.l d1, (a0)+
pub fn i_0137ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x137AE;
    cpu.ir = 0x20C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0137AE: move.l d1, (a0)+
pub fn i_0137ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x137B0;
    cpu.ir = 0x20C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0137B0: move.l d1, (a0)+
pub fn i_0137b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x137B2;
    cpu.ir = 0x20C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0137B2: move.l d1, (a0)+
pub fn i_0137b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x137B4;
    cpu.ir = 0x20C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0137B4: move.l d1, (a0)+
pub fn i_0137b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x137B6;
    cpu.ir = 0x20C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0137B6: dbra d0, $137a6
pub fn i_0137b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x137BA;
    cpu.ir = 0x51C8;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x137BA; return 12; }
    let n = (cpu.d[0] as u16).wrapping_sub(1);
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x137BA; return 14; }
    cpu.pc = 0x137A6;
    10
}
/// $0137BA: clr.b $10a2d2.l
pub fn i_0137ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x137C0;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10A2D2) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10A2D2, r2 as u8);
    16
}
/// $0137C0: jmp $526b8.l
pub fn i_0137c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x137C6;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x526B8;
    8
}
// ===== Pal_FlushDirtyToHW_0137c6  ($0137C6, 184 B, video/palette_engine_sprite_grid_0133b0.s)
/// $0137C6: tst.b $10a2ce.l
pub fn i_0137c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x137CC;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10A2CE) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $0137CC: beq.w $1382a
pub fn i_0137cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x137D0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x1382A; return 10; }
    8
}
/// $0137D0: move.b #$1, $3a000f.l
pub fn i_0137d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x137D8;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A000F, v1 as u8);
    16
}
/// $0137D8: lea.l $10a2d4.l, a0
pub fn i_0137d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x137DE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10A2D4;
    20
}
/// $0137DE: movea.l #$400000, a1
pub fn i_0137de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x137E4;
    cpu.ir = 0x227C;
    let v1: u32 = 0x400000;
    cpu.a[1] = v1;
    4
}
/// $0137E4: moveq #$20, d1
pub fn i_0137e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x137E6;
    cpu.ir = 0x7220;
    cpu.d[1] = 0x20;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0137E6: move.w #$c3, d0
pub fn i_0137e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x137EA;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0137EA: tst.w (a0)
pub fn i_0137ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x137EC;
    cpu.ir = 0x4A50;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    8
}
/// $0137EC: bne.w $13866
pub fn i_0137ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x137F0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x13866; return 10; }
    8
}
/// $0137F0: adda.l d1, a0
pub fn i_0137f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x137F2;
    cpu.ir = 0xD1C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add(v1);
    8
}
/// $0137F2: adda.l d1, a1
pub fn i_0137f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x137F4;
    cpu.ir = 0xD3C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $0137F4: dbra d0, $137ea
pub fn i_0137f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x137F8;
    cpu.ir = 0x51C8;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x137F8; return 12; }
    let n = (cpu.d[0] as u16).wrapping_sub(1);
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x137F8; return 14; }
    cpu.pc = 0x137EA;
    10
}
/// $0137F8: lea.l $10a2d4.l, a0
pub fn i_0137f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x137FE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10A2D4;
    20
}
/// $0137FE: tst.w $1fe0(a0)
pub fn i_0137fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13802;
    cpu.ir = 0x4A68;
    let ad2 = cpu.a[0].wrapping_add(0x1FE0);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $013802: beq.w $1382a
pub fn i_013802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13806;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x1382A; return 10; }
    8
}
/// $013806: adda.l #$1fe0, a0
pub fn i_013806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1380C;
    cpu.ir = 0xD1FC;
    let v1: u32 = 0x1FE0;
    cpu.a[0] = cpu.a[0].wrapping_add(v1);
    8
}
/// $01380C: movea.l #$400000, a1
pub fn i_01380c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13812;
    cpu.ir = 0x227C;
    let v1: u32 = 0x400000;
    cpu.a[1] = v1;
    4
}
/// $013812: adda.l #$1fe2, a1
pub fn i_013812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13818;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x1FE2;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $013818: clr.w (a0)+
pub fn i_013818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1381A;
    cpu.ir = 0x4258;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $01381A: move.w (a0)+, (a1)+
pub fn i_01381a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1381C;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $01381C: move.l (a0)+, (a1)+
pub fn i_01381c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1381E;
    cpu.ir = 0x22D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $01381E: move.l (a0)+, (a1)+
pub fn i_01381e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13820;
    cpu.ir = 0x22D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $013820: move.l (a0)+, (a1)+
pub fn i_013820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13822;
    cpu.ir = 0x22D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $013822: move.l (a0)+, (a1)+
pub fn i_013822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13824;
    cpu.ir = 0x22D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $013824: move.l (a0)+, (a1)+
pub fn i_013824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13826;
    cpu.ir = 0x22D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $013826: move.l (a0)+, (a1)+
pub fn i_013826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13828;
    cpu.ir = 0x22D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $013828: move.l (a0)+, (a1)+
pub fn i_013828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1382A;
    cpu.ir = 0x22D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $01382A: tst.b $10a2d1.l
pub fn i_01382a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13830;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10A2D1) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $013830: bne.w $13856
pub fn i_013830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13834;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x13856; return 10; }
    8
}
/// $013834: tst.b $10a2d0.l
pub fn i_013834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1383A;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10A2D0) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $01383A: beq.w $1384c
pub fn i_01383a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1383E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x1384C; return 10; }
    8
}
/// $01383E: subq.b #$1, $10a2d0.l
pub fn i_01383e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13844;
    cpu.ir = 0x5339;
    let v1 = bus.read8(0x10A2D0) as u32;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    bus.write8(0x10A2D0, r2 as u8);
    16
}
/// $013844: move.b #$1, $10a2d1.l
pub fn i_013844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1384C;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2D1, v1 as u8);
    16
}
/// $01384C: move.b #$1, $3a000f.l
pub fn i_01384c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13854;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A000F, v1 as u8);
    16
}
/// $013854: rts
pub fn i_013854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13856;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $013856: move.b #$1, $3a001f.l
pub fn i_013856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1385E;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001F, v1 as u8);
    16
}
/// $01385E: subq.b #$1, $10a2d1.l
pub fn i_01385e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13864;
    cpu.ir = 0x5339;
    let v1 = bus.read8(0x10A2D1) as u32;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    bus.write8(0x10A2D1, r2 as u8);
    16
}
/// $013864: rts
pub fn i_013864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13866;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $013866: clr.w (a0)+
pub fn i_013866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13868;
    cpu.ir = 0x4258;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $013868: addq.w #$2, a1
pub fn i_013868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1386A;
    cpu.ir = 0x5449;
    cpu.a[1] = cpu.a[1].wrapping_add(2);
    8
}
/// $01386A: move.w (a0)+, (a1)+
pub fn i_01386a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1386C;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $01386C: move.l (a0)+, (a1)+
pub fn i_01386c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1386E;
    cpu.ir = 0x22D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $01386E: move.l (a0)+, (a1)+
pub fn i_01386e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13870;
    cpu.ir = 0x22D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $013870: move.l (a0)+, (a1)+
pub fn i_013870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13872;
    cpu.ir = 0x22D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $013872: move.l (a0)+, (a1)+
pub fn i_013872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13874;
    cpu.ir = 0x22D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $013874: move.l (a0)+, (a1)+
pub fn i_013874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13876;
    cpu.ir = 0x22D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $013876: move.l (a0)+, (a1)+
pub fn i_013876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13878;
    cpu.ir = 0x22D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $013878: move.l (a0)+, (a1)+
pub fn i_013878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1387A;
    cpu.ir = 0x22D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $01387A: bra.w $137f4
pub fn i_01387a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1387E;
    cpu.ir = 0x6000;
    cpu.pc = 0x137F4;
    10
}
// ===== Pal_WhiteOutNextBank_01387e  ($01387E, 104 B, video/palette_engine_sprite_grid_0133b0.s)
/// $01387E: move.b #$1, $3a001f.l
pub fn i_01387e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13886;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001F, v1 as u8);
    16
}
/// $013886: movea.l #$400000, a0
pub fn i_013886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1388C;
    cpu.ir = 0x207C;
    let v1: u32 = 0x400000;
    cpu.a[0] = v1;
    4
}
/// $01388C: move.b $10a2d2.l, d0
pub fn i_01388c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13892;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10A2D2) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $013892: andi.l #$ff, d0
pub fn i_013892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13898;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $013898: add.l d0, d0
pub fn i_013898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1389A;
    cpu.ir = 0xD080;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[0] = r;
    4
}
/// $01389A: lsl.l #$8, d0
pub fn i_01389a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1389C;
    cpu.ir = 0xE188;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 1, true);
    cpu.d[0] = r;
    22
}
/// $01389C: adda.l d0, a0
pub fn i_01389c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1389E;
    cpu.ir = 0xD1C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add(v1);
    8
}
/// $01389E: move.l #$7fff, d2
pub fn i_01389e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x138A4;
    cpu.ir = 0x243C;
    let v1: u32 = 0x7FFF;
    cpu.d[2] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0138A4: move.l #$7fff7fff, d1
pub fn i_0138a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x138AA;
    cpu.ir = 0x223C;
    let v1: u32 = 0x7FFF7FFF;
    cpu.d[1] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0138AA: move.w #$f, d0
pub fn i_0138aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x138AE;
    cpu.ir = 0x303C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0138AE: move.l d2, (a0)+
pub fn i_0138ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x138B0;
    cpu.ir = 0x20C2;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0138B0: move.l d1, (a0)+
pub fn i_0138b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x138B2;
    cpu.ir = 0x20C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0138B2: move.l d1, (a0)+
pub fn i_0138b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x138B4;
    cpu.ir = 0x20C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0138B4: move.l d1, (a0)+
pub fn i_0138b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x138B6;
    cpu.ir = 0x20C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0138B6: move.l d1, (a0)+
pub fn i_0138b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x138B8;
    cpu.ir = 0x20C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0138B8: move.l d1, (a0)+
pub fn i_0138b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x138BA;
    cpu.ir = 0x20C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0138BA: move.l d1, (a0)+
pub fn i_0138ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x138BC;
    cpu.ir = 0x20C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0138BC: move.l d1, (a0)+
pub fn i_0138bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x138BE;
    cpu.ir = 0x20C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0138BE: dbra d0, $138ae
pub fn i_0138be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x138C2;
    cpu.ir = 0x51C8;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x138C2; return 12; }
    let n = (cpu.d[0] as u16).wrapping_sub(1);
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x138C2; return 14; }
    cpu.pc = 0x138AE;
    10
}
/// $0138C2: move.b #$1, $3a000f.l
pub fn i_0138c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x138CA;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A000F, v1 as u8);
    16
}
/// $0138CA: addq.b #$1, $10a2d2.l
pub fn i_0138ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x138D0;
    cpu.ir = 0x5239;
    let v1 = bus.read8(0x10A2D2) as u32;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    bus.write8(0x10A2D2, r2 as u8);
    16
}
/// $0138D0: cmpi.b #$10, $10a2d2.l
pub fn i_0138d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x138D8;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10A2D2) as u32;
    sub_with_flags(cpu, v2, 0x10, Size::Byte, false);
    20
}
/// $0138D8: blt.w $138e4
pub fn i_0138d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x138DC;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x138E4; return 10; }
    8
}
/// $0138DC: move.b #$ff, $10a2d2.l
pub fn i_0138dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x138E4;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2D2, v1 as u8);
    16
}
/// $0138E4: rts
pub fn i_0138e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x138E6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pal_WhiteOutIsDone_0138e6  ($0138E6, 12 B, video/palette_engine_sprite_grid_0133b0.s)
/// $0138E6: cmpi.b #$ff, $10a2d2.l
pub fn i_0138e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x138EE;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10A2D2) as u32;
    sub_with_flags(cpu, v2, 0xFF, Size::Byte, false);
    20
}
/// $0138EE: beq.w $138f8
pub fn i_0138ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x138F2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x138F8; return 10; }
    8
}
// ===== ClearXN_0138f2  ($0138F2, 6 B, thunks/ccr_helpers.c)
/// $0138F2: andi.b #$ee, ccr
pub fn i_0138f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x138F6;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0138F6: rts
pub fn i_0138f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x138F8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_0138f8  ($0138F8, 6 B, thunks/ccr_helpers.c)
/// $0138F8: ori.b #$11, ccr
pub fn i_0138f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x138FC;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0138FC: rts
pub fn i_0138fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x138FE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_AddTimer0_74_0138FE  ($0138FE, 8 B, entity/add_timer_0138fe.s)
/// $0138FE: addi.w #$74, $1c(a6)
pub fn i_0138fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13904;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x1C);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x74, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $013904: rts
pub fn i_013904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13906;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Scratch_Alloc_01390E  ($01390E, 68 B, video/sprite_allocator_01390e.s)
/// $01390E: jsr $5a88a.l
pub fn i_01390e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13914;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x13914);
    cpu.pc = 0x5A88A;
    16
}
/// $013914: move.w #$0, $10e1f4.l
pub fn i_013914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1391C;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10E1F4, v1 as u16);
    16
}
/// $01391C: move.w #$0, $10e1fe.l
pub fn i_01391c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13924;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10E1FE, v1 as u16);
    16
}
/// $013924: move.w d0, $10e1f6.l
pub fn i_013924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1392A;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10E1F6, v1 as u16);
    16
}
/// $01392A: move.w #$17c, d0
pub fn i_01392a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1392E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x17C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $01392E: sub.w d2, d0
pub fn i_01392e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13930;
    cpu.ir = 0x9042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $013930: move.w d0, $10e1f8.l
pub fn i_013930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13936;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10E1F8, v1 as u16);
    16
}
/// $013936: sub.w d1, d0
pub fn i_013936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13938;
    cpu.ir = 0x9041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $013938: move.w d0, $10e1fa.l
pub fn i_013938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1393E;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10E1FA, v1 as u16);
    16
}
/// $01393E: move.w d0, $10e1fc.l
pub fn i_01393e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13944;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10E1FC, v1 as u16);
    16
}
/// $013944: move.w #$0, d0
pub fn i_013944(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13948;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $013948: move.w #$17b, d1
pub fn i_013948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1394C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x17B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $01394C: jsr $1399c(pc)
pub fn i_01394c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13950;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x13950);
    cpu.pc = 0x1399C;
    16
}
/// $013950: rts
pub fn i_013950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13952;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Spawn_TypeB_013952  ($013952, 46 B, video/sprite_allocator_01390e.s)
/// $013952: move.w $10e1fe.l, d2
pub fn i_013952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13958;
    cpu.ir = 0x3439;
    let v2 = bus.read16(0x10E1FE) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $013958: move.w d2, d3
pub fn i_013958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1395A;
    cpu.ir = 0x3602;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $01395A: add.w d0, d3
pub fn i_01395a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1395C;
    cpu.ir = 0xD640;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $01395C: cmp.w $10e1f6.l, d3
pub fn i_01395c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13962;
    cpu.ir = 0xB679;
    let v2 = bus.read16(0x10E1F6) as u32;
    let h1 = cpu.d[3] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    16
}
/// $013962: bhi.w $13980
pub fn i_013962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13966;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x13980; return 10; }
    8
}
/// $013966: move.w d3, $10e1fe.l
pub fn i_013966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1396C;
    cpu.ir = 0x33C3;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10E1FE, v1 as u16);
    16
}
/// $01396C: move.w d2, d0
pub fn i_01396c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1396E;
    cpu.ir = 0x3002;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $01396E: move.w d3, d1
pub fn i_01396e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13970;
    cpu.ir = 0x3203;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $013970: subq.w #$1, d1
pub fn i_013970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13972;
    cpu.ir = 0x5341;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $013972: movem.l d0-d1, -(a7)
pub fn i_013972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13976;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $013976: jsr $139be(pc)
pub fn i_013976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1397A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x1397A);
    cpu.pc = 0x139BE;
    16
}
/// $01397A: movem.l (a7)+, d0-d1
pub fn i_01397a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1397E;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $01397E: rts
pub fn i_01397e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13980;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SpriteTrapGuard_013980  ($013980, 2 B, video/sprite_allocator_01390e.s)
/// $013980: trap #$f
pub fn i_013980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13982;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
// ===== Spawn_TypeA_013982  ($013982, 26 B, video/sprite_allocator_01390e.s)
/// $013982: move.w $10e1fc.l, d2
pub fn i_013982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13988;
    cpu.ir = 0x3439;
    let v2 = bus.read16(0x10E1FC) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $013988: move.w d2, d3
pub fn i_013988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1398A;
    cpu.ir = 0x3602;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $01398A: add.w d0, d3
pub fn i_01398a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1398C;
    cpu.ir = 0xD640;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $01398C: cmpi.w #$17c, d3
pub fn i_01398c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13990;
    cpu.ir = 0xC43;
    let v1 = cpu.d[3] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x17C, Size::Word, false);
    8
}
/// $013990: bhi.b $13980
pub fn i_013990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13992;
    cpu.ir = 0x62EE;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x13980; return 10; }
    8
}
/// $013992: move.w d3, $10e1fc.l
pub fn i_013992(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13998;
    cpu.ir = 0x33C3;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10E1FC, v1 as u16);
    16
}
/// $013998: move.w d2, d0
pub fn i_013998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1399A;
    cpu.ir = 0x3002;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $01399A: bra.b $1396e
pub fn i_01399a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x1399C;
    cpu.ir = 0x60D2;
    cpu.pc = 0x1396E;
    10
}
// ===== SpriteRange_DisableAll_01399C  ($01399C, 34 B, video/sprite_allocator_01390e.s)
/// $01399C: addi.w #$8201, d0
pub fn i_01399c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x139A0;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x8201, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0139A0: addi.w #$8201, d1
pub fn i_0139a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x139A4;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x8201, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0139A4: moveq #$0, d2
pub fn i_0139a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x139A6;
    cpu.ir = 0x7400;
    cpu.d[2] = 0x0;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0139A6: clr.w $3c0004.l
pub fn i_0139a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x139AC;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x3C0004) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x3C0004, r2 as u16);
    16
}
/// $0139AC: movea.l #$3c0000, a0
pub fn i_0139ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x139B2;
    cpu.ir = 0x207C;
    let v1: u32 = 0x3C0000;
    cpu.a[0] = v1;
    4
}
/// $0139B2: movem.w d0/d2, (a0)
pub fn i_0139b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x139B6;
    cpu.ir = 0x4890;
    let mut a = cpu.a[0];
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[2] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0139B6: addq.w #$1, d0
pub fn i_0139b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x139B8;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0139B8: cmp.w d0, d1
pub fn i_0139b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x139BA;
    cpu.ir = 0xB240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $0139BA: bcc.b $139b2
pub fn i_0139ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x139BC;
    cpu.ir = 0x64F6;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x139B2; return 10; }
    8
}
/// $0139BC: rts
pub fn i_0139bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x139BE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SpriteRange_InitChain_0139BE  ($0139BE, 64 B, video/sprite_allocator_01390e.s)
/// $0139BE: move.w d0, d2
pub fn i_0139be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x139C0;
    cpu.ir = 0x3400;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0139C0: addi.w #$8201, d0
pub fn i_0139c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x139C4;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x8201, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0139C4: addi.w #$8201, d1
pub fn i_0139c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x139C8;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x8201, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0139C8: addi.w #$8001, d2
pub fn i_0139c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x139CC;
    cpu.ir = 0x642;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x8001, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0139CC: moveq #$0, d3
pub fn i_0139cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x139CE;
    cpu.ir = 0x7600;
    cpu.d[3] = 0x0;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0139CE: move.w #$fff, d4
pub fn i_0139ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x139D2;
    cpu.ir = 0x383C;
    let v1: u32 = 0xFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0139D2: move.w #$40, d5
pub fn i_0139d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x139D6;
    cpu.ir = 0x3A3C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0139D6: movea.l #$3c0000, a0
pub fn i_0139d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x139DC;
    cpu.ir = 0x207C;
    let v1: u32 = 0x3C0000;
    cpu.a[0] = v1;
    4
}
/// $0139DC: movem.w d0/d3, (a0)
pub fn i_0139dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x139E0;
    cpu.ir = 0x4890;
    let mut a = cpu.a[0];
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[3] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0139E0: addq.w #$1, d0
pub fn i_0139e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x139E2;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0139E2: movem.w d2/d4, (a0)
pub fn i_0139e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x139E6;
    cpu.ir = 0x4890;
    let mut a = cpu.a[0];
    bus.write16(a, cpu.d[2] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[4] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0139E6: addq.w #$1, d2
pub fn i_0139e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x139E8;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0139E8: cmp.w d0, d1
pub fn i_0139e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x139EA;
    cpu.ir = 0xB240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $0139EA: bcs.w $139fc
pub fn i_0139ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x139EE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x139FC; return 10; }
    8
}
/// $0139EE: movem.w d0/d5, (a0)
pub fn i_0139ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x139F2;
    cpu.ir = 0x4890;
    let mut a = cpu.a[0];
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[5] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0139F2: addq.w #$1, d0
pub fn i_0139f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x139F4;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0139F4: movem.w d2/d4, (a0)
pub fn i_0139f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x139F8;
    cpu.ir = 0x4890;
    let mut a = cpu.a[0];
    bus.write16(a, cpu.d[2] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[4] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $0139F8: addq.w #$1, d2
pub fn i_0139f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x139FA;
    cpu.ir = 0x5242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0139FA: bra.b $139e8
pub fn i_0139fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x139FC;
    cpu.ir = 0x60EC;
    cpu.pc = 0x139E8;
    10
}
/// $0139FC: rts
pub fn i_0139fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x139FE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Vec_PolarToXY_013c0e  ($013C0E, 30 B, video/palette_engine_sprite_grid_0133b0.s)
/// $013C0E: add.w d0, d0
pub fn i_013c0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13C10;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $013C10: move.w d1, d2
pub fn i_013c10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13C12;
    cpu.ir = 0x3401;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $013C12: lea.l $2c07ac.l, a2
pub fn i_013c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13C18;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2C07AC;
    20
}
/// $013C18: muls.w (a2, d0.w), d1
pub fn i_013c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13C1C;
    cpu.ir = 0xC3F2;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let r = ((v3 as u16 as i16 as i32).wrapping_mul(cpu.d[1] as u16 as i16 as i32)) as u32;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    80
}
/// $013C1C: asr.l #$8, d1
pub fn i_013c1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13C1E;
    cpu.ir = 0xE081;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[1] = r;
    22
}
/// $013C1E: lea.l $2c072c.l, a2
pub fn i_013c1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13C24;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2C072C;
    20
}
/// $013C24: muls.w (a2, d0.w), d2
pub fn i_013c24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13C28;
    cpu.ir = 0xC5F2;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let r = ((v3 as u16 as i16 as i32).wrapping_mul(cpu.d[2] as u16 as i16 as i32)) as u32;
    cpu.d[2] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    80
}
/// $013C28: asr.l #$8, d2
pub fn i_013c28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13C2A;
    cpu.ir = 0xE082;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[2] = r;
    22
}
/// $013C2A: rts
pub fn i_013c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13C2C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pause_Poll_013c3c  ($013C3C, 104 B, video/palette_engine_sprite_grid_0133b0.s)
/// $013C3C: tst.b $10fd82.l
pub fn i_013c3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13C42;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FD82) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $013C42: beq.w $13c4a
pub fn i_013c42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13C46;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x13C4A; return 10; }
    8
}
/// $013C46: bra.w $13d0c
pub fn i_013c46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13C4A;
    cpu.ir = 0x6000;
    cpu.pc = 0x13D0C;
    10
}
/// $013C4A: tst.b $10e274.l
pub fn i_013c4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13C50;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10E274) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $013C50: beq.w $13d0c
pub fn i_013c50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13C54;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x13D0C; return 10; }
    8
}
/// $013C54: move.b $10fdb6.l, d0
pub fn i_013c54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13C5A;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10FDB6) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $013C5A: cmpi.b #$1, d0
pub fn i_013c5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13C5E;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $013C5E: seq.b d7
pub fn i_013c5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13C60;
    cpu.ir = 0x57C7;
    let c1: u32 = if cpu.sr.evaluate(Condition::Eq) { 0xFF } else { 0 };
    let v2 = cpu.d[7] & 0xFF;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (c1 & 0xFF);
    4
}
/// $013C60: andi.b #$2, d7
pub fn i_013c60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13C64;
    cpu.ir = 0x207;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = (v1 & 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $013C64: move.b $10fdb7.l, d0
pub fn i_013c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13C6A;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10FDB7) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $013C6A: cmpi.b #$1, d0
pub fn i_013c6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13C6E;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $013C6E: seq.b d6
pub fn i_013c6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13C70;
    cpu.ir = 0x57C6;
    let c1: u32 = if cpu.sr.evaluate(Condition::Eq) { 0xFF } else { 0 };
    let v2 = cpu.d[6] & 0xFF;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (c1 & 0xFF);
    4
}
/// $013C70: andi.b #$8, d6
pub fn i_013c70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13C74;
    cpu.ir = 0x206;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = (v1 & 0x8) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    8
}
/// $013C74: or.b d6, d7
pub fn i_013c74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13C76;
    cpu.ir = 0x8E06;
    let v1 = cpu.d[6] & 0xFF;
    let r = (v1 | cpu.d[7]) & 0xFF;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $013C76: tst.b $10e272.l
pub fn i_013c76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13C7C;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10E272) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $013C7C: bne.w $13cae
pub fn i_013c7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13C80;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x13CAE; return 10; }
    8
}
/// $013C80: move.b $10e20d.l, d0
pub fn i_013c80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13C86;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10E20D) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $013C86: and.b d7, d0
pub fn i_013c86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13C88;
    cpu.ir = 0xC007;
    let v1 = cpu.d[7] & 0xFF;
    let r = (v1 & cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $013C88: beq.w $13caa
pub fn i_013c88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13C8C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x13CAA; return 10; }
    8
}
/// $013C8C: move.b #$ff, $10e272.l
pub fn i_013c8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13C94;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E272, v1 as u8);
    16
}
/// $013C94: clr.b $10e273.l
pub fn i_013c94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13C9A;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E273) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E273, r2 as u8);
    16
}
/// $013C9A: move.w #$10e0, d0
pub fn i_013c9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13C9E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10E0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $013C9E: jsr $2352.l
pub fn i_013c9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13CA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x13CA4);
    cpu.pc = 0x2352;
    16
}
// ===== SetC_013ca4  ($013CA4, 6 B, thunks/ccr_helpers.c)
/// $013CA4: ori.b #$1, ccr
pub fn i_013ca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13CA8;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $013CA8: rts
pub fn i_013ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13CAA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pause_Poll_Reject_013caa  ($013CAA, 4 B, video/palette_engine_sprite_grid_0133b0.s)
/// $013CAA: bra.w $13d0c
pub fn i_013caa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13CAE;
    cpu.ir = 0x6000;
    cpu.pc = 0x13D0C;
    10
}
// ===== Pause_Active_013cae  ($013CAE, 88 B, video/palette_engine_sprite_grid_0133b0.s)
/// $013CAE: move.b $10e20d.l, d0
pub fn i_013cae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13CB4;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10E20D) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $013CB4: and.b d7, d0
pub fn i_013cb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13CB6;
    cpu.ir = 0xC007;
    let v1 = cpu.d[7] & 0xFF;
    let r = (v1 & cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $013CB6: beq.w $13cdc
pub fn i_013cb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13CBA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x13CDC; return 10; }
    8
}
/// $013CBA: clr.b $10e272.l
pub fn i_013cba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13CC0;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E272) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E272, r2 as u8);
    16
}
/// $013CC0: bsr.w $13d3e
pub fn i_013cc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13CC4;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x13CC4);
    cpu.pc = 0x13D3E;
    18
}
/// $013CC4: move.w #$10e0, d0
pub fn i_013cc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13CC8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10E0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $013CC8: jsr $2222.l
pub fn i_013cc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13CCE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x13CCE);
    cpu.pc = 0x2222;
    16
}
/// $013CCE: move.w #$10e1, d0
pub fn i_013cce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13CD2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10E1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $013CD2: jsr $2352.l
pub fn i_013cd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13CD8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x13CD8);
    cpu.pc = 0x2352;
    16
}
/// $013CD8: bra.w $13d0c
pub fn i_013cd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13CDC;
    cpu.ir = 0x6000;
    cpu.pc = 0x13D0C;
    10
}
/// $013CDC: move.b $10e273.l, d0
pub fn i_013cdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13CE2;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10E273) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $013CE2: addq.b #$1, $10e273.l
pub fn i_013ce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13CE8;
    cpu.ir = 0x5239;
    let v1 = bus.read8(0x10E273) as u32;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    bus.write8(0x10E273, r2 as u8);
    16
}
/// $013CE8: move.b d0, d1
pub fn i_013ce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13CEA;
    cpu.ir = 0x1200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $013CEA: andi.b #$1f, d0
pub fn i_013cea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13CEE;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1F) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $013CEE: bne.w $13cfa
pub fn i_013cee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13CF2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x13CFA; return 10; }
    8
}
/// $013CF2: bsr.w $13d46
pub fn i_013cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13CF6;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x13CF6);
    cpu.pc = 0x13D46;
    18
}
/// $013CF6: bra.w $13d06
pub fn i_013cf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13CFA;
    cpu.ir = 0x6000;
    cpu.pc = 0x13D06;
    10
}
/// $013CFA: cmpi.b #$18, d0
pub fn i_013cfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13CFE;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x18, Size::Byte, false);
    8
}
/// $013CFE: bne.w $13d06
pub fn i_013cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13D02;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x13D06; return 10; }
    8
}
/// $013D02: bsr.w $13d3e
pub fn i_013d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13D06;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x13D06);
    cpu.pc = 0x13D3E;
    18
}
// ===== SetC_013d06  ($013D06, 6 B, thunks/ccr_helpers.c)
/// $013D06: ori.b #$1, ccr
pub fn i_013d06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13D0A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $013D0A: rts
pub fn i_013d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13D0C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearC_013d0c  ($013D0C, 6 B, thunks/ccr_helpers.c)
/// $013D0C: andi.b #$fe, ccr
pub fn i_013d0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13D10;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $013D10: rts
pub fn i_013d10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13D12;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pause_Clear_013d12  ($013D12, 6 B, video/palette_engine_sprite_grid_0133b0.s)
/// $013D12: clr.b $10e272.l
pub fn i_013d12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13D18;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E272) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E272, r2 as u8);
    16
}
// ===== ClrRamWord_013d18  ($013D18, 8 B, thunks/clr_ram_word.c)
/// $013D18: clr.b $10e274.l
pub fn i_013d18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13D1E;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E274) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E274, r2 as u8);
    16
}
/// $013D1E: rts
pub fn i_013d1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13D20;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetRamByteFF_013d20  ($013D20, 10 B, video/fix_pause_text_013d20.s)
/// $013D20: move.b #$ff, $10e274.l
pub fn i_013d20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13D28;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E274, v1 as u8);
    16
}
/// $013D28: rts
pub fn i_013d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13D2A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClrRamWord_013d2a  ($013D2A, 8 B, thunks/clr_ram_word.c)
/// $013D2A: clr.b $10e274.l
pub fn i_013d2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13D30;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E274) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E274, r2 as u8);
    16
}
/// $013D30: rts
pub fn i_013d30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13D32;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Fix_DrawPauseBlank_013d3e  ($013D3E, 8 B, video/fix_pause_text_013d20.s)
/// $013D3E: lea.l $13d38(pc), a2
pub fn i_013d3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13D42;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0x13D38;
    16
}
/// $013D42: bra.w $13d4a
pub fn i_013d42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13D46;
    cpu.ir = 0x6000;
    cpu.pc = 0x13D4A;
    10
}
// ===== Fix_DrawPause_013d46  ($013D46, 36 B, video/fix_pause_text_013d20.s)
/// $013D46: lea.l $13d32(pc), a2
pub fn i_013d46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13D4A;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0x13D32;
    16
}
/// $013D4A: move.w #$725c, d0
pub fn i_013d4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13D4E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x725C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $013D4E: swap d0
pub fn i_013d4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13D50;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $013D50: move.w #$2300, d0
pub fn i_013d50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13D54;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $013D54: move.b (a2)+, d0
pub fn i_013d54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13D56;
    cpu.ir = 0x101A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    8
}
/// $013D56: beq.w $13d68
pub fn i_013d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13D5A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x13D68; return 10; }
    8
}
/// $013D5A: move.l d0, $3c0000.l
pub fn i_013d5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13D60;
    cpu.ir = 0x23C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    bus.write32(0x3C0000, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $013D60: addi.l #$200000, d0
pub fn i_013d60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13D66;
    cpu.ir = 0x680;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = add_with_flags(cpu, v1, 0x200000, Size::Long, true);
    cpu.d[0] = r2;
    8
}
/// $013D66: bra.b $13d54
pub fn i_013d66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13D68;
    cpu.ir = 0x60EC;
    cpu.pc = 0x13D54;
    10
}
/// $013D68: rts
pub fn i_013d68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x13D6A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x0133B0, i_0133b0),
    (0x0133B2, i_0133b2),
    (0x0133B4, i_0133b4),
    (0x0133B6, i_0133b6),
    (0x0133BA, i_0133ba),
    (0x0133BC, i_0133bc),
    (0x0133BE, i_0133be),
    (0x0133C0, i_0133c0),
    (0x0133C2, i_0133c2),
    (0x0133C4, i_0133c4),
    (0x0133C6, i_0133c6),
    (0x0133C8, i_0133c8),
    (0x0133CE, i_0133ce),
    (0x0133D0, i_0133d0),
    (0x0133D2, i_0133d2),
    (0x0133D4, i_0133d4),
    (0x0133D6, i_0133d6),
    (0x0133DA, i_0133da),
    (0x0133E0, i_0133e0),
    (0x0133E4, i_0133e4),
    (0x0133E6, i_0133e6),
    (0x0133E8, i_0133e8),
    (0x0133EA, i_0133ea),
    (0x0133EC, i_0133ec),
    (0x0133EE, i_0133ee),
    (0x0133F0, i_0133f0),
    (0x0133F2, i_0133f2),
    (0x0133F4, i_0133f4),
    (0x0133F6, i_0133f6),
    (0x0133F8, i_0133f8),
    (0x0133FA, i_0133fa),
    (0x0133FC, i_0133fc),
    (0x013402, i_013402),
    (0x013406, i_013406),
    (0x013408, i_013408),
    (0x01340C, i_01340c),
    (0x013410, i_013410),
    (0x013414, i_013414),
    (0x013418, i_013418),
    (0x01341C, i_01341c),
    (0x013420, i_013420),
    (0x013424, i_013424),
    (0x013428, i_013428),
    (0x01342C, i_01342c),
    (0x013430, i_013430),
    (0x013434, i_013434),
    (0x013438, i_013438),
    (0x01343C, i_01343c),
    (0x013442, i_013442),
    (0x013446, i_013446),
    (0x01344A, i_01344a),
    (0x01344E, i_01344e),
    (0x013452, i_013452),
    (0x013456, i_013456),
    (0x013458, i_013458),
    (0x01345C, i_01345c),
    (0x013460, i_013460),
    (0x013462, i_013462),
    (0x013466, i_013466),
    (0x01346A, i_01346a),
    (0x013470, i_013470),
    (0x013476, i_013476),
    (0x01347A, i_01347a),
    (0x01347E, i_01347e),
    (0x013480, i_013480),
    (0x013484, i_013484),
    (0x01348A, i_01348a),
    (0x01348E, i_01348e),
    (0x013490, i_013490),
    (0x013492, i_013492),
    (0x013496, i_013496),
    (0x01349A, i_01349a),
    (0x01349E, i_01349e),
    (0x0134A2, i_0134a2),
    (0x0134A6, i_0134a6),
    (0x0134AC, i_0134ac),
    (0x0134B0, i_0134b0),
    (0x0134B8, i_0134b8),
    (0x0134BC, i_0134bc),
    (0x0134C0, i_0134c0),
    (0x0134C4, i_0134c4),
    (0x0134C8, i_0134c8),
    (0x0134CC, i_0134cc),
    (0x0134CE, i_0134ce),
    (0x0134D0, i_0134d0),
    (0x0134D2, i_0134d2),
    (0x0134D4, i_0134d4),
    (0x0134DA, i_0134da),
    (0x0134E0, i_0134e0),
    (0x0134E2, i_0134e2),
    (0x0134E4, i_0134e4),
    (0x0134E6, i_0134e6),
    (0x0134EA, i_0134ea),
    (0x0134EC, i_0134ec),
    (0x0134EE, i_0134ee),
    (0x0134F2, i_0134f2),
    (0x0134F6, i_0134f6),
    (0x0134FC, i_0134fc),
    (0x013502, i_013502),
    (0x013504, i_013504),
    (0x013506, i_013506),
    (0x01350A, i_01350a),
    (0x01350C, i_01350c),
    (0x013510, i_013510),
    (0x013514, i_013514),
    (0x013516, i_013516),
    (0x013518, i_013518),
    (0x01351A, i_01351a),
    (0x01351C, i_01351c),
    (0x01351E, i_01351e),
    (0x013520, i_013520),
    (0x013522, i_013522),
    (0x013524, i_013524),
    (0x013526, i_013526),
    (0x01352A, i_01352a),
    (0x01352E, i_01352e),
    (0x013530, i_013530),
    (0x013534, i_013534),
    (0x013536, i_013536),
    (0x013538, i_013538),
    (0x01353C, i_01353c),
    (0x013540, i_013540),
    (0x013544, i_013544),
    (0x013546, i_013546),
    (0x01354A, i_01354a),
    (0x01354C, i_01354c),
    (0x01354E, i_01354e),
    (0x013550, i_013550),
    (0x013554, i_013554),
    (0x013556, i_013556),
    (0x013558, i_013558),
    (0x01355A, i_01355a),
    (0x01355C, i_01355c),
    (0x01355E, i_01355e),
    (0x013560, i_013560),
    (0x013562, i_013562),
    (0x013564, i_013564),
    (0x013566, i_013566),
    (0x013568, i_013568),
    (0x01356C, i_01356c),
    (0x013570, i_013570),
    (0x013572, i_013572),
    (0x013576, i_013576),
    (0x013578, i_013578),
    (0x01357A, i_01357a),
    (0x01357E, i_01357e),
    (0x013582, i_013582),
    (0x013586, i_013586),
    (0x013588, i_013588),
    (0x01358C, i_01358c),
    (0x01358E, i_01358e),
    (0x013590, i_013590),
    (0x013592, i_013592),
    (0x013596, i_013596),
    (0x013598, i_013598),
    (0x01359A, i_01359a),
    (0x01359C, i_01359c),
    (0x01359E, i_01359e),
    (0x0135A0, i_0135a0),
    (0x0135A2, i_0135a2),
    (0x0135A4, i_0135a4),
    (0x0135A6, i_0135a6),
    (0x0135A8, i_0135a8),
    (0x0135AA, i_0135aa),
    (0x0135AE, i_0135ae),
    (0x0135B2, i_0135b2),
    (0x0135B4, i_0135b4),
    (0x0135B8, i_0135b8),
    (0x0135BA, i_0135ba),
    (0x0135BC, i_0135bc),
    (0x0135C0, i_0135c0),
    (0x0135C4, i_0135c4),
    (0x0135C8, i_0135c8),
    (0x0135CA, i_0135ca),
    (0x0135CE, i_0135ce),
    (0x0135D0, i_0135d0),
    (0x0135D2, i_0135d2),
    (0x0135D4, i_0135d4),
    (0x0135D8, i_0135d8),
    (0x0135DA, i_0135da),
    (0x0135DC, i_0135dc),
    (0x0135DE, i_0135de),
    (0x0135E0, i_0135e0),
    (0x0135E2, i_0135e2),
    (0x0135E6, i_0135e6),
    (0x0135EA, i_0135ea),
    (0x0135EE, i_0135ee),
    (0x0135F2, i_0135f2),
    (0x0135F4, i_0135f4),
    (0x0135F6, i_0135f6),
    (0x0135F8, i_0135f8),
    (0x0135FC, i_0135fc),
    (0x0135FE, i_0135fe),
    (0x013600, i_013600),
    (0x013604, i_013604),
    (0x013608, i_013608),
    (0x01360C, i_01360c),
    (0x013610, i_013610),
    (0x013612, i_013612),
    (0x013616, i_013616),
    (0x01361A, i_01361a),
    (0x013620, i_013620),
    (0x013622, i_013622),
    (0x013624, i_013624),
    (0x013626, i_013626),
    (0x01362A, i_01362a),
    (0x01362C, i_01362c),
    (0x013630, i_013630),
    (0x013632, i_013632),
    (0x013634, i_013634),
    (0x013636, i_013636),
    (0x013638, i_013638),
    (0x01363C, i_01363c),
    (0x01363E, i_01363e),
    (0x013644, i_013644),
    (0x013648, i_013648),
    (0x01364A, i_01364a),
    (0x01364C, i_01364c),
    (0x01364E, i_01364e),
    (0x013650, i_013650),
    (0x013652, i_013652),
    (0x013654, i_013654),
    (0x013656, i_013656),
    (0x01365A, i_01365a),
    (0x01365C, i_01365c),
    (0x013662, i_013662),
    (0x013666, i_013666),
    (0x013668, i_013668),
    (0x01366A, i_01366a),
    (0x01366C, i_01366c),
    (0x01366E, i_01366e),
    (0x013670, i_013670),
    (0x013672, i_013672),
    (0x013674, i_013674),
    (0x013678, i_013678),
    (0x01367A, i_01367a),
    (0x013680, i_013680),
    (0x013684, i_013684),
    (0x013686, i_013686),
    (0x01368A, i_01368a),
    (0x01368C, i_01368c),
    (0x013690, i_013690),
    (0x013692, i_013692),
    (0x013694, i_013694),
    (0x013696, i_013696),
    (0x01369A, i_01369a),
    (0x01369E, i_01369e),
    (0x0136A0, i_0136a0),
    (0x0136A4, i_0136a4),
    (0x0136A6, i_0136a6),
    (0x0136A8, i_0136a8),
    (0x0136AA, i_0136aa),
    (0x0136AE, i_0136ae),
    (0x0136B0, i_0136b0),
    (0x0136B6, i_0136b6),
    (0x0136BA, i_0136ba),
    (0x0136BC, i_0136bc),
    (0x0136BE, i_0136be),
    (0x0136C0, i_0136c0),
    (0x0136C4, i_0136c4),
    (0x0136C6, i_0136c6),
    (0x0136C8, i_0136c8),
    (0x0136CA, i_0136ca),
    (0x0136CE, i_0136ce),
    (0x0136D0, i_0136d0),
    (0x0136D6, i_0136d6),
    (0x0136DA, i_0136da),
    (0x0136DC, i_0136dc),
    (0x0136DE, i_0136de),
    (0x0136E0, i_0136e0),
    (0x0136E4, i_0136e4),
    (0x0136E6, i_0136e6),
    (0x0136E8, i_0136e8),
    (0x0136EA, i_0136ea),
    (0x0136EE, i_0136ee),
    (0x0136F0, i_0136f0),
    (0x0136F6, i_0136f6),
    (0x0136FA, i_0136fa),
    (0x0136FC, i_0136fc),
    (0x013700, i_013700),
    (0x013702, i_013702),
    (0x013706, i_013706),
    (0x013708, i_013708),
    (0x01379A, i_01379a),
    (0x0137A0, i_0137a0),
    (0x0137A2, i_0137a2),
    (0x0137A6, i_0137a6),
    (0x0137A8, i_0137a8),
    (0x0137AA, i_0137aa),
    (0x0137AC, i_0137ac),
    (0x0137AE, i_0137ae),
    (0x0137B0, i_0137b0),
    (0x0137B2, i_0137b2),
    (0x0137B4, i_0137b4),
    (0x0137B6, i_0137b6),
    (0x0137BA, i_0137ba),
    (0x0137C0, i_0137c0),
    (0x0137C6, i_0137c6),
    (0x0137CC, i_0137cc),
    (0x0137D0, i_0137d0),
    (0x0137D8, i_0137d8),
    (0x0137DE, i_0137de),
    (0x0137E4, i_0137e4),
    (0x0137E6, i_0137e6),
    (0x0137EA, i_0137ea),
    (0x0137EC, i_0137ec),
    (0x0137F0, i_0137f0),
    (0x0137F2, i_0137f2),
    (0x0137F4, i_0137f4),
    (0x0137F8, i_0137f8),
    (0x0137FE, i_0137fe),
    (0x013802, i_013802),
    (0x013806, i_013806),
    (0x01380C, i_01380c),
    (0x013812, i_013812),
    (0x013818, i_013818),
    (0x01381A, i_01381a),
    (0x01381C, i_01381c),
    (0x01381E, i_01381e),
    (0x013820, i_013820),
    (0x013822, i_013822),
    (0x013824, i_013824),
    (0x013826, i_013826),
    (0x013828, i_013828),
    (0x01382A, i_01382a),
    (0x013830, i_013830),
    (0x013834, i_013834),
    (0x01383A, i_01383a),
    (0x01383E, i_01383e),
    (0x013844, i_013844),
    (0x01384C, i_01384c),
    (0x013854, i_013854),
    (0x013856, i_013856),
    (0x01385E, i_01385e),
    (0x013864, i_013864),
    (0x013866, i_013866),
    (0x013868, i_013868),
    (0x01386A, i_01386a),
    (0x01386C, i_01386c),
    (0x01386E, i_01386e),
    (0x013870, i_013870),
    (0x013872, i_013872),
    (0x013874, i_013874),
    (0x013876, i_013876),
    (0x013878, i_013878),
    (0x01387A, i_01387a),
    (0x01387E, i_01387e),
    (0x013886, i_013886),
    (0x01388C, i_01388c),
    (0x013892, i_013892),
    (0x013898, i_013898),
    (0x01389A, i_01389a),
    (0x01389C, i_01389c),
    (0x01389E, i_01389e),
    (0x0138A4, i_0138a4),
    (0x0138AA, i_0138aa),
    (0x0138AE, i_0138ae),
    (0x0138B0, i_0138b0),
    (0x0138B2, i_0138b2),
    (0x0138B4, i_0138b4),
    (0x0138B6, i_0138b6),
    (0x0138B8, i_0138b8),
    (0x0138BA, i_0138ba),
    (0x0138BC, i_0138bc),
    (0x0138BE, i_0138be),
    (0x0138C2, i_0138c2),
    (0x0138CA, i_0138ca),
    (0x0138D0, i_0138d0),
    (0x0138D8, i_0138d8),
    (0x0138DC, i_0138dc),
    (0x0138E4, i_0138e4),
    (0x0138E6, i_0138e6),
    (0x0138EE, i_0138ee),
    (0x0138F2, i_0138f2),
    (0x0138F6, i_0138f6),
    (0x0138F8, i_0138f8),
    (0x0138FC, i_0138fc),
    (0x0138FE, i_0138fe),
    (0x013904, i_013904),
    (0x01390E, i_01390e),
    (0x013914, i_013914),
    (0x01391C, i_01391c),
    (0x013924, i_013924),
    (0x01392A, i_01392a),
    (0x01392E, i_01392e),
    (0x013930, i_013930),
    (0x013936, i_013936),
    (0x013938, i_013938),
    (0x01393E, i_01393e),
    (0x013944, i_013944),
    (0x013948, i_013948),
    (0x01394C, i_01394c),
    (0x013950, i_013950),
    (0x013952, i_013952),
    (0x013958, i_013958),
    (0x01395A, i_01395a),
    (0x01395C, i_01395c),
    (0x013962, i_013962),
    (0x013966, i_013966),
    (0x01396C, i_01396c),
    (0x01396E, i_01396e),
    (0x013970, i_013970),
    (0x013972, i_013972),
    (0x013976, i_013976),
    (0x01397A, i_01397a),
    (0x01397E, i_01397e),
    (0x013980, i_013980),
    (0x013982, i_013982),
    (0x013988, i_013988),
    (0x01398A, i_01398a),
    (0x01398C, i_01398c),
    (0x013990, i_013990),
    (0x013992, i_013992),
    (0x013998, i_013998),
    (0x01399A, i_01399a),
    (0x01399C, i_01399c),
    (0x0139A0, i_0139a0),
    (0x0139A4, i_0139a4),
    (0x0139A6, i_0139a6),
    (0x0139AC, i_0139ac),
    (0x0139B2, i_0139b2),
    (0x0139B6, i_0139b6),
    (0x0139B8, i_0139b8),
    (0x0139BA, i_0139ba),
    (0x0139BC, i_0139bc),
    (0x0139BE, i_0139be),
    (0x0139C0, i_0139c0),
    (0x0139C4, i_0139c4),
    (0x0139C8, i_0139c8),
    (0x0139CC, i_0139cc),
    (0x0139CE, i_0139ce),
    (0x0139D2, i_0139d2),
    (0x0139D6, i_0139d6),
    (0x0139DC, i_0139dc),
    (0x0139E0, i_0139e0),
    (0x0139E2, i_0139e2),
    (0x0139E6, i_0139e6),
    (0x0139E8, i_0139e8),
    (0x0139EA, i_0139ea),
    (0x0139EE, i_0139ee),
    (0x0139F2, i_0139f2),
    (0x0139F4, i_0139f4),
    (0x0139F8, i_0139f8),
    (0x0139FA, i_0139fa),
    (0x0139FC, i_0139fc),
    (0x013C0E, i_013c0e),
    (0x013C10, i_013c10),
    (0x013C12, i_013c12),
    (0x013C18, i_013c18),
    (0x013C1C, i_013c1c),
    (0x013C1E, i_013c1e),
    (0x013C24, i_013c24),
    (0x013C28, i_013c28),
    (0x013C2A, i_013c2a),
    (0x013C3C, i_013c3c),
    (0x013C42, i_013c42),
    (0x013C46, i_013c46),
    (0x013C4A, i_013c4a),
    (0x013C50, i_013c50),
    (0x013C54, i_013c54),
    (0x013C5A, i_013c5a),
    (0x013C5E, i_013c5e),
    (0x013C60, i_013c60),
    (0x013C64, i_013c64),
    (0x013C6A, i_013c6a),
    (0x013C6E, i_013c6e),
    (0x013C70, i_013c70),
    (0x013C74, i_013c74),
    (0x013C76, i_013c76),
    (0x013C7C, i_013c7c),
    (0x013C80, i_013c80),
    (0x013C86, i_013c86),
    (0x013C88, i_013c88),
    (0x013C8C, i_013c8c),
    (0x013C94, i_013c94),
    (0x013C9A, i_013c9a),
    (0x013C9E, i_013c9e),
    (0x013CA4, i_013ca4),
    (0x013CA8, i_013ca8),
    (0x013CAA, i_013caa),
    (0x013CAE, i_013cae),
    (0x013CB4, i_013cb4),
    (0x013CB6, i_013cb6),
    (0x013CBA, i_013cba),
    (0x013CC0, i_013cc0),
    (0x013CC4, i_013cc4),
    (0x013CC8, i_013cc8),
    (0x013CCE, i_013cce),
    (0x013CD2, i_013cd2),
    (0x013CD8, i_013cd8),
    (0x013CDC, i_013cdc),
    (0x013CE2, i_013ce2),
    (0x013CE8, i_013ce8),
    (0x013CEA, i_013cea),
    (0x013CEE, i_013cee),
    (0x013CF2, i_013cf2),
    (0x013CF6, i_013cf6),
    (0x013CFA, i_013cfa),
    (0x013CFE, i_013cfe),
    (0x013D02, i_013d02),
    (0x013D06, i_013d06),
    (0x013D0A, i_013d0a),
    (0x013D0C, i_013d0c),
    (0x013D10, i_013d10),
    (0x013D12, i_013d12),
    (0x013D18, i_013d18),
    (0x013D1E, i_013d1e),
    (0x013D20, i_013d20),
    (0x013D28, i_013d28),
    (0x013D2A, i_013d2a),
    (0x013D30, i_013d30),
    (0x013D3E, i_013d3e),
    (0x013D42, i_013d42),
    (0x013D46, i_013d46),
    (0x013D4A, i_013d4a),
    (0x013D4E, i_013d4e),
    (0x013D50, i_013d50),
    (0x013D54, i_013d54),
    (0x013D56, i_013d56),
    (0x013D5A, i_013d5a),
    (0x013D60, i_013d60),
    (0x013D66, i_013d66),
    (0x013D68, i_013d68),
];
