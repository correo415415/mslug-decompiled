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
// ===== Squad_ChildInitTableA_080110  ($080110, 214 B, enemies/squad/deploy_module_07fbd2.s)
/// $080110: move.w #$132, d1
pub fn i_080110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80114;
    cpu.ir = 0x323C;
    let v1: u32 = 0x132;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $080114: jsr $236e.l
pub fn i_080114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8011A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8011A);
    cpu.pc = 0x236E;
    16
}
/// $08011A: lea.l $2e3ebc.l, a0
pub fn i_08011a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80120;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E3EBC;
    20
}
/// $080120: move.w $78(a6), d0
pub fn i_080120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80124;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $080124: lsl.w #$3, d0
pub fn i_080124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80126;
    cpu.ir = 0xE748;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $080126: movea.l (a0, d0.w), a1
pub fn i_080126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8012A;
    cpu.ir = 0x2270;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    18
}
/// $08012A: movea.l $4(a0, d0.w), a2
pub fn i_08012a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8012E;
    cpu.ir = 0x2470;
    let ad2 = cpu.a[0].wrapping_add(0x4).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    18
}
/// $08012E: move.w $84(a6), d0
pub fn i_08012e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80132;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $080132: move.b (a2, d0.w), $32(a6)
pub fn i_080132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80138;
    cpu.ir = 0x1D72;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad4, v3 as u8);
    22
}
/// $080138: move.b (a2, d0.w), $33(a6)
pub fn i_080138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8013E;
    cpu.ir = 0x1D72;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad4, v3 as u8);
    22
}
/// $08013E: add.w d0, d0
pub fn i_08013e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80140;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $080140: add.w d0, d0
pub fn i_080140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80142;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $080142: move.w $22(a6), d1
pub fn i_080142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80146;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $080146: add.w (a1, d0.w), d1
pub fn i_080146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8014A;
    cpu.ir = 0xD271;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $08014A: move.w d1, $22(a6)
pub fn i_08014a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8014E;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08014E: move.w $24(a6), d1
pub fn i_08014e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80152;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $080152: add.w $2(a1, d0.w), d1
pub fn i_080152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80156;
    cpu.ir = 0xD271;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $080156: move.w d1, $24(a6)
pub fn i_080156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8015A;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08015A: clr.b $21(a6)
pub fn i_08015a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8015E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $08015E: jsr $267e2.l
pub fn i_08015e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80164;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80164);
    cpu.pc = 0x267E2;
    16
}
/// $080164: move.w #$fd00, $28(a6)
pub fn i_080164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8016A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFD00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08016A: move.w #$1000, d0
pub fn i_08016a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8016E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08016E: jsr $28134.l
pub fn i_08016e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80174;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80174);
    cpu.pc = 0x28134;
    16
}
/// $080174: andi.w #$ffe3, $38(a6)
pub fn i_080174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8017A;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $08017A: ori.w #$8, $38(a6)
pub fn i_08017a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80180;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x8) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $080180: tst.w $84(a6)
pub fn i_080180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80184;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $080184: bne.w $80192
pub fn i_080184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80188;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x80192; return 10; }
    8
}
/// $080188: move.w #$10ba, d0
pub fn i_080188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8018C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10BA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08018C: jsr $2352.l
pub fn i_08018c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80192;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80192);
    cpu.pc = 0x2352;
    16
}
/// $080192: lea.l $2e2fca.l, a0
pub fn i_080192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80198;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E2FCA;
    20
}
/// $080198: jsr $28cd4.l
pub fn i_080198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8019E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8019E);
    cpu.pc = 0x28CD4;
    16
}
/// $08019E: bclr.b #$3, $13(a6)
pub fn i_08019e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x801A4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0801A4: lea.l $80996(pc), a1
pub fn i_0801a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x801A8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x80996;
    16
}
/// $0801A8: jsr $4ae.l
pub fn i_0801a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x801AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x801AE);
    cpu.pc = 0x4AE;
    16
}
/// $0801AE: jsr $5dd02.l
pub fn i_0801ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x801B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x801B4);
    cpu.pc = 0x5DD02;
    16
}
/// $0801B4: lea.l $801ba(pc), a1
pub fn i_0801b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x801B8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x801BA;
    16
}
/// $0801B8: move.l a1, (a6)
pub fn i_0801b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x801BA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0801BA: jsr $27cee.l
pub fn i_0801ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x801C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x801C0);
    cpu.pc = 0x27CEE;
    16
}
/// $0801C0: jsr $28d70.l
pub fn i_0801c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x801C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x801C6);
    cpu.pc = 0x28D70;
    16
}
/// $0801C6: cmpi.w #$e0, $22(a6)
pub fn i_0801c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x801CC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xE0, Size::Word, false);
    16
}
/// $0801CC: bge.w $801d6
pub fn i_0801cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x801D0;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x801D6; return 10; }
    8
}
/// $0801D0: lea.l $801ee(pc), a1
pub fn i_0801d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x801D4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x801EE;
    16
}
/// $0801D4: move.l a1, (a6)
pub fn i_0801d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x801D6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0801D6: movea.l #$ffffffff, a0
pub fn i_0801d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x801DC;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0801DC: jsr $5dd5c.l
pub fn i_0801dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x801E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x801E2);
    cpu.pc = 0x5DD5C;
    16
}
/// $0801E2: bcc.w $801ec
pub fn i_0801e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x801E6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x801EC; return 10; }
    8
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
// ===== Squad_ChildAltSprite_0801ee  ($0801EE, 56 B, enemies/squad/deploy_module_07fbd2.s)
/// $0801EE: lea.l $2e2fd6.l, a0
pub fn i_0801ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x801F4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E2FD6;
    20
}
/// $0801F4: jsr $28cd4.l
pub fn i_0801f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x801FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x801FA);
    cpu.pc = 0x28CD4;
    16
}
/// $0801FA: lea.l $80200(pc), a1
pub fn i_0801fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x801FE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x80200;
    16
}
/// $0801FE: move.l a1, (a6)
pub fn i_0801fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80200;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $080200: jsr $27cee.l
pub fn i_080200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80206;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80206);
    cpu.pc = 0x27CEE;
    16
}
/// $080206: jsr $28d70.l
pub fn i_080206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8020C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8020C);
    cpu.pc = 0x28D70;
    16
}
/// $08020C: bcc.w $80216
pub fn i_08020c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80210;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x80216; return 10; }
    8
}
/// $080210: lea.l $8022e(pc), a1
pub fn i_080210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80214;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8022E;
    16
}
/// $080214: move.l a1, (a6)
pub fn i_080214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80216;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $080216: movea.l #$ffffffff, a0
pub fn i_080216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8021C;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $08021C: jsr $5dd5c.l
pub fn i_08021c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80222;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80222);
    cpu.pc = 0x5DD5C;
    16
}
/// $080222: bcc.w $8022c
pub fn i_080222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80226;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8022C; return 10; }
    8
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
/// $0803E6: rts
pub fn i_0803e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x803E8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
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
/// $0807AC: move.b $86(a6), $3a(a6)
pub fn i_0807ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x807B2;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $0807B2: move.b #$0, $20(a6)
pub fn i_0807b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x807B8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0807B8: move.w #$ffb0, $2e(a6)
pub fn i_0807b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x807BE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFB0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0807BE: move.w #$d000, d0
pub fn i_0807be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x807C2;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0807C2: jsr $28134.l
pub fn i_0807c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x807C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x807C8);
    cpu.pc = 0x28134;
    16
}
/// $0807C8: andi.w #$ffe3, $38(a6)
pub fn i_0807c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x807CE;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0807CE: ori.w #$0, $38(a6)
pub fn i_0807ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x807D4;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0807D4: move.w #$1, $66(a6)
pub fn i_0807d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x807DA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0807DA: move.l #$2e3af8, $48(a6)
pub fn i_0807da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x807E2;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2E3AF8;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0807E2: bclr.b #$3, $13(a6)
pub fn i_0807e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x807E8;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0807E8: lea.l $2e394c.l, a0
pub fn i_0807e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x807EE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E394C;
    20
}
/// $0807EE: jsr $28cd4.l
pub fn i_0807ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x807F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x807F4);
    cpu.pc = 0x28CD4;
    16
}
/// $0807F4: lea.l $807fa(pc), a1
pub fn i_0807f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x807F8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x807FA;
    16
}
/// $0807F8: move.l a1, (a6)
pub fn i_0807f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x807FA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0807FA: jsr $27d50.l
pub fn i_0807fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80800;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80800);
    cpu.pc = 0x27D50;
    16
}
/// $080800: bcc.w $8080a
pub fn i_080800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80804;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8080A; return 10; }
    8
}
/// $080804: lea.l $8166e(pc), a1
pub fn i_080804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80808;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8166E;
    16
}
/// $080808: move.l a1, (a6)
pub fn i_080808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8080A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08080A: tst.b $20(a6)
pub fn i_08080a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8080E;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $08080E: bne.w $80850
pub fn i_08080e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80812;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x80850; return 10; }
    8
}
/// $080812: jsr $80f1c(pc)
pub fn i_080812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80816;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x80816);
    cpu.pc = 0x80F1C;
    16
}
/// $080816: bcc.w $80850
pub fn i_080816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8081A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x80850; return 10; }
    8
}
/// $08081A: move.b #$ff, $20(a6)
pub fn i_08081a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80820;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $080820: movea.l $c(a6), a0
pub fn i_080820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80824;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $080824: movea.l $c(a0), a0
pub fn i_080824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80828;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $080828: cmpi.b #$2, $9f(a0)
pub fn i_080828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8082E;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x9F);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $08082E: blt.w $80850
pub fn i_08082e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80832;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x80850; return 10; }
    8
}
/// $080832: lea.l $78908.l, a1
pub fn i_080832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80838;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x78908;
    20
}
/// $080838: jsr $4ae.l
pub fn i_080838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8083E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8083E);
    cpu.pc = 0x4AE;
    16
}
/// $08083E: jsr $5dd02.l
pub fn i_08083e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80844;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80844);
    cpu.pc = 0x5DD02;
    16
}
/// $080844: move.w #$ffff, $38(a0)
pub fn i_080844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8084A;
    cpu.ir = 0x317C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08084A: move.w #$120, $24(a0)
pub fn i_08084a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80850;
    cpu.ir = 0x317C;
    let v1: u32 = 0x120;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $080850: jsr $5e826.l
pub fn i_080850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80856;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80856);
    cpu.pc = 0x5E826;
    16
}
/// $080856: jsr $28d70.l
pub fn i_080856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8085C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8085C);
    cpu.pc = 0x28D70;
    16
}
/// $08085C: jsr $283d8.l
pub fn i_08085c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80862;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80862);
    cpu.pc = 0x283D8;
    16
}
/// $080862: btst.b #$1, $13(a6)
pub fn i_080862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80868;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $080868: beq.w $80872
pub fn i_080868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8086C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x80872; return 10; }
    8
}
/// $08086C: lea.l $81678(pc), a1
pub fn i_08086c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80870;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81678;
    16
}
/// $080870: move.l a1, (a6)
pub fn i_080870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80872;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $080872: jsr $2870a.l
pub fn i_080872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80878;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80878);
    cpu.pc = 0x2870A;
    16
}
/// $080878: bcc.w $80882
pub fn i_080878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8087C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x80882; return 10; }
    8
}
/// $08087C: lea.l $8166e(pc), a1
pub fn i_08087c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80880;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8166E;
    16
}
/// $080880: move.l a1, (a6)
pub fn i_080880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80882;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $080882: movea.l #$ffffffff, a0
pub fn i_080882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80888;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $080888: lea.l $2e3c44.l, a0
pub fn i_080888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8088E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E3C44;
    20
}
/// $08088E: jsr $5dd56.l
pub fn i_08088e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80894;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80894);
    cpu.pc = 0x5DD56;
    16
}
/// $080894: bcc.w $8089e
pub fn i_080894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80898;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8089E; return 10; }
    8
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
/// $080B1A: move.w #$fe00, $2a(a6)
pub fn i_080b1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80B20;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFE00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $080B20: move.w #$ffb0, $2e(a6)
pub fn i_080b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80B26;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFB0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $080B26: move.w $36(a6), d0
pub fn i_080b26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80B2A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $080B2A: btst.b #$0, $3a(a6)
pub fn i_080b2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80B30;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $080B30: bne.w $80b36
pub fn i_080b30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80B34;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x80B36; return 10; }
    8
}
/// $080B34: neg.w d0
pub fn i_080b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80B36;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $080B36: asr.w #$1, d0
pub fn i_080b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80B38;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $080B38: move.w d0, $28(a6)
pub fn i_080b38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80B3C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $080B3C: jsr $5e9b6.l
pub fn i_080b3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80B42;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80B42);
    cpu.pc = 0x5E9B6;
    16
}
/// $080B42: andi.w #$3f, d0
pub fn i_080b42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80B46;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $080B46: addi.w #$3c, d0
pub fn i_080b46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80B4A;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x3C, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $080B4A: move.w d0, $70(a6)
pub fn i_080b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80B4E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $080B4E: lea.l $2e305c.l, a0
pub fn i_080b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80B54;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E305C;
    20
}
/// $080B54: jsr $28cd4.l
pub fn i_080b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80B5A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80B5A);
    cpu.pc = 0x28CD4;
    16
}
/// $080B5A: lea.l $80b60(pc), a1
pub fn i_080b5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80B5E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x80B60;
    16
}
/// $080B5E: move.l a1, (a6)
pub fn i_080b5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80B60;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $080B60: subq.w #$1, $70(a6)
pub fn i_080b60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80B64;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $080B64: bne.w $80b86
pub fn i_080b64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80B68;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x80B86; return 10; }
    8
}
/// $080B68: lea.l $80b86(pc), a1
pub fn i_080b68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80B6C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x80B86;
    16
}
/// $080B6C: move.l a1, (a6)
pub fn i_080b6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80B6E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $080B6E: jsr $5e9b6.l
pub fn i_080b6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80B74;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80B74);
    cpu.pc = 0x5E9B6;
    16
}
/// $080B74: andi.w #$f, d0
pub fn i_080b74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80B78;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $080B78: cmpi.w #$5, d0
pub fn i_080b78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80B7C;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x5, Size::Word, false);
    8
}
/// $080B7C: bgt.w $80b86
pub fn i_080b7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80B80;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x80B86; return 10; }
    8
}
/// $080B80: lea.l $81018(pc), a1
pub fn i_080b80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80B84;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81018;
    16
}
/// $080B84: move.l a1, (a6)
pub fn i_080b84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80B86;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $080B86: jsr $27d50.l
pub fn i_080b86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80B8C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80B8C);
    cpu.pc = 0x27D50;
    16
}
/// $080B8C: bcc.w $80b96
pub fn i_080b8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80B90;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x80B96; return 10; }
    8
}
/// $080B90: lea.l $80f32(pc), a1
pub fn i_080b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80B94;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x80F32;
    16
}
/// $080B94: move.l a1, (a6)
pub fn i_080b94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80B96;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $080B96: jsr $28d70.l
pub fn i_080b96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80B9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80B9C);
    cpu.pc = 0x28D70;
    16
}
/// $080B9C: jsr $80efa(pc)
pub fn i_080b9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80BA0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x80BA0);
    cpu.pc = 0x80EFA;
    16
}
/// $080BA0: bcc.w $80baa
pub fn i_080ba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80BA4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x80BAA; return 10; }
    8
}
/// $080BA4: lea.l $81018(pc), a1
pub fn i_080ba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80BA8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81018;
    16
}
/// $080BA8: move.l a1, (a6)
pub fn i_080ba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80BAA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $080BAA: jsr $80f1c(pc)
pub fn i_080baa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80BAE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x80BAE);
    cpu.pc = 0x80F1C;
    16
}
/// $080BAE: bcc.w $80bb8
pub fn i_080bae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80BB2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x80BB8; return 10; }
    8
}
/// $080BB2: lea.l $80f3c(pc), a1
pub fn i_080bb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80BB6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x80F3C;
    16
}
/// $080BB6: move.l a1, (a6)
pub fn i_080bb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80BB8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $080BB8: movea.l #$ffffffff, a0
pub fn i_080bb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80BBE;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $080BBE: lea.l $2e3c44.l, a0
pub fn i_080bbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80BC4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E3C44;
    20
}
/// $080BC4: jsr $5dd5c.l
pub fn i_080bc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80BCA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80BCA);
    cpu.pc = 0x5DD5C;
    16
}
/// $080BCA: bcc.w $80bd4
pub fn i_080bca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80BCE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x80BD4; return 10; }
    8
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
/// $080C08: cmpi.b #$18, $34(a6)
pub fn i_080c08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80C0E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x18, Size::Byte, false);
    16
}
/// $080C0E: blt.w $80c1c
pub fn i_080c0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80C12;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x80C1C; return 10; }
    8
}
/// $080C12: cmpi.b #$68, $34(a6)
pub fn i_080c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80C18;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x68, Size::Byte, false);
    16
}
/// $080C18: ble.w $80c3a
pub fn i_080c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80C1C;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x80C3A; return 10; }
    8
}
/// $080C1C: move.l #$2e2e02, $90(a6)
pub fn i_080c1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80C24;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2E2E02;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $080C24: btst.b #$0, $3a(a6)
pub fn i_080c24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80C2A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $080C2A: beq.w $80c54
pub fn i_080c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80C2E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x80C54; return 10; }
    8
}
/// $080C2E: move.l #$2e2e74, $90(a6)
pub fn i_080c2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80C36;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2E2E74;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $080C36: bra.w $80c54
pub fn i_080c36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80C3A;
    cpu.ir = 0x6000;
    cpu.pc = 0x80C54;
    10
}
/// $080C3A: move.l #$2e2e3e, $90(a6)
pub fn i_080c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80C42;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2E2E3E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $080C42: btst.b #$0, $3a(a6)
pub fn i_080c42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80C48;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $080C48: beq.w $80c54
pub fn i_080c48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80C4C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x80C54; return 10; }
    8
}
/// $080C4C: move.l #$2e2eb0, $90(a6)
pub fn i_080c4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80C54;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2E2EB0;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $080C54: moveq #$0, d0
pub fn i_080c54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80C56;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $080C56: move.b d0, $20(a6)
pub fn i_080c56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80C5A;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $080C5A: move.w d0, $94(a6)
pub fn i_080c5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80C5E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    bus.write16(ad2, v1 as u16);
    12
}
/// $080C5E: move.b d0, $96(a6)
pub fn i_080c5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80C62;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x96);
    bus.write8(ad2, v1 as u8);
    12
}
/// $080C62: lea.l $80c68(pc), a1
pub fn i_080c62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80C66;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x80C68;
    16
}
/// $080C66: move.l a1, (a6)
pub fn i_080c66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80C68;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
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
/// $080DA6: move.l #$2e2ee6, $90(a6)
pub fn i_080da6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80DAE;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2E2EE6;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $080DAE: btst.b #$0, $3a(a6)
pub fn i_080dae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80DB4;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $080DB4: beq.w $80dc0
pub fn i_080db4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80DB8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x80DC0; return 10; }
    8
}
/// $080DB8: move.l #$2e2f06, $90(a6)
pub fn i_080db8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80DC0;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2E2F06;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $080DC0: moveq #$0, d0
pub fn i_080dc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80DC2;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $080DC2: move.b d0, $20(a6)
pub fn i_080dc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80DC6;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $080DC6: move.w d0, $94(a6)
pub fn i_080dc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80DCA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    bus.write16(ad2, v1 as u16);
    12
}
/// $080DCA: move.b d0, $96(a6)
pub fn i_080dca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80DCE;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x96);
    bus.write8(ad2, v1 as u8);
    12
}
/// $080DCE: move.w #$2000, d0
pub fn i_080dce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80DD2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $080DD2: jsr $28134.l
pub fn i_080dd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80DD8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80DD8);
    cpu.pc = 0x28134;
    16
}
/// $080DD8: andi.w #$ffe3, $38(a6)
pub fn i_080dd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80DDE;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $080DDE: ori.w #$14, $38(a6)
pub fn i_080dde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80DE4;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $080DE4: move.w #$1, $70(a6)
pub fn i_080de4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80DEA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $080DEA: lea.l $80df0(pc), a1
pub fn i_080dea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80DEE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x80DF0;
    16
}
/// $080DEE: move.l a1, (a6)
pub fn i_080dee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80DF0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $080DF0: jsr $8179e(pc)
pub fn i_080df0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80DF4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x80DF4);
    cpu.pc = 0x8179E;
    16
}
/// $080DF4: cmpi.b #$10, $32(a6)
pub fn i_080df4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80DFA;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x10, Size::Byte, false);
    16
}
/// $080DFA: bls.w $80e14
pub fn i_080dfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80DFE;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x80E14; return 10; }
    8
}
/// $080DFE: subq.w #$1, $70(a6)
pub fn i_080dfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80E02;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $080E02: bne.w $80e14
pub fn i_080e02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80E06;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x80E14; return 10; }
    8
}
/// $080E06: move.w #$1, $70(a6)
pub fn i_080e06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80E0C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $080E0C: subq.b #$1, $32(a6)
pub fn i_080e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80E10;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x32);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $080E10: subq.b #$1, $33(a6)
pub fn i_080e10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80E14;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x33);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $080E14: jsr $78f8a.l
pub fn i_080e14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80E1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80E1A);
    cpu.pc = 0x78F8A;
    16
}
/// $080E1A: bcc.w $80e24
pub fn i_080e1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80E1E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x80E24; return 10; }
    8
}
/// $080E1E: lea.l $80f32(pc), a1
pub fn i_080e1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80E22;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x80F32;
    16
}
/// $080E22: move.l a1, (a6)
pub fn i_080e22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80E24;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $080E24: jsr $28d70.l
pub fn i_080e24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80E2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80E2A);
    cpu.pc = 0x28D70;
    16
}
/// $080E2A: movea.l #$ffffffff, a0
pub fn i_080e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80E30;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $080E30: lea.l $2e3c44.l, a0
pub fn i_080e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80E36;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E3C44;
    20
}
/// $080E36: jsr $5dd5c.l
pub fn i_080e36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80E3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80E3C);
    cpu.pc = 0x5DD5C;
    16
}
/// $080E3C: bcc.w $80e46
pub fn i_080e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80E40;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x80E46; return 10; }
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
/// $080F3C: movea.l $c(a6), a0
pub fn i_080f3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80F40;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $080F40: cmpi.b #$2, $9f(a0)
pub fn i_080f40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80F46;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x9F);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $080F46: blt.w $80f68
pub fn i_080f46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80F4A;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x80F68; return 10; }
    8
}
/// $080F4A: lea.l $78890.l, a1
pub fn i_080f4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80F50;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x78890;
    20
}
/// $080F50: jsr $4ae.l
pub fn i_080f50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80F56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80F56);
    cpu.pc = 0x4AE;
    16
}
/// $080F56: jsr $5dd02.l
pub fn i_080f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80F5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x80F5C);
    cpu.pc = 0x5DD02;
    16
}
/// $080F5C: move.w #$ffff, $38(a0)
pub fn i_080f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80F62;
    cpu.ir = 0x317C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $080F62: move.w #$120, $24(a0)
pub fn i_080f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80F68;
    cpu.ir = 0x317C;
    let v1: u32 = 0x120;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
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
/// $081018: movea.l $c(a6), a0
pub fn i_081018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8101C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $08101C: btst.b #$0, $9f(a0)
pub fn i_08101c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81022;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x9F);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $081022: bne.w $810a0
pub fn i_081022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81026;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x810A0; return 10; }
    8
}
/// $081026: jsr $5e804.l
pub fn i_081026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8102C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8102C);
    cpu.pc = 0x5E804;
    16
}
/// $08102C: bcs.w $810a0
pub fn i_08102c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81030;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x810A0; return 10; }
    8
}
/// $081030: lea.l $8121c(pc), a1
pub fn i_081030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81034;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8121C;
    16
}
/// $081034: jsr $4ae.l
pub fn i_081034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8103A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8103A);
    cpu.pc = 0x4AE;
    16
}
/// $08103A: jsr $5dd02.l
pub fn i_08103a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81040;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81040);
    cpu.pc = 0x5DD02;
    16
}
/// $081040: move.b $32(a6), $32(a0)
pub fn i_081040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81046;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x32);
    bus.write8(ad4, v3 as u8);
    20
}
/// $081046: move.b $33(a6), $33(a0)
pub fn i_081046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8104C;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x33);
    bus.write8(ad4, v3 as u8);
    20
}
/// $08104C: lea.l $8123c(pc), a1
pub fn i_08104c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81050;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8123C;
    16
}
/// $081050: jsr $4ae.l
pub fn i_081050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81056;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81056);
    cpu.pc = 0x4AE;
    16
}
/// $081056: jsr $5dd02.l
pub fn i_081056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8105C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8105C);
    cpu.pc = 0x5DD02;
    16
}
/// $08105C: move.b $32(a6), $32(a0)
pub fn i_08105c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81062;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x32);
    bus.write8(ad4, v3 as u8);
    20
}
/// $081062: move.b $33(a6), $33(a0)
pub fn i_081062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81068;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x33);
    bus.write8(ad4, v3 as u8);
    20
}
/// $081068: lea.l $81260(pc), a1
pub fn i_081068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8106C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81260;
    16
}
/// $08106C: jsr $4ae.l
pub fn i_08106c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81072;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81072);
    cpu.pc = 0x4AE;
    16
}
/// $081072: jsr $5dd02.l
pub fn i_081072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81078;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81078);
    cpu.pc = 0x5DD02;
    16
}
/// $081078: move.b $32(a6), $32(a0)
pub fn i_081078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8107E;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x32);
    bus.write8(ad4, v3 as u8);
    20
}
/// $08107E: move.b $33(a6), $33(a0)
pub fn i_08107e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81084;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x33);
    bus.write8(ad4, v3 as u8);
    20
}
/// $081084: lea.l $81284(pc), a1
pub fn i_081084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81088;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81284;
    16
}
/// $081088: jsr $4ae.l
pub fn i_081088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8108E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8108E);
    cpu.pc = 0x4AE;
    16
}
/// $08108E: jsr $5dd02.l
pub fn i_08108e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81094;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81094);
    cpu.pc = 0x5DD02;
    16
}
/// $081094: move.b $32(a6), $32(a0)
pub fn i_081094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8109A;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x32);
    bus.write8(ad4, v3 as u8);
    20
}
/// $08109A: move.b $33(a6), $33(a0)
pub fn i_08109a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x810A0;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x33);
    bus.write8(ad4, v3 as u8);
    20
}
/// $0810A0: move.w #$10ba, d0
pub fn i_0810a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x810A4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10BA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0810A4: jsr $2222.l
pub fn i_0810a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x810AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x810AA);
    cpu.pc = 0x2222;
    16
}
/// $0810AA: lea.l $77fd6.l, a1
pub fn i_0810aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x810B0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $0810B0: jsr $4ae.l
pub fn i_0810b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x810B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x810B6);
    cpu.pc = 0x4AE;
    16
}
/// $0810B6: jsr $5dd02.l
pub fn i_0810b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x810BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x810BC);
    cpu.pc = 0x5DD02;
    16
}
/// $0810BC: move.b $32(a6), $32(a0)
pub fn i_0810bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x810C2;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x32);
    bus.write8(ad4, v3 as u8);
    20
}
/// $0810C2: move.b $33(a6), $33(a0)
pub fn i_0810c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x810C8;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x33);
    bus.write8(ad4, v3 as u8);
    20
}
/// $0810C8: lea.l $2e3c66.l, a1
pub fn i_0810c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x810CE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E3C66;
    20
}
/// $0810CE: jsr $77c7e.l
pub fn i_0810ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x810D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x810D4);
    cpu.pc = 0x77C7E;
    16
}
/// $0810D4: move.b $32(a6), $32(a0)
pub fn i_0810d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x810DA;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x32);
    bus.write8(ad4, v3 as u8);
    20
}
/// $0810DA: move.b $33(a6), $33(a0)
pub fn i_0810da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x810E0;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x33);
    bus.write8(ad4, v3 as u8);
    20
}
/// $0810E0: move.b $34(a6), $34(a0)
pub fn i_0810e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x810E6;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x34);
    bus.write8(ad4, v3 as u8);
    20
}
/// $0810E6: move.w $36(a6), $36(a0)
pub fn i_0810e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x810EC;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x36);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0810EC: lea.l $8139c(pc), a1
pub fn i_0810ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x810F0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8139C;
    16
}
/// $0810F0: jsr $4ae.l
pub fn i_0810f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x810F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x810F6);
    cpu.pc = 0x4AE;
    16
}
/// $0810F6: jsr $5dd02.l
pub fn i_0810f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x810FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x810FC);
    cpu.pc = 0x5DD02;
    16
}
/// $0810FC: move.b $32(a6), $32(a0)
pub fn i_0810fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81102;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x32);
    bus.write8(ad4, v3 as u8);
    20
}
/// $081102: move.b $33(a6), $33(a0)
pub fn i_081102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81108;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x33);
    bus.write8(ad4, v3 as u8);
    20
}
/// $081108: move.b $34(a6), $34(a0)
pub fn i_081108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8110E;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x34);
    bus.write8(ad4, v3 as u8);
    20
}
/// $08110E: move.w $36(a6), $36(a0)
pub fn i_08110e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81114;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x36);
    bus.write16(ad4, v3 as u16);
    20
}
/// $081114: lea.l $8139c(pc), a1
pub fn i_081114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81118;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8139C;
    16
}
/// $081118: jsr $4ae.l
pub fn i_081118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8111E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8111E);
    cpu.pc = 0x4AE;
    16
}
/// $08111E: jsr $5dd02.l
pub fn i_08111e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81124;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81124);
    cpu.pc = 0x5DD02;
    16
}
/// $081124: move.b $32(a6), $32(a0)
pub fn i_081124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8112A;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x32);
    bus.write8(ad4, v3 as u8);
    20
}
/// $08112A: move.b $33(a6), $33(a0)
pub fn i_08112a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81130;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x33);
    bus.write8(ad4, v3 as u8);
    20
}
/// $081130: move.b $34(a6), $34(a0)
pub fn i_081130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81136;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x34);
    bus.write8(ad4, v3 as u8);
    20
}
/// $081136: move.w $36(a6), $36(a0)
pub fn i_081136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8113C;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x36);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08113C: eori.b #$1, $3a(a0)
pub fn i_08113c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81142;
    cpu.ir = 0xA28;
    let ad1 = cpu.a[0].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $081142: lea.l $813ac(pc), a1
pub fn i_081142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81146;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x813AC;
    16
}
/// $081146: jsr $4ae.l
pub fn i_081146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8114C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8114C);
    cpu.pc = 0x4AE;
    16
}
/// $08114C: jsr $5dd02.l
pub fn i_08114c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81152;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81152);
    cpu.pc = 0x5DD02;
    16
}
/// $081152: move.b $32(a6), $32(a0)
pub fn i_081152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81158;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x32);
    bus.write8(ad4, v3 as u8);
    20
}
/// $081158: move.b $33(a6), $33(a0)
pub fn i_081158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8115E;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x33);
    bus.write8(ad4, v3 as u8);
    20
}
/// $08115E: move.b $34(a6), $34(a0)
pub fn i_08115e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81164;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x34);
    bus.write8(ad4, v3 as u8);
    20
}
/// $081164: move.w $36(a6), $36(a0)
pub fn i_081164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8116A;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x36);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08116A: lea.l $813dc(pc), a1
pub fn i_08116a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8116E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x813DC;
    16
}
/// $08116E: jsr $4ae.l
pub fn i_08116e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81174;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81174);
    cpu.pc = 0x4AE;
    16
}
/// $081174: jsr $5dd02.l
pub fn i_081174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8117A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8117A);
    cpu.pc = 0x5DD02;
    16
}
/// $08117A: move.b $32(a6), $32(a0)
pub fn i_08117a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81180;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x32);
    bus.write8(ad4, v3 as u8);
    20
}
/// $081180: move.b $33(a6), $33(a0)
pub fn i_081180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81186;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x33);
    bus.write8(ad4, v3 as u8);
    20
}
/// $081186: move.b $34(a6), $34(a0)
pub fn i_081186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8118C;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x34);
    bus.write8(ad4, v3 as u8);
    20
}
/// $08118C: move.w $36(a6), $36(a0)
pub fn i_08118c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81192;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x36);
    bus.write16(ad4, v3 as u16);
    20
}
/// $081192: bra.w $811cc
pub fn i_081192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81196;
    cpu.ir = 0x6000;
    cpu.pc = 0x811CC;
    10
}
/// $081196: bclr.b #$1, $12(a6)
pub fn i_081196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8119C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08119C: movea.l $c(a6), a0
pub fn i_08119c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x811A0;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0811A0: move.w $84(a6), d0
pub fn i_0811a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x811A4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0811A4: bclr.b d0, $78(a0)
pub fn i_0811a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x811A8;
    cpu.ir = 0x1A8;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x78);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0811A8: add.w d0, d0
pub fn i_0811a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x811AA;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0811AA: addi.w #$80, d0
pub fn i_0811aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x811AE;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x80, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0811AE: move.w $66(a6), (a0, d0.w)
pub fn i_0811ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x811B4;
    cpu.ir = 0x31AE;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    bus.write16(ad4, v3 as u16);
    22
}
/// $0811B4: subq.b #$1, $76(a0)
pub fn i_0811b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x811B8;
    cpu.ir = 0x5328;
    let ad1 = cpu.a[0].wrapping_add(0x76);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $0811B8: move.b #$ff, $20(a6)
pub fn i_0811b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x811BE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
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
/// $0811CA: rts
pub fn i_0811ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x811CC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
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
/// $0811E2: rts
pub fn i_0811e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x811E4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0811E4: bclr.b #$1, $12(a6)
pub fn i_0811e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x811EA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0811EA: movea.l $c(a6), a0
pub fn i_0811ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x811EE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0811EE: move.w $84(a6), d0
pub fn i_0811ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x811F2;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0811F2: bclr.b d0, $78(a0)
pub fn i_0811f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x811F6;
    cpu.ir = 0x1A8;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x78);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0811F6: add.w d0, d0
pub fn i_0811f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x811F8;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0811F8: addi.w #$80, d0
pub fn i_0811f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x811FC;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x80, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0811FC: move.w $66(a6), (a0, d0.w)
pub fn i_0811fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81202;
    cpu.ir = 0x31AE;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    bus.write16(ad4, v3 as u16);
    22
}
/// $081202: subq.b #$1, $76(a0)
pub fn i_081202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81206;
    cpu.ir = 0x5328;
    let ad1 = cpu.a[0].wrapping_add(0x76);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $081206: move.b #$ff, $20(a6)
pub fn i_081206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8120C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08120C: jmp $518.l
pub fn i_08120c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81212;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $081212: rts
pub fn i_081212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81214;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_081214  ($081214, 8 B, kernel/jmp_to_scheduler.c)
/// $081214: jmp $518.l
pub fn i_081214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8121A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08121A: rts
pub fn i_08121a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8121C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
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
/// $081332: jsr $5e9b6.l
pub fn i_081332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81338;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81338);
    cpu.pc = 0x5E9B6;
    16
}
/// $081338: andi.w #$f, d0
pub fn i_081338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8133C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08133C: cmpi.w #$6, d0
pub fn i_08133c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81340;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x6, Size::Word, false);
    8
}
/// $081340: bge.w $8134a
pub fn i_081340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81344;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x8134A; return 10; }
    8
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
// ===== Squad_ShardSprites_08134c  ($08134C, 742 B, enemies/squad/death_handlers_080736.s)
/// $08134C: lea.l $2e3990.l, a0
pub fn i_08134c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81352;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E3990;
    20
}
/// $081352: jsr $28cd4.l
pub fn i_081352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81358;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81358);
    cpu.pc = 0x28CD4;
    16
}
/// $081358: bra.w $81378
pub fn i_081358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8135C;
    cpu.ir = 0x6000;
    cpu.pc = 0x81378;
    10
}
/// $08135C: lea.l $2e39ec.l, a0
pub fn i_08135c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81362;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E39EC;
    20
}
/// $081362: jsr $28cd4.l
pub fn i_081362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81368;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81368);
    cpu.pc = 0x28CD4;
    16
}
/// $081368: bra.w $81378
pub fn i_081368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8136C;
    cpu.ir = 0x6000;
    cpu.pc = 0x81378;
    10
}
/// $08136C: lea.l $2e3a48.l, a0
pub fn i_08136c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81372;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E3A48;
    20
}
/// $081372: jsr $28cd4.l
pub fn i_081372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81378;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81378);
    cpu.pc = 0x28CD4;
    16
}
/// $081378: move.w #$c000, d0
pub fn i_081378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8137C;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08137C: jsr $28134.l
pub fn i_08137c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81382;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81382);
    cpu.pc = 0x28134;
    16
}
/// $081382: andi.w #$ffe3, $38(a6)
pub fn i_081382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81388;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $081388: ori.w #$1c, $38(a6)
pub fn i_081388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8138E;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $08138E: move.w #$48, d1
pub fn i_08138e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81392;
    cpu.ir = 0x323C;
    let v1: u32 = 0x48;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $081392: jsr $236e.l
pub fn i_081392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81398;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81398);
    cpu.pc = 0x236E;
    16
}
/// $081398: bra.w $81492
pub fn i_081398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8139C;
    cpu.ir = 0x6000;
    cpu.pc = 0x81492;
    10
}
/// $08139C: lea.l $2e3990.l, a0
pub fn i_08139c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x813A2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E3990;
    20
}
/// $0813A2: jsr $28cd4.l
pub fn i_0813a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x813A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x813A8);
    cpu.pc = 0x28CD4;
    16
}
/// $0813A8: bra.w $813e8
pub fn i_0813a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x813AC;
    cpu.ir = 0x6000;
    cpu.pc = 0x813E8;
    10
}
/// $0813AC: lea.l $2e39ec.l, a0
pub fn i_0813ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x813B2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E39EC;
    20
}
/// $0813B2: jsr $28cd4.l
pub fn i_0813b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x813B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x813B8);
    cpu.pc = 0x28CD4;
    16
}
/// $0813B8: move.b #$1, $3a(a6)
pub fn i_0813b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x813BE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0813BE: cmpi.b #$40, $34(a6)
pub fn i_0813be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x813C4;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x40, Size::Byte, false);
    16
}
/// $0813C4: bls.w $813e8
pub fn i_0813c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x813C8;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x813E8; return 10; }
    8
}
/// $0813C8: cmpi.b #$c0, $34(a6)
pub fn i_0813c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x813CE;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xC0, Size::Byte, false);
    16
}
/// $0813CE: bhi.w $813e8
pub fn i_0813ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x813D2;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x813E8; return 10; }
    8
}
/// $0813D2: move.b #$0, $3a(a6)
pub fn i_0813d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x813D8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0813D8: bra.w $813e8
pub fn i_0813d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x813DC;
    cpu.ir = 0x6000;
    cpu.pc = 0x813E8;
    10
}
/// $0813DC: lea.l $2e3a48.l, a0
pub fn i_0813dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x813E2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E3A48;
    20
}
/// $0813E2: jsr $28cd4.l
pub fn i_0813e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x813E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x813E8);
    cpu.pc = 0x28CD4;
    16
}
/// $0813E8: move.w #$c000, d0
pub fn i_0813e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x813EC;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0813EC: jsr $28134.l
pub fn i_0813ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x813F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x813F2);
    cpu.pc = 0x28134;
    16
}
/// $0813F2: andi.w #$ffe3, $38(a6)
pub fn i_0813f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x813F8;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0813F8: ori.w #$1c, $38(a6)
pub fn i_0813f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x813FE;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0813FE: move.w #$48, d1
pub fn i_0813fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81402;
    cpu.ir = 0x323C;
    let v1: u32 = 0x48;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $081402: jsr $236e.l
pub fn i_081402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81408;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81408);
    cpu.pc = 0x236E;
    16
}
/// $081408: bra.w $8154a
pub fn i_081408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8140C;
    cpu.ir = 0x6000;
    cpu.pc = 0x8154A;
    10
}
/// $08140C: lea.l $2de3c4.l, a0
pub fn i_08140c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81412;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE3C4;
    20
}
/// $081412: jsr $28cd4.l
pub fn i_081412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81418;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81418);
    cpu.pc = 0x28CD4;
    16
}
/// $081418: moveq #$a, d1
pub fn i_081418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8141A;
    cpu.ir = 0x720A;
    cpu.d[1] = 0xA;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08141A: bra.w $81462
pub fn i_08141a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8141E;
    cpu.ir = 0x6000;
    cpu.pc = 0x81462;
    10
}
/// $08141E: lea.l $2de34e.l, a0
pub fn i_08141e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81424;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE34E;
    20
}
/// $081424: jsr $28cd4.l
pub fn i_081424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8142A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8142A);
    cpu.pc = 0x28CD4;
    16
}
/// $08142A: moveq #$a, d1
pub fn i_08142a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8142C;
    cpu.ir = 0x720A;
    cpu.d[1] = 0xA;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08142C: bra.w $81462
pub fn i_08142c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81430;
    cpu.ir = 0x6000;
    cpu.pc = 0x81462;
    10
}
/// $081430: lea.l $2de43a.l, a0
pub fn i_081430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81436;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE43A;
    20
}
/// $081436: jsr $28cd4.l
pub fn i_081436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8143C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8143C);
    cpu.pc = 0x28CD4;
    16
}
/// $08143C: moveq #$a, d1
pub fn i_08143c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8143E;
    cpu.ir = 0x720A;
    cpu.d[1] = 0xA;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08143E: bra.w $81462
pub fn i_08143e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81442;
    cpu.ir = 0x6000;
    cpu.pc = 0x81462;
    10
}
/// $081442: lea.l $2de5f4.l, a0
pub fn i_081442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81448;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE5F4;
    20
}
/// $081448: jsr $28cd4.l
pub fn i_081448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8144E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8144E);
    cpu.pc = 0x28CD4;
    16
}
/// $08144E: moveq #$b, d1
pub fn i_08144e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81450;
    cpu.ir = 0x720B;
    cpu.d[1] = 0xB;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $081450: bra.w $81462
pub fn i_081450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81454;
    cpu.ir = 0x6000;
    cpu.pc = 0x81462;
    10
}
/// $081454: lea.l $2de66a.l, a0
pub fn i_081454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8145A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE66A;
    20
}
/// $08145A: jsr $28cd4.l
pub fn i_08145a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81460;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81460);
    cpu.pc = 0x28CD4;
    16
}
/// $081460: moveq #$b, d1
pub fn i_081460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81462;
    cpu.ir = 0x720B;
    cpu.d[1] = 0xB;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $081462: jsr $236e.l
pub fn i_081462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81468;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81468);
    cpu.pc = 0x236E;
    16
}
/// $081468: movea.l $c(a6), a0
pub fn i_081468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8146C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $08146C: move.w $36(a0), $36(a6)
pub fn i_08146c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81472;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad4, v3 as u16);
    20
}
/// $081472: move.w $34(a0), $34(a6)
pub fn i_081472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81478;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad4, v3 as u16);
    20
}
/// $081478: move.w #$c000, d0
pub fn i_081478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8147C;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08147C: jsr $28134.l
pub fn i_08147c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81482;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81482);
    cpu.pc = 0x28134;
    16
}
/// $081482: andi.w #$ffe3, $38(a6)
pub fn i_081482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81488;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $081488: ori.w #$1c, $38(a6)
pub fn i_081488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8148E;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $08148E: bra.w $8154a
pub fn i_08148e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81492;
    cpu.ir = 0x6000;
    cpu.pc = 0x8154A;
    10
}
/// $081492: cmpi.w #$0, $36(a6)
pub fn i_081492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81498;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $081498: bgt.w $814a2
pub fn i_081498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8149C;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x814A2; return 10; }
    8
}
/// $08149C: move.w #$400, $36(a6)
pub fn i_08149c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x814A2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0814A2: jsr $5e9b6.l
pub fn i_0814a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x814A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x814A8);
    cpu.pc = 0x5E9B6;
    16
}
/// $0814A8: move.w d0, d3
pub fn i_0814a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x814AA;
    cpu.ir = 0x3600;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0814AA: andi.l #$1e, d0
pub fn i_0814aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x814B0;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0x1E) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $0814B0: addi.w #$30, d0
pub fn i_0814b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x814B4;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x30, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0814B4: add.w d0, d0
pub fn i_0814b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x814B6;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0814B6: lea.l $2c072c.l, a1
pub fn i_0814b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x814BC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C072C;
    20
}
/// $0814BC: move.w (a1, d0.w), d1
pub fn i_0814bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x814C0;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $0814C0: lea.l $2c07ac.l, a1
pub fn i_0814c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x814C6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C07AC;
    20
}
/// $0814C6: move.w (a1, d0.w), d2
pub fn i_0814c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x814CA;
    cpu.ir = 0x3431;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $0814CA: andi.w #$7, d3
pub fn i_0814ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x814CE;
    cpu.ir = 0x243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0814CE: addq.w #$1, d3
pub fn i_0814ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x814D0;
    cpu.ir = 0x5243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0814D0: muls.w d3, d1
pub fn i_0814d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x814D2;
    cpu.ir = 0xC3C3;
    let v1 = cpu.d[3] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[1] as u16 as i16 as i32)) as u32;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $0814D2: asr.w #$2, d1
pub fn i_0814d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x814D4;
    cpu.ir = 0xE441;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $0814D4: move.w $36(a6), d0
pub fn i_0814d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x814D8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0814D8: muls.w d0, d1
pub fn i_0814d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x814DA;
    cpu.ir = 0xC3C0;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[1] as u16 as i16 as i32)) as u32;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $0814DA: muls.w d0, d2
pub fn i_0814da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x814DC;
    cpu.ir = 0xC5C0;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[2] as u16 as i16 as i32)) as u32;
    cpu.d[2] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $0814DC: asr.l #$8, d1
pub fn i_0814dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x814DE;
    cpu.ir = 0xE081;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[1] = r;
    22
}
/// $0814DE: asr.l #$8, d2
pub fn i_0814de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x814E0;
    cpu.ir = 0xE082;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[2] = r;
    22
}
/// $0814E0: move.w d1, $2a(a6)
pub fn i_0814e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x814E4;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0814E4: move.w d2, $28(a6)
pub fn i_0814e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x814E8;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0814E8: jsr $5e9b6.l
pub fn i_0814e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x814EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x814EE);
    cpu.pc = 0x5E9B6;
    16
}
/// $0814EE: cmpi.b #$0, d0
pub fn i_0814ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x814F2;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $0814F2: bgt.w $81526
pub fn i_0814f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x814F6;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x81526; return 10; }
    8
}
/// $0814F6: jsr $5e9b6.l
pub fn i_0814f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x814FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x814FC);
    cpu.pc = 0x5E9B6;
    16
}
/// $0814FC: andi.w #$3, d0
pub fn i_0814fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81500;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $081500: addq.w #$2, d0
pub fn i_081500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81502;
    cpu.ir = 0x5440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $081502: move.w $28(a6), d1
pub fn i_081502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81506;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $081506: muls.w d1, d0
pub fn i_081506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81508;
    cpu.ir = 0xC1C1;
    let v1 = cpu.d[1] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[0] as u16 as i16 as i32)) as u32;
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $081508: asr.w #$1, d0
pub fn i_081508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8150A;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $08150A: move.w d0, $28(a6)
pub fn i_08150a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8150E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08150E: jsr $5e9b6.l
pub fn i_08150e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81514;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81514);
    cpu.pc = 0x5E9B6;
    16
}
/// $081514: andi.w #$3, d0
pub fn i_081514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81518;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $081518: addq.w #$2, d0
pub fn i_081518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8151A;
    cpu.ir = 0x5440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08151A: move.w $2a(a6), d1
pub fn i_08151a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8151E;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08151E: muls.w d1, d0
pub fn i_08151e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81520;
    cpu.ir = 0xC1C1;
    let v1 = cpu.d[1] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[0] as u16 as i16 as i32)) as u32;
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $081520: asr.w #$1, d0
pub fn i_081520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81522;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $081522: move.w d0, $2a(a6)
pub fn i_081522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81526;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $081526: move.w $2a(a6), d0
pub fn i_081526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8152A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08152A: cmpi.w #$c00, d0
pub fn i_08152a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8152E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xC00, Size::Word, false);
    8
}
/// $08152E: ble.w $81536
pub fn i_08152e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81532;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x81536; return 10; }
    8
}
/// $081532: move.w #$c00, d0
pub fn i_081532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81536;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $081536: move.w d0, $74(a6)
pub fn i_081536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8153A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08153A: move.w #$ffe0, $2e(a6)
pub fn i_08153a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81540;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $081540: lea.l $815e8(pc), a1
pub fn i_081540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81544;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x815E8;
    16
}
/// $081544: move.l a1, (a6)
pub fn i_081544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81546;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $081546: bra.w $815e8
pub fn i_081546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8154A;
    cpu.ir = 0x6000;
    cpu.pc = 0x815E8;
    10
}
/// $08154A: cmpi.w #$0, $36(a6)
pub fn i_08154a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81550;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $081550: bgt.w $8155a
pub fn i_081550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81554;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8155A; return 10; }
    8
}
/// $081554: move.w #$400, $36(a6)
pub fn i_081554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8155A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08155A: jsr $5e9b6.l
pub fn i_08155a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81560;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81560);
    cpu.pc = 0x5E9B6;
    16
}
/// $081560: andi.w #$1f, d0
pub fn i_081560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81564;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $081564: subi.w #$10, d0
pub fn i_081564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81568;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $081568: move.b $34(a6), d1
pub fn i_081568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8156C;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08156C: andi.w #$ff, d1
pub fn i_08156c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81570;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $081570: add.w d1, d0
pub fn i_081570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81572;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $081572: andi.w #$ff, d0
pub fn i_081572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81576;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $081576: move.w $36(a6), d1
pub fn i_081576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8157A;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08157A: asl.w #$1, d1
pub fn i_08157a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8157C;
    cpu.ir = 0xE341;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $08157C: jsr $13c0e.l
pub fn i_08157c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81582;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81582);
    cpu.pc = 0x13C0E;
    16
}
/// $081582: move.w d1, $28(a6)
pub fn i_081582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81586;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $081586: move.w d2, $2a(a6)
pub fn i_081586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8158A;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08158A: jsr $5e9b6.l
pub fn i_08158a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81590;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81590);
    cpu.pc = 0x5E9B6;
    16
}
/// $081590: cmpi.b #$0, d0
pub fn i_081590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81594;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $081594: bgt.w $815c8
pub fn i_081594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81598;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x815C8; return 10; }
    8
}
/// $081598: jsr $5e9b6.l
pub fn i_081598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8159E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8159E);
    cpu.pc = 0x5E9B6;
    16
}
/// $08159E: andi.w #$3, d0
pub fn i_08159e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x815A2;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0815A2: addq.w #$2, d0
pub fn i_0815a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x815A4;
    cpu.ir = 0x5440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0815A4: move.w $28(a6), d1
pub fn i_0815a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x815A8;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0815A8: muls.w d1, d0
pub fn i_0815a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x815AA;
    cpu.ir = 0xC1C1;
    let v1 = cpu.d[1] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[0] as u16 as i16 as i32)) as u32;
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $0815AA: asr.w #$1, d0
pub fn i_0815aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x815AC;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $0815AC: move.w d0, $28(a6)
pub fn i_0815ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x815B0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0815B0: jsr $5e9b6.l
pub fn i_0815b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x815B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x815B6);
    cpu.pc = 0x5E9B6;
    16
}
/// $0815B6: andi.w #$3, d0
pub fn i_0815b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x815BA;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0815BA: addq.w #$2, d0
pub fn i_0815ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x815BC;
    cpu.ir = 0x5440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0815BC: move.w $2a(a6), d1
pub fn i_0815bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x815C0;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0815C0: muls.w d1, d0
pub fn i_0815c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x815C2;
    cpu.ir = 0xC1C1;
    let v1 = cpu.d[1] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[0] as u16 as i16 as i32)) as u32;
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $0815C2: asr.w #$1, d0
pub fn i_0815c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x815C4;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $0815C4: move.w d0, $2a(a6)
pub fn i_0815c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x815C8;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0815C8: move.w $2a(a6), d0
pub fn i_0815c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x815CC;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0815CC: cmpi.w #$c00, d0
pub fn i_0815cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x815D0;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xC00, Size::Word, false);
    8
}
/// $0815D0: ble.w $815d8
pub fn i_0815d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x815D4;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x815D8; return 10; }
    8
}
/// $0815D4: move.w #$c00, d0
pub fn i_0815d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x815D8;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0815D8: move.w d0, $74(a6)
pub fn i_0815d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x815DC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0815DC: move.w #$ffe0, $2e(a6)
pub fn i_0815dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x815E2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0815E2: lea.l $815e8(pc), a1
pub fn i_0815e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x815E6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x815E8;
    16
}
/// $0815E6: move.l a1, (a6)
pub fn i_0815e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x815E8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0815E8: jsr $27d50.l
pub fn i_0815e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x815EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x815EE);
    cpu.pc = 0x27D50;
    16
}
/// $0815EE: bcc.w $81616
pub fn i_0815ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x815F2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x81616; return 10; }
    8
}
/// $0815F2: move.w #$ffe0, $2e(a6)
pub fn i_0815f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x815F8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0815F8: move.w $28(a6), d0
pub fn i_0815f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x815FC;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0815FC: asr.w #$2, d0
pub fn i_0815fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x815FE;
    cpu.ir = 0xE440;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $0815FE: move.w d0, $28(a6)
pub fn i_0815fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81602;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $081602: move.w $74(a6), d0
pub fn i_081602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81606;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $081606: asr.w #$1, d0
pub fn i_081606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81608;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $081608: move.w d0, $2a(a6)
pub fn i_081608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8160C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08160C: move.w d0, $74(a6)
pub fn i_08160c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81610;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $081610: lea.l $81214(pc), a1
pub fn i_081610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81614;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81214;
    16
}
/// $081614: move.l a1, (a6)
pub fn i_081614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81616;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $081616: jsr $28d70.l
pub fn i_081616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8161C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8161C);
    cpu.pc = 0x28D70;
    16
}
/// $08161C: movea.l #$ffffffff, a0
pub fn i_08161c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81622;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $081622: lea.l $2e3c4c.l, a0
pub fn i_081622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81628;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E3C4C;
    20
}
/// $081628: jsr $5dd56.l
pub fn i_081628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8162E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8162E);
    cpu.pc = 0x5DD56;
    16
}
/// $08162E: bcc.w $81638
pub fn i_08162e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81632;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x81638; return 10; }
    8
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
/// $0817D6: move.w #$40, $2e(a6)
pub fn i_0817d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x817DC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0817DC: rts
pub fn i_0817dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x817DE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0817DE: lea.l $2e2ff6.l, a0
pub fn i_0817de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x817E4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E2FF6;
    20
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
// ===== Squad_LeaSprite_0817ec  ($0817EC, 6 B, enemies/squad/death_handlers_080736.s)
/// $0817EC: lea.l $2e300a.l, a0
pub fn i_0817ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x817F2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E300A;
    20
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
// ===== Squad_LeaSprite_0817fa  ($0817FA, 6 B, enemies/squad/death_handlers_080736.s)
/// $0817FA: lea.l $2e303a.l, a0
pub fn i_0817fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81800;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E303A;
    20
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
// ===== Squad_LeaSprite_081808  ($081808, 6 B, enemies/squad/death_handlers_080736.s)
/// $081808: lea.l $2e305c.l, a0
pub fn i_081808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8180E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E305C;
    20
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
// ===== LeaSprite_081816  ($081816, 6 B, enemies/squad/para_squad_module_081816.s)
/// $081816: lea.l $2e308c.l, a0
pub fn i_081816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8181C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E308C;
    20
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
// ===== LeaSpriteAlt_081824  ($081824, 14 B, enemies/squad/para_squad_module_081816.s)
/// $081824: tst.b $7f(a6)
pub fn i_081824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81828;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x7F);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $081828: bne.w $8183a
pub fn i_081828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8182C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8183A; return 10; }
    8
}
/// $08182C: lea.l $2e30ae.l, a0
pub fn i_08182c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81832;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E30AE;
    20
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
// ===== LeaSprite_08183a  ($08183A, 6 B, enemies/squad/para_squad_module_081816.s)
/// $08183A: lea.l $2e31dc.l, a0
pub fn i_08183a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81840;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E31DC;
    20
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
// ===== LeaSpriteAlt_081848  ($081848, 14 B, enemies/squad/para_squad_module_081816.s)
/// $081848: tst.b $7f(a6)
pub fn i_081848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8184C;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x7F);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $08184C: bne.w $8185e
pub fn i_08184c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81850;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8185E; return 10; }
    8
}
/// $081850: lea.l $2e330a.l, a0
pub fn i_081850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81856;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E330A;
    20
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
// ===== LeaSprite_08185e  ($08185E, 6 B, enemies/squad/para_squad_module_081816.s)
/// $08185E: lea.l $2e3438.l, a0
pub fn i_08185e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81864;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E3438;
    20
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
    (0x080110, i_080110),
    (0x080114, i_080114),
    (0x08011A, i_08011a),
    (0x080120, i_080120),
    (0x080124, i_080124),
    (0x080126, i_080126),
    (0x08012A, i_08012a),
    (0x08012E, i_08012e),
    (0x080132, i_080132),
    (0x080138, i_080138),
    (0x08013E, i_08013e),
    (0x080140, i_080140),
    (0x080142, i_080142),
    (0x080146, i_080146),
    (0x08014A, i_08014a),
    (0x08014E, i_08014e),
    (0x080152, i_080152),
    (0x080156, i_080156),
    (0x08015A, i_08015a),
    (0x08015E, i_08015e),
    (0x080164, i_080164),
    (0x08016A, i_08016a),
    (0x08016E, i_08016e),
    (0x080174, i_080174),
    (0x08017A, i_08017a),
    (0x080180, i_080180),
    (0x080184, i_080184),
    (0x080188, i_080188),
    (0x08018C, i_08018c),
    (0x080192, i_080192),
    (0x080198, i_080198),
    (0x08019E, i_08019e),
    (0x0801A4, i_0801a4),
    (0x0801A8, i_0801a8),
    (0x0801AE, i_0801ae),
    (0x0801B4, i_0801b4),
    (0x0801B8, i_0801b8),
    (0x0801BA, i_0801ba),
    (0x0801C0, i_0801c0),
    (0x0801C6, i_0801c6),
    (0x0801CC, i_0801cc),
    (0x0801D0, i_0801d0),
    (0x0801D4, i_0801d4),
    (0x0801D6, i_0801d6),
    (0x0801DC, i_0801dc),
    (0x0801E2, i_0801e2),
    (0x0801E6, i_0801e6),
    (0x0801EA, i_0801ea),
    (0x0801EC, i_0801ec),
    (0x0801EE, i_0801ee),
    (0x0801F4, i_0801f4),
    (0x0801FA, i_0801fa),
    (0x0801FE, i_0801fe),
    (0x080200, i_080200),
    (0x080206, i_080206),
    (0x08020C, i_08020c),
    (0x080210, i_080210),
    (0x080214, i_080214),
    (0x080216, i_080216),
    (0x08021C, i_08021c),
    (0x080222, i_080222),
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
    (0x0803E6, i_0803e6),
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
    (0x0807AC, i_0807ac),
    (0x0807B2, i_0807b2),
    (0x0807B8, i_0807b8),
    (0x0807BE, i_0807be),
    (0x0807C2, i_0807c2),
    (0x0807C8, i_0807c8),
    (0x0807CE, i_0807ce),
    (0x0807D4, i_0807d4),
    (0x0807DA, i_0807da),
    (0x0807E2, i_0807e2),
    (0x0807E8, i_0807e8),
    (0x0807EE, i_0807ee),
    (0x0807F4, i_0807f4),
    (0x0807F8, i_0807f8),
    (0x0807FA, i_0807fa),
    (0x080800, i_080800),
    (0x080804, i_080804),
    (0x080808, i_080808),
    (0x08080A, i_08080a),
    (0x08080E, i_08080e),
    (0x080812, i_080812),
    (0x080816, i_080816),
    (0x08081A, i_08081a),
    (0x080820, i_080820),
    (0x080824, i_080824),
    (0x080828, i_080828),
    (0x08082E, i_08082e),
    (0x080832, i_080832),
    (0x080838, i_080838),
    (0x08083E, i_08083e),
    (0x080844, i_080844),
    (0x08084A, i_08084a),
    (0x080850, i_080850),
    (0x080856, i_080856),
    (0x08085C, i_08085c),
    (0x080862, i_080862),
    (0x080868, i_080868),
    (0x08086C, i_08086c),
    (0x080870, i_080870),
    (0x080872, i_080872),
    (0x080878, i_080878),
    (0x08087C, i_08087c),
    (0x080880, i_080880),
    (0x080882, i_080882),
    (0x080888, i_080888),
    (0x08088E, i_08088e),
    (0x080894, i_080894),
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
    (0x080B1A, i_080b1a),
    (0x080B20, i_080b20),
    (0x080B26, i_080b26),
    (0x080B2A, i_080b2a),
    (0x080B30, i_080b30),
    (0x080B34, i_080b34),
    (0x080B36, i_080b36),
    (0x080B38, i_080b38),
    (0x080B3C, i_080b3c),
    (0x080B42, i_080b42),
    (0x080B46, i_080b46),
    (0x080B4A, i_080b4a),
    (0x080B4E, i_080b4e),
    (0x080B54, i_080b54),
    (0x080B5A, i_080b5a),
    (0x080B5E, i_080b5e),
    (0x080B60, i_080b60),
    (0x080B64, i_080b64),
    (0x080B68, i_080b68),
    (0x080B6C, i_080b6c),
    (0x080B6E, i_080b6e),
    (0x080B74, i_080b74),
    (0x080B78, i_080b78),
    (0x080B7C, i_080b7c),
    (0x080B80, i_080b80),
    (0x080B84, i_080b84),
    (0x080B86, i_080b86),
    (0x080B8C, i_080b8c),
    (0x080B90, i_080b90),
    (0x080B94, i_080b94),
    (0x080B96, i_080b96),
    (0x080B9C, i_080b9c),
    (0x080BA0, i_080ba0),
    (0x080BA4, i_080ba4),
    (0x080BA8, i_080ba8),
    (0x080BAA, i_080baa),
    (0x080BAE, i_080bae),
    (0x080BB2, i_080bb2),
    (0x080BB6, i_080bb6),
    (0x080BB8, i_080bb8),
    (0x080BBE, i_080bbe),
    (0x080BC4, i_080bc4),
    (0x080BCA, i_080bca),
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
    (0x080C08, i_080c08),
    (0x080C0E, i_080c0e),
    (0x080C12, i_080c12),
    (0x080C18, i_080c18),
    (0x080C1C, i_080c1c),
    (0x080C24, i_080c24),
    (0x080C2A, i_080c2a),
    (0x080C2E, i_080c2e),
    (0x080C36, i_080c36),
    (0x080C3A, i_080c3a),
    (0x080C42, i_080c42),
    (0x080C48, i_080c48),
    (0x080C4C, i_080c4c),
    (0x080C54, i_080c54),
    (0x080C56, i_080c56),
    (0x080C5A, i_080c5a),
    (0x080C5E, i_080c5e),
    (0x080C62, i_080c62),
    (0x080C66, i_080c66),
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
    (0x080DA6, i_080da6),
    (0x080DAE, i_080dae),
    (0x080DB4, i_080db4),
    (0x080DB8, i_080db8),
    (0x080DC0, i_080dc0),
    (0x080DC2, i_080dc2),
    (0x080DC6, i_080dc6),
    (0x080DCA, i_080dca),
    (0x080DCE, i_080dce),
    (0x080DD2, i_080dd2),
    (0x080DD8, i_080dd8),
    (0x080DDE, i_080dde),
    (0x080DE4, i_080de4),
    (0x080DEA, i_080dea),
    (0x080DEE, i_080dee),
    (0x080DF0, i_080df0),
    (0x080DF4, i_080df4),
    (0x080DFA, i_080dfa),
    (0x080DFE, i_080dfe),
    (0x080E02, i_080e02),
    (0x080E06, i_080e06),
    (0x080E0C, i_080e0c),
    (0x080E10, i_080e10),
    (0x080E14, i_080e14),
    (0x080E1A, i_080e1a),
    (0x080E1E, i_080e1e),
    (0x080E22, i_080e22),
    (0x080E24, i_080e24),
    (0x080E2A, i_080e2a),
    (0x080E30, i_080e30),
    (0x080E36, i_080e36),
    (0x080E3C, i_080e3c),
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
    (0x080F3C, i_080f3c),
    (0x080F40, i_080f40),
    (0x080F46, i_080f46),
    (0x080F4A, i_080f4a),
    (0x080F50, i_080f50),
    (0x080F56, i_080f56),
    (0x080F5C, i_080f5c),
    (0x080F62, i_080f62),
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
    (0x081018, i_081018),
    (0x08101C, i_08101c),
    (0x081022, i_081022),
    (0x081026, i_081026),
    (0x08102C, i_08102c),
    (0x081030, i_081030),
    (0x081034, i_081034),
    (0x08103A, i_08103a),
    (0x081040, i_081040),
    (0x081046, i_081046),
    (0x08104C, i_08104c),
    (0x081050, i_081050),
    (0x081056, i_081056),
    (0x08105C, i_08105c),
    (0x081062, i_081062),
    (0x081068, i_081068),
    (0x08106C, i_08106c),
    (0x081072, i_081072),
    (0x081078, i_081078),
    (0x08107E, i_08107e),
    (0x081084, i_081084),
    (0x081088, i_081088),
    (0x08108E, i_08108e),
    (0x081094, i_081094),
    (0x08109A, i_08109a),
    (0x0810A0, i_0810a0),
    (0x0810A4, i_0810a4),
    (0x0810AA, i_0810aa),
    (0x0810B0, i_0810b0),
    (0x0810B6, i_0810b6),
    (0x0810BC, i_0810bc),
    (0x0810C2, i_0810c2),
    (0x0810C8, i_0810c8),
    (0x0810CE, i_0810ce),
    (0x0810D4, i_0810d4),
    (0x0810DA, i_0810da),
    (0x0810E0, i_0810e0),
    (0x0810E6, i_0810e6),
    (0x0810EC, i_0810ec),
    (0x0810F0, i_0810f0),
    (0x0810F6, i_0810f6),
    (0x0810FC, i_0810fc),
    (0x081102, i_081102),
    (0x081108, i_081108),
    (0x08110E, i_08110e),
    (0x081114, i_081114),
    (0x081118, i_081118),
    (0x08111E, i_08111e),
    (0x081124, i_081124),
    (0x08112A, i_08112a),
    (0x081130, i_081130),
    (0x081136, i_081136),
    (0x08113C, i_08113c),
    (0x081142, i_081142),
    (0x081146, i_081146),
    (0x08114C, i_08114c),
    (0x081152, i_081152),
    (0x081158, i_081158),
    (0x08115E, i_08115e),
    (0x081164, i_081164),
    (0x08116A, i_08116a),
    (0x08116E, i_08116e),
    (0x081174, i_081174),
    (0x08117A, i_08117a),
    (0x081180, i_081180),
    (0x081186, i_081186),
    (0x08118C, i_08118c),
    (0x081192, i_081192),
    (0x081196, i_081196),
    (0x08119C, i_08119c),
    (0x0811A0, i_0811a0),
    (0x0811A4, i_0811a4),
    (0x0811A8, i_0811a8),
    (0x0811AA, i_0811aa),
    (0x0811AE, i_0811ae),
    (0x0811B4, i_0811b4),
    (0x0811B8, i_0811b8),
    (0x0811BE, i_0811be),
    (0x0811C4, i_0811c4),
    (0x0811CA, i_0811ca),
    (0x0811CC, i_0811cc),
    (0x0811D0, i_0811d0),
    (0x0811D6, i_0811d6),
    (0x0811DC, i_0811dc),
    (0x0811E2, i_0811e2),
    (0x0811E4, i_0811e4),
    (0x0811EA, i_0811ea),
    (0x0811EE, i_0811ee),
    (0x0811F2, i_0811f2),
    (0x0811F6, i_0811f6),
    (0x0811F8, i_0811f8),
    (0x0811FC, i_0811fc),
    (0x081202, i_081202),
    (0x081206, i_081206),
    (0x08120C, i_08120c),
    (0x081212, i_081212),
    (0x081214, i_081214),
    (0x08121A, i_08121a),
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
    (0x081332, i_081332),
    (0x081338, i_081338),
    (0x08133C, i_08133c),
    (0x081340, i_081340),
    (0x081344, i_081344),
    (0x081348, i_081348),
    (0x08134A, i_08134a),
    (0x08134C, i_08134c),
    (0x081352, i_081352),
    (0x081358, i_081358),
    (0x08135C, i_08135c),
    (0x081362, i_081362),
    (0x081368, i_081368),
    (0x08136C, i_08136c),
    (0x081372, i_081372),
    (0x081378, i_081378),
    (0x08137C, i_08137c),
    (0x081382, i_081382),
    (0x081388, i_081388),
    (0x08138E, i_08138e),
    (0x081392, i_081392),
    (0x081398, i_081398),
    (0x08139C, i_08139c),
    (0x0813A2, i_0813a2),
    (0x0813A8, i_0813a8),
    (0x0813AC, i_0813ac),
    (0x0813B2, i_0813b2),
    (0x0813B8, i_0813b8),
    (0x0813BE, i_0813be),
    (0x0813C4, i_0813c4),
    (0x0813C8, i_0813c8),
    (0x0813CE, i_0813ce),
    (0x0813D2, i_0813d2),
    (0x0813D8, i_0813d8),
    (0x0813DC, i_0813dc),
    (0x0813E2, i_0813e2),
    (0x0813E8, i_0813e8),
    (0x0813EC, i_0813ec),
    (0x0813F2, i_0813f2),
    (0x0813F8, i_0813f8),
    (0x0813FE, i_0813fe),
    (0x081402, i_081402),
    (0x081408, i_081408),
    (0x08140C, i_08140c),
    (0x081412, i_081412),
    (0x081418, i_081418),
    (0x08141A, i_08141a),
    (0x08141E, i_08141e),
    (0x081424, i_081424),
    (0x08142A, i_08142a),
    (0x08142C, i_08142c),
    (0x081430, i_081430),
    (0x081436, i_081436),
    (0x08143C, i_08143c),
    (0x08143E, i_08143e),
    (0x081442, i_081442),
    (0x081448, i_081448),
    (0x08144E, i_08144e),
    (0x081450, i_081450),
    (0x081454, i_081454),
    (0x08145A, i_08145a),
    (0x081460, i_081460),
    (0x081462, i_081462),
    (0x081468, i_081468),
    (0x08146C, i_08146c),
    (0x081472, i_081472),
    (0x081478, i_081478),
    (0x08147C, i_08147c),
    (0x081482, i_081482),
    (0x081488, i_081488),
    (0x08148E, i_08148e),
    (0x081492, i_081492),
    (0x081498, i_081498),
    (0x08149C, i_08149c),
    (0x0814A2, i_0814a2),
    (0x0814A8, i_0814a8),
    (0x0814AA, i_0814aa),
    (0x0814B0, i_0814b0),
    (0x0814B4, i_0814b4),
    (0x0814B6, i_0814b6),
    (0x0814BC, i_0814bc),
    (0x0814C0, i_0814c0),
    (0x0814C6, i_0814c6),
    (0x0814CA, i_0814ca),
    (0x0814CE, i_0814ce),
    (0x0814D0, i_0814d0),
    (0x0814D2, i_0814d2),
    (0x0814D4, i_0814d4),
    (0x0814D8, i_0814d8),
    (0x0814DA, i_0814da),
    (0x0814DC, i_0814dc),
    (0x0814DE, i_0814de),
    (0x0814E0, i_0814e0),
    (0x0814E4, i_0814e4),
    (0x0814E8, i_0814e8),
    (0x0814EE, i_0814ee),
    (0x0814F2, i_0814f2),
    (0x0814F6, i_0814f6),
    (0x0814FC, i_0814fc),
    (0x081500, i_081500),
    (0x081502, i_081502),
    (0x081506, i_081506),
    (0x081508, i_081508),
    (0x08150A, i_08150a),
    (0x08150E, i_08150e),
    (0x081514, i_081514),
    (0x081518, i_081518),
    (0x08151A, i_08151a),
    (0x08151E, i_08151e),
    (0x081520, i_081520),
    (0x081522, i_081522),
    (0x081526, i_081526),
    (0x08152A, i_08152a),
    (0x08152E, i_08152e),
    (0x081532, i_081532),
    (0x081536, i_081536),
    (0x08153A, i_08153a),
    (0x081540, i_081540),
    (0x081544, i_081544),
    (0x081546, i_081546),
    (0x08154A, i_08154a),
    (0x081550, i_081550),
    (0x081554, i_081554),
    (0x08155A, i_08155a),
    (0x081560, i_081560),
    (0x081564, i_081564),
    (0x081568, i_081568),
    (0x08156C, i_08156c),
    (0x081570, i_081570),
    (0x081572, i_081572),
    (0x081576, i_081576),
    (0x08157A, i_08157a),
    (0x08157C, i_08157c),
    (0x081582, i_081582),
    (0x081586, i_081586),
    (0x08158A, i_08158a),
    (0x081590, i_081590),
    (0x081594, i_081594),
    (0x081598, i_081598),
    (0x08159E, i_08159e),
    (0x0815A2, i_0815a2),
    (0x0815A4, i_0815a4),
    (0x0815A8, i_0815a8),
    (0x0815AA, i_0815aa),
    (0x0815AC, i_0815ac),
    (0x0815B0, i_0815b0),
    (0x0815B6, i_0815b6),
    (0x0815BA, i_0815ba),
    (0x0815BC, i_0815bc),
    (0x0815C0, i_0815c0),
    (0x0815C2, i_0815c2),
    (0x0815C4, i_0815c4),
    (0x0815C8, i_0815c8),
    (0x0815CC, i_0815cc),
    (0x0815D0, i_0815d0),
    (0x0815D4, i_0815d4),
    (0x0815D8, i_0815d8),
    (0x0815DC, i_0815dc),
    (0x0815E2, i_0815e2),
    (0x0815E6, i_0815e6),
    (0x0815E8, i_0815e8),
    (0x0815EE, i_0815ee),
    (0x0815F2, i_0815f2),
    (0x0815F8, i_0815f8),
    (0x0815FC, i_0815fc),
    (0x0815FE, i_0815fe),
    (0x081602, i_081602),
    (0x081606, i_081606),
    (0x081608, i_081608),
    (0x08160C, i_08160c),
    (0x081610, i_081610),
    (0x081614, i_081614),
    (0x081616, i_081616),
    (0x08161C, i_08161c),
    (0x081622, i_081622),
    (0x081628, i_081628),
    (0x08162E, i_08162e),
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
    (0x0817D6, i_0817d6),
    (0x0817DC, i_0817dc),
    (0x0817DE, i_0817de),
    (0x0817E4, i_0817e4),
    (0x0817EA, i_0817ea),
    (0x0817EC, i_0817ec),
    (0x0817F2, i_0817f2),
    (0x0817F8, i_0817f8),
    (0x0817FA, i_0817fa),
    (0x081800, i_081800),
    (0x081806, i_081806),
    (0x081808, i_081808),
    (0x08180E, i_08180e),
    (0x081814, i_081814),
    (0x081816, i_081816),
    (0x08181C, i_08181c),
    (0x081822, i_081822),
    (0x081824, i_081824),
    (0x081828, i_081828),
    (0x08182C, i_08182c),
    (0x081832, i_081832),
    (0x081838, i_081838),
    (0x08183A, i_08183a),
    (0x081840, i_081840),
    (0x081846, i_081846),
    (0x081848, i_081848),
    (0x08184C, i_08184c),
    (0x081850, i_081850),
    (0x081856, i_081856),
    (0x08185C, i_08185c),
    (0x08185E, i_08185e),
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
];
