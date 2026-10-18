// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p02_6) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $02F152: move.l a1, (a6)
pub fn i_02f152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F154;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F154: cmpi.b #$29, $92(a6)
pub fn i_02f154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F15A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x92);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x29, Size::Byte, false);
    16
}
/// $02F15A: bhi.w $2f16e
pub fn i_02f15a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F15E;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x2F16E; return 10; }
    8
}
/// $02F15E: move.w #$600, $28(a6)
pub fn i_02f15e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F164;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x600;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02F164: clr.w $2c(a6)
pub fn i_02f164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F168;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02F168: lea.l $2f16e(pc), a1
pub fn i_02f168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F16C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2F16E;
    16
}
/// $02F16C: move.l a1, (a6)
pub fn i_02f16c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F16E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F16E: jsr $2aa24(pc)
pub fn i_02f16e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F172;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F172);
    cpu.pc = 0x2AA24;
    16
}
/// $02F172: bset.b #$4, $8d(a6)
pub fn i_02f172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F178;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02F178: jsr $28992.l
pub fn i_02f178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F17E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2F17E);
    cpu.pc = 0x28992;
    16
}
/// $02F17E: jsr $2a7d8(pc)
pub fn i_02f17e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F182;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F182);
    cpu.pc = 0x2A7D8;
    16
}
/// $02F182: bcc.w $2f18c
pub fn i_02f182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F186;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2F18C; return 10; }
    8
}
/// $02F186: lea.l $2fc8e(pc), a1
pub fn i_02f186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F18A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2FC8E;
    16
}
/// $02F18A: move.l a1, (a6)
pub fn i_02f18a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F18C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F18C: jsr $28d70.l
pub fn i_02f18c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F192;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2F192);
    cpu.pc = 0x28D70;
    16
}
/// $02F192: move.b $106f28.l, d0
pub fn i_02f192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F198;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $02F198: andi.b #$1, d0
pub fn i_02f198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F19C;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $02F19C: bne.w $2f1b4
pub fn i_02f19c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F1A0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2F1B4; return 10; }
    8
}
/// $02F1A0: lea.l $29834(pc), a0
pub fn i_02f1a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F1A4;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x29834;
    16
}
/// $02F1A4: move.l a0, $4c(a6)
pub fn i_02f1a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F1A8;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02F1A8: jsr $283ca.l
pub fn i_02f1a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F1AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2F1AE);
    cpu.pc = 0x283CA;
    16
}
/// $02F1AE: jsr $283ca.l
pub fn i_02f1ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F1B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2F1B4);
    cpu.pc = 0x283CA;
    16
}
/// $02F1B4: jsr $283d8.l
pub fn i_02f1b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F1BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2F1BA);
    cpu.pc = 0x283D8;
    16
}
/// $02F1BA: jsr $2fb04(pc)
pub fn i_02f1ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F1BE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F1BE);
    cpu.pc = 0x2FB04;
    16
}
/// $02F1BE: bcc.w $2f1c8
pub fn i_02f1be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F1C2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2F1C8; return 10; }
    8
}
/// $02F1C2: lea.l $2fe6a(pc), a1
pub fn i_02f1c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F1C6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2FE6A;
    16
}
/// $02F1C6: move.l a1, (a6)
pub fn i_02f1c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F1C8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F1C8: jsr $2ffb0(pc)
pub fn i_02f1c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F1CC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F1CC);
    cpu.pc = 0x2FFB0;
    16
}
/// $02F1CC: tst.b $92(a6)
pub fn i_02f1cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F1D0;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x92);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $02F1D0: beq.w $2f1da
pub fn i_02f1d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F1D4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2F1DA; return 10; }
    8
}
/// $02F1D4: subi.b #$1, $92(a6)
pub fn i_02f1d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F1DA;
    cpu.ir = 0x42E;
    let ad1 = cpu.a[6].wrapping_add(0x92);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $02F1DA: bhi.w $2f1e4
pub fn i_02f1da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F1DE;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x2F1E4; return 10; }
    8
}
/// $02F1DE: lea.l $2fe6a(pc), a1
pub fn i_02f1de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F1E2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2FE6A;
    16
}
/// $02F1E2: move.l a1, (a6)
pub fn i_02f1e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F1E4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F1E4: cmpi.w #$117, $22(a6)
pub fn i_02f1e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F1EA;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x117, Size::Word, false);
    16
}
/// $02F1EA: blt.w $2f1f4
pub fn i_02f1ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F1EE;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x2F1F4; return 10; }
    8
}
/// $02F1EE: lea.l $2fe6a(pc), a1
pub fn i_02f1ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F1F2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2FE6A;
    16
}
/// $02F1F2: move.l a1, (a6)
pub fn i_02f1f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F1F4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F1F4: clr.w $2a(a6)
pub fn i_02f1f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F1F8;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02F1F8: clr.w $2e(a6)
pub fn i_02f1f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F1FC;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
// ===== JsrPcThunk_02f1fc  ($02F1FC, 6 B, thunks/jsr_pc_thunks.c)
/// $02F1FC: jsr $2ac80(pc)
pub fn i_02f1fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F200;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F200);
    cpu.pc = 0x2AC80;
    16
}
/// $02F200: rts
pub fn i_02f200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F202;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_DecelA_02f202  ($02F202, 264 B, slug/vehicle_02dd20.s)
/// $02F202: clr.w d0
pub fn i_02f202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F204;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02F204: sub.w $28(a6), d0
pub fn i_02f204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F208;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $02F208: asr.w #$3, d0
pub fn i_02f208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F20A;
    cpu.ir = 0xE640;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $02F20A: move.w d0, $2c(a6)
pub fn i_02f20a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F20E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02F20E: move.b #$8, $91(a6)
pub fn i_02f20e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F214;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x91);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02F214: jsr $2a9a0(pc)
pub fn i_02f214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F218;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F218);
    cpu.pc = 0x2A9A0;
    16
}
/// $02F218: movea.l #$2793bc, a0
pub fn i_02f218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F21E;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2793BC;
    cpu.a[0] = v1;
    4
}
/// $02F21E: lsl.w #$2, d0
pub fn i_02f21e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F220;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02F220: movea.l (a0, d0.w), a0
pub fn i_02f220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F224;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02F224: cmpa.l #$ffffffff, a0
pub fn i_02f224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F22A;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02F22A: beq.w $2f234
pub fn i_02f22a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F22E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2F234; return 10; }
    8
}
/// $02F22E: jsr $28cd4.l
pub fn i_02f22e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F234;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2F234);
    cpu.pc = 0x28CD4;
    16
}
/// $02F234: move.w #$0, d0
pub fn i_02f234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F238;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02F238: move.b #$96, $20(a6)
pub fn i_02f238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F23E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x96;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02F23E: andi.w #$ff, d0
pub fn i_02f23e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F242;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02F242: lsr.w #$2, d0
pub fn i_02f242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F244;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02F244: add.b d0, $20(a6)
pub fn i_02f244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F248;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02F248: lea.l $2f24e(pc), a1
pub fn i_02f248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F24C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2F24E;
    16
}
/// $02F24C: move.l a1, (a6)
pub fn i_02f24c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F24E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F24E: subq.b #$1, $91(a6)
pub fn i_02f24e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F252;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x91);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02F252: bne.w $2f264
pub fn i_02f252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F256;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2F264; return 10; }
    8
}
/// $02F256: clr.w $28(a6)
pub fn i_02f256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F25A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02F25A: clr.w $2c(a6)
pub fn i_02f25a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F25E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02F25E: lea.l $2f264(pc), a1
pub fn i_02f25e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F262;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2F264;
    16
}
/// $02F262: move.l a1, (a6)
pub fn i_02f262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F264;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F264: jsr $2aa24(pc)
pub fn i_02f264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F268;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F268);
    cpu.pc = 0x2AA24;
    16
}
/// $02F268: bset.b #$4, $8d(a6)
pub fn i_02f268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F26E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02F26E: jsr $28992.l
pub fn i_02f26e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F274;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2F274);
    cpu.pc = 0x28992;
    16
}
/// $02F274: jsr $2a824(pc)
pub fn i_02f274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F278;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F278);
    cpu.pc = 0x2A824;
    16
}
/// $02F278: bcc.w $2f282
pub fn i_02f278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F27C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2F282; return 10; }
    8
}
/// $02F27C: lea.l $2eca6(pc), a1
pub fn i_02f27c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F280;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2ECA6;
    16
}
/// $02F280: move.l a1, (a6)
pub fn i_02f280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F282;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F282: jsr $2a4f0(pc)
pub fn i_02f282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F286;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F286);
    cpu.pc = 0x2A4F0;
    16
}
/// $02F286: bcc.w $2f290
pub fn i_02f286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F28A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2F290; return 10; }
    8
}
/// $02F28A: lea.l $2eca6(pc), a1
pub fn i_02f28a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F28E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2ECA6;
    16
}
/// $02F28E: move.l a1, (a6)
pub fn i_02f28e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F290;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F290: jsr $5cef8.l
pub fn i_02f290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F296;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2F296);
    cpu.pc = 0x5CEF8;
    16
}
/// $02F296: bcs.w $2f2a0
pub fn i_02f296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F29A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2F2A0; return 10; }
    8
}
/// $02F29A: lea.l $2e9c8(pc), a1
pub fn i_02f29a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F29E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2E9C8;
    16
}
/// $02F29E: move.l a1, (a6)
pub fn i_02f29e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F2A0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F2A0: jsr $28d70.l
pub fn i_02f2a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F2A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2F2A6);
    cpu.pc = 0x28D70;
    16
}
/// $02F2A6: bcc.w $2f2b0
pub fn i_02f2a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F2AA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2F2B0; return 10; }
    8
}
/// $02F2AA: lea.l $2ebb0(pc), a1
pub fn i_02f2aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F2AE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2EBB0;
    16
}
/// $02F2AE: move.l a1, (a6)
pub fn i_02f2ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F2B0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F2B0: jsr $283d8.l
pub fn i_02f2b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F2B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2F2B6);
    cpu.pc = 0x283D8;
    16
}
/// $02F2B6: jsr $2a328(pc)
pub fn i_02f2b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F2BA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F2BA);
    cpu.pc = 0x2A328;
    16
}
/// $02F2BA: bcc.w $2f2c4
pub fn i_02f2ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F2BE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2F2C4; return 10; }
    8
}
/// $02F2BE: lea.l $2f422(pc), a1
pub fn i_02f2be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F2C2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2F422;
    16
}
/// $02F2C2: move.l a1, (a6)
pub fn i_02f2c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F2C4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F2C4: jsr $2a34e(pc)
pub fn i_02f2c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F2C8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F2C8);
    cpu.pc = 0x2A34E;
    16
}
/// $02F2C8: bcc.w $2f2d2
pub fn i_02f2c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F2CC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2F2D2; return 10; }
    8
}
/// $02F2CC: lea.l $2f4fc(pc), a1
pub fn i_02f2cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F2D0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2F4FC;
    16
}
/// $02F2D0: move.l a1, (a6)
pub fn i_02f2d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F2D2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F2D2: jsr $2a8c0(pc)
pub fn i_02f2d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F2D6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F2D6);
    cpu.pc = 0x2A8C0;
    16
}
/// $02F2D6: bcc.w $2f2e0
pub fn i_02f2d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F2DA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2F2E0; return 10; }
    8
}
/// $02F2DA: lea.l $2d02e(pc), a1
pub fn i_02f2da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F2DE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D02E;
    16
}
/// $02F2DE: move.l a1, (a6)
pub fn i_02f2de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F2E0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F2E0: jsr $2ac80(pc)
pub fn i_02f2e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F2E4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F2E4);
    cpu.pc = 0x2AC80;
    16
}
/// $02F2E4: bcc.w $2f2ee
pub fn i_02f2e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F2E8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2F2EE; return 10; }
    8
}
/// $02F2E8: lea.l $2dd74(pc), a1
pub fn i_02f2e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F2EC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DD74;
    16
}
/// $02F2EC: move.l a1, (a6)
pub fn i_02f2ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F2EE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F2EE: jsr $2a5cc(pc)
pub fn i_02f2ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F2F2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F2F2);
    cpu.pc = 0x2A5CC;
    16
}
/// $02F2F2: bcc.w $2f2fc
pub fn i_02f2f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F2F6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2F2FC; return 10; }
    8
}
/// $02F2F6: lea.l $2f0e6(pc), a1
pub fn i_02f2f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F2FA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2F0E6;
    16
}
/// $02F2FA: move.l a1, (a6)
pub fn i_02f2fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F2FC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F2FC: jsr $2a690(pc)
pub fn i_02f2fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F300;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F300);
    cpu.pc = 0x2A690;
    16
}
/// $02F300: lea.l $2a060(pc), a0
pub fn i_02f300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F304;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A060;
    16
}
/// $02F304: movea.l #$ffffffff, a1
pub fn i_02f304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F30A;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
// ===== JsrAbsThunk_02f30a  ($02F30A, 8 B, thunks/jsr_abs_thunks.c)
/// $02F30A: jsr $772.l
pub fn i_02f30a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F310;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2F310);
    cpu.pc = 0x772;
    16
}
/// $02F310: rts
pub fn i_02f310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F312;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_DecelB_02f312  ($02F312, 264 B, slug/vehicle_02dd20.s)
/// $02F312: clr.w d0
pub fn i_02f312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F314;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02F314: sub.w $28(a6), d0
pub fn i_02f314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F318;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $02F318: asr.w #$3, d0
pub fn i_02f318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F31A;
    cpu.ir = 0xE640;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $02F31A: move.w d0, $2c(a6)
pub fn i_02f31a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F31E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02F31E: move.b #$8, $91(a6)
pub fn i_02f31e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F324;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x91);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02F324: jsr $2a9a0(pc)
pub fn i_02f324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F328;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F328);
    cpu.pc = 0x2A9A0;
    16
}
/// $02F328: movea.l #$2793f8, a0
pub fn i_02f328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F32E;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2793F8;
    cpu.a[0] = v1;
    4
}
/// $02F32E: lsl.w #$2, d0
pub fn i_02f32e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F330;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02F330: movea.l (a0, d0.w), a0
pub fn i_02f330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F334;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02F334: cmpa.l #$ffffffff, a0
pub fn i_02f334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F33A;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02F33A: beq.w $2f344
pub fn i_02f33a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F33E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2F344; return 10; }
    8
}
/// $02F33E: jsr $28cd4.l
pub fn i_02f33e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F344;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2F344);
    cpu.pc = 0x28CD4;
    16
}
/// $02F344: move.w #$0, d0
pub fn i_02f344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F348;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02F348: move.b #$96, $20(a6)
pub fn i_02f348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F34E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x96;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02F34E: andi.w #$ff, d0
pub fn i_02f34e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F352;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02F352: lsr.w #$2, d0
pub fn i_02f352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F354;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02F354: add.b d0, $20(a6)
pub fn i_02f354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F358;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02F358: lea.l $2f35e(pc), a1
pub fn i_02f358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F35C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2F35E;
    16
}
/// $02F35C: move.l a1, (a6)
pub fn i_02f35c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F35E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F35E: subq.b #$1, $91(a6)
pub fn i_02f35e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F362;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x91);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02F362: bne.w $2f374
pub fn i_02f362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F366;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2F374; return 10; }
    8
}
/// $02F366: clr.w $28(a6)
pub fn i_02f366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F36A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02F36A: clr.w $2c(a6)
pub fn i_02f36a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F36E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02F36E: lea.l $2f374(pc), a1
pub fn i_02f36e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F372;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2F374;
    16
}
/// $02F372: move.l a1, (a6)
pub fn i_02f372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F374;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F374: jsr $2aa24(pc)
pub fn i_02f374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F378;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F378);
    cpu.pc = 0x2AA24;
    16
}
/// $02F378: bset.b #$4, $8d(a6)
pub fn i_02f378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F37E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02F37E: jsr $28992.l
pub fn i_02f37e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F384;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2F384);
    cpu.pc = 0x28992;
    16
}
/// $02F384: jsr $2a824(pc)
pub fn i_02f384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F388;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F388);
    cpu.pc = 0x2A824;
    16
}
/// $02F388: bcc.w $2f392
pub fn i_02f388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F38C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2F392; return 10; }
    8
}
/// $02F38C: lea.l $2ecb4(pc), a1
pub fn i_02f38c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F390;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2ECB4;
    16
}
/// $02F390: move.l a1, (a6)
pub fn i_02f390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F392;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F392: jsr $2a4f0(pc)
pub fn i_02f392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F396;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F396);
    cpu.pc = 0x2A4F0;
    16
}
/// $02F396: bcc.w $2f3a0
pub fn i_02f396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F39A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2F3A0; return 10; }
    8
}
/// $02F39A: lea.l $2ecb4(pc), a1
pub fn i_02f39a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F39E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2ECB4;
    16
}
/// $02F39E: move.l a1, (a6)
pub fn i_02f39e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F3A0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F3A0: jsr $5cef8.l
pub fn i_02f3a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F3A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2F3A6);
    cpu.pc = 0x5CEF8;
    16
}
/// $02F3A6: bcs.w $2f3b0
pub fn i_02f3a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F3AA;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2F3B0; return 10; }
    8
}
/// $02F3AA: lea.l $2e9c8(pc), a1
pub fn i_02f3aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F3AE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2E9C8;
    16
}
/// $02F3AE: move.l a1, (a6)
pub fn i_02f3ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F3B0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F3B0: jsr $28d70.l
pub fn i_02f3b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F3B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2F3B6);
    cpu.pc = 0x28D70;
    16
}
/// $02F3B6: bcc.w $2f3c0
pub fn i_02f3b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F3BA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2F3C0; return 10; }
    8
}
/// $02F3BA: lea.l $2ebb0(pc), a1
pub fn i_02f3ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F3BE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2EBB0;
    16
}
/// $02F3BE: move.l a1, (a6)
pub fn i_02f3be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F3C0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F3C0: jsr $283d8.l
pub fn i_02f3c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F3C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2F3C6);
    cpu.pc = 0x283D8;
    16
}
/// $02F3C6: jsr $2a328(pc)
pub fn i_02f3c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F3CA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F3CA);
    cpu.pc = 0x2A328;
    16
}
/// $02F3CA: bcc.w $2f3d4
pub fn i_02f3ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F3CE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2F3D4; return 10; }
    8
}
/// $02F3CE: lea.l $2f422(pc), a1
pub fn i_02f3ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F3D2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2F422;
    16
}
/// $02F3D2: move.l a1, (a6)
pub fn i_02f3d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F3D4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F3D4: jsr $2a34e(pc)
pub fn i_02f3d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F3D8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F3D8);
    cpu.pc = 0x2A34E;
    16
}
/// $02F3D8: bcc.w $2f3e2
pub fn i_02f3d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F3DC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2F3E2; return 10; }
    8
}
/// $02F3DC: lea.l $2f4fc(pc), a1
pub fn i_02f3dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F3E0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2F4FC;
    16
}
/// $02F3E0: move.l a1, (a6)
pub fn i_02f3e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F3E2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F3E2: jsr $2a8c0(pc)
pub fn i_02f3e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F3E6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F3E6);
    cpu.pc = 0x2A8C0;
    16
}
/// $02F3E6: bcc.w $2f3f0
pub fn i_02f3e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F3EA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2F3F0; return 10; }
    8
}
/// $02F3EA: lea.l $2d02e(pc), a1
pub fn i_02f3ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F3EE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D02E;
    16
}
/// $02F3EE: move.l a1, (a6)
pub fn i_02f3ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F3F0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F3F0: jsr $2ac80(pc)
pub fn i_02f3f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F3F4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F3F4);
    cpu.pc = 0x2AC80;
    16
}
/// $02F3F4: bcc.w $2f3fe
pub fn i_02f3f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F3F8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2F3FE; return 10; }
    8
}
/// $02F3F8: lea.l $2dd74(pc), a1
pub fn i_02f3f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F3FC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DD74;
    16
}
/// $02F3FC: move.l a1, (a6)
pub fn i_02f3fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F3FE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F3FE: jsr $2a5cc(pc)
pub fn i_02f3fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F402;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F402);
    cpu.pc = 0x2A5CC;
    16
}
/// $02F402: bcc.w $2f40c
pub fn i_02f402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F406;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2F40C; return 10; }
    8
}
/// $02F406: lea.l $2f0e6(pc), a1
pub fn i_02f406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F40A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2F0E6;
    16
}
/// $02F40A: move.l a1, (a6)
pub fn i_02f40a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F40C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F40C: jsr $2a690(pc)
pub fn i_02f40c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F410;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F410);
    cpu.pc = 0x2A690;
    16
}
/// $02F410: lea.l $2a060(pc), a0
pub fn i_02f410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F414;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A060;
    16
}
/// $02F414: movea.l #$ffffffff, a1
pub fn i_02f414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F41A;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
// ===== JsrAbsThunk_02f41a  ($02F41A, 8 B, thunks/jsr_abs_thunks.c)
/// $02F41A: jsr $772.l
pub fn i_02f41a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F420;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2F420);
    cpu.pc = 0x772;
    16
}
/// $02F420: rts
pub fn i_02f420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F422;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_CruiseRight_02f422  ($02F422, 210 B, slug/vehicle_02dd20.s)
/// $02F422: move.w #$2a0, $28(a6)
pub fn i_02f422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F428;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2A0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02F428: clr.w $2c(a6)
pub fn i_02f428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F42C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02F42C: jsr $2a9a0(pc)
pub fn i_02f42c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F430;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F430);
    cpu.pc = 0x2A9A0;
    16
}
/// $02F430: movea.l #$279434, a0
pub fn i_02f430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F436;
    cpu.ir = 0x207C;
    let v1: u32 = 0x279434;
    cpu.a[0] = v1;
    4
}
/// $02F436: lsl.w #$2, d0
pub fn i_02f436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F438;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02F438: movea.l (a0, d0.w), a0
pub fn i_02f438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F43C;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02F43C: cmpa.l #$ffffffff, a0
pub fn i_02f43c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F442;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02F442: beq.w $2f44c
pub fn i_02f442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F446;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2F44C; return 10; }
    8
}
/// $02F446: jsr $28cd4.l
pub fn i_02f446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F44C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2F44C);
    cpu.pc = 0x28CD4;
    16
}
/// $02F44C: move.w #$0, d0
pub fn i_02f44c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F450;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02F450: move.b #$96, $20(a6)
pub fn i_02f450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F456;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x96;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02F456: andi.w #$ff, d0
pub fn i_02f456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F45A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02F45A: lsr.w #$2, d0
pub fn i_02f45a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F45C;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02F45C: add.b d0, $20(a6)
pub fn i_02f45c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F460;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02F460: lea.l $2f466(pc), a1
pub fn i_02f460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F464;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2F466;
    16
}
/// $02F464: move.l a1, (a6)
pub fn i_02f464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F466;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F466: jsr $2aa24(pc)
pub fn i_02f466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F46A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F46A);
    cpu.pc = 0x2AA24;
    16
}
/// $02F46A: bset.b #$4, $8d(a6)
pub fn i_02f46a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F470;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02F470: jsr $28992.l
pub fn i_02f470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F476;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2F476);
    cpu.pc = 0x28992;
    16
}
/// $02F476: jsr $2a824(pc)
pub fn i_02f476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F47A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F47A);
    cpu.pc = 0x2A824;
    16
}
/// $02F47A: bcc.w $2f484
pub fn i_02f47a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F47E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2F484; return 10; }
    8
}
/// $02F47E: lea.l $2eca6(pc), a1
pub fn i_02f47e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F482;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2ECA6;
    16
}
/// $02F482: move.l a1, (a6)
pub fn i_02f482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F484;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F484: jsr $2a4f0(pc)
pub fn i_02f484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F488;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F488);
    cpu.pc = 0x2A4F0;
    16
}
/// $02F488: bcc.w $2f492
pub fn i_02f488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F48C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2F492; return 10; }
    8
}
/// $02F48C: lea.l $2eca6(pc), a1
pub fn i_02f48c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F490;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2ECA6;
    16
}
/// $02F490: move.l a1, (a6)
pub fn i_02f490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F492;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F492: jsr $5cef8.l
pub fn i_02f492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F498;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2F498);
    cpu.pc = 0x5CEF8;
    16
}
/// $02F498: bcs.w $2f4a2
pub fn i_02f498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F49C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2F4A2; return 10; }
    8
}
/// $02F49C: lea.l $2e9c8(pc), a1
pub fn i_02f49c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F4A0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2E9C8;
    16
}
/// $02F4A0: move.l a1, (a6)
pub fn i_02f4a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F4A2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F4A2: jsr $2a328(pc)
pub fn i_02f4a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F4A6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F4A6);
    cpu.pc = 0x2A328;
    16
}
/// $02F4A6: bcs.w $2f4b0
pub fn i_02f4a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F4AA;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2F4B0; return 10; }
    8
}
/// $02F4AA: lea.l $2f202(pc), a1
pub fn i_02f4aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F4AE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2F202;
    16
}
/// $02F4AE: move.l a1, (a6)
pub fn i_02f4ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F4B0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F4B0: jsr $28d70.l
pub fn i_02f4b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F4B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2F4B6);
    cpu.pc = 0x28D70;
    16
}
/// $02F4B6: jsr $283d8.l
pub fn i_02f4b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F4BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2F4BC);
    cpu.pc = 0x283D8;
    16
}
/// $02F4BC: jsr $2a8c0(pc)
pub fn i_02f4bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F4C0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F4C0);
    cpu.pc = 0x2A8C0;
    16
}
/// $02F4C0: bcc.w $2f4ca
pub fn i_02f4c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F4C4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2F4CA; return 10; }
    8
}
/// $02F4C4: lea.l $2d02e(pc), a1
pub fn i_02f4c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F4C8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D02E;
    16
}
/// $02F4C8: move.l a1, (a6)
pub fn i_02f4c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F4CA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F4CA: jsr $2ac80(pc)
pub fn i_02f4ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F4CE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F4CE);
    cpu.pc = 0x2AC80;
    16
}
/// $02F4CE: bcc.w $2f4d8
pub fn i_02f4ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F4D2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2F4D8; return 10; }
    8
}
/// $02F4D2: lea.l $2dd74(pc), a1
pub fn i_02f4d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F4D6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DD74;
    16
}
/// $02F4D6: move.l a1, (a6)
pub fn i_02f4d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F4D8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F4D8: jsr $2a5cc(pc)
pub fn i_02f4d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F4DC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F4DC);
    cpu.pc = 0x2A5CC;
    16
}
/// $02F4DC: bcc.w $2f4e6
pub fn i_02f4dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F4E0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2F4E6; return 10; }
    8
}
/// $02F4E0: lea.l $2f0e6(pc), a1
pub fn i_02f4e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F4E4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2F0E6;
    16
}
/// $02F4E4: move.l a1, (a6)
pub fn i_02f4e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F4E6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F4E6: jsr $2a690(pc)
pub fn i_02f4e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F4EA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F4EA);
    cpu.pc = 0x2A690;
    16
}
/// $02F4EA: lea.l $2a060(pc), a0
pub fn i_02f4ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F4EE;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A060;
    16
}
/// $02F4EE: movea.l #$ffffffff, a1
pub fn i_02f4ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F4F4;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
// ===== JsrAbsThunk_02f4f4  ($02F4F4, 8 B, thunks/jsr_abs_thunks.c)
/// $02F4F4: jsr $772.l
pub fn i_02f4f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F4FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2F4FA);
    cpu.pc = 0x772;
    16
}
/// $02F4FA: rts
pub fn i_02f4fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F4FC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_CruiseLeft_02f4fc  ($02F4FC, 210 B, slug/vehicle_02dd20.s)
/// $02F4FC: move.w #$fd60, $28(a6)
pub fn i_02f4fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F502;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFD60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02F502: clr.w $2c(a6)
pub fn i_02f502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F506;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02F506: jsr $2a9a0(pc)
pub fn i_02f506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F50A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F50A);
    cpu.pc = 0x2A9A0;
    16
}
/// $02F50A: movea.l #$279448, a0
pub fn i_02f50a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F510;
    cpu.ir = 0x207C;
    let v1: u32 = 0x279448;
    cpu.a[0] = v1;
    4
}
/// $02F510: lsl.w #$2, d0
pub fn i_02f510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F512;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02F512: movea.l (a0, d0.w), a0
pub fn i_02f512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F516;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02F516: cmpa.l #$ffffffff, a0
pub fn i_02f516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F51C;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02F51C: beq.w $2f526
pub fn i_02f51c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F520;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2F526; return 10; }
    8
}
/// $02F520: jsr $28cd4.l
pub fn i_02f520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F526;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2F526);
    cpu.pc = 0x28CD4;
    16
}
/// $02F526: move.w #$0, d0
pub fn i_02f526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F52A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02F52A: move.b #$96, $20(a6)
pub fn i_02f52a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F530;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x96;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02F530: andi.w #$ff, d0
pub fn i_02f530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F534;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02F534: lsr.w #$2, d0
pub fn i_02f534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F536;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02F536: add.b d0, $20(a6)
pub fn i_02f536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F53A;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02F53A: lea.l $2f540(pc), a1
pub fn i_02f53a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F53E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2F540;
    16
}
/// $02F53E: move.l a1, (a6)
pub fn i_02f53e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F540;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F540: jsr $2aa24(pc)
pub fn i_02f540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F544;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F544);
    cpu.pc = 0x2AA24;
    16
}
/// $02F544: bset.b #$4, $8d(a6)
pub fn i_02f544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F54A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02F54A: jsr $28992.l
pub fn i_02f54a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F550;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2F550);
    cpu.pc = 0x28992;
    16
}
/// $02F550: jsr $2a824(pc)
pub fn i_02f550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F554;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F554);
    cpu.pc = 0x2A824;
    16
}
/// $02F554: bcc.w $2f55e
pub fn i_02f554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F558;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2F55E; return 10; }
    8
}
/// $02F558: lea.l $2ecb4(pc), a1
pub fn i_02f558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F55C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2ECB4;
    16
}
/// $02F55C: move.l a1, (a6)
pub fn i_02f55c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F55E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F55E: jsr $2a4f0(pc)
pub fn i_02f55e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F562;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F562);
    cpu.pc = 0x2A4F0;
    16
}
/// $02F562: bcc.w $2f56c
pub fn i_02f562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F566;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2F56C; return 10; }
    8
}
/// $02F566: lea.l $2ecb4(pc), a1
pub fn i_02f566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F56A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2ECB4;
    16
}
/// $02F56A: move.l a1, (a6)
pub fn i_02f56a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F56C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F56C: jsr $5cef8.l
pub fn i_02f56c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F572;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2F572);
    cpu.pc = 0x5CEF8;
    16
}
/// $02F572: bcs.w $2f57c
pub fn i_02f572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F576;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2F57C; return 10; }
    8
}
/// $02F576: lea.l $2e9c8(pc), a1
pub fn i_02f576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F57A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2E9C8;
    16
}
/// $02F57A: move.l a1, (a6)
pub fn i_02f57a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F57C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F57C: jsr $2a34e(pc)
pub fn i_02f57c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F580;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F580);
    cpu.pc = 0x2A34E;
    16
}
/// $02F580: bcs.w $2f58a
pub fn i_02f580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F584;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2F58A; return 10; }
    8
}
/// $02F584: lea.l $2f312(pc), a1
pub fn i_02f584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F588;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2F312;
    16
}
/// $02F588: move.l a1, (a6)
pub fn i_02f588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F58A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F58A: jsr $28d70.l
pub fn i_02f58a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F590;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2F590);
    cpu.pc = 0x28D70;
    16
}
/// $02F590: jsr $283d8.l
pub fn i_02f590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F596;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2F596);
    cpu.pc = 0x283D8;
    16
}
/// $02F596: jsr $2a8c0(pc)
pub fn i_02f596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F59A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F59A);
    cpu.pc = 0x2A8C0;
    16
}
/// $02F59A: bcc.w $2f5a4
pub fn i_02f59a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F59E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2F5A4; return 10; }
    8
}
/// $02F59E: lea.l $2d02e(pc), a1
pub fn i_02f59e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F5A2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D02E;
    16
}
/// $02F5A2: move.l a1, (a6)
pub fn i_02f5a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F5A4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F5A4: jsr $2ac80(pc)
pub fn i_02f5a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F5A8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F5A8);
    cpu.pc = 0x2AC80;
    16
}
/// $02F5A8: bcc.w $2f5b2
pub fn i_02f5a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F5AC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2F5B2; return 10; }
    8
}
/// $02F5AC: lea.l $2dd74(pc), a1
pub fn i_02f5ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F5B0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DD74;
    16
}
/// $02F5B0: move.l a1, (a6)
pub fn i_02f5b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F5B2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F5B2: jsr $2a5cc(pc)
pub fn i_02f5b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F5B6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F5B6);
    cpu.pc = 0x2A5CC;
    16
}
/// $02F5B6: bcc.w $2f5c0
pub fn i_02f5b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F5BA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2F5C0; return 10; }
    8
}
/// $02F5BA: lea.l $2f0e6(pc), a1
pub fn i_02f5ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F5BE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2F0E6;
    16
}
/// $02F5BE: move.l a1, (a6)
pub fn i_02f5be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F5C0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F5C0: jsr $2a690(pc)
pub fn i_02f5c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F5C4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F5C4);
    cpu.pc = 0x2A690;
    16
}
/// $02F5C4: lea.l $2a060(pc), a0
pub fn i_02f5c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F5C8;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A060;
    16
}
/// $02F5C8: movea.l #$ffffffff, a1
pub fn i_02f5c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F5CE;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
// ===== JsrAbsThunk_02f5ce  ($02F5CE, 8 B, thunks/jsr_abs_thunks.c)
/// $02F5CE: jsr $772.l
pub fn i_02f5ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F5D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2F5D4);
    cpu.pc = 0x772;
    16
}
/// $02F5D4: rts
pub fn i_02f5d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F5D6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_SetSpeed_02f5d6  ($02F5D6, 118 B, slug/vehicle_02dd20.s)
/// $02F5D6: jsr $2a9a0(pc)
pub fn i_02f5d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F5DA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2F5DA);
    cpu.pc = 0x2A9A0;
    16
}
/// $02F5DA: movea.l #$279308, a0
pub fn i_02f5da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F5E0;
    cpu.ir = 0x207C;
    let v1: u32 = 0x279308;
    cpu.a[0] = v1;
    4
}
/// $02F5E0: lsl.w #$2, d0
pub fn i_02f5e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F5E2;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02F5E2: movea.l (a0, d0.w), a0
pub fn i_02f5e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F5E6;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02F5E6: cmpa.l #$ffffffff, a0
pub fn i_02f5e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F5EC;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02F5EC: beq.w $2f5f6
pub fn i_02f5ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F5F0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2F5F6; return 10; }
    8
}
/// $02F5F0: jsr $28cd4.l
pub fn i_02f5f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F5F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2F5F6);
    cpu.pc = 0x28CD4;
    16
}
/// $02F5F6: move.w #$0, d0
pub fn i_02f5f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F5FA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02F5FA: move.b #$96, $20(a6)
pub fn i_02f5fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F600;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x96;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02F600: andi.w #$ff, d0
pub fn i_02f600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F604;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02F604: lsr.w #$2, d0
pub fn i_02f604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F606;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02F606: add.b d0, $20(a6)
pub fn i_02f606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F60A;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02F60A: lea.l $2c95c(pc), a1
pub fn i_02f60a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F60E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2C95C;
    16
}
/// $02F60E: move.l a1, (a6)
pub fn i_02f60e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F610;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F610: bra.w $2c95c
pub fn i_02f610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F614;
    cpu.ir = 0x6000;
    cpu.pc = 0x2C95C;
    10
}
// ===== SlugFx_Smoke_02f64c  ($02F64C, 70 B, slug/vehicle_02dd20.s)
/// $02F64C: move.w #$4, d1
pub fn i_02f64c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F650;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02F650: jsr $236e.l
pub fn i_02f650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F656;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2F656);
    cpu.pc = 0x236E;
    16
}
/// $02F656: lea.l $2dd6a6.l, a0
pub fn i_02f656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F65C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DD6A6;
    20
}
/// $02F65C: jsr $28cd4.l
pub fn i_02f65c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F662;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2F662);
    cpu.pc = 0x28CD4;
    16
}
/// $02F662: move.w #$2000, $38(a6)
pub fn i_02f662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F668;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02F668: lea.l $2f692(pc), a1
pub fn i_02f668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F66C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2F692;
    16
}
/// $02F66C: move.l a1, (a6)
pub fn i_02f66c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F66E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02F66E: bra.w $2f692
pub fn i_02f66e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F672;
    cpu.ir = 0x6000;
    cpu.pc = 0x2F692;
    10
}
/// $02F672: move.w #$163, d1
pub fn i_02f672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F676;
    cpu.ir = 0x323C;
    let v1: u32 = 0x163;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02F676: jsr $236e.l
pub fn i_02f676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F67C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2F67C);
    cpu.pc = 0x236E;
    16
}
/// $02F67C: lea.l $2f6c0(pc), a0
pub fn i_02f67c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F680;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2F6C0;
    16
}
/// $02F680: jsr $28cd4.l
pub fn i_02f680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F686;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2F686);
    cpu.pc = 0x28CD4;
    16
}
/// $02F686: move.w #$d000, $38(a6)
pub fn i_02f686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F68C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02F68C: lea.l $2f692(pc), a1
pub fn i_02f68c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F690;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2F692;
    16
}
/// $02F690: move.l a1, (a6)
pub fn i_02f690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F692;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== SlugFx_Fall_02f692  ($02F692, 46 B, slug/vehicle_02dd20.s)
/// $02F692: jsr $2783a.l
pub fn i_02f692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F698;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2F698);
    cpu.pc = 0x2783A;
    16
}
/// $02F698: jsr $28d70.l
pub fn i_02f698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F69E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2F69E);
    cpu.pc = 0x28D70;
    16
}
/// $02F69E: bcc.w $2f6a8
pub fn i_02f69e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F6A2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2F6A8; return 10; }
    8
}
/// $02F6A2: jmp $518.l
pub fn i_02f6a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F6A8;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $02F6A8: movea.l #$ffffffff, a0
pub fn i_02f6a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F6AE;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $02F6AE: jsr $5dd56.l
pub fn i_02f6ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F6B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2F6B4);
    cpu.pc = 0x5DD56;
    16
}
/// $02F6B4: bcc.w $2f6be
pub fn i_02f6b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F6B8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2F6BE; return 10; }
    8
}
/// $02F6B8: jmp $518.l
pub fn i_02f6b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F6BE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $02F6BE: rts
pub fn i_02f6be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F6C0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_TypeIfAir_02f84a  ($02F84A, 16 B, slug/vehicle_02dd20.s)
/// $02F84A: btst.b #$2, $8d(a6)
pub fn i_02f84a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F850;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $02F850: beq.w $2f858
pub fn i_02f850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F854;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2F858; return 10; }
    8
}
/// $02F854: move.w $1c(a6), d2
pub fn i_02f854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F858;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02F858: rts
pub fn i_02f858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F85A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Players_AnyFlag8D3_02f85a  ($02F85A, 32 B, slug/vehicle_02dd20.s)
/// $02F85A: lea.l $100440.l, a0
pub fn i_02f85a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F860;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100440;
    20
}
/// $02F860: btst.b #$3, $8d(a0)
pub fn i_02f860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F866;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x8D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $02F866: bne.w $2f880
pub fn i_02f866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F86A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2F880; return 10; }
    8
}
/// $02F86A: lea.l $1004e0.l, a0
pub fn i_02f86a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F870;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1004E0;
    20
}
/// $02F870: btst.b #$3, $8d(a0)
pub fn i_02f870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F876;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x8D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $02F876: bne.w $2f880
pub fn i_02f876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F87A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2F880; return 10; }
    8
}
// ===== ClearXN_02f87a  ($02F87A, 6 B, thunks/ccr_helpers.c)
/// $02F87A: andi.b #$ee, ccr
pub fn i_02f87a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F87E;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02F87E: rts
pub fn i_02f87e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F880;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Players_AnyFlag8D3_SetC_02f880  ($02F880, 6 B, slug/vehicle_02dd20.s)
/// $02F880: ori.b #$11, ccr
pub fn i_02f880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F884;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02F884: rts
pub fn i_02f884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F886;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_ResetDamageIdx_02fada  ($02FADA, 10 B, slug/vehicle_02dd20.s)
/// $02FADA: clr.b $88(a6)
pub fn i_02fada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FADE;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x88);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $02FADE: clr.w $86(a6)
pub fn i_02fade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FAE2;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02FAE2: rts
pub fn i_02fae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FAE4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_UpdateDamageSprite_02fae4  ($02FAE4, 168 B, slug/vehicle_02dd20.s)
/// $02FAE4: cmpi.w #$1, $66(a6)
pub fn i_02fae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FAEA;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $02FAEA: bhi.w $2faf6
pub fn i_02faea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FAEE;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x2FAF6; return 10; }
    8
}
/// $02FAEE: lea.l $2f886(pc), a1
pub fn i_02faee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FAF2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2F886;
    16
}
/// $02FAF2: bra.w $2fb0c
pub fn i_02faf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FAF6;
    cpu.ir = 0x6000;
    cpu.pc = 0x2FB0C;
    10
}
/// $02FAF6: andi.b #$ee, ccr
pub fn i_02faf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FAFA;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02FAFA: rts
pub fn i_02fafa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FAFC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $02FAFC: lea.l $2f8ae(pc), a1
pub fn i_02fafc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB00;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2F8AE;
    16
}
/// $02FB00: bra.w $2fb0c
pub fn i_02fb00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB04;
    cpu.ir = 0x6000;
    cpu.pc = 0x2FB0C;
    10
}
/// $02FB04: lea.l $2fa3c(pc), a1
pub fn i_02fb04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB08;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2FA3C;
    16
}
/// $02FB08: bra.w $2fb0c
pub fn i_02fb08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB0C;
    cpu.ir = 0x6000;
    cpu.pc = 0x2FB0C;
    10
}
/// $02FB0C: moveq #$0, d0
pub fn i_02fb0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB0E;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $02FB0E: moveq #$0, d7
pub fn i_02fb0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB10;
    cpu.ir = 0x7E00;
    cpu.d[7] = 0x0;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $02FB10: move.w $86(a6), d0
pub fn i_02fb10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB14;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02FB14: move.b (a1, d0.w), d2
pub fn i_02fb14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB18;
    cpu.ir = 0x1431;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    14
}
/// $02FB18: cmpi.b #$4, d2
pub fn i_02fb18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB1C;
    cpu.ir = 0xC02;
    let v1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, v1, 0x4, Size::Byte, false);
    8
}
/// $02FB1C: bcs.w $2fb2c
pub fn i_02fb1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB20;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2FB2C; return 10; }
    8
}
/// $02FB20: nop
pub fn i_02fb20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB22;
    cpu.ir = 0x4E71;
    4
}
/// $02FB22: nop
pub fn i_02fb22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB24;
    cpu.ir = 0x4E71;
    4
}
/// $02FB24: cmpi.b #$4, d2
pub fn i_02fb24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB28;
    cpu.ir = 0xC02;
    let v1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, v1, 0x4, Size::Byte, false);
    8
}
/// $02FB28: nop
pub fn i_02fb28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB2A;
    cpu.ir = 0x4E71;
    4
}
/// $02FB2A: trap #$f
pub fn i_02fb2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB2C;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $02FB2C: andi.l #$ff, d2
pub fn i_02fb2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB32;
    cpu.ir = 0x282;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[2] = r2;
    8
}
/// $02FB32: lsl.l #$2, d2
pub fn i_02fb32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB34;
    cpu.ir = 0xE58A;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Long, 1, true);
    cpu.d[2] = r;
    10
}
/// $02FB34: lea.l $2faca(pc), a2
pub fn i_02fb34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB38;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0x2FACA;
    16
}
/// $02FB38: movea.l (a2, d2.w), a3
pub fn i_02fb38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB3C;
    cpu.ir = 0x2672;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[3] = v3;
    18
}
/// $02FB3C: jmp (a3)
pub fn i_02fb3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB3E;
    cpu.ir = 0x4ED3;
    cpu.jump_to(cpu.a[3]);
    8
}
/// $02FB3E: andi.b #$f7, $12(a6)
pub fn i_02fb3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB44;
    cpu.ir = 0x22E;
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0xF7) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02FB44: move.b $3(a1, d0.w), d5
pub fn i_02fb44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB48;
    cpu.ir = 0x1A31;
    let ad2 = cpu.a[1].wrapping_add(0x3).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    14
}
/// $02FB48: or.b d5, $12(a6)
pub fn i_02fb48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB4C;
    cpu.ir = 0x8B2E;
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | cpu.d[5]) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $02FB4C: addq.b #$1, $88(a6)
pub fn i_02fb4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB50;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x88);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02FB50: move.b $88(a6), d4
pub fn i_02fb50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB54;
    cpu.ir = 0x182E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    12
}
/// $02FB54: cmp.b $2(a1, d0.w), d4
pub fn i_02fb54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB58;
    cpu.ir = 0xB831;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[4] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    14
}
/// $02FB58: blt.w $2fb68
pub fn i_02fb58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB5C;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x2FB68; return 10; }
    8
}
/// $02FB5C: clr.b $88(a6)
pub fn i_02fb5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB60;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x88);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $02FB60: move.b $1(a1, d0.w), d7
pub fn i_02fb60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB64;
    cpu.ir = 0x1E31;
    let ad2 = cpu.a[1].wrapping_add(0x1).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    14
}
/// $02FB64: add.w d7, $86(a6)
pub fn i_02fb64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB68;
    cpu.ir = 0xDF6E;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[7] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $02FB68: andi.b #$ee, ccr
pub fn i_02fb68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB6C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02FB6C: rts
pub fn i_02fb6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB6E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $02FB6E: move.w $2(a1, d0.w), d3
pub fn i_02fb6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB72;
    cpu.ir = 0x3631;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $02FB72: move.w (a6, d3.w), $14(a6)
pub fn i_02fb72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB78;
    cpu.ir = 0x3D76;
    let ad2 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    22
}
/// $02FB78: move.b $1(a1, d0.w), d7
pub fn i_02fb78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB7C;
    cpu.ir = 0x1E31;
    let ad2 = cpu.a[1].wrapping_add(0x1).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    14
}
/// $02FB7C: add.w d7, $86(a6)
pub fn i_02fb7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB80;
    cpu.ir = 0xDF6E;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[7] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $02FB80: bra.b $2fb0c
pub fn i_02fb80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB82;
    cpu.ir = 0x608A;
    cpu.pc = 0x2FB0C;
    10
}
// ===== SetXN_02fb8c  ($02FB8C, 6 B, thunks/ccr_helpers.c)
/// $02FB8C: ori.b #$11, ccr
pub fn i_02fb8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB90;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02FB90: rts
pub fn i_02fb90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB92;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $02FB9A: clr.w $70(a6)
pub fn i_02fb9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB9E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02FB9E: move.w #$196, d1
pub fn i_02fb9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FBA2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x196;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02FBA2: jsr $236e.l
pub fn i_02fba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FBA8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2FBA8);
    cpu.pc = 0x236E;
    16
}
/// $02FBA8: move.w #$197, d1
pub fn i_02fba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FBAC;
    cpu.ir = 0x323C;
    let v1: u32 = 0x197;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02FBAC: jsr $236e.l
pub fn i_02fbac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FBB2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2FBB2);
    cpu.pc = 0x236E;
    16
}
/// $02FBB2: move.w #$198, d1
pub fn i_02fbb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FBB6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x198;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02FBB6: jsr $236e.l
pub fn i_02fbb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FBBC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2FBBC);
    cpu.pc = 0x236E;
    16
}
/// $02FBBC: move.w #$199, d1
pub fn i_02fbbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FBC0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x199;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02FBC0: jsr $236e.l
pub fn i_02fbc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FBC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2FBC6);
    cpu.pc = 0x236E;
    16
}
/// $02FBC6: lea.l $2fbcc(pc), a1
pub fn i_02fbc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FBCA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2FBCC;
    16
}
/// $02FBCA: move.l a1, (a6)
pub fn i_02fbca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FBCC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02FBCC: movea.l $c(a6), a0
pub fn i_02fbcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FBD0;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $02FBD0: btst.b #$0, $13(a0)
pub fn i_02fbd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FBD6;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $02FBD6: beq.w $2fbe8
pub fn i_02fbd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FBDA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2FBE8; return 10; }
    8
}
/// $02FBDA: lea.l $2fc6a(pc), a1
pub fn i_02fbda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FBDE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2FC6A;
    16
}
/// $02FBDE: move.l a1, (a6)
pub fn i_02fbde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FBE0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02FBE0: clr.w $70(a6)
pub fn i_02fbe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FBE4;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02FBE4: bra.w $2fbfa
pub fn i_02fbe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FBE8;
    cpu.ir = 0x6000;
    cpu.pc = 0x2FBFA;
    10
}
/// $02FBE8: cmpi.w #$10, $66(a0)
pub fn i_02fbe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FBEE;
    cpu.ir = 0xC68;
    let ad2 = cpu.a[0].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x10, Size::Word, false);
    16
}
/// $02FBEE: bhi.w $2fbf6
pub fn i_02fbee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FBF2;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x2FBF6; return 10; }
    8
}
/// $02FBF2: bra.w $2fbfa
pub fn i_02fbf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FBF6;
    cpu.ir = 0x6000;
    cpu.pc = 0x2FBFA;
    10
}
/// $02FBF6: jmp $2fc54(pc)
pub fn i_02fbf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FBFA;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x2FC54;
    8
}
/// $02FBFA: btst.b #$3, $12(a0)
pub fn i_02fbfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC00;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x12);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $02FC00: beq.w $2fc54
pub fn i_02fc00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC04;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2FC54; return 10; }
    8
}
/// $02FC04: addq.w #$2, $70(a6)
pub fn i_02fc04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC08;
    cpu.ir = 0x546E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $02FC08: andi.w #$6, $70(a6)
pub fn i_02fc08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC0E;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x6) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $02FC0E: moveq #$0, d0
pub fn i_02fc0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC10;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $02FC10: moveq #$0, d1
pub fn i_02fc10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC12;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $02FC12: move.w $70(a6), d0
pub fn i_02fc12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC16;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02FC16: lea.l $2fb92(pc), a1
pub fn i_02fc16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC1A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2FB92;
    16
}
/// $02FC1A: move.w (a1, d0.w), d1
pub fn i_02fc1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC1E;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $02FC1E: move.w (a6, d1.w), $14(a6)
pub fn i_02fc1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC24;
    cpu.ir = 0x3D76;
    let ad2 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    22
}
/// $02FC24: cmpi.w #$ffff, $40(a0)
pub fn i_02fc24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC2A;
    cpu.ir = 0xC68;
    let ad2 = cpu.a[0].wrapping_add(0x40);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFFF, Size::Word, false);
    16
}
/// $02FC2A: bne.w $2fc36
pub fn i_02fc2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC2E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2FC36; return 10; }
    8
}
/// $02FC2E: movea.l $3c(a0), a2
pub fn i_02fc2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC32;
    cpu.ir = 0x2468;
    let ad2 = cpu.a[0].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $02FC32: bra.w $2fc3a
pub fn i_02fc32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC36;
    cpu.ir = 0x6000;
    cpu.pc = 0x2FC3A;
    10
}
/// $02FC36: movea.l $40(a0), a2
pub fn i_02fc36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC3A;
    cpu.ir = 0x2468;
    let ad2 = cpu.a[0].wrapping_add(0x40);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $02FC3A: move.l a2, $3c(a6)
pub fn i_02fc3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC3E;
    cpu.ir = 0x2D4A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02FC3E: move.l a2, $40(a6)
pub fn i_02fc3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC42;
    cpu.ir = 0x2D4A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x40);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02FC42: jsr $5e4b2.l
pub fn i_02fc42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC48;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2FC48);
    cpu.pc = 0x5E4B2;
    16
}
/// $02FC48: subi.w #$2, $38(a6)
pub fn i_02fc48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC4E;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $02FC4E: jsr $28d70.l
pub fn i_02fc4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2FC54);
    cpu.pc = 0x28D70;
    16
}
/// $02FC54: movea.l $c(a6), a0
pub fn i_02fc54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC58;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $02FC58: cmpi.l #$2dc5c, (a0)
pub fn i_02fc58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC5E;
    cpu.ir = 0xC90;
    let v2 = bus.read32(cpu.a[0]);
    sub_with_flags(cpu, v2, 0x2DC5C, Size::Long, false);
    16
}
/// $02FC5E: bne.w $2fc68
pub fn i_02fc5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC62;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2FC68; return 10; }
    8
}
/// $02FC62: jmp $518.l
pub fn i_02fc62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC68;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $02FC68: rts
pub fn i_02fc68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC6A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_WheelAnim_Entry_02fc6a  ($02FC6A, 6 B, slug/vehicle_02dd20.s)
/// $02FC6A: movea.l $c(a6), a0
pub fn i_02fc6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC6E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $02FC6E: bra.b $2fbfa
pub fn i_02fc6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC70;
    cpu.ir = 0x608A;
    cpu.pc = 0x2FBFA;
    10
}
// ===== Slug_Destroyed_02fc70  ($02FC70, 500 B, slug/vehicle_02dd20.s)
/// $02FC70: move.w #$10e9, d0
pub fn i_02fc70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC74;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10E9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02FC74: jsr $2352.l
pub fn i_02fc74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC7A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2FC7A);
    cpu.pc = 0x2352;
    16
}
/// $02FC7A: jsr $2feda(pc)
pub fn i_02fc7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC7E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2FC7E);
    cpu.pc = 0x2FEDA;
    16
}
/// $02FC7E: move.b #$30, $92(a6)
pub fn i_02fc7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC84;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x92);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02FC84: move.w #$10af, d0
pub fn i_02fc84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC88;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10AF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02FC88: jsr $2352.l
pub fn i_02fc88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC8E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2FC8E);
    cpu.pc = 0x2352;
    16
}
/// $02FC8E: move.w #$600, $36(a6)
pub fn i_02fc8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC94;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x600;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02FC94: bra.w $2fc9c
pub fn i_02fc94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC98;
    cpu.ir = 0x6000;
    cpu.pc = 0x2FC9C;
    10
}
/// $02FC9C: bclr.b #$5, $8c(a6)
pub fn i_02fc9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FCA2;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02FCA2: clr.w $2c(a6)
pub fn i_02fca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FCA6;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02FCA6: jsr $2a958(pc)
pub fn i_02fca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FCAA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2FCAA);
    cpu.pc = 0x2A958;
    16
}
/// $02FCAA: move.w d2, $80(a6)
pub fn i_02fcaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FCAE;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02FCAE: move.w d2, d0
pub fn i_02fcae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FCB0;
    cpu.ir = 0x3002;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02FCB0: asr.w #$1, d0
pub fn i_02fcb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FCB2;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02FCB2: neg.w d0
pub fn i_02fcb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FCB4;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02FCB4: addi.w #$20, d0
pub fn i_02fcb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FCB8;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02FCB8: cmpi.w #$40, d0
pub fn i_02fcb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FCBC;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $02FCBC: bls.w $2fcc4
pub fn i_02fcbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FCC0;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x2FCC4; return 10; }
    8
}
/// $02FCC0: move.w #$40, d0
pub fn i_02fcc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FCC4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02FCC4: movem.w d0, -(a7)
pub fn i_02fcc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FCC8;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $02FCC8: jsr $295a6(pc)
pub fn i_02fcc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FCCC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2FCCC);
    cpu.pc = 0x295A6;
    16
}
/// $02FCCC: movea.l #$2b0c30, a0
pub fn i_02fccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FCD2;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2B0C30;
    cpu.a[0] = v1;
    4
}
/// $02FCD2: lsl.w #$2, d0
pub fn i_02fcd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FCD4;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02FCD4: movea.l (a0, d0.w), a0
pub fn i_02fcd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FCD8;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02FCD8: cmpa.l #$ffffffff, a0
pub fn i_02fcd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FCDE;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02FCDE: beq.w $2fce8
pub fn i_02fcde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FCE2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2FCE8; return 10; }
    8
}
/// $02FCE2: jsr $28cd4.l
pub fn i_02fce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FCE8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2FCE8);
    cpu.pc = 0x28CD4;
    16
}
/// $02FCE8: movem.w (a7)+, d0
pub fn i_02fce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FCEC;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $02FCEC: move.b #$0, $20(a6)
pub fn i_02fcec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FCF2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02FCF2: lsr.w #$2, d0
pub fn i_02fcf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FCF4;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02FCF4: andi.w #$ff, d0
pub fn i_02fcf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FCF8;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02FCF8: add.b d0, $20(a6)
pub fn i_02fcf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FCFC;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02FCFC: lea.l $ffff.w, a0
pub fn i_02fcfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FD00;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $02FD00: move.l a0, $48(a6)
pub fn i_02fd00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FD04;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02FD04: move.l #$295b4, $60(a6)
pub fn i_02fd04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FD0C;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x295B4;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02FD0C: move.l #$2964c, $60(a6)
pub fn i_02fd0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FD14;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2964C;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02FD14: lea.l $2fd1a(pc), a1
pub fn i_02fd14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FD18;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2FD1A;
    16
}
/// $02FD18: move.l a1, (a6)
pub fn i_02fd18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FD1A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02FD1A: bset.b #$2, $8d(a6)
pub fn i_02fd1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FD20;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02FD20: jsr $2aa24(pc)
pub fn i_02fd20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FD24;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2FD24);
    cpu.pc = 0x2AA24;
    16
}
/// $02FD24: move.w $36(a6), d7
pub fn i_02fd24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FD28;
    cpu.ir = 0x3E2E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02FD28: cmpi.b #$3, $106f2a.l
pub fn i_02fd28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FD30;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106F2A) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    20
}
/// $02FD30: bne.w $2fd4a
pub fn i_02fd30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FD34;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2FD4A; return 10; }
    8
}
/// $02FD34: jsr $2a958(pc)
pub fn i_02fd34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FD38;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2FD38);
    cpu.pc = 0x2A958;
    16
}
/// $02FD38: sub.w $80(a6), d2
pub fn i_02fd38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FD3C;
    cpu.ir = 0x946E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $02FD3C: eor.w d7, d2
pub fn i_02fd3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FD3E;
    cpu.ir = 0xBF42;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = (v1 ^ cpu.d[7]) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02FD3E: asl.w #$1, d2
pub fn i_02fd3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FD40;
    cpu.ir = 0xE342;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02FD40: bcs.w $2fd4a
pub fn i_02fd40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FD44;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2FD4A; return 10; }
    8
}
/// $02FD44: move.w d7, d2
pub fn i_02fd44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FD46;
    cpu.ir = 0x3407;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02FD46: asr.w #$1, d2
pub fn i_02fd46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FD48;
    cpu.ir = 0xE242;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02FD48: add.w d2, d7
pub fn i_02fd48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FD4A;
    cpu.ir = 0xDE42;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $02FD4A: move.w d7, $28(a6)
pub fn i_02fd4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FD4E;
    cpu.ir = 0x3D47;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02FD4E: jsr $2a478(pc)
pub fn i_02fd4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FD52;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2FD52);
    cpu.pc = 0x2A478;
    16
}
/// $02FD52: jsr $2a760(pc)
pub fn i_02fd52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FD56;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2FD56);
    cpu.pc = 0x2A760;
    16
}
/// $02FD56: jsr $2a958(pc)
pub fn i_02fd56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FD5A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2FD5A);
    cpu.pc = 0x2A958;
    16
}
/// $02FD5A: cmp.w $80(a6), d2
pub fn i_02fd5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FD5E;
    cpu.ir = 0xB46E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $02FD5E: beq.w $2fdbc
pub fn i_02fd5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FD62;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2FDBC; return 10; }
    8
}
/// $02FD62: move.w d2, $80(a6)
pub fn i_02fd62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FD66;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02FD66: move.w d2, d0
pub fn i_02fd66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FD68;
    cpu.ir = 0x3002;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02FD68: asr.w #$1, d0
pub fn i_02fd68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FD6A;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02FD6A: neg.w d0
pub fn i_02fd6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FD6C;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02FD6C: addi.w #$20, d0
pub fn i_02fd6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FD70;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02FD70: cmpi.w #$40, d0
pub fn i_02fd70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FD74;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $02FD74: bls.w $2fd7c
pub fn i_02fd74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FD78;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x2FD7C; return 10; }
    8
}
/// $02FD78: move.w #$40, d0
pub fn i_02fd78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FD7C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02FD7C: movem.w d0, -(a7)
pub fn i_02fd7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FD80;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $02FD80: jsr $295a6(pc)
pub fn i_02fd80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FD84;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2FD84);
    cpu.pc = 0x295A6;
    16
}
/// $02FD84: movea.l #$2b0c30, a0
pub fn i_02fd84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FD8A;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2B0C30;
    cpu.a[0] = v1;
    4
}
/// $02FD8A: lsl.w #$2, d0
pub fn i_02fd8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FD8C;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02FD8C: movea.l (a0, d0.w), a0
pub fn i_02fd8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FD90;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02FD90: cmpa.l #$ffffffff, a0
pub fn i_02fd90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FD96;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02FD96: beq.w $2fda0
pub fn i_02fd96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FD9A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2FDA0; return 10; }
    8
}
/// $02FD9A: jsr $28cd4.l
pub fn i_02fd9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FDA0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2FDA0);
    cpu.pc = 0x28CD4;
    16
}
/// $02FDA0: movem.w (a7)+, d0
pub fn i_02fda0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FDA4;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $02FDA4: move.b #$0, $20(a6)
pub fn i_02fda4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FDAA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02FDAA: lsr.w #$2, d0
pub fn i_02fdaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FDAC;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02FDAC: andi.w #$ff, d0
pub fn i_02fdac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FDB0;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02FDB0: add.b d0, $20(a6)
pub fn i_02fdb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FDB4;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02FDB4: lea.l $ffff.w, a0
pub fn i_02fdb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FDB8;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $02FDB8: move.l a0, $48(a6)
pub fn i_02fdb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FDBC;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02FDBC: jsr $28d70.l
pub fn i_02fdbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FDC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2FDC2);
    cpu.pc = 0x28D70;
    16
}
/// $02FDC2: move.b $106f28.l, d0
pub fn i_02fdc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FDC8;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $02FDC8: andi.b #$1, d0
pub fn i_02fdc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FDCC;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $02FDCC: bne.w $2fdea
pub fn i_02fdcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FDD0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2FDEA; return 10; }
    8
}
/// $02FDD0: lea.l $29834(pc), a0
pub fn i_02fdd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FDD4;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x29834;
    16
}
/// $02FDD4: move.l a0, $4c(a6)
pub fn i_02fdd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FDD8;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02FDD8: jsr $283ca.l
pub fn i_02fdd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FDDE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2FDDE);
    cpu.pc = 0x283CA;
    16
}
/// $02FDDE: jsr $283ca.l
pub fn i_02fdde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FDE4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2FDE4);
    cpu.pc = 0x283CA;
    16
}
/// $02FDE4: jsr $283d8.l
pub fn i_02fde4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FDEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2FDEA);
    cpu.pc = 0x283D8;
    16
}
/// $02FDEA: jsr $2a4ec(pc)
pub fn i_02fdea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FDEE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2FDEE);
    cpu.pc = 0x2A4EC;
    16
}
/// $02FDEE: bcs.w $2fe0c
pub fn i_02fdee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FDF2;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2FE0C; return 10; }
    8
}
/// $02FDF2: bra.w $2fdf6
pub fn i_02fdf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FDF6;
    cpu.ir = 0x6000;
    cpu.pc = 0x2FDF6;
    10
}
/// $02FDF6: tst.b $92(a6)
pub fn i_02fdf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FDFA;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x92);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $02FDFA: beq.w $2fe0c
pub fn i_02fdfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FDFE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2FE0C; return 10; }
    8
}
/// $02FDFE: tst.w $36(a6)
pub fn i_02fdfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FE02;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $02FE02: ble.w $2fe0c
pub fn i_02fe02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FE06;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x2FE0C; return 10; }
    8
}
/// $02FE06: lea.l $2bba4(pc), a1
pub fn i_02fe06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FE0A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2BBA4;
    16
}
/// $02FE0A: move.l a1, (a6)
pub fn i_02fe0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FE0C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02FE0C: tst.b $3b(a6)
pub fn i_02fe0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FE10;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x3B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $02FE10: bne.w $2fe1a
pub fn i_02fe10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FE14;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2FE1A; return 10; }
    8
}
/// $02FE14: jsr $283d8.l
pub fn i_02fe14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FE1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2FE1A);
    cpu.pc = 0x283D8;
    16
}
/// $02FE1A: jsr $2fb04(pc)
pub fn i_02fe1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FE1E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2FE1E);
    cpu.pc = 0x2FB04;
    16
}
/// $02FE1E: bcc.w $2fe28
pub fn i_02fe1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FE22;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2FE28; return 10; }
    8
}
/// $02FE22: lea.l $2fe6a(pc), a1
pub fn i_02fe22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FE26;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2FE6A;
    16
}
/// $02FE26: move.l a1, (a6)
pub fn i_02fe26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FE28;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02FE28: jsr $2ffb0(pc)
pub fn i_02fe28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FE2C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2FE2C);
    cpu.pc = 0x2FFB0;
    16
}
/// $02FE2C: tst.b $92(a6)
pub fn i_02fe2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FE30;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x92);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $02FE30: bne.w $2fe46
pub fn i_02fe30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FE34;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2FE46; return 10; }
    8
}
/// $02FE34: clr.b $92(a6)
pub fn i_02fe34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FE38;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x92);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $02FE38: clr.w $36(a6)
pub fn i_02fe38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FE3C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x36);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02FE3C: lea.l $2fe6a(pc), a1
pub fn i_02fe3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FE40;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2FE6A;
    16
}
/// $02FE40: move.l a1, (a6)
pub fn i_02fe40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FE42;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02FE42: bra.w $2fe4c
pub fn i_02fe42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FE46;
    cpu.ir = 0x6000;
    cpu.pc = 0x2FE4C;
    10
}
/// $02FE46: subi.b #$1, $92(a6)
pub fn i_02fe46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FE4C;
    cpu.ir = 0x42E;
    let ad1 = cpu.a[6].wrapping_add(0x92);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $02FE4C: cmpi.w #$117, $22(a6)
pub fn i_02fe4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FE52;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x117, Size::Word, false);
    16
}
/// $02FE52: blt.w $2fe5c
pub fn i_02fe52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FE56;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x2FE5C; return 10; }
    8
}
/// $02FE56: lea.l $2fe6a(pc), a1
pub fn i_02fe56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FE5A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2FE6A;
    16
}
/// $02FE5A: move.l a1, (a6)
pub fn i_02fe5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FE5C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02FE5C: clr.w $2a(a6)
pub fn i_02fe5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FE60;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02FE60: clr.w $2e(a6)
pub fn i_02fe60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FE64;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
// ===== JsrPcThunk_02fe64  ($02FE64, 6 B, thunks/jsr_pc_thunks.c)
/// $02FE64: jsr $2ac80(pc)
pub fn i_02fe64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FE68;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2FE68);
    cpu.pc = 0x2AC80;
    16
}
/// $02FE68: rts
pub fn i_02fe68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FE6A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_SelfDestructAttack_02fe6a  ($02FE6A, 106 B, slug/vehicle_02dd20.s)
/// $02FE6A: bclr.b #$5, $8c(a6)
pub fn i_02fe6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FE70;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02FE70: lea.l $2fe76(pc), a1
pub fn i_02fe70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FE74;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2FE76;
    16
}
/// $02FE74: move.l a1, (a6)
pub fn i_02fe74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FE76;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02FE76: jsr $2aa24(pc)
pub fn i_02fe76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FE7A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2FE7A);
    cpu.pc = 0x2AA24;
    16
}
/// $02FE7A: bset.b #$4, $8d(a6)
pub fn i_02fe7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FE80;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02FE80: move.b #$1, $10a2d1.l
pub fn i_02fe80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FE88;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2D1, v1 as u8);
    16
}
/// $02FE88: lea.l $29790(pc), a0
pub fn i_02fe88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FE8C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x29790;
    16
}
/// $02FE8C: move.l a0, $4c(a6)
pub fn i_02fe8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FE90;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02FE90: jsr $283ca.l
pub fn i_02fe90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FE96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2FE96);
    cpu.pc = 0x283CA;
    16
}
/// $02FE96: jsr $283ca.l
pub fn i_02fe96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FE9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2FE9C);
    cpu.pc = 0x283CA;
    16
}
/// $02FE9C: jsr $283d8.l
pub fn i_02fe9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FEA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2FEA2);
    cpu.pc = 0x283D8;
    16
}
/// $02FEA2: lea.l $ffff.w, a0
pub fn i_02fea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FEA6;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $02FEA6: move.l a0, $4c(a6)
pub fn i_02fea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FEAA;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02FEAA: jsr $283ca.l
pub fn i_02feaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FEB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2FEB0);
    cpu.pc = 0x283CA;
    16
}
/// $02FEB0: jsr $2ff22(pc)
pub fn i_02feb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FEB4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2FEB4);
    cpu.pc = 0x2FF22;
    16
}
/// $02FEB4: lea.l $2ff4a(pc), a1
pub fn i_02feb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FEB8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2FF4A;
    16
}
/// $02FEB8: jsr $4ae.l
pub fn i_02feb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FEBE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2FEBE);
    cpu.pc = 0x4AE;
    16
}
/// $02FEBE: jsr $517fe.l
pub fn i_02febe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FEC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2FEC4);
    cpu.pc = 0x517FE;
    16
}
/// $02FEC4: jsr $2a752(pc)
pub fn i_02fec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FEC8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2FEC8);
    cpu.pc = 0x2A752;
    16
}
/// $02FEC8: jsr $28d70.l
pub fn i_02fec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FECE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2FECE);
    cpu.pc = 0x28D70;
    16
}
/// $02FECE: lea.l $2dc5c(pc), a1
pub fn i_02fece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FED2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DC5C;
    16
}
/// $02FED2: move.l a1, (a6)
pub fn i_02fed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FED4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrPcThunk_02fed4  ($02FED4, 6 B, thunks/jsr_pc_thunks.c)
/// $02FED4: jsr $2ac80(pc)
pub fn i_02fed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FED8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2FED8);
    cpu.pc = 0x2AC80;
    16
}
/// $02FED8: rts
pub fn i_02fed8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FEDA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_KillInit_02feda  ($02FEDA, 66 B, slug/vehicle_02dd20.s)
/// $02FEDA: clr.b $59(a6)
pub fn i_02feda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FEDE;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x59);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $02FEDE: move.b #$fe, $45(a6)
pub fn i_02fede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FEE4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFE;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02FEE4: clr.w $66(a6)
pub fn i_02fee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FEE8;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x66);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02FEE8: bset.b #$0, $13(a6)
pub fn i_02fee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FEEE;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02FEEE: jsr $13600.l
pub fn i_02feee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FEF4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2FEF4);
    cpu.pc = 0x13600;
    16
}
/// $02FEF4: move.w #$19c, d1
pub fn i_02fef4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FEF8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x19C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02FEF8: jsr $236e.l
pub fn i_02fef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FEFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2FEFE);
    cpu.pc = 0x236E;
    16
}
/// $02FEFE: move.w #$19d, d1
pub fn i_02fefe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FF02;
    cpu.ir = 0x323C;
    let v1: u32 = 0x19D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02FF02: jsr $236e.l
pub fn i_02ff02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FF08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2FF08);
    cpu.pc = 0x236E;
    16
}
/// $02FF08: move.w #$19e, d1
pub fn i_02ff08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FF0C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x19E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02FF0C: jsr $236e.l
pub fn i_02ff0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FF12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2FF12);
    cpu.pc = 0x236E;
    16
}
/// $02FF12: move.w #$19a, d1
pub fn i_02ff12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FF16;
    cpu.ir = 0x323C;
    let v1: u32 = 0x19A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02FF16: jsr $236e.l
pub fn i_02ff16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FF1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2FF1C);
    cpu.pc = 0x236E;
    16
}
// ===== JsrPcThunk_02ff1c  ($02FF1C, 6 B, thunks/jsr_pc_thunks.c)
/// $02FF1C: jsr $2fada(pc)
pub fn i_02ff1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FF20;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2FF20);
    cpu.pc = 0x2FADA;
    16
}
/// $02FF20: rts
pub fn i_02ff20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FF22;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_ExplodeFx_02ff22  ($02FF22, 32 B, slug/vehicle_02dd20.s)
/// $02FF22: move.w #$1034, d0
pub fn i_02ff22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FF26;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1034;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02FF26: jsr $2352.l
pub fn i_02ff26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FF2C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2FF2C);
    cpu.pc = 0x2352;
    16
}
/// $02FF2C: lea.l $2f672(pc), a1
pub fn i_02ff2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FF30;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2F672;
    16
}
/// $02FF30: jsr $4ae.l
pub fn i_02ff30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FF36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2FF36);
    cpu.pc = 0x4AE;
    16
}
/// $02FF36: jsr $5dd02.l
pub fn i_02ff36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FF3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2FF3C);
    cpu.pc = 0x5DD02;
    16
}
/// $02FF3C: jsr $8189c.l
pub fn i_02ff3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FF42;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2FF42);
    cpu.pc = 0x8189C;
    16
}
// ===== JsrAbsThunk_02ff42  ($02FF42, 8 B, thunks/jsr_abs_thunks.c)
/// $02FF42: jsr $818aa.l
pub fn i_02ff42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FF48;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2FF48);
    cpu.pc = 0x818AA;
    16
}
/// $02FF48: rts
pub fn i_02ff48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FF4A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_BlastAttack_02ff4a  ($02FF4A, 52 B, slug/vehicle_02dd20.s)
/// $02FF4A: move.w #$a0, $22(a6)
pub fn i_02ff4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FF50;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02FF50: move.w #$180, $24(a6)
pub fn i_02ff50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FF56;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02FF56: lea.l $298d8(pc), a0
pub fn i_02ff56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FF5A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x298D8;
    16
}
/// $02FF5A: move.l a0, $4c(a6)
pub fn i_02ff5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FF5E;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02FF5E: jsr $283ca.l
pub fn i_02ff5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FF64;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2FF64);
    cpu.pc = 0x283CA;
    16
}
/// $02FF64: jsr $283ca.l
pub fn i_02ff64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FF6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2FF6A);
    cpu.pc = 0x283CA;
    16
}
/// $02FF6A: jsr $283d8.l
pub fn i_02ff6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FF70;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2FF70);
    cpu.pc = 0x283D8;
    16
}
/// $02FF70: lea.l $ffff.w, a0
pub fn i_02ff70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FF74;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $02FF74: move.l a0, $4c(a6)
pub fn i_02ff74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FF78;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02FF78: jsr $283ca.l
pub fn i_02ff78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FF7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2FF7E);
    cpu.pc = 0x283CA;
    16
}
// ===== SetTaskHandler_02ff7e  ($02FF7E, 8 B, kernel/task_handlers.c)
/// $02FF7E: lea.l $2ff86(pc), a1
pub fn i_02ff7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FF82;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2FF86;
    16
}
/// $02FF82: move.l a1, (a6)
pub fn i_02ff82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FF84;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02FF84: rts
pub fn i_02ff84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FF86;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_02ff86  ($02FF86, 8 B, kernel/jmp_to_scheduler.c)
/// $02FF86: jmp $518.l
pub fn i_02ff86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FF8C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Slug_AngleIndex_02ff8e  ($02FF8E, 34 B, slug/vehicle_02dd20.s)
/// $02FF8E: lea.l $100580.l, a0
pub fn i_02ff8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FF94;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100580;
    20
}
/// $02FF94: move.w $80(a0), d2
pub fn i_02ff94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FF98;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02FF98: move.w d2, d0
pub fn i_02ff98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FF9A;
    cpu.ir = 0x3002;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02FF9A: asr.w #$1, d0
pub fn i_02ff9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FF9C;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02FF9C: neg.w d0
pub fn i_02ff9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FF9E;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02FF9E: addi.w #$20, d0
pub fn i_02ff9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FFA2;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02FFA2: cmpi.w #$40, d0
pub fn i_02ffa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FFA6;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $02FFA6: bls.w $2ffae
pub fn i_02ffa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FFAA;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x2FFAE; return 10; }
    8
}
/// $02FFAA: move.w #$40, d0
pub fn i_02ffaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FFAE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02FFAE: rts
pub fn i_02ffae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FFB0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_ClampField92_02ffb0  ($02FFB0, 42 B, slug/vehicle_02dd20.s)
/// $02FFB0: cmpi.w #$c0, $22(a6)
pub fn i_02ffb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FFB6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xC0, Size::Word, false);
    16
}
/// $02FFB6: blt.w $2ffe0
pub fn i_02ffb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FFBA;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x2FFE0; return 10; }
    8
}
/// $02FFBA: movea.l #$ffffffff, a0
pub fn i_02ffba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FFC0;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $02FFC0: jsr $5dd56.l
pub fn i_02ffc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FFC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2FFC6);
    cpu.pc = 0x5DD56;
    16
}
/// $02FFC6: bcc.w $2ffe0
pub fn i_02ffc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FFCA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2FFE0; return 10; }
    8
}
/// $02FFCA: cmpi.b #$a, $92(a6)
pub fn i_02ffca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FFD0;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x92);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xA, Size::Byte, false);
    16
}
/// $02FFD0: ble.w $2ffe0
pub fn i_02ffd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FFD4;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x2FFE0; return 10; }
    8
}
/// $02FFD4: move.b #$a, $92(a6)
pub fn i_02ffd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FFDA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x92);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== SetXN_02ffda  ($02FFDA, 6 B, thunks/ccr_helpers.c)
/// $02FFDA: ori.b #$11, ccr
pub fn i_02ffda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FFDE;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02FFDE: rts
pub fn i_02ffde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FFE0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_ClampField92_ClearXN_02ffe0  ($02FFE0, 6 B, slug/vehicle_02dd20.s)
/// $02FFE0: andi.b #$ee, ccr
pub fn i_02ffe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FFE4;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02FFE4: rts
pub fn i_02ffe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FFE6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CmpField10WithLink8_02ffe6  ($02FFE6, 16 B, slug/vehicle_02dd20.s)
/// $02FFE6: movea.l $8(a6), a1
pub fn i_02ffe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FFEA;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $02FFEA: move.b $10(a6), d0
pub fn i_02ffea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FFEE;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $02FFEE: cmp.b $10(a1), d0
pub fn i_02ffee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FFF2;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $02FFF2: bcs.w $2fffc
pub fn i_02fff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FFF6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2FFFC; return 10; }
    8
}
// ===== ClearXN_02fff6  ($02FFF6, 6 B, thunks/ccr_helpers.c)
/// $02FFF6: andi.b #$ee, ccr
pub fn i_02fff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FFFA;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02FFFA: rts
pub fn i_02fffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FFFC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CmpField10WithLink8_SetXN_02fffc  ($02FFFC, 6 B, slug/vehicle_02dd20.s)
/// $02FFFC: ori.b #$11, ccr
pub fn i_02fffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30000;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x02F152, i_02f152),
    (0x02F154, i_02f154),
    (0x02F15A, i_02f15a),
    (0x02F15E, i_02f15e),
    (0x02F164, i_02f164),
    (0x02F168, i_02f168),
    (0x02F16C, i_02f16c),
    (0x02F16E, i_02f16e),
    (0x02F172, i_02f172),
    (0x02F178, i_02f178),
    (0x02F17E, i_02f17e),
    (0x02F182, i_02f182),
    (0x02F186, i_02f186),
    (0x02F18A, i_02f18a),
    (0x02F18C, i_02f18c),
    (0x02F192, i_02f192),
    (0x02F198, i_02f198),
    (0x02F19C, i_02f19c),
    (0x02F1A0, i_02f1a0),
    (0x02F1A4, i_02f1a4),
    (0x02F1A8, i_02f1a8),
    (0x02F1AE, i_02f1ae),
    (0x02F1B4, i_02f1b4),
    (0x02F1BA, i_02f1ba),
    (0x02F1BE, i_02f1be),
    (0x02F1C2, i_02f1c2),
    (0x02F1C6, i_02f1c6),
    (0x02F1C8, i_02f1c8),
    (0x02F1CC, i_02f1cc),
    (0x02F1D0, i_02f1d0),
    (0x02F1D4, i_02f1d4),
    (0x02F1DA, i_02f1da),
    (0x02F1DE, i_02f1de),
    (0x02F1E2, i_02f1e2),
    (0x02F1E4, i_02f1e4),
    (0x02F1EA, i_02f1ea),
    (0x02F1EE, i_02f1ee),
    (0x02F1F2, i_02f1f2),
    (0x02F1F4, i_02f1f4),
    (0x02F1F8, i_02f1f8),
    (0x02F1FC, i_02f1fc),
    (0x02F200, i_02f200),
    (0x02F202, i_02f202),
    (0x02F204, i_02f204),
    (0x02F208, i_02f208),
    (0x02F20A, i_02f20a),
    (0x02F20E, i_02f20e),
    (0x02F214, i_02f214),
    (0x02F218, i_02f218),
    (0x02F21E, i_02f21e),
    (0x02F220, i_02f220),
    (0x02F224, i_02f224),
    (0x02F22A, i_02f22a),
    (0x02F22E, i_02f22e),
    (0x02F234, i_02f234),
    (0x02F238, i_02f238),
    (0x02F23E, i_02f23e),
    (0x02F242, i_02f242),
    (0x02F244, i_02f244),
    (0x02F248, i_02f248),
    (0x02F24C, i_02f24c),
    (0x02F24E, i_02f24e),
    (0x02F252, i_02f252),
    (0x02F256, i_02f256),
    (0x02F25A, i_02f25a),
    (0x02F25E, i_02f25e),
    (0x02F262, i_02f262),
    (0x02F264, i_02f264),
    (0x02F268, i_02f268),
    (0x02F26E, i_02f26e),
    (0x02F274, i_02f274),
    (0x02F278, i_02f278),
    (0x02F27C, i_02f27c),
    (0x02F280, i_02f280),
    (0x02F282, i_02f282),
    (0x02F286, i_02f286),
    (0x02F28A, i_02f28a),
    (0x02F28E, i_02f28e),
    (0x02F290, i_02f290),
    (0x02F296, i_02f296),
    (0x02F29A, i_02f29a),
    (0x02F29E, i_02f29e),
    (0x02F2A0, i_02f2a0),
    (0x02F2A6, i_02f2a6),
    (0x02F2AA, i_02f2aa),
    (0x02F2AE, i_02f2ae),
    (0x02F2B0, i_02f2b0),
    (0x02F2B6, i_02f2b6),
    (0x02F2BA, i_02f2ba),
    (0x02F2BE, i_02f2be),
    (0x02F2C2, i_02f2c2),
    (0x02F2C4, i_02f2c4),
    (0x02F2C8, i_02f2c8),
    (0x02F2CC, i_02f2cc),
    (0x02F2D0, i_02f2d0),
    (0x02F2D2, i_02f2d2),
    (0x02F2D6, i_02f2d6),
    (0x02F2DA, i_02f2da),
    (0x02F2DE, i_02f2de),
    (0x02F2E0, i_02f2e0),
    (0x02F2E4, i_02f2e4),
    (0x02F2E8, i_02f2e8),
    (0x02F2EC, i_02f2ec),
    (0x02F2EE, i_02f2ee),
    (0x02F2F2, i_02f2f2),
    (0x02F2F6, i_02f2f6),
    (0x02F2FA, i_02f2fa),
    (0x02F2FC, i_02f2fc),
    (0x02F300, i_02f300),
    (0x02F304, i_02f304),
    (0x02F30A, i_02f30a),
    (0x02F310, i_02f310),
    (0x02F312, i_02f312),
    (0x02F314, i_02f314),
    (0x02F318, i_02f318),
    (0x02F31A, i_02f31a),
    (0x02F31E, i_02f31e),
    (0x02F324, i_02f324),
    (0x02F328, i_02f328),
    (0x02F32E, i_02f32e),
    (0x02F330, i_02f330),
    (0x02F334, i_02f334),
    (0x02F33A, i_02f33a),
    (0x02F33E, i_02f33e),
    (0x02F344, i_02f344),
    (0x02F348, i_02f348),
    (0x02F34E, i_02f34e),
    (0x02F352, i_02f352),
    (0x02F354, i_02f354),
    (0x02F358, i_02f358),
    (0x02F35C, i_02f35c),
    (0x02F35E, i_02f35e),
    (0x02F362, i_02f362),
    (0x02F366, i_02f366),
    (0x02F36A, i_02f36a),
    (0x02F36E, i_02f36e),
    (0x02F372, i_02f372),
    (0x02F374, i_02f374),
    (0x02F378, i_02f378),
    (0x02F37E, i_02f37e),
    (0x02F384, i_02f384),
    (0x02F388, i_02f388),
    (0x02F38C, i_02f38c),
    (0x02F390, i_02f390),
    (0x02F392, i_02f392),
    (0x02F396, i_02f396),
    (0x02F39A, i_02f39a),
    (0x02F39E, i_02f39e),
    (0x02F3A0, i_02f3a0),
    (0x02F3A6, i_02f3a6),
    (0x02F3AA, i_02f3aa),
    (0x02F3AE, i_02f3ae),
    (0x02F3B0, i_02f3b0),
    (0x02F3B6, i_02f3b6),
    (0x02F3BA, i_02f3ba),
    (0x02F3BE, i_02f3be),
    (0x02F3C0, i_02f3c0),
    (0x02F3C6, i_02f3c6),
    (0x02F3CA, i_02f3ca),
    (0x02F3CE, i_02f3ce),
    (0x02F3D2, i_02f3d2),
    (0x02F3D4, i_02f3d4),
    (0x02F3D8, i_02f3d8),
    (0x02F3DC, i_02f3dc),
    (0x02F3E0, i_02f3e0),
    (0x02F3E2, i_02f3e2),
    (0x02F3E6, i_02f3e6),
    (0x02F3EA, i_02f3ea),
    (0x02F3EE, i_02f3ee),
    (0x02F3F0, i_02f3f0),
    (0x02F3F4, i_02f3f4),
    (0x02F3F8, i_02f3f8),
    (0x02F3FC, i_02f3fc),
    (0x02F3FE, i_02f3fe),
    (0x02F402, i_02f402),
    (0x02F406, i_02f406),
    (0x02F40A, i_02f40a),
    (0x02F40C, i_02f40c),
    (0x02F410, i_02f410),
    (0x02F414, i_02f414),
    (0x02F41A, i_02f41a),
    (0x02F420, i_02f420),
    (0x02F422, i_02f422),
    (0x02F428, i_02f428),
    (0x02F42C, i_02f42c),
    (0x02F430, i_02f430),
    (0x02F436, i_02f436),
    (0x02F438, i_02f438),
    (0x02F43C, i_02f43c),
    (0x02F442, i_02f442),
    (0x02F446, i_02f446),
    (0x02F44C, i_02f44c),
    (0x02F450, i_02f450),
    (0x02F456, i_02f456),
    (0x02F45A, i_02f45a),
    (0x02F45C, i_02f45c),
    (0x02F460, i_02f460),
    (0x02F464, i_02f464),
    (0x02F466, i_02f466),
    (0x02F46A, i_02f46a),
    (0x02F470, i_02f470),
    (0x02F476, i_02f476),
    (0x02F47A, i_02f47a),
    (0x02F47E, i_02f47e),
    (0x02F482, i_02f482),
    (0x02F484, i_02f484),
    (0x02F488, i_02f488),
    (0x02F48C, i_02f48c),
    (0x02F490, i_02f490),
    (0x02F492, i_02f492),
    (0x02F498, i_02f498),
    (0x02F49C, i_02f49c),
    (0x02F4A0, i_02f4a0),
    (0x02F4A2, i_02f4a2),
    (0x02F4A6, i_02f4a6),
    (0x02F4AA, i_02f4aa),
    (0x02F4AE, i_02f4ae),
    (0x02F4B0, i_02f4b0),
    (0x02F4B6, i_02f4b6),
    (0x02F4BC, i_02f4bc),
    (0x02F4C0, i_02f4c0),
    (0x02F4C4, i_02f4c4),
    (0x02F4C8, i_02f4c8),
    (0x02F4CA, i_02f4ca),
    (0x02F4CE, i_02f4ce),
    (0x02F4D2, i_02f4d2),
    (0x02F4D6, i_02f4d6),
    (0x02F4D8, i_02f4d8),
    (0x02F4DC, i_02f4dc),
    (0x02F4E0, i_02f4e0),
    (0x02F4E4, i_02f4e4),
    (0x02F4E6, i_02f4e6),
    (0x02F4EA, i_02f4ea),
    (0x02F4EE, i_02f4ee),
    (0x02F4F4, i_02f4f4),
    (0x02F4FA, i_02f4fa),
    (0x02F4FC, i_02f4fc),
    (0x02F502, i_02f502),
    (0x02F506, i_02f506),
    (0x02F50A, i_02f50a),
    (0x02F510, i_02f510),
    (0x02F512, i_02f512),
    (0x02F516, i_02f516),
    (0x02F51C, i_02f51c),
    (0x02F520, i_02f520),
    (0x02F526, i_02f526),
    (0x02F52A, i_02f52a),
    (0x02F530, i_02f530),
    (0x02F534, i_02f534),
    (0x02F536, i_02f536),
    (0x02F53A, i_02f53a),
    (0x02F53E, i_02f53e),
    (0x02F540, i_02f540),
    (0x02F544, i_02f544),
    (0x02F54A, i_02f54a),
    (0x02F550, i_02f550),
    (0x02F554, i_02f554),
    (0x02F558, i_02f558),
    (0x02F55C, i_02f55c),
    (0x02F55E, i_02f55e),
    (0x02F562, i_02f562),
    (0x02F566, i_02f566),
    (0x02F56A, i_02f56a),
    (0x02F56C, i_02f56c),
    (0x02F572, i_02f572),
    (0x02F576, i_02f576),
    (0x02F57A, i_02f57a),
    (0x02F57C, i_02f57c),
    (0x02F580, i_02f580),
    (0x02F584, i_02f584),
    (0x02F588, i_02f588),
    (0x02F58A, i_02f58a),
    (0x02F590, i_02f590),
    (0x02F596, i_02f596),
    (0x02F59A, i_02f59a),
    (0x02F59E, i_02f59e),
    (0x02F5A2, i_02f5a2),
    (0x02F5A4, i_02f5a4),
    (0x02F5A8, i_02f5a8),
    (0x02F5AC, i_02f5ac),
    (0x02F5B0, i_02f5b0),
    (0x02F5B2, i_02f5b2),
    (0x02F5B6, i_02f5b6),
    (0x02F5BA, i_02f5ba),
    (0x02F5BE, i_02f5be),
    (0x02F5C0, i_02f5c0),
    (0x02F5C4, i_02f5c4),
    (0x02F5C8, i_02f5c8),
    (0x02F5CE, i_02f5ce),
    (0x02F5D4, i_02f5d4),
    (0x02F5D6, i_02f5d6),
    (0x02F5DA, i_02f5da),
    (0x02F5E0, i_02f5e0),
    (0x02F5E2, i_02f5e2),
    (0x02F5E6, i_02f5e6),
    (0x02F5EC, i_02f5ec),
    (0x02F5F0, i_02f5f0),
    (0x02F5F6, i_02f5f6),
    (0x02F5FA, i_02f5fa),
    (0x02F600, i_02f600),
    (0x02F604, i_02f604),
    (0x02F606, i_02f606),
    (0x02F60A, i_02f60a),
    (0x02F60E, i_02f60e),
    (0x02F610, i_02f610),
    (0x02F64C, i_02f64c),
    (0x02F650, i_02f650),
    (0x02F656, i_02f656),
    (0x02F65C, i_02f65c),
    (0x02F662, i_02f662),
    (0x02F668, i_02f668),
    (0x02F66C, i_02f66c),
    (0x02F66E, i_02f66e),
    (0x02F672, i_02f672),
    (0x02F676, i_02f676),
    (0x02F67C, i_02f67c),
    (0x02F680, i_02f680),
    (0x02F686, i_02f686),
    (0x02F68C, i_02f68c),
    (0x02F690, i_02f690),
    (0x02F692, i_02f692),
    (0x02F698, i_02f698),
    (0x02F69E, i_02f69e),
    (0x02F6A2, i_02f6a2),
    (0x02F6A8, i_02f6a8),
    (0x02F6AE, i_02f6ae),
    (0x02F6B4, i_02f6b4),
    (0x02F6B8, i_02f6b8),
    (0x02F6BE, i_02f6be),
    (0x02F84A, i_02f84a),
    (0x02F850, i_02f850),
    (0x02F854, i_02f854),
    (0x02F858, i_02f858),
    (0x02F85A, i_02f85a),
    (0x02F860, i_02f860),
    (0x02F866, i_02f866),
    (0x02F86A, i_02f86a),
    (0x02F870, i_02f870),
    (0x02F876, i_02f876),
    (0x02F87A, i_02f87a),
    (0x02F87E, i_02f87e),
    (0x02F880, i_02f880),
    (0x02F884, i_02f884),
    (0x02FADA, i_02fada),
    (0x02FADE, i_02fade),
    (0x02FAE2, i_02fae2),
    (0x02FAE4, i_02fae4),
    (0x02FAEA, i_02faea),
    (0x02FAEE, i_02faee),
    (0x02FAF2, i_02faf2),
    (0x02FAF6, i_02faf6),
    (0x02FAFA, i_02fafa),
    (0x02FAFC, i_02fafc),
    (0x02FB00, i_02fb00),
    (0x02FB04, i_02fb04),
    (0x02FB08, i_02fb08),
    (0x02FB0C, i_02fb0c),
    (0x02FB0E, i_02fb0e),
    (0x02FB10, i_02fb10),
    (0x02FB14, i_02fb14),
    (0x02FB18, i_02fb18),
    (0x02FB1C, i_02fb1c),
    (0x02FB20, i_02fb20),
    (0x02FB22, i_02fb22),
    (0x02FB24, i_02fb24),
    (0x02FB28, i_02fb28),
    (0x02FB2A, i_02fb2a),
    (0x02FB2C, i_02fb2c),
    (0x02FB32, i_02fb32),
    (0x02FB34, i_02fb34),
    (0x02FB38, i_02fb38),
    (0x02FB3C, i_02fb3c),
    (0x02FB3E, i_02fb3e),
    (0x02FB44, i_02fb44),
    (0x02FB48, i_02fb48),
    (0x02FB4C, i_02fb4c),
    (0x02FB50, i_02fb50),
    (0x02FB54, i_02fb54),
    (0x02FB58, i_02fb58),
    (0x02FB5C, i_02fb5c),
    (0x02FB60, i_02fb60),
    (0x02FB64, i_02fb64),
    (0x02FB68, i_02fb68),
    (0x02FB6C, i_02fb6c),
    (0x02FB6E, i_02fb6e),
    (0x02FB72, i_02fb72),
    (0x02FB78, i_02fb78),
    (0x02FB7C, i_02fb7c),
    (0x02FB80, i_02fb80),
    (0x02FB8C, i_02fb8c),
    (0x02FB90, i_02fb90),
    (0x02FB9A, i_02fb9a),
    (0x02FB9E, i_02fb9e),
    (0x02FBA2, i_02fba2),
    (0x02FBA8, i_02fba8),
    (0x02FBAC, i_02fbac),
    (0x02FBB2, i_02fbb2),
    (0x02FBB6, i_02fbb6),
    (0x02FBBC, i_02fbbc),
    (0x02FBC0, i_02fbc0),
    (0x02FBC6, i_02fbc6),
    (0x02FBCA, i_02fbca),
    (0x02FBCC, i_02fbcc),
    (0x02FBD0, i_02fbd0),
    (0x02FBD6, i_02fbd6),
    (0x02FBDA, i_02fbda),
    (0x02FBDE, i_02fbde),
    (0x02FBE0, i_02fbe0),
    (0x02FBE4, i_02fbe4),
    (0x02FBE8, i_02fbe8),
    (0x02FBEE, i_02fbee),
    (0x02FBF2, i_02fbf2),
    (0x02FBF6, i_02fbf6),
    (0x02FBFA, i_02fbfa),
    (0x02FC00, i_02fc00),
    (0x02FC04, i_02fc04),
    (0x02FC08, i_02fc08),
    (0x02FC0E, i_02fc0e),
    (0x02FC10, i_02fc10),
    (0x02FC12, i_02fc12),
    (0x02FC16, i_02fc16),
    (0x02FC1A, i_02fc1a),
    (0x02FC1E, i_02fc1e),
    (0x02FC24, i_02fc24),
    (0x02FC2A, i_02fc2a),
    (0x02FC2E, i_02fc2e),
    (0x02FC32, i_02fc32),
    (0x02FC36, i_02fc36),
    (0x02FC3A, i_02fc3a),
    (0x02FC3E, i_02fc3e),
    (0x02FC42, i_02fc42),
    (0x02FC48, i_02fc48),
    (0x02FC4E, i_02fc4e),
    (0x02FC54, i_02fc54),
    (0x02FC58, i_02fc58),
    (0x02FC5E, i_02fc5e),
    (0x02FC62, i_02fc62),
    (0x02FC68, i_02fc68),
    (0x02FC6A, i_02fc6a),
    (0x02FC6E, i_02fc6e),
    (0x02FC70, i_02fc70),
    (0x02FC74, i_02fc74),
    (0x02FC7A, i_02fc7a),
    (0x02FC7E, i_02fc7e),
    (0x02FC84, i_02fc84),
    (0x02FC88, i_02fc88),
    (0x02FC8E, i_02fc8e),
    (0x02FC94, i_02fc94),
    (0x02FC9C, i_02fc9c),
    (0x02FCA2, i_02fca2),
    (0x02FCA6, i_02fca6),
    (0x02FCAA, i_02fcaa),
    (0x02FCAE, i_02fcae),
    (0x02FCB0, i_02fcb0),
    (0x02FCB2, i_02fcb2),
    (0x02FCB4, i_02fcb4),
    (0x02FCB8, i_02fcb8),
    (0x02FCBC, i_02fcbc),
    (0x02FCC0, i_02fcc0),
    (0x02FCC4, i_02fcc4),
    (0x02FCC8, i_02fcc8),
    (0x02FCCC, i_02fccc),
    (0x02FCD2, i_02fcd2),
    (0x02FCD4, i_02fcd4),
    (0x02FCD8, i_02fcd8),
    (0x02FCDE, i_02fcde),
    (0x02FCE2, i_02fce2),
    (0x02FCE8, i_02fce8),
    (0x02FCEC, i_02fcec),
    (0x02FCF2, i_02fcf2),
    (0x02FCF4, i_02fcf4),
    (0x02FCF8, i_02fcf8),
    (0x02FCFC, i_02fcfc),
    (0x02FD00, i_02fd00),
    (0x02FD04, i_02fd04),
    (0x02FD0C, i_02fd0c),
    (0x02FD14, i_02fd14),
    (0x02FD18, i_02fd18),
    (0x02FD1A, i_02fd1a),
    (0x02FD20, i_02fd20),
    (0x02FD24, i_02fd24),
    (0x02FD28, i_02fd28),
    (0x02FD30, i_02fd30),
    (0x02FD34, i_02fd34),
    (0x02FD38, i_02fd38),
    (0x02FD3C, i_02fd3c),
    (0x02FD3E, i_02fd3e),
    (0x02FD40, i_02fd40),
    (0x02FD44, i_02fd44),
    (0x02FD46, i_02fd46),
    (0x02FD48, i_02fd48),
    (0x02FD4A, i_02fd4a),
    (0x02FD4E, i_02fd4e),
    (0x02FD52, i_02fd52),
    (0x02FD56, i_02fd56),
    (0x02FD5A, i_02fd5a),
    (0x02FD5E, i_02fd5e),
    (0x02FD62, i_02fd62),
    (0x02FD66, i_02fd66),
    (0x02FD68, i_02fd68),
    (0x02FD6A, i_02fd6a),
    (0x02FD6C, i_02fd6c),
    (0x02FD70, i_02fd70),
    (0x02FD74, i_02fd74),
    (0x02FD78, i_02fd78),
    (0x02FD7C, i_02fd7c),
    (0x02FD80, i_02fd80),
    (0x02FD84, i_02fd84),
    (0x02FD8A, i_02fd8a),
    (0x02FD8C, i_02fd8c),
    (0x02FD90, i_02fd90),
    (0x02FD96, i_02fd96),
    (0x02FD9A, i_02fd9a),
    (0x02FDA0, i_02fda0),
    (0x02FDA4, i_02fda4),
    (0x02FDAA, i_02fdaa),
    (0x02FDAC, i_02fdac),
    (0x02FDB0, i_02fdb0),
    (0x02FDB4, i_02fdb4),
    (0x02FDB8, i_02fdb8),
    (0x02FDBC, i_02fdbc),
    (0x02FDC2, i_02fdc2),
    (0x02FDC8, i_02fdc8),
    (0x02FDCC, i_02fdcc),
    (0x02FDD0, i_02fdd0),
    (0x02FDD4, i_02fdd4),
    (0x02FDD8, i_02fdd8),
    (0x02FDDE, i_02fdde),
    (0x02FDE4, i_02fde4),
    (0x02FDEA, i_02fdea),
    (0x02FDEE, i_02fdee),
    (0x02FDF2, i_02fdf2),
    (0x02FDF6, i_02fdf6),
    (0x02FDFA, i_02fdfa),
    (0x02FDFE, i_02fdfe),
    (0x02FE02, i_02fe02),
    (0x02FE06, i_02fe06),
    (0x02FE0A, i_02fe0a),
    (0x02FE0C, i_02fe0c),
    (0x02FE10, i_02fe10),
    (0x02FE14, i_02fe14),
    (0x02FE1A, i_02fe1a),
    (0x02FE1E, i_02fe1e),
    (0x02FE22, i_02fe22),
    (0x02FE26, i_02fe26),
    (0x02FE28, i_02fe28),
    (0x02FE2C, i_02fe2c),
    (0x02FE30, i_02fe30),
    (0x02FE34, i_02fe34),
    (0x02FE38, i_02fe38),
    (0x02FE3C, i_02fe3c),
    (0x02FE40, i_02fe40),
    (0x02FE42, i_02fe42),
    (0x02FE46, i_02fe46),
    (0x02FE4C, i_02fe4c),
    (0x02FE52, i_02fe52),
    (0x02FE56, i_02fe56),
    (0x02FE5A, i_02fe5a),
    (0x02FE5C, i_02fe5c),
    (0x02FE60, i_02fe60),
    (0x02FE64, i_02fe64),
    (0x02FE68, i_02fe68),
    (0x02FE6A, i_02fe6a),
    (0x02FE70, i_02fe70),
    (0x02FE74, i_02fe74),
    (0x02FE76, i_02fe76),
    (0x02FE7A, i_02fe7a),
    (0x02FE80, i_02fe80),
    (0x02FE88, i_02fe88),
    (0x02FE8C, i_02fe8c),
    (0x02FE90, i_02fe90),
    (0x02FE96, i_02fe96),
    (0x02FE9C, i_02fe9c),
    (0x02FEA2, i_02fea2),
    (0x02FEA6, i_02fea6),
    (0x02FEAA, i_02feaa),
    (0x02FEB0, i_02feb0),
    (0x02FEB4, i_02feb4),
    (0x02FEB8, i_02feb8),
    (0x02FEBE, i_02febe),
    (0x02FEC4, i_02fec4),
    (0x02FEC8, i_02fec8),
    (0x02FECE, i_02fece),
    (0x02FED2, i_02fed2),
    (0x02FED4, i_02fed4),
    (0x02FED8, i_02fed8),
    (0x02FEDA, i_02feda),
    (0x02FEDE, i_02fede),
    (0x02FEE4, i_02fee4),
    (0x02FEE8, i_02fee8),
    (0x02FEEE, i_02feee),
    (0x02FEF4, i_02fef4),
    (0x02FEF8, i_02fef8),
    (0x02FEFE, i_02fefe),
    (0x02FF02, i_02ff02),
    (0x02FF08, i_02ff08),
    (0x02FF0C, i_02ff0c),
    (0x02FF12, i_02ff12),
    (0x02FF16, i_02ff16),
    (0x02FF1C, i_02ff1c),
    (0x02FF20, i_02ff20),
    (0x02FF22, i_02ff22),
    (0x02FF26, i_02ff26),
    (0x02FF2C, i_02ff2c),
    (0x02FF30, i_02ff30),
    (0x02FF36, i_02ff36),
    (0x02FF3C, i_02ff3c),
    (0x02FF42, i_02ff42),
    (0x02FF48, i_02ff48),
    (0x02FF4A, i_02ff4a),
    (0x02FF50, i_02ff50),
    (0x02FF56, i_02ff56),
    (0x02FF5A, i_02ff5a),
    (0x02FF5E, i_02ff5e),
    (0x02FF64, i_02ff64),
    (0x02FF6A, i_02ff6a),
    (0x02FF70, i_02ff70),
    (0x02FF74, i_02ff74),
    (0x02FF78, i_02ff78),
    (0x02FF7E, i_02ff7e),
    (0x02FF82, i_02ff82),
    (0x02FF84, i_02ff84),
    (0x02FF86, i_02ff86),
    (0x02FF8E, i_02ff8e),
    (0x02FF94, i_02ff94),
    (0x02FF98, i_02ff98),
    (0x02FF9A, i_02ff9a),
    (0x02FF9C, i_02ff9c),
    (0x02FF9E, i_02ff9e),
    (0x02FFA2, i_02ffa2),
    (0x02FFA6, i_02ffa6),
    (0x02FFAA, i_02ffaa),
    (0x02FFAE, i_02ffae),
    (0x02FFB0, i_02ffb0),
    (0x02FFB6, i_02ffb6),
    (0x02FFBA, i_02ffba),
    (0x02FFC0, i_02ffc0),
    (0x02FFC6, i_02ffc6),
    (0x02FFCA, i_02ffca),
    (0x02FFD0, i_02ffd0),
    (0x02FFD4, i_02ffd4),
    (0x02FFDA, i_02ffda),
    (0x02FFDE, i_02ffde),
    (0x02FFE0, i_02ffe0),
    (0x02FFE4, i_02ffe4),
    (0x02FFE6, i_02ffe6),
    (0x02FFEA, i_02ffea),
    (0x02FFEE, i_02ffee),
    (0x02FFF2, i_02fff2),
    (0x02FFF6, i_02fff6),
    (0x02FFFA, i_02fffa),
    (0x02FFFC, i_02fffc),
];
