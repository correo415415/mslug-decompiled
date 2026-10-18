// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p05_2) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $05B13A: andi.w #$1ff, d1
pub fn i_05b13a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B13E;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1FF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05B13E: lsl.l #$7, d1
pub fn i_05b13e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B140;
    cpu.ir = 0xEF89;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 7, Size::Long, 1, true);
    cpu.d[1] = r;
    20
}
/// $05B140: or.l d6, d1
pub fn i_05b140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B142;
    cpu.ir = 0x8286;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    let r = (v1 | cpu.d[1]) & 0xFFFFFFFF;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B142: clr.w d5
pub fn i_05b142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B144;
    cpu.ir = 0x4245;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05B144: swap d5
pub fn i_05b144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B146;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B146: subq.b #$1, d5
pub fn i_05b146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B148;
    cpu.ir = 0x5305;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05B148: add.w d6, d6
pub fn i_05b148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B14A;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B14A: adda.w d6, a0
pub fn i_05b14a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B14C;
    cpu.ir = 0xD0C6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05B14C: movea.l d4, a4
pub fn i_05b14c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B14E;
    cpu.ir = 0x2844;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    cpu.a[4] = v1;
    4
}
/// $05B14E: move.b $616e(a5), d3
pub fn i_05b14e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B152;
    cpu.ir = 0x162D;
    let ad2 = cpu.a[5].wrapping_add(0x616E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05B152: add.b d3, $616d(a5)
pub fn i_05b152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B156;
    cpu.ir = 0xD72D;
    let ad1 = cpu.a[5].wrapping_add(0x616D);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[3] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $05B156: bcc.w $5b162
pub fn i_05b156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B15A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B162; return 10; }
    8
}
/// $05B15A: swap d4
pub fn i_05b15a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B15C;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B15C: tst.w d4
pub fn i_05b15c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B15E;
    cpu.ir = 0x4A44;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05B15E: bmi.w $5b18e
pub fn i_05b15e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B162;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x5B18E; return 10; }
    8
}
/// $05B162: move.w d4, d3
pub fn i_05b162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B164;
    cpu.ir = 0x3604;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B164: andi.w #$f00, d3
pub fn i_05b164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B168;
    cpu.ir = 0x243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = (v1 & 0xF00) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05B168: addi.w #$100, d3
pub fn i_05b168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B16C;
    cpu.ir = 0x643;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05B16C: lsr.w #$1, d3
pub fn i_05b16c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B16E;
    cpu.ir = 0xE24B;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $05B16E: sub.w d3, d1
pub fn i_05b16e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B170;
    cpu.ir = 0x9243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B170: cmpi.w #$f800, d1
pub fn i_05b170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B174;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xF800, Size::Word, false);
    8
}
/// $05B174: bcc.w $5b180
pub fn i_05b174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B178;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B180; return 10; }
    8
}
/// $05B178: cmpi.w #$a000, d1
pub fn i_05b178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B17C;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA000, Size::Word, false);
    8
}
/// $05B17C: bcc.w $5b18e
pub fn i_05b17c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B180;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B18E; return 10; }
    8
}
/// $05B180: move.l a0, (a3)+
pub fn i_05b180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B182;
    cpu.ir = 0x26C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B182: move.w d2, (a3)+
pub fn i_05b182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B184;
    cpu.ir = 0x36C2;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $05B184: move.w d4, (a3)+
pub fn i_05b184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B186;
    cpu.ir = 0x36C4;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $05B186: move.l d1, (a3)+
pub fn i_05b186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B188;
    cpu.ir = 0x26C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B188: cmpa.l a2, a3
pub fn i_05b188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B18A;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05B18A: bcc.w $5b19e
pub fn i_05b18a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B18E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B19E; return 10; }
    8
}
/// $05B18E: move.l a4, d4
pub fn i_05b18e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B190;
    cpu.ir = 0x280C;
    let v1 = cpu.a[4] & 0xFFFFFFFF;
    cpu.d[4] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B190: adda.w d6, a0
pub fn i_05b190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B192;
    cpu.ir = 0xD0C6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05B192: dbra d5, $5b14e
pub fn i_05b192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B196;
    cpu.ir = 0x51CD;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5B196; return 12; }
    let n = (cpu.d[5] as u16).wrapping_sub(1);
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5B196; return 14; }
    cpu.pc = 0x5B14E;
    10
}
/// $05B196: suba.w d6, a0
pub fn i_05b196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B198;
    cpu.ir = 0x90C6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.a[0] = cpu.a[0].wrapping_sub((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05B198: subq.b #$1, (a7)
pub fn i_05b198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B19A;
    cpu.ir = 0x5317;
    let ad1 = cpu.a[7];
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    8
}
/// $05B19A: bne.w $5b0b6
pub fn i_05b19a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B19E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5B0B6; return 10; }
    8
}
/// $05B19E: addq.w #$2, a7
pub fn i_05b19e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1A0;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $05B1A0: lea.l $4258(a5), a2
pub fn i_05b1a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1A4;
    cpu.ir = 0x45ED;
    let ad1 = cpu.a[5].wrapping_add(0x4258);
    cpu.a[2] = ad1;
    16
}
/// $05B1A4: cmpa.l a2, a3
pub fn i_05b1a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1A6;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05B1A6: beq.w $5b1b0
pub fn i_05b1a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1AA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5B1B0; return 10; }
    8
}
/// $05B1AA: ori.b #$80, -$6(a3)
pub fn i_05b1aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1B0;
    cpu.ir = 0x2B;
    let ad1 = cpu.a[3].wrapping_add(0xFFFFFFFA);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x80) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05B1B0: rts
pub fn i_05b1b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1B2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sprite_DispatchSplashHook_05b1b2  ($05B1B2, 96 B, video/sprite_queue_render_05aa96.s)
/// $05B1B2: movem.l d0-d5/a0-a1, -(a7)
pub fn i_05b1b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1B6;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $05B1B6: cmpi.b #$0, $106ece.l
pub fn i_05b1b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1BE;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    20
}
/// $05B1BE: bne.w $5b20c
pub fn i_05b1be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1C2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5B20C; return 10; }
    8
}
/// $05B1C2: move.l $106f50.l, d6
pub fn i_05b1c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1C8;
    cpu.ir = 0x2C39;
    let v2 = bus.read32(0x106F50);
    cpu.d[6] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $05B1C8: swap d6
pub fn i_05b1c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1CA;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B1CA: cmpi.w #$780, d6
pub fn i_05b1ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1CE;
    cpu.ir = 0xC46;
    let v1 = cpu.d[6] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x780, Size::Word, false);
    8
}
/// $05B1CE: bgt.w $5b20c
pub fn i_05b1ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1D2;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x5B20C; return 10; }
    8
}
/// $05B1D2: move.w $22(a6), d0
pub fn i_05b1d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1D6;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B1D6: move.w $24(a6), d1
pub fn i_05b1d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1DA;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B1DA: addi.w #$8, d1
pub fn i_05b1da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1DE;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x8, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05B1DE: jsr $43f02.l
pub fn i_05b1de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5B1E4);
    cpu.pc = 0x43F02;
    16
}
/// $05B1E4: move.b d0, d6
pub fn i_05b1e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1E6;
    cpu.ir = 0x1C00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05B1E6: cmpi.b #$3a, d6
pub fn i_05b1e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1EA;
    cpu.ir = 0xC06;
    let v1 = cpu.d[6] & 0xFF;
    sub_with_flags(cpu, v1, 0x3A, Size::Byte, false);
    8
}
/// $05B1EA: bne.w $5b20c
pub fn i_05b1ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1EE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5B20C; return 10; }
    8
}
/// $05B1EE: movem.l (a7)+, d0-d5/a0-a1
pub fn i_05b1ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1F2;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $05B1F2: movem.l d0-d5/a0-a1, -(a7)
pub fn i_05b1f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1F6;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $05B1F6: eori.b #$2, d5
pub fn i_05b1f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1FA;
    cpu.ir = 0xA05;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 ^ 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05B1FA: move.b #$7f, d3
pub fn i_05b1fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1FE;
    cpu.ir = 0x163C;
    let v1: u32 = 0x7F;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05B1FE: jsr $5b212(pc)
pub fn i_05b1fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B202;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5B202);
    cpu.pc = 0x5B212;
    16
}
/// $05B202: jsr $5a9ea(pc)
pub fn i_05b202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B206;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5B206);
    cpu.pc = 0x5A9EA;
    16
}
/// $05B206: movem.l (a7)+, d0-d5/a0-a1
pub fn i_05b206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B20A;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $05B20A: rts
pub fn i_05b20a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B20C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05B20C: movem.l (a7)+, d0-d5/a0-a1
pub fn i_05b20c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B210;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $05B210: rts
pub fn i_05b210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B212;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sprite_SplashScreenY_05b212  ($05B212, 32 B, video/sprite_queue_render_05aa96.s)
/// $05B212: movem.l d0-d4, -(a7)
pub fn i_05b212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B216;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.d[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $05B216: move.w d0, d6
pub fn i_05b216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B218;
    cpu.ir = 0x3C00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B218: addq.w #$2, d6
pub fn i_05b218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B21A;
    cpu.ir = 0x5446;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 2, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05B21A: move.w #$e8, d1
pub fn i_05b21a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B21E;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B21E: jsr $96a5a.l
pub fn i_05b21e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B224;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5B224);
    cpu.pc = 0x96A5A;
    16
}
/// $05B224: sub.w d1, d6
pub fn i_05b224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B226;
    cpu.ir = 0x9C41;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B226: sub.w d6, d1
pub fn i_05b226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B228;
    cpu.ir = 0x9246;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B228: move.w d1, d6
pub fn i_05b228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B22A;
    cpu.ir = 0x3C01;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B22A: movem.l (a7)+, d0-d4
pub fn i_05b22a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B22E;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $05B22E: move.w d6, d0
pub fn i_05b22e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B230;
    cpu.ir = 0x3006;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B230: rts
pub fn i_05b230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B232;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SpriteQueue_SortAndRenderSCB1_05b232  ($05B232, 318 B, video/sprite_queue_render_05aa96.s)
/// $05B232: lea.l $108080.l, a5
pub fn i_05b232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B238;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x108080;
    20
}
/// $05B238: lea.l $5424(a5), a0
pub fn i_05b238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B23C;
    cpu.ir = 0x41ED;
    let ad1 = cpu.a[5].wrapping_add(0x5424);
    cpu.a[0] = ad1;
    16
}
/// $05B23C: move.w $6148(a5), d7
pub fn i_05b23c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B240;
    cpu.ir = 0x3E2D;
    let ad2 = cpu.a[5].wrapping_add(0x6148);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B240: beq.w $5b2de
pub fn i_05b240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B244;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5B2DE; return 10; }
    8
}
/// $05B244: move.w d7, d6
pub fn i_05b244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B246;
    cpu.ir = 0x3C07;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B246: lsr.w #$1, d6
pub fn i_05b246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B248;
    cpu.ir = 0xE24E;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $05B248: andi.w #$fffc, d6
pub fn i_05b248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B24C;
    cpu.ir = 0x246;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = (v1 & 0xFFFC) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05B24C: beq.w $5b2de
pub fn i_05b24c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B250;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5B2DE; return 10; }
    8
}
/// $05B250: move.l (a0, d6.w), d0
pub fn i_05b250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B254;
    cpu.ir = 0x2030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $05B254: move.w d6, d5
pub fn i_05b254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B256;
    cpu.ir = 0x3A06;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B256: move.w d5, d4
pub fn i_05b256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B258;
    cpu.ir = 0x3805;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B258: add.w d4, d4
pub fn i_05b258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B25A;
    cpu.ir = 0xD844;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B25A: move.l (a0, d4.w), d1
pub fn i_05b25a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B25E;
    cpu.ir = 0x2230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $05B25E: cmp.w d7, d4
pub fn i_05b25e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B260;
    cpu.ir = 0xB847;
    let v1 = cpu.d[7] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05B260: bcc.w $5b272
pub fn i_05b260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B264;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B272; return 10; }
    8
}
/// $05B264: move.l $4(a0, d4.w), d2
pub fn i_05b264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B268;
    cpu.ir = 0x2430;
    let ad2 = cpu.a[0].wrapping_add(0x4).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $05B268: cmp.l d1, d2
pub fn i_05b268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B26A;
    cpu.ir = 0xB481;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    4
}
/// $05B26A: bcs.w $5b272
pub fn i_05b26a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B26E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5B272; return 10; }
    8
}
/// $05B26E: move.l d2, d1
pub fn i_05b26e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B270;
    cpu.ir = 0x2202;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[1] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B270: addq.w #$4, d4
pub fn i_05b270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B272;
    cpu.ir = 0x5844;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05B272: cmp.l d0, d1
pub fn i_05b272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B274;
    cpu.ir = 0xB280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    4
}
/// $05B274: bcs.w $5b284
pub fn i_05b274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B278;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5B284; return 10; }
    8
}
/// $05B278: move.l d1, (a0, d5.w)
pub fn i_05b278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B27C;
    cpu.ir = 0x2181;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $05B27C: move.w d4, d5
pub fn i_05b27c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B27E;
    cpu.ir = 0x3A04;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B27E: add.w d4, d4
pub fn i_05b27e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B280;
    cpu.ir = 0xD844;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B280: cmp.w d4, d7
pub fn i_05b280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B282;
    cpu.ir = 0xBE44;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05B282: bcc.b $5b25a
pub fn i_05b282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B284;
    cpu.ir = 0x64D6;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B25A; return 10; }
    8
}
/// $05B284: move.l d0, (a0, d5.w)
pub fn i_05b284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B288;
    cpu.ir = 0x2180;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $05B288: subq.w #$4, d6
pub fn i_05b288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B28A;
    cpu.ir = 0x5946;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05B28A: bne.b $5b250
pub fn i_05b28a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B28C;
    cpu.ir = 0x66C4;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5B250; return 10; }
    8
}
/// $05B28C: move.w #$4, d6
pub fn i_05b28c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B290;
    cpu.ir = 0x3C3C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B290: move.l $4(a0), d1
pub fn i_05b290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B294;
    cpu.ir = 0x2228;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $05B294: move.l (a0, d7.w), d0
pub fn i_05b294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B298;
    cpu.ir = 0x2030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $05B298: move.l d1, (a0, d7.w)
pub fn i_05b298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B29C;
    cpu.ir = 0x2181;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $05B29C: subq.w #$4, d7
pub fn i_05b29c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B29E;
    cpu.ir = 0x5947;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05B29E: cmp.w d6, d7
pub fn i_05b29e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2A0;
    cpu.ir = 0xBE46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05B2A0: beq.w $5b2da
pub fn i_05b2a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2A4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5B2DA; return 10; }
    8
}
/// $05B2A4: move.w d6, d5
pub fn i_05b2a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2A6;
    cpu.ir = 0x3A06;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B2A6: move.w d5, d4
pub fn i_05b2a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2A8;
    cpu.ir = 0x3805;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B2A8: add.w d4, d4
pub fn i_05b2a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2AA;
    cpu.ir = 0xD844;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B2AA: move.l (a0, d4.w), d1
pub fn i_05b2aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2AE;
    cpu.ir = 0x2230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $05B2AE: cmp.w d7, d4
pub fn i_05b2ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2B0;
    cpu.ir = 0xB847;
    let v1 = cpu.d[7] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05B2B0: bcc.w $5b2c2
pub fn i_05b2b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2B4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B2C2; return 10; }
    8
}
/// $05B2B4: move.l $4(a0, d4.w), d2
pub fn i_05b2b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2B8;
    cpu.ir = 0x2430;
    let ad2 = cpu.a[0].wrapping_add(0x4).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $05B2B8: cmp.l d1, d2
pub fn i_05b2b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2BA;
    cpu.ir = 0xB481;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    4
}
/// $05B2BA: bcs.w $5b2c2
pub fn i_05b2ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2BE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5B2C2; return 10; }
    8
}
/// $05B2BE: move.l d2, d1
pub fn i_05b2be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2C0;
    cpu.ir = 0x2202;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[1] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B2C0: addq.w #$4, d4
pub fn i_05b2c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2C2;
    cpu.ir = 0x5844;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05B2C2: cmp.l d0, d1
pub fn i_05b2c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2C4;
    cpu.ir = 0xB280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    4
}
/// $05B2C4: bcs.w $5b2d4
pub fn i_05b2c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2C8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5B2D4; return 10; }
    8
}
/// $05B2C8: move.l d1, (a0, d5.w)
pub fn i_05b2c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2CC;
    cpu.ir = 0x2181;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $05B2CC: move.w d4, d5
pub fn i_05b2cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2CE;
    cpu.ir = 0x3A04;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B2CE: add.w d4, d4
pub fn i_05b2ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2D0;
    cpu.ir = 0xD844;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B2D0: cmp.w d4, d7
pub fn i_05b2d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2D2;
    cpu.ir = 0xBE44;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05B2D2: bcc.b $5b2aa
pub fn i_05b2d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2D4;
    cpu.ir = 0x64D6;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B2AA; return 10; }
    8
}
/// $05B2D4: move.l d0, (a0, d5.w)
pub fn i_05b2d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2D8;
    cpu.ir = 0x2180;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $05B2D8: bra.b $5b290
pub fn i_05b2d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2DA;
    cpu.ir = 0x60B6;
    cpu.pc = 0x5B290;
    10
}
/// $05B2DA: move.l d0, $4(a0)
pub fn i_05b2da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2DE;
    cpu.ir = 0x2140;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $05B2DE: lea.l $5428(a5), a0
pub fn i_05b2de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2E2;
    cpu.ir = 0x41ED;
    let ad1 = cpu.a[5].wrapping_add(0x5428);
    cpu.a[0] = ad1;
    16
}
/// $05B2E2: lea.l $348(a0), a1
pub fn i_05b2e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2E6;
    cpu.ir = 0x43E8;
    let ad1 = cpu.a[0].wrapping_add(0x348);
    cpu.a[1] = ad1;
    16
}
/// $05B2E6: move.w $614a(a5), d7
pub fn i_05b2e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2EA;
    cpu.ir = 0x3E2D;
    let ad2 = cpu.a[5].wrapping_add(0x614A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B2EA: cmpi.w #$344, d7
pub fn i_05b2ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2EE;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x344, Size::Word, false);
    8
}
/// $05B2EE: bcc.w $5b31a
pub fn i_05b2ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2F2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B31A; return 10; }
    8
}
/// $05B2F2: adda.w d7, a0
pub fn i_05b2f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2F4;
    cpu.ir = 0xD0C7;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05B2F4: lea.l $4(a0), a2
pub fn i_05b2f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2F8;
    cpu.ir = 0x45E8;
    let ad1 = cpu.a[0].wrapping_add(0x4);
    cpu.a[2] = ad1;
    16
}
/// $05B2F8: movea.l a2, a3
pub fn i_05b2f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2FA;
    cpu.ir = 0x264A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    cpu.a[3] = v1;
    4
}
/// $05B2FA: move.l (a2)+, d0
pub fn i_05b2fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2FC;
    cpu.ir = 0x201A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B2FC: move.l -(a3), d1
pub fn i_05b2fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2FE;
    cpu.ir = 0x2223;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $05B2FE: cmp.l d0, d1
pub fn i_05b2fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B300;
    cpu.ir = 0xB280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    4
}
/// $05B300: bls.w $5b316
pub fn i_05b300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B304;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x5B316; return 10; }
    8
}
/// $05B304: move.l d1, $4(a3)
pub fn i_05b304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B308;
    cpu.ir = 0x2741;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[3].wrapping_add(0x4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $05B308: move.l -(a3), d1
pub fn i_05b308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B30A;
    cpu.ir = 0x2223;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $05B30A: cmpa.l a3, a0
pub fn i_05b30a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B30C;
    cpu.ir = 0xB1CB;
    let v1 = cpu.a[3] & 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05B30C: bhi.b $5b312
pub fn i_05b30c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B30E;
    cpu.ir = 0x6204;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x5B312; return 10; }
    8
}
/// $05B30E: cmp.l d0, d1
pub fn i_05b30e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B310;
    cpu.ir = 0xB280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    4
}
/// $05B310: bhi.b $5b304
pub fn i_05b310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B312;
    cpu.ir = 0x62F2;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x5B304; return 10; }
    8
}
/// $05B312: move.l d0, $4(a3)
pub fn i_05b312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B316;
    cpu.ir = 0x2740;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[3].wrapping_add(0x4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $05B316: cmpa.l a1, a2
pub fn i_05b316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B318;
    cpu.ir = 0xB5C9;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let h1 = cpu.a[2];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05B318: bcs.b $5b2f8
pub fn i_05b318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B31A;
    cpu.ir = 0x65DE;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5B2F8; return 10; }
    8
}
/// $05B31A: move.l a6, -(a7)
pub fn i_05b31a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B31C;
    cpu.ir = 0x2F0E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $05B31C: lea.l $3c0000.l, a4
pub fn i_05b31c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B322;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0x3C0000;
    20
}
/// $05B322: lea.l $614c(a5), a0
pub fn i_05b322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B326;
    cpu.ir = 0x41ED;
    let ad1 = cpu.a[5].wrapping_add(0x614C);
    cpu.a[0] = ad1;
    16
}
/// $05B326: move.w $10e1f6.l, (a0)
pub fn i_05b326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B32C;
    cpu.ir = 0x30B9;
    let v2 = bus.read16(0x10E1F6) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v2 as u16);
    20
}
/// $05B32C: move.w $10e1fa.l, $2(a0)
pub fn i_05b32c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B334;
    cpu.ir = 0x3179;
    let v2 = bus.read16(0x10E1FA) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_add(0x2);
    bus.write16(ad3, v2 as u16);
    24
}
/// $05B334: lea.l $542a(a5), a1
pub fn i_05b334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B338;
    cpu.ir = 0x43ED;
    let ad1 = cpu.a[5].wrapping_add(0x542A);
    cpu.a[1] = ad1;
    16
}
/// $05B338: move.w $6148(a5), d7
pub fn i_05b338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B33C;
    cpu.ir = 0x3E2D;
    let ad2 = cpu.a[5].wrapping_add(0x6148);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B33C: lea.l (a1, d7.w), a2
pub fn i_05b33c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B340;
    cpu.ir = 0x45F1;
    let ad1 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    cpu.a[2] = ad1;
    18
}
/// $05B340: bsr.w $5b370
pub fn i_05b340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B344;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5B344);
    cpu.pc = 0x5B370;
    18
}
/// $05B344: lea.l $6158(a5), a0
pub fn i_05b344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B348;
    cpu.ir = 0x41ED;
    let ad1 = cpu.a[5].wrapping_add(0x6158);
    cpu.a[0] = ad1;
    16
}
/// $05B348: move.w $10e1f8.l, (a0)
pub fn i_05b348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B34E;
    cpu.ir = 0x30B9;
    let v2 = bus.read16(0x10E1F8) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v2 as u16);
    20
}
/// $05B34E: move.w #$17c, $2(a0)
pub fn i_05b34e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B354;
    cpu.ir = 0x317C;
    let v1: u32 = 0x17C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05B354: lea.l $542a(a5), a1
pub fn i_05b354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B358;
    cpu.ir = 0x43ED;
    let ad1 = cpu.a[5].wrapping_add(0x542A);
    cpu.a[1] = ad1;
    16
}
/// $05B358: lea.l $348(a1), a2
pub fn i_05b358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B35C;
    cpu.ir = 0x45E9;
    let ad1 = cpu.a[1].wrapping_add(0x348);
    cpu.a[2] = ad1;
    16
}
/// $05B35C: move.w $614a(a5), d7
pub fn i_05b35c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B360;
    cpu.ir = 0x3E2D;
    let ad2 = cpu.a[5].wrapping_add(0x614A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B360: adda.w d7, a1
pub fn i_05b360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B362;
    cpu.ir = 0xD2C7;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.a[1] = cpu.a[1].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05B362: bsr.w $5b370
pub fn i_05b362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B366;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5B366);
    cpu.pc = 0x5B370;
    18
}
/// $05B366: movea.l (a7)+, a6
pub fn i_05b366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B368;
    cpu.ir = 0x2C5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    12
}
/// $05B368: move.b #$ff, $616c(a5)
pub fn i_05b368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B36E;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x616C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05B36E: rts
pub fn i_05b36e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B370;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SpriteQueue_RenderRange_05b370  ($05B370, 144 B, video/sprite_queue_render_05aa96.s)
/// $05B370: move.w $4(a0), d2
pub fn i_05b370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B374;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B374: not.b $8(a0)
pub fn i_05b374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B378;
    cpu.ir = 0x4628;
    let ad1 = cpu.a[0].wrapping_add(0x8);
    let v2 = bus.read8(ad1) as u32;
    let r3 = !v2 & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $05B378: bpl.w $5b3be
pub fn i_05b378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B37C;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x5B3BE; return 10; }
    8
}
/// $05B37C: move.w (a0), d0
pub fn i_05b37c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B37E;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $05B37E: move.w d0, d1
pub fn i_05b37e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B380;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B380: lsl.w #$6, d1
pub fn i_05b380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B382;
    cpu.ir = 0xED49;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 6, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    18
}
/// $05B382: move.w $2(a0), d3
pub fn i_05b382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B386;
    cpu.ir = 0x3628;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B386: cmpa.l a1, a2
pub fn i_05b386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B388;
    cpu.ir = 0xB5C9;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let h1 = cpu.a[2];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05B388: bls.w $5b3ba
pub fn i_05b388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B38C;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x5B3BA; return 10; }
    8
}
/// $05B38C: move.w (a1), d5
pub fn i_05b38c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B38E;
    cpu.ir = 0x3A11;
    let v2 = bus.read16(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $05B38E: lea.l $4258(a5), a3
pub fn i_05b38e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B392;
    cpu.ir = 0x47ED;
    let ad1 = cpu.a[5].wrapping_add(0x4258);
    cpu.a[3] = ad1;
    16
}
/// $05B392: adda.w d5, a3
pub fn i_05b392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B394;
    cpu.ir = 0xD6C5;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.a[3] = cpu.a[3].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05B394: cmp.w d3, d0
pub fn i_05b394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B396;
    cpu.ir = 0xB043;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05B396: bcc.w $5b3fa
pub fn i_05b396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B39A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B3FA; return 10; }
    8
}
/// $05B39A: cmp.w d2, d0
pub fn i_05b39a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B39C;
    cpu.ir = 0xB042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05B39C: bcc.w $5b3fa
pub fn i_05b39c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3A0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B3FA; return 10; }
    8
}
/// $05B3A0: bsr.w $5b52e
pub fn i_05b3a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3A4;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5B3A4);
    cpu.pc = 0x5B52E;
    18
}
/// $05B3A4: addq.w #$1, d0
pub fn i_05b3a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3A6;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05B3A6: addi.w #$40, d1
pub fn i_05b3a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3AA;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x40, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05B3AA: tst.b $6(a3)
pub fn i_05b3aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3AE;
    cpu.ir = 0x4A2B;
    let ad2 = cpu.a[3].wrapping_add(0x6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $05B3AE: adda.w #$c, a3
pub fn i_05b3ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3B2;
    cpu.ir = 0xD6FC;
    let v1: u32 = 0xC;
    cpu.a[3] = cpu.a[3].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05B3B2: bpl.b $5b394
pub fn i_05b3b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3B4;
    cpu.ir = 0x6AE0;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x5B394; return 10; }
    8
}
/// $05B3B4: adda.w #$4, a1
pub fn i_05b3b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3B8;
    cpu.ir = 0xD2FC;
    let v1: u32 = 0x4;
    cpu.a[1] = cpu.a[1].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05B3B8: bra.b $5b386
pub fn i_05b3b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3BA;
    cpu.ir = 0x60CC;
    cpu.pc = 0x5B386;
    10
}
/// $05B3BA: bra.w $5b3fa
pub fn i_05b3ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3BE;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B3FA;
    10
}
/// $05B3BE: move.w $2(a0), d0
pub fn i_05b3be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3C2;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B3C2: move.w d0, d1
pub fn i_05b3c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3C4;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B3C4: lsl.w #$6, d1
pub fn i_05b3c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3C6;
    cpu.ir = 0xED49;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 6, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    18
}
/// $05B3C6: move.w (a0), d3
pub fn i_05b3c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3C8;
    cpu.ir = 0x3610;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $05B3C8: cmpa.l a1, a2
pub fn i_05b3c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3CA;
    cpu.ir = 0xB5C9;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let h1 = cpu.a[2];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05B3CA: bls.w $5b3fa
pub fn i_05b3ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3CE;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x5B3FA; return 10; }
    8
}
/// $05B3CE: subq.l #$4, a2
pub fn i_05b3ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3D0;
    cpu.ir = 0x598A;
    cpu.a[2] = cpu.a[2].wrapping_sub(4);
    8
}
/// $05B3D0: move.w (a2), d5
pub fn i_05b3d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3D2;
    cpu.ir = 0x3A12;
    let v2 = bus.read16(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $05B3D2: lea.l $4258(a5), a3
pub fn i_05b3d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3D6;
    cpu.ir = 0x47ED;
    let ad1 = cpu.a[5].wrapping_add(0x4258);
    cpu.a[3] = ad1;
    16
}
/// $05B3D6: adda.w d5, a3
pub fn i_05b3d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3D8;
    cpu.ir = 0xD6C5;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.a[3] = cpu.a[3].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05B3D8: cmp.w d0, d3
pub fn i_05b3d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3DA;
    cpu.ir = 0xB640;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[3] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05B3DA: bcc.w $5b3fa
pub fn i_05b3da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3DE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B3FA; return 10; }
    8
}
/// $05B3DE: cmp.w d0, d2
pub fn i_05b3de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3E0;
    cpu.ir = 0xB440;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05B3E0: bcc.w $5b3fa
pub fn i_05b3e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3E4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B3FA; return 10; }
    8
}
/// $05B3E4: subq.w #$1, d0
pub fn i_05b3e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3E6;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05B3E6: subi.w #$40, d1
pub fn i_05b3e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3EA;
    cpu.ir = 0x441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x40, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05B3EA: bsr.w $5b52e
pub fn i_05b3ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3EE;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5B3EE);
    cpu.pc = 0x5B52E;
    18
}
/// $05B3EE: tst.b $6(a3)
pub fn i_05b3ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3F2;
    cpu.ir = 0x4A2B;
    let ad2 = cpu.a[3].wrapping_add(0x6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $05B3F2: adda.w #$c, a3
pub fn i_05b3f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3F6;
    cpu.ir = 0xD6FC;
    let v1: u32 = 0xC;
    cpu.a[3] = cpu.a[3].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05B3F6: bpl.b $5b3d8
pub fn i_05b3f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3F8;
    cpu.ir = 0x6AE0;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x5B3D8; return 10; }
    8
}
/// $05B3F8: bra.b $5b3c8
pub fn i_05b3f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3FA;
    cpu.ir = 0x60CE;
    cpu.pc = 0x5B3C8;
    10
}
/// $05B3FA: move.w d0, $6(a0)
pub fn i_05b3fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3FE;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x6);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05B3FE: rts
pub fn i_05b3fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B400;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SpriteQueue_RenderSCB234_05b400  ($05B400, 302 B, video/sprite_queue_render_05aa96.s)
/// $05B400: tst.b $8(a0)
pub fn i_05b400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B404;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x8);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $05B404: bpl.w $5b496
pub fn i_05b404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B408;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x5B496; return 10; }
    8
}
/// $05B408: move.w (a0), d0
pub fn i_05b408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B40A;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $05B40A: move.w d0, d1
pub fn i_05b40a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B40C;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B40C: lsl.w #$6, d1
pub fn i_05b40c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B40E;
    cpu.ir = 0xED49;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 6, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    18
}
/// $05B40E: move.w $2(a0), d2
pub fn i_05b40e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B412;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B412: move.w #$40, d3
pub fn i_05b412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B416;
    cpu.ir = 0x363C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B416: cmpa.l a1, a2
pub fn i_05b416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B418;
    cpu.ir = 0xB5C9;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let h1 = cpu.a[2];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05B418: bls.w $5b45c
pub fn i_05b418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B41C;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x5B45C; return 10; }
    8
}
/// $05B41C: move.w (a1), d5
pub fn i_05b41c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B41E;
    cpu.ir = 0x3A11;
    let v2 = bus.read16(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $05B41E: lea.l $4258(a5), a3
pub fn i_05b41e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B422;
    cpu.ir = 0x47ED;
    let ad1 = cpu.a[5].wrapping_add(0x4258);
    cpu.a[3] = ad1;
    16
}
/// $05B422: adda.w d5, a3
pub fn i_05b422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B424;
    cpu.ir = 0xD6C5;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.a[3] = cpu.a[3].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05B424: cmp.w d2, d0
pub fn i_05b424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B426;
    cpu.ir = 0xB042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05B426: bcc.w $5b524
pub fn i_05b426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B42A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B524; return 10; }
    8
}
/// $05B42A: cmp.w $6(a0), d0
pub fn i_05b42a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B42E;
    cpu.ir = 0xB068;
    let ad2 = cpu.a[0].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $05B42E: bcc.w $5b444
pub fn i_05b42e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B432;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B444; return 10; }
    8
}
/// $05B432: move.w #$8201, d7
pub fn i_05b432(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B436;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x8201;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B436: add.w d0, d7
pub fn i_05b436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B438;
    cpu.ir = 0xDE40;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B438: swap d7
pub fn i_05b438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B43A;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B43A: move.w $8(a3), d7
pub fn i_05b43a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B43E;
    cpu.ir = 0x3E2B;
    let ad2 = cpu.a[3].wrapping_add(0x8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B43E: move.l d7, (a4)
pub fn i_05b43e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B440;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B440: bra.w $5b448
pub fn i_05b440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B444;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B448;
    10
}
/// $05B444: bsr.w $5bf76
pub fn i_05b444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B448;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5B448);
    cpu.pc = 0x5BF76;
    18
}
/// $05B448: addq.w #$1, d0
pub fn i_05b448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B44A;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05B44A: add.w d3, d1
pub fn i_05b44a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B44C;
    cpu.ir = 0xD243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B44C: tst.b $6(a3)
pub fn i_05b44c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B450;
    cpu.ir = 0x4A2B;
    let ad2 = cpu.a[3].wrapping_add(0x6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $05B450: adda.w #$c, a3
pub fn i_05b450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B454;
    cpu.ir = 0xD6FC;
    let v1: u32 = 0xC;
    cpu.a[3] = cpu.a[3].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05B454: bpl.b $5b424
pub fn i_05b454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B456;
    cpu.ir = 0x6ACE;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x5B424; return 10; }
    8
}
/// $05B456: adda.w #$4, a1
pub fn i_05b456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B45A;
    cpu.ir = 0xD2FC;
    let v1: u32 = 0x4;
    cpu.a[1] = cpu.a[1].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05B45A: bra.b $5b416
pub fn i_05b45a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B45C;
    cpu.ir = 0x60BA;
    cpu.pc = 0x5B416;
    10
}
/// $05B45C: move.w d2, d1
pub fn i_05b45c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B45E;
    cpu.ir = 0x3202;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B45E: sub.w d0, d1
pub fn i_05b45e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B460;
    cpu.ir = 0x9240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B460: move.w d1, $a(a0)
pub fn i_05b460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B464;
    cpu.ir = 0x3141;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0xA);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05B464: move.w $4(a0), d1
pub fn i_05b464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B468;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B468: bpl.w $5b46e
pub fn i_05b468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B46C;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x5B46E; return 10; }
    8
}
/// $05B46C: move.w d2, d1
pub fn i_05b46c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B46E;
    cpu.ir = 0x3202;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B46E: cmp.w d0, d1
pub fn i_05b46e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B470;
    cpu.ir = 0xB240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05B470: bcc.w $5b476
pub fn i_05b470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B474;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B476; return 10; }
    8
}
/// $05B474: move.w d0, d1
pub fn i_05b474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B476;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B476: sub.w d1, d2
pub fn i_05b476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B478;
    cpu.ir = 0x9441;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B478: move.l #$8201, d7
pub fn i_05b478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B47E;
    cpu.ir = 0x2E3C;
    let v1: u32 = 0x8201;
    cpu.d[7] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B47E: add.w d1, d7
pub fn i_05b47e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B480;
    cpu.ir = 0xDE41;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B480: swap d7
pub fn i_05b480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B482;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B482: moveq #$1, d6
pub fn i_05b482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B484;
    cpu.ir = 0x7C01;
    cpu.d[6] = 0x1;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B484: swap d6
pub fn i_05b484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B486;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B486: bra.w $5b48e
pub fn i_05b486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B48A;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B48E;
    10
}
/// $05B48A: move.l d7, (a4)
pub fn i_05b48a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B48C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B48C: add.l d6, d7
pub fn i_05b48c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B48E;
    cpu.ir = 0xDE86;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B48E: dbra d2, $5b48a
pub fn i_05b48e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B492;
    cpu.ir = 0x51CA;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5B492; return 12; }
    let n = (cpu.d[2] as u16).wrapping_sub(1);
    cpu.d[2] = (cpu.d[2] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5B492; return 14; }
    cpu.pc = 0x5B48A;
    10
}
/// $05B492: bra.w $5b51e
pub fn i_05b492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B496;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B51E;
    10
}
/// $05B496: move.w $2(a0), d0
pub fn i_05b496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B49A;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B49A: move.w d0, d1
pub fn i_05b49a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B49C;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B49C: lsl.w #$6, d1
pub fn i_05b49c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B49E;
    cpu.ir = 0xED49;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 6, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    18
}
/// $05B49E: move.w (a0), d2
pub fn i_05b49e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4A0;
    cpu.ir = 0x3410;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $05B4A0: move.w #$40, d3
pub fn i_05b4a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4A4;
    cpu.ir = 0x363C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B4A4: cmpa.l a1, a2
pub fn i_05b4a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4A6;
    cpu.ir = 0xB5C9;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let h1 = cpu.a[2];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05B4A6: bls.w $5b4e8
pub fn i_05b4a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4AA;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x5B4E8; return 10; }
    8
}
/// $05B4AA: subq.l #$4, a2
pub fn i_05b4aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4AC;
    cpu.ir = 0x598A;
    cpu.a[2] = cpu.a[2].wrapping_sub(4);
    8
}
/// $05B4AC: move.w (a2), d5
pub fn i_05b4ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4AE;
    cpu.ir = 0x3A12;
    let v2 = bus.read16(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $05B4AE: lea.l $4258(a5), a3
pub fn i_05b4ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4B2;
    cpu.ir = 0x47ED;
    let ad1 = cpu.a[5].wrapping_add(0x4258);
    cpu.a[3] = ad1;
    16
}
/// $05B4B2: adda.w d5, a3
pub fn i_05b4b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4B4;
    cpu.ir = 0xD6C5;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.a[3] = cpu.a[3].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05B4B4: cmp.w d0, d2
pub fn i_05b4b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4B6;
    cpu.ir = 0xB440;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05B4B6: bcc.w $5b524
pub fn i_05b4b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4BA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B524; return 10; }
    8
}
/// $05B4BA: subq.w #$1, d0
pub fn i_05b4ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4BC;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05B4BC: sub.w d3, d1
pub fn i_05b4bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4BE;
    cpu.ir = 0x9243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B4BE: cmp.w $6(a0), d0
pub fn i_05b4be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4C2;
    cpu.ir = 0xB068;
    let ad2 = cpu.a[0].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $05B4C2: bcs.w $5b4d8
pub fn i_05b4c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4C6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5B4D8; return 10; }
    8
}
/// $05B4C6: move.w #$8201, d7
pub fn i_05b4c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4CA;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x8201;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B4CA: add.w d0, d7
pub fn i_05b4ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4CC;
    cpu.ir = 0xDE40;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B4CC: swap d7
pub fn i_05b4cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4CE;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B4CE: move.w $8(a3), d7
pub fn i_05b4ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4D2;
    cpu.ir = 0x3E2B;
    let ad2 = cpu.a[3].wrapping_add(0x8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B4D2: move.l d7, (a4)
pub fn i_05b4d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4D4;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B4D4: bra.w $5b4dc
pub fn i_05b4d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4D8;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B4DC;
    10
}
/// $05B4D8: bsr.w $5bf76
pub fn i_05b4d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4DC;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5B4DC);
    cpu.pc = 0x5BF76;
    18
}
/// $05B4DC: tst.b $6(a3)
pub fn i_05b4dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4E0;
    cpu.ir = 0x4A2B;
    let ad2 = cpu.a[3].wrapping_add(0x6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $05B4E0: adda.w #$c, a3
pub fn i_05b4e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4E4;
    cpu.ir = 0xD6FC;
    let v1: u32 = 0xC;
    cpu.a[3] = cpu.a[3].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05B4E4: bpl.b $5b4b4
pub fn i_05b4e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4E6;
    cpu.ir = 0x6ACE;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x5B4B4; return 10; }
    8
}
/// $05B4E6: bra.b $5b4a4
pub fn i_05b4e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4E8;
    cpu.ir = 0x60BC;
    cpu.pc = 0x5B4A4;
    10
}
/// $05B4E8: move.w d0, d1
pub fn i_05b4e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4EA;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B4EA: sub.w d2, d1
pub fn i_05b4ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4EC;
    cpu.ir = 0x9242;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B4EC: move.w d1, $a(a0)
pub fn i_05b4ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4F0;
    cpu.ir = 0x3141;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0xA);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05B4F0: move.w $4(a0), d1
pub fn i_05b4f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4F4;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B4F4: bpl.w $5b4fa
pub fn i_05b4f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4F8;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x5B4FA; return 10; }
    8
}
/// $05B4F8: move.w d2, d1
pub fn i_05b4f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4FA;
    cpu.ir = 0x3202;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B4FA: cmp.w d0, d1
pub fn i_05b4fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4FC;
    cpu.ir = 0xB240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05B4FC: bcs.w $5b502
pub fn i_05b4fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B500;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5B502; return 10; }
    8
}
/// $05B500: move.w d0, d1
pub fn i_05b500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B502;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B502: sub.w d2, d1
pub fn i_05b502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B504;
    cpu.ir = 0x9242;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B504: move.l #$8201, d7
pub fn i_05b504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B50A;
    cpu.ir = 0x2E3C;
    let v1: u32 = 0x8201;
    cpu.d[7] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B50A: add.w d2, d7
pub fn i_05b50a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B50C;
    cpu.ir = 0xDE42;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B50C: swap d7
pub fn i_05b50c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B50E;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B50E: moveq #$1, d6
pub fn i_05b50e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B510;
    cpu.ir = 0x7C01;
    cpu.d[6] = 0x1;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B510: swap d6
pub fn i_05b510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B512;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B512: bra.w $5b51a
pub fn i_05b512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B516;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B51A;
    10
}
/// $05B516: move.l d7, (a4)
pub fn i_05b516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B518;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B518: add.l d6, d7
pub fn i_05b518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B51A;
    cpu.ir = 0xDE86;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B51A: dbra d1, $5b516
pub fn i_05b51a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B51E;
    cpu.ir = 0x51C9;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5B51E; return 12; }
    let n = (cpu.d[1] as u16).wrapping_sub(1);
    cpu.d[1] = (cpu.d[1] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5B51E; return 14; }
    cpu.pc = 0x5B516;
    10
}
/// $05B51E: move.w d0, $4(a0)
pub fn i_05b51e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B522;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05B522: rts
pub fn i_05b522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B524;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05B524: move.w d0, $4(a0)
pub fn i_05b524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B528;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05B528: clr.w $a(a0)
pub fn i_05b528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B52C;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0xA);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $05B52C: rts
pub fn i_05b52c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B52E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SCB1_WriteTileColumn_05b52e  ($05B52E, 2632 B, video/sprite_queue_render_05aa96.s)
/// $05B52E: move.w $6(a3), d7
pub fn i_05b52e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B532;
    cpu.ir = 0x3E2B;
    let ad2 = cpu.a[3].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B532: addq.b #$1, d7
pub fn i_05b532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B534;
    cpu.ir = 0x5207;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05B534: sne.b d6
pub fn i_05b534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B536;
    cpu.ir = 0x56C6;
    let c1: u32 = if cpu.sr.evaluate(Condition::Ne) { 0xFF } else { 0 };
    let v2 = cpu.d[6] & 0xFF;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (c1 & 0xFF);
    4
}
/// $05B536: andi.b #$4, d6
pub fn i_05b536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B53A;
    cpu.ir = 0x206;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = (v1 & 0x4) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05B53A: rol.w #$7, d7
pub fn i_05b53a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B53C;
    cpu.ir = 0xEF5F;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 7, Size::Word, 3, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    20
}
/// $05B53C: andi.w #$8, d7
pub fn i_05b53c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B540;
    cpu.ir = 0x247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = (v1 & 0x8) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05B540: or.b d6, d7
pub fn i_05b540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B542;
    cpu.ir = 0x8E06;
    let v1 = cpu.d[6] & 0xFF;
    let r = (v1 | cpu.d[7]) & 0xFF;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05B542: jmp $5b546(pc, d7.w)
pub fn i_05b542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B546;
    cpu.ir = 0x4EFB;
    let ad1 = 0x5B546_u32.wrapping_add((cpu.d[7] as i16 as i32 as u32));
    cpu.jump_to(ad1);
    8
}
/// $05B546: bra.w $5b556
pub fn i_05b546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B54A;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B556;
    10
}
/// $05B54A: bra.w $5b962
pub fn i_05b54a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B54E;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B962;
    10
}
/// $05B54E: bra.w $5b75c
pub fn i_05b54e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B552;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B75C;
    10
}
/// $05B552: bra.w $5bc6c
pub fn i_05b552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B556;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BC6C;
    10
}
/// $05B556: move.w #$40, d7
pub fn i_05b556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B55A;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B55A: add.w d1, d7
pub fn i_05b55a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B55C;
    cpu.ir = 0xDE41;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B55C: move.w d7, d5
pub fn i_05b55c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B55E;
    cpu.ir = 0x3A07;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B55E: addq.w #$1, d5
pub fn i_05b55e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B560;
    cpu.ir = 0x5245;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05B560: swap d5
pub fn i_05b560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B562;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B562: move.w $4(a3), d5
pub fn i_05b562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B566;
    cpu.ir = 0x3A2B;
    let ad2 = cpu.a[3].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B566: swap d7
pub fn i_05b566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B568;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B568: moveq #$2, d4
pub fn i_05b568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B56A;
    cpu.ir = 0x7802;
    cpu.d[4] = 0x2;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B56A: swap d4
pub fn i_05b56a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B56C;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B56C: movea.l (a3), a6
pub fn i_05b56c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B56E;
    cpu.ir = 0x2C53;
    let v2 = bus.read32(cpu.a[3]);
    cpu.a[6] = v2;
    12
}
/// $05B56E: move.w $a(a3), d6
pub fn i_05b56e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B572;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B572: subq.b #$1, d6
pub fn i_05b572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B574;
    cpu.ir = 0x5306;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05B574: andi.w #$1f, d6
pub fn i_05b574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B578;
    cpu.ir = 0x246;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05B578: add.w d6, d6
pub fn i_05b578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B57A;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B57A: add.w d6, d6
pub fn i_05b57a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B57C;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B57C: jmp $5b580(pc, d6.w)
pub fn i_05b57c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B580;
    cpu.ir = 0x4EFB;
    let ad1 = 0x5B580_u32.wrapping_add((cpu.d[6] as i16 as i32 as u32));
    cpu.jump_to(ad1);
    8
}
/// $05B580: bra.w $5b732
pub fn i_05b580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B584;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B732;
    10
}
/// $05B584: bra.w $5b728
pub fn i_05b584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B588;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B728;
    10
}
/// $05B588: bra.w $5b71e
pub fn i_05b588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B58C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B71E;
    10
}
/// $05B58C: bra.w $5b714
pub fn i_05b58c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B590;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B714;
    10
}
/// $05B590: bra.w $5b70a
pub fn i_05b590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B594;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B70A;
    10
}
/// $05B594: bra.w $5b700
pub fn i_05b594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B598;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B700;
    10
}
/// $05B598: bra.w $5b6f6
pub fn i_05b598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B59C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B6F6;
    10
}
/// $05B59C: bra.w $5b6ec
pub fn i_05b59c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B5A0;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B6EC;
    10
}
/// $05B5A0: bra.w $5b6e2
pub fn i_05b5a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B5A4;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B6E2;
    10
}
/// $05B5A4: bra.w $5b6d8
pub fn i_05b5a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B5A8;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B6D8;
    10
}
/// $05B5A8: bra.w $5b6ce
pub fn i_05b5a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B5AC;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B6CE;
    10
}
/// $05B5AC: bra.w $5b6c4
pub fn i_05b5ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B5B0;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B6C4;
    10
}
/// $05B5B0: bra.w $5b6ba
pub fn i_05b5b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B5B4;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B6BA;
    10
}
/// $05B5B4: bra.w $5b6b0
pub fn i_05b5b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B5B8;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B6B0;
    10
}
/// $05B5B8: bra.w $5b6a6
pub fn i_05b5b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B5BC;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B6A6;
    10
}
/// $05B5BC: bra.w $5b69c
pub fn i_05b5bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B5C0;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B69C;
    10
}
/// $05B5DC: bra.w $5b64c
pub fn i_05b5dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B5E0;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B64C;
    10
}
/// $05B64C: move.w (a6)+, d7
pub fn i_05b64c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B64E;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B64E: move.l d7, (a4)
pub fn i_05b64e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B650;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B650: add.l d4, d7
pub fn i_05b650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B652;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B652: move.l d5, (a4)
pub fn i_05b652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B654;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B654: add.l d4, d5
pub fn i_05b654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B656;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B656: move.w (a6)+, d7
pub fn i_05b656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B658;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B658: move.l d7, (a4)
pub fn i_05b658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B65A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B65A: add.l d4, d7
pub fn i_05b65a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B65C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B65C: move.l d5, (a4)
pub fn i_05b65c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B65E;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B65E: add.l d4, d5
pub fn i_05b65e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B660;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B660: move.w (a6)+, d7
pub fn i_05b660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B662;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B662: move.l d7, (a4)
pub fn i_05b662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B664;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B664: add.l d4, d7
pub fn i_05b664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B666;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B666: move.l d5, (a4)
pub fn i_05b666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B668;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B668: add.l d4, d5
pub fn i_05b668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B66A;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B66A: move.w (a6)+, d7
pub fn i_05b66a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B66C;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B66C: move.l d7, (a4)
pub fn i_05b66c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B66E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B66E: add.l d4, d7
pub fn i_05b66e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B670;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B670: move.l d5, (a4)
pub fn i_05b670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B672;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B672: add.l d4, d5
pub fn i_05b672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B674;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B674: move.w (a6)+, d7
pub fn i_05b674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B676;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B676: move.l d7, (a4)
pub fn i_05b676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B678;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B678: add.l d4, d7
pub fn i_05b678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B67A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B67A: move.l d5, (a4)
pub fn i_05b67a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B67C;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B67C: add.l d4, d5
pub fn i_05b67c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B67E;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B67E: move.w (a6)+, d7
pub fn i_05b67e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B680;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B680: move.l d7, (a4)
pub fn i_05b680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B682;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B682: add.l d4, d7
pub fn i_05b682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B684;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B684: move.l d5, (a4)
pub fn i_05b684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B686;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B686: add.l d4, d5
pub fn i_05b686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B688;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B688: move.w (a6)+, d7
pub fn i_05b688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B68A;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B68A: move.l d7, (a4)
pub fn i_05b68a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B68C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B68C: add.l d4, d7
pub fn i_05b68c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B68E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B68E: move.l d5, (a4)
pub fn i_05b68e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B690;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B690: add.l d4, d5
pub fn i_05b690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B692;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B692: move.w (a6)+, d7
pub fn i_05b692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B694;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B694: move.l d7, (a4)
pub fn i_05b694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B696;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B696: add.l d4, d7
pub fn i_05b696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B698;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B698: move.l d5, (a4)
pub fn i_05b698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B69A;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B69A: add.l d4, d5
pub fn i_05b69a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B69C;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B69C: move.w (a6)+, d7
pub fn i_05b69c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B69E;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B69E: move.l d7, (a4)
pub fn i_05b69e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6A0;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6A0: add.l d4, d7
pub fn i_05b6a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6A2;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B6A2: move.l d5, (a4)
pub fn i_05b6a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6A4;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6A4: add.l d4, d5
pub fn i_05b6a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6A6;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B6A6: move.w (a6)+, d7
pub fn i_05b6a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6A8;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B6A8: move.l d7, (a4)
pub fn i_05b6a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6AA;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6AA: add.l d4, d7
pub fn i_05b6aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6AC;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B6AC: move.l d5, (a4)
pub fn i_05b6ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6AE;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6AE: add.l d4, d5
pub fn i_05b6ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6B0;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B6B0: move.w (a6)+, d7
pub fn i_05b6b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6B2;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B6B2: move.l d7, (a4)
pub fn i_05b6b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6B4;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6B4: add.l d4, d7
pub fn i_05b6b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6B6;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B6B6: move.l d5, (a4)
pub fn i_05b6b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6B8;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6B8: add.l d4, d5
pub fn i_05b6b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6BA;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B6BA: move.w (a6)+, d7
pub fn i_05b6ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6BC;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B6BC: move.l d7, (a4)
pub fn i_05b6bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6BE;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6BE: add.l d4, d7
pub fn i_05b6be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6C0;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B6C0: move.l d5, (a4)
pub fn i_05b6c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6C2;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6C2: add.l d4, d5
pub fn i_05b6c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6C4;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B6C4: move.w (a6)+, d7
pub fn i_05b6c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6C6;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B6C6: move.l d7, (a4)
pub fn i_05b6c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6C8;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6C8: add.l d4, d7
pub fn i_05b6c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6CA;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B6CA: move.l d5, (a4)
pub fn i_05b6ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6CC;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6CC: add.l d4, d5
pub fn i_05b6cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6CE;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B6CE: move.w (a6)+, d7
pub fn i_05b6ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6D0;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B6D0: move.l d7, (a4)
pub fn i_05b6d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6D2;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6D2: add.l d4, d7
pub fn i_05b6d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6D4;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B6D4: move.l d5, (a4)
pub fn i_05b6d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6D6;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6D6: add.l d4, d5
pub fn i_05b6d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6D8;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B6D8: move.w (a6)+, d7
pub fn i_05b6d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6DA;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B6DA: move.l d7, (a4)
pub fn i_05b6da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6DC;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6DC: add.l d4, d7
pub fn i_05b6dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6DE;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B6DE: move.l d5, (a4)
pub fn i_05b6de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6E0;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6E0: add.l d4, d5
pub fn i_05b6e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6E2;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B6E2: move.w (a6)+, d7
pub fn i_05b6e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6E4;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B6E4: move.l d7, (a4)
pub fn i_05b6e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6E6;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6E6: add.l d4, d7
pub fn i_05b6e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6E8;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B6E8: move.l d5, (a4)
pub fn i_05b6e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6EA;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6EA: add.l d4, d5
pub fn i_05b6ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6EC;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B6EC: move.w (a6)+, d7
pub fn i_05b6ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6EE;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B6EE: move.l d7, (a4)
pub fn i_05b6ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6F0;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6F0: add.l d4, d7
pub fn i_05b6f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6F2;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B6F2: move.l d5, (a4)
pub fn i_05b6f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6F4;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6F4: add.l d4, d5
pub fn i_05b6f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6F6;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B6F6: move.w (a6)+, d7
pub fn i_05b6f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6F8;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B6F8: move.l d7, (a4)
pub fn i_05b6f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6FA;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6FA: add.l d4, d7
pub fn i_05b6fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6FC;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B6FC: move.l d5, (a4)
pub fn i_05b6fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6FE;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6FE: add.l d4, d5
pub fn i_05b6fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B700;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B700: move.w (a6)+, d7
pub fn i_05b700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B702;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B702: move.l d7, (a4)
pub fn i_05b702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B704;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B704: add.l d4, d7
pub fn i_05b704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B706;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B706: move.l d5, (a4)
pub fn i_05b706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B708;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B708: add.l d4, d5
pub fn i_05b708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B70A;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B70A: move.w (a6)+, d7
pub fn i_05b70a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B70C;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B70C: move.l d7, (a4)
pub fn i_05b70c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B70E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B70E: add.l d4, d7
pub fn i_05b70e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B710;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B710: move.l d5, (a4)
pub fn i_05b710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B712;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B712: add.l d4, d5
pub fn i_05b712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B714;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B714: move.w (a6)+, d7
pub fn i_05b714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B716;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B716: move.l d7, (a4)
pub fn i_05b716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B718;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B718: add.l d4, d7
pub fn i_05b718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B71A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B71A: move.l d5, (a4)
pub fn i_05b71a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B71C;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B71C: add.l d4, d5
pub fn i_05b71c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B71E;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B71E: move.w (a6)+, d7
pub fn i_05b71e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B720;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B720: move.l d7, (a4)
pub fn i_05b720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B722;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B722: add.l d4, d7
pub fn i_05b722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B724;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B724: move.l d5, (a4)
pub fn i_05b724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B726;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B726: add.l d4, d5
pub fn i_05b726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B728;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B728: move.w (a6)+, d7
pub fn i_05b728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B72A;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B72A: move.l d7, (a4)
pub fn i_05b72a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B72C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B72C: add.l d4, d7
pub fn i_05b72c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B72E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B72E: move.l d5, (a4)
pub fn i_05b72e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B730;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B730: add.l d4, d5
pub fn i_05b730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B732;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B732: move.w (a6)+, d7
pub fn i_05b732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B734;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B734: move.l d7, (a4)
pub fn i_05b734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B736;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B736: add.l d4, d7
pub fn i_05b736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B738;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B738: move.l d5, (a4)
pub fn i_05b738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B73A;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B73A: add.l d4, d5
pub fn i_05b73a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B73C;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B73C: move.w #$8401, d6
pub fn i_05b73c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B740;
    cpu.ir = 0x3C3C;
    let v1: u32 = 0x8401;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B740: add.w d0, d6
pub fn i_05b740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B742;
    cpu.ir = 0xDC40;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B742: swap d6
pub fn i_05b742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B744;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B744: move.w $a(a3), d6
pub fn i_05b744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B748;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B748: move.l #$2000000, d5
pub fn i_05b748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B74E;
    cpu.ir = 0x2A3C;
    let v1: u32 = 0x2000000;
    cpu.d[5] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B74E: move.l d6, (a4)
pub fn i_05b74e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B750;
    cpu.ir = 0x2886;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B750: sub.l d5, d6
pub fn i_05b750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B752;
    cpu.ir = 0x9C85;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[6] = r;
    4
}
/// $05B752: sub.l d5, d6
pub fn i_05b752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B754;
    cpu.ir = 0x9C85;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[6] = r;
    4
}
/// $05B754: move.w $6(a3), d6
pub fn i_05b754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B758;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B758: move.l d6, (a4)
pub fn i_05b758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B75A;
    cpu.ir = 0x2886;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B75A: rts
pub fn i_05b75a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B75C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05B75C: move.w #$40, d7
pub fn i_05b75c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B760;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B760: add.w d1, d7
pub fn i_05b760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B762;
    cpu.ir = 0xDE41;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B762: move.w d7, d5
pub fn i_05b762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B764;
    cpu.ir = 0x3A07;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B764: addq.w #$1, d5
pub fn i_05b764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B766;
    cpu.ir = 0x5245;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05B766: swap d5
pub fn i_05b766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B768;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B768: move.w $4(a3), d5
pub fn i_05b768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B76C;
    cpu.ir = 0x3A2B;
    let ad2 = cpu.a[3].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B76C: swap d7
pub fn i_05b76c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B76E;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B76E: moveq #$2, d4
pub fn i_05b76e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B770;
    cpu.ir = 0x7802;
    cpu.d[4] = 0x2;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B770: swap d4
pub fn i_05b770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B772;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B772: movea.l (a3), a6
pub fn i_05b772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B774;
    cpu.ir = 0x2C53;
    let v2 = bus.read32(cpu.a[3]);
    cpu.a[6] = v2;
    12
}
/// $05B774: move.w $a(a3), d6
pub fn i_05b774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B778;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B778: subq.b #$1, d6
pub fn i_05b778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B77A;
    cpu.ir = 0x5306;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05B77A: andi.w #$1f, d6
pub fn i_05b77a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B77E;
    cpu.ir = 0x246;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05B77E: add.w d6, d6
pub fn i_05b77e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B780;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B780: add.w d6, d6
pub fn i_05b780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B782;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B782: jmp $5b786(pc, d6.w)
pub fn i_05b782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B786;
    cpu.ir = 0x4EFB;
    let ad1 = 0x5B786_u32.wrapping_add((cpu.d[6] as i16 as i32 as u32));
    cpu.jump_to(ad1);
    8
}
/// $05B78A: bra.w $5b92e
pub fn i_05b78a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B78E;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B92E;
    10
}
/// $05B796: bra.w $5b910
pub fn i_05b796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B79A;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B910;
    10
}
/// $05B79A: bra.w $5b906
pub fn i_05b79a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B79E;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B906;
    10
}
/// $05B79E: bra.w $5b8fc
pub fn i_05b79e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B7A2;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B8FC;
    10
}
/// $05B7A2: bra.w $5b8f2
pub fn i_05b7a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B7A6;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B8F2;
    10
}
/// $05B7A6: bra.w $5b8e8
pub fn i_05b7a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B7AA;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B8E8;
    10
}
/// $05B7AA: bra.w $5b8de
pub fn i_05b7aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B7AE;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B8DE;
    10
}
/// $05B8DE: move.w -(a6), d7
pub fn i_05b8de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8E0;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05B8E0: move.l d7, (a4)
pub fn i_05b8e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8E2;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B8E2: add.l d4, d7
pub fn i_05b8e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8E4;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B8E4: move.l d5, (a4)
pub fn i_05b8e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8E6;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B8E6: add.l d4, d5
pub fn i_05b8e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8E8;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B8E8: move.w -(a6), d7
pub fn i_05b8e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8EA;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05B8EA: move.l d7, (a4)
pub fn i_05b8ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8EC;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B8EC: add.l d4, d7
pub fn i_05b8ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8EE;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B8EE: move.l d5, (a4)
pub fn i_05b8ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8F0;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B8F0: add.l d4, d5
pub fn i_05b8f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8F2;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B8F2: move.w -(a6), d7
pub fn i_05b8f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8F4;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05B8F4: move.l d7, (a4)
pub fn i_05b8f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8F6;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B8F6: add.l d4, d7
pub fn i_05b8f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8F8;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B8F8: move.l d5, (a4)
pub fn i_05b8f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8FA;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B8FA: add.l d4, d5
pub fn i_05b8fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8FC;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B8FC: move.w -(a6), d7
pub fn i_05b8fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8FE;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05B8FE: move.l d7, (a4)
pub fn i_05b8fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B900;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B900: add.l d4, d7
pub fn i_05b900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B902;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B902: move.l d5, (a4)
pub fn i_05b902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B904;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B904: add.l d4, d5
pub fn i_05b904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B906;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B906: move.w -(a6), d7
pub fn i_05b906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B908;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05B908: move.l d7, (a4)
pub fn i_05b908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B90A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B90A: add.l d4, d7
pub fn i_05b90a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B90C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B90C: move.l d5, (a4)
pub fn i_05b90c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B90E;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B90E: add.l d4, d5
pub fn i_05b90e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B910;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B910: move.w -(a6), d7
pub fn i_05b910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B912;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05B912: move.l d7, (a4)
pub fn i_05b912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B914;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B914: add.l d4, d7
pub fn i_05b914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B916;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B916: move.l d5, (a4)
pub fn i_05b916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B918;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B918: add.l d4, d5
pub fn i_05b918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B91A;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B91A: move.w -(a6), d7
pub fn i_05b91a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B91C;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05B91C: move.l d7, (a4)
pub fn i_05b91c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B91E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B91E: add.l d4, d7
pub fn i_05b91e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B920;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B920: move.l d5, (a4)
pub fn i_05b920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B922;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B922: add.l d4, d5
pub fn i_05b922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B924;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B924: move.w -(a6), d7
pub fn i_05b924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B926;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05B926: move.l d7, (a4)
pub fn i_05b926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B928;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B928: add.l d4, d7
pub fn i_05b928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B92A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B92A: move.l d5, (a4)
pub fn i_05b92a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B92C;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B92C: add.l d4, d5
pub fn i_05b92c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B92E;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B92E: move.w -(a6), d7
pub fn i_05b92e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B930;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05B930: move.l d7, (a4)
pub fn i_05b930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B932;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B932: add.l d4, d7
pub fn i_05b932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B934;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B934: move.l d5, (a4)
pub fn i_05b934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B936;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B936: add.l d4, d5
pub fn i_05b936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B938;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B938: move.w -(a6), d7
pub fn i_05b938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B93A;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05B93A: move.l d7, (a4)
pub fn i_05b93a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B93C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B93C: add.l d4, d7
pub fn i_05b93c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B93E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B93E: move.l d5, (a4)
pub fn i_05b93e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B940;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B940: add.l d4, d5
pub fn i_05b940(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B942;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B942: move.w #$8401, d6
pub fn i_05b942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B946;
    cpu.ir = 0x3C3C;
    let v1: u32 = 0x8401;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B946: add.w d0, d6
pub fn i_05b946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B948;
    cpu.ir = 0xDC40;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B948: swap d6
pub fn i_05b948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B94A;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B94A: move.w $a(a3), d6
pub fn i_05b94a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B94E;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B94E: move.l #$2000000, d5
pub fn i_05b94e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B954;
    cpu.ir = 0x2A3C;
    let v1: u32 = 0x2000000;
    cpu.d[5] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B954: move.l d6, (a4)
pub fn i_05b954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B956;
    cpu.ir = 0x2886;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B956: sub.l d5, d6
pub fn i_05b956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B958;
    cpu.ir = 0x9C85;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[6] = r;
    4
}
/// $05B958: sub.l d5, d6
pub fn i_05b958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B95A;
    cpu.ir = 0x9C85;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[6] = r;
    4
}
/// $05B95A: move.w $6(a3), d6
pub fn i_05b95a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B95E;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B95E: move.l d6, (a4)
pub fn i_05b95e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B960;
    cpu.ir = 0x2886;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B960: rts
pub fn i_05b960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B962;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05B962: move.w #$40, d7
pub fn i_05b962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B966;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B966: add.w d1, d7
pub fn i_05b966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B968;
    cpu.ir = 0xDE41;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B968: move.w d7, d5
pub fn i_05b968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B96A;
    cpu.ir = 0x3A07;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B96A: addq.w #$1, d5
pub fn i_05b96a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B96C;
    cpu.ir = 0x5245;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05B96C: swap d5
pub fn i_05b96c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B96E;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B96E: move.w $4(a3), d5
pub fn i_05b96e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B972;
    cpu.ir = 0x3A2B;
    let ad2 = cpu.a[3].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B972: swap d7
pub fn i_05b972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B974;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B974: moveq #$2, d4
pub fn i_05b974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B976;
    cpu.ir = 0x7802;
    cpu.d[4] = 0x2;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B976: swap d4
pub fn i_05b976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B978;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B978: movea.l (a3), a6
pub fn i_05b978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B97A;
    cpu.ir = 0x2C53;
    let v2 = bus.read32(cpu.a[3]);
    cpu.a[6] = v2;
    12
}
/// $05B97A: move.w $a(a3), d6
pub fn i_05b97a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B97E;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B97E: subq.b #$1, d6
pub fn i_05b97e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B980;
    cpu.ir = 0x5306;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05B980: andi.w #$1f, d6
pub fn i_05b980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B984;
    cpu.ir = 0x246;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05B984: add.w d6, d6
pub fn i_05b984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B986;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B986: add.w d6, d6
pub fn i_05b986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B988;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B988: jmp $5b98c(pc, d6.w)
pub fn i_05b988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B98C;
    cpu.ir = 0x4EFB;
    let ad1 = 0x5B98C_u32.wrapping_add((cpu.d[6] as i16 as i32 as u32));
    cpu.jump_to(ad1);
    8
}
/// $05B98C: bra.w $5bb3e
pub fn i_05b98c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B990;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BB3E;
    10
}
/// $05B990: bra.w $5bb34
pub fn i_05b990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B994;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BB34;
    10
}
/// $05B994: bra.w $5bb2a
pub fn i_05b994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B998;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BB2A;
    10
}
/// $05B998: bra.w $5bb20
pub fn i_05b998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B99C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BB20;
    10
}
/// $05B99C: bra.w $5bb16
pub fn i_05b99c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B9A0;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BB16;
    10
}
/// $05B9A0: bra.w $5bb0c
pub fn i_05b9a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B9A4;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BB0C;
    10
}
/// $05B9A4: bra.w $5bb02
pub fn i_05b9a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B9A8;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BB02;
    10
}
/// $05B9A8: bra.w $5baf8
pub fn i_05b9a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B9AC;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BAF8;
    10
}
/// $05B9AC: bra.w $5baee
pub fn i_05b9ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B9B0;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BAEE;
    10
}
/// $05B9B0: bra.w $5bae4
pub fn i_05b9b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B9B4;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BAE4;
    10
}
/// $05B9B4: bra.w $5bada
pub fn i_05b9b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B9B8;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BADA;
    10
}
/// $05BADA: move.w (a6)+, d7
pub fn i_05bada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BADC;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BADC: move.l d7, (a4)
pub fn i_05badc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BADE;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BADE: add.l d4, d7
pub fn i_05bade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAE0;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BAE0: move.l d5, (a4)
pub fn i_05bae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAE2;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BAE2: add.l d4, d5
pub fn i_05bae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAE4;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BAE4: move.w (a6)+, d7
pub fn i_05bae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAE6;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BAE6: move.l d7, (a4)
pub fn i_05bae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAE8;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BAE8: add.l d4, d7
pub fn i_05bae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAEA;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BAEA: move.l d5, (a4)
pub fn i_05baea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAEC;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BAEC: add.l d4, d5
pub fn i_05baec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAEE;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BAEE: move.w (a6)+, d7
pub fn i_05baee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAF0;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BAF0: move.l d7, (a4)
pub fn i_05baf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAF2;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BAF2: add.l d4, d7
pub fn i_05baf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAF4;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BAF4: move.l d5, (a4)
pub fn i_05baf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAF6;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BAF6: add.l d4, d5
pub fn i_05baf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAF8;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BAF8: move.w (a6)+, d7
pub fn i_05baf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAFA;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BAFA: move.l d7, (a4)
pub fn i_05bafa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAFC;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BAFC: add.l d4, d7
pub fn i_05bafc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAFE;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BAFE: move.l d5, (a4)
pub fn i_05bafe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB00;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB00: add.l d4, d5
pub fn i_05bb00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB02;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BB02: move.w (a6)+, d7
pub fn i_05bb02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB04;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BB04: move.l d7, (a4)
pub fn i_05bb04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB06;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB06: add.l d4, d7
pub fn i_05bb06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB08;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB08: move.l d5, (a4)
pub fn i_05bb08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB0A;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB0A: add.l d4, d5
pub fn i_05bb0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB0C;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BB0C: move.w (a6)+, d7
pub fn i_05bb0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB0E;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BB0E: move.l d7, (a4)
pub fn i_05bb0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB10;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB10: add.l d4, d7
pub fn i_05bb10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB12;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB12: move.l d5, (a4)
pub fn i_05bb12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB14;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB14: add.l d4, d5
pub fn i_05bb14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB16;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BB16: move.w (a6)+, d7
pub fn i_05bb16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB18;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BB18: move.l d7, (a4)
pub fn i_05bb18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB1A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB1A: add.l d4, d7
pub fn i_05bb1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB1C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB1C: move.l d5, (a4)
pub fn i_05bb1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB1E;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB1E: add.l d4, d5
pub fn i_05bb1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB20;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BB20: move.w (a6)+, d7
pub fn i_05bb20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB22;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BB22: move.l d7, (a4)
pub fn i_05bb22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB24;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB24: add.l d4, d7
pub fn i_05bb24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB26;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB26: move.l d5, (a4)
pub fn i_05bb26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB28;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB28: add.l d4, d5
pub fn i_05bb28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB2A;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BB2A: move.w (a6)+, d7
pub fn i_05bb2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB2C;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BB2C: move.l d7, (a4)
pub fn i_05bb2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB2E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB2E: add.l d4, d7
pub fn i_05bb2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB30;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB30: move.l d5, (a4)
pub fn i_05bb30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB32;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB32: add.l d4, d5
pub fn i_05bb32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB34;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BB34: move.w (a6)+, d7
pub fn i_05bb34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB36;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BB36: move.l d7, (a4)
pub fn i_05bb36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB38;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB38: add.l d4, d7
pub fn i_05bb38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB3A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB3A: move.l d5, (a4)
pub fn i_05bb3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB3C;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB3C: add.l d4, d5
pub fn i_05bb3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB3E;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BB3E: move.w (a6)+, d7
pub fn i_05bb3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB40;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BB40: move.l d7, (a4)
pub fn i_05bb40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB42;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB42: add.l d4, d7
pub fn i_05bb42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB44;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB44: move.l d5, (a4)
pub fn i_05bb44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB46;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB46: add.l d4, d5
pub fn i_05bb46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB48;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BB48: clr.w d7
pub fn i_05bb48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB4A;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05BB4A: moveq #$1, d4
pub fn i_05bb4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB4C;
    cpu.ir = 0x7801;
    cpu.d[4] = 0x1;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05BB4C: swap d4
pub fn i_05bb4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB4E;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05BB4E: add.w d6, d6
pub fn i_05bb4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB50;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05BB50: jmp $5bb54(pc, d6.w)
pub fn i_05bb50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB54;
    cpu.ir = 0x4EFB;
    let ad1 = 0x5BB54_u32.wrapping_add((cpu.d[6] as i16 as i32 as u32));
    cpu.jump_to(ad1);
    8
}
/// $05BB54: move.l d7, (a4)
pub fn i_05bb54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB56;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB56: add.l d4, d7
pub fn i_05bb56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB58;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB58: move.l d7, (a4)
pub fn i_05bb58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB5A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB5A: add.l d4, d7
pub fn i_05bb5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB5C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB5C: move.l d7, (a4)
pub fn i_05bb5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB5E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB5E: add.l d4, d7
pub fn i_05bb5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB60;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB60: move.l d7, (a4)
pub fn i_05bb60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB62;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB62: add.l d4, d7
pub fn i_05bb62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB64;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB64: move.l d7, (a4)
pub fn i_05bb64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB66;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB66: add.l d4, d7
pub fn i_05bb66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB68;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB68: move.l d7, (a4)
pub fn i_05bb68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB6A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB6A: add.l d4, d7
pub fn i_05bb6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB6C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB6C: move.l d7, (a4)
pub fn i_05bb6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB6E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB6E: add.l d4, d7
pub fn i_05bb6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB70;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB70: move.l d7, (a4)
pub fn i_05bb70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB72;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB72: add.l d4, d7
pub fn i_05bb72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB74;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB74: move.l d7, (a4)
pub fn i_05bb74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB76;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB76: add.l d4, d7
pub fn i_05bb76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB78;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB78: move.l d7, (a4)
pub fn i_05bb78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB7A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB7A: add.l d4, d7
pub fn i_05bb7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB7C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB7C: move.l d7, (a4)
pub fn i_05bb7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB7E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB7E: add.l d4, d7
pub fn i_05bb7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB80;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB80: move.l d7, (a4)
pub fn i_05bb80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB82;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB82: add.l d4, d7
pub fn i_05bb82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB84;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB84: move.l d7, (a4)
pub fn i_05bb84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB86;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB86: add.l d4, d7
pub fn i_05bb86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB88;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB88: move.l d7, (a4)
pub fn i_05bb88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB8A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB8A: add.l d4, d7
pub fn i_05bb8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB8C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB8C: move.l d7, (a4)
pub fn i_05bb8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB8E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB8E: add.l d4, d7
pub fn i_05bb8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB90;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB90: move.l d7, (a4)
pub fn i_05bb90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB92;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB92: add.l d4, d7
pub fn i_05bb92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB94;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB94: move.l d7, (a4)
pub fn i_05bb94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB96;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB96: add.l d4, d7
pub fn i_05bb96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB98;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB98: move.l d7, (a4)
pub fn i_05bb98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB9A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB9A: add.l d4, d7
pub fn i_05bb9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB9C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB9C: move.l d7, (a4)
pub fn i_05bb9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB9E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB9E: add.l d4, d7
pub fn i_05bb9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBA0;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBA0: move.l d7, (a4)
pub fn i_05bba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBA2;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBA2: add.l d4, d7
pub fn i_05bba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBA4;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBA4: move.l d7, (a4)
pub fn i_05bba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBA6;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBA6: add.l d4, d7
pub fn i_05bba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBA8;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBA8: move.l d7, (a4)
pub fn i_05bba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBAA;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBAA: add.l d4, d7
pub fn i_05bbaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBAC;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBAC: move.l d7, (a4)
pub fn i_05bbac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBAE;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBAE: add.l d4, d7
pub fn i_05bbae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBB0;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBB0: move.l d7, (a4)
pub fn i_05bbb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBB2;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBB2: add.l d4, d7
pub fn i_05bbb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBB4;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBB4: move.l d7, (a4)
pub fn i_05bbb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBB6;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBB6: add.l d4, d7
pub fn i_05bbb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBB8;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBB8: move.l d7, (a4)
pub fn i_05bbb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBBA;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBBA: add.l d4, d7
pub fn i_05bbba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBBC;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBBC: move.l d7, (a4)
pub fn i_05bbbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBBE;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBBE: add.l d4, d7
pub fn i_05bbbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBC0;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBC0: move.l d7, (a4)
pub fn i_05bbc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBC2;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBC2: add.l d4, d7
pub fn i_05bbc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBC4;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBC4: move.l d7, (a4)
pub fn i_05bbc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBC6;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBC6: add.l d4, d7
pub fn i_05bbc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBC8;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBC8: move.l d7, (a4)
pub fn i_05bbc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBCA;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBCA: add.l d4, d7
pub fn i_05bbca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBCC;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBCC: move.l d7, (a4)
pub fn i_05bbcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBCE;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBCE: add.l d4, d7
pub fn i_05bbce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBD0;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBD0: move.l d7, (a4)
pub fn i_05bbd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBD2;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBD2: add.l d4, d7
pub fn i_05bbd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBD4;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBD4: move.l d7, (a4)
pub fn i_05bbd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBD6;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBD6: add.l d4, d7
pub fn i_05bbd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBD8;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBD8: move.l d7, (a4)
pub fn i_05bbd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBDA;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBDA: add.l d4, d7
pub fn i_05bbda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBDC;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBDC: move.l d7, (a4)
pub fn i_05bbdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBDE;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBDE: add.l d4, d7
pub fn i_05bbde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBE0;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBE0: move.l d7, (a4)
pub fn i_05bbe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBE2;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBE2: add.l d4, d7
pub fn i_05bbe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBE4;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBE4: move.l d7, (a4)
pub fn i_05bbe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBE6;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBE6: add.l d4, d7
pub fn i_05bbe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBE8;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBE8: move.l d7, (a4)
pub fn i_05bbe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBEA;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBEA: add.l d4, d7
pub fn i_05bbea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBEC;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBEC: move.l d7, (a4)
pub fn i_05bbec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBEE;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBEE: add.l d4, d7
pub fn i_05bbee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBF0;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBF0: move.l d7, (a4)
pub fn i_05bbf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBF2;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBF2: add.l d4, d7
pub fn i_05bbf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBF4;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBF4: move.l d7, (a4)
pub fn i_05bbf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBF6;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBF6: add.l d4, d7
pub fn i_05bbf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBF8;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBF8: move.l d7, (a4)
pub fn i_05bbf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBFA;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBFA: add.l d4, d7
pub fn i_05bbfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBFC;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBFC: move.l d7, (a4)
pub fn i_05bbfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBFE;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBFE: add.l d4, d7
pub fn i_05bbfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC00;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC00: move.l d7, (a4)
pub fn i_05bc00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC02;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC02: add.l d4, d7
pub fn i_05bc02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC04;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC04: move.l d7, (a4)
pub fn i_05bc04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC06;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC06: add.l d4, d7
pub fn i_05bc06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC08;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC08: move.l d7, (a4)
pub fn i_05bc08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC0A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC0A: add.l d4, d7
pub fn i_05bc0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC0C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC0C: move.l d7, (a4)
pub fn i_05bc0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC0E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC0E: add.l d4, d7
pub fn i_05bc0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC10;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC10: move.l d7, (a4)
pub fn i_05bc10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC12;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC12: add.l d4, d7
pub fn i_05bc12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC14;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC14: move.l d7, (a4)
pub fn i_05bc14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC16;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC16: add.l d4, d7
pub fn i_05bc16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC18;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC18: move.l d7, (a4)
pub fn i_05bc18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC1A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC1A: add.l d4, d7
pub fn i_05bc1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC1C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC1C: move.l d7, (a4)
pub fn i_05bc1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC1E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC1E: add.l d4, d7
pub fn i_05bc1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC20;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC20: move.l d7, (a4)
pub fn i_05bc20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC22;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC22: add.l d4, d7
pub fn i_05bc22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC24;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC24: move.l d7, (a4)
pub fn i_05bc24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC26;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC26: add.l d4, d7
pub fn i_05bc26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC28;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC28: move.l d7, (a4)
pub fn i_05bc28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC2A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC2A: add.l d4, d7
pub fn i_05bc2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC2C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC2C: move.l d7, (a4)
pub fn i_05bc2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC2E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC2E: add.l d4, d7
pub fn i_05bc2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC30;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC30: move.l d7, (a4)
pub fn i_05bc30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC32;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC32: add.l d4, d7
pub fn i_05bc32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC34;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC34: move.l d7, (a4)
pub fn i_05bc34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC36;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC36: add.l d4, d7
pub fn i_05bc36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC38;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC38: move.l d7, (a4)
pub fn i_05bc38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC3A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC3A: add.l d4, d7
pub fn i_05bc3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC3C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC3C: move.l d7, (a4)
pub fn i_05bc3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC3E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC3E: add.l d4, d7
pub fn i_05bc3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC40;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC40: move.l d7, (a4)
pub fn i_05bc40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC42;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC42: add.l d4, d7
pub fn i_05bc42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC44;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC44: move.l d7, (a4)
pub fn i_05bc44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC46;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC46: add.l d4, d7
pub fn i_05bc46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC48;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC48: move.l d7, (a4)
pub fn i_05bc48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC4A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC4A: add.l d4, d7
pub fn i_05bc4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC4C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC4C: move.w #$8401, d6
pub fn i_05bc4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC50;
    cpu.ir = 0x3C3C;
    let v1: u32 = 0x8401;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05BC50: add.w d0, d6
pub fn i_05bc50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC52;
    cpu.ir = 0xDC40;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05BC52: swap d6
pub fn i_05bc52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC54;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05BC54: move.w $a(a3), d6
pub fn i_05bc54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC58;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05BC58: move.l #$2000000, d5
pub fn i_05bc58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC5E;
    cpu.ir = 0x2A3C;
    let v1: u32 = 0x2000000;
    cpu.d[5] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05BC5E: move.l d6, (a4)
pub fn i_05bc5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC60;
    cpu.ir = 0x2886;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC60: sub.l d5, d6
pub fn i_05bc60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC62;
    cpu.ir = 0x9C85;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[6] = r;
    4
}
/// $05BC62: sub.l d5, d6
pub fn i_05bc62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC64;
    cpu.ir = 0x9C85;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[6] = r;
    4
}
/// $05BC64: move.w $6(a3), d6
pub fn i_05bc64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC68;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05BC68: move.l d6, (a4)
pub fn i_05bc68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC6A;
    cpu.ir = 0x2886;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC6A: rts
pub fn i_05bc6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC6C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05BC6C: move.w #$40, d7
pub fn i_05bc6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC70;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05BC70: add.w d1, d7
pub fn i_05bc70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC72;
    cpu.ir = 0xDE41;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05BC72: move.w d7, d5
pub fn i_05bc72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC74;
    cpu.ir = 0x3A07;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05BC74: addq.w #$1, d5
pub fn i_05bc74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC76;
    cpu.ir = 0x5245;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05BC76: swap d5
pub fn i_05bc76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC78;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05BC78: move.w $4(a3), d5
pub fn i_05bc78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC7C;
    cpu.ir = 0x3A2B;
    let ad2 = cpu.a[3].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05BC7C: swap d7
pub fn i_05bc7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC7E;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05BC7E: moveq #$2, d4
pub fn i_05bc7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC80;
    cpu.ir = 0x7802;
    cpu.d[4] = 0x2;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05BC80: swap d4
pub fn i_05bc80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC82;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05BC82: movea.l (a3), a6
pub fn i_05bc82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC84;
    cpu.ir = 0x2C53;
    let v2 = bus.read32(cpu.a[3]);
    cpu.a[6] = v2;
    12
}
/// $05BC84: move.w $a(a3), d6
pub fn i_05bc84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC88;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05BC88: subq.b #$1, d6
pub fn i_05bc88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC8A;
    cpu.ir = 0x5306;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05BC8A: andi.w #$1f, d6
pub fn i_05bc8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC8E;
    cpu.ir = 0x246;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05BC8E: add.w d6, d6
pub fn i_05bc8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC90;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05BC90: add.w d6, d6
pub fn i_05bc90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC92;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05BC92: jmp $5bc96(pc, d6.w)
pub fn i_05bc92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC96;
    cpu.ir = 0x4EFB;
    let ad1 = 0x5BC96_u32.wrapping_add((cpu.d[6] as i16 as i32 as u32));
    cpu.jump_to(ad1);
    8
}
/// $05BC96: bra.w $5be48
pub fn i_05bc96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC9A;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BE48;
    10
}
/// $05BC9A: bra.w $5be3e
pub fn i_05bc9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC9E;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BE3E;
    10
}
/// $05BC9E: bra.w $5be34
pub fn i_05bc9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BCA2;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BE34;
    10
}
/// $05BCA2: bra.w $5be2a
pub fn i_05bca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BCA6;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BE2A;
    10
}
/// $05BCA6: bra.w $5be20
pub fn i_05bca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BCAA;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BE20;
    10
}
/// $05BCAA: bra.w $5be16
pub fn i_05bcaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BCAE;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BE16;
    10
}
/// $05BCAE: bra.w $5be0c
pub fn i_05bcae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BCB2;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BE0C;
    10
}
/// $05BCB2: bra.w $5be02
pub fn i_05bcb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BCB6;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BE02;
    10
}
/// $05BCB6: bra.w $5bdf8
pub fn i_05bcb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BCBA;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BDF8;
    10
}
/// $05BCBA: bra.w $5bdee
pub fn i_05bcba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BCBE;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BDEE;
    10
}
/// $05BDEE: move.w -(a6), d7
pub fn i_05bdee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDF0;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BDF0: move.l d7, (a4)
pub fn i_05bdf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDF2;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BDF2: add.l d4, d7
pub fn i_05bdf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDF4;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BDF4: move.l d5, (a4)
pub fn i_05bdf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDF6;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BDF6: add.l d4, d5
pub fn i_05bdf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDF8;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BDF8: move.w -(a6), d7
pub fn i_05bdf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDFA;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BDFA: move.l d7, (a4)
pub fn i_05bdfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDFC;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BDFC: add.l d4, d7
pub fn i_05bdfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDFE;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BDFE: move.l d5, (a4)
pub fn i_05bdfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE00;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE00: add.l d4, d5
pub fn i_05be00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE02;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BE02: move.w -(a6), d7
pub fn i_05be02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE04;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BE04: move.l d7, (a4)
pub fn i_05be04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE06;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE06: add.l d4, d7
pub fn i_05be06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE08;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE08: move.l d5, (a4)
pub fn i_05be08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE0A;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE0A: add.l d4, d5
pub fn i_05be0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE0C;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BE0C: move.w -(a6), d7
pub fn i_05be0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE0E;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BE0E: move.l d7, (a4)
pub fn i_05be0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE10;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE10: add.l d4, d7
pub fn i_05be10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE12;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE12: move.l d5, (a4)
pub fn i_05be12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE14;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE14: add.l d4, d5
pub fn i_05be14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE16;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BE16: move.w -(a6), d7
pub fn i_05be16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE18;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BE18: move.l d7, (a4)
pub fn i_05be18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE1A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE1A: add.l d4, d7
pub fn i_05be1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE1C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE1C: move.l d5, (a4)
pub fn i_05be1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE1E;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE1E: add.l d4, d5
pub fn i_05be1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE20;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BE20: move.w -(a6), d7
pub fn i_05be20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE22;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BE22: move.l d7, (a4)
pub fn i_05be22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE24;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE24: add.l d4, d7
pub fn i_05be24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE26;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE26: move.l d5, (a4)
pub fn i_05be26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE28;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE28: add.l d4, d5
pub fn i_05be28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE2A;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BE2A: move.w -(a6), d7
pub fn i_05be2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE2C;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BE2C: move.l d7, (a4)
pub fn i_05be2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE2E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE2E: add.l d4, d7
pub fn i_05be2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE30;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE30: move.l d5, (a4)
pub fn i_05be30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE32;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE32: add.l d4, d5
pub fn i_05be32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE34;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BE34: move.w -(a6), d7
pub fn i_05be34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE36;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BE36: move.l d7, (a4)
pub fn i_05be36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE38;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE38: add.l d4, d7
pub fn i_05be38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE3A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE3A: move.l d5, (a4)
pub fn i_05be3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE3C;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE3C: add.l d4, d5
pub fn i_05be3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE3E;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BE3E: move.w -(a6), d7
pub fn i_05be3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE40;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BE40: move.l d7, (a4)
pub fn i_05be40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE42;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE42: add.l d4, d7
pub fn i_05be42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE44;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE44: move.l d5, (a4)
pub fn i_05be44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE46;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE46: add.l d4, d5
pub fn i_05be46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE48;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BE48: move.w -(a6), d7
pub fn i_05be48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE4A;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BE4A: move.l d7, (a4)
pub fn i_05be4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE4C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE4C: add.l d4, d7
pub fn i_05be4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE4E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE4E: move.l d5, (a4)
pub fn i_05be4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE50;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE50: add.l d4, d5
pub fn i_05be50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE52;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BE52: clr.w d7
pub fn i_05be52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE54;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05BE54: moveq #$1, d4
pub fn i_05be54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE56;
    cpu.ir = 0x7801;
    cpu.d[4] = 0x1;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05BE56: swap d4
pub fn i_05be56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE58;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05BE58: add.w d6, d6
pub fn i_05be58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE5A;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05BE5A: jmp $5be5e(pc, d6.w)
pub fn i_05be5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE5E;
    cpu.ir = 0x4EFB;
    let ad1 = 0x5BE5E_u32.wrapping_add((cpu.d[6] as i16 as i32 as u32));
    cpu.jump_to(ad1);
    8
}
/// $05BE5E: move.l d7, (a4)
pub fn i_05be5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE60;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE60: add.l d4, d7
pub fn i_05be60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE62;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE62: move.l d7, (a4)
pub fn i_05be62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE64;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE64: add.l d4, d7
pub fn i_05be64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE66;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE66: move.l d7, (a4)
pub fn i_05be66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE68;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE68: add.l d4, d7
pub fn i_05be68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE6A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE6A: move.l d7, (a4)
pub fn i_05be6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE6C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE6C: add.l d4, d7
pub fn i_05be6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE6E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE6E: move.l d7, (a4)
pub fn i_05be6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE70;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE70: add.l d4, d7
pub fn i_05be70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE72;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE72: move.l d7, (a4)
pub fn i_05be72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE74;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE74: add.l d4, d7
pub fn i_05be74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE76;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE76: move.l d7, (a4)
pub fn i_05be76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE78;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE78: add.l d4, d7
pub fn i_05be78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE7A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE7A: move.l d7, (a4)
pub fn i_05be7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE7C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE7C: add.l d4, d7
pub fn i_05be7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE7E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE7E: move.l d7, (a4)
pub fn i_05be7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE80;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE80: add.l d4, d7
pub fn i_05be80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE82;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE82: move.l d7, (a4)
pub fn i_05be82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE84;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE84: add.l d4, d7
pub fn i_05be84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE86;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE86: move.l d7, (a4)
pub fn i_05be86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE88;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE88: add.l d4, d7
pub fn i_05be88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE8A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE8A: move.l d7, (a4)
pub fn i_05be8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE8C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE8C: add.l d4, d7
pub fn i_05be8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE8E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE8E: move.l d7, (a4)
pub fn i_05be8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE90;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE90: add.l d4, d7
pub fn i_05be90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE92;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE92: move.l d7, (a4)
pub fn i_05be92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE94;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE94: add.l d4, d7
pub fn i_05be94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE96;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE96: move.l d7, (a4)
pub fn i_05be96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE98;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE98: add.l d4, d7
pub fn i_05be98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE9A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE9A: move.l d7, (a4)
pub fn i_05be9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE9C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE9C: add.l d4, d7
pub fn i_05be9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE9E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE9E: move.l d7, (a4)
pub fn i_05be9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEA0;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEA0: add.l d4, d7
pub fn i_05bea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEA2;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEA2: move.l d7, (a4)
pub fn i_05bea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEA4;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEA4: add.l d4, d7
pub fn i_05bea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEA6;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEA6: move.l d7, (a4)
pub fn i_05bea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEA8;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEA8: add.l d4, d7
pub fn i_05bea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEAA;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEAA: move.l d7, (a4)
pub fn i_05beaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEAC;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEAC: add.l d4, d7
pub fn i_05beac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEAE;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEAE: move.l d7, (a4)
pub fn i_05beae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEB0;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEB0: add.l d4, d7
pub fn i_05beb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEB2;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEB2: move.l d7, (a4)
pub fn i_05beb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEB4;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEB4: add.l d4, d7
pub fn i_05beb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEB6;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEB6: move.l d7, (a4)
pub fn i_05beb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEB8;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEB8: add.l d4, d7
pub fn i_05beb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEBA;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEBA: move.l d7, (a4)
pub fn i_05beba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEBC;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEBC: add.l d4, d7
pub fn i_05bebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEBE;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEBE: move.l d7, (a4)
pub fn i_05bebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEC0;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEC0: add.l d4, d7
pub fn i_05bec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEC2;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEC2: move.l d7, (a4)
pub fn i_05bec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEC4;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEC4: add.l d4, d7
pub fn i_05bec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEC6;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEC6: move.l d7, (a4)
pub fn i_05bec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEC8;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEC8: add.l d4, d7
pub fn i_05bec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BECA;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BECA: move.l d7, (a4)
pub fn i_05beca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BECC;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BECC: add.l d4, d7
pub fn i_05becc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BECE;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BECE: move.l d7, (a4)
pub fn i_05bece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BED0;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BED0: add.l d4, d7
pub fn i_05bed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BED2;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BED2: move.l d7, (a4)
pub fn i_05bed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BED4;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BED4: add.l d4, d7
pub fn i_05bed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BED6;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BED6: move.l d7, (a4)
pub fn i_05bed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BED8;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BED8: add.l d4, d7
pub fn i_05bed8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEDA;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEDA: move.l d7, (a4)
pub fn i_05beda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEDC;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEDC: add.l d4, d7
pub fn i_05bedc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEDE;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEDE: move.l d7, (a4)
pub fn i_05bede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEE0;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEE0: add.l d4, d7
pub fn i_05bee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEE2;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEE2: move.l d7, (a4)
pub fn i_05bee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEE4;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEE4: add.l d4, d7
pub fn i_05bee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEE6;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEE6: move.l d7, (a4)
pub fn i_05bee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEE8;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEE8: add.l d4, d7
pub fn i_05bee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEEA;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEEA: move.l d7, (a4)
pub fn i_05beea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEEC;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEEC: add.l d4, d7
pub fn i_05beec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEEE;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEEE: move.l d7, (a4)
pub fn i_05beee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEF0;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEF0: add.l d4, d7
pub fn i_05bef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEF2;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEF2: move.l d7, (a4)
pub fn i_05bef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEF4;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEF4: add.l d4, d7
pub fn i_05bef4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEF6;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEF6: move.l d7, (a4)
pub fn i_05bef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEF8;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEF8: add.l d4, d7
pub fn i_05bef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEFA;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEFA: move.l d7, (a4)
pub fn i_05befa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEFC;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEFC: add.l d4, d7
pub fn i_05befc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEFE;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEFE: move.l d7, (a4)
pub fn i_05befe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF00;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF00: add.l d4, d7
pub fn i_05bf00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF02;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF02: move.l d7, (a4)
pub fn i_05bf02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF04;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF04: add.l d4, d7
pub fn i_05bf04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF06;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF06: move.l d7, (a4)
pub fn i_05bf06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF08;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF08: add.l d4, d7
pub fn i_05bf08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF0A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF0A: move.l d7, (a4)
pub fn i_05bf0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF0C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF0C: add.l d4, d7
pub fn i_05bf0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF0E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF0E: move.l d7, (a4)
pub fn i_05bf0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF10;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF10: add.l d4, d7
pub fn i_05bf10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF12;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF12: move.l d7, (a4)
pub fn i_05bf12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF14;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF14: add.l d4, d7
pub fn i_05bf14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF16;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF16: move.l d7, (a4)
pub fn i_05bf16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF18;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF18: add.l d4, d7
pub fn i_05bf18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF1A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF1A: move.l d7, (a4)
pub fn i_05bf1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF1C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF1C: add.l d4, d7
pub fn i_05bf1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF1E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF1E: move.l d7, (a4)
pub fn i_05bf1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF20;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF20: add.l d4, d7
pub fn i_05bf20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF22;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF22: move.l d7, (a4)
pub fn i_05bf22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF24;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF24: add.l d4, d7
pub fn i_05bf24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF26;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF26: move.l d7, (a4)
pub fn i_05bf26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF28;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF28: add.l d4, d7
pub fn i_05bf28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF2A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF2A: move.l d7, (a4)
pub fn i_05bf2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF2C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF2C: add.l d4, d7
pub fn i_05bf2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF2E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF2E: move.l d7, (a4)
pub fn i_05bf2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF30;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF30: add.l d4, d7
pub fn i_05bf30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF32;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF32: move.l d7, (a4)
pub fn i_05bf32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF34;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF34: add.l d4, d7
pub fn i_05bf34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF36;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF36: move.l d7, (a4)
pub fn i_05bf36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF38;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF38: add.l d4, d7
pub fn i_05bf38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF3A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF3A: move.l d7, (a4)
pub fn i_05bf3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF3C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF3C: add.l d4, d7
pub fn i_05bf3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF3E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF3E: move.l d7, (a4)
pub fn i_05bf3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF40;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF40: add.l d4, d7
pub fn i_05bf40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF42;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF42: move.l d7, (a4)
pub fn i_05bf42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF44;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF44: add.l d4, d7
pub fn i_05bf44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF46;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF46: move.l d7, (a4)
pub fn i_05bf46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF48;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF48: add.l d4, d7
pub fn i_05bf48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF4A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF4A: move.l d7, (a4)
pub fn i_05bf4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF4C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF4C: add.l d4, d7
pub fn i_05bf4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF4E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF4E: move.l d7, (a4)
pub fn i_05bf4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF50;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF50: add.l d4, d7
pub fn i_05bf50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF52;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF52: move.l d7, (a4)
pub fn i_05bf52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF54;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF54: add.l d4, d7
pub fn i_05bf54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF56;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF56: move.w #$8401, d6
pub fn i_05bf56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF5A;
    cpu.ir = 0x3C3C;
    let v1: u32 = 0x8401;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05BF5A: add.w d0, d6
pub fn i_05bf5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF5C;
    cpu.ir = 0xDC40;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05BF5C: swap d6
pub fn i_05bf5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF5E;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05BF5E: move.w $a(a3), d6
pub fn i_05bf5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF62;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05BF62: move.l #$2000000, d5
pub fn i_05bf62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF68;
    cpu.ir = 0x2A3C;
    let v1: u32 = 0x2000000;
    cpu.d[5] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05BF68: move.l d6, (a4)
pub fn i_05bf68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF6A;
    cpu.ir = 0x2886;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF6A: sub.l d5, d6
pub fn i_05bf6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF6C;
    cpu.ir = 0x9C85;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[6] = r;
    4
}
/// $05BF6C: sub.l d5, d6
pub fn i_05bf6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF6E;
    cpu.ir = 0x9C85;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[6] = r;
    4
}
/// $05BF6E: move.w $6(a3), d6
pub fn i_05bf6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF72;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05BF72: move.l d6, (a4)
pub fn i_05bf72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF74;
    cpu.ir = 0x2886;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF74: rts
pub fn i_05bf74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF76;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SCB1_WriteTileColumnTerm_05bf76  ($05BF76, 2656 B, video/sprite_queue_render_05aa96.s)
/// $05BF76: move.w $6(a3), d7
pub fn i_05bf76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF7A;
    cpu.ir = 0x3E2B;
    let ad2 = cpu.a[3].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05BF7A: addq.b #$1, d7
pub fn i_05bf7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF7C;
    cpu.ir = 0x5207;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05BF7C: sne.b d6
pub fn i_05bf7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF7E;
    cpu.ir = 0x56C6;
    let c1: u32 = if cpu.sr.evaluate(Condition::Ne) { 0xFF } else { 0 };
    let v2 = cpu.d[6] & 0xFF;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (c1 & 0xFF);
    4
}
/// $05BF7E: andi.b #$4, d6
pub fn i_05bf7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF82;
    cpu.ir = 0x206;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = (v1 & 0x4) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05BF82: rol.w #$7, d7
pub fn i_05bf82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF84;
    cpu.ir = 0xEF5F;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 7, Size::Word, 3, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    20
}
/// $05BF84: andi.w #$8, d7
pub fn i_05bf84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF88;
    cpu.ir = 0x247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = (v1 & 0x8) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05BF88: or.b d6, d7
pub fn i_05bf88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF8A;
    cpu.ir = 0x8E06;
    let v1 = cpu.d[6] & 0xFF;
    let r = (v1 | cpu.d[7]) & 0xFF;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05BF8A: jmp $5bf8e(pc, d7.w)
pub fn i_05bf8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF8E;
    cpu.ir = 0x4EFB;
    let ad1 = 0x5BF8E_u32.wrapping_add((cpu.d[7] as i16 as i32 as u32));
    cpu.jump_to(ad1);
    8
}
// ===== Vblank_FlushSpriteQueue_05c9d6  ($05C9D6, 84 B, video/sprite_queue_render_05aa96.s)
/// $05C9D6: tst.b $10e1ec.l
pub fn i_05c9d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C9DC;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10E1EC) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $05C9DC: beq.w $5ca28
pub fn i_05c9dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C9E0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5CA28; return 10; }
    8
}
/// $05C9E0: clr.b $10e1ec.l
pub fn i_05c9e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C9E6;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E1EC) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E1EC, r2 as u8);
    16
}
/// $05C9E6: lea.l $3c0000.l, a4
pub fn i_05c9e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C9EC;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0x3C0000;
    20
}
/// $05C9EC: lea.l $108080.l, a5
pub fn i_05c9ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C9F2;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x108080;
    20
}
/// $05C9F2: lea.l $614c(a5), a0
pub fn i_05c9f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C9F6;
    cpu.ir = 0x41ED;
    let ad1 = cpu.a[5].wrapping_add(0x614C);
    cpu.a[0] = ad1;
    16
}
/// $05C9F6: lea.l $542a(a5), a1
pub fn i_05c9f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C9FA;
    cpu.ir = 0x43ED;
    let ad1 = cpu.a[5].wrapping_add(0x542A);
    cpu.a[1] = ad1;
    16
}
/// $05C9FA: move.w $6148(a5), d7
pub fn i_05c9fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C9FE;
    cpu.ir = 0x3E2D;
    let ad2 = cpu.a[5].wrapping_add(0x6148);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05C9FE: movea.l a1, a2
pub fn i_05c9fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA00;
    cpu.ir = 0x2449;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.a[2] = v1;
    4
}
/// $05CA00: adda.w d7, a2
pub fn i_05ca00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA02;
    cpu.ir = 0xD4C7;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.a[2] = cpu.a[2].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05CA02: bsr.w $5b400
pub fn i_05ca02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA06;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5CA06);
    cpu.pc = 0x5B400;
    18
}
/// $05CA06: move.w $a(a0), $6168(a5)
pub fn i_05ca06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA0C;
    cpu.ir = 0x3B68;
    let ad2 = cpu.a[0].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0x6168);
    bus.write16(ad4, v3 as u16);
    20
}
/// $05CA0C: lea.l $6158(a5), a0
pub fn i_05ca0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA10;
    cpu.ir = 0x41ED;
    let ad1 = cpu.a[5].wrapping_add(0x6158);
    cpu.a[0] = ad1;
    16
}
/// $05CA10: lea.l $542a(a5), a1
pub fn i_05ca10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA14;
    cpu.ir = 0x43ED;
    let ad1 = cpu.a[5].wrapping_add(0x542A);
    cpu.a[1] = ad1;
    16
}
/// $05CA14: lea.l $348(a1), a2
pub fn i_05ca14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA18;
    cpu.ir = 0x45E9;
    let ad1 = cpu.a[1].wrapping_add(0x348);
    cpu.a[2] = ad1;
    16
}
/// $05CA18: move.w $614a(a5), d7
pub fn i_05ca18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA1C;
    cpu.ir = 0x3E2D;
    let ad2 = cpu.a[5].wrapping_add(0x614A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05CA1C: adda.w d7, a1
pub fn i_05ca1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA1E;
    cpu.ir = 0xD2C7;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.a[1] = cpu.a[1].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05CA1E: bsr.w $5b400
pub fn i_05ca1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA22;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5CA22);
    cpu.pc = 0x5B400;
    18
}
/// $05CA22: move.w $a(a0), $616a(a5)
pub fn i_05ca22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA28;
    cpu.ir = 0x3B68;
    let ad2 = cpu.a[0].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0x616A);
    bus.write16(ad4, v3 as u16);
    20
}
/// $05CA28: rts
pub fn i_05ca28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA2A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sprite_Dispatch_05CA2A  ($05CA2A, 164 B, video/sprite_setup_dispatch_05ca2a.s)
/// $05CA2A: move.b $3a(a6), d5
pub fn i_05ca2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA2E;
    cpu.ir = 0x1A2E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05CA2E: andi.b #$3, d5
pub fn i_05ca2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA32;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05CA32: move.l $22(a6), d0
pub fn i_05ca32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA36;
    cpu.ir = 0x202E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $05CA36: move.w $38(a6), d1
pub fn i_05ca36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA3A;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05CA3A: move.w $14(a6), d2
pub fn i_05ca3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA3E;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05CA3E: move.b $33(a6), d3
pub fn i_05ca3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA42;
    cpu.ir = 0x162E;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05CA42: move.b $32(a6), d4
pub fn i_05ca42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA46;
    cpu.ir = 0x182E;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05CA46: movea.l $3c(a6), a0
pub fn i_05ca46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA4A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $05CA4A: bra.w $5a9e6
pub fn i_05ca4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA4E;
    cpu.ir = 0x6000;
    cpu.pc = 0x5A9E6;
    10
}
// ===== Camera_ResetCenter_05CACE  ($05CACE, 18 B, video/camera_reset_center_05cace.s)
/// $05CACE: move.w #$a0, $10e1e4.l
pub fn i_05cace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CAD6;
    cpu.ir = 0x33FC;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10E1E4, v1 as u16);
    16
}
/// $05CAD6: move.w #$178, $10e1e6.l
pub fn i_05cad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CADE;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x178;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10E1E6, v1 as u16);
    16
}
/// $05CADE: rts
pub fn i_05cade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CAE0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Input_RisingEdgeSnapshot_05CC0E  ($05CC0E, 186 B, input/rising_edge_snapshot_05cc0e.s)
/// $05CC0E: clr.w $10e20c.l
pub fn i_05cc0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC14;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x10E20C) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x10E20C, r2 as u16);
    16
}
/// $05CC14: jmp $24fb8.l
pub fn i_05cc14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC1A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x24FB8;
    8
}
/// $05CC1A: jsr $25066.l
pub fn i_05cc1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC20;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5CC20);
    cpu.pc = 0x25066;
    16
}
/// $05CC20: lea.l $106eb0.l, a0
pub fn i_05cc20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC26;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x106EB0;
    20
}
/// $05CC26: lea.l $10e200.l, a1
pub fn i_05cc26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC2C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x10E200;
    20
}
/// $05CC2C: move.l (a0)+, (a1)+
pub fn i_05cc2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC2E;
    cpu.ir = 0x22D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $05CC2E: move.w (a0), (a1)
pub fn i_05cc2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC30;
    cpu.ir = 0x3290;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v2 as u16);
    12
}
/// $05CC30: lea.l $106eb6.l, a0
pub fn i_05cc30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC36;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x106EB6;
    20
}
/// $05CC36: lea.l $10e206.l, a1
pub fn i_05cc36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC3C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x10E206;
    20
}
/// $05CC3C: move.l (a0)+, (a1)+
pub fn i_05cc3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC3E;
    cpu.ir = 0x22D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $05CC3E: move.w (a0), (a1)
pub fn i_05cc3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC40;
    cpu.ir = 0x3290;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v2 as u16);
    12
}
/// $05CC40: lea.l $10fdac.l, a0
pub fn i_05cc40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC46;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10FDAC;
    20
}
/// $05CC46: lea.l $10e20c.l, a1
pub fn i_05cc46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC4C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x10E20C;
    20
}
/// $05CC4C: move.b (a1), d1
pub fn i_05cc4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC4E;
    cpu.ir = 0x1211;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $05CC4E: move.b (a0), d0
pub fn i_05cc4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC50;
    cpu.ir = 0x1010;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $05CC50: move.b d0, (a1)
pub fn i_05cc50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC52;
    cpu.ir = 0x1280;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[1], v1 as u8);
    8
}
/// $05CC52: eor.b d0, d1
pub fn i_05cc52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC54;
    cpu.ir = 0xB101;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 ^ cpu.d[0]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05CC54: and.b d0, d1
pub fn i_05cc54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC56;
    cpu.ir = 0xC200;
    let v1 = cpu.d[0] & 0xFF;
    let r = (v1 & cpu.d[1]) & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05CC56: move.b d1, $1(a1)
pub fn i_05cc56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC5A;
    cpu.ir = 0x1341;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x1);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05CC5A: lea.l $10fdb6.l, a0
pub fn i_05cc5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC60;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10FDB6;
    20
}
/// $05CC60: cmpi.b #$0, (a0)
pub fn i_05cc60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC64;
    cpu.ir = 0xC10;
    let v2 = bus.read8(cpu.a[0]) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    12
}
/// $05CC64: beq.w $5cc8e
pub fn i_05cc64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC68;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5CC8E; return 10; }
    8
}
/// $05CC68: cmpi.b #$3, (a0)
pub fn i_05cc68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC6C;
    cpu.ir = 0xC10;
    let v2 = bus.read8(cpu.a[0]) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    12
}
/// $05CC6C: beq.w $5cc84
pub fn i_05cc6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC70;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5CC84; return 10; }
    8
}
/// $05CC70: lea.l $10e200.l, a2
pub fn i_05cc70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC76;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E200;
    20
}
/// $05CC76: lea.l $10e212.l, a1
pub fn i_05cc76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC7C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x10E212;
    20
}
/// $05CC7C: move.l (a2)+, (a1)+
pub fn i_05cc7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC7E;
    cpu.ir = 0x22DA;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $05CC7E: move.w (a2)+, (a1)+
pub fn i_05cc7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC80;
    cpu.ir = 0x32DA;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $05CC80: bra.w $5cc8e
pub fn i_05cc80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC84;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CC8E;
    10
}
/// $05CC84: lea.l $10e212.l, a1
pub fn i_05cc84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC8A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x10E212;
    20
}
/// $05CC8A: clr.l (a1)+
pub fn i_05cc8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC8C;
    cpu.ir = 0x4299;
    let ad1 = cpu.a[1]; cpu.a[1] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $05CC8C: clr.w (a1)+
pub fn i_05cc8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC8E;
    cpu.ir = 0x4259;
    let ad1 = cpu.a[1]; cpu.a[1] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $05CC8E: lea.l $10fdb6.l, a0
pub fn i_05cc8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC94;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10FDB6;
    20
}
/// $05CC94: cmpi.b #$0, $1(a0)
pub fn i_05cc94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC9A;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $05CC9A: beq.w $5ccc6
pub fn i_05cc9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC9E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5CCC6; return 10; }
    8
}
/// $05CC9E: cmpi.b #$3, $1(a0)
pub fn i_05cc9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CCA4;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $05CCA4: beq.w $5ccbc
pub fn i_05cca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CCA8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5CCBC; return 10; }
    8
}
/// $05CCA8: lea.l $10e206.l, a2
pub fn i_05cca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CCAE;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E206;
    20
}
/// $05CCAE: lea.l $10e218.l, a1
pub fn i_05ccae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CCB4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x10E218;
    20
}
/// $05CCB4: move.l (a2)+, (a1)+
pub fn i_05ccb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CCB6;
    cpu.ir = 0x22DA;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $05CCB6: move.w (a2)+, (a1)+
pub fn i_05ccb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CCB8;
    cpu.ir = 0x32DA;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $05CCB8: bra.w $5ccc6
pub fn i_05ccb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CCBC;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CCC6;
    10
}
/// $05CCBC: lea.l $10e218.l, a1
pub fn i_05ccbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CCC2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x10E218;
    20
}
/// $05CCC2: clr.l (a1)+
pub fn i_05ccc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CCC4;
    cpu.ir = 0x4299;
    let ad1 = cpu.a[1]; cpu.a[1] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $05CCC4: clr.w (a1)+
pub fn i_05ccc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CCC6;
    cpu.ir = 0x4259;
    let ad1 = cpu.a[1]; cpu.a[1] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $05CCC6: rts
pub fn i_05ccc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CCC8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CopyField6D_05ccc8  ($05CCC8, 8 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05CCC8: move.b $6d(a6), $6d(a0)
pub fn i_05ccc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CCCE;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x6D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x6D);
    bus.write8(ad4, v3 as u8);
    20
}
/// $05CCCE: rts
pub fn i_05ccce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CCD0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== InputEvtThunk_05cd18  ($05CD18, 18 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05CD18: move.b #$10, d1
pub fn i_05cd18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD1C;
    cpu.ir = 0x123C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CD1C: move.w #$2, d0
pub fn i_05cd1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD20;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CD20: lea.l $10e206.l, a2
pub fn i_05cd20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD26;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E206;
    20
}
/// $05CD26: bra.w $5cff8
pub fn i_05cd26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD2A;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFF8;
    10
}
// ===== InputEvtThunk_05cd6c  ($05CD6C, 12 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05CD6C: move.b #$20, d1
pub fn i_05cd6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD70;
    cpu.ir = 0x123C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CD70: move.w #$2, d0
pub fn i_05cd70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD74;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CD74: bra.w $5cfa8
pub fn i_05cd74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD78;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
    10
}
// ===== InputEvtThunk_05cda8  ($05CDA8, 12 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05CDA8: move.b #$10, d1
pub fn i_05cda8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CDAC;
    cpu.ir = 0x123C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CDAC: move.w #$3, d0
pub fn i_05cdac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CDB0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CDB0: bra.w $5cfa8
pub fn i_05cdb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CDB4;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
    10
}
// ===== InputEvtThunk_05cdb4  ($05CDB4, 12 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05CDB4: move.b #$20, d1
pub fn i_05cdb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CDB8;
    cpu.ir = 0x123C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CDB8: move.w #$3, d0
pub fn i_05cdb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CDBC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CDBC: bra.w $5cfa8
pub fn i_05cdbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CDC0;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
    10
}
// ===== InputEvtThunk_05cdc0  ($05CDC0, 12 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05CDC0: move.b #$40, d1
pub fn i_05cdc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CDC4;
    cpu.ir = 0x123C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CDC4: move.w #$3, d0
pub fn i_05cdc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CDC8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CDC8: bra.w $5cfa8
pub fn i_05cdc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CDCC;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
    10
}
// ===== InputEvtThunk_05cde4  ($05CDE4, 12 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05CDE4: move.b #$50, d1
pub fn i_05cde4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CDE8;
    cpu.ir = 0x123C;
    let v1: u32 = 0x50;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CDE8: move.w #$3, d0
pub fn i_05cde8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CDEC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CDEC: bra.w $5cfa8
pub fn i_05cdec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CDF0;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
    10
}
// ===== InputEvtThunk_05cdfc  ($05CDFC, 12 B, input/event_thunks_05cdfc.s)
/// $05CDFC: move.b #$f0, d1
pub fn i_05cdfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE00;
    cpu.ir = 0x123C;
    let v1: u32 = 0xF0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CE00: move.w #$3, d0
pub fn i_05ce00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE04;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CE04: bra.w $5cfa8
pub fn i_05ce04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE08;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
    10
}
// ===== InputEvtThunk_05ceec  ($05CEEC, 12 B, input/event_thunks_05cdfc.s)
/// $05CEEC: move.b #$1, d1
pub fn i_05ceec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CEF0;
    cpu.ir = 0x123C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CEF0: move.w #$2, d0
pub fn i_05cef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CEF4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CEF4: bra.w $5cfa8
pub fn i_05cef4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CEF8;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
    10
}
// ===== InputEvtThunk_05cef8  ($05CEF8, 12 B, input/event_thunks_05cdfc.s)
/// $05CEF8: move.b #$2, d1
pub fn i_05cef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CEFC;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CEFC: move.w #$2, d0
pub fn i_05cefc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF00;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CF00: bra.w $5cfa8
pub fn i_05cf00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF04;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
    10
}
// ===== InputEvtThunk_05cf04  ($05CF04, 12 B, input/event_thunks_05cdfc.s)
/// $05CF04: move.b #$8, d1
pub fn i_05cf04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF08;
    cpu.ir = 0x123C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CF08: move.w #$2, d0
pub fn i_05cf08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF0C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CF0C: bra.w $5cfa8
pub fn i_05cf0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF10;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
    10
}
// ===== InputEvtThunk_05cf10  ($05CF10, 12 B, input/event_thunks_05cdfc.s)
/// $05CF10: move.b #$4, d1
pub fn i_05cf10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF14;
    cpu.ir = 0x123C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CF14: move.w #$2, d0
pub fn i_05cf14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF18;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CF18: bra.w $5cfa8
pub fn i_05cf18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF1C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
    10
}
// ===== InputEvtThunk_05cf1c  ($05CF1C, 16 B, input/event_thunks_05cdfc.s)
/// $05CF1C: move.b #$4, d1
pub fn i_05cf1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF20;
    cpu.ir = 0x123C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CF20: ori.b #$8, d1
pub fn i_05cf20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF24;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x8) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05CF24: move.w #$2, d0
pub fn i_05cf24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF28;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CF28: bra.w $5cfa8
pub fn i_05cf28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF2C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
    10
}
// ===== InputMask_CheckChannelAvail_05cfa8  ($05CFA8, 32 B, input/check_channel_05cfa8.s)
/// $05CFA8: btst.b #$7, $100000.l
pub fn i_05cfa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CFB0;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((7) % 8);
    let v2 = bus.read8(0x100000) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $05CFB0: bne.w $5cfc8
pub fn i_05cfb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CFB4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5CFC8; return 10; }
    8
}
/// $05CFB4: lea.l $1001c0.l, a4
pub fn i_05cfb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CFBA;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0x1001C0;
    20
}
/// $05CFBA: tst.b $44(a4)
pub fn i_05cfba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CFBE;
    cpu.ir = 0x4A2C;
    let ad2 = cpu.a[4].wrapping_add(0x44);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $05CFBE: beq.w $5cfc8
pub fn i_05cfbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CFC2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5CFC8; return 10; }
    8
}
/// $05CFC2: andi.b #$ee, ccr
pub fn i_05cfc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CFC6;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05CFC6: rts
pub fn i_05cfc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CFC8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== InputMask_ReadCtxSwitchPlayer_05cfc8  ($05CFC8, 48 B, input/ctx_switch_05cfc8.s)
/// $05CFC8: lea.l $5cc08(pc), a2
pub fn i_05cfc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CFCC;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0x5CC08;
    16
}
/// $05CFCC: cmpi.b #$1, $6d(a6)
pub fn i_05cfcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CFD2;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x6D);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $05CFD2: bne.w $5cfe0
pub fn i_05cfd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CFD6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5CFE0; return 10; }
    8
}
/// $05CFD6: lea.l $100300.l, a1
pub fn i_05cfd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CFDC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x100300;
    20
}
/// $05CFDC: movea.l $72(a1), a2
pub fn i_05cfdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CFE0;
    cpu.ir = 0x2469;
    let ad2 = cpu.a[1].wrapping_add(0x72);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $05CFE0: cmpi.b #$2, $6d(a6)
pub fn i_05cfe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CFE6;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x6D);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $05CFE6: bne.w $5cff4
pub fn i_05cfe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CFEA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5CFF4; return 10; }
    8
}
/// $05CFEA: lea.l $1003a0.l, a1
pub fn i_05cfea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CFF0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1003A0;
    20
}
/// $05CFF0: movea.l $72(a1), a2
pub fn i_05cff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CFF4;
    cpu.ir = 0x2469;
    let ad2 = cpu.a[1].wrapping_add(0x72);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $05CFF4: jsr $5d674(pc)
pub fn i_05cff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CFF8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5CFF8);
    cpu.pc = 0x5D674;
    16
}
// ===== InputMask_TestChannelBit_05cff8  ($05CFF8, 22 B, input/test_bit_05cff8.s)
/// $05CFF8: move.b (a2, d0.w), d0
pub fn i_05cff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CFFC;
    cpu.ir = 0x1032;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $05CFFC: and.b d1, d0
pub fn i_05cffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CFFE;
    cpu.ir = 0xC001;
    let v1 = cpu.d[1] & 0xFF;
    let r = (v1 & cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05CFFE: bne.w $5d008
pub fn i_05cffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D002;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5D008; return 10; }
    8
}
/// $05D002: andi.b #$ee, ccr
pub fn i_05d002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D006;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05D006: rts
pub fn i_05d006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D008;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05D008: ori.b #$11, ccr
pub fn i_05d008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D00C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05D00C: rts
pub fn i_05d00c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D00E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== InputEvtThunk_05d00e  ($05D00E, 12 B, input/event_thunks_05d00e.s)
/// $05D00E: move.b #$30, d1
pub fn i_05d00e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D012;
    cpu.ir = 0x123C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05D012: move.w #$3, d0
pub fn i_05d012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D016;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D016: bra.w $5d032
pub fn i_05d016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D01A;
    cpu.ir = 0x6000;
    cpu.pc = 0x5D032;
    10
}
// ===== InputMask_CheckChannelAvail_05d032  ($05D032, 32 B, input/check_channel_05d032.s)
/// $05D032: btst.b #$7, $100000.l
pub fn i_05d032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D03A;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((7) % 8);
    let v2 = bus.read8(0x100000) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $05D03A: bne.w $5d052
pub fn i_05d03a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D03E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5D052; return 10; }
    8
}
/// $05D03E: lea.l $1001c0.l, a4
pub fn i_05d03e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D044;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0x1001C0;
    20
}
/// $05D044: tst.b $44(a4)
pub fn i_05d044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D048;
    cpu.ir = 0x4A2C;
    let ad2 = cpu.a[4].wrapping_add(0x44);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $05D048: beq.w $5d052
pub fn i_05d048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D04C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5D052; return 10; }
    8
}
/// $05D04C: andi.b #$ee, ccr
pub fn i_05d04c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D050;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05D050: rts
pub fn i_05d050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D052;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== InputMask_ReadCtxSwitchPlayer_05d052  ($05D052, 48 B, input/ctx_switch_05d052.s)
/// $05D052: lea.l $5cc08(pc), a2
pub fn i_05d052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D056;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0x5CC08;
    16
}
/// $05D056: cmpi.b #$1, $6d(a6)
pub fn i_05d056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D05C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x6D);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $05D05C: bne.w $5d06a
pub fn i_05d05c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D060;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5D06A; return 10; }
    8
}
/// $05D060: lea.l $100300.l, a1
pub fn i_05d060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D066;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x100300;
    20
}
/// $05D066: movea.l $72(a1), a2
pub fn i_05d066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D06A;
    cpu.ir = 0x2469;
    let ad2 = cpu.a[1].wrapping_add(0x72);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $05D06A: cmpi.b #$2, $6d(a6)
pub fn i_05d06a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D070;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x6D);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $05D070: bne.w $5d07e
pub fn i_05d070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D074;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5D07E; return 10; }
    8
}
/// $05D074: lea.l $1003a0.l, a1
pub fn i_05d074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D07A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1003A0;
    20
}
/// $05D07A: movea.l $72(a1), a2
pub fn i_05d07a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D07E;
    cpu.ir = 0x2469;
    let ad2 = cpu.a[1].wrapping_add(0x72);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $05D07E: jsr $5d674(pc)
pub fn i_05d07e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D082;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5D082);
    cpu.pc = 0x5D674;
    16
}
// ===== InputMask_TestChannelAllBits_05d082  ($05D082, 24 B, input/test_all_bits_05d082.s)
/// $05D082: move.b (a2, d0.w), d0
pub fn i_05d082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D086;
    cpu.ir = 0x1032;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $05D086: and.b d1, d0
pub fn i_05d086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D088;
    cpu.ir = 0xC001;
    let v1 = cpu.d[1] & 0xFF;
    let r = (v1 & cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05D088: cmp.b d1, d0
pub fn i_05d088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D08A;
    cpu.ir = 0xB001;
    let v1 = cpu.d[1] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $05D08A: beq.w $5d094
pub fn i_05d08a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D08E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5D094; return 10; }
    8
}
/// $05D08E: andi.b #$ee, ccr
pub fn i_05d08e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D092;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05D092: rts
pub fn i_05d092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D094;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05D094: ori.b #$11, ccr
pub fn i_05d094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D098;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05D098: rts
pub fn i_05d098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D09A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== InputEvtThunk_05d09a  ($05D09A, 18 B, input/event_thunks_05d09a.s)
/// $05D09A: move.b #$f0, d1
pub fn i_05d09a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D09E;
    cpu.ir = 0x123C;
    let v1: u32 = 0xF0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05D09E: move.w #$3, d0
pub fn i_05d09e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D0A2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D0A2: lea.l $10e200.l, a2
pub fn i_05d0a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D0A8;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E200;
    20
}
/// $05D0A8: bra.w $5d0e2
pub fn i_05d0a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D0AC;
    cpu.ir = 0x6000;
    cpu.pc = 0x5D0E2;
    10
}
// ===== InputEvtThunk_05d0ac  ($05D0AC, 18 B, input/event_thunks_05d09a.s)
/// $05D0AC: move.b #$f0, d1
pub fn i_05d0ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D0B0;
    cpu.ir = 0x123C;
    let v1: u32 = 0xF0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05D0B0: move.w #$3, d0
pub fn i_05d0b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D0B4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D0B4: lea.l $10e206.l, a2
pub fn i_05d0b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D0BA;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E206;
    20
}
/// $05D0BA: bra.w $5d0e2
pub fn i_05d0ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D0BE;
    cpu.ir = 0x6000;
    cpu.pc = 0x5D0E2;
    10
}
// ===== InputMask_TestChannelBit_05d0e2  ($05D0E2, 22 B, input/test_bit_05d0e2.s)
/// $05D0E2: move.b (a2, d0.w), d0
pub fn i_05d0e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D0E6;
    cpu.ir = 0x1032;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $05D0E6: and.b d1, d0
pub fn i_05d0e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D0E8;
    cpu.ir = 0xC001;
    let v1 = cpu.d[1] & 0xFF;
    let r = (v1 & cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05D0E8: bne.w $5d0f2
pub fn i_05d0e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D0EC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5D0F2; return 10; }
    8
}
/// $05D0EC: andi.b #$ee, ccr
pub fn i_05d0ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D0F0;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05D0F0: rts
pub fn i_05d0f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D0F2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05D0F2: ori.b #$11, ccr
pub fn i_05d0f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D0F6;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05D0F6: rts
pub fn i_05d0f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D0F8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== InputEvtThunk_05d11c  ($05D11C, 18 B, input/event_thunks_05d09a.s)
/// $05D11C: move.b #$8, d1
pub fn i_05d11c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D120;
    cpu.ir = 0x123C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05D120: move.w #$2, d0
pub fn i_05d120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D124;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D124: lea.l $10e206.l, a2
pub fn i_05d124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D12A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E206;
    20
}
/// $05D12A: bra.w $5d1c0
pub fn i_05d12a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D12E;
    cpu.ir = 0x6000;
    cpu.pc = 0x5D1C0;
    10
}
// ===== InputMask_TestChannelNibbleXor_05d1c0  ($05D1C0, 26 B, input/test_nibble_xor_05d1c0.s)
/// $05D1C0: move.b (a2, d0.w), d0
pub fn i_05d1c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D1C4;
    cpu.ir = 0x1032;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $05D1C4: andi.b #$f, d0
pub fn i_05d1c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D1C8;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05D1C8: eor.b d1, d0
pub fn i_05d1c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D1CA;
    cpu.ir = 0xB300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 ^ cpu.d[1]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05D1CA: beq.w $5d1d4
pub fn i_05d1ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D1CE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5D1D4; return 10; }
    8
}
/// $05D1CE: andi.b #$ee, ccr
pub fn i_05d1ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D1D2;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05D1D2: rts
pub fn i_05d1d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D1D4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05D1D4: ori.b #$11, ccr
pub fn i_05d1d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D1D8;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05D1D8: rts
pub fn i_05d1d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D1DA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== InputEvt_ToggleChain_05d288  ($05D288, 122 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05D288: move.b #$1, d1
pub fn i_05d288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D28C;
    cpu.ir = 0x123C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05D28C: move.w #$2, d0
pub fn i_05d28c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D290;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D290: lea.l $10e200.l, a2
pub fn i_05d290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D296;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E200;
    20
}
/// $05D296: jsr $5cff8(pc)
pub fn i_05d296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D29A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5D29A);
    cpu.pc = 0x5CFF8;
    16
}
/// $05D29A: bcs.w $5d2a2
pub fn i_05d29a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D29E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5D2A2; return 10; }
    8
}
/// $05D29E: bra.w $5d2ee
pub fn i_05d29e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D2A2;
    cpu.ir = 0x6000;
    cpu.pc = 0x5D2EE;
    10
}
/// $05D2A2: move.b #$80, d1
pub fn i_05d2a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D2A6;
    cpu.ir = 0x123C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05D2A6: move.w #$3, d0
pub fn i_05d2a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D2AA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D2AA: lea.l $10e200.l, a2
pub fn i_05d2aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D2B0;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E200;
    20
}
/// $05D2B0: bra.w $5cff8
pub fn i_05d2b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D2B4;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFF8;
    10
}
/// $05D2EE: move.b #$1, d0
pub fn i_05d2ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D2F2;
    cpu.ir = 0x103C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05D2F2: subi.b #$1, d0
pub fn i_05d2f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D2F6;
    cpu.ir = 0x400;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05D2F6: rts
pub fn i_05d2f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D2F8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== InputLayout_ReadField2_05D5B6  ($05D5B6, 96 B, input/aim_tables_05d316.s)
/// $05D5B6: move.w #$2, d1
pub fn i_05d5b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D5BA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D5BA: lea.l $5cc08(pc), a2
pub fn i_05d5ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D5BE;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0x5CC08;
    16
}
/// $05D5BE: cmpi.b #$1, $6d(a6)
pub fn i_05d5be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D5C4;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x6D);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $05D5C4: bne.w $5d5d2
pub fn i_05d5c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D5C8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5D5D2; return 10; }
    8
}
/// $05D5C8: lea.l $100300.l, a1
pub fn i_05d5c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D5CE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x100300;
    20
}
/// $05D5CE: movea.l $72(a1), a2
pub fn i_05d5ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D5D2;
    cpu.ir = 0x2469;
    let ad2 = cpu.a[1].wrapping_add(0x72);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $05D5D2: cmpi.b #$2, $6d(a6)
pub fn i_05d5d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D5D8;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x6D);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $05D5D8: bne.w $5d5e6
pub fn i_05d5d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D5DC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5D5E6; return 10; }
    8
}
/// $05D5DC: lea.l $1003a0.l, a1
pub fn i_05d5dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D5E2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1003A0;
    20
}
/// $05D5E2: movea.l $72(a1), a2
pub fn i_05d5e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D5E6;
    cpu.ir = 0x2469;
    let ad2 = cpu.a[1].wrapping_add(0x72);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $05D5E6: moveq #$0, d0
pub fn i_05d5e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D5E8;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D5E8: btst.b #$7, $100000.l
pub fn i_05d5e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D5F0;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((7) % 8);
    let v2 = bus.read8(0x100000) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $05D5F0: bne.w $5d606
pub fn i_05d5f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D5F4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5D606; return 10; }
    8
}
/// $05D5F4: lea.l $1001c0.l, a4
pub fn i_05d5f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D5FA;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0x1001C0;
    20
}
/// $05D5FA: tst.b $44(a4)
pub fn i_05d5fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D5FE;
    cpu.ir = 0x4A2C;
    let ad2 = cpu.a[4].wrapping_add(0x44);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $05D5FE: beq.w $5d606
pub fn i_05d5fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D602;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5D606; return 10; }
    8
}
/// $05D602: nop
pub fn i_05d602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D604;
    cpu.ir = 0x4E71;
    4
}
/// $05D604: rts
pub fn i_05d604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D606;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05D606: jsr $5d674(pc)
pub fn i_05d606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D60A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5D60A);
    cpu.pc = 0x5D674;
    16
}
/// $05D60A: move.b (a2, d1.w), d0
pub fn i_05d60a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D60E;
    cpu.ir = 0x1032;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $05D60E: andi.b #$c, d0
pub fn i_05d60e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D612;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xC) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05D612: asr.b #$2, d0
pub fn i_05d612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D614;
    cpu.ir = 0xE400;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Byte, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    10
}
/// $05D614: rts
pub fn i_05d614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D616;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== InputCtx_DemoOverride_05D674  ($05D674, 44 B, input/aim_tables_05d316.s)
/// $05D674: movem.l d0, -(a7)
pub fn i_05d674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D678;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $05D678: tst.b $10e39d.l
pub fn i_05d678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D67E;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10E39D) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $05D67E: beq.w $5d69a
pub fn i_05d67e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D682;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5D69A; return 10; }
    8
}
/// $05D682: move.l a2, d0
pub fn i_05d682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D684;
    cpu.ir = 0x200A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D684: lea.l $10e2e2.l, a2
pub fn i_05d684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D68A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E2E2;
    20
}
/// $05D68A: cmpi.l #$10e200, d0
pub fn i_05d68a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D690;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0x10E200, Size::Long, false);
    8
}
/// $05D690: beq.w $5d69a
pub fn i_05d690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D694;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5D69A; return 10; }
    8
}
/// $05D694: lea.l $10e2e8.l, a2
pub fn i_05d694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D69A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E2E8;
    20
}
/// $05D69A: movem.l (a7)+, d0
pub fn i_05d69a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D69E;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $05D69E: rts
pub fn i_05d69e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6A0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sprite_HexFormat4_05D6C2  ($05D6C2, 90 B, video/sprite_hex_format4_05d6c2.s)
/// $05D6C2: lea.l $10e21e.l, a2
pub fn i_05d6c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6C8;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E21E;
    20
}
/// $05D6C8: moveq #$c, d2
pub fn i_05d6c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6CA;
    cpu.ir = 0x740C;
    cpu.d[2] = 0xC;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D6CA: move.w #$4, d3
pub fn i_05d6ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6CE;
    cpu.ir = 0x363C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D6CE: move.w d3, -(a7)
pub fn i_05d6ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6D0;
    cpu.ir = 0x3F03;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $05D6D0: move.w #$0, d1
pub fn i_05d6d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6D4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D6D4: move.l d2, -(a7)
pub fn i_05d6d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6D6;
    cpu.ir = 0x2F02;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $05D6D6: move.l d0, d3
pub fn i_05d6d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6D8;
    cpu.ir = 0x2600;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[3] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D6D8: lsr.l d2, d3
pub fn i_05d6d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6DA;
    cpu.ir = 0xE4AB;
    let cnt = cpu.d[2] & 0x3F;
    let h1 = cpu.d[3] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, cnt, Size::Long, 1, false);
    cpu.d[3] = r;
    6 + 2 * cnt
}
/// $05D6DA: andi.l #$f, d3
pub fn i_05d6da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6E0;
    cpu.ir = 0x283;
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    let r2 = (v1 & 0xF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[3] = r2;
    8
}
/// $05D6E0: move.w d3, (a2)
pub fn i_05d6e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6E2;
    cpu.ir = 0x3483;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[2], v1 as u16);
    8
}
/// $05D6E2: addq.l #$2, a2
pub fn i_05d6e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6E4;
    cpu.ir = 0x548A;
    cpu.a[2] = cpu.a[2].wrapping_add(2);
    8
}
/// $05D6E4: move.l (a7)+, d2
pub fn i_05d6e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6E6;
    cpu.ir = 0x241F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05D6E6: subq.l #$4, d2
pub fn i_05d6e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6E8;
    cpu.ir = 0x5982;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let r2 = sub_with_flags(cpu, v1, 4, Size::Long, true);
    cpu.d[2] = r2;
    4
}
/// $05D6E8: bne.b $5d6d0
pub fn i_05d6e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6EA;
    cpu.ir = 0x66E6;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5D6D0; return 10; }
    8
}
/// $05D6EA: andi.l #$f, d0
pub fn i_05d6ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6F0;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $05D6F0: move.w d0, (a2)
pub fn i_05d6f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6F2;
    cpu.ir = 0x3480;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[2], v1 as u16);
    8
}
/// $05D6F2: move.w (a7)+, d3
pub fn i_05d6f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6F4;
    cpu.ir = 0x361F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05D6F4: subq.w #$1, d3
pub fn i_05d6f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6F6;
    cpu.ir = 0x5343;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05D6F6: lea.l $10e21e.l, a3
pub fn i_05d6f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6FC;
    cpu.ir = 0x47F9;
    cpu.a[3] = 0x10E21E;
    20
}
/// $05D6FC: move.l a1, d0
pub fn i_05d6fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6FE;
    cpu.ir = 0x2009;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D6FE: move.w (a3)+, d4
pub fn i_05d6fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D700;
    cpu.ir = 0x381B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05D700: move.b $5d71c(pc, d4.w), d2
pub fn i_05d700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D704;
    cpu.ir = 0x143B;
    let ad2 = 0x5D71C_u32.wrapping_add((cpu.d[4] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    14
}
/// $05D704: andi.w #$ff, d2
pub fn i_05d704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D708;
    cpu.ir = 0x242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05D708: add.w d1, d2
pub fn i_05d708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D70A;
    cpu.ir = 0xD441;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05D70A: movem.w d0/d2, $3c0000.l
pub fn i_05d70a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D712;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[2] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $05D712: addi.w #$20, d0
pub fn i_05d712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D716;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05D716: dbra d3, $5d6fe
pub fn i_05d716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D71A;
    cpu.ir = 0x51CB;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5D71A; return 12; }
    let n = (cpu.d[3] as u16).wrapping_sub(1);
    cpu.d[3] = (cpu.d[3] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5D71A; return 14; }
    cpu.pc = 0x5D6FE;
    10
}
/// $05D71A: rts
pub fn i_05d71a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D71C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Decimal_Clamp99999999_05D8F2  ($05D8F2, 18 B, kernel/math_decimal_clamp_05d8f2.s)
/// $05D8F2: cmpi.l #$5f5e0ff, d0
pub fn i_05d8f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8F8;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0x5F5E0FF, Size::Long, false);
    8
}
/// $05D8F8: bcs.w $5d904
pub fn i_05d8f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8FC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5D904; return 10; }
    8
}
/// $05D8FC: move.l #$99999999, d0
pub fn i_05d8fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D902;
    cpu.ir = 0x203C;
    let v1: u32 = 0x99999999;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D902: rts
pub fn i_05d902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D904;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_BinToDecimalDecoder_05D904  ($05D904, 28 B, kernel/math_bin_to_decimal_05d904.s)
/// $05D904: moveq #$0, d4
pub fn i_05d904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D906;
    cpu.ir = 0x7800;
    cpu.d[4] = 0x0;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D906: moveq #$0, d5
pub fn i_05d906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D908;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D908: tst.l d0
pub fn i_05d908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D90A;
    cpu.ir = 0x4A80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D90A: beq.w $5d91c
pub fn i_05d90a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D90E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5D91C; return 10; }
    8
}
/// $05D90E: moveq #$a, d1
pub fn i_05d90e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D910;
    cpu.ir = 0x720A;
    cpu.d[1] = 0xA;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D910: jsr $5d920(pc)
pub fn i_05d910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D914;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5D914);
    cpu.pc = 0x5D920;
    16
}
/// $05D914: asl.l d5, d2
pub fn i_05d914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D916;
    cpu.ir = 0xEBA2;
    let cnt = cpu.d[5] & 0x3F;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, cnt, Size::Long, 0, true);
    cpu.d[2] = r;
    6 + 2 * cnt
}
/// $05D916: or.l d2, d4
pub fn i_05d916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D918;
    cpu.ir = 0x8882;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let r = (v1 | cpu.d[4]) & 0xFFFFFFFF;
    cpu.d[4] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D918: addq.l #$4, d5
pub fn i_05d918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D91A;
    cpu.ir = 0x5885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Long, true);
    cpu.d[5] = r2;
    4
}
/// $05D91A: bra.b $5d908
pub fn i_05d91a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D91C;
    cpu.ir = 0x60EC;
    cpu.pc = 0x5D908;
    10
}
/// $05D91C: move.l d4, d0
pub fn i_05d91c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D91E;
    cpu.ir = 0x2004;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D91E: rts
pub fn i_05d91e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D920;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_LongDivide_05D920  ($05D920, 36 B, kernel/math_long_divide_05d920.s)
/// $05D920: moveq #$0, d2
pub fn i_05d920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D922;
    cpu.ir = 0x7400;
    cpu.d[2] = 0x0;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D922: tst.l d0
pub fn i_05d922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D924;
    cpu.ir = 0x4A80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D924: beq.w $5d942
pub fn i_05d924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D928;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5D942; return 10; }
    8
}
/// $05D928: tst.l d1
pub fn i_05d928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D92A;
    cpu.ir = 0x4A81;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D92A: beq.w $5d944
pub fn i_05d92a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D92E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5D944; return 10; }
    8
}
/// $05D92E: moveq #$1f, d3
pub fn i_05d92e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D930;
    cpu.ir = 0x761F;
    cpu.d[3] = 0x1F;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D930: add.l d0, d0
pub fn i_05d930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D932;
    cpu.ir = 0xD080;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[0] = r;
    4
}
/// $05D932: addx.l d2, d2
pub fn i_05d932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D934;
    cpu.ir = 0xD582;
    let x = cpu.sr.get(StatusRegister::X) as u32;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let h2 = cpu.d[2] & 0xFFFFFFFF;
    let r = addx_with_flags(cpu, h1, h2, x, Size::Long);
    cpu.d[2] = r;
    8
}
/// $05D934: cmp.l d1, d2
pub fn i_05d934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D936;
    cpu.ir = 0xB481;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    4
}
/// $05D936: bcs.w $5d93e
pub fn i_05d936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D93A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5D93E; return 10; }
    8
}
/// $05D93A: addq.w #$1, d0
pub fn i_05d93a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D93C;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05D93C: sub.l d1, d2
pub fn i_05d93c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D93E;
    cpu.ir = 0x9481;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[2] = r;
    4
}
/// $05D93E: dbra d3, $5d930
pub fn i_05d93e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D942;
    cpu.ir = 0x51CB;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5D942; return 12; }
    let n = (cpu.d[3] as u16).wrapping_sub(1);
    cpu.d[3] = (cpu.d[3] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5D942; return 14; }
    cpu.pc = 0x5D930;
    10
}
/// $05D942: rts
pub fn i_05d942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D944;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Trap15_DivByZero_05D944  ($05D944, 2 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05D944: trap #$f
pub fn i_05d944(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D946;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
// ===== Fix_BlitRectToFixLayer  ($05DA56, 70 B, video/fix_blit_rect_05da56.s)
/// $05DA56: movem.l d3-d5, -(a7)
pub fn i_05da56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DA5A;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.d[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $05DA5A: subq.w #$1, d1
pub fn i_05da5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DA5C;
    cpu.ir = 0x5341;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05DA5C: subq.w #$1, d2
pub fn i_05da5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DA5E;
    cpu.ir = 0x5342;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05DA5E: move.w d0, d4
pub fn i_05da5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DA60;
    cpu.ir = 0x3800;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05DA60: move.l a1, d3
pub fn i_05da60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DA62;
    cpu.ir = 0x2609;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.d[3] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05DA62: move.w d1, d5
pub fn i_05da62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DA64;
    cpu.ir = 0x3A01;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05DA64: move.w d4, -(a7)
pub fn i_05da64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DA66;
    cpu.ir = 0x3F04;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $05DA66: cmpi.w #$74ff, d3
pub fn i_05da66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DA6A;
    cpu.ir = 0xC43;
    let v1 = cpu.d[3] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x74FF, Size::Word, false);
    8
}
/// $05DA6A: bgt.w $5da7e
pub fn i_05da6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DA6E;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x5DA7E; return 10; }
    8
}
/// $05DA6E: cmpi.w #$7000, d3
pub fn i_05da6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DA72;
    cpu.ir = 0xC43;
    let v1 = cpu.d[3] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x7000, Size::Word, false);
    8
}
/// $05DA72: blt.w $5da7e
pub fn i_05da72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DA76;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x5DA7E; return 10; }
    8
}
/// $05DA76: movem.w d3-d4, $3c0000.l
pub fn i_05da76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DA7E;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[3] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[4] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $05DA7E: addq.w #$1, d4
pub fn i_05da7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DA80;
    cpu.ir = 0x5244;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05DA80: addi.w #$20, d3
pub fn i_05da80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DA84;
    cpu.ir = 0x643;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05DA84: dbra d5, $5da66
pub fn i_05da84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DA88;
    cpu.ir = 0x51CD;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5DA88; return 12; }
    let n = (cpu.d[5] as u16).wrapping_sub(1);
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5DA88; return 14; }
    cpu.pc = 0x5DA66;
    10
}
/// $05DA88: move.w (a7)+, d4
pub fn i_05da88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DA8A;
    cpu.ir = 0x381F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05DA8A: addi.w #$10, d4
pub fn i_05da8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DA8E;
    cpu.ir = 0x644;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05DA8E: adda.w #$1, a1
pub fn i_05da8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DA92;
    cpu.ir = 0xD2FC;
    let v1: u32 = 0x1;
    cpu.a[1] = cpu.a[1].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05DA92: dbra d2, $5da60
pub fn i_05da92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DA96;
    cpu.ir = 0x51CA;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5DA96; return 12; }
    let n = (cpu.d[2] as u16).wrapping_sub(1);
    cpu.d[2] = (cpu.d[2] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5DA96; return 14; }
    cpu.pc = 0x5DA60;
    10
}
/// $05DA96: movem.l (a7)+, d3-d5
pub fn i_05da96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DA9A;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $05DA9A: rts
pub fn i_05da9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DA9C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Fix_BlitRect_05DA9C  ($05DA9C, 60 B, video/fix_blit_rect_05da9c.s)
/// $05DA9C: movem.l d3-d5, -(a7)
pub fn i_05da9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DAA0;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.d[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $05DAA0: subq.w #$1, d1
pub fn i_05daa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DAA2;
    cpu.ir = 0x5341;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05DAA2: subq.w #$1, d2
pub fn i_05daa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DAA4;
    cpu.ir = 0x5342;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05DAA4: move.w d0, d4
pub fn i_05daa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DAA6;
    cpu.ir = 0x3800;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05DAA6: move.l a1, d3
pub fn i_05daa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DAA8;
    cpu.ir = 0x2609;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.d[3] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05DAA8: move.w d1, d5
pub fn i_05daa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DAAA;
    cpu.ir = 0x3A01;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05DAAA: cmpi.w #$74ff, d3
pub fn i_05daaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DAAE;
    cpu.ir = 0xC43;
    let v1 = cpu.d[3] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x74FF, Size::Word, false);
    8
}
/// $05DAAE: bgt.w $5dac2
pub fn i_05daae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DAB2;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x5DAC2; return 10; }
    8
}
/// $05DAB2: cmpi.w #$7000, d3
pub fn i_05dab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DAB6;
    cpu.ir = 0xC43;
    let v1 = cpu.d[3] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x7000, Size::Word, false);
    8
}
/// $05DAB6: blt.w $5dac2
pub fn i_05dab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DABA;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x5DAC2; return 10; }
    8
}
/// $05DABA: movem.w d3-d4, $3c0000.l
pub fn i_05daba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DAC2;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[3] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[4] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $05DAC2: addi.w #$20, d3
pub fn i_05dac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DAC6;
    cpu.ir = 0x643;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05DAC6: dbra d5, $5daaa
pub fn i_05dac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DACA;
    cpu.ir = 0x51CD;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5DACA; return 12; }
    let n = (cpu.d[5] as u16).wrapping_sub(1);
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5DACA; return 14; }
    cpu.pc = 0x5DAAA;
    10
}
/// $05DACA: adda.w #$1, a1
pub fn i_05daca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DACE;
    cpu.ir = 0xD2FC;
    let v1: u32 = 0x1;
    cpu.a[1] = cpu.a[1].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05DACE: dbra d2, $5daa6
pub fn i_05dace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DAD2;
    cpu.ir = 0x51CA;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5DAD2; return 12; }
    let n = (cpu.d[2] as u16).wrapping_sub(1);
    cpu.d[2] = (cpu.d[2] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5DAD2; return 14; }
    cpu.pc = 0x5DAA6;
    10
}
/// $05DAD2: movem.l (a7)+, d3-d5
pub fn i_05dad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DAD6;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $05DAD6: rts
pub fn i_05dad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DAD8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Fix_BlitStream_05DAD8  ($05DAD8, 66 B, video/fix_blit_stream_05dad8.s)
/// $05DAD8: moveq #$0, d2
pub fn i_05dad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DADA;
    cpu.ir = 0x7400;
    cpu.d[2] = 0x0;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05DADA: move.l a1, d1
pub fn i_05dada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DADC;
    cpu.ir = 0x2209;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.d[1] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05DADC: move.w d1, -(a7)
pub fn i_05dadc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DADE;
    cpu.ir = 0x3F01;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $05DADE: move.b (a2)+, d2
pub fn i_05dade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DAE0;
    cpu.ir = 0x141A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05DAE0: cmpi.b #$fe, d2
pub fn i_05dae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DAE4;
    cpu.ir = 0xC02;
    let v1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, v1, 0xFE, Size::Byte, false);
    8
}
/// $05DAE4: beq.w $5db16
pub fn i_05dae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DAE8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5DB16; return 10; }
    8
}
/// $05DAE8: cmpi.b #$fd, d2
pub fn i_05dae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DAEC;
    cpu.ir = 0xC02;
    let v1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, v1, 0xFD, Size::Byte, false);
    8
}
/// $05DAEC: beq.w $5db10
pub fn i_05daec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DAF0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5DB10; return 10; }
    8
}
/// $05DAF0: cmpi.w #$74ff, d1
pub fn i_05daf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DAF4;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x74FF, Size::Word, false);
    8
}
/// $05DAF4: bgt.w $5db0a
pub fn i_05daf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DAF8;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x5DB0A; return 10; }
    8
}
/// $05DAF8: cmpi.w #$7000, d1
pub fn i_05daf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DAFC;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x7000, Size::Word, false);
    8
}
/// $05DAFC: blt.w $5db0a
pub fn i_05dafc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB00;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x5DB0A; return 10; }
    8
}
/// $05DB00: or.w d0, d2
pub fn i_05db00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB02;
    cpu.ir = 0x8440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = (v1 | cpu.d[2]) & 0xFFFF;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05DB02: movem.w d1-d2, $3c0000.l
pub fn i_05db02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB0A;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[2] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $05DB0A: addi.w #$20, d1
pub fn i_05db0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB0E;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05DB0E: bra.b $5dade
pub fn i_05db0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB10;
    cpu.ir = 0x60CE;
    cpu.pc = 0x5DADE;
    10
}
/// $05DB10: move.w (a7)+, d1
pub fn i_05db10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB12;
    cpu.ir = 0x321F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05DB12: addq.w #$1, d1
pub fn i_05db12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB14;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05DB14: bra.b $5dadc
pub fn i_05db14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB16;
    cpu.ir = 0x60C6;
    cpu.pc = 0x5DADC;
    10
}
/// $05DB16: move.w (a7)+, d1
pub fn i_05db16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB18;
    cpu.ir = 0x321F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05DB18: rts
pub fn i_05db18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB1A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== InstallListPubHead_05DB58  ($05DB58, 18 B, video/fix_layer_backends_05db1a.s)
/// $05DB58: move.l a0, $3c(a6)
pub fn i_05db58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB5C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $05DB5C: move.w $6(a0), d0
pub fn i_05db5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB60;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05DB60: move.w d0, $46(a6)
pub fn i_05db60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB64;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x46);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05DB64: jsr $5dbc2(pc)
pub fn i_05db64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB68;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5DB68);
    cpu.pc = 0x5DBC2;
    16
}
/// $05DB68: rts
pub fn i_05db68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB6A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ListCursor_Step_05db6a  ($05DB6A, 54 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05DB6A: movea.l $3c(a6), a1
pub fn i_05db6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB6E;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $05DB6E: cmpi.w #$ffff, (a1)
pub fn i_05db6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB72;
    cpu.ir = 0xC51;
    let v2 = bus.read16(cpu.a[1]) as u32;
    sub_with_flags(cpu, v2, 0xFFFF, Size::Word, false);
    12
}
/// $05DB72: beq.w $5dbbc
pub fn i_05db72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB76;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5DBBC; return 10; }
    8
}
/// $05DB76: subq.w #$1, $46(a6)
pub fn i_05db76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB7A;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x46);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $05DB7A: cmpi.w #$0, $46(a6)
pub fn i_05db7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB80;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x46);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $05DB80: bgt.w $5dba0
pub fn i_05db80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB84;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x5DBA0; return 10; }
    8
}
/// $05DB84: jsr $5dbdc(pc)
pub fn i_05db84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB88;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5DB88);
    cpu.pc = 0x5DBDC;
    16
}
/// $05DB88: addq.l #$8, $3c(a6)
pub fn i_05db88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB8C;
    cpu.ir = 0x50AE;
    let ad1 = cpu.a[6].wrapping_add(0x3C);
    let v2 = bus.read32(ad1);
    let r3 = add_with_flags(cpu, v2, 8, Size::Long, true);
    bus.write32(ad1, r3);
    16
}
/// $05DB8C: movea.l $3c(a6), a1
pub fn i_05db8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB90;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $05DB90: cmpi.w #$0, (a1)
pub fn i_05db90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB94;
    cpu.ir = 0xC51;
    let v2 = bus.read16(cpu.a[1]) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Word, false);
    12
}
/// $05DB94: bne.w $5dba6
pub fn i_05db94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB98;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5DBA6; return 10; }
    8
}
/// $05DB98: movea.l $2(a1), a0
pub fn i_05db98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB9C;
    cpu.ir = 0x2069;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $05DB9C: jsr $5db58(pc)
pub fn i_05db9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DBA0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5DBA0);
    cpu.pc = 0x5DB58;
    16
}
// ===== ClearXNV_05dba0  ($05DBA0, 6 B, thunks/ccr_helpers.c)
/// $05DBA0: andi.b #$e, ccr
pub fn i_05dba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DBA4;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05DBA4: rts
pub fn i_05dba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DBA6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ListCursor_LoadEntry_05dba6  ($05DBA6, 22 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05DBA6: cmpi.w #$ffff, (a1)
pub fn i_05dba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DBAA;
    cpu.ir = 0xC51;
    let v2 = bus.read16(cpu.a[1]) as u32;
    sub_with_flags(cpu, v2, 0xFFFF, Size::Word, false);
    12
}
/// $05DBAA: beq.w $5dbbc
pub fn i_05dbaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DBAE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5DBBC; return 10; }
    8
}
/// $05DBAE: move.w $6(a1), d0
pub fn i_05dbae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DBB2;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05DBB2: move.w d0, $46(a6)
pub fn i_05dbb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DBB6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x46);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05DBB6: jsr $5dbc2(pc)
pub fn i_05dbb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DBBA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5DBBA);
    cpu.pc = 0x5DBC2;
    16
}
/// $05DBBA: bra.b $5dba0
pub fn i_05dbba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DBBC;
    cpu.ir = 0x60E4;
    cpu.pc = 0x5DBA0;
    10
}
// ===== SetC_05dbbc  ($05DBBC, 6 B, thunks/ccr_helpers.c)
/// $05DBBC: ori.b #$1, ccr
pub fn i_05dbbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DBC0;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $05DBC0: rts
pub fn i_05dbc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DBC2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ListCursor_Reinit_05DBC2  ($05DBC2, 26 B, video/camera_list_ctx_helpers_025012.s)
/// $05DBC2: movea.l $3c(a6), a0
pub fn i_05dbc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DBC6;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $05DBC6: movea.w $22(a6), a1
pub fn i_05dbc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DBCA;
    cpu.ir = 0x326E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.a[1] = v3 as i16 as i32 as u32;
    12
}
/// $05DBCA: move.w (a0), d0
pub fn i_05dbca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DBCC;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $05DBCC: move.w $2(a0), d1
pub fn i_05dbcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DBD0;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05DBD0: move.w $4(a0), d2
pub fn i_05dbd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DBD4;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05DBD4: jsr $5da56.l
pub fn i_05dbd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DBDA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5DBDA);
    cpu.pc = 0x5DA56;
    16
}
/// $05DBDA: rts
pub fn i_05dbda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DBDC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ListCursor_ReinitClipped_05DBDC  ($05DBDC, 36 B, video/camera_list_ctx_helpers_025012.s)
/// $05DBDC: movea.l $3c(a6), a0
pub fn i_05dbdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DBE0;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $05DBE0: cmpi.w #$ffff, (a0)
pub fn i_05dbe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DBE4;
    cpu.ir = 0xC50;
    let v2 = bus.read16(cpu.a[0]) as u32;
    sub_with_flags(cpu, v2, 0xFFFF, Size::Word, false);
    12
}
/// $05DBE4: beq.w $5dbfe
pub fn i_05dbe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DBE8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5DBFE; return 10; }
    8
}
/// $05DBE8: movea.w $22(a6), a1
pub fn i_05dbe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DBEC;
    cpu.ir = 0x326E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.a[1] = v3 as i16 as i32 as u32;
    12
}
/// $05DBEC: move.w #$ff, d0
pub fn i_05dbec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DBF0;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05DBF0: move.w $2(a0), d1
pub fn i_05dbf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DBF4;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05DBF4: move.w $4(a0), d2
pub fn i_05dbf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DBF8;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05DBF8: jsr $5da9c.l
pub fn i_05dbf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DBFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5DBFE);
    cpu.pc = 0x5DA9C;
    16
}
/// $05DBFE: rts
pub fn i_05dbfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC00;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_ClearPtrSlots_05DC1C  ($05DC1C, 24 B, entity/clear_ptr_slots_05dc1c.s)
/// $05DC1C: move.w #$9c, d0
pub fn i_05dc1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC20;
    cpu.ir = 0x303C;
    let v1: u32 = 0x9C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05DC20: move.l #$ffffffff, (a0, d0.w)
pub fn i_05dc20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC28;
    cpu.ir = 0x21BC;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $05DC28: subi.w #$4, d0
pub fn i_05dc28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC2C;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x4, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05DC2C: cmpi.w #$10, d0
pub fn i_05dc2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC30;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x10, Size::Word, false);
    8
}
/// $05DC30: bcc.b $5dc20
pub fn i_05dc30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC32;
    cpu.ir = 0x64EE;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5DC20; return 10; }
    8
}
/// $05DC32: rts
pub fn i_05dc32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC34;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_InitFields_05DC34  ($05DC34, 112 B, entity/init_fields_05dc34.s)
/// $05DC34: clr.w $14(a0)
pub fn i_05dc34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC38;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x14);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $05DC38: move.w #$ffff, $16(a0)
pub fn i_05dc38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC3E;
    cpu.ir = 0x317C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x16);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05DC3E: move.w #$ffff, $18(a0)
pub fn i_05dc3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC44;
    cpu.ir = 0x317C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x18);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05DC44: move.w #$ffff, $1a(a0)
pub fn i_05dc44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC4A;
    cpu.ir = 0x317C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x1A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05DC4A: move.w #$ffff, $1c(a0)
pub fn i_05dc4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC50;
    cpu.ir = 0x317C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05DC50: clr.w $1e(a0)
pub fn i_05dc50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC54;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x1E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $05DC54: clr.b $12(a0)
pub fn i_05dc54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC58;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $05DC58: clr.w $38(a0)
pub fn i_05dc58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC5C;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $05DC5C: clr.b $13(a0)
pub fn i_05dc5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC60;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $05DC60: clr.b $5a(a0)
pub fn i_05dc60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC64;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $05DC64: clr.b $69(a0)
pub fn i_05dc64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC68;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x69);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $05DC68: clr.b $5b(a0)
pub fn i_05dc68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC6C;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x5B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $05DC6C: clr.b $6b(a0)
pub fn i_05dc6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC70;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $05DC70: move.b #$ff, $32(a0)
pub fn i_05dc70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC76;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05DC76: move.b #$ff, $33(a0)
pub fn i_05dc76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC7C;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05DC7C: clr.b $3a(a0)
pub fn i_05dc7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC80;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $05DC80: clr.l $28(a0)
pub fn i_05dc80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC84;
    cpu.ir = 0x42A8;
    let ad1 = cpu.a[0].wrapping_add(0x28);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $05DC84: clr.l $2c(a0)
pub fn i_05dc84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC88;
    cpu.ir = 0x42A8;
    let ad1 = cpu.a[0].wrapping_add(0x2C);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $05DC88: clr.w $26(a0)
pub fn i_05dc88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC8C;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x26);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $05DC8C: clr.b $59(a0)
pub fn i_05dc8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC90;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x59);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $05DC90: clr.b $45(a0)
pub fn i_05dc90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC94;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x45);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $05DC94: clr.b $44(a0)
pub fn i_05dc94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC98;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x44);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $05DC98: clr.b $6a(a0)
pub fn i_05dc98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC9C;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x6A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $05DC9C: bset.b #$6, $13(a0)
pub fn i_05dc9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DCA2;
    cpu.ir = 0x8E8;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05DCA2: rts
pub fn i_05dca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DCA4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_NegIfFacing_05dca4  ($05DCA4, 12 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05DCA4: btst.b #$0, $3a(a6)
pub fn i_05dca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DCAA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05DCAA: beq.w $5dcb0
pub fn i_05dcaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DCAE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5DCB0; return 10; }
    8
}
/// $05DCAE: neg.w d0
pub fn i_05dcae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DCB0;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
// ===== SetTaskW_05dcb0  ($05DCB0, 6 B, kernel/task_setters_w.c)
/// $05DCB0: move.w d0, $28(a6)
pub fn i_05dcb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DCB4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05DCB4: rts
pub fn i_05dcb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DCB6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Spawn_ChildFromDesc_05dcce  ($05DCCE, 52 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05DCCE: movem.l a1/a6, -(a7)
pub fn i_05dcce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DCD2;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $05DCD2: cmpi.b #$2, (a1)
pub fn i_05dcd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DCD6;
    cpu.ir = 0xC11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    sub_with_flags(cpu, v2, 0x2, Size::Byte, false);
    12
}
/// $05DCD6: beq.w $5dcee
pub fn i_05dcd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DCDA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5DCEE; return 10; }
    8
}
/// $05DCDA: lea.l $1008a0.l, a6
pub fn i_05dcda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DCE0;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0x1008A0;
    20
}
/// $05DCE0: movea.l $6(a1), a1
pub fn i_05dce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DCE4;
    cpu.ir = 0x2269;
    let ad2 = cpu.a[1].wrapping_add(0x6);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $05DCE4: jsr $4ae.l
pub fn i_05dce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DCEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5DCEA);
    cpu.pc = 0x4AE;
    16
}
/// $05DCEA: bra.w $5dcf8
pub fn i_05dcea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DCEE;
    cpu.ir = 0x6000;
    cpu.pc = 0x5DCF8;
    10
}
/// $05DCEE: movea.l $6(a1), a1
pub fn i_05dcee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DCF2;
    cpu.ir = 0x2269;
    let ad2 = cpu.a[1].wrapping_add(0x6);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $05DCF2: jsr $4498e.l
pub fn i_05dcf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DCF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5DCF8);
    cpu.pc = 0x4498E;
    16
}
/// $05DCF8: movem.l (a7)+, a1/a6
pub fn i_05dcf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DCFC;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $05DCFC: move.l a1, $3c(a0)
pub fn i_05dcfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD00;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $05DD00: rts
pub fn i_05dd00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD02;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CopyTransform  ($05DD02, 32 B, entity/copy_transform_05dd02.s)
/// $05DD02: move.w $22(a6), $22(a0)
pub fn i_05dd02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD08;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $05DD08: move.w $24(a6), $24(a0)
pub fn i_05dd08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD0E;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $05DD0E: move.w $38(a6), $38(a0)
pub fn i_05dd0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD14;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad4, v3 as u16);
    20
}
/// $05DD14: move.b $3a(a6), $3a(a0)
pub fn i_05dd14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD1A;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x3A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $05DD1A: move.b $11(a6), $11(a0)
pub fn i_05dd1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD20;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x11);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x11);
    bus.write8(ad4, v3 as u8);
    20
}
/// $05DD20: rts
pub fn i_05dd20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD22;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_SetOffscreenFlag_05dd56  ($05DD56, 6 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05DD56: bset.b #$7, $13(a6)
pub fn i_05dd56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD5C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== Entity_CheckOnScreenBox_05dd5c  ($05DD5C, 86 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05DD5C: cmpa.l #$ffffffff, a0
pub fn i_05dd5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD62;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05DD62: bne.w $5dd6a
pub fn i_05dd62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD66;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5DD6A; return 10; }
    8
}
/// $05DD66: lea.l $5dd4c(pc), a0
pub fn i_05dd66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD6A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x5DD4C;
    16
}
/// $05DD6A: btst.b #$7, $13(a6)
pub fn i_05dd6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD70;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05DD70: beq.w $5ddbe
pub fn i_05dd70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD74;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5DDBE; return 10; }
    8
}
/// $05DD74: move.w #$0, d0
pub fn i_05dd74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD78;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05DD78: move.w #$140, d1
pub fn i_05dd78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD7C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x140;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05DD7C: move.w #$100, d2
pub fn i_05dd7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD80;
    cpu.ir = 0x343C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x05B13A, i_05b13a),
    (0x05B13E, i_05b13e),
    (0x05B140, i_05b140),
    (0x05B142, i_05b142),
    (0x05B144, i_05b144),
    (0x05B146, i_05b146),
    (0x05B148, i_05b148),
    (0x05B14A, i_05b14a),
    (0x05B14C, i_05b14c),
    (0x05B14E, i_05b14e),
    (0x05B152, i_05b152),
    (0x05B156, i_05b156),
    (0x05B15A, i_05b15a),
    (0x05B15C, i_05b15c),
    (0x05B15E, i_05b15e),
    (0x05B162, i_05b162),
    (0x05B164, i_05b164),
    (0x05B168, i_05b168),
    (0x05B16C, i_05b16c),
    (0x05B16E, i_05b16e),
    (0x05B170, i_05b170),
    (0x05B174, i_05b174),
    (0x05B178, i_05b178),
    (0x05B17C, i_05b17c),
    (0x05B180, i_05b180),
    (0x05B182, i_05b182),
    (0x05B184, i_05b184),
    (0x05B186, i_05b186),
    (0x05B188, i_05b188),
    (0x05B18A, i_05b18a),
    (0x05B18E, i_05b18e),
    (0x05B190, i_05b190),
    (0x05B192, i_05b192),
    (0x05B196, i_05b196),
    (0x05B198, i_05b198),
    (0x05B19A, i_05b19a),
    (0x05B19E, i_05b19e),
    (0x05B1A0, i_05b1a0),
    (0x05B1A4, i_05b1a4),
    (0x05B1A6, i_05b1a6),
    (0x05B1AA, i_05b1aa),
    (0x05B1B0, i_05b1b0),
    (0x05B1B2, i_05b1b2),
    (0x05B1B6, i_05b1b6),
    (0x05B1BE, i_05b1be),
    (0x05B1C2, i_05b1c2),
    (0x05B1C8, i_05b1c8),
    (0x05B1CA, i_05b1ca),
    (0x05B1CE, i_05b1ce),
    (0x05B1D2, i_05b1d2),
    (0x05B1D6, i_05b1d6),
    (0x05B1DA, i_05b1da),
    (0x05B1DE, i_05b1de),
    (0x05B1E4, i_05b1e4),
    (0x05B1E6, i_05b1e6),
    (0x05B1EA, i_05b1ea),
    (0x05B1EE, i_05b1ee),
    (0x05B1F2, i_05b1f2),
    (0x05B1F6, i_05b1f6),
    (0x05B1FA, i_05b1fa),
    (0x05B1FE, i_05b1fe),
    (0x05B202, i_05b202),
    (0x05B206, i_05b206),
    (0x05B20A, i_05b20a),
    (0x05B20C, i_05b20c),
    (0x05B210, i_05b210),
    (0x05B212, i_05b212),
    (0x05B216, i_05b216),
    (0x05B218, i_05b218),
    (0x05B21A, i_05b21a),
    (0x05B21E, i_05b21e),
    (0x05B224, i_05b224),
    (0x05B226, i_05b226),
    (0x05B228, i_05b228),
    (0x05B22A, i_05b22a),
    (0x05B22E, i_05b22e),
    (0x05B230, i_05b230),
    (0x05B232, i_05b232),
    (0x05B238, i_05b238),
    (0x05B23C, i_05b23c),
    (0x05B240, i_05b240),
    (0x05B244, i_05b244),
    (0x05B246, i_05b246),
    (0x05B248, i_05b248),
    (0x05B24C, i_05b24c),
    (0x05B250, i_05b250),
    (0x05B254, i_05b254),
    (0x05B256, i_05b256),
    (0x05B258, i_05b258),
    (0x05B25A, i_05b25a),
    (0x05B25E, i_05b25e),
    (0x05B260, i_05b260),
    (0x05B264, i_05b264),
    (0x05B268, i_05b268),
    (0x05B26A, i_05b26a),
    (0x05B26E, i_05b26e),
    (0x05B270, i_05b270),
    (0x05B272, i_05b272),
    (0x05B274, i_05b274),
    (0x05B278, i_05b278),
    (0x05B27C, i_05b27c),
    (0x05B27E, i_05b27e),
    (0x05B280, i_05b280),
    (0x05B282, i_05b282),
    (0x05B284, i_05b284),
    (0x05B288, i_05b288),
    (0x05B28A, i_05b28a),
    (0x05B28C, i_05b28c),
    (0x05B290, i_05b290),
    (0x05B294, i_05b294),
    (0x05B298, i_05b298),
    (0x05B29C, i_05b29c),
    (0x05B29E, i_05b29e),
    (0x05B2A0, i_05b2a0),
    (0x05B2A4, i_05b2a4),
    (0x05B2A6, i_05b2a6),
    (0x05B2A8, i_05b2a8),
    (0x05B2AA, i_05b2aa),
    (0x05B2AE, i_05b2ae),
    (0x05B2B0, i_05b2b0),
    (0x05B2B4, i_05b2b4),
    (0x05B2B8, i_05b2b8),
    (0x05B2BA, i_05b2ba),
    (0x05B2BE, i_05b2be),
    (0x05B2C0, i_05b2c0),
    (0x05B2C2, i_05b2c2),
    (0x05B2C4, i_05b2c4),
    (0x05B2C8, i_05b2c8),
    (0x05B2CC, i_05b2cc),
    (0x05B2CE, i_05b2ce),
    (0x05B2D0, i_05b2d0),
    (0x05B2D2, i_05b2d2),
    (0x05B2D4, i_05b2d4),
    (0x05B2D8, i_05b2d8),
    (0x05B2DA, i_05b2da),
    (0x05B2DE, i_05b2de),
    (0x05B2E2, i_05b2e2),
    (0x05B2E6, i_05b2e6),
    (0x05B2EA, i_05b2ea),
    (0x05B2EE, i_05b2ee),
    (0x05B2F2, i_05b2f2),
    (0x05B2F4, i_05b2f4),
    (0x05B2F8, i_05b2f8),
    (0x05B2FA, i_05b2fa),
    (0x05B2FC, i_05b2fc),
    (0x05B2FE, i_05b2fe),
    (0x05B300, i_05b300),
    (0x05B304, i_05b304),
    (0x05B308, i_05b308),
    (0x05B30A, i_05b30a),
    (0x05B30C, i_05b30c),
    (0x05B30E, i_05b30e),
    (0x05B310, i_05b310),
    (0x05B312, i_05b312),
    (0x05B316, i_05b316),
    (0x05B318, i_05b318),
    (0x05B31A, i_05b31a),
    (0x05B31C, i_05b31c),
    (0x05B322, i_05b322),
    (0x05B326, i_05b326),
    (0x05B32C, i_05b32c),
    (0x05B334, i_05b334),
    (0x05B338, i_05b338),
    (0x05B33C, i_05b33c),
    (0x05B340, i_05b340),
    (0x05B344, i_05b344),
    (0x05B348, i_05b348),
    (0x05B34E, i_05b34e),
    (0x05B354, i_05b354),
    (0x05B358, i_05b358),
    (0x05B35C, i_05b35c),
    (0x05B360, i_05b360),
    (0x05B362, i_05b362),
    (0x05B366, i_05b366),
    (0x05B368, i_05b368),
    (0x05B36E, i_05b36e),
    (0x05B370, i_05b370),
    (0x05B374, i_05b374),
    (0x05B378, i_05b378),
    (0x05B37C, i_05b37c),
    (0x05B37E, i_05b37e),
    (0x05B380, i_05b380),
    (0x05B382, i_05b382),
    (0x05B386, i_05b386),
    (0x05B388, i_05b388),
    (0x05B38C, i_05b38c),
    (0x05B38E, i_05b38e),
    (0x05B392, i_05b392),
    (0x05B394, i_05b394),
    (0x05B396, i_05b396),
    (0x05B39A, i_05b39a),
    (0x05B39C, i_05b39c),
    (0x05B3A0, i_05b3a0),
    (0x05B3A4, i_05b3a4),
    (0x05B3A6, i_05b3a6),
    (0x05B3AA, i_05b3aa),
    (0x05B3AE, i_05b3ae),
    (0x05B3B2, i_05b3b2),
    (0x05B3B4, i_05b3b4),
    (0x05B3B8, i_05b3b8),
    (0x05B3BA, i_05b3ba),
    (0x05B3BE, i_05b3be),
    (0x05B3C2, i_05b3c2),
    (0x05B3C4, i_05b3c4),
    (0x05B3C6, i_05b3c6),
    (0x05B3C8, i_05b3c8),
    (0x05B3CA, i_05b3ca),
    (0x05B3CE, i_05b3ce),
    (0x05B3D0, i_05b3d0),
    (0x05B3D2, i_05b3d2),
    (0x05B3D6, i_05b3d6),
    (0x05B3D8, i_05b3d8),
    (0x05B3DA, i_05b3da),
    (0x05B3DE, i_05b3de),
    (0x05B3E0, i_05b3e0),
    (0x05B3E4, i_05b3e4),
    (0x05B3E6, i_05b3e6),
    (0x05B3EA, i_05b3ea),
    (0x05B3EE, i_05b3ee),
    (0x05B3F2, i_05b3f2),
    (0x05B3F6, i_05b3f6),
    (0x05B3F8, i_05b3f8),
    (0x05B3FA, i_05b3fa),
    (0x05B3FE, i_05b3fe),
    (0x05B400, i_05b400),
    (0x05B404, i_05b404),
    (0x05B408, i_05b408),
    (0x05B40A, i_05b40a),
    (0x05B40C, i_05b40c),
    (0x05B40E, i_05b40e),
    (0x05B412, i_05b412),
    (0x05B416, i_05b416),
    (0x05B418, i_05b418),
    (0x05B41C, i_05b41c),
    (0x05B41E, i_05b41e),
    (0x05B422, i_05b422),
    (0x05B424, i_05b424),
    (0x05B426, i_05b426),
    (0x05B42A, i_05b42a),
    (0x05B42E, i_05b42e),
    (0x05B432, i_05b432),
    (0x05B436, i_05b436),
    (0x05B438, i_05b438),
    (0x05B43A, i_05b43a),
    (0x05B43E, i_05b43e),
    (0x05B440, i_05b440),
    (0x05B444, i_05b444),
    (0x05B448, i_05b448),
    (0x05B44A, i_05b44a),
    (0x05B44C, i_05b44c),
    (0x05B450, i_05b450),
    (0x05B454, i_05b454),
    (0x05B456, i_05b456),
    (0x05B45A, i_05b45a),
    (0x05B45C, i_05b45c),
    (0x05B45E, i_05b45e),
    (0x05B460, i_05b460),
    (0x05B464, i_05b464),
    (0x05B468, i_05b468),
    (0x05B46C, i_05b46c),
    (0x05B46E, i_05b46e),
    (0x05B470, i_05b470),
    (0x05B474, i_05b474),
    (0x05B476, i_05b476),
    (0x05B478, i_05b478),
    (0x05B47E, i_05b47e),
    (0x05B480, i_05b480),
    (0x05B482, i_05b482),
    (0x05B484, i_05b484),
    (0x05B486, i_05b486),
    (0x05B48A, i_05b48a),
    (0x05B48C, i_05b48c),
    (0x05B48E, i_05b48e),
    (0x05B492, i_05b492),
    (0x05B496, i_05b496),
    (0x05B49A, i_05b49a),
    (0x05B49C, i_05b49c),
    (0x05B49E, i_05b49e),
    (0x05B4A0, i_05b4a0),
    (0x05B4A4, i_05b4a4),
    (0x05B4A6, i_05b4a6),
    (0x05B4AA, i_05b4aa),
    (0x05B4AC, i_05b4ac),
    (0x05B4AE, i_05b4ae),
    (0x05B4B2, i_05b4b2),
    (0x05B4B4, i_05b4b4),
    (0x05B4B6, i_05b4b6),
    (0x05B4BA, i_05b4ba),
    (0x05B4BC, i_05b4bc),
    (0x05B4BE, i_05b4be),
    (0x05B4C2, i_05b4c2),
    (0x05B4C6, i_05b4c6),
    (0x05B4CA, i_05b4ca),
    (0x05B4CC, i_05b4cc),
    (0x05B4CE, i_05b4ce),
    (0x05B4D2, i_05b4d2),
    (0x05B4D4, i_05b4d4),
    (0x05B4D8, i_05b4d8),
    (0x05B4DC, i_05b4dc),
    (0x05B4E0, i_05b4e0),
    (0x05B4E4, i_05b4e4),
    (0x05B4E6, i_05b4e6),
    (0x05B4E8, i_05b4e8),
    (0x05B4EA, i_05b4ea),
    (0x05B4EC, i_05b4ec),
    (0x05B4F0, i_05b4f0),
    (0x05B4F4, i_05b4f4),
    (0x05B4F8, i_05b4f8),
    (0x05B4FA, i_05b4fa),
    (0x05B4FC, i_05b4fc),
    (0x05B500, i_05b500),
    (0x05B502, i_05b502),
    (0x05B504, i_05b504),
    (0x05B50A, i_05b50a),
    (0x05B50C, i_05b50c),
    (0x05B50E, i_05b50e),
    (0x05B510, i_05b510),
    (0x05B512, i_05b512),
    (0x05B516, i_05b516),
    (0x05B518, i_05b518),
    (0x05B51A, i_05b51a),
    (0x05B51E, i_05b51e),
    (0x05B522, i_05b522),
    (0x05B524, i_05b524),
    (0x05B528, i_05b528),
    (0x05B52C, i_05b52c),
    (0x05B52E, i_05b52e),
    (0x05B532, i_05b532),
    (0x05B534, i_05b534),
    (0x05B536, i_05b536),
    (0x05B53A, i_05b53a),
    (0x05B53C, i_05b53c),
    (0x05B540, i_05b540),
    (0x05B542, i_05b542),
    (0x05B546, i_05b546),
    (0x05B54A, i_05b54a),
    (0x05B54E, i_05b54e),
    (0x05B552, i_05b552),
    (0x05B556, i_05b556),
    (0x05B55A, i_05b55a),
    (0x05B55C, i_05b55c),
    (0x05B55E, i_05b55e),
    (0x05B560, i_05b560),
    (0x05B562, i_05b562),
    (0x05B566, i_05b566),
    (0x05B568, i_05b568),
    (0x05B56A, i_05b56a),
    (0x05B56C, i_05b56c),
    (0x05B56E, i_05b56e),
    (0x05B572, i_05b572),
    (0x05B574, i_05b574),
    (0x05B578, i_05b578),
    (0x05B57A, i_05b57a),
    (0x05B57C, i_05b57c),
    (0x05B580, i_05b580),
    (0x05B584, i_05b584),
    (0x05B588, i_05b588),
    (0x05B58C, i_05b58c),
    (0x05B590, i_05b590),
    (0x05B594, i_05b594),
    (0x05B598, i_05b598),
    (0x05B59C, i_05b59c),
    (0x05B5A0, i_05b5a0),
    (0x05B5A4, i_05b5a4),
    (0x05B5A8, i_05b5a8),
    (0x05B5AC, i_05b5ac),
    (0x05B5B0, i_05b5b0),
    (0x05B5B4, i_05b5b4),
    (0x05B5B8, i_05b5b8),
    (0x05B5BC, i_05b5bc),
    (0x05B5DC, i_05b5dc),
    (0x05B64C, i_05b64c),
    (0x05B64E, i_05b64e),
    (0x05B650, i_05b650),
    (0x05B652, i_05b652),
    (0x05B654, i_05b654),
    (0x05B656, i_05b656),
    (0x05B658, i_05b658),
    (0x05B65A, i_05b65a),
    (0x05B65C, i_05b65c),
    (0x05B65E, i_05b65e),
    (0x05B660, i_05b660),
    (0x05B662, i_05b662),
    (0x05B664, i_05b664),
    (0x05B666, i_05b666),
    (0x05B668, i_05b668),
    (0x05B66A, i_05b66a),
    (0x05B66C, i_05b66c),
    (0x05B66E, i_05b66e),
    (0x05B670, i_05b670),
    (0x05B672, i_05b672),
    (0x05B674, i_05b674),
    (0x05B676, i_05b676),
    (0x05B678, i_05b678),
    (0x05B67A, i_05b67a),
    (0x05B67C, i_05b67c),
    (0x05B67E, i_05b67e),
    (0x05B680, i_05b680),
    (0x05B682, i_05b682),
    (0x05B684, i_05b684),
    (0x05B686, i_05b686),
    (0x05B688, i_05b688),
    (0x05B68A, i_05b68a),
    (0x05B68C, i_05b68c),
    (0x05B68E, i_05b68e),
    (0x05B690, i_05b690),
    (0x05B692, i_05b692),
    (0x05B694, i_05b694),
    (0x05B696, i_05b696),
    (0x05B698, i_05b698),
    (0x05B69A, i_05b69a),
    (0x05B69C, i_05b69c),
    (0x05B69E, i_05b69e),
    (0x05B6A0, i_05b6a0),
    (0x05B6A2, i_05b6a2),
    (0x05B6A4, i_05b6a4),
    (0x05B6A6, i_05b6a6),
    (0x05B6A8, i_05b6a8),
    (0x05B6AA, i_05b6aa),
    (0x05B6AC, i_05b6ac),
    (0x05B6AE, i_05b6ae),
    (0x05B6B0, i_05b6b0),
    (0x05B6B2, i_05b6b2),
    (0x05B6B4, i_05b6b4),
    (0x05B6B6, i_05b6b6),
    (0x05B6B8, i_05b6b8),
    (0x05B6BA, i_05b6ba),
    (0x05B6BC, i_05b6bc),
    (0x05B6BE, i_05b6be),
    (0x05B6C0, i_05b6c0),
    (0x05B6C2, i_05b6c2),
    (0x05B6C4, i_05b6c4),
    (0x05B6C6, i_05b6c6),
    (0x05B6C8, i_05b6c8),
    (0x05B6CA, i_05b6ca),
    (0x05B6CC, i_05b6cc),
    (0x05B6CE, i_05b6ce),
    (0x05B6D0, i_05b6d0),
    (0x05B6D2, i_05b6d2),
    (0x05B6D4, i_05b6d4),
    (0x05B6D6, i_05b6d6),
    (0x05B6D8, i_05b6d8),
    (0x05B6DA, i_05b6da),
    (0x05B6DC, i_05b6dc),
    (0x05B6DE, i_05b6de),
    (0x05B6E0, i_05b6e0),
    (0x05B6E2, i_05b6e2),
    (0x05B6E4, i_05b6e4),
    (0x05B6E6, i_05b6e6),
    (0x05B6E8, i_05b6e8),
    (0x05B6EA, i_05b6ea),
    (0x05B6EC, i_05b6ec),
    (0x05B6EE, i_05b6ee),
    (0x05B6F0, i_05b6f0),
    (0x05B6F2, i_05b6f2),
    (0x05B6F4, i_05b6f4),
    (0x05B6F6, i_05b6f6),
    (0x05B6F8, i_05b6f8),
    (0x05B6FA, i_05b6fa),
    (0x05B6FC, i_05b6fc),
    (0x05B6FE, i_05b6fe),
    (0x05B700, i_05b700),
    (0x05B702, i_05b702),
    (0x05B704, i_05b704),
    (0x05B706, i_05b706),
    (0x05B708, i_05b708),
    (0x05B70A, i_05b70a),
    (0x05B70C, i_05b70c),
    (0x05B70E, i_05b70e),
    (0x05B710, i_05b710),
    (0x05B712, i_05b712),
    (0x05B714, i_05b714),
    (0x05B716, i_05b716),
    (0x05B718, i_05b718),
    (0x05B71A, i_05b71a),
    (0x05B71C, i_05b71c),
    (0x05B71E, i_05b71e),
    (0x05B720, i_05b720),
    (0x05B722, i_05b722),
    (0x05B724, i_05b724),
    (0x05B726, i_05b726),
    (0x05B728, i_05b728),
    (0x05B72A, i_05b72a),
    (0x05B72C, i_05b72c),
    (0x05B72E, i_05b72e),
    (0x05B730, i_05b730),
    (0x05B732, i_05b732),
    (0x05B734, i_05b734),
    (0x05B736, i_05b736),
    (0x05B738, i_05b738),
    (0x05B73A, i_05b73a),
    (0x05B73C, i_05b73c),
    (0x05B740, i_05b740),
    (0x05B742, i_05b742),
    (0x05B744, i_05b744),
    (0x05B748, i_05b748),
    (0x05B74E, i_05b74e),
    (0x05B750, i_05b750),
    (0x05B752, i_05b752),
    (0x05B754, i_05b754),
    (0x05B758, i_05b758),
    (0x05B75A, i_05b75a),
    (0x05B75C, i_05b75c),
    (0x05B760, i_05b760),
    (0x05B762, i_05b762),
    (0x05B764, i_05b764),
    (0x05B766, i_05b766),
    (0x05B768, i_05b768),
    (0x05B76C, i_05b76c),
    (0x05B76E, i_05b76e),
    (0x05B770, i_05b770),
    (0x05B772, i_05b772),
    (0x05B774, i_05b774),
    (0x05B778, i_05b778),
    (0x05B77A, i_05b77a),
    (0x05B77E, i_05b77e),
    (0x05B780, i_05b780),
    (0x05B782, i_05b782),
    (0x05B78A, i_05b78a),
    (0x05B796, i_05b796),
    (0x05B79A, i_05b79a),
    (0x05B79E, i_05b79e),
    (0x05B7A2, i_05b7a2),
    (0x05B7A6, i_05b7a6),
    (0x05B7AA, i_05b7aa),
    (0x05B8DE, i_05b8de),
    (0x05B8E0, i_05b8e0),
    (0x05B8E2, i_05b8e2),
    (0x05B8E4, i_05b8e4),
    (0x05B8E6, i_05b8e6),
    (0x05B8E8, i_05b8e8),
    (0x05B8EA, i_05b8ea),
    (0x05B8EC, i_05b8ec),
    (0x05B8EE, i_05b8ee),
    (0x05B8F0, i_05b8f0),
    (0x05B8F2, i_05b8f2),
    (0x05B8F4, i_05b8f4),
    (0x05B8F6, i_05b8f6),
    (0x05B8F8, i_05b8f8),
    (0x05B8FA, i_05b8fa),
    (0x05B8FC, i_05b8fc),
    (0x05B8FE, i_05b8fe),
    (0x05B900, i_05b900),
    (0x05B902, i_05b902),
    (0x05B904, i_05b904),
    (0x05B906, i_05b906),
    (0x05B908, i_05b908),
    (0x05B90A, i_05b90a),
    (0x05B90C, i_05b90c),
    (0x05B90E, i_05b90e),
    (0x05B910, i_05b910),
    (0x05B912, i_05b912),
    (0x05B914, i_05b914),
    (0x05B916, i_05b916),
    (0x05B918, i_05b918),
    (0x05B91A, i_05b91a),
    (0x05B91C, i_05b91c),
    (0x05B91E, i_05b91e),
    (0x05B920, i_05b920),
    (0x05B922, i_05b922),
    (0x05B924, i_05b924),
    (0x05B926, i_05b926),
    (0x05B928, i_05b928),
    (0x05B92A, i_05b92a),
    (0x05B92C, i_05b92c),
    (0x05B92E, i_05b92e),
    (0x05B930, i_05b930),
    (0x05B932, i_05b932),
    (0x05B934, i_05b934),
    (0x05B936, i_05b936),
    (0x05B938, i_05b938),
    (0x05B93A, i_05b93a),
    (0x05B93C, i_05b93c),
    (0x05B93E, i_05b93e),
    (0x05B940, i_05b940),
    (0x05B942, i_05b942),
    (0x05B946, i_05b946),
    (0x05B948, i_05b948),
    (0x05B94A, i_05b94a),
    (0x05B94E, i_05b94e),
    (0x05B954, i_05b954),
    (0x05B956, i_05b956),
    (0x05B958, i_05b958),
    (0x05B95A, i_05b95a),
    (0x05B95E, i_05b95e),
    (0x05B960, i_05b960),
    (0x05B962, i_05b962),
    (0x05B966, i_05b966),
    (0x05B968, i_05b968),
    (0x05B96A, i_05b96a),
    (0x05B96C, i_05b96c),
    (0x05B96E, i_05b96e),
    (0x05B972, i_05b972),
    (0x05B974, i_05b974),
    (0x05B976, i_05b976),
    (0x05B978, i_05b978),
    (0x05B97A, i_05b97a),
    (0x05B97E, i_05b97e),
    (0x05B980, i_05b980),
    (0x05B984, i_05b984),
    (0x05B986, i_05b986),
    (0x05B988, i_05b988),
    (0x05B98C, i_05b98c),
    (0x05B990, i_05b990),
    (0x05B994, i_05b994),
    (0x05B998, i_05b998),
    (0x05B99C, i_05b99c),
    (0x05B9A0, i_05b9a0),
    (0x05B9A4, i_05b9a4),
    (0x05B9A8, i_05b9a8),
    (0x05B9AC, i_05b9ac),
    (0x05B9B0, i_05b9b0),
    (0x05B9B4, i_05b9b4),
    (0x05BADA, i_05bada),
    (0x05BADC, i_05badc),
    (0x05BADE, i_05bade),
    (0x05BAE0, i_05bae0),
    (0x05BAE2, i_05bae2),
    (0x05BAE4, i_05bae4),
    (0x05BAE6, i_05bae6),
    (0x05BAE8, i_05bae8),
    (0x05BAEA, i_05baea),
    (0x05BAEC, i_05baec),
    (0x05BAEE, i_05baee),
    (0x05BAF0, i_05baf0),
    (0x05BAF2, i_05baf2),
    (0x05BAF4, i_05baf4),
    (0x05BAF6, i_05baf6),
    (0x05BAF8, i_05baf8),
    (0x05BAFA, i_05bafa),
    (0x05BAFC, i_05bafc),
    (0x05BAFE, i_05bafe),
    (0x05BB00, i_05bb00),
    (0x05BB02, i_05bb02),
    (0x05BB04, i_05bb04),
    (0x05BB06, i_05bb06),
    (0x05BB08, i_05bb08),
    (0x05BB0A, i_05bb0a),
    (0x05BB0C, i_05bb0c),
    (0x05BB0E, i_05bb0e),
    (0x05BB10, i_05bb10),
    (0x05BB12, i_05bb12),
    (0x05BB14, i_05bb14),
    (0x05BB16, i_05bb16),
    (0x05BB18, i_05bb18),
    (0x05BB1A, i_05bb1a),
    (0x05BB1C, i_05bb1c),
    (0x05BB1E, i_05bb1e),
    (0x05BB20, i_05bb20),
    (0x05BB22, i_05bb22),
    (0x05BB24, i_05bb24),
    (0x05BB26, i_05bb26),
    (0x05BB28, i_05bb28),
    (0x05BB2A, i_05bb2a),
    (0x05BB2C, i_05bb2c),
    (0x05BB2E, i_05bb2e),
    (0x05BB30, i_05bb30),
    (0x05BB32, i_05bb32),
    (0x05BB34, i_05bb34),
    (0x05BB36, i_05bb36),
    (0x05BB38, i_05bb38),
    (0x05BB3A, i_05bb3a),
    (0x05BB3C, i_05bb3c),
    (0x05BB3E, i_05bb3e),
    (0x05BB40, i_05bb40),
    (0x05BB42, i_05bb42),
    (0x05BB44, i_05bb44),
    (0x05BB46, i_05bb46),
    (0x05BB48, i_05bb48),
    (0x05BB4A, i_05bb4a),
    (0x05BB4C, i_05bb4c),
    (0x05BB4E, i_05bb4e),
    (0x05BB50, i_05bb50),
    (0x05BB54, i_05bb54),
    (0x05BB56, i_05bb56),
    (0x05BB58, i_05bb58),
    (0x05BB5A, i_05bb5a),
    (0x05BB5C, i_05bb5c),
    (0x05BB5E, i_05bb5e),
    (0x05BB60, i_05bb60),
    (0x05BB62, i_05bb62),
    (0x05BB64, i_05bb64),
    (0x05BB66, i_05bb66),
    (0x05BB68, i_05bb68),
    (0x05BB6A, i_05bb6a),
    (0x05BB6C, i_05bb6c),
    (0x05BB6E, i_05bb6e),
    (0x05BB70, i_05bb70),
    (0x05BB72, i_05bb72),
    (0x05BB74, i_05bb74),
    (0x05BB76, i_05bb76),
    (0x05BB78, i_05bb78),
    (0x05BB7A, i_05bb7a),
    (0x05BB7C, i_05bb7c),
    (0x05BB7E, i_05bb7e),
    (0x05BB80, i_05bb80),
    (0x05BB82, i_05bb82),
    (0x05BB84, i_05bb84),
    (0x05BB86, i_05bb86),
    (0x05BB88, i_05bb88),
    (0x05BB8A, i_05bb8a),
    (0x05BB8C, i_05bb8c),
    (0x05BB8E, i_05bb8e),
    (0x05BB90, i_05bb90),
    (0x05BB92, i_05bb92),
    (0x05BB94, i_05bb94),
    (0x05BB96, i_05bb96),
    (0x05BB98, i_05bb98),
    (0x05BB9A, i_05bb9a),
    (0x05BB9C, i_05bb9c),
    (0x05BB9E, i_05bb9e),
    (0x05BBA0, i_05bba0),
    (0x05BBA2, i_05bba2),
    (0x05BBA4, i_05bba4),
    (0x05BBA6, i_05bba6),
    (0x05BBA8, i_05bba8),
    (0x05BBAA, i_05bbaa),
    (0x05BBAC, i_05bbac),
    (0x05BBAE, i_05bbae),
    (0x05BBB0, i_05bbb0),
    (0x05BBB2, i_05bbb2),
    (0x05BBB4, i_05bbb4),
    (0x05BBB6, i_05bbb6),
    (0x05BBB8, i_05bbb8),
    (0x05BBBA, i_05bbba),
    (0x05BBBC, i_05bbbc),
    (0x05BBBE, i_05bbbe),
    (0x05BBC0, i_05bbc0),
    (0x05BBC2, i_05bbc2),
    (0x05BBC4, i_05bbc4),
    (0x05BBC6, i_05bbc6),
    (0x05BBC8, i_05bbc8),
    (0x05BBCA, i_05bbca),
    (0x05BBCC, i_05bbcc),
    (0x05BBCE, i_05bbce),
    (0x05BBD0, i_05bbd0),
    (0x05BBD2, i_05bbd2),
    (0x05BBD4, i_05bbd4),
    (0x05BBD6, i_05bbd6),
    (0x05BBD8, i_05bbd8),
    (0x05BBDA, i_05bbda),
    (0x05BBDC, i_05bbdc),
    (0x05BBDE, i_05bbde),
    (0x05BBE0, i_05bbe0),
    (0x05BBE2, i_05bbe2),
    (0x05BBE4, i_05bbe4),
    (0x05BBE6, i_05bbe6),
    (0x05BBE8, i_05bbe8),
    (0x05BBEA, i_05bbea),
    (0x05BBEC, i_05bbec),
    (0x05BBEE, i_05bbee),
    (0x05BBF0, i_05bbf0),
    (0x05BBF2, i_05bbf2),
    (0x05BBF4, i_05bbf4),
    (0x05BBF6, i_05bbf6),
    (0x05BBF8, i_05bbf8),
    (0x05BBFA, i_05bbfa),
    (0x05BBFC, i_05bbfc),
    (0x05BBFE, i_05bbfe),
    (0x05BC00, i_05bc00),
    (0x05BC02, i_05bc02),
    (0x05BC04, i_05bc04),
    (0x05BC06, i_05bc06),
    (0x05BC08, i_05bc08),
    (0x05BC0A, i_05bc0a),
    (0x05BC0C, i_05bc0c),
    (0x05BC0E, i_05bc0e),
    (0x05BC10, i_05bc10),
    (0x05BC12, i_05bc12),
    (0x05BC14, i_05bc14),
    (0x05BC16, i_05bc16),
    (0x05BC18, i_05bc18),
    (0x05BC1A, i_05bc1a),
    (0x05BC1C, i_05bc1c),
    (0x05BC1E, i_05bc1e),
    (0x05BC20, i_05bc20),
    (0x05BC22, i_05bc22),
    (0x05BC24, i_05bc24),
    (0x05BC26, i_05bc26),
    (0x05BC28, i_05bc28),
    (0x05BC2A, i_05bc2a),
    (0x05BC2C, i_05bc2c),
    (0x05BC2E, i_05bc2e),
    (0x05BC30, i_05bc30),
    (0x05BC32, i_05bc32),
    (0x05BC34, i_05bc34),
    (0x05BC36, i_05bc36),
    (0x05BC38, i_05bc38),
    (0x05BC3A, i_05bc3a),
    (0x05BC3C, i_05bc3c),
    (0x05BC3E, i_05bc3e),
    (0x05BC40, i_05bc40),
    (0x05BC42, i_05bc42),
    (0x05BC44, i_05bc44),
    (0x05BC46, i_05bc46),
    (0x05BC48, i_05bc48),
    (0x05BC4A, i_05bc4a),
    (0x05BC4C, i_05bc4c),
    (0x05BC50, i_05bc50),
    (0x05BC52, i_05bc52),
    (0x05BC54, i_05bc54),
    (0x05BC58, i_05bc58),
    (0x05BC5E, i_05bc5e),
    (0x05BC60, i_05bc60),
    (0x05BC62, i_05bc62),
    (0x05BC64, i_05bc64),
    (0x05BC68, i_05bc68),
    (0x05BC6A, i_05bc6a),
    (0x05BC6C, i_05bc6c),
    (0x05BC70, i_05bc70),
    (0x05BC72, i_05bc72),
    (0x05BC74, i_05bc74),
    (0x05BC76, i_05bc76),
    (0x05BC78, i_05bc78),
    (0x05BC7C, i_05bc7c),
    (0x05BC7E, i_05bc7e),
    (0x05BC80, i_05bc80),
    (0x05BC82, i_05bc82),
    (0x05BC84, i_05bc84),
    (0x05BC88, i_05bc88),
    (0x05BC8A, i_05bc8a),
    (0x05BC8E, i_05bc8e),
    (0x05BC90, i_05bc90),
    (0x05BC92, i_05bc92),
    (0x05BC96, i_05bc96),
    (0x05BC9A, i_05bc9a),
    (0x05BC9E, i_05bc9e),
    (0x05BCA2, i_05bca2),
    (0x05BCA6, i_05bca6),
    (0x05BCAA, i_05bcaa),
    (0x05BCAE, i_05bcae),
    (0x05BCB2, i_05bcb2),
    (0x05BCB6, i_05bcb6),
    (0x05BCBA, i_05bcba),
    (0x05BDEE, i_05bdee),
    (0x05BDF0, i_05bdf0),
    (0x05BDF2, i_05bdf2),
    (0x05BDF4, i_05bdf4),
    (0x05BDF6, i_05bdf6),
    (0x05BDF8, i_05bdf8),
    (0x05BDFA, i_05bdfa),
    (0x05BDFC, i_05bdfc),
    (0x05BDFE, i_05bdfe),
    (0x05BE00, i_05be00),
    (0x05BE02, i_05be02),
    (0x05BE04, i_05be04),
    (0x05BE06, i_05be06),
    (0x05BE08, i_05be08),
    (0x05BE0A, i_05be0a),
    (0x05BE0C, i_05be0c),
    (0x05BE0E, i_05be0e),
    (0x05BE10, i_05be10),
    (0x05BE12, i_05be12),
    (0x05BE14, i_05be14),
    (0x05BE16, i_05be16),
    (0x05BE18, i_05be18),
    (0x05BE1A, i_05be1a),
    (0x05BE1C, i_05be1c),
    (0x05BE1E, i_05be1e),
    (0x05BE20, i_05be20),
    (0x05BE22, i_05be22),
    (0x05BE24, i_05be24),
    (0x05BE26, i_05be26),
    (0x05BE28, i_05be28),
    (0x05BE2A, i_05be2a),
    (0x05BE2C, i_05be2c),
    (0x05BE2E, i_05be2e),
    (0x05BE30, i_05be30),
    (0x05BE32, i_05be32),
    (0x05BE34, i_05be34),
    (0x05BE36, i_05be36),
    (0x05BE38, i_05be38),
    (0x05BE3A, i_05be3a),
    (0x05BE3C, i_05be3c),
    (0x05BE3E, i_05be3e),
    (0x05BE40, i_05be40),
    (0x05BE42, i_05be42),
    (0x05BE44, i_05be44),
    (0x05BE46, i_05be46),
    (0x05BE48, i_05be48),
    (0x05BE4A, i_05be4a),
    (0x05BE4C, i_05be4c),
    (0x05BE4E, i_05be4e),
    (0x05BE50, i_05be50),
    (0x05BE52, i_05be52),
    (0x05BE54, i_05be54),
    (0x05BE56, i_05be56),
    (0x05BE58, i_05be58),
    (0x05BE5A, i_05be5a),
    (0x05BE5E, i_05be5e),
    (0x05BE60, i_05be60),
    (0x05BE62, i_05be62),
    (0x05BE64, i_05be64),
    (0x05BE66, i_05be66),
    (0x05BE68, i_05be68),
    (0x05BE6A, i_05be6a),
    (0x05BE6C, i_05be6c),
    (0x05BE6E, i_05be6e),
    (0x05BE70, i_05be70),
    (0x05BE72, i_05be72),
    (0x05BE74, i_05be74),
    (0x05BE76, i_05be76),
    (0x05BE78, i_05be78),
    (0x05BE7A, i_05be7a),
    (0x05BE7C, i_05be7c),
    (0x05BE7E, i_05be7e),
    (0x05BE80, i_05be80),
    (0x05BE82, i_05be82),
    (0x05BE84, i_05be84),
    (0x05BE86, i_05be86),
    (0x05BE88, i_05be88),
    (0x05BE8A, i_05be8a),
    (0x05BE8C, i_05be8c),
    (0x05BE8E, i_05be8e),
    (0x05BE90, i_05be90),
    (0x05BE92, i_05be92),
    (0x05BE94, i_05be94),
    (0x05BE96, i_05be96),
    (0x05BE98, i_05be98),
    (0x05BE9A, i_05be9a),
    (0x05BE9C, i_05be9c),
    (0x05BE9E, i_05be9e),
    (0x05BEA0, i_05bea0),
    (0x05BEA2, i_05bea2),
    (0x05BEA4, i_05bea4),
    (0x05BEA6, i_05bea6),
    (0x05BEA8, i_05bea8),
    (0x05BEAA, i_05beaa),
    (0x05BEAC, i_05beac),
    (0x05BEAE, i_05beae),
    (0x05BEB0, i_05beb0),
    (0x05BEB2, i_05beb2),
    (0x05BEB4, i_05beb4),
    (0x05BEB6, i_05beb6),
    (0x05BEB8, i_05beb8),
    (0x05BEBA, i_05beba),
    (0x05BEBC, i_05bebc),
    (0x05BEBE, i_05bebe),
    (0x05BEC0, i_05bec0),
    (0x05BEC2, i_05bec2),
    (0x05BEC4, i_05bec4),
    (0x05BEC6, i_05bec6),
    (0x05BEC8, i_05bec8),
    (0x05BECA, i_05beca),
    (0x05BECC, i_05becc),
    (0x05BECE, i_05bece),
    (0x05BED0, i_05bed0),
    (0x05BED2, i_05bed2),
    (0x05BED4, i_05bed4),
    (0x05BED6, i_05bed6),
    (0x05BED8, i_05bed8),
    (0x05BEDA, i_05beda),
    (0x05BEDC, i_05bedc),
    (0x05BEDE, i_05bede),
    (0x05BEE0, i_05bee0),
    (0x05BEE2, i_05bee2),
    (0x05BEE4, i_05bee4),
    (0x05BEE6, i_05bee6),
    (0x05BEE8, i_05bee8),
    (0x05BEEA, i_05beea),
    (0x05BEEC, i_05beec),
    (0x05BEEE, i_05beee),
    (0x05BEF0, i_05bef0),
    (0x05BEF2, i_05bef2),
    (0x05BEF4, i_05bef4),
    (0x05BEF6, i_05bef6),
    (0x05BEF8, i_05bef8),
    (0x05BEFA, i_05befa),
    (0x05BEFC, i_05befc),
    (0x05BEFE, i_05befe),
    (0x05BF00, i_05bf00),
    (0x05BF02, i_05bf02),
    (0x05BF04, i_05bf04),
    (0x05BF06, i_05bf06),
    (0x05BF08, i_05bf08),
    (0x05BF0A, i_05bf0a),
    (0x05BF0C, i_05bf0c),
    (0x05BF0E, i_05bf0e),
    (0x05BF10, i_05bf10),
    (0x05BF12, i_05bf12),
    (0x05BF14, i_05bf14),
    (0x05BF16, i_05bf16),
    (0x05BF18, i_05bf18),
    (0x05BF1A, i_05bf1a),
    (0x05BF1C, i_05bf1c),
    (0x05BF1E, i_05bf1e),
    (0x05BF20, i_05bf20),
    (0x05BF22, i_05bf22),
    (0x05BF24, i_05bf24),
    (0x05BF26, i_05bf26),
    (0x05BF28, i_05bf28),
    (0x05BF2A, i_05bf2a),
    (0x05BF2C, i_05bf2c),
    (0x05BF2E, i_05bf2e),
    (0x05BF30, i_05bf30),
    (0x05BF32, i_05bf32),
    (0x05BF34, i_05bf34),
    (0x05BF36, i_05bf36),
    (0x05BF38, i_05bf38),
    (0x05BF3A, i_05bf3a),
    (0x05BF3C, i_05bf3c),
    (0x05BF3E, i_05bf3e),
    (0x05BF40, i_05bf40),
    (0x05BF42, i_05bf42),
    (0x05BF44, i_05bf44),
    (0x05BF46, i_05bf46),
    (0x05BF48, i_05bf48),
    (0x05BF4A, i_05bf4a),
    (0x05BF4C, i_05bf4c),
    (0x05BF4E, i_05bf4e),
    (0x05BF50, i_05bf50),
    (0x05BF52, i_05bf52),
    (0x05BF54, i_05bf54),
    (0x05BF56, i_05bf56),
    (0x05BF5A, i_05bf5a),
    (0x05BF5C, i_05bf5c),
    (0x05BF5E, i_05bf5e),
    (0x05BF62, i_05bf62),
    (0x05BF68, i_05bf68),
    (0x05BF6A, i_05bf6a),
    (0x05BF6C, i_05bf6c),
    (0x05BF6E, i_05bf6e),
    (0x05BF72, i_05bf72),
    (0x05BF74, i_05bf74),
    (0x05BF76, i_05bf76),
    (0x05BF7A, i_05bf7a),
    (0x05BF7C, i_05bf7c),
    (0x05BF7E, i_05bf7e),
    (0x05BF82, i_05bf82),
    (0x05BF84, i_05bf84),
    (0x05BF88, i_05bf88),
    (0x05BF8A, i_05bf8a),
    (0x05C9D6, i_05c9d6),
    (0x05C9DC, i_05c9dc),
    (0x05C9E0, i_05c9e0),
    (0x05C9E6, i_05c9e6),
    (0x05C9EC, i_05c9ec),
    (0x05C9F2, i_05c9f2),
    (0x05C9F6, i_05c9f6),
    (0x05C9FA, i_05c9fa),
    (0x05C9FE, i_05c9fe),
    (0x05CA00, i_05ca00),
    (0x05CA02, i_05ca02),
    (0x05CA06, i_05ca06),
    (0x05CA0C, i_05ca0c),
    (0x05CA10, i_05ca10),
    (0x05CA14, i_05ca14),
    (0x05CA18, i_05ca18),
    (0x05CA1C, i_05ca1c),
    (0x05CA1E, i_05ca1e),
    (0x05CA22, i_05ca22),
    (0x05CA28, i_05ca28),
    (0x05CA2A, i_05ca2a),
    (0x05CA2E, i_05ca2e),
    (0x05CA32, i_05ca32),
    (0x05CA36, i_05ca36),
    (0x05CA3A, i_05ca3a),
    (0x05CA3E, i_05ca3e),
    (0x05CA42, i_05ca42),
    (0x05CA46, i_05ca46),
    (0x05CA4A, i_05ca4a),
    (0x05CACE, i_05cace),
    (0x05CAD6, i_05cad6),
    (0x05CADE, i_05cade),
    (0x05CC0E, i_05cc0e),
    (0x05CC14, i_05cc14),
    (0x05CC1A, i_05cc1a),
    (0x05CC20, i_05cc20),
    (0x05CC26, i_05cc26),
    (0x05CC2C, i_05cc2c),
    (0x05CC2E, i_05cc2e),
    (0x05CC30, i_05cc30),
    (0x05CC36, i_05cc36),
    (0x05CC3C, i_05cc3c),
    (0x05CC3E, i_05cc3e),
    (0x05CC40, i_05cc40),
    (0x05CC46, i_05cc46),
    (0x05CC4C, i_05cc4c),
    (0x05CC4E, i_05cc4e),
    (0x05CC50, i_05cc50),
    (0x05CC52, i_05cc52),
    (0x05CC54, i_05cc54),
    (0x05CC56, i_05cc56),
    (0x05CC5A, i_05cc5a),
    (0x05CC60, i_05cc60),
    (0x05CC64, i_05cc64),
    (0x05CC68, i_05cc68),
    (0x05CC6C, i_05cc6c),
    (0x05CC70, i_05cc70),
    (0x05CC76, i_05cc76),
    (0x05CC7C, i_05cc7c),
    (0x05CC7E, i_05cc7e),
    (0x05CC80, i_05cc80),
    (0x05CC84, i_05cc84),
    (0x05CC8A, i_05cc8a),
    (0x05CC8C, i_05cc8c),
    (0x05CC8E, i_05cc8e),
    (0x05CC94, i_05cc94),
    (0x05CC9A, i_05cc9a),
    (0x05CC9E, i_05cc9e),
    (0x05CCA4, i_05cca4),
    (0x05CCA8, i_05cca8),
    (0x05CCAE, i_05ccae),
    (0x05CCB4, i_05ccb4),
    (0x05CCB6, i_05ccb6),
    (0x05CCB8, i_05ccb8),
    (0x05CCBC, i_05ccbc),
    (0x05CCC2, i_05ccc2),
    (0x05CCC4, i_05ccc4),
    (0x05CCC6, i_05ccc6),
    (0x05CCC8, i_05ccc8),
    (0x05CCCE, i_05ccce),
    (0x05CD18, i_05cd18),
    (0x05CD1C, i_05cd1c),
    (0x05CD20, i_05cd20),
    (0x05CD26, i_05cd26),
    (0x05CD6C, i_05cd6c),
    (0x05CD70, i_05cd70),
    (0x05CD74, i_05cd74),
    (0x05CDA8, i_05cda8),
    (0x05CDAC, i_05cdac),
    (0x05CDB0, i_05cdb0),
    (0x05CDB4, i_05cdb4),
    (0x05CDB8, i_05cdb8),
    (0x05CDBC, i_05cdbc),
    (0x05CDC0, i_05cdc0),
    (0x05CDC4, i_05cdc4),
    (0x05CDC8, i_05cdc8),
    (0x05CDE4, i_05cde4),
    (0x05CDE8, i_05cde8),
    (0x05CDEC, i_05cdec),
    (0x05CDFC, i_05cdfc),
    (0x05CE00, i_05ce00),
    (0x05CE04, i_05ce04),
    (0x05CEEC, i_05ceec),
    (0x05CEF0, i_05cef0),
    (0x05CEF4, i_05cef4),
    (0x05CEF8, i_05cef8),
    (0x05CEFC, i_05cefc),
    (0x05CF00, i_05cf00),
    (0x05CF04, i_05cf04),
    (0x05CF08, i_05cf08),
    (0x05CF0C, i_05cf0c),
    (0x05CF10, i_05cf10),
    (0x05CF14, i_05cf14),
    (0x05CF18, i_05cf18),
    (0x05CF1C, i_05cf1c),
    (0x05CF20, i_05cf20),
    (0x05CF24, i_05cf24),
    (0x05CF28, i_05cf28),
    (0x05CFA8, i_05cfa8),
    (0x05CFB0, i_05cfb0),
    (0x05CFB4, i_05cfb4),
    (0x05CFBA, i_05cfba),
    (0x05CFBE, i_05cfbe),
    (0x05CFC2, i_05cfc2),
    (0x05CFC6, i_05cfc6),
    (0x05CFC8, i_05cfc8),
    (0x05CFCC, i_05cfcc),
    (0x05CFD2, i_05cfd2),
    (0x05CFD6, i_05cfd6),
    (0x05CFDC, i_05cfdc),
    (0x05CFE0, i_05cfe0),
    (0x05CFE6, i_05cfe6),
    (0x05CFEA, i_05cfea),
    (0x05CFF0, i_05cff0),
    (0x05CFF4, i_05cff4),
    (0x05CFF8, i_05cff8),
    (0x05CFFC, i_05cffc),
    (0x05CFFE, i_05cffe),
    (0x05D002, i_05d002),
    (0x05D006, i_05d006),
    (0x05D008, i_05d008),
    (0x05D00C, i_05d00c),
    (0x05D00E, i_05d00e),
    (0x05D012, i_05d012),
    (0x05D016, i_05d016),
    (0x05D032, i_05d032),
    (0x05D03A, i_05d03a),
    (0x05D03E, i_05d03e),
    (0x05D044, i_05d044),
    (0x05D048, i_05d048),
    (0x05D04C, i_05d04c),
    (0x05D050, i_05d050),
    (0x05D052, i_05d052),
    (0x05D056, i_05d056),
    (0x05D05C, i_05d05c),
    (0x05D060, i_05d060),
    (0x05D066, i_05d066),
    (0x05D06A, i_05d06a),
    (0x05D070, i_05d070),
    (0x05D074, i_05d074),
    (0x05D07A, i_05d07a),
    (0x05D07E, i_05d07e),
    (0x05D082, i_05d082),
    (0x05D086, i_05d086),
    (0x05D088, i_05d088),
    (0x05D08A, i_05d08a),
    (0x05D08E, i_05d08e),
    (0x05D092, i_05d092),
    (0x05D094, i_05d094),
    (0x05D098, i_05d098),
    (0x05D09A, i_05d09a),
    (0x05D09E, i_05d09e),
    (0x05D0A2, i_05d0a2),
    (0x05D0A8, i_05d0a8),
    (0x05D0AC, i_05d0ac),
    (0x05D0B0, i_05d0b0),
    (0x05D0B4, i_05d0b4),
    (0x05D0BA, i_05d0ba),
    (0x05D0E2, i_05d0e2),
    (0x05D0E6, i_05d0e6),
    (0x05D0E8, i_05d0e8),
    (0x05D0EC, i_05d0ec),
    (0x05D0F0, i_05d0f0),
    (0x05D0F2, i_05d0f2),
    (0x05D0F6, i_05d0f6),
    (0x05D11C, i_05d11c),
    (0x05D120, i_05d120),
    (0x05D124, i_05d124),
    (0x05D12A, i_05d12a),
    (0x05D1C0, i_05d1c0),
    (0x05D1C4, i_05d1c4),
    (0x05D1C8, i_05d1c8),
    (0x05D1CA, i_05d1ca),
    (0x05D1CE, i_05d1ce),
    (0x05D1D2, i_05d1d2),
    (0x05D1D4, i_05d1d4),
    (0x05D1D8, i_05d1d8),
    (0x05D288, i_05d288),
    (0x05D28C, i_05d28c),
    (0x05D290, i_05d290),
    (0x05D296, i_05d296),
    (0x05D29A, i_05d29a),
    (0x05D29E, i_05d29e),
    (0x05D2A2, i_05d2a2),
    (0x05D2A6, i_05d2a6),
    (0x05D2AA, i_05d2aa),
    (0x05D2B0, i_05d2b0),
    (0x05D2EE, i_05d2ee),
    (0x05D2F2, i_05d2f2),
    (0x05D2F6, i_05d2f6),
    (0x05D5B6, i_05d5b6),
    (0x05D5BA, i_05d5ba),
    (0x05D5BE, i_05d5be),
    (0x05D5C4, i_05d5c4),
    (0x05D5C8, i_05d5c8),
    (0x05D5CE, i_05d5ce),
    (0x05D5D2, i_05d5d2),
    (0x05D5D8, i_05d5d8),
    (0x05D5DC, i_05d5dc),
    (0x05D5E2, i_05d5e2),
    (0x05D5E6, i_05d5e6),
    (0x05D5E8, i_05d5e8),
    (0x05D5F0, i_05d5f0),
    (0x05D5F4, i_05d5f4),
    (0x05D5FA, i_05d5fa),
    (0x05D5FE, i_05d5fe),
    (0x05D602, i_05d602),
    (0x05D604, i_05d604),
    (0x05D606, i_05d606),
    (0x05D60A, i_05d60a),
    (0x05D60E, i_05d60e),
    (0x05D612, i_05d612),
    (0x05D614, i_05d614),
    (0x05D674, i_05d674),
    (0x05D678, i_05d678),
    (0x05D67E, i_05d67e),
    (0x05D682, i_05d682),
    (0x05D684, i_05d684),
    (0x05D68A, i_05d68a),
    (0x05D690, i_05d690),
    (0x05D694, i_05d694),
    (0x05D69A, i_05d69a),
    (0x05D69E, i_05d69e),
    (0x05D6C2, i_05d6c2),
    (0x05D6C8, i_05d6c8),
    (0x05D6CA, i_05d6ca),
    (0x05D6CE, i_05d6ce),
    (0x05D6D0, i_05d6d0),
    (0x05D6D4, i_05d6d4),
    (0x05D6D6, i_05d6d6),
    (0x05D6D8, i_05d6d8),
    (0x05D6DA, i_05d6da),
    (0x05D6E0, i_05d6e0),
    (0x05D6E2, i_05d6e2),
    (0x05D6E4, i_05d6e4),
    (0x05D6E6, i_05d6e6),
    (0x05D6E8, i_05d6e8),
    (0x05D6EA, i_05d6ea),
    (0x05D6F0, i_05d6f0),
    (0x05D6F2, i_05d6f2),
    (0x05D6F4, i_05d6f4),
    (0x05D6F6, i_05d6f6),
    (0x05D6FC, i_05d6fc),
    (0x05D6FE, i_05d6fe),
    (0x05D700, i_05d700),
    (0x05D704, i_05d704),
    (0x05D708, i_05d708),
    (0x05D70A, i_05d70a),
    (0x05D712, i_05d712),
    (0x05D716, i_05d716),
    (0x05D71A, i_05d71a),
    (0x05D8F2, i_05d8f2),
    (0x05D8F8, i_05d8f8),
    (0x05D8FC, i_05d8fc),
    (0x05D902, i_05d902),
    (0x05D904, i_05d904),
    (0x05D906, i_05d906),
    (0x05D908, i_05d908),
    (0x05D90A, i_05d90a),
    (0x05D90E, i_05d90e),
    (0x05D910, i_05d910),
    (0x05D914, i_05d914),
    (0x05D916, i_05d916),
    (0x05D918, i_05d918),
    (0x05D91A, i_05d91a),
    (0x05D91C, i_05d91c),
    (0x05D91E, i_05d91e),
    (0x05D920, i_05d920),
    (0x05D922, i_05d922),
    (0x05D924, i_05d924),
    (0x05D928, i_05d928),
    (0x05D92A, i_05d92a),
    (0x05D92E, i_05d92e),
    (0x05D930, i_05d930),
    (0x05D932, i_05d932),
    (0x05D934, i_05d934),
    (0x05D936, i_05d936),
    (0x05D93A, i_05d93a),
    (0x05D93C, i_05d93c),
    (0x05D93E, i_05d93e),
    (0x05D942, i_05d942),
    (0x05D944, i_05d944),
    (0x05DA56, i_05da56),
    (0x05DA5A, i_05da5a),
    (0x05DA5C, i_05da5c),
    (0x05DA5E, i_05da5e),
    (0x05DA60, i_05da60),
    (0x05DA62, i_05da62),
    (0x05DA64, i_05da64),
    (0x05DA66, i_05da66),
    (0x05DA6A, i_05da6a),
    (0x05DA6E, i_05da6e),
    (0x05DA72, i_05da72),
    (0x05DA76, i_05da76),
    (0x05DA7E, i_05da7e),
    (0x05DA80, i_05da80),
    (0x05DA84, i_05da84),
    (0x05DA88, i_05da88),
    (0x05DA8A, i_05da8a),
    (0x05DA8E, i_05da8e),
    (0x05DA92, i_05da92),
    (0x05DA96, i_05da96),
    (0x05DA9A, i_05da9a),
    (0x05DA9C, i_05da9c),
    (0x05DAA0, i_05daa0),
    (0x05DAA2, i_05daa2),
    (0x05DAA4, i_05daa4),
    (0x05DAA6, i_05daa6),
    (0x05DAA8, i_05daa8),
    (0x05DAAA, i_05daaa),
    (0x05DAAE, i_05daae),
    (0x05DAB2, i_05dab2),
    (0x05DAB6, i_05dab6),
    (0x05DABA, i_05daba),
    (0x05DAC2, i_05dac2),
    (0x05DAC6, i_05dac6),
    (0x05DACA, i_05daca),
    (0x05DACE, i_05dace),
    (0x05DAD2, i_05dad2),
    (0x05DAD6, i_05dad6),
    (0x05DAD8, i_05dad8),
    (0x05DADA, i_05dada),
    (0x05DADC, i_05dadc),
    (0x05DADE, i_05dade),
    (0x05DAE0, i_05dae0),
    (0x05DAE4, i_05dae4),
    (0x05DAE8, i_05dae8),
    (0x05DAEC, i_05daec),
    (0x05DAF0, i_05daf0),
    (0x05DAF4, i_05daf4),
    (0x05DAF8, i_05daf8),
    (0x05DAFC, i_05dafc),
    (0x05DB00, i_05db00),
    (0x05DB02, i_05db02),
    (0x05DB0A, i_05db0a),
    (0x05DB0E, i_05db0e),
    (0x05DB10, i_05db10),
    (0x05DB12, i_05db12),
    (0x05DB14, i_05db14),
    (0x05DB16, i_05db16),
    (0x05DB18, i_05db18),
    (0x05DB58, i_05db58),
    (0x05DB5C, i_05db5c),
    (0x05DB60, i_05db60),
    (0x05DB64, i_05db64),
    (0x05DB68, i_05db68),
    (0x05DB6A, i_05db6a),
    (0x05DB6E, i_05db6e),
    (0x05DB72, i_05db72),
    (0x05DB76, i_05db76),
    (0x05DB7A, i_05db7a),
    (0x05DB80, i_05db80),
    (0x05DB84, i_05db84),
    (0x05DB88, i_05db88),
    (0x05DB8C, i_05db8c),
    (0x05DB90, i_05db90),
    (0x05DB94, i_05db94),
    (0x05DB98, i_05db98),
    (0x05DB9C, i_05db9c),
    (0x05DBA0, i_05dba0),
    (0x05DBA4, i_05dba4),
    (0x05DBA6, i_05dba6),
    (0x05DBAA, i_05dbaa),
    (0x05DBAE, i_05dbae),
    (0x05DBB2, i_05dbb2),
    (0x05DBB6, i_05dbb6),
    (0x05DBBA, i_05dbba),
    (0x05DBBC, i_05dbbc),
    (0x05DBC0, i_05dbc0),
    (0x05DBC2, i_05dbc2),
    (0x05DBC6, i_05dbc6),
    (0x05DBCA, i_05dbca),
    (0x05DBCC, i_05dbcc),
    (0x05DBD0, i_05dbd0),
    (0x05DBD4, i_05dbd4),
    (0x05DBDA, i_05dbda),
    (0x05DBDC, i_05dbdc),
    (0x05DBE0, i_05dbe0),
    (0x05DBE4, i_05dbe4),
    (0x05DBE8, i_05dbe8),
    (0x05DBEC, i_05dbec),
    (0x05DBF0, i_05dbf0),
    (0x05DBF4, i_05dbf4),
    (0x05DBF8, i_05dbf8),
    (0x05DBFE, i_05dbfe),
    (0x05DC1C, i_05dc1c),
    (0x05DC20, i_05dc20),
    (0x05DC28, i_05dc28),
    (0x05DC2C, i_05dc2c),
    (0x05DC30, i_05dc30),
    (0x05DC32, i_05dc32),
    (0x05DC34, i_05dc34),
    (0x05DC38, i_05dc38),
    (0x05DC3E, i_05dc3e),
    (0x05DC44, i_05dc44),
    (0x05DC4A, i_05dc4a),
    (0x05DC50, i_05dc50),
    (0x05DC54, i_05dc54),
    (0x05DC58, i_05dc58),
    (0x05DC5C, i_05dc5c),
    (0x05DC60, i_05dc60),
    (0x05DC64, i_05dc64),
    (0x05DC68, i_05dc68),
    (0x05DC6C, i_05dc6c),
    (0x05DC70, i_05dc70),
    (0x05DC76, i_05dc76),
    (0x05DC7C, i_05dc7c),
    (0x05DC80, i_05dc80),
    (0x05DC84, i_05dc84),
    (0x05DC88, i_05dc88),
    (0x05DC8C, i_05dc8c),
    (0x05DC90, i_05dc90),
    (0x05DC94, i_05dc94),
    (0x05DC98, i_05dc98),
    (0x05DC9C, i_05dc9c),
    (0x05DCA2, i_05dca2),
    (0x05DCA4, i_05dca4),
    (0x05DCAA, i_05dcaa),
    (0x05DCAE, i_05dcae),
    (0x05DCB0, i_05dcb0),
    (0x05DCB4, i_05dcb4),
    (0x05DCCE, i_05dcce),
    (0x05DCD2, i_05dcd2),
    (0x05DCD6, i_05dcd6),
    (0x05DCDA, i_05dcda),
    (0x05DCE0, i_05dce0),
    (0x05DCE4, i_05dce4),
    (0x05DCEA, i_05dcea),
    (0x05DCEE, i_05dcee),
    (0x05DCF2, i_05dcf2),
    (0x05DCF8, i_05dcf8),
    (0x05DCFC, i_05dcfc),
    (0x05DD00, i_05dd00),
    (0x05DD02, i_05dd02),
    (0x05DD08, i_05dd08),
    (0x05DD0E, i_05dd0e),
    (0x05DD14, i_05dd14),
    (0x05DD1A, i_05dd1a),
    (0x05DD20, i_05dd20),
    (0x05DD56, i_05dd56),
    (0x05DD5C, i_05dd5c),
    (0x05DD62, i_05dd62),
    (0x05DD66, i_05dd66),
    (0x05DD6A, i_05dd6a),
    (0x05DD70, i_05dd70),
    (0x05DD74, i_05dd74),
    (0x05DD78, i_05dd78),
    (0x05DD7C, i_05dd7c),
];
