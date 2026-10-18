// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p08_0) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $080000: andi.l #$ffff, d0
pub fn i_080000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80006;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $080006: divu.w #$4, d0
pub fn i_080006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8000A;
    cpu.ir = 0x80FC;
    let v1: u32 = 0x4;
    cpu.sr.set(StatusRegister::C, false);
    let dv = v1 & 0xFFFF;
    if dv == 0 { cpu.enter_group2_exception(bus, Exception::DivideByZero); return 38; }
    let q = cpu.d[0] / dv; let rm = cpu.d[0] % dv;
    if q > 0xFFFF { cpu.sr.set(StatusRegister::V, true); cpu.sr.set(StatusRegister::N, true); cpu.sr.set(StatusRegister::Z, false); return 76; }
    cpu.d[0] = (rm << 16) | (q & 0xFFFF);
    cpu.sr.set_nz(q, Size::Word); cpu.sr.set(StatusRegister::V, false);
    140
}
/// $08000A: andi.l #$ffff, d0
pub fn i_08000a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80010;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $080010: moveq #$0, d1
pub fn i_080010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80012;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $080012: move.w $66(a6), d1
pub fn i_080012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80016;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $080016: divu.w d0, d1
pub fn i_080016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80018;
    cpu.ir = 0x82C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set(StatusRegister::C, false);
    let dv = v1 & 0xFFFF;
    if dv == 0 { cpu.enter_group2_exception(bus, Exception::DivideByZero); return 38; }
    let q = cpu.d[1] / dv; let rm = cpu.d[1] % dv;
    if q > 0xFFFF { cpu.sr.set(StatusRegister::V, true); cpu.sr.set(StatusRegister::N, true); cpu.sr.set(StatusRegister::Z, false); return 76; }
    cpu.d[1] = (rm << 16) | (q & 0xFFFF);
    cpu.sr.set_nz(q, Size::Word); cpu.sr.set(StatusRegister::V, false);
    140
}
/// $080018: move.w #$3, d0
pub fn i_080018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8001C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08001C: sub.w d1, d0
pub fn i_08001c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8001E;
    cpu.ir = 0x9041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $08001E: bpl.w $80024
pub fn i_08001e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80022;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x80024; return 10; }
    8
}
/// $080022: clr.w d0
pub fn i_080022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80024;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $080024: rts
pub fn i_080024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80026;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $080026: movea.l $c(a6), a0
pub fn i_080026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8002A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $08002A: move.w $7c(a0), d0
pub fn i_08002a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8002E;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
// ===== SetTaskW_08002e  ($08002E, 6 B, kernel/task_setters_w.c)
/// $08002E: move.w d0, $7c(a6)
pub fn i_08002e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80032;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $080032: rts
pub fn i_080032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80034;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_LeaderGoneCheck_080034  ($080034, 20 B, enemies/squad/deploy_module_07fbd2.s)
/// $080034: movea.l $c(a6), a0
pub fn i_080034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80038;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $080038: cmpi.b #$ff, $20(a0)
pub fn i_080038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8003E;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $08003E: bne.w $8004e
pub fn i_08003e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80042;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8004E; return 10; }
    8
}
/// $080042: jsr $4a166.l
pub fn i_080042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80048;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80048);
    cpu.pc = 0x4A166;
    16
}
// ===== SetC_080048  ($080048, 6 B, thunks/ccr_helpers.c)
/// $080048: ori.b #$1, ccr
pub fn i_080048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8004C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $08004C: rts
pub fn i_08004c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8004E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearC_08004e  ($08004E, 6 B, thunks/ccr_helpers.c)
/// $08004E: andi.b #$fe, ccr
pub fn i_08004e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80052;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $080052: rts
pub fn i_080052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80054;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_DeferredRelease_080054  ($080054, 46 B, enemies/squad/deploy_module_07fbd2.s)
/// $080054: jsr $80034(pc)
pub fn i_080054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80058;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x80058);
    cpu.pc = 0x80034;
    16
}
/// $080058: tst.b $21(a6)
pub fn i_080058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8005C;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $08005C: beq.w $80082
pub fn i_08005c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80060;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x80082; return 10; }
    8
}
/// $080060: jsr $2870a.l
pub fn i_080060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80066;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80066);
    cpu.pc = 0x2870A;
    16
}
/// $080066: bcc.w $80088
pub fn i_080066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8006A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x80088; return 10; }
    8
}
/// $08006A: movea.l $c(a6), a0
pub fn i_08006a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8006E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $08006E: move.w $78(a6), d0
pub fn i_08006e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80072;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $080072: bclr.b d0, $21(a0)
pub fn i_080072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80076;
    cpu.ir = 0x1A8;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $080076: tst.w d0
pub fn i_080076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80078;
    cpu.ir = 0x4A40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $080078: bne.w $80082
pub fn i_080078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8007C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x80082; return 10; }
    8
}
/// $08007C: move.b #$1, $78(a0)
pub fn i_08007c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80082;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== JsrAbsThunk_080082  ($080082, 8 B, thunks/jsr_abs_thunks.c)
/// $080082: jsr $49fd0.l
pub fn i_080082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80088;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80088);
    cpu.pc = 0x49FD0;
    16
}
/// $080088: rts
pub fn i_080088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8008A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_CompareDepth_08008a  ($08008A, 16 B, enemies/squad/deploy_module_07fbd2.s)
/// $08008A: movea.l $8(a6), a1
pub fn i_08008a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8008E;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08008E: move.b $10(a6), d0
pub fn i_08008e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80092;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $080092: cmp.b $10(a1), d0
pub fn i_080092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80096;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $080096: bcs.w $800a0
pub fn i_080096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8009A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x800A0; return 10; }
    8
}
// ===== ClearXN_08009a  ($08009A, 6 B, thunks/ccr_helpers.c)
/// $08009A: andi.b #$ee, ccr
pub fn i_08009a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8009E;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $08009E: rts
pub fn i_08009e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x800A0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_0800a0  ($0800A0, 6 B, thunks/ccr_helpers.c)
/// $0800A0: ori.b #$11, ccr
pub fn i_0800a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x800A4;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0800A4: rts
pub fn i_0800a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x800A6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_MgrInit_0800a6  ($0800A6, 30 B, enemies/squad/deploy_module_07fbd2.s)
/// $0800A6: moveq #$0, d0
pub fn i_0800a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x800A8;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0800A8: move.b d0, $20(a6)
pub fn i_0800a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x800AC;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0800AC: move.b d0, $21(a6)
pub fn i_0800ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x800B0;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0800B0: jsr $5e9b6.l
pub fn i_0800b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x800B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x800B6);
    cpu.pc = 0x5E9B6;
    16
}
/// $0800B6: andi.w #$3, d0
pub fn i_0800b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x800BA;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0800BA: clr.w d2
pub fn i_0800ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x800BC;
    cpu.ir = 0x4242;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0800BC: move.b $98(a6), d2
pub fn i_0800bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x800C0;
    cpu.ir = 0x142E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0800C0: bne.w $800cc
pub fn i_0800c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x800C4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x800CC; return 10; }
    8
}
// ===== SetTaskHandler_0800c4  ($0800C4, 8 B, kernel/task_handlers.c)
/// $0800C4: lea.l $803e8(pc), a1
pub fn i_0800c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x800C8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x803E8;
    16
}
/// $0800C8: move.l a1, (a6)
pub fn i_0800c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x800CA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0800CA: rts
pub fn i_0800ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x800CC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_MgrSpawnLoop_0800cc  ($0800CC, 60 B, enemies/squad/deploy_module_07fbd2.s)
/// $0800CC: subq.w #$1, d2
pub fn i_0800cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x800CE;
    cpu.ir = 0x5342;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0800CE: clr.w d1
pub fn i_0800ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x800D0;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0800D0: movem.l d0-d2, -(a7)
pub fn i_0800d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x800D4;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $0800D4: lea.l $80110(pc), a1
pub fn i_0800d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x800D8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x80110;
    16
}
/// $0800D8: jsr $4ae.l
pub fn i_0800d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x800DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x800DE);
    cpu.pc = 0x4AE;
    16
}
/// $0800DE: jsr $5dd02.l
pub fn i_0800de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x800E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x800E4);
    cpu.pc = 0x5DD02;
    16
}
/// $0800E4: movem.l (a7)+, d0-d2
pub fn i_0800e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x800E8;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $0800E8: move.w d0, $78(a0)
pub fn i_0800e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x800EC;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x78);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0800EC: move.w d1, $84(a0)
pub fn i_0800ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x800F0;
    cpu.ir = 0x3141;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x84);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0800F0: addq.w #$1, d1
pub fn i_0800f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x800F2;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0800F2: dbra d2, $800d0
pub fn i_0800f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x800F6;
    cpu.ir = 0x51CA;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x800F6; return 12; }
    let n = (cpu.d[2] as u16).wrapping_sub(1);
    cpu.d[2] = (cpu.d[2] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x800F6; return 14; }
    cpu.pc = 0x800D0;
    10
}
/// $0800F6: lea.l $800fc(pc), a1
pub fn i_0800f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x800FA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x800FC;
    16
}
/// $0800FA: move.l a1, (a6)
pub fn i_0800fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x800FC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0800FC: move.b $98(a6), d0
pub fn i_0800fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80100;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $080100: cmp.b $20(a6), d0
pub fn i_080100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80104;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $080104: bne.w $8010e
pub fn i_080104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80108;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8010E; return 10; }
    8
}
// ===== SetTaskHandler_080108  ($080108, 8 B, kernel/task_handlers.c)
/// $080108: lea.l $803e8(pc), a1
pub fn i_080108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8010C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x803E8;
    16
}
/// $08010C: move.l a1, (a6)
pub fn i_08010c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8010E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08010E: rts
pub fn i_08010e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80110;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetTaskHandler_0801e6  ($0801E6, 8 B, kernel/task_handlers.c)
/// $0801E6: lea.l $803d2(pc), a1
pub fn i_0801e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x801EA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x803D2;
    16
}
/// $0801EA: move.l a1, (a6)
pub fn i_0801ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x801EC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0801EC: rts
pub fn i_0801ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x801EE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetTaskHandler_080226  ($080226, 8 B, kernel/task_handlers.c)
/// $080226: lea.l $803d2(pc), a1
pub fn i_080226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8022A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x803D2;
    16
}
/// $08022A: move.l a1, (a6)
pub fn i_08022a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8022C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08022C: rts
pub fn i_08022c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8022E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_ChildLeap_08022e  ($08022E, 170 B, enemies/squad/deploy_module_07fbd2.s)
/// $08022E: move.w #$400, d1
pub fn i_08022e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80232;
    cpu.ir = 0x323C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $080232: move.w #$88, d0
pub fn i_080232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80236;
    cpu.ir = 0x303C;
    let v1: u32 = 0x88;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $080236: jsr $13c0e.l
pub fn i_080236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8023C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8023C);
    cpu.pc = 0x13C0E;
    16
}
/// $08023C: move.w d1, $28(a6)
pub fn i_08023c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80240;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $080240: move.w d2, $2a(a6)
pub fn i_080240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80244;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $080244: clr.b $21(a6)
pub fn i_080244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80248;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $080248: move.w #$ffe0, $2c(a6)
pub fn i_080248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8024E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08024E: tst.w $84(a6)
pub fn i_08024e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80252;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $080252: bne.w $80260
pub fn i_080252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80256;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x80260; return 10; }
    8
}
/// $080256: move.w #$10b9, d0
pub fn i_080256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8025A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10B9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08025A: jsr $2352.l
pub fn i_08025a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80260;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80260);
    cpu.pc = 0x2352;
    16
}
/// $080260: lea.l $80266(pc), a1
pub fn i_080260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80264;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x80266;
    16
}
/// $080264: move.l a1, (a6)
pub fn i_080264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80266;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $080266: jsr $27cee.l
pub fn i_080266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8026C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8026C);
    cpu.pc = 0x27CEE;
    16
}
/// $08026C: tst.b $21(a6)
pub fn i_08026c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80270;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $080270: bne.w $8029a
pub fn i_080270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80274;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8029A; return 10; }
    8
}
/// $080274: cmpi.b #$80, $32(a6)
pub fn i_080274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8027A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x80, Size::Byte, false);
    16
}
/// $08027A: bcs.w $8029a
pub fn i_08027a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8027E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8029A; return 10; }
    8
}
/// $08027E: move.b #$ff, $21(a6)
pub fn i_08027e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80284;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $080284: move.w #$1000, d0
pub fn i_080284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80288;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $080288: jsr $28134.l
pub fn i_080288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8028E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8028E);
    cpu.pc = 0x28134;
    16
}
/// $08028E: andi.w #$ffe3, $38(a6)
pub fn i_08028e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80294;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $080294: ori.w #$c, $38(a6)
pub fn i_080294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8029A;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0xC) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $08029A: jsr $28d70.l
pub fn i_08029a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x802A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x802A0);
    cpu.pc = 0x28D70;
    16
}
/// $0802A0: cmpi.b #$ff, $32(a6)
pub fn i_0802a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x802A6;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $0802A6: beq.w $802c8
pub fn i_0802a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x802AA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x802C8; return 10; }
    8
}
/// $0802AA: addq.b #$3, $32(a6)
pub fn i_0802aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x802AE;
    cpu.ir = 0x562E;
    let ad1 = cpu.a[6].wrapping_add(0x32);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 3, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $0802AE: addq.b #$3, $33(a6)
pub fn i_0802ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x802B2;
    cpu.ir = 0x562E;
    let ad1 = cpu.a[6].wrapping_add(0x33);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 3, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $0802B2: cmpi.b #$2, $32(a6)
pub fn i_0802b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x802B8;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $0802B8: bhi.w $802c8
pub fn i_0802b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x802BC;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x802C8; return 10; }
    8
}
/// $0802BC: move.b #$ff, $32(a6)
pub fn i_0802bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x802C2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0802C2: move.b #$ff, $33(a6)
pub fn i_0802c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x802C8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0802C8: movea.l #$ffffffff, a0
pub fn i_0802c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x802CE;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0802CE: jsr $5dd5c.l
pub fn i_0802ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x802D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x802D4);
    cpu.pc = 0x5DD5C;
    16
}
/// $0802D4: bcc.w $802de
pub fn i_0802d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x802D8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x802DE; return 10; }
    8
}
// ===== SetTaskHandler_0802d8  ($0802D8, 8 B, kernel/task_handlers.c)
/// $0802D8: lea.l $803d2(pc), a1
pub fn i_0802d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x802DC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x803D2;
    16
}
/// $0802DC: move.l a1, (a6)
pub fn i_0802dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x802DE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0802DE: rts
pub fn i_0802de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x802E0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_ChildInitFixed_0802e0  ($0802E0, 154 B, enemies/squad/deploy_module_07fbd2.s)
/// $0802E0: move.w #$133, d1
pub fn i_0802e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x802E4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x133;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0802E4: jsr $236e.l
pub fn i_0802e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x802EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x802EA);
    cpu.pc = 0x236E;
    16
}
/// $0802EA: jsr $267e2.l
pub fn i_0802ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x802F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x802F0);
    cpu.pc = 0x267E2;
    16
}
/// $0802F0: lea.l $2beed0.l, a0
pub fn i_0802f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x802F6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BEED0;
    20
}
/// $0802F6: move.w $78(a6), d0
pub fn i_0802f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x802FA;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0802FA: lsl.w #$3, d0
pub fn i_0802fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x802FC;
    cpu.ir = 0xE748;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0802FC: move.w (a0, d0.w), $22(a6)
pub fn i_0802fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80302;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    22
}
/// $080302: move.w $2(a0, d0.w), $24(a6)
pub fn i_080302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80308;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    22
}
/// $080308: move.w $4(a0, d0.w), $72(a6)
pub fn i_080308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8030E;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x4).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad4, v3 as u16);
    22
}
/// $08030E: move.b #$80, $32(a6)
pub fn i_08030e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80314;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $080314: move.b #$80, $33(a6)
pub fn i_080314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8031A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08031A: move.b #$ff, $21(a6)
pub fn i_08031a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80320;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $080320: move.w #$1000, d0
pub fn i_080320(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80324;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $080324: jsr $28134.l
pub fn i_080324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8032A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8032A);
    cpu.pc = 0x28134;
    16
}
/// $08032A: andi.w #$ffe3, $38(a6)
pub fn i_08032a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80330;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $080330: ori.w #$c, $38(a6)
pub fn i_080330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80336;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0xC) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $080336: lea.l $2e3d68.l, a0
pub fn i_080336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8033C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E3D68;
    20
}
/// $08033C: move.w $78(a6), d0
pub fn i_08033c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80340;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $080340: add.w d0, d0
pub fn i_080340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80342;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $080342: add.w d0, d0
pub fn i_080342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80344;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $080344: move.l (a0, d0.w), $90(a6)
pub fn i_080344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8034A;
    cpu.ir = 0x2D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x90);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    30
}
/// $08034A: moveq #$0, d1
pub fn i_08034a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8034C;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08034C: move.w d1, $94(a6)
pub fn i_08034c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80350;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    bus.write16(ad2, v1 as u16);
    12
}
/// $080350: move.b d1, $96(a6)
pub fn i_080350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80354;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x96);
    bus.write8(ad2, v1 as u8);
    12
}
/// $080354: clr.b $7e(a6)
pub fn i_080354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80358;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x7E);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $080358: move.w #$10ba, d0
pub fn i_080358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8035C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10BA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08035C: jsr $2352.l
pub fn i_08035c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80362;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80362);
    cpu.pc = 0x2352;
    16
}
/// $080362: lea.l $723da.l, a1
pub fn i_080362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80368;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x723DA;
    20
}
/// $080368: jsr $4ae.l
pub fn i_080368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8036E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8036E);
    cpu.pc = 0x4AE;
    16
}
/// $08036E: jsr $5dd02.l
pub fn i_08036e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80374;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80374);
    cpu.pc = 0x5DD02;
    16
}
/// $080374: move.w #$fff0, $98(a0)
pub fn i_080374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8037A;
    cpu.ir = 0x317C;
    let v1: u32 = 0xFFF0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_08037a  ($08037A, 8 B, kernel/task_handlers.c)
/// $08037A: lea.l $80382(pc), a1
pub fn i_08037a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8037E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x80382;
    16
}
/// $08037E: move.l a1, (a6)
pub fn i_08037e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80380;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $080380: rts
pub fn i_080380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80382;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_ChildRun_080382  ($080382, 72 B, enemies/squad/deploy_module_07fbd2.s)
/// $080382: move.w $84(a6), d0
pub fn i_080382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80386;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $080386: add.w d0, d0
pub fn i_080386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80388;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $080388: addi.w #$80, d0
pub fn i_080388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8038C;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x80, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08038C: movea.l $c(a6), a0
pub fn i_08038c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80390;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $080390: movea.l $c(a0), a0
pub fn i_080390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80394;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $080394: move.w (a0, d0.w), $66(a6)
pub fn i_080394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8039A;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad4, v3 as u16);
    22
}
/// $08039A: lea.l $803a0(pc), a1
pub fn i_08039a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8039E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x803A0;
    16
}
/// $08039E: move.l a1, (a6)
pub fn i_08039e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x803A0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0803A0: jsr $8179e(pc)
pub fn i_0803a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x803A4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x803A4);
    cpu.pc = 0x8179E;
    16
}
/// $0803A4: jsr $78f8a.l
pub fn i_0803a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x803AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x803AA);
    cpu.pc = 0x78F8A;
    16
}
/// $0803AA: bcc.w $803b4
pub fn i_0803aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x803AE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x803B4; return 10; }
    8
}
/// $0803AE: lea.l $803d2(pc), a1
pub fn i_0803ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x803B2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x803D2;
    16
}
/// $0803B2: move.l a1, (a6)
pub fn i_0803b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x803B4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0803B4: jsr $28d70.l
pub fn i_0803b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x803BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x803BA);
    cpu.pc = 0x28D70;
    16
}
/// $0803BA: movea.l #$ffffffff, a0
pub fn i_0803ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x803C0;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0803C0: jsr $5dd5c.l
pub fn i_0803c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x803C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x803C6);
    cpu.pc = 0x5DD5C;
    16
}
/// $0803C6: bcc.w $803d0
pub fn i_0803c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x803CA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x803D0; return 10; }
    8
}
// ===== SetTaskHandler_0803ca  ($0803CA, 8 B, kernel/task_handlers.c)
/// $0803CA: lea.l $803d2(pc), a1
pub fn i_0803ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x803CE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x803D2;
    16
}
/// $0803CE: move.l a1, (a6)
pub fn i_0803ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x803D0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0803D0: rts
pub fn i_0803d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x803D2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_NotifyParent_0803d2  ($0803D2, 8 B, enemies/squad/deploy_module_07fbd2.s)
/// $0803D2: movea.l $c(a6), a0
pub fn i_0803d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x803D6;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0803D6: addq.b #$1, $20(a0)
pub fn i_0803d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x803DA;
    cpu.ir = 0x5228;
    let ad1 = cpu.a[0].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
// ===== Jsr5B6ThenJmpScheduler_0803da  ($0803DA, 14 B, kernel/jsr5b6_jmp518.c)
/// $0803DA: jsr $5b6.l
pub fn i_0803da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x803E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x803E0);
    cpu.pc = 0x5B6;
    16
}
/// $0803E0: jmp $518.l
pub fn i_0803e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x803E6;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Squad_Mgr8Slots_0803e8  ($0803E8, 280 B, enemies/squad/deploy_module_07fbd2.s)
/// $0803E8: move.w #$ffff, d0
pub fn i_0803e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x803EC;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0803EC: move.w d0, $80(a6)
pub fn i_0803ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x803F0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0803F0: move.w d0, $82(a6)
pub fn i_0803f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x803F4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0803F4: move.w d0, $84(a6)
pub fn i_0803f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x803F8;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0803F8: move.w d0, $86(a6)
pub fn i_0803f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x803FC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0803FC: move.w d0, $88(a6)
pub fn i_0803fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80400;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write16(ad2, v1 as u16);
    12
}
/// $080400: move.w d0, $8a(a6)
pub fn i_080400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80404;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $080404: move.w d0, $8c(a6)
pub fn i_080404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80408;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $080408: move.w d0, $8e(a6)
pub fn i_080408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8040C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08040C: moveq #$0, d0
pub fn i_08040c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8040E;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08040E: move.w d0, $70(a6)
pub fn i_08040e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80412;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $080412: move.b d0, $76(a6)
pub fn i_080412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80416;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write8(ad2, v1 as u8);
    12
}
/// $080416: move.b d0, $78(a6)
pub fn i_080416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8041A;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08041A: move.b d0, $77(a6)
pub fn i_08041a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8041E;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x77);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08041E: lea.l $2bea3e.l, a0
pub fn i_08041e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80424;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BEA3E;
    20
}
/// $080424: jsr $799de.l
pub fn i_080424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8042A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8042A);
    cpu.pc = 0x799DE;
    16
}
/// $08042A: move.w d0, $72(a6)
pub fn i_08042a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8042E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08042E: lea.l $2beac0.l, a0
pub fn i_08042e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80434;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BEAC0;
    20
}
/// $080434: jsr $799de.l
pub fn i_080434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8043A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8043A);
    cpu.pc = 0x799DE;
    16
}
/// $08043A: move.w d0, $74(a6)
pub fn i_08043a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8043E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08043E: lea.l $2beb42.l, a0
pub fn i_08043e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80444;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BEB42;
    20
}
/// $080444: jsr $799de.l
pub fn i_080444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8044A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8044A);
    cpu.pc = 0x799DE;
    16
}
/// $08044A: move.w d0, $66(a6)
pub fn i_08044a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8044E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08044E: lea.l $80454(pc), a1
pub fn i_08044e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80452;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x80454;
    16
}
/// $080452: move.l a1, (a6)
pub fn i_080452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80454;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $080454: tst.b $99(a6)
pub fn i_080454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80458;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $080458: bne.w $80466
pub fn i_080458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8045C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x80466; return 10; }
    8
}
/// $08045C: bclr.b #$1, $12(a6)
pub fn i_08045c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80462;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $080462: jmp $811d6(pc)
pub fn i_080462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80466;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x811D6;
    8
}
/// $080466: addq.w #$1, $70(a6)
pub fn i_080466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8046A;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08046A: move.w $70(a6), d0
pub fn i_08046a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8046E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08046E: cmp.w $74(a6), d0
pub fn i_08046e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80472;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $080472: blt.w $80506
pub fn i_080472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80476;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x80506; return 10; }
    8
}
/// $080476: move.w #$0, $70(a6)
pub fn i_080476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8047C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08047C: clr.w d0
pub fn i_08047c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8047E;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08047E: btst.l d0, $77(a6)
pub fn i_08047e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80482;
    cpu.ir = 0x12E;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x77);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $080482: beq.w $8048e
pub fn i_080482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80486;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8048E; return 10; }
    8
}
/// $080486: btst.l d0, $78(a6)
pub fn i_080486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8048A;
    cpu.ir = 0x12E;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $08048A: beq.w $804ac
pub fn i_08048a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8048E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x804AC; return 10; }
    8
}
/// $08048E: addq.w #$1, d0
pub fn i_08048e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80490;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $080490: cmpi.w #$8, d0
pub fn i_080490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80494;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x8, Size::Word, false);
    8
}
/// $080494: blt.b $8047e
pub fn i_080494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80496;
    cpu.ir = 0x6DE8;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x8047E; return 10; }
    8
}
/// $080496: clr.w d0
pub fn i_080496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80498;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $080498: btst.l d0, $77(a6)
pub fn i_080498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8049C;
    cpu.ir = 0x12E;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x77);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $08049C: beq.w $804ac
pub fn i_08049c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x804A0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x804AC; return 10; }
    8
}
/// $0804A0: addq.w #$1, d0
pub fn i_0804a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x804A2;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0804A2: cmpi.w #$8, d0
pub fn i_0804a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x804A6;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x8, Size::Word, false);
    8
}
/// $0804A6: blt.b $80498
pub fn i_0804a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x804A8;
    cpu.ir = 0x6DF0;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x80498; return 10; }
    8
}
/// $0804A8: bra.w $80500
pub fn i_0804a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x804AC;
    cpu.ir = 0x6000;
    cpu.pc = 0x80500;
    10
}
/// $0804AC: move.l d0, -(a7)
pub fn i_0804ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x804AE;
    cpu.ir = 0x2F00;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $0804AE: lea.l $80540(pc), a1
pub fn i_0804ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x804B2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x80540;
    16
}
/// $0804B2: jsr $4ae.l
pub fn i_0804b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x804B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x804B8);
    cpu.pc = 0x4AE;
    16
}
/// $0804B8: jsr $5dd02.l
pub fn i_0804b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x804BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x804BE);
    cpu.pc = 0x5DD02;
    16
}
/// $0804BE: move.l (a7)+, d0
pub fn i_0804be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x804C0;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0804C0: move.w d0, $84(a0)
pub fn i_0804c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x804C4;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x84);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0804C4: bset.b d0, $77(a6)
pub fn i_0804c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x804C8;
    cpu.ir = 0x1EE;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x77);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0804C8: bset.b d0, $78(a6)
pub fn i_0804c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x804CC;
    cpu.ir = 0x1EE;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0804CC: add.w d0, d0
pub fn i_0804cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x804CE;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0804CE: addi.w #$80, d0
pub fn i_0804ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x804D2;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x80, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0804D2: cmpi.w #$ffff, (a6, d0.w)
pub fn i_0804d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x804D8;
    cpu.ir = 0xC76;
    let ad2 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFFF, Size::Word, false);
    18
}
/// $0804D8: bne.w $804e2
pub fn i_0804d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x804DC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x804E2; return 10; }
    8
}
/// $0804DC: move.w $66(a6), (a6, d0.w)
pub fn i_0804dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x804E2;
    cpu.ir = 0x3DAE;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    bus.write16(ad4, v3 as u16);
    22
}
/// $0804E2: move.w $66(a6), $66(a0)
pub fn i_0804e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x804E8;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x66);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0804E8: addq.b #$1, $76(a6)
pub fn i_0804e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x804EC;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $0804EC: move.b $76(a6), d0
pub fn i_0804ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x804F0;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0804F0: cmp.b $99(a6), d0
pub fn i_0804f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x804F4;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $0804F4: bge.w $80500
pub fn i_0804f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x804F8;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x80500; return 10; }
    8
}
/// $0804F8: cmp.b $9a(a6), d0
pub fn i_0804f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x804FC;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $0804FC: blt.w $80506
pub fn i_0804fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80500;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x80506; return 10; }
    8
}
// ===== SetTaskHandler_080500  ($080500, 8 B, kernel/task_handlers.c)
/// $080500: lea.l $80508(pc), a1
pub fn i_080500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80504;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x80508;
    16
}
/// $080504: move.l a1, (a6)
pub fn i_080504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80506;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $080506: rts
pub fn i_080506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80508;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_MgrIdle_080508  ($080508, 48 B, enemies/squad/deploy_module_07fbd2.s)
/// $080508: tst.b $99(a6)
pub fn i_080508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8050C;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $08050C: bne.w $8051a
pub fn i_08050c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80510;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8051A; return 10; }
    8
}
/// $080510: bclr.b #$1, $12(a6)
pub fn i_080510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80516;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $080516: jmp $811d6(pc)
pub fn i_080516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8051A;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x811D6;
    8
}
/// $08051A: tst.b $76(a6)
pub fn i_08051a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8051E;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $08051E: bne.w $8053e
pub fn i_08051e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80522;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8053E; return 10; }
    8
}
/// $080522: addq.w #$1, $70(a6)
pub fn i_080522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80526;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $080526: move.w $70(a6), d0
pub fn i_080526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8052A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08052A: cmp.w $72(a6), d0
pub fn i_08052a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8052E;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $08052E: blt.w $8053e
pub fn i_08052e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80532;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x8053E; return 10; }
    8
}
/// $080532: move.w #$0, $70(a6)
pub fn i_080532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80538;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_080538  ($080538, 8 B, kernel/task_handlers.c)
/// $080538: lea.l $80454(pc), a1
pub fn i_080538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8053C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x80454;
    16
}
/// $08053C: move.l a1, (a6)
pub fn i_08053c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8053E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08053E: rts
pub fn i_08053e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80540;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_ChildSpawnInit_080540  ($080540, 166 B, enemies/squad/deploy_module_07fbd2.s)
/// $080540: move.w #$48, d1
pub fn i_080540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80544;
    cpu.ir = 0x323C;
    let v1: u32 = 0x48;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $080544: jsr $236e.l
pub fn i_080544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8054A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8054A);
    cpu.pc = 0x236E;
    16
}
/// $08054A: move.w #$9, $1c(a6)
pub fn i_08054a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80550;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $080550: jsr $138fe.l
pub fn i_080550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80556;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80556);
    cpu.pc = 0x138FE;
    16
}
/// $080556: bset.b #$1, $12(a6)
pub fn i_080556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8055C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08055C: move.w #$c000, d0
pub fn i_08055c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80560;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $080560: jsr $28134.l
pub fn i_080560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80566;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80566);
    cpu.pc = 0x28134;
    16
}
/// $080566: andi.w #$ffe3, $38(a6)
pub fn i_080566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8056C;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $08056C: ori.w #$14, $38(a6)
pub fn i_08056c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80572;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $080572: move.l #$2e3aa4, $48(a6)
pub fn i_080572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8057A;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2E3AA4;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08057A: clr.b $20(a6)
pub fn i_08057a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8057E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $08057E: jsr $8169a(pc)
pub fn i_08057e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80582;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x80582);
    cpu.pc = 0x8169A;
    16
}
/// $080582: lea.l $802e0(pc), a1
pub fn i_080582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80586;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x802E0;
    16
}
/// $080586: jsr $4ae.l
pub fn i_080586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8058C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8058C);
    cpu.pc = 0x4AE;
    16
}
/// $08058C: move.w $84(a6), $84(a0)
pub fn i_08058c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80592;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x84);
    bus.write16(ad4, v3 as u16);
    20
}
/// $080592: move.w $78(a6), $78(a0)
pub fn i_080592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80598;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x78);
    bus.write16(ad4, v3 as u16);
    20
}
/// $080598: move.w $66(a6), $66(a0)
pub fn i_080598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8059E;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x66);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08059E: lea.l $805a4(pc), a1
pub fn i_08059e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x805A2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x805A4;
    16
}
/// $0805A2: move.l a1, (a6)
pub fn i_0805a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x805A4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0805A4: tst.b $20(a6)
pub fn i_0805a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x805A8;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0805A8: bne.w $805ae
pub fn i_0805a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x805AC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x805AE; return 10; }
    8
}
/// $0805AC: rts
pub fn i_0805ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x805AE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0805AE: lea.l $723da.l, a1
pub fn i_0805ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x805B4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x723DA;
    20
}
/// $0805B4: jsr $4ae.l
pub fn i_0805b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x805BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x805BA);
    cpu.pc = 0x4AE;
    16
}
/// $0805BA: jsr $5dd02.l
pub fn i_0805ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x805C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x805C0);
    cpu.pc = 0x5DD02;
    16
}
/// $0805C0: move.w #$ffe0, $98(a0)
pub fn i_0805c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x805C6;
    cpu.ir = 0x317C;
    let v1: u32 = 0xFFE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0805C6: lea.l $809cc(pc), a1
pub fn i_0805c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x805CA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x809CC;
    16
}
/// $0805CA: jsr $4ae.l
pub fn i_0805ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x805D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x805D0);
    cpu.pc = 0x4AE;
    16
}
/// $0805D0: jsr $5dd02.l
pub fn i_0805d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x805D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x805D6);
    cpu.pc = 0x5DD02;
    16
}
/// $0805D6: lea.l $8067e(pc), a1
pub fn i_0805d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x805DA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8067E;
    16
}
/// $0805DA: jsr $4ae.l
pub fn i_0805da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x805E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x805E0);
    cpu.pc = 0x4AE;
    16
}
/// $0805E0: jsr $5dd02.l
pub fn i_0805e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x805E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x805E6);
    cpu.pc = 0x5DD02;
    16
}
// ===== SetTaskHandler_0805e6  ($0805E6, 8 B, kernel/task_handlers.c)
/// $0805E6: lea.l $805ee(pc), a1
pub fn i_0805e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x805EA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x805EE;
    16
}
/// $0805EA: move.l a1, (a6)
pub fn i_0805ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x805EC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0805EC: rts
pub fn i_0805ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x805EE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_ChildActive_0805ee  ($0805EE, 136 B, enemies/squad/deploy_module_07fbd2.s)
/// $0805EE: move.w $84(a6), d0
pub fn i_0805ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x805F2;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0805F2: add.w d0, d0
pub fn i_0805f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x805F4;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0805F4: addi.w #$80, d0
pub fn i_0805f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x805F8;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x80, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0805F8: movea.l $c(a6), a0
pub fn i_0805f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x805FC;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0805FC: move.w (a0, d0.w), $66(a6)
pub fn i_0805fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80602;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad4, v3 as u16);
    22
}
/// $080602: lea.l $80608(pc), a1
pub fn i_080602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80606;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x80608;
    16
}
/// $080606: move.l a1, (a6)
pub fn i_080606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80608;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $080608: jsr $8179e(pc)
pub fn i_080608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8060C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8060C);
    cpu.pc = 0x8179E;
    16
}
/// $08060C: jsr $78f8a.l
pub fn i_08060c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80612;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80612);
    cpu.pc = 0x78F8A;
    16
}
/// $080612: bcc.w $8061c
pub fn i_080612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80616;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8061C; return 10; }
    8
}
/// $080616: lea.l $81196(pc), a1
pub fn i_080616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8061A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81196;
    16
}
/// $08061A: move.l a1, (a6)
pub fn i_08061a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8061C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08061C: jsr $28d70.l
pub fn i_08061c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80622;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80622);
    cpu.pc = 0x28D70;
    16
}
/// $080622: jsr $2870a.l
pub fn i_080622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80628;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80628);
    cpu.pc = 0x2870A;
    16
}
/// $080628: bcc.w $80646
pub fn i_080628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8062C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x80646; return 10; }
    8
}
/// $08062C: bclr.b #$3, $13(a6)
pub fn i_08062c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80632;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $080632: tst.b $21(a6)
pub fn i_080632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80636;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $080636: bne.w $80646
pub fn i_080636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8063A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x80646; return 10; }
    8
}
/// $08063A: lea.l $2e3566.l, a0
pub fn i_08063a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80640;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E3566;
    20
}
/// $080640: jsr $28cd4.l
pub fn i_080640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80646;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80646);
    cpu.pc = 0x28CD4;
    16
}
/// $080646: lea.l $5e766.l, a0
pub fn i_080646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8064C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $08064C: jsr $5e770.l
pub fn i_08064c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80652;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80652);
    cpu.pc = 0x5E770;
    16
}
/// $080652: jsr $28758.l
pub fn i_080652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80658;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80658);
    cpu.pc = 0x28758;
    16
}
/// $080658: bcc.w $80660
pub fn i_080658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8065C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x80660; return 10; }
    8
}
/// $08065C: jsr $80aaa(pc)
pub fn i_08065c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80660;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x80660);
    cpu.pc = 0x80AAA;
    16
}
/// $080660: movea.l #$ffffffff, a0
pub fn i_080660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80666;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $080666: lea.l $2e3c44.l, a0
pub fn i_080666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8066C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E3C44;
    20
}
/// $08066C: jsr $5dd5c.l
pub fn i_08066c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80672;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80672);
    cpu.pc = 0x5DD5C;
    16
}
/// $080672: bcc.w $8067c
pub fn i_080672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80676;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8067C; return 10; }
    8
}
// ===== SetTaskHandler_080676  ($080676, 8 B, kernel/task_handlers.c)
/// $080676: lea.l $811e4(pc), a1
pub fn i_080676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8067A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x811E4;
    16
}
/// $08067A: move.l a1, (a6)
pub fn i_08067a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8067C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08067C: rts
pub fn i_08067c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8067E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_CmdrInit_08067e  ($08067E, 176 B, enemies/squad/deploy_module_07fbd2.s)
/// $08067E: move.w #$49, d1
pub fn i_08067e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80682;
    cpu.ir = 0x323C;
    let v1: u32 = 0x49;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $080682: jsr $236e.l
pub fn i_080682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80688;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80688);
    cpu.pc = 0x236E;
    16
}
/// $080688: move.w #$170, d1
pub fn i_080688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8068C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x170;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08068C: jsr $236e.l
pub fn i_08068c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80692;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80692);
    cpu.pc = 0x236E;
    16
}
/// $080692: move.w #$171, d1
pub fn i_080692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80696;
    cpu.ir = 0x323C;
    let v1: u32 = 0x171;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $080696: jsr $236e.l
pub fn i_080696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8069C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8069C);
    cpu.pc = 0x236E;
    16
}
/// $08069C: move.w #$6, $1c(a6)
pub fn i_08069c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x806A2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0806A2: jsr $138fe.l
pub fn i_0806a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x806A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x806A8);
    cpu.pc = 0x138FE;
    16
}
/// $0806A8: lea.l $2bef82.l, a0
pub fn i_0806a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x806AE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BEF82;
    20
}
/// $0806AE: jsr $799de.l
pub fn i_0806ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x806B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x806B4);
    cpu.pc = 0x799DE;
    16
}
/// $0806B4: move.w d0, $36(a6)
pub fn i_0806b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x806B8;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0806B8: lea.l $2e3916.l, a0
pub fn i_0806b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x806BE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E3916;
    20
}
/// $0806BE: jsr $28cd4.l
pub fn i_0806be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x806C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x806C4);
    cpu.pc = 0x28CD4;
    16
}
/// $0806C4: lea.l $806ca(pc), a1
pub fn i_0806c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x806C8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x806CA;
    16
}
/// $0806C8: move.l a1, (a6)
pub fn i_0806c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x806CA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0806CA: jsr $5e506.l
pub fn i_0806ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x806D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x806D0);
    cpu.pc = 0x5E506;
    16
}
/// $0806D0: jsr $808e8(pc)
pub fn i_0806d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x806D4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x806D4);
    cpu.pc = 0x808E8;
    16
}
/// $0806D4: bcs.w $806e8
pub fn i_0806d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x806D8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x806E8; return 10; }
    8
}
/// $0806D8: jsr $808a0(pc)
pub fn i_0806d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x806DC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x806DC);
    cpu.pc = 0x808A0;
    16
}
/// $0806DC: jsr $5e826.l
pub fn i_0806dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x806E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x806E2);
    cpu.pc = 0x5E826;
    16
}
/// $0806E2: jsr $28d70.l
pub fn i_0806e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x806E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x806E8);
    cpu.pc = 0x28D70;
    16
}
/// $0806E8: jsr $80932(pc)
pub fn i_0806e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x806EC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x806EC);
    cpu.pc = 0x80932;
    16
}
/// $0806EC: bcc.w $806f6
pub fn i_0806ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x806F0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x806F6; return 10; }
    8
}
/// $0806F0: lea.l $80736(pc), a1
pub fn i_0806f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x806F4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x80736;
    16
}
/// $0806F4: move.l a1, (a6)
pub fn i_0806f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x806F6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0806F6: jsr $8094a(pc)
pub fn i_0806f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x806FA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x806FA);
    cpu.pc = 0x8094A;
    16
}
/// $0806FA: bcc.w $80704
pub fn i_0806fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x806FE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x80704; return 10; }
    8
}
/// $0806FE: lea.l $80736(pc), a1
pub fn i_0806fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80702;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x80736;
    16
}
/// $080702: move.l a1, (a6)
pub fn i_080702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80704;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $080704: movea.l $c(a6), a0
pub fn i_080704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80708;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $080708: cmpi.b #$ff, $20(a0)
pub fn i_080708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8070E;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $08070E: bne.w $80718
pub fn i_08070e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80712;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x80718; return 10; }
    8
}
/// $080712: lea.l $81214(pc), a1
pub fn i_080712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80716;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81214;
    16
}
/// $080716: move.l a1, (a6)
pub fn i_080716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80718;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $080718: movea.l #$ffffffff, a0
pub fn i_080718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8071E;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $08071E: lea.l $2e3c44.l, a0
pub fn i_08071e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80724;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E3C44;
    20
}
/// $080724: jsr $5dd5c.l
pub fn i_080724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8072A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8072A);
    cpu.pc = 0x5DD5C;
    16
}
/// $08072A: bcc.w $80734
pub fn i_08072a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8072E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x80734; return 10; }
    8
}
// ===== SetTaskHandler_08072e  ($08072E, 8 B, kernel/task_handlers.c)
/// $08072E: lea.l $81214(pc), a1
pub fn i_08072e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80732;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81214;
    16
}
/// $080732: move.l a1, (a6)
pub fn i_080732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80734;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $080734: rts
pub fn i_080734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80736;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_080736  ($080736, 354 B, enemies/squad/death_handlers_080736.s)
/// $080736: move.b #$1, $86(a6)
pub fn i_080736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8073C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08073C: movea.l $c(a6), a0
pub fn i_08073c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80740;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $080740: move.w $36(a0), d0
pub fn i_080740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80744;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $080744: asr.w #$1, d0
pub fn i_080744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80746;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $080746: cmpi.b #$40, $34(a0)
pub fn i_080746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8074C;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x34);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x40, Size::Byte, false);
    16
}
/// $08074C: bls.w $80760
pub fn i_08074c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80750;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x80760; return 10; }
    8
}
/// $080750: cmpi.b #$c0, $34(a0)
pub fn i_080750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80756;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x34);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xC0, Size::Byte, false);
    16
}
/// $080756: bhi.w $80760
pub fn i_080756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8075A;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x80760; return 10; }
    8
}
/// $08075A: clr.b $86(a6)
pub fn i_08075a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8075E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $08075E: neg.w d0
pub fn i_08075e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80760;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $080760: move.w d0, $28(a6)
pub fn i_080760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80764;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $080764: bset.b #$4, $6b(a6)
pub fn i_080764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8076A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08076A: movea.l $c(a6), a0
pub fn i_08076a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8076E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $08076E: clr.b $7f(a0)
pub fn i_08076e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80772;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x7F);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $080772: lea.l $2e3922.l, a0
pub fn i_080772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80778;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E3922;
    20
}
/// $080778: jsr $28cd4.l
pub fn i_080778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8077E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8077E);
    cpu.pc = 0x28CD4;
    16
}
/// $08077E: lea.l $80784(pc), a1
pub fn i_08077e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80782;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x80784;
    16
}
/// $080782: move.l a1, (a6)
pub fn i_080782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80784;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $080784: jsr $5e506.l
pub fn i_080784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8078A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8078A);
    cpu.pc = 0x5E506;
    16
}
/// $08078A: jsr $808e8(pc)
pub fn i_08078a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8078E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8078E);
    cpu.pc = 0x808E8;
    16
}
/// $08078E: jsr $808a0(pc)
pub fn i_08078e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80792;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x80792);
    cpu.pc = 0x808A0;
    16
}
/// $080792: jsr $5e826.l
pub fn i_080792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80798;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80798);
    cpu.pc = 0x5E826;
    16
}
/// $080798: jsr $28d70.l
pub fn i_080798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8079E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8079E);
    cpu.pc = 0x28D70;
    16
}
/// $08079E: bcc.w $807a8
pub fn i_08079e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x807A2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x807A8; return 10; }
    8
}
/// $0807A2: lea.l $807ac(pc), a1
pub fn i_0807a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x807A6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x807AC;
    16
}
/// $0807A6: move.l a1, (a6)
pub fn i_0807a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x807A8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0807A8: bra.w $80704
pub fn i_0807a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x807AC;
    cpu.ir = 0x6000;
    cpu.pc = 0x80704;
    10
}
// ===== SetTaskHandler_080898  ($080898, 8 B, kernel/task_handlers.c)
/// $080898: lea.l $81214(pc), a1
pub fn i_080898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8089C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81214;
    16
}
/// $08089C: move.l a1, (a6)
pub fn i_08089c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8089E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08089E: rts
pub fn i_08089e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x808A0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_000808A0  ($0808A0, 134 B, enemies/squad/death_handlers_080736.s)
/// $0808A0: movea.l $c(a6), a0
pub fn i_0808a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x808A4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0808A4: move.b $7e(a0), d0
pub fn i_0808a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x808A8;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x7E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0808A8: andi.l #$7, d0
pub fn i_0808a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x808AE;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0x7) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $0808AE: lsl.l #$3, d0
pub fn i_0808ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x808B0;
    cpu.ir = 0xE788;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Long, 1, true);
    cpu.d[0] = r;
    12
}
/// $0808B0: lea.l $2e3d80.l, a1
pub fn i_0808b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x808B6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E3D80;
    20
}
/// $0808B6: adda.l d0, a1
pub fn i_0808b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x808B8;
    cpu.ir = 0xD3C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $0808B8: move.w $4(a1), d0
pub fn i_0808b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x808BC;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0808BC: add.w d0, $38(a6)
pub fn i_0808bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x808C0;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0808C0: move.w $2(a1), d0
pub fn i_0808c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x808C4;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0808C4: btst.b #$1, $3a(a6)
pub fn i_0808c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x808CA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0808CA: beq.w $808d0
pub fn i_0808ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x808CE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x808D0; return 10; }
    8
}
/// $0808CE: neg.w d0
pub fn i_0808ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x808D0;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0808D0: add.w d0, $24(a6)
pub fn i_0808d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x808D4;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0808D4: move.w (a1), d0
pub fn i_0808d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x808D6;
    cpu.ir = 0x3011;
    let v2 = bus.read16(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $0808D6: btst.b #$0, $3a(a6)
pub fn i_0808d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x808DC;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0808DC: beq.w $808e2
pub fn i_0808dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x808E0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x808E2; return 10; }
    8
}
/// $0808E0: neg.w d0
pub fn i_0808e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x808E2;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0808E2: add.w d0, $22(a6)
pub fn i_0808e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x808E6;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0808E6: rts
pub fn i_0808e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x808E8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0808E8: movea.l $c(a6), a0
pub fn i_0808e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x808EC;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0808EC: tst.b $82(a0)
pub fn i_0808ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x808F0;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x82);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0808F0: beq.w $80926
pub fn i_0808f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x808F4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x80926; return 10; }
    8
}
/// $0808F4: clr.b $7e(a0)
pub fn i_0808f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x808F8;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x7E);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0808F8: move.w $76(a0), d0
pub fn i_0808f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x808FC;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0808FC: beq.w $80916
pub fn i_0808fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80900;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x80916; return 10; }
    8
}
/// $080900: cmpi.w #$8, d0
pub fn i_080900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80904;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x8, Size::Word, false);
    8
}
/// $080904: bne.w $8092c
pub fn i_080904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80908;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8092C; return 10; }
    8
}
/// $080908: cmpi.b #$1, $82(a0)
pub fn i_080908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8090E;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x82);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $08090E: beq.w $80926
pub fn i_08090e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80912;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x80926; return 10; }
    8
}
/// $080912: bra.w $80920
pub fn i_080912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80916;
    cpu.ir = 0x6000;
    cpu.pc = 0x80920;
    10
}
/// $080916: cmpi.b #$2, $82(a0)
pub fn i_080916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8091C;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x82);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $08091C: beq.w $80926
pub fn i_08091c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80920;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x80926; return 10; }
    8
}
/// $080920: eori.b #$3, $3a(a6)
pub fn i_080920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80926;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x3) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== ClearXN_080926  ($080926, 6 B, thunks/ccr_helpers.c)
/// $080926: andi.b #$ee, ccr
pub fn i_080926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8092A;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $08092A: rts
pub fn i_08092a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8092C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_08092c  ($08092C, 6 B, thunks/ccr_helpers.c)
/// $08092C: ori.b #$11, ccr
pub fn i_08092c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80930;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $080930: rts
pub fn i_080930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80932;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_00080932  ($080932, 12 B, enemies/squad/death_handlers_080736.s)
/// $080932: movea.l $c(a6), a0
pub fn i_080932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80936;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $080936: tst.b $7f(a0)
pub fn i_080936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8093A;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x7F);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $08093A: beq.w $80944
pub fn i_08093a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8093E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x80944; return 10; }
    8
}
// ===== ClearXN_08093e  ($08093E, 6 B, thunks/ccr_helpers.c)
/// $08093E: andi.b #$ee, ccr
pub fn i_08093e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80942;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $080942: rts
pub fn i_080942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80944;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_080944  ($080944, 6 B, thunks/ccr_helpers.c)
/// $080944: ori.b #$11, ccr
pub fn i_080944(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80948;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $080948: rts
pub fn i_080948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8094A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_0008094A  ($08094A, 14 B, enemies/squad/death_handlers_080736.s)
/// $08094A: movea.l $c(a6), a0
pub fn i_08094a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8094E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $08094E: cmpi.b #$1, $7f(a0)
pub fn i_08094e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80954;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x7F);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $080954: beq.w $8095e
pub fn i_080954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80958;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8095E; return 10; }
    8
}
// ===== ClearXN_080958  ($080958, 6 B, thunks/ccr_helpers.c)
/// $080958: andi.b #$ee, ccr
pub fn i_080958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8095C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $08095C: rts
pub fn i_08095c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8095E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_ProbeLeaderNear_08095e  ($08095E, 30 B, enemies/squad/death_handlers_080736.s)
/// $08095E: movea.l $7a(a0), a1
pub fn i_08095e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80962;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0x7A);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $080962: cmpi.w #$3, $78(a0)
pub fn i_080962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80968;
    cpu.ir = 0xC68;
    let ad2 = cpu.a[0].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Word, false);
    16
}
/// $080968: bge.w $80982
pub fn i_080968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8096C;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x80982; return 10; }
    8
}
/// $08096C: move.w $22(a6), d0
pub fn i_08096c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80970;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $080970: sub.w $22(a1), d0
pub fn i_080970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80974;
    cpu.ir = 0x9069;
    let ad2 = cpu.a[1].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $080974: cmpi.w #$60, d0
pub fn i_080974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80978;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x60, Size::Word, false);
    8
}
/// $080978: ble.w $80990
pub fn i_080978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8097C;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x80990; return 10; }
    8
}
// ===== ClearXN_08097c  ($08097C, 6 B, thunks/ccr_helpers.c)
/// $08097C: andi.b #$ee, ccr
pub fn i_08097c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80980;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $080980: rts
pub fn i_080980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80982;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_ProbeLeaderFar_080982  ($080982, 14 B, enemies/squad/death_handlers_080736.s)
/// $080982: move.w $22(a1), d0
pub fn i_080982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80986;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $080986: sub.w $22(a6), d0
pub fn i_080986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8098A;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $08098A: cmpi.w #$60, d0
pub fn i_08098a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8098E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x60, Size::Word, false);
    8
}
/// $08098E: bgt.b $8097c
pub fn i_08098e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80990;
    cpu.ir = 0x6EEC;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8097C; return 10; }
    8
}
// ===== SetXN_080990  ($080990, 6 B, thunks/ccr_helpers.c)
/// $080990: ori.b #$11, ccr
pub fn i_080990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80994;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $080994: rts
pub fn i_080994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80996;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_080996  ($080996, 46 B, enemies/squad/death_handlers_080736.s)
/// $080996: move.w #$48, d1
pub fn i_080996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8099A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x48;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08099A: jsr $236e.l
pub fn i_08099a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x809A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x809A0);
    cpu.pc = 0x236E;
    16
}
/// $0809A0: lea.l $2e35b8.l, a0
pub fn i_0809a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x809A6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E35B8;
    20
}
/// $0809A6: jsr $28cd4.l
pub fn i_0809a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x809AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x809AC);
    cpu.pc = 0x28CD4;
    16
}
/// $0809AC: lea.l $809b2(pc), a1
pub fn i_0809ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x809B0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x809B2;
    16
}
/// $0809B0: move.l a1, (a6)
pub fn i_0809b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x809B2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0809B2: jsr $5e506.l
pub fn i_0809b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x809B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x809B8);
    cpu.pc = 0x5E506;
    16
}
/// $0809B8: move.b $32(a0), $32(a6)
pub fn i_0809b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x809BE;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x32);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad4, v3 as u8);
    20
}
/// $0809BE: move.b $33(a0), $33(a6)
pub fn i_0809be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x809C4;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x33);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad4, v3 as u8);
    20
}
// ===== JsrAbsThunk_0809c4  ($0809C4, 8 B, thunks/jsr_abs_thunks.c)
/// $0809C4: jsr $28d70.l
pub fn i_0809c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x809CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x809CA);
    cpu.pc = 0x28D70;
    16
}
/// $0809CA: rts
pub fn i_0809ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x809CC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_0809cc  ($0809CC, 108 B, enemies/squad/death_handlers_080736.s)
/// $0809CC: move.w #$48, d1
pub fn i_0809cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x809D0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x48;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0809D0: jsr $236e.l
pub fn i_0809d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x809D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x809D6);
    cpu.pc = 0x236E;
    16
}
/// $0809D6: move.w #$9, $1c(a6)
pub fn i_0809d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x809DC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0809DC: jsr $138fe.l
pub fn i_0809dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x809E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x809E2);
    cpu.pc = 0x138FE;
    16
}
/// $0809E2: lea.l $2e35b8.l, a0
pub fn i_0809e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x809E8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E35B8;
    20
}
/// $0809E8: jsr $28cd4.l
pub fn i_0809e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x809EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x809EE);
    cpu.pc = 0x28CD4;
    16
}
/// $0809EE: lea.l $809f4(pc), a1
pub fn i_0809ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x809F2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x809F4;
    16
}
/// $0809F2: move.l a1, (a6)
pub fn i_0809f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x809F4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0809F4: jsr $5e506.l
pub fn i_0809f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x809FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x809FA);
    cpu.pc = 0x5E506;
    16
}
/// $0809FA: movea.l $c(a6), a0
pub fn i_0809fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x809FE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0809FE: move.w $76(a0), $76(a6)
pub fn i_0809fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80A04;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad4, v3 as u16);
    20
}
/// $080A04: move.b $32(a0), $32(a6)
pub fn i_080a04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80A0A;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x32);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad4, v3 as u8);
    20
}
/// $080A0A: move.b $33(a0), $33(a6)
pub fn i_080a0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80A10;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x33);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad4, v3 as u8);
    20
}
/// $080A10: cmpi.b #$1, $21(a0)
pub fn i_080a10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80A16;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $080A16: beq.w $80a40
pub fn i_080a16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80A1A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x80A40; return 10; }
    8
}
/// $080A1A: cmpi.b #$ff, $20(a0)
pub fn i_080a1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80A20;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $080A20: beq.w $81214
pub fn i_080a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80A24;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x81214; return 10; }
    8
}
/// $080A24: movea.l $c(a6), a0
pub fn i_080a24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80A28;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $080A28: btst.b #$0, $5a(a0)
pub fn i_080a28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80A2E;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $080A2E: beq.w $80a38
pub fn i_080a2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80A32;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x80A38; return 10; }
    8
}
/// $080A32: bset.b #$0, $5a(a6)
pub fn i_080a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80A38;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== JsrAbsThunk_080a38  ($080A38, 8 B, thunks/jsr_abs_thunks.c)
/// $080A38: jsr $28d70.l
pub fn i_080a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80A3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80A3E);
    cpu.pc = 0x28D70;
    16
}
/// $080A3E: rts
pub fn i_080a3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80A40;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_ChildInitVar2_080a40  ($080A40, 98 B, enemies/squad/death_handlers_080736.s)
/// $080A40: move.w #$48, d1
pub fn i_080a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80A44;
    cpu.ir = 0x323C;
    let v1: u32 = 0x48;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $080A44: jsr $236e.l
pub fn i_080a44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80A4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80A4A);
    cpu.pc = 0x236E;
    16
}
/// $080A4A: lea.l $2e36b4.l, a0
pub fn i_080a4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80A50;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E36B4;
    20
}
/// $080A50: jsr $28cd4.l
pub fn i_080a50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80A56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80A56);
    cpu.pc = 0x28CD4;
    16
}
/// $080A56: lea.l $80a5c(pc), a1
pub fn i_080a56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80A5A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x80A5C;
    16
}
/// $080A5A: move.l a1, (a6)
pub fn i_080a5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80A5C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $080A5C: jsr $5e506.l
pub fn i_080a5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80A62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80A62);
    cpu.pc = 0x5E506;
    16
}
/// $080A62: clr.b $3a(a6)
pub fn i_080a62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80A66;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $080A66: movea.l $c(a6), a0
pub fn i_080a66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80A6A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $080A6A: move.w $76(a0), $76(a6)
pub fn i_080a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80A70;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad4, v3 as u16);
    20
}
/// $080A70: move.b $32(a0), $32(a6)
pub fn i_080a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80A76;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x32);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad4, v3 as u8);
    20
}
/// $080A76: move.b $33(a0), $33(a6)
pub fn i_080a76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80A7C;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x33);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad4, v3 as u8);
    20
}
/// $080A7C: tst.b $21(a0)
pub fn i_080a7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80A80;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $080A80: beq.w $809cc
pub fn i_080a80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80A84;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x809CC; return 10; }
    8
}
/// $080A84: cmpi.b #$ff, $20(a0)
pub fn i_080a84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80A8A;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $080A8A: beq.w $81214
pub fn i_080a8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80A8E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x81214; return 10; }
    8
}
/// $080A8E: movea.l $c(a6), a0
pub fn i_080a8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80A92;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $080A92: btst.b #$0, $5a(a0)
pub fn i_080a92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80A98;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $080A98: beq.w $80aa2
pub fn i_080a98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80A9C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x80AA2; return 10; }
    8
}
/// $080A9C: bset.b #$0, $5a(a6)
pub fn i_080a9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80AA2;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== JsrAbsThunk_080aa2  ($080AA2, 8 B, thunks/jsr_abs_thunks.c)
/// $080AA2: jsr $28d70.l
pub fn i_080aa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80AA8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80AA8);
    cpu.pc = 0x28D70;
    16
}
/// $080AA8: rts
pub fn i_080aa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80AAA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_00080AAA  ($080AAA, 292 B, enemies/squad/death_handlers_080736.s)
/// $080AAA: lea.l $ffff.w, a0
pub fn i_080aaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80AAE;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $080AAE: move.l a0, $48(a6)
pub fn i_080aae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80AB2;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $080AB2: bclr.b #$1, $12(a6)
pub fn i_080ab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80AB8;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $080AB8: cmpi.b #$1, $7f(a6)
pub fn i_080ab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80ABE;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x7F);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $080ABE: bne.w $80ac8
pub fn i_080abe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80AC2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x80AC8; return 10; }
    8
}
/// $080AC2: move.b #$ff, $7f(a6)
pub fn i_080ac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80AC8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7F);
    bus.write8(ad2, v1 as u8);
    12
}
/// $080AC8: movea.l $c(a6), a0
pub fn i_080ac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80ACC;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $080ACC: subq.b #$1, $99(a0)
pub fn i_080acc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80AD0;
    cpu.ir = 0x5328;
    let ad1 = cpu.a[0].wrapping_add(0x99);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $080AD0: move.w $84(a6), d0
pub fn i_080ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80AD4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $080AD4: bclr.b d0, $77(a0)
pub fn i_080ad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80AD8;
    cpu.ir = 0x1A8;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x77);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $080AD8: bclr.b d0, $78(a0)
pub fn i_080ad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80ADC;
    cpu.ir = 0x1A8;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x78);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $080ADC: add.w d0, d0
pub fn i_080adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80ADE;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $080ADE: addi.w #$80, d0
pub fn i_080ade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80AE2;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x80, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $080AE2: move.w #$ffff, (a0, d0.w)
pub fn i_080ae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80AE8;
    cpu.ir = 0x31BC;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $080AE8: subq.b #$1, $76(a0)
pub fn i_080ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80AEC;
    cpu.ir = 0x5328;
    let ad1 = cpu.a[0].wrapping_add(0x76);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $080AEC: clr.w d0
pub fn i_080aec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80AEE;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $080AEE: move.b $20(a6), d0
pub fn i_080aee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80AF2;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $080AF2: lea.l $2e3dc0.l, a0
pub fn i_080af2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80AF8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E3DC0;
    20
}
/// $080AF8: add.w d0, d0
pub fn i_080af8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80AFA;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $080AFA: add.w d0, d0
pub fn i_080afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80AFC;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $080AFC: movea.l (a0, d0.w), a0
pub fn i_080afc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80B00;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $080B00: move.l a0, -(a7)
pub fn i_080b00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80B02;
    cpu.ir = 0x2F08;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $080B02: jsr $5e9b6.l
pub fn i_080b02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80B08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80B08);
    cpu.pc = 0x5E9B6;
    16
}
/// $080B08: andi.w #$7, d0
pub fn i_080b08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80B0C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $080B0C: add.w d0, d0
pub fn i_080b0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80B0E;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $080B0E: add.w d0, d0
pub fn i_080b0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80B10;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $080B10: movea.l (a7)+, a0
pub fn i_080b10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80B12;
    cpu.ir = 0x205F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    12
}
/// $080B12: movea.l (a0, d0.w), a0
pub fn i_080b12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80B16;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $080B16: move.l a0, (a6)
pub fn i_080b16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80B18;
    cpu.ir = 0x2C88;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $080B18: rts
pub fn i_080b18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80B1A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetTaskHandler_080bce  ($080BCE, 8 B, kernel/task_handlers.c)
/// $080BCE: lea.l $811cc(pc), a1
pub fn i_080bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80BD2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x811CC;
    16
}
/// $080BD2: move.l a1, (a6)
pub fn i_080bd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80BD4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $080BD4: rts
pub fn i_080bd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80BD6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_DeathTumble_080bd6  ($080BD6, 254 B, enemies/squad/death_handlers_080736.s)
/// $080BD6: move.l #$2e2d42, $90(a6)
pub fn i_080bd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80BDE;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2E2D42;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $080BDE: btst.b #$0, $3a(a6)
pub fn i_080bde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80BE4;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $080BE4: beq.w $80bf0
pub fn i_080be4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80BE8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x80BF0; return 10; }
    8
}
/// $080BE8: move.l #$2e2d78, $90(a6)
pub fn i_080be8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80BF0;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2E2D78;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $080BF0: moveq #$0, d0
pub fn i_080bf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80BF2;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $080BF2: move.b d0, $20(a6)
pub fn i_080bf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80BF6;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $080BF6: move.w d0, $94(a6)
pub fn i_080bf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80BFA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    bus.write16(ad2, v1 as u16);
    12
}
/// $080BFA: move.b d0, $96(a6)
pub fn i_080bfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80BFE;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x96);
    bus.write8(ad2, v1 as u8);
    12
}
/// $080BFE: lea.l $80c68(pc), a1
pub fn i_080bfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80C02;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x80C68;
    16
}
/// $080C02: move.l a1, (a6)
pub fn i_080c02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80C04;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $080C04: bra.w $80c68
pub fn i_080c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80C08;
    cpu.ir = 0x6000;
    cpu.pc = 0x80C68;
    10
}
/// $080C68: jsr $8179e(pc)
pub fn i_080c68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80C6C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x80C6C);
    cpu.pc = 0x8179E;
    16
}
/// $080C6C: jsr $78f8a.l
pub fn i_080c6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80C72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80C72);
    cpu.pc = 0x78F8A;
    16
}
/// $080C72: bcc.w $80c7c
pub fn i_080c72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80C76;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x80C7C; return 10; }
    8
}
/// $080C76: lea.l $811cc(pc), a1
pub fn i_080c76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80C7A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x811CC;
    16
}
/// $080C7A: move.l a1, (a6)
pub fn i_080c7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80C7C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $080C7C: jsr $28d70.l
pub fn i_080c7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80C82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80C82);
    cpu.pc = 0x28D70;
    16
}
/// $080C82: move.w $22(a6), d1
pub fn i_080c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80C86;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $080C86: move.w $24(a6), d2
pub fn i_080c86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80C8A;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $080C8A: jsr $280c6.l
pub fn i_080c8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80C90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80C90);
    cpu.pc = 0x280C6;
    16
}
/// $080C90: bcc.w $80ca2
pub fn i_080c90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80C94;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x80CA2; return 10; }
    8
}
/// $080C94: andi.b #$f0, d0
pub fn i_080c94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80C98;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $080C98: bne.w $80ca2
pub fn i_080c98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80C9C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x80CA2; return 10; }
    8
}
/// $080C9C: lea.l $80f32(pc), a1
pub fn i_080c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80CA0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x80F32;
    16
}
/// $080CA0: move.l a1, (a6)
pub fn i_080ca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80CA2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $080CA2: jsr $80efa(pc)
pub fn i_080ca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80CA6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x80CA6);
    cpu.pc = 0x80EFA;
    16
}
/// $080CA6: bcc.w $80cb0
pub fn i_080ca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80CAA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x80CB0; return 10; }
    8
}
/// $080CAA: lea.l $81018(pc), a1
pub fn i_080caa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80CAE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81018;
    16
}
/// $080CAE: move.l a1, (a6)
pub fn i_080cae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80CB0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $080CB0: jsr $80f1c(pc)
pub fn i_080cb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80CB4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x80CB4);
    cpu.pc = 0x80F1C;
    16
}
/// $080CB4: bcc.w $80cbe
pub fn i_080cb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80CB8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x80CBE; return 10; }
    8
}
/// $080CB8: lea.l $80f3c(pc), a1
pub fn i_080cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80CBC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x80F3C;
    16
}
/// $080CBC: move.l a1, (a6)
pub fn i_080cbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80CBE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $080CBE: movea.l #$ffffffff, a0
pub fn i_080cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80CC4;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $080CC4: lea.l $2e3c44.l, a0
pub fn i_080cc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80CCA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E3C44;
    20
}
/// $080CCA: jsr $5dd5c.l
pub fn i_080cca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80CD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80CD0);
    cpu.pc = 0x5DD5C;
    16
}
/// $080CD0: bcc.w $80cda
pub fn i_080cd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80CD4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x80CDA; return 10; }
    8
}
// ===== SetTaskHandler_080cd4  ($080CD4, 8 B, kernel/task_handlers.c)
/// $080CD4: lea.l $811cc(pc), a1
pub fn i_080cd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80CD8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x811CC;
    16
}
/// $080CD8: move.l a1, (a6)
pub fn i_080cd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80CDA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $080CDA: rts
pub fn i_080cda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80CDC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_DeathSkid_080cdc  ($080CDC, 174 B, enemies/squad/death_handlers_080736.s)
/// $080CDC: move.l #$2e2dae, $90(a6)
pub fn i_080cdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80CE4;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2E2DAE;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $080CE4: btst.b #$0, $3a(a6)
pub fn i_080ce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80CEA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $080CEA: beq.w $80cf6
pub fn i_080cea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80CEE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x80CF6; return 10; }
    8
}
/// $080CEE: move.l #$2e2dd8, $90(a6)
pub fn i_080cee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80CF6;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2E2DD8;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $080CF6: moveq #$0, d0
pub fn i_080cf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80CF8;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $080CF8: move.b d0, $20(a6)
pub fn i_080cf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80CFC;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $080CFC: move.w d0, $94(a6)
pub fn i_080cfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80D00;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    bus.write16(ad2, v1 as u16);
    12
}
/// $080D00: move.b d0, $96(a6)
pub fn i_080d00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80D04;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x96);
    bus.write8(ad2, v1 as u8);
    12
}
/// $080D04: asl.w $72(a6)
pub fn i_080d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80D08;
    cpu.ir = 0xE1EE;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = do_shift_rotate(cpu, v2, 1, Size::Word, 0, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $080D08: lea.l $80d0e(pc), a1
pub fn i_080d08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80D0C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x80D0E;
    16
}
/// $080D0C: move.l a1, (a6)
pub fn i_080d0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80D0E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $080D0E: move.w $72(a6), d0
pub fn i_080d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80D12;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $080D12: sub.w $2e(a6), d0
pub fn i_080d12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80D16;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $080D16: bpl.w $80d28
pub fn i_080d16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80D1A;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x80D28; return 10; }
    8
}
/// $080D1A: clr.w d0
pub fn i_080d1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80D1C;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $080D1C: move.w #$ffe0, $2e(a6)
pub fn i_080d1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80D22;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $080D22: lea.l $80d92(pc), a1
pub fn i_080d22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80D26;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x80D92;
    16
}
/// $080D26: move.l a1, (a6)
pub fn i_080d26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80D28;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $080D28: move.w d0, $72(a6)
pub fn i_080d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80D2C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $080D2C: jsr $8179e(pc)
pub fn i_080d2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80D30;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x80D30);
    cpu.pc = 0x8179E;
    16
}
/// $080D30: jsr $78f8a.l
pub fn i_080d30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80D36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80D36);
    cpu.pc = 0x78F8A;
    16
}
/// $080D36: bcc.w $80d40
pub fn i_080d36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80D3A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x80D40; return 10; }
    8
}
/// $080D3A: lea.l $811cc(pc), a1
pub fn i_080d3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80D3E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x811CC;
    16
}
/// $080D3E: move.l a1, (a6)
pub fn i_080d3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80D40;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $080D40: jsr $28d70.l
pub fn i_080d40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80D46;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80D46);
    cpu.pc = 0x28D70;
    16
}
/// $080D46: move.w $22(a6), d1
pub fn i_080d46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80D4A;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $080D4A: move.w $24(a6), d2
pub fn i_080d4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80D4E;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $080D4E: jsr $280c6.l
pub fn i_080d4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80D54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80D54);
    cpu.pc = 0x280C6;
    16
}
/// $080D54: bcc.w $80d66
pub fn i_080d54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80D58;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x80D66; return 10; }
    8
}
/// $080D58: andi.b #$f0, d0
pub fn i_080d58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80D5C;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $080D5C: bne.w $80d66
pub fn i_080d5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80D60;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x80D66; return 10; }
    8
}
/// $080D60: lea.l $80f32(pc), a1
pub fn i_080d60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80D64;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x80F32;
    16
}
/// $080D64: move.l a1, (a6)
pub fn i_080d64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80D66;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $080D66: jsr $80f1c(pc)
pub fn i_080d66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80D6A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x80D6A);
    cpu.pc = 0x80F1C;
    16
}
/// $080D6A: bcc.w $80d74
pub fn i_080d6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80D6E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x80D74; return 10; }
    8
}
/// $080D6E: lea.l $80f3c(pc), a1
pub fn i_080d6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80D72;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x80F3C;
    16
}
/// $080D72: move.l a1, (a6)
pub fn i_080d72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80D74;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $080D74: movea.l #$ffffffff, a0
pub fn i_080d74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80D7A;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $080D7A: lea.l $2e3c44.l, a0
pub fn i_080d7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80D80;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E3C44;
    20
}
/// $080D80: jsr $5dd5c.l
pub fn i_080d80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80D86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80D86);
    cpu.pc = 0x5DD5C;
    16
}
/// $080D86: bcc.w $80d90
pub fn i_080d86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80D8A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x80D90; return 10; }
    8
}
// ===== SetTaskHandler_080d8a  ($080D8A, 8 B, kernel/task_handlers.c)
/// $080D8A: lea.l $811cc(pc), a1
pub fn i_080d8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80D8E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x811CC;
    16
}
/// $080D8E: move.l a1, (a6)
pub fn i_080d8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80D90;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $080D90: rts
pub fn i_080d90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80D92;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_DeathSkidBrake_080d92  ($080D92, 174 B, enemies/squad/death_handlers_080736.s)
/// $080D92: jsr $27d50.l
pub fn i_080d92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80D98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80D98);
    cpu.pc = 0x27D50;
    16
}
/// $080D98: bcc.w $80da2
pub fn i_080d98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80D9C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x80DA2; return 10; }
    8
}
/// $080D9C: lea.l $80f32(pc), a1
pub fn i_080d9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80DA0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x80F32;
    16
}
/// $080DA0: move.l a1, (a6)
pub fn i_080da0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80DA2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $080DA2: jmp $80d30(pc)
pub fn i_080da2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80DA6;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x80D30;
    8
}
// ===== SetTaskHandler_080e40  ($080E40, 8 B, kernel/task_handlers.c)
/// $080E40: lea.l $811cc(pc), a1
pub fn i_080e40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80E44;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x811CC;
    16
}
/// $080E44: move.l a1, (a6)
pub fn i_080e44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80E46;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $080E46: rts
pub fn i_080e46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80E48;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_DeathArcJump_080e48  ($080E48, 170 B, enemies/squad/death_handlers_080736.s)
/// $080E48: lea.l $2e30ae.l, a0
pub fn i_080e48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80E4E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E30AE;
    20
}
/// $080E4E: jsr $28cd4.l
pub fn i_080e4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80E54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80E54);
    cpu.pc = 0x28CD4;
    16
}
/// $080E54: btst.b #$0, $3a(a6)
pub fn i_080e54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80E5A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $080E5A: beq.w $80e6a
pub fn i_080e5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80E5E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x80E6A; return 10; }
    8
}
/// $080E5E: lea.l $2e330a.l, a0
pub fn i_080e5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80E64;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E330A;
    20
}
/// $080E64: jsr $28cd4.l
pub fn i_080e64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80E6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80E6A);
    cpu.pc = 0x28CD4;
    16
}
/// $080E6A: moveq #$0, d0
pub fn i_080e6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80E6C;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $080E6C: move.b d0, $20(a6)
pub fn i_080e6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80E70;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $080E70: move.b #$1, $21(a6)
pub fn i_080e70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80E76;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $080E76: move.b $34(a6), d0
pub fn i_080e76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80E7A;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $080E7A: move.w d0, d1
pub fn i_080e7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80E7C;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $080E7C: cmpi.w #$f8, d1
pub fn i_080e7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80E80;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xF8, Size::Word, false);
    8
}
/// $080E80: bcs.w $80e86
pub fn i_080e80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80E84;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x80E86; return 10; }
    8
}
/// $080E84: moveq #$0, d1
pub fn i_080e84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80E86;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $080E86: asr.w #$4, d1
pub fn i_080e86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80E88;
    cpu.ir = 0xE841;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $080E88: move.w d1, $76(a6)
pub fn i_080e88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80E8C;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $080E8C: move.w $36(a6), d1
pub fn i_080e8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80E90;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $080E90: move.w d1, d2
pub fn i_080e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80E92;
    cpu.ir = 0x3401;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $080E92: asr.w #$1, d2
pub fn i_080e92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80E94;
    cpu.ir = 0xE242;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $080E94: add.w d2, d1
pub fn i_080e94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80E96;
    cpu.ir = 0xD242;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $080E96: jsr $13c0e.l
pub fn i_080e96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80E9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80E9C);
    cpu.pc = 0x13C0E;
    16
}
/// $080E9C: move.w d1, $28(a6)
pub fn i_080e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80EA0;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $080EA0: move.w d2, $2a(a6)
pub fn i_080ea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80EA4;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $080EA4: lea.l $80eaa(pc), a1
pub fn i_080ea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80EA8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x80EAA;
    16
}
/// $080EA8: move.l a1, (a6)
pub fn i_080ea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80EAA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $080EAA: jsr $27cee.l
pub fn i_080eaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80EB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80EB0);
    cpu.pc = 0x27CEE;
    16
}
/// $080EB0: bcc.w $80eba
pub fn i_080eb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80EB4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x80EBA; return 10; }
    8
}
/// $080EB4: lea.l $80f32(pc), a1
pub fn i_080eb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80EB8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x80F32;
    16
}
/// $080EB8: move.l a1, (a6)
pub fn i_080eb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80EBA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $080EBA: jsr $28d70.l
pub fn i_080eba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80EC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80EC0);
    cpu.pc = 0x28D70;
    16
}
/// $080EC0: jsr $80f1c(pc)
pub fn i_080ec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80EC4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x80EC4);
    cpu.pc = 0x80F1C;
    16
}
/// $080EC4: bcc.w $80ece
pub fn i_080ec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80EC8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x80ECE; return 10; }
    8
}
/// $080EC8: lea.l $80f3c(pc), a1
pub fn i_080ec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80ECC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x80F3C;
    16
}
/// $080ECC: move.l a1, (a6)
pub fn i_080ecc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80ECE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $080ECE: jsr $80efa(pc)
pub fn i_080ece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80ED2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x80ED2);
    cpu.pc = 0x80EFA;
    16
}
/// $080ED2: bcc.w $80edc
pub fn i_080ed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80ED6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x80EDC; return 10; }
    8
}
/// $080ED6: lea.l $81018(pc), a1
pub fn i_080ed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80EDA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81018;
    16
}
/// $080EDA: move.l a1, (a6)
pub fn i_080eda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80EDC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $080EDC: movea.l #$ffffffff, a0
pub fn i_080edc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80EE2;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $080EE2: lea.l $2e3c44.l, a0
pub fn i_080ee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80EE8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E3C44;
    20
}
/// $080EE8: jsr $5dd5c.l
pub fn i_080ee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80EEE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80EEE);
    cpu.pc = 0x5DD5C;
    16
}
/// $080EEE: bcc.w $80ef8
pub fn i_080eee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80EF2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x80EF8; return 10; }
    8
}
// ===== SetTaskHandler_080ef2  ($080EF2, 8 B, kernel/task_handlers.c)
/// $080EF2: lea.l $811cc(pc), a1
pub fn i_080ef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80EF6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x811CC;
    16
}
/// $080EF6: move.l a1, (a6)
pub fn i_080ef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80EF8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $080EF8: rts
pub fn i_080ef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80EFA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_BoundsCheck_080efa  ($080EFA, 44 B, enemies/squad/death_handlers_080736.s)
/// $080EFA: cmpi.w #$20, $22(a6)
pub fn i_080efa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80F00;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x20, Size::Word, false);
    16
}
/// $080F00: ble.w $80f2c
pub fn i_080f00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80F04;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x80F2C; return 10; }
    8
}
/// $080F04: cmpi.w #$120, $22(a6)
pub fn i_080f04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80F0A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x120, Size::Word, false);
    16
}
/// $080F0A: bge.w $80f2c
pub fn i_080f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80F0E;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x80F2C; return 10; }
    8
}
/// $080F0E: cmpi.w #$200, $24(a6)
pub fn i_080f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80F14;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x200, Size::Word, false);
    16
}
/// $080F14: bge.w $80f2c
pub fn i_080f14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80F18;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x80F2C; return 10; }
    8
}
/// $080F18: bra.w $80f26
pub fn i_080f18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80F1C;
    cpu.ir = 0x6000;
    cpu.pc = 0x80F26;
    10
}
/// $080F1C: cmpi.w #$110, $24(a6)
pub fn i_080f1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80F22;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x110, Size::Word, false);
    16
}
/// $080F22: ble.w $80f2c
pub fn i_080f22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80F26;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x80F2C; return 10; }
    8
}
// ===== ClearC_080f26  ($080F26, 6 B, thunks/ccr_helpers.c)
/// $080F26: andi.b #$fe, ccr
pub fn i_080f26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80F2A;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $080F2A: rts
pub fn i_080f2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80F2C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetC_080f2c  ($080F2C, 6 B, thunks/ccr_helpers.c)
/// $080F2C: ori.b #$1, ccr
pub fn i_080f2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80F30;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $080F30: rts
pub fn i_080f30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80F32;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_DeathPieces_080f32  ($080F32, 652 B, enemies/squad/death_handlers_080736.s)
/// $080F32: jsr $434dc.l
pub fn i_080f32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80F38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80F38);
    cpu.pc = 0x434DC;
    16
}
/// $080F38: bra.w $80f68
pub fn i_080f38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80F3C;
    cpu.ir = 0x6000;
    cpu.pc = 0x80F68;
    10
}
/// $080F68: lea.l $77fd6.l, a1
pub fn i_080f68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80F6E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $080F6E: jsr $4ae.l
pub fn i_080f6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80F74;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80F74);
    cpu.pc = 0x4AE;
    16
}
/// $080F74: jsr $5dd02.l
pub fn i_080f74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80F7A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80F7A);
    cpu.pc = 0x5DD02;
    16
}
/// $080F7A: move.b $32(a6), $32(a0)
pub fn i_080f7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80F80;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x32);
    bus.write8(ad4, v3 as u8);
    20
}
/// $080F80: move.b $33(a6), $33(a0)
pub fn i_080f80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80F86;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x33);
    bus.write8(ad4, v3 as u8);
    20
}
/// $080F86: lea.l $2e3c54.l, a1
pub fn i_080f86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80F8C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E3C54;
    20
}
/// $080F8C: jsr $77c7e.l
pub fn i_080f8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80F92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80F92);
    cpu.pc = 0x77C7E;
    16
}
/// $080F92: move.b $32(a6), $32(a0)
pub fn i_080f92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80F98;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x32);
    bus.write8(ad4, v3 as u8);
    20
}
/// $080F98: move.b $33(a6), $33(a0)
pub fn i_080f98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80F9E;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x33);
    bus.write8(ad4, v3 as u8);
    20
}
/// $080F9E: lea.l $8134c(pc), a1
pub fn i_080f9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80FA2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8134C;
    16
}
/// $080FA2: jsr $4ae.l
pub fn i_080fa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80FA8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80FA8);
    cpu.pc = 0x4AE;
    16
}
/// $080FA8: jsr $5dd02.l
pub fn i_080fa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80FAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80FAE);
    cpu.pc = 0x5DD02;
    16
}
/// $080FAE: move.b $32(a6), $32(a0)
pub fn i_080fae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80FB4;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x32);
    bus.write8(ad4, v3 as u8);
    20
}
/// $080FB4: move.b $33(a6), $33(a0)
pub fn i_080fb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80FBA;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x33);
    bus.write8(ad4, v3 as u8);
    20
}
/// $080FBA: lea.l $8134c(pc), a1
pub fn i_080fba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80FBE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8134C;
    16
}
/// $080FBE: jsr $4ae.l
pub fn i_080fbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80FC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80FC4);
    cpu.pc = 0x4AE;
    16
}
/// $080FC4: jsr $5dd02.l
pub fn i_080fc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80FCA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80FCA);
    cpu.pc = 0x5DD02;
    16
}
/// $080FCA: eori.b #$1, $3a(a0)
pub fn i_080fca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80FD0;
    cpu.ir = 0xA28;
    let ad1 = cpu.a[0].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $080FD0: move.b $32(a6), $32(a0)
pub fn i_080fd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80FD6;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x32);
    bus.write8(ad4, v3 as u8);
    20
}
/// $080FD6: move.b $33(a6), $33(a0)
pub fn i_080fd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80FDC;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x33);
    bus.write8(ad4, v3 as u8);
    20
}
/// $080FDC: lea.l $8135c(pc), a1
pub fn i_080fdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80FE0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8135C;
    16
}
/// $080FE0: jsr $4ae.l
pub fn i_080fe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80FE6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80FE6);
    cpu.pc = 0x4AE;
    16
}
/// $080FE6: jsr $5dd02.l
pub fn i_080fe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80FEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80FEC);
    cpu.pc = 0x5DD02;
    16
}
/// $080FEC: move.b $32(a6), $32(a0)
pub fn i_080fec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80FF2;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x32);
    bus.write8(ad4, v3 as u8);
    20
}
/// $080FF2: move.b $33(a6), $33(a0)
pub fn i_080ff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80FF8;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x33);
    bus.write8(ad4, v3 as u8);
    20
}
/// $080FF8: lea.l $8136c(pc), a1
pub fn i_080ff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80FFC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8136C;
    16
}
/// $080FFC: jsr $4ae.l
pub fn i_080ffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81002;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81002);
    cpu.pc = 0x4AE;
    16
}
/// $081002: jsr $5dd02.l
pub fn i_081002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81008;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81008);
    cpu.pc = 0x5DD02;
    16
}
/// $081008: move.b $32(a6), $32(a0)
pub fn i_081008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8100E;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x32);
    bus.write8(ad4, v3 as u8);
    20
}
/// $08100E: move.b $33(a6), $33(a0)
pub fn i_08100e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81014;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x33);
    bus.write8(ad4, v3 as u8);
    20
}
/// $081014: bra.w $811cc
pub fn i_081014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81018;
    cpu.ir = 0x6000;
    cpu.pc = 0x811CC;
    10
}
// ===== Jsr5B6ThenJmpScheduler_0811be  ($0811BE, 14 B, kernel/jsr5b6_jmp518.c)
/// $0811BE: jsr $5b6.l
pub fn i_0811be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x811C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x811C4);
    cpu.pc = 0x5B6;
    16
}
/// $0811C4: jmp $518.l
pub fn i_0811c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x811CA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== TaskHandler_0811cc  ($0811CC, 72 B, enemies/squad/death_handlers_080736.s)
/// $0811CC: move.w #$1023, d0
pub fn i_0811cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x811D0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1023;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0811D0: jsr $2352.l
pub fn i_0811d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x811D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x811D6);
    cpu.pc = 0x2352;
    16
}
/// $0811D6: move.b #$ff, $20(a6)
pub fn i_0811d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x811DC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0811DC: jmp $518.l
pub fn i_0811dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x811E2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== JmpToScheduler_081214  ($081214, 8 B, kernel/jmp_to_scheduler.c)
/// $081214: jmp $518.l
pub fn i_081214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8121A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Template_08121C  ($08121C, 154 B, enemies/squad/death_handlers_080736.s)
/// $08121C: jsr $5e9b6.l
pub fn i_08121c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81222;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81222);
    cpu.pc = 0x5E9B6;
    16
}
/// $081222: andi.w #$3f, d0
pub fn i_081222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81226;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $081226: move.w #$2000, d1
pub fn i_081226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8122A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08122A: jsr $13c0e.l
pub fn i_08122a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81230;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81230);
    cpu.pc = 0x13C0E;
    16
}
/// $081230: move.w d1, $28(a6)
pub fn i_081230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81234;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $081234: move.w d2, $2a(a6)
pub fn i_081234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81238;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $081238: bra.w $812a4
pub fn i_081238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8123C;
    cpu.ir = 0x6000;
    cpu.pc = 0x812A4;
    10
}
/// $08123C: jsr $5e9b6.l
pub fn i_08123c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81242;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81242);
    cpu.pc = 0x5E9B6;
    16
}
/// $081242: andi.w #$3f, d0
pub fn i_081242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81246;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $081246: addi.w #$40, d0
pub fn i_081246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8124A;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x40, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08124A: move.w #$2000, d1
pub fn i_08124a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8124E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08124E: jsr $13c0e.l
pub fn i_08124e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81254;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81254);
    cpu.pc = 0x13C0E;
    16
}
/// $081254: move.w d1, $28(a6)
pub fn i_081254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81258;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $081258: move.w d2, $2a(a6)
pub fn i_081258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8125C;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08125C: bra.w $812a4
pub fn i_08125c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81260;
    cpu.ir = 0x6000;
    cpu.pc = 0x812A4;
    10
}
/// $081260: jsr $5e9b6.l
pub fn i_081260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81266;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81266);
    cpu.pc = 0x5E9B6;
    16
}
/// $081266: andi.w #$3f, d0
pub fn i_081266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8126A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08126A: addi.w #$80, d0
pub fn i_08126a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8126E;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x80, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08126E: move.w #$2000, d1
pub fn i_08126e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81272;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $081272: jsr $13c0e.l
pub fn i_081272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81278;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81278);
    cpu.pc = 0x13C0E;
    16
}
/// $081278: move.w d1, $28(a6)
pub fn i_081278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8127C;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08127C: move.w d2, $2a(a6)
pub fn i_08127c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81280;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $081280: bra.w $812a4
pub fn i_081280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81284;
    cpu.ir = 0x6000;
    cpu.pc = 0x812A4;
    10
}
/// $081284: jsr $5e9b6.l
pub fn i_081284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8128A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8128A);
    cpu.pc = 0x5E9B6;
    16
}
/// $08128A: andi.w #$3f, d0
pub fn i_08128a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8128E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08128E: addi.w #$c0, d0
pub fn i_08128e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81292;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xC0, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $081292: move.w #$2000, d1
pub fn i_081292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81296;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $081296: jsr $13c0e.l
pub fn i_081296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8129C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8129C);
    cpu.pc = 0x13C0E;
    16
}
/// $08129C: move.w d1, $28(a6)
pub fn i_08129c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x812A0;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0812A0: move.w d2, $2a(a6)
pub fn i_0812a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x812A4;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0812A4: move.w #$3, $70(a6)
pub fn i_0812a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x812AA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0812AA: move.w #$3, $72(a6)
pub fn i_0812aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x812B0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0812B0: move.w #$4, $74(a6)
pub fn i_0812b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x812B6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_0812b6  ($0812B6, 8 B, kernel/task_handlers.c)
/// $0812B6: lea.l $812be(pc), a1
pub fn i_0812b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x812BA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x812BE;
    16
}
/// $0812BA: move.l a1, (a6)
pub fn i_0812ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x812BC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0812BC: rts
pub fn i_0812bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x812BE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_0812be  ($0812BE, 134 B, enemies/squad/death_handlers_080736.s)
/// $0812BE: jsr $2783a.l
pub fn i_0812be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x812C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x812C4);
    cpu.pc = 0x2783A;
    16
}
/// $0812C4: subq.w #$1, $70(a6)
pub fn i_0812c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x812C8;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0812C8: bne.w $81330
pub fn i_0812c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x812CC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x81330; return 10; }
    8
}
/// $0812CC: move.w $72(a6), $70(a6)
pub fn i_0812cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x812D2;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0812D2: move.w $28(a6), d0
pub fn i_0812d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x812D6;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0812D6: move.w d0, d1
pub fn i_0812d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x812D8;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0812D8: asr.w #$8, d1
pub fn i_0812d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x812DA;
    cpu.ir = 0xE041;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $0812DA: add.b d0, $26(a6)
pub fn i_0812da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x812DE;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x26);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $0812DE: moveq #$0, d0
pub fn i_0812de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x812E0;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0812E0: addx.w d0, d1
pub fn i_0812e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x812E2;
    cpu.ir = 0xD340;
    let x = cpu.sr.get(StatusRegister::X) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let h2 = cpu.d[0] & 0xFFFF;
    let r = addx_with_flags(cpu, h1, h2, x, Size::Word);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0812E2: add.w d1, $22(a6)
pub fn i_0812e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x812E6;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0812E6: move.w $2a(a6), d0
pub fn i_0812e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x812EA;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0812EA: move.w d0, d1
pub fn i_0812ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x812EC;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0812EC: asr.w #$8, d1
pub fn i_0812ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x812EE;
    cpu.ir = 0xE041;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $0812EE: add.b d0, $27(a6)
pub fn i_0812ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x812F2;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x27);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $0812F2: moveq #$0, d0
pub fn i_0812f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x812F4;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0812F4: addx.w d0, d1
pub fn i_0812f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x812F6;
    cpu.ir = 0xD340;
    let x = cpu.sr.get(StatusRegister::X) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let h2 = cpu.d[0] & 0xFFFF;
    let r = addx_with_flags(cpu, h1, h2, x, Size::Word);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0812F6: add.w d1, $24(a6)
pub fn i_0812f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x812FA;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0812FA: jsr $5e804.l
pub fn i_0812fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81300;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81300);
    cpu.pc = 0x5E804;
    16
}
/// $081300: bcs.w $8132a
pub fn i_081300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81304;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8132A; return 10; }
    8
}
/// $081304: lea.l $77efe.l, a1
pub fn i_081304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8130A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77EFE;
    20
}
/// $08130A: jsr $4ae.l
pub fn i_08130a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81310;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81310);
    cpu.pc = 0x4AE;
    16
}
/// $081310: jsr $5dd02.l
pub fn i_081310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81316;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81316);
    cpu.pc = 0x5DD02;
    16
}
/// $081316: move.b $32(a6), $32(a0)
pub fn i_081316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8131C;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x32);
    bus.write8(ad4, v3 as u8);
    20
}
/// $08131C: move.b $33(a6), $33(a0)
pub fn i_08131c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81322;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x33);
    bus.write8(ad4, v3 as u8);
    20
}
/// $081322: subq.w #$1, $74(a6)
pub fn i_081322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81326;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $081326: bne.w $81330
pub fn i_081326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8132A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x81330; return 10; }
    8
}
/// $08132A: jmp $518.l
pub fn i_08132a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81330;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $081330: rts
pub fn i_081330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81332;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetTaskHandler_081344  ($081344, 8 B, kernel/task_handlers.c)
/// $081344: lea.l $81018(pc), a1
pub fn i_081344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81348;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81018;
    16
}
/// $081348: move.l a1, (a6)
pub fn i_081348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8134A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08134A: rts
pub fn i_08134a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8134C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetTaskHandler_081632  ($081632, 8 B, kernel/task_handlers.c)
/// $081632: lea.l $81214(pc), a1
pub fn i_081632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81636;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81214;
    16
}
/// $081636: move.l a1, (a6)
pub fn i_081636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81638;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $081638: rts
pub fn i_081638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8163A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_CmdrExitHop_08163a  ($08163A, 44 B, enemies/squad/death_handlers_080736.s)
/// $08163A: move.w #$4, d1
pub fn i_08163a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8163E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08163E: jsr $236e.l
pub fn i_08163e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81644;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81644);
    cpu.pc = 0x236E;
    16
}
/// $081644: lea.l $2dd8b6.l, a0
pub fn i_081644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8164A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DD8B6;
    20
}
/// $08164A: jsr $28cd4.l
pub fn i_08164a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81650;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81650);
    cpu.pc = 0x28CD4;
    16
}
/// $081650: lea.l $81656(pc), a1
pub fn i_081650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81654;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81656;
    16
}
/// $081654: move.l a1, (a6)
pub fn i_081654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81656;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $081656: jsr $2783a.l
pub fn i_081656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8165C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8165C);
    cpu.pc = 0x2783A;
    16
}
/// $08165C: jsr $28d70.l
pub fn i_08165c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81662;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81662);
    cpu.pc = 0x28D70;
    16
}
/// $081662: bcc.w $8166c
pub fn i_081662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81666;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8166C; return 10; }
    8
}
// ===== SetTaskHandler_081666  ($081666, 8 B, kernel/task_handlers.c)
/// $081666: lea.l $81214(pc), a1
pub fn i_081666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8166A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81214;
    16
}
/// $08166A: move.l a1, (a6)
pub fn i_08166a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8166C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08166C: rts
pub fn i_08166c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8166E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_CmdrEscapeInit_08166e  ($08166E, 44 B, enemies/squad/death_handlers_080736.s)
/// $08166E: move.w #$1022, d0
pub fn i_08166e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81672;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1022;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $081672: jsr $2352.l
pub fn i_081672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81678;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81678);
    cpu.pc = 0x2352;
    16
}
/// $081678: move.w #$2000, d0
pub fn i_081678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8167C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08167C: jsr $28134.l
pub fn i_08167c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81682;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81682);
    cpu.pc = 0x28134;
    16
}
/// $081682: andi.w #$ffe3, $38(a6)
pub fn i_081682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81688;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $081688: ori.w #$14, $38(a6)
pub fn i_081688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8168E;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $08168E: jsr $13600.l
pub fn i_08168e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81694;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81694);
    cpu.pc = 0x13600;
    16
}
/// $081694: jmp $77f6a.l
pub fn i_081694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8169A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x77F6A;
    8
}
// ===== Sub_0008169A  ($08169A, 196 B, enemies/squad/death_handlers_080736.s)
/// $08169A: jsr $5e1ea.l
pub fn i_08169a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x816A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x816A0);
    cpu.pc = 0x5E1EA;
    16
}
/// $0816A0: move.l a0, $7a(a6)
pub fn i_0816a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x816A4;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0816A4: clr.w d2
pub fn i_0816a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x816A6;
    cpu.ir = 0x4242;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0816A6: lea.l $2e3d18.l, a1
pub fn i_0816a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x816AC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E3D18;
    20
}
/// $0816AC: cmpi.w #$d0, $22(a0)
pub fn i_0816ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x816B2;
    cpu.ir = 0xC68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xD0, Size::Word, false);
    16
}
/// $0816B2: bgt.w $816d4
pub fn i_0816b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x816B6;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x816D4; return 10; }
    8
}
/// $0816B6: lea.l $2e3cf8.l, a1
pub fn i_0816b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x816BC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E3CF8;
    20
}
/// $0816BC: cmpi.w #$70, $22(a0)
pub fn i_0816bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x816C2;
    cpu.ir = 0xC68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x70, Size::Word, false);
    16
}
/// $0816C2: ble.w $816d4
pub fn i_0816c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x816C6;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x816D4; return 10; }
    8
}
/// $0816C6: jsr $5e9b6.l
pub fn i_0816c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x816CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x816CC);
    cpu.pc = 0x5E9B6;
    16
}
/// $0816CC: andi.w #$1, d0
pub fn i_0816cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x816D0;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0816D0: lsl.w #$5, d0
pub fn i_0816d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x816D2;
    cpu.ir = 0xEB48;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $0816D2: move.w d0, d2
pub fn i_0816d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x816D4;
    cpu.ir = 0x3400;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0816D4: movea.l $c(a6), a0
pub fn i_0816d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x816D8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0816D8: move.b $9b(a0), d0
pub fn i_0816d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x816DC;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0816DC: cmpi.b #$3, d0
pub fn i_0816dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x816E0;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $0816E0: bcs.w $816ee
pub fn i_0816e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x816E4;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x816EE; return 10; }
    8
}
/// $0816E4: move.l d2, -(a7)
pub fn i_0816e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x816E6;
    cpu.ir = 0x2F02;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $0816E6: jsr $5e9b6.l
pub fn i_0816e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x816EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x816EC);
    cpu.pc = 0x5E9B6;
    16
}
/// $0816EC: move.l (a7)+, d2
pub fn i_0816ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x816EE;
    cpu.ir = 0x241F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0816EE: andi.w #$3, d0
pub fn i_0816ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x816F2;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0816F2: move.w d0, d3
pub fn i_0816f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x816F4;
    cpu.ir = 0x3600;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0816F4: addi.w #$9c, d0
pub fn i_0816f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x816F8;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x9C, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0816F8: move.b (a0, d0.w), d1
pub fn i_0816f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x816FC;
    cpu.ir = 0x1230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    14
}
/// $0816FC: ext.w d1
pub fn i_0816fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x816FE;
    cpu.ir = 0x4881;
    let v = cpu.d[1] as i8 as i16 as u16 as u32; cpu.d[1] = (cpu.d[1] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $0816FE: move.w d1, $24(a6)
pub fn i_0816fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81702;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $081702: lsl.l #$3, d3
pub fn i_081702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81704;
    cpu.ir = 0xE78B;
    let h1 = cpu.d[3] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Long, 1, true);
    cpu.d[3] = r;
    12
}
/// $081704: add.w d3, d2
pub fn i_081704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81706;
    cpu.ir = 0xD443;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $081706: move.w (a1, d2.w), $22(a6)
pub fn i_081706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8170C;
    cpu.ir = 0x3D71;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    22
}
/// $08170C: move.w $2(a1, d2.w), d1
pub fn i_08170c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81710;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $081710: add.w d1, $24(a6)
pub fn i_081710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81714;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $081714: move.w $4(a1, d2.w), d1
pub fn i_081714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81718;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x4).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $081718: move.b d1, $3a(a6)
pub fn i_081718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8171C;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08171C: move.w $6(a1, d2.w), d0
pub fn i_08171c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81720;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x6).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $081720: move.w d0, $78(a6)
pub fn i_081720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81724;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad2, v1 as u16);
    12
}
/// $081724: add.w d0, d0
pub fn i_081724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81726;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $081726: add.w d0, d0
pub fn i_081726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81728;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $081728: lea.l $2e3d38.l, a0
pub fn i_081728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8172E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E3D38;
    20
}
/// $08172E: move.l (a0, d0.w), $90(a6)
pub fn i_08172e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81734;
    cpu.ir = 0x2D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x90);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    30
}
/// $081734: moveq #$0, d1
pub fn i_081734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81736;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $081736: move.w d1, $94(a6)
pub fn i_081736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8173A;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08173A: move.b d1, $96(a6)
pub fn i_08173a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8173E;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x96);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08173E: lea.l $2e3d50.l, a0
pub fn i_08173e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81744;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E3D50;
    20
}
/// $081744: movea.l (a0, d0.w), a0
pub fn i_081744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81748;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $081748: jsr $799de.l
pub fn i_081748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8174E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8174E);
    cpu.pc = 0x799DE;
    16
}
/// $08174E: move.w d0, $72(a6)
pub fn i_08174e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81752;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $081752: clr.b $7e(a6)
pub fn i_081752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81756;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x7E);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $081756: move.b #$ff, $7f(a6)
pub fn i_081756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8175C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7F);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08175C: rts
pub fn i_08175c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8175E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_0008179E  ($08179E, 44 B, enemies/squad/death_handlers_080736.s)
/// $08179E: lea.l $8175e(pc), a0
pub fn i_08179e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x817A2;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x8175E;
    16
}
/// $0817A2: move.b $34(a6), d0
pub fn i_0817a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x817A6;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0817A6: andi.l #$ff, d0
pub fn i_0817a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x817AC;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $0817AC: lsr.l #$3, d0
pub fn i_0817ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x817AE;
    cpu.ir = 0xE688;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Long, 1, false);
    cpu.d[0] = r;
    12
}
/// $0817AE: adda.l d0, a0
pub fn i_0817ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x817B0;
    cpu.ir = 0xD1C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add(v1);
    8
}
/// $0817B0: adda.l d0, a0
pub fn i_0817b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x817B2;
    cpu.ir = 0xD1C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add(v1);
    8
}
/// $0817B2: move.w (a0), d0
pub fn i_0817b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x817B4;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $0817B4: move.w $72(a6), d1
pub fn i_0817b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x817B8;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0817B8: muls.w d1, d0
pub fn i_0817b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x817BA;
    cpu.ir = 0xC1C1;
    let v1 = cpu.d[1] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[0] as u16 as i16 as i32)) as u32;
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $0817BA: asr.l #$8, d0
pub fn i_0817ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x817BC;
    cpu.ir = 0xE080;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[0] = r;
    22
}
/// $0817BC: move.w d0, $36(a6)
pub fn i_0817bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x817C0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0817C0: move.b $34(a6), d0
pub fn i_0817c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x817C4;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0817C4: andi.w #$ff, d0
pub fn i_0817c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x817C8;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0817C8: lsr.w #$4, d0
pub fn i_0817c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x817CA;
    cpu.ir = 0xE848;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
// ===== SetTaskW_0817ca  ($0817CA, 6 B, kernel/task_setters_w.c)
/// $0817CA: move.w d0, $76(a6)
pub fn i_0817ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x817CE;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0817CE: rts
pub fn i_0817ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x817D0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_SpeedTailFrags_0817d0  ($0817D0, 20 B, enemies/squad/death_handlers_080736.s)
/// $0817D0: asl.w $36(a6)
pub fn i_0817d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x817D4;
    cpu.ir = 0xE1EE;
    let ad1 = cpu.a[6].wrapping_add(0x36);
    let v2 = bus.read16(ad1) as u32;
    let r3 = do_shift_rotate(cpu, v2, 1, Size::Word, 0, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0817D4: rts
pub fn i_0817d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x817D6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrAbsThunk_0817e4  ($0817E4, 8 B, thunks/jsr_abs_thunks.c)
/// $0817E4: jsr $28cd4.l
pub fn i_0817e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x817EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x817EA);
    cpu.pc = 0x28CD4;
    16
}
/// $0817EA: rts
pub fn i_0817ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x817EC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrAbsThunk_0817f2  ($0817F2, 8 B, thunks/jsr_abs_thunks.c)
/// $0817F2: jsr $28cd4.l
pub fn i_0817f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x817F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x817F8);
    cpu.pc = 0x28CD4;
    16
}
/// $0817F8: rts
pub fn i_0817f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x817FA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrAbsThunk_081800  ($081800, 8 B, thunks/jsr_abs_thunks.c)
/// $081800: jsr $28cd4.l
pub fn i_081800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81806;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81806);
    cpu.pc = 0x28CD4;
    16
}
/// $081806: rts
pub fn i_081806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81808;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrAbsThunk_08180e  ($08180E, 8 B, thunks/jsr_abs_thunks.c)
/// $08180E: jsr $28cd4.l
pub fn i_08180e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81814;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81814);
    cpu.pc = 0x28CD4;
    16
}
/// $081814: rts
pub fn i_081814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81816;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrAbsThunk_08181c  ($08181C, 8 B, thunks/jsr_abs_thunks.c)
/// $08181C: jsr $28cd4.l
pub fn i_08181c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81822;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81822);
    cpu.pc = 0x28CD4;
    16
}
/// $081822: rts
pub fn i_081822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81824;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrAbsThunk_081832  ($081832, 8 B, thunks/jsr_abs_thunks.c)
/// $081832: jsr $28cd4.l
pub fn i_081832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81838;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81838);
    cpu.pc = 0x28CD4;
    16
}
/// $081838: rts
pub fn i_081838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8183A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrAbsThunk_081840  ($081840, 8 B, thunks/jsr_abs_thunks.c)
/// $081840: jsr $28cd4.l
pub fn i_081840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81846;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81846);
    cpu.pc = 0x28CD4;
    16
}
/// $081846: rts
pub fn i_081846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81848;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrAbsThunk_081856  ($081856, 8 B, thunks/jsr_abs_thunks.c)
/// $081856: jsr $28cd4.l
pub fn i_081856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8185C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8185C);
    cpu.pc = 0x28CD4;
    16
}
/// $08185C: rts
pub fn i_08185c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8185E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrAbsThunk_081864  ($081864, 8 B, thunks/jsr_abs_thunks.c)
/// $081864: jsr $28cd4.l
pub fn i_081864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8186A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8186A);
    cpu.pc = 0x28CD4;
    16
}
/// $08186A: rts
pub fn i_08186a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8186C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_0008186C  ($08186C, 18 B, enemies/squad/para_squad_module_081816.s)
/// $08186C: move.w $36(a6), d0
pub fn i_08186c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81870;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $081870: asr.w #$7, d0
pub fn i_081870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81872;
    cpu.ir = 0xEE40;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 7, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    20
}
/// $081872: btst.b #$0, $3a(a6)
pub fn i_081872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81878;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $081878: bne.w $8187e
pub fn i_081878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8187C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8187E; return 10; }
    8
}
/// $08187C: neg.w d0
pub fn i_08187c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8187E;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
// ===== SetTaskW_08187e  ($08187E, 6 B, kernel/task_setters_w.c)
/// $08187E: move.w d0, $2c(a6)
pub fn i_08187e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81882;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $081882: rts
pub fn i_081882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81884;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_00081884  ($081884, 18 B, enemies/squad/para_squad_module_081816.s)
/// $081884: move.w $36(a6), d0
pub fn i_081884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81888;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $081888: asr.w #$4, d0
pub fn i_081888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8188A;
    cpu.ir = 0xE840;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $08188A: btst.b #$0, $3a(a6)
pub fn i_08188a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81890;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $081890: bne.w $81896
pub fn i_081890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81894;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x81896; return 10; }
    8
}
/// $081894: neg.w d0
pub fn i_081894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81896;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
// ===== SetTaskW_081896  ($081896, 6 B, kernel/task_setters_w.c)
/// $081896: move.w d0, $2c(a6)
pub fn i_081896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8189A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08189A: rts
pub fn i_08189a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8189C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== LeaPieceList_08189c  ($08189C, 6 B, enemies/squad/para_squad_module_081816.s)
/// $08189C: lea.l $2e3c54.l, a1
pub fn i_08189c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x818A2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E3C54;
    20
}
// ===== JsrAbsThunk_0818a2  ($0818A2, 8 B, thunks/jsr_abs_thunks.c)
/// $0818A2: jsr $77c7e.l
pub fn i_0818a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x818A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x818A8);
    cpu.pc = 0x77C7E;
    16
}
/// $0818A8: rts
pub fn i_0818a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x818AA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_Build4FromTemplates_0818AA  ($0818AA, 66 B, entity/build4_from_templates_0818aa.s)
/// $0818AA: lea.l $8121c(pc), a1
pub fn i_0818aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x818AE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8121C;
    16
}
/// $0818AE: jsr $4ae.l
pub fn i_0818ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x818B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x818B4);
    cpu.pc = 0x4AE;
    16
}
/// $0818B4: jsr $5dd02.l
pub fn i_0818b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x818BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x818BA);
    cpu.pc = 0x5DD02;
    16
}
/// $0818BA: lea.l $8123c(pc), a1
pub fn i_0818ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x818BE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8123C;
    16
}
/// $0818BE: jsr $4ae.l
pub fn i_0818be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x818C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x818C4);
    cpu.pc = 0x4AE;
    16
}
/// $0818C4: jsr $5dd02.l
pub fn i_0818c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x818CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x818CA);
    cpu.pc = 0x5DD02;
    16
}
/// $0818CA: lea.l $81260(pc), a1
pub fn i_0818ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x818CE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81260;
    16
}
/// $0818CE: jsr $4ae.l
pub fn i_0818ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x818D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x818D4);
    cpu.pc = 0x4AE;
    16
}
/// $0818D4: jsr $5dd02.l
pub fn i_0818d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x818DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x818DA);
    cpu.pc = 0x5DD02;
    16
}
/// $0818DA: lea.l $81284(pc), a1
pub fn i_0818da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x818DE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81284;
    16
}
/// $0818DE: jsr $4ae.l
pub fn i_0818de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x818E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x818E4);
    cpu.pc = 0x4AE;
    16
}
/// $0818E4: jsr $5dd02.l
pub fn i_0818e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x818EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x818EA);
    cpu.pc = 0x5DD02;
    16
}
/// $0818EA: rts
pub fn i_0818ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x818EC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_000818EC  ($0818EC, 16 B, enemies/squad/para_squad_module_081816.s)
/// $0818EC: movea.l $8(a6), a1
pub fn i_0818ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x818F0;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0818F0: move.b $10(a6), d0
pub fn i_0818f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x818F4;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0818F4: cmp.b $10(a1), d0
pub fn i_0818f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x818F8;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $0818F8: bcs.w $81902
pub fn i_0818f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x818FC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x81902; return 10; }
    8
}
// ===== ClearXN_0818fc  ($0818FC, 6 B, thunks/ccr_helpers.c)
/// $0818FC: andi.b #$ee, ccr
pub fn i_0818fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81900;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $081900: rts
pub fn i_081900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81902;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_081902  ($081902, 6 B, thunks/ccr_helpers.c)
/// $081902: ori.b #$11, ccr
pub fn i_081902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81906;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $081906: rts
pub fn i_081906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81908;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_081908  ($081908, 924 B, enemies/squad/para_squad_module_081816.s)
/// $081908: move.w #$28, d0
pub fn i_081908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8190C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08190C: jsr $2352.l
pub fn i_08190c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81912;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81912);
    cpu.pc = 0x2352;
    16
}
/// $081912: jsr $267e2.l
pub fn i_081912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81918;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81918);
    cpu.pc = 0x267E2;
    16
}
/// $081918: lea.l $82c54(pc), a1
pub fn i_081918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8191C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82C54;
    16
}
/// $08191C: jsr $4ae.l
pub fn i_08191c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81922;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81922);
    cpu.pc = 0x4AE;
    16
}
/// $081922: lea.l $81db0(pc), a1
pub fn i_081922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81926;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81DB0;
    16
}
/// $081926: jsr $4ae.l
pub fn i_081926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8192C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8192C);
    cpu.pc = 0x4AE;
    16
}
/// $08192C: jsr $5dd02.l
pub fn i_08192c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81932;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81932);
    cpu.pc = 0x5DD02;
    16
}
/// $081932: lea.l $82052(pc), a1
pub fn i_081932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81936;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82052;
    16
}
/// $081936: jsr $4ae.l
pub fn i_081936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8193C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8193C);
    cpu.pc = 0x4AE;
    16
}
/// $08193C: jsr $5dd02.l
pub fn i_08193c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81942;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81942);
    cpu.pc = 0x5DD02;
    16
}
/// $081942: lea.l $82062(pc), a1
pub fn i_081942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81946;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82062;
    16
}
/// $081946: jsr $4ae.l
pub fn i_081946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8194C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8194C);
    cpu.pc = 0x4AE;
    16
}
/// $08194C: jsr $5dd02.l
pub fn i_08194c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81952;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81952);
    cpu.pc = 0x5DD02;
    16
}
/// $081952: move.b #$6, $5c(a6)
pub fn i_081952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81958;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $081958: lea.l $81cd6(pc), a1
pub fn i_081958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8195C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81CD6;
    16
}
/// $08195C: jsr $4ae.l
pub fn i_08195c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81962;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81962);
    cpu.pc = 0x4AE;
    16
}
/// $081962: jsr $5dd02.l
pub fn i_081962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81968;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81968);
    cpu.pc = 0x5DD02;
    16
}
/// $081968: subq.b #$1, $5c(a6)
pub fn i_081968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8196C;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x5C);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $08196C: move.b $5c(a6), $7c(a0)
pub fn i_08196c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81972;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x7C);
    bus.write8(ad4, v3 as u8);
    20
}
/// $081972: tst.b $5c(a6)
pub fn i_081972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81976;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $081976: bne.b $81958
pub fn i_081976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81978;
    cpu.ir = 0x66E0;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x81958; return 10; }
    8
}
/// $081978: clr.b $7b(a6)
pub fn i_081978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8197C;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x7B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $08197C: clr.w $5c(a6)
pub fn i_08197c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81980;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x5C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $081980: clr.b $72(a6)
pub fn i_081980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81984;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $081984: clr.b $73(a6)
pub fn i_081984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81988;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x73);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $081988: clr.b $20(a6)
pub fn i_081988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8198C;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $08198C: move.b #$3, $7d(a6)
pub fn i_08198c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81992;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $081992: move.l #$2e3f76, $48(a6)
pub fn i_081992(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8199A;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2E3F76;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08199A: move.w #$8000, d0
pub fn i_08199a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8199E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08199E: jsr $28134.l
pub fn i_08199e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x819A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x819A4);
    cpu.pc = 0x28134;
    16
}
/// $0819A4: andi.w #$ffe3, $38(a6)
pub fn i_0819a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x819AA;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0819AA: ori.w #$14, $38(a6)
pub fn i_0819aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x819B0;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0819B0: move.w #$72, d1
pub fn i_0819b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x819B4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x72;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0819B4: jsr $236e.l
pub fn i_0819b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x819BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x819BA);
    cpu.pc = 0x236E;
    16
}
/// $0819BA: move.w #$f, $1c(a6)
pub fn i_0819ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x819C0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0819C0: jsr $138fe.l
pub fn i_0819c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x819C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x819C6);
    cpu.pc = 0x138FE;
    16
}
/// $0819C6: lea.l $2be016.l, a0
pub fn i_0819c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x819CC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BE016;
    20
}
/// $0819CC: jsr $799de.l
pub fn i_0819cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x819D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x819D2);
    cpu.pc = 0x799DE;
    16
}
/// $0819D2: move.w d0, $66(a6)
pub fn i_0819d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x819D6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0819D6: jsr $5e0d4.l
pub fn i_0819d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x819DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x819DC);
    cpu.pc = 0x5E0D4;
    16
}
/// $0819DC: move.l a0, $94(a6)
pub fn i_0819dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x819E0;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0819E0: jsr $82f90(pc)
pub fn i_0819e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x819E4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x819E4);
    cpu.pc = 0x82F90;
    16
}
/// $0819E4: move.w $5c(a6), d0
pub fn i_0819e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x819E8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0819E8: movea.l #$2e541e, a0
pub fn i_0819e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x819EE;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2E541E;
    cpu.a[0] = v1;
    4
}
/// $0819EE: lsl.w #$2, d0
pub fn i_0819ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x819F0;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $0819F0: movea.l (a0, d0.w), a0
pub fn i_0819f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x819F4;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $0819F4: cmpa.l #$ffffffff, a0
pub fn i_0819f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x819FA;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $0819FA: beq.w $81a04
pub fn i_0819fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x819FE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x81A04; return 10; }
    8
}
/// $0819FE: jsr $28cd4.l
pub fn i_0819fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81A04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81A04);
    cpu.pc = 0x28CD4;
    16
}
/// $081A04: lea.l $81a0a(pc), a1
pub fn i_081a04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81A08;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81A0A;
    16
}
/// $081A08: move.l a1, (a6)
pub fn i_081a08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81A0A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $081A0A: jsr $2783a.l
pub fn i_081a0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81A10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81A10);
    cpu.pc = 0x2783A;
    16
}
/// $081A10: jsr $82d12(pc)
pub fn i_081a10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81A14;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x81A14);
    cpu.pc = 0x82D12;
    16
}
/// $081A14: jsr $82cb2(pc)
pub fn i_081a14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81A18;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x81A18);
    cpu.pc = 0x82CB2;
    16
}
/// $081A18: jsr $28d70.l
pub fn i_081a18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81A1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81A1E);
    cpu.pc = 0x28D70;
    16
}
/// $081A1E: jsr $82dae(pc)
pub fn i_081a1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81A22;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x81A22);
    cpu.pc = 0x82DAE;
    16
}
/// $081A22: bcc.w $81a2c
pub fn i_081a22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81A26;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x81A2C; return 10; }
    8
}
/// $081A26: lea.l $81a30(pc), a1
pub fn i_081a26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81A2A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81A30;
    16
}
/// $081A2A: move.l a1, (a6)
pub fn i_081a2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81A2C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $081A2C: bra.w $81cac
pub fn i_081a2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81A30;
    cpu.ir = 0x6000;
    cpu.pc = 0x81CAC;
    10
}
// ===== ClrRamWord_081ca4  ($081CA4, 8 B, thunks/clr_ram_word.c)
/// $081CA4: clr.b $106ed2.l
pub fn i_081ca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81CAA;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106ED2) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106ED2, r2 as u8);
    16
}
/// $081CAA: rts
pub fn i_081caa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81CAC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_00081CAC  ($081CAC, 34 B, enemies/squad/para_squad_module_081816.s)
/// $081CAC: jsr $82e7a(pc)
pub fn i_081cac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81CB0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x81CB0);
    cpu.pc = 0x82E7A;
    16
}
/// $081CB0: jsr $82d70(pc)
pub fn i_081cb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81CB4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x81CB4);
    cpu.pc = 0x82D70;
    16
}
/// $081CB4: jsr $2870a.l
pub fn i_081cb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81CBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81CBA);
    cpu.pc = 0x2870A;
    16
}
/// $081CBA: bcc.w $81cc4
pub fn i_081cba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81CBE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x81CC4; return 10; }
    8
}
/// $081CBE: bclr.b #$3, $13(a6)
pub fn i_081cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81CC4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $081CC4: jsr $28758.l
pub fn i_081cc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81CCA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81CCA);
    cpu.pc = 0x28758;
    16
}
/// $081CCA: bcc.w $81cd4
pub fn i_081cca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81CCE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x81CD4; return 10; }
    8
}
// ===== SetTaskHandler_081cce  ($081CCE, 8 B, kernel/task_handlers.c)
/// $081CCE: lea.l $81bee(pc), a1
pub fn i_081cce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81CD2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81BEE;
    16
}
/// $081CD2: move.l a1, (a6)
pub fn i_081cd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81CD4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $081CD4: rts
pub fn i_081cd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81CD6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_081cd6  ($081CD6, 134 B, enemies/squad/para_squad_module_081816.s)
/// $081CD6: move.w #$71, d1
pub fn i_081cd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81CDA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x71;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $081CDA: jsr $236e.l
pub fn i_081cda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81CE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81CE0);
    cpu.pc = 0x236E;
    16
}
/// $081CE0: move.w #$d, $1c(a6)
pub fn i_081ce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81CE6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $081CE6: jsr $138fe.l
pub fn i_081ce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81CEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81CEC);
    cpu.pc = 0x138FE;
    16
}
/// $081CEC: lea.l $2be21e.l, a0
pub fn i_081cec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81CF2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BE21E;
    20
}
/// $081CF2: jsr $799de.l
pub fn i_081cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81CF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81CF8);
    cpu.pc = 0x799DE;
    16
}
/// $081CF8: move.w d0, $66(a6)
pub fn i_081cf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81CFC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $081CFC: move.b $7c(a6), d0
pub fn i_081cfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D00;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $081D00: andi.w #$7, d0
pub fn i_081d00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D04;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $081D04: movea.l #$2e542e, a0
pub fn i_081d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D0A;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2E542E;
    cpu.a[0] = v1;
    4
}
/// $081D0A: lsl.w #$2, d0
pub fn i_081d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D0C;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $081D0C: movea.l (a0, d0.w), a0
pub fn i_081d0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D10;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $081D10: cmpa.l #$ffffffff, a0
pub fn i_081d10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D16;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $081D16: beq.w $81d20
pub fn i_081d16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D1A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x81D20; return 10; }
    8
}
/// $081D1A: jsr $28cd4.l
pub fn i_081d1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D20;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81D20);
    cpu.pc = 0x28CD4;
    16
}
/// $081D20: lea.l $81d26(pc), a1
pub fn i_081d20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D24;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81D26;
    16
}
/// $081D24: move.l a1, (a6)
pub fn i_081d24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D26;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $081D26: jsr $82d40(pc)
pub fn i_081d26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D2A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x81D2A);
    cpu.pc = 0x82D40;
    16
}
/// $081D2A: jsr $82cc4(pc)
pub fn i_081d2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D2E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x81D2E);
    cpu.pc = 0x82CC4;
    16
}
/// $081D2E: jsr $28d70.l
pub fn i_081d2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D34;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81D34);
    cpu.pc = 0x28D70;
    16
}
/// $081D34: jsr $2870a.l
pub fn i_081d34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81D3A);
    cpu.pc = 0x2870A;
    16
}
/// $081D3A: bcc.w $81d44
pub fn i_081d3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D3E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x81D44; return 10; }
    8
}
/// $081D3E: bclr.b #$3, $13(a6)
pub fn i_081d3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D44;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $081D44: jsr $28758.l
pub fn i_081d44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81D4A);
    cpu.pc = 0x28758;
    16
}
/// $081D4A: bcc.w $81d54
pub fn i_081d4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D4E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x81D54; return 10; }
    8
}
/// $081D4E: lea.l $81d64(pc), a1
pub fn i_081d4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D52;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81D64;
    16
}
/// $081D52: move.l a1, (a6)
pub fn i_081d52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D54;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $081D54: jsr $82ee0(pc)
pub fn i_081d54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D58;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x81D58);
    cpu.pc = 0x82EE0;
    16
}
/// $081D58: bcc.w $81d62
pub fn i_081d58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D5C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x81D62; return 10; }
    8
}
// ===== SetTaskHandler_081d5c  ($081D5C, 8 B, kernel/task_handlers.c)
/// $081D5C: lea.l $81d64(pc), a1
pub fn i_081d5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D60;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81D64;
    16
}
/// $081D60: move.l a1, (a6)
pub fn i_081d60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D62;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $081D62: rts
pub fn i_081d62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D64;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_081d64  ($081D64, 68 B, enemies/squad/para_squad_module_081816.s)
/// $081D64: jsr $831da(pc)
pub fn i_081d64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D68;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x81D68);
    cpu.pc = 0x831DA;
    16
}
/// $081D68: bclr.b #$0, $13(a6)
pub fn i_081d68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D6E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $081D6E: move.l #$ffffffff, $48(a6)
pub fn i_081d6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D76;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $081D76: move.b $7c(a6), d0
pub fn i_081d76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D7A;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $081D7A: andi.w #$7, d0
pub fn i_081d7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D7E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $081D7E: movea.l #$2e5446, a0
pub fn i_081d7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D84;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2E5446;
    cpu.a[0] = v1;
    4
}
/// $081D84: lsl.w #$2, d0
pub fn i_081d84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D86;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $081D86: movea.l (a0, d0.w), a0
pub fn i_081d86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D8A;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $081D8A: cmpa.l #$ffffffff, a0
pub fn i_081d8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D90;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $081D90: beq.w $81d9a
pub fn i_081d90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D94;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x81D9A; return 10; }
    8
}
/// $081D94: jsr $28cd4.l
pub fn i_081d94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D9A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81D9A);
    cpu.pc = 0x28CD4;
    16
}
/// $081D9A: lea.l $81da0(pc), a1
pub fn i_081d9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D9E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81DA0;
    16
}
/// $081D9E: move.l a1, (a6)
pub fn i_081d9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81DA0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $081DA0: jsr $82d40(pc)
pub fn i_081da0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81DA4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x81DA4);
    cpu.pc = 0x82D40;
    16
}
/// $081DA4: jsr $82cc4(pc)
pub fn i_081da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81DA8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x81DA8);
    cpu.pc = 0x82CC4;
    16
}
// ===== JsrAbsThunk_081da8  ($081DA8, 8 B, thunks/jsr_abs_thunks.c)
/// $081DA8: jsr $28d70.l
pub fn i_081da8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81DAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81DAE);
    cpu.pc = 0x28D70;
    16
}
/// $081DAE: rts
pub fn i_081dae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81DB0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_081db0  ($081DB0, 568 B, enemies/squad/para_squad_module_081816.s)
/// $081DB0: move.w #$71, d1
pub fn i_081db0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81DB4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x71;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $081DB4: jsr $236e.l
pub fn i_081db4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81DBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81DBA);
    cpu.pc = 0x236E;
    16
}
/// $081DBA: move.w #$d, $1c(a6)
pub fn i_081dba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81DC0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $081DC0: jsr $138fe.l
pub fn i_081dc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81DC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81DC6);
    cpu.pc = 0x138FE;
    16
}
/// $081DC6: clr.b $7c(a6)
pub fn i_081dc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81DCA;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x7C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $081DCA: clr.b $20(a6)
pub fn i_081dca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81DCE;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $081DCE: clr.w $80(a6)
pub fn i_081dce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81DD2;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x80);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $081DD2: move.w #$38, $82(a6)
pub fn i_081dd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81DD8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x38;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $081DD8: move.w #$3, $5c(a6)
pub fn i_081dd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81DDE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $081DDE: lea.l $2be322.l, a0
pub fn i_081dde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81DE4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BE322;
    20
}
/// $081DE4: tst.b $73(a6)
pub fn i_081de4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81DE8;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $081DE8: beq.w $81df2
pub fn i_081de8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81DEC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x81DF2; return 10; }
    8
}
/// $081DEC: lea.l $2be3a4.l, a0
pub fn i_081dec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81DF2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BE3A4;
    20
}
/// $081DF2: jsr $799de.l
pub fn i_081df2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81DF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81DF8);
    cpu.pc = 0x799DE;
    16
}
/// $081DF8: move.w d0, $70(a6)
pub fn i_081df8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81DFC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $081DFC: lea.l $2e5c98.l, a0
pub fn i_081dfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E02;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E5C98;
    20
}
/// $081E02: jsr $28cd4.l
pub fn i_081e02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81E08);
    cpu.pc = 0x28CD4;
    16
}
/// $081E08: lea.l $81e0e(pc), a1
pub fn i_081e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E0C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81E0E;
    16
}
/// $081E0C: move.l a1, (a6)
pub fn i_081e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E0E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $081E0E: jsr $82ce2(pc)
pub fn i_081e0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E12;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x81E12);
    cpu.pc = 0x82CE2;
    16
}
/// $081E12: jsr $82d40(pc)
pub fn i_081e12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E16;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x81E16);
    cpu.pc = 0x82D40;
    16
}
/// $081E16: jsr $28d70.l
pub fn i_081e16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81E1C);
    cpu.pc = 0x28D70;
    16
}
/// $081E1C: jsr $82f24(pc)
pub fn i_081e1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E20;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x81E20);
    cpu.pc = 0x82F24;
    16
}
/// $081E20: bcc.w $81e36
pub fn i_081e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E24;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x81E36; return 10; }
    8
}
/// $081E24: move.b $106f28.l, d1
pub fn i_081e24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E2A;
    cpu.ir = 0x1239;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    16
}
/// $081E2A: andi.b #$7, d1
pub fn i_081e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E2E;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0x7) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $081E2E: bne.w $81e36
pub fn i_081e2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E32;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x81E36; return 10; }
    8
}
/// $081E32: add.w d0, $5c(a6)
pub fn i_081e32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E36;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x5C);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $081E36: movea.l $c(a6), a0
pub fn i_081e36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E3A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $081E3A: btst.b #$2, $72(a0)
pub fn i_081e3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E40;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $081E40: beq.w $81e4a
pub fn i_081e40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E44;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x81E4A; return 10; }
    8
}
/// $081E44: lea.l $81e4e(pc), a1
pub fn i_081e44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E48;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81E4E;
    16
}
/// $081E48: move.l a1, (a6)
pub fn i_081e48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E4A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $081E4A: bra.w $8202a
pub fn i_081e4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E4E;
    cpu.ir = 0x6000;
    cpu.pc = 0x8202A;
    10
}
// ===== SetTaskHandler_081fe8  ($081FE8, 8 B, kernel/task_handlers.c)
/// $081FE8: lea.l $81ff0(pc), a1
pub fn i_081fe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81FEC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81FF0;
    16
}
/// $081FEC: move.l a1, (a6)
pub fn i_081fec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81FEE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $081FEE: rts
pub fn i_081fee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81FF0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_081ff0  ($081FF0, 50 B, enemies/squad/para_squad_module_081816.s)
/// $081FF0: lea.l $2e5aa6.l, a1
pub fn i_081ff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81FF6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E5AA6;
    20
}
/// $081FF6: jsr $77c7e.l
pub fn i_081ff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81FFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81FFC);
    cpu.pc = 0x77C7E;
    16
}
/// $081FFC: lea.l $2e59fe.l, a1
pub fn i_081ffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82002;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E59FE;
    20
}
/// $082002: jsr $77c7e.l
pub fn i_082002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82008;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82008);
    cpu.pc = 0x77C7E;
    16
}
/// $082008: lea.l $2e5d20.l, a0
pub fn i_082008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8200E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E5D20;
    20
}
/// $08200E: jsr $28cd4.l
pub fn i_08200e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82014;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82014);
    cpu.pc = 0x28CD4;
    16
}
/// $082014: lea.l $8201a(pc), a1
pub fn i_082014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82018;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8201A;
    16
}
/// $082018: move.l a1, (a6)
pub fn i_082018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8201A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08201A: jsr $82ce2(pc)
pub fn i_08201a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8201E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8201E);
    cpu.pc = 0x82CE2;
    16
}
/// $08201E: jsr $82d40(pc)
pub fn i_08201e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82022;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x82022);
    cpu.pc = 0x82D40;
    16
}
// ===== JsrAbsThunk_082022  ($082022, 8 B, thunks/jsr_abs_thunks.c)
/// $082022: jsr $28d70.l
pub fn i_082022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82028;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82028);
    cpu.pc = 0x28D70;
    16
}
/// $082028: rts
pub fn i_082028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8202A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_0008202A  ($08202A, 32 B, enemies/squad/para_squad_module_081816.s)
/// $08202A: subq.w #$1, $70(a6)
pub fn i_08202a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8202E;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08202E: cmpi.w #$0, $70(a6)
pub fn i_08202e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82034;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $082034: bgt.w $82042
pub fn i_082034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82038;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x82042; return 10; }
    8
}
/// $082038: movea.l $c(a6), a0
pub fn i_082038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8203C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $08203C: bset.b #$4, $72(a0)
pub fn i_08203c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82042;
    cpu.ir = 0x8E8;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $082042: jsr $82ee0(pc)
pub fn i_082042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82046;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x82046);
    cpu.pc = 0x82EE0;
    16
}
/// $082046: bcc.w $82050
pub fn i_082046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8204A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x82050; return 10; }
    8
}
// ===== SetTaskHandler_08204a  ($08204A, 8 B, kernel/task_handlers.c)
/// $08204A: lea.l $81fac(pc), a1
pub fn i_08204a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8204E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81FAC;
    16
}
/// $08204E: move.l a1, (a6)
pub fn i_08204e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82050;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082050: rts
pub fn i_082050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82052;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_082052  ($082052, 936 B, enemies/squad/para_squad_module_081816.s)
/// $082052: move.w #$4d, $80(a6)
pub fn i_082052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82058;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x4D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $082058: move.w #$2e, $82(a6)
pub fn i_082058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8205E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08205E: bra.w $82074
pub fn i_08205e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82062;
    cpu.ir = 0x6000;
    cpu.pc = 0x82074;
    10
}
/// $082074: move.w #$71, d1
pub fn i_082074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82078;
    cpu.ir = 0x323C;
    let v1: u32 = 0x71;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $082078: jsr $236e.l
pub fn i_082078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8207E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8207E);
    cpu.pc = 0x236E;
    16
}
/// $08207E: move.w #$e, $1c(a6)
pub fn i_08207e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82084;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $082084: jsr $138fe.l
pub fn i_082084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8208A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8208A);
    cpu.pc = 0x138FE;
    16
}
/// $08208A: lea.l $2be19c.l, a0
pub fn i_08208a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82090;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BE19C;
    20
}
/// $082090: jsr $799de.l
pub fn i_082090(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82096;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82096);
    cpu.pc = 0x799DE;
    16
}
/// $082096: move.w d0, $66(a6)
pub fn i_082096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8209A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08209A: clr.b $7c(a6)
pub fn i_08209a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8209E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x7C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $08209E: move.l #$2e3fca, $48(a6)
pub fn i_08209e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x820A6;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2E3FCA;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0820A6: lea.l $2be426.l, a0
pub fn i_0820a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x820AC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BE426;
    20
}
/// $0820AC: tst.b $73(a6)
pub fn i_0820ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x820B0;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0820B0: beq.w $820ba
pub fn i_0820b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x820B4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x820BA; return 10; }
    8
}
/// $0820B4: lea.l $2be4a8.l, a0
pub fn i_0820b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x820BA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BE4A8;
    20
}
/// $0820BA: jsr $799de.l
pub fn i_0820ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x820C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x820C0);
    cpu.pc = 0x799DE;
    16
}
/// $0820C0: move.w d0, $70(a6)
pub fn i_0820c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x820C4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0820C4: clr.w $5c(a6)
pub fn i_0820c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x820C8;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x5C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0820C8: lea.l $2e5d2c.l, a0
pub fn i_0820c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x820CE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E5D2C;
    20
}
/// $0820CE: jsr $28cd4.l
pub fn i_0820ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x820D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x820D4);
    cpu.pc = 0x28CD4;
    16
}
/// $0820D4: lea.l $820da(pc), a1
pub fn i_0820d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x820D8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x820DA;
    16
}
/// $0820D8: move.l a1, (a6)
pub fn i_0820d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x820DA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0820DA: jsr $82ce2(pc)
pub fn i_0820da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x820DE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x820DE);
    cpu.pc = 0x82CE2;
    16
}
/// $0820DE: jsr $28d70.l
pub fn i_0820de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x820E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x820E4);
    cpu.pc = 0x28D70;
    16
}
/// $0820E4: jsr $82ef8(pc)
pub fn i_0820e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x820E8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x820E8);
    cpu.pc = 0x82EF8;
    16
}
/// $0820E8: lea.l $2e5846.l, a0
pub fn i_0820e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x820EE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E5846;
    20
}
/// $0820EE: movea.l #$ffffffff, a1
pub fn i_0820ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x820F4;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $0820F4: jsr $772.l
pub fn i_0820f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x820FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x820FA);
    cpu.pc = 0x772;
    16
}
/// $0820FA: bra.w $82402
pub fn i_0820fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x820FE;
    cpu.ir = 0x6000;
    cpu.pc = 0x82402;
    10
}
// ===== JsrAbsThunk_0823fa  ($0823FA, 8 B, thunks/jsr_abs_thunks.c)
/// $0823FA: jsr $28d70.l
pub fn i_0823fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82400;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82400);
    cpu.pc = 0x28D70;
    16
}
/// $082400: rts
pub fn i_082400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82402;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_00082402  ($082402, 76 B, enemies/squad/para_squad_module_081816.s)
/// $082402: subq.w #$1, $70(a6)
pub fn i_082402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82406;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $082406: cmpi.w #$0, $70(a6)
pub fn i_082406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8240C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08240C: bgt.w $8241a
pub fn i_08240c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82410;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8241A; return 10; }
    8
}
/// $082410: movea.l $c(a6), a0
pub fn i_082410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82414;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $082414: bset.b #$3, $72(a0)
pub fn i_082414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8241A;
    cpu.ir = 0x8E8;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08241A: jsr $2870a.l
pub fn i_08241a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82420;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82420);
    cpu.pc = 0x2870A;
    16
}
/// $082420: bcc.w $82436
pub fn i_082420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82424;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x82436; return 10; }
    8
}
/// $082424: bclr.b #$3, $13(a6)
pub fn i_082424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8242A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08242A: lea.l $5e766.l, a0
pub fn i_08242a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82430;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $082430: jsr $5e770.l
pub fn i_082430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82436;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82436);
    cpu.pc = 0x5E770;
    16
}
/// $082436: jsr $82ee0(pc)
pub fn i_082436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8243A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8243A);
    cpu.pc = 0x82EE0;
    16
}
/// $08243A: bcc.w $82444
pub fn i_08243a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8243E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x82444; return 10; }
    8
}
/// $08243E: lea.l $82388(pc), a1
pub fn i_08243e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82442;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82388;
    16
}
/// $082442: move.l a1, (a6)
pub fn i_082442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82444;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082444: jsr $28758.l
pub fn i_082444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8244A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8244A);
    cpu.pc = 0x28758;
    16
}
/// $08244A: bcc.w $82454
pub fn i_08244a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8244E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x82454; return 10; }
    8
}
// ===== SetTaskHandler_08244e  ($08244E, 8 B, kernel/task_handlers.c)
/// $08244E: lea.l $82388(pc), a1
pub fn i_08244e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82452;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82388;
    16
}
/// $082452: move.l a1, (a6)
pub fn i_082452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82454;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082454: rts
pub fn i_082454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82456;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Jsr5B6ThenJmpScheduler_082456  ($082456, 14 B, kernel/jsr5b6_jmp518.c)
/// $082456: jsr $5b6.l
pub fn i_082456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8245C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8245C);
    cpu.pc = 0x5B6;
    16
}
/// $08245C: jmp $518.l
pub fn i_08245c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82462;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== JmpToScheduler_082464  ($082464, 8 B, kernel/jmp_to_scheduler.c)
/// $082464: jmp $518.l
pub fn i_082464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8246A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== TaskHandler_08246c  ($08246C, 160 B, enemies/squad/para_squad_module_081816.s)
/// $08246C: jsr $13600.l
pub fn i_08246c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82472;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82472);
    cpu.pc = 0x13600;
    16
}
/// $082472: move.l #$ffffffff, $48(a6)
pub fn i_082472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8247A;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08247A: jmp $77f6a.l
pub fn i_08247a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82480;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x77F6A;
    8
}
// ===== SetTaskHandler_08250c  ($08250C, 8 B, kernel/task_handlers.c)
/// $08250C: lea.l $82456(pc), a1
pub fn i_08250c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82510;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82456;
    16
}
/// $082510: move.l a1, (a6)
pub fn i_082510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82512;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082512: rts
pub fn i_082512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82514;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_082514  ($082514, 70 B, enemies/squad/para_squad_module_081816.s)
/// $082514: jsr $5e1aa.l
pub fn i_082514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8251A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8251A);
    cpu.pc = 0x5E1AA;
    16
}
/// $08251A: move.b d0, $7a(a6)
pub fn i_08251a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8251E;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08251E: move.w #$74, d1
pub fn i_08251e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82522;
    cpu.ir = 0x323C;
    let v1: u32 = 0x74;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $082522: jsr $236e.l
pub fn i_082522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82528;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82528);
    cpu.pc = 0x236E;
    16
}
/// $082528: lea.l $2e62fc.l, a0
pub fn i_082528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8252E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E62FC;
    20
}
/// $08252E: jsr $28cd4.l
pub fn i_08252e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82534;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82534);
    cpu.pc = 0x28CD4;
    16
}
/// $082534: lea.l $8253a(pc), a1
pub fn i_082534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82538;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8253A;
    16
}
/// $082538: move.l a1, (a6)
pub fn i_082538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8253A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08253A: jsr $5e506.l
pub fn i_08253a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82540;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82540);
    cpu.pc = 0x5E506;
    16
}
/// $082540: move.w $5c(a0), $5c(a6)
pub fn i_082540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82546;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad4, v3 as u16);
    20
}
/// $082546: subi.w #$38, $24(a6)
pub fn i_082546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8254C;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x38, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08254C: jsr $28d70.l
pub fn i_08254c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82552;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82552);
    cpu.pc = 0x28D70;
    16
}
/// $082552: jsr $82ee0(pc)
pub fn i_082552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82556;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x82556);
    cpu.pc = 0x82EE0;
    16
}
/// $082556: bcc.w $82560
pub fn i_082556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8255A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x82560; return 10; }
    8
}
// ===== SetTaskHandler_08255a  ($08255A, 8 B, kernel/task_handlers.c)
/// $08255A: lea.l $82464(pc), a1
pub fn i_08255a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8255E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82464;
    16
}
/// $08255E: move.l a1, (a6)
pub fn i_08255e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82560;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082560: rts
pub fn i_082560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82562;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_082562  ($082562, 188 B, enemies/squad/para_squad_module_081816.s)
/// $082562: lea.l $2e55d6.l, a0
pub fn i_082562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82568;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E55D6;
    20
}
/// $082568: move.w $5c(a6), d0
pub fn i_082568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8256C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08256C: lsl.w #$3, d0
pub fn i_08256c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8256E;
    cpu.ir = 0xE748;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $08256E: move.w $5e(a6), d1
pub fn i_08256e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82572;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x5E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $082572: add.w d1, d1
pub fn i_082572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82574;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $082574: add.w d1, d0
pub fn i_082574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82576;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $082576: move.w (a0, d0.w), d2
pub fn i_082576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8257A;
    cpu.ir = 0x3430;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $08257A: add.w d2, $22(a6)
pub fn i_08257a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8257E;
    cpu.ir = 0xD56E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08257E: subi.w #$18, $24(a6)
pub fn i_08257e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82584;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $082584: jsr $5e9b6.l
pub fn i_082584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8258A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8258A);
    cpu.pc = 0x5E9B6;
    16
}
/// $08258A: andi.w #$f, d0
pub fn i_08258a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8258E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08258E: move.w d0, d1
pub fn i_08258e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82590;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $082590: add.w d0, d0
pub fn i_082590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82592;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $082592: add.w d1, d0
pub fn i_082592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82594;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $082594: add.w d0, d0
pub fn i_082594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82596;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $082596: lea.l $2e560e.l, a1
pub fn i_082596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8259C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E560E;
    20
}
/// $08259C: move.w $5c(a6), d1
pub fn i_08259c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x825A0;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0825A0: lsl.w #$2, d1
pub fn i_0825a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x825A2;
    cpu.ir = 0xE549;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $0825A2: movea.l (a1, d1.w), a0
pub fn i_0825a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x825A6;
    cpu.ir = 0x2071;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $0825A6: move.w (a0, d0.w), $28(a6)
pub fn i_0825a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x825AC;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad4, v3 as u16);
    22
}
/// $0825AC: move.w $2(a0, d0.w), $2e(a6)
pub fn i_0825ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x825B2;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad4, v3 as u16);
    22
}
/// $0825B2: move.w $4(a0, d0.w), $2a(a6)
pub fn i_0825b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x825B8;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x4).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad4, v3 as u16);
    22
}
/// $0825B8: neg.w $2a(a6)
pub fn i_0825b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x825BC;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0825BC: neg.w $2e(a6)
pub fn i_0825bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x825C0;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0825C0: move.w #$76, d1
pub fn i_0825c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x825C4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x76;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0825C4: jsr $236e.l
pub fn i_0825c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x825CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x825CA);
    cpu.pc = 0x236E;
    16
}
/// $0825CA: move.w #$c000, d0
pub fn i_0825ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x825CE;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0825CE: jsr $28134.l
pub fn i_0825ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x825D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x825D4);
    cpu.pc = 0x28134;
    16
}
/// $0825D4: andi.w #$ffe3, $38(a6)
pub fn i_0825d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x825DA;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0825DA: ori.w #$14, $38(a6)
pub fn i_0825da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x825E0;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0825E0: lea.l $2e69e4.l, a0
pub fn i_0825e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x825E6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E69E4;
    20
}
/// $0825E6: jsr $28cd4.l
pub fn i_0825e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x825EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x825EC);
    cpu.pc = 0x28CD4;
    16
}
/// $0825EC: lea.l $825f2(pc), a1
pub fn i_0825ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x825F0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x825F2;
    16
}
/// $0825F0: move.l a1, (a6)
pub fn i_0825f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x825F2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0825F2: jsr $27d50.l
pub fn i_0825f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x825F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x825F8);
    cpu.pc = 0x27D50;
    16
}
/// $0825F8: jsr $28d70.l
pub fn i_0825f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x825FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x825FE);
    cpu.pc = 0x28D70;
    16
}
/// $0825FE: bcc.w $82608
pub fn i_0825fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82602;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x82608; return 10; }
    8
}
/// $082602: lea.l $82456(pc), a1
pub fn i_082602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82606;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82456;
    16
}
/// $082606: move.l a1, (a6)
pub fn i_082606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82608;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082608: movea.l #$ffffffff, a0
pub fn i_082608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8260E;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $08260E: lea.l $2e5416.l, a0
pub fn i_08260e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82614;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E5416;
    20
}
/// $082614: jsr $5dd56.l
pub fn i_082614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8261A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8261A);
    cpu.pc = 0x5DD56;
    16
}
/// $08261A: bcc.w $82624
pub fn i_08261a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8261E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x82624; return 10; }
    8
}
// ===== SetTaskHandler_08261e  ($08261E, 8 B, kernel/task_handlers.c)
/// $08261E: lea.l $82456(pc), a1
pub fn i_08261e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82622;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82456;
    16
}
/// $082622: move.l a1, (a6)
pub fn i_082622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82624;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082624: rts
pub fn i_082624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82626;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_082626  ($082626, 78 B, enemies/squad/para_squad_module_081816.s)
/// $082626: move.w #$75, d1
pub fn i_082626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8262A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x75;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08262A: jsr $236e.l
pub fn i_08262a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82630;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82630);
    cpu.pc = 0x236E;
    16
}
/// $082630: lea.l $2e6152.l, a0
pub fn i_082630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82636;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6152;
    20
}
/// $082636: jsr $28cd4.l
pub fn i_082636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8263C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8263C);
    cpu.pc = 0x28CD4;
    16
}
/// $08263C: lea.l $82642(pc), a1
pub fn i_08263c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82640;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82642;
    16
}
/// $082640: move.l a1, (a6)
pub fn i_082640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82642;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082642: jsr $5e506.l
pub fn i_082642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82648;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82648);
    cpu.pc = 0x5E506;
    16
}
/// $082648: subq.w #$1, $38(a6)
pub fn i_082648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8264C;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08264C: jsr $28d70.l
pub fn i_08264c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82652;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82652);
    cpu.pc = 0x28D70;
    16
}
/// $082652: bcc.w $82666
pub fn i_082652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82656;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x82666; return 10; }
    8
}
/// $082656: movea.l $c(a6), a0
pub fn i_082656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8265A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $08265A: move.b #$1, $21(a0)
pub fn i_08265a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82660;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $082660: lea.l $82464(pc), a1
pub fn i_082660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82664;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82464;
    16
}
/// $082664: move.l a1, (a6)
pub fn i_082664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82666;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082666: jsr $283d8.l
pub fn i_082666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8266C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8266C);
    cpu.pc = 0x283D8;
    16
}
/// $08266C: jsr $82ee0(pc)
pub fn i_08266c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82670;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x82670);
    cpu.pc = 0x82EE0;
    16
}
/// $082670: bcc.w $8267a
pub fn i_082670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82674;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8267A; return 10; }
    8
}
// ===== SetTaskHandler_082674  ($082674, 8 B, kernel/task_handlers.c)
/// $082674: lea.l $82464(pc), a1
pub fn i_082674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82678;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82464;
    16
}
/// $082678: move.l a1, (a6)
pub fn i_082678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8267A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08267A: rts
pub fn i_08267a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8267C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_08267c  ($08267C, 440 B, enemies/squad/para_squad_module_081816.s)
/// $08267C: clr.w $70(a6)
pub fn i_08267c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82680;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $082680: lea.l $2e571a.l, a0
pub fn i_082680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82686;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E571A;
    20
}
/// $082686: move.w $5c(a6), d0
pub fn i_082686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8268A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08268A: lsl.w #$2, d0
pub fn i_08268a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8268C;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $08268C: move.l (a0, d0.w), $5c(a6)
pub fn i_08268c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82692;
    cpu.ir = 0x2D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x5C);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    30
}
/// $082692: move.b #$6, $7a(a6)
pub fn i_082692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82698;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $082698: lea.l $8269e(pc), a1
pub fn i_082698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8269C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8269E;
    16
}
/// $08269C: move.l a1, (a6)
pub fn i_08269c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8269E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08269E: btst.b #$0, $70(a6)
pub fn i_08269e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x826A4;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0826A4: bne.w $8271a
pub fn i_0826a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x826A8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8271A; return 10; }
    8
}
/// $0826A8: move.w #$4000, d0
pub fn i_0826a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x826AC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0826AC: jsr $28134.l
pub fn i_0826ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x826B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x826B2);
    cpu.pc = 0x28134;
    16
}
/// $0826B2: andi.w #$ffe3, $38(a6)
pub fn i_0826b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x826B8;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0826B8: ori.w #$14, $38(a6)
pub fn i_0826b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x826BE;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0826BE: subq.b #$1, $7a(a6)
pub fn i_0826be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x826C2;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x7A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $0826C2: bne.w $826cc
pub fn i_0826c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x826C6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x826CC; return 10; }
    8
}
/// $0826C6: lea.l $82464(pc), a1
pub fn i_0826c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x826CA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82464;
    16
}
/// $0826CA: move.l a1, (a6)
pub fn i_0826ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x826CC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0826CC: lea.l $77f22.l, a1
pub fn i_0826cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x826D2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77F22;
    20
}
/// $0826D2: jsr $4ae.l
pub fn i_0826d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x826D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x826D8);
    cpu.pc = 0x4AE;
    16
}
/// $0826D8: jsr $5dd02.l
pub fn i_0826d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x826DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x826DE);
    cpu.pc = 0x5DD02;
    16
}
/// $0826DE: movea.l $5c(a6), a1
pub fn i_0826de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x826E2;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0826E2: move.w (a1), d0
pub fn i_0826e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x826E4;
    cpu.ir = 0x3011;
    let v2 = bus.read16(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $0826E4: move.w $4(a1), d1
pub fn i_0826e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x826E8;
    cpu.ir = 0x3229;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0826E8: add.w d0, $22(a0)
pub fn i_0826e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x826EC;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0826EC: add.w d1, $24(a0)
pub fn i_0826ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x826F0;
    cpu.ir = 0xD368;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0826F0: lea.l $77f22.l, a1
pub fn i_0826f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x826F6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77F22;
    20
}
/// $0826F6: jsr $4ae.l
pub fn i_0826f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x826FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x826FC);
    cpu.pc = 0x4AE;
    16
}
/// $0826FC: jsr $5dd02.l
pub fn i_0826fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82702;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82702);
    cpu.pc = 0x5DD02;
    16
}
/// $082702: movea.l $5c(a6), a1
pub fn i_082702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82706;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $082706: move.w $2(a1), d0
pub fn i_082706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8270A;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08270A: move.w $4(a1), d1
pub fn i_08270a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8270E;
    cpu.ir = 0x3229;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08270E: add.w d0, $22(a0)
pub fn i_08270e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82712;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $082712: add.w d1, $24(a0)
pub fn i_082712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82716;
    cpu.ir = 0xD368;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $082716: addq.l #$6, $5c(a6)
pub fn i_082716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8271A;
    cpu.ir = 0x5CAE;
    let ad1 = cpu.a[6].wrapping_add(0x5C);
    let v2 = bus.read32(ad1);
    let r3 = add_with_flags(cpu, v2, 6, Size::Long, true);
    bus.write32(ad1, r3);
    16
}
/// $08271A: addq.b #$1, $70(a6)
pub fn i_08271a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8271E;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $08271E: rts
pub fn i_08271e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82720;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0827FE: jsr $283d8.l
pub fn i_0827fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82804;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82804);
    cpu.pc = 0x283D8;
    16
}
/// $082804: btst.b #$1, $13(a6)
pub fn i_082804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8280A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $08280A: beq.w $82814
pub fn i_08280a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8280E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x82814; return 10; }
    8
}
/// $08280E: lea.l $8246c(pc), a1
pub fn i_08280e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82812;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8246C;
    16
}
/// $082812: move.l a1, (a6)
pub fn i_082812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82814;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082814: jsr $28758.l
pub fn i_082814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8281A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8281A);
    cpu.pc = 0x28758;
    16
}
/// $08281A: bcc.w $82824
pub fn i_08281a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8281E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x82824; return 10; }
    8
}
/// $08281E: lea.l $828e0(pc), a1
pub fn i_08281e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82822;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x828E0;
    16
}
/// $082822: move.l a1, (a6)
pub fn i_082822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82824;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082824: movea.l #$ffffffff, a0
pub fn i_082824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8282A;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $08282A: jsr $5dd56.l
pub fn i_08282a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82830;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82830);
    cpu.pc = 0x5DD56;
    16
}
/// $082830: bcc.w $8283a
pub fn i_082830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82834;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8283A; return 10; }
    8
}
// ===== SetTaskHandler_082834  ($082834, 8 B, kernel/task_handlers.c)
/// $082834: lea.l $82456(pc), a1
pub fn i_082834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82838;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82456;
    16
}
/// $082838: move.l a1, (a6)
pub fn i_082838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8283A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08283A: rts
pub fn i_08283a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8283C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_08283c  ($08283C, 72 B, enemies/squad/para_squad_helpers_08283c.s)
/// $08283C: bset.b #$4, $6b(a6)
pub fn i_08283c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82842;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $082842: move.w #$ffc0, $2e(a6)
pub fn i_082842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82848;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFC0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $082848: lea.l $2e6940.l, a0
pub fn i_082848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8284E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6940;
    20
}
/// $08284E: jsr $28cd4.l
pub fn i_08284e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82854;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82854);
    cpu.pc = 0x28CD4;
    16
}
/// $082854: lea.l $8285a(pc), a1
pub fn i_082854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82858;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8285A;
    16
}
/// $082858: move.l a1, (a6)
pub fn i_082858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8285A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08285A: jsr $27d50.l
pub fn i_08285a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82860;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82860);
    cpu.pc = 0x27D50;
    16
}
/// $082860: bcc.w $8286a
pub fn i_082860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82864;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8286A; return 10; }
    8
}
/// $082864: lea.l $82884(pc), a1
pub fn i_082864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82868;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82884;
    16
}
/// $082868: move.l a1, (a6)
pub fn i_082868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8286A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08286A: jsr $28d70.l
pub fn i_08286a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82870;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82870);
    cpu.pc = 0x28D70;
    16
}
/// $082870: jsr $2870a.l
pub fn i_082870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82876;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82876);
    cpu.pc = 0x2870A;
    16
}
/// $082876: bcc.w $82880
pub fn i_082876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8287A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x82880; return 10; }
    8
}
/// $08287A: bclr.b #$3, $13(a6)
pub fn i_08287a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82880;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $082880: bra.w $827fe
pub fn i_082880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82884;
    cpu.ir = 0x6000;
    cpu.pc = 0x827FE;
    10
}
// ===== TaskHandler_082884  ($082884, 92 B, enemies/squad/para_squad_helpers_08283c.s)
/// $082884: lea.l $77e10.l, a1
pub fn i_082884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8288A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77E10;
    20
}
/// $08288A: jsr $4ae.l
pub fn i_08288a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82890;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82890);
    cpu.pc = 0x4AE;
    16
}
/// $082890: jsr $5dd02.l
pub fn i_082890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82896;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82896);
    cpu.pc = 0x5DD02;
    16
}
/// $082896: lea.l $2e6956.l, a0
pub fn i_082896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8289C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6956;
    20
}
/// $08289C: jsr $28cd4.l
pub fn i_08289c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x828A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x828A2);
    cpu.pc = 0x28CD4;
    16
}
/// $0828A2: lea.l $828a8(pc), a1
pub fn i_0828a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x828A6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x828A8;
    16
}
/// $0828A6: move.l a1, (a6)
pub fn i_0828a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x828A8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0828A8: jsr $2783a.l
pub fn i_0828a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x828AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x828AE);
    cpu.pc = 0x2783A;
    16
}
/// $0828AE: jsr $28d70.l
pub fn i_0828ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x828B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x828B4);
    cpu.pc = 0x28D70;
    16
}
/// $0828B4: bcc.w $828be
pub fn i_0828b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x828B8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x828BE; return 10; }
    8
}
/// $0828B8: lea.l $828d2(pc), a1
pub fn i_0828b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x828BC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x828D2;
    16
}
/// $0828BC: move.l a1, (a6)
pub fn i_0828bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x828BE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0828BE: jsr $2870a.l
pub fn i_0828be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x828C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x828C4);
    cpu.pc = 0x2870A;
    16
}
/// $0828C4: bcc.w $828ce
pub fn i_0828c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x828C8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x828CE; return 10; }
    8
}
/// $0828C8: bclr.b #$3, $13(a6)
pub fn i_0828c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x828CE;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0828CE: bra.w $827fe
pub fn i_0828ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x828D2;
    cpu.ir = 0x6000;
    cpu.pc = 0x827FE;
    10
}
// ===== TaskHandler_0828e0  ($0828E0, 14 B, enemies/squad/para_squad_helpers_08283c.s)
/// $0828E0: move.w #$109f, d0
pub fn i_0828e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x828E4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x109F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0828E4: jsr $2352.l
pub fn i_0828e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x828EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x828EA);
    cpu.pc = 0x2352;
    16
}
/// $0828EA: bra.w $8246c
pub fn i_0828ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x828EE;
    cpu.ir = 0x6000;
    cpu.pc = 0x8246C;
    10
}
// ===== TaskHandler_0828ee  ($0828EE, 54 B, enemies/squad/para_squad_helpers_08283c.s)
/// $0828EE: move.w #$193, d1
pub fn i_0828ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x828F2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x193;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0828F2: jsr $236e.l
pub fn i_0828f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x828F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x828F8);
    cpu.pc = 0x236E;
    16
}
/// $0828F8: lea.l $2e6728.l, a0
pub fn i_0828f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x828FE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6728;
    20
}
/// $0828FE: jsr $28cd4.l
pub fn i_0828fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82904;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82904);
    cpu.pc = 0x28CD4;
    16
}
/// $082904: lea.l $8290a(pc), a1
pub fn i_082904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82908;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8290A;
    16
}
/// $082908: move.l a1, (a6)
pub fn i_082908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8290A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08290A: jsr $5e506.l
pub fn i_08290a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82910;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82910);
    cpu.pc = 0x5E506;
    16
}
/// $082910: move.w $5e(a0), $5e(a6)
pub fn i_082910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82916;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x5E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x5E);
    bus.write16(ad4, v3 as u16);
    20
}
/// $082916: jsr $28d70.l
pub fn i_082916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8291C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8291C);
    cpu.pc = 0x28D70;
    16
}
/// $08291C: jsr $82ee0(pc)
pub fn i_08291c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82920;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x82920);
    cpu.pc = 0x82EE0;
    16
}
/// $082920: bcc.w $8292a
pub fn i_082920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82924;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8292A; return 10; }
    8
}
// ===== SetTaskHandler_082924  ($082924, 8 B, kernel/task_handlers.c)
/// $082924: lea.l $82464(pc), a1
pub fn i_082924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82928;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82464;
    16
}
/// $082928: move.l a1, (a6)
pub fn i_082928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8292A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08292A: rts
pub fn i_08292a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8292C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_08292c  ($08292C, 120 B, enemies/squad/para_squad_helpers_08283c.s)
/// $08292C: move.w #$d2, d1
pub fn i_08292c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82930;
    cpu.ir = 0x323C;
    let v1: u32 = 0xD2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $082930: jsr $236e.l
pub fn i_082930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82936;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82936);
    cpu.pc = 0x236E;
    16
}
/// $082936: move.w #$c000, d0
pub fn i_082936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8293A;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08293A: jsr $28134.l
pub fn i_08293a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82940;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82940);
    cpu.pc = 0x28134;
    16
}
/// $082940: andi.w #$ffe3, $38(a6)
pub fn i_082940(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82946;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $082946: ori.w #$14, $38(a6)
pub fn i_082946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8294C;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $08294C: jsr $5e9b6.l
pub fn i_08294c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82952;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82952);
    cpu.pc = 0x5E9B6;
    16
}
/// $082952: andi.w #$7f, d0
pub fn i_082952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82956;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $082956: subi.w #$40, d0
pub fn i_082956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8295A;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x40, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08295A: add.w d0, $28(a6)
pub fn i_08295a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8295E;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08295E: jsr $5e9b6.l
pub fn i_08295e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82964;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82964);
    cpu.pc = 0x5E9B6;
    16
}
/// $082964: andi.w #$3f, d0
pub fn i_082964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82968;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $082968: add.w d0, $2a(a6)
pub fn i_082968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8296C;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08296C: lea.l $2ee4e8.l, a0
pub fn i_08296c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82972;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EE4E8;
    20
}
/// $082972: jsr $28cd4.l
pub fn i_082972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82978;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82978);
    cpu.pc = 0x28CD4;
    16
}
/// $082978: lea.l $8297e(pc), a1
pub fn i_082978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8297C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8297E;
    16
}
/// $08297C: move.l a1, (a6)
pub fn i_08297c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8297E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08297E: jsr $27bc8.l
pub fn i_08297e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82984;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82984);
    cpu.pc = 0x27BC8;
    16
}
/// $082984: bcc.w $8298e
pub fn i_082984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82988;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8298E; return 10; }
    8
}
/// $082988: lea.l $82456(pc), a1
pub fn i_082988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8298C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82456;
    16
}
/// $08298C: move.l a1, (a6)
pub fn i_08298c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8298E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08298E: jsr $28d70.l
pub fn i_08298e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82994;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82994);
    cpu.pc = 0x28D70;
    16
}
/// $082994: movea.l #$ffffffff, a0
pub fn i_082994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8299A;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $08299A: jsr $5dd56.l
pub fn i_08299a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x829A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x829A0);
    cpu.pc = 0x5DD56;
    16
}
/// $0829A0: bcc.w $829aa
pub fn i_0829a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x829A4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x829AA; return 10; }
    8
}
// ===== SetTaskHandler_0829a4  ($0829A4, 8 B, kernel/task_handlers.c)
/// $0829A4: lea.l $82456(pc), a1
pub fn i_0829a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x829A8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82456;
    16
}
/// $0829A8: move.l a1, (a6)
pub fn i_0829a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x829AA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0829AA: rts
pub fn i_0829aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x829AC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_0829ac  ($0829AC, 178 B, enemies/squad/para_squad_helpers_08283c.s)
/// $0829AC: move.w #$1b4, d1
pub fn i_0829ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x829B0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1B4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0829B0: jsr $236e.l
pub fn i_0829b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x829B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x829B6);
    cpu.pc = 0x236E;
    16
}
/// $0829B6: move.w $5c(a6), d0
pub fn i_0829b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x829BA;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0829BA: add.w d0, d0
pub fn i_0829ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x829BC;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0829BC: lea.l $2e588e.l, a1
pub fn i_0829bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x829C2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E588E;
    20
}
/// $0829C2: move.w (a1, d0.w), d0
pub fn i_0829c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x829C6;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $0829C6: btst.b #$0, $3a(a6)
pub fn i_0829c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x829CC;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0829CC: beq.w $829d2
pub fn i_0829cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x829D0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x829D2; return 10; }
    8
}
/// $0829D0: neg.w d0
pub fn i_0829d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x829D2;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0829D2: add.w $5e(a6), d0
pub fn i_0829d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x829D6;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x5E);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0829D6: move.w d0, $22(a6)
pub fn i_0829d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x829DA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0829DA: move.w #$e4c, d0
pub fn i_0829da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x829DE;
    cpu.ir = 0x303C;
    let v1: u32 = 0xE4C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0829DE: move.w #$68, d1
pub fn i_0829de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x829E2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x68;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0829E2: jsr $440d0.l
pub fn i_0829e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x829E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x829E8);
    cpu.pc = 0x440D0;
    16
}
/// $0829E8: move.w d1, $24(a6)
pub fn i_0829e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x829EC;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0829EC: jsr $5e9b6.l
pub fn i_0829ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x829F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x829F2);
    cpu.pc = 0x5E9B6;
    16
}
/// $0829F2: andi.w #$7, d0
pub fn i_0829f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x829F6;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0829F6: subq.w #$4, d0
pub fn i_0829f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x829F8;
    cpu.ir = 0x5940;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0829F8: add.w d0, $22(a6)
pub fn i_0829f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x829FC;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0829FC: jsr $5e9b6.l
pub fn i_0829fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82A02);
    cpu.pc = 0x5E9B6;
    16
}
/// $082A02: andi.w #$3, d0
pub fn i_082a02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A06;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $082A06: sub.w d0, $24(a6)
pub fn i_082a06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A0A;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $082A0A: move.w #$8000, d0
pub fn i_082a0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A0E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $082A0E: jsr $28134.l
pub fn i_082a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82A14);
    cpu.pc = 0x28134;
    16
}
/// $082A14: andi.w #$ffe3, $38(a6)
pub fn i_082a14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A1A;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $082A1A: ori.w #$14, $38(a6)
pub fn i_082a1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A20;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $082A20: lea.l $2e687e.l, a0
pub fn i_082a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A26;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E687E;
    20
}
/// $082A26: jsr $28cd4.l
pub fn i_082a26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A2C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82A2C);
    cpu.pc = 0x28CD4;
    16
}
/// $082A2C: lea.l $82a32(pc), a1
pub fn i_082a2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A30;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82A32;
    16
}
/// $082A30: move.l a1, (a6)
pub fn i_082a30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A32;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082A32: jsr $2783a.l
pub fn i_082a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82A38);
    cpu.pc = 0x2783A;
    16
}
/// $082A38: jsr $28d70.l
pub fn i_082a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82A3E);
    cpu.pc = 0x28D70;
    16
}
/// $082A3E: bcc.w $82a48
pub fn i_082a3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A42;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x82A48; return 10; }
    8
}
/// $082A42: lea.l $82456(pc), a1
pub fn i_082a42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A46;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82456;
    16
}
/// $082A46: move.l a1, (a6)
pub fn i_082a46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A48;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082A48: jsr $283d8.l
pub fn i_082a48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82A4E);
    cpu.pc = 0x283D8;
    16
}
/// $082A4E: movea.l #$ffffffff, a0
pub fn i_082a4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A54;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $082A54: jsr $5dd56.l
pub fn i_082a54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A5A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82A5A);
    cpu.pc = 0x5DD56;
    16
}
/// $082A5A: bcc.w $82a64
pub fn i_082a5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A5E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x82A64; return 10; }
    8
}
// ===== SetTaskHandler_082a5e  ($082A5E, 8 B, kernel/task_handlers.c)
/// $082A5E: lea.l $82456(pc), a1
pub fn i_082a5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A62;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82456;
    16
}
/// $082A62: move.l a1, (a6)
pub fn i_082a62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A64;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082A64: rts
pub fn i_082a64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A66;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_082a66  ($082A66, 216 B, enemies/squad/para_squad_helpers_08283c.s)
/// $082A66: move.w $34(a6), d3
pub fn i_082a66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A6A;
    cpu.ir = 0x362E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $082A6A: add.w d3, d3
pub fn i_082a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A6C;
    cpu.ir = 0xD643;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $082A6C: move.w #$2000, d0
pub fn i_082a6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A70;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $082A70: lea.l $2c07ac.l, a1
pub fn i_082a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A76;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C07AC;
    20
}
/// $082A76: lea.l $2c072c.l, a2
pub fn i_082a76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A7C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2C072C;
    20
}
/// $082A7C: move.w (a1, d3.w), d1
pub fn i_082a7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A80;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $082A80: move.w (a2, d3.w), d2
pub fn i_082a80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A84;
    cpu.ir = 0x3432;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $082A84: muls.w d0, d1
pub fn i_082a84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A86;
    cpu.ir = 0xC3C0;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[1] as u16 as i16 as i32)) as u32;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $082A86: muls.w d0, d2
pub fn i_082a86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A88;
    cpu.ir = 0xC5C0;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[2] as u16 as i16 as i32)) as u32;
    cpu.d[2] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $082A88: asr.l #$8, d1
pub fn i_082a88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A8A;
    cpu.ir = 0xE081;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[1] = r;
    22
}
/// $082A8A: asr.l #$8, d2
pub fn i_082a8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A8C;
    cpu.ir = 0xE082;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[2] = r;
    22
}
/// $082A8C: btst.b #$0, $3a(a6)
pub fn i_082a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A92;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $082A92: beq.w $82a98
pub fn i_082a92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A96;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x82A98; return 10; }
    8
}
/// $082A96: neg.w d1
pub fn i_082a96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A98;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $082A98: move.w d1, $28(a6)
pub fn i_082a98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A9C;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $082A9C: move.w d2, $2a(a6)
pub fn i_082a9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82AA0;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $082AA0: clr.w $34(a6)
pub fn i_082aa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82AA4;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $082AA4: move.w $22(a6), $5e(a6)
pub fn i_082aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82AAA;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x5E);
    bus.write16(ad4, v3 as u16);
    20
}
/// $082AAA: move.w #$2, d1
pub fn i_082aaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82AAE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $082AAE: jsr $236e.l
pub fn i_082aae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82AB4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82AB4);
    cpu.pc = 0x236E;
    16
}
/// $082AB4: bset.b #$4, $6b(a6)
pub fn i_082ab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82ABA;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $082ABA: move.w #$d000, d0
pub fn i_082aba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82ABE;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $082ABE: jsr $28134.l
pub fn i_082abe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82AC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82AC4);
    cpu.pc = 0x28134;
    16
}
/// $082AC4: andi.w #$ffe3, $38(a6)
pub fn i_082ac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82ACA;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $082ACA: ori.w #$14, $38(a6)
pub fn i_082aca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82AD0;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $082AD0: lea.l $2e68f0.l, a0
pub fn i_082ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82AD6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E68F0;
    20
}
/// $082AD6: jsr $28cd4.l
pub fn i_082ad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82ADC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82ADC);
    cpu.pc = 0x28CD4;
    16
}
/// $082ADC: lea.l $82ae2(pc), a1
pub fn i_082adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82AE0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82AE2;
    16
}
/// $082AE0: move.l a1, (a6)
pub fn i_082ae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82AE2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082AE2: jsr $27cee.l
pub fn i_082ae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82AE8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82AE8);
    cpu.pc = 0x27CEE;
    16
}
/// $082AE8: bcc.w $82af4
pub fn i_082ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82AEC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x82AF4; return 10; }
    8
}
/// $082AEC: lea.l $31d26.l, a1
pub fn i_082aec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82AF2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x31D26;
    20
}
/// $082AF2: move.l a1, (a6)
pub fn i_082af2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82AF4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082AF4: jsr $28d70.l
pub fn i_082af4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82AFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82AFA);
    cpu.pc = 0x28D70;
    16
}
/// $082AFA: move.w #$e4c, d0
pub fn i_082afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82AFE;
    cpu.ir = 0x303C;
    let v1: u32 = 0xE4C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $082AFE: move.w #$68, d1
pub fn i_082afe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B02;
    cpu.ir = 0x323C;
    let v1: u32 = 0x68;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $082B02: jsr $440d0.l
pub fn i_082b02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82B08);
    cpu.pc = 0x440D0;
    16
}
/// $082B08: cmp.w $24(a6), d1
pub fn i_082b08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B0C;
    cpu.ir = 0xB26E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $082B0C: bgt.w $82b16
pub fn i_082b0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B10;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x82B16; return 10; }
    8
}
/// $082B10: lea.l $829ac(pc), a1
pub fn i_082b10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B14;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x829AC;
    16
}
/// $082B14: move.l a1, (a6)
pub fn i_082b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B16;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082B16: jsr $283d8.l
pub fn i_082b16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82B1C);
    cpu.pc = 0x283D8;
    16
}
/// $082B1C: btst.b #$1, $13(a6)
pub fn i_082b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B22;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $082B22: beq.w $82b2e
pub fn i_082b22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B26;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x82B2E; return 10; }
    8
}
/// $082B26: lea.l $31d26.l, a1
pub fn i_082b26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B2C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x31D26;
    20
}
/// $082B2C: move.l a1, (a6)
pub fn i_082b2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B2E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082B2E: movea.l #$ffffffff, a0
pub fn i_082b2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B34;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $082B34: jsr $5dd56.l
pub fn i_082b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82B3A);
    cpu.pc = 0x5DD56;
    16
}
/// $082B3A: bcc.w $82b44
pub fn i_082b3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B3E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x82B44; return 10; }
    8
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x080000, i_080000),
    (0x080006, i_080006),
    (0x08000A, i_08000a),
    (0x080010, i_080010),
    (0x080012, i_080012),
    (0x080016, i_080016),
    (0x080018, i_080018),
    (0x08001C, i_08001c),
    (0x08001E, i_08001e),
    (0x080022, i_080022),
    (0x080024, i_080024),
    (0x080026, i_080026),
    (0x08002A, i_08002a),
    (0x08002E, i_08002e),
    (0x080032, i_080032),
    (0x080034, i_080034),
    (0x080038, i_080038),
    (0x08003E, i_08003e),
    (0x080042, i_080042),
    (0x080048, i_080048),
    (0x08004C, i_08004c),
    (0x08004E, i_08004e),
    (0x080052, i_080052),
    (0x080054, i_080054),
    (0x080058, i_080058),
    (0x08005C, i_08005c),
    (0x080060, i_080060),
    (0x080066, i_080066),
    (0x08006A, i_08006a),
    (0x08006E, i_08006e),
    (0x080072, i_080072),
    (0x080076, i_080076),
    (0x080078, i_080078),
    (0x08007C, i_08007c),
    (0x080082, i_080082),
    (0x080088, i_080088),
    (0x08008A, i_08008a),
    (0x08008E, i_08008e),
    (0x080092, i_080092),
    (0x080096, i_080096),
    (0x08009A, i_08009a),
    (0x08009E, i_08009e),
    (0x0800A0, i_0800a0),
    (0x0800A4, i_0800a4),
    (0x0800A6, i_0800a6),
    (0x0800A8, i_0800a8),
    (0x0800AC, i_0800ac),
    (0x0800B0, i_0800b0),
    (0x0800B6, i_0800b6),
    (0x0800BA, i_0800ba),
    (0x0800BC, i_0800bc),
    (0x0800C0, i_0800c0),
    (0x0800C4, i_0800c4),
    (0x0800C8, i_0800c8),
    (0x0800CA, i_0800ca),
    (0x0800CC, i_0800cc),
    (0x0800CE, i_0800ce),
    (0x0800D0, i_0800d0),
    (0x0800D4, i_0800d4),
    (0x0800D8, i_0800d8),
    (0x0800DE, i_0800de),
    (0x0800E4, i_0800e4),
    (0x0800E8, i_0800e8),
    (0x0800EC, i_0800ec),
    (0x0800F0, i_0800f0),
    (0x0800F2, i_0800f2),
    (0x0800F6, i_0800f6),
    (0x0800FA, i_0800fa),
    (0x0800FC, i_0800fc),
    (0x080100, i_080100),
    (0x080104, i_080104),
    (0x080108, i_080108),
    (0x08010C, i_08010c),
    (0x08010E, i_08010e),
    (0x0801E6, i_0801e6),
    (0x0801EA, i_0801ea),
    (0x0801EC, i_0801ec),
    (0x080226, i_080226),
    (0x08022A, i_08022a),
    (0x08022C, i_08022c),
    (0x08022E, i_08022e),
    (0x080232, i_080232),
    (0x080236, i_080236),
    (0x08023C, i_08023c),
    (0x080240, i_080240),
    (0x080244, i_080244),
    (0x080248, i_080248),
    (0x08024E, i_08024e),
    (0x080252, i_080252),
    (0x080256, i_080256),
    (0x08025A, i_08025a),
    (0x080260, i_080260),
    (0x080264, i_080264),
    (0x080266, i_080266),
    (0x08026C, i_08026c),
    (0x080270, i_080270),
    (0x080274, i_080274),
    (0x08027A, i_08027a),
    (0x08027E, i_08027e),
    (0x080284, i_080284),
    (0x080288, i_080288),
    (0x08028E, i_08028e),
    (0x080294, i_080294),
    (0x08029A, i_08029a),
    (0x0802A0, i_0802a0),
    (0x0802A6, i_0802a6),
    (0x0802AA, i_0802aa),
    (0x0802AE, i_0802ae),
    (0x0802B2, i_0802b2),
    (0x0802B8, i_0802b8),
    (0x0802BC, i_0802bc),
    (0x0802C2, i_0802c2),
    (0x0802C8, i_0802c8),
    (0x0802CE, i_0802ce),
    (0x0802D4, i_0802d4),
    (0x0802D8, i_0802d8),
    (0x0802DC, i_0802dc),
    (0x0802DE, i_0802de),
    (0x0802E0, i_0802e0),
    (0x0802E4, i_0802e4),
    (0x0802EA, i_0802ea),
    (0x0802F0, i_0802f0),
    (0x0802F6, i_0802f6),
    (0x0802FA, i_0802fa),
    (0x0802FC, i_0802fc),
    (0x080302, i_080302),
    (0x080308, i_080308),
    (0x08030E, i_08030e),
    (0x080314, i_080314),
    (0x08031A, i_08031a),
    (0x080320, i_080320),
    (0x080324, i_080324),
    (0x08032A, i_08032a),
    (0x080330, i_080330),
    (0x080336, i_080336),
    (0x08033C, i_08033c),
    (0x080340, i_080340),
    (0x080342, i_080342),
    (0x080344, i_080344),
    (0x08034A, i_08034a),
    (0x08034C, i_08034c),
    (0x080350, i_080350),
    (0x080354, i_080354),
    (0x080358, i_080358),
    (0x08035C, i_08035c),
    (0x080362, i_080362),
    (0x080368, i_080368),
    (0x08036E, i_08036e),
    (0x080374, i_080374),
    (0x08037A, i_08037a),
    (0x08037E, i_08037e),
    (0x080380, i_080380),
    (0x080382, i_080382),
    (0x080386, i_080386),
    (0x080388, i_080388),
    (0x08038C, i_08038c),
    (0x080390, i_080390),
    (0x080394, i_080394),
    (0x08039A, i_08039a),
    (0x08039E, i_08039e),
    (0x0803A0, i_0803a0),
    (0x0803A4, i_0803a4),
    (0x0803AA, i_0803aa),
    (0x0803AE, i_0803ae),
    (0x0803B2, i_0803b2),
    (0x0803B4, i_0803b4),
    (0x0803BA, i_0803ba),
    (0x0803C0, i_0803c0),
    (0x0803C6, i_0803c6),
    (0x0803CA, i_0803ca),
    (0x0803CE, i_0803ce),
    (0x0803D0, i_0803d0),
    (0x0803D2, i_0803d2),
    (0x0803D6, i_0803d6),
    (0x0803DA, i_0803da),
    (0x0803E0, i_0803e0),
    (0x0803E8, i_0803e8),
    (0x0803EC, i_0803ec),
    (0x0803F0, i_0803f0),
    (0x0803F4, i_0803f4),
    (0x0803F8, i_0803f8),
    (0x0803FC, i_0803fc),
    (0x080400, i_080400),
    (0x080404, i_080404),
    (0x080408, i_080408),
    (0x08040C, i_08040c),
    (0x08040E, i_08040e),
    (0x080412, i_080412),
    (0x080416, i_080416),
    (0x08041A, i_08041a),
    (0x08041E, i_08041e),
    (0x080424, i_080424),
    (0x08042A, i_08042a),
    (0x08042E, i_08042e),
    (0x080434, i_080434),
    (0x08043A, i_08043a),
    (0x08043E, i_08043e),
    (0x080444, i_080444),
    (0x08044A, i_08044a),
    (0x08044E, i_08044e),
    (0x080452, i_080452),
    (0x080454, i_080454),
    (0x080458, i_080458),
    (0x08045C, i_08045c),
    (0x080462, i_080462),
    (0x080466, i_080466),
    (0x08046A, i_08046a),
    (0x08046E, i_08046e),
    (0x080472, i_080472),
    (0x080476, i_080476),
    (0x08047C, i_08047c),
    (0x08047E, i_08047e),
    (0x080482, i_080482),
    (0x080486, i_080486),
    (0x08048A, i_08048a),
    (0x08048E, i_08048e),
    (0x080490, i_080490),
    (0x080494, i_080494),
    (0x080496, i_080496),
    (0x080498, i_080498),
    (0x08049C, i_08049c),
    (0x0804A0, i_0804a0),
    (0x0804A2, i_0804a2),
    (0x0804A6, i_0804a6),
    (0x0804A8, i_0804a8),
    (0x0804AC, i_0804ac),
    (0x0804AE, i_0804ae),
    (0x0804B2, i_0804b2),
    (0x0804B8, i_0804b8),
    (0x0804BE, i_0804be),
    (0x0804C0, i_0804c0),
    (0x0804C4, i_0804c4),
    (0x0804C8, i_0804c8),
    (0x0804CC, i_0804cc),
    (0x0804CE, i_0804ce),
    (0x0804D2, i_0804d2),
    (0x0804D8, i_0804d8),
    (0x0804DC, i_0804dc),
    (0x0804E2, i_0804e2),
    (0x0804E8, i_0804e8),
    (0x0804EC, i_0804ec),
    (0x0804F0, i_0804f0),
    (0x0804F4, i_0804f4),
    (0x0804F8, i_0804f8),
    (0x0804FC, i_0804fc),
    (0x080500, i_080500),
    (0x080504, i_080504),
    (0x080506, i_080506),
    (0x080508, i_080508),
    (0x08050C, i_08050c),
    (0x080510, i_080510),
    (0x080516, i_080516),
    (0x08051A, i_08051a),
    (0x08051E, i_08051e),
    (0x080522, i_080522),
    (0x080526, i_080526),
    (0x08052A, i_08052a),
    (0x08052E, i_08052e),
    (0x080532, i_080532),
    (0x080538, i_080538),
    (0x08053C, i_08053c),
    (0x08053E, i_08053e),
    (0x080540, i_080540),
    (0x080544, i_080544),
    (0x08054A, i_08054a),
    (0x080550, i_080550),
    (0x080556, i_080556),
    (0x08055C, i_08055c),
    (0x080560, i_080560),
    (0x080566, i_080566),
    (0x08056C, i_08056c),
    (0x080572, i_080572),
    (0x08057A, i_08057a),
    (0x08057E, i_08057e),
    (0x080582, i_080582),
    (0x080586, i_080586),
    (0x08058C, i_08058c),
    (0x080592, i_080592),
    (0x080598, i_080598),
    (0x08059E, i_08059e),
    (0x0805A2, i_0805a2),
    (0x0805A4, i_0805a4),
    (0x0805A8, i_0805a8),
    (0x0805AC, i_0805ac),
    (0x0805AE, i_0805ae),
    (0x0805B4, i_0805b4),
    (0x0805BA, i_0805ba),
    (0x0805C0, i_0805c0),
    (0x0805C6, i_0805c6),
    (0x0805CA, i_0805ca),
    (0x0805D0, i_0805d0),
    (0x0805D6, i_0805d6),
    (0x0805DA, i_0805da),
    (0x0805E0, i_0805e0),
    (0x0805E6, i_0805e6),
    (0x0805EA, i_0805ea),
    (0x0805EC, i_0805ec),
    (0x0805EE, i_0805ee),
    (0x0805F2, i_0805f2),
    (0x0805F4, i_0805f4),
    (0x0805F8, i_0805f8),
    (0x0805FC, i_0805fc),
    (0x080602, i_080602),
    (0x080606, i_080606),
    (0x080608, i_080608),
    (0x08060C, i_08060c),
    (0x080612, i_080612),
    (0x080616, i_080616),
    (0x08061A, i_08061a),
    (0x08061C, i_08061c),
    (0x080622, i_080622),
    (0x080628, i_080628),
    (0x08062C, i_08062c),
    (0x080632, i_080632),
    (0x080636, i_080636),
    (0x08063A, i_08063a),
    (0x080640, i_080640),
    (0x080646, i_080646),
    (0x08064C, i_08064c),
    (0x080652, i_080652),
    (0x080658, i_080658),
    (0x08065C, i_08065c),
    (0x080660, i_080660),
    (0x080666, i_080666),
    (0x08066C, i_08066c),
    (0x080672, i_080672),
    (0x080676, i_080676),
    (0x08067A, i_08067a),
    (0x08067C, i_08067c),
    (0x08067E, i_08067e),
    (0x080682, i_080682),
    (0x080688, i_080688),
    (0x08068C, i_08068c),
    (0x080692, i_080692),
    (0x080696, i_080696),
    (0x08069C, i_08069c),
    (0x0806A2, i_0806a2),
    (0x0806A8, i_0806a8),
    (0x0806AE, i_0806ae),
    (0x0806B4, i_0806b4),
    (0x0806B8, i_0806b8),
    (0x0806BE, i_0806be),
    (0x0806C4, i_0806c4),
    (0x0806C8, i_0806c8),
    (0x0806CA, i_0806ca),
    (0x0806D0, i_0806d0),
    (0x0806D4, i_0806d4),
    (0x0806D8, i_0806d8),
    (0x0806DC, i_0806dc),
    (0x0806E2, i_0806e2),
    (0x0806E8, i_0806e8),
    (0x0806EC, i_0806ec),
    (0x0806F0, i_0806f0),
    (0x0806F4, i_0806f4),
    (0x0806F6, i_0806f6),
    (0x0806FA, i_0806fa),
    (0x0806FE, i_0806fe),
    (0x080702, i_080702),
    (0x080704, i_080704),
    (0x080708, i_080708),
    (0x08070E, i_08070e),
    (0x080712, i_080712),
    (0x080716, i_080716),
    (0x080718, i_080718),
    (0x08071E, i_08071e),
    (0x080724, i_080724),
    (0x08072A, i_08072a),
    (0x08072E, i_08072e),
    (0x080732, i_080732),
    (0x080734, i_080734),
    (0x080736, i_080736),
    (0x08073C, i_08073c),
    (0x080740, i_080740),
    (0x080744, i_080744),
    (0x080746, i_080746),
    (0x08074C, i_08074c),
    (0x080750, i_080750),
    (0x080756, i_080756),
    (0x08075A, i_08075a),
    (0x08075E, i_08075e),
    (0x080760, i_080760),
    (0x080764, i_080764),
    (0x08076A, i_08076a),
    (0x08076E, i_08076e),
    (0x080772, i_080772),
    (0x080778, i_080778),
    (0x08077E, i_08077e),
    (0x080782, i_080782),
    (0x080784, i_080784),
    (0x08078A, i_08078a),
    (0x08078E, i_08078e),
    (0x080792, i_080792),
    (0x080798, i_080798),
    (0x08079E, i_08079e),
    (0x0807A2, i_0807a2),
    (0x0807A6, i_0807a6),
    (0x0807A8, i_0807a8),
    (0x080898, i_080898),
    (0x08089C, i_08089c),
    (0x08089E, i_08089e),
    (0x0808A0, i_0808a0),
    (0x0808A4, i_0808a4),
    (0x0808A8, i_0808a8),
    (0x0808AE, i_0808ae),
    (0x0808B0, i_0808b0),
    (0x0808B6, i_0808b6),
    (0x0808B8, i_0808b8),
    (0x0808BC, i_0808bc),
    (0x0808C0, i_0808c0),
    (0x0808C4, i_0808c4),
    (0x0808CA, i_0808ca),
    (0x0808CE, i_0808ce),
    (0x0808D0, i_0808d0),
    (0x0808D4, i_0808d4),
    (0x0808D6, i_0808d6),
    (0x0808DC, i_0808dc),
    (0x0808E0, i_0808e0),
    (0x0808E2, i_0808e2),
    (0x0808E6, i_0808e6),
    (0x0808E8, i_0808e8),
    (0x0808EC, i_0808ec),
    (0x0808F0, i_0808f0),
    (0x0808F4, i_0808f4),
    (0x0808F8, i_0808f8),
    (0x0808FC, i_0808fc),
    (0x080900, i_080900),
    (0x080904, i_080904),
    (0x080908, i_080908),
    (0x08090E, i_08090e),
    (0x080912, i_080912),
    (0x080916, i_080916),
    (0x08091C, i_08091c),
    (0x080920, i_080920),
    (0x080926, i_080926),
    (0x08092A, i_08092a),
    (0x08092C, i_08092c),
    (0x080930, i_080930),
    (0x080932, i_080932),
    (0x080936, i_080936),
    (0x08093A, i_08093a),
    (0x08093E, i_08093e),
    (0x080942, i_080942),
    (0x080944, i_080944),
    (0x080948, i_080948),
    (0x08094A, i_08094a),
    (0x08094E, i_08094e),
    (0x080954, i_080954),
    (0x080958, i_080958),
    (0x08095C, i_08095c),
    (0x08095E, i_08095e),
    (0x080962, i_080962),
    (0x080968, i_080968),
    (0x08096C, i_08096c),
    (0x080970, i_080970),
    (0x080974, i_080974),
    (0x080978, i_080978),
    (0x08097C, i_08097c),
    (0x080980, i_080980),
    (0x080982, i_080982),
    (0x080986, i_080986),
    (0x08098A, i_08098a),
    (0x08098E, i_08098e),
    (0x080990, i_080990),
    (0x080994, i_080994),
    (0x080996, i_080996),
    (0x08099A, i_08099a),
    (0x0809A0, i_0809a0),
    (0x0809A6, i_0809a6),
    (0x0809AC, i_0809ac),
    (0x0809B0, i_0809b0),
    (0x0809B2, i_0809b2),
    (0x0809B8, i_0809b8),
    (0x0809BE, i_0809be),
    (0x0809C4, i_0809c4),
    (0x0809CA, i_0809ca),
    (0x0809CC, i_0809cc),
    (0x0809D0, i_0809d0),
    (0x0809D6, i_0809d6),
    (0x0809DC, i_0809dc),
    (0x0809E2, i_0809e2),
    (0x0809E8, i_0809e8),
    (0x0809EE, i_0809ee),
    (0x0809F2, i_0809f2),
    (0x0809F4, i_0809f4),
    (0x0809FA, i_0809fa),
    (0x0809FE, i_0809fe),
    (0x080A04, i_080a04),
    (0x080A0A, i_080a0a),
    (0x080A10, i_080a10),
    (0x080A16, i_080a16),
    (0x080A1A, i_080a1a),
    (0x080A20, i_080a20),
    (0x080A24, i_080a24),
    (0x080A28, i_080a28),
    (0x080A2E, i_080a2e),
    (0x080A32, i_080a32),
    (0x080A38, i_080a38),
    (0x080A3E, i_080a3e),
    (0x080A40, i_080a40),
    (0x080A44, i_080a44),
    (0x080A4A, i_080a4a),
    (0x080A50, i_080a50),
    (0x080A56, i_080a56),
    (0x080A5A, i_080a5a),
    (0x080A5C, i_080a5c),
    (0x080A62, i_080a62),
    (0x080A66, i_080a66),
    (0x080A6A, i_080a6a),
    (0x080A70, i_080a70),
    (0x080A76, i_080a76),
    (0x080A7C, i_080a7c),
    (0x080A80, i_080a80),
    (0x080A84, i_080a84),
    (0x080A8A, i_080a8a),
    (0x080A8E, i_080a8e),
    (0x080A92, i_080a92),
    (0x080A98, i_080a98),
    (0x080A9C, i_080a9c),
    (0x080AA2, i_080aa2),
    (0x080AA8, i_080aa8),
    (0x080AAA, i_080aaa),
    (0x080AAE, i_080aae),
    (0x080AB2, i_080ab2),
    (0x080AB8, i_080ab8),
    (0x080ABE, i_080abe),
    (0x080AC2, i_080ac2),
    (0x080AC8, i_080ac8),
    (0x080ACC, i_080acc),
    (0x080AD0, i_080ad0),
    (0x080AD4, i_080ad4),
    (0x080AD8, i_080ad8),
    (0x080ADC, i_080adc),
    (0x080ADE, i_080ade),
    (0x080AE2, i_080ae2),
    (0x080AE8, i_080ae8),
    (0x080AEC, i_080aec),
    (0x080AEE, i_080aee),
    (0x080AF2, i_080af2),
    (0x080AF8, i_080af8),
    (0x080AFA, i_080afa),
    (0x080AFC, i_080afc),
    (0x080B00, i_080b00),
    (0x080B02, i_080b02),
    (0x080B08, i_080b08),
    (0x080B0C, i_080b0c),
    (0x080B0E, i_080b0e),
    (0x080B10, i_080b10),
    (0x080B12, i_080b12),
    (0x080B16, i_080b16),
    (0x080B18, i_080b18),
    (0x080BCE, i_080bce),
    (0x080BD2, i_080bd2),
    (0x080BD4, i_080bd4),
    (0x080BD6, i_080bd6),
    (0x080BDE, i_080bde),
    (0x080BE4, i_080be4),
    (0x080BE8, i_080be8),
    (0x080BF0, i_080bf0),
    (0x080BF2, i_080bf2),
    (0x080BF6, i_080bf6),
    (0x080BFA, i_080bfa),
    (0x080BFE, i_080bfe),
    (0x080C02, i_080c02),
    (0x080C04, i_080c04),
    (0x080C68, i_080c68),
    (0x080C6C, i_080c6c),
    (0x080C72, i_080c72),
    (0x080C76, i_080c76),
    (0x080C7A, i_080c7a),
    (0x080C7C, i_080c7c),
    (0x080C82, i_080c82),
    (0x080C86, i_080c86),
    (0x080C8A, i_080c8a),
    (0x080C90, i_080c90),
    (0x080C94, i_080c94),
    (0x080C98, i_080c98),
    (0x080C9C, i_080c9c),
    (0x080CA0, i_080ca0),
    (0x080CA2, i_080ca2),
    (0x080CA6, i_080ca6),
    (0x080CAA, i_080caa),
    (0x080CAE, i_080cae),
    (0x080CB0, i_080cb0),
    (0x080CB4, i_080cb4),
    (0x080CB8, i_080cb8),
    (0x080CBC, i_080cbc),
    (0x080CBE, i_080cbe),
    (0x080CC4, i_080cc4),
    (0x080CCA, i_080cca),
    (0x080CD0, i_080cd0),
    (0x080CD4, i_080cd4),
    (0x080CD8, i_080cd8),
    (0x080CDA, i_080cda),
    (0x080CDC, i_080cdc),
    (0x080CE4, i_080ce4),
    (0x080CEA, i_080cea),
    (0x080CEE, i_080cee),
    (0x080CF6, i_080cf6),
    (0x080CF8, i_080cf8),
    (0x080CFC, i_080cfc),
    (0x080D00, i_080d00),
    (0x080D04, i_080d04),
    (0x080D08, i_080d08),
    (0x080D0C, i_080d0c),
    (0x080D0E, i_080d0e),
    (0x080D12, i_080d12),
    (0x080D16, i_080d16),
    (0x080D1A, i_080d1a),
    (0x080D1C, i_080d1c),
    (0x080D22, i_080d22),
    (0x080D26, i_080d26),
    (0x080D28, i_080d28),
    (0x080D2C, i_080d2c),
    (0x080D30, i_080d30),
    (0x080D36, i_080d36),
    (0x080D3A, i_080d3a),
    (0x080D3E, i_080d3e),
    (0x080D40, i_080d40),
    (0x080D46, i_080d46),
    (0x080D4A, i_080d4a),
    (0x080D4E, i_080d4e),
    (0x080D54, i_080d54),
    (0x080D58, i_080d58),
    (0x080D5C, i_080d5c),
    (0x080D60, i_080d60),
    (0x080D64, i_080d64),
    (0x080D66, i_080d66),
    (0x080D6A, i_080d6a),
    (0x080D6E, i_080d6e),
    (0x080D72, i_080d72),
    (0x080D74, i_080d74),
    (0x080D7A, i_080d7a),
    (0x080D80, i_080d80),
    (0x080D86, i_080d86),
    (0x080D8A, i_080d8a),
    (0x080D8E, i_080d8e),
    (0x080D90, i_080d90),
    (0x080D92, i_080d92),
    (0x080D98, i_080d98),
    (0x080D9C, i_080d9c),
    (0x080DA0, i_080da0),
    (0x080DA2, i_080da2),
    (0x080E40, i_080e40),
    (0x080E44, i_080e44),
    (0x080E46, i_080e46),
    (0x080E48, i_080e48),
    (0x080E4E, i_080e4e),
    (0x080E54, i_080e54),
    (0x080E5A, i_080e5a),
    (0x080E5E, i_080e5e),
    (0x080E64, i_080e64),
    (0x080E6A, i_080e6a),
    (0x080E6C, i_080e6c),
    (0x080E70, i_080e70),
    (0x080E76, i_080e76),
    (0x080E7A, i_080e7a),
    (0x080E7C, i_080e7c),
    (0x080E80, i_080e80),
    (0x080E84, i_080e84),
    (0x080E86, i_080e86),
    (0x080E88, i_080e88),
    (0x080E8C, i_080e8c),
    (0x080E90, i_080e90),
    (0x080E92, i_080e92),
    (0x080E94, i_080e94),
    (0x080E96, i_080e96),
    (0x080E9C, i_080e9c),
    (0x080EA0, i_080ea0),
    (0x080EA4, i_080ea4),
    (0x080EA8, i_080ea8),
    (0x080EAA, i_080eaa),
    (0x080EB0, i_080eb0),
    (0x080EB4, i_080eb4),
    (0x080EB8, i_080eb8),
    (0x080EBA, i_080eba),
    (0x080EC0, i_080ec0),
    (0x080EC4, i_080ec4),
    (0x080EC8, i_080ec8),
    (0x080ECC, i_080ecc),
    (0x080ECE, i_080ece),
    (0x080ED2, i_080ed2),
    (0x080ED6, i_080ed6),
    (0x080EDA, i_080eda),
    (0x080EDC, i_080edc),
    (0x080EE2, i_080ee2),
    (0x080EE8, i_080ee8),
    (0x080EEE, i_080eee),
    (0x080EF2, i_080ef2),
    (0x080EF6, i_080ef6),
    (0x080EF8, i_080ef8),
    (0x080EFA, i_080efa),
    (0x080F00, i_080f00),
    (0x080F04, i_080f04),
    (0x080F0A, i_080f0a),
    (0x080F0E, i_080f0e),
    (0x080F14, i_080f14),
    (0x080F18, i_080f18),
    (0x080F1C, i_080f1c),
    (0x080F22, i_080f22),
    (0x080F26, i_080f26),
    (0x080F2A, i_080f2a),
    (0x080F2C, i_080f2c),
    (0x080F30, i_080f30),
    (0x080F32, i_080f32),
    (0x080F38, i_080f38),
    (0x080F68, i_080f68),
    (0x080F6E, i_080f6e),
    (0x080F74, i_080f74),
    (0x080F7A, i_080f7a),
    (0x080F80, i_080f80),
    (0x080F86, i_080f86),
    (0x080F8C, i_080f8c),
    (0x080F92, i_080f92),
    (0x080F98, i_080f98),
    (0x080F9E, i_080f9e),
    (0x080FA2, i_080fa2),
    (0x080FA8, i_080fa8),
    (0x080FAE, i_080fae),
    (0x080FB4, i_080fb4),
    (0x080FBA, i_080fba),
    (0x080FBE, i_080fbe),
    (0x080FC4, i_080fc4),
    (0x080FCA, i_080fca),
    (0x080FD0, i_080fd0),
    (0x080FD6, i_080fd6),
    (0x080FDC, i_080fdc),
    (0x080FE0, i_080fe0),
    (0x080FE6, i_080fe6),
    (0x080FEC, i_080fec),
    (0x080FF2, i_080ff2),
    (0x080FF8, i_080ff8),
    (0x080FFC, i_080ffc),
    (0x081002, i_081002),
    (0x081008, i_081008),
    (0x08100E, i_08100e),
    (0x081014, i_081014),
    (0x0811BE, i_0811be),
    (0x0811C4, i_0811c4),
    (0x0811CC, i_0811cc),
    (0x0811D0, i_0811d0),
    (0x0811D6, i_0811d6),
    (0x0811DC, i_0811dc),
    (0x081214, i_081214),
    (0x08121C, i_08121c),
    (0x081222, i_081222),
    (0x081226, i_081226),
    (0x08122A, i_08122a),
    (0x081230, i_081230),
    (0x081234, i_081234),
    (0x081238, i_081238),
    (0x08123C, i_08123c),
    (0x081242, i_081242),
    (0x081246, i_081246),
    (0x08124A, i_08124a),
    (0x08124E, i_08124e),
    (0x081254, i_081254),
    (0x081258, i_081258),
    (0x08125C, i_08125c),
    (0x081260, i_081260),
    (0x081266, i_081266),
    (0x08126A, i_08126a),
    (0x08126E, i_08126e),
    (0x081272, i_081272),
    (0x081278, i_081278),
    (0x08127C, i_08127c),
    (0x081280, i_081280),
    (0x081284, i_081284),
    (0x08128A, i_08128a),
    (0x08128E, i_08128e),
    (0x081292, i_081292),
    (0x081296, i_081296),
    (0x08129C, i_08129c),
    (0x0812A0, i_0812a0),
    (0x0812A4, i_0812a4),
    (0x0812AA, i_0812aa),
    (0x0812B0, i_0812b0),
    (0x0812B6, i_0812b6),
    (0x0812BA, i_0812ba),
    (0x0812BC, i_0812bc),
    (0x0812BE, i_0812be),
    (0x0812C4, i_0812c4),
    (0x0812C8, i_0812c8),
    (0x0812CC, i_0812cc),
    (0x0812D2, i_0812d2),
    (0x0812D6, i_0812d6),
    (0x0812D8, i_0812d8),
    (0x0812DA, i_0812da),
    (0x0812DE, i_0812de),
    (0x0812E0, i_0812e0),
    (0x0812E2, i_0812e2),
    (0x0812E6, i_0812e6),
    (0x0812EA, i_0812ea),
    (0x0812EC, i_0812ec),
    (0x0812EE, i_0812ee),
    (0x0812F2, i_0812f2),
    (0x0812F4, i_0812f4),
    (0x0812F6, i_0812f6),
    (0x0812FA, i_0812fa),
    (0x081300, i_081300),
    (0x081304, i_081304),
    (0x08130A, i_08130a),
    (0x081310, i_081310),
    (0x081316, i_081316),
    (0x08131C, i_08131c),
    (0x081322, i_081322),
    (0x081326, i_081326),
    (0x08132A, i_08132a),
    (0x081330, i_081330),
    (0x081344, i_081344),
    (0x081348, i_081348),
    (0x08134A, i_08134a),
    (0x081632, i_081632),
    (0x081636, i_081636),
    (0x081638, i_081638),
    (0x08163A, i_08163a),
    (0x08163E, i_08163e),
    (0x081644, i_081644),
    (0x08164A, i_08164a),
    (0x081650, i_081650),
    (0x081654, i_081654),
    (0x081656, i_081656),
    (0x08165C, i_08165c),
    (0x081662, i_081662),
    (0x081666, i_081666),
    (0x08166A, i_08166a),
    (0x08166C, i_08166c),
    (0x08166E, i_08166e),
    (0x081672, i_081672),
    (0x081678, i_081678),
    (0x08167C, i_08167c),
    (0x081682, i_081682),
    (0x081688, i_081688),
    (0x08168E, i_08168e),
    (0x081694, i_081694),
    (0x08169A, i_08169a),
    (0x0816A0, i_0816a0),
    (0x0816A4, i_0816a4),
    (0x0816A6, i_0816a6),
    (0x0816AC, i_0816ac),
    (0x0816B2, i_0816b2),
    (0x0816B6, i_0816b6),
    (0x0816BC, i_0816bc),
    (0x0816C2, i_0816c2),
    (0x0816C6, i_0816c6),
    (0x0816CC, i_0816cc),
    (0x0816D0, i_0816d0),
    (0x0816D2, i_0816d2),
    (0x0816D4, i_0816d4),
    (0x0816D8, i_0816d8),
    (0x0816DC, i_0816dc),
    (0x0816E0, i_0816e0),
    (0x0816E4, i_0816e4),
    (0x0816E6, i_0816e6),
    (0x0816EC, i_0816ec),
    (0x0816EE, i_0816ee),
    (0x0816F2, i_0816f2),
    (0x0816F4, i_0816f4),
    (0x0816F8, i_0816f8),
    (0x0816FC, i_0816fc),
    (0x0816FE, i_0816fe),
    (0x081702, i_081702),
    (0x081704, i_081704),
    (0x081706, i_081706),
    (0x08170C, i_08170c),
    (0x081710, i_081710),
    (0x081714, i_081714),
    (0x081718, i_081718),
    (0x08171C, i_08171c),
    (0x081720, i_081720),
    (0x081724, i_081724),
    (0x081726, i_081726),
    (0x081728, i_081728),
    (0x08172E, i_08172e),
    (0x081734, i_081734),
    (0x081736, i_081736),
    (0x08173A, i_08173a),
    (0x08173E, i_08173e),
    (0x081744, i_081744),
    (0x081748, i_081748),
    (0x08174E, i_08174e),
    (0x081752, i_081752),
    (0x081756, i_081756),
    (0x08175C, i_08175c),
    (0x08179E, i_08179e),
    (0x0817A2, i_0817a2),
    (0x0817A6, i_0817a6),
    (0x0817AC, i_0817ac),
    (0x0817AE, i_0817ae),
    (0x0817B0, i_0817b0),
    (0x0817B2, i_0817b2),
    (0x0817B4, i_0817b4),
    (0x0817B8, i_0817b8),
    (0x0817BA, i_0817ba),
    (0x0817BC, i_0817bc),
    (0x0817C0, i_0817c0),
    (0x0817C4, i_0817c4),
    (0x0817C8, i_0817c8),
    (0x0817CA, i_0817ca),
    (0x0817CE, i_0817ce),
    (0x0817D0, i_0817d0),
    (0x0817D4, i_0817d4),
    (0x0817E4, i_0817e4),
    (0x0817EA, i_0817ea),
    (0x0817F2, i_0817f2),
    (0x0817F8, i_0817f8),
    (0x081800, i_081800),
    (0x081806, i_081806),
    (0x08180E, i_08180e),
    (0x081814, i_081814),
    (0x08181C, i_08181c),
    (0x081822, i_081822),
    (0x081832, i_081832),
    (0x081838, i_081838),
    (0x081840, i_081840),
    (0x081846, i_081846),
    (0x081856, i_081856),
    (0x08185C, i_08185c),
    (0x081864, i_081864),
    (0x08186A, i_08186a),
    (0x08186C, i_08186c),
    (0x081870, i_081870),
    (0x081872, i_081872),
    (0x081878, i_081878),
    (0x08187C, i_08187c),
    (0x08187E, i_08187e),
    (0x081882, i_081882),
    (0x081884, i_081884),
    (0x081888, i_081888),
    (0x08188A, i_08188a),
    (0x081890, i_081890),
    (0x081894, i_081894),
    (0x081896, i_081896),
    (0x08189A, i_08189a),
    (0x08189C, i_08189c),
    (0x0818A2, i_0818a2),
    (0x0818A8, i_0818a8),
    (0x0818AA, i_0818aa),
    (0x0818AE, i_0818ae),
    (0x0818B4, i_0818b4),
    (0x0818BA, i_0818ba),
    (0x0818BE, i_0818be),
    (0x0818C4, i_0818c4),
    (0x0818CA, i_0818ca),
    (0x0818CE, i_0818ce),
    (0x0818D4, i_0818d4),
    (0x0818DA, i_0818da),
    (0x0818DE, i_0818de),
    (0x0818E4, i_0818e4),
    (0x0818EA, i_0818ea),
    (0x0818EC, i_0818ec),
    (0x0818F0, i_0818f0),
    (0x0818F4, i_0818f4),
    (0x0818F8, i_0818f8),
    (0x0818FC, i_0818fc),
    (0x081900, i_081900),
    (0x081902, i_081902),
    (0x081906, i_081906),
    (0x081908, i_081908),
    (0x08190C, i_08190c),
    (0x081912, i_081912),
    (0x081918, i_081918),
    (0x08191C, i_08191c),
    (0x081922, i_081922),
    (0x081926, i_081926),
    (0x08192C, i_08192c),
    (0x081932, i_081932),
    (0x081936, i_081936),
    (0x08193C, i_08193c),
    (0x081942, i_081942),
    (0x081946, i_081946),
    (0x08194C, i_08194c),
    (0x081952, i_081952),
    (0x081958, i_081958),
    (0x08195C, i_08195c),
    (0x081962, i_081962),
    (0x081968, i_081968),
    (0x08196C, i_08196c),
    (0x081972, i_081972),
    (0x081976, i_081976),
    (0x081978, i_081978),
    (0x08197C, i_08197c),
    (0x081980, i_081980),
    (0x081984, i_081984),
    (0x081988, i_081988),
    (0x08198C, i_08198c),
    (0x081992, i_081992),
    (0x08199A, i_08199a),
    (0x08199E, i_08199e),
    (0x0819A4, i_0819a4),
    (0x0819AA, i_0819aa),
    (0x0819B0, i_0819b0),
    (0x0819B4, i_0819b4),
    (0x0819BA, i_0819ba),
    (0x0819C0, i_0819c0),
    (0x0819C6, i_0819c6),
    (0x0819CC, i_0819cc),
    (0x0819D2, i_0819d2),
    (0x0819D6, i_0819d6),
    (0x0819DC, i_0819dc),
    (0x0819E0, i_0819e0),
    (0x0819E4, i_0819e4),
    (0x0819E8, i_0819e8),
    (0x0819EE, i_0819ee),
    (0x0819F0, i_0819f0),
    (0x0819F4, i_0819f4),
    (0x0819FA, i_0819fa),
    (0x0819FE, i_0819fe),
    (0x081A04, i_081a04),
    (0x081A08, i_081a08),
    (0x081A0A, i_081a0a),
    (0x081A10, i_081a10),
    (0x081A14, i_081a14),
    (0x081A18, i_081a18),
    (0x081A1E, i_081a1e),
    (0x081A22, i_081a22),
    (0x081A26, i_081a26),
    (0x081A2A, i_081a2a),
    (0x081A2C, i_081a2c),
    (0x081CA4, i_081ca4),
    (0x081CAA, i_081caa),
    (0x081CAC, i_081cac),
    (0x081CB0, i_081cb0),
    (0x081CB4, i_081cb4),
    (0x081CBA, i_081cba),
    (0x081CBE, i_081cbe),
    (0x081CC4, i_081cc4),
    (0x081CCA, i_081cca),
    (0x081CCE, i_081cce),
    (0x081CD2, i_081cd2),
    (0x081CD4, i_081cd4),
    (0x081CD6, i_081cd6),
    (0x081CDA, i_081cda),
    (0x081CE0, i_081ce0),
    (0x081CE6, i_081ce6),
    (0x081CEC, i_081cec),
    (0x081CF2, i_081cf2),
    (0x081CF8, i_081cf8),
    (0x081CFC, i_081cfc),
    (0x081D00, i_081d00),
    (0x081D04, i_081d04),
    (0x081D0A, i_081d0a),
    (0x081D0C, i_081d0c),
    (0x081D10, i_081d10),
    (0x081D16, i_081d16),
    (0x081D1A, i_081d1a),
    (0x081D20, i_081d20),
    (0x081D24, i_081d24),
    (0x081D26, i_081d26),
    (0x081D2A, i_081d2a),
    (0x081D2E, i_081d2e),
    (0x081D34, i_081d34),
    (0x081D3A, i_081d3a),
    (0x081D3E, i_081d3e),
    (0x081D44, i_081d44),
    (0x081D4A, i_081d4a),
    (0x081D4E, i_081d4e),
    (0x081D52, i_081d52),
    (0x081D54, i_081d54),
    (0x081D58, i_081d58),
    (0x081D5C, i_081d5c),
    (0x081D60, i_081d60),
    (0x081D62, i_081d62),
    (0x081D64, i_081d64),
    (0x081D68, i_081d68),
    (0x081D6E, i_081d6e),
    (0x081D76, i_081d76),
    (0x081D7A, i_081d7a),
    (0x081D7E, i_081d7e),
    (0x081D84, i_081d84),
    (0x081D86, i_081d86),
    (0x081D8A, i_081d8a),
    (0x081D90, i_081d90),
    (0x081D94, i_081d94),
    (0x081D9A, i_081d9a),
    (0x081D9E, i_081d9e),
    (0x081DA0, i_081da0),
    (0x081DA4, i_081da4),
    (0x081DA8, i_081da8),
    (0x081DAE, i_081dae),
    (0x081DB0, i_081db0),
    (0x081DB4, i_081db4),
    (0x081DBA, i_081dba),
    (0x081DC0, i_081dc0),
    (0x081DC6, i_081dc6),
    (0x081DCA, i_081dca),
    (0x081DCE, i_081dce),
    (0x081DD2, i_081dd2),
    (0x081DD8, i_081dd8),
    (0x081DDE, i_081dde),
    (0x081DE4, i_081de4),
    (0x081DE8, i_081de8),
    (0x081DEC, i_081dec),
    (0x081DF2, i_081df2),
    (0x081DF8, i_081df8),
    (0x081DFC, i_081dfc),
    (0x081E02, i_081e02),
    (0x081E08, i_081e08),
    (0x081E0C, i_081e0c),
    (0x081E0E, i_081e0e),
    (0x081E12, i_081e12),
    (0x081E16, i_081e16),
    (0x081E1C, i_081e1c),
    (0x081E20, i_081e20),
    (0x081E24, i_081e24),
    (0x081E2A, i_081e2a),
    (0x081E2E, i_081e2e),
    (0x081E32, i_081e32),
    (0x081E36, i_081e36),
    (0x081E3A, i_081e3a),
    (0x081E40, i_081e40),
    (0x081E44, i_081e44),
    (0x081E48, i_081e48),
    (0x081E4A, i_081e4a),
    (0x081FE8, i_081fe8),
    (0x081FEC, i_081fec),
    (0x081FEE, i_081fee),
    (0x081FF0, i_081ff0),
    (0x081FF6, i_081ff6),
    (0x081FFC, i_081ffc),
    (0x082002, i_082002),
    (0x082008, i_082008),
    (0x08200E, i_08200e),
    (0x082014, i_082014),
    (0x082018, i_082018),
    (0x08201A, i_08201a),
    (0x08201E, i_08201e),
    (0x082022, i_082022),
    (0x082028, i_082028),
    (0x08202A, i_08202a),
    (0x08202E, i_08202e),
    (0x082034, i_082034),
    (0x082038, i_082038),
    (0x08203C, i_08203c),
    (0x082042, i_082042),
    (0x082046, i_082046),
    (0x08204A, i_08204a),
    (0x08204E, i_08204e),
    (0x082050, i_082050),
    (0x082052, i_082052),
    (0x082058, i_082058),
    (0x08205E, i_08205e),
    (0x082074, i_082074),
    (0x082078, i_082078),
    (0x08207E, i_08207e),
    (0x082084, i_082084),
    (0x08208A, i_08208a),
    (0x082090, i_082090),
    (0x082096, i_082096),
    (0x08209A, i_08209a),
    (0x08209E, i_08209e),
    (0x0820A6, i_0820a6),
    (0x0820AC, i_0820ac),
    (0x0820B0, i_0820b0),
    (0x0820B4, i_0820b4),
    (0x0820BA, i_0820ba),
    (0x0820C0, i_0820c0),
    (0x0820C4, i_0820c4),
    (0x0820C8, i_0820c8),
    (0x0820CE, i_0820ce),
    (0x0820D4, i_0820d4),
    (0x0820D8, i_0820d8),
    (0x0820DA, i_0820da),
    (0x0820DE, i_0820de),
    (0x0820E4, i_0820e4),
    (0x0820E8, i_0820e8),
    (0x0820EE, i_0820ee),
    (0x0820F4, i_0820f4),
    (0x0820FA, i_0820fa),
    (0x0823FA, i_0823fa),
    (0x082400, i_082400),
    (0x082402, i_082402),
    (0x082406, i_082406),
    (0x08240C, i_08240c),
    (0x082410, i_082410),
    (0x082414, i_082414),
    (0x08241A, i_08241a),
    (0x082420, i_082420),
    (0x082424, i_082424),
    (0x08242A, i_08242a),
    (0x082430, i_082430),
    (0x082436, i_082436),
    (0x08243A, i_08243a),
    (0x08243E, i_08243e),
    (0x082442, i_082442),
    (0x082444, i_082444),
    (0x08244A, i_08244a),
    (0x08244E, i_08244e),
    (0x082452, i_082452),
    (0x082454, i_082454),
    (0x082456, i_082456),
    (0x08245C, i_08245c),
    (0x082464, i_082464),
    (0x08246C, i_08246c),
    (0x082472, i_082472),
    (0x08247A, i_08247a),
    (0x08250C, i_08250c),
    (0x082510, i_082510),
    (0x082512, i_082512),
    (0x082514, i_082514),
    (0x08251A, i_08251a),
    (0x08251E, i_08251e),
    (0x082522, i_082522),
    (0x082528, i_082528),
    (0x08252E, i_08252e),
    (0x082534, i_082534),
    (0x082538, i_082538),
    (0x08253A, i_08253a),
    (0x082540, i_082540),
    (0x082546, i_082546),
    (0x08254C, i_08254c),
    (0x082552, i_082552),
    (0x082556, i_082556),
    (0x08255A, i_08255a),
    (0x08255E, i_08255e),
    (0x082560, i_082560),
    (0x082562, i_082562),
    (0x082568, i_082568),
    (0x08256C, i_08256c),
    (0x08256E, i_08256e),
    (0x082572, i_082572),
    (0x082574, i_082574),
    (0x082576, i_082576),
    (0x08257A, i_08257a),
    (0x08257E, i_08257e),
    (0x082584, i_082584),
    (0x08258A, i_08258a),
    (0x08258E, i_08258e),
    (0x082590, i_082590),
    (0x082592, i_082592),
    (0x082594, i_082594),
    (0x082596, i_082596),
    (0x08259C, i_08259c),
    (0x0825A0, i_0825a0),
    (0x0825A2, i_0825a2),
    (0x0825A6, i_0825a6),
    (0x0825AC, i_0825ac),
    (0x0825B2, i_0825b2),
    (0x0825B8, i_0825b8),
    (0x0825BC, i_0825bc),
    (0x0825C0, i_0825c0),
    (0x0825C4, i_0825c4),
    (0x0825CA, i_0825ca),
    (0x0825CE, i_0825ce),
    (0x0825D4, i_0825d4),
    (0x0825DA, i_0825da),
    (0x0825E0, i_0825e0),
    (0x0825E6, i_0825e6),
    (0x0825EC, i_0825ec),
    (0x0825F0, i_0825f0),
    (0x0825F2, i_0825f2),
    (0x0825F8, i_0825f8),
    (0x0825FE, i_0825fe),
    (0x082602, i_082602),
    (0x082606, i_082606),
    (0x082608, i_082608),
    (0x08260E, i_08260e),
    (0x082614, i_082614),
    (0x08261A, i_08261a),
    (0x08261E, i_08261e),
    (0x082622, i_082622),
    (0x082624, i_082624),
    (0x082626, i_082626),
    (0x08262A, i_08262a),
    (0x082630, i_082630),
    (0x082636, i_082636),
    (0x08263C, i_08263c),
    (0x082640, i_082640),
    (0x082642, i_082642),
    (0x082648, i_082648),
    (0x08264C, i_08264c),
    (0x082652, i_082652),
    (0x082656, i_082656),
    (0x08265A, i_08265a),
    (0x082660, i_082660),
    (0x082664, i_082664),
    (0x082666, i_082666),
    (0x08266C, i_08266c),
    (0x082670, i_082670),
    (0x082674, i_082674),
    (0x082678, i_082678),
    (0x08267A, i_08267a),
    (0x08267C, i_08267c),
    (0x082680, i_082680),
    (0x082686, i_082686),
    (0x08268A, i_08268a),
    (0x08268C, i_08268c),
    (0x082692, i_082692),
    (0x082698, i_082698),
    (0x08269C, i_08269c),
    (0x08269E, i_08269e),
    (0x0826A4, i_0826a4),
    (0x0826A8, i_0826a8),
    (0x0826AC, i_0826ac),
    (0x0826B2, i_0826b2),
    (0x0826B8, i_0826b8),
    (0x0826BE, i_0826be),
    (0x0826C2, i_0826c2),
    (0x0826C6, i_0826c6),
    (0x0826CA, i_0826ca),
    (0x0826CC, i_0826cc),
    (0x0826D2, i_0826d2),
    (0x0826D8, i_0826d8),
    (0x0826DE, i_0826de),
    (0x0826E2, i_0826e2),
    (0x0826E4, i_0826e4),
    (0x0826E8, i_0826e8),
    (0x0826EC, i_0826ec),
    (0x0826F0, i_0826f0),
    (0x0826F6, i_0826f6),
    (0x0826FC, i_0826fc),
    (0x082702, i_082702),
    (0x082706, i_082706),
    (0x08270A, i_08270a),
    (0x08270E, i_08270e),
    (0x082712, i_082712),
    (0x082716, i_082716),
    (0x08271A, i_08271a),
    (0x08271E, i_08271e),
    (0x0827FE, i_0827fe),
    (0x082804, i_082804),
    (0x08280A, i_08280a),
    (0x08280E, i_08280e),
    (0x082812, i_082812),
    (0x082814, i_082814),
    (0x08281A, i_08281a),
    (0x08281E, i_08281e),
    (0x082822, i_082822),
    (0x082824, i_082824),
    (0x08282A, i_08282a),
    (0x082830, i_082830),
    (0x082834, i_082834),
    (0x082838, i_082838),
    (0x08283A, i_08283a),
    (0x08283C, i_08283c),
    (0x082842, i_082842),
    (0x082848, i_082848),
    (0x08284E, i_08284e),
    (0x082854, i_082854),
    (0x082858, i_082858),
    (0x08285A, i_08285a),
    (0x082860, i_082860),
    (0x082864, i_082864),
    (0x082868, i_082868),
    (0x08286A, i_08286a),
    (0x082870, i_082870),
    (0x082876, i_082876),
    (0x08287A, i_08287a),
    (0x082880, i_082880),
    (0x082884, i_082884),
    (0x08288A, i_08288a),
    (0x082890, i_082890),
    (0x082896, i_082896),
    (0x08289C, i_08289c),
    (0x0828A2, i_0828a2),
    (0x0828A6, i_0828a6),
    (0x0828A8, i_0828a8),
    (0x0828AE, i_0828ae),
    (0x0828B4, i_0828b4),
    (0x0828B8, i_0828b8),
    (0x0828BC, i_0828bc),
    (0x0828BE, i_0828be),
    (0x0828C4, i_0828c4),
    (0x0828C8, i_0828c8),
    (0x0828CE, i_0828ce),
    (0x0828E0, i_0828e0),
    (0x0828E4, i_0828e4),
    (0x0828EA, i_0828ea),
    (0x0828EE, i_0828ee),
    (0x0828F2, i_0828f2),
    (0x0828F8, i_0828f8),
    (0x0828FE, i_0828fe),
    (0x082904, i_082904),
    (0x082908, i_082908),
    (0x08290A, i_08290a),
    (0x082910, i_082910),
    (0x082916, i_082916),
    (0x08291C, i_08291c),
    (0x082920, i_082920),
    (0x082924, i_082924),
    (0x082928, i_082928),
    (0x08292A, i_08292a),
    (0x08292C, i_08292c),
    (0x082930, i_082930),
    (0x082936, i_082936),
    (0x08293A, i_08293a),
    (0x082940, i_082940),
    (0x082946, i_082946),
    (0x08294C, i_08294c),
    (0x082952, i_082952),
    (0x082956, i_082956),
    (0x08295A, i_08295a),
    (0x08295E, i_08295e),
    (0x082964, i_082964),
    (0x082968, i_082968),
    (0x08296C, i_08296c),
    (0x082972, i_082972),
    (0x082978, i_082978),
    (0x08297C, i_08297c),
    (0x08297E, i_08297e),
    (0x082984, i_082984),
    (0x082988, i_082988),
    (0x08298C, i_08298c),
    (0x08298E, i_08298e),
    (0x082994, i_082994),
    (0x08299A, i_08299a),
    (0x0829A0, i_0829a0),
    (0x0829A4, i_0829a4),
    (0x0829A8, i_0829a8),
    (0x0829AA, i_0829aa),
    (0x0829AC, i_0829ac),
    (0x0829B0, i_0829b0),
    (0x0829B6, i_0829b6),
    (0x0829BA, i_0829ba),
    (0x0829BC, i_0829bc),
    (0x0829C2, i_0829c2),
    (0x0829C6, i_0829c6),
    (0x0829CC, i_0829cc),
    (0x0829D0, i_0829d0),
    (0x0829D2, i_0829d2),
    (0x0829D6, i_0829d6),
    (0x0829DA, i_0829da),
    (0x0829DE, i_0829de),
    (0x0829E2, i_0829e2),
    (0x0829E8, i_0829e8),
    (0x0829EC, i_0829ec),
    (0x0829F2, i_0829f2),
    (0x0829F6, i_0829f6),
    (0x0829F8, i_0829f8),
    (0x0829FC, i_0829fc),
    (0x082A02, i_082a02),
    (0x082A06, i_082a06),
    (0x082A0A, i_082a0a),
    (0x082A0E, i_082a0e),
    (0x082A14, i_082a14),
    (0x082A1A, i_082a1a),
    (0x082A20, i_082a20),
    (0x082A26, i_082a26),
    (0x082A2C, i_082a2c),
    (0x082A30, i_082a30),
    (0x082A32, i_082a32),
    (0x082A38, i_082a38),
    (0x082A3E, i_082a3e),
    (0x082A42, i_082a42),
    (0x082A46, i_082a46),
    (0x082A48, i_082a48),
    (0x082A4E, i_082a4e),
    (0x082A54, i_082a54),
    (0x082A5A, i_082a5a),
    (0x082A5E, i_082a5e),
    (0x082A62, i_082a62),
    (0x082A64, i_082a64),
    (0x082A66, i_082a66),
    (0x082A6A, i_082a6a),
    (0x082A6C, i_082a6c),
    (0x082A70, i_082a70),
    (0x082A76, i_082a76),
    (0x082A7C, i_082a7c),
    (0x082A80, i_082a80),
    (0x082A84, i_082a84),
    (0x082A86, i_082a86),
    (0x082A88, i_082a88),
    (0x082A8A, i_082a8a),
    (0x082A8C, i_082a8c),
    (0x082A92, i_082a92),
    (0x082A96, i_082a96),
    (0x082A98, i_082a98),
    (0x082A9C, i_082a9c),
    (0x082AA0, i_082aa0),
    (0x082AA4, i_082aa4),
    (0x082AAA, i_082aaa),
    (0x082AAE, i_082aae),
    (0x082AB4, i_082ab4),
    (0x082ABA, i_082aba),
    (0x082ABE, i_082abe),
    (0x082AC4, i_082ac4),
    (0x082ACA, i_082aca),
    (0x082AD0, i_082ad0),
    (0x082AD6, i_082ad6),
    (0x082ADC, i_082adc),
    (0x082AE0, i_082ae0),
    (0x082AE2, i_082ae2),
    (0x082AE8, i_082ae8),
    (0x082AEC, i_082aec),
    (0x082AF2, i_082af2),
    (0x082AF4, i_082af4),
    (0x082AFA, i_082afa),
    (0x082AFE, i_082afe),
    (0x082B02, i_082b02),
    (0x082B08, i_082b08),
    (0x082B0C, i_082b0c),
    (0x082B10, i_082b10),
    (0x082B14, i_082b14),
    (0x082B16, i_082b16),
    (0x082B1C, i_082b1c),
    (0x082B22, i_082b22),
    (0x082B26, i_082b26),
    (0x082B2C, i_082b2c),
    (0x082B2E, i_082b2e),
    (0x082B34, i_082b34),
    (0x082B3A, i_082b3a),
];
