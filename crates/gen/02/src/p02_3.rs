// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p02_3) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $02A4A0: rts
pub fn i_02a4a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A4A2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_ResetTurnTimer_02a4a2  ($02A4A2, 6 B, slug/helpers_0295a6.s)
/// $02A4A2: clr.b $84(a6)
pub fn i_02a4a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A4A6;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x84);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $02A4A6: rts
pub fn i_02a4a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A4A8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_TurnTimerTick_02a4a8  ($02A4A8, 20 B, slug/helpers_0295a6.s)
/// $02A4A8: jsr $5cd60.l
pub fn i_02a4a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A4AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A4AE);
    cpu.pc = 0x5CD60;
    16
}
/// $02A4AE: bcc.w $2a4ce
pub fn i_02a4ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A4B2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2A4CE; return 10; }
    8
}
/// $02A4B2: cmpi.b #$14, $84(a6)
pub fn i_02a4b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A4B8;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x14, Size::Byte, false);
    16
}
/// $02A4B8: bne.w $2a4c6
pub fn i_02a4b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A4BC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2A4C6; return 10; }
    8
}
// ===== SetXN_02a4bc  ($02A4BC, 6 B, thunks/ccr_helpers.c)
/// $02A4BC: ori.b #$11, ccr
pub fn i_02a4bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A4C0;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02A4C0: rts
pub fn i_02a4c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A4C2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_TurnTimerDone_02a4c2  ($02A4C2, 12 B, slug/helpers_0295a6.s)
/// $02A4C2: bra.w $2a4ca
pub fn i_02a4c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A4C6;
    cpu.ir = 0x6000;
    cpu.pc = 0x2A4CA;
    10
}
/// $02A4C6: addq.b #$1, $84(a6)
pub fn i_02a4c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A4CA;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x84);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02A4CA: bra.w $2a4e6
pub fn i_02a4ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A4CE;
    cpu.ir = 0x6000;
    cpu.pc = 0x2A4E6;
    10
}
// ===== Slug_TurnTimerTickB_02a4ce  ($02A4CE, 24 B, slug/helpers_0295a6.s)
/// $02A4CE: cmpi.b #$14, $84(a6)
pub fn i_02a4ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A4D4;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x14, Size::Byte, false);
    16
}
/// $02A4D4: bne.w $2a4e2
pub fn i_02a4d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A4D8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2A4E2; return 10; }
    8
}
/// $02A4D8: move.b #$ff, $84(a6)
pub fn i_02a4d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A4DE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02A4DE: bra.w $2a4e6
pub fn i_02a4de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A4E2;
    cpu.ir = 0x6000;
    cpu.pc = 0x2A4E6;
    10
}
/// $02A4E2: clr.b $84(a6)
pub fn i_02a4e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A4E6;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x84);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
// ===== ClearXN_02a4e6  ($02A4E6, 6 B, thunks/ccr_helpers.c)
/// $02A4E6: andi.b #$ee, ccr
pub fn i_02a4e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A4EA;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02A4EA: rts
pub fn i_02a4ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A4EC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_TerrainIsSlope_02a4ec  ($02A4EC, 4 B, slug/helpers_0295a6.s)
/// $02A4EC: bra.w $2a506
pub fn i_02a4ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A4F0;
    cpu.ir = 0x6000;
    cpu.pc = 0x2A506;
    10
}
// ===== Slug_UpdateAngleIsSlope_02a4f0  ($02A4F0, 12 B, slug/helpers_0295a6.s)
/// $02A4F0: jsr $2a958(pc)
pub fn i_02a4f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A4F4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2A4F4);
    cpu.pc = 0x2A958;
    16
}
/// $02A4F4: cmp.w $80(a6), d2
pub fn i_02a4f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A4F8;
    cpu.ir = 0xB46E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $02A4F8: bne.w $2a502
pub fn i_02a4f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A4FC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2A502; return 10; }
    8
}
// ===== ClearXN_02a4fc  ($02A4FC, 6 B, thunks/ccr_helpers.c)
/// $02A4FC: andi.b #$ee, ccr
pub fn i_02a4fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A500;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02A500: rts
pub fn i_02a500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A502;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_SetAngleIsSlope_02a502  ($02A502, 22 B, slug/helpers_0295a6.s)
/// $02A502: move.w d2, $80(a6)
pub fn i_02a502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A506;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02A506: move.w $80(a6), d7
pub fn i_02a506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A50A;
    cpu.ir = 0x3E2E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02A50A: lea.l $2a558(pc), a0
pub fn i_02a50a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A50E;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A558;
    16
}
/// $02A50E: move.b (a0, d7.w), d0
pub fn i_02a50e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A512;
    cpu.ir = 0x1030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $02A512: cmpi.b #$1, d0
pub fn i_02a512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A516;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $02A516: rts
pub fn i_02a516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A518;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_CheckFreeThenC_02a59a  ($02A59A, 8 B, slug/helpers_0295a6.s)
/// $02A59A: jsr $2abc6(pc)
pub fn i_02a59a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A59E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2A59E);
    cpu.pc = 0x2ABC6;
    16
}
/// $02A59E: bcc.w $2a5a8
pub fn i_02a59e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A5A2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2A5A8; return 10; }
    8
}
// ===== SetXN_02a5a2  ($02A5A2, 6 B, thunks/ccr_helpers.c)
/// $02A5A2: ori.b #$11, ccr
pub fn i_02a5a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A5A6;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02A5A6: rts
pub fn i_02a5a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A5A8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_02a5a8  ($02A5A8, 6 B, thunks/ccr_helpers.c)
/// $02A5A8: andi.b #$ee, ccr
pub fn i_02a5a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A5AC;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02A5AC: rts
pub fn i_02a5ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A5AE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrAbsThunk_02a5cc  ($02A5CC, 8 B, thunks/jsr_abs_thunks.c)
/// $02A5CC: jsr $5d00e.l
pub fn i_02a5cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A5D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A5D2);
    cpu.pc = 0x5D00E;
    16
}
/// $02A5D2: rts
pub fn i_02a5d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A5D4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_MusicByTerrainA_02a5d4  ($02A5D4, 30 B, slug/helpers_0295a6.s)
/// $02A5D4: jsr $27eba.l
pub fn i_02a5d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A5DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A5DA);
    cpu.pc = 0x27EBA;
    16
}
/// $02A5DA: andi.b #$c0, d0
pub fn i_02a5da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A5DE;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xC0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $02A5DE: cmpi.b #$40, d0
pub fn i_02a5de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A5E2;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x40, Size::Byte, false);
    8
}
/// $02A5E2: beq.w $2a5fa
pub fn i_02a5e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A5E6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2A5FA; return 10; }
    8
}
/// $02A5E6: cmpi.b #$80, d0
pub fn i_02a5e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A5EA;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x80, Size::Byte, false);
    8
}
/// $02A5EA: beq.w $2a5fa
pub fn i_02a5ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A5EE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2A5FA; return 10; }
    8
}
/// $02A5EE: move.w #$1090, d0
pub fn i_02a5ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A5F2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1090;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_02a5f2  ($02A5F2, 8 B, thunks/jsr_abs_thunks.c)
/// $02A5F2: jsr $2352.l
pub fn i_02a5f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A5F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A5F8);
    cpu.pc = 0x2352;
    16
}
/// $02A5F8: rts
pub fn i_02a5f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A5FA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_Music1052_02a5fa  ($02A5FA, 4 B, slug/helpers_0295a6.s)
/// $02A5FA: move.w #$1052, d0
pub fn i_02a5fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A5FE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1052;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_02a5fe  ($02A5FE, 8 B, thunks/jsr_abs_thunks.c)
/// $02A5FE: jsr $2352.l
pub fn i_02a5fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A604;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A604);
    cpu.pc = 0x2352;
    16
}
/// $02A604: rts
pub fn i_02a604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A606;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_MusicByTerrainB_02a606  ($02A606, 30 B, slug/helpers_0295a6.s)
/// $02A606: jsr $27eba.l
pub fn i_02a606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A60C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A60C);
    cpu.pc = 0x27EBA;
    16
}
/// $02A60C: andi.b #$c0, d0
pub fn i_02a60c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A610;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xC0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $02A610: cmpi.b #$40, d0
pub fn i_02a610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A614;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x40, Size::Byte, false);
    8
}
/// $02A614: beq.w $2a62c
pub fn i_02a614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A618;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2A62C; return 10; }
    8
}
/// $02A618: cmpi.b #$80, d0
pub fn i_02a618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A61C;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x80, Size::Byte, false);
    8
}
/// $02A61C: beq.w $2a62c
pub fn i_02a61c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A620;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2A62C; return 10; }
    8
}
/// $02A620: move.w #$1091, d0
pub fn i_02a620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A624;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1091;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_02a624  ($02A624, 8 B, thunks/jsr_abs_thunks.c)
/// $02A624: jsr $2352.l
pub fn i_02a624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A62A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A62A);
    cpu.pc = 0x2352;
    16
}
/// $02A62A: rts
pub fn i_02a62a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A62C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_Music1051_02a62c  ($02A62C, 4 B, slug/helpers_0295a6.s)
/// $02A62C: move.w #$1051, d0
pub fn i_02a62c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A630;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1051;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_02a630  ($02A630, 8 B, thunks/jsr_abs_thunks.c)
/// $02A630: jsr $2352.l
pub fn i_02a630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A636;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A636);
    cpu.pc = 0x2352;
    16
}
/// $02A636: rts
pub fn i_02a636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A638;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_DestroyedMusicAndBubble_02a638  ($02A638, 36 B, slug/helpers_0295a6.s)
/// $02A638: btst.b #$0, $13(a6)
pub fn i_02a638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A63E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $02A63E: beq.w $2a64c
pub fn i_02a63e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A642;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2A64C; return 10; }
    8
}
/// $02A642: move.w #$10e9, d0
pub fn i_02a642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A646;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10E9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02A646: jsr $2352.l
pub fn i_02a646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A64C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A64C);
    cpu.pc = 0x2352;
    16
}
/// $02A64C: btst.b #$0, $13(a6)
pub fn i_02a64c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A652;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $02A652: beq.w $2a662
pub fn i_02a652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A656;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2A662; return 10; }
    8
}
/// $02A656: lea.l $3207c.l, a1
pub fn i_02a656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A65C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x3207C;
    20
}
// ===== JsrAbsThunk_02a65c  ($02A65C, 8 B, thunks/jsr_abs_thunks.c)
/// $02A65C: jsr $4ae.l
pub fn i_02a65c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A662;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A662);
    cpu.pc = 0x4AE;
    16
}
/// $02A662: rts
pub fn i_02a662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A664;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_TryStartDestroyed_02a664  ($02A664, 32 B, slug/helpers_0295a6.s)
/// $02A664: btst.b #$5, $8d(a6)
pub fn i_02a664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A66A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $02A66A: bne.w $2a68a
pub fn i_02a66a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A66E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2A68A; return 10; }
    8
}
/// $02A66E: tst.w $106e92.l
pub fn i_02a66e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A674;
    cpu.ir = 0x4A79;
    let v2 = bus.read16(0x106E92) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    16
}
/// $02A674: bne.w $2a68a
pub fn i_02a674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A678;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2A68A; return 10; }
    8
}
/// $02A678: bset.b #$0, $13(a6)
pub fn i_02a678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A67E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02A67E: lea.l $2dcc0(pc), a1
pub fn i_02a67e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A682;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DCC0;
    16
}
/// $02A682: move.l a1, (a6)
pub fn i_02a682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A684;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== ClearXN_02a684  ($02A684, 6 B, thunks/ccr_helpers.c)
/// $02A684: andi.b #$ee, ccr
pub fn i_02a684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A688;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02A688: rts
pub fn i_02a688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A68A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpAbsThunk_02a68a  ($02A68A, 6 B, thunks/jmp_abs_thunks.c)
/// $02A68A: jmp $28758.l
pub fn i_02a68a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A690;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x28758;
    8
}
// ===== Slug_TryStartDestroyedB_02a690  ($02A690, 40 B, slug/helpers_0295a6.s)
/// $02A690: btst.b #$5, $8d(a6)
pub fn i_02a690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A696;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $02A696: bne.w $2a6be
pub fn i_02a696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A69A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2A6BE; return 10; }
    8
}
/// $02A69A: tst.w $106e92.l
pub fn i_02a69a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A6A0;
    cpu.ir = 0x4A79;
    let v2 = bus.read16(0x106E92) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    16
}
/// $02A6A0: bne.w $2a6be
pub fn i_02a6a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A6A4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2A6BE; return 10; }
    8
}
/// $02A6A4: bset.b #$3, $13(a6)
pub fn i_02a6a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A6AA;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02A6AA: bset.b #$0, $13(a6)
pub fn i_02a6aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A6B0;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02A6B0: lea.l $2dcc0(pc), a1
pub fn i_02a6b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A6B4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DCC0;
    16
}
/// $02A6B4: move.l a1, (a6)
pub fn i_02a6b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A6B6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02A6B6: clr.w d0
pub fn i_02a6b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A6B8;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
// ===== ClearXN_02a6b8  ($02A6B8, 6 B, thunks/ccr_helpers.c)
/// $02A6B8: andi.b #$ee, ccr
pub fn i_02a6b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A6BC;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02A6BC: rts
pub fn i_02a6bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A6BE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_DamageTick_02a6be  ($02A6BE, 72 B, slug/helpers_0295a6.s)
/// $02A6BE: tst.b $45(a6)
pub fn i_02a6be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A6C2;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $02A6C2: beq.w $2a6cc
pub fn i_02a6c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A6C6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2A6CC; return 10; }
    8
}
/// $02A6C6: jsr $8f6da.l
pub fn i_02a6c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A6CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A6CC);
    cpu.pc = 0x8F6DA;
    16
}
/// $02A6CC: btst.b #$3, $100001.l
pub fn i_02a6cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A6D4;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((3) % 8);
    let v2 = bus.read8(0x100001) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $02A6D4: bne.w $2a712
pub fn i_02a6d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A6D8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2A712; return 10; }
    8
}
/// $02A6D8: btst.b #$5, $100001.l
pub fn i_02a6d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A6E0;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((5) % 8);
    let v2 = bus.read8(0x100001) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $02A6E0: beq.w $2a6ea
pub fn i_02a6e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A6E4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2A6EA; return 10; }
    8
}
/// $02A6E4: move.w #$300, $66(a6)
pub fn i_02a6e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A6EA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02A6EA: jsr $2870a.l
pub fn i_02a6ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A6F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A6F0);
    cpu.pc = 0x2870A;
    16
}
/// $02A6F0: bcc.w $2a70c
pub fn i_02a6f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A6F4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2A70C; return 10; }
    8
}
/// $02A6F4: movem.w d0, -(a7)
pub fn i_02a6f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A6F8;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $02A6F8: move.w #$1089, d0
pub fn i_02a6f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A6FC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1089;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02A6FC: jsr $2352.l
pub fn i_02a6fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A702;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A702);
    cpu.pc = 0x2352;
    16
}
/// $02A702: movem.w (a7)+, d0
pub fn i_02a702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A706;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
// ===== SetXN_02a706  ($02A706, 6 B, thunks/ccr_helpers.c)
/// $02A706: ori.b #$11, ccr
pub fn i_02a706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A70A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02A70A: rts
pub fn i_02a70a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A70C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_02a70c  ($02A70C, 6 B, thunks/ccr_helpers.c)
/// $02A70C: andi.b #$ee, ccr
pub fn i_02a70c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A710;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02A710: rts
pub fn i_02a710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A712;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_ResetHP_02a712  ($02A712, 8 B, slug/helpers_0295a6.s)
/// $02A712: move.w #$30, $66(a6)
pub fn i_02a712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A718;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02A718: clr.w d0
pub fn i_02a718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A71A;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
// ===== ClearXN_02a71a  ($02A71A, 6 B, thunks/ccr_helpers.c)
/// $02A71A: andi.b #$ee, ccr
pub fn i_02a71a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A71E;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02A71E: rts
pub fn i_02a71e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A720;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_UpdateInputFlags_02a720  ($02A720, 50 B, slug/helpers_0295a6.s)
/// $02A720: bclr.b #$4, $13(a6)
pub fn i_02a720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A726;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02A726: bclr.b #$5, $13(a6)
pub fn i_02a726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A72C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02A72C: jsr $5ceec.l
pub fn i_02a72c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A732;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A732);
    cpu.pc = 0x5CEEC;
    16
}
/// $02A732: bcc.w $2a740
pub fn i_02a732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A736;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2A740; return 10; }
    8
}
/// $02A736: bset.b #$4, $13(a6)
pub fn i_02a736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A73C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02A73C: bra.w $2a740
pub fn i_02a73c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A740;
    cpu.ir = 0x6000;
    cpu.pc = 0x2A740;
    10
}
/// $02A740: jsr $5cef8.l
pub fn i_02a740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A746;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A746);
    cpu.pc = 0x5CEF8;
    16
}
/// $02A746: bcc.w $2a750
pub fn i_02a746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A74A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2A750; return 10; }
    8
}
/// $02A74A: bset.b #$5, $13(a6)
pub fn i_02a74a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A750;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02A750: rts
pub fn i_02a750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A752;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_PhysicsA_02a752  ($02A752, 6 B, slug/helpers_0295a6.s)
/// $02A752: jsr $28992.l
pub fn i_02a752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A758;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A758);
    cpu.pc = 0x28992;
    16
}
// ===== JsrAbsThunk_02a758  ($02A758, 8 B, thunks/jsr_abs_thunks.c)
/// $02A758: jsr $2785c.l
pub fn i_02a758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A75E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A75E);
    cpu.pc = 0x2785C;
    16
}
/// $02A75E: rts
pub fn i_02a75e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A760;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_PhysicsB_02a760  ($02A760, 6 B, slug/helpers_0295a6.s)
/// $02A760: jsr $28992.l
pub fn i_02a760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A766;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A766);
    cpu.pc = 0x28992;
    16
}
// ===== Slug_PhysicsC_02a766  ($02A766, 58 B, slug/helpers_0295a6.s)
/// $02A766: jsr $304c4.l
pub fn i_02a766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A76C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A76C);
    cpu.pc = 0x304C4;
    16
}
/// $02A76C: jsr $30704.l
pub fn i_02a76c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A772;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A772);
    cpu.pc = 0x30704;
    16
}
/// $02A772: beq.w $2a77c
pub fn i_02a772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A776;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2A77C; return 10; }
    8
}
/// $02A776: jsr $281b0.l
pub fn i_02a776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A77C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A77C);
    cpu.pc = 0x281B0;
    16
}
/// $02A77C: jsr $2788c.l
pub fn i_02a77c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A782;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A782);
    cpu.pc = 0x2788C;
    16
}
/// $02A782: jsr $2a720(pc)
pub fn i_02a782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A786;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2A786);
    cpu.pc = 0x2A720;
    16
}
/// $02A786: btst.b #$5, $5a(a6)
pub fn i_02a786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A78C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $02A78C: beq.w $2a796
pub fn i_02a78c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A790;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2A796; return 10; }
    8
}
/// $02A790: addi.w #$10, $82(a6)
pub fn i_02a790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A796;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $02A796: jsr $3076a.l
pub fn i_02a796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A79C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A79C);
    cpu.pc = 0x3076A;
    16
}
/// $02A79C: beq.w $2a7a6
pub fn i_02a79c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A7A0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2A7A6; return 10; }
    8
}
// ===== SetXN_02a7a0  ($02A7A0, 6 B, thunks/ccr_helpers.c)
/// $02A7A0: ori.b #$11, ccr
pub fn i_02a7a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A7A4;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02A7A4: rts
pub fn i_02a7a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A7A6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_02a7a6  ($02A7A6, 6 B, thunks/ccr_helpers.c)
/// $02A7A6: andi.b #$ee, ccr
pub fn i_02a7a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A7AA;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02A7AA: rts
pub fn i_02a7aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A7AC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_PhysicsD_02a7ac  ($02A7AC, 32 B, slug/helpers_0295a6.s)
/// $02A7AC: jsr $28992.l
pub fn i_02a7ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A7B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A7B2);
    cpu.pc = 0x28992;
    16
}
/// $02A7B2: jsr $304c4.l
pub fn i_02a7b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A7B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A7B8);
    cpu.pc = 0x304C4;
    16
}
/// $02A7B8: jsr $27a18.l
pub fn i_02a7b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A7BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A7BE);
    cpu.pc = 0x27A18;
    16
}
/// $02A7BE: jsr $2a720(pc)
pub fn i_02a7be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A7C2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2A7C2);
    cpu.pc = 0x2A720;
    16
}
/// $02A7C2: jsr $3076a.l
pub fn i_02a7c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A7C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A7C8);
    cpu.pc = 0x3076A;
    16
}
/// $02A7C8: beq.w $2a7d2
pub fn i_02a7c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A7CC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2A7D2; return 10; }
    8
}
// ===== SetXN_02a7cc  ($02A7CC, 6 B, thunks/ccr_helpers.c)
/// $02A7CC: ori.b #$11, ccr
pub fn i_02a7cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A7D0;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02A7D0: rts
pub fn i_02a7d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A7D2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_02a7d2  ($02A7D2, 6 B, thunks/ccr_helpers.c)
/// $02A7D2: andi.b #$ee, ccr
pub fn i_02a7d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A7D6;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02A7D6: rts
pub fn i_02a7d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A7D8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_PhysicsE_02a7d8  ($02A7D8, 64 B, slug/helpers_0295a6.s)
/// $02A7D8: jsr $28992.l
pub fn i_02a7d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A7DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A7DE);
    cpu.pc = 0x28992;
    16
}
/// $02A7DE: jsr $30704.l
pub fn i_02a7de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A7E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A7E4);
    cpu.pc = 0x30704;
    16
}
/// $02A7E4: beq.w $2a7ee
pub fn i_02a7e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A7E8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2A7EE; return 10; }
    8
}
/// $02A7E8: jsr $281b0.l
pub fn i_02a7e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A7EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A7EE);
    cpu.pc = 0x281B0;
    16
}
/// $02A7EE: jsr $304c4.l
pub fn i_02a7ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A7F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A7F4);
    cpu.pc = 0x304C4;
    16
}
/// $02A7F4: jsr $2788c.l
pub fn i_02a7f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A7FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A7FA);
    cpu.pc = 0x2788C;
    16
}
/// $02A7FA: jsr $2a720(pc)
pub fn i_02a7fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A7FE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2A7FE);
    cpu.pc = 0x2A720;
    16
}
/// $02A7FE: btst.b #$5, $5a(a6)
pub fn i_02a7fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A804;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $02A804: beq.w $2a80e
pub fn i_02a804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A808;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2A80E; return 10; }
    8
}
/// $02A808: addi.w #$10, $82(a6)
pub fn i_02a808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A80E;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $02A80E: jsr $3076a.l
pub fn i_02a80e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A814;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A814);
    cpu.pc = 0x3076A;
    16
}
/// $02A814: beq.w $2a81e
pub fn i_02a814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A818;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2A81E; return 10; }
    8
}
// ===== SetXN_02a818  ($02A818, 6 B, thunks/ccr_helpers.c)
/// $02A818: ori.b #$11, ccr
pub fn i_02a818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A81C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02A81C: rts
pub fn i_02a81c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A81E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_02a81e  ($02A81E, 6 B, thunks/ccr_helpers.c)
/// $02A81E: andi.b #$ee, ccr
pub fn i_02a81e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A822;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02A822: rts
pub fn i_02a822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A824;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_PhysicsF_02a824  ($02A824, 42 B, slug/helpers_0295a6.s)
/// $02A824: jsr $304c4.l
pub fn i_02a824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A82A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A82A);
    cpu.pc = 0x304C4;
    16
}
/// $02A82A: jsr $30704.l
pub fn i_02a82a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A830;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A830);
    cpu.pc = 0x30704;
    16
}
/// $02A830: beq.w $2a83a
pub fn i_02a830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A834;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2A83A; return 10; }
    8
}
/// $02A834: jsr $281b0.l
pub fn i_02a834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A83A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A83A);
    cpu.pc = 0x281B0;
    16
}
/// $02A83A: jsr $2788c.l
pub fn i_02a83a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A840;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A840);
    cpu.pc = 0x2788C;
    16
}
/// $02A840: jsr $2a720(pc)
pub fn i_02a840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A844;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2A844);
    cpu.pc = 0x2A720;
    16
}
/// $02A844: jsr $3076a.l
pub fn i_02a844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A84A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A84A);
    cpu.pc = 0x3076A;
    16
}
/// $02A84A: beq.w $2a854
pub fn i_02a84a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A84E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2A854; return 10; }
    8
}
// ===== SetXN_02a84e  ($02A84E, 6 B, thunks/ccr_helpers.c)
/// $02A84E: ori.b #$11, ccr
pub fn i_02a84e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A852;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02A852: rts
pub fn i_02a852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A854;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_02a854  ($02A854, 6 B, thunks/ccr_helpers.c)
/// $02A854: andi.b #$ee, ccr
pub fn i_02a854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A858;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02A858: rts
pub fn i_02a858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A85A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_PhysicsG_02a85a  ($02A85A, 22 B, slug/helpers_0295a6.s)
/// $02A85A: jsr $28992.l
pub fn i_02a85a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A860;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A860);
    cpu.pc = 0x28992;
    16
}
/// $02A860: jsr $30704.l
pub fn i_02a860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A866;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A866);
    cpu.pc = 0x30704;
    16
}
/// $02A866: beq.w $2a870
pub fn i_02a866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A86A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2A870; return 10; }
    8
}
/// $02A86A: jsr $281b0.l
pub fn i_02a86a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A870;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A870);
    cpu.pc = 0x281B0;
    16
}
// ===== JsrAbsThunk_02a870  ($02A870, 8 B, thunks/jsr_abs_thunks.c)
/// $02A870: jsr $2788c.l
pub fn i_02a870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A876;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A876);
    cpu.pc = 0x2788C;
    16
}
/// $02A876: rts
pub fn i_02a876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A878;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_PhysicsAir_02a878  ($02A878, 54 B, slug/helpers_0295a6.s)
/// $02A878: jsr $28992.l
pub fn i_02a878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A87E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A87E);
    cpu.pc = 0x28992;
    16
}
/// $02A87E: jsr $30704.l
pub fn i_02a87e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A884;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A884);
    cpu.pc = 0x30704;
    16
}
/// $02A884: beq.w $2a88e
pub fn i_02a884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A888;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2A88E; return 10; }
    8
}
/// $02A888: jsr $281b0.l
pub fn i_02a888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A88E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A88E);
    cpu.pc = 0x281B0;
    16
}
/// $02A88E: jsr $27a18.l
pub fn i_02a88e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A894;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A894);
    cpu.pc = 0x27A18;
    16
}
/// $02A894: bcs.w $2a8b4
pub fn i_02a894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A898;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2A8B4; return 10; }
    8
}
/// $02A898: tst.w $2a(a6)
pub fn i_02a898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A89C;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $02A89C: bge.w $2a8a8
pub fn i_02a89c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A8A0;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x2A8A8; return 10; }
    8
}
/// $02A8A0: jsr $2a8c0(pc)
pub fn i_02a8a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A8A4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2A8A4);
    cpu.pc = 0x2A8C0;
    16
}
/// $02A8A4: bcc.w $2a8b4
pub fn i_02a8a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A8A8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2A8B4; return 10; }
    8
}
/// $02A8A8: bset.b #$1, $8d(a6)
pub fn i_02a8a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A8AE;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== ClearXN_02a8ae  ($02A8AE, 6 B, thunks/ccr_helpers.c)
/// $02A8AE: andi.b #$ee, ccr
pub fn i_02a8ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A8B2;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02A8B2: rts
pub fn i_02a8b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A8B4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_ClearBit1Field8D_02a8b4  ($02A8B4, 6 B, slug/helpers_0295a6.s)
/// $02A8B4: bclr.b #$1, $8d(a6)
pub fn i_02a8b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A8BA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== SetXN_02a8ba  ($02A8BA, 6 B, thunks/ccr_helpers.c)
/// $02A8BA: ori.b #$11, ccr
pub fn i_02a8ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A8BE;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02A8BE: rts
pub fn i_02a8be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A8C0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_GroundContact_02a8c0  ($02A8C0, 110 B, slug/helpers_0295a6.s)
/// $02A8C0: btst.b #$6, $6b(a6)
pub fn i_02a8c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A8C6;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x6B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $02A8C6: beq.w $2a8d0
pub fn i_02a8c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A8CA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2A8D0; return 10; }
    8
}
/// $02A8CA: jmp $27eba.l
pub fn i_02a8ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A8D0;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x27EBA;
    8
}
/// $02A8D0: bclr.b #$3, $5b(a6)
pub fn i_02a8d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A8D6;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02A8D6: move.w $22(a6), d1
pub fn i_02a8d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A8DA;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02A8DA: move.w $82(a6), d2
pub fn i_02a8da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A8DE;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02A8DE: jsr $27ec2.l
pub fn i_02a8de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A8E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A8E4);
    cpu.pc = 0x27EC2;
    16
}
/// $02A8E4: bcs.w $2a8fa
pub fn i_02a8e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A8E8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2A8FA; return 10; }
    8
}
/// $02A8E8: movea.l $74(a6), a2
pub fn i_02a8e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A8EC;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $02A8EC: movea.l $78(a6), a3
pub fn i_02a8ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A8F0;
    cpu.ir = 0x266E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read32(ad2);
    cpu.a[3] = v3;
    16
}
/// $02A8F0: movea.l $7c(a6), a4
pub fn i_02a8f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A8F4;
    cpu.ir = 0x286E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read32(ad2);
    cpu.a[4] = v3;
    16
}
/// $02A8F4: moveq #$3, d1
pub fn i_02a8f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A8F6;
    cpu.ir = 0x7203;
    cpu.d[1] = 0x3;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $02A8F6: bra.w $2a940
pub fn i_02a8f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A8FA;
    cpu.ir = 0x6000;
    cpu.pc = 0x2A940;
    10
}
/// $02A8FA: movea.l $74(a6), a2
pub fn i_02a8fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A8FE;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $02A8FE: movea.l $78(a6), a3
pub fn i_02a8fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A902;
    cpu.ir = 0x266E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read32(ad2);
    cpu.a[3] = v3;
    16
}
/// $02A902: movea.l $7c(a6), a4
pub fn i_02a902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A906;
    cpu.ir = 0x286E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read32(ad2);
    cpu.a[4] = v3;
    16
}
/// $02A906: moveq #$0, d1
pub fn i_02a906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A908;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $02A908: add.b $84(a4), d1
pub fn i_02a908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A90C;
    cpu.ir = 0xD22C;
    let ad2 = cpu.a[4].wrapping_add(0x84);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[1] & 0xFF;
    let r = add_with_flags(cpu, h1, v3, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    12
}
/// $02A90C: add.b $84(a2), d1
pub fn i_02a90c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A910;
    cpu.ir = 0xD22A;
    let ad2 = cpu.a[2].wrapping_add(0x84);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[1] & 0xFF;
    let r = add_with_flags(cpu, h1, v3, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    12
}
/// $02A910: add.b $84(a3), d1
pub fn i_02a910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A914;
    cpu.ir = 0xD22B;
    let ad2 = cpu.a[3].wrapping_add(0x84);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[1] & 0xFF;
    let r = add_with_flags(cpu, h1, v3, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    12
}
/// $02A914: cmpi.b #$2, d1
pub fn i_02a914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A918;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $02A918: bge.w $2a934
pub fn i_02a918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A91C;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x2A934; return 10; }
    8
}
/// $02A91C: andi.w #$1ff, $24(a2)
pub fn i_02a91c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A922;
    cpu.ir = 0x26A;
    let ad1 = cpu.a[2].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x1FF) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $02A922: andi.w #$1ff, $24(a3)
pub fn i_02a922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A928;
    cpu.ir = 0x26B;
    let ad1 = cpu.a[3].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x1FF) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $02A928: andi.w #$1ff, $24(a4)
pub fn i_02a928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A92E;
    cpu.ir = 0x26C;
    let ad1 = cpu.a[4].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x1FF) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
// ===== SetXN_02a92e  ($02A92E, 6 B, thunks/ccr_helpers.c)
/// $02A92E: ori.b #$11, ccr
pub fn i_02a92e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A932;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02A932: rts
pub fn i_02a932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A934;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_GroundContactWrapY_02a934  ($02A934, 30 B, slug/helpers_0295a6.s)
/// $02A934: bset.b #$3, $5b(a6)
pub fn i_02a934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A93A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02A93A: move.w $24(a6), $82(a6)
pub fn i_02a93a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A940;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad4, v3 as u16);
    20
}
/// $02A940: andi.w #$1ff, $24(a2)
pub fn i_02a940(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A946;
    cpu.ir = 0x26A;
    let ad1 = cpu.a[2].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x1FF) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $02A946: andi.w #$1ff, $24(a3)
pub fn i_02a946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A94C;
    cpu.ir = 0x26B;
    let ad1 = cpu.a[3].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x1FF) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $02A94C: andi.w #$1ff, $24(a4)
pub fn i_02a94c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A952;
    cpu.ir = 0x26C;
    let ad1 = cpu.a[4].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x1FF) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
// ===== ClearXN_02a952  ($02A952, 6 B, thunks/ccr_helpers.c)
/// $02A952: andi.b #$ee, ccr
pub fn i_02a952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A956;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02A956: rts
pub fn i_02a956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A958;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_TerrainSlope_02a958  ($02A958, 72 B, slug/helpers_0295a6.s)
/// $02A958: movea.l $74(a6), a1
pub fn i_02a958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A95C;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $02A95C: move.w $22(a1), d1
pub fn i_02a95c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A960;
    cpu.ir = 0x3229;
    let ad2 = cpu.a[1].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02A960: move.w $9a(a1), d2
pub fn i_02a960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A964;
    cpu.ir = 0x3429;
    let ad2 = cpu.a[1].wrapping_add(0x9A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02A964: movea.l $78(a6), a2
pub fn i_02a964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A968;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $02A968: move.w $22(a2), d3
pub fn i_02a968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A96C;
    cpu.ir = 0x362A;
    let ad2 = cpu.a[2].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02A96C: move.w $9a(a2), d4
pub fn i_02a96c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A970;
    cpu.ir = 0x382A;
    let ad2 = cpu.a[2].wrapping_add(0x9A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02A970: sub.w d3, d1
pub fn i_02a970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A972;
    cpu.ir = 0x9243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $02A972: move.w d1, d0
pub fn i_02a972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A974;
    cpu.ir = 0x3001;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02A974: sub.w d4, d2
pub fn i_02a974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A976;
    cpu.ir = 0x9444;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $02A976: bne.w $2a980
pub fn i_02a976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A97A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2A980; return 10; }
    8
}
/// $02A97A: clr.w d1
pub fn i_02a97a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A97C;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02A97C: bra.w $2a982
pub fn i_02a97c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A980;
    cpu.ir = 0x6000;
    cpu.pc = 0x2A982;
    10
}
/// $02A980: muls.w d2, d1
pub fn i_02a980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A982;
    cpu.ir = 0xC3C2;
    let v1 = cpu.d[2] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[1] as u16 as i16 as i32)) as u32;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $02A982: cmpi.w #$ffc0, d2
pub fn i_02a982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A986;
    cpu.ir = 0xC42;
    let v1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFC0, Size::Word, false);
    8
}
/// $02A986: bge.w $2a992
pub fn i_02a986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A98A;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x2A992; return 10; }
    8
}
/// $02A98A: move.w #$ffc0, d2
pub fn i_02a98a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A98E;
    cpu.ir = 0x343C;
    let v1: u32 = 0xFFC0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02A98E: bra.w $2a99e
pub fn i_02a98e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A992;
    cpu.ir = 0x6000;
    cpu.pc = 0x2A99E;
    10
}
/// $02A992: cmpi.w #$40, d2
pub fn i_02a992(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A996;
    cpu.ir = 0xC42;
    let v1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $02A996: ble.w $2a99e
pub fn i_02a996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A99A;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x2A99E; return 10; }
    8
}
/// $02A99A: move.w #$40, d2
pub fn i_02a99a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A99E;
    cpu.ir = 0x343C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02A99E: rts
pub fn i_02a99e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A9A0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_SlopeToAnimIdx_02a9a0  ($02A9A0, 110 B, slug/helpers_0295a6.s)
/// $02A9A0: bra.w $2a9ce
pub fn i_02a9a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A9A4;
    cpu.ir = 0x6000;
    cpu.pc = 0x2A9CE;
    10
}
/// $02A9CE: jsr $2a958(pc)
pub fn i_02a9ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A9D2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2A9D2);
    cpu.pc = 0x2A958;
    16
}
/// $02A9D2: clr.w d0
pub fn i_02a9d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A9D4;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02A9D4: cmpi.w #$384, d1
pub fn i_02a9d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A9D8;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x384, Size::Word, false);
    8
}
/// $02A9D8: bge.w $2a9f6
pub fn i_02a9d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A9DC;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x2A9F6; return 10; }
    8
}
/// $02A9DC: cmpi.w #$fc7c, d1
pub fn i_02a9dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A9E0;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFC7C, Size::Word, false);
    8
}
/// $02A9E0: ble.w $2a9fc
pub fn i_02a9e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A9E4;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x2A9FC; return 10; }
    8
}
/// $02A9E4: cmpi.w #$5a, d1
pub fn i_02a9e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A9E8;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x5A, Size::Word, false);
    8
}
/// $02A9E8: bge.w $2aa02
pub fn i_02a9e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A9EC;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x2AA02; return 10; }
    8
}
/// $02A9EC: cmpi.w #$ffa6, d1
pub fn i_02a9ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A9F0;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFA6, Size::Word, false);
    8
}
/// $02A9F0: ble.w $2aa08
pub fn i_02a9f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A9F4;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x2AA08; return 10; }
    8
}
/// $02A9F4: rts
pub fn i_02a9f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A9F6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $02A9F6: move.w #$3, d0
pub fn i_02a9f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A9FA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02A9FA: bra.b $2a9f4
pub fn i_02a9fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A9FC;
    cpu.ir = 0x60F8;
    cpu.pc = 0x2A9F4;
    10
}
/// $02A9FC: move.w #$4, d0
pub fn i_02a9fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA00;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02AA00: bra.b $2a9f4
pub fn i_02aa00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA02;
    cpu.ir = 0x60F2;
    cpu.pc = 0x2A9F4;
    10
}
/// $02AA02: move.w #$1, d0
pub fn i_02aa02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA06;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02AA06: bra.b $2a9f4
pub fn i_02aa06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA08;
    cpu.ir = 0x60EC;
    cpu.pc = 0x2A9F4;
    10
}
/// $02AA08: move.w #$2, d0
pub fn i_02aa08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA0C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02AA0C: bra.b $2a9f4
pub fn i_02aa0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA0E;
    cpu.ir = 0x60E6;
    cpu.pc = 0x2A9F4;
    10
}
// ===== Slug_UpdateAnimKeepIdx_02aa0e  ($02AA0E, 16 B, slug/helpers_0295a6.s)
/// $02AA0E: move.w $94(a6), d0
pub fn i_02aa0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA12;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02AA12: movem.w d0, -(a7)
pub fn i_02aa12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA16;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $02AA16: jsr $2aa24(pc)
pub fn i_02aa16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA1A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2AA1A);
    cpu.pc = 0x2AA24;
    16
}
/// $02AA1A: movem.w (a7)+, d0
pub fn i_02aa1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA1E;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
// ===== SetTaskW_02aa1e  ($02AA1E, 6 B, kernel/task_setters_w.c)
/// $02AA1E: move.w d0, $94(a6)
pub fn i_02aa1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA22;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02AA22: rts
pub fn i_02aa22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA24;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_UpdateAnimAndChassis_02aa24  ($02AA24, 140 B, slug/helpers_0295a6.s)
/// $02AA24: jsr $283ca.l
pub fn i_02aa24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2AA2A);
    cpu.pc = 0x283CA;
    16
}
/// $02AA2A: cmpi.b #$0, $68(a6)
pub fn i_02aa2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA30;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $02AA30: beq.b $2aa3c
pub fn i_02aa30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA32;
    cpu.ir = 0x670A;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2AA3C; return 10; }
    8
}
/// $02AA32: cmpi.b #$1, $68(a6)
pub fn i_02aa32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA38;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $02AA38: bne.w $2aa42
pub fn i_02aa38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA3C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2AA42; return 10; }
    8
}
/// $02AA3C: move.b $68(a6), $85(a6)
pub fn i_02aa3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA42;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x85);
    bus.write8(ad4, v3 as u8);
    20
}
/// $02AA42: jsr $2a4a2(pc)
pub fn i_02aa42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA46;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2AA46);
    cpu.pc = 0x2A4A2;
    16
}
/// $02AA46: btst.b #$0, $13(a6)
pub fn i_02aa46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA4C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $02AA4C: bne.w $2aa54
pub fn i_02aa4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA50;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2AA54; return 10; }
    8
}
/// $02AA50: jsr $2fae4(pc)
pub fn i_02aa50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA54;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2AA54);
    cpu.pc = 0x2FAE4;
    16
}
/// $02AA54: jsr $2a9a0(pc)
pub fn i_02aa54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA58;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2AA58);
    cpu.pc = 0x2A9A0;
    16
}
/// $02AA58: move.w d0, $94(a6)
pub fn i_02aa58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA5C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02AA5C: move.b $8e(a6), d0
pub fn i_02aa5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA60;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x8E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $02AA60: clr.b d1
pub fn i_02aa60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA62;
    cpu.ir = 0x4201;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $02AA62: subi.b #$1, d0
pub fn i_02aa62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA66;
    cpu.ir = 0x400;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $02AA66: addx.b d1, d0
pub fn i_02aa66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA68;
    cpu.ir = 0xD101;
    let x = cpu.sr.get(StatusRegister::X) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let h2 = cpu.d[1] & 0xFF;
    let r = addx_with_flags(cpu, h1, h2, x, Size::Byte);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    4
}
/// $02AA68: move.b d0, $8e(a6)
pub fn i_02aa68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA6C;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8E);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02AA6C: movea.l $74(a6), a2
pub fn i_02aa6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA70;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $02AA70: movea.l $78(a6), a3
pub fn i_02aa70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA74;
    cpu.ir = 0x266E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read32(ad2);
    cpu.a[3] = v3;
    16
}
/// $02AA74: move.w $9a(a2), d1
pub fn i_02aa74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA78;
    cpu.ir = 0x322A;
    let ad2 = cpu.a[2].wrapping_add(0x9A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02AA78: move.w $9a(a3), d0
pub fn i_02aa78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA7C;
    cpu.ir = 0x302B;
    let ad2 = cpu.a[3].wrapping_add(0x9A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02AA7C: clr.w d3
pub fn i_02aa7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA7E;
    cpu.ir = 0x4243;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02AA7E: sub.w d0, d1
pub fn i_02aa7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA80;
    cpu.ir = 0x9240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $02AA80: bge.w $2aa8a
pub fn i_02aa80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA84;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x2AA8A; return 10; }
    8
}
/// $02AA84: move.w $9a(a2), d0
pub fn i_02aa84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA88;
    cpu.ir = 0x302A;
    let ad2 = cpu.a[2].wrapping_add(0x9A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02AA88: moveq #$ff, d3
pub fn i_02aa88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA8A;
    cpu.ir = 0x76FF;
    cpu.d[3] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $02AA8A: asl.w #$5, d1
pub fn i_02aa8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA8C;
    cpu.ir = 0xEB41;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 0, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $02AA8C: move.w $22(a2), d2
pub fn i_02aa8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA90;
    cpu.ir = 0x342A;
    let ad2 = cpu.a[2].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02AA90: sub.w $22(a3), d2
pub fn i_02aa90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA94;
    cpu.ir = 0x946B;
    let ad2 = cpu.a[3].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $02AA94: beq.w $2aaa0
pub fn i_02aa94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA98;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2AAA0; return 10; }
    8
}
/// $02AA98: ext.l d1
pub fn i_02aa98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA9A;
    cpu.ir = 0x48C1;
    cpu.d[1] = cpu.d[1] as i16 as i32 as u32;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $02AA9A: divs.w d2, d1
pub fn i_02aa9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AA9C;
    cpu.ir = 0x83C2;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set(StatusRegister::C, false);
    let dv = v1 as u16 as i16 as i32;
    if dv == 0 { cpu.enter_group2_exception(bus, Exception::DivideByZero); return 38; }
    let dd = cpu.d[1] as i32; let q = dd.wrapping_div(dv); let rm = dd.wrapping_rem(dv);
    if !(-32768..=32767).contains(&q) { cpu.sr.set(StatusRegister::V, true); cpu.sr.set(StatusRegister::N, true); cpu.sr.set(StatusRegister::Z, false); return 158; }
    cpu.d[1] = ((rm as u32 & 0xFFFF) << 16) | (q as u32 & 0xFFFF);
    cpu.sr.set_nz(q as u32 & 0xFFFF, Size::Word); cpu.sr.set(StatusRegister::V, false);
    158
}
/// $02AA9C: bra.w $2aaa2
pub fn i_02aa9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AAA0;
    cpu.ir = 0x6000;
    cpu.pc = 0x2AAA2;
    10
}
/// $02AAA0: clr.w d1
pub fn i_02aaa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AAA2;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02AAA2: asr.w #$1, d1
pub fn i_02aaa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AAA4;
    cpu.ir = 0xE241;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02AAA4: eor.w d3, d1
pub fn i_02aaa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AAA6;
    cpu.ir = 0xB741;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 ^ cpu.d[3]) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02AAA6: sub.w d3, d1
pub fn i_02aaa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AAA8;
    cpu.ir = 0x9243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $02AAA8: add.w d0, d1
pub fn i_02aaa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AAAA;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $02AAAA: move.w d1, $24(a6)
pub fn i_02aaaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AAAE;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02AAAE: rts
pub fn i_02aaae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AAB0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_GaugeTick_02aab0  ($02AAB0, 10 B, slug/helpers_0295a6.s)
/// $02AAB0: move.b $90(a6), d0
pub fn i_02aab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AAB4;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $02AAB4: moveq #$0, d1
pub fn i_02aab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AAB6;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $02AAB6: subq.b #$1, d0
pub fn i_02aab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AAB8;
    cpu.ir = 0x5300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $02AAB8: addx.b d1, d0
pub fn i_02aab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AABA;
    cpu.ir = 0xD101;
    let x = cpu.sr.get(StatusRegister::X) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let h2 = cpu.d[1] & 0xFF;
    let r = addx_with_flags(cpu, h1, h2, x, Size::Byte);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    4
}
// ===== SetTaskB_02aaba  ($02AABA, 6 B, kernel/task_setters_b.c)
/// $02AABA: move.b d0, $90(a6)
pub fn i_02aaba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AABE;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02AABE: rts
pub fn i_02aabe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AAC0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_CanFire_02aac0  ($02AAC0, 30 B, slug/helpers_0295a6.s)
/// $02AAC0: cmpi.b #$0, $90(a6)
pub fn i_02aac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AAC6;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $02AAC6: beq.w $2aade
pub fn i_02aac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AACA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2AADE; return 10; }
    8
}
/// $02AACA: move.b #$2, d0
pub fn i_02aaca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AACE;
    cpu.ir = 0x103C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $02AACE: cmp.b $8f(a6), d0
pub fn i_02aace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AAD2;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x8F);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $02AAD2: bls.w $2aade
pub fn i_02aad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AAD6;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x2AADE; return 10; }
    8
}
/// $02AAD6: tst.b $8e(a6)
pub fn i_02aad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AADA;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x8E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $02AADA: beq.w $2aae4
pub fn i_02aada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AADE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2AAE4; return 10; }
    8
}
// ===== ClearXN_02aade  ($02AADE, 6 B, thunks/ccr_helpers.c)
/// $02AADE: andi.b #$ee, ccr
pub fn i_02aade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AAE2;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02AAE2: rts
pub fn i_02aae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AAE4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrPcThunk_02aae4  ($02AAE4, 6 B, thunks/jsr_pc_thunks.c)
/// $02AAE4: jsr $2ab86(pc)
pub fn i_02aae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AAE8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2AAE8);
    cpu.pc = 0x2AB86;
    16
}
/// $02AAE8: rts
pub fn i_02aae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AAEA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_JmpInput5CDA8_02aaea  ($02AAEA, 6 B, slug/helpers_0295a6.s)
/// $02AAEA: jmp $5cda8.l
pub fn i_02aaea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AAF0;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x5CDA8;
    8
}
// ===== Slug_InputDirByLayoutA_02aaf0  ($02AAF0, 56 B, slug/helpers_0295a6.s)
/// $02AAF0: cmpi.b #$0, $106f2a.l
pub fn i_02aaf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AAF8;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106F2A) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    20
}
/// $02AAF8: bne.w $2ab06
pub fn i_02aaf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AAFC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2AB06; return 10; }
    8
}
/// $02AAFC: jsr $5cdb4.l
pub fn i_02aafc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2AB02);
    cpu.pc = 0x5CDB4;
    16
}
/// $02AB02: bra.w $2ab22
pub fn i_02ab02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB06;
    cpu.ir = 0x6000;
    cpu.pc = 0x2AB22;
    10
}
/// $02AB06: cmpi.b #$4, $106f2a.l
pub fn i_02ab06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB0E;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106F2A) as u32;
    sub_with_flags(cpu, v2, 0x4, Size::Byte, false);
    20
}
/// $02AB0E: bne.w $2ab1c
pub fn i_02ab0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB12;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2AB1C; return 10; }
    8
}
/// $02AB12: jsr $5cdb4.l
pub fn i_02ab12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2AB18);
    cpu.pc = 0x5CDB4;
    16
}
/// $02AB18: bra.w $2ab22
pub fn i_02ab18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB1C;
    cpu.ir = 0x6000;
    cpu.pc = 0x2AB22;
    10
}
/// $02AB1C: jsr $5ceec.l
pub fn i_02ab1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB22;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2AB22);
    cpu.pc = 0x5CEEC;
    16
}
/// $02AB22: bcs.w $2ab2e
pub fn i_02ab22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB26;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2AB2E; return 10; }
    8
}
/// $02AB26: moveq #$0, d0
pub fn i_02ab26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB28;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
// ===== ClearXN_02ab28  ($02AB28, 6 B, thunks/ccr_helpers.c)
/// $02AB28: andi.b #$ee, ccr
pub fn i_02ab28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB2C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02AB2C: rts
pub fn i_02ab2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB2E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_InputDirA_02ab2e  ($02AB2E, 8 B, slug/helpers_0295a6.s)
/// $02AB2E: jsr $5d5b6.l
pub fn i_02ab2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB34;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2AB34);
    cpu.pc = 0x5D5B6;
    16
}
/// $02AB34: addq.w #$1, d0
pub fn i_02ab34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB36;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
// ===== SetXN_02ab36  ($02AB36, 6 B, thunks/ccr_helpers.c)
/// $02AB36: ori.b #$11, ccr
pub fn i_02ab36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB3A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02AB3A: rts
pub fn i_02ab3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB3C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_InputDirByLayoutB_02ab3c  ($02AB3C, 54 B, slug/helpers_0295a6.s)
/// $02AB3C: cmpi.b #$0, $106f2a.l
pub fn i_02ab3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB44;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106F2A) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    20
}
/// $02AB44: bne.w $2ab52
pub fn i_02ab44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB48;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2AB52; return 10; }
    8
}
/// $02AB48: jsr $5cdb4.l
pub fn i_02ab48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2AB4E);
    cpu.pc = 0x5CDB4;
    16
}
/// $02AB4E: bra.w $2ab6c
pub fn i_02ab4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB52;
    cpu.ir = 0x6000;
    cpu.pc = 0x2AB6C;
    10
}
/// $02AB52: cmpi.b #$4, $106f2a.l
pub fn i_02ab52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB5A;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106F2A) as u32;
    sub_with_flags(cpu, v2, 0x4, Size::Byte, false);
    20
}
/// $02AB5A: bne.w $2ab68
pub fn i_02ab5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB5E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2AB68; return 10; }
    8
}
/// $02AB5E: jsr $5cdb4.l
pub fn i_02ab5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB64;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2AB64);
    cpu.pc = 0x5CDB4;
    16
}
/// $02AB64: bra.w $2ab6c
pub fn i_02ab64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB68;
    cpu.ir = 0x6000;
    cpu.pc = 0x2AB6C;
    10
}
/// $02AB68: jsr $2a374(pc)
pub fn i_02ab68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB6C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2AB6C);
    cpu.pc = 0x2A374;
    16
}
/// $02AB6C: bcs.w $2ab78
pub fn i_02ab6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB70;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2AB78; return 10; }
    8
}
/// $02AB70: moveq #$0, d0
pub fn i_02ab70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB72;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
// ===== ClearXN_02ab72  ($02AB72, 6 B, thunks/ccr_helpers.c)
/// $02AB72: andi.b #$ee, ccr
pub fn i_02ab72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB76;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02AB76: rts
pub fn i_02ab76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB78;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_InputDirB_02ab78  ($02AB78, 8 B, slug/helpers_0295a6.s)
/// $02AB78: jsr $5d5b6.l
pub fn i_02ab78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2AB7E);
    cpu.pc = 0x5D5B6;
    16
}
/// $02AB7E: addq.w #$1, d0
pub fn i_02ab7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB80;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
// ===== SetXN_02ab80  ($02AB80, 6 B, thunks/ccr_helpers.c)
/// $02AB80: ori.b #$11, ccr
pub fn i_02ab80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB84;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02AB84: rts
pub fn i_02ab84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB86;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_InputFireByLayout_02ab86  ($02AB86, 44 B, slug/helpers_0295a6.s)
/// $02AB86: cmpi.b #$4, $106f2a.l
pub fn i_02ab86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB8E;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106F2A) as u32;
    sub_with_flags(cpu, v2, 0x4, Size::Byte, false);
    20
}
/// $02AB8E: bne.w $2ab9c
pub fn i_02ab8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB92;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2AB9C; return 10; }
    8
}
/// $02AB92: jsr $5cda8.l
pub fn i_02ab92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2AB98);
    cpu.pc = 0x5CDA8;
    16
}
/// $02AB98: bra.w $2abb8
pub fn i_02ab98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AB9C;
    cpu.ir = 0x6000;
    cpu.pc = 0x2ABB8;
    10
}
/// $02AB9C: cmpi.b #$0, $106f2a.l
pub fn i_02ab9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ABA4;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106F2A) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    20
}
/// $02ABA4: bne.w $2abb2
pub fn i_02aba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ABA8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2ABB2; return 10; }
    8
}
/// $02ABA8: jsr $5cdc0.l
pub fn i_02aba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ABAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2ABAE);
    cpu.pc = 0x5CDC0;
    16
}
/// $02ABAE: bra.w $2abb8
pub fn i_02abae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ABB2;
    cpu.ir = 0x6000;
    cpu.pc = 0x2ABB8;
    10
}
// ===== JsrAbsThunk_02abb2  ($02ABB2, 8 B, thunks/jsr_abs_thunks.c)
/// $02ABB2: jsr $5cdfc.l
pub fn i_02abb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ABB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2ABB8);
    cpu.pc = 0x5CDFC;
    16
}
/// $02ABB8: rts
pub fn i_02abb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ABBA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_02abba  ($02ABBA, 6 B, thunks/ccr_helpers.c)
/// $02ABBA: andi.b #$ee, ccr
pub fn i_02abba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ABBE;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02ABBE: rts
pub fn i_02abbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ABC0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_02abc0  ($02ABC0, 6 B, thunks/ccr_helpers.c)
/// $02ABC0: andi.b #$ee, ccr
pub fn i_02abc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ABC4;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02ABC4: rts
pub fn i_02abc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ABC6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpAbsThunk_02abc6  ($02ABC6, 6 B, thunks/jmp_abs_thunks.c)
/// $02ABC6: jmp $5cef8.l
pub fn i_02abc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ABCC;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x5CEF8;
    8
}
// ===== ClearXN_02abcc  ($02ABCC, 6 B, thunks/ccr_helpers.c)
/// $02ABCC: andi.b #$ee, ccr
pub fn i_02abcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ABD0;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02ABD0: rts
pub fn i_02abd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ABD2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_TestBit4Field8D_02abd2  ($02ABD2, 24 B, slug/helpers_0295a6.s)
/// $02ABD2: lea.l $100580.l, a0
pub fn i_02abd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ABD8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100580;
    20
}
/// $02ABD8: btst.b #$4, $8d(a0)
pub fn i_02abd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ABDE;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x8D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $02ABDE: bne.w $2abea
pub fn i_02abde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ABE2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2ABEA; return 10; }
    8
}
/// $02ABE2: andi.b #$ee, ccr
pub fn i_02abe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ABE6;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02ABE6: bra.w $2abee
pub fn i_02abe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ABEA;
    cpu.ir = 0x6000;
    cpu.pc = 0x2ABEE;
    10
}
// ===== SetXN_02abea  ($02ABEA, 6 B, thunks/ccr_helpers.c)
/// $02ABEA: ori.b #$11, ccr
pub fn i_02abea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ABEE;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02ABEE: rts
pub fn i_02abee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ABF0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_TestBit1Field8D_02abf0  ($02ABF0, 24 B, slug/helpers_0295a6.s)
/// $02ABF0: lea.l $100580.l, a0
pub fn i_02abf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ABF6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100580;
    20
}
/// $02ABF6: btst.b #$1, $8d(a0)
pub fn i_02abf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ABFC;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x8D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $02ABFC: bne.w $2ac08
pub fn i_02abfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC00;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2AC08; return 10; }
    8
}
/// $02AC00: andi.b #$ee, ccr
pub fn i_02ac00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC04;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02AC04: bra.w $2ac0c
pub fn i_02ac04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC08;
    cpu.ir = 0x6000;
    cpu.pc = 0x2AC0C;
    10
}
// ===== SetXN_02ac08  ($02AC08, 6 B, thunks/ccr_helpers.c)
/// $02AC08: ori.b #$11, ccr
pub fn i_02ac08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC0C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02AC0C: rts
pub fn i_02ac0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC0E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_IsRiddenByPlayer_02ac0e  ($02AC0E, 50 B, slug/helpers_0295a6.s)
/// $02AC0E: cmpa.l #$100440, a0
pub fn i_02ac0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC14;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0x100440;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02AC14: bne.w $2ac1e
pub fn i_02ac14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC18;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2AC1E; return 10; }
    8
}
/// $02AC18: moveq #$1, d0
pub fn i_02ac18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC1A;
    cpu.ir = 0x7001;
    cpu.d[0] = 0x1;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $02AC1A: bra.w $2ac32
pub fn i_02ac1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC1E;
    cpu.ir = 0x6000;
    cpu.pc = 0x2AC32;
    10
}
/// $02AC1E: cmpa.l #$1004e0, a0
pub fn i_02ac1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC24;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0x1004E0;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02AC24: bne.w $2ac2e
pub fn i_02ac24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC28;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2AC2E; return 10; }
    8
}
/// $02AC28: moveq #$2, d0
pub fn i_02ac28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC2A;
    cpu.ir = 0x7002;
    cpu.d[0] = 0x2;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $02AC2A: bra.w $2ac32
pub fn i_02ac2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC2E;
    cpu.ir = 0x6000;
    cpu.pc = 0x2AC32;
    10
}
/// $02AC2E: bra.w $2ac46
pub fn i_02ac2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC32;
    cpu.ir = 0x6000;
    cpu.pc = 0x2AC46;
    10
}
/// $02AC32: lea.l $100580.l, a1
pub fn i_02ac32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC38;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x100580;
    20
}
/// $02AC38: cmp.b $6d(a1), d0
pub fn i_02ac38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC3C;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x6D);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $02AC3C: bne.w $2ac46
pub fn i_02ac3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC40;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2AC46; return 10; }
    8
}
// ===== JsrPcThunk_02ac40  ($02AC40, 6 B, thunks/jsr_pc_thunks.c)
/// $02AC40: jsr $2ac4c(pc)
pub fn i_02ac40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC44;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2AC44);
    cpu.pc = 0x2AC4C;
    16
}
/// $02AC44: rts
pub fn i_02ac44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC46;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_02ac46  ($02AC46, 6 B, thunks/ccr_helpers.c)
/// $02AC46: andi.b #$ee, ccr
pub fn i_02ac46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC4A;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02AC4A: rts
pub fn i_02ac4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC4C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_TestField100609_02ac4c  ($02AC4C, 10 B, slug/helpers_0295a6.s)
/// $02AC4C: move.b $100609.l, d0
pub fn i_02ac4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC52;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x100609) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $02AC52: bne.w $2ac5c
pub fn i_02ac52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC56;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2AC5C; return 10; }
    8
}
// ===== ClearXN_02ac56  ($02AC56, 6 B, thunks/ccr_helpers.c)
/// $02AC56: andi.b #$ee, ccr
pub fn i_02ac56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC5A;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02AC5A: rts
pub fn i_02ac5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC5C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_CopyField68_02ac5c  ($02AC5C, 8 B, slug/helpers_0295a6.s)
/// $02AC5C: move.b $68(a0), d2
pub fn i_02ac5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC60;
    cpu.ir = 0x1428;
    let ad2 = cpu.a[0].wrapping_add(0x68);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $02AC60: move.b d2, $68(a6)
pub fn i_02ac60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC64;
    cpu.ir = 0x1D42;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== SetXN_02ac64  ($02AC64, 6 B, thunks/ccr_helpers.c)
/// $02AC64: ori.b #$11, ccr
pub fn i_02ac64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC68;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02AC68: rts
pub fn i_02ac68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC6A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_TestField100609B_02ac6a  ($02AC6A, 10 B, slug/helpers_0295a6.s)
/// $02AC6A: move.b $100609.l, d0
pub fn i_02ac6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC70;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x100609) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $02AC70: bne.w $2ac7a
pub fn i_02ac70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC74;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2AC7A; return 10; }
    8
}
// ===== ClearXN_02ac74  ($02AC74, 6 B, thunks/ccr_helpers.c)
/// $02AC74: andi.b #$ee, ccr
pub fn i_02ac74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC78;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02AC78: rts
pub fn i_02ac78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC7A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_02ac7a  ($02AC7A, 6 B, thunks/ccr_helpers.c)
/// $02AC7A: ori.b #$11, ccr
pub fn i_02ac7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC7E;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02AC7E: rts
pub fn i_02ac7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC80;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_ConsumeField89_02ac80  ($02AC80, 28 B, slug/helpers_0295a6.s)
/// $02AC80: cmpi.b #$0, $89(a6)
pub fn i_02ac80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC86;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x89);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $02AC86: bne.w $2ac98
pub fn i_02ac86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC8A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2AC98; return 10; }
    8
}
/// $02AC8A: move.b #$ff, $6d(a6)
pub fn i_02ac8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC90;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x6D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02AC90: ori.b #$11, ccr
pub fn i_02ac90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC94;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02AC94: bra.w $2aca0
pub fn i_02ac94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC98;
    cpu.ir = 0x6000;
    cpu.pc = 0x2ACA0;
    10
}
/// $02AC98: clr.b $89(a6)
pub fn i_02ac98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AC9C;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x89);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
// ===== ClearXN_02ac9c  ($02AC9C, 6 B, thunks/ccr_helpers.c)
/// $02AC9C: andi.b #$ee, ccr
pub fn i_02ac9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ACA0;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02ACA0: rts
pub fn i_02aca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ACA2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_SetField89_02aca2  ($02ACA2, 22 B, slug/helpers_0295a6.s)
/// $02ACA2: movem.l a0, -(a7)
pub fn i_02aca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ACA6;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $02ACA6: lea.l $100580.l, a0
pub fn i_02aca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ACAC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100580;
    20
}
/// $02ACAC: move.b #$ff, $89(a0)
pub fn i_02acac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ACB2;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x89);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02ACB2: movem.l (a7)+, a0
pub fn i_02acb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ACB6;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $02ACB6: rts
pub fn i_02acb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ACB8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_CheckPlayersNear_02acb8  ($02ACB8, 56 B, slug/helpers_0295a6.s)
/// $02ACB8: cmpi.b #$0, $106ed3.l
pub fn i_02acb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ACC0;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ED3) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    20
}
/// $02ACC0: bne.w $2acc8
pub fn i_02acc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ACC4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2ACC8; return 10; }
    8
}
/// $02ACC4: bra.w $2acf0
pub fn i_02acc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ACC8;
    cpu.ir = 0x6000;
    cpu.pc = 0x2ACF0;
    10
}
/// $02ACC8: jsr $3eecc.l
pub fn i_02acc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ACCE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2ACCE);
    cpu.pc = 0x3EECC;
    16
}
/// $02ACCE: bcc.w $2acdc
pub fn i_02acce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ACD2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2ACDC; return 10; }
    8
}
/// $02ACD2: jsr $32d6c.l
pub fn i_02acd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ACD8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2ACD8);
    cpu.pc = 0x32D6C;
    16
}
/// $02ACD8: bcs.w $2acf6
pub fn i_02acd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ACDC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2ACF6; return 10; }
    8
}
/// $02ACDC: jsr $3ef14.l
pub fn i_02acdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ACE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2ACE2);
    cpu.pc = 0x3EF14;
    16
}
/// $02ACE2: bcc.w $2acf0
pub fn i_02ace2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ACE6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2ACF0; return 10; }
    8
}
/// $02ACE6: jsr $32d6c.l
pub fn i_02ace6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ACEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2ACEC);
    cpu.pc = 0x32D6C;
    16
}
/// $02ACEC: bcs.w $2acf6
pub fn i_02acec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ACF0;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2ACF6; return 10; }
    8
}
// ===== ClearXN_02acf0  ($02ACF0, 6 B, thunks/ccr_helpers.c)
/// $02ACF0: andi.b #$ee, ccr
pub fn i_02acf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ACF4;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02ACF4: rts
pub fn i_02acf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ACF6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_02acf6  ($02ACF6, 6 B, thunks/ccr_helpers.c)
/// $02ACF6: ori.b #$11, ccr
pub fn i_02acf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ACFA;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02ACFA: rts
pub fn i_02acfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ACFC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_IsAlive_02acfc  ($02ACFC, 46 B, slug/helpers_0295a6.s)
/// $02ACFC: lea.l $100580.l, a0
pub fn i_02acfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AD02;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100580;
    20
}
/// $02AD02: cmpi.l #$ffffffff, (a0)
pub fn i_02ad02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AD08;
    cpu.ir = 0xC90;
    let v2 = bus.read32(cpu.a[0]);
    sub_with_flags(cpu, v2, 0xFFFFFFFF, Size::Long, false);
    16
}
/// $02AD08: beq.w $2ad30
pub fn i_02ad08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AD0C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2AD30; return 10; }
    8
}
/// $02AD0C: cmpi.l #$52a, (a0)
pub fn i_02ad0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AD12;
    cpu.ir = 0xC90;
    let v2 = bus.read32(cpu.a[0]);
    sub_with_flags(cpu, v2, 0x52A, Size::Long, false);
    16
}
/// $02AD12: beq.w $2ad30
pub fn i_02ad12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AD16;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2AD30; return 10; }
    8
}
/// $02AD16: cmpi.l #$400, (a0)
pub fn i_02ad16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AD1C;
    cpu.ir = 0xC90;
    let v2 = bus.read32(cpu.a[0]);
    sub_with_flags(cpu, v2, 0x400, Size::Long, false);
    16
}
/// $02AD1C: beq.w $2ad30
pub fn i_02ad1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AD20;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2AD30; return 10; }
    8
}
/// $02AD20: cmpi.l #$2ae3e, (a0)
pub fn i_02ad20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AD26;
    cpu.ir = 0xC90;
    let v2 = bus.read32(cpu.a[0]);
    sub_with_flags(cpu, v2, 0x2AE3E, Size::Long, false);
    16
}
/// $02AD26: beq.w $2ad30
pub fn i_02ad26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AD2A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2AD30; return 10; }
    8
}
// ===== SetXN_02ad2a  ($02AD2A, 6 B, thunks/ccr_helpers.c)
/// $02AD2A: ori.b #$11, ccr
pub fn i_02ad2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AD2E;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02AD2E: rts
pub fn i_02ad2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AD30;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_02ad30  ($02AD30, 6 B, thunks/ccr_helpers.c)
/// $02AD30: andi.b #$ee, ccr
pub fn i_02ad30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AD34;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02AD34: rts
pub fn i_02ad34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AD36;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_GetRiderAndField98_02ad36  ($02AD36, 40 B, slug/helpers_0295a6.s)
/// $02AD36: jsr $2acfc(pc)
pub fn i_02ad36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AD3A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2AD3A);
    cpu.pc = 0x2ACFC;
    16
}
/// $02AD3A: bcc.w $2ad64
pub fn i_02ad3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AD3E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2AD64; return 10; }
    8
}
/// $02AD3E: move.b $85(a0), d0
pub fn i_02ad3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AD42;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x85);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $02AD42: cmp.b $6e(a6), d0
pub fn i_02ad42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AD46;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x6E);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $02AD46: bne.w $2ad64
pub fn i_02ad46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AD4A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2AD64; return 10; }
    8
}
/// $02AD4A: move.l $98(a0), d2
pub fn i_02ad4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AD4E;
    cpu.ir = 0x2428;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02AD4E: cmpi.l #$ffffffff, d2
pub fn i_02ad4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AD54;
    cpu.ir = 0xC82;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0xFFFFFFFF, Size::Long, false);
    8
}
/// $02AD54: bne.w $2ad5e
pub fn i_02ad54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AD58;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2AD5E; return 10; }
    8
}
/// $02AD58: move.l #$10000, d2
pub fn i_02ad58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AD5E;
    cpu.ir = 0x243C;
    let v1: u32 = 0x10000;
    cpu.d[2] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
// ===== SetXN_02ad5e  ($02AD5E, 6 B, thunks/ccr_helpers.c)
/// $02AD5E: ori.b #$11, ccr
pub fn i_02ad5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AD62;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02AD62: rts
pub fn i_02ad62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AD64;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_NoRider_02ad64  ($02AD64, 6 B, slug/helpers_0295a6.s)
/// $02AD64: move.b #$ff, d0
pub fn i_02ad64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AD68;
    cpu.ir = 0x103C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $02AD68: clr.l d2
pub fn i_02ad68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AD6A;
    cpu.ir = 0x4282;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[2] = r2;
    4
}
// ===== ClearXN_02ad6a  ($02AD6A, 6 B, thunks/ccr_helpers.c)
/// $02AD6A: andi.b #$ee, ccr
pub fn i_02ad6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AD6E;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02AD6E: rts
pub fn i_02ad6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AD70;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_SpawnRiderMarker_02ad70  ($02AD70, 102 B, slug/helpers_0295a6.s)
/// $02AD70: jsr $2acfc(pc)
pub fn i_02ad70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AD74;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2AD74);
    cpu.pc = 0x2ACFC;
    16
}
/// $02AD74: bcc.w $2add4
pub fn i_02ad74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AD78;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2ADD4; return 10; }
    8
}
/// $02AD78: move.b $85(a0), d0
pub fn i_02ad78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AD7C;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x85);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $02AD7C: cmpi.b #$0, d0
pub fn i_02ad7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AD80;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $02AD80: bne.w $2ada6
pub fn i_02ad80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AD84;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2ADA6; return 10; }
    8
}
/// $02AD84: lea.l $28dd3c.l, a1
pub fn i_02ad84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AD8A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x28DD3C;
    20
}
/// $02AD8A: jsr $4ae.l
pub fn i_02ad8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AD90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2AD90);
    cpu.pc = 0x4AE;
    16
}
/// $02AD90: jsr $5dd02.l
pub fn i_02ad90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AD96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2AD96);
    cpu.pc = 0x5DD02;
    16
}
/// $02AD96: move.w #$30, $22(a0)
pub fn i_02ad96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AD9C;
    cpu.ir = 0x317C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02AD9C: move.w #$174, $24(a0)
pub fn i_02ad9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ADA2;
    cpu.ir = 0x317C;
    let v1: u32 = 0x174;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02ADA2: bra.w $2add4
pub fn i_02ada2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ADA6;
    cpu.ir = 0x6000;
    cpu.pc = 0x2ADD4;
    10
}
/// $02ADA6: move.b $85(a0), d0
pub fn i_02ada6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ADAA;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x85);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $02ADAA: cmpi.b #$1, d0
pub fn i_02adaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ADAE;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $02ADAE: bne.w $2add4
pub fn i_02adae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ADB2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2ADD4; return 10; }
    8
}
/// $02ADB2: lea.l $28dd3c.l, a1
pub fn i_02adb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ADB8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x28DD3C;
    20
}
/// $02ADB8: jsr $4ae.l
pub fn i_02adb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ADBE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2ADBE);
    cpu.pc = 0x4AE;
    16
}
/// $02ADBE: jsr $5dd02.l
pub fn i_02adbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ADC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2ADC4);
    cpu.pc = 0x5DD02;
    16
}
/// $02ADC4: move.w #$b8, $22(a0)
pub fn i_02adc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ADCA;
    cpu.ir = 0x317C;
    let v1: u32 = 0xB8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02ADCA: move.w #$174, $24(a0)
pub fn i_02adca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ADD0;
    cpu.ir = 0x317C;
    let v1: u32 = 0x174;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02ADD0: bra.w $2add4
pub fn i_02add0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ADD4;
    cpu.ir = 0x6000;
    cpu.pc = 0x2ADD4;
    10
}
/// $02ADD4: rts
pub fn i_02add4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ADD6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_SpawnAtBossArena_02add6  ($02ADD6, 96 B, slug/helpers_0295a6.s)
/// $02ADD6: move.w #$40, $22(a6)
pub fn i_02add6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ADDC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02ADDC: move.w #$171, $24(a6)
pub fn i_02addc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ADE2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x171;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02ADE2: move.w #$171, $82(a6)
pub fn i_02ade2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ADE8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x171;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02ADE8: move.w #$3, d1
pub fn i_02ade8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ADEC;
    cpu.ir = 0x323C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02ADEC: jsr $2a1aa(pc)
pub fn i_02adec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ADF0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2ADF0);
    cpu.pc = 0x2A1AA;
    16
}
/// $02ADF0: ori.w #$2, $38(a6)
pub fn i_02adf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ADF6;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $02ADF6: lea.l $2792b0.l, a0
pub fn i_02adf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ADFC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2792B0;
    20
}
/// $02ADFC: jsr $28cd4.l
pub fn i_02adfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AE02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2AE02);
    cpu.pc = 0x28CD4;
    16
}
/// $02AE02: clr.w $28(a6)
pub fn i_02ae02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AE06;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02AE06: clr.w $2a(a6)
pub fn i_02ae06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AE0A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02AE0A: clr.w $2c(a6)
pub fn i_02ae0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AE0E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02AE0E: clr.w $2e(a6)
pub fn i_02ae0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AE12;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02AE12: clr.b $26(a6)
pub fn i_02ae12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AE16;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x26);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $02AE16: clr.b $27(a6)
pub fn i_02ae16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AE1A;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x27);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $02AE1A: clr.b $8e(a6)
pub fn i_02ae1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AE1E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x8E);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $02AE1E: bset.b #$6, $13(a6)
pub fn i_02ae1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AE24;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02AE24: jsr $2aa0e(pc)
pub fn i_02ae24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AE28;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2AE28);
    cpu.pc = 0x2AA0E;
    16
}
/// $02AE28: jsr $2a878(pc)
pub fn i_02ae28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AE2C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2AE2C);
    cpu.pc = 0x2A878;
    16
}
/// $02AE2C: jsr $2a478(pc)
pub fn i_02ae2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AE30;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2AE30);
    cpu.pc = 0x2A478;
    16
}
/// $02AE30: jsr $28d70.l
pub fn i_02ae30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AE36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2AE36);
    cpu.pc = 0x28D70;
    16
}
// ===== SetTaskHandler_02ae36  ($02AE36, 8 B, kernel/task_handlers.c)
/// $02AE36: lea.l $2d02e(pc), a1
pub fn i_02ae36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AE3A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D02E;
    16
}
/// $02AE3A: move.l a1, (a6)
pub fn i_02ae3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AE3C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02AE3C: rts
pub fn i_02ae3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AE3E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_DeadHandler_02ae3e  ($02AE3E, 18 B, slug/states_02ae3e.s)
/// $02AE3E: move.l #$ffffffff, $48(a6)
pub fn i_02ae3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AE46;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02AE46: move.l #$ffffffff, $60(a6)
pub fn i_02ae46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AE4E;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02AE4E: rts
pub fn i_02ae4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AE50;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_SpawnDrop_02ae50  ($02AE50, 64 B, slug/states_02ae3e.s)
/// $02AE50: jsr $2acfc(pc)
pub fn i_02ae50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AE54;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2AE54);
    cpu.pc = 0x2ACFC;
    16
}
/// $02AE54: bcs.w $2ae8e
pub fn i_02ae54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AE58;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2AE8E; return 10; }
    8
}
/// $02AE58: lea.l $2ae90(pc), a0
pub fn i_02ae58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AE5C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2AE90;
    16
}
/// $02AE5C: moveq #$0, d0
pub fn i_02ae5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AE5E;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $02AE5E: move.b $98(a6), d0
pub fn i_02ae5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AE62;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $02AE62: cmpi.b #$5, d0
pub fn i_02ae62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AE66;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x5, Size::Byte, false);
    8
}
/// $02AE66: bcs.w $2ae6e
pub fn i_02ae66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AE6A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2AE6E; return 10; }
    8
}
/// $02AE6A: move.b #$4, d0
pub fn i_02ae6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AE6E;
    cpu.ir = 0x103C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $02AE6E: lsl.w #$2, d0
pub fn i_02ae6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AE70;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02AE70: movea.l (a0, d0.w), a1
pub fn i_02ae70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AE74;
    cpu.ir = 0x2270;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    18
}
/// $02AE74: lea.l $100580.l, a0
pub fn i_02ae74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AE7A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100580;
    20
}
/// $02AE7A: move.l a1, (a0)
pub fn i_02ae7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AE7C;
    cpu.ir = 0x2089;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02AE7C: jsr $5dd02.l
pub fn i_02ae7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AE82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2AE82);
    cpu.pc = 0x5DD02;
    16
}
/// $02AE82: move.w $24(a6), $82(a0)
pub fn i_02ae82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AE88;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x82);
    bus.write16(ad4, v3 as u16);
    20
}
/// $02AE88: jmp $518.l
pub fn i_02ae88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AE8E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $02AE8E: rts
pub fn i_02ae8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AE90;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_SpawnDropB_02aea4  ($02AEA4, 62 B, slug/states_02ae3e.s)
/// $02AEA4: jsr $2acfc(pc)
pub fn i_02aea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AEA8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2AEA8);
    cpu.pc = 0x2ACFC;
    16
}
/// $02AEA8: bcs.w $2aedc
pub fn i_02aea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AEAC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2AEDC; return 10; }
    8
}
/// $02AEAC: lea.l $2aee4(pc), a0
pub fn i_02aeac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AEB0;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2AEE4;
    16
}
/// $02AEB0: moveq #$0, d0
pub fn i_02aeb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AEB2;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $02AEB2: move.b $98(a6), d0
pub fn i_02aeb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AEB6;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $02AEB6: cmpi.b #$5, d0
pub fn i_02aeb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AEBA;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x5, Size::Byte, false);
    8
}
/// $02AEBA: bcs.w $2aec2
pub fn i_02aeba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AEBE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2AEC2; return 10; }
    8
}
/// $02AEBE: move.b #$4, d0
pub fn i_02aebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AEC2;
    cpu.ir = 0x103C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $02AEC2: lsl.w #$2, d0
pub fn i_02aec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AEC4;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02AEC4: movea.l (a0, d0.w), a1
pub fn i_02aec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AEC8;
    cpu.ir = 0x2270;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    18
}
/// $02AEC8: lea.l $100580.l, a0
pub fn i_02aec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AECE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100580;
    20
}
/// $02AECE: move.l a1, (a0)
pub fn i_02aece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AED0;
    cpu.ir = 0x2089;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02AED0: jsr $5dd02.l
pub fn i_02aed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AED6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2AED6);
    cpu.pc = 0x5DD02;
    16
}
/// $02AED6: move.w $24(a6), $82(a0)
pub fn i_02aed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AEDC;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x82);
    bus.write16(ad4, v3 as u16);
    20
}
/// $02AEDC: jmp $518.l
pub fn i_02aedc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AEE2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Slug_SpawnDropRts_02aee2  ($02AEE2, 2 B, slug/states_02ae3e.s)
/// $02AEE2: rts
pub fn i_02aee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AEE4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_DropVariant0_02aef8  ($02AEF8, 36 B, slug/states_02ae3e.s)
/// $02AEF8: move.w #$3, d1
pub fn i_02aef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AEFC;
    cpu.ir = 0x323C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02AEFC: jsr $2a0f8(pc)
pub fn i_02aefc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AF00;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2AF00);
    cpu.pc = 0x2A0F8;
    16
}
/// $02AF00: ori.w #$2, $38(a6)
pub fn i_02af00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AF06;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $02AF06: lea.l $459a8.l, a1
pub fn i_02af06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AF0C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x459A8;
    20
}
/// $02AF0C: jsr $4ae.l
pub fn i_02af0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AF12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2AF12);
    cpu.pc = 0x4AE;
    16
}
/// $02AF12: jsr $5dd02.l
pub fn i_02af12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AF18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2AF18);
    cpu.pc = 0x5DD02;
    16
}
/// $02AF18: bra.w $2afac
pub fn i_02af18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AF1C;
    cpu.ir = 0x6000;
    cpu.pc = 0x2AFAC;
    10
}
// ===== Slug_DropVariant1_02af1c  ($02AF1C, 36 B, slug/states_02ae3e.s)
/// $02AF1C: move.w #$3, d1
pub fn i_02af1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AF20;
    cpu.ir = 0x323C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02AF20: jsr $2a0f8(pc)
pub fn i_02af20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AF24;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2AF24);
    cpu.pc = 0x2A0F8;
    16
}
/// $02AF24: ori.w #$2, $38(a6)
pub fn i_02af24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AF2A;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $02AF2A: lea.l $459c0.l, a1
pub fn i_02af2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AF30;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x459C0;
    20
}
/// $02AF30: jsr $4ae.l
pub fn i_02af30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AF36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2AF36);
    cpu.pc = 0x4AE;
    16
}
/// $02AF36: jsr $5dd02.l
pub fn i_02af36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AF3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2AF3C);
    cpu.pc = 0x5DD02;
    16
}
/// $02AF3C: bra.w $2afac
pub fn i_02af3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AF40;
    cpu.ir = 0x6000;
    cpu.pc = 0x2AFAC;
    10
}
// ===== Slug_DropVariant2_02af40  ($02AF40, 36 B, slug/states_02ae3e.s)
/// $02AF40: move.w #$3, d1
pub fn i_02af40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AF44;
    cpu.ir = 0x323C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02AF44: jsr $2a0f8(pc)
pub fn i_02af44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AF48;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2AF48);
    cpu.pc = 0x2A0F8;
    16
}
/// $02AF48: ori.w #$2, $38(a6)
pub fn i_02af48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AF4E;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $02AF4E: lea.l $459d8.l, a1
pub fn i_02af4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AF54;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x459D8;
    20
}
/// $02AF54: jsr $4ae.l
pub fn i_02af54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AF5A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2AF5A);
    cpu.pc = 0x4AE;
    16
}
/// $02AF5A: jsr $5dd02.l
pub fn i_02af5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AF60;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2AF60);
    cpu.pc = 0x5DD02;
    16
}
/// $02AF60: bra.w $2afac
pub fn i_02af60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AF64;
    cpu.ir = 0x6000;
    cpu.pc = 0x2AFAC;
    10
}
// ===== Slug_DropVariant3_02af64  ($02AF64, 36 B, slug/states_02ae3e.s)
/// $02AF64: move.w #$3, d1
pub fn i_02af64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AF68;
    cpu.ir = 0x323C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02AF68: jsr $2a0f8(pc)
pub fn i_02af68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AF6C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2AF6C);
    cpu.pc = 0x2A0F8;
    16
}
/// $02AF6C: ori.w #$2, $38(a6)
pub fn i_02af6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AF72;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $02AF72: lea.l $459f0.l, a1
pub fn i_02af72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AF78;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x459F0;
    20
}
/// $02AF78: jsr $4ae.l
pub fn i_02af78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AF7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2AF7E);
    cpu.pc = 0x4AE;
    16
}
/// $02AF7E: jsr $5dd02.l
pub fn i_02af7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AF84;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2AF84);
    cpu.pc = 0x5DD02;
    16
}
/// $02AF84: bra.w $2afac
pub fn i_02af84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AF88;
    cpu.ir = 0x6000;
    cpu.pc = 0x2AFAC;
    10
}
// ===== Slug_DropVariant4_02af88  ($02AF88, 206 B, slug/states_02ae3e.s)
/// $02AF88: move.w #$3, d1
pub fn i_02af88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AF8C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02AF8C: jsr $2a0f8(pc)
pub fn i_02af8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AF90;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2AF90);
    cpu.pc = 0x2A0F8;
    16
}
/// $02AF90: ori.w #$2, $38(a6)
pub fn i_02af90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AF96;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $02AF96: lea.l $45abe.l, a1
pub fn i_02af96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AF9C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x45ABE;
    20
}
/// $02AF9C: jsr $4ae.l
pub fn i_02af9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AFA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2AFA2);
    cpu.pc = 0x4AE;
    16
}
/// $02AFA2: jsr $5dd02.l
pub fn i_02afa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AFA8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2AFA8);
    cpu.pc = 0x5DD02;
    16
}
/// $02AFA8: bra.w $2afac
pub fn i_02afa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AFAC;
    cpu.ir = 0x6000;
    cpu.pc = 0x2AFAC;
    10
}
/// $02AFAC: lea.l $31fca.l, a1
pub fn i_02afac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AFB2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x31FCA;
    20
}
/// $02AFB2: jsr $4ae.l
pub fn i_02afb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AFB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2AFB8);
    cpu.pc = 0x4AE;
    16
}
/// $02AFB8: lea.l $2fb9a(pc), a1
pub fn i_02afb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AFBC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2FB9A;
    16
}
/// $02AFBC: jsr $4ae.l
pub fn i_02afbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AFC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2AFC2);
    cpu.pc = 0x4AE;
    16
}
/// $02AFC2: jsr $5dd02.l
pub fn i_02afc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AFC8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2AFC8);
    cpu.pc = 0x5DD02;
    16
}
/// $02AFC8: lea.l $2792b0.l, a0
pub fn i_02afc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AFCE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2792B0;
    20
}
/// $02AFCE: jsr $28cd4.l
pub fn i_02afce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AFD4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2AFD4);
    cpu.pc = 0x28CD4;
    16
}
/// $02AFD4: clr.w $28(a6)
pub fn i_02afd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AFD8;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02AFD8: clr.w $2a(a6)
pub fn i_02afd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AFDC;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02AFDC: clr.w $2c(a6)
pub fn i_02afdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AFE0;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02AFE0: clr.w $2e(a6)
pub fn i_02afe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AFE4;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02AFE4: clr.b $26(a6)
pub fn i_02afe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AFE8;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x26);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $02AFE8: clr.b $27(a6)
pub fn i_02afe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AFEC;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x27);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $02AFEC: clr.b $8e(a6)
pub fn i_02afec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AFF0;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x8E);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $02AFF0: lea.l $29698(pc), a0
pub fn i_02aff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AFF4;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x29698;
    16
}
/// $02AFF4: move.l a0, $4c(a6)
pub fn i_02aff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AFF8;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02AFF8: jsr $283ca.l
pub fn i_02aff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AFFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2AFFE);
    cpu.pc = 0x283CA;
    16
}
/// $02AFFE: lea.l $2b004(pc), a1
pub fn i_02affe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B002;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B004;
    16
}
/// $02B002: move.l a1, (a6)
pub fn i_02b002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B004;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B004: move.b #$5a, $45(a6)
pub fn i_02b004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B00A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x5A;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02B00A: jsr $2aa0e(pc)
pub fn i_02b00a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B00E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B00E);
    cpu.pc = 0x2AA0E;
    16
}
/// $02B00E: move.w #$fc00, $2a(a6)
pub fn i_02b00e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B014;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFC00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02B014: jsr $2a878(pc)
pub fn i_02b014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B018;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B018);
    cpu.pc = 0x2A878;
    16
}
/// $02B018: jsr $2a478(pc)
pub fn i_02b018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B01C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B01C);
    cpu.pc = 0x2A478;
    16
}
/// $02B01C: move.w $38(a6), d1
pub fn i_02b01c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B020;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02B020: move.w #$9fe0, d0
pub fn i_02b020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B024;
    cpu.ir = 0x303C;
    let v1: u32 = 0x9FE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02B024: andi.w #$ffe3, d1
pub fn i_02b024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B028;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02B028: ori.w #$c, d0
pub fn i_02b028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B02C;
    cpu.ir = 0x40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 | 0xC) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02B02C: or.w d0, d1
pub fn i_02b02c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B02E;
    cpu.ir = 0x8240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = (v1 | cpu.d[1]) & 0xFFFF;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $02B02E: move.w d1, $38(a6)
pub fn i_02b02e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B032;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02B032: jsr $28d70.l
pub fn i_02b032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B038;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B038);
    cpu.pc = 0x28D70;
    16
}
/// $02B038: jsr $283d8.l
pub fn i_02b038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B03E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B03E);
    cpu.pc = 0x283D8;
    16
}
/// $02B03E: cmpi.w #$180, $24(a6)
pub fn i_02b03e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B044;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x180, Size::Word, false);
    16
}
/// $02B044: bgt.w $2b04e
pub fn i_02b044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B048;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x2B04E; return 10; }
    8
}
/// $02B048: lea.l $2b05e(pc), a1
pub fn i_02b048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B04C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B05E;
    16
}
/// $02B04C: move.l a1, (a6)
pub fn i_02b04c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B04E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B04E: jsr $2a8c0(pc)
pub fn i_02b04e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B052;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B052);
    cpu.pc = 0x2A8C0;
    16
}
/// $02B052: bcs.w $2b262
pub fn i_02b052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B056;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2B262; return 10; }
    8
}
// ===== SetTaskHandler_02b056  ($02B056, 8 B, kernel/task_handlers.c)
/// $02B056: lea.l $2b05e(pc), a1
pub fn i_02b056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B05A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B05E;
    16
}
/// $02B05A: move.l a1, (a6)
pub fn i_02b05a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B05C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B05C: rts
pub fn i_02b05c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B05E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_DropDescend_02b05e  ($02B05E, 264 B, slug/states_02ae3e.s)
/// $02B05E: lea.l $2792b0.l, a0
pub fn i_02b05e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B064;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2792B0;
    20
}
/// $02B064: jsr $28cd4.l
pub fn i_02b064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B06A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B06A);
    cpu.pc = 0x28CD4;
    16
}
/// $02B06A: addq.w #$8, $82(a6)
pub fn i_02b06a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B06E;
    cpu.ir = 0x506E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $02B06E: move.w $24(a6), d0
pub fn i_02b06e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B072;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02B072: cmp.w $82(a6), d0
pub fn i_02b072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B076;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $02B076: ble.w $2b07e
pub fn i_02b076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B07A;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x2B07E; return 10; }
    8
}
/// $02B07A: move.w d0, $82(a6)
pub fn i_02b07a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B07E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02B07E: jsr $2a958(pc)
pub fn i_02b07e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B082;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B082);
    cpu.pc = 0x2A958;
    16
}
/// $02B082: cmp.w $80(a6), d2
pub fn i_02b082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B086;
    cpu.ir = 0xB46E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $02B086: beq.w $2b08e
pub fn i_02b086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B08A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2B08E; return 10; }
    8
}
/// $02B08A: move.w d2, $80(a6)
pub fn i_02b08a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B08E;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02B08E: jsr $2a478(pc)
pub fn i_02b08e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B092;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B092);
    cpu.pc = 0x2A478;
    16
}
/// $02B092: move.w #$fc00, $2a(a6)
pub fn i_02b092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B098;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFC00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02B098: lea.l $2b09e(pc), a1
pub fn i_02b098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B09C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B09E;
    16
}
/// $02B09C: move.l a1, (a6)
pub fn i_02b09c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B09E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B09E: move.b #$46, $45(a6)
pub fn i_02b09e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B0A4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x46;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02B0A4: bset.b #$1, $8d(a6)
pub fn i_02b0a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B0AA;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02B0AA: jsr $2aa0e(pc)
pub fn i_02b0aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B0AE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B0AE);
    cpu.pc = 0x2AA0E;
    16
}
/// $02B0AE: move.w $2a(a6), d0
pub fn i_02b0ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B0B2;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02B0B2: move.w #$400, d1
pub fn i_02b0b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B0B6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02B0B6: jsr $267f4.l
pub fn i_02b0b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B0BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B0BC);
    cpu.pc = 0x267F4;
    16
}
/// $02B0BC: move.w d0, $2a(a6)
pub fn i_02b0bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B0C0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02B0C0: jsr $2a878(pc)
pub fn i_02b0c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B0C4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B0C4);
    cpu.pc = 0x2A878;
    16
}
/// $02B0C4: bcc.w $2b0ce
pub fn i_02b0c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B0C8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2B0CE; return 10; }
    8
}
/// $02B0C8: lea.l $2dd20(pc), a1
pub fn i_02b0c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B0CC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DD20;
    16
}
/// $02B0CC: move.l a1, (a6)
pub fn i_02b0cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B0CE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B0CE: jsr $2a478(pc)
pub fn i_02b0ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B0D2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B0D2);
    cpu.pc = 0x2A478;
    16
}
/// $02B0D2: move.w $38(a6), d1
pub fn i_02b0d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B0D6;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02B0D6: move.w #$9fe0, d0
pub fn i_02b0d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B0DA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x9FE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02B0DA: andi.w #$ffe3, d1
pub fn i_02b0da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B0DE;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02B0DE: ori.w #$c, d0
pub fn i_02b0de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B0E2;
    cpu.ir = 0x40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 | 0xC) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02B0E2: or.w d0, d1
pub fn i_02b0e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B0E4;
    cpu.ir = 0x8240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = (v1 | cpu.d[1]) & 0xFFFF;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $02B0E4: move.w d1, $38(a6)
pub fn i_02b0e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B0E8;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02B0E8: jsr $28d70.l
pub fn i_02b0e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B0EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B0EE);
    cpu.pc = 0x28D70;
    16
}
/// $02B0EE: jsr $283d8.l
pub fn i_02b0ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B0F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B0F4);
    cpu.pc = 0x283D8;
    16
}
/// $02B0F4: tst.w $2a(a6)
pub fn i_02b0f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B0F8;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $02B0F8: bgt.w $2b164
pub fn i_02b0f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B0FC;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x2B164; return 10; }
    8
}
/// $02B0FC: jsr $2a8c0(pc)
pub fn i_02b0fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B100;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B100);
    cpu.pc = 0x2A8C0;
    16
}
/// $02B100: cmpi.b #$0, d1
pub fn i_02b100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B104;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $02B104: ble.w $2b110
pub fn i_02b104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B108;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x2B110; return 10; }
    8
}
/// $02B108: clr.w $28(a6)
pub fn i_02b108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B10C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02B10C: clr.w $2c(a6)
pub fn i_02b10c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B110;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02B110: jsr $2a958(pc)
pub fn i_02b110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B114;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B114);
    cpu.pc = 0x2A958;
    16
}
/// $02B114: cmp.w $80(a6), d2
pub fn i_02b114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B118;
    cpu.ir = 0xB46E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $02B118: beq.w $2b164
pub fn i_02b118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B11C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2B164; return 10; }
    8
}
/// $02B11C: move.w d2, $80(a6)
pub fn i_02b11c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B120;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02B120: move.w d2, d0
pub fn i_02b120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B122;
    cpu.ir = 0x3002;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02B122: asr.w #$1, d0
pub fn i_02b122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B124;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02B124: neg.w d0
pub fn i_02b124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B126;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02B126: addi.w #$20, d0
pub fn i_02b126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B12A;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02B12A: cmpi.w #$40, d0
pub fn i_02b12a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B12E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $02B12E: bls.w $2b136
pub fn i_02b12e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B132;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x2B136; return 10; }
    8
}
/// $02B132: move.w #$40, d0
pub fn i_02b132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B136;
    cpu.ir = 0x303C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02B136: movem.w d0, -(a7)
pub fn i_02b136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B13A;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $02B13A: jsr $295a6(pc)
pub fn i_02b13a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B13E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B13E);
    cpu.pc = 0x295A6;
    16
}
/// $02B13E: movea.l #$2b0c30, a0
pub fn i_02b13e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B144;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2B0C30;
    cpu.a[0] = v1;
    4
}
/// $02B144: lsl.w #$2, d0
pub fn i_02b144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B146;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02B146: movea.l (a0, d0.w), a0
pub fn i_02b146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B14A;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02B14A: cmpa.l #$ffffffff, a0
pub fn i_02b14a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B150;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02B150: beq.w $2b15a
pub fn i_02b150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B154;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2B15A; return 10; }
    8
}
/// $02B154: jsr $28cd4.l
pub fn i_02b154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B15A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B15A);
    cpu.pc = 0x28CD4;
    16
}
/// $02B15A: movem.w (a7)+, d0
pub fn i_02b15a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B15E;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $02B15E: move.b #$14, $20(a6)
pub fn i_02b15e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B164;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02B164: rts
pub fn i_02b164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B166;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_DropBossInit_02b166  ($02B166, 246 B, slug/states_02ae3e.s)
/// $02B166: move.w #$68, $22(a6)
pub fn i_02b166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B16C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x68;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02B16C: move.w #$1d8, $24(a6)
pub fn i_02b16c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B172;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1D8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02B172: move.w #$1d8, $82(a6)
pub fn i_02b172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B178;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1D8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02B178: move.w #$3, d1
pub fn i_02b178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B17C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02B17C: jsr $2a1aa(pc)
pub fn i_02b17c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B180;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B180);
    cpu.pc = 0x2A1AA;
    16
}
/// $02B180: ori.w #$2, $38(a6)
pub fn i_02b180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B186;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $02B186: bra.w $2b1c2
pub fn i_02b186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B18A;
    cpu.ir = 0x6000;
    cpu.pc = 0x2B1C2;
    10
}
/// $02B1C2: lea.l $2792b0.l, a0
pub fn i_02b1c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B1C8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2792B0;
    20
}
/// $02B1C8: jsr $28cd4.l
pub fn i_02b1c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B1CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B1CE);
    cpu.pc = 0x28CD4;
    16
}
/// $02B1CE: clr.w $28(a6)
pub fn i_02b1ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B1D2;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02B1D2: clr.w $2a(a6)
pub fn i_02b1d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B1D6;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02B1D6: clr.w $2c(a6)
pub fn i_02b1d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B1DA;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02B1DA: clr.w $2e(a6)
pub fn i_02b1da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B1DE;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02B1DE: clr.b $26(a6)
pub fn i_02b1de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B1E2;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x26);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $02B1E2: clr.b $27(a6)
pub fn i_02b1e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B1E6;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x27);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $02B1E6: clr.b $8e(a6)
pub fn i_02b1e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B1EA;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x8E);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $02B1EA: move.w #$200, $2a(a6)
pub fn i_02b1ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B1F0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x200;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02B1F0: move.w #$fe00, $2e(a6)
pub fn i_02b1f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B1F6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFE00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02B1F6: lea.l $29698(pc), a0
pub fn i_02b1f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B1FA;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x29698;
    16
}
/// $02B1FA: move.l a0, $4c(a6)
pub fn i_02b1fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B1FE;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02B1FE: jsr $283ca.l
pub fn i_02b1fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B204;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B204);
    cpu.pc = 0x283CA;
    16
}
/// $02B204: lea.l $2b20a(pc), a1
pub fn i_02b204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B208;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B20A;
    16
}
/// $02B208: move.l a1, (a6)
pub fn i_02b208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B20A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B20A: bset.b #$6, $13(a6)
pub fn i_02b20a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B210;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02B210: move.b #$5a, $45(a6)
pub fn i_02b210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B216;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x5A;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02B216: jsr $2aa0e(pc)
pub fn i_02b216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B21A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B21A);
    cpu.pc = 0x2AA0E;
    16
}
/// $02B21A: jsr $2a878(pc)
pub fn i_02b21a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B21E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B21E);
    cpu.pc = 0x2A878;
    16
}
/// $02B21E: jsr $2a478(pc)
pub fn i_02b21e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B222;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B222);
    cpu.pc = 0x2A478;
    16
}
/// $02B222: move.w $38(a6), d1
pub fn i_02b222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B226;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02B226: move.w #$9fe0, d0
pub fn i_02b226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B22A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x9FE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02B22A: andi.w #$ffe3, d1
pub fn i_02b22a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B22E;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02B22E: ori.w #$c, d0
pub fn i_02b22e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B232;
    cpu.ir = 0x40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 | 0xC) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02B232: or.w d0, d1
pub fn i_02b232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B234;
    cpu.ir = 0x8240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = (v1 | cpu.d[1]) & 0xFFFF;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $02B234: move.w d1, $38(a6)
pub fn i_02b234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B238;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02B238: jsr $28d70.l
pub fn i_02b238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B23E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B23E);
    cpu.pc = 0x28D70;
    16
}
/// $02B23E: jsr $283d8.l
pub fn i_02b23e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B244;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B244);
    cpu.pc = 0x283D8;
    16
}
/// $02B244: cmpi.w #$180, $24(a6)
pub fn i_02b244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B24A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x180, Size::Word, false);
    16
}
/// $02B24A: bgt.w $2b254
pub fn i_02b24a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B24E;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x2B254; return 10; }
    8
}
/// $02B24E: lea.l $2b264(pc), a1
pub fn i_02b24e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B252;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B264;
    16
}
/// $02B252: move.l a1, (a6)
pub fn i_02b252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B254;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B254: jsr $2a8c0(pc)
pub fn i_02b254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B258;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B258);
    cpu.pc = 0x2A8C0;
    16
}
/// $02B258: bcs.w $2b262
pub fn i_02b258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B25C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2B262; return 10; }
    8
}
// ===== SetTaskHandler_02b25c  ($02B25C, 8 B, kernel/task_handlers.c)
/// $02B25C: lea.l $2b264(pc), a1
pub fn i_02b25c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B260;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B264;
    16
}
/// $02B260: move.l a1, (a6)
pub fn i_02b260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B262;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B262: rts
pub fn i_02b262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B264;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_DropBossDescend_02b264  ($02B264, 296 B, slug/states_02ae3e.s)
/// $02B264: lea.l $2792b0.l, a0
pub fn i_02b264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B26A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2792B0;
    20
}
/// $02B26A: jsr $28cd4.l
pub fn i_02b26a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B270;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B270);
    cpu.pc = 0x28CD4;
    16
}
/// $02B270: addq.w #$8, $82(a6)
pub fn i_02b270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B274;
    cpu.ir = 0x506E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $02B274: move.w #$200, $2a(a6)
pub fn i_02b274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B27A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x200;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02B27A: move.w #$fe00, $2e(a6)
pub fn i_02b27a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B280;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFE00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02B280: move.w $24(a6), d0
pub fn i_02b280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B284;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02B284: cmp.w $82(a6), d0
pub fn i_02b284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B288;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $02B288: ble.w $2b290
pub fn i_02b288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B28C;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x2B290; return 10; }
    8
}
/// $02B28C: move.w d0, $82(a6)
pub fn i_02b28c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B290;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02B290: jsr $2a958(pc)
pub fn i_02b290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B294;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B294);
    cpu.pc = 0x2A958;
    16
}
/// $02B294: cmp.w $80(a6), d2
pub fn i_02b294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B298;
    cpu.ir = 0xB46E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $02B298: beq.w $2b2a0
pub fn i_02b298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B29C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2B2A0; return 10; }
    8
}
/// $02B29C: move.w d2, $80(a6)
pub fn i_02b29c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B2A0;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02B2A0: jsr $2a478(pc)
pub fn i_02b2a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B2A4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B2A4);
    cpu.pc = 0x2A478;
    16
}
/// $02B2A4: lea.l $2b2aa(pc), a1
pub fn i_02b2a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B2A8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B2AA;
    16
}
/// $02B2A8: move.l a1, (a6)
pub fn i_02b2a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B2AA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B2AA: cmpi.w #$180, $24(a6)
pub fn i_02b2aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B2B0;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x180, Size::Word, false);
    16
}
/// $02B2B0: bgt.w $2b2be
pub fn i_02b2b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B2B4;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x2B2BE; return 10; }
    8
}
/// $02B2B4: clr.w $2a(a6)
pub fn i_02b2b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B2B8;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02B2B8: lea.l $2b2be(pc), a1
pub fn i_02b2b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B2BC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B2BE;
    16
}
/// $02B2BC: move.l a1, (a6)
pub fn i_02b2bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B2BE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B2BE: bset.b #$6, $13(a6)
pub fn i_02b2be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B2C4;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02B2C4: move.b #$46, $45(a6)
pub fn i_02b2c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B2CA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x46;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02B2CA: bset.b #$1, $8d(a6)
pub fn i_02b2ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B2D0;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02B2D0: jsr $2aa0e(pc)
pub fn i_02b2d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B2D4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B2D4);
    cpu.pc = 0x2AA0E;
    16
}
/// $02B2D4: move.w $2a(a6), d0
pub fn i_02b2d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B2D8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02B2D8: move.w #$400, d1
pub fn i_02b2d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B2DC;
    cpu.ir = 0x323C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02B2DC: jsr $267f4.l
pub fn i_02b2dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B2E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B2E2);
    cpu.pc = 0x267F4;
    16
}
/// $02B2E2: move.w d0, $2a(a6)
pub fn i_02b2e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B2E6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02B2E6: jsr $2a878(pc)
pub fn i_02b2e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B2EA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B2EA);
    cpu.pc = 0x2A878;
    16
}
/// $02B2EA: bcc.w $2b2f4
pub fn i_02b2ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B2EE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2B2F4; return 10; }
    8
}
/// $02B2EE: lea.l $2dd74(pc), a1
pub fn i_02b2ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B2F2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DD74;
    16
}
/// $02B2F2: move.l a1, (a6)
pub fn i_02b2f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B2F4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B2F4: jsr $2a478(pc)
pub fn i_02b2f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B2F8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B2F8);
    cpu.pc = 0x2A478;
    16
}
/// $02B2F8: move.w $38(a6), d1
pub fn i_02b2f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B2FC;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02B2FC: move.w #$9fe0, d0
pub fn i_02b2fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B300;
    cpu.ir = 0x303C;
    let v1: u32 = 0x9FE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02B300: andi.w #$ffe3, d1
pub fn i_02b300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B304;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02B304: ori.w #$c, d0
pub fn i_02b304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B308;
    cpu.ir = 0x40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 | 0xC) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02B308: or.w d0, d1
pub fn i_02b308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B30A;
    cpu.ir = 0x8240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = (v1 | cpu.d[1]) & 0xFFFF;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $02B30A: move.w d1, $38(a6)
pub fn i_02b30a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B30E;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02B30E: jsr $28d70.l
pub fn i_02b30e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B314;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B314);
    cpu.pc = 0x28D70;
    16
}
/// $02B314: jsr $283d8.l
pub fn i_02b314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B31A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B31A);
    cpu.pc = 0x283D8;
    16
}
/// $02B31A: tst.w $2a(a6)
pub fn i_02b31a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B31E;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $02B31E: bgt.w $2b38a
pub fn i_02b31e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B322;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x2B38A; return 10; }
    8
}
/// $02B322: jsr $2a8c0(pc)
pub fn i_02b322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B326;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B326);
    cpu.pc = 0x2A8C0;
    16
}
/// $02B326: cmpi.b #$0, d1
pub fn i_02b326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B32A;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $02B32A: ble.w $2b336
pub fn i_02b32a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B32E;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x2B336; return 10; }
    8
}
/// $02B32E: clr.w $28(a6)
pub fn i_02b32e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B332;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02B332: clr.w $2c(a6)
pub fn i_02b332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B336;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02B336: jsr $2a958(pc)
pub fn i_02b336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B33A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B33A);
    cpu.pc = 0x2A958;
    16
}
/// $02B33A: cmp.w $80(a6), d2
pub fn i_02b33a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B33E;
    cpu.ir = 0xB46E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $02B33E: beq.w $2b38a
pub fn i_02b33e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B342;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2B38A; return 10; }
    8
}
/// $02B342: move.w d2, $80(a6)
pub fn i_02b342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B346;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02B346: move.w d2, d0
pub fn i_02b346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B348;
    cpu.ir = 0x3002;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02B348: asr.w #$1, d0
pub fn i_02b348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B34A;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02B34A: neg.w d0
pub fn i_02b34a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B34C;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02B34C: addi.w #$20, d0
pub fn i_02b34c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B350;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02B350: cmpi.w #$40, d0
pub fn i_02b350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B354;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $02B354: bls.w $2b35c
pub fn i_02b354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B358;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x2B35C; return 10; }
    8
}
/// $02B358: move.w #$40, d0
pub fn i_02b358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B35C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02B35C: movem.w d0, -(a7)
pub fn i_02b35c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B360;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $02B360: jsr $295a6(pc)
pub fn i_02b360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B364;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B364);
    cpu.pc = 0x295A6;
    16
}
/// $02B364: movea.l #$2b0c30, a0
pub fn i_02b364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B36A;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2B0C30;
    cpu.a[0] = v1;
    4
}
/// $02B36A: lsl.w #$2, d0
pub fn i_02b36a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B36C;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02B36C: movea.l (a0, d0.w), a0
pub fn i_02b36c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B370;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02B370: cmpa.l #$ffffffff, a0
pub fn i_02b370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B376;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02B376: beq.w $2b380
pub fn i_02b376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B37A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2B380; return 10; }
    8
}
/// $02B37A: jsr $28cd4.l
pub fn i_02b37a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B380;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B380);
    cpu.pc = 0x28CD4;
    16
}
/// $02B380: movem.w (a7)+, d0
pub fn i_02b380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B384;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $02B384: move.b #$14, $20(a6)
pub fn i_02b384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B38A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02B38A: rts
pub fn i_02b38a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B38C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_IdleFlat_02b38c  ($02B38C, 318 B, slug/states_02ae3e.s)
/// $02B38C: jsr $267e6.l
pub fn i_02b38c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B392;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B392);
    cpu.pc = 0x267E6;
    16
}
/// $02B392: clr.w $28(a6)
pub fn i_02b392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B396;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02B396: jsr $2a9a0(pc)
pub fn i_02b396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B39A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B39A);
    cpu.pc = 0x2A9A0;
    16
}
/// $02B39A: movea.l #$2792f4, a0
pub fn i_02b39a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B3A0;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2792F4;
    cpu.a[0] = v1;
    4
}
/// $02B3A0: lsl.w #$2, d0
pub fn i_02b3a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B3A2;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02B3A2: movea.l (a0, d0.w), a0
pub fn i_02b3a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B3A6;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02B3A6: cmpa.l #$ffffffff, a0
pub fn i_02b3a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B3AC;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02B3AC: beq.w $2b3b6
pub fn i_02b3ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B3B0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2B3B6; return 10; }
    8
}
/// $02B3B0: jsr $28cd4.l
pub fn i_02b3b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B3B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B3B6);
    cpu.pc = 0x28CD4;
    16
}
/// $02B3B6: lea.l $29ffc(pc), a1
pub fn i_02b3b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B3BA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x29FFC;
    16
}
/// $02B3BA: movea.l (a1, d0.w), a0
pub fn i_02b3ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B3BE;
    cpu.ir = 0x2071;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02B3BE: move.l a0, $48(a6)
pub fn i_02b3be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B3C2;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02B3C2: move.b #$14, $20(a6)
pub fn i_02b3c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B3C8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02B3C8: andi.w #$ff, d0
pub fn i_02b3c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B3CC;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02B3CC: lsr.w #$2, d0
pub fn i_02b3cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B3CE;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02B3CE: add.b d0, $20(a6)
pub fn i_02b3ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B3D2;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02B3D2: move.l #$295b4, $60(a6)
pub fn i_02b3d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B3DA;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x295B4;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02B3DA: lea.l $2b3e0(pc), a1
pub fn i_02b3da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B3DE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B3E0;
    16
}
/// $02B3DE: move.l a1, (a6)
pub fn i_02b3de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B3E0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B3E0: lea.l $29698(pc), a0
pub fn i_02b3e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B3E4;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x29698;
    16
}
/// $02B3E4: move.l a0, $4c(a6)
pub fn i_02b3e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B3E8;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02B3E8: jsr $283ca.l
pub fn i_02b3e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B3EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B3EE);
    cpu.pc = 0x283CA;
    16
}
/// $02B3EE: jsr $2aa24(pc)
pub fn i_02b3ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B3F2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B3F2);
    cpu.pc = 0x2AA24;
    16
}
/// $02B3F2: jsr $2a7d8(pc)
pub fn i_02b3f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B3F6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B3F6);
    cpu.pc = 0x2A7D8;
    16
}
/// $02B3F6: jsr $2a4f0(pc)
pub fn i_02b3f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B3FA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B3FA);
    cpu.pc = 0x2A4F0;
    16
}
/// $02B3FA: bcc.w $2b404
pub fn i_02b3fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B3FE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2B404; return 10; }
    8
}
/// $02B3FE: lea.l $2b4d2(pc), a1
pub fn i_02b3fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B402;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B4D2;
    16
}
/// $02B402: move.l a1, (a6)
pub fn i_02b402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B404;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B404: jsr $2abc0(pc)
pub fn i_02b404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B408;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B408);
    cpu.pc = 0x2ABC0;
    16
}
/// $02B408: bcs.w $2b428
pub fn i_02b408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B40C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2B428; return 10; }
    8
}
/// $02B40C: jsr $2a328(pc)
pub fn i_02b40c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B410;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B410);
    cpu.pc = 0x2A328;
    16
}
/// $02B410: bcc.w $2b41a
pub fn i_02b410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B414;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2B41A; return 10; }
    8
}
/// $02B414: lea.l $2b8de(pc), a1
pub fn i_02b414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B418;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B8DE;
    16
}
/// $02B418: move.l a1, (a6)
pub fn i_02b418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B41A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B41A: jsr $2a34e(pc)
pub fn i_02b41a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B41E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B41E);
    cpu.pc = 0x2A34E;
    16
}
/// $02B41E: bcc.w $2b428
pub fn i_02b41e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B422;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2B428; return 10; }
    8
}
/// $02B422: lea.l $2ba44(pc), a1
pub fn i_02b422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B426;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2BA44;
    16
}
/// $02B426: move.l a1, (a6)
pub fn i_02b426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B428;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B428: jsr $2a59a(pc)
pub fn i_02b428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B42C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B42C);
    cpu.pc = 0x2A59A;
    16
}
/// $02B42C: bcc.w $2b448
pub fn i_02b42c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B430;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2B448; return 10; }
    8
}
/// $02B430: jsr $2a9a0(pc)
pub fn i_02b430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B434;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B434);
    cpu.pc = 0x2A9A0;
    16
}
/// $02B434: andi.w #$ff, d0
pub fn i_02b434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B438;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02B438: lea.l $2a078(pc), a0
pub fn i_02b438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B43C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A078;
    16
}
/// $02B43C: movea.l #$ffffffff, a1
pub fn i_02b43c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B442;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $02B442: jsr $772.l
pub fn i_02b442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B448;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B448);
    cpu.pc = 0x772;
    16
}
/// $02B448: jsr $28d70.l
pub fn i_02b448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B44E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B44E);
    cpu.pc = 0x28D70;
    16
}
/// $02B44E: jsr $2a8c0(pc)
pub fn i_02b44e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B452;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B452);
    cpu.pc = 0x2A8C0;
    16
}
/// $02B452: bcc.w $2b45c
pub fn i_02b452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B456;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2B45C; return 10; }
    8
}
/// $02B456: lea.l $2d02e(pc), a1
pub fn i_02b456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B45A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D02E;
    16
}
/// $02B45A: move.l a1, (a6)
pub fn i_02b45a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B45C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B45C: jsr $2aaf0(pc)
pub fn i_02b45c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B460;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B460);
    cpu.pc = 0x2AAF0;
    16
}
/// $02B460: lea.l $2c9b0(pc), a0
pub fn i_02b460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B464;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2C9B0;
    16
}
/// $02B464: movea.l #$ffffffff, a1
pub fn i_02b464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B46A;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $02B46A: jsr $772.l
pub fn i_02b46a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B470;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B470);
    cpu.pc = 0x772;
    16
}
/// $02B470: jsr $2aac0(pc)
pub fn i_02b470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B474;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B474);
    cpu.pc = 0x2AAC0;
    16
}
/// $02B474: bcc.w $2b47e
pub fn i_02b474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B478;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2B47E; return 10; }
    8
}
/// $02B478: lea.l $2c24a(pc), a1
pub fn i_02b478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B47C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2C24A;
    16
}
/// $02B47C: move.l a1, (a6)
pub fn i_02b47c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B47E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B47E: jsr $2ac80(pc)
pub fn i_02b47e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B482;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B482);
    cpu.pc = 0x2AC80;
    16
}
/// $02B482: bcc.w $2b48c
pub fn i_02b482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B486;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2B48C; return 10; }
    8
}
/// $02B486: lea.l $2dd74(pc), a1
pub fn i_02b486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B48A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DD74;
    16
}
/// $02B48A: move.l a1, (a6)
pub fn i_02b48a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B48C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B48C: jsr $2a5cc(pc)
pub fn i_02b48c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B490;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B490);
    cpu.pc = 0x2A5CC;
    16
}
/// $02B490: bcc.w $2b49a
pub fn i_02b490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B494;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2B49A; return 10; }
    8
}
/// $02B494: lea.l $2bbf2(pc), a1
pub fn i_02b494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B498;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2BBF2;
    16
}
/// $02B498: move.l a1, (a6)
pub fn i_02b498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B49A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B49A: jsr $2a690(pc)
pub fn i_02b49a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B49E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B49E);
    cpu.pc = 0x2A690;
    16
}
/// $02B49E: lea.l $2a060(pc), a0
pub fn i_02b49e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B4A2;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A060;
    16
}
/// $02B4A2: movea.l #$ffffffff, a1
pub fn i_02b4a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B4A8;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $02B4A8: jsr $772.l
pub fn i_02b4a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B4AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B4AE);
    cpu.pc = 0x772;
    16
}
/// $02B4AE: movea.l #$ffffffff, a0
pub fn i_02b4ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B4B4;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $02B4B4: lea.l $27964e.l, a0
pub fn i_02b4b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B4BA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27964E;
    20
}
/// $02B4BA: jsr $5dd56.l
pub fn i_02b4ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B4C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B4C0);
    cpu.pc = 0x5DD56;
    16
}
/// $02B4C0: bcc.w $2b4c8
pub fn i_02b4c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B4C4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2B4C8; return 10; }
    8
}
/// $02B4C4: jmp $2dcbc(pc)
pub fn i_02b4c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B4C8;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x2DCBC;
    8
}
/// $02B4C8: rts
pub fn i_02b4c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B4CA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_IdleSlopeJmp_02b4ca  ($02B4CA, 8 B, slug/states_02ae3e.s)
/// $02B4CA: bra.w $2b4d2
pub fn i_02b4ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B4CE;
    cpu.ir = 0x6000;
    cpu.pc = 0x2B4D2;
    10
}
/// $02B4CE: bra.w $2b4d2
pub fn i_02b4ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B4D2;
    cpu.ir = 0x6000;
    cpu.pc = 0x2B4D2;
    10
}
// ===== Slug_IdleSlope_02b4d2  ($02B4D2, 546 B, slug/states_02ae3e.s)
/// $02B4D2: clr.w $36(a6)
pub fn i_02b4d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B4D6;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x36);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02B4D6: bclr.b #$5, $8c(a6)
pub fn i_02b4d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B4DC;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02B4DC: clr.w $2c(a6)
pub fn i_02b4dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B4E0;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02B4E0: jsr $2a958(pc)
pub fn i_02b4e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B4E4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B4E4);
    cpu.pc = 0x2A958;
    16
}
/// $02B4E4: move.w d2, $80(a6)
pub fn i_02b4e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B4E8;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02B4E8: move.w d2, d0
pub fn i_02b4e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B4EA;
    cpu.ir = 0x3002;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02B4EA: asr.w #$1, d0
pub fn i_02b4ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B4EC;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02B4EC: neg.w d0
pub fn i_02b4ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B4EE;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02B4EE: addi.w #$20, d0
pub fn i_02b4ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B4F2;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02B4F2: cmpi.w #$40, d0
pub fn i_02b4f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B4F6;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $02B4F6: bls.w $2b4fe
pub fn i_02b4f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B4FA;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x2B4FE; return 10; }
    8
}
/// $02B4FA: move.w #$40, d0
pub fn i_02b4fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B4FE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02B4FE: movem.w d0, -(a7)
pub fn i_02b4fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B502;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $02B502: jsr $295a6(pc)
pub fn i_02b502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B506;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B506);
    cpu.pc = 0x295A6;
    16
}
/// $02B506: movea.l #$2b0c30, a0
pub fn i_02b506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B50C;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2B0C30;
    cpu.a[0] = v1;
    4
}
/// $02B50C: lsl.w #$2, d0
pub fn i_02b50c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B50E;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02B50E: movea.l (a0, d0.w), a0
pub fn i_02b50e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B512;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02B512: cmpa.l #$ffffffff, a0
pub fn i_02b512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B518;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02B518: beq.w $2b522
pub fn i_02b518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B51C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2B522; return 10; }
    8
}
/// $02B51C: jsr $28cd4.l
pub fn i_02b51c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B522;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B522);
    cpu.pc = 0x28CD4;
    16
}
/// $02B522: movem.w (a7)+, d0
pub fn i_02b522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B526;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $02B526: move.b #$0, $20(a6)
pub fn i_02b526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B52C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02B52C: lsr.w #$2, d0
pub fn i_02b52c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B52E;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02B52E: andi.w #$ff, d0
pub fn i_02b52e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B532;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02B532: add.b d0, $20(a6)
pub fn i_02b532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B536;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02B536: move.l #$295b4, $60(a6)
pub fn i_02b536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B53E;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x295B4;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02B53E: lea.l $2b544(pc), a1
pub fn i_02b53e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B542;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B544;
    16
}
/// $02B542: move.l a1, (a6)
pub fn i_02b542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B544;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B544: lea.l $29698(pc), a0
pub fn i_02b544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B548;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x29698;
    16
}
/// $02B548: move.l a0, $4c(a6)
pub fn i_02b548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B54C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02B54C: jsr $283ca.l
pub fn i_02b54c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B552;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B552);
    cpu.pc = 0x283CA;
    16
}
/// $02B552: bset.b #$2, $8d(a6)
pub fn i_02b552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B558;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02B558: jsr $2aa24(pc)
pub fn i_02b558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B55C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B55C);
    cpu.pc = 0x2AA24;
    16
}
/// $02B55C: move.w $36(a6), d7
pub fn i_02b55c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B560;
    cpu.ir = 0x3E2E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02B560: jsr $2abc0(pc)
pub fn i_02b560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B564;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B564);
    cpu.pc = 0x2ABC0;
    16
}
/// $02B564: bcs.w $2b59c
pub fn i_02b564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B568;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2B59C; return 10; }
    8
}
/// $02B568: tst.b $92(a6)
pub fn i_02b568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B56C;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x92);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $02B56C: bne.w $2b59c
pub fn i_02b56c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B570;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2B59C; return 10; }
    8
}
/// $02B570: jsr $2a328(pc)
pub fn i_02b570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B574;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B574);
    cpu.pc = 0x2A328;
    16
}
/// $02B574: bcc.w $2b588
pub fn i_02b574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B578;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2B588; return 10; }
    8
}
/// $02B578: tst.w $36(a6)
pub fn i_02b578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B57C;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $02B57C: bne.w $2b584
pub fn i_02b57c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B580;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2B584; return 10; }
    8
}
/// $02B580: move.w #$2a0, d7
pub fn i_02b580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B584;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x2A0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02B584: bra.w $2b59c
pub fn i_02b584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B588;
    cpu.ir = 0x6000;
    cpu.pc = 0x2B59C;
    10
}
/// $02B588: jsr $2a34e(pc)
pub fn i_02b588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B58C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B58C);
    cpu.pc = 0x2A34E;
    16
}
/// $02B58C: bcc.w $2b59c
pub fn i_02b58c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B590;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2B59C; return 10; }
    8
}
/// $02B590: tst.w $36(a6)
pub fn i_02b590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B594;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $02B594: bne.w $2b59c
pub fn i_02b594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B598;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2B59C; return 10; }
    8
}
/// $02B598: move.w #$fd60, d7
pub fn i_02b598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B59C;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0xFD60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02B59C: cmpi.b #$3, $106f2a.l
pub fn i_02b59c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B5A4;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106F2A) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    20
}
/// $02B5A4: bne.w $2b5be
pub fn i_02b5a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B5A8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2B5BE; return 10; }
    8
}
/// $02B5A8: jsr $2a958(pc)
pub fn i_02b5a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B5AC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B5AC);
    cpu.pc = 0x2A958;
    16
}
/// $02B5AC: sub.w $80(a6), d2
pub fn i_02b5ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B5B0;
    cpu.ir = 0x946E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $02B5B0: eor.w d7, d2
pub fn i_02b5b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B5B2;
    cpu.ir = 0xBF42;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = (v1 ^ cpu.d[7]) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02B5B2: asl.w #$1, d2
pub fn i_02b5b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B5B4;
    cpu.ir = 0xE342;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02B5B4: bcs.w $2b5be
pub fn i_02b5b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B5B8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2B5BE; return 10; }
    8
}
/// $02B5B8: move.w d7, d2
pub fn i_02b5b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B5BA;
    cpu.ir = 0x3407;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02B5BA: asr.w #$1, d2
pub fn i_02b5ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B5BC;
    cpu.ir = 0xE242;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02B5BC: add.w d2, d7
pub fn i_02b5bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B5BE;
    cpu.ir = 0xDE42;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $02B5BE: move.w d7, $28(a6)
pub fn i_02b5be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B5C2;
    cpu.ir = 0x3D47;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02B5C2: jsr $2a478(pc)
pub fn i_02b5c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B5C6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B5C6);
    cpu.pc = 0x2A478;
    16
}
/// $02B5C6: jsr $2a760(pc)
pub fn i_02b5c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B5CA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B5CA);
    cpu.pc = 0x2A760;
    16
}
/// $02B5CA: jsr $2a958(pc)
pub fn i_02b5ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B5CE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B5CE);
    cpu.pc = 0x2A958;
    16
}
/// $02B5CE: cmp.w $80(a6), d2
pub fn i_02b5ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B5D2;
    cpu.ir = 0xB46E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $02B5D2: beq.w $2b628
pub fn i_02b5d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B5D6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2B628; return 10; }
    8
}
/// $02B5D6: move.w d2, $80(a6)
pub fn i_02b5d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B5DA;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02B5DA: move.w d2, d0
pub fn i_02b5da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B5DC;
    cpu.ir = 0x3002;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02B5DC: asr.w #$1, d0
pub fn i_02b5dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B5DE;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02B5DE: neg.w d0
pub fn i_02b5de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B5E0;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02B5E0: addi.w #$20, d0
pub fn i_02b5e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B5E4;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02B5E4: cmpi.w #$40, d0
pub fn i_02b5e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B5E8;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $02B5E8: bls.w $2b5f0
pub fn i_02b5e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B5EC;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x2B5F0; return 10; }
    8
}
/// $02B5EC: move.w #$40, d0
pub fn i_02b5ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B5F0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02B5F0: movem.w d0, -(a7)
pub fn i_02b5f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B5F4;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $02B5F4: jsr $295a6(pc)
pub fn i_02b5f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B5F8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B5F8);
    cpu.pc = 0x295A6;
    16
}
/// $02B5F8: movea.l #$2b0c30, a0
pub fn i_02b5f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B5FE;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2B0C30;
    cpu.a[0] = v1;
    4
}
/// $02B5FE: lsl.w #$2, d0
pub fn i_02b5fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B600;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02B600: movea.l (a0, d0.w), a0
pub fn i_02b600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B604;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02B604: cmpa.l #$ffffffff, a0
pub fn i_02b604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B60A;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02B60A: beq.w $2b614
pub fn i_02b60a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B60E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2B614; return 10; }
    8
}
/// $02B60E: jsr $28cd4.l
pub fn i_02b60e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B614;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B614);
    cpu.pc = 0x28CD4;
    16
}
/// $02B614: movem.w (a7)+, d0
pub fn i_02b614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B618;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $02B618: move.b #$0, $20(a6)
pub fn i_02b618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B61E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02B61E: lsr.w #$2, d0
pub fn i_02b61e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B620;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02B620: andi.w #$ff, d0
pub fn i_02b620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B624;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02B624: add.b d0, $20(a6)
pub fn i_02b624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B628;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02B628: jsr $28d70.l
pub fn i_02b628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B62E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B62E);
    cpu.pc = 0x28D70;
    16
}
/// $02B62E: jsr $2a4ec(pc)
pub fn i_02b62e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B632;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B632);
    cpu.pc = 0x2A4EC;
    16
}
/// $02B632: bcs.w $2b65c
pub fn i_02b632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B636;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2B65C; return 10; }
    8
}
/// $02B636: bra.w $2b63a
pub fn i_02b636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B63A;
    cpu.ir = 0x6000;
    cpu.pc = 0x2B63A;
    10
}
/// $02B63A: lea.l $2b38c(pc), a1
pub fn i_02b63a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B63E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B38C;
    16
}
/// $02B63E: move.l a1, (a6)
pub fn i_02b63e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B640;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B640: jsr $2a328(pc)
pub fn i_02b640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B644;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B644);
    cpu.pc = 0x2A328;
    16
}
/// $02B644: bcc.w $2b64e
pub fn i_02b644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B648;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2B64E; return 10; }
    8
}
/// $02B648: lea.l $2bf64(pc), a1
pub fn i_02b648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B64C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2BF64;
    16
}
/// $02B64C: move.l a1, (a6)
pub fn i_02b64c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B64E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B64E: jsr $2a34e(pc)
pub fn i_02b64e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B652;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B652);
    cpu.pc = 0x2A34E;
    16
}
/// $02B652: bcc.w $2b65c
pub fn i_02b652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B656;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2B65C; return 10; }
    8
}
/// $02B656: lea.l $2c07a(pc), a1
pub fn i_02b656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B65A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2C07A;
    16
}
/// $02B65A: move.l a1, (a6)
pub fn i_02b65a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B65C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B65C: tst.b $3b(a6)
pub fn i_02b65c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B660;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x3B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $02B660: bne.w $2b66a
pub fn i_02b660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B664;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2B66A; return 10; }
    8
}
/// $02B664: jsr $283d8.l
pub fn i_02b664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B66A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B66A);
    cpu.pc = 0x283D8;
    16
}
/// $02B66A: jsr $2a59a(pc)
pub fn i_02b66a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B66E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B66E);
    cpu.pc = 0x2A59A;
    16
}
/// $02B66E: bcc.w $2b678
pub fn i_02b66e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B672;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2B678; return 10; }
    8
}
/// $02B672: lea.l $2b6f4(pc), a1
pub fn i_02b672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B676;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B6F4;
    16
}
/// $02B676: move.l a1, (a6)
pub fn i_02b676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B678;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B678: jsr $2a8c0(pc)
pub fn i_02b678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B67C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B67C);
    cpu.pc = 0x2A8C0;
    16
}
/// $02B67C: bcc.w $2b686
pub fn i_02b67c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B680;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2B686; return 10; }
    8
}
/// $02B680: lea.l $2cffa(pc), a1
pub fn i_02b680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B684;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2CFFA;
    16
}
/// $02B684: move.l a1, (a6)
pub fn i_02b684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B686;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B686: jsr $2ab3c(pc)
pub fn i_02b686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B68A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B68A);
    cpu.pc = 0x2AB3C;
    16
}
/// $02B68A: lea.l $2c9b0(pc), a0
pub fn i_02b68a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B68E;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2C9B0;
    16
}
/// $02B68E: movea.l #$ffffffff, a1
pub fn i_02b68e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B694;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $02B694: jsr $772.l
pub fn i_02b694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B69A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B69A);
    cpu.pc = 0x772;
    16
}
/// $02B69A: jsr $2aac0(pc)
pub fn i_02b69a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B69E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B69E);
    cpu.pc = 0x2AAC0;
    16
}
/// $02B69E: bcc.w $2b6a8
pub fn i_02b69e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B6A2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2B6A8; return 10; }
    8
}
/// $02B6A2: lea.l $2c71e(pc), a1
pub fn i_02b6a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B6A6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2C71E;
    16
}
/// $02B6A6: move.l a1, (a6)
pub fn i_02b6a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B6A8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B6A8: jsr $2ac80(pc)
pub fn i_02b6a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B6AC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B6AC);
    cpu.pc = 0x2AC80;
    16
}
/// $02B6AC: bcc.w $2b6b6
pub fn i_02b6ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B6B0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2B6B6; return 10; }
    8
}
/// $02B6B0: lea.l $2dd74(pc), a1
pub fn i_02b6b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B6B4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DD74;
    16
}
/// $02B6B4: move.l a1, (a6)
pub fn i_02b6b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B6B6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B6B6: jsr $2a5cc(pc)
pub fn i_02b6b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B6BA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B6BA);
    cpu.pc = 0x2A5CC;
    16
}
/// $02B6BA: bcc.w $2b6c4
pub fn i_02b6ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B6BE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2B6C4; return 10; }
    8
}
/// $02B6BE: lea.l $2fc70(pc), a1
pub fn i_02b6be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B6C2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2FC70;
    16
}
/// $02B6C2: move.l a1, (a6)
pub fn i_02b6c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B6C4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B6C4: jsr $2a690(pc)
pub fn i_02b6c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B6C8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B6C8);
    cpu.pc = 0x2A690;
    16
}
/// $02B6C8: lea.l $2a060(pc), a0
pub fn i_02b6c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B6CC;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A060;
    16
}
/// $02B6CC: movea.l #$ffffffff, a1
pub fn i_02b6cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B6D2;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $02B6D2: jsr $772.l
pub fn i_02b6d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B6D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B6D8);
    cpu.pc = 0x772;
    16
}
/// $02B6D8: movea.l #$ffffffff, a0
pub fn i_02b6d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B6DE;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $02B6DE: lea.l $27964e.l, a0
pub fn i_02b6de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B6E4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27964E;
    20
}
/// $02B6E4: jsr $5dd56.l
pub fn i_02b6e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B6EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B6EA);
    cpu.pc = 0x5DD56;
    16
}
/// $02B6EA: bcc.w $2b6f2
pub fn i_02b6ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B6EE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2B6F2; return 10; }
    8
}
/// $02B6EE: jmp $2dcbc(pc)
pub fn i_02b6ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B6F2;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x2DCBC;
    8
}
/// $02B6F2: rts
pub fn i_02b6f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B6F4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_SlopeIdleEnter_02b6f4  ($02B6F4, 222 B, slug/states_02ae3e.s)
/// $02B6F4: bclr.b #$5, $8c(a6)
pub fn i_02b6f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B6FA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02B6FA: clr.w $28(a6)
pub fn i_02b6fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B6FE;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02B6FE: clr.w $2c(a6)
pub fn i_02b6fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B702;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02B702: jsr $2a958(pc)
pub fn i_02b702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B706;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B706);
    cpu.pc = 0x2A958;
    16
}
/// $02B706: move.w $80(a6), d2
pub fn i_02b706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B70A;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02B70A: move.w d2, d0
pub fn i_02b70a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B70C;
    cpu.ir = 0x3002;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02B70C: asr.w #$1, d0
pub fn i_02b70c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B70E;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02B70E: neg.w d0
pub fn i_02b70e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B710;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02B710: addi.w #$20, d0
pub fn i_02b710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B714;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02B714: cmpi.w #$40, d0
pub fn i_02b714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B718;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $02B718: bls.w $2b720
pub fn i_02b718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B71C;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x2B720; return 10; }
    8
}
/// $02B71C: move.w #$40, d0
pub fn i_02b71c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B720;
    cpu.ir = 0x303C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02B720: movem.w d0, -(a7)
pub fn i_02b720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B724;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $02B724: jsr $295a6(pc)
pub fn i_02b724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B728;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B728);
    cpu.pc = 0x295A6;
    16
}
/// $02B728: movea.l #$2b0cb8, a0
pub fn i_02b728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B72E;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2B0CB8;
    cpu.a[0] = v1;
    4
}
/// $02B72E: lsl.w #$2, d0
pub fn i_02b72e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B730;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02B730: movea.l (a0, d0.w), a0
pub fn i_02b730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B734;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02B734: cmpa.l #$ffffffff, a0
pub fn i_02b734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B73A;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02B73A: beq.w $2b744
pub fn i_02b73a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B73E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2B744; return 10; }
    8
}
/// $02B73E: jsr $28cd4.l
pub fn i_02b73e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B744;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B744);
    cpu.pc = 0x28CD4;
    16
}
/// $02B744: movem.w (a7)+, d0
pub fn i_02b744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B748;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $02B748: move.b #$41, $20(a6)
pub fn i_02b748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B74E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x41;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02B74E: lsr.w #$2, d0
pub fn i_02b74e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B750;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02B750: andi.w #$ff, d0
pub fn i_02b750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B754;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02B754: add.b d0, $20(a6)
pub fn i_02b754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B758;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02B758: move.b #$ff, $8b(a6)
pub fn i_02b758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B75E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02B75E: lea.l $2b764(pc), a1
pub fn i_02b75e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B762;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B764;
    16
}
/// $02B762: move.l a1, (a6)
pub fn i_02b762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B764;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B764: bset.b #$2, $8d(a6)
pub fn i_02b764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B76A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02B76A: jsr $2aa24(pc)
pub fn i_02b76a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B76E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B76E);
    cpu.pc = 0x2AA24;
    16
}
/// $02B76E: jsr $2a478(pc)
pub fn i_02b76e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B772;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B772);
    cpu.pc = 0x2A478;
    16
}
/// $02B772: jsr $2a760(pc)
pub fn i_02b772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B776;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B776);
    cpu.pc = 0x2A760;
    16
}
/// $02B776: jsr $28d70.l
pub fn i_02b776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B77C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B77C);
    cpu.pc = 0x28D70;
    16
}
/// $02B77C: bcc.w $2b786
pub fn i_02b77c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B780;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2B786; return 10; }
    8
}
/// $02B780: lea.l $2ecc2(pc), a1
pub fn i_02b780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B784;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2ECC2;
    16
}
/// $02B784: move.l a1, (a6)
pub fn i_02b784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B786;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B786: jsr $2a8c0(pc)
pub fn i_02b786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B78A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B78A);
    cpu.pc = 0x2A8C0;
    16
}
/// $02B78A: bcc.w $2b794
pub fn i_02b78a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B78E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2B794; return 10; }
    8
}
/// $02B78E: lea.l $2cffa(pc), a1
pub fn i_02b78e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B792;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2CFFA;
    16
}
/// $02B792: move.l a1, (a6)
pub fn i_02b792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B794;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B794: jsr $2aaf0(pc)
pub fn i_02b794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B798;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B798);
    cpu.pc = 0x2AAF0;
    16
}
/// $02B798: lea.l $2c9b0(pc), a0
pub fn i_02b798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B79C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2C9B0;
    16
}
/// $02B79C: movea.l #$ffffffff, a1
pub fn i_02b79c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B7A2;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $02B7A2: jsr $772.l
pub fn i_02b7a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B7A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B7A8);
    cpu.pc = 0x772;
    16
}
/// $02B7A8: jsr $2ac80(pc)
pub fn i_02b7a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B7AC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B7AC);
    cpu.pc = 0x2AC80;
    16
}
/// $02B7AC: bcc.w $2b7b6
pub fn i_02b7ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B7B0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2B7B6; return 10; }
    8
}
/// $02B7B0: lea.l $2dd74(pc), a1
pub fn i_02b7b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B7B4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DD74;
    16
}
/// $02B7B4: move.l a1, (a6)
pub fn i_02b7b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B7B6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B7B6: jsr $2a5cc(pc)
pub fn i_02b7b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B7BA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B7BA);
    cpu.pc = 0x2A5CC;
    16
}
/// $02B7BA: bcc.w $2b7c4
pub fn i_02b7ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B7BE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2B7C4; return 10; }
    8
}
/// $02B7BE: lea.l $2fc70(pc), a1
pub fn i_02b7be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B7C2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2FC70;
    16
}
/// $02B7C2: move.l a1, (a6)
pub fn i_02b7c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B7C4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B7C4: jsr $2a690(pc)
pub fn i_02b7c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B7C8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B7C8);
    cpu.pc = 0x2A690;
    16
}
/// $02B7C8: lea.l $2a060(pc), a0
pub fn i_02b7c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B7CC;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A060;
    16
}
/// $02B7CC: movea.l #$ffffffff, a1
pub fn i_02b7cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B7D2;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
// ===== JsrAbsThunk_02b7d2  ($02B7D2, 8 B, thunks/jsr_abs_thunks.c)
/// $02B7D2: jsr $772.l
pub fn i_02b7d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B7D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B7D8);
    cpu.pc = 0x772;
    16
}
/// $02B7D8: rts
pub fn i_02b7d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B7DA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_SlopeMount_02b7da  ($02B7DA, 236 B, slug/states_02ae3e.s)
/// $02B7DA: bclr.b #$5, $8c(a6)
pub fn i_02b7da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B7E0;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02B7E0: clr.w $28(a6)
pub fn i_02b7e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B7E4;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02B7E4: clr.w $2c(a6)
pub fn i_02b7e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B7E8;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02B7E8: jsr $2a958(pc)
pub fn i_02b7e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B7EC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B7EC);
    cpu.pc = 0x2A958;
    16
}
/// $02B7EC: move.w $80(a6), d2
pub fn i_02b7ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B7F0;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02B7F0: move.w d2, d0
pub fn i_02b7f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B7F2;
    cpu.ir = 0x3002;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02B7F2: asr.w #$1, d0
pub fn i_02b7f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B7F4;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02B7F4: neg.w d0
pub fn i_02b7f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B7F6;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02B7F6: addi.w #$20, d0
pub fn i_02b7f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B7FA;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02B7FA: cmpi.w #$40, d0
pub fn i_02b7fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B7FE;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $02B7FE: bls.w $2b806
pub fn i_02b7fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B802;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x2B806; return 10; }
    8
}
/// $02B802: move.w #$40, d0
pub fn i_02b802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B806;
    cpu.ir = 0x303C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02B806: movem.w d0, -(a7)
pub fn i_02b806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B80A;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $02B80A: jsr $295a6(pc)
pub fn i_02b80a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B80E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B80E);
    cpu.pc = 0x295A6;
    16
}
/// $02B80E: movea.l #$2b0cfc, a0
pub fn i_02b80e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B814;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2B0CFC;
    cpu.a[0] = v1;
    4
}
/// $02B814: lsl.w #$2, d0
pub fn i_02b814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B816;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02B816: movea.l (a0, d0.w), a0
pub fn i_02b816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B81A;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02B81A: cmpa.l #$ffffffff, a0
pub fn i_02b81a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B820;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02B820: beq.w $2b82a
pub fn i_02b820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B824;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2B82A; return 10; }
    8
}
/// $02B824: jsr $28cd4.l
pub fn i_02b824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B82A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B82A);
    cpu.pc = 0x28CD4;
    16
}
/// $02B82A: movem.w (a7)+, d0
pub fn i_02b82a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B82E;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $02B82E: move.b #$82, $20(a6)
pub fn i_02b82e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B834;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x82;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02B834: lsr.w #$2, d0
pub fn i_02b834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B836;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02B836: andi.w #$ff, d0
pub fn i_02b836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B83A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02B83A: add.b d0, $20(a6)
pub fn i_02b83a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B83E;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02B83E: move.b #$ff, $8b(a6)
pub fn i_02b83e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B844;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02B844: lea.l $2b84a(pc), a1
pub fn i_02b844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B848;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B84A;
    16
}
/// $02B848: move.l a1, (a6)
pub fn i_02b848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B84A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B84A: bset.b #$2, $8d(a6)
pub fn i_02b84a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B850;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02B850: jsr $2aa24(pc)
pub fn i_02b850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B854;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B854);
    cpu.pc = 0x2AA24;
    16
}
/// $02B854: jsr $2a478(pc)
pub fn i_02b854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B858;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B858);
    cpu.pc = 0x2A478;
    16
}
/// $02B858: jsr $2a760(pc)
pub fn i_02b858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B85C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B85C);
    cpu.pc = 0x2A760;
    16
}
/// $02B85C: jsr $28d70.l
pub fn i_02b85c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B862;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B862);
    cpu.pc = 0x28D70;
    16
}
/// $02B862: bcc.w $2b86c
pub fn i_02b862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B866;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2B86C; return 10; }
    8
}
/// $02B866: lea.l $2b4d2(pc), a1
pub fn i_02b866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B86A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B4D2;
    16
}
/// $02B86A: move.l a1, (a6)
pub fn i_02b86a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B86C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B86C: jsr $2a8c0(pc)
pub fn i_02b86c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B870;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B870);
    cpu.pc = 0x2A8C0;
    16
}
/// $02B870: bcc.w $2b87a
pub fn i_02b870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B874;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2B87A; return 10; }
    8
}
/// $02B874: lea.l $2cffa(pc), a1
pub fn i_02b874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B878;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2CFFA;
    16
}
/// $02B878: move.l a1, (a6)
pub fn i_02b878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B87A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B87A: jsr $2aaf0(pc)
pub fn i_02b87a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B87E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B87E);
    cpu.pc = 0x2AAF0;
    16
}
/// $02B87E: lea.l $2c9b0(pc), a0
pub fn i_02b87e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B882;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2C9B0;
    16
}
/// $02B882: movea.l #$ffffffff, a1
pub fn i_02b882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B888;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $02B888: jsr $772.l
pub fn i_02b888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B88E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B88E);
    cpu.pc = 0x772;
    16
}
/// $02B88E: jsr $2aac0(pc)
pub fn i_02b88e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B892;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B892);
    cpu.pc = 0x2AAC0;
    16
}
/// $02B892: bcc.w $2b89c
pub fn i_02b892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B896;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2B89C; return 10; }
    8
}
/// $02B896: lea.l $2c71e(pc), a1
pub fn i_02b896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B89A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2C71E;
    16
}
/// $02B89A: move.l a1, (a6)
pub fn i_02b89a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B89C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B89C: jsr $2ac80(pc)
pub fn i_02b89c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B8A0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B8A0);
    cpu.pc = 0x2AC80;
    16
}
/// $02B8A0: bcc.w $2b8aa
pub fn i_02b8a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B8A4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2B8AA; return 10; }
    8
}
/// $02B8A4: lea.l $2dd74(pc), a1
pub fn i_02b8a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B8A8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DD74;
    16
}
/// $02B8A8: move.l a1, (a6)
pub fn i_02b8a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B8AA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B8AA: jsr $2a5cc(pc)
pub fn i_02b8aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B8AE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B8AE);
    cpu.pc = 0x2A5CC;
    16
}
/// $02B8AE: bcc.w $2b8b8
pub fn i_02b8ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B8B2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2B8B8; return 10; }
    8
}
/// $02B8B2: lea.l $2fc70(pc), a1
pub fn i_02b8b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B8B6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2FC70;
    16
}
/// $02B8B6: move.l a1, (a6)
pub fn i_02b8b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B8B8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B8B8: jsr $2a690(pc)
pub fn i_02b8b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B8BC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B8BC);
    cpu.pc = 0x2A690;
    16
}
/// $02B8BC: lea.l $2a060(pc), a0
pub fn i_02b8bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B8C0;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A060;
    16
}
/// $02B8C0: movea.l #$ffffffff, a1
pub fn i_02b8c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B8C6;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
// ===== JsrAbsThunk_02b8c6  ($02B8C6, 8 B, thunks/jsr_abs_thunks.c)
/// $02B8C6: jsr $772.l
pub fn i_02b8c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B8CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B8CC);
    cpu.pc = 0x772;
    16
}
/// $02B8CC: rts
pub fn i_02b8cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B8CE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_AccelRightB_02b8d8  ($02B8D8, 340 B, slug/states_02ae3e.s)
/// $02B8D8: jsr $31e5e.l
pub fn i_02b8d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B8DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B8DE);
    cpu.pc = 0x31E5E;
    16
}
/// $02B8DE: move.w #$2a0, d0
pub fn i_02b8de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B8E2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2A0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02B8E2: sub.w $28(a6), d0
pub fn i_02b8e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B8E6;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $02B8E6: asr.w #$3, d0
pub fn i_02b8e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B8E8;
    cpu.ir = 0xE640;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $02B8E8: move.w d0, $2c(a6)
pub fn i_02b8e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B8EC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02B8EC: move.b #$8, $91(a6)
pub fn i_02b8ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B8F2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x91);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02B8F2: jsr $2a9a0(pc)
pub fn i_02b8f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B8F6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B8F6);
    cpu.pc = 0x2A9A0;
    16
}
/// $02B8F6: movea.l #$27945c, a0
pub fn i_02b8f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B8FC;
    cpu.ir = 0x207C;
    let v1: u32 = 0x27945C;
    cpu.a[0] = v1;
    4
}
/// $02B8FC: lsl.w #$2, d0
pub fn i_02b8fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B8FE;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02B8FE: movea.l (a0, d0.w), a0
pub fn i_02b8fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B902;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02B902: cmpa.l #$ffffffff, a0
pub fn i_02b902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B908;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02B908: beq.w $2b912
pub fn i_02b908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B90C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2B912; return 10; }
    8
}
/// $02B90C: jsr $28cd4.l
pub fn i_02b90c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B912;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B912);
    cpu.pc = 0x28CD4;
    16
}
/// $02B912: movem.w d0, -(a7)
pub fn i_02b912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B916;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $02B916: lsr.w #$1, d0
pub fn i_02b916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B918;
    cpu.ir = 0xE248;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02B918: lea.l $2b8ce(pc), a0
pub fn i_02b918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B91C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2B8CE;
    16
}
/// $02B91C: move.w (a0, d0.w), d0
pub fn i_02b91c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B920;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $02B920: cmpi.w #$ffff, d0
pub fn i_02b920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B924;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFFF, Size::Word, false);
    8
}
/// $02B924: beq.w $2b92e
pub fn i_02b924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B928;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2B92E; return 10; }
    8
}
/// $02B928: jsr $2352.l
pub fn i_02b928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B92E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B92E);
    cpu.pc = 0x2352;
    16
}
/// $02B92E: movem.w (a7)+, d0
pub fn i_02b92e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B932;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $02B932: move.b #$19, $20(a6)
pub fn i_02b932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B938;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x19;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02B938: andi.w #$ff, d0
pub fn i_02b938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B93C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02B93C: lsr.w #$2, d0
pub fn i_02b93c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B93E;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02B93E: add.b d0, $20(a6)
pub fn i_02b93e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B942;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02B942: lea.l $2b948(pc), a1
pub fn i_02b942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B946;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B948;
    16
}
/// $02B946: move.l a1, (a6)
pub fn i_02b946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B948;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B948: subq.b #$1, $91(a6)
pub fn i_02b948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B94C;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x91);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02B94C: bne.w $2b960
pub fn i_02b94c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B950;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2B960; return 10; }
    8
}
/// $02B950: move.w #$2a0, $28(a6)
pub fn i_02b950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B956;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2A0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02B956: clr.w $2c(a6)
pub fn i_02b956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B95A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02B95A: lea.l $2b960(pc), a1
pub fn i_02b95a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B95E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B960;
    16
}
/// $02B95E: move.l a1, (a6)
pub fn i_02b95e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B960;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B960: jsr $2aa24(pc)
pub fn i_02b960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B964;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B964);
    cpu.pc = 0x2AA24;
    16
}
/// $02B964: jsr $2a7d8(pc)
pub fn i_02b964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B968;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B968);
    cpu.pc = 0x2A7D8;
    16
}
/// $02B968: bcc.w $2b972
pub fn i_02b968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B96C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2B972; return 10; }
    8
}
/// $02B96C: lea.l $2b4ca(pc), a1
pub fn i_02b96c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B970;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B4CA;
    16
}
/// $02B970: move.l a1, (a6)
pub fn i_02b970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B972;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B972: jsr $2a4f0(pc)
pub fn i_02b972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B976;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B976);
    cpu.pc = 0x2A4F0;
    16
}
/// $02B976: bcc.w $2b980
pub fn i_02b976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B97A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2B980; return 10; }
    8
}
/// $02B97A: lea.l $2b4ca(pc), a1
pub fn i_02b97a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B97E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B4CA;
    16
}
/// $02B97E: move.l a1, (a6)
pub fn i_02b97e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B980;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B980: jsr $28d70.l
pub fn i_02b980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B986;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B986);
    cpu.pc = 0x28D70;
    16
}
/// $02B986: bcc.w $2b990
pub fn i_02b986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B98A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2B990; return 10; }
    8
}
/// $02B98A: lea.l $2bf64(pc), a1
pub fn i_02b98a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B98E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2BF64;
    16
}
/// $02B98E: move.l a1, (a6)
pub fn i_02b98e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B990;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B990: jsr $283d8.l
pub fn i_02b990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B996;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B996);
    cpu.pc = 0x283D8;
    16
}
/// $02B996: jsr $2a328(pc)
pub fn i_02b996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B99A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B99A);
    cpu.pc = 0x2A328;
    16
}
/// $02B99A: bcs.w $2b9a4
pub fn i_02b99a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B99E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2B9A4; return 10; }
    8
}
/// $02B99E: lea.l $2bd00(pc), a1
pub fn i_02b99e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B9A2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2BD00;
    16
}
/// $02B9A2: move.l a1, (a6)
pub fn i_02b9a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B9A4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B9A4: jsr $2a34e(pc)
pub fn i_02b9a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B9A8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B9A8);
    cpu.pc = 0x2A34E;
    16
}
/// $02B9A8: bcc.w $2b9b2
pub fn i_02b9a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B9AC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2B9B2; return 10; }
    8
}
/// $02B9AC: lea.l $2c07a(pc), a1
pub fn i_02b9ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B9B0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2C07A;
    16
}
/// $02B9B0: move.l a1, (a6)
pub fn i_02b9b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B9B2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B9B2: jsr $2a59a(pc)
pub fn i_02b9b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B9B6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B9B6);
    cpu.pc = 0x2A59A;
    16
}
/// $02B9B6: bcc.w $2b9d2
pub fn i_02b9b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B9BA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2B9D2; return 10; }
    8
}
/// $02B9BA: jsr $2a9a0(pc)
pub fn i_02b9ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B9BE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B9BE);
    cpu.pc = 0x2A9A0;
    16
}
/// $02B9BE: andi.w #$ff, d0
pub fn i_02b9be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B9C2;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02B9C2: lea.l $2a078(pc), a0
pub fn i_02b9c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B9C6;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A078;
    16
}
/// $02B9C6: movea.l #$ffffffff, a1
pub fn i_02b9c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B9CC;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $02B9CC: jsr $772.l
pub fn i_02b9cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B9D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B9D2);
    cpu.pc = 0x772;
    16
}
/// $02B9D2: jsr $2a8c0(pc)
pub fn i_02b9d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B9D6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B9D6);
    cpu.pc = 0x2A8C0;
    16
}
/// $02B9D6: bcc.w $2b9e0
pub fn i_02b9d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B9DA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2B9E0; return 10; }
    8
}
/// $02B9DA: lea.l $2d02e(pc), a1
pub fn i_02b9da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B9DE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D02E;
    16
}
/// $02B9DE: move.l a1, (a6)
pub fn i_02b9de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B9E0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02B9E0: jsr $2aaf0(pc)
pub fn i_02b9e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B9E4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B9E4);
    cpu.pc = 0x2AAF0;
    16
}
/// $02B9E4: lea.l $2c9b0(pc), a0
pub fn i_02b9e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B9E8;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2C9B0;
    16
}
/// $02B9E8: movea.l #$ffffffff, a1
pub fn i_02b9e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B9EE;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $02B9EE: jsr $772.l
pub fn i_02b9ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B9F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2B9F4);
    cpu.pc = 0x772;
    16
}
/// $02B9F4: jsr $2aac0(pc)
pub fn i_02b9f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B9F8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2B9F8);
    cpu.pc = 0x2AAC0;
    16
}
/// $02B9F8: bcc.w $2ba02
pub fn i_02b9f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B9FC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2BA02; return 10; }
    8
}
/// $02B9FC: lea.l $2c24a(pc), a1
pub fn i_02b9fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BA00;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2C24A;
    16
}
/// $02BA00: move.l a1, (a6)
pub fn i_02ba00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BA02;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02BA02: jsr $2ac80(pc)
pub fn i_02ba02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BA06;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BA06);
    cpu.pc = 0x2AC80;
    16
}
/// $02BA06: bcc.w $2ba10
pub fn i_02ba06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BA0A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2BA10; return 10; }
    8
}
/// $02BA0A: lea.l $2dd74(pc), a1
pub fn i_02ba0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BA0E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DD74;
    16
}
/// $02BA0E: move.l a1, (a6)
pub fn i_02ba0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BA10;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02BA10: jsr $2a5cc(pc)
pub fn i_02ba10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BA14;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BA14);
    cpu.pc = 0x2A5CC;
    16
}
/// $02BA14: bcc.w $2ba1e
pub fn i_02ba14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BA18;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2BA1E; return 10; }
    8
}
/// $02BA18: lea.l $2bbf2(pc), a1
pub fn i_02ba18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BA1C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2BBF2;
    16
}
/// $02BA1C: move.l a1, (a6)
pub fn i_02ba1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BA1E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02BA1E: jsr $2a690(pc)
pub fn i_02ba1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BA22;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BA22);
    cpu.pc = 0x2A690;
    16
}
/// $02BA22: lea.l $2a060(pc), a0
pub fn i_02ba22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BA26;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A060;
    16
}
/// $02BA26: movea.l #$ffffffff, a1
pub fn i_02ba26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BA2C;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
// ===== JsrAbsThunk_02ba2c  ($02BA2C, 8 B, thunks/jsr_abs_thunks.c)
/// $02BA2C: jsr $772.l
pub fn i_02ba2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BA32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2BA32);
    cpu.pc = 0x772;
    16
}
/// $02BA32: rts
pub fn i_02ba32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BA34;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_AccelLeftB_02ba3e  ($02BA3E, 340 B, slug/states_02ae3e.s)
/// $02BA3E: jsr $31e76.l
pub fn i_02ba3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BA44;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2BA44);
    cpu.pc = 0x31E76;
    16
}
/// $02BA44: move.w #$fd60, d0
pub fn i_02ba44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BA48;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFD60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02BA48: sub.w $28(a6), d0
pub fn i_02ba48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BA4C;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $02BA4C: asr.w #$3, d0
pub fn i_02ba4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BA4E;
    cpu.ir = 0xE640;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $02BA4E: move.w d0, $2c(a6)
pub fn i_02ba4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BA52;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02BA52: move.b #$8, $91(a6)
pub fn i_02ba52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BA58;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x91);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02BA58: jsr $2a9a0(pc)
pub fn i_02ba58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BA5C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BA5C);
    cpu.pc = 0x2A9A0;
    16
}
/// $02BA5C: movea.l #$279538, a0
pub fn i_02ba5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BA62;
    cpu.ir = 0x207C;
    let v1: u32 = 0x279538;
    cpu.a[0] = v1;
    4
}
/// $02BA62: lsl.w #$2, d0
pub fn i_02ba62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BA64;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02BA64: movea.l (a0, d0.w), a0
pub fn i_02ba64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BA68;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02BA68: cmpa.l #$ffffffff, a0
pub fn i_02ba68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BA6E;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02BA6E: beq.w $2ba78
pub fn i_02ba6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BA72;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2BA78; return 10; }
    8
}
/// $02BA72: jsr $28cd4.l
pub fn i_02ba72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BA78;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2BA78);
    cpu.pc = 0x28CD4;
    16
}
/// $02BA78: movem.w d0, -(a7)
pub fn i_02ba78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BA7C;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $02BA7C: lsr.w #$1, d0
pub fn i_02ba7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BA7E;
    cpu.ir = 0xE248;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02BA7E: lea.l $2ba34(pc), a0
pub fn i_02ba7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BA82;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2BA34;
    16
}
/// $02BA82: move.w (a0, d0.w), d0
pub fn i_02ba82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BA86;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $02BA86: cmpi.w #$ffff, d0
pub fn i_02ba86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BA8A;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFFF, Size::Word, false);
    8
}
/// $02BA8A: beq.w $2ba94
pub fn i_02ba8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BA8E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2BA94; return 10; }
    8
}
/// $02BA8E: jsr $2352.l
pub fn i_02ba8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BA94;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2BA94);
    cpu.pc = 0x2352;
    16
}
/// $02BA94: movem.w (a7)+, d0
pub fn i_02ba94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BA98;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $02BA98: move.b #$1e, $20(a6)
pub fn i_02ba98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BA9E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02BA9E: andi.w #$ff, d0
pub fn i_02ba9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BAA2;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02BAA2: lsr.w #$2, d0
pub fn i_02baa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BAA4;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02BAA4: add.b d0, $20(a6)
pub fn i_02baa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BAA8;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02BAA8: lea.l $2baae(pc), a1
pub fn i_02baa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BAAC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2BAAE;
    16
}
/// $02BAAC: move.l a1, (a6)
pub fn i_02baac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BAAE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02BAAE: subq.b #$1, $91(a6)
pub fn i_02baae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BAB2;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x91);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02BAB2: bne.w $2bac6
pub fn i_02bab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BAB6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2BAC6; return 10; }
    8
}
/// $02BAB6: move.w #$fd60, $28(a6)
pub fn i_02bab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BABC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFD60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02BABC: clr.w $2c(a6)
pub fn i_02babc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BAC0;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02BAC0: lea.l $2bac6(pc), a1
pub fn i_02bac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BAC4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2BAC6;
    16
}
/// $02BAC4: move.l a1, (a6)
pub fn i_02bac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BAC6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02BAC6: jsr $2aa24(pc)
pub fn i_02bac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BACA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BACA);
    cpu.pc = 0x2AA24;
    16
}
/// $02BACA: jsr $2a7d8(pc)
pub fn i_02baca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BACE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BACE);
    cpu.pc = 0x2A7D8;
    16
}
/// $02BACE: bcc.w $2bad8
pub fn i_02bace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BAD2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2BAD8; return 10; }
    8
}
/// $02BAD2: lea.l $2b4ce(pc), a1
pub fn i_02bad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BAD6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B4CE;
    16
}
/// $02BAD6: move.l a1, (a6)
pub fn i_02bad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BAD8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02BAD8: jsr $2a4f0(pc)
pub fn i_02bad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BADC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BADC);
    cpu.pc = 0x2A4F0;
    16
}
/// $02BADC: bcc.w $2bae6
pub fn i_02badc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BAE0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2BAE6; return 10; }
    8
}
/// $02BAE0: lea.l $2b4ce(pc), a1
pub fn i_02bae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BAE4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B4CE;
    16
}
/// $02BAE4: move.l a1, (a6)
pub fn i_02bae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BAE6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02BAE6: jsr $28d70.l
pub fn i_02bae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BAEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2BAEC);
    cpu.pc = 0x28D70;
    16
}
/// $02BAEC: bcc.w $2baf6
pub fn i_02baec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BAF0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2BAF6; return 10; }
    8
}
/// $02BAF0: lea.l $2c07a(pc), a1
pub fn i_02baf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BAF4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2C07A;
    16
}
/// $02BAF4: move.l a1, (a6)
pub fn i_02baf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BAF6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02BAF6: jsr $283d8.l
pub fn i_02baf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BAFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2BAFC);
    cpu.pc = 0x283D8;
    16
}
/// $02BAFC: jsr $2a34e(pc)
pub fn i_02bafc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BB00;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BB00);
    cpu.pc = 0x2A34E;
    16
}
/// $02BB00: bcs.w $2bb0a
pub fn i_02bb00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BB04;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2BB0A; return 10; }
    8
}
/// $02BB04: lea.l $2be32(pc), a1
pub fn i_02bb04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BB08;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2BE32;
    16
}
/// $02BB08: move.l a1, (a6)
pub fn i_02bb08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BB0A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02BB0A: jsr $2a328(pc)
pub fn i_02bb0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BB0E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BB0E);
    cpu.pc = 0x2A328;
    16
}
/// $02BB0E: bcc.w $2bb18
pub fn i_02bb0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BB12;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2BB18; return 10; }
    8
}
/// $02BB12: lea.l $2bf64(pc), a1
pub fn i_02bb12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BB16;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2BF64;
    16
}
/// $02BB16: move.l a1, (a6)
pub fn i_02bb16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BB18;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02BB18: jsr $2a59a(pc)
pub fn i_02bb18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BB1C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BB1C);
    cpu.pc = 0x2A59A;
    16
}
/// $02BB1C: bcc.w $2bb38
pub fn i_02bb1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BB20;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2BB38; return 10; }
    8
}
/// $02BB20: jsr $2a9a0(pc)
pub fn i_02bb20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BB24;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BB24);
    cpu.pc = 0x2A9A0;
    16
}
/// $02BB24: andi.w #$ff, d0
pub fn i_02bb24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BB28;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02BB28: lea.l $2a078(pc), a0
pub fn i_02bb28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BB2C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A078;
    16
}
/// $02BB2C: movea.l #$ffffffff, a1
pub fn i_02bb2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BB32;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $02BB32: jsr $772.l
pub fn i_02bb32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BB38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2BB38);
    cpu.pc = 0x772;
    16
}
/// $02BB38: jsr $2a8c0(pc)
pub fn i_02bb38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BB3C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BB3C);
    cpu.pc = 0x2A8C0;
    16
}
/// $02BB3C: bcc.w $2bb46
pub fn i_02bb3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BB40;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2BB46; return 10; }
    8
}
/// $02BB40: lea.l $2d02e(pc), a1
pub fn i_02bb40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BB44;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D02E;
    16
}
/// $02BB44: move.l a1, (a6)
pub fn i_02bb44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BB46;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02BB46: jsr $2aaf0(pc)
pub fn i_02bb46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BB4A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BB4A);
    cpu.pc = 0x2AAF0;
    16
}
/// $02BB4A: lea.l $2c9b0(pc), a0
pub fn i_02bb4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BB4E;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2C9B0;
    16
}
/// $02BB4E: movea.l #$ffffffff, a1
pub fn i_02bb4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BB54;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $02BB54: jsr $772.l
pub fn i_02bb54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BB5A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2BB5A);
    cpu.pc = 0x772;
    16
}
/// $02BB5A: jsr $2aac0(pc)
pub fn i_02bb5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BB5E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BB5E);
    cpu.pc = 0x2AAC0;
    16
}
/// $02BB5E: bcc.w $2bb68
pub fn i_02bb5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BB62;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2BB68; return 10; }
    8
}
/// $02BB62: lea.l $2c24a(pc), a1
pub fn i_02bb62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BB66;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2C24A;
    16
}
/// $02BB66: move.l a1, (a6)
pub fn i_02bb66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BB68;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02BB68: jsr $2ac80(pc)
pub fn i_02bb68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BB6C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BB6C);
    cpu.pc = 0x2AC80;
    16
}
/// $02BB6C: bcc.w $2bb76
pub fn i_02bb6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BB70;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2BB76; return 10; }
    8
}
/// $02BB70: lea.l $2dd74(pc), a1
pub fn i_02bb70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BB74;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DD74;
    16
}
/// $02BB74: move.l a1, (a6)
pub fn i_02bb74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BB76;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02BB76: jsr $2a5cc(pc)
pub fn i_02bb76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BB7A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BB7A);
    cpu.pc = 0x2A5CC;
    16
}
/// $02BB7A: bcc.w $2bb84
pub fn i_02bb7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BB7E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2BB84; return 10; }
    8
}
/// $02BB7E: lea.l $2bbf2(pc), a1
pub fn i_02bb7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BB82;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2BBF2;
    16
}
/// $02BB82: move.l a1, (a6)
pub fn i_02bb82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BB84;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02BB84: jsr $2a690(pc)
pub fn i_02bb84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BB88;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BB88);
    cpu.pc = 0x2A690;
    16
}
/// $02BB88: lea.l $2a060(pc), a0
pub fn i_02bb88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BB8C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A060;
    16
}
/// $02BB8C: movea.l #$ffffffff, a1
pub fn i_02bb8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BB92;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
// ===== JsrAbsThunk_02bb92  ($02BB92, 8 B, thunks/jsr_abs_thunks.c)
/// $02BB92: jsr $772.l
pub fn i_02bb92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BB98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2BB98);
    cpu.pc = 0x772;
    16
}
/// $02BB98: rts
pub fn i_02bb98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BB9A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_DestroyedSlide_02bba4  ($02BBA4, 78 B, slug/states_02ae3e.s)
/// $02BBA4: move.w #$600, $28(a6)
pub fn i_02bba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BBAA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x600;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02BBAA: clr.w $2c(a6)
pub fn i_02bbaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BBAE;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02BBAE: jsr $2a9a0(pc)
pub fn i_02bbae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BBB2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BBB2);
    cpu.pc = 0x2A9A0;
    16
}
/// $02BBB2: movea.l #$27936c, a0
pub fn i_02bbb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BBB8;
    cpu.ir = 0x207C;
    let v1: u32 = 0x27936C;
    cpu.a[0] = v1;
    4
}
/// $02BBB8: lsl.w #$2, d0
pub fn i_02bbb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BBBA;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02BBBA: movea.l (a0, d0.w), a0
pub fn i_02bbba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BBBE;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02BBBE: cmpa.l #$ffffffff, a0
pub fn i_02bbbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BBC4;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02BBC4: beq.w $2bbce
pub fn i_02bbc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BBC8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2BBCE; return 10; }
    8
}
/// $02BBC8: jsr $28cd4.l
pub fn i_02bbc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BBCE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2BBCE);
    cpu.pc = 0x28CD4;
    16
}
/// $02BBCE: move.b #$19, $20(a6)
pub fn i_02bbce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BBD4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x19;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02BBD4: andi.w #$ff, d0
pub fn i_02bbd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BBD8;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02BBD8: lsr.w #$2, d0
pub fn i_02bbd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BBDA;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02BBDA: add.b d0, $20(a6)
pub fn i_02bbda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BBDE;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02BBDE: move.w #$10b2, d0
pub fn i_02bbde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BBE2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10B2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02BBE2: jsr $2352.l
pub fn i_02bbe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BBE8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2BBE8);
    cpu.pc = 0x2352;
    16
}
/// $02BBE8: lea.l $2bc78(pc), a1
pub fn i_02bbe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BBEC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2BC78;
    16
}
/// $02BBEC: move.l a1, (a6)
pub fn i_02bbec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BBEE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02BBEE: bra.w $2bc78
pub fn i_02bbee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BBF2;
    cpu.ir = 0x6000;
    cpu.pc = 0x2BC78;
    10
}
// ===== Slug_DestroyedSlideInit_02bbf2  ($02BBF2, 134 B, slug/states_02ae3e.s)
/// $02BBF2: move.w #$10e9, d0
pub fn i_02bbf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BBF6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10E9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02BBF6: jsr $2352.l
pub fn i_02bbf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BBFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2BBFC);
    cpu.pc = 0x2352;
    16
}
/// $02BBFC: jsr $2feda(pc)
pub fn i_02bbfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BC00;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BC00);
    cpu.pc = 0x2FEDA;
    16
}
/// $02BC00: move.w #$600, d0
pub fn i_02bc00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BC04;
    cpu.ir = 0x303C;
    let v1: u32 = 0x600;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02BC04: move.w d0, $28(a6)
pub fn i_02bc04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BC08;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02BC08: move.b #$30, $92(a6)
pub fn i_02bc08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BC0E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x92);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02BC0E: jsr $2a9a0(pc)
pub fn i_02bc0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BC12;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BC12);
    cpu.pc = 0x2A9A0;
    16
}
/// $02BC12: movea.l #$27936c, a0
pub fn i_02bc12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BC18;
    cpu.ir = 0x207C;
    let v1: u32 = 0x27936C;
    cpu.a[0] = v1;
    4
}
/// $02BC18: lsl.w #$2, d0
pub fn i_02bc18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BC1A;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02BC1A: movea.l (a0, d0.w), a0
pub fn i_02bc1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BC1E;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02BC1E: cmpa.l #$ffffffff, a0
pub fn i_02bc1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BC24;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02BC24: beq.w $2bc2e
pub fn i_02bc24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BC28;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2BC2E; return 10; }
    8
}
/// $02BC28: jsr $28cd4.l
pub fn i_02bc28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BC2E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2BC2E);
    cpu.pc = 0x28CD4;
    16
}
/// $02BC2E: move.b #$19, $20(a6)
pub fn i_02bc2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BC34;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x19;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02BC34: andi.w #$ff, d0
pub fn i_02bc34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BC38;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02BC38: lsr.w #$2, d0
pub fn i_02bc38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BC3A;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02BC3A: add.b d0, $20(a6)
pub fn i_02bc3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BC3E;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02BC3E: move.w #$10b2, d0
pub fn i_02bc3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BC42;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10B2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02BC42: jsr $2352.l
pub fn i_02bc42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BC48;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2BC48);
    cpu.pc = 0x2352;
    16
}
/// $02BC48: move.l #$2964c, $60(a6)
pub fn i_02bc48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BC50;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2964C;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02BC50: lea.l $2bc56(pc), a1
pub fn i_02bc50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BC54;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2BC56;
    16
}
/// $02BC54: move.l a1, (a6)
pub fn i_02bc54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BC56;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02BC56: lea.l $ffff.w, a0
pub fn i_02bc56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BC5A;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $02BC5A: move.l a0, $48(a6)
pub fn i_02bc5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BC5E;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02BC5E: cmpi.b #$29, $92(a6)
pub fn i_02bc5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BC64;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x92);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x29, Size::Byte, false);
    16
}
/// $02BC64: bhi.w $2bc78
pub fn i_02bc64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BC68;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x2BC78; return 10; }
    8
}
/// $02BC68: move.w #$600, $28(a6)
pub fn i_02bc68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BC6E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x600;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02BC6E: clr.w $2c(a6)
pub fn i_02bc6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BC72;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02BC72: lea.l $2bc78(pc), a1
pub fn i_02bc72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BC76;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2BC78;
    16
}
/// $02BC76: move.l a1, (a6)
pub fn i_02bc76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BC78;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Slug_DestroyedSlide_Loop_02bc78  ($02BC78, 130 B, slug/states_02ae3e.s)
/// $02BC78: jsr $2aa24(pc)
pub fn i_02bc78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BC7C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BC7C);
    cpu.pc = 0x2AA24;
    16
}
/// $02BC7C: jsr $2a7d8(pc)
pub fn i_02bc7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BC80;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BC80);
    cpu.pc = 0x2A7D8;
    16
}
/// $02BC80: bcc.w $2bc8a
pub fn i_02bc80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BC84;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2BC8A; return 10; }
    8
}
/// $02BC84: lea.l $2fc8e(pc), a1
pub fn i_02bc84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BC88;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2FC8E;
    16
}
/// $02BC88: move.l a1, (a6)
pub fn i_02bc88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BC8A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02BC8A: jsr $28d70.l
pub fn i_02bc8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BC90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2BC90);
    cpu.pc = 0x28D70;
    16
}
/// $02BC90: move.b $106f28.l, d0
pub fn i_02bc90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BC96;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $02BC96: andi.b #$1, d0
pub fn i_02bc96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BC9A;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $02BC9A: bne.w $2bcb2
pub fn i_02bc9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BC9E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2BCB2; return 10; }
    8
}
/// $02BC9E: lea.l $29834(pc), a0
pub fn i_02bc9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BCA2;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x29834;
    16
}
/// $02BCA2: move.l a0, $4c(a6)
pub fn i_02bca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BCA6;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02BCA6: jsr $283ca.l
pub fn i_02bca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BCAC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2BCAC);
    cpu.pc = 0x283CA;
    16
}
/// $02BCAC: jsr $283ca.l
pub fn i_02bcac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BCB2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2BCB2);
    cpu.pc = 0x283CA;
    16
}
/// $02BCB2: jsr $283d8.l
pub fn i_02bcb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BCB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2BCB8);
    cpu.pc = 0x283D8;
    16
}
/// $02BCB8: jsr $2fb04(pc)
pub fn i_02bcb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BCBC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BCBC);
    cpu.pc = 0x2FB04;
    16
}
/// $02BCBC: bcc.w $2bcc6
pub fn i_02bcbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BCC0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2BCC6; return 10; }
    8
}
/// $02BCC0: lea.l $2fe6a(pc), a1
pub fn i_02bcc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BCC4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2FE6A;
    16
}
/// $02BCC4: move.l a1, (a6)
pub fn i_02bcc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BCC6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02BCC6: jsr $2ffb0(pc)
pub fn i_02bcc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BCCA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BCCA);
    cpu.pc = 0x2FFB0;
    16
}
/// $02BCCA: tst.b $92(a6)
pub fn i_02bcca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BCCE;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x92);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $02BCCE: beq.w $2bcd8
pub fn i_02bcce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BCD2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2BCD8; return 10; }
    8
}
/// $02BCD2: subi.b #$1, $92(a6)
pub fn i_02bcd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BCD8;
    cpu.ir = 0x42E;
    let ad1 = cpu.a[6].wrapping_add(0x92);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $02BCD8: bhi.w $2bce2
pub fn i_02bcd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BCDC;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x2BCE2; return 10; }
    8
}
/// $02BCDC: lea.l $2fe6a(pc), a1
pub fn i_02bcdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BCE0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2FE6A;
    16
}
/// $02BCE0: move.l a1, (a6)
pub fn i_02bce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BCE2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02BCE2: cmpi.w #$117, $22(a6)
pub fn i_02bce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BCE8;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x117, Size::Word, false);
    16
}
/// $02BCE8: blt.w $2bcf2
pub fn i_02bce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BCEC;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x2BCF2; return 10; }
    8
}
/// $02BCEC: lea.l $2fe6a(pc), a1
pub fn i_02bcec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BCF0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2FE6A;
    16
}
/// $02BCF0: move.l a1, (a6)
pub fn i_02bcf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BCF2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02BCF2: clr.w $2a(a6)
pub fn i_02bcf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BCF6;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02BCF6: clr.w $2e(a6)
pub fn i_02bcf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BCFA;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
// ===== JsrPcThunk_02bcfa  ($02BCFA, 6 B, thunks/jsr_pc_thunks.c)
/// $02BCFA: jsr $2ac80(pc)
pub fn i_02bcfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BCFE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BCFE);
    cpu.pc = 0x2AC80;
    16
}
/// $02BCFE: rts
pub fn i_02bcfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BD00;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_BrakeRight_02bd00  ($02BD00, 298 B, slug/states_02ae3e.s)
/// $02BD00: clr.w d0
pub fn i_02bd00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BD02;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02BD02: sub.w $28(a6), d0
pub fn i_02bd02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BD06;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $02BD06: asr.w #$3, d0
pub fn i_02bd06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BD08;
    cpu.ir = 0xE640;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $02BD08: move.w d0, $2c(a6)
pub fn i_02bd08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BD0C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02BD0C: move.b #$8, $91(a6)
pub fn i_02bd0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BD12;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x91);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02BD12: jsr $2a9a0(pc)
pub fn i_02bd12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BD16;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BD16);
    cpu.pc = 0x2A9A0;
    16
}
/// $02BD16: movea.l #$279358, a0
pub fn i_02bd16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BD1C;
    cpu.ir = 0x207C;
    let v1: u32 = 0x279358;
    cpu.a[0] = v1;
    4
}
/// $02BD1C: lsl.w #$2, d0
pub fn i_02bd1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BD1E;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02BD1E: movea.l (a0, d0.w), a0
pub fn i_02bd1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BD22;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02BD22: cmpa.l #$ffffffff, a0
pub fn i_02bd22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BD28;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02BD28: beq.w $2bd32
pub fn i_02bd28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BD2C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2BD32; return 10; }
    8
}
/// $02BD2C: jsr $28cd4.l
pub fn i_02bd2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BD32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2BD32);
    cpu.pc = 0x28CD4;
    16
}
/// $02BD32: move.b #$a, $20(a6)
pub fn i_02bd32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BD38;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02BD38: andi.w #$ff, d0
pub fn i_02bd38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BD3C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02BD3C: lsr.w #$2, d0
pub fn i_02bd3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BD3E;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02BD3E: add.b d0, $20(a6)
pub fn i_02bd3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BD42;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02BD42: lea.l $2bd48(pc), a1
pub fn i_02bd42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BD46;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2BD48;
    16
}
/// $02BD46: move.l a1, (a6)
pub fn i_02bd46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BD48;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02BD48: subq.b #$1, $91(a6)
pub fn i_02bd48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BD4C;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x91);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02BD4C: bne.w $2bd5e
pub fn i_02bd4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BD50;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2BD5E; return 10; }
    8
}
/// $02BD50: clr.w $28(a6)
pub fn i_02bd50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BD54;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02BD54: clr.w $2c(a6)
pub fn i_02bd54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BD58;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02BD58: lea.l $2bd5e(pc), a1
pub fn i_02bd58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BD5C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2BD5E;
    16
}
/// $02BD5C: move.l a1, (a6)
pub fn i_02bd5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BD5E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02BD5E: jsr $2aa24(pc)
pub fn i_02bd5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BD62;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BD62);
    cpu.pc = 0x2AA24;
    16
}
/// $02BD62: jsr $2a7d8(pc)
pub fn i_02bd62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BD66;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BD66);
    cpu.pc = 0x2A7D8;
    16
}
/// $02BD66: bcc.w $2bd70
pub fn i_02bd66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BD6A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2BD70; return 10; }
    8
}
/// $02BD6A: lea.l $2b4ca(pc), a1
pub fn i_02bd6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BD6E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B4CA;
    16
}
/// $02BD6E: move.l a1, (a6)
pub fn i_02bd6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BD70;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02BD70: jsr $2a4f0(pc)
pub fn i_02bd70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BD74;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BD74);
    cpu.pc = 0x2A4F0;
    16
}
/// $02BD74: bcc.w $2bd7e
pub fn i_02bd74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BD78;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2BD7E; return 10; }
    8
}
/// $02BD78: lea.l $2b4ca(pc), a1
pub fn i_02bd78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BD7C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B4CA;
    16
}
/// $02BD7C: move.l a1, (a6)
pub fn i_02bd7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BD7E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02BD7E: jsr $28d70.l
pub fn i_02bd7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BD84;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2BD84);
    cpu.pc = 0x28D70;
    16
}
/// $02BD84: bcc.w $2bd8e
pub fn i_02bd84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BD88;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2BD8E; return 10; }
    8
}
/// $02BD88: lea.l $2b38c(pc), a1
pub fn i_02bd88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BD8C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B38C;
    16
}
/// $02BD8C: move.l a1, (a6)
pub fn i_02bd8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BD8E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02BD8E: jsr $283d8.l
pub fn i_02bd8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BD94;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2BD94);
    cpu.pc = 0x283D8;
    16
}
/// $02BD94: jsr $2a328(pc)
pub fn i_02bd94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BD98;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BD98);
    cpu.pc = 0x2A328;
    16
}
/// $02BD98: bcc.w $2bda2
pub fn i_02bd98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BD9C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2BDA2; return 10; }
    8
}
/// $02BD9C: lea.l $2bf64(pc), a1
pub fn i_02bd9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BDA0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2BF64;
    16
}
/// $02BDA0: move.l a1, (a6)
pub fn i_02bda0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BDA2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02BDA2: jsr $2a34e(pc)
pub fn i_02bda2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BDA6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BDA6);
    cpu.pc = 0x2A34E;
    16
}
/// $02BDA6: bcc.w $2bdb0
pub fn i_02bda6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BDAA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2BDB0; return 10; }
    8
}
/// $02BDAA: lea.l $2c07a(pc), a1
pub fn i_02bdaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BDAE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2C07A;
    16
}
/// $02BDAE: move.l a1, (a6)
pub fn i_02bdae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BDB0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02BDB0: jsr $2a59a(pc)
pub fn i_02bdb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BDB4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BDB4);
    cpu.pc = 0x2A59A;
    16
}
/// $02BDB4: bcc.w $2bdd0
pub fn i_02bdb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BDB8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2BDD0; return 10; }
    8
}
/// $02BDB8: jsr $2a9a0(pc)
pub fn i_02bdb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BDBC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BDBC);
    cpu.pc = 0x2A9A0;
    16
}
/// $02BDBC: andi.w #$ff, d0
pub fn i_02bdbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BDC0;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02BDC0: lea.l $2a078(pc), a0
pub fn i_02bdc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BDC4;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A078;
    16
}
/// $02BDC4: movea.l #$ffffffff, a1
pub fn i_02bdc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BDCA;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $02BDCA: jsr $772.l
pub fn i_02bdca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BDD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2BDD0);
    cpu.pc = 0x772;
    16
}
/// $02BDD0: jsr $2a8c0(pc)
pub fn i_02bdd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BDD4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BDD4);
    cpu.pc = 0x2A8C0;
    16
}
/// $02BDD4: bcc.w $2bdde
pub fn i_02bdd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BDD8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2BDDE; return 10; }
    8
}
/// $02BDD8: lea.l $2d02e(pc), a1
pub fn i_02bdd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BDDC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D02E;
    16
}
/// $02BDDC: move.l a1, (a6)
pub fn i_02bddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BDDE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02BDDE: jsr $2aaf0(pc)
pub fn i_02bdde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BDE2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BDE2);
    cpu.pc = 0x2AAF0;
    16
}
/// $02BDE2: lea.l $2c9b0(pc), a0
pub fn i_02bde2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BDE6;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2C9B0;
    16
}
/// $02BDE6: movea.l #$ffffffff, a1
pub fn i_02bde6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BDEC;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $02BDEC: jsr $772.l
pub fn i_02bdec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BDF2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2BDF2);
    cpu.pc = 0x772;
    16
}
/// $02BDF2: jsr $2aac0(pc)
pub fn i_02bdf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BDF6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BDF6);
    cpu.pc = 0x2AAC0;
    16
}
/// $02BDF6: bcc.w $2be00
pub fn i_02bdf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BDFA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2BE00; return 10; }
    8
}
/// $02BDFA: lea.l $2c24a(pc), a1
pub fn i_02bdfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BDFE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2C24A;
    16
}
/// $02BDFE: move.l a1, (a6)
pub fn i_02bdfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BE00;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02BE00: jsr $2ac80(pc)
pub fn i_02be00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BE04;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BE04);
    cpu.pc = 0x2AC80;
    16
}
/// $02BE04: bcc.w $2be0e
pub fn i_02be04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BE08;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2BE0E; return 10; }
    8
}
/// $02BE08: lea.l $2dd74(pc), a1
pub fn i_02be08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BE0C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DD74;
    16
}
/// $02BE0C: move.l a1, (a6)
pub fn i_02be0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BE0E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02BE0E: jsr $2a5cc(pc)
pub fn i_02be0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BE12;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BE12);
    cpu.pc = 0x2A5CC;
    16
}
/// $02BE12: bcc.w $2be1c
pub fn i_02be12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BE16;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2BE1C; return 10; }
    8
}
/// $02BE16: lea.l $2bbf2(pc), a1
pub fn i_02be16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BE1A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2BBF2;
    16
}
/// $02BE1A: move.l a1, (a6)
pub fn i_02be1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BE1C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02BE1C: jsr $2a690(pc)
pub fn i_02be1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BE20;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BE20);
    cpu.pc = 0x2A690;
    16
}
/// $02BE20: lea.l $2a060(pc), a0
pub fn i_02be20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BE24;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A060;
    16
}
/// $02BE24: movea.l #$ffffffff, a1
pub fn i_02be24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BE2A;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
// ===== JsrAbsThunk_02be2a  ($02BE2A, 8 B, thunks/jsr_abs_thunks.c)
/// $02BE2A: jsr $772.l
pub fn i_02be2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BE30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2BE30);
    cpu.pc = 0x772;
    16
}
/// $02BE30: rts
pub fn i_02be30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BE32;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_BrakeLeft_02be32  ($02BE32, 298 B, slug/states_02ae3e.s)
/// $02BE32: clr.w d0
pub fn i_02be32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BE34;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02BE34: sub.w $28(a6), d0
pub fn i_02be34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BE38;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $02BE38: asr.w #$3, d0
pub fn i_02be38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BE3A;
    cpu.ir = 0xE640;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $02BE3A: move.w d0, $2c(a6)
pub fn i_02be3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BE3E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02BE3E: move.b #$8, $91(a6)
pub fn i_02be3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BE44;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x91);
    bus.write8(ad2, v1 as u8);
    12
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x02A4A0, i_02a4a0),
    (0x02A4A2, i_02a4a2),
    (0x02A4A6, i_02a4a6),
    (0x02A4A8, i_02a4a8),
    (0x02A4AE, i_02a4ae),
    (0x02A4B2, i_02a4b2),
    (0x02A4B8, i_02a4b8),
    (0x02A4BC, i_02a4bc),
    (0x02A4C0, i_02a4c0),
    (0x02A4C2, i_02a4c2),
    (0x02A4C6, i_02a4c6),
    (0x02A4CA, i_02a4ca),
    (0x02A4CE, i_02a4ce),
    (0x02A4D4, i_02a4d4),
    (0x02A4D8, i_02a4d8),
    (0x02A4DE, i_02a4de),
    (0x02A4E2, i_02a4e2),
    (0x02A4E6, i_02a4e6),
    (0x02A4EA, i_02a4ea),
    (0x02A4EC, i_02a4ec),
    (0x02A4F0, i_02a4f0),
    (0x02A4F4, i_02a4f4),
    (0x02A4F8, i_02a4f8),
    (0x02A4FC, i_02a4fc),
    (0x02A500, i_02a500),
    (0x02A502, i_02a502),
    (0x02A506, i_02a506),
    (0x02A50A, i_02a50a),
    (0x02A50E, i_02a50e),
    (0x02A512, i_02a512),
    (0x02A516, i_02a516),
    (0x02A59A, i_02a59a),
    (0x02A59E, i_02a59e),
    (0x02A5A2, i_02a5a2),
    (0x02A5A6, i_02a5a6),
    (0x02A5A8, i_02a5a8),
    (0x02A5AC, i_02a5ac),
    (0x02A5CC, i_02a5cc),
    (0x02A5D2, i_02a5d2),
    (0x02A5D4, i_02a5d4),
    (0x02A5DA, i_02a5da),
    (0x02A5DE, i_02a5de),
    (0x02A5E2, i_02a5e2),
    (0x02A5E6, i_02a5e6),
    (0x02A5EA, i_02a5ea),
    (0x02A5EE, i_02a5ee),
    (0x02A5F2, i_02a5f2),
    (0x02A5F8, i_02a5f8),
    (0x02A5FA, i_02a5fa),
    (0x02A5FE, i_02a5fe),
    (0x02A604, i_02a604),
    (0x02A606, i_02a606),
    (0x02A60C, i_02a60c),
    (0x02A610, i_02a610),
    (0x02A614, i_02a614),
    (0x02A618, i_02a618),
    (0x02A61C, i_02a61c),
    (0x02A620, i_02a620),
    (0x02A624, i_02a624),
    (0x02A62A, i_02a62a),
    (0x02A62C, i_02a62c),
    (0x02A630, i_02a630),
    (0x02A636, i_02a636),
    (0x02A638, i_02a638),
    (0x02A63E, i_02a63e),
    (0x02A642, i_02a642),
    (0x02A646, i_02a646),
    (0x02A64C, i_02a64c),
    (0x02A652, i_02a652),
    (0x02A656, i_02a656),
    (0x02A65C, i_02a65c),
    (0x02A662, i_02a662),
    (0x02A664, i_02a664),
    (0x02A66A, i_02a66a),
    (0x02A66E, i_02a66e),
    (0x02A674, i_02a674),
    (0x02A678, i_02a678),
    (0x02A67E, i_02a67e),
    (0x02A682, i_02a682),
    (0x02A684, i_02a684),
    (0x02A688, i_02a688),
    (0x02A68A, i_02a68a),
    (0x02A690, i_02a690),
    (0x02A696, i_02a696),
    (0x02A69A, i_02a69a),
    (0x02A6A0, i_02a6a0),
    (0x02A6A4, i_02a6a4),
    (0x02A6AA, i_02a6aa),
    (0x02A6B0, i_02a6b0),
    (0x02A6B4, i_02a6b4),
    (0x02A6B6, i_02a6b6),
    (0x02A6B8, i_02a6b8),
    (0x02A6BC, i_02a6bc),
    (0x02A6BE, i_02a6be),
    (0x02A6C2, i_02a6c2),
    (0x02A6C6, i_02a6c6),
    (0x02A6CC, i_02a6cc),
    (0x02A6D4, i_02a6d4),
    (0x02A6D8, i_02a6d8),
    (0x02A6E0, i_02a6e0),
    (0x02A6E4, i_02a6e4),
    (0x02A6EA, i_02a6ea),
    (0x02A6F0, i_02a6f0),
    (0x02A6F4, i_02a6f4),
    (0x02A6F8, i_02a6f8),
    (0x02A6FC, i_02a6fc),
    (0x02A702, i_02a702),
    (0x02A706, i_02a706),
    (0x02A70A, i_02a70a),
    (0x02A70C, i_02a70c),
    (0x02A710, i_02a710),
    (0x02A712, i_02a712),
    (0x02A718, i_02a718),
    (0x02A71A, i_02a71a),
    (0x02A71E, i_02a71e),
    (0x02A720, i_02a720),
    (0x02A726, i_02a726),
    (0x02A72C, i_02a72c),
    (0x02A732, i_02a732),
    (0x02A736, i_02a736),
    (0x02A73C, i_02a73c),
    (0x02A740, i_02a740),
    (0x02A746, i_02a746),
    (0x02A74A, i_02a74a),
    (0x02A750, i_02a750),
    (0x02A752, i_02a752),
    (0x02A758, i_02a758),
    (0x02A75E, i_02a75e),
    (0x02A760, i_02a760),
    (0x02A766, i_02a766),
    (0x02A76C, i_02a76c),
    (0x02A772, i_02a772),
    (0x02A776, i_02a776),
    (0x02A77C, i_02a77c),
    (0x02A782, i_02a782),
    (0x02A786, i_02a786),
    (0x02A78C, i_02a78c),
    (0x02A790, i_02a790),
    (0x02A796, i_02a796),
    (0x02A79C, i_02a79c),
    (0x02A7A0, i_02a7a0),
    (0x02A7A4, i_02a7a4),
    (0x02A7A6, i_02a7a6),
    (0x02A7AA, i_02a7aa),
    (0x02A7AC, i_02a7ac),
    (0x02A7B2, i_02a7b2),
    (0x02A7B8, i_02a7b8),
    (0x02A7BE, i_02a7be),
    (0x02A7C2, i_02a7c2),
    (0x02A7C8, i_02a7c8),
    (0x02A7CC, i_02a7cc),
    (0x02A7D0, i_02a7d0),
    (0x02A7D2, i_02a7d2),
    (0x02A7D6, i_02a7d6),
    (0x02A7D8, i_02a7d8),
    (0x02A7DE, i_02a7de),
    (0x02A7E4, i_02a7e4),
    (0x02A7E8, i_02a7e8),
    (0x02A7EE, i_02a7ee),
    (0x02A7F4, i_02a7f4),
    (0x02A7FA, i_02a7fa),
    (0x02A7FE, i_02a7fe),
    (0x02A804, i_02a804),
    (0x02A808, i_02a808),
    (0x02A80E, i_02a80e),
    (0x02A814, i_02a814),
    (0x02A818, i_02a818),
    (0x02A81C, i_02a81c),
    (0x02A81E, i_02a81e),
    (0x02A822, i_02a822),
    (0x02A824, i_02a824),
    (0x02A82A, i_02a82a),
    (0x02A830, i_02a830),
    (0x02A834, i_02a834),
    (0x02A83A, i_02a83a),
    (0x02A840, i_02a840),
    (0x02A844, i_02a844),
    (0x02A84A, i_02a84a),
    (0x02A84E, i_02a84e),
    (0x02A852, i_02a852),
    (0x02A854, i_02a854),
    (0x02A858, i_02a858),
    (0x02A85A, i_02a85a),
    (0x02A860, i_02a860),
    (0x02A866, i_02a866),
    (0x02A86A, i_02a86a),
    (0x02A870, i_02a870),
    (0x02A876, i_02a876),
    (0x02A878, i_02a878),
    (0x02A87E, i_02a87e),
    (0x02A884, i_02a884),
    (0x02A888, i_02a888),
    (0x02A88E, i_02a88e),
    (0x02A894, i_02a894),
    (0x02A898, i_02a898),
    (0x02A89C, i_02a89c),
    (0x02A8A0, i_02a8a0),
    (0x02A8A4, i_02a8a4),
    (0x02A8A8, i_02a8a8),
    (0x02A8AE, i_02a8ae),
    (0x02A8B2, i_02a8b2),
    (0x02A8B4, i_02a8b4),
    (0x02A8BA, i_02a8ba),
    (0x02A8BE, i_02a8be),
    (0x02A8C0, i_02a8c0),
    (0x02A8C6, i_02a8c6),
    (0x02A8CA, i_02a8ca),
    (0x02A8D0, i_02a8d0),
    (0x02A8D6, i_02a8d6),
    (0x02A8DA, i_02a8da),
    (0x02A8DE, i_02a8de),
    (0x02A8E4, i_02a8e4),
    (0x02A8E8, i_02a8e8),
    (0x02A8EC, i_02a8ec),
    (0x02A8F0, i_02a8f0),
    (0x02A8F4, i_02a8f4),
    (0x02A8F6, i_02a8f6),
    (0x02A8FA, i_02a8fa),
    (0x02A8FE, i_02a8fe),
    (0x02A902, i_02a902),
    (0x02A906, i_02a906),
    (0x02A908, i_02a908),
    (0x02A90C, i_02a90c),
    (0x02A910, i_02a910),
    (0x02A914, i_02a914),
    (0x02A918, i_02a918),
    (0x02A91C, i_02a91c),
    (0x02A922, i_02a922),
    (0x02A928, i_02a928),
    (0x02A92E, i_02a92e),
    (0x02A932, i_02a932),
    (0x02A934, i_02a934),
    (0x02A93A, i_02a93a),
    (0x02A940, i_02a940),
    (0x02A946, i_02a946),
    (0x02A94C, i_02a94c),
    (0x02A952, i_02a952),
    (0x02A956, i_02a956),
    (0x02A958, i_02a958),
    (0x02A95C, i_02a95c),
    (0x02A960, i_02a960),
    (0x02A964, i_02a964),
    (0x02A968, i_02a968),
    (0x02A96C, i_02a96c),
    (0x02A970, i_02a970),
    (0x02A972, i_02a972),
    (0x02A974, i_02a974),
    (0x02A976, i_02a976),
    (0x02A97A, i_02a97a),
    (0x02A97C, i_02a97c),
    (0x02A980, i_02a980),
    (0x02A982, i_02a982),
    (0x02A986, i_02a986),
    (0x02A98A, i_02a98a),
    (0x02A98E, i_02a98e),
    (0x02A992, i_02a992),
    (0x02A996, i_02a996),
    (0x02A99A, i_02a99a),
    (0x02A99E, i_02a99e),
    (0x02A9A0, i_02a9a0),
    (0x02A9CE, i_02a9ce),
    (0x02A9D2, i_02a9d2),
    (0x02A9D4, i_02a9d4),
    (0x02A9D8, i_02a9d8),
    (0x02A9DC, i_02a9dc),
    (0x02A9E0, i_02a9e0),
    (0x02A9E4, i_02a9e4),
    (0x02A9E8, i_02a9e8),
    (0x02A9EC, i_02a9ec),
    (0x02A9F0, i_02a9f0),
    (0x02A9F4, i_02a9f4),
    (0x02A9F6, i_02a9f6),
    (0x02A9FA, i_02a9fa),
    (0x02A9FC, i_02a9fc),
    (0x02AA00, i_02aa00),
    (0x02AA02, i_02aa02),
    (0x02AA06, i_02aa06),
    (0x02AA08, i_02aa08),
    (0x02AA0C, i_02aa0c),
    (0x02AA0E, i_02aa0e),
    (0x02AA12, i_02aa12),
    (0x02AA16, i_02aa16),
    (0x02AA1A, i_02aa1a),
    (0x02AA1E, i_02aa1e),
    (0x02AA22, i_02aa22),
    (0x02AA24, i_02aa24),
    (0x02AA2A, i_02aa2a),
    (0x02AA30, i_02aa30),
    (0x02AA32, i_02aa32),
    (0x02AA38, i_02aa38),
    (0x02AA3C, i_02aa3c),
    (0x02AA42, i_02aa42),
    (0x02AA46, i_02aa46),
    (0x02AA4C, i_02aa4c),
    (0x02AA50, i_02aa50),
    (0x02AA54, i_02aa54),
    (0x02AA58, i_02aa58),
    (0x02AA5C, i_02aa5c),
    (0x02AA60, i_02aa60),
    (0x02AA62, i_02aa62),
    (0x02AA66, i_02aa66),
    (0x02AA68, i_02aa68),
    (0x02AA6C, i_02aa6c),
    (0x02AA70, i_02aa70),
    (0x02AA74, i_02aa74),
    (0x02AA78, i_02aa78),
    (0x02AA7C, i_02aa7c),
    (0x02AA7E, i_02aa7e),
    (0x02AA80, i_02aa80),
    (0x02AA84, i_02aa84),
    (0x02AA88, i_02aa88),
    (0x02AA8A, i_02aa8a),
    (0x02AA8C, i_02aa8c),
    (0x02AA90, i_02aa90),
    (0x02AA94, i_02aa94),
    (0x02AA98, i_02aa98),
    (0x02AA9A, i_02aa9a),
    (0x02AA9C, i_02aa9c),
    (0x02AAA0, i_02aaa0),
    (0x02AAA2, i_02aaa2),
    (0x02AAA4, i_02aaa4),
    (0x02AAA6, i_02aaa6),
    (0x02AAA8, i_02aaa8),
    (0x02AAAA, i_02aaaa),
    (0x02AAAE, i_02aaae),
    (0x02AAB0, i_02aab0),
    (0x02AAB4, i_02aab4),
    (0x02AAB6, i_02aab6),
    (0x02AAB8, i_02aab8),
    (0x02AABA, i_02aaba),
    (0x02AABE, i_02aabe),
    (0x02AAC0, i_02aac0),
    (0x02AAC6, i_02aac6),
    (0x02AACA, i_02aaca),
    (0x02AACE, i_02aace),
    (0x02AAD2, i_02aad2),
    (0x02AAD6, i_02aad6),
    (0x02AADA, i_02aada),
    (0x02AADE, i_02aade),
    (0x02AAE2, i_02aae2),
    (0x02AAE4, i_02aae4),
    (0x02AAE8, i_02aae8),
    (0x02AAEA, i_02aaea),
    (0x02AAF0, i_02aaf0),
    (0x02AAF8, i_02aaf8),
    (0x02AAFC, i_02aafc),
    (0x02AB02, i_02ab02),
    (0x02AB06, i_02ab06),
    (0x02AB0E, i_02ab0e),
    (0x02AB12, i_02ab12),
    (0x02AB18, i_02ab18),
    (0x02AB1C, i_02ab1c),
    (0x02AB22, i_02ab22),
    (0x02AB26, i_02ab26),
    (0x02AB28, i_02ab28),
    (0x02AB2C, i_02ab2c),
    (0x02AB2E, i_02ab2e),
    (0x02AB34, i_02ab34),
    (0x02AB36, i_02ab36),
    (0x02AB3A, i_02ab3a),
    (0x02AB3C, i_02ab3c),
    (0x02AB44, i_02ab44),
    (0x02AB48, i_02ab48),
    (0x02AB4E, i_02ab4e),
    (0x02AB52, i_02ab52),
    (0x02AB5A, i_02ab5a),
    (0x02AB5E, i_02ab5e),
    (0x02AB64, i_02ab64),
    (0x02AB68, i_02ab68),
    (0x02AB6C, i_02ab6c),
    (0x02AB70, i_02ab70),
    (0x02AB72, i_02ab72),
    (0x02AB76, i_02ab76),
    (0x02AB78, i_02ab78),
    (0x02AB7E, i_02ab7e),
    (0x02AB80, i_02ab80),
    (0x02AB84, i_02ab84),
    (0x02AB86, i_02ab86),
    (0x02AB8E, i_02ab8e),
    (0x02AB92, i_02ab92),
    (0x02AB98, i_02ab98),
    (0x02AB9C, i_02ab9c),
    (0x02ABA4, i_02aba4),
    (0x02ABA8, i_02aba8),
    (0x02ABAE, i_02abae),
    (0x02ABB2, i_02abb2),
    (0x02ABB8, i_02abb8),
    (0x02ABBA, i_02abba),
    (0x02ABBE, i_02abbe),
    (0x02ABC0, i_02abc0),
    (0x02ABC4, i_02abc4),
    (0x02ABC6, i_02abc6),
    (0x02ABCC, i_02abcc),
    (0x02ABD0, i_02abd0),
    (0x02ABD2, i_02abd2),
    (0x02ABD8, i_02abd8),
    (0x02ABDE, i_02abde),
    (0x02ABE2, i_02abe2),
    (0x02ABE6, i_02abe6),
    (0x02ABEA, i_02abea),
    (0x02ABEE, i_02abee),
    (0x02ABF0, i_02abf0),
    (0x02ABF6, i_02abf6),
    (0x02ABFC, i_02abfc),
    (0x02AC00, i_02ac00),
    (0x02AC04, i_02ac04),
    (0x02AC08, i_02ac08),
    (0x02AC0C, i_02ac0c),
    (0x02AC0E, i_02ac0e),
    (0x02AC14, i_02ac14),
    (0x02AC18, i_02ac18),
    (0x02AC1A, i_02ac1a),
    (0x02AC1E, i_02ac1e),
    (0x02AC24, i_02ac24),
    (0x02AC28, i_02ac28),
    (0x02AC2A, i_02ac2a),
    (0x02AC2E, i_02ac2e),
    (0x02AC32, i_02ac32),
    (0x02AC38, i_02ac38),
    (0x02AC3C, i_02ac3c),
    (0x02AC40, i_02ac40),
    (0x02AC44, i_02ac44),
    (0x02AC46, i_02ac46),
    (0x02AC4A, i_02ac4a),
    (0x02AC4C, i_02ac4c),
    (0x02AC52, i_02ac52),
    (0x02AC56, i_02ac56),
    (0x02AC5A, i_02ac5a),
    (0x02AC5C, i_02ac5c),
    (0x02AC60, i_02ac60),
    (0x02AC64, i_02ac64),
    (0x02AC68, i_02ac68),
    (0x02AC6A, i_02ac6a),
    (0x02AC70, i_02ac70),
    (0x02AC74, i_02ac74),
    (0x02AC78, i_02ac78),
    (0x02AC7A, i_02ac7a),
    (0x02AC7E, i_02ac7e),
    (0x02AC80, i_02ac80),
    (0x02AC86, i_02ac86),
    (0x02AC8A, i_02ac8a),
    (0x02AC90, i_02ac90),
    (0x02AC94, i_02ac94),
    (0x02AC98, i_02ac98),
    (0x02AC9C, i_02ac9c),
    (0x02ACA0, i_02aca0),
    (0x02ACA2, i_02aca2),
    (0x02ACA6, i_02aca6),
    (0x02ACAC, i_02acac),
    (0x02ACB2, i_02acb2),
    (0x02ACB6, i_02acb6),
    (0x02ACB8, i_02acb8),
    (0x02ACC0, i_02acc0),
    (0x02ACC4, i_02acc4),
    (0x02ACC8, i_02acc8),
    (0x02ACCE, i_02acce),
    (0x02ACD2, i_02acd2),
    (0x02ACD8, i_02acd8),
    (0x02ACDC, i_02acdc),
    (0x02ACE2, i_02ace2),
    (0x02ACE6, i_02ace6),
    (0x02ACEC, i_02acec),
    (0x02ACF0, i_02acf0),
    (0x02ACF4, i_02acf4),
    (0x02ACF6, i_02acf6),
    (0x02ACFA, i_02acfa),
    (0x02ACFC, i_02acfc),
    (0x02AD02, i_02ad02),
    (0x02AD08, i_02ad08),
    (0x02AD0C, i_02ad0c),
    (0x02AD12, i_02ad12),
    (0x02AD16, i_02ad16),
    (0x02AD1C, i_02ad1c),
    (0x02AD20, i_02ad20),
    (0x02AD26, i_02ad26),
    (0x02AD2A, i_02ad2a),
    (0x02AD2E, i_02ad2e),
    (0x02AD30, i_02ad30),
    (0x02AD34, i_02ad34),
    (0x02AD36, i_02ad36),
    (0x02AD3A, i_02ad3a),
    (0x02AD3E, i_02ad3e),
    (0x02AD42, i_02ad42),
    (0x02AD46, i_02ad46),
    (0x02AD4A, i_02ad4a),
    (0x02AD4E, i_02ad4e),
    (0x02AD54, i_02ad54),
    (0x02AD58, i_02ad58),
    (0x02AD5E, i_02ad5e),
    (0x02AD62, i_02ad62),
    (0x02AD64, i_02ad64),
    (0x02AD68, i_02ad68),
    (0x02AD6A, i_02ad6a),
    (0x02AD6E, i_02ad6e),
    (0x02AD70, i_02ad70),
    (0x02AD74, i_02ad74),
    (0x02AD78, i_02ad78),
    (0x02AD7C, i_02ad7c),
    (0x02AD80, i_02ad80),
    (0x02AD84, i_02ad84),
    (0x02AD8A, i_02ad8a),
    (0x02AD90, i_02ad90),
    (0x02AD96, i_02ad96),
    (0x02AD9C, i_02ad9c),
    (0x02ADA2, i_02ada2),
    (0x02ADA6, i_02ada6),
    (0x02ADAA, i_02adaa),
    (0x02ADAE, i_02adae),
    (0x02ADB2, i_02adb2),
    (0x02ADB8, i_02adb8),
    (0x02ADBE, i_02adbe),
    (0x02ADC4, i_02adc4),
    (0x02ADCA, i_02adca),
    (0x02ADD0, i_02add0),
    (0x02ADD4, i_02add4),
    (0x02ADD6, i_02add6),
    (0x02ADDC, i_02addc),
    (0x02ADE2, i_02ade2),
    (0x02ADE8, i_02ade8),
    (0x02ADEC, i_02adec),
    (0x02ADF0, i_02adf0),
    (0x02ADF6, i_02adf6),
    (0x02ADFC, i_02adfc),
    (0x02AE02, i_02ae02),
    (0x02AE06, i_02ae06),
    (0x02AE0A, i_02ae0a),
    (0x02AE0E, i_02ae0e),
    (0x02AE12, i_02ae12),
    (0x02AE16, i_02ae16),
    (0x02AE1A, i_02ae1a),
    (0x02AE1E, i_02ae1e),
    (0x02AE24, i_02ae24),
    (0x02AE28, i_02ae28),
    (0x02AE2C, i_02ae2c),
    (0x02AE30, i_02ae30),
    (0x02AE36, i_02ae36),
    (0x02AE3A, i_02ae3a),
    (0x02AE3C, i_02ae3c),
    (0x02AE3E, i_02ae3e),
    (0x02AE46, i_02ae46),
    (0x02AE4E, i_02ae4e),
    (0x02AE50, i_02ae50),
    (0x02AE54, i_02ae54),
    (0x02AE58, i_02ae58),
    (0x02AE5C, i_02ae5c),
    (0x02AE5E, i_02ae5e),
    (0x02AE62, i_02ae62),
    (0x02AE66, i_02ae66),
    (0x02AE6A, i_02ae6a),
    (0x02AE6E, i_02ae6e),
    (0x02AE70, i_02ae70),
    (0x02AE74, i_02ae74),
    (0x02AE7A, i_02ae7a),
    (0x02AE7C, i_02ae7c),
    (0x02AE82, i_02ae82),
    (0x02AE88, i_02ae88),
    (0x02AE8E, i_02ae8e),
    (0x02AEA4, i_02aea4),
    (0x02AEA8, i_02aea8),
    (0x02AEAC, i_02aeac),
    (0x02AEB0, i_02aeb0),
    (0x02AEB2, i_02aeb2),
    (0x02AEB6, i_02aeb6),
    (0x02AEBA, i_02aeba),
    (0x02AEBE, i_02aebe),
    (0x02AEC2, i_02aec2),
    (0x02AEC4, i_02aec4),
    (0x02AEC8, i_02aec8),
    (0x02AECE, i_02aece),
    (0x02AED0, i_02aed0),
    (0x02AED6, i_02aed6),
    (0x02AEDC, i_02aedc),
    (0x02AEE2, i_02aee2),
    (0x02AEF8, i_02aef8),
    (0x02AEFC, i_02aefc),
    (0x02AF00, i_02af00),
    (0x02AF06, i_02af06),
    (0x02AF0C, i_02af0c),
    (0x02AF12, i_02af12),
    (0x02AF18, i_02af18),
    (0x02AF1C, i_02af1c),
    (0x02AF20, i_02af20),
    (0x02AF24, i_02af24),
    (0x02AF2A, i_02af2a),
    (0x02AF30, i_02af30),
    (0x02AF36, i_02af36),
    (0x02AF3C, i_02af3c),
    (0x02AF40, i_02af40),
    (0x02AF44, i_02af44),
    (0x02AF48, i_02af48),
    (0x02AF4E, i_02af4e),
    (0x02AF54, i_02af54),
    (0x02AF5A, i_02af5a),
    (0x02AF60, i_02af60),
    (0x02AF64, i_02af64),
    (0x02AF68, i_02af68),
    (0x02AF6C, i_02af6c),
    (0x02AF72, i_02af72),
    (0x02AF78, i_02af78),
    (0x02AF7E, i_02af7e),
    (0x02AF84, i_02af84),
    (0x02AF88, i_02af88),
    (0x02AF8C, i_02af8c),
    (0x02AF90, i_02af90),
    (0x02AF96, i_02af96),
    (0x02AF9C, i_02af9c),
    (0x02AFA2, i_02afa2),
    (0x02AFA8, i_02afa8),
    (0x02AFAC, i_02afac),
    (0x02AFB2, i_02afb2),
    (0x02AFB8, i_02afb8),
    (0x02AFBC, i_02afbc),
    (0x02AFC2, i_02afc2),
    (0x02AFC8, i_02afc8),
    (0x02AFCE, i_02afce),
    (0x02AFD4, i_02afd4),
    (0x02AFD8, i_02afd8),
    (0x02AFDC, i_02afdc),
    (0x02AFE0, i_02afe0),
    (0x02AFE4, i_02afe4),
    (0x02AFE8, i_02afe8),
    (0x02AFEC, i_02afec),
    (0x02AFF0, i_02aff0),
    (0x02AFF4, i_02aff4),
    (0x02AFF8, i_02aff8),
    (0x02AFFE, i_02affe),
    (0x02B002, i_02b002),
    (0x02B004, i_02b004),
    (0x02B00A, i_02b00a),
    (0x02B00E, i_02b00e),
    (0x02B014, i_02b014),
    (0x02B018, i_02b018),
    (0x02B01C, i_02b01c),
    (0x02B020, i_02b020),
    (0x02B024, i_02b024),
    (0x02B028, i_02b028),
    (0x02B02C, i_02b02c),
    (0x02B02E, i_02b02e),
    (0x02B032, i_02b032),
    (0x02B038, i_02b038),
    (0x02B03E, i_02b03e),
    (0x02B044, i_02b044),
    (0x02B048, i_02b048),
    (0x02B04C, i_02b04c),
    (0x02B04E, i_02b04e),
    (0x02B052, i_02b052),
    (0x02B056, i_02b056),
    (0x02B05A, i_02b05a),
    (0x02B05C, i_02b05c),
    (0x02B05E, i_02b05e),
    (0x02B064, i_02b064),
    (0x02B06A, i_02b06a),
    (0x02B06E, i_02b06e),
    (0x02B072, i_02b072),
    (0x02B076, i_02b076),
    (0x02B07A, i_02b07a),
    (0x02B07E, i_02b07e),
    (0x02B082, i_02b082),
    (0x02B086, i_02b086),
    (0x02B08A, i_02b08a),
    (0x02B08E, i_02b08e),
    (0x02B092, i_02b092),
    (0x02B098, i_02b098),
    (0x02B09C, i_02b09c),
    (0x02B09E, i_02b09e),
    (0x02B0A4, i_02b0a4),
    (0x02B0AA, i_02b0aa),
    (0x02B0AE, i_02b0ae),
    (0x02B0B2, i_02b0b2),
    (0x02B0B6, i_02b0b6),
    (0x02B0BC, i_02b0bc),
    (0x02B0C0, i_02b0c0),
    (0x02B0C4, i_02b0c4),
    (0x02B0C8, i_02b0c8),
    (0x02B0CC, i_02b0cc),
    (0x02B0CE, i_02b0ce),
    (0x02B0D2, i_02b0d2),
    (0x02B0D6, i_02b0d6),
    (0x02B0DA, i_02b0da),
    (0x02B0DE, i_02b0de),
    (0x02B0E2, i_02b0e2),
    (0x02B0E4, i_02b0e4),
    (0x02B0E8, i_02b0e8),
    (0x02B0EE, i_02b0ee),
    (0x02B0F4, i_02b0f4),
    (0x02B0F8, i_02b0f8),
    (0x02B0FC, i_02b0fc),
    (0x02B100, i_02b100),
    (0x02B104, i_02b104),
    (0x02B108, i_02b108),
    (0x02B10C, i_02b10c),
    (0x02B110, i_02b110),
    (0x02B114, i_02b114),
    (0x02B118, i_02b118),
    (0x02B11C, i_02b11c),
    (0x02B120, i_02b120),
    (0x02B122, i_02b122),
    (0x02B124, i_02b124),
    (0x02B126, i_02b126),
    (0x02B12A, i_02b12a),
    (0x02B12E, i_02b12e),
    (0x02B132, i_02b132),
    (0x02B136, i_02b136),
    (0x02B13A, i_02b13a),
    (0x02B13E, i_02b13e),
    (0x02B144, i_02b144),
    (0x02B146, i_02b146),
    (0x02B14A, i_02b14a),
    (0x02B150, i_02b150),
    (0x02B154, i_02b154),
    (0x02B15A, i_02b15a),
    (0x02B15E, i_02b15e),
    (0x02B164, i_02b164),
    (0x02B166, i_02b166),
    (0x02B16C, i_02b16c),
    (0x02B172, i_02b172),
    (0x02B178, i_02b178),
    (0x02B17C, i_02b17c),
    (0x02B180, i_02b180),
    (0x02B186, i_02b186),
    (0x02B1C2, i_02b1c2),
    (0x02B1C8, i_02b1c8),
    (0x02B1CE, i_02b1ce),
    (0x02B1D2, i_02b1d2),
    (0x02B1D6, i_02b1d6),
    (0x02B1DA, i_02b1da),
    (0x02B1DE, i_02b1de),
    (0x02B1E2, i_02b1e2),
    (0x02B1E6, i_02b1e6),
    (0x02B1EA, i_02b1ea),
    (0x02B1F0, i_02b1f0),
    (0x02B1F6, i_02b1f6),
    (0x02B1FA, i_02b1fa),
    (0x02B1FE, i_02b1fe),
    (0x02B204, i_02b204),
    (0x02B208, i_02b208),
    (0x02B20A, i_02b20a),
    (0x02B210, i_02b210),
    (0x02B216, i_02b216),
    (0x02B21A, i_02b21a),
    (0x02B21E, i_02b21e),
    (0x02B222, i_02b222),
    (0x02B226, i_02b226),
    (0x02B22A, i_02b22a),
    (0x02B22E, i_02b22e),
    (0x02B232, i_02b232),
    (0x02B234, i_02b234),
    (0x02B238, i_02b238),
    (0x02B23E, i_02b23e),
    (0x02B244, i_02b244),
    (0x02B24A, i_02b24a),
    (0x02B24E, i_02b24e),
    (0x02B252, i_02b252),
    (0x02B254, i_02b254),
    (0x02B258, i_02b258),
    (0x02B25C, i_02b25c),
    (0x02B260, i_02b260),
    (0x02B262, i_02b262),
    (0x02B264, i_02b264),
    (0x02B26A, i_02b26a),
    (0x02B270, i_02b270),
    (0x02B274, i_02b274),
    (0x02B27A, i_02b27a),
    (0x02B280, i_02b280),
    (0x02B284, i_02b284),
    (0x02B288, i_02b288),
    (0x02B28C, i_02b28c),
    (0x02B290, i_02b290),
    (0x02B294, i_02b294),
    (0x02B298, i_02b298),
    (0x02B29C, i_02b29c),
    (0x02B2A0, i_02b2a0),
    (0x02B2A4, i_02b2a4),
    (0x02B2A8, i_02b2a8),
    (0x02B2AA, i_02b2aa),
    (0x02B2B0, i_02b2b0),
    (0x02B2B4, i_02b2b4),
    (0x02B2B8, i_02b2b8),
    (0x02B2BC, i_02b2bc),
    (0x02B2BE, i_02b2be),
    (0x02B2C4, i_02b2c4),
    (0x02B2CA, i_02b2ca),
    (0x02B2D0, i_02b2d0),
    (0x02B2D4, i_02b2d4),
    (0x02B2D8, i_02b2d8),
    (0x02B2DC, i_02b2dc),
    (0x02B2E2, i_02b2e2),
    (0x02B2E6, i_02b2e6),
    (0x02B2EA, i_02b2ea),
    (0x02B2EE, i_02b2ee),
    (0x02B2F2, i_02b2f2),
    (0x02B2F4, i_02b2f4),
    (0x02B2F8, i_02b2f8),
    (0x02B2FC, i_02b2fc),
    (0x02B300, i_02b300),
    (0x02B304, i_02b304),
    (0x02B308, i_02b308),
    (0x02B30A, i_02b30a),
    (0x02B30E, i_02b30e),
    (0x02B314, i_02b314),
    (0x02B31A, i_02b31a),
    (0x02B31E, i_02b31e),
    (0x02B322, i_02b322),
    (0x02B326, i_02b326),
    (0x02B32A, i_02b32a),
    (0x02B32E, i_02b32e),
    (0x02B332, i_02b332),
    (0x02B336, i_02b336),
    (0x02B33A, i_02b33a),
    (0x02B33E, i_02b33e),
    (0x02B342, i_02b342),
    (0x02B346, i_02b346),
    (0x02B348, i_02b348),
    (0x02B34A, i_02b34a),
    (0x02B34C, i_02b34c),
    (0x02B350, i_02b350),
    (0x02B354, i_02b354),
    (0x02B358, i_02b358),
    (0x02B35C, i_02b35c),
    (0x02B360, i_02b360),
    (0x02B364, i_02b364),
    (0x02B36A, i_02b36a),
    (0x02B36C, i_02b36c),
    (0x02B370, i_02b370),
    (0x02B376, i_02b376),
    (0x02B37A, i_02b37a),
    (0x02B380, i_02b380),
    (0x02B384, i_02b384),
    (0x02B38A, i_02b38a),
    (0x02B38C, i_02b38c),
    (0x02B392, i_02b392),
    (0x02B396, i_02b396),
    (0x02B39A, i_02b39a),
    (0x02B3A0, i_02b3a0),
    (0x02B3A2, i_02b3a2),
    (0x02B3A6, i_02b3a6),
    (0x02B3AC, i_02b3ac),
    (0x02B3B0, i_02b3b0),
    (0x02B3B6, i_02b3b6),
    (0x02B3BA, i_02b3ba),
    (0x02B3BE, i_02b3be),
    (0x02B3C2, i_02b3c2),
    (0x02B3C8, i_02b3c8),
    (0x02B3CC, i_02b3cc),
    (0x02B3CE, i_02b3ce),
    (0x02B3D2, i_02b3d2),
    (0x02B3DA, i_02b3da),
    (0x02B3DE, i_02b3de),
    (0x02B3E0, i_02b3e0),
    (0x02B3E4, i_02b3e4),
    (0x02B3E8, i_02b3e8),
    (0x02B3EE, i_02b3ee),
    (0x02B3F2, i_02b3f2),
    (0x02B3F6, i_02b3f6),
    (0x02B3FA, i_02b3fa),
    (0x02B3FE, i_02b3fe),
    (0x02B402, i_02b402),
    (0x02B404, i_02b404),
    (0x02B408, i_02b408),
    (0x02B40C, i_02b40c),
    (0x02B410, i_02b410),
    (0x02B414, i_02b414),
    (0x02B418, i_02b418),
    (0x02B41A, i_02b41a),
    (0x02B41E, i_02b41e),
    (0x02B422, i_02b422),
    (0x02B426, i_02b426),
    (0x02B428, i_02b428),
    (0x02B42C, i_02b42c),
    (0x02B430, i_02b430),
    (0x02B434, i_02b434),
    (0x02B438, i_02b438),
    (0x02B43C, i_02b43c),
    (0x02B442, i_02b442),
    (0x02B448, i_02b448),
    (0x02B44E, i_02b44e),
    (0x02B452, i_02b452),
    (0x02B456, i_02b456),
    (0x02B45A, i_02b45a),
    (0x02B45C, i_02b45c),
    (0x02B460, i_02b460),
    (0x02B464, i_02b464),
    (0x02B46A, i_02b46a),
    (0x02B470, i_02b470),
    (0x02B474, i_02b474),
    (0x02B478, i_02b478),
    (0x02B47C, i_02b47c),
    (0x02B47E, i_02b47e),
    (0x02B482, i_02b482),
    (0x02B486, i_02b486),
    (0x02B48A, i_02b48a),
    (0x02B48C, i_02b48c),
    (0x02B490, i_02b490),
    (0x02B494, i_02b494),
    (0x02B498, i_02b498),
    (0x02B49A, i_02b49a),
    (0x02B49E, i_02b49e),
    (0x02B4A2, i_02b4a2),
    (0x02B4A8, i_02b4a8),
    (0x02B4AE, i_02b4ae),
    (0x02B4B4, i_02b4b4),
    (0x02B4BA, i_02b4ba),
    (0x02B4C0, i_02b4c0),
    (0x02B4C4, i_02b4c4),
    (0x02B4C8, i_02b4c8),
    (0x02B4CA, i_02b4ca),
    (0x02B4CE, i_02b4ce),
    (0x02B4D2, i_02b4d2),
    (0x02B4D6, i_02b4d6),
    (0x02B4DC, i_02b4dc),
    (0x02B4E0, i_02b4e0),
    (0x02B4E4, i_02b4e4),
    (0x02B4E8, i_02b4e8),
    (0x02B4EA, i_02b4ea),
    (0x02B4EC, i_02b4ec),
    (0x02B4EE, i_02b4ee),
    (0x02B4F2, i_02b4f2),
    (0x02B4F6, i_02b4f6),
    (0x02B4FA, i_02b4fa),
    (0x02B4FE, i_02b4fe),
    (0x02B502, i_02b502),
    (0x02B506, i_02b506),
    (0x02B50C, i_02b50c),
    (0x02B50E, i_02b50e),
    (0x02B512, i_02b512),
    (0x02B518, i_02b518),
    (0x02B51C, i_02b51c),
    (0x02B522, i_02b522),
    (0x02B526, i_02b526),
    (0x02B52C, i_02b52c),
    (0x02B52E, i_02b52e),
    (0x02B532, i_02b532),
    (0x02B536, i_02b536),
    (0x02B53E, i_02b53e),
    (0x02B542, i_02b542),
    (0x02B544, i_02b544),
    (0x02B548, i_02b548),
    (0x02B54C, i_02b54c),
    (0x02B552, i_02b552),
    (0x02B558, i_02b558),
    (0x02B55C, i_02b55c),
    (0x02B560, i_02b560),
    (0x02B564, i_02b564),
    (0x02B568, i_02b568),
    (0x02B56C, i_02b56c),
    (0x02B570, i_02b570),
    (0x02B574, i_02b574),
    (0x02B578, i_02b578),
    (0x02B57C, i_02b57c),
    (0x02B580, i_02b580),
    (0x02B584, i_02b584),
    (0x02B588, i_02b588),
    (0x02B58C, i_02b58c),
    (0x02B590, i_02b590),
    (0x02B594, i_02b594),
    (0x02B598, i_02b598),
    (0x02B59C, i_02b59c),
    (0x02B5A4, i_02b5a4),
    (0x02B5A8, i_02b5a8),
    (0x02B5AC, i_02b5ac),
    (0x02B5B0, i_02b5b0),
    (0x02B5B2, i_02b5b2),
    (0x02B5B4, i_02b5b4),
    (0x02B5B8, i_02b5b8),
    (0x02B5BA, i_02b5ba),
    (0x02B5BC, i_02b5bc),
    (0x02B5BE, i_02b5be),
    (0x02B5C2, i_02b5c2),
    (0x02B5C6, i_02b5c6),
    (0x02B5CA, i_02b5ca),
    (0x02B5CE, i_02b5ce),
    (0x02B5D2, i_02b5d2),
    (0x02B5D6, i_02b5d6),
    (0x02B5DA, i_02b5da),
    (0x02B5DC, i_02b5dc),
    (0x02B5DE, i_02b5de),
    (0x02B5E0, i_02b5e0),
    (0x02B5E4, i_02b5e4),
    (0x02B5E8, i_02b5e8),
    (0x02B5EC, i_02b5ec),
    (0x02B5F0, i_02b5f0),
    (0x02B5F4, i_02b5f4),
    (0x02B5F8, i_02b5f8),
    (0x02B5FE, i_02b5fe),
    (0x02B600, i_02b600),
    (0x02B604, i_02b604),
    (0x02B60A, i_02b60a),
    (0x02B60E, i_02b60e),
    (0x02B614, i_02b614),
    (0x02B618, i_02b618),
    (0x02B61E, i_02b61e),
    (0x02B620, i_02b620),
    (0x02B624, i_02b624),
    (0x02B628, i_02b628),
    (0x02B62E, i_02b62e),
    (0x02B632, i_02b632),
    (0x02B636, i_02b636),
    (0x02B63A, i_02b63a),
    (0x02B63E, i_02b63e),
    (0x02B640, i_02b640),
    (0x02B644, i_02b644),
    (0x02B648, i_02b648),
    (0x02B64C, i_02b64c),
    (0x02B64E, i_02b64e),
    (0x02B652, i_02b652),
    (0x02B656, i_02b656),
    (0x02B65A, i_02b65a),
    (0x02B65C, i_02b65c),
    (0x02B660, i_02b660),
    (0x02B664, i_02b664),
    (0x02B66A, i_02b66a),
    (0x02B66E, i_02b66e),
    (0x02B672, i_02b672),
    (0x02B676, i_02b676),
    (0x02B678, i_02b678),
    (0x02B67C, i_02b67c),
    (0x02B680, i_02b680),
    (0x02B684, i_02b684),
    (0x02B686, i_02b686),
    (0x02B68A, i_02b68a),
    (0x02B68E, i_02b68e),
    (0x02B694, i_02b694),
    (0x02B69A, i_02b69a),
    (0x02B69E, i_02b69e),
    (0x02B6A2, i_02b6a2),
    (0x02B6A6, i_02b6a6),
    (0x02B6A8, i_02b6a8),
    (0x02B6AC, i_02b6ac),
    (0x02B6B0, i_02b6b0),
    (0x02B6B4, i_02b6b4),
    (0x02B6B6, i_02b6b6),
    (0x02B6BA, i_02b6ba),
    (0x02B6BE, i_02b6be),
    (0x02B6C2, i_02b6c2),
    (0x02B6C4, i_02b6c4),
    (0x02B6C8, i_02b6c8),
    (0x02B6CC, i_02b6cc),
    (0x02B6D2, i_02b6d2),
    (0x02B6D8, i_02b6d8),
    (0x02B6DE, i_02b6de),
    (0x02B6E4, i_02b6e4),
    (0x02B6EA, i_02b6ea),
    (0x02B6EE, i_02b6ee),
    (0x02B6F2, i_02b6f2),
    (0x02B6F4, i_02b6f4),
    (0x02B6FA, i_02b6fa),
    (0x02B6FE, i_02b6fe),
    (0x02B702, i_02b702),
    (0x02B706, i_02b706),
    (0x02B70A, i_02b70a),
    (0x02B70C, i_02b70c),
    (0x02B70E, i_02b70e),
    (0x02B710, i_02b710),
    (0x02B714, i_02b714),
    (0x02B718, i_02b718),
    (0x02B71C, i_02b71c),
    (0x02B720, i_02b720),
    (0x02B724, i_02b724),
    (0x02B728, i_02b728),
    (0x02B72E, i_02b72e),
    (0x02B730, i_02b730),
    (0x02B734, i_02b734),
    (0x02B73A, i_02b73a),
    (0x02B73E, i_02b73e),
    (0x02B744, i_02b744),
    (0x02B748, i_02b748),
    (0x02B74E, i_02b74e),
    (0x02B750, i_02b750),
    (0x02B754, i_02b754),
    (0x02B758, i_02b758),
    (0x02B75E, i_02b75e),
    (0x02B762, i_02b762),
    (0x02B764, i_02b764),
    (0x02B76A, i_02b76a),
    (0x02B76E, i_02b76e),
    (0x02B772, i_02b772),
    (0x02B776, i_02b776),
    (0x02B77C, i_02b77c),
    (0x02B780, i_02b780),
    (0x02B784, i_02b784),
    (0x02B786, i_02b786),
    (0x02B78A, i_02b78a),
    (0x02B78E, i_02b78e),
    (0x02B792, i_02b792),
    (0x02B794, i_02b794),
    (0x02B798, i_02b798),
    (0x02B79C, i_02b79c),
    (0x02B7A2, i_02b7a2),
    (0x02B7A8, i_02b7a8),
    (0x02B7AC, i_02b7ac),
    (0x02B7B0, i_02b7b0),
    (0x02B7B4, i_02b7b4),
    (0x02B7B6, i_02b7b6),
    (0x02B7BA, i_02b7ba),
    (0x02B7BE, i_02b7be),
    (0x02B7C2, i_02b7c2),
    (0x02B7C4, i_02b7c4),
    (0x02B7C8, i_02b7c8),
    (0x02B7CC, i_02b7cc),
    (0x02B7D2, i_02b7d2),
    (0x02B7D8, i_02b7d8),
    (0x02B7DA, i_02b7da),
    (0x02B7E0, i_02b7e0),
    (0x02B7E4, i_02b7e4),
    (0x02B7E8, i_02b7e8),
    (0x02B7EC, i_02b7ec),
    (0x02B7F0, i_02b7f0),
    (0x02B7F2, i_02b7f2),
    (0x02B7F4, i_02b7f4),
    (0x02B7F6, i_02b7f6),
    (0x02B7FA, i_02b7fa),
    (0x02B7FE, i_02b7fe),
    (0x02B802, i_02b802),
    (0x02B806, i_02b806),
    (0x02B80A, i_02b80a),
    (0x02B80E, i_02b80e),
    (0x02B814, i_02b814),
    (0x02B816, i_02b816),
    (0x02B81A, i_02b81a),
    (0x02B820, i_02b820),
    (0x02B824, i_02b824),
    (0x02B82A, i_02b82a),
    (0x02B82E, i_02b82e),
    (0x02B834, i_02b834),
    (0x02B836, i_02b836),
    (0x02B83A, i_02b83a),
    (0x02B83E, i_02b83e),
    (0x02B844, i_02b844),
    (0x02B848, i_02b848),
    (0x02B84A, i_02b84a),
    (0x02B850, i_02b850),
    (0x02B854, i_02b854),
    (0x02B858, i_02b858),
    (0x02B85C, i_02b85c),
    (0x02B862, i_02b862),
    (0x02B866, i_02b866),
    (0x02B86A, i_02b86a),
    (0x02B86C, i_02b86c),
    (0x02B870, i_02b870),
    (0x02B874, i_02b874),
    (0x02B878, i_02b878),
    (0x02B87A, i_02b87a),
    (0x02B87E, i_02b87e),
    (0x02B882, i_02b882),
    (0x02B888, i_02b888),
    (0x02B88E, i_02b88e),
    (0x02B892, i_02b892),
    (0x02B896, i_02b896),
    (0x02B89A, i_02b89a),
    (0x02B89C, i_02b89c),
    (0x02B8A0, i_02b8a0),
    (0x02B8A4, i_02b8a4),
    (0x02B8A8, i_02b8a8),
    (0x02B8AA, i_02b8aa),
    (0x02B8AE, i_02b8ae),
    (0x02B8B2, i_02b8b2),
    (0x02B8B6, i_02b8b6),
    (0x02B8B8, i_02b8b8),
    (0x02B8BC, i_02b8bc),
    (0x02B8C0, i_02b8c0),
    (0x02B8C6, i_02b8c6),
    (0x02B8CC, i_02b8cc),
    (0x02B8D8, i_02b8d8),
    (0x02B8DE, i_02b8de),
    (0x02B8E2, i_02b8e2),
    (0x02B8E6, i_02b8e6),
    (0x02B8E8, i_02b8e8),
    (0x02B8EC, i_02b8ec),
    (0x02B8F2, i_02b8f2),
    (0x02B8F6, i_02b8f6),
    (0x02B8FC, i_02b8fc),
    (0x02B8FE, i_02b8fe),
    (0x02B902, i_02b902),
    (0x02B908, i_02b908),
    (0x02B90C, i_02b90c),
    (0x02B912, i_02b912),
    (0x02B916, i_02b916),
    (0x02B918, i_02b918),
    (0x02B91C, i_02b91c),
    (0x02B920, i_02b920),
    (0x02B924, i_02b924),
    (0x02B928, i_02b928),
    (0x02B92E, i_02b92e),
    (0x02B932, i_02b932),
    (0x02B938, i_02b938),
    (0x02B93C, i_02b93c),
    (0x02B93E, i_02b93e),
    (0x02B942, i_02b942),
    (0x02B946, i_02b946),
    (0x02B948, i_02b948),
    (0x02B94C, i_02b94c),
    (0x02B950, i_02b950),
    (0x02B956, i_02b956),
    (0x02B95A, i_02b95a),
    (0x02B95E, i_02b95e),
    (0x02B960, i_02b960),
    (0x02B964, i_02b964),
    (0x02B968, i_02b968),
    (0x02B96C, i_02b96c),
    (0x02B970, i_02b970),
    (0x02B972, i_02b972),
    (0x02B976, i_02b976),
    (0x02B97A, i_02b97a),
    (0x02B97E, i_02b97e),
    (0x02B980, i_02b980),
    (0x02B986, i_02b986),
    (0x02B98A, i_02b98a),
    (0x02B98E, i_02b98e),
    (0x02B990, i_02b990),
    (0x02B996, i_02b996),
    (0x02B99A, i_02b99a),
    (0x02B99E, i_02b99e),
    (0x02B9A2, i_02b9a2),
    (0x02B9A4, i_02b9a4),
    (0x02B9A8, i_02b9a8),
    (0x02B9AC, i_02b9ac),
    (0x02B9B0, i_02b9b0),
    (0x02B9B2, i_02b9b2),
    (0x02B9B6, i_02b9b6),
    (0x02B9BA, i_02b9ba),
    (0x02B9BE, i_02b9be),
    (0x02B9C2, i_02b9c2),
    (0x02B9C6, i_02b9c6),
    (0x02B9CC, i_02b9cc),
    (0x02B9D2, i_02b9d2),
    (0x02B9D6, i_02b9d6),
    (0x02B9DA, i_02b9da),
    (0x02B9DE, i_02b9de),
    (0x02B9E0, i_02b9e0),
    (0x02B9E4, i_02b9e4),
    (0x02B9E8, i_02b9e8),
    (0x02B9EE, i_02b9ee),
    (0x02B9F4, i_02b9f4),
    (0x02B9F8, i_02b9f8),
    (0x02B9FC, i_02b9fc),
    (0x02BA00, i_02ba00),
    (0x02BA02, i_02ba02),
    (0x02BA06, i_02ba06),
    (0x02BA0A, i_02ba0a),
    (0x02BA0E, i_02ba0e),
    (0x02BA10, i_02ba10),
    (0x02BA14, i_02ba14),
    (0x02BA18, i_02ba18),
    (0x02BA1C, i_02ba1c),
    (0x02BA1E, i_02ba1e),
    (0x02BA22, i_02ba22),
    (0x02BA26, i_02ba26),
    (0x02BA2C, i_02ba2c),
    (0x02BA32, i_02ba32),
    (0x02BA3E, i_02ba3e),
    (0x02BA44, i_02ba44),
    (0x02BA48, i_02ba48),
    (0x02BA4C, i_02ba4c),
    (0x02BA4E, i_02ba4e),
    (0x02BA52, i_02ba52),
    (0x02BA58, i_02ba58),
    (0x02BA5C, i_02ba5c),
    (0x02BA62, i_02ba62),
    (0x02BA64, i_02ba64),
    (0x02BA68, i_02ba68),
    (0x02BA6E, i_02ba6e),
    (0x02BA72, i_02ba72),
    (0x02BA78, i_02ba78),
    (0x02BA7C, i_02ba7c),
    (0x02BA7E, i_02ba7e),
    (0x02BA82, i_02ba82),
    (0x02BA86, i_02ba86),
    (0x02BA8A, i_02ba8a),
    (0x02BA8E, i_02ba8e),
    (0x02BA94, i_02ba94),
    (0x02BA98, i_02ba98),
    (0x02BA9E, i_02ba9e),
    (0x02BAA2, i_02baa2),
    (0x02BAA4, i_02baa4),
    (0x02BAA8, i_02baa8),
    (0x02BAAC, i_02baac),
    (0x02BAAE, i_02baae),
    (0x02BAB2, i_02bab2),
    (0x02BAB6, i_02bab6),
    (0x02BABC, i_02babc),
    (0x02BAC0, i_02bac0),
    (0x02BAC4, i_02bac4),
    (0x02BAC6, i_02bac6),
    (0x02BACA, i_02baca),
    (0x02BACE, i_02bace),
    (0x02BAD2, i_02bad2),
    (0x02BAD6, i_02bad6),
    (0x02BAD8, i_02bad8),
    (0x02BADC, i_02badc),
    (0x02BAE0, i_02bae0),
    (0x02BAE4, i_02bae4),
    (0x02BAE6, i_02bae6),
    (0x02BAEC, i_02baec),
    (0x02BAF0, i_02baf0),
    (0x02BAF4, i_02baf4),
    (0x02BAF6, i_02baf6),
    (0x02BAFC, i_02bafc),
    (0x02BB00, i_02bb00),
    (0x02BB04, i_02bb04),
    (0x02BB08, i_02bb08),
    (0x02BB0A, i_02bb0a),
    (0x02BB0E, i_02bb0e),
    (0x02BB12, i_02bb12),
    (0x02BB16, i_02bb16),
    (0x02BB18, i_02bb18),
    (0x02BB1C, i_02bb1c),
    (0x02BB20, i_02bb20),
    (0x02BB24, i_02bb24),
    (0x02BB28, i_02bb28),
    (0x02BB2C, i_02bb2c),
    (0x02BB32, i_02bb32),
    (0x02BB38, i_02bb38),
    (0x02BB3C, i_02bb3c),
    (0x02BB40, i_02bb40),
    (0x02BB44, i_02bb44),
    (0x02BB46, i_02bb46),
    (0x02BB4A, i_02bb4a),
    (0x02BB4E, i_02bb4e),
    (0x02BB54, i_02bb54),
    (0x02BB5A, i_02bb5a),
    (0x02BB5E, i_02bb5e),
    (0x02BB62, i_02bb62),
    (0x02BB66, i_02bb66),
    (0x02BB68, i_02bb68),
    (0x02BB6C, i_02bb6c),
    (0x02BB70, i_02bb70),
    (0x02BB74, i_02bb74),
    (0x02BB76, i_02bb76),
    (0x02BB7A, i_02bb7a),
    (0x02BB7E, i_02bb7e),
    (0x02BB82, i_02bb82),
    (0x02BB84, i_02bb84),
    (0x02BB88, i_02bb88),
    (0x02BB8C, i_02bb8c),
    (0x02BB92, i_02bb92),
    (0x02BB98, i_02bb98),
    (0x02BBA4, i_02bba4),
    (0x02BBAA, i_02bbaa),
    (0x02BBAE, i_02bbae),
    (0x02BBB2, i_02bbb2),
    (0x02BBB8, i_02bbb8),
    (0x02BBBA, i_02bbba),
    (0x02BBBE, i_02bbbe),
    (0x02BBC4, i_02bbc4),
    (0x02BBC8, i_02bbc8),
    (0x02BBCE, i_02bbce),
    (0x02BBD4, i_02bbd4),
    (0x02BBD8, i_02bbd8),
    (0x02BBDA, i_02bbda),
    (0x02BBDE, i_02bbde),
    (0x02BBE2, i_02bbe2),
    (0x02BBE8, i_02bbe8),
    (0x02BBEC, i_02bbec),
    (0x02BBEE, i_02bbee),
    (0x02BBF2, i_02bbf2),
    (0x02BBF6, i_02bbf6),
    (0x02BBFC, i_02bbfc),
    (0x02BC00, i_02bc00),
    (0x02BC04, i_02bc04),
    (0x02BC08, i_02bc08),
    (0x02BC0E, i_02bc0e),
    (0x02BC12, i_02bc12),
    (0x02BC18, i_02bc18),
    (0x02BC1A, i_02bc1a),
    (0x02BC1E, i_02bc1e),
    (0x02BC24, i_02bc24),
    (0x02BC28, i_02bc28),
    (0x02BC2E, i_02bc2e),
    (0x02BC34, i_02bc34),
    (0x02BC38, i_02bc38),
    (0x02BC3A, i_02bc3a),
    (0x02BC3E, i_02bc3e),
    (0x02BC42, i_02bc42),
    (0x02BC48, i_02bc48),
    (0x02BC50, i_02bc50),
    (0x02BC54, i_02bc54),
    (0x02BC56, i_02bc56),
    (0x02BC5A, i_02bc5a),
    (0x02BC5E, i_02bc5e),
    (0x02BC64, i_02bc64),
    (0x02BC68, i_02bc68),
    (0x02BC6E, i_02bc6e),
    (0x02BC72, i_02bc72),
    (0x02BC76, i_02bc76),
    (0x02BC78, i_02bc78),
    (0x02BC7C, i_02bc7c),
    (0x02BC80, i_02bc80),
    (0x02BC84, i_02bc84),
    (0x02BC88, i_02bc88),
    (0x02BC8A, i_02bc8a),
    (0x02BC90, i_02bc90),
    (0x02BC96, i_02bc96),
    (0x02BC9A, i_02bc9a),
    (0x02BC9E, i_02bc9e),
    (0x02BCA2, i_02bca2),
    (0x02BCA6, i_02bca6),
    (0x02BCAC, i_02bcac),
    (0x02BCB2, i_02bcb2),
    (0x02BCB8, i_02bcb8),
    (0x02BCBC, i_02bcbc),
    (0x02BCC0, i_02bcc0),
    (0x02BCC4, i_02bcc4),
    (0x02BCC6, i_02bcc6),
    (0x02BCCA, i_02bcca),
    (0x02BCCE, i_02bcce),
    (0x02BCD2, i_02bcd2),
    (0x02BCD8, i_02bcd8),
    (0x02BCDC, i_02bcdc),
    (0x02BCE0, i_02bce0),
    (0x02BCE2, i_02bce2),
    (0x02BCE8, i_02bce8),
    (0x02BCEC, i_02bcec),
    (0x02BCF0, i_02bcf0),
    (0x02BCF2, i_02bcf2),
    (0x02BCF6, i_02bcf6),
    (0x02BCFA, i_02bcfa),
    (0x02BCFE, i_02bcfe),
    (0x02BD00, i_02bd00),
    (0x02BD02, i_02bd02),
    (0x02BD06, i_02bd06),
    (0x02BD08, i_02bd08),
    (0x02BD0C, i_02bd0c),
    (0x02BD12, i_02bd12),
    (0x02BD16, i_02bd16),
    (0x02BD1C, i_02bd1c),
    (0x02BD1E, i_02bd1e),
    (0x02BD22, i_02bd22),
    (0x02BD28, i_02bd28),
    (0x02BD2C, i_02bd2c),
    (0x02BD32, i_02bd32),
    (0x02BD38, i_02bd38),
    (0x02BD3C, i_02bd3c),
    (0x02BD3E, i_02bd3e),
    (0x02BD42, i_02bd42),
    (0x02BD46, i_02bd46),
    (0x02BD48, i_02bd48),
    (0x02BD4C, i_02bd4c),
    (0x02BD50, i_02bd50),
    (0x02BD54, i_02bd54),
    (0x02BD58, i_02bd58),
    (0x02BD5C, i_02bd5c),
    (0x02BD5E, i_02bd5e),
    (0x02BD62, i_02bd62),
    (0x02BD66, i_02bd66),
    (0x02BD6A, i_02bd6a),
    (0x02BD6E, i_02bd6e),
    (0x02BD70, i_02bd70),
    (0x02BD74, i_02bd74),
    (0x02BD78, i_02bd78),
    (0x02BD7C, i_02bd7c),
    (0x02BD7E, i_02bd7e),
    (0x02BD84, i_02bd84),
    (0x02BD88, i_02bd88),
    (0x02BD8C, i_02bd8c),
    (0x02BD8E, i_02bd8e),
    (0x02BD94, i_02bd94),
    (0x02BD98, i_02bd98),
    (0x02BD9C, i_02bd9c),
    (0x02BDA0, i_02bda0),
    (0x02BDA2, i_02bda2),
    (0x02BDA6, i_02bda6),
    (0x02BDAA, i_02bdaa),
    (0x02BDAE, i_02bdae),
    (0x02BDB0, i_02bdb0),
    (0x02BDB4, i_02bdb4),
    (0x02BDB8, i_02bdb8),
    (0x02BDBC, i_02bdbc),
    (0x02BDC0, i_02bdc0),
    (0x02BDC4, i_02bdc4),
    (0x02BDCA, i_02bdca),
    (0x02BDD0, i_02bdd0),
    (0x02BDD4, i_02bdd4),
    (0x02BDD8, i_02bdd8),
    (0x02BDDC, i_02bddc),
    (0x02BDDE, i_02bdde),
    (0x02BDE2, i_02bde2),
    (0x02BDE6, i_02bde6),
    (0x02BDEC, i_02bdec),
    (0x02BDF2, i_02bdf2),
    (0x02BDF6, i_02bdf6),
    (0x02BDFA, i_02bdfa),
    (0x02BDFE, i_02bdfe),
    (0x02BE00, i_02be00),
    (0x02BE04, i_02be04),
    (0x02BE08, i_02be08),
    (0x02BE0C, i_02be0c),
    (0x02BE0E, i_02be0e),
    (0x02BE12, i_02be12),
    (0x02BE16, i_02be16),
    (0x02BE1A, i_02be1a),
    (0x02BE1C, i_02be1c),
    (0x02BE20, i_02be20),
    (0x02BE24, i_02be24),
    (0x02BE2A, i_02be2a),
    (0x02BE30, i_02be30),
    (0x02BE32, i_02be32),
    (0x02BE34, i_02be34),
    (0x02BE38, i_02be38),
    (0x02BE3A, i_02be3a),
    (0x02BE3E, i_02be3e),
];
