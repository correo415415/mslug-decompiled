// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p09_3) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $099CD4: clr.b $76(a6)
pub fn i_099cd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99CD8;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $099CD8: jsr $99fd2(pc)
pub fn i_099cd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99CDC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x99CDC);
    cpu.pc = 0x99FD2;
    16
}
/// $099CDC: jsr $99ff2(pc)
pub fn i_099cdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99CE0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x99CE0);
    cpu.pc = 0x99FF2;
    16
}
/// $099CE0: jsr $9a086(pc)
pub fn i_099ce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99CE4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x99CE4);
    cpu.pc = 0x9A086;
    16
}
/// $099CE4: jsr $99f3a(pc)
pub fn i_099ce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99CE8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x99CE8);
    cpu.pc = 0x99F3A;
    16
}
/// $099CE8: lea.l $99cee(pc), a1
pub fn i_099ce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99CEC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99CEE;
    16
}
/// $099CEC: move.l a1, (a6)
pub fn i_099cec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99CEE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $099CEE: jsr $99ed2(pc)
pub fn i_099cee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99CF2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x99CF2);
    cpu.pc = 0x99ED2;
    16
}
/// $099CF2: btst.b #$4, d0
pub fn i_099cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99CF6;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((4) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $099CF6: beq.w $99d50
pub fn i_099cf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99CFA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x99D50; return 10; }
    8
}
/// $099CFA: move.w $7a(a6), d0
pub fn i_099cfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99CFE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $099CFE: cmp.w $78(a6), d0
pub fn i_099cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99D02;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $099D02: bne.w $99d50
pub fn i_099d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99D06;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x99D50; return 10; }
    8
}
/// $099D06: lea.l $2f624c.l, a1
pub fn i_099d06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99D0C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2F624C;
    20
}
/// $099D0C: clr.w d0
pub fn i_099d0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99D0E;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $099D0E: move.b $72(a6), d0
pub fn i_099d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99D12;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $099D12: move.b (a1, d0.w), $10fd8b.l
pub fn i_099d12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99D1A;
    cpu.ir = 0x13F1;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10FD8B, v3 as u8);
    26
}
/// $099D1A: move.b $73(a6), $10fd88.l
pub fn i_099d1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99D22;
    cpu.ir = 0x13EE;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10FD88, v3 as u8);
    24
}
/// $099D22: move.b $74(a6), d0
pub fn i_099d22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99D26;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $099D26: subq.b #$1, d0
pub fn i_099d26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99D28;
    cpu.ir = 0x5300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $099D28: move.b d0, $10e486.l
pub fn i_099d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99D2E;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E486, v1 as u8);
    16
}
/// $099D2E: cmpi.b #$2, $10fd83.l
pub fn i_099d2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99D36;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10FD83) as u32;
    sub_with_flags(cpu, v2, 0x2, Size::Byte, false);
    20
}
/// $099D36: bne.w $99d42
pub fn i_099d36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99D3A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x99D42; return 10; }
    8
}
/// $099D3A: move.b $76(a6), $10fd92.l
pub fn i_099d3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99D42;
    cpu.ir = 0x13EE;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10FD92, v3 as u8);
    24
}
/// $099D42: clr.b $106ed2.l
pub fn i_099d42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99D48;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106ED2) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106ED2, r2 as u8);
    16
}
/// $099D48: jmp $518.l
pub fn i_099d48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99D4E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $099D4E: rts
pub fn i_099d4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99D50;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $099D50: andi.b #$c, d0
pub fn i_099d50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99D54;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xC) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $099D54: beq.w $99d94
pub fn i_099d54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99D58;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x99D94; return 10; }
    8
}
/// $099D58: cmpi.w #$0, $78(a6)
pub fn i_099d58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99D5E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $099D5E: bne.w $99d68
pub fn i_099d5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99D62;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x99D68; return 10; }
    8
}
// ===== JsrPcThunk_099d62  ($099D62, 6 B, thunks/jsr_pc_thunks.c)
/// $099D62: jsr $99de4(pc)
pub fn i_099d62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99D66;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x99D66);
    cpu.pc = 0x99DE4;
    16
}
/// $099D66: rts
pub fn i_099d66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99D68;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== OptionsMenu_Row1_099d68  ($099D68, 10 B, frontend/hiscore_memcard_options_09773c.s)
/// $099D68: cmpi.w #$1, $78(a6)
pub fn i_099d68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99D6E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $099D6E: bne.w $99d78
pub fn i_099d6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99D72;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x99D78; return 10; }
    8
}
// ===== JsrPcThunk_099d72  ($099D72, 6 B, thunks/jsr_pc_thunks.c)
/// $099D72: jsr $99e14(pc)
pub fn i_099d72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99D76;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x99D76);
    cpu.pc = 0x99E14;
    16
}
/// $099D76: rts
pub fn i_099d76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99D78;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== OptionsMenu_Row2_099d78  ($099D78, 22 B, frontend/hiscore_memcard_options_09773c.s)
/// $099D78: cmpi.b #$2, $10fd83.l
pub fn i_099d78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99D80;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10FD83) as u32;
    sub_with_flags(cpu, v2, 0x2, Size::Byte, false);
    20
}
/// $099D80: bne.w $99d94
pub fn i_099d80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99D84;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x99D94; return 10; }
    8
}
/// $099D84: cmpi.w #$2, $78(a6)
pub fn i_099d84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99D8A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Word, false);
    16
}
/// $099D8A: bne.w $99d94
pub fn i_099d8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99D8E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x99D94; return 10; }
    8
}
// ===== JsrPcThunk_099d8e  ($099D8E, 6 B, thunks/jsr_pc_thunks.c)
/// $099D8E: jsr $99e9c(pc)
pub fn i_099d8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99D92;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x99D92);
    cpu.pc = 0x99E9C;
    16
}
/// $099D92: rts
pub fn i_099d92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99D94;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== OptionsMenu_NavDown_099d94  ($099D94, 36 B, frontend/hiscore_memcard_options_09773c.s)
/// $099D94: jsr $99ed2(pc)
pub fn i_099d94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99D98;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x99D98);
    cpu.pc = 0x99ED2;
    16
}
/// $099D98: btst.b #$1, d0
pub fn i_099d98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99D9C;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((1) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $099D9C: beq.w $99dbe
pub fn i_099d9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99DA0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x99DBE; return 10; }
    8
}
/// $099DA0: jsr $99f86(pc)
pub fn i_099da0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99DA4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x99DA4);
    cpu.pc = 0x99F86;
    16
}
/// $099DA4: addq.w #$1, $78(a6)
pub fn i_099da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99DA8;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $099DA8: move.w $78(a6), d0
pub fn i_099da8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99DAC;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $099DAC: cmp.w $7a(a6), d0
pub fn i_099dac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99DB0;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $099DB0: bls.w $99db8
pub fn i_099db0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99DB4;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x99DB8; return 10; }
    8
}
/// $099DB4: clr.w $78(a6)
pub fn i_099db4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99DB8;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
// ===== JsrPcThunk_099db8  ($099DB8, 6 B, thunks/jsr_pc_thunks.c)
/// $099DB8: jsr $99f3a(pc)
pub fn i_099db8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99DBC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x99DBC);
    cpu.pc = 0x99F3A;
    16
}
/// $099DBC: rts
pub fn i_099dbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99DBE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== OptionsMenu_NavUp_099dbe  ($099DBE, 30 B, frontend/hiscore_memcard_options_09773c.s)
/// $099DBE: jsr $99ed2(pc)
pub fn i_099dbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99DC2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x99DC2);
    cpu.pc = 0x99ED2;
    16
}
/// $099DC2: btst.b #$0, d0
pub fn i_099dc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99DC6;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $099DC6: beq.w $99de2
pub fn i_099dc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99DCA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x99DE2; return 10; }
    8
}
/// $099DCA: jsr $99f86(pc)
pub fn i_099dca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99DCE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x99DCE);
    cpu.pc = 0x99F86;
    16
}
/// $099DCE: subq.w #$1, $78(a6)
pub fn i_099dce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99DD2;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $099DD2: bcc.w $99ddc
pub fn i_099dd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99DD6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x99DDC; return 10; }
    8
}
/// $099DD6: move.w $7a(a6), $78(a6)
pub fn i_099dd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99DDC;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad4, v3 as u16);
    20
}
// ===== JsrPcThunk_099ddc  ($099DDC, 6 B, thunks/jsr_pc_thunks.c)
/// $099DDC: jsr $99f3a(pc)
pub fn i_099ddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99DE0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x99DE0);
    cpu.pc = 0x99F3A;
    16
}
/// $099DE0: rts
pub fn i_099de0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99DE2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== OptionsMenu_Rts_099de2  ($099DE2, 2 B, frontend/hiscore_memcard_options_09773c.s)
/// $099DE2: rts
pub fn i_099de2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99DE4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== OptionsMenu_AdjDifficulty_099de4  ($099DE4, 18 B, frontend/hiscore_memcard_options_09773c.s)
/// $099DE4: btst.b #$2, d0
pub fn i_099de4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99DE8;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((2) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $099DE8: beq.w $99dfc
pub fn i_099de8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99DEC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x99DFC; return 10; }
    8
}
/// $099DEC: subq.b #$1, $72(a6)
pub fn i_099dec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99DF0;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $099DF0: andi.b #$3, $72(a6)
pub fn i_099df0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99DF6;
    cpu.ir = 0x22E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0x3) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== JsrPcThunk_099df6  ($099DF6, 6 B, thunks/jsr_pc_thunks.c)
/// $099DF6: jsr $99fd2(pc)
pub fn i_099df6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99DFA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x99DFA);
    cpu.pc = 0x99FD2;
    16
}
/// $099DFA: rts
pub fn i_099dfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99DFC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== OptionsMenu_AdjDifficulty_Right_099dfc  ($099DFC, 18 B, frontend/hiscore_memcard_options_09773c.s)
/// $099DFC: btst.b #$3, d0
pub fn i_099dfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E00;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((3) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $099E00: beq.w $99e12
pub fn i_099e00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E04;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x99E12; return 10; }
    8
}
/// $099E04: addq.b #$1, $72(a6)
pub fn i_099e04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E08;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $099E08: andi.b #$3, $72(a6)
pub fn i_099e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E0E;
    cpu.ir = 0x22E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0x3) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== JsrPcThunk_099e0e  ($099E0E, 6 B, thunks/jsr_pc_thunks.c)
/// $099E0E: jsr $99fd2(pc)
pub fn i_099e0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E12;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x99E12);
    cpu.pc = 0x99FD2;
    16
}
/// $099E12: rts
pub fn i_099e12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E14;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== OptionsMenu_AdjLives_099e14  ($099E14, 28 B, frontend/hiscore_memcard_options_09773c.s)
/// $099E14: btst.b #$2, d0
pub fn i_099e14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E18;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((2) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $099E18: beq.w $99e36
pub fn i_099e18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E1C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x99E36; return 10; }
    8
}
/// $099E1C: subq.b #$1, $73(a6)
pub fn i_099e1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E20;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x73);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $099E20: cmpi.b #$1, $73(a6)
pub fn i_099e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E26;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $099E26: bcc.w $99e30
pub fn i_099e26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E2A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x99E30; return 10; }
    8
}
/// $099E2A: move.b #$1, $73(a6)
pub fn i_099e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E30;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== JsrPcThunk_099e30  ($099E30, 6 B, thunks/jsr_pc_thunks.c)
/// $099E30: jsr $99ff2(pc)
pub fn i_099e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E34;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x99E34);
    cpu.pc = 0x99FF2;
    16
}
/// $099E34: rts
pub fn i_099e34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E36;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== OptionsMenu_AdjLives_Right_099e36  ($099E36, 28 B, frontend/hiscore_memcard_options_09773c.s)
/// $099E36: btst.b #$3, d0
pub fn i_099e36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E3A;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((3) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $099E3A: beq.w $99e56
pub fn i_099e3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E3E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x99E56; return 10; }
    8
}
/// $099E3E: addq.b #$1, $73(a6)
pub fn i_099e3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E42;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x73);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $099E42: cmpi.b #$5, $73(a6)
pub fn i_099e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E48;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x5, Size::Byte, false);
    16
}
/// $099E48: bls.w $99e52
pub fn i_099e48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E4C;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x99E52; return 10; }
    8
}
/// $099E4C: move.b #$5, $73(a6)
pub fn i_099e4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E52;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== JsrPcThunk_099e52  ($099E52, 6 B, thunks/jsr_pc_thunks.c)
/// $099E52: jsr $99ff2(pc)
pub fn i_099e52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E56;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x99E56);
    cpu.pc = 0x99FF2;
    16
}
/// $099E56: rts
pub fn i_099e56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E58;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== OptionsMenu_AdjCredits_099e58  ($099E58, 28 B, frontend/hiscore_memcard_options_09773c.s)
/// $099E58: btst.b #$2, d0
pub fn i_099e58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E5C;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((2) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $099E5C: beq.w $99e7a
pub fn i_099e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E60;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x99E7A; return 10; }
    8
}
/// $099E60: subq.b #$1, $74(a6)
pub fn i_099e60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E64;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $099E64: cmpi.b #$1, $74(a6)
pub fn i_099e64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E6A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $099E6A: bcc.w $99e74
pub fn i_099e6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E6E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x99E74; return 10; }
    8
}
/// $099E6E: move.b #$1, $74(a6)
pub fn i_099e6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E74;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== JsrPcThunk_099e74  ($099E74, 6 B, thunks/jsr_pc_thunks.c)
/// $099E74: jsr $9a03c(pc)
pub fn i_099e74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E78;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x99E78);
    cpu.pc = 0x9A03C;
    16
}
/// $099E78: rts
pub fn i_099e78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E7A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== OptionsMenu_AdjCredits_Right_099e7a  ($099E7A, 28 B, frontend/hiscore_memcard_options_09773c.s)
/// $099E7A: btst.b #$3, d0
pub fn i_099e7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E7E;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((3) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $099E7E: beq.w $99e9a
pub fn i_099e7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E82;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x99E9A; return 10; }
    8
}
/// $099E82: addq.b #$1, $74(a6)
pub fn i_099e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E86;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $099E86: cmpi.b #$5, $74(a6)
pub fn i_099e86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E8C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x5, Size::Byte, false);
    16
}
/// $099E8C: bls.w $99e96
pub fn i_099e8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E90;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x99E96; return 10; }
    8
}
/// $099E90: move.b #$5, $74(a6)
pub fn i_099e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E96;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== JsrPcThunk_099e96  ($099E96, 6 B, thunks/jsr_pc_thunks.c)
/// $099E96: jsr $9a03c(pc)
pub fn i_099e96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E9A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x99E9A);
    cpu.pc = 0x9A03C;
    16
}
/// $099E9A: rts
pub fn i_099e9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E9C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== OptionsMenu_Adj2PMode_099e9c  ($099E9C, 28 B, frontend/hiscore_memcard_options_09773c.s)
/// $099E9C: cmpi.b #$2, $10fd83.l
pub fn i_099e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99EA4;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10FD83) as u32;
    sub_with_flags(cpu, v2, 0x2, Size::Byte, false);
    20
}
/// $099EA4: beq.w $99eaa
pub fn i_099ea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99EA8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x99EAA; return 10; }
    8
}
/// $099EA8: rts
pub fn i_099ea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99EAA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $099EAA: btst.b #$2, d0
pub fn i_099eaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99EAE;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((2) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $099EAE: beq.w $99ebe
pub fn i_099eae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99EB2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x99EBE; return 10; }
    8
}
/// $099EB2: move.b #$0, $76(a6)
pub fn i_099eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99EB8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== JsrPcThunk_099eb8  ($099EB8, 6 B, thunks/jsr_pc_thunks.c)
/// $099EB8: jsr $9a086(pc)
pub fn i_099eb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99EBC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x99EBC);
    cpu.pc = 0x9A086;
    16
}
/// $099EBC: rts
pub fn i_099ebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99EBE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== OptionsMenu_Adj2PMode_Right_099ebe  ($099EBE, 14 B, frontend/hiscore_memcard_options_09773c.s)
/// $099EBE: btst.b #$3, d0
pub fn i_099ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99EC2;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((3) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $099EC2: beq.w $99ed0
pub fn i_099ec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99EC6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x99ED0; return 10; }
    8
}
/// $099EC6: move.b #$1, $76(a6)
pub fn i_099ec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99ECC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== JsrPcThunk_099ecc  ($099ECC, 6 B, thunks/jsr_pc_thunks.c)
/// $099ECC: jsr $9a086(pc)
pub fn i_099ecc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99ED0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x99ED0);
    cpu.pc = 0x9A086;
    16
}
/// $099ED0: rts
pub fn i_099ed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99ED2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Input_PressedEither_099ed2  ($099ED2, 18 B, frontend/hiscore_memcard_options_09773c.s)
/// $099ED2: move.b $10e215.l, d0
pub fn i_099ed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99ED8;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10E215) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $099ED8: bne.w $99ee2
pub fn i_099ed8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99EDC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x99EE2; return 10; }
    8
}
/// $099EDC: move.b $10e21b.l, d0
pub fn i_099edc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99EE2;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10E21B) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $099EE2: rts
pub fn i_099ee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99EE4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== OptionSelect_DrawCursor_099ee4  ($099EE4, 44 B, frontend/hiscore_memcard_options_09773c.s)
/// $099EE4: lea.l $2f6232.l, a1
pub fn i_099ee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99EEA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2F6232;
    20
}
/// $099EEA: move.w $78(a6), d2
pub fn i_099eea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99EEE;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $099EEE: lsl.w #$1, d2
pub fn i_099eee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99EF0;
    cpu.ir = 0xE34A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $099EF0: move.w (a1, d2.w), d0
pub fn i_099ef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99EF4;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $099EF4: move.w #$237b, d1
pub fn i_099ef4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99EF8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x237B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $099EF8: movem.w d0-d1, $3c0000.l
pub fn i_099ef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F00;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $099F00: addi.w #$20, d0
pub fn i_099f00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F04;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $099F04: addq.w #$1, d1
pub fn i_099f04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F06;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $099F06: movem.w d0-d1, $3c0000.l
pub fn i_099f06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F0E;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $099F0E: rts
pub fn i_099f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F10;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== OptionSelect_EraseCursor_099f10  ($099F10, 42 B, frontend/hiscore_memcard_options_09773c.s)
/// $099F10: lea.l $2f6232.l, a1
pub fn i_099f10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F16;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2F6232;
    20
}
/// $099F16: move.w $78(a6), d2
pub fn i_099f16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F1A;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $099F1A: lsl.w #$1, d2
pub fn i_099f1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F1C;
    cpu.ir = 0xE34A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $099F1C: move.w (a1, d2.w), d0
pub fn i_099f1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F20;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $099F20: move.w #$2320, d1
pub fn i_099f20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F24;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2320;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $099F24: movem.w d0-d1, $3c0000.l
pub fn i_099f24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F2C;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $099F2C: addi.w #$20, d0
pub fn i_099f2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F30;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $099F30: movem.w d0-d1, $3c0000.l
pub fn i_099f30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F38;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $099F38: rts
pub fn i_099f38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F3A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FixGlyph16_DrawCursorA_099F3A  ($099F3A, 76 B, video/hud_fix_glyph16_099f3a.s)
/// $099F3A: movea.l $80(a6), a1
pub fn i_099f3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F3E;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $099F3E: move.w $78(a6), d2
pub fn i_099f3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F42;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $099F42: lsl.w #$1, d2
pub fn i_099f42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F44;
    cpu.ir = 0xE34A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $099F44: move.w (a1, d2.w), d0
pub fn i_099f44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F48;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $099F48: move.w #$4b22, d1
pub fn i_099f48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F4C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4B22;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $099F4C: movem.w d0-d1, $3c0000.l
pub fn i_099f4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F54;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $099F54: addi.w #$20, d0
pub fn i_099f54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F58;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $099F58: addq.w #$1, d1
pub fn i_099f58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F5A;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $099F5A: movem.w d0-d1, $3c0000.l
pub fn i_099f5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F62;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $099F62: subi.w #$1f, d0
pub fn i_099f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F66;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x1F, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $099F66: addi.w #$f, d1
pub fn i_099f66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F6A;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xF, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $099F6A: movem.w d0-d1, $3c0000.l
pub fn i_099f6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F72;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $099F72: addi.w #$20, d0
pub fn i_099f72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F76;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $099F76: addq.w #$1, d1
pub fn i_099f76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F78;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $099F78: movem.w d0-d1, $3c0000.l
pub fn i_099f78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F80;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $099F80: subi.w #$21, d0
pub fn i_099f80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F84;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x21, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $099F84: rts
pub fn i_099f84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F86;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FixGlyph16_DrawCursorB_099F86  ($099F86, 76 B, video/hud_fix_glyph16_099f3a.s)
/// $099F86: movea.l $80(a6), a1
pub fn i_099f86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F8A;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $099F8A: move.w $78(a6), d2
pub fn i_099f8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F8E;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $099F8E: lsl.w #$1, d2
pub fn i_099f8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F90;
    cpu.ir = 0xE34A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $099F90: move.w (a1, d2.w), d0
pub fn i_099f90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F94;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $099F94: move.w #$4b40, d1
pub fn i_099f94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F98;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4B40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $099F98: movem.w d0-d1, $3c0000.l
pub fn i_099f98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99FA0;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $099FA0: addi.w #$20, d0
pub fn i_099fa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99FA4;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $099FA4: addq.w #$1, d1
pub fn i_099fa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99FA6;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $099FA6: movem.w d0-d1, $3c0000.l
pub fn i_099fa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99FAE;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $099FAE: subi.w #$1f, d0
pub fn i_099fae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99FB2;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x1F, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $099FB2: addi.w #$f, d1
pub fn i_099fb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99FB6;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xF, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $099FB6: movem.w d0-d1, $3c0000.l
pub fn i_099fb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99FBE;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $099FBE: addi.w #$20, d0
pub fn i_099fbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99FC2;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $099FC2: addq.w #$1, d1
pub fn i_099fc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99FC4;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $099FC4: movem.w d0-d1, $3c0000.l
pub fn i_099fc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99FCC;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $099FCC: subi.w #$21, d0
pub fn i_099fcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99FD0;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x21, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $099FD0: rts
pub fn i_099fd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99FD2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FixGlyphRun_Draw2F61F0_099FD2  ($099FD2, 24 B, video/hud_fix_glyph16_099f3a.s)
/// $099FD2: clr.l d0
pub fn i_099fd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99FD4;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $099FD4: move.b $72(a6), d0
pub fn i_099fd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99FD8;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $099FD8: lsl.l #$3, d0
pub fn i_099fd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99FDA;
    cpu.ir = 0xE788;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Long, 1, true);
    cpu.d[0] = r;
    12
}
/// $099FDA: lea.l $2f61f0.l, a2
pub fn i_099fda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99FE0;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2F61F0;
    20
}
/// $099FE0: adda.l d0, a2
pub fn i_099fe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99FE2;
    cpu.ir = 0xD5C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[2] = cpu.a[2].wrapping_add(v1);
    8
}
/// $099FE2: movea.w #$72eb, a1
pub fn i_099fe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99FE6;
    cpu.ir = 0x327C;
    let v1: u32 = 0x72EB;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $099FE6: move.w #$4, d1
pub fn i_099fe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99FEA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_099fea  ($099FEA, 8 B, thunks/jsr_abs_thunks.c)
/// $099FEA: jsr $4784c.l
pub fn i_099fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99FF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99FF0);
    cpu.pc = 0x4784C;
    16
}
/// $099FF0: rts
pub fn i_099ff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99FF2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FixGlyph16_DrawDigit72EF_099FF2  ($099FF2, 74 B, video/hud_fix_glyph16_099f3a.s)
/// $099FF2: move.w #$72ef, d0
pub fn i_099ff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99FF6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x72EF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $099FF6: clr.w d1
pub fn i_099ff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99FF8;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $099FF8: move.b $73(a6), d1
pub fn i_099ff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99FFC;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $099FFC: lsl.w #$1, d1
pub fn i_099ffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99FFE;
    cpu.ir = 0xE349;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $099FFE: addi.w #$4b60, d1
pub fn i_099ffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A002;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x4B60, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09A002: movem.w d0-d1, $3c0000.l
pub fn i_09a002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A00A;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $09A00A: addi.w #$20, d0
pub fn i_09a00a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A00E;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09A00E: addq.w #$1, d1
pub fn i_09a00e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A010;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $09A010: movem.w d0-d1, $3c0000.l
pub fn i_09a010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A018;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $09A018: subi.w #$1f, d0
pub fn i_09a018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A01C;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x1F, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09A01C: addi.w #$f, d1
pub fn i_09a01c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A020;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xF, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09A020: movem.w d0-d1, $3c0000.l
pub fn i_09a020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A028;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $09A028: addi.w #$20, d0
pub fn i_09a028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A02C;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09A02C: addq.w #$1, d1
pub fn i_09a02c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A02E;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $09A02E: movem.w d0-d1, $3c0000.l
pub fn i_09a02e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A036;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $09A036: subi.w #$21, d0
pub fn i_09a036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A03A;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x21, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09A03A: rts
pub fn i_09a03a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A03C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FixGlyph16_DrawDigit72F3_09A03C  ($09A03C, 74 B, video/hud_fix_glyph16_099f3a.s)
/// $09A03C: move.w #$72f3, d0
pub fn i_09a03c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A040;
    cpu.ir = 0x303C;
    let v1: u32 = 0x72F3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09A040: clr.w d1
pub fn i_09a040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A042;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $09A042: move.b $74(a6), d1
pub fn i_09a042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A046;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09A046: lsl.w #$1, d1
pub fn i_09a046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A048;
    cpu.ir = 0xE349;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $09A048: addi.w #$4b60, d1
pub fn i_09a048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A04C;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x4B60, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09A04C: movem.w d0-d1, $3c0000.l
pub fn i_09a04c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A054;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $09A054: addi.w #$20, d0
pub fn i_09a054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A058;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09A058: addq.w #$1, d1
pub fn i_09a058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A05A;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $09A05A: movem.w d0-d1, $3c0000.l
pub fn i_09a05a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A062;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $09A062: subi.w #$1f, d0
pub fn i_09a062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A066;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x1F, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09A066: addi.w #$f, d1
pub fn i_09a066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A06A;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xF, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09A06A: movem.w d0-d1, $3c0000.l
pub fn i_09a06a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A072;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $09A072: addi.w #$20, d0
pub fn i_09a072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A076;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09A076: addq.w #$1, d1
pub fn i_09a076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A078;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $09A078: movem.w d0-d1, $3c0000.l
pub fn i_09a078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A080;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $09A080: subi.w #$21, d0
pub fn i_09a080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A084;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x21, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09A084: rts
pub fn i_09a084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A086;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FixGlyphRun_DrawPad2P_09A086  ($09A086, 46 B, video/hud_fix_glyph16_099f3a.s)
/// $09A086: cmpi.b #$2, $10fd83.l
pub fn i_09a086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A08E;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10FD83) as u32;
    sub_with_flags(cpu, v2, 0x2, Size::Byte, false);
    20
}
/// $09A08E: bne.w $9a0ba
pub fn i_09a08e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A092;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9A0BA; return 10; }
    8
}
/// $09A092: cmpi.b #$1, $76(a6)
pub fn i_09a092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A098;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $09A098: bne.w $9a0a6
pub fn i_09a098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A09C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9A0A6; return 10; }
    8
}
/// $09A09C: lea.l $2f6226.l, a2
pub fn i_09a09c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A0A2;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2F6226;
    20
}
/// $09A0A2: bra.w $9a0ac
pub fn i_09a0a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A0A6;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A0AC;
    10
}
/// $09A0A6: lea.l $2f621a.l, a2
pub fn i_09a0a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A0AC;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2F621A;
    20
}
/// $09A0AC: movea.w #$72f3, a1
pub fn i_09a0ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A0B0;
    cpu.ir = 0x327C;
    let v1: u32 = 0x72F3;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $09A0B0: move.w #$4, d1
pub fn i_09a0b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A0B4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_09a0b4  ($09A0B4, 8 B, thunks/jsr_abs_thunks.c)
/// $09A0B4: jsr $477fc.l
pub fn i_09a0b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A0BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A0BA);
    cpu.pc = 0x477FC;
    16
}
/// $09A0BA: rts
pub fn i_09a0ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A0BC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CmpPrioWithSibling_09a0bc  ($09A0BC, 16 B, props/items_score_crates_09a0bc.s)
/// $09A0BC: movea.l $8(a6), a1
pub fn i_09a0bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A0C0;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $09A0C0: move.b $10(a6), d0
pub fn i_09a0c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A0C4;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09A0C4: cmp.b $10(a1), d0
pub fn i_09a0c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A0C8;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $09A0C8: bcs.w $9a0d2
pub fn i_09a0c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A0CC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x9A0D2; return 10; }
    8
}
// ===== ClearXN_09a0cc  ($09A0CC, 6 B, thunks/ccr_helpers.c)
/// $09A0CC: andi.b #$ee, ccr
pub fn i_09a0cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A0D0;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $09A0D0: rts
pub fn i_09a0d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A0D2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_09a0d2  ($09A0D2, 6 B, thunks/ccr_helpers.c)
/// $09A0D2: ori.b #$11, ccr
pub fn i_09a0d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A0D6;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $09A0D6: rts
pub fn i_09a0d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A0D8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gun_GroundTail_09a0e0  ($09A0E0, 14 B, props/items_score_crates_09a0bc.s)
/// $09A0E0: lea.l $9a0d8(pc), a0
pub fn i_09a0e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A0E4;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x9A0D8;
    16
}
/// $09A0E4: jsr $5dd5c.l
pub fn i_09a0e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A0EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A0EA);
    cpu.pc = 0x5DD5C;
    16
}
/// $09A0EA: bcc.w $9a0fa
pub fn i_09a0ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A0EE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9A0FA; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_09a0ee  ($09A0EE, 14 B, kernel/jsr5b6_jmp518.c)
/// $09A0EE: jsr $5b6.l
pub fn i_09a0ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A0F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A0F4);
    cpu.pc = 0x5B6;
    16
}
/// $09A0F4: jmp $518.l
pub fn i_09a0f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A0FA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $09A0FA: rts
pub fn i_09a0fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A0FC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gun_UpdateAngleSprite_09a0fc  ($09A0FC, 138 B, props/items_score_crates_09a0bc.s)
/// $09A0FC: move.w $70(a6), d0
pub fn i_09a0fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A100;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09A100: asr.w #$8, d0
pub fn i_09a100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A102;
    cpu.ir = 0xE040;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $09A102: addq.w #$4, d0
pub fn i_09a102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A104;
    cpu.ir = 0x5840;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $09A104: andi.w #$f8, d0
pub fn i_09a104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A108;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF8) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09A108: cmpi.b #$88, d0
pub fn i_09a108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A10C;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x88, Size::Byte, false);
    8
}
/// $09A10C: bcs.w $9a112
pub fn i_09a10c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A110;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x9A112; return 10; }
    8
}
/// $09A110: neg.b d0
pub fn i_09a110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A112;
    cpu.ir = 0x4400;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.set(StatusRegister::V, v1 == 0x80); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $09A112: cmpi.w #$40, d0
pub fn i_09a112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A116;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $09A116: bcs.w $9a12e
pub fn i_09a116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A11A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x9A12E; return 10; }
    8
}
/// $09A11A: bset.b #$0, $3a(a6)
pub fn i_09a11a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A120;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $09A120: subi.b #$40, d0
pub fn i_09a120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A124;
    cpu.ir = 0x400;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x40, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $09A124: neg.b d0
pub fn i_09a124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A126;
    cpu.ir = 0x4400;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.set(StatusRegister::V, v1 == 0x80); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $09A126: addi.b #$40, d0
pub fn i_09a126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A12A;
    cpu.ir = 0x600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x40, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $09A12A: bra.w $9a134
pub fn i_09a12a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A12E;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A134;
    10
}
/// $09A12E: bclr.b #$0, $3a(a6)
pub fn i_09a12e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A134;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $09A134: andi.w #$78, d0
pub fn i_09a134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A138;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x78) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09A138: lea.l $2f6540.l, a0
pub fn i_09a138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A13E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F6540;
    20
}
/// $09A13E: movea.l (a0, d0.w), a1
pub fn i_09a13e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A142;
    cpu.ir = 0x2270;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    18
}
/// $09A142: move.b $7(a0, d0.w), d0
pub fn i_09a142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A146;
    cpu.ir = 0x1030;
    let ad2 = cpu.a[0].wrapping_add(0x7).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $09A146: btst.b #$0, $3a(a6)
pub fn i_09a146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A14C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $09A14C: beq.w $9a15a
pub fn i_09a14c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A150;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9A15A; return 10; }
    8
}
/// $09A150: subi.b #$40, d0
pub fn i_09a150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A154;
    cpu.ir = 0x400;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x40, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $09A154: neg.b d0
pub fn i_09a154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A156;
    cpu.ir = 0x4400;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.set(StatusRegister::V, v1 == 0x80); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $09A156: addi.b #$40, d0
pub fn i_09a156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A15A;
    cpu.ir = 0x600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x40, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $09A15A: move.b d0, $82(a6)
pub fn i_09a15a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A15E;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09A15E: moveq #$0, d0
pub fn i_09a15e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A160;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $09A160: move.b $3b(a6), d0
pub fn i_09a160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A164;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x3B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09A164: beq.w $9a16e
pub fn i_09a164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A168;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9A16E; return 10; }
    8
}
/// $09A168: subq.b #$1, d0
pub fn i_09a168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A16A;
    cpu.ir = 0x5300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $09A16A: move.b d0, $3b(a6)
pub fn i_09a16a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A16E;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09A16E: add.w d0, d0
pub fn i_09a16e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A170;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09A170: add.w d0, d0
pub fn i_09a170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A172;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09A172: add.w d0, d0
pub fn i_09a172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A174;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09A174: move.l (a1, d0.w), $74(a6)
pub fn i_09a174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A17A;
    cpu.ir = 0x2D71;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    30
}
/// $09A17A: move.l $4(a1, d0.w), $3c(a6)
pub fn i_09a17a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A180;
    cpu.ir = 0x2D71;
    let ad2 = cpu.a[1].wrapping_add(0x4).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    30
}
/// $09A180: jmp $5ca2a.l
pub fn i_09a180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A186;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x5CA2A;
    8
}
// ===== Gun_RegisterTarget_09a186  ($09A186, 48 B, props/items_score_crates_09a0bc.s)
/// $09A186: move.b $70(a6), d0
pub fn i_09a186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A18A;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09A18A: andi.w #$f8, d0
pub fn i_09a18a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A18E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF8) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09A18E: cmpi.b #$88, d0
pub fn i_09a18e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A192;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x88, Size::Byte, false);
    8
}
/// $09A192: bcs.w $9a198
pub fn i_09a192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A196;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x9A198; return 10; }
    8
}
/// $09A196: neg.b d0
pub fn i_09a196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A198;
    cpu.ir = 0x4400;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.set(StatusRegister::V, v1 == 0x80); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $09A198: lsr.w #$2, d0
pub fn i_09a198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A19A;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $09A19A: lea.l $2f6588.l, a0
pub fn i_09a19a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A1A0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F6588;
    20
}
/// $09A1A0: move.w (a0, d0.w), d0
pub fn i_09a1a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A1A4;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $09A1A4: add.w $22(a6), d0
pub fn i_09a1a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A1A8;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $09A1A8: move.w $24(a6), d1
pub fn i_09a1a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A1AC;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09A1AC: move.b $72(a6), d2
pub fn i_09a1ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A1B0;
    cpu.ir = 0x142E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09A1B0: jmp $8f3be.l
pub fn i_09a1b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A1B6;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x8F3BE;
    8
}
// ===== Gun_MuzzlePos_09a1b6  ($09A1B6, 202 B, props/items_score_crates_09a0bc.s)
/// $09A1B6: move.b $82(a6), d7
pub fn i_09a1b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A1BA;
    cpu.ir = 0x1E2E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09A1BA: andi.w #$f0, d7
pub fn i_09a1ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A1BE;
    cpu.ir = 0x247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = (v1 & 0xF0) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09A1BE: lsr.w #$1, d7
pub fn i_09a1be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A1C0;
    cpu.ir = 0xE24F;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $09A1C0: lea.l $2f65aa.l, a1
pub fn i_09a1c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A1C6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2F65AA;
    20
}
/// $09A1C6: move.w (a1, d7.w), d0
pub fn i_09a1c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A1CA;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $09A1CA: move.w $2(a1, d7.w), d1
pub fn i_09a1ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A1CE;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $09A1CE: add.w $22(a6), d0
pub fn i_09a1ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A1D2;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $09A1D2: add.w $24(a6), d1
pub fn i_09a1d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A1D6;
    cpu.ir = 0xD26E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $09A1D6: rts
pub fn i_09a1d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A1D8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $09A1D8: tst.b $7b(a6)
pub fn i_09a1d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A1DC;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $09A1DC: bne.w $9a1ea
pub fn i_09a1dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A1E0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9A1EA; return 10; }
    8
}
/// $09A1E0: jmp $2783a.l
pub fn i_09a1e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A1E6;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x2783A;
    8
}
/// $09A1E6: bra.w $9a20c
pub fn i_09a1e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A1EA;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A20C;
    10
}
/// $09A1EA: movea.l $c(a6), a0
pub fn i_09a1ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A1EE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $09A1EE: move.w $22(a0), d0
pub fn i_09a1ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A1F2;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09A1F2: add.w $7c(a6), d0
pub fn i_09a1f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A1F6;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $09A1F6: move.w d0, $22(a6)
pub fn i_09a1f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A1FA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09A1FA: move.w $24(a0), d0
pub fn i_09a1fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A1FE;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09A1FE: add.w $7e(a6), d0
pub fn i_09a1fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A202;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $09A202: move.w d0, $24(a6)
pub fn i_09a202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A206;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09A206: jmp $28108.l
pub fn i_09a206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A20C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x28108;
    8
}
/// $09A20C: rts
pub fn i_09a20c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A20E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $09A20E: move.b #$5, $3b(a6)
pub fn i_09a20e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A214;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09A214: move.l a6, -(a7)
pub fn i_09a214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A216;
    cpu.ir = 0x2F0E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $09A216: lea.l $100800.l, a6
pub fn i_09a216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A21C;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0x100800;
    20
}
/// $09A21C: move.w #$10f4, d0
pub fn i_09a21c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A220;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10F4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09A220: jsr $2352.l
pub fn i_09a220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A226;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A226);
    cpu.pc = 0x2352;
    16
}
/// $09A226: lea.l $9a520(pc), a1
pub fn i_09a226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A22A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9A520;
    16
}
/// $09A22A: jsr $4ae.l
pub fn i_09a22a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A230;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A230);
    cpu.pc = 0x4AE;
    16
}
/// $09A230: movea.l (a7)+, a6
pub fn i_09a230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A232;
    cpu.ir = 0x2C5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    12
}
/// $09A232: move.b $82(a6), $98(a0)
pub fn i_09a232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A238;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad4, v3 as u8);
    20
}
/// $09A238: bsr.w $9a1b6
pub fn i_09a238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A23C;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x9A23C);
    cpu.pc = 0x9A1B6;
    18
}
/// $09A23C: move.w d0, $22(a0)
pub fn i_09a23c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A240;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09A240: move.w d1, $24(a0)
pub fn i_09a240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A244;
    cpu.ir = 0x3141;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09A244: rts
pub fn i_09a244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A246;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $09A246: move.b #$5, $3b(a6)
pub fn i_09a246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A24C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09A24C: move.l a6, -(a7)
pub fn i_09a24c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A24E;
    cpu.ir = 0x2F0E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $09A24E: lea.l $100800.l, a6
pub fn i_09a24e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A254;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0x100800;
    20
}
/// $09A254: move.w #$10f4, d0
pub fn i_09a254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A258;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10F4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09A258: jsr $2352.l
pub fn i_09a258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A25E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A25E);
    cpu.pc = 0x2352;
    16
}
/// $09A25E: lea.l $3093a.l, a1
pub fn i_09a25e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A264;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x3093A;
    20
}
/// $09A264: jsr $4ae.l
pub fn i_09a264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A26A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A26A);
    cpu.pc = 0x4AE;
    16
}
/// $09A26A: movea.l (a7)+, a6
pub fn i_09a26a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A26C;
    cpu.ir = 0x2C5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    12
}
/// $09A26C: move.b $82(a6), $98(a0)
pub fn i_09a26c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A272;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad4, v3 as u8);
    20
}
/// $09A272: bsr.w $9a1b6
pub fn i_09a272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A276;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x9A276);
    cpu.pc = 0x9A1B6;
    18
}
/// $09A276: move.w d0, $22(a0)
pub fn i_09a276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A27A;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09A27A: move.w d1, $24(a0)
pub fn i_09a27a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A27E;
    cpu.ir = 0x3141;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09A27E: rts
pub fn i_09a27e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A280;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gun_Child_Init_09a280  ($09A280, 48 B, props/items_score_crates_09a0bc.s)
/// $09A280: movea.l $c(a6), a0
pub fn i_09a280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A284;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $09A284: move.b $73(a0), d0
pub fn i_09a284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A288;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09A288: bmi.w $9a2b6
pub fn i_09a288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A28C;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x9A2B6; return 10; }
    8
}
/// $09A28C: jsr $13600.l
pub fn i_09a28c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A292;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A292);
    cpu.pc = 0x13600;
    16
}
/// $09A292: movea.l $c(a6), a0
pub fn i_09a292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A296;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $09A296: move.b $73(a0), d0
pub fn i_09a296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A29A;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09A29A: andi.w #$ff, d0
pub fn i_09a29a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A29E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09A29E: add.w d0, d0
pub fn i_09a29e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A2A0;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09A2A0: lea.l $2f65ee.l, a0
pub fn i_09a2a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A2A6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F65EE;
    20
}
/// $09A2A6: move.w (a0, d0.w), d1
pub fn i_09a2a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A2AA;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $09A2AA: jsr $236e.l
pub fn i_09a2aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A2B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A2B0);
    cpu.pc = 0x236E;
    16
}
// ===== SetTaskHandler_09a2b0  ($09A2B0, 8 B, kernel/task_handlers.c)
/// $09A2B0: lea.l $9a2b8(pc), a1
pub fn i_09a2b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A2B4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9A2B8;
    16
}
/// $09A2B4: move.l a1, (a6)
pub fn i_09a2b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A2B6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09A2B6: rts
pub fn i_09a2b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A2B8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gun_Child_Sync_09a2b8  ($09A2B8, 64 B, props/items_score_crates_09a0bc.s)
/// $09A2B8: movea.l $c(a6), a0
pub fn i_09a2b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A2BC;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $09A2BC: move.b $3a(a0), $3a(a6)
pub fn i_09a2bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A2C2;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $09A2C2: movea.l $74(a0), a1
pub fn i_09a2c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A2C6;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $09A2C6: move.b $73(a0), d0
pub fn i_09a2c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A2CA;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09A2CA: bmi.w $9a2f8
pub fn i_09a2ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A2CE;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x9A2F8; return 10; }
    8
}
/// $09A2CE: andi.w #$ff, d0
pub fn i_09a2ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A2D2;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09A2D2: add.w d0, d0
pub fn i_09a2d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A2D4;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09A2D4: add.w d0, d0
pub fn i_09a2d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A2D6;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09A2D6: move.l (a1, d0.w), $3c(a6)
pub fn i_09a2d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A2DC;
    cpu.ir = 0x2D71;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    30
}
/// $09A2DC: move.w $22(a0), $22(a6)
pub fn i_09a2dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A2E2;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $09A2E2: move.w $24(a0), $24(a6)
pub fn i_09a2e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A2E8;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $09A2E8: move.w $38(a0), $38(a6)
pub fn i_09a2e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A2EE;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad4, v3 as u16);
    20
}
/// $09A2EE: jsr $5ca2a.l
pub fn i_09a2ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A2F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A2F4);
    cpu.pc = 0x5CA2A;
    16
}
/// $09A2F4: bra.w $9a2fe
pub fn i_09a2f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A2F8;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A2FE;
    10
}
// ===== SetTaskHandler_09a2f8  ($09A2F8, 8 B, kernel/task_handlers.c)
/// $09A2F8: lea.l $9a280(pc), a1
pub fn i_09a2f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A2FC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9A280;
    16
}
/// $09A2FC: move.l a1, (a6)
pub fn i_09a2fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A2FE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09A2FE: rts
pub fn i_09a2fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A300;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gun_Init_09a300  ($09A300, 116 B, props/items_score_crates_09a0bc.s)
/// $09A300: move.b #$ff, $7b(a6)
pub fn i_09a300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A306;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09A306: movea.l $c(a6), a0
pub fn i_09a306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A30A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $09A30A: move.w $22(a6), $7c(a6)
pub fn i_09a30a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A310;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad4, v3 as u16);
    20
}
/// $09A310: move.w $24(a6), $7e(a6)
pub fn i_09a310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A316;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad4, v3 as u16);
    20
}
/// $09A316: bra.w $9a31e
pub fn i_09a316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A31A;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A31E;
    10
}
/// $09A31A: clr.b $7b(a6)
pub fn i_09a31a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A31E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x7B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $09A31E: clr.b $3b(a6)
pub fn i_09a31e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A322;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x3B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $09A322: move.w #$d1, d1
pub fn i_09a322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A326;
    cpu.ir = 0x323C;
    let v1: u32 = 0xD1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09A326: jsr $236e.l
pub fn i_09a326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A32C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A32C);
    cpu.pc = 0x236E;
    16
}
/// $09A32C: move.w #$8000, d0
pub fn i_09a32c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A330;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09A330: jsr $28134.l
pub fn i_09a330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A336;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A336);
    cpu.pc = 0x28134;
    16
}
/// $09A336: andi.w #$ffe3, $38(a6)
pub fn i_09a336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A33C;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09A33C: ori.w #$4, $38(a6)
pub fn i_09a33c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A342;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x4) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09A342: jsr $8f3a6.l
pub fn i_09a342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A348;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A348);
    cpu.pc = 0x8F3A6;
    16
}
/// $09A348: not.b d0
pub fn i_09a348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A34A;
    cpu.ir = 0x4600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = !v1 & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $09A34A: move.b d0, $72(a6)
pub fn i_09a34a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A34E;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09A34E: clr.w $70(a6)
pub fn i_09a34e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A352;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $09A352: lea.l $9a280(pc), a1
pub fn i_09a352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A356;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9A280;
    16
}
/// $09A356: jsr $4ae.l
pub fn i_09a356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A35C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A35C);
    cpu.pc = 0x4AE;
    16
}
/// $09A35C: move.w #$64, $66(a6)
pub fn i_09a35c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A362;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x64;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09A362: move.w #$2, $1c(a6)
pub fn i_09a362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A368;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09A368: jsr $138fe.l
pub fn i_09a368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A36E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A36E);
    cpu.pc = 0x138FE;
    16
}
/// $09A36E: lea.l $9a374(pc), a1
pub fn i_09a36e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A372;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9A374;
    16
}
/// $09A372: move.l a1, (a6)
pub fn i_09a372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A374;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Gun_Idle_09a374  ($09A374, 60 B, props/items_score_crates_09a0bc.s)
/// $09A374: clr.b $7a(a6)
pub fn i_09a374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A378;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x7A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $09A378: clr.b $3b(a6)
pub fn i_09a378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A37C;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x3B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $09A37C: move.b #$ff, $73(a6)
pub fn i_09a37c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A382;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09A382: move.l #$ffffffff, $48(a6)
pub fn i_09a382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A38A;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $09A38A: lea.l $9a390(pc), a1
pub fn i_09a38a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A38E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9A390;
    16
}
/// $09A38E: move.l a1, (a6)
pub fn i_09a38e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A390;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09A390: bsr.w $9a1d8
pub fn i_09a390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A394;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x9A394);
    cpu.pc = 0x9A1D8;
    18
}
/// $09A394: bsr.w $9a0fc
pub fn i_09a394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A398;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x9A398);
    cpu.pc = 0x9A0FC;
    18
}
/// $09A398: bsr.w $9a186
pub fn i_09a398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A39C;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x9A39C);
    cpu.pc = 0x9A186;
    18
}
/// $09A39C: tst.b d4
pub fn i_09a39c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A39E;
    cpu.ir = 0x4A04;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $09A39E: bmi.w $9a3ac
pub fn i_09a39e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A3A2;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x9A3AC; return 10; }
    8
}
/// $09A3A2: move.b d4, $73(a6)
pub fn i_09a3a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A3A6;
    cpu.ir = 0x1D44;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09A3A6: lea.l $9a3b0(pc), a1
pub fn i_09a3a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A3AA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9A3B0;
    16
}
/// $09A3AA: move.l a1, (a6)
pub fn i_09a3aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A3AC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09A3AC: bra.w $9a0e0
pub fn i_09a3ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A3B0;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A0E0;
    10
}
// ===== Gun_Aim_09a3b0  ($09A3B0, 346 B, props/items_score_crates_09a0bc.s)
/// $09A3B0: lea.l $9a3b6(pc), a1
pub fn i_09a3b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A3B4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9A3B6;
    16
}
/// $09A3B4: move.l a1, (a6)
pub fn i_09a3b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A3B6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09A3B6: cmpi.b #$2, $73(a6)
pub fn i_09a3b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A3BC;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $09A3BC: bne.w $9a43a
pub fn i_09a3bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A3C0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9A43A; return 10; }
    8
}
/// $09A3C0: jsr $5e136.l
pub fn i_09a3c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A3C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A3C6);
    cpu.pc = 0x5E136;
    16
}
/// $09A3C6: bcs.w $9a436
pub fn i_09a3c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A3CA;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x9A436; return 10; }
    8
}
/// $09A3CA: cmpi.b #$3, $7a(a6)
pub fn i_09a3ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A3D0;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $09A3D0: bne.w $9a3e0
pub fn i_09a3d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A3D4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9A3E0; return 10; }
    8
}
/// $09A3D4: lsl.w #$8, d0
pub fn i_09a3d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A3D6;
    cpu.ir = 0xE148;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $09A3D6: sub.w $70(a6), d0
pub fn i_09a3d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A3DA;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $09A3DA: asr.w #$3, d0
pub fn i_09a3da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A3DC;
    cpu.ir = 0xE640;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $09A3DC: add.w d0, $70(a6)
pub fn i_09a3dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A3E0;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09A3E0: tst.w $78(a6)
pub fn i_09a3e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A3E4;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $09A3E4: beq.w $9a3f0
pub fn i_09a3e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A3E8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9A3F0; return 10; }
    8
}
/// $09A3E8: subq.w #$1, $78(a6)
pub fn i_09a3e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A3EC;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09A3EC: bra.w $9a436
pub fn i_09a3ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A3F0;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A436;
    10
}
/// $09A3F0: addi.w #$80, d0
pub fn i_09a3f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A3F4;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x80, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09A3F4: andi.w #$ff00, d0
pub fn i_09a3f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A3F8;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF00) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09A3F8: bne.w $9a436
pub fn i_09a3f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A3FC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9A436; return 10; }
    8
}
/// $09A3FC: move.b $7a(a6), d1
pub fn i_09a3fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A400;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09A400: bne.w $9a41e
pub fn i_09a400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A404;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9A41E; return 10; }
    8
}
/// $09A404: move.b #$3, $7a(a6)
pub fn i_09a404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A40A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09A40A: lea.l $2f6648.l, a0
pub fn i_09a40a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A410;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F6648;
    20
}
/// $09A410: jsr $799de.l
pub fn i_09a410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A416;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A416);
    cpu.pc = 0x799DE;
    16
}
/// $09A416: move.w d0, $78(a6)
pub fn i_09a416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A41A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09A41A: bra.w $9a436
pub fn i_09a41a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A41E;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A436;
    10
}
/// $09A41E: subq.w #$1, $78(a6)
pub fn i_09a41e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A422;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09A422: bne.w $9a436
pub fn i_09a422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A426;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9A436; return 10; }
    8
}
/// $09A426: move.w #$2, $78(a6)
pub fn i_09a426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A42C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09A42C: subq.b #$1, d1
pub fn i_09a42c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A42E;
    cpu.ir = 0x5301;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $09A42E: move.b d1, $7a(a6)
pub fn i_09a42e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A432;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09A432: bsr.w $9a20e
pub fn i_09a432(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A436;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x9A436);
    cpu.pc = 0x9A20E;
    18
}
/// $09A436: bra.w $9a4dc
pub fn i_09a436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A43A;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A4DC;
    10
}
/// $09A43A: cmpi.b #$0, $73(a6)
pub fn i_09a43a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A440;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $09A440: bne.w $9a44e
pub fn i_09a440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A444;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9A44E; return 10; }
    8
}
/// $09A444: lea.l $10e200.l, a2
pub fn i_09a444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A44A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E200;
    20
}
/// $09A44A: bra.w $9a454
pub fn i_09a44a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A44E;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A454;
    10
}
/// $09A44E: lea.l $10e206.l, a2
pub fn i_09a44e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A454;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E206;
    20
}
/// $09A454: move.b $2(a2), d0
pub fn i_09a454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A458;
    cpu.ir = 0x102A;
    let ad2 = cpu.a[2].wrapping_add(0x2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09A458: andi.w #$c, d0
pub fn i_09a458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A45C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xC) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09A45C: beq.w $9a4a2
pub fn i_09a45c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A460;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9A4A2; return 10; }
    8
}
/// $09A460: andi.b #$4, d0
pub fn i_09a460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A464;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x4) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $09A464: bne.w $9a470
pub fn i_09a464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A468;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9A470; return 10; }
    8
}
/// $09A468: move.w #$8000, d0
pub fn i_09a468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A46C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09A46C: bra.w $9a472
pub fn i_09a46c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A470;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A472;
    10
}
/// $09A470: clr.w d0
pub fn i_09a470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A472;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $09A472: sub.w $70(a6), d0
pub fn i_09a472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A476;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $09A476: asr.w #$2, d0
pub fn i_09a476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A478;
    cpu.ir = 0xE440;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $09A478: ext.l d0
pub fn i_09a478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A47A;
    cpu.ir = 0x48C0;
    cpu.d[0] = cpu.d[0] as i16 as i32 as u32;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $09A47A: move.l d0, d1
pub fn i_09a47a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A47C;
    cpu.ir = 0x2200;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[1] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $09A47C: swap d1
pub fn i_09a47c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A47E;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $09A47E: eor.w d1, d0
pub fn i_09a47e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A480;
    cpu.ir = 0xB340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 ^ cpu.d[1]) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $09A480: sub.w d1, d0
pub fn i_09a480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A482;
    cpu.ir = 0x9041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09A482: move.w $80(a6), d2
pub fn i_09a482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A486;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09A486: addi.w #$100, d2
pub fn i_09a486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A48A;
    cpu.ir = 0x642;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09A48A: cmp.w d0, d2
pub fn i_09a48a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A48C;
    cpu.ir = 0xB440;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $09A48C: bcs.w $9a492
pub fn i_09a48c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A490;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x9A492; return 10; }
    8
}
/// $09A490: move.w d0, d2
pub fn i_09a490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A492;
    cpu.ir = 0x3400;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09A492: move.w d2, $80(a6)
pub fn i_09a492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A496;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09A496: eor.w d1, d2
pub fn i_09a496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A498;
    cpu.ir = 0xB342;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = (v1 ^ cpu.d[1]) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $09A498: sub.w d1, d2
pub fn i_09a498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A49A;
    cpu.ir = 0x9441;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09A49A: add.w d2, $70(a6)
pub fn i_09a49a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A49E;
    cpu.ir = 0xD56E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09A49E: bra.w $9a4a6
pub fn i_09a49e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A4A2;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A4A6;
    10
}
/// $09A4A2: clr.w $80(a6)
pub fn i_09a4a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A4A6;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x80);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $09A4A6: btst.b #$4, $3(a2)
pub fn i_09a4a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A4AC;
    cpu.ir = 0x82A;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad2 = cpu.a[2].wrapping_add(0x3);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $09A4AC: beq.w $9a4bc
pub fn i_09a4ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A4B0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9A4BC; return 10; }
    8
}
/// $09A4B0: move.b #$3, $7a(a6)
pub fn i_09a4b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A4B6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09A4B6: move.b #$1, $78(a6)
pub fn i_09a4b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A4BC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09A4BC: move.b $7a(a6), d0
pub fn i_09a4bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A4C0;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09A4C0: beq.w $9a4dc
pub fn i_09a4c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A4C4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9A4DC; return 10; }
    8
}
/// $09A4C4: subq.b #$1, $78(a6)
pub fn i_09a4c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A4C8;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $09A4C8: bne.w $9a4dc
pub fn i_09a4c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A4CC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9A4DC; return 10; }
    8
}
/// $09A4CC: move.b #$2, $78(a6)
pub fn i_09a4cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A4D2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09A4D2: subq.b #$1, d0
pub fn i_09a4d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A4D4;
    cpu.ir = 0x5300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $09A4D4: move.b d0, $7a(a6)
pub fn i_09a4d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A4D8;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09A4D8: bsr.w $9a246
pub fn i_09a4d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A4DC;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x9A4DC);
    cpu.pc = 0x9A246;
    18
}
/// $09A4DC: bsr.w $9a1d8
pub fn i_09a4dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A4E0;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x9A4E0);
    cpu.pc = 0x9A1D8;
    18
}
/// $09A4E0: bsr.w $9a0fc
pub fn i_09a4e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A4E4;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x9A4E4);
    cpu.pc = 0x9A0FC;
    18
}
/// $09A4E4: jsr $28d70.l
pub fn i_09a4e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A4EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A4EA);
    cpu.pc = 0x28D70;
    16
}
/// $09A4EA: move.w $22(a6), d0
pub fn i_09a4ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A4EE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09A4EE: move.w $24(a6), d1
pub fn i_09a4ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A4F2;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09A4F2: move.b $72(a6), d2
pub fn i_09a4f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A4F6;
    cpu.ir = 0x142E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09A4F6: bsr.w $9a186
pub fn i_09a4f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A4FA;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x9A4FA);
    cpu.pc = 0x9A186;
    18
}
/// $09A4FA: tst.b d4
pub fn i_09a4fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A4FC;
    cpu.ir = 0x4A04;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $09A4FC: bpl.w $9a506
pub fn i_09a4fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A500;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x9A506; return 10; }
    8
}
/// $09A500: lea.l $9a374(pc), a1
pub fn i_09a500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A504;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9A374;
    16
}
/// $09A504: move.l a1, (a6)
pub fn i_09a504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A506;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09A506: bra.w $9a0e0
pub fn i_09a506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A50A;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A0E0;
    10
}
// ===== Gun_Destroyed_09a50a  ($09A50A, 22 B, props/items_score_crates_09a0bc.s)
/// $09A50A: move.w #$1021, d0
pub fn i_09a50a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A50E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1021;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09A50E: jsr $2352.l
pub fn i_09a50e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A514;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A514);
    cpu.pc = 0x2352;
    16
}
/// $09A514: jsr $13600.l
pub fn i_09a514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A51A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A51A);
    cpu.pc = 0x13600;
    16
}
/// $09A51A: jmp $77fd6.l
pub fn i_09a51a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A520;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x77FD6;
    8
}
// ===== Gun_Shell_09a520  ($09A520, 184 B, props/items_score_crates_09a0bc.s)
/// $09A520: move.w #$d1, d1
pub fn i_09a520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A524;
    cpu.ir = 0x323C;
    let v1: u32 = 0xD1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09A524: jsr $236e.l
pub fn i_09a524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A52A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A52A);
    cpu.pc = 0x236E;
    16
}
/// $09A52A: bset.b #$4, $6b(a6)
pub fn i_09a52a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A530;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $09A530: lea.l $2f65f4.l, a0
pub fn i_09a530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A536;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F65F4;
    20
}
/// $09A536: move.l a0, $4c(a6)
pub fn i_09a536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A53A;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $09A53A: jsr $283ca.l
pub fn i_09a53a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A540;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A540);
    cpu.pc = 0x283CA;
    16
}
/// $09A540: move.w #$d000, $38(a6)
pub fn i_09a540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A546;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09A546: lea.l $2f66ca.l, a0
pub fn i_09a546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A54C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F66CA;
    20
}
/// $09A54C: jsr $799de.l
pub fn i_09a54c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A552;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A552);
    cpu.pc = 0x799DE;
    16
}
/// $09A552: move.w d0, d1
pub fn i_09a552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A554;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09A554: move.b $98(a6), d0
pub fn i_09a554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A558;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09A558: addq.b #$8, d0
pub fn i_09a558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A55A;
    cpu.ir = 0x5000;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 8, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $09A55A: andi.w #$f0, d0
pub fn i_09a55a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A55E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF0) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09A55E: jsr $13c0e.l
pub fn i_09a55e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A564;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A564);
    cpu.pc = 0x13C0E;
    16
}
/// $09A564: move.w d1, $28(a6)
pub fn i_09a564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A568;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09A568: move.w d2, $2a(a6)
pub fn i_09a568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A56C;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09A56C: move.b $98(a6), d0
pub fn i_09a56c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A570;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09A570: andi.w #$ff, d0
pub fn i_09a570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A574;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09A574: asr.w #$4, d0
pub fn i_09a574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A576;
    cpu.ir = 0xE840;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $09A576: movea.l #$29d452, a0
pub fn i_09a576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A57C;
    cpu.ir = 0x207C;
    let v1: u32 = 0x29D452;
    cpu.a[0] = v1;
    4
}
/// $09A57C: lsl.w #$2, d0
pub fn i_09a57c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A57E;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $09A57E: movea.l (a0, d0.w), a0
pub fn i_09a57e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A582;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $09A582: cmpa.l #$ffffffff, a0
pub fn i_09a582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A588;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $09A588: beq.w $9a592
pub fn i_09a588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A58C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9A592; return 10; }
    8
}
/// $09A58C: jsr $28cd4.l
pub fn i_09a58c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A592;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A592);
    cpu.pc = 0x28CD4;
    16
}
/// $09A592: lea.l $9a598(pc), a1
pub fn i_09a592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A596;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9A598;
    16
}
/// $09A596: move.l a1, (a6)
pub fn i_09a596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A598;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09A598: bset.b #$6, $13(a6)
pub fn i_09a598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A59E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $09A59E: jsr $27cee.l
pub fn i_09a59e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A5A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A5A4);
    cpu.pc = 0x27CEE;
    16
}
/// $09A5A4: jsr $28d70.l
pub fn i_09a5a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A5AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A5AA);
    cpu.pc = 0x28D70;
    16
}
/// $09A5AA: jsr $283d8.l
pub fn i_09a5aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A5B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A5B0);
    cpu.pc = 0x283D8;
    16
}
/// $09A5B0: btst.b #$1, $13(a6)
pub fn i_09a5b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A5B6;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $09A5B6: beq.w $9a5c0
pub fn i_09a5b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A5BA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9A5C0; return 10; }
    8
}
/// $09A5BA: jmp $518.l
pub fn i_09a5ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A5C0;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $09A5C0: movea.l #$ffffffff, a0
pub fn i_09a5c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A5C6;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $09A5C6: jsr $5dd56.l
pub fn i_09a5c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A5CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A5CC);
    cpu.pc = 0x5DD56;
    16
}
/// $09A5CC: bcc.w $9a5d6
pub fn i_09a5cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A5D0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9A5D6; return 10; }
    8
}
/// $09A5D0: jmp $518.l
pub fn i_09a5d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A5D6;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $09A5D6: rts
pub fn i_09a5d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A5D8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CmpPrioWithSibling_09a5d8  ($09A5D8, 16 B, props/items_score_crates_09a0bc.s)
/// $09A5D8: movea.l $8(a6), a1
pub fn i_09a5d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A5DC;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $09A5DC: move.b $10(a6), d0
pub fn i_09a5dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A5E0;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09A5E0: cmp.b $10(a1), d0
pub fn i_09a5e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A5E4;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $09A5E4: bcs.w $9a5ee
pub fn i_09a5e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A5E8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x9A5EE; return 10; }
    8
}
// ===== ClearXN_09a5e8  ($09A5E8, 6 B, thunks/ccr_helpers.c)
/// $09A5E8: andi.b #$ee, ccr
pub fn i_09a5e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A5EC;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $09A5EC: rts
pub fn i_09a5ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A5EE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_09a5ee  ($09A5EE, 6 B, thunks/ccr_helpers.c)
/// $09A5EE: ori.b #$11, ccr
pub fn i_09a5ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A5F2;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $09A5F2: rts
pub fn i_09a5f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A5F4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Item_GroundTail_09a75a  ($09A75A, 80 B, props/items_score_crates_09a0bc.s)
/// $09A75A: cmpi.b #$1, $90(a6)
pub fn i_09a75a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A760;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $09A760: bne.w $9a76a
pub fn i_09a760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A764;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9A76A; return 10; }
    8
}
/// $09A764: addq.b #$1, $10e48b.l
pub fn i_09a764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A76A;
    cpu.ir = 0x5239;
    let v1 = bus.read8(0x10E48B) as u32;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    bus.write8(0x10E48B, r2 as u8);
    16
}
/// $09A76A: tst.b $9f(a6)
pub fn i_09a76a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A76E;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9F);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $09A76E: beq.w $9a78e
pub fn i_09a76e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A772;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9A78E; return 10; }
    8
}
/// $09A772: movea.l #$ffffffff, a0
pub fn i_09a772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A778;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $09A778: lea.l $9a5f4(pc), a0
pub fn i_09a778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A77C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x9A5F4;
    16
}
/// $09A77C: jsr $5dd56.l
pub fn i_09a77c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A782;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A782);
    cpu.pc = 0x5DD56;
    16
}
/// $09A782: bcc.w $9a78c
pub fn i_09a782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A786;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9A78C; return 10; }
    8
}
/// $09A786: jmp $518.l
pub fn i_09a786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A78C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $09A78C: rts
pub fn i_09a78c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A78E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $09A78E: movea.l #$ffffffff, a0
pub fn i_09a78e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A794;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $09A794: lea.l $9a5f4(pc), a0
pub fn i_09a794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A798;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x9A5F4;
    16
}
/// $09A798: jsr $5dd5c.l
pub fn i_09a798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A79E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A79E);
    cpu.pc = 0x5DD5C;
    16
}
/// $09A79E: bcc.w $9a7a8
pub fn i_09a79e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7A2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9A7A8; return 10; }
    8
}
/// $09A7A2: jmp $518.l
pub fn i_09a7a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7A8;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $09A7A8: rts
pub fn i_09a7a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7AA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_ProbeMoveX_09A7AA  ($09A7AA, 34 B, entity/probe_move_x_09a7aa.s)
/// $09A7AA: jsr $9a7cc(pc)
pub fn i_09a7aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7AE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9A7AE);
    cpu.pc = 0x9A7CC;
    16
}
/// $09A7AE: bcs.w $9a7ca
pub fn i_09a7ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7B2;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x9A7CA; return 10; }
    8
}
/// $09A7B2: move.w #$10, d0
pub fn i_09a7b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7B6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09A7B6: btst.b #$0, $3a(a6)
pub fn i_09a7b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7BC;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $09A7BC: beq.w $9a7c2
pub fn i_09a7bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7C0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9A7C2; return 10; }
    8
}
/// $09A7C0: neg.w d0
pub fn i_09a7c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7C2;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $09A7C2: add.w d0, $22(a0)
pub fn i_09a7c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7C6;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09A7C6: andi.b #$ee, ccr
pub fn i_09a7c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7CA;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $09A7CA: rts
pub fn i_09a7ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7CC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Item_SpawnFromParent_09a7cc  ($09A7CC, 106 B, props/items_score_crates_09a0bc.s)
/// $09A7CC: tst.b d0
pub fn i_09a7cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7CE;
    cpu.ir = 0x4A00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $09A7CE: beq.w $9a830
pub fn i_09a7ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7D2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9A830; return 10; }
    8
}
/// $09A7D2: move.w d1, -(a7)
pub fn i_09a7d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7D4;
    cpu.ir = 0x3F01;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $09A7D4: andi.w #$3f, d0
pub fn i_09a7d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7D8;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09A7D8: cmpi.w #$1e, d0
pub fn i_09a7d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7DC;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x1E, Size::Word, false);
    8
}
/// $09A7DC: bcs.w $9a7e4
pub fn i_09a7dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7E0;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x9A7E4; return 10; }
    8
}
/// $09A7E0: move.w #$a, d0
pub fn i_09a7e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7E4;
    cpu.ir = 0x303C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09A7E4: add.w d0, d0
pub fn i_09a7e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7E6;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09A7E6: add.w d0, d0
pub fn i_09a7e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7E8;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09A7E8: lea.l $9a6de(pc), a0
pub fn i_09a7e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7EC;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x9A6DE;
    16
}
/// $09A7EC: movea.l (a0, d0.w), a1
pub fn i_09a7ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7F0;
    cpu.ir = 0x2270;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    18
}
/// $09A7F0: move.l a6, -(a7)
pub fn i_09a7f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7F2;
    cpu.ir = 0x2F0E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $09A7F2: lea.l $100800.l, a6
pub fn i_09a7f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7F8;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0x100800;
    20
}
/// $09A7F8: jsr $4ae.l
pub fn i_09a7f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A7FE);
    cpu.pc = 0x4AE;
    16
}
/// $09A7FE: movea.l (a7)+, a6
pub fn i_09a7fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A800;
    cpu.ir = 0x2C5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    12
}
/// $09A800: move.w $24(a6), $24(a0)
pub fn i_09a800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A806;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $09A806: move.w #$10, d0
pub fn i_09a806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A80A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09A80A: btst.b #$0, $3a(a6)
pub fn i_09a80a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A810;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $09A810: bne.w $9a816
pub fn i_09a810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A814;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9A816; return 10; }
    8
}
/// $09A814: neg.w d0
pub fn i_09a814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A816;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $09A816: add.w $22(a6), d0
pub fn i_09a816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A81A;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $09A81A: move.w d0, $22(a0)
pub fn i_09a81a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A81E;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09A81E: move.w (a7)+, d1
pub fn i_09a81e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A820;
    cpu.ir = 0x321F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $09A820: move.b (a6, d1.w), $98(a0)
pub fn i_09a820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A826;
    cpu.ir = 0x1176;
    let ad2 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad4, v3 as u8);
    22
}
/// $09A826: move.b $1(a6, d1.w), $99(a0)
pub fn i_09a826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A82C;
    cpu.ir = 0x1176;
    let ad2 = cpu.a[6].wrapping_add(0x1).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x99);
    bus.write8(ad4, v3 as u8);
    22
}
/// $09A82C: ori.b #$11, ccr
pub fn i_09a82c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A830;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $09A830: eori.b #$11, ccr
pub fn i_09a830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A834;
    cpu.ir = 0xA3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) ^ 0x11);
    20
}
/// $09A834: rts
pub fn i_09a834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A836;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Item_PickupTestA_09a836  ($09A836, 12 B, props/items_score_crates_09a0bc.s)
/// $09A836: lea.l $9a6ce(pc), a1
pub fn i_09a836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A83A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9A6CE;
    16
}
/// $09A83A: tst.b $45(a6)
pub fn i_09a83a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A83E;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $09A83E: beq.w $9a848
pub fn i_09a83e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A842;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9A848; return 10; }
    8
}
// ===== ClearC_09a842  ($09A842, 6 B, thunks/ccr_helpers.c)
/// $09A842: andi.b #$fe, ccr
pub fn i_09a842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A846;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $09A846: rts
pub fn i_09a846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A848;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpAbsThunk_09a848  ($09A848, 6 B, thunks/jmp_abs_thunks.c)
/// $09A848: jmp $3ee48.l
pub fn i_09a848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A84E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x3EE48;
    8
}
// ===== Item_PickupTestB_09a84e  ($09A84E, 6 B, props/items_score_crates_09a0bc.s)
/// $09A84E: lea.l $9a6d6(pc), a1
pub fn i_09a84e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A852;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9A6D6;
    16
}
/// $09A852: bra.b $9a83a
pub fn i_09a852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A854;
    cpu.ir = 0x60E6;
    cpu.pc = 0x9A83A;
    10
}
// ===== Item_GiveAmmoKind_09a854  ($09A854, 34 B, props/items_score_crates_09a0bc.s)
/// $09A854: move.b $98(a6), d2
pub fn i_09a854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A858;
    cpu.ir = 0x142E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09A858: jsr $32b58.l
pub fn i_09a858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A85E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A85E);
    cpu.pc = 0x32B58;
    16
}
/// $09A85E: cmpi.b #$2, $70(a1)
pub fn i_09a85e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A864;
    cpu.ir = 0xC29;
    let ad2 = cpu.a[1].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $09A864: bne.w $9a876
pub fn i_09a864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A868;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9A876; return 10; }
    8
}
/// $09A868: move.l a1, -(a7)
pub fn i_09a868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A86A;
    cpu.ir = 0x2F09;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $09A86A: move.w #$10fd, d0
pub fn i_09a86a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A86E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10FD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09A86E: jsr $2352.l
pub fn i_09a86e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A874;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A874);
    cpu.pc = 0x2352;
    16
}
/// $09A874: movea.l (a7)+, a1
pub fn i_09a874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A876;
    cpu.ir = 0x225F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    12
}
// ===== JsrAbsThunk_09a876  ($09A876, 8 B, thunks/jsr_abs_thunks.c)
/// $09A876: jsr $32cba.l
pub fn i_09a876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A87C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A87C);
    cpu.pc = 0x32CBA;
    16
}
/// $09A87C: rts
pub fn i_09a87c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A87E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Item_SetMapFromSeq_09a87e  ($09A87E, 56 B, props/items_score_crates_09a0bc.s)
/// $09A87E: movea.l $76(a6), a0
pub fn i_09a87e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A882;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $09A882: move.w $7a(a6), d0
pub fn i_09a882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A886;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09A886: move.b (a0, d0.w), d0
pub fn i_09a886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A88A;
    cpu.ir = 0x1030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $09A88A: cmpi.b #$ff, d0
pub fn i_09a88a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A88E;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $09A88E: bne.w $9a89c
pub fn i_09a88e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A892;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9A89C; return 10; }
    8
}
/// $09A892: clr.w $7a(a6)
pub fn i_09a892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A896;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x7A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $09A896: movea.l $76(a6), a0
pub fn i_09a896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A89A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $09A89A: move.b (a0), d0
pub fn i_09a89a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A89C;
    cpu.ir = 0x1010;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $09A89C: andi.w #$3, d0
pub fn i_09a89c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A8A0;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09A8A0: movea.l #$2f7a28, a0
pub fn i_09a8a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A8A6;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2F7A28;
    cpu.a[0] = v1;
    4
}
/// $09A8A6: lsl.w #$2, d0
pub fn i_09a8a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A8A8;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $09A8A8: movea.l (a0, d0.w), a0
pub fn i_09a8a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A8AC;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $09A8AC: cmpa.l #$ffffffff, a0
pub fn i_09a8ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A8B2;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $09A8B2: beq.w $9a8bc
pub fn i_09a8b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A8B6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9A8BC; return 10; }
    8
}
// ===== JsrAbsThunk_09a8b6  ($09A8B6, 8 B, thunks/jsr_abs_thunks.c)
/// $09A8B6: jsr $28cd4.l
pub fn i_09a8b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A8BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A8BC);
    cpu.pc = 0x28CD4;
    16
}
/// $09A8BC: rts
pub fn i_09a8bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A8BE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Item_Tmpl_PowCounted_09a8be  ($09A8BE, 18 B, props/items_score_crates_09a0bc.s)
/// $09A8BE: move.b #$1, $90(a6)
pub fn i_09a8be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A8C4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09A8C4: cmpi.b #$2, $10e48a.l
pub fn i_09a8c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A8CC;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10E48A) as u32;
    sub_with_flags(cpu, v2, 0x2, Size::Byte, false);
    20
}
/// $09A8CC: blt.w $9a8de
pub fn i_09a8cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A8D0;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x9A8DE; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_09a8d0  ($09A8D0, 14 B, kernel/jsr5b6_jmp518.c)
/// $09A8D0: jsr $5b6.l
pub fn i_09a8d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A8D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A8D6);
    cpu.pc = 0x5B6;
    16
}
/// $09A8D6: jmp $518.l
pub fn i_09a8d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A8DC;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $09A8DC: rts
pub fn i_09a8dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A8DE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Item_FlyAway_09a8de  ($09A8DE, 58 B, props/items_score_crates_09a0bc.s)
/// $09A8DE: move.w #$180, d1
pub fn i_09a8de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A8E2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09A8E2: jsr $236e.l
pub fn i_09a8e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A8E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A8E8);
    cpu.pc = 0x236E;
    16
}
/// $09A8E8: lea.l $2f678a.l, a0
pub fn i_09a8e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A8EE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F678A;
    20
}
/// $09A8EE: jsr $28cd4.l
pub fn i_09a8ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A8F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A8F4);
    cpu.pc = 0x28CD4;
    16
}
/// $09A8F4: lea.l $9a8fa(pc), a1
pub fn i_09a8f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A8F8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9A8FA;
    16
}
/// $09A8F8: move.l a1, (a6)
pub fn i_09a8f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A8FA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09A8FA: jsr $2783a.l
pub fn i_09a8fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A900;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A900);
    cpu.pc = 0x2783A;
    16
}
/// $09A900: jsr $28d70.l
pub fn i_09a900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A906;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A906);
    cpu.pc = 0x28D70;
    16
}
/// $09A906: bcc.w $9a914
pub fn i_09a906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A90A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9A914; return 10; }
    8
}
/// $09A90A: jsr $13600.l
pub fn i_09a90a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A910;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A910);
    cpu.pc = 0x13600;
    16
}
/// $09A910: move.l $5c(a6), (a6)
pub fn i_09a910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A914;
    cpu.ir = 0x2CAE;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read32(ad2);
    bus.write32(cpu.a[6], v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $09A914: bra.w $9a75a
pub fn i_09a914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A918;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A75A;
    10
}
// ===== Item_Tmpl284_Ammo4_09a918  ($09A918, 94 B, props/items_score_crates_09a0bc.s)
/// $09A918: move.w #$17d, d1
pub fn i_09a918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A91C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x17D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09A91C: jsr $236e.l
pub fn i_09a91c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A922;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A922);
    cpu.pc = 0x236E;
    16
}
/// $09A922: move.b #$8, $45(a6)
pub fn i_09a922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A928;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09A928: move.w #$8000, d0
pub fn i_09a928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A92C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09A92C: jsr $28134.l
pub fn i_09a92c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A932;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A932);
    cpu.pc = 0x28134;
    16
}
/// $09A932: andi.w #$ffe3, $38(a6)
pub fn i_09a932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A938;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09A938: ori.w #$0, $38(a6)
pub fn i_09a938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A93E;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09A93E: lea.l $2f67dc.l, a0
pub fn i_09a93e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A944;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F67DC;
    20
}
/// $09A944: jsr $28cd4.l
pub fn i_09a944(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A94A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A94A);
    cpu.pc = 0x28CD4;
    16
}
/// $09A94A: lea.l $9a950(pc), a1
pub fn i_09a94a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A94E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9A950;
    16
}
/// $09A94E: move.l a1, (a6)
pub fn i_09a94e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A950;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09A950: jsr $27c8c.l
pub fn i_09a950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A956;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A956);
    cpu.pc = 0x27C8C;
    16
}
/// $09A956: jsr $28d70.l
pub fn i_09a956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A95C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A95C);
    cpu.pc = 0x28D70;
    16
}
/// $09A95C: jsr $9a836(pc)
pub fn i_09a95c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A960;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9A960);
    cpu.pc = 0x9A836;
    16
}
/// $09A960: bcc.w $9a972
pub fn i_09a960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A964;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9A972; return 10; }
    8
}
/// $09A964: move.b #$4, d1
pub fn i_09a964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A968;
    cpu.ir = 0x123C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $09A968: jsr $9a854(pc)
pub fn i_09a968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A96C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9A96C);
    cpu.pc = 0x9A854;
    16
}
/// $09A96C: lea.l $9ae26(pc), a1
pub fn i_09a96c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A970;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9AE26;
    16
}
/// $09A970: move.l a1, (a6)
pub fn i_09a970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A972;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09A972: bra.w $9a75a
pub fn i_09a972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A976;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A75A;
    10
}
// ===== Item_Tmpl285_Ammo3_09a976  ($09A976, 94 B, props/items_score_crates_09a0bc.s)
/// $09A976: move.w #$17d, d1
pub fn i_09a976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A97A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x17D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09A97A: jsr $236e.l
pub fn i_09a97a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A980;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A980);
    cpu.pc = 0x236E;
    16
}
/// $09A980: move.b #$8, $45(a6)
pub fn i_09a980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A986;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09A986: move.w #$8000, d0
pub fn i_09a986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A98A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09A98A: jsr $28134.l
pub fn i_09a98a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A990;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A990);
    cpu.pc = 0x28134;
    16
}
/// $09A990: andi.w #$ffe3, $38(a6)
pub fn i_09a990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A996;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09A996: ori.w #$0, $38(a6)
pub fn i_09a996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A99C;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09A99C: lea.l $2f67e8.l, a0
pub fn i_09a99c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A9A2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F67E8;
    20
}
/// $09A9A2: jsr $28cd4.l
pub fn i_09a9a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A9A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A9A8);
    cpu.pc = 0x28CD4;
    16
}
/// $09A9A8: lea.l $9a9ae(pc), a1
pub fn i_09a9a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A9AC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9A9AE;
    16
}
/// $09A9AC: move.l a1, (a6)
pub fn i_09a9ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A9AE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09A9AE: jsr $27c8c.l
pub fn i_09a9ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A9B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A9B4);
    cpu.pc = 0x27C8C;
    16
}
/// $09A9B4: jsr $28d70.l
pub fn i_09a9b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A9BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A9BA);
    cpu.pc = 0x28D70;
    16
}
/// $09A9BA: jsr $9a836(pc)
pub fn i_09a9ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A9BE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9A9BE);
    cpu.pc = 0x9A836;
    16
}
/// $09A9BE: bcc.w $9a9d0
pub fn i_09a9be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A9C2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9A9D0; return 10; }
    8
}
/// $09A9C2: move.b #$3, d1
pub fn i_09a9c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A9C6;
    cpu.ir = 0x123C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $09A9C6: jsr $9a854(pc)
pub fn i_09a9c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A9CA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9A9CA);
    cpu.pc = 0x9A854;
    16
}
/// $09A9CA: lea.l $9ae26(pc), a1
pub fn i_09a9ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A9CE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9AE26;
    16
}
/// $09A9CE: move.l a1, (a6)
pub fn i_09a9ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A9D0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09A9D0: bra.w $9a75a
pub fn i_09a9d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A9D4;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A75A;
    10
}
// ===== Item_Tmpl286_Ammo1_09a9d4  ($09A9D4, 94 B, props/items_score_crates_09a0bc.s)
/// $09A9D4: move.w #$17d, d1
pub fn i_09a9d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A9D8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x17D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09A9D8: jsr $236e.l
pub fn i_09a9d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A9DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A9DE);
    cpu.pc = 0x236E;
    16
}
/// $09A9DE: move.b #$8, $45(a6)
pub fn i_09a9de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A9E4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09A9E4: move.w #$8000, d0
pub fn i_09a9e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A9E8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09A9E8: jsr $28134.l
pub fn i_09a9e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A9EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A9EE);
    cpu.pc = 0x28134;
    16
}
/// $09A9EE: andi.w #$ffe3, $38(a6)
pub fn i_09a9ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A9F4;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09A9F4: ori.w #$0, $38(a6)
pub fn i_09a9f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A9FA;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09A9FA: lea.l $2f67f4.l, a0
pub fn i_09a9fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA00;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F67F4;
    20
}
/// $09AA00: jsr $28cd4.l
pub fn i_09aa00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA06;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AA06);
    cpu.pc = 0x28CD4;
    16
}
/// $09AA06: lea.l $9aa0c(pc), a1
pub fn i_09aa06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA0A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9AA0C;
    16
}
/// $09AA0A: move.l a1, (a6)
pub fn i_09aa0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA0C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09AA0C: jsr $27c8c.l
pub fn i_09aa0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AA12);
    cpu.pc = 0x27C8C;
    16
}
/// $09AA12: jsr $28d70.l
pub fn i_09aa12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AA18);
    cpu.pc = 0x28D70;
    16
}
/// $09AA18: jsr $9a836(pc)
pub fn i_09aa18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA1C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9AA1C);
    cpu.pc = 0x9A836;
    16
}
/// $09AA1C: bcc.w $9aa2e
pub fn i_09aa1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA20;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9AA2E; return 10; }
    8
}
/// $09AA20: move.b #$1, d1
pub fn i_09aa20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA24;
    cpu.ir = 0x123C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $09AA24: jsr $9a854(pc)
pub fn i_09aa24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA28;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9AA28);
    cpu.pc = 0x9A854;
    16
}
/// $09AA28: lea.l $9ae26(pc), a1
pub fn i_09aa28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA2C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9AE26;
    16
}
/// $09AA2C: move.l a1, (a6)
pub fn i_09aa2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA2E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09AA2E: bra.w $9a75a
pub fn i_09aa2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA32;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A75A;
    10
}
// ===== Item_Tmpl287_Ammo2_09aa32  ($09AA32, 94 B, props/items_score_crates_09a0bc.s)
/// $09AA32: move.w #$17d, d1
pub fn i_09aa32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA36;
    cpu.ir = 0x323C;
    let v1: u32 = 0x17D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09AA36: jsr $236e.l
pub fn i_09aa36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AA3C);
    cpu.pc = 0x236E;
    16
}
/// $09AA3C: move.b #$8, $45(a6)
pub fn i_09aa3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA42;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09AA42: move.w #$8000, d0
pub fn i_09aa42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA46;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09AA46: jsr $28134.l
pub fn i_09aa46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AA4C);
    cpu.pc = 0x28134;
    16
}
/// $09AA4C: andi.w #$ffe3, $38(a6)
pub fn i_09aa4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA52;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09AA52: ori.w #$0, $38(a6)
pub fn i_09aa52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA58;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09AA58: lea.l $2f6800.l, a0
pub fn i_09aa58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA5E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F6800;
    20
}
/// $09AA5E: jsr $28cd4.l
pub fn i_09aa5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA64;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AA64);
    cpu.pc = 0x28CD4;
    16
}
/// $09AA64: lea.l $9aa6a(pc), a1
pub fn i_09aa64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA68;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9AA6A;
    16
}
/// $09AA68: move.l a1, (a6)
pub fn i_09aa68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA6A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09AA6A: jsr $27c8c.l
pub fn i_09aa6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA70;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AA70);
    cpu.pc = 0x27C8C;
    16
}
/// $09AA70: jsr $28d70.l
pub fn i_09aa70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AA76);
    cpu.pc = 0x28D70;
    16
}
/// $09AA76: jsr $9a836(pc)
pub fn i_09aa76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA7A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9AA7A);
    cpu.pc = 0x9A836;
    16
}
/// $09AA7A: bcc.w $9aa8c
pub fn i_09aa7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA7E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9AA8C; return 10; }
    8
}
/// $09AA7E: move.b #$2, d1
pub fn i_09aa7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA82;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $09AA82: jsr $9a854(pc)
pub fn i_09aa82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA86;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9AA86);
    cpu.pc = 0x9A854;
    16
}
/// $09AA86: lea.l $9ae26(pc), a1
pub fn i_09aa86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA8A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9AE26;
    16
}
/// $09AA8A: move.l a1, (a6)
pub fn i_09aa8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA8C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09AA8C: bra.w $9a75a
pub fn i_09aa8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA90;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A75A;
    10
}
// ===== Item_Tmpl288_AmmoSeq_09aa90  ($09AA90, 82 B, props/items_score_crates_09a0bc.s)
/// $09AA90: move.w #$17d, d1
pub fn i_09aa90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA94;
    cpu.ir = 0x323C;
    let v1: u32 = 0x17D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09AA94: jsr $236e.l
pub fn i_09aa94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA9A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AA9A);
    cpu.pc = 0x236E;
    16
}
/// $09AA9A: lea.l $2f79ce.l, a0
pub fn i_09aa9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AAA0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F79CE;
    20
}
/// $09AAA0: move.b $98(a6), d0
pub fn i_09aaa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AAA4;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09AAA4: cmpi.b #$b, d0
pub fn i_09aaa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AAA8;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xB, Size::Byte, false);
    8
}
/// $09AAA8: bcs.w $9aab0
pub fn i_09aaa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AAAC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x9AAB0; return 10; }
    8
}
/// $09AAAC: move.b #$a, d0
pub fn i_09aaac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AAB0;
    cpu.ir = 0x103C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $09AAB0: andi.w #$f, d0
pub fn i_09aab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AAB4;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09AAB4: lsl.w #$2, d0
pub fn i_09aab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AAB6;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $09AAB6: move.l (a0, d0.w), d0
pub fn i_09aab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AABA;
    cpu.ir = 0x2030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $09AABA: move.l d0, $76(a6)
pub fn i_09aaba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AABE;
    cpu.ir = 0x2D40;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $09AABE: clr.w $7a(a6)
pub fn i_09aabe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AAC2;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x7A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $09AAC2: jsr $9a87e(pc)
pub fn i_09aac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AAC6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9AAC6);
    cpu.pc = 0x9A87E;
    16
}
/// $09AAC6: move.b #$8, $45(a6)
pub fn i_09aac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AACC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09AACC: move.w #$8000, d0
pub fn i_09aacc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AAD0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09AAD0: jsr $28134.l
pub fn i_09aad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AAD6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AAD6);
    cpu.pc = 0x28134;
    16
}
/// $09AAD6: andi.w #$ffe3, $38(a6)
pub fn i_09aad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AADC;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09AADC: ori.w #$0, $38(a6)
pub fn i_09aadc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AAE2;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
// ===== Item_AmmoSeq_Show_09aae2  ($09AAE2, 114 B, props/items_score_crates_09a0bc.s)
/// $09AAE2: move.b #$ff, $32(a6)
pub fn i_09aae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AAE8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09AAE8: move.w #$50, $70(a6)
pub fn i_09aae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AAEE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x50;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09AAEE: lea.l $9aaf4(pc), a1
pub fn i_09aaee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AAF2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9AAF4;
    16
}
/// $09AAF2: move.l a1, (a6)
pub fn i_09aaf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AAF4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09AAF4: subq.w #$1, $70(a6)
pub fn i_09aaf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AAF8;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09AAF8: cmpi.w #$0, $70(a6)
pub fn i_09aaf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AAFE;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $09AAFE: bgt.w $9ab08
pub fn i_09aafe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB02;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x9AB08; return 10; }
    8
}
/// $09AB02: lea.l $9ab5c(pc), a1
pub fn i_09ab02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB06;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9AB5C;
    16
}
/// $09AB06: move.l a1, (a6)
pub fn i_09ab06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB08;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09AB08: jsr $27c8c.l
pub fn i_09ab08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AB0E);
    cpu.pc = 0x27C8C;
    16
}
/// $09AB0E: jsr $28d70.l
pub fn i_09ab0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AB14);
    cpu.pc = 0x28D70;
    16
}
/// $09AB14: jsr $9a836(pc)
pub fn i_09ab14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB18;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9AB18);
    cpu.pc = 0x9A836;
    16
}
/// $09AB18: bcc.w $9ab50
pub fn i_09ab18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB1C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9AB50; return 10; }
    8
}
/// $09AB1C: movea.l $76(a6), a2
pub fn i_09ab1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB20;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $09AB20: move.w $7a(a6), d2
pub fn i_09ab20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB24;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09AB24: move.b (a2, d2.w), d2
pub fn i_09ab24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB28;
    cpu.ir = 0x1432;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    14
}
/// $09AB28: andi.w #$3, d2
pub fn i_09ab28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB2C;
    cpu.ir = 0x242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09AB2C: lea.l $2f7a38.l, a2
pub fn i_09ab2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB32;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2F7A38;
    20
}
/// $09AB32: move.b (a2, d2.w), d1
pub fn i_09ab32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB36;
    cpu.ir = 0x1232;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    14
}
/// $09AB36: lea.l $2f7a3c.l, a2
pub fn i_09ab36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB3C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2F7A3C;
    20
}
/// $09AB3C: move.b (a2, d2.w), d2
pub fn i_09ab3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB40;
    cpu.ir = 0x1432;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    14
}
/// $09AB40: jsr $9a858(pc)
pub fn i_09ab40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB44;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9AB44);
    cpu.pc = 0x9A858;
    16
}
/// $09AB44: move.b #$ff, $32(a6)
pub fn i_09ab44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB4A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09AB4A: lea.l $9ae26(pc), a1
pub fn i_09ab4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB4E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9AE26;
    16
}
/// $09AB4E: move.l a1, (a6)
pub fn i_09ab4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB50;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09AB50: bra.w $9a75a
pub fn i_09ab50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB54;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A75A;
    10
}
// ===== Item_AmmoSeq_FadeOut_09ab5c  ($09AB5C, 50 B, props/items_score_crates_09a0bc.s)
/// $09AB5C: clr.w $70(a6)
pub fn i_09ab5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB60;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $09AB60: lea.l $9ab66(pc), a1
pub fn i_09ab60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB64;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9AB66;
    16
}
/// $09AB64: move.l a1, (a6)
pub fn i_09ab64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB66;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09AB66: lea.l $9ab54(pc), a0
pub fn i_09ab66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB6A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x9AB54;
    16
}
/// $09AB6A: move.w $70(a6), d0
pub fn i_09ab6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB6E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09AB6E: move.b (a0, d0.w), d0
pub fn i_09ab6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB72;
    cpu.ir = 0x1030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $09AB72: move.b d0, $32(a6)
pub fn i_09ab72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB76;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09AB76: addq.w #$1, $70(a6)
pub fn i_09ab76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB7A;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09AB7A: cmpi.w #$7, $70(a6)
pub fn i_09ab7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB80;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x7, Size::Word, false);
    16
}
/// $09AB80: bne.w $9ab8a
pub fn i_09ab80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB84;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9AB8A; return 10; }
    8
}
/// $09AB84: lea.l $9ab8e(pc), a1
pub fn i_09ab84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB88;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9AB8E;
    16
}
/// $09AB88: move.l a1, (a6)
pub fn i_09ab88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB8A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09AB8A: bra.w $9ab08
pub fn i_09ab8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB8E;
    cpu.ir = 0x6000;
    cpu.pc = 0x9AB08;
    10
}
// ===== Item_AmmoSeq_FadeIn_09ab8e  ($09AB8E, 52 B, props/items_score_crates_09a0bc.s)
/// $09AB8E: addq.w #$1, $7a(a6)
pub fn i_09ab8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB92;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x7A);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09AB92: jsr $9a87e(pc)
pub fn i_09ab92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB96;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9AB96);
    cpu.pc = 0x9A87E;
    16
}
/// $09AB96: lea.l $9ab9c(pc), a1
pub fn i_09ab96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB9A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9AB9C;
    16
}
/// $09AB9A: move.l a1, (a6)
pub fn i_09ab9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB9C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09AB9C: lea.l $9ab54(pc), a0
pub fn i_09ab9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ABA0;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x9AB54;
    16
}
/// $09ABA0: move.w $70(a6), d0
pub fn i_09aba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ABA4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09ABA4: move.b (a0, d0.w), d0
pub fn i_09aba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ABA8;
    cpu.ir = 0x1030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $09ABA8: move.b d0, $32(a6)
pub fn i_09aba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ABAC;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09ABAC: subq.w #$1, $70(a6)
pub fn i_09abac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ABB0;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09ABB0: tst.w $70(a6)
pub fn i_09abb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ABB4;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $09ABB4: bne.w $9abbe
pub fn i_09abb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ABB8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9ABBE; return 10; }
    8
}
/// $09ABB8: lea.l $9aae2(pc), a1
pub fn i_09abb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ABBC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9AAE2;
    16
}
/// $09ABBC: move.l a1, (a6)
pub fn i_09abbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ABBE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09ABBE: bra.w $9ab08
pub fn i_09abbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ABC2;
    cpu.ir = 0x6000;
    cpu.pc = 0x9AB08;
    10
}
// ===== Item_Tmpl289_Weapon_09abc2  ($09ABC2, 144 B, props/items_score_crates_09a0bc.s)
/// $09ABC2: move.w #$17f, d1
pub fn i_09abc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ABC6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x17F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09ABC6: jsr $236e.l
pub fn i_09abc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ABCC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9ABCC);
    cpu.pc = 0x236E;
    16
}
/// $09ABCC: move.w #$1a7, d1
pub fn i_09abcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ABD0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1A7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09ABD0: jsr $236e.l
pub fn i_09abd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ABD6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9ABD6);
    cpu.pc = 0x236E;
    16
}
/// $09ABD6: move.b #$8, $45(a6)
pub fn i_09abd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ABDC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09ABDC: move.w #$8000, d0
pub fn i_09abdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ABE0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09ABE0: jsr $28134.l
pub fn i_09abe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ABE6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9ABE6);
    cpu.pc = 0x28134;
    16
}
/// $09ABE6: andi.w #$ffe3, $38(a6)
pub fn i_09abe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ABEC;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09ABEC: ori.w #$0, $38(a6)
pub fn i_09abec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ABF2;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09ABF2: lea.l $2f680c.l, a0
pub fn i_09abf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ABF8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F680C;
    20
}
/// $09ABF8: jsr $28cd4.l
pub fn i_09abf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ABFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9ABFE);
    cpu.pc = 0x28CD4;
    16
}
/// $09ABFE: tst.b $98(a6)
pub fn i_09abfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC02;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $09AC02: bne.w $9ac0c
pub fn i_09ac02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC06;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9AC0C; return 10; }
    8
}
/// $09AC06: move.b #$a, $98(a6)
pub fn i_09ac06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC0C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09AC0C: tst.b $99(a6)
pub fn i_09ac0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC10;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $09AC10: bne.w $9ac1a
pub fn i_09ac10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC14;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9AC1A; return 10; }
    8
}
/// $09AC14: move.b #$64, $99(a6)
pub fn i_09ac14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC1A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x64;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09AC1A: lea.l $9ac20(pc), a1
pub fn i_09ac1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC1E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9AC20;
    16
}
/// $09AC1E: move.l a1, (a6)
pub fn i_09ac1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC20;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09AC20: jsr $27c8c.l
pub fn i_09ac20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AC26);
    cpu.pc = 0x27C8C;
    16
}
/// $09AC26: jsr $28d70.l
pub fn i_09ac26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC2C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AC2C);
    cpu.pc = 0x28D70;
    16
}
/// $09AC2C: jsr $9a836(pc)
pub fn i_09ac2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC30;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9AC30);
    cpu.pc = 0x9A836;
    16
}
/// $09AC30: bcc.w $9ac4e
pub fn i_09ac30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC34;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9AC4E; return 10; }
    8
}
/// $09AC34: move.b d0, $68(a6)
pub fn i_09ac34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC38;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09AC38: jsr $32c12.l
pub fn i_09ac38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AC3E);
    cpu.pc = 0x32C12;
    16
}
/// $09AC3E: lea.l $9ae36(pc), a1
pub fn i_09ac3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC42;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9AE36;
    16
}
/// $09AC42: move.l a1, (a6)
pub fn i_09ac42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC44;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09AC44: move.w #$1124, d0
pub fn i_09ac44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC48;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1124;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09AC48: jsr $2352.l
pub fn i_09ac48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AC4E);
    cpu.pc = 0x2352;
    16
}
/// $09AC4E: bra.w $9a75a
pub fn i_09ac4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC52;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A75A;
    10
}
// ===== Item_Tmpl290_WeaponSwap_09ac52  ($09AC52, 258 B, props/items_score_crates_09a0bc.s)
/// $09AC52: move.w #$17e, d1
pub fn i_09ac52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC56;
    cpu.ir = 0x323C;
    let v1: u32 = 0x17E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09AC56: jsr $236e.l
pub fn i_09ac56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AC5C);
    cpu.pc = 0x236E;
    16
}
/// $09AC5C: move.w #$1a8, d1
pub fn i_09ac5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC60;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1A8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09AC60: jsr $236e.l
pub fn i_09ac60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AC66);
    cpu.pc = 0x236E;
    16
}
/// $09AC66: move.b #$8, $45(a6)
pub fn i_09ac66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC6C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09AC6C: move.w #$8000, d0
pub fn i_09ac6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC70;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09AC70: jsr $28134.l
pub fn i_09ac70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AC76);
    cpu.pc = 0x28134;
    16
}
/// $09AC76: andi.w #$ffe3, $38(a6)
pub fn i_09ac76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC7C;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09AC7C: ori.w #$0, $38(a6)
pub fn i_09ac7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC82;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09AC82: lea.l $2f685c.l, a0
pub fn i_09ac82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC88;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F685C;
    20
}
/// $09AC88: jsr $28cd4.l
pub fn i_09ac88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC8E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AC8E);
    cpu.pc = 0x28CD4;
    16
}
/// $09AC8E: tst.b $98(a6)
pub fn i_09ac8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC92;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $09AC92: bne.w $9ac9c
pub fn i_09ac92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC96;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9AC9C; return 10; }
    8
}
/// $09AC96: move.b #$a, $98(a6)
pub fn i_09ac96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC9C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09AC9C: tst.b $99(a6)
pub fn i_09ac9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ACA0;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $09ACA0: bne.w $9acaa
pub fn i_09aca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ACA4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9ACAA; return 10; }
    8
}
/// $09ACA4: move.b #$a, $99(a6)
pub fn i_09aca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ACAA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09ACAA: lea.l $9acb0(pc), a1
pub fn i_09acaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ACAE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9ACB0;
    16
}
/// $09ACAE: move.l a1, (a6)
pub fn i_09acae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ACB0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09ACB0: jsr $27c8c.l
pub fn i_09acb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ACB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9ACB6);
    cpu.pc = 0x27C8C;
    16
}
/// $09ACB6: jsr $28d70.l
pub fn i_09acb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ACBC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9ACBC);
    cpu.pc = 0x28D70;
    16
}
/// $09ACBC: jsr $9a836(pc)
pub fn i_09acbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ACC0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9ACC0);
    cpu.pc = 0x9A836;
    16
}
/// $09ACC0: bcc.w $9ad50
pub fn i_09acc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ACC4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9AD50; return 10; }
    8
}
/// $09ACC4: cmpi.b #$0, d0
pub fn i_09acc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ACC8;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $09ACC8: bne.w $9acd6
pub fn i_09acc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ACCC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9ACD6; return 10; }
    8
}
/// $09ACCC: lea.l $100440.l, a1
pub fn i_09accc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ACD2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x100440;
    20
}
/// $09ACD2: bra.w $9acdc
pub fn i_09acd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ACD6;
    cpu.ir = 0x6000;
    cpu.pc = 0x9ACDC;
    10
}
/// $09ACD6: lea.l $1004e0.l, a1
pub fn i_09acd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ACDC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1004E0;
    20
}
/// $09ACDC: cmpi.b #$2, $70(a1)
pub fn i_09acdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ACE2;
    cpu.ir = 0xC29;
    let ad2 = cpu.a[1].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $09ACE2: bne.w $9ad16
pub fn i_09ace2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ACE6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9AD16; return 10; }
    8
}
/// $09ACE6: move.b d0, $68(a6)
pub fn i_09ace6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ACEA;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09ACEA: move.b $99(a6), $98(a6)
pub fn i_09acea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ACF0;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x98);
    bus.write8(ad4, v3 as u8);
    20
}
/// $09ACF0: jsr $2a28e.l
pub fn i_09acf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ACF6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9ACF6);
    cpu.pc = 0x2A28E;
    16
}
/// $09ACF6: lea.l $9b48a(pc), a1
pub fn i_09acf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ACFA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B48A;
    16
}
/// $09ACFA: jsr $4ae.l
pub fn i_09acfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD00;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AD00);
    cpu.pc = 0x4AE;
    16
}
/// $09AD00: jsr $5dd02.l
pub fn i_09ad00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD06;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AD06);
    cpu.pc = 0x5DD02;
    16
}
/// $09AD06: move.b $68(a6), $68(a0)
pub fn i_09ad06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD0C;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x68);
    bus.write8(ad4, v3 as u8);
    20
}
/// $09AD0C: move.b #$1, $98(a0)
pub fn i_09ad0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD12;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09AD12: bra.w $9ad40
pub fn i_09ad12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD16;
    cpu.ir = 0x6000;
    cpu.pc = 0x9AD40;
    10
}
/// $09AD16: move.b d0, $68(a6)
pub fn i_09ad16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD1A;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09AD1A: move.b $98(a6), $98(a6)
pub fn i_09ad1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD20;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x98);
    bus.write8(ad4, v3 as u8);
    20
}
/// $09AD20: jsr $32c7e.l
pub fn i_09ad20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AD26);
    cpu.pc = 0x32C7E;
    16
}
/// $09AD26: lea.l $9b48a(pc), a1
pub fn i_09ad26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD2A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B48A;
    16
}
/// $09AD2A: jsr $4ae.l
pub fn i_09ad2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AD30);
    cpu.pc = 0x4AE;
    16
}
/// $09AD30: jsr $5dd02.l
pub fn i_09ad30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AD36);
    cpu.pc = 0x5DD02;
    16
}
/// $09AD36: move.b $68(a6), $68(a0)
pub fn i_09ad36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD3C;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x68);
    bus.write8(ad4, v3 as u8);
    20
}
/// $09AD3C: clr.b $98(a0)
pub fn i_09ad3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD40;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x98);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $09AD40: lea.l $9ae2e(pc), a1
pub fn i_09ad40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD44;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9AE2E;
    16
}
/// $09AD44: move.l a1, (a6)
pub fn i_09ad44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD46;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09AD46: move.w #$1124, d0
pub fn i_09ad46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD4A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1124;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09AD4A: jsr $2352.l
pub fn i_09ad4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD50;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AD50);
    cpu.pc = 0x2352;
    16
}
/// $09AD50: bra.w $9a75a
pub fn i_09ad50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD54;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A75A;
    10
}
// ===== Item_Tmpl291_Bombs_09ad54  ($09AD54, 210 B, props/items_score_crates_09a0bc.s)
/// $09AD54: move.w #$180, d1
pub fn i_09ad54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD58;
    cpu.ir = 0x323C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09AD58: jsr $236e.l
pub fn i_09ad58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD5E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AD5E);
    cpu.pc = 0x236E;
    16
}
/// $09AD5E: move.w #$1a9, d1
pub fn i_09ad5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD62;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1A9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09AD62: jsr $236e.l
pub fn i_09ad62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD68;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AD68);
    cpu.pc = 0x236E;
    16
}
/// $09AD68: move.b #$8, $45(a6)
pub fn i_09ad68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD6E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09AD6E: move.w #$8000, d0
pub fn i_09ad6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD72;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09AD72: jsr $28134.l
pub fn i_09ad72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD78;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AD78);
    cpu.pc = 0x28134;
    16
}
/// $09AD78: andi.w #$ffe3, $38(a6)
pub fn i_09ad78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD7E;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09AD7E: ori.w #$0, $38(a6)
pub fn i_09ad7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD84;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09AD84: lea.l $2f68ac.l, a0
pub fn i_09ad84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD8A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F68AC;
    20
}
/// $09AD8A: jsr $28cd4.l
pub fn i_09ad8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AD90);
    cpu.pc = 0x28CD4;
    16
}
/// $09AD90: lea.l $9ad96(pc), a1
pub fn i_09ad90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD94;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9AD96;
    16
}
/// $09AD94: move.l a1, (a6)
pub fn i_09ad94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD96;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09AD96: jsr $27c8c.l
pub fn i_09ad96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AD9C);
    cpu.pc = 0x27C8C;
    16
}
/// $09AD9C: jsr $28d70.l
pub fn i_09ad9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ADA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9ADA2);
    cpu.pc = 0x28D70;
    16
}
/// $09ADA2: jsr $9a836(pc)
pub fn i_09ada2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ADA6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9ADA6);
    cpu.pc = 0x9A836;
    16
}
/// $09ADA6: bcc.w $9ae22
pub fn i_09ada6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ADAA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9AE22; return 10; }
    8
}
/// $09ADAA: cmpi.b #$0, d0
pub fn i_09adaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ADAE;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $09ADAE: bne.w $9adbc
pub fn i_09adae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ADB2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9ADBC; return 10; }
    8
}
/// $09ADB2: lea.l $100440.l, a1
pub fn i_09adb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ADB8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x100440;
    20
}
/// $09ADB8: bra.w $9adc2
pub fn i_09adb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ADBC;
    cpu.ir = 0x6000;
    cpu.pc = 0x9ADC2;
    10
}
/// $09ADBC: lea.l $1004e0.l, a1
pub fn i_09adbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ADC2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1004E0;
    20
}
/// $09ADC2: move.b d0, $68(a6)
pub fn i_09adc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ADC6;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09ADC6: cmpi.b #$2, $70(a1)
pub fn i_09adc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ADCC;
    cpu.ir = 0xC29;
    let ad2 = cpu.a[1].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $09ADCC: bne.w $9ae0e
pub fn i_09adcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ADD0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9AE0E; return 10; }
    8
}
/// $09ADD0: move.b d0, $68(a6)
pub fn i_09add0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ADD4;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09ADD4: move.b $98(a6), d0
pub fn i_09add4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ADD8;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09ADD8: andi.w #$ff, d0
pub fn i_09add8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ADDC;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09ADDC: jsr $2a2ba.l
pub fn i_09addc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ADE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9ADE2);
    cpu.pc = 0x2A2BA;
    16
}
/// $09ADE2: bcc.w $9adee
pub fn i_09ade2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ADE6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9ADEE; return 10; }
    8
}
/// $09ADE6: jsr $9ba34(pc)
pub fn i_09ade6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ADEA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9ADEA);
    cpu.pc = 0x9BA34;
    16
}
/// $09ADEA: bra.w $9ae0a
pub fn i_09adea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ADEE;
    cpu.ir = 0x6000;
    cpu.pc = 0x9AE0A;
    10
}
/// $09ADEE: lea.l $9b48a(pc), a1
pub fn i_09adee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ADF2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B48A;
    16
}
/// $09ADF2: jsr $4ae.l
pub fn i_09adf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ADF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9ADF8);
    cpu.pc = 0x4AE;
    16
}
/// $09ADF8: jsr $5dd02.l
pub fn i_09adf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ADFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9ADFE);
    cpu.pc = 0x5DD02;
    16
}
/// $09ADFE: move.b $68(a6), $68(a0)
pub fn i_09adfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE04;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x68);
    bus.write8(ad4, v3 as u8);
    20
}
/// $09AE04: move.b #$3, $98(a0)
pub fn i_09ae04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE0A;
    cpu.ir = 0x117C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09AE0A: bra.w $9ae12
pub fn i_09ae0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE0E;
    cpu.ir = 0x6000;
    cpu.pc = 0x9AE12;
    10
}
/// $09AE0E: jsr $9ba34(pc)
pub fn i_09ae0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE12;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9AE12);
    cpu.pc = 0x9BA34;
    16
}
/// $09AE12: lea.l $9ae3e(pc), a1
pub fn i_09ae12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE16;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9AE3E;
    16
}
/// $09AE16: move.l a1, (a6)
pub fn i_09ae16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE18;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09AE18: move.w #$1124, d0
pub fn i_09ae18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE1C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1124;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09AE1C: jsr $2352.l
pub fn i_09ae1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE22;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AE22);
    cpu.pc = 0x2352;
    16
}
/// $09AE22: bra.w $9a75a
pub fn i_09ae22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE26;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A75A;
    10
}
// ===== Item_Taken_SndA_09ae26  ($09AE26, 8 B, props/items_score_crates_09a0bc.s)
/// $09AE26: move.w #$1a6, d1
pub fn i_09ae26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE2A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1A6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09AE2A: bra.w $9ae42
pub fn i_09ae2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE2E;
    cpu.ir = 0x6000;
    cpu.pc = 0x9AE42;
    10
}
// ===== Item_Taken_SndB_09ae2e  ($09AE2E, 8 B, props/items_score_crates_09a0bc.s)
/// $09AE2E: move.w #$1a7, d1
pub fn i_09ae2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE32;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1A7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09AE32: bra.w $9ae42
pub fn i_09ae32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE36;
    cpu.ir = 0x6000;
    cpu.pc = 0x9AE42;
    10
}
// ===== Item_Taken_SndC_09ae36  ($09AE36, 8 B, props/items_score_crates_09a0bc.s)
/// $09AE36: move.w #$1a8, d1
pub fn i_09ae36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE3A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1A8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09AE3A: bra.w $9ae42
pub fn i_09ae3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE3E;
    cpu.ir = 0x6000;
    cpu.pc = 0x9AE42;
    10
}
// ===== Item_Taken_SndD_09ae3e  ($09AE3E, 80 B, props/items_score_crates_09a0bc.s)
/// $09AE3E: move.w #$1a9, d1
pub fn i_09ae3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE42;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1A9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09AE42: jsr $236e.l
pub fn i_09ae42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE48;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AE48);
    cpu.pc = 0x236E;
    16
}
/// $09AE48: move.w #$2, $70(a6)
pub fn i_09ae48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE4E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09AE4E: move.w #$c000, d0
pub fn i_09ae4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE52;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09AE52: jsr $28134.l
pub fn i_09ae52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE58;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AE58);
    cpu.pc = 0x28134;
    16
}
/// $09AE58: andi.w #$ffe3, $38(a6)
pub fn i_09ae58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE5E;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09AE5E: ori.w #$0, $38(a6)
pub fn i_09ae5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE64;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09AE64: lea.l $9ae6a(pc), a1
pub fn i_09ae64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE68;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9AE6A;
    16
}
/// $09AE68: move.l a1, (a6)
pub fn i_09ae68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE6A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09AE6A: jsr $2783a.l
pub fn i_09ae6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE70;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AE70);
    cpu.pc = 0x2783A;
    16
}
/// $09AE70: jsr $28d70.l
pub fn i_09ae70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AE76);
    cpu.pc = 0x28D70;
    16
}
/// $09AE76: subq.w #$1, $70(a6)
pub fn i_09ae76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE7A;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09AE7A: cmpi.w #$0, $70(a6)
pub fn i_09ae7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE80;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $09AE80: bgt.w $9ae8a
pub fn i_09ae80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE84;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x9AE8A; return 10; }
    8
}
/// $09AE84: lea.l $9ae8e(pc), a1
pub fn i_09ae84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE88;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9AE8E;
    16
}
/// $09AE88: move.l a1, (a6)
pub fn i_09ae88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE8A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09AE8A: bra.w $9a76a
pub fn i_09ae8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE8E;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A76A;
    10
}
// ===== Item_Taken_Rise_09ae8e  ($09AE8E, 54 B, props/items_score_crates_09a0bc.s)
/// $09AE8E: move.w #$180, d1
pub fn i_09ae8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE92;
    cpu.ir = 0x323C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09AE92: jsr $236e.l
pub fn i_09ae92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AE98);
    cpu.pc = 0x236E;
    16
}
/// $09AE98: lea.l $2f674c.l, a0
pub fn i_09ae98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE9E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F674C;
    20
}
/// $09AE9E: jsr $28cd4.l
pub fn i_09ae9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AEA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AEA4);
    cpu.pc = 0x28CD4;
    16
}
/// $09AEA4: lea.l $9aeaa(pc), a1
pub fn i_09aea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AEA8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9AEAA;
    16
}
/// $09AEA8: move.l a1, (a6)
pub fn i_09aea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AEAA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09AEAA: jsr $2783a.l
pub fn i_09aeaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AEB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AEB0);
    cpu.pc = 0x2783A;
    16
}
/// $09AEB0: jsr $28d70.l
pub fn i_09aeb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AEB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AEB6);
    cpu.pc = 0x28D70;
    16
}
/// $09AEB6: bcc.w $9aec0
pub fn i_09aeb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AEBA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9AEC0; return 10; }
    8
}
/// $09AEBA: jmp $518.l
pub fn i_09aeba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AEC0;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $09AEC0: bra.w $9a76a
pub fn i_09aec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AEC4;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A76A;
    10
}
// ===== Item_Tmpl294_Food_09aec4  ($09AEC4, 484 B, props/items_score_crates_09a0bc.s)
/// $09AEC4: clr.b $9c(a6)
pub fn i_09aec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AEC8;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x9C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $09AEC8: lea.l $2f6b78.l, a0
pub fn i_09aec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AECE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F6B78;
    20
}
/// $09AECE: jsr $28cd4.l
pub fn i_09aece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AED4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AED4);
    cpu.pc = 0x28CD4;
    16
}
/// $09AED4: bra.w $9b05c
pub fn i_09aed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AED8;
    cpu.ir = 0x6000;
    cpu.pc = 0x9B05C;
    10
}
/// $09AED8: clr.b $9c(a6)
pub fn i_09aed8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AEDC;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x9C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $09AEDC: lea.l $2f6b22.l, a0
pub fn i_09aedc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AEE2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F6B22;
    20
}
/// $09AEE2: jsr $28cd4.l
pub fn i_09aee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AEE8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AEE8);
    cpu.pc = 0x28CD4;
    16
}
/// $09AEE8: bra.w $9b05c
pub fn i_09aee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AEEC;
    cpu.ir = 0x6000;
    cpu.pc = 0x9B05C;
    10
}
/// $09AEEC: move.b #$1, $9c(a6)
pub fn i_09aeec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AEF2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09AEF2: lea.l $2f6ae0.l, a0
pub fn i_09aef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AEF8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F6AE0;
    20
}
/// $09AEF8: jsr $28cd4.l
pub fn i_09aef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AEFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AEFE);
    cpu.pc = 0x28CD4;
    16
}
/// $09AEFE: bra.w $9b05c
pub fn i_09aefe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF02;
    cpu.ir = 0x6000;
    cpu.pc = 0x9B05C;
    10
}
/// $09AF02: clr.b $9c(a6)
pub fn i_09af02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF06;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x9C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $09AF06: lea.l $2f68fc.l, a0
pub fn i_09af06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF0C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F68FC;
    20
}
/// $09AF0C: jsr $28cd4.l
pub fn i_09af0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AF12);
    cpu.pc = 0x28CD4;
    16
}
/// $09AF12: bra.w $9b05c
pub fn i_09af12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF16;
    cpu.ir = 0x6000;
    cpu.pc = 0x9B05C;
    10
}
/// $09AF16: clr.b $9c(a6)
pub fn i_09af16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF1A;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x9C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $09AF1A: lea.l $2f6bce.l, a0
pub fn i_09af1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF20;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F6BCE;
    20
}
/// $09AF20: jsr $28cd4.l
pub fn i_09af20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AF26);
    cpu.pc = 0x28CD4;
    16
}
/// $09AF26: bra.w $9b064
pub fn i_09af26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF2A;
    cpu.ir = 0x6000;
    cpu.pc = 0x9B064;
    10
}
/// $09AF2A: clr.b $9c(a6)
pub fn i_09af2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF2E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x9C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $09AF2E: lea.l $2f6cca.l, a0
pub fn i_09af2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF34;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F6CCA;
    20
}
/// $09AF34: jsr $28cd4.l
pub fn i_09af34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AF3A);
    cpu.pc = 0x28CD4;
    16
}
/// $09AF3A: bra.w $9b064
pub fn i_09af3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF3E;
    cpu.ir = 0x6000;
    cpu.pc = 0x9B064;
    10
}
/// $09AF3E: clr.b $9c(a6)
pub fn i_09af3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF42;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x9C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $09AF42: lea.l $2f6e4c.l, a0
pub fn i_09af42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF48;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F6E4C;
    20
}
/// $09AF48: jsr $28cd4.l
pub fn i_09af48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AF4E);
    cpu.pc = 0x28CD4;
    16
}
/// $09AF4E: bra.w $9b064
pub fn i_09af4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF52;
    cpu.ir = 0x6000;
    cpu.pc = 0x9B064;
    10
}
/// $09AF52: move.b #$1, $9c(a6)
pub fn i_09af52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF58;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09AF58: lea.l $2f7002.l, a0
pub fn i_09af58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF5E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F7002;
    20
}
/// $09AF5E: jsr $28cd4.l
pub fn i_09af5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF64;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AF64);
    cpu.pc = 0x28CD4;
    16
}
/// $09AF64: bra.w $9b05c
pub fn i_09af64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF68;
    cpu.ir = 0x6000;
    cpu.pc = 0x9B05C;
    10
}
/// $09AF68: clr.b $9c(a6)
pub fn i_09af68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF6C;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x9C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $09AF6C: lea.l $2f706c.l, a0
pub fn i_09af6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF72;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F706C;
    20
}
/// $09AF72: jsr $28cd4.l
pub fn i_09af72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF78;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AF78);
    cpu.pc = 0x28CD4;
    16
}
/// $09AF78: bra.w $9b05c
pub fn i_09af78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF7C;
    cpu.ir = 0x6000;
    cpu.pc = 0x9B05C;
    10
}
/// $09AF7C: clr.b $9c(a6)
pub fn i_09af7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF80;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x9C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $09AF80: lea.l $2f70b8.l, a0
pub fn i_09af80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF86;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F70B8;
    20
}
/// $09AF86: jsr $28cd4.l
pub fn i_09af86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF8C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AF8C);
    cpu.pc = 0x28CD4;
    16
}
/// $09AF8C: bra.w $9b064
pub fn i_09af8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF90;
    cpu.ir = 0x6000;
    cpu.pc = 0x9B064;
    10
}
/// $09AF90: clr.b $9c(a6)
pub fn i_09af90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF94;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x9C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $09AF94: lea.l $2f7186.l, a0
pub fn i_09af94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF9A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F7186;
    20
}
/// $09AF9A: jsr $28cd4.l
pub fn i_09af9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AFA0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AFA0);
    cpu.pc = 0x28CD4;
    16
}
/// $09AFA0: bra.w $9b05c
pub fn i_09afa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AFA4;
    cpu.ir = 0x6000;
    cpu.pc = 0x9B05C;
    10
}
/// $09AFA4: clr.b $9c(a6)
pub fn i_09afa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AFA8;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x9C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $09AFA8: lea.l $2f7204.l, a0
pub fn i_09afa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AFAE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F7204;
    20
}
/// $09AFAE: jsr $28cd4.l
pub fn i_09afae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AFB4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AFB4);
    cpu.pc = 0x28CD4;
    16
}
/// $09AFB4: bra.w $9b05c
pub fn i_09afb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AFB8;
    cpu.ir = 0x6000;
    cpu.pc = 0x9B05C;
    10
}
/// $09AFB8: clr.b $9c(a6)
pub fn i_09afb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AFBC;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x9C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $09AFBC: lea.l $2f7290.l, a0
pub fn i_09afbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AFC2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F7290;
    20
}
/// $09AFC2: jsr $28cd4.l
pub fn i_09afc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AFC8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AFC8);
    cpu.pc = 0x28CD4;
    16
}
/// $09AFC8: bra.w $9b064
pub fn i_09afc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AFCC;
    cpu.ir = 0x6000;
    cpu.pc = 0x9B064;
    10
}
/// $09AFCC: clr.b $9c(a6)
pub fn i_09afcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AFD0;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x9C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $09AFD0: lea.l $2f73b8.l, a0
pub fn i_09afd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AFD6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F73B8;
    20
}
/// $09AFD6: jsr $28cd4.l
pub fn i_09afd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AFDC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AFDC);
    cpu.pc = 0x28CD4;
    16
}
/// $09AFDC: bra.w $9b064
pub fn i_09afdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AFE0;
    cpu.ir = 0x6000;
    cpu.pc = 0x9B064;
    10
}
/// $09AFE0: clr.b $9c(a6)
pub fn i_09afe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AFE4;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x9C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $09AFE4: lea.l $2f745e.l, a0
pub fn i_09afe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AFEA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F745E;
    20
}
/// $09AFEA: jsr $28cd4.l
pub fn i_09afea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AFF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AFF0);
    cpu.pc = 0x28CD4;
    16
}
/// $09AFF0: bra.w $9b064
pub fn i_09aff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AFF4;
    cpu.ir = 0x6000;
    cpu.pc = 0x9B064;
    10
}
/// $09AFF4: clr.b $9c(a6)
pub fn i_09aff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AFF8;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x9C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $09AFF8: lea.l $2f74a0.l, a0
pub fn i_09aff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AFFE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F74A0;
    20
}
/// $09AFFE: jsr $28cd4.l
pub fn i_09affe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B004;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B004);
    cpu.pc = 0x28CD4;
    16
}
/// $09B004: bra.w $9b05c
pub fn i_09b004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B008;
    cpu.ir = 0x6000;
    cpu.pc = 0x9B05C;
    10
}
/// $09B008: move.b #$2, $9c(a6)
pub fn i_09b008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B00E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09B00E: lea.l $2f75f2.l, a0
pub fn i_09b00e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B014;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F75F2;
    20
}
/// $09B014: jsr $28cd4.l
pub fn i_09b014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B01A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B01A);
    cpu.pc = 0x28CD4;
    16
}
/// $09B01A: bra.w $9b064
pub fn i_09b01a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B01E;
    cpu.ir = 0x6000;
    cpu.pc = 0x9B064;
    10
}
/// $09B01E: move.b #$1, $9c(a6)
pub fn i_09b01e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B024;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09B024: lea.l $2f76e8.l, a0
pub fn i_09b024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B02A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F76E8;
    20
}
/// $09B02A: jsr $28cd4.l
pub fn i_09b02a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B030;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B030);
    cpu.pc = 0x28CD4;
    16
}
/// $09B030: bra.w $9b05c
pub fn i_09b030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B034;
    cpu.ir = 0x6000;
    cpu.pc = 0x9B05C;
    10
}
/// $09B034: clr.b $9c(a6)
pub fn i_09b034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B038;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x9C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $09B038: lea.l $2f755a.l, a0
pub fn i_09b038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B03E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F755A;
    20
}
/// $09B03E: jsr $28cd4.l
pub fn i_09b03e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B044;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B044);
    cpu.pc = 0x28CD4;
    16
}
/// $09B044: bra.w $9b05c
pub fn i_09b044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B048;
    cpu.ir = 0x6000;
    cpu.pc = 0x9B05C;
    10
}
/// $09B048: clr.b $9c(a6)
pub fn i_09b048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B04C;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x9C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $09B04C: lea.l $2f75b0.l, a0
pub fn i_09b04c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B052;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F75B0;
    20
}
/// $09B052: jsr $28cd4.l
pub fn i_09b052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B058;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B058);
    cpu.pc = 0x28CD4;
    16
}
/// $09B058: bra.w $9b064
pub fn i_09b058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B05C;
    cpu.ir = 0x6000;
    cpu.pc = 0x9B064;
    10
}
/// $09B05C: move.w #$1ac, d1
pub fn i_09b05c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B060;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1AC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B060: bra.w $9b068
pub fn i_09b060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B064;
    cpu.ir = 0x6000;
    cpu.pc = 0x9B068;
    10
}
/// $09B064: move.w #$1c6, d1
pub fn i_09b064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B068;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1C6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B068: cmpi.b #$ff, $9a(a6)
pub fn i_09b068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B06E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $09B06E: beq.w $9b078
pub fn i_09b06e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B072;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9B078; return 10; }
    8
}
/// $09B072: eori.b #$1, $3a(a6)
pub fn i_09b072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B078;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $09B078: jsr $236e.l
pub fn i_09b078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B07E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B07E);
    cpu.pc = 0x236E;
    16
}
/// $09B07E: move.b #$8, $45(a6)
pub fn i_09b07e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B084;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09B084: move.w #$8000, d0
pub fn i_09b084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B088;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B088: jsr $28134.l
pub fn i_09b088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B08E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B08E);
    cpu.pc = 0x28134;
    16
}
/// $09B08E: andi.w #$ffe3, $38(a6)
pub fn i_09b08e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B094;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09B094: ori.w #$0, $38(a6)
pub fn i_09b094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B09A;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09B09A: jsr $9bb3c(pc)
pub fn i_09b09a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B09E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9B09E);
    cpu.pc = 0x9BB3C;
    16
}
/// $09B09E: move.w d0, $70(a6)
pub fn i_09b09e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0A2;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B0A2: lea.l $9b0a8(pc), a1
pub fn i_09b0a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0A6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B0A8;
    16
}
/// $09B0A6: move.l a1, (a6)
pub fn i_09b0a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0A8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Item_Food_Wait_09b0a8  ($09B0A8, 48 B, props/items_score_crates_09a0bc.s)
/// $09B0A8: jsr $9b8c0(pc)
pub fn i_09b0a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0AC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9B0AC);
    cpu.pc = 0x9B8C0;
    16
}
/// $09B0AC: jsr $28d70.l
pub fn i_09b0ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B0B2);
    cpu.pc = 0x28D70;
    16
}
/// $09B0B2: subq.w #$1, $70(a6)
pub fn i_09b0b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0B6;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09B0B6: cmpi.w #$0, $70(a6)
pub fn i_09b0b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0BC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $09B0BC: bgt.w $9b0c6
pub fn i_09b0bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0C0;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x9B0C6; return 10; }
    8
}
/// $09B0C0: lea.l $9b0d8(pc), a1
pub fn i_09b0c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0C4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B0D8;
    16
}
/// $09B0C4: move.l a1, (a6)
pub fn i_09b0c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0C6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B0C6: jsr $9b920(pc)
pub fn i_09b0c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0CA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9B0CA);
    cpu.pc = 0x9B920;
    16
}
/// $09B0CA: bcc.w $9b0d4
pub fn i_09b0ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0CE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9B0D4; return 10; }
    8
}
/// $09B0CE: lea.l $9b474(pc), a1
pub fn i_09b0ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0D2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B474;
    16
}
/// $09B0D2: move.l a1, (a6)
pub fn i_09b0d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0D4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B0D4: bra.w $9a75a
pub fn i_09b0d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0D8;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A75A;
    10
}
// ===== Item_Food_Blink_09b0d8  ($09B0D8, 6 B, props/items_score_crates_09a0bc.s)
/// $09B0D8: move.b #$1e, $70(a6)
pub fn i_09b0d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0DE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== Item_Food_BlinkLoop_09b0de  ($09B0DE, 368 B, props/items_score_crates_09a0bc.s)
/// $09B0DE: move.w #$8000, d0
pub fn i_09b0de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0E2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B0E2: jsr $28134.l
pub fn i_09b0e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B0E8);
    cpu.pc = 0x28134;
    16
}
/// $09B0E8: andi.w #$ffe3, $38(a6)
pub fn i_09b0e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0EE;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09B0EE: ori.w #$0, $38(a6)
pub fn i_09b0ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0F4;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09B0F4: lea.l $9b0fa(pc), a1
pub fn i_09b0f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0F8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B0FA;
    16
}
/// $09B0F8: move.l a1, (a6)
pub fn i_09b0f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0FA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B0FA: jsr $9b8c0(pc)
pub fn i_09b0fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0FE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9B0FE);
    cpu.pc = 0x9B8C0;
    16
}
/// $09B0FE: btst.b #$0, $70(a6)
pub fn i_09b0fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B104;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $09B104: beq.w $9b10e
pub fn i_09b104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B108;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9B10E; return 10; }
    8
}
/// $09B108: jsr $28d70.l
pub fn i_09b108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B10E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B10E);
    cpu.pc = 0x28D70;
    16
}
/// $09B10E: subq.b #$1, $70(a6)
pub fn i_09b10e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B112;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $09B112: bcc.w $9b11c
pub fn i_09b112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B116;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9B11C; return 10; }
    8
}
/// $09B116: lea.l $9b474(pc), a1
pub fn i_09b116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B11A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B474;
    16
}
/// $09B11A: move.l a1, (a6)
pub fn i_09b11a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B11C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B11C: bra.b $9b0c6
pub fn i_09b11c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B11E;
    cpu.ir = 0x60A8;
    cpu.pc = 0x9B0C6;
    10
}
/// $09B11E: clr.b $9c(a6)
pub fn i_09b11e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B122;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x9C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $09B122: cmpi.b #$ff, $9a(a6)
pub fn i_09b122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B128;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $09B128: beq.w $9b132
pub fn i_09b128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B12C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9B132; return 10; }
    8
}
/// $09B12C: eori.b #$1, $3a(a6)
pub fn i_09b12c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B132;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $09B132: move.b $99(a6), d0
pub fn i_09b132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B136;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09B136: andi.w #$ff, d0
pub fn i_09b136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B13A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09B13A: lsl.w #$1, d0
pub fn i_09b13a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B13C;
    cpu.ir = 0xE348;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $09B13C: tst.w d0
pub fn i_09b13c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B13E;
    cpu.ir = 0x4A40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $09B13E: bne.w $9b146
pub fn i_09b13e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B142;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9B146; return 10; }
    8
}
/// $09B142: move.w #$7fff, d0
pub fn i_09b142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B146;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7FFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B146: move.w d0, $70(a6)
pub fn i_09b146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B14A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B14A: clr.w $72(a6)
pub fn i_09b14a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B14E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $09B14E: move.w #$1ac, d1
pub fn i_09b14e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B152;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1AC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B152: jsr $236e.l
pub fn i_09b152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B158;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B158);
    cpu.pc = 0x236E;
    16
}
/// $09B158: move.w #$8000, d0
pub fn i_09b158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B15C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B15C: jsr $28134.l
pub fn i_09b15c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B162;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B162);
    cpu.pc = 0x28134;
    16
}
/// $09B162: andi.w #$ffe3, $38(a6)
pub fn i_09b162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B168;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09B168: ori.w #$0, $38(a6)
pub fn i_09b168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B16E;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09B16E: lea.l $2f6a1a.l, a0
pub fn i_09b16e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B174;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F6A1A;
    20
}
/// $09B174: jsr $28cd4.l
pub fn i_09b174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B17A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B17A);
    cpu.pc = 0x28CD4;
    16
}
/// $09B17A: lea.l $9b180(pc), a1
pub fn i_09b17a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B17E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B180;
    16
}
/// $09B17E: move.l a1, (a6)
pub fn i_09b17e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B180;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B180: jsr $9b8c0(pc)
pub fn i_09b180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B184;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9B184);
    cpu.pc = 0x9B8C0;
    16
}
/// $09B184: jsr $28d70.l
pub fn i_09b184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B18A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B18A);
    cpu.pc = 0x28D70;
    16
}
/// $09B18A: bcc.w $9b1a6
pub fn i_09b18a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B18E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9B1A6; return 10; }
    8
}
/// $09B18E: lea.l $2f6aa8.l, a0
pub fn i_09b18e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B194;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F6AA8;
    20
}
/// $09B194: jsr $28cd4.l
pub fn i_09b194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B19A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B19A);
    cpu.pc = 0x28CD4;
    16
}
/// $09B19A: move.b #$14, $70(a6)
pub fn i_09b19a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B1A0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09B1A0: lea.l $9b0de(pc), a1
pub fn i_09b1a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B1A4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B0DE;
    16
}
/// $09B1A4: move.l a1, (a6)
pub fn i_09b1a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B1A6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B1A6: subq.w #$1, $70(a6)
pub fn i_09b1a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B1AA;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09B1AA: cmpi.w #$0, $70(a6)
pub fn i_09b1aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B1B0;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $09B1B0: bgt.w $9b1c6
pub fn i_09b1b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B1B4;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x9B1C6; return 10; }
    8
}
/// $09B1B4: lea.l $2f6a5c.l, a0
pub fn i_09b1b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B1BA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F6A5C;
    20
}
/// $09B1BA: jsr $28cd4.l
pub fn i_09b1ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B1C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B1C0);
    cpu.pc = 0x28CD4;
    16
}
/// $09B1C0: move.w #$7fff, $70(a6)
pub fn i_09b1c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B1C6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7FFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B1C6: bra.w $9b0c6
pub fn i_09b1c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B1CA;
    cpu.ir = 0x6000;
    cpu.pc = 0x9B0C6;
    10
}
/// $09B1CA: clr.b $9c(a6)
pub fn i_09b1ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B1CE;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x9C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $09B1CE: cmpi.b #$ff, $9a(a6)
pub fn i_09b1ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B1D4;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $09B1D4: beq.w $9b1de
pub fn i_09b1d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B1D8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9B1DE; return 10; }
    8
}
/// $09B1D8: eori.b #$1, $3a(a6)
pub fn i_09b1d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B1DE;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $09B1DE: jsr $9bb3c(pc)
pub fn i_09b1de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B1E2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9B1E2);
    cpu.pc = 0x9BB3C;
    16
}
/// $09B1E2: cmpi.w #$46, d0
pub fn i_09b1e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B1E6;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x46, Size::Word, false);
    8
}
/// $09B1E6: bgt.w $9b1ee
pub fn i_09b1e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B1EA;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x9B1EE; return 10; }
    8
}
/// $09B1EA: move.w #$46, d0
pub fn i_09b1ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B1EE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x46;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B1EE: subi.w #$46, d0
pub fn i_09b1ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B1F2;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x46, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09B1F2: move.w d0, $70(a6)
pub fn i_09b1f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B1F6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B1F6: clr.w $72(a6)
pub fn i_09b1f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B1FA;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $09B1FA: move.w #$1c6, d1
pub fn i_09b1fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B1FE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1C6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B1FE: jsr $236e.l
pub fn i_09b1fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B204;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B204);
    cpu.pc = 0x236E;
    16
}
/// $09B204: move.w #$8000, d0
pub fn i_09b204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B208;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B208: jsr $28134.l
pub fn i_09b208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B20E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B20E);
    cpu.pc = 0x28134;
    16
}
/// $09B20E: andi.w #$ffe3, $38(a6)
pub fn i_09b20e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B214;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09B214: ori.w #$0, $38(a6)
pub fn i_09b214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B21A;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09B21A: lea.l $2f6f06.l, a0
pub fn i_09b21a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B220;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F6F06;
    20
}
/// $09B220: jsr $28cd4.l
pub fn i_09b220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B226;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B226);
    cpu.pc = 0x28CD4;
    16
}
/// $09B226: lea.l $9b22c(pc), a1
pub fn i_09b226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B22A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B22C;
    16
}
/// $09B22A: move.l a1, (a6)
pub fn i_09b22a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B22C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B22C: jsr $9b8c0(pc)
pub fn i_09b22c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B230;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9B230);
    cpu.pc = 0x9B8C0;
    16
}
/// $09B230: jsr $28d70.l
pub fn i_09b230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B236;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B236);
    cpu.pc = 0x28D70;
    16
}
/// $09B236: subq.w #$1, $70(a6)
pub fn i_09b236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B23A;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09B23A: cmpi.w #$0, $70(a6)
pub fn i_09b23a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B240;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $09B240: bgt.w $9b24a
pub fn i_09b240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B244;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x9B24A; return 10; }
    8
}
/// $09B244: lea.l $9b24e(pc), a1
pub fn i_09b244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B248;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B24E;
    16
}
/// $09B248: move.l a1, (a6)
pub fn i_09b248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B24A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B24A: bra.w $9b0c6
pub fn i_09b24a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B24E;
    cpu.ir = 0x6000;
    cpu.pc = 0x9B0C6;
    10
}
// ===== Item_Food_Rot_09b24e  ($09B24E, 64 B, props/items_score_crates_09a0bc.s)
/// $09B24E: lea.l $2f6f34.l, a0
pub fn i_09b24e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B254;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F6F34;
    20
}
/// $09B254: jsr $28cd4.l
pub fn i_09b254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B25A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B25A);
    cpu.pc = 0x28CD4;
    16
}
/// $09B25A: move.w #$8000, d0
pub fn i_09b25a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B25E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B25E: jsr $28134.l
pub fn i_09b25e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B264;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B264);
    cpu.pc = 0x28134;
    16
}
/// $09B264: andi.w #$ffe3, $38(a6)
pub fn i_09b264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B26A;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09B26A: ori.w #$0, $38(a6)
pub fn i_09b26a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B270;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09B270: lea.l $9b276(pc), a1
pub fn i_09b270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B274;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B276;
    16
}
/// $09B274: move.l a1, (a6)
pub fn i_09b274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B276;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B276: jsr $9b8c0(pc)
pub fn i_09b276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B27A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9B27A);
    cpu.pc = 0x9B8C0;
    16
}
/// $09B27A: jsr $28d70.l
pub fn i_09b27a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B280;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B280);
    cpu.pc = 0x28D70;
    16
}
/// $09B280: bcc.w $9b28a
pub fn i_09b280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B284;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9B28A; return 10; }
    8
}
/// $09B284: lea.l $9b474(pc), a1
pub fn i_09b284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B288;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B474;
    16
}
/// $09B288: move.l a1, (a6)
pub fn i_09b288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B28A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B28A: bra.w $9b0c6
pub fn i_09b28a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B28E;
    cpu.ir = 0x6000;
    cpu.pc = 0x9B0C6;
    10
}
// ===== Item_Food_Thrown_09b28e  ($09B28E, 200 B, props/items_score_crates_09a0bc.s)
/// $09B28E: move.w #$1ac, d1
pub fn i_09b28e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B292;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1AC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B292: jsr $236e.l
pub fn i_09b292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B298;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B298);
    cpu.pc = 0x236E;
    16
}
/// $09B298: jsr $5e9b6.l
pub fn i_09b298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B29E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B29E);
    cpu.pc = 0x5E9B6;
    16
}
/// $09B29E: andi.b #$1, d0
pub fn i_09b29e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B2A2;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $09B2A2: move.b d0, $3a(a6)
pub fn i_09b2a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B2A6;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09B2A6: jsr $5e9b6.l
pub fn i_09b2a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B2AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B2AC);
    cpu.pc = 0x5E9B6;
    16
}
/// $09B2AC: andi.w #$3, d0
pub fn i_09b2ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B2B0;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09B2B0: move.w d0, d1
pub fn i_09b2b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B2B2;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B2B2: add.w d0, d0
pub fn i_09b2b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B2B4;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09B2B4: add.w d1, d0
pub fn i_09b2b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B2B6;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09B2B6: add.w d0, d0
pub fn i_09b2b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B2B8;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09B2B8: move.w d0, $5c(a6)
pub fn i_09b2b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B2BC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B2BC: movea.l #$2f7a40, a0
pub fn i_09b2bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B2C2;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2F7A40;
    cpu.a[0] = v1;
    4
}
/// $09B2C2: move.w (a0, d0.w), d1
pub fn i_09b2c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B2C6;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $09B2C6: move.w $2(a0, d0.w), d2
pub fn i_09b2c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B2CA;
    cpu.ir = 0x3430;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $09B2CA: move.w $4(a0, d0.w), d3
pub fn i_09b2ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B2CE;
    cpu.ir = 0x3630;
    let ad2 = cpu.a[0].wrapping_add(0x4).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $09B2CE: btst.b #$0, $3a(a6)
pub fn i_09b2ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B2D4;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $09B2D4: bne.w $9b2da
pub fn i_09b2d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B2D8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9B2DA; return 10; }
    8
}
/// $09B2D8: neg.w d1
pub fn i_09b2d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B2DA;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $09B2DA: move.w d1, $28(a6)
pub fn i_09b2da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B2DE;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B2DE: move.w d2, $2e(a6)
pub fn i_09b2de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B2E2;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B2E2: move.w d3, $2a(a6)
pub fn i_09b2e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B2E6;
    cpu.ir = 0x3D43;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B2E6: move.b #$1, $9c(a6)
pub fn i_09b2e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B2EC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09B2EC: move.b #$a, $99(a6)
pub fn i_09b2ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B2F2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09B2F2: jsr $9bb3c(pc)
pub fn i_09b2f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B2F6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9B2F6);
    cpu.pc = 0x9BB3C;
    16
}
/// $09B2F6: move.w d0, $70(a6)
pub fn i_09b2f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B2FA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B2FA: move.b #$8, $45(a6)
pub fn i_09b2fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B300;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09B300: move.w #$8000, d0
pub fn i_09b300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B304;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B304: jsr $28134.l
pub fn i_09b304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B30A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B30A);
    cpu.pc = 0x28134;
    16
}
/// $09B30A: andi.w #$ffe3, $38(a6)
pub fn i_09b30a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B310;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09B310: ori.w #$0, $38(a6)
pub fn i_09b310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B316;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09B316: lea.l $2f7002.l, a0
pub fn i_09b316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B31C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F7002;
    20
}
/// $09B31C: jsr $28cd4.l
pub fn i_09b31c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B322;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B322);
    cpu.pc = 0x28CD4;
    16
}
/// $09B322: lea.l $9b328(pc), a1
pub fn i_09b322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B326;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B328;
    16
}
/// $09B326: move.l a1, (a6)
pub fn i_09b326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B328;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B328: jsr $27d50.l
pub fn i_09b328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B32E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B32E);
    cpu.pc = 0x27D50;
    16
}
/// $09B32E: bcc.w $9b338
pub fn i_09b32e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B332;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9B338; return 10; }
    8
}
/// $09B332: lea.l $9b356(pc), a1
pub fn i_09b332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B336;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B356;
    16
}
/// $09B336: move.l a1, (a6)
pub fn i_09b336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B338;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B338: jsr $28d70.l
pub fn i_09b338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B33E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B33E);
    cpu.pc = 0x28D70;
    16
}
/// $09B33E: subq.w #$1, $70(a6)
pub fn i_09b33e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B342;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09B342: cmpi.w #$0, $70(a6)
pub fn i_09b342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B348;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $09B348: bgt.w $9b352
pub fn i_09b348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B34C;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x9B352; return 10; }
    8
}
/// $09B34C: lea.l $9b0d8(pc), a1
pub fn i_09b34c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B350;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B0D8;
    16
}
/// $09B350: move.l a1, (a6)
pub fn i_09b350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B352;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B352: bra.w $9a75a
pub fn i_09b352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B356;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A75A;
    10
}
// ===== Item_Food_Bounce_09b356  ($09B356, 112 B, props/items_score_crates_09a0bc.s)
/// $09B356: movea.l #$2f7a58, a0
pub fn i_09b356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B35C;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2F7A58;
    cpu.a[0] = v1;
    4
}
/// $09B35C: move.w $5c(a6), d0
pub fn i_09b35c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B360;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09B360: move.w (a0, d0.w), d1
pub fn i_09b360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B364;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $09B364: move.w $2(a0, d0.w), d2
pub fn i_09b364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B368;
    cpu.ir = 0x3430;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $09B368: move.w $4(a0, d0.w), d3
pub fn i_09b368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B36C;
    cpu.ir = 0x3630;
    let ad2 = cpu.a[0].wrapping_add(0x4).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $09B36C: btst.b #$0, $3a(a6)
pub fn i_09b36c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B372;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $09B372: bne.w $9b378
pub fn i_09b372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B376;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9B378; return 10; }
    8
}
/// $09B376: neg.w d1
pub fn i_09b376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B378;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $09B378: move.w d1, $28(a6)
pub fn i_09b378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B37C;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B37C: move.w d2, $2e(a6)
pub fn i_09b37c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B380;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B380: move.w d3, $2a(a6)
pub fn i_09b380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B384;
    cpu.ir = 0x3D43;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B384: lea.l $9b38a(pc), a1
pub fn i_09b384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B388;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B38A;
    16
}
/// $09B388: move.l a1, (a6)
pub fn i_09b388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B38A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B38A: jsr $27d50.l
pub fn i_09b38a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B390;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B390);
    cpu.pc = 0x27D50;
    16
}
/// $09B390: bcc.w $9b39a
pub fn i_09b390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B394;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9B39A; return 10; }
    8
}
/// $09B394: lea.l $9b0a8(pc), a1
pub fn i_09b394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B398;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B0A8;
    16
}
/// $09B398: move.l a1, (a6)
pub fn i_09b398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B39A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B39A: jsr $28d70.l
pub fn i_09b39a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B3A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B3A0);
    cpu.pc = 0x28D70;
    16
}
/// $09B3A0: subq.w #$1, $70(a6)
pub fn i_09b3a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B3A4;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09B3A4: cmpi.w #$0, $70(a6)
pub fn i_09b3a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B3AA;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $09B3AA: bgt.w $9b3b4
pub fn i_09b3aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B3AE;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x9B3B4; return 10; }
    8
}
/// $09B3AE: lea.l $9b0d8(pc), a1
pub fn i_09b3ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B3B2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B0D8;
    16
}
/// $09B3B2: move.l a1, (a6)
pub fn i_09b3b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B3B4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B3B4: jsr $9b920(pc)
pub fn i_09b3b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B3B8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9B3B8);
    cpu.pc = 0x9B920;
    16
}
/// $09B3B8: bcc.w $9b3c2
pub fn i_09b3b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B3BC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9B3C2; return 10; }
    8
}
/// $09B3BC: lea.l $9b474(pc), a1
pub fn i_09b3bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B3C0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B474;
    16
}
/// $09B3C0: move.l a1, (a6)
pub fn i_09b3c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B3C2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B3C2: bra.w $9a75a
pub fn i_09b3c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B3C6;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A75A;
    10
}
// ===== Item_Tmpl293_Pow_09b3c6  ($09B3C6, 120 B, props/items_score_crates_09a0bc.s)
/// $09B3C6: move.w #$1ab, d1
pub fn i_09b3c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B3CA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1AB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B3CA: jsr $236e.l
pub fn i_09b3ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B3D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B3D0);
    cpu.pc = 0x236E;
    16
}
/// $09B3D0: move.l #$9a672, $60(a6)
pub fn i_09b3d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B3D8;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x9A672;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $09B3D8: move.w #$8000, d0
pub fn i_09b3d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B3DC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B3DC: jsr $28134.l
pub fn i_09b3dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B3E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B3E2);
    cpu.pc = 0x28134;
    16
}
/// $09B3E2: andi.w #$ffe3, $38(a6)
pub fn i_09b3e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B3E8;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09B3E8: ori.w #$0, $38(a6)
pub fn i_09b3e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B3EE;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09B3EE: lea.l $2f772a.l, a0
pub fn i_09b3ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B3F4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F772A;
    20
}
/// $09B3F4: jsr $28cd4.l
pub fn i_09b3f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B3FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B3FA);
    cpu.pc = 0x28CD4;
    16
}
/// $09B3FA: lea.l $9b400(pc), a1
pub fn i_09b3fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B3FE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B400;
    16
}
/// $09B3FE: move.l a1, (a6)
pub fn i_09b3fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B400;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B400: jsr $9b8c0(pc)
pub fn i_09b400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B404;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9B404);
    cpu.pc = 0x9B8C0;
    16
}
/// $09B404: jsr $9b8ee(pc)
pub fn i_09b404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B408;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9B408);
    cpu.pc = 0x9B8EE;
    16
}
/// $09B408: jsr $28998.l
pub fn i_09b408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B40E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B40E);
    cpu.pc = 0x28998;
    16
}
/// $09B40E: move.w #$8000, $38(a6)
pub fn i_09b40e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B414;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B414: jsr $28d70.l
pub fn i_09b414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B41A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B41A);
    cpu.pc = 0x28D70;
    16
}
/// $09B41A: jsr $2870a.l
pub fn i_09b41a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B420;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B420);
    cpu.pc = 0x2870A;
    16
}
/// $09B420: bcc.w $9b42a
pub fn i_09b420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B424;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9B42A; return 10; }
    8
}
/// $09B424: bclr.b #$3, $13(a6)
pub fn i_09b424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B42A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $09B42A: jsr $28758.l
pub fn i_09b42a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B430;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B430);
    cpu.pc = 0x28758;
    16
}
/// $09B430: bcc.w $9b43a
pub fn i_09b430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B434;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9B43A; return 10; }
    8
}
/// $09B434: lea.l $9b43e(pc), a1
pub fn i_09b434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B438;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B43E;
    16
}
/// $09B438: move.l a1, (a6)
pub fn i_09b438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B43A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B43A: bra.w $9a76a
pub fn i_09b43a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B43E;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A76A;
    10
}
// ===== Item_Pow_Die_09b43e  ($09B43E, 54 B, props/items_score_crates_09a0bc.s)
/// $09B43E: move.w #$102e, d0
pub fn i_09b43e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B442;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B442: jsr $2352.l
pub fn i_09b442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B448;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B448);
    cpu.pc = 0x2352;
    16
}
/// $09B448: lea.l $2f773c.l, a0
pub fn i_09b448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B44E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F773C;
    20
}
/// $09B44E: jsr $28cd4.l
pub fn i_09b44e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B454;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B454);
    cpu.pc = 0x28CD4;
    16
}
/// $09B454: lea.l $9b45a(pc), a1
pub fn i_09b454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B458;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B45A;
    16
}
/// $09B458: move.l a1, (a6)
pub fn i_09b458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B45A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B45A: jsr $2783a.l
pub fn i_09b45a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B460;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B460);
    cpu.pc = 0x2783A;
    16
}
/// $09B460: jsr $28d70.l
pub fn i_09b460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B466;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B466);
    cpu.pc = 0x28D70;
    16
}
/// $09B466: bcc.w $9b470
pub fn i_09b466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B46A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9B470; return 10; }
    8
}
/// $09B46A: lea.l $9b474(pc), a1
pub fn i_09b46a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B46E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B474;
    16
}
/// $09B46E: move.l a1, (a6)
pub fn i_09b46e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B470;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B470: bra.w $9a76a
pub fn i_09b470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B474;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A76A;
    10
}
// ===== Item_Free_09b474  ($09B474, 6 B, props/items_score_crates_09a0bc.s)
/// $09B474: jmp $518.l
pub fn i_09b474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B47A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Item_Rts_09b47a  ($09B47A, 2 B, props/items_score_crates_09a0bc.s)
/// $09B47A: rts
pub fn i_09b47a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B47C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Jsr5B6ThenJmpScheduler_09b47c  ($09B47C, 14 B, kernel/jsr5b6_jmp518.c)
/// $09B47C: jsr $5b6.l
pub fn i_09b47c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B482;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B482);
    cpu.pc = 0x5B6;
    16
}
/// $09B482: jmp $518.l
pub fn i_09b482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B488;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $09B488: rts
pub fn i_09b488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B48A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Score_Popup_Table_09b48a  ($09B48A, 148 B, props/items_score_crates_09a0bc.s)
/// $09B48A: move.b $68(a6), $6e(a6)
pub fn i_09b48a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B490;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x6E);
    bus.write8(ad4, v3 as u8);
    20
}
/// $09B490: move.b $98(a6), d1
pub fn i_09b490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B494;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09B494: lea.l $2f79a4.l, a1
pub fn i_09b494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B49A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2F79A4;
    20
}
/// $09B49A: andi.w #$3, d1
pub fn i_09b49a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B49E;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09B49E: lsl.w #$2, d1
pub fn i_09b49e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B4A0;
    cpu.ir = 0xE549;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $09B4A0: movea.l (a1, d1.w), a1
pub fn i_09b4a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B4A4;
    cpu.ir = 0x2271;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    18
}
/// $09B4A4: move.b (a1), d1
pub fn i_09b4a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B4A6;
    cpu.ir = 0x1211;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $09B4A6: addq.l #$1, a1
pub fn i_09b4a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B4A8;
    cpu.ir = 0x5289;
    cpu.a[1] = cpu.a[1].wrapping_add(1);
    8
}
/// $09B4A8: move.l a1, $5c(a6)
pub fn i_09b4a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B4AC;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $09B4AC: move.b d1, $72(a6)
pub fn i_09b4ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B4B0;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09B4B0: move.b d1, $21(a6)
pub fn i_09b4b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B4B4;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09B4B4: andi.w #$f, d1
pub fn i_09b4b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B4B8;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09B4B8: move.w #$8, d0
pub fn i_09b4b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B4BC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B4BC: muls.w d1, d0
pub fn i_09b4bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B4BE;
    cpu.ir = 0xC1C1;
    let v1 = cpu.d[1] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[0] as u16 as i16 as i32)) as u32;
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $09B4BE: add.w d0, $22(a6)
pub fn i_09b4be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B4C2;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09B4C2: clr.b $3a(a6)
pub fn i_09b4c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B4C6;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $09B4C6: jsr $5e9b6.l
pub fn i_09b4c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B4CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B4CC);
    cpu.pc = 0x5E9B6;
    16
}
/// $09B4CC: andi.w #$e, d0
pub fn i_09b4cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B4D0;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xE) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09B4D0: addi.w #$9, d0
pub fn i_09b4d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B4D4;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x9, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09B4D4: add.w d0, $24(a6)
pub fn i_09b4d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B4D8;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09B4D8: lea.l $9b66a(pc), a1
pub fn i_09b4d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B4DC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B66A;
    16
}
/// $09B4DC: jsr $4ae.l
pub fn i_09b4dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B4E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B4E2);
    cpu.pc = 0x4AE;
    16
}
/// $09B4E2: jsr $5dd02.l
pub fn i_09b4e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B4E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B4E8);
    cpu.pc = 0x5DD02;
    16
}
/// $09B4E8: move.b $68(a6), $68(a0)
pub fn i_09b4e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B4EE;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x68);
    bus.write8(ad4, v3 as u8);
    20
}
/// $09B4EE: subq.w #$8, $22(a6)
pub fn i_09b4ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B4F2;
    cpu.ir = 0x516E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09B4F2: movea.l $5c(a6), a1
pub fn i_09b4f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B4F6;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $09B4F6: addq.l #$1, $5c(a6)
pub fn i_09b4f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B4FA;
    cpu.ir = 0x52AE;
    let ad1 = cpu.a[6].wrapping_add(0x5C);
    let v2 = bus.read32(ad1);
    let r3 = add_with_flags(cpu, v2, 1, Size::Long, true);
    bus.write32(ad1, r3);
    16
}
/// $09B4FA: move.b (a1), d0
pub fn i_09b4fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B4FC;
    cpu.ir = 0x1011;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $09B4FC: move.w d0, $74(a0)
pub fn i_09b4fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B500;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B500: move.b $72(a6), d0
pub fn i_09b500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B504;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09B504: andi.w #$7, d0
pub fn i_09b504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B508;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09B508: lsl.w #$1, d0
pub fn i_09b508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B50A;
    cpu.ir = 0xE348;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $09B50A: move.w d0, $70(a0)
pub fn i_09b50a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B50E;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B50E: subq.b #$1, $72(a6)
pub fn i_09b50e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B512;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $09B512: cmpi.b #$0, $72(a6)
pub fn i_09b512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B518;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $09B518: bgt.b $9b4d8
pub fn i_09b518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B51A;
    cpu.ir = 0x6EBE;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x9B4D8; return 10; }
    8
}
/// $09B51A: bra.w $9b5e4
pub fn i_09b51a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B51E;
    cpu.ir = 0x6000;
    cpu.pc = 0x9B5E4;
    10
}
// ===== Score_Popup_Value_09b51e  ($09B51E, 324 B, props/items_score_crates_09a0bc.s)
/// $09B51E: move.b $68(a6), $6e(a6)
pub fn i_09b51e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B524;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x6E);
    bus.write8(ad4, v3 as u8);
    20
}
/// $09B524: move.b $98(a6), d1
pub fn i_09b524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B528;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09B528: lea.l $2f792c.l, a1
pub fn i_09b528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B52E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2F792C;
    20
}
/// $09B52E: tst.b $9c(a6)
pub fn i_09b52e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B532;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $09B532: beq.w $9b53c
pub fn i_09b532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B536;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9B53C; return 10; }
    8
}
/// $09B536: lea.l $2f794c.l, a1
pub fn i_09b536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B53C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2F794C;
    20
}
/// $09B53C: andi.w #$f, d1
pub fn i_09b53c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B540;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09B540: lsl.w #$2, d1
pub fn i_09b540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B542;
    cpu.ir = 0xE549;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $09B542: move.l (a1, d1.w), d0
pub fn i_09b542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B546;
    cpu.ir = 0x2031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $09B546: move.l d0, $80(a6)
pub fn i_09b546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B54A;
    cpu.ir = 0x2D40;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $09B54A: jsr $51a44.l
pub fn i_09b54a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B550;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B550);
    cpu.pc = 0x51A44;
    16
}
/// $09B550: move.b $98(a6), d1
pub fn i_09b550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B554;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09B554: lea.l $2f798c.l, a1
pub fn i_09b554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B55A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2F798C;
    20
}
/// $09B55A: tst.b $9c(a6)
pub fn i_09b55a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B55E;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $09B55E: beq.w $9b568
pub fn i_09b55e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B562;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9B568; return 10; }
    8
}
/// $09B562: lea.l $2f7994.l, a1
pub fn i_09b562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B568;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2F7994;
    20
}
/// $09B568: andi.w #$f, d1
pub fn i_09b568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B56C;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09B56C: move.b (a1, d1.w), d1
pub fn i_09b56c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B570;
    cpu.ir = 0x1231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    14
}
/// $09B570: move.b d1, $72(a6)
pub fn i_09b570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B574;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09B574: move.b d1, $21(a6)
pub fn i_09b574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B578;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09B578: andi.w #$f, d1
pub fn i_09b578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B57C;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09B57C: move.w #$8, d0
pub fn i_09b57c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B580;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B580: muls.w d1, d0
pub fn i_09b580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B582;
    cpu.ir = 0xC1C1;
    let v1 = cpu.d[1] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[0] as u16 as i16 as i32)) as u32;
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $09B582: add.w d0, $22(a6)
pub fn i_09b582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B586;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09B586: jsr $5e9b6.l
pub fn i_09b586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B58C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B58C);
    cpu.pc = 0x5E9B6;
    16
}
/// $09B58C: andi.w #$e, d0
pub fn i_09b58c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B590;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xE) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09B590: addi.w #$9, d0
pub fn i_09b590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B594;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x9, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09B594: add.w d0, $24(a6)
pub fn i_09b594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B598;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09B598: clr.b $3a(a6)
pub fn i_09b598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B59C;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $09B59C: lea.l $9b66a(pc), a1
pub fn i_09b59c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5A0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B66A;
    16
}
/// $09B5A0: jsr $4ae.l
pub fn i_09b5a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B5A6);
    cpu.pc = 0x4AE;
    16
}
/// $09B5A6: jsr $5dd02.l
pub fn i_09b5a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B5AC);
    cpu.pc = 0x5DD02;
    16
}
/// $09B5AC: move.b $68(a6), $68(a0)
pub fn i_09b5ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5B2;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x68);
    bus.write8(ad4, v3 as u8);
    20
}
/// $09B5B2: subq.w #$8, $22(a6)
pub fn i_09b5b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5B6;
    cpu.ir = 0x516E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09B5B6: move.l $80(a6), d0
pub fn i_09b5b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5BA;
    cpu.ir = 0x202E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $09B5BA: move.l d0, d1
pub fn i_09b5ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5BC;
    cpu.ir = 0x2200;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[1] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $09B5BC: lsr.l #$4, d1
pub fn i_09b5bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5BE;
    cpu.ir = 0xE889;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Long, 1, false);
    cpu.d[1] = r;
    14
}
/// $09B5BE: move.l d1, $80(a6)
pub fn i_09b5be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5C2;
    cpu.ir = 0x2D41;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $09B5C2: andi.w #$f, d0
pub fn i_09b5c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5C6;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09B5C6: move.w d0, $74(a0)
pub fn i_09b5c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5CA;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B5CA: move.b $72(a6), d0
pub fn i_09b5ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5CE;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09B5CE: andi.w #$7, d0
pub fn i_09b5ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5D2;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09B5D2: lsl.w #$1, d0
pub fn i_09b5d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5D4;
    cpu.ir = 0xE348;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $09B5D4: move.w d0, $70(a0)
pub fn i_09b5d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5D8;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B5D8: subq.b #$1, $72(a6)
pub fn i_09b5d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5DC;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $09B5DC: cmpi.b #$0, $72(a6)
pub fn i_09b5dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5E2;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $09B5E2: bgt.b $9b59c
pub fn i_09b5e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5E4;
    cpu.ir = 0x6EB8;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x9B59C; return 10; }
    8
}
/// $09B5E4: move.w #$1e, $70(a6)
pub fn i_09b5e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5EA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B5EA: move.w #$96, $84(a6)
pub fn i_09b5ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5F0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x96;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B5F0: lea.l $9b5f6(pc), a1
pub fn i_09b5f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5F4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B5F6;
    16
}
/// $09B5F4: move.l a1, (a6)
pub fn i_09b5f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5F6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B5F6: jsr $2783a.l
pub fn i_09b5f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B5FC);
    cpu.pc = 0x2783A;
    16
}
/// $09B5FC: cmpi.b #$ff, $21(a6)
pub fn i_09b5fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B602;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $09B602: beq.w $9b60e
pub fn i_09b602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B606;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9B60E; return 10; }
    8
}
/// $09B606: tst.b $21(a6)
pub fn i_09b606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B60A;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $09B60A: bne.w $9b63a
pub fn i_09b60a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B60E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9B63A; return 10; }
    8
}
/// $09B60E: subq.w #$1, $70(a6)
pub fn i_09b60e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B612;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09B612: cmpi.w #$0, $70(a6)
pub fn i_09b612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B618;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $09B618: bgt.w $9b63a
pub fn i_09b618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B61C;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x9B63A; return 10; }
    8
}
/// $09B61C: tst.b $21(a6)
pub fn i_09b61c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B620;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $09B620: bne.w $9b634
pub fn i_09b620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B624;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9B634; return 10; }
    8
}
/// $09B624: move.b #$ff, $21(a6)
pub fn i_09b624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B62A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09B62A: move.w #$a, $70(a6)
pub fn i_09b62a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B630;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B630: bra.w $9b63a
pub fn i_09b630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B634;
    cpu.ir = 0x6000;
    cpu.pc = 0x9B63A;
    10
}
/// $09B634: lea.l $9b47c(pc), a1
pub fn i_09b634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B638;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B47C;
    16
}
/// $09B638: move.l a1, (a6)
pub fn i_09b638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B63A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B63A: subq.w #$1, $84(a6)
pub fn i_09b63a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B63E;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x84);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09B63E: cmpi.w #$0, $84(a6)
pub fn i_09b63e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B644;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $09B644: bgt.w $9b64e
pub fn i_09b644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B648;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x9B64E; return 10; }
    8
}
/// $09B648: lea.l $9b47c(pc), a1
pub fn i_09b648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B64C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B47C;
    16
}
/// $09B64C: move.l a1, (a6)
pub fn i_09b64c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B64E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B64E: movea.l #$ffffffff, a0
pub fn i_09b64e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B654;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $09B654: lea.l $9a6c6(pc), a0
pub fn i_09b654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B658;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x9A6C6;
    16
}
/// $09B658: jsr $5dd56.l
pub fn i_09b658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B65E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B65E);
    cpu.pc = 0x5DD56;
    16
}
/// $09B65E: bcc.w $9b668
pub fn i_09b65e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B662;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9B668; return 10; }
    8
}
// ===== SetTaskHandler_09b662  ($09B662, 8 B, kernel/task_handlers.c)
/// $09B662: lea.l $9b47c(pc), a1
pub fn i_09b662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B666;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B47C;
    16
}
/// $09B666: move.l a1, (a6)
pub fn i_09b666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B668;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B668: rts
pub fn i_09b668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B66A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Score_Digit_09b66a  ($09B66A, 232 B, props/items_score_crates_09a0bc.s)
/// $09B66A: move.b $68(a6), d0
pub fn i_09b66a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B66E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09B66E: cmpi.b #$0, d0
pub fn i_09b66e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B672;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $09B672: bne.w $9b68e
pub fn i_09b672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B676;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9B68E; return 10; }
    8
}
/// $09B676: move.w #$1b8, d1
pub fn i_09b676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B67A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1B8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B67A: jsr $236e.l
pub fn i_09b67a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B680;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B680);
    cpu.pc = 0x236E;
    16
}
/// $09B680: move.w #$1b9, d1
pub fn i_09b680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B684;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1B9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B684: jsr $236e.l
pub fn i_09b684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B68A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B68A);
    cpu.pc = 0x236E;
    16
}
/// $09B68A: bra.w $9b6a2
pub fn i_09b68a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B68E;
    cpu.ir = 0x6000;
    cpu.pc = 0x9B6A2;
    10
}
/// $09B68E: move.w #$1cd, d1
pub fn i_09b68e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B692;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1CD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B692: jsr $236e.l
pub fn i_09b692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B698;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B698);
    cpu.pc = 0x236E;
    16
}
/// $09B698: move.w #$1ce, d1
pub fn i_09b698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B69C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1CE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B69C: jsr $236e.l
pub fn i_09b69c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B6A2);
    cpu.pc = 0x236E;
    16
}
/// $09B6A2: move.w #$c000, d0
pub fn i_09b6a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6A6;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B6A6: jsr $28134.l
pub fn i_09b6a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B6AC);
    cpu.pc = 0x28134;
    16
}
/// $09B6AC: andi.w #$ffe3, $38(a6)
pub fn i_09b6ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6B2;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09B6B2: ori.w #$1c, $38(a6)
pub fn i_09b6b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6B8;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09B6B8: move.w #$0, d0
pub fn i_09b6b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6BC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B6BC: jsr $5dca4.l
pub fn i_09b6bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B6C2);
    cpu.pc = 0x5DCA4;
    16
}
/// $09B6C2: move.w d0, $28(a6)
pub fn i_09b6c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6C6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B6C6: move.w #$886, $2a(a6)
pub fn i_09b6c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6CC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x886;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B6CC: move.w #$fedd, $2e(a6)
pub fn i_09b6cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6D2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFEDD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B6D2: move.w #$0, $2c(a6)
pub fn i_09b6d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6D8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B6D8: move.w $74(a6), d0
pub fn i_09b6d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6DC;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09B6DC: andi.w #$1f, d0
pub fn i_09b6dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6E0;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09B6E0: movea.l #$2f77a4, a0
pub fn i_09b6e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6E6;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2F77A4;
    cpu.a[0] = v1;
    4
}
/// $09B6E6: lsl.w #$2, d0
pub fn i_09b6e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6E8;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $09B6E8: movea.l (a0, d0.w), a0
pub fn i_09b6e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6EC;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $09B6EC: cmpa.l #$ffffffff, a0
pub fn i_09b6ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6F2;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $09B6F2: beq.w $9b6fc
pub fn i_09b6f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6F6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9B6FC; return 10; }
    8
}
/// $09B6F6: jsr $28cd4.l
pub fn i_09b6f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B6FC);
    cpu.pc = 0x28CD4;
    16
}
/// $09B6FC: lea.l $9b702(pc), a1
pub fn i_09b6fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B700;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B702;
    16
}
/// $09B700: move.l a1, (a6)
pub fn i_09b700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B702;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B702: jsr $2783a.l
pub fn i_09b702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B708;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B708);
    cpu.pc = 0x2783A;
    16
}
/// $09B708: subq.w #$1, $70(a6)
pub fn i_09b708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B70C;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09B70C: cmpi.w #$0, $70(a6)
pub fn i_09b70c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B712;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $09B712: bgt.w $9b748
pub fn i_09b712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B716;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x9B748; return 10; }
    8
}
/// $09B716: jsr $27cee.l
pub fn i_09b716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B71C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B71C);
    cpu.pc = 0x27CEE;
    16
}
/// $09B71C: movea.l $c(a6), a0
pub fn i_09b71c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B720;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $09B720: move.w $24(a0), d0
pub fn i_09b720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B724;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09B724: cmp.w $24(a6), d0
pub fn i_09b724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B728;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $09B728: ble.w $9b73e
pub fn i_09b728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B72C;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x9B73E; return 10; }
    8
}
/// $09B72C: addi.w #$10, d0
pub fn i_09b72c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B730;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09B730: move.w d0, $24(a6)
pub fn i_09b730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B734;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B734: subq.b #$1, $21(a0)
pub fn i_09b734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B738;
    cpu.ir = 0x5328;
    let ad1 = cpu.a[0].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $09B738: lea.l $9b75a(pc), a1
pub fn i_09b738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B73C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B75A;
    16
}
/// $09B73C: move.l a1, (a6)
pub fn i_09b73c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B73E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B73E: jsr $9bb22(pc)
pub fn i_09b73e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B742;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9B742);
    cpu.pc = 0x9BB22;
    16
}
/// $09B742: jsr $28d70.l
pub fn i_09b742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B748;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B748);
    cpu.pc = 0x28D70;
    16
}
/// $09B748: jsr $5e45a.l
pub fn i_09b748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B74E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B74E);
    cpu.pc = 0x5E45A;
    16
}
/// $09B74E: bcc.w $9b758
pub fn i_09b74e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B752;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9B758; return 10; }
    8
}
// ===== SetTaskHandler_09b752  ($09B752, 8 B, kernel/task_handlers.c)
/// $09B752: lea.l $9b47c(pc), a1
pub fn i_09b752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B756;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B47C;
    16
}
/// $09B756: move.l a1, (a6)
pub fn i_09b756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B758;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B758: rts
pub fn i_09b758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B75A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Score_Digit_Lift_09b75a  ($09B75A, 90 B, props/items_score_crates_09a0bc.s)
/// $09B75A: jsr $267e2.l
pub fn i_09b75a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B760;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B760);
    cpu.pc = 0x267E2;
    16
}
/// $09B760: move.w #$400, $2a(a6)
pub fn i_09b760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B766;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B766: move.w #$2, $70(a6)
pub fn i_09b766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B76C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B76C: lea.l $9b772(pc), a1
pub fn i_09b76c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B770;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B772;
    16
}
/// $09B770: move.l a1, (a6)
pub fn i_09b770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B772;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B772: jsr $27cee.l
pub fn i_09b772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B778;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B778);
    cpu.pc = 0x27CEE;
    16
}
/// $09B778: jsr $9bb22(pc)
pub fn i_09b778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B77C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9B77C);
    cpu.pc = 0x9BB22;
    16
}
/// $09B77C: jsr $28d70.l
pub fn i_09b77c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B782;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B782);
    cpu.pc = 0x28D70;
    16
}
/// $09B782: subq.w #$1, $70(a6)
pub fn i_09b782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B786;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09B786: cmpi.w #$0, $70(a6)
pub fn i_09b786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B78C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $09B78C: bgt.w $9b796
pub fn i_09b78c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B790;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x9B796; return 10; }
    8
}
/// $09B790: lea.l $9b7bc(pc), a1
pub fn i_09b790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B794;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B7BC;
    16
}
/// $09B794: move.l a1, (a6)
pub fn i_09b794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B796;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B796: movea.l $c(a6), a0
pub fn i_09b796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B79A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $09B79A: cmpi.b #$ff, $21(a0)
pub fn i_09b79a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7A0;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $09B7A0: bne.w $9b7aa
pub fn i_09b7a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7A4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9B7AA; return 10; }
    8
}
/// $09B7A4: lea.l $9b7f8(pc), a1
pub fn i_09b7a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7A8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B7F8;
    16
}
/// $09B7A8: move.l a1, (a6)
pub fn i_09b7a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7AA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B7AA: jsr $5e45a.l
pub fn i_09b7aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B7B0);
    cpu.pc = 0x5E45A;
    16
}
/// $09B7B0: bcc.w $9b7ba
pub fn i_09b7b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7B4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9B7BA; return 10; }
    8
}
// ===== SetTaskHandler_09b7b4  ($09B7B4, 8 B, kernel/task_handlers.c)
/// $09B7B4: lea.l $9b47c(pc), a1
pub fn i_09b7b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7B8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B47C;
    16
}
/// $09B7B8: move.l a1, (a6)
pub fn i_09b7b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7BA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B7BA: rts
pub fn i_09b7ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7BC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Score_Digit_Hold_09b7bc  ($09B7BC, 52 B, props/items_score_crates_09a0bc.s)
/// $09B7BC: lea.l $9b7c2(pc), a1
pub fn i_09b7bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7C0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B7C2;
    16
}
/// $09B7C0: move.l a1, (a6)
pub fn i_09b7c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7C2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B7C2: jsr $2783a.l
pub fn i_09b7c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B7C8);
    cpu.pc = 0x2783A;
    16
}
/// $09B7C8: jsr $9bb22(pc)
pub fn i_09b7c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7CC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9B7CC);
    cpu.pc = 0x9BB22;
    16
}
/// $09B7CC: jsr $28d70.l
pub fn i_09b7cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B7D2);
    cpu.pc = 0x28D70;
    16
}
/// $09B7D2: movea.l $c(a6), a0
pub fn i_09b7d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7D6;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $09B7D6: cmpi.b #$ff, $21(a0)
pub fn i_09b7d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7DC;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $09B7DC: bne.w $9b7e6
pub fn i_09b7dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7E0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9B7E6; return 10; }
    8
}
/// $09B7E0: lea.l $9b7f8(pc), a1
pub fn i_09b7e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7E4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B7F8;
    16
}
/// $09B7E4: move.l a1, (a6)
pub fn i_09b7e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7E6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B7E6: jsr $5e45a.l
pub fn i_09b7e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B7EC);
    cpu.pc = 0x5E45A;
    16
}
/// $09B7EC: bcc.w $9b7f6
pub fn i_09b7ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7F0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9B7F6; return 10; }
    8
}
// ===== SetTaskHandler_09b7f0  ($09B7F0, 8 B, kernel/task_handlers.c)
/// $09B7F0: lea.l $9b47c(pc), a1
pub fn i_09b7f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7F4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B47C;
    16
}
/// $09B7F4: move.l a1, (a6)
pub fn i_09b7f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7F6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B7F6: rts
pub fn i_09b7f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7F8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Score_Digit_Blink_09b7f8  ($09B7F8, 44 B, props/items_score_crates_09a0bc.s)
/// $09B7F8: lea.l $9b7fe(pc), a1
pub fn i_09b7f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7FC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B7FE;
    16
}
/// $09B7FC: move.l a1, (a6)
pub fn i_09b7fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7FE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B7FE: jsr $2783a.l
pub fn i_09b7fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B804;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B804);
    cpu.pc = 0x2783A;
    16
}
/// $09B804: btst.b #$0, $106f28.l
pub fn i_09b804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B80C;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((0) % 8);
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $09B80C: beq.w $9b81a
pub fn i_09b80c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B810;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9B81A; return 10; }
    8
}
/// $09B810: jsr $9bb22(pc)
pub fn i_09b810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B814;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9B814);
    cpu.pc = 0x9BB22;
    16
}
/// $09B814: jsr $28d70.l
pub fn i_09b814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B81A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B81A);
    cpu.pc = 0x28D70;
    16
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x099CD4, i_099cd4),
    (0x099CD8, i_099cd8),
    (0x099CDC, i_099cdc),
    (0x099CE0, i_099ce0),
    (0x099CE4, i_099ce4),
    (0x099CE8, i_099ce8),
    (0x099CEC, i_099cec),
    (0x099CEE, i_099cee),
    (0x099CF2, i_099cf2),
    (0x099CF6, i_099cf6),
    (0x099CFA, i_099cfa),
    (0x099CFE, i_099cfe),
    (0x099D02, i_099d02),
    (0x099D06, i_099d06),
    (0x099D0C, i_099d0c),
    (0x099D0E, i_099d0e),
    (0x099D12, i_099d12),
    (0x099D1A, i_099d1a),
    (0x099D22, i_099d22),
    (0x099D26, i_099d26),
    (0x099D28, i_099d28),
    (0x099D2E, i_099d2e),
    (0x099D36, i_099d36),
    (0x099D3A, i_099d3a),
    (0x099D42, i_099d42),
    (0x099D48, i_099d48),
    (0x099D4E, i_099d4e),
    (0x099D50, i_099d50),
    (0x099D54, i_099d54),
    (0x099D58, i_099d58),
    (0x099D5E, i_099d5e),
    (0x099D62, i_099d62),
    (0x099D66, i_099d66),
    (0x099D68, i_099d68),
    (0x099D6E, i_099d6e),
    (0x099D72, i_099d72),
    (0x099D76, i_099d76),
    (0x099D78, i_099d78),
    (0x099D80, i_099d80),
    (0x099D84, i_099d84),
    (0x099D8A, i_099d8a),
    (0x099D8E, i_099d8e),
    (0x099D92, i_099d92),
    (0x099D94, i_099d94),
    (0x099D98, i_099d98),
    (0x099D9C, i_099d9c),
    (0x099DA0, i_099da0),
    (0x099DA4, i_099da4),
    (0x099DA8, i_099da8),
    (0x099DAC, i_099dac),
    (0x099DB0, i_099db0),
    (0x099DB4, i_099db4),
    (0x099DB8, i_099db8),
    (0x099DBC, i_099dbc),
    (0x099DBE, i_099dbe),
    (0x099DC2, i_099dc2),
    (0x099DC6, i_099dc6),
    (0x099DCA, i_099dca),
    (0x099DCE, i_099dce),
    (0x099DD2, i_099dd2),
    (0x099DD6, i_099dd6),
    (0x099DDC, i_099ddc),
    (0x099DE0, i_099de0),
    (0x099DE2, i_099de2),
    (0x099DE4, i_099de4),
    (0x099DE8, i_099de8),
    (0x099DEC, i_099dec),
    (0x099DF0, i_099df0),
    (0x099DF6, i_099df6),
    (0x099DFA, i_099dfa),
    (0x099DFC, i_099dfc),
    (0x099E00, i_099e00),
    (0x099E04, i_099e04),
    (0x099E08, i_099e08),
    (0x099E0E, i_099e0e),
    (0x099E12, i_099e12),
    (0x099E14, i_099e14),
    (0x099E18, i_099e18),
    (0x099E1C, i_099e1c),
    (0x099E20, i_099e20),
    (0x099E26, i_099e26),
    (0x099E2A, i_099e2a),
    (0x099E30, i_099e30),
    (0x099E34, i_099e34),
    (0x099E36, i_099e36),
    (0x099E3A, i_099e3a),
    (0x099E3E, i_099e3e),
    (0x099E42, i_099e42),
    (0x099E48, i_099e48),
    (0x099E4C, i_099e4c),
    (0x099E52, i_099e52),
    (0x099E56, i_099e56),
    (0x099E58, i_099e58),
    (0x099E5C, i_099e5c),
    (0x099E60, i_099e60),
    (0x099E64, i_099e64),
    (0x099E6A, i_099e6a),
    (0x099E6E, i_099e6e),
    (0x099E74, i_099e74),
    (0x099E78, i_099e78),
    (0x099E7A, i_099e7a),
    (0x099E7E, i_099e7e),
    (0x099E82, i_099e82),
    (0x099E86, i_099e86),
    (0x099E8C, i_099e8c),
    (0x099E90, i_099e90),
    (0x099E96, i_099e96),
    (0x099E9A, i_099e9a),
    (0x099E9C, i_099e9c),
    (0x099EA4, i_099ea4),
    (0x099EA8, i_099ea8),
    (0x099EAA, i_099eaa),
    (0x099EAE, i_099eae),
    (0x099EB2, i_099eb2),
    (0x099EB8, i_099eb8),
    (0x099EBC, i_099ebc),
    (0x099EBE, i_099ebe),
    (0x099EC2, i_099ec2),
    (0x099EC6, i_099ec6),
    (0x099ECC, i_099ecc),
    (0x099ED0, i_099ed0),
    (0x099ED2, i_099ed2),
    (0x099ED8, i_099ed8),
    (0x099EDC, i_099edc),
    (0x099EE2, i_099ee2),
    (0x099EE4, i_099ee4),
    (0x099EEA, i_099eea),
    (0x099EEE, i_099eee),
    (0x099EF0, i_099ef0),
    (0x099EF4, i_099ef4),
    (0x099EF8, i_099ef8),
    (0x099F00, i_099f00),
    (0x099F04, i_099f04),
    (0x099F06, i_099f06),
    (0x099F0E, i_099f0e),
    (0x099F10, i_099f10),
    (0x099F16, i_099f16),
    (0x099F1A, i_099f1a),
    (0x099F1C, i_099f1c),
    (0x099F20, i_099f20),
    (0x099F24, i_099f24),
    (0x099F2C, i_099f2c),
    (0x099F30, i_099f30),
    (0x099F38, i_099f38),
    (0x099F3A, i_099f3a),
    (0x099F3E, i_099f3e),
    (0x099F42, i_099f42),
    (0x099F44, i_099f44),
    (0x099F48, i_099f48),
    (0x099F4C, i_099f4c),
    (0x099F54, i_099f54),
    (0x099F58, i_099f58),
    (0x099F5A, i_099f5a),
    (0x099F62, i_099f62),
    (0x099F66, i_099f66),
    (0x099F6A, i_099f6a),
    (0x099F72, i_099f72),
    (0x099F76, i_099f76),
    (0x099F78, i_099f78),
    (0x099F80, i_099f80),
    (0x099F84, i_099f84),
    (0x099F86, i_099f86),
    (0x099F8A, i_099f8a),
    (0x099F8E, i_099f8e),
    (0x099F90, i_099f90),
    (0x099F94, i_099f94),
    (0x099F98, i_099f98),
    (0x099FA0, i_099fa0),
    (0x099FA4, i_099fa4),
    (0x099FA6, i_099fa6),
    (0x099FAE, i_099fae),
    (0x099FB2, i_099fb2),
    (0x099FB6, i_099fb6),
    (0x099FBE, i_099fbe),
    (0x099FC2, i_099fc2),
    (0x099FC4, i_099fc4),
    (0x099FCC, i_099fcc),
    (0x099FD0, i_099fd0),
    (0x099FD2, i_099fd2),
    (0x099FD4, i_099fd4),
    (0x099FD8, i_099fd8),
    (0x099FDA, i_099fda),
    (0x099FE0, i_099fe0),
    (0x099FE2, i_099fe2),
    (0x099FE6, i_099fe6),
    (0x099FEA, i_099fea),
    (0x099FF0, i_099ff0),
    (0x099FF2, i_099ff2),
    (0x099FF6, i_099ff6),
    (0x099FF8, i_099ff8),
    (0x099FFC, i_099ffc),
    (0x099FFE, i_099ffe),
    (0x09A002, i_09a002),
    (0x09A00A, i_09a00a),
    (0x09A00E, i_09a00e),
    (0x09A010, i_09a010),
    (0x09A018, i_09a018),
    (0x09A01C, i_09a01c),
    (0x09A020, i_09a020),
    (0x09A028, i_09a028),
    (0x09A02C, i_09a02c),
    (0x09A02E, i_09a02e),
    (0x09A036, i_09a036),
    (0x09A03A, i_09a03a),
    (0x09A03C, i_09a03c),
    (0x09A040, i_09a040),
    (0x09A042, i_09a042),
    (0x09A046, i_09a046),
    (0x09A048, i_09a048),
    (0x09A04C, i_09a04c),
    (0x09A054, i_09a054),
    (0x09A058, i_09a058),
    (0x09A05A, i_09a05a),
    (0x09A062, i_09a062),
    (0x09A066, i_09a066),
    (0x09A06A, i_09a06a),
    (0x09A072, i_09a072),
    (0x09A076, i_09a076),
    (0x09A078, i_09a078),
    (0x09A080, i_09a080),
    (0x09A084, i_09a084),
    (0x09A086, i_09a086),
    (0x09A08E, i_09a08e),
    (0x09A092, i_09a092),
    (0x09A098, i_09a098),
    (0x09A09C, i_09a09c),
    (0x09A0A2, i_09a0a2),
    (0x09A0A6, i_09a0a6),
    (0x09A0AC, i_09a0ac),
    (0x09A0B0, i_09a0b0),
    (0x09A0B4, i_09a0b4),
    (0x09A0BA, i_09a0ba),
    (0x09A0BC, i_09a0bc),
    (0x09A0C0, i_09a0c0),
    (0x09A0C4, i_09a0c4),
    (0x09A0C8, i_09a0c8),
    (0x09A0CC, i_09a0cc),
    (0x09A0D0, i_09a0d0),
    (0x09A0D2, i_09a0d2),
    (0x09A0D6, i_09a0d6),
    (0x09A0E0, i_09a0e0),
    (0x09A0E4, i_09a0e4),
    (0x09A0EA, i_09a0ea),
    (0x09A0EE, i_09a0ee),
    (0x09A0F4, i_09a0f4),
    (0x09A0FA, i_09a0fa),
    (0x09A0FC, i_09a0fc),
    (0x09A100, i_09a100),
    (0x09A102, i_09a102),
    (0x09A104, i_09a104),
    (0x09A108, i_09a108),
    (0x09A10C, i_09a10c),
    (0x09A110, i_09a110),
    (0x09A112, i_09a112),
    (0x09A116, i_09a116),
    (0x09A11A, i_09a11a),
    (0x09A120, i_09a120),
    (0x09A124, i_09a124),
    (0x09A126, i_09a126),
    (0x09A12A, i_09a12a),
    (0x09A12E, i_09a12e),
    (0x09A134, i_09a134),
    (0x09A138, i_09a138),
    (0x09A13E, i_09a13e),
    (0x09A142, i_09a142),
    (0x09A146, i_09a146),
    (0x09A14C, i_09a14c),
    (0x09A150, i_09a150),
    (0x09A154, i_09a154),
    (0x09A156, i_09a156),
    (0x09A15A, i_09a15a),
    (0x09A15E, i_09a15e),
    (0x09A160, i_09a160),
    (0x09A164, i_09a164),
    (0x09A168, i_09a168),
    (0x09A16A, i_09a16a),
    (0x09A16E, i_09a16e),
    (0x09A170, i_09a170),
    (0x09A172, i_09a172),
    (0x09A174, i_09a174),
    (0x09A17A, i_09a17a),
    (0x09A180, i_09a180),
    (0x09A186, i_09a186),
    (0x09A18A, i_09a18a),
    (0x09A18E, i_09a18e),
    (0x09A192, i_09a192),
    (0x09A196, i_09a196),
    (0x09A198, i_09a198),
    (0x09A19A, i_09a19a),
    (0x09A1A0, i_09a1a0),
    (0x09A1A4, i_09a1a4),
    (0x09A1A8, i_09a1a8),
    (0x09A1AC, i_09a1ac),
    (0x09A1B0, i_09a1b0),
    (0x09A1B6, i_09a1b6),
    (0x09A1BA, i_09a1ba),
    (0x09A1BE, i_09a1be),
    (0x09A1C0, i_09a1c0),
    (0x09A1C6, i_09a1c6),
    (0x09A1CA, i_09a1ca),
    (0x09A1CE, i_09a1ce),
    (0x09A1D2, i_09a1d2),
    (0x09A1D6, i_09a1d6),
    (0x09A1D8, i_09a1d8),
    (0x09A1DC, i_09a1dc),
    (0x09A1E0, i_09a1e0),
    (0x09A1E6, i_09a1e6),
    (0x09A1EA, i_09a1ea),
    (0x09A1EE, i_09a1ee),
    (0x09A1F2, i_09a1f2),
    (0x09A1F6, i_09a1f6),
    (0x09A1FA, i_09a1fa),
    (0x09A1FE, i_09a1fe),
    (0x09A202, i_09a202),
    (0x09A206, i_09a206),
    (0x09A20C, i_09a20c),
    (0x09A20E, i_09a20e),
    (0x09A214, i_09a214),
    (0x09A216, i_09a216),
    (0x09A21C, i_09a21c),
    (0x09A220, i_09a220),
    (0x09A226, i_09a226),
    (0x09A22A, i_09a22a),
    (0x09A230, i_09a230),
    (0x09A232, i_09a232),
    (0x09A238, i_09a238),
    (0x09A23C, i_09a23c),
    (0x09A240, i_09a240),
    (0x09A244, i_09a244),
    (0x09A246, i_09a246),
    (0x09A24C, i_09a24c),
    (0x09A24E, i_09a24e),
    (0x09A254, i_09a254),
    (0x09A258, i_09a258),
    (0x09A25E, i_09a25e),
    (0x09A264, i_09a264),
    (0x09A26A, i_09a26a),
    (0x09A26C, i_09a26c),
    (0x09A272, i_09a272),
    (0x09A276, i_09a276),
    (0x09A27A, i_09a27a),
    (0x09A27E, i_09a27e),
    (0x09A280, i_09a280),
    (0x09A284, i_09a284),
    (0x09A288, i_09a288),
    (0x09A28C, i_09a28c),
    (0x09A292, i_09a292),
    (0x09A296, i_09a296),
    (0x09A29A, i_09a29a),
    (0x09A29E, i_09a29e),
    (0x09A2A0, i_09a2a0),
    (0x09A2A6, i_09a2a6),
    (0x09A2AA, i_09a2aa),
    (0x09A2B0, i_09a2b0),
    (0x09A2B4, i_09a2b4),
    (0x09A2B6, i_09a2b6),
    (0x09A2B8, i_09a2b8),
    (0x09A2BC, i_09a2bc),
    (0x09A2C2, i_09a2c2),
    (0x09A2C6, i_09a2c6),
    (0x09A2CA, i_09a2ca),
    (0x09A2CE, i_09a2ce),
    (0x09A2D2, i_09a2d2),
    (0x09A2D4, i_09a2d4),
    (0x09A2D6, i_09a2d6),
    (0x09A2DC, i_09a2dc),
    (0x09A2E2, i_09a2e2),
    (0x09A2E8, i_09a2e8),
    (0x09A2EE, i_09a2ee),
    (0x09A2F4, i_09a2f4),
    (0x09A2F8, i_09a2f8),
    (0x09A2FC, i_09a2fc),
    (0x09A2FE, i_09a2fe),
    (0x09A300, i_09a300),
    (0x09A306, i_09a306),
    (0x09A30A, i_09a30a),
    (0x09A310, i_09a310),
    (0x09A316, i_09a316),
    (0x09A31A, i_09a31a),
    (0x09A31E, i_09a31e),
    (0x09A322, i_09a322),
    (0x09A326, i_09a326),
    (0x09A32C, i_09a32c),
    (0x09A330, i_09a330),
    (0x09A336, i_09a336),
    (0x09A33C, i_09a33c),
    (0x09A342, i_09a342),
    (0x09A348, i_09a348),
    (0x09A34A, i_09a34a),
    (0x09A34E, i_09a34e),
    (0x09A352, i_09a352),
    (0x09A356, i_09a356),
    (0x09A35C, i_09a35c),
    (0x09A362, i_09a362),
    (0x09A368, i_09a368),
    (0x09A36E, i_09a36e),
    (0x09A372, i_09a372),
    (0x09A374, i_09a374),
    (0x09A378, i_09a378),
    (0x09A37C, i_09a37c),
    (0x09A382, i_09a382),
    (0x09A38A, i_09a38a),
    (0x09A38E, i_09a38e),
    (0x09A390, i_09a390),
    (0x09A394, i_09a394),
    (0x09A398, i_09a398),
    (0x09A39C, i_09a39c),
    (0x09A39E, i_09a39e),
    (0x09A3A2, i_09a3a2),
    (0x09A3A6, i_09a3a6),
    (0x09A3AA, i_09a3aa),
    (0x09A3AC, i_09a3ac),
    (0x09A3B0, i_09a3b0),
    (0x09A3B4, i_09a3b4),
    (0x09A3B6, i_09a3b6),
    (0x09A3BC, i_09a3bc),
    (0x09A3C0, i_09a3c0),
    (0x09A3C6, i_09a3c6),
    (0x09A3CA, i_09a3ca),
    (0x09A3D0, i_09a3d0),
    (0x09A3D4, i_09a3d4),
    (0x09A3D6, i_09a3d6),
    (0x09A3DA, i_09a3da),
    (0x09A3DC, i_09a3dc),
    (0x09A3E0, i_09a3e0),
    (0x09A3E4, i_09a3e4),
    (0x09A3E8, i_09a3e8),
    (0x09A3EC, i_09a3ec),
    (0x09A3F0, i_09a3f0),
    (0x09A3F4, i_09a3f4),
    (0x09A3F8, i_09a3f8),
    (0x09A3FC, i_09a3fc),
    (0x09A400, i_09a400),
    (0x09A404, i_09a404),
    (0x09A40A, i_09a40a),
    (0x09A410, i_09a410),
    (0x09A416, i_09a416),
    (0x09A41A, i_09a41a),
    (0x09A41E, i_09a41e),
    (0x09A422, i_09a422),
    (0x09A426, i_09a426),
    (0x09A42C, i_09a42c),
    (0x09A42E, i_09a42e),
    (0x09A432, i_09a432),
    (0x09A436, i_09a436),
    (0x09A43A, i_09a43a),
    (0x09A440, i_09a440),
    (0x09A444, i_09a444),
    (0x09A44A, i_09a44a),
    (0x09A44E, i_09a44e),
    (0x09A454, i_09a454),
    (0x09A458, i_09a458),
    (0x09A45C, i_09a45c),
    (0x09A460, i_09a460),
    (0x09A464, i_09a464),
    (0x09A468, i_09a468),
    (0x09A46C, i_09a46c),
    (0x09A470, i_09a470),
    (0x09A472, i_09a472),
    (0x09A476, i_09a476),
    (0x09A478, i_09a478),
    (0x09A47A, i_09a47a),
    (0x09A47C, i_09a47c),
    (0x09A47E, i_09a47e),
    (0x09A480, i_09a480),
    (0x09A482, i_09a482),
    (0x09A486, i_09a486),
    (0x09A48A, i_09a48a),
    (0x09A48C, i_09a48c),
    (0x09A490, i_09a490),
    (0x09A492, i_09a492),
    (0x09A496, i_09a496),
    (0x09A498, i_09a498),
    (0x09A49A, i_09a49a),
    (0x09A49E, i_09a49e),
    (0x09A4A2, i_09a4a2),
    (0x09A4A6, i_09a4a6),
    (0x09A4AC, i_09a4ac),
    (0x09A4B0, i_09a4b0),
    (0x09A4B6, i_09a4b6),
    (0x09A4BC, i_09a4bc),
    (0x09A4C0, i_09a4c0),
    (0x09A4C4, i_09a4c4),
    (0x09A4C8, i_09a4c8),
    (0x09A4CC, i_09a4cc),
    (0x09A4D2, i_09a4d2),
    (0x09A4D4, i_09a4d4),
    (0x09A4D8, i_09a4d8),
    (0x09A4DC, i_09a4dc),
    (0x09A4E0, i_09a4e0),
    (0x09A4E4, i_09a4e4),
    (0x09A4EA, i_09a4ea),
    (0x09A4EE, i_09a4ee),
    (0x09A4F2, i_09a4f2),
    (0x09A4F6, i_09a4f6),
    (0x09A4FA, i_09a4fa),
    (0x09A4FC, i_09a4fc),
    (0x09A500, i_09a500),
    (0x09A504, i_09a504),
    (0x09A506, i_09a506),
    (0x09A50A, i_09a50a),
    (0x09A50E, i_09a50e),
    (0x09A514, i_09a514),
    (0x09A51A, i_09a51a),
    (0x09A520, i_09a520),
    (0x09A524, i_09a524),
    (0x09A52A, i_09a52a),
    (0x09A530, i_09a530),
    (0x09A536, i_09a536),
    (0x09A53A, i_09a53a),
    (0x09A540, i_09a540),
    (0x09A546, i_09a546),
    (0x09A54C, i_09a54c),
    (0x09A552, i_09a552),
    (0x09A554, i_09a554),
    (0x09A558, i_09a558),
    (0x09A55A, i_09a55a),
    (0x09A55E, i_09a55e),
    (0x09A564, i_09a564),
    (0x09A568, i_09a568),
    (0x09A56C, i_09a56c),
    (0x09A570, i_09a570),
    (0x09A574, i_09a574),
    (0x09A576, i_09a576),
    (0x09A57C, i_09a57c),
    (0x09A57E, i_09a57e),
    (0x09A582, i_09a582),
    (0x09A588, i_09a588),
    (0x09A58C, i_09a58c),
    (0x09A592, i_09a592),
    (0x09A596, i_09a596),
    (0x09A598, i_09a598),
    (0x09A59E, i_09a59e),
    (0x09A5A4, i_09a5a4),
    (0x09A5AA, i_09a5aa),
    (0x09A5B0, i_09a5b0),
    (0x09A5B6, i_09a5b6),
    (0x09A5BA, i_09a5ba),
    (0x09A5C0, i_09a5c0),
    (0x09A5C6, i_09a5c6),
    (0x09A5CC, i_09a5cc),
    (0x09A5D0, i_09a5d0),
    (0x09A5D6, i_09a5d6),
    (0x09A5D8, i_09a5d8),
    (0x09A5DC, i_09a5dc),
    (0x09A5E0, i_09a5e0),
    (0x09A5E4, i_09a5e4),
    (0x09A5E8, i_09a5e8),
    (0x09A5EC, i_09a5ec),
    (0x09A5EE, i_09a5ee),
    (0x09A5F2, i_09a5f2),
    (0x09A75A, i_09a75a),
    (0x09A760, i_09a760),
    (0x09A764, i_09a764),
    (0x09A76A, i_09a76a),
    (0x09A76E, i_09a76e),
    (0x09A772, i_09a772),
    (0x09A778, i_09a778),
    (0x09A77C, i_09a77c),
    (0x09A782, i_09a782),
    (0x09A786, i_09a786),
    (0x09A78C, i_09a78c),
    (0x09A78E, i_09a78e),
    (0x09A794, i_09a794),
    (0x09A798, i_09a798),
    (0x09A79E, i_09a79e),
    (0x09A7A2, i_09a7a2),
    (0x09A7A8, i_09a7a8),
    (0x09A7AA, i_09a7aa),
    (0x09A7AE, i_09a7ae),
    (0x09A7B2, i_09a7b2),
    (0x09A7B6, i_09a7b6),
    (0x09A7BC, i_09a7bc),
    (0x09A7C0, i_09a7c0),
    (0x09A7C2, i_09a7c2),
    (0x09A7C6, i_09a7c6),
    (0x09A7CA, i_09a7ca),
    (0x09A7CC, i_09a7cc),
    (0x09A7CE, i_09a7ce),
    (0x09A7D2, i_09a7d2),
    (0x09A7D4, i_09a7d4),
    (0x09A7D8, i_09a7d8),
    (0x09A7DC, i_09a7dc),
    (0x09A7E0, i_09a7e0),
    (0x09A7E4, i_09a7e4),
    (0x09A7E6, i_09a7e6),
    (0x09A7E8, i_09a7e8),
    (0x09A7EC, i_09a7ec),
    (0x09A7F0, i_09a7f0),
    (0x09A7F2, i_09a7f2),
    (0x09A7F8, i_09a7f8),
    (0x09A7FE, i_09a7fe),
    (0x09A800, i_09a800),
    (0x09A806, i_09a806),
    (0x09A80A, i_09a80a),
    (0x09A810, i_09a810),
    (0x09A814, i_09a814),
    (0x09A816, i_09a816),
    (0x09A81A, i_09a81a),
    (0x09A81E, i_09a81e),
    (0x09A820, i_09a820),
    (0x09A826, i_09a826),
    (0x09A82C, i_09a82c),
    (0x09A830, i_09a830),
    (0x09A834, i_09a834),
    (0x09A836, i_09a836),
    (0x09A83A, i_09a83a),
    (0x09A83E, i_09a83e),
    (0x09A842, i_09a842),
    (0x09A846, i_09a846),
    (0x09A848, i_09a848),
    (0x09A84E, i_09a84e),
    (0x09A852, i_09a852),
    (0x09A854, i_09a854),
    (0x09A858, i_09a858),
    (0x09A85E, i_09a85e),
    (0x09A864, i_09a864),
    (0x09A868, i_09a868),
    (0x09A86A, i_09a86a),
    (0x09A86E, i_09a86e),
    (0x09A874, i_09a874),
    (0x09A876, i_09a876),
    (0x09A87C, i_09a87c),
    (0x09A87E, i_09a87e),
    (0x09A882, i_09a882),
    (0x09A886, i_09a886),
    (0x09A88A, i_09a88a),
    (0x09A88E, i_09a88e),
    (0x09A892, i_09a892),
    (0x09A896, i_09a896),
    (0x09A89A, i_09a89a),
    (0x09A89C, i_09a89c),
    (0x09A8A0, i_09a8a0),
    (0x09A8A6, i_09a8a6),
    (0x09A8A8, i_09a8a8),
    (0x09A8AC, i_09a8ac),
    (0x09A8B2, i_09a8b2),
    (0x09A8B6, i_09a8b6),
    (0x09A8BC, i_09a8bc),
    (0x09A8BE, i_09a8be),
    (0x09A8C4, i_09a8c4),
    (0x09A8CC, i_09a8cc),
    (0x09A8D0, i_09a8d0),
    (0x09A8D6, i_09a8d6),
    (0x09A8DC, i_09a8dc),
    (0x09A8DE, i_09a8de),
    (0x09A8E2, i_09a8e2),
    (0x09A8E8, i_09a8e8),
    (0x09A8EE, i_09a8ee),
    (0x09A8F4, i_09a8f4),
    (0x09A8F8, i_09a8f8),
    (0x09A8FA, i_09a8fa),
    (0x09A900, i_09a900),
    (0x09A906, i_09a906),
    (0x09A90A, i_09a90a),
    (0x09A910, i_09a910),
    (0x09A914, i_09a914),
    (0x09A918, i_09a918),
    (0x09A91C, i_09a91c),
    (0x09A922, i_09a922),
    (0x09A928, i_09a928),
    (0x09A92C, i_09a92c),
    (0x09A932, i_09a932),
    (0x09A938, i_09a938),
    (0x09A93E, i_09a93e),
    (0x09A944, i_09a944),
    (0x09A94A, i_09a94a),
    (0x09A94E, i_09a94e),
    (0x09A950, i_09a950),
    (0x09A956, i_09a956),
    (0x09A95C, i_09a95c),
    (0x09A960, i_09a960),
    (0x09A964, i_09a964),
    (0x09A968, i_09a968),
    (0x09A96C, i_09a96c),
    (0x09A970, i_09a970),
    (0x09A972, i_09a972),
    (0x09A976, i_09a976),
    (0x09A97A, i_09a97a),
    (0x09A980, i_09a980),
    (0x09A986, i_09a986),
    (0x09A98A, i_09a98a),
    (0x09A990, i_09a990),
    (0x09A996, i_09a996),
    (0x09A99C, i_09a99c),
    (0x09A9A2, i_09a9a2),
    (0x09A9A8, i_09a9a8),
    (0x09A9AC, i_09a9ac),
    (0x09A9AE, i_09a9ae),
    (0x09A9B4, i_09a9b4),
    (0x09A9BA, i_09a9ba),
    (0x09A9BE, i_09a9be),
    (0x09A9C2, i_09a9c2),
    (0x09A9C6, i_09a9c6),
    (0x09A9CA, i_09a9ca),
    (0x09A9CE, i_09a9ce),
    (0x09A9D0, i_09a9d0),
    (0x09A9D4, i_09a9d4),
    (0x09A9D8, i_09a9d8),
    (0x09A9DE, i_09a9de),
    (0x09A9E4, i_09a9e4),
    (0x09A9E8, i_09a9e8),
    (0x09A9EE, i_09a9ee),
    (0x09A9F4, i_09a9f4),
    (0x09A9FA, i_09a9fa),
    (0x09AA00, i_09aa00),
    (0x09AA06, i_09aa06),
    (0x09AA0A, i_09aa0a),
    (0x09AA0C, i_09aa0c),
    (0x09AA12, i_09aa12),
    (0x09AA18, i_09aa18),
    (0x09AA1C, i_09aa1c),
    (0x09AA20, i_09aa20),
    (0x09AA24, i_09aa24),
    (0x09AA28, i_09aa28),
    (0x09AA2C, i_09aa2c),
    (0x09AA2E, i_09aa2e),
    (0x09AA32, i_09aa32),
    (0x09AA36, i_09aa36),
    (0x09AA3C, i_09aa3c),
    (0x09AA42, i_09aa42),
    (0x09AA46, i_09aa46),
    (0x09AA4C, i_09aa4c),
    (0x09AA52, i_09aa52),
    (0x09AA58, i_09aa58),
    (0x09AA5E, i_09aa5e),
    (0x09AA64, i_09aa64),
    (0x09AA68, i_09aa68),
    (0x09AA6A, i_09aa6a),
    (0x09AA70, i_09aa70),
    (0x09AA76, i_09aa76),
    (0x09AA7A, i_09aa7a),
    (0x09AA7E, i_09aa7e),
    (0x09AA82, i_09aa82),
    (0x09AA86, i_09aa86),
    (0x09AA8A, i_09aa8a),
    (0x09AA8C, i_09aa8c),
    (0x09AA90, i_09aa90),
    (0x09AA94, i_09aa94),
    (0x09AA9A, i_09aa9a),
    (0x09AAA0, i_09aaa0),
    (0x09AAA4, i_09aaa4),
    (0x09AAA8, i_09aaa8),
    (0x09AAAC, i_09aaac),
    (0x09AAB0, i_09aab0),
    (0x09AAB4, i_09aab4),
    (0x09AAB6, i_09aab6),
    (0x09AABA, i_09aaba),
    (0x09AABE, i_09aabe),
    (0x09AAC2, i_09aac2),
    (0x09AAC6, i_09aac6),
    (0x09AACC, i_09aacc),
    (0x09AAD0, i_09aad0),
    (0x09AAD6, i_09aad6),
    (0x09AADC, i_09aadc),
    (0x09AAE2, i_09aae2),
    (0x09AAE8, i_09aae8),
    (0x09AAEE, i_09aaee),
    (0x09AAF2, i_09aaf2),
    (0x09AAF4, i_09aaf4),
    (0x09AAF8, i_09aaf8),
    (0x09AAFE, i_09aafe),
    (0x09AB02, i_09ab02),
    (0x09AB06, i_09ab06),
    (0x09AB08, i_09ab08),
    (0x09AB0E, i_09ab0e),
    (0x09AB14, i_09ab14),
    (0x09AB18, i_09ab18),
    (0x09AB1C, i_09ab1c),
    (0x09AB20, i_09ab20),
    (0x09AB24, i_09ab24),
    (0x09AB28, i_09ab28),
    (0x09AB2C, i_09ab2c),
    (0x09AB32, i_09ab32),
    (0x09AB36, i_09ab36),
    (0x09AB3C, i_09ab3c),
    (0x09AB40, i_09ab40),
    (0x09AB44, i_09ab44),
    (0x09AB4A, i_09ab4a),
    (0x09AB4E, i_09ab4e),
    (0x09AB50, i_09ab50),
    (0x09AB5C, i_09ab5c),
    (0x09AB60, i_09ab60),
    (0x09AB64, i_09ab64),
    (0x09AB66, i_09ab66),
    (0x09AB6A, i_09ab6a),
    (0x09AB6E, i_09ab6e),
    (0x09AB72, i_09ab72),
    (0x09AB76, i_09ab76),
    (0x09AB7A, i_09ab7a),
    (0x09AB80, i_09ab80),
    (0x09AB84, i_09ab84),
    (0x09AB88, i_09ab88),
    (0x09AB8A, i_09ab8a),
    (0x09AB8E, i_09ab8e),
    (0x09AB92, i_09ab92),
    (0x09AB96, i_09ab96),
    (0x09AB9A, i_09ab9a),
    (0x09AB9C, i_09ab9c),
    (0x09ABA0, i_09aba0),
    (0x09ABA4, i_09aba4),
    (0x09ABA8, i_09aba8),
    (0x09ABAC, i_09abac),
    (0x09ABB0, i_09abb0),
    (0x09ABB4, i_09abb4),
    (0x09ABB8, i_09abb8),
    (0x09ABBC, i_09abbc),
    (0x09ABBE, i_09abbe),
    (0x09ABC2, i_09abc2),
    (0x09ABC6, i_09abc6),
    (0x09ABCC, i_09abcc),
    (0x09ABD0, i_09abd0),
    (0x09ABD6, i_09abd6),
    (0x09ABDC, i_09abdc),
    (0x09ABE0, i_09abe0),
    (0x09ABE6, i_09abe6),
    (0x09ABEC, i_09abec),
    (0x09ABF2, i_09abf2),
    (0x09ABF8, i_09abf8),
    (0x09ABFE, i_09abfe),
    (0x09AC02, i_09ac02),
    (0x09AC06, i_09ac06),
    (0x09AC0C, i_09ac0c),
    (0x09AC10, i_09ac10),
    (0x09AC14, i_09ac14),
    (0x09AC1A, i_09ac1a),
    (0x09AC1E, i_09ac1e),
    (0x09AC20, i_09ac20),
    (0x09AC26, i_09ac26),
    (0x09AC2C, i_09ac2c),
    (0x09AC30, i_09ac30),
    (0x09AC34, i_09ac34),
    (0x09AC38, i_09ac38),
    (0x09AC3E, i_09ac3e),
    (0x09AC42, i_09ac42),
    (0x09AC44, i_09ac44),
    (0x09AC48, i_09ac48),
    (0x09AC4E, i_09ac4e),
    (0x09AC52, i_09ac52),
    (0x09AC56, i_09ac56),
    (0x09AC5C, i_09ac5c),
    (0x09AC60, i_09ac60),
    (0x09AC66, i_09ac66),
    (0x09AC6C, i_09ac6c),
    (0x09AC70, i_09ac70),
    (0x09AC76, i_09ac76),
    (0x09AC7C, i_09ac7c),
    (0x09AC82, i_09ac82),
    (0x09AC88, i_09ac88),
    (0x09AC8E, i_09ac8e),
    (0x09AC92, i_09ac92),
    (0x09AC96, i_09ac96),
    (0x09AC9C, i_09ac9c),
    (0x09ACA0, i_09aca0),
    (0x09ACA4, i_09aca4),
    (0x09ACAA, i_09acaa),
    (0x09ACAE, i_09acae),
    (0x09ACB0, i_09acb0),
    (0x09ACB6, i_09acb6),
    (0x09ACBC, i_09acbc),
    (0x09ACC0, i_09acc0),
    (0x09ACC4, i_09acc4),
    (0x09ACC8, i_09acc8),
    (0x09ACCC, i_09accc),
    (0x09ACD2, i_09acd2),
    (0x09ACD6, i_09acd6),
    (0x09ACDC, i_09acdc),
    (0x09ACE2, i_09ace2),
    (0x09ACE6, i_09ace6),
    (0x09ACEA, i_09acea),
    (0x09ACF0, i_09acf0),
    (0x09ACF6, i_09acf6),
    (0x09ACFA, i_09acfa),
    (0x09AD00, i_09ad00),
    (0x09AD06, i_09ad06),
    (0x09AD0C, i_09ad0c),
    (0x09AD12, i_09ad12),
    (0x09AD16, i_09ad16),
    (0x09AD1A, i_09ad1a),
    (0x09AD20, i_09ad20),
    (0x09AD26, i_09ad26),
    (0x09AD2A, i_09ad2a),
    (0x09AD30, i_09ad30),
    (0x09AD36, i_09ad36),
    (0x09AD3C, i_09ad3c),
    (0x09AD40, i_09ad40),
    (0x09AD44, i_09ad44),
    (0x09AD46, i_09ad46),
    (0x09AD4A, i_09ad4a),
    (0x09AD50, i_09ad50),
    (0x09AD54, i_09ad54),
    (0x09AD58, i_09ad58),
    (0x09AD5E, i_09ad5e),
    (0x09AD62, i_09ad62),
    (0x09AD68, i_09ad68),
    (0x09AD6E, i_09ad6e),
    (0x09AD72, i_09ad72),
    (0x09AD78, i_09ad78),
    (0x09AD7E, i_09ad7e),
    (0x09AD84, i_09ad84),
    (0x09AD8A, i_09ad8a),
    (0x09AD90, i_09ad90),
    (0x09AD94, i_09ad94),
    (0x09AD96, i_09ad96),
    (0x09AD9C, i_09ad9c),
    (0x09ADA2, i_09ada2),
    (0x09ADA6, i_09ada6),
    (0x09ADAA, i_09adaa),
    (0x09ADAE, i_09adae),
    (0x09ADB2, i_09adb2),
    (0x09ADB8, i_09adb8),
    (0x09ADBC, i_09adbc),
    (0x09ADC2, i_09adc2),
    (0x09ADC6, i_09adc6),
    (0x09ADCC, i_09adcc),
    (0x09ADD0, i_09add0),
    (0x09ADD4, i_09add4),
    (0x09ADD8, i_09add8),
    (0x09ADDC, i_09addc),
    (0x09ADE2, i_09ade2),
    (0x09ADE6, i_09ade6),
    (0x09ADEA, i_09adea),
    (0x09ADEE, i_09adee),
    (0x09ADF2, i_09adf2),
    (0x09ADF8, i_09adf8),
    (0x09ADFE, i_09adfe),
    (0x09AE04, i_09ae04),
    (0x09AE0A, i_09ae0a),
    (0x09AE0E, i_09ae0e),
    (0x09AE12, i_09ae12),
    (0x09AE16, i_09ae16),
    (0x09AE18, i_09ae18),
    (0x09AE1C, i_09ae1c),
    (0x09AE22, i_09ae22),
    (0x09AE26, i_09ae26),
    (0x09AE2A, i_09ae2a),
    (0x09AE2E, i_09ae2e),
    (0x09AE32, i_09ae32),
    (0x09AE36, i_09ae36),
    (0x09AE3A, i_09ae3a),
    (0x09AE3E, i_09ae3e),
    (0x09AE42, i_09ae42),
    (0x09AE48, i_09ae48),
    (0x09AE4E, i_09ae4e),
    (0x09AE52, i_09ae52),
    (0x09AE58, i_09ae58),
    (0x09AE5E, i_09ae5e),
    (0x09AE64, i_09ae64),
    (0x09AE68, i_09ae68),
    (0x09AE6A, i_09ae6a),
    (0x09AE70, i_09ae70),
    (0x09AE76, i_09ae76),
    (0x09AE7A, i_09ae7a),
    (0x09AE80, i_09ae80),
    (0x09AE84, i_09ae84),
    (0x09AE88, i_09ae88),
    (0x09AE8A, i_09ae8a),
    (0x09AE8E, i_09ae8e),
    (0x09AE92, i_09ae92),
    (0x09AE98, i_09ae98),
    (0x09AE9E, i_09ae9e),
    (0x09AEA4, i_09aea4),
    (0x09AEA8, i_09aea8),
    (0x09AEAA, i_09aeaa),
    (0x09AEB0, i_09aeb0),
    (0x09AEB6, i_09aeb6),
    (0x09AEBA, i_09aeba),
    (0x09AEC0, i_09aec0),
    (0x09AEC4, i_09aec4),
    (0x09AEC8, i_09aec8),
    (0x09AECE, i_09aece),
    (0x09AED4, i_09aed4),
    (0x09AED8, i_09aed8),
    (0x09AEDC, i_09aedc),
    (0x09AEE2, i_09aee2),
    (0x09AEE8, i_09aee8),
    (0x09AEEC, i_09aeec),
    (0x09AEF2, i_09aef2),
    (0x09AEF8, i_09aef8),
    (0x09AEFE, i_09aefe),
    (0x09AF02, i_09af02),
    (0x09AF06, i_09af06),
    (0x09AF0C, i_09af0c),
    (0x09AF12, i_09af12),
    (0x09AF16, i_09af16),
    (0x09AF1A, i_09af1a),
    (0x09AF20, i_09af20),
    (0x09AF26, i_09af26),
    (0x09AF2A, i_09af2a),
    (0x09AF2E, i_09af2e),
    (0x09AF34, i_09af34),
    (0x09AF3A, i_09af3a),
    (0x09AF3E, i_09af3e),
    (0x09AF42, i_09af42),
    (0x09AF48, i_09af48),
    (0x09AF4E, i_09af4e),
    (0x09AF52, i_09af52),
    (0x09AF58, i_09af58),
    (0x09AF5E, i_09af5e),
    (0x09AF64, i_09af64),
    (0x09AF68, i_09af68),
    (0x09AF6C, i_09af6c),
    (0x09AF72, i_09af72),
    (0x09AF78, i_09af78),
    (0x09AF7C, i_09af7c),
    (0x09AF80, i_09af80),
    (0x09AF86, i_09af86),
    (0x09AF8C, i_09af8c),
    (0x09AF90, i_09af90),
    (0x09AF94, i_09af94),
    (0x09AF9A, i_09af9a),
    (0x09AFA0, i_09afa0),
    (0x09AFA4, i_09afa4),
    (0x09AFA8, i_09afa8),
    (0x09AFAE, i_09afae),
    (0x09AFB4, i_09afb4),
    (0x09AFB8, i_09afb8),
    (0x09AFBC, i_09afbc),
    (0x09AFC2, i_09afc2),
    (0x09AFC8, i_09afc8),
    (0x09AFCC, i_09afcc),
    (0x09AFD0, i_09afd0),
    (0x09AFD6, i_09afd6),
    (0x09AFDC, i_09afdc),
    (0x09AFE0, i_09afe0),
    (0x09AFE4, i_09afe4),
    (0x09AFEA, i_09afea),
    (0x09AFF0, i_09aff0),
    (0x09AFF4, i_09aff4),
    (0x09AFF8, i_09aff8),
    (0x09AFFE, i_09affe),
    (0x09B004, i_09b004),
    (0x09B008, i_09b008),
    (0x09B00E, i_09b00e),
    (0x09B014, i_09b014),
    (0x09B01A, i_09b01a),
    (0x09B01E, i_09b01e),
    (0x09B024, i_09b024),
    (0x09B02A, i_09b02a),
    (0x09B030, i_09b030),
    (0x09B034, i_09b034),
    (0x09B038, i_09b038),
    (0x09B03E, i_09b03e),
    (0x09B044, i_09b044),
    (0x09B048, i_09b048),
    (0x09B04C, i_09b04c),
    (0x09B052, i_09b052),
    (0x09B058, i_09b058),
    (0x09B05C, i_09b05c),
    (0x09B060, i_09b060),
    (0x09B064, i_09b064),
    (0x09B068, i_09b068),
    (0x09B06E, i_09b06e),
    (0x09B072, i_09b072),
    (0x09B078, i_09b078),
    (0x09B07E, i_09b07e),
    (0x09B084, i_09b084),
    (0x09B088, i_09b088),
    (0x09B08E, i_09b08e),
    (0x09B094, i_09b094),
    (0x09B09A, i_09b09a),
    (0x09B09E, i_09b09e),
    (0x09B0A2, i_09b0a2),
    (0x09B0A6, i_09b0a6),
    (0x09B0A8, i_09b0a8),
    (0x09B0AC, i_09b0ac),
    (0x09B0B2, i_09b0b2),
    (0x09B0B6, i_09b0b6),
    (0x09B0BC, i_09b0bc),
    (0x09B0C0, i_09b0c0),
    (0x09B0C4, i_09b0c4),
    (0x09B0C6, i_09b0c6),
    (0x09B0CA, i_09b0ca),
    (0x09B0CE, i_09b0ce),
    (0x09B0D2, i_09b0d2),
    (0x09B0D4, i_09b0d4),
    (0x09B0D8, i_09b0d8),
    (0x09B0DE, i_09b0de),
    (0x09B0E2, i_09b0e2),
    (0x09B0E8, i_09b0e8),
    (0x09B0EE, i_09b0ee),
    (0x09B0F4, i_09b0f4),
    (0x09B0F8, i_09b0f8),
    (0x09B0FA, i_09b0fa),
    (0x09B0FE, i_09b0fe),
    (0x09B104, i_09b104),
    (0x09B108, i_09b108),
    (0x09B10E, i_09b10e),
    (0x09B112, i_09b112),
    (0x09B116, i_09b116),
    (0x09B11A, i_09b11a),
    (0x09B11C, i_09b11c),
    (0x09B11E, i_09b11e),
    (0x09B122, i_09b122),
    (0x09B128, i_09b128),
    (0x09B12C, i_09b12c),
    (0x09B132, i_09b132),
    (0x09B136, i_09b136),
    (0x09B13A, i_09b13a),
    (0x09B13C, i_09b13c),
    (0x09B13E, i_09b13e),
    (0x09B142, i_09b142),
    (0x09B146, i_09b146),
    (0x09B14A, i_09b14a),
    (0x09B14E, i_09b14e),
    (0x09B152, i_09b152),
    (0x09B158, i_09b158),
    (0x09B15C, i_09b15c),
    (0x09B162, i_09b162),
    (0x09B168, i_09b168),
    (0x09B16E, i_09b16e),
    (0x09B174, i_09b174),
    (0x09B17A, i_09b17a),
    (0x09B17E, i_09b17e),
    (0x09B180, i_09b180),
    (0x09B184, i_09b184),
    (0x09B18A, i_09b18a),
    (0x09B18E, i_09b18e),
    (0x09B194, i_09b194),
    (0x09B19A, i_09b19a),
    (0x09B1A0, i_09b1a0),
    (0x09B1A4, i_09b1a4),
    (0x09B1A6, i_09b1a6),
    (0x09B1AA, i_09b1aa),
    (0x09B1B0, i_09b1b0),
    (0x09B1B4, i_09b1b4),
    (0x09B1BA, i_09b1ba),
    (0x09B1C0, i_09b1c0),
    (0x09B1C6, i_09b1c6),
    (0x09B1CA, i_09b1ca),
    (0x09B1CE, i_09b1ce),
    (0x09B1D4, i_09b1d4),
    (0x09B1D8, i_09b1d8),
    (0x09B1DE, i_09b1de),
    (0x09B1E2, i_09b1e2),
    (0x09B1E6, i_09b1e6),
    (0x09B1EA, i_09b1ea),
    (0x09B1EE, i_09b1ee),
    (0x09B1F2, i_09b1f2),
    (0x09B1F6, i_09b1f6),
    (0x09B1FA, i_09b1fa),
    (0x09B1FE, i_09b1fe),
    (0x09B204, i_09b204),
    (0x09B208, i_09b208),
    (0x09B20E, i_09b20e),
    (0x09B214, i_09b214),
    (0x09B21A, i_09b21a),
    (0x09B220, i_09b220),
    (0x09B226, i_09b226),
    (0x09B22A, i_09b22a),
    (0x09B22C, i_09b22c),
    (0x09B230, i_09b230),
    (0x09B236, i_09b236),
    (0x09B23A, i_09b23a),
    (0x09B240, i_09b240),
    (0x09B244, i_09b244),
    (0x09B248, i_09b248),
    (0x09B24A, i_09b24a),
    (0x09B24E, i_09b24e),
    (0x09B254, i_09b254),
    (0x09B25A, i_09b25a),
    (0x09B25E, i_09b25e),
    (0x09B264, i_09b264),
    (0x09B26A, i_09b26a),
    (0x09B270, i_09b270),
    (0x09B274, i_09b274),
    (0x09B276, i_09b276),
    (0x09B27A, i_09b27a),
    (0x09B280, i_09b280),
    (0x09B284, i_09b284),
    (0x09B288, i_09b288),
    (0x09B28A, i_09b28a),
    (0x09B28E, i_09b28e),
    (0x09B292, i_09b292),
    (0x09B298, i_09b298),
    (0x09B29E, i_09b29e),
    (0x09B2A2, i_09b2a2),
    (0x09B2A6, i_09b2a6),
    (0x09B2AC, i_09b2ac),
    (0x09B2B0, i_09b2b0),
    (0x09B2B2, i_09b2b2),
    (0x09B2B4, i_09b2b4),
    (0x09B2B6, i_09b2b6),
    (0x09B2B8, i_09b2b8),
    (0x09B2BC, i_09b2bc),
    (0x09B2C2, i_09b2c2),
    (0x09B2C6, i_09b2c6),
    (0x09B2CA, i_09b2ca),
    (0x09B2CE, i_09b2ce),
    (0x09B2D4, i_09b2d4),
    (0x09B2D8, i_09b2d8),
    (0x09B2DA, i_09b2da),
    (0x09B2DE, i_09b2de),
    (0x09B2E2, i_09b2e2),
    (0x09B2E6, i_09b2e6),
    (0x09B2EC, i_09b2ec),
    (0x09B2F2, i_09b2f2),
    (0x09B2F6, i_09b2f6),
    (0x09B2FA, i_09b2fa),
    (0x09B300, i_09b300),
    (0x09B304, i_09b304),
    (0x09B30A, i_09b30a),
    (0x09B310, i_09b310),
    (0x09B316, i_09b316),
    (0x09B31C, i_09b31c),
    (0x09B322, i_09b322),
    (0x09B326, i_09b326),
    (0x09B328, i_09b328),
    (0x09B32E, i_09b32e),
    (0x09B332, i_09b332),
    (0x09B336, i_09b336),
    (0x09B338, i_09b338),
    (0x09B33E, i_09b33e),
    (0x09B342, i_09b342),
    (0x09B348, i_09b348),
    (0x09B34C, i_09b34c),
    (0x09B350, i_09b350),
    (0x09B352, i_09b352),
    (0x09B356, i_09b356),
    (0x09B35C, i_09b35c),
    (0x09B360, i_09b360),
    (0x09B364, i_09b364),
    (0x09B368, i_09b368),
    (0x09B36C, i_09b36c),
    (0x09B372, i_09b372),
    (0x09B376, i_09b376),
    (0x09B378, i_09b378),
    (0x09B37C, i_09b37c),
    (0x09B380, i_09b380),
    (0x09B384, i_09b384),
    (0x09B388, i_09b388),
    (0x09B38A, i_09b38a),
    (0x09B390, i_09b390),
    (0x09B394, i_09b394),
    (0x09B398, i_09b398),
    (0x09B39A, i_09b39a),
    (0x09B3A0, i_09b3a0),
    (0x09B3A4, i_09b3a4),
    (0x09B3AA, i_09b3aa),
    (0x09B3AE, i_09b3ae),
    (0x09B3B2, i_09b3b2),
    (0x09B3B4, i_09b3b4),
    (0x09B3B8, i_09b3b8),
    (0x09B3BC, i_09b3bc),
    (0x09B3C0, i_09b3c0),
    (0x09B3C2, i_09b3c2),
    (0x09B3C6, i_09b3c6),
    (0x09B3CA, i_09b3ca),
    (0x09B3D0, i_09b3d0),
    (0x09B3D8, i_09b3d8),
    (0x09B3DC, i_09b3dc),
    (0x09B3E2, i_09b3e2),
    (0x09B3E8, i_09b3e8),
    (0x09B3EE, i_09b3ee),
    (0x09B3F4, i_09b3f4),
    (0x09B3FA, i_09b3fa),
    (0x09B3FE, i_09b3fe),
    (0x09B400, i_09b400),
    (0x09B404, i_09b404),
    (0x09B408, i_09b408),
    (0x09B40E, i_09b40e),
    (0x09B414, i_09b414),
    (0x09B41A, i_09b41a),
    (0x09B420, i_09b420),
    (0x09B424, i_09b424),
    (0x09B42A, i_09b42a),
    (0x09B430, i_09b430),
    (0x09B434, i_09b434),
    (0x09B438, i_09b438),
    (0x09B43A, i_09b43a),
    (0x09B43E, i_09b43e),
    (0x09B442, i_09b442),
    (0x09B448, i_09b448),
    (0x09B44E, i_09b44e),
    (0x09B454, i_09b454),
    (0x09B458, i_09b458),
    (0x09B45A, i_09b45a),
    (0x09B460, i_09b460),
    (0x09B466, i_09b466),
    (0x09B46A, i_09b46a),
    (0x09B46E, i_09b46e),
    (0x09B470, i_09b470),
    (0x09B474, i_09b474),
    (0x09B47A, i_09b47a),
    (0x09B47C, i_09b47c),
    (0x09B482, i_09b482),
    (0x09B488, i_09b488),
    (0x09B48A, i_09b48a),
    (0x09B490, i_09b490),
    (0x09B494, i_09b494),
    (0x09B49A, i_09b49a),
    (0x09B49E, i_09b49e),
    (0x09B4A0, i_09b4a0),
    (0x09B4A4, i_09b4a4),
    (0x09B4A6, i_09b4a6),
    (0x09B4A8, i_09b4a8),
    (0x09B4AC, i_09b4ac),
    (0x09B4B0, i_09b4b0),
    (0x09B4B4, i_09b4b4),
    (0x09B4B8, i_09b4b8),
    (0x09B4BC, i_09b4bc),
    (0x09B4BE, i_09b4be),
    (0x09B4C2, i_09b4c2),
    (0x09B4C6, i_09b4c6),
    (0x09B4CC, i_09b4cc),
    (0x09B4D0, i_09b4d0),
    (0x09B4D4, i_09b4d4),
    (0x09B4D8, i_09b4d8),
    (0x09B4DC, i_09b4dc),
    (0x09B4E2, i_09b4e2),
    (0x09B4E8, i_09b4e8),
    (0x09B4EE, i_09b4ee),
    (0x09B4F2, i_09b4f2),
    (0x09B4F6, i_09b4f6),
    (0x09B4FA, i_09b4fa),
    (0x09B4FC, i_09b4fc),
    (0x09B500, i_09b500),
    (0x09B504, i_09b504),
    (0x09B508, i_09b508),
    (0x09B50A, i_09b50a),
    (0x09B50E, i_09b50e),
    (0x09B512, i_09b512),
    (0x09B518, i_09b518),
    (0x09B51A, i_09b51a),
    (0x09B51E, i_09b51e),
    (0x09B524, i_09b524),
    (0x09B528, i_09b528),
    (0x09B52E, i_09b52e),
    (0x09B532, i_09b532),
    (0x09B536, i_09b536),
    (0x09B53C, i_09b53c),
    (0x09B540, i_09b540),
    (0x09B542, i_09b542),
    (0x09B546, i_09b546),
    (0x09B54A, i_09b54a),
    (0x09B550, i_09b550),
    (0x09B554, i_09b554),
    (0x09B55A, i_09b55a),
    (0x09B55E, i_09b55e),
    (0x09B562, i_09b562),
    (0x09B568, i_09b568),
    (0x09B56C, i_09b56c),
    (0x09B570, i_09b570),
    (0x09B574, i_09b574),
    (0x09B578, i_09b578),
    (0x09B57C, i_09b57c),
    (0x09B580, i_09b580),
    (0x09B582, i_09b582),
    (0x09B586, i_09b586),
    (0x09B58C, i_09b58c),
    (0x09B590, i_09b590),
    (0x09B594, i_09b594),
    (0x09B598, i_09b598),
    (0x09B59C, i_09b59c),
    (0x09B5A0, i_09b5a0),
    (0x09B5A6, i_09b5a6),
    (0x09B5AC, i_09b5ac),
    (0x09B5B2, i_09b5b2),
    (0x09B5B6, i_09b5b6),
    (0x09B5BA, i_09b5ba),
    (0x09B5BC, i_09b5bc),
    (0x09B5BE, i_09b5be),
    (0x09B5C2, i_09b5c2),
    (0x09B5C6, i_09b5c6),
    (0x09B5CA, i_09b5ca),
    (0x09B5CE, i_09b5ce),
    (0x09B5D2, i_09b5d2),
    (0x09B5D4, i_09b5d4),
    (0x09B5D8, i_09b5d8),
    (0x09B5DC, i_09b5dc),
    (0x09B5E2, i_09b5e2),
    (0x09B5E4, i_09b5e4),
    (0x09B5EA, i_09b5ea),
    (0x09B5F0, i_09b5f0),
    (0x09B5F4, i_09b5f4),
    (0x09B5F6, i_09b5f6),
    (0x09B5FC, i_09b5fc),
    (0x09B602, i_09b602),
    (0x09B606, i_09b606),
    (0x09B60A, i_09b60a),
    (0x09B60E, i_09b60e),
    (0x09B612, i_09b612),
    (0x09B618, i_09b618),
    (0x09B61C, i_09b61c),
    (0x09B620, i_09b620),
    (0x09B624, i_09b624),
    (0x09B62A, i_09b62a),
    (0x09B630, i_09b630),
    (0x09B634, i_09b634),
    (0x09B638, i_09b638),
    (0x09B63A, i_09b63a),
    (0x09B63E, i_09b63e),
    (0x09B644, i_09b644),
    (0x09B648, i_09b648),
    (0x09B64C, i_09b64c),
    (0x09B64E, i_09b64e),
    (0x09B654, i_09b654),
    (0x09B658, i_09b658),
    (0x09B65E, i_09b65e),
    (0x09B662, i_09b662),
    (0x09B666, i_09b666),
    (0x09B668, i_09b668),
    (0x09B66A, i_09b66a),
    (0x09B66E, i_09b66e),
    (0x09B672, i_09b672),
    (0x09B676, i_09b676),
    (0x09B67A, i_09b67a),
    (0x09B680, i_09b680),
    (0x09B684, i_09b684),
    (0x09B68A, i_09b68a),
    (0x09B68E, i_09b68e),
    (0x09B692, i_09b692),
    (0x09B698, i_09b698),
    (0x09B69C, i_09b69c),
    (0x09B6A2, i_09b6a2),
    (0x09B6A6, i_09b6a6),
    (0x09B6AC, i_09b6ac),
    (0x09B6B2, i_09b6b2),
    (0x09B6B8, i_09b6b8),
    (0x09B6BC, i_09b6bc),
    (0x09B6C2, i_09b6c2),
    (0x09B6C6, i_09b6c6),
    (0x09B6CC, i_09b6cc),
    (0x09B6D2, i_09b6d2),
    (0x09B6D8, i_09b6d8),
    (0x09B6DC, i_09b6dc),
    (0x09B6E0, i_09b6e0),
    (0x09B6E6, i_09b6e6),
    (0x09B6E8, i_09b6e8),
    (0x09B6EC, i_09b6ec),
    (0x09B6F2, i_09b6f2),
    (0x09B6F6, i_09b6f6),
    (0x09B6FC, i_09b6fc),
    (0x09B700, i_09b700),
    (0x09B702, i_09b702),
    (0x09B708, i_09b708),
    (0x09B70C, i_09b70c),
    (0x09B712, i_09b712),
    (0x09B716, i_09b716),
    (0x09B71C, i_09b71c),
    (0x09B720, i_09b720),
    (0x09B724, i_09b724),
    (0x09B728, i_09b728),
    (0x09B72C, i_09b72c),
    (0x09B730, i_09b730),
    (0x09B734, i_09b734),
    (0x09B738, i_09b738),
    (0x09B73C, i_09b73c),
    (0x09B73E, i_09b73e),
    (0x09B742, i_09b742),
    (0x09B748, i_09b748),
    (0x09B74E, i_09b74e),
    (0x09B752, i_09b752),
    (0x09B756, i_09b756),
    (0x09B758, i_09b758),
    (0x09B75A, i_09b75a),
    (0x09B760, i_09b760),
    (0x09B766, i_09b766),
    (0x09B76C, i_09b76c),
    (0x09B770, i_09b770),
    (0x09B772, i_09b772),
    (0x09B778, i_09b778),
    (0x09B77C, i_09b77c),
    (0x09B782, i_09b782),
    (0x09B786, i_09b786),
    (0x09B78C, i_09b78c),
    (0x09B790, i_09b790),
    (0x09B794, i_09b794),
    (0x09B796, i_09b796),
    (0x09B79A, i_09b79a),
    (0x09B7A0, i_09b7a0),
    (0x09B7A4, i_09b7a4),
    (0x09B7A8, i_09b7a8),
    (0x09B7AA, i_09b7aa),
    (0x09B7B0, i_09b7b0),
    (0x09B7B4, i_09b7b4),
    (0x09B7B8, i_09b7b8),
    (0x09B7BA, i_09b7ba),
    (0x09B7BC, i_09b7bc),
    (0x09B7C0, i_09b7c0),
    (0x09B7C2, i_09b7c2),
    (0x09B7C8, i_09b7c8),
    (0x09B7CC, i_09b7cc),
    (0x09B7D2, i_09b7d2),
    (0x09B7D6, i_09b7d6),
    (0x09B7DC, i_09b7dc),
    (0x09B7E0, i_09b7e0),
    (0x09B7E4, i_09b7e4),
    (0x09B7E6, i_09b7e6),
    (0x09B7EC, i_09b7ec),
    (0x09B7F0, i_09b7f0),
    (0x09B7F4, i_09b7f4),
    (0x09B7F6, i_09b7f6),
    (0x09B7F8, i_09b7f8),
    (0x09B7FC, i_09b7fc),
    (0x09B7FE, i_09b7fe),
    (0x09B804, i_09b804),
    (0x09B80C, i_09b80c),
    (0x09B810, i_09b810),
    (0x09B814, i_09b814),
];
