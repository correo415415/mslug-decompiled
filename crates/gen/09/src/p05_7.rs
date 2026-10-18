// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p05_7) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


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
// ===== InputEvtThunk_05ccd0  ($05CCD0, 18 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05CCD0: move.b #$10, d1
pub fn i_05ccd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CCD4;
    cpu.ir = 0x123C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CCD4: move.w #$2, d0
pub fn i_05ccd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CCD8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CCD8: lea.l $10e200.l, a2
pub fn i_05ccd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CCDE;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E200;
    20
}
/// $05CCDE: bra.w $5cff8
pub fn i_05ccde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CCE2;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFF8;
    10
}
// ===== InputEvtThunk_05cce2  ($05CCE2, 18 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05CCE2: move.b #$20, d1
pub fn i_05cce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CCE6;
    cpu.ir = 0x123C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CCE6: move.w #$2, d0
pub fn i_05cce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CCEA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CCEA: lea.l $10e200.l, a2
pub fn i_05ccea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CCF0;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E200;
    20
}
/// $05CCF0: bra.w $5cff8
pub fn i_05ccf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CCF4;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFF8;
    10
}
// ===== InputEvtThunk_05ccf4  ($05CCF4, 18 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05CCF4: move.b #$40, d1
pub fn i_05ccf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CCF8;
    cpu.ir = 0x123C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CCF8: move.w #$2, d0
pub fn i_05ccf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CCFC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CCFC: lea.l $10e200.l, a2
pub fn i_05ccfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD02;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E200;
    20
}
/// $05CD02: bra.w $5cff8
pub fn i_05cd02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD06;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFF8;
    10
}
// ===== InputEvtThunk_05cd06  ($05CD06, 18 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05CD06: move.b #$80, d1
pub fn i_05cd06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD0A;
    cpu.ir = 0x123C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CD0A: move.w #$2, d0
pub fn i_05cd0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD0E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CD0E: lea.l $10e200.l, a2
pub fn i_05cd0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD14;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E200;
    20
}
/// $05CD14: bra.w $5cff8
pub fn i_05cd14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD18;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFF8;
    10
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
// ===== InputEvtThunk_05cd2a  ($05CD2A, 18 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05CD2A: move.b #$20, d1
pub fn i_05cd2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD2E;
    cpu.ir = 0x123C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CD2E: move.w #$2, d0
pub fn i_05cd2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD32;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CD32: lea.l $10e206.l, a2
pub fn i_05cd32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD38;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E206;
    20
}
/// $05CD38: bra.w $5cff8
pub fn i_05cd38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD3C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFF8;
    10
}
// ===== InputEvtThunk_05cd3c  ($05CD3C, 18 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05CD3C: move.b #$40, d1
pub fn i_05cd3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD40;
    cpu.ir = 0x123C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CD40: move.w #$2, d0
pub fn i_05cd40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD44;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CD44: lea.l $10e206.l, a2
pub fn i_05cd44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD4A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E206;
    20
}
/// $05CD4A: bra.w $5cff8
pub fn i_05cd4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD4E;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFF8;
    10
}
// ===== InputEvtThunk_05cd4e  ($05CD4E, 18 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05CD4E: move.b #$80, d1
pub fn i_05cd4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD52;
    cpu.ir = 0x123C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CD52: move.w #$2, d0
pub fn i_05cd52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD56;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CD56: lea.l $10e206.l, a2
pub fn i_05cd56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD5C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E206;
    20
}
/// $05CD5C: bra.w $5cff8
pub fn i_05cd5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD60;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFF8;
    10
}
// ===== InputEvtThunk_05cd60  ($05CD60, 12 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05CD60: move.b #$10, d1
pub fn i_05cd60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD64;
    cpu.ir = 0x123C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CD64: move.w #$2, d0
pub fn i_05cd64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD68;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CD68: bra.w $5cfa8
pub fn i_05cd68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD6C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
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
// ===== InputEvtThunk_05cd78  ($05CD78, 12 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05CD78: move.b #$40, d1
pub fn i_05cd78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD7C;
    cpu.ir = 0x123C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CD7C: move.w #$2, d0
pub fn i_05cd7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD80;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CD80: bra.w $5cfa8
pub fn i_05cd80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD84;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
    10
}
// ===== InputEvtThunk_05cd84  ($05CD84, 12 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05CD84: move.b #$80, d1
pub fn i_05cd84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD88;
    cpu.ir = 0x123C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CD88: move.w #$2, d0
pub fn i_05cd88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD8C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CD8C: bra.w $5cfa8
pub fn i_05cd8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD90;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
    10
}
// ===== InputEvtThunk_05cd90  ($05CD90, 12 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05CD90: move.b #$f0, d1
pub fn i_05cd90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD94;
    cpu.ir = 0x123C;
    let v1: u32 = 0xF0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CD94: move.w #$2, d0
pub fn i_05cd94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD98;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CD98: bra.w $5cfa8
pub fn i_05cd98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD9C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
    10
}
// ===== InputEvtThunk_05cd9c  ($05CD9C, 12 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05CD9C: move.b #$e0, d1
pub fn i_05cd9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CDA0;
    cpu.ir = 0x123C;
    let v1: u32 = 0xE0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CDA0: move.w #$2, d0
pub fn i_05cda0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CDA4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CDA4: bra.w $5cfa8
pub fn i_05cda4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CDA8;
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
// ===== InputEvtThunk_05cdcc  ($05CDCC, 12 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05CDCC: move.b #$80, d1
pub fn i_05cdcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CDD0;
    cpu.ir = 0x123C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CDD0: move.w #$3, d0
pub fn i_05cdd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CDD4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CDD4: bra.w $5cfa8
pub fn i_05cdd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CDD8;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
    10
}
// ===== InputEvtThunk_05cdd8  ($05CDD8, 12 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05CDD8: move.b #$30, d1
pub fn i_05cdd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CDDC;
    cpu.ir = 0x123C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CDDC: move.w #$3, d0
pub fn i_05cddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CDE0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CDE0: bra.w $5cfa8
pub fn i_05cde0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CDE4;
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
// ===== InputEvtThunk_05cdf0  ($05CDF0, 12 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05CDF0: move.b #$70, d1
pub fn i_05cdf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CDF4;
    cpu.ir = 0x123C;
    let v1: u32 = 0x70;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CDF4: move.w #$3, d0
pub fn i_05cdf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CDF8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CDF8: bra.w $5cfa8
pub fn i_05cdf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CDFC;
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
// ===== InputEvtThunk_05ce08  ($05CE08, 12 B, input/event_thunks_05cdfc.s)
/// $05CE08: move.b #$e0, d1
pub fn i_05ce08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE0C;
    cpu.ir = 0x123C;
    let v1: u32 = 0xE0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CE0C: move.w #$3, d0
pub fn i_05ce0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE10;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CE10: bra.w $5cfa8
pub fn i_05ce10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE14;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
    10
}
// ===== InputEvtThunk_05ce14  ($05CE14, 18 B, input/event_thunks_05cdfc.s)
/// $05CE14: move.b #$10, d1
pub fn i_05ce14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE18;
    cpu.ir = 0x123C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CE18: move.w #$3, d0
pub fn i_05ce18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE1C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CE1C: lea.l $10e200.l, a2
pub fn i_05ce1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE22;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E200;
    20
}
/// $05CE22: bra.w $5cff8
pub fn i_05ce22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE26;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFF8;
    10
}
// ===== InputEvtThunk_05ce26  ($05CE26, 18 B, input/event_thunks_05cdfc.s)
/// $05CE26: move.b #$20, d1
pub fn i_05ce26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE2A;
    cpu.ir = 0x123C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CE2A: move.w #$3, d0
pub fn i_05ce2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE2E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CE2E: lea.l $10e200.l, a2
pub fn i_05ce2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE34;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E200;
    20
}
/// $05CE34: bra.w $5cff8
pub fn i_05ce34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE38;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFF8;
    10
}
// ===== InputEvtThunk_05ce38  ($05CE38, 18 B, input/event_thunks_05cdfc.s)
/// $05CE38: move.b #$40, d1
pub fn i_05ce38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE3C;
    cpu.ir = 0x123C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CE3C: move.w #$3, d0
pub fn i_05ce3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE40;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CE40: lea.l $10e200.l, a2
pub fn i_05ce40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE46;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E200;
    20
}
/// $05CE46: bra.w $5cff8
pub fn i_05ce46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE4A;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFF8;
    10
}
// ===== InputEvtThunk_05ce4a  ($05CE4A, 18 B, input/event_thunks_05cdfc.s)
/// $05CE4A: move.b #$80, d1
pub fn i_05ce4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE4E;
    cpu.ir = 0x123C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CE4E: move.w #$3, d0
pub fn i_05ce4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE52;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CE52: lea.l $10e200.l, a2
pub fn i_05ce52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE58;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E200;
    20
}
/// $05CE58: bra.w $5cff8
pub fn i_05ce58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE5C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFF8;
    10
}
// ===== InputEvtThunk_05ce5c  ($05CE5C, 18 B, input/event_thunks_05cdfc.s)
/// $05CE5C: move.b #$10, d1
pub fn i_05ce5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE60;
    cpu.ir = 0x123C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CE60: move.w #$3, d0
pub fn i_05ce60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE64;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CE64: lea.l $10e206.l, a2
pub fn i_05ce64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE6A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E206;
    20
}
/// $05CE6A: bra.w $5cff8
pub fn i_05ce6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE6E;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFF8;
    10
}
// ===== InputEvtThunk_05ce6e  ($05CE6E, 18 B, input/event_thunks_05cdfc.s)
/// $05CE6E: move.b #$20, d1
pub fn i_05ce6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE72;
    cpu.ir = 0x123C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CE72: move.w #$3, d0
pub fn i_05ce72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE76;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CE76: lea.l $10e206.l, a2
pub fn i_05ce76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE7C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E206;
    20
}
/// $05CE7C: bra.w $5cff8
pub fn i_05ce7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE80;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFF8;
    10
}
// ===== InputEvtThunk_05ce80  ($05CE80, 18 B, input/event_thunks_05cdfc.s)
/// $05CE80: move.b #$40, d1
pub fn i_05ce80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE84;
    cpu.ir = 0x123C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CE84: move.w #$3, d0
pub fn i_05ce84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE88;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CE88: lea.l $10e206.l, a2
pub fn i_05ce88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE8E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E206;
    20
}
/// $05CE8E: bra.w $5cff8
pub fn i_05ce8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE92;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFF8;
    10
}
// ===== InputEvtThunk_05ce92  ($05CE92, 18 B, input/event_thunks_05cdfc.s)
/// $05CE92: move.b #$80, d1
pub fn i_05ce92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE96;
    cpu.ir = 0x123C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CE96: move.w #$3, d0
pub fn i_05ce96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE9A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CE9A: lea.l $10e206.l, a2
pub fn i_05ce9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CEA0;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E206;
    20
}
/// $05CEA0: bra.w $5cff8
pub fn i_05cea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CEA4;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFF8;
    10
}
// ===== InputEvtThunk_05cea4  ($05CEA4, 18 B, input/event_thunks_05cdfc.s)
/// $05CEA4: move.b #$1, d1
pub fn i_05cea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CEA8;
    cpu.ir = 0x123C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CEA8: move.w #$2, d0
pub fn i_05cea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CEAC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CEAC: lea.l $10e200.l, a2
pub fn i_05ceac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CEB2;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E200;
    20
}
/// $05CEB2: bra.w $5cff8
pub fn i_05ceb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CEB6;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFF8;
    10
}
// ===== InputEvtThunk_05ceb6  ($05CEB6, 18 B, input/event_thunks_05cdfc.s)
/// $05CEB6: move.b #$2, d1
pub fn i_05ceb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CEBA;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CEBA: move.w #$2, d0
pub fn i_05ceba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CEBE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CEBE: lea.l $10e200.l, a2
pub fn i_05cebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CEC4;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E200;
    20
}
/// $05CEC4: bra.w $5cff8
pub fn i_05cec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CEC8;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFF8;
    10
}
// ===== InputEvtThunk_05cec8  ($05CEC8, 18 B, input/event_thunks_05cdfc.s)
/// $05CEC8: move.b #$8, d1
pub fn i_05cec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CECC;
    cpu.ir = 0x123C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CECC: move.w #$2, d0
pub fn i_05cecc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CED0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CED0: lea.l $10e200.l, a2
pub fn i_05ced0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CED6;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E200;
    20
}
/// $05CED6: bra.w $5cff8
pub fn i_05ced6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CEDA;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFF8;
    10
}
// ===== InputEvtThunk_05ceda  ($05CEDA, 18 B, input/event_thunks_05cdfc.s)
/// $05CEDA: move.b #$4, d1
pub fn i_05ceda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CEDE;
    cpu.ir = 0x123C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CEDE: move.w #$2, d0
pub fn i_05cede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CEE2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CEE2: lea.l $10e200.l, a2
pub fn i_05cee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CEE8;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E200;
    20
}
/// $05CEE8: bra.w $5cff8
pub fn i_05cee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CEEC;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFF8;
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
// ===== InputEvtThunk_05cf2c  ($05CF2C, 16 B, input/event_thunks_05cdfc.s)
/// $05CF2C: move.b #$1, d1
pub fn i_05cf2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF30;
    cpu.ir = 0x123C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CF30: ori.b #$8, d1
pub fn i_05cf30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF34;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x8) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05CF34: move.w #$2, d0
pub fn i_05cf34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF38;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CF38: bra.w $5cfa8
pub fn i_05cf38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF3C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
    10
}
// ===== InputEvtThunk_05cf3c  ($05CF3C, 16 B, input/event_thunks_05cdfc.s)
/// $05CF3C: move.b #$2, d1
pub fn i_05cf3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF40;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CF40: ori.b #$8, d1
pub fn i_05cf40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF44;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x8) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05CF44: move.w #$2, d0
pub fn i_05cf44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF48;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CF48: bra.w $5cfa8
pub fn i_05cf48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF4C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
    10
}
// ===== InputEvtThunk_05cf4c  ($05CF4C, 16 B, input/event_thunks_05cdfc.s)
/// $05CF4C: move.b #$1, d1
pub fn i_05cf4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF50;
    cpu.ir = 0x123C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CF50: ori.b #$4, d1
pub fn i_05cf50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF54;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x4) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05CF54: move.w #$2, d0
pub fn i_05cf54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF58;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CF58: bra.w $5cfa8
pub fn i_05cf58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF5C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
    10
}
// ===== InputEvtThunk_05cf5c  ($05CF5C, 16 B, input/event_thunks_05cdfc.s)
/// $05CF5C: move.b #$2, d1
pub fn i_05cf5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF60;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CF60: ori.b #$4, d1
pub fn i_05cf60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF64;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x4) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05CF64: move.w #$2, d0
pub fn i_05cf64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF68;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CF68: bra.w $5cfa8
pub fn i_05cf68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF6C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
    10
}
// ===== InputEvtThunk_05cf6c  ($05CF6C, 12 B, input/event_thunks_05cdfc.s)
/// $05CF6C: move.b #$8, d1
pub fn i_05cf6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF70;
    cpu.ir = 0x123C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CF70: move.w #$3, d0
pub fn i_05cf70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF74;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CF74: bra.w $5cfa8
pub fn i_05cf74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF78;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
    10
}
// ===== InputEvtThunk_05cf78  ($05CF78, 12 B, input/event_thunks_05cdfc.s)
/// $05CF78: move.b #$4, d1
pub fn i_05cf78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF7C;
    cpu.ir = 0x123C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CF7C: move.w #$3, d0
pub fn i_05cf7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF80;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CF80: bra.w $5cfa8
pub fn i_05cf80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF84;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
    10
}
// ===== InputEvtThunk_05cf84  ($05CF84, 12 B, input/event_thunks_05cdfc.s)
/// $05CF84: move.b #$1, d1
pub fn i_05cf84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF88;
    cpu.ir = 0x123C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CF88: move.w #$3, d0
pub fn i_05cf88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF8C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CF8C: bra.w $5cfa8
pub fn i_05cf8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF90;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
    10
}
// ===== InputEvtThunk_05cf90  ($05CF90, 12 B, input/event_thunks_05cdfc.s)
/// $05CF90: move.b #$2, d1
pub fn i_05cf90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF94;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CF94: move.w #$3, d0
pub fn i_05cf94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF98;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CF98: bra.w $5cfa8
pub fn i_05cf98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF9C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
    10
}
// ===== InputEvtThunk_05cf9c  ($05CF9C, 12 B, input/event_thunks_05cdfc.s)
/// $05CF9C: move.b #$c, d1
pub fn i_05cf9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CFA0;
    cpu.ir = 0x123C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CFA0: move.w #$3, d0
pub fn i_05cfa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CFA4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CFA4: bra.w $5cfa8
pub fn i_05cfa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CFA8;
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
// ===== InputEvtThunk_05d01a  ($05D01A, 12 B, input/event_thunks_05d00e.s)
/// $05D01A: move.b #$60, d1
pub fn i_05d01a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D01E;
    cpu.ir = 0x123C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05D01E: move.w #$3, d0
pub fn i_05d01e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D022;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D022: bra.w $5d032
pub fn i_05d022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D026;
    cpu.ir = 0x6000;
    cpu.pc = 0x5D032;
    10
}
// ===== InputEvtThunk_05d026  ($05D026, 12 B, input/event_thunks_05d00e.s)
/// $05D026: move.b #$a0, d1
pub fn i_05d026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D02A;
    cpu.ir = 0x123C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05D02A: move.w #$3, d0
pub fn i_05d02a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D02E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D02E: bra.w $5d032
pub fn i_05d02e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D032;
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
// ===== InputEvtThunk_05d0be  ($05D0BE, 18 B, input/event_thunks_05d09a.s)
/// $05D0BE: move.b #$f0, d1
pub fn i_05d0be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D0C2;
    cpu.ir = 0x123C;
    let v1: u32 = 0xF0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05D0C2: move.w #$2, d0
pub fn i_05d0c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D0C6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D0C6: lea.l $10e200.l, a2
pub fn i_05d0c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D0CC;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E200;
    20
}
/// $05D0CC: bra.w $5d0e2
pub fn i_05d0cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D0D0;
    cpu.ir = 0x6000;
    cpu.pc = 0x5D0E2;
    10
}
// ===== InputEvtThunk_05d0d0  ($05D0D0, 18 B, input/event_thunks_05d09a.s)
/// $05D0D0: move.b #$f0, d1
pub fn i_05d0d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D0D4;
    cpu.ir = 0x123C;
    let v1: u32 = 0xF0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05D0D4: move.w #$2, d0
pub fn i_05d0d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D0D8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D0D8: lea.l $10e206.l, a2
pub fn i_05d0d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D0DE;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E206;
    20
}
/// $05D0DE: bra.w $5d0e2
pub fn i_05d0de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D0E2;
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
// ===== InputEvtThunk_05d0f8  ($05D0F8, 18 B, input/event_thunks_05d09a.s)
/// $05D0F8: move.b #$1, d1
pub fn i_05d0f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D0FC;
    cpu.ir = 0x123C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05D0FC: move.w #$2, d0
pub fn i_05d0fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D100;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D100: lea.l $10e206.l, a2
pub fn i_05d100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D106;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E206;
    20
}
/// $05D106: bra.w $5d1c0
pub fn i_05d106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D10A;
    cpu.ir = 0x6000;
    cpu.pc = 0x5D1C0;
    10
}
// ===== InputEvtThunk_05d10a  ($05D10A, 18 B, input/event_thunks_05d09a.s)
/// $05D10A: move.b #$2, d1
pub fn i_05d10a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D10E;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05D10E: move.w #$2, d0
pub fn i_05d10e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D112;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D112: lea.l $10e206.l, a2
pub fn i_05d112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D118;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E206;
    20
}
/// $05D118: bra.w $5d1c0
pub fn i_05d118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D11C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5D1C0;
    10
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
// ===== InputEvtThunk_05d12e  ($05D12E, 18 B, input/event_thunks_05d09a.s)
/// $05D12E: move.b #$4, d1
pub fn i_05d12e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D132;
    cpu.ir = 0x123C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05D132: move.w #$2, d0
pub fn i_05d132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D136;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D136: lea.l $10e206.l, a2
pub fn i_05d136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D13C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E206;
    20
}
/// $05D13C: bra.w $5d1c0
pub fn i_05d13c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D140;
    cpu.ir = 0x6000;
    cpu.pc = 0x5D1C0;
    10
}
// ===== InputEvtThunk_05d140  ($05D140, 12 B, input/event_thunks_05d09a.s)
/// $05D140: move.b #$1, d1
pub fn i_05d140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D144;
    cpu.ir = 0x123C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05D144: move.w #$2, d0
pub fn i_05d144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D148;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D148: bra.w $5d170
pub fn i_05d148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D14C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5D170;
    10
}
// ===== InputEvtThunk_05d14c  ($05D14C, 12 B, input/event_thunks_05d09a.s)
/// $05D14C: move.b #$2, d1
pub fn i_05d14c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D150;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05D150: move.w #$2, d0
pub fn i_05d150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D154;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D154: bra.w $5d170
pub fn i_05d154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D158;
    cpu.ir = 0x6000;
    cpu.pc = 0x5D170;
    10
}
// ===== InputEvtThunk_05d158  ($05D158, 12 B, input/event_thunks_05d09a.s)
/// $05D158: move.b #$8, d1
pub fn i_05d158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D15C;
    cpu.ir = 0x123C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05D15C: move.w #$2, d0
pub fn i_05d15c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D160;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D160: bra.w $5d170
pub fn i_05d160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D164;
    cpu.ir = 0x6000;
    cpu.pc = 0x5D170;
    10
}
// ===== InputEvtThunk_05d164  ($05D164, 12 B, input/event_thunks_05d09a.s)
/// $05D164: move.b #$4, d1
pub fn i_05d164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D168;
    cpu.ir = 0x123C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05D168: move.w #$2, d0
pub fn i_05d168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D16C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D16C: bra.w $5d170
pub fn i_05d16c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D170;
    cpu.ir = 0x6000;
    cpu.pc = 0x5D170;
    10
}
// ===== InputMask_CheckChannelAvail_05d170  ($05D170, 32 B, input/check_channel_05d170.s)
/// $05D170: btst.b #$7, $100000.l
pub fn i_05d170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D178;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((7) % 8);
    let v2 = bus.read8(0x100000) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $05D178: bne.w $5d190
pub fn i_05d178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D17C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5D190; return 10; }
    8
}
/// $05D17C: lea.l $1001c0.l, a4
pub fn i_05d17c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D182;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0x1001C0;
    20
}
/// $05D182: tst.b $44(a4)
pub fn i_05d182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D186;
    cpu.ir = 0x4A2C;
    let ad2 = cpu.a[4].wrapping_add(0x44);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $05D186: beq.w $5d190
pub fn i_05d186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D18A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5D190; return 10; }
    8
}
/// $05D18A: andi.b #$ee, ccr
pub fn i_05d18a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D18E;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05D18E: rts
pub fn i_05d18e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D190;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== InputMask_ReadCtxSwitchPlayer_05d190  ($05D190, 48 B, input/ctx_switch_05d190.s)
/// $05D190: lea.l $5cc08(pc), a2
pub fn i_05d190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D194;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0x5CC08;
    16
}
/// $05D194: cmpi.b #$1, $6d(a6)
pub fn i_05d194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D19A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x6D);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $05D19A: bne.w $5d1a8
pub fn i_05d19a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D19E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5D1A8; return 10; }
    8
}
/// $05D19E: lea.l $100300.l, a1
pub fn i_05d19e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D1A4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x100300;
    20
}
/// $05D1A4: movea.l $72(a1), a2
pub fn i_05d1a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D1A8;
    cpu.ir = 0x2469;
    let ad2 = cpu.a[1].wrapping_add(0x72);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $05D1A8: cmpi.b #$2, $6d(a6)
pub fn i_05d1a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D1AE;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x6D);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $05D1AE: bne.w $5d1bc
pub fn i_05d1ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D1B2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5D1BC; return 10; }
    8
}
/// $05D1B2: lea.l $1003a0.l, a1
pub fn i_05d1b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D1B8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1003A0;
    20
}
/// $05D1B8: movea.l $72(a1), a2
pub fn i_05d1b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D1BC;
    cpu.ir = 0x2469;
    let ad2 = cpu.a[1].wrapping_add(0x72);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $05D1BC: jsr $5d674(pc)
pub fn i_05d1bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D1C0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5D1C0);
    cpu.pc = 0x5D674;
    16
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
// ===== InputEvt_Thunk4aThenMask60_05d1da  ($05D1DA, 42 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05D1DA: jsr $5ce4a(pc)
pub fn i_05d1da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D1DE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5D1DE);
    cpu.pc = 0x5CE4A;
    16
}
/// $05D1DE: bcc.w $5d204
pub fn i_05d1de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D1E2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5D204; return 10; }
    8
}
/// $05D1E2: move.b #$20, d1
pub fn i_05d1e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D1E6;
    cpu.ir = 0x123C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05D1E6: ori.b #$40, d1
pub fn i_05d1e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D1EA;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x40) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05D1EA: move.w #$2, d0
pub fn i_05d1ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D1EE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D1EE: lea.l $10e206.l, a2
pub fn i_05d1ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D1F4;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E206;
    20
}
/// $05D1F4: bra.w $5d1f8
pub fn i_05d1f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D1F8;
    cpu.ir = 0x6000;
    cpu.pc = 0x5D1F8;
    10
}
/// $05D1F8: move.b (a2, d0.w), d0
pub fn i_05d1f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D1FC;
    cpu.ir = 0x1032;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $05D1FC: and.b d1, d0
pub fn i_05d1fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D1FE;
    cpu.ir = 0xC001;
    let v1 = cpu.d[1] & 0xFF;
    let r = (v1 & cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05D1FE: cmp.b d1, d0
pub fn i_05d1fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D200;
    cpu.ir = 0xB001;
    let v1 = cpu.d[1] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $05D200: beq.w $5d20a
pub fn i_05d200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D204;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5D20A; return 10; }
    8
}
// ===== ClearXN_05d204  ($05D204, 6 B, thunks/ccr_helpers.c)
/// $05D204: andi.b #$ee, ccr
pub fn i_05d204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D208;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05D208: rts
pub fn i_05d208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D20A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_05d20a  ($05D20A, 6 B, thunks/ccr_helpers.c)
/// $05D20A: ori.b #$11, ccr
pub fn i_05d20a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D20E;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05D20E: rts
pub fn i_05d20e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D210;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== InputEvt_Thunk4aThenMask40_05d210  ($05D210, 36 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05D210: jsr $5ce4a(pc)
pub fn i_05d210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D214;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5D214);
    cpu.pc = 0x5CE4A;
    16
}
/// $05D214: bcc.b $5d204
pub fn i_05d214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D216;
    cpu.ir = 0x64EE;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5D204; return 10; }
    8
}
/// $05D216: move.b #$40, d1
pub fn i_05d216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D21A;
    cpu.ir = 0x123C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05D21A: move.w #$2, d0
pub fn i_05d21a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D21E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D21E: lea.l $10e206.l, a2
pub fn i_05d21e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D224;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E206;
    20
}
/// $05D224: bra.w $5d228
pub fn i_05d224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D228;
    cpu.ir = 0x6000;
    cpu.pc = 0x5D228;
    10
}
/// $05D228: move.b (a2, d0.w), d0
pub fn i_05d228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D22C;
    cpu.ir = 0x1032;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $05D22C: and.b d1, d0
pub fn i_05d22c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D22E;
    cpu.ir = 0xC001;
    let v1 = cpu.d[1] & 0xFF;
    let r = (v1 & cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05D22E: cmp.b d1, d0
pub fn i_05d22e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D230;
    cpu.ir = 0xB001;
    let v1 = cpu.d[1] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $05D230: beq.w $5d23a
pub fn i_05d230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D234;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5D23A; return 10; }
    8
}
// ===== ClearXN_05d234  ($05D234, 6 B, thunks/ccr_helpers.c)
/// $05D234: andi.b #$ee, ccr
pub fn i_05d234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D238;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05D238: rts
pub fn i_05d238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D23A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_05d23a  ($05D23A, 6 B, thunks/ccr_helpers.c)
/// $05D23A: ori.b #$11, ccr
pub fn i_05d23a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D23E;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05D23E: rts
pub fn i_05d23e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D240;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== InputEvt_Thunk4aThenToggle_05d240  ($05D240, 72 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05D240: jsr $5ce4a(pc)
pub fn i_05d240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D244;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5D244);
    cpu.pc = 0x5CE4A;
    16
}
/// $05D244: bcs.w $5d24c
pub fn i_05d244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D248;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5D24C; return 10; }
    8
}
/// $05D248: bra.w $5d2ee
pub fn i_05d248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D24C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5D2EE;
    10
}
/// $05D24C: move.b #$8, d1
pub fn i_05d24c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D250;
    cpu.ir = 0x123C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05D250: move.w #$0, d0
pub fn i_05d250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D254;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D254: bra.w $5d2de
pub fn i_05d254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D258;
    cpu.ir = 0x6000;
    cpu.pc = 0x5D2DE;
    10
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
/// $05D2DE: lea.l $10e20c.l, a2
pub fn i_05d2de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D2E4;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E20C;
    20
}
/// $05D2E4: move.b (a2, d0.w), d0
pub fn i_05d2e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D2E8;
    cpu.ir = 0x1032;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $05D2E8: eor.b d1, d0
pub fn i_05d2e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D2EA;
    cpu.ir = 0xB300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 ^ cpu.d[1]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05D2EA: beq.w $5d2f8
pub fn i_05d2ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D2EE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5D2F8; return 10; }
    8
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
/// $05D2F8: move.b #$0, d0
pub fn i_05d2f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D2FC;
    cpu.ir = 0x103C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05D2FC: subi.b #$1, d0
pub fn i_05d2fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D300;
    cpu.ir = 0x400;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05D300: rts
pub fn i_05d300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D302;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== InputEvt_ThunkMaskF0_05d302  ($05D302, 8 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05D302: jsr $5cd90(pc)
pub fn i_05d302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D306;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5D306);
    cpu.pc = 0x5CD90;
    16
}
/// $05D306: bcc.w $5d310
pub fn i_05d306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D30A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5D310; return 10; }
    8
}
// ===== JsrPcThunk_05d30a  ($05D30A, 6 B, thunks/jsr_pc_thunks.c)
/// $05D30A: jsr $5cf6c(pc)
pub fn i_05d30a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D30E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5D30E);
    cpu.pc = 0x5CF6C;
    16
}
/// $05D30E: rts
pub fn i_05d30e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D310;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_05d310  ($05D310, 6 B, thunks/ccr_helpers.c)
/// $05D310: andi.b #$ee, ccr
pub fn i_05d310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D314;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05D314: rts
pub fn i_05d314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D316;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AimDirRows_05D316  ($05D316, 16 B, input/aim_tables_05d316.s)
/// $05D316: dc.w $ff08
pub fn i_05d316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D318;
    cpu.ir = 0xFF08;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
// ===== SpawnRows_05D586  ($05D586, 16 B, input/aim_tables_05d316.s)
/// $05D586: ori.b #$ff, d1
pub fn i_05d586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D58A;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0xFF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05D58A: ori.b #$ff, d1
pub fn i_05d58a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D58E;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0xFF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05D58E: ori.b #$ff, d0
pub fn i_05d58e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D592;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0xFF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05D592: dc.w $ffff
pub fn i_05d592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D594;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
// ===== SpawnRows_05D596  ($05D596, 16 B, input/aim_tables_05d316.s)
/// $05D596: ori.b #$ff, d1
pub fn i_05d596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D59A;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0xFF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05D59A: ori.b #$ff, d0
pub fn i_05d59a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D59E;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0xFF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05D59E: ori.b #$ff, d1
pub fn i_05d59e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D5A2;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0xFF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05D5A2: dc.w $ffff
pub fn i_05d5a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D5A4;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
// ===== SpawnRows_05D5A6  ($05D5A6, 16 B, input/aim_tables_05d316.s)
/// $05D5A6: ori.b #$ff, d1
pub fn i_05d5a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D5AA;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0xFF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05D5AA: ori.b #$ff, d0
pub fn i_05d5aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D5AE;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0xFF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05D5AE: ori.b #$ff, d0
pub fn i_05d5ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D5B2;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0xFF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05D5B2: dc.w $ffff
pub fn i_05d5b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D5B4;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
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
// ===== InputLayout_ReadField0_05D616  ($05D616, 94 B, input/aim_tables_05d316.s)
/// $05D616: move.w #$2, d1
pub fn i_05d616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D61A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D61A: lea.l $5cc08(pc), a2
pub fn i_05d61a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D61E;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0x5CC08;
    16
}
/// $05D61E: cmpi.b #$1, $6d(a6)
pub fn i_05d61e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D624;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x6D);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $05D624: bne.w $5d632
pub fn i_05d624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D628;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5D632; return 10; }
    8
}
/// $05D628: movea.l $72(a1), a2
pub fn i_05d628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D62C;
    cpu.ir = 0x2469;
    let ad2 = cpu.a[1].wrapping_add(0x72);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $05D62C: lea.l $100300.l, a1
pub fn i_05d62c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D632;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x100300;
    20
}
/// $05D632: cmpi.b #$2, $6d(a6)
pub fn i_05d632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D638;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x6D);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $05D638: bne.w $5d646
pub fn i_05d638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D63C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5D646; return 10; }
    8
}
/// $05D63C: lea.l $1003a0.l, a1
pub fn i_05d63c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D642;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1003A0;
    20
}
/// $05D642: movea.l $72(a1), a2
pub fn i_05d642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D646;
    cpu.ir = 0x2469;
    let ad2 = cpu.a[1].wrapping_add(0x72);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $05D646: moveq #$0, d0
pub fn i_05d646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D648;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D648: btst.b #$7, $100000.l
pub fn i_05d648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D650;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((7) % 8);
    let v2 = bus.read8(0x100000) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $05D650: bne.w $5d666
pub fn i_05d650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D654;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5D666; return 10; }
    8
}
/// $05D654: lea.l $1001c0.l, a4
pub fn i_05d654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D65A;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0x1001C0;
    20
}
/// $05D65A: tst.b $44(a4)
pub fn i_05d65a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D65E;
    cpu.ir = 0x4A2C;
    let ad2 = cpu.a[4].wrapping_add(0x44);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $05D65E: beq.w $5d666
pub fn i_05d65e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D662;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5D666; return 10; }
    8
}
/// $05D662: nop
pub fn i_05d662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D664;
    cpu.ir = 0x4E71;
    4
}
/// $05D664: rts
pub fn i_05d664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D666;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05D666: jsr $5d674(pc)
pub fn i_05d666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D66A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5D66A);
    cpu.pc = 0x5D674;
    16
}
/// $05D66A: move.b (a2, d1.w), d0
pub fn i_05d66a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D66E;
    cpu.ir = 0x1032;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $05D66E: andi.b #$3, d0
pub fn i_05d66e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D672;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05D672: rts
pub fn i_05d672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D674;
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
// ===== DebugHex_SetupA1_05D6A0  ($05D6A0, 10 B, input/aim_tables_05d316.s)
/// $05D6A0: movea.l #$7063, a1
pub fn i_05d6a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6A6;
    cpu.ir = 0x227C;
    let v1: u32 = 0x7063;
    cpu.a[1] = v1;
    4
}
/// $05D6A6: move.w #$300, d1
pub fn i_05d6a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6AA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrPcThunk_05d6aa  ($05D6AA, 6 B, thunks/jsr_pc_thunks.c)
/// $05D6AA: jsr $5d6c2(pc)
pub fn i_05d6aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6AE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5D6AE);
    cpu.pc = 0x5D6C2;
    16
}
/// $05D6AE: rts
pub fn i_05d6ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6B0;
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
// ===== HEX_TABLE_5D71C  ($05D71C, 16 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05D71C: move.w $33(a1, d3.w), d0
pub fn i_05d71c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D720;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x33).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $05D720: move.w $37(a5, d3.w), d2
pub fn i_05d720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D724;
    cpu.ir = 0x3435;
    let ad2 = cpu.a[5].wrapping_add(0x37).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $05D724: move.w $41424344.l, d4
pub fn i_05d724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D72A;
    cpu.ir = 0x3839;
    let v2 = bus.read16(0x41424344) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $05D72A: ?
pub fn i_05d72a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D72C;
    cpu.ir = 0x4546;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
// ===== Debug_HexDrawToFix8_05d7be  ($05D7BE, 26 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05D7BE: swap d1
pub fn i_05d7be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D7C0;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D7C0: lsr.l #$4, d1
pub fn i_05d7c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D7C2;
    cpu.ir = 0xE889;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Long, 1, false);
    cpu.d[1] = r;
    14
}
/// $05D7C2: andi.w #$f000, d1
pub fn i_05d7c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D7C6;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0xF000) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05D7C6: lea.l $10e21e.l, a2
pub fn i_05d7c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D7CC;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E21E;
    20
}
/// $05D7CC: moveq #$1c, d2
pub fn i_05d7cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D7CE;
    cpu.ir = 0x741C;
    cpu.d[2] = 0x1C;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D7CE: move.w #$8, d3
pub fn i_05d7ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D7D2;
    cpu.ir = 0x363C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D7D2: move.w d3, -(a7)
pub fn i_05d7d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D7D4;
    cpu.ir = 0x3F03;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $05D7D4: bra.w $5d7ee
pub fn i_05d7d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D7D8;
    cpu.ir = 0x6000;
    cpu.pc = 0x5D7EE;
    10
}
// ===== Debug_HexDrawToFix4_05d7d8  ($05D7D8, 140 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05D7D8: swap d1
pub fn i_05d7d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D7DA;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D7DA: lsr.l #$4, d1
pub fn i_05d7da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D7DC;
    cpu.ir = 0xE889;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Long, 1, false);
    cpu.d[1] = r;
    14
}
/// $05D7DC: andi.w #$f000, d1
pub fn i_05d7dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D7E0;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0xF000) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05D7E0: lea.l $10e21e.l, a2
pub fn i_05d7e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D7E6;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E21E;
    20
}
/// $05D7E6: moveq #$c, d2
pub fn i_05d7e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D7E8;
    cpu.ir = 0x740C;
    cpu.d[2] = 0xC;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D7E8: move.w #$4, d3
pub fn i_05d7e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D7EC;
    cpu.ir = 0x363C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D7EC: move.w d3, -(a7)
pub fn i_05d7ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D7EE;
    cpu.ir = 0x3F03;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $05D7EE: move.l d2, -(a7)
pub fn i_05d7ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D7F0;
    cpu.ir = 0x2F02;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $05D7F0: move.l d0, d3
pub fn i_05d7f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D7F2;
    cpu.ir = 0x2600;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[3] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D7F2: lsr.l d2, d3
pub fn i_05d7f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D7F4;
    cpu.ir = 0xE4AB;
    let cnt = cpu.d[2] & 0x3F;
    let h1 = cpu.d[3] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, cnt, Size::Long, 1, false);
    cpu.d[3] = r;
    6 + 2 * cnt
}
/// $05D7F4: andi.l #$f, d3
pub fn i_05d7f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D7FA;
    cpu.ir = 0x283;
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    let r2 = (v1 & 0xF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[3] = r2;
    8
}
/// $05D7FA: move.w d3, (a2)
pub fn i_05d7fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D7FC;
    cpu.ir = 0x3483;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[2], v1 as u16);
    8
}
/// $05D7FC: addq.l #$2, a2
pub fn i_05d7fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D7FE;
    cpu.ir = 0x548A;
    cpu.a[2] = cpu.a[2].wrapping_add(2);
    8
}
/// $05D7FE: move.l (a7)+, d2
pub fn i_05d7fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D800;
    cpu.ir = 0x241F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05D800: subq.l #$4, d2
pub fn i_05d800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D802;
    cpu.ir = 0x5982;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let r2 = sub_with_flags(cpu, v1, 4, Size::Long, true);
    cpu.d[2] = r2;
    4
}
/// $05D802: bne.b $5d7ee
pub fn i_05d802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D804;
    cpu.ir = 0x66EA;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5D7EE; return 10; }
    8
}
/// $05D804: andi.l #$f, d0
pub fn i_05d804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D80A;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $05D80A: move.w d0, (a2)
pub fn i_05d80a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D80C;
    cpu.ir = 0x3480;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[2], v1 as u16);
    8
}
/// $05D80C: move.w (a7)+, d3
pub fn i_05d80c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D80E;
    cpu.ir = 0x361F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05D80E: subq.w #$1, d3
pub fn i_05d80e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D810;
    cpu.ir = 0x5343;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05D810: clr.w d2
pub fn i_05d810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D812;
    cpu.ir = 0x4242;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05D812: lea.l $10e21e.l, a2
pub fn i_05d812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D818;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E21E;
    20
}
/// $05D818: lea.l $5d864(pc), a3
pub fn i_05d818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D81C;
    cpu.ir = 0x47FA;
    cpu.a[3] = 0x5D864;
    16
}
/// $05D81C: move.w (a2)+, d4
pub fn i_05d81c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D81E;
    cpu.ir = 0x381A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05D81E: tst.w d4
pub fn i_05d81e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D820;
    cpu.ir = 0x4A44;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05D820: bne.w $5d838
pub fn i_05d820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D824;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5D838; return 10; }
    8
}
/// $05D824: tst.w d2
pub fn i_05d824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D826;
    cpu.ir = 0x4A42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05D826: bne.w $5d838
pub fn i_05d826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D82A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5D838; return 10; }
    8
}
/// $05D82A: tst.w d3
pub fn i_05d82a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D82C;
    cpu.ir = 0x4A43;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05D82C: beq.w $5d838
pub fn i_05d82c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D830;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5D838; return 10; }
    8
}
/// $05D830: move.w #$10, d4
pub fn i_05d830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D834;
    cpu.ir = 0x383C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D834: bra.w $5d83c
pub fn i_05d834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D838;
    cpu.ir = 0x6000;
    cpu.pc = 0x5D83C;
    10
}
/// $05D838: move.w #$ffff, d2
pub fn i_05d838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D83C;
    cpu.ir = 0x343C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D83C: add.w d4, d4
pub fn i_05d83c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D83E;
    cpu.ir = 0xD844;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05D83E: move.w (a3, d4.w), d0
pub fn i_05d83e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D842;
    cpu.ir = 0x3033;
    let ad2 = cpu.a[3].wrapping_add(0x0).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $05D842: add.w d1, d0
pub fn i_05d842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D844;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05D844: movem.l d1-d4/a1-a3, -(a7)
pub fn i_05d844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D848;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $05D848: move.w #$2, d1
pub fn i_05d848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D84C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D84C: move.w #$2, d2
pub fn i_05d84c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D850;
    cpu.ir = 0x343C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D850: jsr $5da56.l
pub fn i_05d850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D856;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5D856);
    cpu.pc = 0x5DA56;
    16
}
/// $05D856: movem.l (a7)+, d1-d4/a1-a3
pub fn i_05d856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D85A;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $05D85A: adda.w #$40, a1
pub fn i_05d85a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D85E;
    cpu.ir = 0xD2FC;
    let v1: u32 = 0x40;
    cpu.a[1] = cpu.a[1].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05D85E: dbra d3, $5d81c
pub fn i_05d85e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D862;
    cpu.ir = 0x51CB;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5D862; return 12; }
    let n = (cpu.d[3] as u16).wrapping_sub(1);
    cpu.d[3] = (cpu.d[3] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5D862; return 14; }
    cpu.pc = 0x5D81C;
    10
}
/// $05D862: rts
pub fn i_05d862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D864;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Bin16_ToBcd4_05d886  ($05D886, 108 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05D886: movem.l d1-d3, -(a7)
pub fn i_05d886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D88A;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $05D88A: moveq #$0, d3
pub fn i_05d88a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D88C;
    cpu.ir = 0x7600;
    cpu.d[3] = 0x0;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D88C: move.w #$a, d2
pub fn i_05d88c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D890;
    cpu.ir = 0x343C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D890: andi.l #$ffff, d0
pub fn i_05d890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D896;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $05D896: beq.w $5d8ea
pub fn i_05d896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D89A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5D8EA; return 10; }
    8
}
/// $05D89A: divu.w d2, d0
pub fn i_05d89a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D89C;
    cpu.ir = 0x80C2;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set(StatusRegister::C, false);
    let dv = v1 & 0xFFFF;
    if dv == 0 { cpu.enter_group2_exception(bus, Exception::DivideByZero); return 38; }
    let q = cpu.d[0] / dv; let rm = cpu.d[0] % dv;
    if q > 0xFFFF { cpu.sr.set(StatusRegister::V, true); cpu.sr.set(StatusRegister::N, true); cpu.sr.set(StatusRegister::Z, false); return 76; }
    cpu.d[0] = (rm << 16) | (q & 0xFFFF);
    cpu.sr.set_nz(q, Size::Word); cpu.sr.set(StatusRegister::V, false);
    140
}
/// $05D89C: swap d0
pub fn i_05d89c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D89E;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D89E: andi.w #$f, d0
pub fn i_05d89e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8A2;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05D8A2: move.w d0, d3
pub fn i_05d8a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8A4;
    cpu.ir = 0x3600;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D8A4: swap d0
pub fn i_05d8a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8A6;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D8A6: andi.l #$ffff, d0
pub fn i_05d8a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8AC;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $05D8AC: beq.w $5d8ea
pub fn i_05d8ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8B0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5D8EA; return 10; }
    8
}
/// $05D8B0: divu.w d2, d0
pub fn i_05d8b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8B2;
    cpu.ir = 0x80C2;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set(StatusRegister::C, false);
    let dv = v1 & 0xFFFF;
    if dv == 0 { cpu.enter_group2_exception(bus, Exception::DivideByZero); return 38; }
    let q = cpu.d[0] / dv; let rm = cpu.d[0] % dv;
    if q > 0xFFFF { cpu.sr.set(StatusRegister::V, true); cpu.sr.set(StatusRegister::N, true); cpu.sr.set(StatusRegister::Z, false); return 76; }
    cpu.d[0] = (rm << 16) | (q & 0xFFFF);
    cpu.sr.set_nz(q, Size::Word); cpu.sr.set(StatusRegister::V, false);
    140
}
/// $05D8B2: swap d0
pub fn i_05d8b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8B4;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D8B4: andi.w #$f, d0
pub fn i_05d8b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8B8;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05D8B8: lsl.w #$4, d0
pub fn i_05d8b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8BA;
    cpu.ir = 0xE948;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $05D8BA: or.w d0, d3
pub fn i_05d8ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8BC;
    cpu.ir = 0x8640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = (v1 | cpu.d[3]) & 0xFFFF;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05D8BC: swap d0
pub fn i_05d8bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8BE;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D8BE: andi.l #$ffff, d0
pub fn i_05d8be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8C4;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $05D8C4: beq.w $5d8ea
pub fn i_05d8c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8C8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5D8EA; return 10; }
    8
}
/// $05D8C8: divu.w d2, d0
pub fn i_05d8c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8CA;
    cpu.ir = 0x80C2;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set(StatusRegister::C, false);
    let dv = v1 & 0xFFFF;
    if dv == 0 { cpu.enter_group2_exception(bus, Exception::DivideByZero); return 38; }
    let q = cpu.d[0] / dv; let rm = cpu.d[0] % dv;
    if q > 0xFFFF { cpu.sr.set(StatusRegister::V, true); cpu.sr.set(StatusRegister::N, true); cpu.sr.set(StatusRegister::Z, false); return 76; }
    cpu.d[0] = (rm << 16) | (q & 0xFFFF);
    cpu.sr.set_nz(q, Size::Word); cpu.sr.set(StatusRegister::V, false);
    140
}
/// $05D8CA: swap d0
pub fn i_05d8ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8CC;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D8CC: andi.w #$f, d0
pub fn i_05d8cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8D0;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05D8D0: lsl.w #$8, d0
pub fn i_05d8d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8D2;
    cpu.ir = 0xE148;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $05D8D2: or.w d0, d3
pub fn i_05d8d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8D4;
    cpu.ir = 0x8640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = (v1 | cpu.d[3]) & 0xFFFF;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05D8D4: swap d0
pub fn i_05d8d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8D6;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D8D6: andi.l #$ffff, d0
pub fn i_05d8d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8DC;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $05D8DC: divu.w d2, d0
pub fn i_05d8dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8DE;
    cpu.ir = 0x80C2;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set(StatusRegister::C, false);
    let dv = v1 & 0xFFFF;
    if dv == 0 { cpu.enter_group2_exception(bus, Exception::DivideByZero); return 38; }
    let q = cpu.d[0] / dv; let rm = cpu.d[0] % dv;
    if q > 0xFFFF { cpu.sr.set(StatusRegister::V, true); cpu.sr.set(StatusRegister::N, true); cpu.sr.set(StatusRegister::Z, false); return 76; }
    cpu.d[0] = (rm << 16) | (q & 0xFFFF);
    cpu.sr.set_nz(q, Size::Word); cpu.sr.set(StatusRegister::V, false);
    140
}
/// $05D8DE: swap d0
pub fn i_05d8de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8E0;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D8E0: andi.w #$f, d0
pub fn i_05d8e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8E4;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05D8E4: lsl.w #$8, d0
pub fn i_05d8e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8E6;
    cpu.ir = 0xE148;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $05D8E6: lsl.w #$4, d0
pub fn i_05d8e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8E8;
    cpu.ir = 0xE948;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $05D8E8: or.w d0, d3
pub fn i_05d8e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8EA;
    cpu.ir = 0x8640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = (v1 | cpu.d[3]) & 0xFFFF;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05D8EA: move.w d3, d0
pub fn i_05d8ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8EC;
    cpu.ir = 0x3003;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D8EC: movem.l (a7)+, d1-d3
pub fn i_05d8ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8F0;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $05D8F0: rts
pub fn i_05d8f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8F2;
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
// ===== Noise_LookupByIndex_05d946  ($05D946, 16 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05D946: move.w $10e22e.l, d0
pub fn i_05d946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D94C;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x10E22E) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $05D94C: andi.w #$ff, d0
pub fn i_05d94c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D950;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05D950: move.b $5d956(pc, d0.w), d0
pub fn i_05d950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D954;
    cpu.ir = 0x103B;
    let ad2 = 0x5D956_u32.wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $05D954: rts
pub fn i_05d954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D956;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== NoiseLut256_05d956  ($05D956, 256 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05D956: movea.w $3b(a2, a4.l), a2
pub fn i_05d956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D95A;
    cpu.ir = 0x3472;
    let ad2 = cpu.a[2].wrapping_add(0x3B).wrapping_add(cpu.a[4]);
    let v3 = bus.read16(ad2) as u32;
    cpu.a[2] = v3 as i16 as i32 as u32;
    14
}
/// $05D95A: divs.w d5, d1
pub fn i_05d95a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D95C;
    cpu.ir = 0x83C5;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set(StatusRegister::C, false);
    let dv = v1 as u16 as i16 as i32;
    if dv == 0 { cpu.enter_group2_exception(bus, Exception::DivideByZero); return 38; }
    let dd = cpu.d[1] as i32; let q = dd.wrapping_div(dv); let rm = dd.wrapping_rem(dv);
    if !(-32768..=32767).contains(&q) { cpu.sr.set(StatusRegister::V, true); cpu.sr.set(StatusRegister::N, true); cpu.sr.set(StatusRegister::Z, false); return 158; }
    cpu.d[1] = ((rm as u32 & 0xFFFF) << 16) | (q as u32 & 0xFFFF);
    cpu.sr.set_nz(q as u32 & 0xFFFF, Size::Word); cpu.sr.set(StatusRegister::V, false);
    158
}
/// $05D95C: sub.w a2, d1
pub fn i_05d95c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D95E;
    cpu.ir = 0x924A;
    let v1 = cpu.a[2] as i16 as i32 as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05D95E: or.w $8e6b0bd3.l, d2
pub fn i_05d95e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D964;
    cpu.ir = 0x8479;
    let v2 = bus.read16(0x8E6B0BD3) as u32;
    let r = (v2 | cpu.d[2]) & 0xFFFF;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    16
}
/// $05D964: divs.w d7, d6
pub fn i_05d964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D966;
    cpu.ir = 0x8DC7;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set(StatusRegister::C, false);
    let dv = v1 as u16 as i16 as i32;
    if dv == 0 { cpu.enter_group2_exception(bus, Exception::DivideByZero); return 38; }
    let dd = cpu.d[6] as i32; let q = dd.wrapping_div(dv); let rm = dd.wrapping_rem(dv);
    if !(-32768..=32767).contains(&q) { cpu.sr.set(StatusRegister::V, true); cpu.sr.set(StatusRegister::N, true); cpu.sr.set(StatusRegister::Z, false); return 158; }
    cpu.d[6] = ((rm as u32 & 0xFFFF) << 16) | (q as u32 & 0xFFFF);
    cpu.sr.set_nz(q as u32 & 0xFFFF, Size::Word); cpu.sr.set(StatusRegister::V, false);
    158
}
/// $05D966: movem.w (a3)+, d0-d1/d6-d7/a1-a3/a6
pub fn i_05d966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D96A;
    cpu.ir = 0x4C9B;
    let mut a = cpu.a[3];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[1] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[6] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[7] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[1] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[2] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[3] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[6] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[3] = a;
    20
}
/// $05D96A: ?
pub fn i_05d96a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D96C;
    cpu.ir = 0x4D68;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
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
// ===== FixLayer_MultiRowBlit_05DB1A  ($05DB1A, 34 B, video/fix_layer_backends_05db1a.s)
/// $05DB1A: move.w d0, d3
pub fn i_05db1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB1C;
    cpu.ir = 0x3600;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05DB1C: move.w (a2)+, d0
pub fn i_05db1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB1E;
    cpu.ir = 0x301A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05DB1E: cmpi.w #$ffff, d0
pub fn i_05db1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB22;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFFF, Size::Word, false);
    8
}
/// $05DB22: bne.w $5db28
pub fn i_05db22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB26;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5DB28; return 10; }
    8
}
/// $05DB26: rts
pub fn i_05db26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB28;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05DB28: or.w d3, d0
pub fn i_05db28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB2A;
    cpu.ir = 0x8043;
    let v1 = cpu.d[3] & 0xFFFF;
    let r = (v1 | cpu.d[0]) & 0xFFFF;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05DB2A: movem.l d1-d3/a1, -(a7)
pub fn i_05db2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB2E;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $05DB2E: jsr $5da56(pc)
pub fn i_05db2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB32;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5DB32);
    cpu.pc = 0x5DA56;
    16
}
/// $05DB32: movem.l (a7)+, d1-d3/a1
pub fn i_05db32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB36;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $05DB36: adda.w #$40, a1
pub fn i_05db36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB3A;
    cpu.ir = 0xD2FC;
    let v1: u32 = 0x40;
    cpu.a[1] = cpu.a[1].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05DB3A: bra.b $5db1c
pub fn i_05db3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB3C;
    cpu.ir = 0x60E0;
    cpu.pc = 0x5DB1C;
    10
}
// ===== CompareField10_CCR_05DB3C  ($05DB3C, 28 B, video/fix_layer_backends_05db1a.s)
/// $05DB3C: movea.l $8(a6), a1
pub fn i_05db3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB40;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $05DB40: move.b $10(a6), d0
pub fn i_05db40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB44;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05DB44: cmp.b $10(a1), d0
pub fn i_05db44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB48;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $05DB48: bcs.w $5db52
pub fn i_05db48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB4C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5DB52; return 10; }
    8
}
/// $05DB4C: andi.b #$ee, ccr
pub fn i_05db4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB50;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05DB50: rts
pub fn i_05db50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB52;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05DB52: ori.b #$11, ccr
pub fn i_05db52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB56;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05DB56: rts
pub fn i_05db56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB58;
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
// ===== CompareField10_CCR_05DC00  ($05DC00, 28 B, video/camera_list_ctx_helpers_025012.s)
/// $05DC00: movea.l $8(a6), a1
pub fn i_05dc00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC04;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $05DC04: move.b $10(a6), d0
pub fn i_05dc04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC08;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05DC08: cmp.b $10(a1), d0
pub fn i_05dc08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC0C;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $05DC0C: bcs.w $5dc16
pub fn i_05dc0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC10;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5DC16; return 10; }
    8
}
/// $05DC10: andi.b #$ee, ccr
pub fn i_05dc10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC14;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05DC14: rts
pub fn i_05dc14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC16;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05DC16: ori.b #$11, ccr
pub fn i_05dc16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC1A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05DC1A: rts
pub fn i_05dc1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC1C;
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
// ===== Entity_SetVx_NegIfFacing_05dcb6  ($05DCB6, 18 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05DCB6: move.w d0, $36(a6)
pub fn i_05dcb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DCBA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05DCBA: btst.b #$0, $3a(a6)
pub fn i_05dcba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DCC0;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05DCC0: beq.w $5dcc8
pub fn i_05dcc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DCC4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5DCC8; return 10; }
    8
}
/// $05DCC4: neg.w $28(a6)
pub fn i_05dcc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DCC8;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
// ===== SetTaskW_05dcc8  ($05DCC8, 6 B, kernel/task_setters_w.c)
/// $05DCC8: move.w d0, $28(a6)
pub fn i_05dcc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DCCC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05DCCC: rts
pub fn i_05dccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DCCE;
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
// ===== Handler_ApplyCameraSelf_CopyTransform_05dd22  ($05DD22, 8 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05DD22: jsr $440e4.l
pub fn i_05dd22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5DD28);
    cpu.pc = 0x440E4;
    16
}
/// $05DD28: bra.b $5dd02
pub fn i_05dd28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD2A;
    cpu.ir = 0x60D8;
    cpu.pc = 0x5DD02;
    10
}
// ===== Task_WalkTwoArenas_05DD2A  ($05DD2A, 34 B, kernel/task_walk_two_arenas_05dd2a.s)
/// $05DD2A: movem.l a6, -(a7)
pub fn i_05dd2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD2E;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $05DD2E: lea.l $100800.l, a6
pub fn i_05dd2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD34;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0x100800;
    20
}
/// $05DD34: jsr $5b6.l
pub fn i_05dd34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5DD3A);
    cpu.pc = 0x5B6;
    16
}
/// $05DD3A: lea.l $1008a0.l, a6
pub fn i_05dd3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD40;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0x1008A0;
    20
}
/// $05DD40: jsr $5b6.l
pub fn i_05dd40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD46;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5DD46);
    cpu.pc = 0x5B6;
    16
}
/// $05DD46: movem.l (a7)+, a6
pub fn i_05dd46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD4A;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $05DD4A: rts
pub fn i_05dd4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD4C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ScreenBox_Default_05dd4c  ($05DD4C, 10 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05DD4C: ori.b #$1, d0
pub fn i_05dd4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD50;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05DD50: ori.b #$1, d0
pub fn i_05dd50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD54;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05DD54: dc.w $ffff
pub fn i_05dd54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD56;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
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
/// $05DD80: move.w #$1f0, d3
pub fn i_05dd80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD84;
    cpu.ir = 0x363C;
    let v1: u32 = 0x1F0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05DD84: add.w (a0), d0
pub fn i_05dd84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD86;
    cpu.ir = 0xD050;
    let v2 = bus.read16(cpu.a[0]) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $05DD86: add.w $2(a0), d1
pub fn i_05dd86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD8A;
    cpu.ir = 0xD268;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $05DD8A: sub.w $6(a0), d2
pub fn i_05dd8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD8E;
    cpu.ir = 0x9468;
    let ad2 = cpu.a[0].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $05DD8E: sub.w $4(a0), d3
pub fn i_05dd8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD92;
    cpu.ir = 0x9668;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $05DD92: cmp.w $22(a6), d0
pub fn i_05dd92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD96;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $05DD96: bgt.w $5ddb8
pub fn i_05dd96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD9A;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x5DDB8; return 10; }
    8
}
/// $05DD9A: cmp.w $22(a6), d1
pub fn i_05dd9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD9E;
    cpu.ir = 0xB26E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $05DD9E: blt.w $5ddb8
pub fn i_05dd9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DDA2;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x5DDB8; return 10; }
    8
}
/// $05DDA2: cmp.w $24(a6), d2
pub fn i_05dda2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DDA6;
    cpu.ir = 0xB46E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $05DDA6: bgt.w $5ddb8
pub fn i_05dda6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DDAA;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x5DDB8; return 10; }
    8
}
/// $05DDAA: cmp.w $24(a6), d3
pub fn i_05ddaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DDAE;
    cpu.ir = 0xB66E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[3] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $05DDAE: blt.w $5ddb8
pub fn i_05ddae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DDB2;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x5DDB8; return 10; }
    8
}
// ===== ClearC_05ddb2  ($05DDB2, 6 B, thunks/ccr_helpers.c)
/// $05DDB2: andi.b #$fe, ccr
pub fn i_05ddb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DDB6;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $05DDB6: rts
pub fn i_05ddb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DDB8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetC_05ddb8  ($05DDB8, 6 B, thunks/ccr_helpers.c)
/// $05DDB8: ori.b #$1, ccr
pub fn i_05ddb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DDBC;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $05DDBC: rts
pub fn i_05ddbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DDBE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CheckEnterScreen_05ddbe  ($05DDBE, 46 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05DDBE: cmpi.w #$140, $22(a6)
pub fn i_05ddbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DDC4;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x140, Size::Word, false);
    16
}
/// $05DDC4: bge.w $5ddf2
pub fn i_05ddc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DDC8;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x5DDF2; return 10; }
    8
}
/// $05DDC8: cmpi.w #$0, $22(a6)
pub fn i_05ddc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DDCE;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $05DDCE: ble.w $5ddf2
pub fn i_05ddce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DDD2;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x5DDF2; return 10; }
    8
}
/// $05DDD2: cmpi.w #$1f0, $24(a6)
pub fn i_05ddd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DDD8;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1F0, Size::Word, false);
    16
}
/// $05DDD8: bge.w $5ddf2
pub fn i_05ddd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DDDC;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x5DDF2; return 10; }
    8
}
/// $05DDDC: cmpi.w #$100, $24(a6)
pub fn i_05dddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DDE2;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x100, Size::Word, false);
    16
}
/// $05DDE2: ble.w $5ddf2
pub fn i_05dde2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DDE6;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x5DDF2; return 10; }
    8
}
/// $05DDE6: bset.b #$7, $13(a6)
pub fn i_05dde6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DDEC;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== ClearC_05ddec  ($05DDEC, 6 B, thunks/ccr_helpers.c)
/// $05DDEC: andi.b #$fe, ccr
pub fn i_05ddec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DDF0;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $05DDF0: rts
pub fn i_05ddf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DDF2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CheckLeaveScreenWide_05ddf2  ($05DDF2, 32 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05DDF2: cmpi.w #$ff80, $22(a6)
pub fn i_05ddf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DDF8;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF80, Size::Word, false);
    16
}
/// $05DDF8: ble.b $5dde6
pub fn i_05ddf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DDFA;
    cpu.ir = 0x6FEC;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x5DDE6; return 10; }
    8
}
/// $05DDFA: cmpi.w #$200, $22(a6)
pub fn i_05ddfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DE00;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x200, Size::Word, false);
    16
}
/// $05DE00: bge.b $5dde6
pub fn i_05de00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DE02;
    cpu.ir = 0x6CE4;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x5DDE6; return 10; }
    8
}
/// $05DE02: cmpi.w #$0, $24(a6)
pub fn i_05de02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DE08;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $05DE08: ble.b $5dde6
pub fn i_05de08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DE0A;
    cpu.ir = 0x6FDC;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x5DDE6; return 10; }
    8
}
/// $05DE0A: cmpi.w #$280, $24(a6)
pub fn i_05de0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DE10;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x280, Size::Word, false);
    16
}
/// $05DE10: bge.b $5dde6
pub fn i_05de10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DE12;
    cpu.ir = 0x6CD4;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x5DDE6; return 10; }
    8
}
// ===== ClearC_05de12  ($05DE12, 6 B, thunks/ccr_helpers.c)
/// $05DE12: andi.b #$fe, ccr
pub fn i_05de12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DE16;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $05DE16: rts
pub fn i_05de16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DE18;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Atan2_Angle256_05e018  ($05E018, 88 B, props/late_props_turrets_05e000.s)
/// $05E018: asr.w #$1, d0
pub fn i_05e018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E01A;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $05E01A: smi.b d2
pub fn i_05e01a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E01C;
    cpu.ir = 0x5BC2;
    let c1: u32 = if cpu.sr.evaluate(Condition::Mi) { 0xFF } else { 0 };
    let v2 = cpu.d[2] & 0xFF;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (c1 & 0xFF);
    4
}
/// $05E01C: eor.b d2, d0
pub fn i_05e01c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E01E;
    cpu.ir = 0xB500;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 ^ cpu.d[2]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05E01E: asr.w #$1, d1
pub fn i_05e01e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E020;
    cpu.ir = 0xE241;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $05E020: smi.b d3
pub fn i_05e020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E022;
    cpu.ir = 0x5BC3;
    let c1: u32 = if cpu.sr.evaluate(Condition::Mi) { 0xFF } else { 0 };
    let v2 = cpu.d[3] & 0xFF;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (c1 & 0xFF);
    4
}
/// $05E022: eor.b d3, d1
pub fn i_05e022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E024;
    cpu.ir = 0xB701;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 ^ cpu.d[3]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05E024: move.w #$ff, d4
pub fn i_05e024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E028;
    cpu.ir = 0x383C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05E028: and.w d4, d0
pub fn i_05e028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E02A;
    cpu.ir = 0xC044;
    let v1 = cpu.d[4] & 0xFFFF;
    let r = (v1 & cpu.d[0]) & 0xFFFF;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05E02A: and.w d4, d1
pub fn i_05e02a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E02C;
    cpu.ir = 0xC244;
    let v1 = cpu.d[4] & 0xFFFF;
    let r = (v1 & cpu.d[1]) & 0xFFFF;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05E02C: cmp.b d1, d0
pub fn i_05e02c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E02E;
    cpu.ir = 0xB001;
    let v1 = cpu.d[1] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $05E02E: smi.b d4
pub fn i_05e02e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E030;
    cpu.ir = 0x5BC4;
    let c1: u32 = if cpu.sr.evaluate(Condition::Mi) { 0xFF } else { 0 };
    let v2 = cpu.d[4] & 0xFF;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (c1 & 0xFF);
    4
}
/// $05E030: bcs.w $5e036
pub fn i_05e030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E034;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5E036; return 10; }
    8
}
/// $05E034: exg.l d1, d0
pub fn i_05e034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E036;
    cpu.ir = 0xC340;
    cpu.d.swap(1, 0);
    6
}
/// $05E036: tst.b d0
pub fn i_05e036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E038;
    cpu.ir = 0x4A00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05E038: bne.w $5e046
pub fn i_05e038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E03C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5E046; return 10; }
    8
}
/// $05E03C: move.b d4, d0
pub fn i_05e03c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E03E;
    cpu.ir = 0x1004;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05E03E: andi.b #$40, d0
pub fn i_05e03e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E042;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x40) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05E042: bra.w $5e062
pub fn i_05e042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E046;
    cpu.ir = 0x6000;
    cpu.pc = 0x5E062;
    10
}
/// $05E046: lea.l $5de18(pc), a0
pub fn i_05e046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E04A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x5DE18;
    16
}
/// $05E04A: move.b (a0, d0.w), d0
pub fn i_05e04a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E04E;
    cpu.ir = 0x1030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $05E04E: sub.b (a0, d1.w), d0
pub fn i_05e04e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E052;
    cpu.ir = 0x9030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    14
}
/// $05E052: lea.l $5df18(pc), a0
pub fn i_05e052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E056;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x5DF18;
    16
}
/// $05E056: move.b (a0, d0.w), d0
pub fn i_05e056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E05A;
    cpu.ir = 0x1030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $05E05A: eor.b d4, d0
pub fn i_05e05a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E05C;
    cpu.ir = 0xB900;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 ^ cpu.d[4]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05E05C: sub.b d4, d0
pub fn i_05e05c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E05E;
    cpu.ir = 0x9004;
    let v1 = cpu.d[4] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    4
}
/// $05E05E: addi.b #$20, d0
pub fn i_05e05e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E062;
    cpu.ir = 0x600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05E062: eor.b d2, d3
pub fn i_05e062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E064;
    cpu.ir = 0xB503;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 ^ cpu.d[2]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05E064: eor.b d3, d0
pub fn i_05e064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E066;
    cpu.ir = 0xB700;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 ^ cpu.d[3]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05E066: sub.b d3, d0
pub fn i_05e066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E068;
    cpu.ir = 0x9003;
    let v1 = cpu.d[3] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    4
}
/// $05E068: andi.b #$80, d2
pub fn i_05e068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E06C;
    cpu.ir = 0x202;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 & 0x80) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05E06C: add.b d2, d0
pub fn i_05e06c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E06E;
    cpu.ir = 0xD002;
    let v1 = cpu.d[2] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    let r = add_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    4
}
/// $05E06E: rts
pub fn i_05e06e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E070;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Target_DeltaThenAtan2_05e070  ($05E070, 16 B, props/late_props_turrets_05e000.s)
/// $05E070: move.w $22(a0), d0
pub fn i_05e070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E074;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05E074: move.w $24(a0), d1
pub fn i_05e074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E078;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05E078: sub.w $22(a6), d0
pub fn i_05e078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E07C;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $05E07C: sub.w $24(a6), d1
pub fn i_05e07c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E080;
    cpu.ir = 0x926E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
// ===== JsrPcThunk_05e080  ($05E080, 6 B, thunks/jsr_pc_thunks.c)
/// $05E080: jsr $5e018(pc)
pub fn i_05e080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E084;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5E084);
    cpu.pc = 0x5E018;
    16
}
/// $05E084: rts
pub fn i_05e084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E086;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Target_AcquireNearestPlayer_05e086  ($05E086, 150 B, props/late_props_turrets_05e000.s)
/// $05E086: move.l a0, -(a7)
pub fn i_05e086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E088;
    cpu.ir = 0x2F08;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $05E088: jsr $5e1aa(pc)
pub fn i_05e088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E08C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5E08C);
    cpu.pc = 0x5E1AA;
    16
}
/// $05E08C: bcs.w $5e0ce
pub fn i_05e08c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E090;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5E0CE; return 10; }
    8
}
/// $05E090: lea.l $100440.l, a2
pub fn i_05e090(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E096;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x100440;
    20
}
/// $05E096: lea.l $1004e0.l, a3
pub fn i_05e096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E09C;
    cpu.ir = 0x47F9;
    cpu.a[3] = 0x1004E0;
    20
}
/// $05E09C: movea.l (a7)+, a1
pub fn i_05e09c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E09E;
    cpu.ir = 0x225F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    12
}
/// $05E09E: btst.b #$0, d0
pub fn i_05e09e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E0A2;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $05E0A2: beq.w $5e0b4
pub fn i_05e0a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E0A6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E0B4; return 10; }
    8
}
/// $05E0A6: movea.l a2, a0
pub fn i_05e0a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E0A8;
    cpu.ir = 0x204A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $05E0A8: jsr $5e260(pc)
pub fn i_05e0a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E0AC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5E0AC);
    cpu.pc = 0x5E260;
    16
}
/// $05E0AC: bcc.w $5e0b4
pub fn i_05e0ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E0B0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5E0B4; return 10; }
    8
}
/// $05E0B0: eori.b #$1, d0
pub fn i_05e0b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E0B4;
    cpu.ir = 0xA00;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05E0B4: btst.b #$1, d0
pub fn i_05e0b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E0B8;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((1) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $05E0B8: beq.w $5e0ca
pub fn i_05e0b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E0BC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E0CA; return 10; }
    8
}
/// $05E0BC: movea.l a3, a0
pub fn i_05e0bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E0BE;
    cpu.ir = 0x204B;
    let v1 = cpu.a[3] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $05E0BE: jsr $5e260(pc)
pub fn i_05e0be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E0C2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5E0C2);
    cpu.pc = 0x5E260;
    16
}
/// $05E0C2: bcc.w $5e0ca
pub fn i_05e0c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E0C6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5E0CA; return 10; }
    8
}
/// $05E0C6: eori.b #$2, d0
pub fn i_05e0c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E0CA;
    cpu.ir = 0xA00;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 ^ 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05E0CA: bra.w $5e0d8
pub fn i_05e0ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E0CE;
    cpu.ir = 0x6000;
    cpu.pc = 0x5E0D8;
    10
}
/// $05E0CE: movea.l (a7)+, a1
pub fn i_05e0ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E0D0;
    cpu.ir = 0x225F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    12
}
/// $05E0D0: bra.w $5e0d8
pub fn i_05e0d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E0D4;
    cpu.ir = 0x6000;
    cpu.pc = 0x5E0D8;
    10
}
/// $05E0D4: jsr $5e1aa(pc)
pub fn i_05e0d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E0D8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5E0D8);
    cpu.pc = 0x5E1AA;
    16
}
/// $05E0D8: tst.b d0
pub fn i_05e0d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E0DA;
    cpu.ir = 0x4A00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05E0DA: beq.w $5e12a
pub fn i_05e0da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E0DE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E12A; return 10; }
    8
}
/// $05E0DE: lea.l $100440.l, a0
pub fn i_05e0de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E0E4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100440;
    20
}
/// $05E0E4: lea.l $1004e0.l, a1
pub fn i_05e0e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E0EA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1004E0;
    20
}
/// $05E0EA: cmpi.b #$1, d0
pub fn i_05e0ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E0EE;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $05E0EE: beq.w $5e11c
pub fn i_05e0ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E0F2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E11C; return 10; }
    8
}
/// $05E0F2: cmpi.b #$2, d0
pub fn i_05e0f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E0F6;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $05E0F6: beq.w $5e122
pub fn i_05e0f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E0FA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E122; return 10; }
    8
}
/// $05E0FA: move.w $22(a6), d0
pub fn i_05e0fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E0FE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05E0FE: move.w $22(a0), d1
pub fn i_05e0fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E102;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05E102: move.w $22(a1), d2
pub fn i_05e102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E106;
    cpu.ir = 0x3429;
    let ad2 = cpu.a[1].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05E106: sub.w d0, d1
pub fn i_05e106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E108;
    cpu.ir = 0x9240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05E108: bge.w $5e10e
pub fn i_05e108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E10C;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x5E10E; return 10; }
    8
}
/// $05E10C: neg.w d1
pub fn i_05e10c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E10E;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05E10E: sub.w d0, d2
pub fn i_05e10e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E110;
    cpu.ir = 0x9440;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05E110: bge.w $5e116
pub fn i_05e110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E114;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x5E116; return 10; }
    8
}
/// $05E114: neg.w d2
pub fn i_05e114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E116;
    cpu.ir = 0x4442;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05E116: cmp.w d1, d2
pub fn i_05e116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E118;
    cpu.ir = 0xB441;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05E118: blt.w $5e122
pub fn i_05e118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E11C;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x5E122; return 10; }
    8
}
// ===== ClearXN_05e11c  ($05E11C, 6 B, thunks/ccr_helpers.c)
/// $05E11C: andi.b #$ee, ccr
pub fn i_05e11c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E120;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05E120: rts
pub fn i_05e120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E122;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Target_UseP2_05e122  ($05E122, 2 B, props/late_props_turrets_05e000.s)
/// $05E122: movea.l a1, a0
pub fn i_05e122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E124;
    cpu.ir = 0x2049;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
// ===== ClearXN_05e124  ($05E124, 6 B, thunks/ccr_helpers.c)
/// $05E124: andi.b #$ee, ccr
pub fn i_05e124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E128;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05E128: rts
pub fn i_05e128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E12A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Target_DefaultP2_05e12a  ($05E12A, 6 B, props/late_props_turrets_05e000.s)
/// $05E12A: lea.l $1004e0.l, a0
pub fn i_05e12a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E130;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1004E0;
    20
}
// ===== SetXN_05e130  ($05E130, 6 B, thunks/ccr_helpers.c)
/// $05E130: ori.b #$11, ccr
pub fn i_05e130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E134;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05E134: rts
pub fn i_05e134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E136;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Target_AngleToPlayer_05e136  ($05E136, 36 B, props/late_props_turrets_05e000.s)
/// $05E136: jsr $5e0d4(pc)
pub fn i_05e136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E13A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5E13A);
    cpu.pc = 0x5E0D4;
    16
}
/// $05E13A: bcs.w $5e160
pub fn i_05e13a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E13E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5E160; return 10; }
    8
}
/// $05E13E: move.w $22(a0), d0
pub fn i_05e13e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E142;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05E142: move.w $24(a0), d1
pub fn i_05e142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E146;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05E146: sub.w $22(a6), d0
pub fn i_05e146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E14A;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $05E14A: sub.w $24(a6), d1
pub fn i_05e14a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E14E;
    cpu.ir = 0x926E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $05E14E: movem.w a0, -(a7)
pub fn i_05e14e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E152;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.a[0] as u16);
    cpu.a[7] = a;
    20
}
/// $05E152: jsr $5e018(pc)
pub fn i_05e152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E156;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5E156);
    cpu.pc = 0x5E018;
    16
}
/// $05E156: movem.w (a7)+, a0
pub fn i_05e156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E15A;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
// ===== ClearXN_05e15a  ($05E15A, 6 B, thunks/ccr_helpers.c)
/// $05E15A: andi.b #$ee, ccr
pub fn i_05e15a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E15E;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05E15E: rts
pub fn i_05e15e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E160;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_05e160  ($05E160, 6 B, thunks/ccr_helpers.c)
/// $05E160: ori.b #$11, ccr
pub fn i_05e160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E164;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05E164: rts
pub fn i_05e164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E166;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TargetBox_Default_05e166  ($05E166, 56 B, props/late_props_turrets_05e000.s)
/// $05E166: dc.w $ff20
pub fn i_05e166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E168;
    cpu.ir = 0xFF20;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $05E172: lea.l $5e166(pc), a0
pub fn i_05e172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E176;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x5E166;
    16
}
/// $05E176: jsr $5e086(pc)
pub fn i_05e176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E17A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5E17A);
    cpu.pc = 0x5E086;
    16
}
/// $05E17A: bcs.w $5e1a4
pub fn i_05e17a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E17E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5E1A4; return 10; }
    8
}
/// $05E17E: move.w $22(a0), d0
pub fn i_05e17e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E182;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05E182: move.w $24(a0), d1
pub fn i_05e182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E186;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05E186: addi.w #$1c, d1
pub fn i_05e186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E18A;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x1C, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05E18A: sub.w $22(a6), d0
pub fn i_05e18a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E18E;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $05E18E: sub.w $24(a6), d1
pub fn i_05e18e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E192;
    cpu.ir = 0x926E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $05E192: movem.w a0, -(a7)
pub fn i_05e192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E196;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.a[0] as u16);
    cpu.a[7] = a;
    20
}
/// $05E196: jsr $5e018(pc)
pub fn i_05e196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E19A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5E19A);
    cpu.pc = 0x5E018;
    16
}
/// $05E19A: movem.w (a7)+, a0
pub fn i_05e19a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E19E;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
// ===== ClearXN_05e19e  ($05E19E, 6 B, thunks/ccr_helpers.c)
/// $05E19E: andi.b #$ee, ccr
pub fn i_05e19e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E1A2;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05E1A2: rts
pub fn i_05e1a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E1A4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_05e1a4  ($05E1A4, 6 B, thunks/ccr_helpers.c)
/// $05E1A4: ori.b #$11, ccr
pub fn i_05e1a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E1A8;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05E1A8: rts
pub fn i_05e1a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E1AA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Players_AliveMask_05e1aa  ($05E1AA, 52 B, props/late_props_turrets_05e000.s)
/// $05E1AA: clr.l d0
pub fn i_05e1aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E1AC;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $05E1AC: jsr $5e3a2(pc)
pub fn i_05e1ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E1B0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5E1B0);
    cpu.pc = 0x5E3A2;
    16
}
/// $05E1B0: bcc.w $5e1bc
pub fn i_05e1b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E1B4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5E1BC; return 10; }
    8
}
/// $05E1B4: move.w #$1, d0
pub fn i_05e1b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E1B8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05E1B8: bra.w $5e1be
pub fn i_05e1b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E1BC;
    cpu.ir = 0x6000;
    cpu.pc = 0x5E1BE;
    10
}
/// $05E1BC: clr.w d0
pub fn i_05e1bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E1BE;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05E1BE: move.l d0, -(a7)
pub fn i_05e1be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E1C0;
    cpu.ir = 0x2F00;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $05E1C0: moveq #$1, d0
pub fn i_05e1c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E1C2;
    cpu.ir = 0x7001;
    cpu.d[0] = 0x1;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05E1C2: jsr $5e3a2(pc)
pub fn i_05e1c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E1C6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5E1C6);
    cpu.pc = 0x5E3A2;
    16
}
/// $05E1C6: bcc.w $5e1d2
pub fn i_05e1c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E1CA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5E1D2; return 10; }
    8
}
/// $05E1CA: move.w #$2, d1
pub fn i_05e1ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E1CE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05E1CE: bra.w $5e1d4
pub fn i_05e1ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E1D2;
    cpu.ir = 0x6000;
    cpu.pc = 0x5E1D4;
    10
}
/// $05E1D2: clr.w d1
pub fn i_05e1d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E1D4;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05E1D4: move.l (a7)+, d0
pub fn i_05e1d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E1D6;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05E1D6: or.w d1, d0
pub fn i_05e1d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E1D8;
    cpu.ir = 0x8041;
    let v1 = cpu.d[1] & 0xFFFF;
    let r = (v1 | cpu.d[0]) & 0xFFFF;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05E1D8: tst.w d0
pub fn i_05e1d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E1DA;
    cpu.ir = 0x4A40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05E1DA: beq.w $5e1e4
pub fn i_05e1da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E1DE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E1E4; return 10; }
    8
}
// ===== ClearXN_05e1de  ($05E1DE, 6 B, thunks/ccr_helpers.c)
/// $05E1DE: andi.b #$ee, ccr
pub fn i_05e1de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E1E2;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05E1E2: rts
pub fn i_05e1e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E1E4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_05e1e4  ($05E1E4, 6 B, thunks/ccr_helpers.c)
/// $05E1E4: ori.b #$11, ccr
pub fn i_05e1e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E1E8;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05E1E8: rts
pub fn i_05e1e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E1EA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Players_PickRandomAlive_05e1ea  ($05E1EA, 52 B, props/late_props_turrets_05e000.s)
/// $05E1EA: jsr $5e1aa(pc)
pub fn i_05e1ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E1EE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5E1EE);
    cpu.pc = 0x5E1AA;
    16
}
/// $05E1EE: cmpi.w #$3, d0
pub fn i_05e1ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E1F2;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x3, Size::Word, false);
    8
}
/// $05E1F2: bcs.w $5e202
pub fn i_05e1f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E1F6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5E202; return 10; }
    8
}
/// $05E1F6: jsr $5e9b6.l
pub fn i_05e1f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E1FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5E1FC);
    cpu.pc = 0x5E9B6;
    16
}
/// $05E1FC: andi.w #$1, d0
pub fn i_05e1fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E200;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05E200: addq.w #$1, d0
pub fn i_05e200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E202;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05E202: lea.l $100440.l, a0
pub fn i_05e202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E208;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100440;
    20
}
/// $05E208: cmpi.w #$2, d0
pub fn i_05e208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E20C;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x2, Size::Word, false);
    8
}
/// $05E20C: bne.w $5e216
pub fn i_05e20c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E210;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5E216; return 10; }
    8
}
/// $05E210: lea.l $1004e0.l, a0
pub fn i_05e210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E216;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1004E0;
    20
}
/// $05E216: cmpi.w #$0, d0
pub fn i_05e216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E21A;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $05E21A: beq.w $5e224
pub fn i_05e21a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E21E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E224; return 10; }
    8
}
// ===== ClearXN_05e21e  ($05E21E, 6 B, thunks/ccr_helpers.c)
/// $05E21E: andi.b #$ee, ccr
pub fn i_05e21e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E222;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05E222: rts
pub fn i_05e222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E224;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_05e224  ($05E224, 6 B, thunks/ccr_helpers.c)
/// $05E224: ori.b #$11, ccr
pub fn i_05e224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E228;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05E228: rts
pub fn i_05e228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E22A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Dist_Approx_05e22a  ($05E22A, 54 B, props/late_props_turrets_05e000.s)
/// $05E22A: move.w $22(a6), d0
pub fn i_05e22a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E22E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05E22E: move.w $24(a6), d1
pub fn i_05e22e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E232;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05E232: move.w $22(a0), d2
pub fn i_05e232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E236;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05E236: move.w $24(a0), d3
pub fn i_05e236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E23A;
    cpu.ir = 0x3628;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05E23A: sub.w d2, d0
pub fn i_05e23a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E23C;
    cpu.ir = 0x9042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05E23C: sub.w d3, d1
pub fn i_05e23c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E23E;
    cpu.ir = 0x9243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05E23E: cmpi.w #$0, d0
pub fn i_05e23e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E242;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $05E242: bge.w $5e248
pub fn i_05e242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E246;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x5E248; return 10; }
    8
}
/// $05E246: neg.w d0
pub fn i_05e246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E248;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05E248: cmpi.w #$0, d1
pub fn i_05e248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E24C;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $05E24C: bge.w $5e252
pub fn i_05e24c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E250;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x5E252; return 10; }
    8
}
/// $05E250: neg.w d1
pub fn i_05e250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E252;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05E252: cmp.w d1, d0
pub fn i_05e252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E254;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05E254: bgt.w $5e25a
pub fn i_05e254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E258;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x5E25A; return 10; }
    8
}
/// $05E258: exg.l d0, d1
pub fn i_05e258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E25A;
    cpu.ir = 0xC141;
    cpu.d.swap(0, 1);
    6
}
/// $05E25A: lsr.w #$1, d1
pub fn i_05e25a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E25C;
    cpu.ir = 0xE249;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $05E25C: add.w d1, d0
pub fn i_05e25c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E25E;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05E25E: rts
pub fn i_05e25e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E260;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Target_InRangeBox_05e260  ($05E260, 108 B, props/late_props_turrets_05e000.s)
/// $05E260: move.w $22(a0), d1
pub fn i_05e260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E264;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05E264: sub.w $22(a6), d1
pub fn i_05e264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E268;
    cpu.ir = 0x926E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $05E268: btst.b #$0, $3a(a6)
pub fn i_05e268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E26E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05E26E: bne.w $5e284
pub fn i_05e26e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E272;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5E284; return 10; }
    8
}
/// $05E272: cmp.w (a1), d1
pub fn i_05e272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E274;
    cpu.ir = 0xB251;
    let v2 = bus.read16(cpu.a[1]) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    8
}
/// $05E274: blt.w $5e2d2
pub fn i_05e274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E278;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x5E2D2; return 10; }
    8
}
/// $05E278: cmp.w $2(a1), d1
pub fn i_05e278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E27C;
    cpu.ir = 0xB269;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $05E27C: bgt.w $5e2d2
pub fn i_05e27c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E280;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x5E2D2; return 10; }
    8
}
/// $05E280: bra.w $5e294
pub fn i_05e280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E284;
    cpu.ir = 0x6000;
    cpu.pc = 0x5E294;
    10
}
/// $05E284: neg.w d1
pub fn i_05e284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E286;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05E286: cmp.w (a1), d1
pub fn i_05e286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E288;
    cpu.ir = 0xB251;
    let v2 = bus.read16(cpu.a[1]) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    8
}
/// $05E288: blt.w $5e2d2
pub fn i_05e288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E28C;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x5E2D2; return 10; }
    8
}
/// $05E28C: cmp.w $2(a1), d1
pub fn i_05e28c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E290;
    cpu.ir = 0xB269;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $05E290: bgt.w $5e2d2
pub fn i_05e290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E294;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x5E2D2; return 10; }
    8
}
/// $05E294: move.w $24(a0), d1
pub fn i_05e294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E298;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05E298: sub.w $24(a6), d1
pub fn i_05e298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E29C;
    cpu.ir = 0x926E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $05E29C: btst.b #$1, $3a(a6)
pub fn i_05e29c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E2A2;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05E2A2: bne.w $5e2ba
pub fn i_05e2a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E2A6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5E2BA; return 10; }
    8
}
/// $05E2A6: cmp.w $4(a1), d1
pub fn i_05e2a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E2AA;
    cpu.ir = 0xB269;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $05E2AA: blt.w $5e2d2
pub fn i_05e2aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E2AE;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x5E2D2; return 10; }
    8
}
/// $05E2AE: cmp.w $6(a1), d1
pub fn i_05e2ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E2B2;
    cpu.ir = 0xB269;
    let ad2 = cpu.a[1].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $05E2B2: bgt.w $5e2d2
pub fn i_05e2b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E2B6;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x5E2D2; return 10; }
    8
}
/// $05E2B6: bra.w $5e2cc
pub fn i_05e2b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E2BA;
    cpu.ir = 0x6000;
    cpu.pc = 0x5E2CC;
    10
}
/// $05E2BA: neg.w d1
pub fn i_05e2ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E2BC;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05E2BC: cmp.w $4(a1), d1
pub fn i_05e2bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E2C0;
    cpu.ir = 0xB269;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $05E2C0: blt.w $5e2d2
pub fn i_05e2c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E2C4;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x5E2D2; return 10; }
    8
}
/// $05E2C4: cmp.w $6(a1), d1
pub fn i_05e2c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E2C8;
    cpu.ir = 0xB269;
    let ad2 = cpu.a[1].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $05E2C8: bgt.w $5e2d2
pub fn i_05e2c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E2CC;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x5E2D2; return 10; }
    8
}
// ===== ClearXN_05e2cc  ($05E2CC, 6 B, thunks/ccr_helpers.c)
/// $05E2CC: andi.b #$ee, ccr
pub fn i_05e2cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E2D0;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05E2D0: rts
pub fn i_05e2d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E2D2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_05e2d2  ($05E2D2, 6 B, thunks/ccr_helpers.c)
/// $05E2D2: ori.b #$11, ccr
pub fn i_05e2d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E2D6;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05E2D6: rts
pub fn i_05e2d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E2D8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SlotExtractCoords_05E2D8  ($05E2D8, 96 B, video/fix_layer_backends_05db1a.s)
/// $05E2D8: moveq #$ff, d0
pub fn i_05e2d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E2DA;
    cpu.ir = 0x70FF;
    cpu.d[0] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05E2DA: moveq #$ff, d1
pub fn i_05e2da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E2DC;
    cpu.ir = 0x72FF;
    cpu.d[1] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05E2DC: cmpi.l #$ffffffff, (a0)
pub fn i_05e2dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E2E2;
    cpu.ir = 0xC90;
    let v2 = bus.read32(cpu.a[0]);
    sub_with_flags(cpu, v2, 0xFFFFFFFF, Size::Long, false);
    16
}
/// $05E2E2: beq.w $5e336
pub fn i_05e2e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E2E6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E336; return 10; }
    8
}
/// $05E2E6: cmpi.l #$52a, (a0)
pub fn i_05e2e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E2EC;
    cpu.ir = 0xC90;
    let v2 = bus.read32(cpu.a[0]);
    sub_with_flags(cpu, v2, 0x52A, Size::Long, false);
    16
}
/// $05E2EC: beq.w $5e336
pub fn i_05e2ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E2F0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E336; return 10; }
    8
}
/// $05E2F0: cmpi.l #$400, (a0)
pub fn i_05e2f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E2F6;
    cpu.ir = 0xC90;
    let v2 = bus.read32(cpu.a[0]);
    sub_with_flags(cpu, v2, 0x400, Size::Long, false);
    16
}
/// $05E2F6: beq.w $5e336
pub fn i_05e2f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E2FA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E336; return 10; }
    8
}
/// $05E2FA: cmpi.b #$3, $106ece.l
pub fn i_05e2fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E302;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    20
}
/// $05E302: beq.w $5e320
pub fn i_05e302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E306;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E320; return 10; }
    8
}
/// $05E306: moveq #$0, d0
pub fn i_05e306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E308;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05E308: move.w $22(a0), d0
pub fn i_05e308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E30C;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05E30C: btst.b #$7, $5b(a0)
pub fn i_05e30c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E312;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x5B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05E312: beq.w $5e336
pub fn i_05e312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E316;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E336; return 10; }
    8
}
/// $05E316: moveq #$0, d1
pub fn i_05e316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E318;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05E318: move.w $24(a0), d1
pub fn i_05e318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E31C;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05E31C: bra.w $5e336
pub fn i_05e31c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E320;
    cpu.ir = 0x6000;
    cpu.pc = 0x5E336;
    10
}
/// $05E320: moveq #$0, d0
pub fn i_05e320(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E322;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05E322: move.w $22(a0), d0
pub fn i_05e322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E326;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05E326: btst.b #$7, $5b(a0)
pub fn i_05e326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E32C;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x5B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05E32C: beq.w $5e336
pub fn i_05e32c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E330;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E336; return 10; }
    8
}
/// $05E330: moveq #$0, d1
pub fn i_05e330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E332;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05E332: move.w $24(a0), d1
pub fn i_05e332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E336;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05E336: rts
pub fn i_05e336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E338;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_IndexFromPtr_05e338  ($05E338, 34 B, props/late_props_turrets_05e000.s)
/// $05E338: clr.w d0
pub fn i_05e338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E33A;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05E33A: move.l a0, d1
pub fn i_05e33a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E33C;
    cpu.ir = 0x2208;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.d[1] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05E33C: cmpi.l #$100440, d1
pub fn i_05e33c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E342;
    cpu.ir = 0xC81;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0x100440, Size::Long, false);
    8
}
/// $05E342: beq.w $5e352
pub fn i_05e342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E346;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E352; return 10; }
    8
}
/// $05E346: addq.w #$1, d0
pub fn i_05e346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E348;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05E348: cmpi.l #$1004e0, d1
pub fn i_05e348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E34E;
    cpu.ir = 0xC81;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0x1004E0, Size::Long, false);
    8
}
/// $05E34E: bne.w $5e360
pub fn i_05e34e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E352;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5E360; return 10; }
    8
}
/// $05E352: jsr $5e42a(pc)
pub fn i_05e352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E356;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5E356);
    cpu.pc = 0x5E42A;
    16
}
/// $05E356: bcc.w $5e360
pub fn i_05e356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E35A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5E360; return 10; }
    8
}
// ===== ClearXN_05e35a  ($05E35A, 6 B, thunks/ccr_helpers.c)
/// $05E35A: andi.b #$ee, ccr
pub fn i_05e35a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E35E;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05E35E: rts
pub fn i_05e35e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E360;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_05e360  ($05E360, 6 B, thunks/ccr_helpers.c)
/// $05E360: ori.b #$11, ccr
pub fn i_05e360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E364;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05E364: rts
pub fn i_05e364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E366;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Players_AnyAliveNotHit_05e366  ($05E366, 20 B, props/late_props_turrets_05e000.s)
/// $05E366: clr.w d0
pub fn i_05e366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E368;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05E368: jsr $5e3a2(pc)
pub fn i_05e368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E36C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5E36C);
    cpu.pc = 0x5E3A2;
    16
}
/// $05E36C: bcc.w $5e380
pub fn i_05e36c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E370;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5E380; return 10; }
    8
}
/// $05E370: btst.b #$0, $13(a0)
pub fn i_05e370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E376;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05E376: bne.w $5e380
pub fn i_05e376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E37A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5E380; return 10; }
    8
}
// ===== ClearXN_05e37a  ($05E37A, 6 B, thunks/ccr_helpers.c)
/// $05E37A: andi.b #$ee, ccr
pub fn i_05e37a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E37E;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05E37E: rts
pub fn i_05e37e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E380;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player2_AliveNotHit_05e380  ($05E380, 22 B, props/late_props_turrets_05e000.s)
/// $05E380: move.w #$1, d0
pub fn i_05e380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E384;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05E384: jsr $5e3a2(pc)
pub fn i_05e384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E388;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5E388);
    cpu.pc = 0x5E3A2;
    16
}
/// $05E388: bcc.w $5e39c
pub fn i_05e388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E38C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5E39C; return 10; }
    8
}
/// $05E38C: btst.b #$0, $13(a0)
pub fn i_05e38c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E392;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05E392: bne.w $5e39c
pub fn i_05e392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E396;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5E39C; return 10; }
    8
}
// ===== ClearXN_05e396  ($05E396, 6 B, thunks/ccr_helpers.c)
/// $05E396: andi.b #$ee, ccr
pub fn i_05e396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E39A;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05E39A: rts
pub fn i_05e39a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E39C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_05e39c  ($05E39C, 6 B, thunks/ccr_helpers.c)
/// $05E39C: ori.b #$11, ccr
pub fn i_05e39c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E3A0;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05E3A0: rts
pub fn i_05e3a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E3A2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_GetEntity_05e3a2  ($05E3A2, 64 B, props/late_props_turrets_05e000.s)
/// $05E3A2: andi.w #$1, d0
pub fn i_05e3a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E3A6;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05E3A6: lea.l $5e3f4(pc), a0
pub fn i_05e3a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E3AA;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x5E3F4;
    16
}
/// $05E3AA: asl.l #$2, d0
pub fn i_05e3aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E3AC;
    cpu.ir = 0xE580;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Long, 0, true);
    cpu.d[0] = r;
    10
}
/// $05E3AC: movea.l (a0, d0.w), a0
pub fn i_05e3ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E3B0;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $05E3B0: btst.b #$7, $5b(a0)
pub fn i_05e3b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E3B6;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x5B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05E3B6: beq.w $5e3e8
pub fn i_05e3b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E3BA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E3E8; return 10; }
    8
}
/// $05E3BA: cmpi.l #$ffffffff, (a0)
pub fn i_05e3ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E3C0;
    cpu.ir = 0xC90;
    let v2 = bus.read32(cpu.a[0]);
    sub_with_flags(cpu, v2, 0xFFFFFFFF, Size::Long, false);
    16
}
/// $05E3C0: beq.w $5e3e8
pub fn i_05e3c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E3C4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E3E8; return 10; }
    8
}
/// $05E3C4: cmpi.l #$52a, (a0)
pub fn i_05e3c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E3CA;
    cpu.ir = 0xC90;
    let v2 = bus.read32(cpu.a[0]);
    sub_with_flags(cpu, v2, 0x52A, Size::Long, false);
    16
}
/// $05E3CA: beq.w $5e3e8
pub fn i_05e3ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E3CE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E3E8; return 10; }
    8
}
/// $05E3CE: cmpi.l #$400, (a0)
pub fn i_05e3ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E3D4;
    cpu.ir = 0xC90;
    let v2 = bus.read32(cpu.a[0]);
    sub_with_flags(cpu, v2, 0x400, Size::Long, false);
    16
}
/// $05E3D4: beq.w $5e3e8
pub fn i_05e3d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E3D8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E3E8; return 10; }
    8
}
/// $05E3D8: cmpi.l #$2ae3e, (a0)
pub fn i_05e3d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E3DE;
    cpu.ir = 0xC90;
    let v2 = bus.read32(cpu.a[0]);
    sub_with_flags(cpu, v2, 0x2AE3E, Size::Long, false);
    16
}
/// $05E3DE: beq.w $5e3e8
pub fn i_05e3de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E3E2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E3E8; return 10; }
    8
}
// ===== SetXN_05e3e2  ($05E3E2, 6 B, thunks/ccr_helpers.c)
/// $05E3E2: ori.b #$11, ccr
pub fn i_05e3e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E3E6;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05E3E6: rts
pub fn i_05e3e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E3E8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_GetEntityFail_05e3e8  ($05E3E8, 6 B, props/late_props_turrets_05e000.s)
/// $05E3E8: movea.l #$ffffffff, a0
pub fn i_05e3e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E3EE;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
// ===== ClearXN_05e3ee  ($05E3EE, 6 B, thunks/ccr_helpers.c)
/// $05E3EE: andi.b #$ee, ccr
pub fn i_05e3ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E3F2;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05E3F2: rts
pub fn i_05e3f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E3F4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05E3FC: clr.w d0
pub fn i_05e3fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E3FE;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05E3FE: jsr $5e42a(pc)
pub fn i_05e3fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E402;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5E402);
    cpu.pc = 0x5E42A;
    16
}
/// $05E402: bcc.w $5e40c
pub fn i_05e402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E406;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5E40C; return 10; }
    8
}
/// $05E406: tst.w d0
pub fn i_05e406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E408;
    cpu.ir = 0x4A40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05E408: beq.w $5e424
pub fn i_05e408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E40C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E424; return 10; }
    8
}
/// $05E40C: move.w #$1, d0
pub fn i_05e40c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E410;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05E410: jsr $5e42a(pc)
pub fn i_05e410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E414;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5E414);
    cpu.pc = 0x5E42A;
    16
}
/// $05E414: bcc.w $5e41e
pub fn i_05e414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E418;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5E41E; return 10; }
    8
}
/// $05E418: tst.w d0
pub fn i_05e418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E41A;
    cpu.ir = 0x4A40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05E41A: beq.w $5e424
pub fn i_05e41a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E41E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E424; return 10; }
    8
}
// ===== ClearXN_05e41e  ($05E41E, 6 B, thunks/ccr_helpers.c)
/// $05E41E: andi.b #$ee, ccr
pub fn i_05e41e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E422;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05E422: rts
pub fn i_05e422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E424;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_05e424  ($05E424, 6 B, thunks/ccr_helpers.c)
/// $05E424: ori.b #$11, ccr
pub fn i_05e424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E428;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05E428: rts
pub fn i_05e428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E42A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_ProbeFlag6B_05e42a  ($05E42A, 28 B, props/late_props_turrets_05e000.s)
/// $05E42A: jsr $5e3a2(pc)
pub fn i_05e42a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E42E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5E42E);
    cpu.pc = 0x5E3A2;
    16
}
/// $05E42E: bcc.w $5e44c
pub fn i_05e42e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E432;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5E44C; return 10; }
    8
}
/// $05E432: btst.b #$0, $6b(a6)
pub fn i_05e432(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E438;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x6B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05E438: beq.w $5e442
pub fn i_05e438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E43C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E442; return 10; }
    8
}
/// $05E43C: clr.b d0
pub fn i_05e43c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E43E;
    cpu.ir = 0x4200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05E43E: bra.w $5e446
pub fn i_05e43e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E442;
    cpu.ir = 0x6000;
    cpu.pc = 0x5E446;
    10
}
/// $05E442: move.b #$ff, d0
pub fn i_05e442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E446;
    cpu.ir = 0x103C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
// ===== SetXN_05e446  ($05E446, 6 B, thunks/ccr_helpers.c)
/// $05E446: ori.b #$11, ccr
pub fn i_05e446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E44A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05E44A: rts
pub fn i_05e44a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E44C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_05e44c  ($05E44C, 6 B, thunks/ccr_helpers.c)
/// $05E44C: andi.b #$ee, ccr
pub fn i_05e44c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E450;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05E450: rts
pub fn i_05e450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E452;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Parent_IsLiveHandler_05e452  ($05E452, 8 B, props/late_props_turrets_05e000.s)
/// $05E452: movea.l $c(a6), a0
pub fn i_05e452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E456;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $05E456: jmp $5e3ba(pc)
pub fn i_05e456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E45A;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x5E3BA;
    8
}
// ===== Parent_IsFreed_05e45a  ($05E45A, 34 B, props/late_props_turrets_05e000.s)
/// $05E45A: movea.l $c(a6), a0
pub fn i_05e45a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E45E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $05E45E: cmpi.l #$ffffffff, (a0)
pub fn i_05e45e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E464;
    cpu.ir = 0xC90;
    let v2 = bus.read32(cpu.a[0]);
    sub_with_flags(cpu, v2, 0xFFFFFFFF, Size::Long, false);
    16
}
/// $05E464: beq.w $5e482
pub fn i_05e464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E468;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E482; return 10; }
    8
}
/// $05E468: cmpi.l #$52a, (a0)
pub fn i_05e468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E46E;
    cpu.ir = 0xC90;
    let v2 = bus.read32(cpu.a[0]);
    sub_with_flags(cpu, v2, 0x52A, Size::Long, false);
    16
}
/// $05E46E: beq.w $5e482
pub fn i_05e46e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E472;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E482; return 10; }
    8
}
/// $05E472: cmpi.l #$400, (a0)
pub fn i_05e472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E478;
    cpu.ir = 0xC90;
    let v2 = bus.read32(cpu.a[0]);
    sub_with_flags(cpu, v2, 0x400, Size::Long, false);
    16
}
/// $05E478: beq.w $5e482
pub fn i_05e478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E47C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E482; return 10; }
    8
}
// ===== ClearXN_05e47c  ($05E47C, 6 B, thunks/ccr_helpers.c)
/// $05E47C: andi.b #$ee, ccr
pub fn i_05e47c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E480;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05E480: rts
pub fn i_05e480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E482;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_05e482  ($05E482, 6 B, thunks/ccr_helpers.c)
/// $05E482: ori.b #$11, ccr
pub fn i_05e482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E486;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05E486: rts
pub fn i_05e486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E488;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_IsFreedA0_05e488  ($05E488, 30 B, props/late_props_turrets_05e000.s)
/// $05E488: cmpi.l #$ffffffff, (a0)
pub fn i_05e488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E48E;
    cpu.ir = 0xC90;
    let v2 = bus.read32(cpu.a[0]);
    sub_with_flags(cpu, v2, 0xFFFFFFFF, Size::Long, false);
    16
}
/// $05E48E: beq.w $5e4ac
pub fn i_05e48e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E492;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E4AC; return 10; }
    8
}
/// $05E492: cmpi.l #$52a, (a0)
pub fn i_05e492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E498;
    cpu.ir = 0xC90;
    let v2 = bus.read32(cpu.a[0]);
    sub_with_flags(cpu, v2, 0x52A, Size::Long, false);
    16
}
/// $05E498: beq.w $5e4ac
pub fn i_05e498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E49C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E4AC; return 10; }
    8
}
/// $05E49C: cmpi.l #$400, (a0)
pub fn i_05e49c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E4A2;
    cpu.ir = 0xC90;
    let v2 = bus.read32(cpu.a[0]);
    sub_with_flags(cpu, v2, 0x400, Size::Long, false);
    16
}
/// $05E4A2: beq.w $5e4ac
pub fn i_05e4a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E4A6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E4AC; return 10; }
    8
}
// ===== SetXN_05e4a6  ($05E4A6, 6 B, thunks/ccr_helpers.c)
/// $05E4A6: ori.b #$11, ccr
pub fn i_05e4a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E4AA;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05E4AA: rts
pub fn i_05e4aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E4AC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_05e4ac  ($05E4AC, 6 B, thunks/ccr_helpers.c)
/// $05E4AC: andi.b #$ee, ccr
pub fn i_05e4ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E4B0;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05E4B0: rts
pub fn i_05e4b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E4B2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_ReserveAndSetPos_05E4B2  ($05E4B2, 24 B, entity/reserve_and_setpos_05e4b2.s)
/// $05E4B2: bset.b #$6, $13(a6)
pub fn i_05e4b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E4B8;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05E4B8: jsr $5e4ca(pc)
pub fn i_05e4b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E4BC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5E4BC);
    cpu.pc = 0x5E4CA;
    16
}
/// $05E4BC: move.w d0, $38(a6)
pub fn i_05e4bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E4C0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05E4C0: move.w d1, $22(a6)
pub fn i_05e4c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E4C4;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05E4C4: move.w d2, $24(a6)
pub fn i_05e4c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E4C8;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05E4C8: rts
pub fn i_05e4c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E4CA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Parent_GetPrioPos_05e4ca  ($05E4CA, 18 B, props/late_props_turrets_05e000.s)
/// $05E4CA: movea.l $c(a6), a0
pub fn i_05e4ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E4CE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $05E4CE: move.w $38(a0), d0
pub fn i_05e4ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E4D2;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05E4D2: move.w $22(a0), d1
pub fn i_05e4d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E4D6;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05E4D6: move.w $24(a0), d2
pub fn i_05e4d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E4DA;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05E4DA: rts
pub fn i_05e4da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E4DC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Parent_CopyPos_05e4dc  ($05E4DC, 18 B, props/late_props_turrets_05e000.s)
/// $05E4DC: movea.l $c(a6), a0
pub fn i_05e4dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E4E0;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $05E4E0: move.w $22(a0), $22(a6)
pub fn i_05e4e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E4E6;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $05E4E6: move.w $24(a0), $24(a6)
pub fn i_05e4e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E4EC;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $05E4EC: rts
pub fn i_05e4ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E4EE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Parent_CopyPosPrio_05e4ee  ($05E4EE, 24 B, props/late_props_turrets_05e000.s)
/// $05E4EE: movea.l $c(a6), a0
pub fn i_05e4ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E4F2;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $05E4F2: move.w $22(a0), $22(a6)
pub fn i_05e4f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E4F8;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $05E4F8: move.w $24(a0), $24(a6)
pub fn i_05e4f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E4FE;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $05E4FE: move.w $38(a0), $38(a6)
pub fn i_05e4fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E504;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad4, v3 as u16);
    20
}
/// $05E504: rts
pub fn i_05e504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E506;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Parent_CopyPosPrioFacing_05e506  ($05E506, 42 B, props/late_props_turrets_05e000.s)
/// $05E506: movea.l $c(a6), a0
pub fn i_05e506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E50A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $05E50A: move.w $22(a0), $22(a6)
pub fn i_05e50a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E510;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $05E510: move.w $24(a0), $24(a6)
pub fn i_05e510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E516;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $05E516: move.b $26(a0), $26(a6)
pub fn i_05e516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E51C;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x26);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x26);
    bus.write8(ad4, v3 as u8);
    20
}
/// $05E51C: move.b $27(a0), $27(a6)
pub fn i_05e51c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E522;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x27);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x27);
    bus.write8(ad4, v3 as u8);
    20
}
/// $05E522: move.b $3a(a0), $3a(a6)
pub fn i_05e522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E528;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $05E528: move.w $38(a0), $38(a6)
pub fn i_05e528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E52E;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad4, v3 as u16);
    20
}
/// $05E52E: rts
pub fn i_05e52e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E530;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Rng_PickWordFromTable_05e530  ($05E530, 12 B, props/late_props_turrets_05e000.s)
/// $05E530: jsr $5e9e4.l
pub fn i_05e530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E536;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5E536);
    cpu.pc = 0x5E9E4;
    16
}
/// $05E536: asl.w #$1, d0
pub fn i_05e536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E538;
    cpu.ir = 0xE340;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $05E538: move.w (a1, d0.w), d0
pub fn i_05e538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E53C;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
// ===== JsrAbsThunk_05e53c  ($05E53C, 8 B, thunks/jsr_abs_thunks.c)
/// $05E53C: jsr $2352.l
pub fn i_05e53c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E542;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5E542);
    cpu.pc = 0x2352;
    16
}
/// $05E542: rts
pub fn i_05e542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E544;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrPcThunk_05e54c  ($05E54C, 6 B, thunks/jsr_pc_thunks.c)
/// $05E54C: jsr $5e530(pc)
pub fn i_05e54c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E550;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5E550);
    cpu.pc = 0x5E530;
    16
}
/// $05E550: rts
pub fn i_05e550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E552;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05E55C: moveq #$0, d7
pub fn i_05e55c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E55E;
    cpu.ir = 0x7E00;
    cpu.d[7] = 0x0;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05E55E: clr.w d0
pub fn i_05e55e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E560;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05E560: jsr $5e3a2(pc)
pub fn i_05e560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E564;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5E564);
    cpu.pc = 0x5E3A2;
    16
}
/// $05E564: bcc.w $5e580
pub fn i_05e564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E568;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5E580; return 10; }
    8
}
/// $05E568: btst.b #$0, $13(a0)
pub fn i_05e568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E56E;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05E56E: bne.w $5e580
pub fn i_05e56e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E572;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5E580; return 10; }
    8
}
/// $05E572: btst.b #$7, $5b(a0)
pub fn i_05e572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E578;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x5B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05E578: beq.w $5e580
pub fn i_05e578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E57C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E580; return 10; }
    8
}
/// $05E57C: bset.b #$0, d7
pub fn i_05e57c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E580;
    cpu.ir = 0x8C7;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 | mk;
    cpu.d[7] = r2;
    8
}
/// $05E580: move.w #$1, d0
pub fn i_05e580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E584;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05E584: jsr $5e3a2(pc)
pub fn i_05e584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E588;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5E588);
    cpu.pc = 0x5E3A2;
    16
}
/// $05E588: bcc.w $5e5a4
pub fn i_05e588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E58C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5E5A4; return 10; }
    8
}
/// $05E58C: btst.b #$0, $13(a0)
pub fn i_05e58c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E592;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05E592: bne.w $5e5a4
pub fn i_05e592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E596;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5E5A4; return 10; }
    8
}
/// $05E596: btst.b #$7, $5b(a0)
pub fn i_05e596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E59C;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x5B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05E59C: beq.w $5e5a4
pub fn i_05e59c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E5A0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E5A4; return 10; }
    8
}
/// $05E5A0: bset.b #$1, d7
pub fn i_05e5a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E5A4;
    cpu.ir = 0x8C7;
    let mk: u32 = 1u32 << ((1) % 32);
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 | mk;
    cpu.d[7] = r2;
    8
}
/// $05E5A4: move.b d7, d0
pub fn i_05e5a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E5A6;
    cpu.ir = 0x1007;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05E5A6: rts
pub fn i_05e5a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E5A8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ProbeTwoAttemptsCcr_05E5A8  ($05E5A8, 56 B, kernel/probe_two_attempts_05e5a8.s)
/// $05E5A8: clr.w d0
pub fn i_05e5a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E5AA;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05E5AA: jsr $5e3a2(pc)
pub fn i_05e5aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E5AE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5E5AE);
    cpu.pc = 0x5E3A2;
    16
}
/// $05E5AE: bcc.w $5e5c0
pub fn i_05e5ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E5B2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5E5C0; return 10; }
    8
}
/// $05E5B2: jsr $5e618(pc)
pub fn i_05e5b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E5B6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5E5B6);
    cpu.pc = 0x5E618;
    16
}
/// $05E5B6: bcs.w $5e5c0
pub fn i_05e5b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E5BA;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5E5C0; return 10; }
    8
}
/// $05E5BA: andi.b #$ee, ccr
pub fn i_05e5ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E5BE;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05E5BE: rts
pub fn i_05e5be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E5C0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05E5C0: move.w #$1, d0
pub fn i_05e5c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E5C4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05E5C4: jsr $5e3a2(pc)
pub fn i_05e5c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E5C8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5E5C8);
    cpu.pc = 0x5E3A2;
    16
}
/// $05E5C8: bcc.w $5e5da
pub fn i_05e5c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E5CC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5E5DA; return 10; }
    8
}
/// $05E5CC: jsr $5e618(pc)
pub fn i_05e5cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E5D0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5E5D0);
    cpu.pc = 0x5E618;
    16
}
/// $05E5D0: bcs.w $5e5da
pub fn i_05e5d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E5D4;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5E5DA; return 10; }
    8
}
/// $05E5D4: andi.b #$ee, ccr
pub fn i_05e5d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E5D8;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05E5D8: rts
pub fn i_05e5d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E5DA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05E5DA: ori.b #$11, ccr
pub fn i_05e5da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E5DE;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05E5DE: rts
pub fn i_05e5de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E5E0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Players_AnyAhead_05e5e0  ($05E5E0, 18 B, props/late_props_turrets_05e000.s)
/// $05E5E0: clr.w d0
pub fn i_05e5e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E5E2;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05E5E2: jsr $5e3a2(pc)
pub fn i_05e5e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E5E6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5E5E6);
    cpu.pc = 0x5E3A2;
    16
}
/// $05E5E6: bcc.w $5e5f8
pub fn i_05e5e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E5EA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5E5F8; return 10; }
    8
}
/// $05E5EA: jsr $5e618(pc)
pub fn i_05e5ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E5EE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5E5EE);
    cpu.pc = 0x5E618;
    16
}
/// $05E5EE: bcc.w $5e5f8
pub fn i_05e5ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E5F2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5E5F8; return 10; }
    8
}
// ===== ClearXN_05e5f2  ($05E5F2, 6 B, thunks/ccr_helpers.c)
/// $05E5F2: andi.b #$ee, ccr
pub fn i_05e5f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E5F6;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05E5F6: rts
pub fn i_05e5f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E5F8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player2_IsAhead_05e5f8  ($05E5F8, 20 B, props/late_props_turrets_05e000.s)
/// $05E5F8: move.w #$1, d0
pub fn i_05e5f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E5FC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05E5FC: jsr $5e3a2(pc)
pub fn i_05e5fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E600;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5E600);
    cpu.pc = 0x5E3A2;
    16
}
/// $05E600: bcc.w $5e612
pub fn i_05e600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E604;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5E612; return 10; }
    8
}
/// $05E604: jsr $5e618(pc)
pub fn i_05e604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E608;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5E608);
    cpu.pc = 0x5E618;
    16
}
/// $05E608: bcc.w $5e612
pub fn i_05e608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E60C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5E612; return 10; }
    8
}
// ===== ClearXN_05e60c  ($05E60C, 6 B, thunks/ccr_helpers.c)
/// $05E60C: andi.b #$ee, ccr
pub fn i_05e60c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E610;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05E610: rts
pub fn i_05e610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E612;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_05e612  ($05E612, 6 B, thunks/ccr_helpers.c)
/// $05E612: ori.b #$11, ccr
pub fn i_05e612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E616;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05E616: rts
pub fn i_05e616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E618;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Target_IsAhead_05e618  ($05E618, 26 B, props/late_props_turrets_05e000.s)
/// $05E618: move.w $22(a6), d0
pub fn i_05e618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E61C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05E61C: sub.w $22(a0), d0
pub fn i_05e61c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E620;
    cpu.ir = 0x9068;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $05E620: btst.b #$0, $3a(a6)
pub fn i_05e620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E626;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05E626: bne.w $5e638
pub fn i_05e626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E62A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5E638; return 10; }
    8
}
/// $05E62A: cmpi.w #$0, d0
pub fn i_05e62a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E62E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $05E62E: blt.w $5e646
pub fn i_05e62e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E632;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x5E646; return 10; }
    8
}
// ===== ClearXN_05e632  ($05E632, 6 B, thunks/ccr_helpers.c)
/// $05E632: andi.b #$ee, ccr
pub fn i_05e632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E636;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05E636: rts
pub fn i_05e636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E638;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Target_IsAhead_FacingLeft_05e638  ($05E638, 8 B, props/late_props_turrets_05e000.s)
/// $05E638: cmpi.w #$0, d0
pub fn i_05e638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E63C;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $05E63C: bgt.w $5e646
pub fn i_05e63c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E640;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x5E646; return 10; }
    8
}
// ===== ClearXN_05e640  ($05E640, 6 B, thunks/ccr_helpers.c)
/// $05E640: andi.b #$ee, ccr
pub fn i_05e640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E644;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05E644: rts
pub fn i_05e644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E646;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_05e646  ($05E646, 6 B, thunks/ccr_helpers.c)
/// $05E646: ori.b #$11, ccr
pub fn i_05e646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E64A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05E64A: rts
pub fn i_05e64a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E64C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Marker_FollowParent_05e64c  ($05E64C, 68 B, props/late_props_turrets_05e000.s)
/// $05E64C: movea.l $c(a6), a1
pub fn i_05e64c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E650;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $05E650: move.w $16(a1), $16(a6)
pub fn i_05e650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E656;
    cpu.ir = 0x3D69;
    let ad2 = cpu.a[1].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x16);
    bus.write16(ad4, v3 as u16);
    20
}
/// $05E656: move.w #$e000, $38(a6)
pub fn i_05e656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E65C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xE000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05E65C: lea.l $5e698(pc), a0
pub fn i_05e65c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E660;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x5E698;
    16
}
/// $05E660: jsr $28cd4.l
pub fn i_05e660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E666;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5E666);
    cpu.pc = 0x28CD4;
    16
}
/// $05E666: lea.l $5e66c(pc), a1
pub fn i_05e666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E66A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5E66C;
    16
}
/// $05E66A: move.l a1, (a6)
pub fn i_05e66a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E66C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05E66C: jsr $5e4dc(pc)
pub fn i_05e66c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E670;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5E670);
    cpu.pc = 0x5E4DC;
    16
}
/// $05E670: jsr $28d70.l
pub fn i_05e670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E676;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5E676);
    cpu.pc = 0x28D70;
    16
}
/// $05E676: movea.l $c(a6), a1
pub fn i_05e676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E67A;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $05E67A: cmpi.l #$ffffffff, (a1)
pub fn i_05e67a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E680;
    cpu.ir = 0xC91;
    let v2 = bus.read32(cpu.a[1]);
    sub_with_flags(cpu, v2, 0xFFFFFFFF, Size::Long, false);
    16
}
/// $05E680: beq.w $5e690
pub fn i_05e680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E684;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E690; return 10; }
    8
}
/// $05E684: cmpi.l #$52a, (a1)
pub fn i_05e684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E68A;
    cpu.ir = 0xC91;
    let v2 = bus.read32(cpu.a[1]);
    sub_with_flags(cpu, v2, 0x52A, Size::Long, false);
    16
}
/// $05E68A: beq.w $5e690
pub fn i_05e68a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E68E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E690; return 10; }
    8
}
/// $05E68E: rts
pub fn i_05e68e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E690;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_05e690  ($05E690, 8 B, kernel/jmp_to_scheduler.c)
/// $05E690: jmp $518.l
pub fn i_05e690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E696;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Hit_ClassifyAttack_05e6a4  ($05E6A4, 102 B, props/late_props_turrets_05e000.s)
/// $05E6A4: ori.b #$3, d1
pub fn i_05e6a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E6A8;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05E6A8: subi.b #$1, d4
pub fn i_05e6a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E6AC;
    cpu.ir = 0x404;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x1, Size::Byte, true);
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05E6AC: andi.b #$1, d3
pub fn i_05e6ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E6B0;
    cpu.ir = 0x203;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05E6B0: andi.b #$2, d2
pub fn i_05e6b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E6B4;
    cpu.ir = 0x202;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 & 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05E6B4: andi.b #$b9, d2
pub fn i_05e6b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E6B8;
    cpu.ir = 0x202;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 & 0xB9) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05E6B8: ori.b #$a, d2
pub fn i_05e6b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E6BC;
    cpu.ir = 0x2;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 | 0xA) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05E6BC: bcc.w $5e710
pub fn i_05e6bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E6C0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5E710; return 10; }
    8
}
/// $05E6C0: clr.l d0
pub fn i_05e6c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E6C2;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $05E6C2: move.b $58(a6), d0
pub fn i_05e6c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E6C6;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05E6C6: andi.l #$ff, d0
pub fn i_05e6c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E6CC;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $05E6CC: clr.b d1
pub fn i_05e6cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E6CE;
    cpu.ir = 0x4201;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05E6CE: cmpi.b #$5, d0
pub fn i_05e6ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E6D2;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x5, Size::Byte, false);
    8
}
/// $05E6D2: ble.w $5e6da
pub fn i_05e6d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E6D6;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x5E6DA; return 10; }
    8
}
/// $05E6D6: move.b #$1, d1
pub fn i_05e6d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E6DA;
    cpu.ir = 0x123C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05E6DA: cmpi.w #$22, d0
pub fn i_05e6da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E6DE;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x22, Size::Word, false);
    8
}
/// $05E6DE: blt.w $5e6ee
pub fn i_05e6de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E6E2;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x5E6EE; return 10; }
    8
}
/// $05E6E2: nop
pub fn i_05e6e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E6E4;
    cpu.ir = 0x4E71;
    4
}
/// $05E6E4: nop
pub fn i_05e6e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E6E6;
    cpu.ir = 0x4E71;
    4
}
/// $05E6E6: cmpi.w #$22, d0
pub fn i_05e6e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E6EA;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x22, Size::Word, false);
    8
}
/// $05E6EA: nop
pub fn i_05e6ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E6EC;
    cpu.ir = 0x4E71;
    4
}
/// $05E6EC: trap #$f
pub fn i_05e6ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E6EE;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $05E6EE: lea.l $5e6a4(pc), a0
pub fn i_05e6ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E6F2;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x5E6A4;
    16
}
/// $05E6F2: move.b (a0, d0.w), d0
pub fn i_05e6f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E6F6;
    cpu.ir = 0x1030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $05E6F6: cmpi.w #$5, d0
pub fn i_05e6f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E6FA;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x5, Size::Word, false);
    8
}
/// $05E6FA: blt.w $5e70a
pub fn i_05e6fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E6FE;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x5E70A; return 10; }
    8
}
/// $05E6FE: nop
pub fn i_05e6fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E700;
    cpu.ir = 0x4E71;
    4
}
/// $05E700: nop
pub fn i_05e700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E702;
    cpu.ir = 0x4E71;
    4
}
/// $05E702: cmpi.w #$5, d0
pub fn i_05e702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E706;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x5, Size::Word, false);
    8
}
/// $05E706: nop
pub fn i_05e706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E708;
    cpu.ir = 0x4E71;
    4
}
/// $05E708: trap #$f
pub fn i_05e708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E70A;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
// ===== SetXN_05e70a  ($05E70A, 6 B, thunks/ccr_helpers.c)
/// $05E70A: ori.b #$11, ccr
pub fn i_05e70a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E70E;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05E70E: rts
pub fn i_05e70e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E710;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hit_ClassNone_05e710  ($05E710, 4 B, props/late_props_turrets_05e000.s)
/// $05E710: clr.l d1
pub fn i_05e710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E712;
    cpu.ir = 0x4281;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = r2;
    4
}
/// $05E712: clr.l d0
pub fn i_05e712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E714;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
// ===== ClearXN_05e714  ($05E714, 6 B, thunks/ccr_helpers.c)
/// $05E714: andi.b #$ee, ccr
pub fn i_05e714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E718;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05E718: rts
pub fn i_05e718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E71A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hit_HPDepletedClassify_05e71a  ($05E71A, 8 B, props/late_props_turrets_05e000.s)
/// $05E71A: jsr $28758.l
pub fn i_05e71a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E720;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5E720);
    cpu.pc = 0x28758;
    16
}
/// $05E720: bra.b $5e6bc
pub fn i_05e720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E722;
    cpu.ir = 0x609A;
    cpu.pc = 0x5E6BC;
    10
}
// ===== SetTaskB_05e722  ($05E722, 6 B, kernel/task_setters_b.c)
/// $05E722: move.b d0, $59(a6)
pub fn i_05e722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E726;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x59);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05E726: rts
pub fn i_05e726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E728;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hit_SoundA_05e728  ($05E728, 28 B, props/late_props_turrets_05e000.s)
/// $05E728: btst.b #$0, $5a(a6)
pub fn i_05e728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E72E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05E72E: beq.w $5e74a
pub fn i_05e72e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E732;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E74A; return 10; }
    8
}
/// $05E732: btst.b #$0, $13(a6)
pub fn i_05e732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E738;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05E738: bne.w $5e74a
pub fn i_05e738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E73C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5E74A; return 10; }
    8
}
/// $05E73C: cmpi.w #$ffff, d0
pub fn i_05e73c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E740;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFFF, Size::Word, false);
    8
}
/// $05E740: beq.w $5e74a
pub fn i_05e740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E744;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E74A; return 10; }
    8
}
// ===== JsrAbsThunk_05e744  ($05E744, 8 B, thunks/jsr_abs_thunks.c)
/// $05E744: jsr $2352.l
pub fn i_05e744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E74A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5E74A);
    cpu.pc = 0x2352;
    16
}
/// $05E74A: rts
pub fn i_05e74a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E74C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hit_SoundB_05e74c  ($05E74C, 18 B, props/late_props_turrets_05e000.s)
/// $05E74C: btst.b #$1, $5a(a6)
pub fn i_05e74c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E752;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05E752: beq.w $5e764
pub fn i_05e752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E756;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E764; return 10; }
    8
}
/// $05E756: cmpi.w #$ffff, d0
pub fn i_05e756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E75A;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFFF, Size::Word, false);
    8
}
/// $05E75A: beq.w $5e764
pub fn i_05e75a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E75E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E764; return 10; }
    8
}
// ===== JsrAbsThunk_05e75e  ($05E75E, 8 B, thunks/jsr_abs_thunks.c)
/// $05E75E: jsr $2352.l
pub fn i_05e75e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E764;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5E764);
    cpu.pc = 0x2352;
    16
}
/// $05E764: rts
pub fn i_05e764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E766;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05E770: btst.b #$0, $13(a6)
pub fn i_05e770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E776;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05E776: bne.w $5e796
pub fn i_05e776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E77A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5E796; return 10; }
    8
}
/// $05E77A: move.b $58(a6), d0
pub fn i_05e77a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E77E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05E77E: andi.w #$ff, d0
pub fn i_05e77e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E782;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05E782: subq.w #$1, d0
pub fn i_05e782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E784;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05E784: cmpi.w #$5, d0
pub fn i_05e784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E788;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x5, Size::Word, false);
    8
}
/// $05E788: bcc.w $5e796
pub fn i_05e788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E78C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5E796; return 10; }
    8
}
/// $05E78C: asl.w #$1, d0
pub fn i_05e78c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E78E;
    cpu.ir = 0xE340;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $05E78E: move.w (a0, d0.w), d0
pub fn i_05e78e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E792;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $05E792: jmp $5e728(pc)
pub fn i_05e792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E796;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x5E728;
    8
}
/// $05E796: rts
pub fn i_05e796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E798;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hit_SoundByAttackClassTbl_05e798  ($05E798, 40 B, props/late_props_turrets_05e000.s)
/// $05E798: btst.b #$0, $13(a6)
pub fn i_05e798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E79E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05E79E: bne.w $5e7be
pub fn i_05e79e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E7A2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5E7BE; return 10; }
    8
}
/// $05E7A2: move.b $58(a6), d0
pub fn i_05e7a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E7A6;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05E7A6: andi.w #$ff, d0
pub fn i_05e7a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E7AA;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05E7AA: subq.w #$1, d0
pub fn i_05e7aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E7AC;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05E7AC: cmpi.w #$5, d0
pub fn i_05e7ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E7B0;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x5, Size::Word, false);
    8
}
/// $05E7B0: bcc.w $5e7be
pub fn i_05e7b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E7B4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5E7BE; return 10; }
    8
}
/// $05E7B4: asl.w #$1, d0
pub fn i_05e7b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E7B6;
    cpu.ir = 0xE340;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $05E7B6: move.w (a0, d0.w), d0
pub fn i_05e7b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E7BA;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $05E7BA: jmp $5e73c(pc)
pub fn i_05e7ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E7BE;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x5E73C;
    8
}
/// $05E7BE: rts
pub fn i_05e7be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E7C0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_SnapToGround_05e7c0  ($05E7C0, 24 B, props/late_props_turrets_05e000.s)
/// $05E7C0: andi.w #$3ff, $24(a6)
pub fn i_05e7c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E7C6;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x3FF) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $05E7C6: cmpi.w #$0, $24(a6)
pub fn i_05e7c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E7CC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $05E7CC: blt.w $5e7de
pub fn i_05e7cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E7D0;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x5E7DE; return 10; }
    8
}
/// $05E7D0: jsr $27c8c.l
pub fn i_05e7d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E7D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5E7D6);
    cpu.pc = 0x27C8C;
    16
}
/// $05E7D6: bcc.b $5e7c6
pub fn i_05e7d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E7D8;
    cpu.ir = 0x64EE;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5E7C6; return 10; }
    8
}
// ===== SetXN_05e7d8  ($05E7D8, 6 B, thunks/ccr_helpers.c)
/// $05E7D8: ori.b #$11, ccr
pub fn i_05e7d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E7DC;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05E7DC: rts
pub fn i_05e7dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E7DE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_ResetPosDefault_05e7de  ($05E7DE, 12 B, props/late_props_turrets_05e000.s)
/// $05E7DE: move.w #$a0, $22(a6)
pub fn i_05e7de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E7E4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05E7E4: move.w #$180, $24(a6)
pub fn i_05e7e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E7EA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== ClearXN_05e7ea  ($05E7EA, 6 B, thunks/ccr_helpers.c)
/// $05E7EA: andi.b #$ee, ccr
pub fn i_05e7ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E7EE;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05E7EE: rts
pub fn i_05e7ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E7F0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Fix_Write106E8C_05e7f0  ($05E7F0, 12 B, props/late_props_turrets_05e000.s)
/// $05E7F0: movea.l #$7144, a1
pub fn i_05e7f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E7F6;
    cpu.ir = 0x227C;
    let v1: u32 = 0x7144;
    cpu.a[1] = v1;
    4
}
/// $05E7F6: move.w $106e8c.l, d0
pub fn i_05e7f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E7FC;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106E8C) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
// ===== JsrAbsThunk_05e7fc  ($05E7FC, 8 B, thunks/jsr_abs_thunks.c)
/// $05E7FC: jsr $5d6c2.l
pub fn i_05e7fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E802;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5E802);
    cpu.pc = 0x5D6C2;
    16
}
/// $05E802: rts
pub fn i_05e802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E804;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Scroll_IsPastQuarter_05e804  ($05E804, 26 B, props/late_props_turrets_05e000.s)
/// $05E804: move.w $10e1e8.l, d0
pub fn i_05e804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E80A;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x10E1E8) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $05E80A: move.w $10e1fa.l, d1
pub fn i_05e80a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E810;
    cpu.ir = 0x3239;
    let v2 = bus.read16(0x10E1FA) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $05E810: sub.w $10e1f6.l, d1
pub fn i_05e810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E816;
    cpu.ir = 0x9279;
    let v2 = bus.read16(0x10E1F6) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $05E816: move.w d1, d2
pub fn i_05e816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E818;
    cpu.ir = 0x3401;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05E818: lsr.w #$2, d2
pub fn i_05e818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E81A;
    cpu.ir = 0xE44A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $05E81A: cmp.w d2, d0
pub fn i_05e81a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E81C;
    cpu.ir = 0xB042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05E81C: rts
pub fn i_05e81c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E81E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_SetField14By106F28_05e81e  ($05E81E, 38 B, props/late_props_turrets_05e000.s)
/// $05E81E: ori.b #$18, (a6)
pub fn i_05e81e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E822;
    cpu.ir = 0x16;
    let ad1 = cpu.a[6];
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x18) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $05E822: ori.b #$1a, (a6)
pub fn i_05e822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E826;
    cpu.ir = 0x16;
    let ad1 = cpu.a[6];
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x1A) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $05E826: moveq #$0, d0
pub fn i_05e826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E828;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05E828: move.b $106f28.l, d0
pub fn i_05e828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E82E;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $05E82E: andi.w #$3, d0
pub fn i_05e82e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E832;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05E832: lsl.w #$1, d0
pub fn i_05e832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E834;
    cpu.ir = 0xE348;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $05E834: lea.l $5e81e(pc), a0
pub fn i_05e834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E838;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x5E81E;
    16
}
/// $05E838: move.w (a0, d0.w), d1
pub fn i_05e838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E83C;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $05E83C: move.w (a6, d1.w), $14(a6)
pub fn i_05e83c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E842;
    cpu.ir = 0x3D76;
    let ad2 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    22
}
/// $05E842: rts
pub fn i_05e842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E844;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Attack_IsHeavyType_05e844  ($05E844, 70 B, props/late_props_turrets_05e000.s)
/// $05E844: cmpi.b #$2, $58(a6)
pub fn i_05e844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E84A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $05E84A: beq.w $5e890
pub fn i_05e84a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E84E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E890; return 10; }
    8
}
/// $05E84E: cmpi.b #$3, $58(a6)
pub fn i_05e84e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E854;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $05E854: beq.w $5e890
pub fn i_05e854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E858;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E890; return 10; }
    8
}
/// $05E858: cmpi.b #$15, $58(a6)
pub fn i_05e858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E85E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x15, Size::Byte, false);
    16
}
/// $05E85E: beq.w $5e890
pub fn i_05e85e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E862;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E890; return 10; }
    8
}
/// $05E862: cmpi.b #$16, $58(a6)
pub fn i_05e862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E868;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x16, Size::Byte, false);
    16
}
/// $05E868: beq.w $5e890
pub fn i_05e868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E86C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E890; return 10; }
    8
}
/// $05E86C: cmpi.b #$17, $58(a6)
pub fn i_05e86c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E872;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x17, Size::Byte, false);
    16
}
/// $05E872: beq.w $5e890
pub fn i_05e872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E876;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E890; return 10; }
    8
}
/// $05E876: cmpi.b #$18, $58(a6)
pub fn i_05e876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E87C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x18, Size::Byte, false);
    16
}
/// $05E87C: beq.w $5e890
pub fn i_05e87c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E880;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E890; return 10; }
    8
}
/// $05E880: cmpi.b #$1e, $58(a6)
pub fn i_05e880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E886;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1E, Size::Byte, false);
    16
}
/// $05E886: beq.w $5e890
pub fn i_05e886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E88A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E890; return 10; }
    8
}
// ===== ClearXN_05e88a  ($05E88A, 6 B, thunks/ccr_helpers.c)
/// $05E88A: andi.b #$ee, ccr
pub fn i_05e88a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E88E;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05E88E: rts
pub fn i_05e88e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E890;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_05e890  ($05E890, 6 B, thunks/ccr_helpers.c)
/// $05E890: ori.b #$11, ccr
pub fn i_05e890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E894;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05E894: rts
pub fn i_05e894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E896;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Players_AnyHit2AC0E_05e896  ($05E896, 34 B, props/late_props_turrets_05e000.s)
/// $05E896: lea.l $100440.l, a0
pub fn i_05e896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E89C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100440;
    20
}
/// $05E89C: jsr $2ac0e.l
pub fn i_05e89c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E8A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5E8A2);
    cpu.pc = 0x2AC0E;
    16
}
/// $05E8A2: bcs.w $5e8b8
pub fn i_05e8a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E8A6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5E8B8; return 10; }
    8
}
/// $05E8A6: lea.l $1004e0.l, a0
pub fn i_05e8a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E8AC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1004E0;
    20
}
/// $05E8AC: jsr $2ac0e.l
pub fn i_05e8ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E8B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5E8B2);
    cpu.pc = 0x2AC0E;
    16
}
/// $05E8B2: bcs.w $5e8b8
pub fn i_05e8b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E8B6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5E8B8; return 10; }
    8
}
/// $05E8B6: rts
pub fn i_05e8b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E8B8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Stub_0005E8B8  ($05E8B8, 2 B, thunks/trivial_rts.c)
/// $05E8B8: rts
pub fn i_05e8b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E8BA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_IsOnscreenMin_05e8ba  ($05E8BA, 20 B, props/late_props_turrets_05e000.s)
/// $05E8BA: cmpi.w #$20, $24(a6)
pub fn i_05e8ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E8C0;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x20, Size::Word, false);
    16
}
/// $05E8C0: blt.w $5e8d4
pub fn i_05e8c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E8C4;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x5E8D4; return 10; }
    8
}
/// $05E8C4: cmpi.w #$ff80, $22(a6)
pub fn i_05e8c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E8CA;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF80, Size::Word, false);
    16
}
/// $05E8CA: blt.w $5e8d4
pub fn i_05e8ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E8CE;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x5E8D4; return 10; }
    8
}
// ===== JsrPcThunk_05e8ce  ($05E8CE, 6 B, thunks/jsr_pc_thunks.c)
/// $05E8CE: jsr $5dd5c(pc)
pub fn i_05e8ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E8D2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5E8D2);
    cpu.pc = 0x5DD5C;
    16
}
/// $05E8D2: rts
pub fn i_05e8d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E8D4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_05e8d4  ($05E8D4, 6 B, thunks/ccr_helpers.c)
/// $05E8D4: ori.b #$11, ccr
pub fn i_05e8d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E8D8;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05E8D8: rts
pub fn i_05e8d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E8DA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Attack_ApplyThenSound10FA_05e8da  ($05E8DA, 20 B, props/late_props_turrets_05e000.s)
/// $05E8DA: jsr $283d8.l
pub fn i_05e8da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E8E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5E8E0);
    cpu.pc = 0x283D8;
    16
}
/// $05E8E0: btst.b #$1, $5a(a6)
pub fn i_05e8e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E8E6;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05E8E6: beq.w $5e8f4
pub fn i_05e8e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E8EA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E8F4; return 10; }
    8
}
/// $05E8EA: move.w #$10fa, d0
pub fn i_05e8ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E8EE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10FA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_05e8ee  ($05E8EE, 8 B, thunks/jsr_abs_thunks.c)
/// $05E8EE: jsr $2352.l
pub fn i_05e8ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E8F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5E8F4);
    cpu.pc = 0x2352;
    16
}
/// $05E8F4: rts
pub fn i_05e8f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E8F6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Owner_GenCheck_05e8f6  ($05E8F6, 16 B, props/late_props_turrets_05e000.s)
/// $05E8F6: movea.l $8(a6), a1
pub fn i_05e8f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E8FA;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $05E8FA: move.b $10(a6), d0
pub fn i_05e8fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E8FE;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05E8FE: cmp.b $10(a1), d0
pub fn i_05e8fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E902;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $05E902: bcs.w $5e90c
pub fn i_05e902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E906;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5E90C; return 10; }
    8
}
// ===== ClearXN_05e906  ($05E906, 6 B, thunks/ccr_helpers.c)
/// $05E906: andi.b #$ee, ccr
pub fn i_05e906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E90A;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05E90A: rts
pub fn i_05e90a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E90C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_05e90c  ($05E90C, 6 B, thunks/ccr_helpers.c)
/// $05E90C: ori.b #$11, ccr
pub fn i_05e90c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E910;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05E910: rts
pub fn i_05e910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E912;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hud_WriteTimerCounters_05e912  ($05E912, 112 B, props/late_props_turrets_05e000.s)
/// $05E912: movea.l #$70a5, a1
pub fn i_05e912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E918;
    cpu.ir = 0x227C;
    let v1: u32 = 0x70A5;
    cpu.a[1] = v1;
    4
}
/// $05E918: moveq #$0, d0
pub fn i_05e918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E91A;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05E91A: move.b $32(a6), d0
pub fn i_05e91a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E91E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05E91E: jsr $5d6c2.l
pub fn i_05e91e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E924;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5E924);
    cpu.pc = 0x5D6C2;
    16
}
/// $05E924: move.w #$0, d1
pub fn i_05e924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E928;
    cpu.ir = 0x323C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05E928: move.w d1, d2
pub fn i_05e928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E92A;
    cpu.ir = 0x3401;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05E92A: move.w $30(a6), d1
pub fn i_05e92a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E92E;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05E92E: move.b $32(a6), d2
pub fn i_05e92e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E932;
    cpu.ir = 0x142E;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05E932: add.w d2, d1
pub fn i_05e932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E934;
    cpu.ir = 0xD242;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05E934: cmpi.w #$ff, d1
pub fn i_05e934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E938;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Word, false);
    8
}
/// $05E938: ble.w $5e94a
pub fn i_05e938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E93C;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x5E94A; return 10; }
    8
}
/// $05E93C: move.b #$ff, d1
pub fn i_05e93c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E940;
    cpu.ir = 0x123C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05E940: move.w #$ffff, $30(a6)
pub fn i_05e940(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E946;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05E946: bra.w $5e95c
pub fn i_05e946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E94A;
    cpu.ir = 0x6000;
    cpu.pc = 0x5E95C;
    10
}
/// $05E94A: cmpi.w #$1, d1
pub fn i_05e94a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E94E;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x1, Size::Word, false);
    8
}
/// $05E94E: bge.w $5e95c
pub fn i_05e94e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E952;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x5E95C; return 10; }
    8
}
/// $05E952: move.b #$1, d1
pub fn i_05e952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E956;
    cpu.ir = 0x123C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05E956: move.w #$1, $30(a6)
pub fn i_05e956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E95C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05E95C: move.b d1, $32(a6)
pub fn i_05e95c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E960;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05E960: move.b d1, $33(a6)
pub fn i_05e960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E964;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05E964: movea.l #$70a7, a1
pub fn i_05e964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E96A;
    cpu.ir = 0x227C;
    let v1: u32 = 0x70A7;
    cpu.a[1] = v1;
    4
}
/// $05E96A: move.w $10e1e4.l, d0
pub fn i_05e96a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E970;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x10E1E4) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $05E970: jsr $5d6c2.l
pub fn i_05e970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E976;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5E976);
    cpu.pc = 0x5D6C2;
    16
}
/// $05E976: movea.l #$7147, a1
pub fn i_05e976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E97C;
    cpu.ir = 0x227C;
    let v1: u32 = 0x7147;
    cpu.a[1] = v1;
    4
}
/// $05E97C: move.w $10e1e6.l, d0
pub fn i_05e97c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E982;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x10E1E6) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
// ===== JsrAbsThunk_05e982  ($05E982, 8 B, thunks/jsr_abs_thunks.c)
/// $05E982: jsr $5d6c2.l
pub fn i_05e982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E988;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5E988);
    cpu.pc = 0x5D6C2;
    16
}
/// $05E988: rts
pub fn i_05e988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E98A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_MarkFlag2TimerMax_05e98a  ($05E98A, 14 B, props/late_props_turrets_05e000.s)
/// $05E98A: ori.b #$4, $12(a6)
pub fn i_05e98a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E990;
    cpu.ir = 0x2E;
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x4) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05E990: move.w #$ffff, $30(a6)
pub fn i_05e990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E996;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05E996: rts
pub fn i_05e996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E998;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Rng_Seed_05e998  ($05E998, 30 B, props/late_props_turrets_05e000.s)
/// $05E998: lea.l $10e230.l, a0
pub fn i_05e998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E99E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E230;
    20
}
/// $05E99E: moveq #$1f, d7
pub fn i_05e99e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9A0;
    cpu.ir = 0x7E1F;
    cpu.d[7] = 0x1F;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05E9A0: move.w d0, (a0)+
pub fn i_05e9a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9A2;
    cpu.ir = 0x30C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $05E9A2: mulu.w #$b7c7, d0
pub fn i_05e9a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9A6;
    cpu.ir = 0xC0FC;
    let v1: u32 = 0xB7C7;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[0] & 0xFFFF);
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $05E9A6: addi.w #$81f5, d0
pub fn i_05e9a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9AA;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x81F5, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05E9AA: dbra d7, $5e9a0
pub fn i_05e9aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9AE;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5E9AE; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5E9AE; return 14; }
    cpu.pc = 0x5E9A0;
    10
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
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
    (0x05CCD0, i_05ccd0),
    (0x05CCD4, i_05ccd4),
    (0x05CCD8, i_05ccd8),
    (0x05CCDE, i_05ccde),
    (0x05CCE2, i_05cce2),
    (0x05CCE6, i_05cce6),
    (0x05CCEA, i_05ccea),
    (0x05CCF0, i_05ccf0),
    (0x05CCF4, i_05ccf4),
    (0x05CCF8, i_05ccf8),
    (0x05CCFC, i_05ccfc),
    (0x05CD02, i_05cd02),
    (0x05CD06, i_05cd06),
    (0x05CD0A, i_05cd0a),
    (0x05CD0E, i_05cd0e),
    (0x05CD14, i_05cd14),
    (0x05CD18, i_05cd18),
    (0x05CD1C, i_05cd1c),
    (0x05CD20, i_05cd20),
    (0x05CD26, i_05cd26),
    (0x05CD2A, i_05cd2a),
    (0x05CD2E, i_05cd2e),
    (0x05CD32, i_05cd32),
    (0x05CD38, i_05cd38),
    (0x05CD3C, i_05cd3c),
    (0x05CD40, i_05cd40),
    (0x05CD44, i_05cd44),
    (0x05CD4A, i_05cd4a),
    (0x05CD4E, i_05cd4e),
    (0x05CD52, i_05cd52),
    (0x05CD56, i_05cd56),
    (0x05CD5C, i_05cd5c),
    (0x05CD60, i_05cd60),
    (0x05CD64, i_05cd64),
    (0x05CD68, i_05cd68),
    (0x05CD6C, i_05cd6c),
    (0x05CD70, i_05cd70),
    (0x05CD74, i_05cd74),
    (0x05CD78, i_05cd78),
    (0x05CD7C, i_05cd7c),
    (0x05CD80, i_05cd80),
    (0x05CD84, i_05cd84),
    (0x05CD88, i_05cd88),
    (0x05CD8C, i_05cd8c),
    (0x05CD90, i_05cd90),
    (0x05CD94, i_05cd94),
    (0x05CD98, i_05cd98),
    (0x05CD9C, i_05cd9c),
    (0x05CDA0, i_05cda0),
    (0x05CDA4, i_05cda4),
    (0x05CDA8, i_05cda8),
    (0x05CDAC, i_05cdac),
    (0x05CDB0, i_05cdb0),
    (0x05CDB4, i_05cdb4),
    (0x05CDB8, i_05cdb8),
    (0x05CDBC, i_05cdbc),
    (0x05CDC0, i_05cdc0),
    (0x05CDC4, i_05cdc4),
    (0x05CDC8, i_05cdc8),
    (0x05CDCC, i_05cdcc),
    (0x05CDD0, i_05cdd0),
    (0x05CDD4, i_05cdd4),
    (0x05CDD8, i_05cdd8),
    (0x05CDDC, i_05cddc),
    (0x05CDE0, i_05cde0),
    (0x05CDE4, i_05cde4),
    (0x05CDE8, i_05cde8),
    (0x05CDEC, i_05cdec),
    (0x05CDF0, i_05cdf0),
    (0x05CDF4, i_05cdf4),
    (0x05CDF8, i_05cdf8),
    (0x05CDFC, i_05cdfc),
    (0x05CE00, i_05ce00),
    (0x05CE04, i_05ce04),
    (0x05CE08, i_05ce08),
    (0x05CE0C, i_05ce0c),
    (0x05CE10, i_05ce10),
    (0x05CE14, i_05ce14),
    (0x05CE18, i_05ce18),
    (0x05CE1C, i_05ce1c),
    (0x05CE22, i_05ce22),
    (0x05CE26, i_05ce26),
    (0x05CE2A, i_05ce2a),
    (0x05CE2E, i_05ce2e),
    (0x05CE34, i_05ce34),
    (0x05CE38, i_05ce38),
    (0x05CE3C, i_05ce3c),
    (0x05CE40, i_05ce40),
    (0x05CE46, i_05ce46),
    (0x05CE4A, i_05ce4a),
    (0x05CE4E, i_05ce4e),
    (0x05CE52, i_05ce52),
    (0x05CE58, i_05ce58),
    (0x05CE5C, i_05ce5c),
    (0x05CE60, i_05ce60),
    (0x05CE64, i_05ce64),
    (0x05CE6A, i_05ce6a),
    (0x05CE6E, i_05ce6e),
    (0x05CE72, i_05ce72),
    (0x05CE76, i_05ce76),
    (0x05CE7C, i_05ce7c),
    (0x05CE80, i_05ce80),
    (0x05CE84, i_05ce84),
    (0x05CE88, i_05ce88),
    (0x05CE8E, i_05ce8e),
    (0x05CE92, i_05ce92),
    (0x05CE96, i_05ce96),
    (0x05CE9A, i_05ce9a),
    (0x05CEA0, i_05cea0),
    (0x05CEA4, i_05cea4),
    (0x05CEA8, i_05cea8),
    (0x05CEAC, i_05ceac),
    (0x05CEB2, i_05ceb2),
    (0x05CEB6, i_05ceb6),
    (0x05CEBA, i_05ceba),
    (0x05CEBE, i_05cebe),
    (0x05CEC4, i_05cec4),
    (0x05CEC8, i_05cec8),
    (0x05CECC, i_05cecc),
    (0x05CED0, i_05ced0),
    (0x05CED6, i_05ced6),
    (0x05CEDA, i_05ceda),
    (0x05CEDE, i_05cede),
    (0x05CEE2, i_05cee2),
    (0x05CEE8, i_05cee8),
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
    (0x05CF2C, i_05cf2c),
    (0x05CF30, i_05cf30),
    (0x05CF34, i_05cf34),
    (0x05CF38, i_05cf38),
    (0x05CF3C, i_05cf3c),
    (0x05CF40, i_05cf40),
    (0x05CF44, i_05cf44),
    (0x05CF48, i_05cf48),
    (0x05CF4C, i_05cf4c),
    (0x05CF50, i_05cf50),
    (0x05CF54, i_05cf54),
    (0x05CF58, i_05cf58),
    (0x05CF5C, i_05cf5c),
    (0x05CF60, i_05cf60),
    (0x05CF64, i_05cf64),
    (0x05CF68, i_05cf68),
    (0x05CF6C, i_05cf6c),
    (0x05CF70, i_05cf70),
    (0x05CF74, i_05cf74),
    (0x05CF78, i_05cf78),
    (0x05CF7C, i_05cf7c),
    (0x05CF80, i_05cf80),
    (0x05CF84, i_05cf84),
    (0x05CF88, i_05cf88),
    (0x05CF8C, i_05cf8c),
    (0x05CF90, i_05cf90),
    (0x05CF94, i_05cf94),
    (0x05CF98, i_05cf98),
    (0x05CF9C, i_05cf9c),
    (0x05CFA0, i_05cfa0),
    (0x05CFA4, i_05cfa4),
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
    (0x05D01A, i_05d01a),
    (0x05D01E, i_05d01e),
    (0x05D022, i_05d022),
    (0x05D026, i_05d026),
    (0x05D02A, i_05d02a),
    (0x05D02E, i_05d02e),
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
    (0x05D0BE, i_05d0be),
    (0x05D0C2, i_05d0c2),
    (0x05D0C6, i_05d0c6),
    (0x05D0CC, i_05d0cc),
    (0x05D0D0, i_05d0d0),
    (0x05D0D4, i_05d0d4),
    (0x05D0D8, i_05d0d8),
    (0x05D0DE, i_05d0de),
    (0x05D0E2, i_05d0e2),
    (0x05D0E6, i_05d0e6),
    (0x05D0E8, i_05d0e8),
    (0x05D0EC, i_05d0ec),
    (0x05D0F0, i_05d0f0),
    (0x05D0F2, i_05d0f2),
    (0x05D0F6, i_05d0f6),
    (0x05D0F8, i_05d0f8),
    (0x05D0FC, i_05d0fc),
    (0x05D100, i_05d100),
    (0x05D106, i_05d106),
    (0x05D10A, i_05d10a),
    (0x05D10E, i_05d10e),
    (0x05D112, i_05d112),
    (0x05D118, i_05d118),
    (0x05D11C, i_05d11c),
    (0x05D120, i_05d120),
    (0x05D124, i_05d124),
    (0x05D12A, i_05d12a),
    (0x05D12E, i_05d12e),
    (0x05D132, i_05d132),
    (0x05D136, i_05d136),
    (0x05D13C, i_05d13c),
    (0x05D140, i_05d140),
    (0x05D144, i_05d144),
    (0x05D148, i_05d148),
    (0x05D14C, i_05d14c),
    (0x05D150, i_05d150),
    (0x05D154, i_05d154),
    (0x05D158, i_05d158),
    (0x05D15C, i_05d15c),
    (0x05D160, i_05d160),
    (0x05D164, i_05d164),
    (0x05D168, i_05d168),
    (0x05D16C, i_05d16c),
    (0x05D170, i_05d170),
    (0x05D178, i_05d178),
    (0x05D17C, i_05d17c),
    (0x05D182, i_05d182),
    (0x05D186, i_05d186),
    (0x05D18A, i_05d18a),
    (0x05D18E, i_05d18e),
    (0x05D190, i_05d190),
    (0x05D194, i_05d194),
    (0x05D19A, i_05d19a),
    (0x05D19E, i_05d19e),
    (0x05D1A4, i_05d1a4),
    (0x05D1A8, i_05d1a8),
    (0x05D1AE, i_05d1ae),
    (0x05D1B2, i_05d1b2),
    (0x05D1B8, i_05d1b8),
    (0x05D1BC, i_05d1bc),
    (0x05D1C0, i_05d1c0),
    (0x05D1C4, i_05d1c4),
    (0x05D1C8, i_05d1c8),
    (0x05D1CA, i_05d1ca),
    (0x05D1CE, i_05d1ce),
    (0x05D1D2, i_05d1d2),
    (0x05D1D4, i_05d1d4),
    (0x05D1D8, i_05d1d8),
    (0x05D1DA, i_05d1da),
    (0x05D1DE, i_05d1de),
    (0x05D1E2, i_05d1e2),
    (0x05D1E6, i_05d1e6),
    (0x05D1EA, i_05d1ea),
    (0x05D1EE, i_05d1ee),
    (0x05D1F4, i_05d1f4),
    (0x05D1F8, i_05d1f8),
    (0x05D1FC, i_05d1fc),
    (0x05D1FE, i_05d1fe),
    (0x05D200, i_05d200),
    (0x05D204, i_05d204),
    (0x05D208, i_05d208),
    (0x05D20A, i_05d20a),
    (0x05D20E, i_05d20e),
    (0x05D210, i_05d210),
    (0x05D214, i_05d214),
    (0x05D216, i_05d216),
    (0x05D21A, i_05d21a),
    (0x05D21E, i_05d21e),
    (0x05D224, i_05d224),
    (0x05D228, i_05d228),
    (0x05D22C, i_05d22c),
    (0x05D22E, i_05d22e),
    (0x05D230, i_05d230),
    (0x05D234, i_05d234),
    (0x05D238, i_05d238),
    (0x05D23A, i_05d23a),
    (0x05D23E, i_05d23e),
    (0x05D240, i_05d240),
    (0x05D244, i_05d244),
    (0x05D248, i_05d248),
    (0x05D24C, i_05d24c),
    (0x05D250, i_05d250),
    (0x05D254, i_05d254),
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
    (0x05D2DE, i_05d2de),
    (0x05D2E4, i_05d2e4),
    (0x05D2E8, i_05d2e8),
    (0x05D2EA, i_05d2ea),
    (0x05D2EE, i_05d2ee),
    (0x05D2F2, i_05d2f2),
    (0x05D2F6, i_05d2f6),
    (0x05D2F8, i_05d2f8),
    (0x05D2FC, i_05d2fc),
    (0x05D300, i_05d300),
    (0x05D302, i_05d302),
    (0x05D306, i_05d306),
    (0x05D30A, i_05d30a),
    (0x05D30E, i_05d30e),
    (0x05D310, i_05d310),
    (0x05D314, i_05d314),
    (0x05D316, i_05d316),
    (0x05D586, i_05d586),
    (0x05D58A, i_05d58a),
    (0x05D58E, i_05d58e),
    (0x05D592, i_05d592),
    (0x05D596, i_05d596),
    (0x05D59A, i_05d59a),
    (0x05D59E, i_05d59e),
    (0x05D5A2, i_05d5a2),
    (0x05D5A6, i_05d5a6),
    (0x05D5AA, i_05d5aa),
    (0x05D5AE, i_05d5ae),
    (0x05D5B2, i_05d5b2),
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
    (0x05D616, i_05d616),
    (0x05D61A, i_05d61a),
    (0x05D61E, i_05d61e),
    (0x05D624, i_05d624),
    (0x05D628, i_05d628),
    (0x05D62C, i_05d62c),
    (0x05D632, i_05d632),
    (0x05D638, i_05d638),
    (0x05D63C, i_05d63c),
    (0x05D642, i_05d642),
    (0x05D646, i_05d646),
    (0x05D648, i_05d648),
    (0x05D650, i_05d650),
    (0x05D654, i_05d654),
    (0x05D65A, i_05d65a),
    (0x05D65E, i_05d65e),
    (0x05D662, i_05d662),
    (0x05D664, i_05d664),
    (0x05D666, i_05d666),
    (0x05D66A, i_05d66a),
    (0x05D66E, i_05d66e),
    (0x05D672, i_05d672),
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
    (0x05D6A0, i_05d6a0),
    (0x05D6A6, i_05d6a6),
    (0x05D6AA, i_05d6aa),
    (0x05D6AE, i_05d6ae),
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
    (0x05D71C, i_05d71c),
    (0x05D720, i_05d720),
    (0x05D724, i_05d724),
    (0x05D72A, i_05d72a),
    (0x05D7BE, i_05d7be),
    (0x05D7C0, i_05d7c0),
    (0x05D7C2, i_05d7c2),
    (0x05D7C6, i_05d7c6),
    (0x05D7CC, i_05d7cc),
    (0x05D7CE, i_05d7ce),
    (0x05D7D2, i_05d7d2),
    (0x05D7D4, i_05d7d4),
    (0x05D7D8, i_05d7d8),
    (0x05D7DA, i_05d7da),
    (0x05D7DC, i_05d7dc),
    (0x05D7E0, i_05d7e0),
    (0x05D7E6, i_05d7e6),
    (0x05D7E8, i_05d7e8),
    (0x05D7EC, i_05d7ec),
    (0x05D7EE, i_05d7ee),
    (0x05D7F0, i_05d7f0),
    (0x05D7F2, i_05d7f2),
    (0x05D7F4, i_05d7f4),
    (0x05D7FA, i_05d7fa),
    (0x05D7FC, i_05d7fc),
    (0x05D7FE, i_05d7fe),
    (0x05D800, i_05d800),
    (0x05D802, i_05d802),
    (0x05D804, i_05d804),
    (0x05D80A, i_05d80a),
    (0x05D80C, i_05d80c),
    (0x05D80E, i_05d80e),
    (0x05D810, i_05d810),
    (0x05D812, i_05d812),
    (0x05D818, i_05d818),
    (0x05D81C, i_05d81c),
    (0x05D81E, i_05d81e),
    (0x05D820, i_05d820),
    (0x05D824, i_05d824),
    (0x05D826, i_05d826),
    (0x05D82A, i_05d82a),
    (0x05D82C, i_05d82c),
    (0x05D830, i_05d830),
    (0x05D834, i_05d834),
    (0x05D838, i_05d838),
    (0x05D83C, i_05d83c),
    (0x05D83E, i_05d83e),
    (0x05D842, i_05d842),
    (0x05D844, i_05d844),
    (0x05D848, i_05d848),
    (0x05D84C, i_05d84c),
    (0x05D850, i_05d850),
    (0x05D856, i_05d856),
    (0x05D85A, i_05d85a),
    (0x05D85E, i_05d85e),
    (0x05D862, i_05d862),
    (0x05D886, i_05d886),
    (0x05D88A, i_05d88a),
    (0x05D88C, i_05d88c),
    (0x05D890, i_05d890),
    (0x05D896, i_05d896),
    (0x05D89A, i_05d89a),
    (0x05D89C, i_05d89c),
    (0x05D89E, i_05d89e),
    (0x05D8A2, i_05d8a2),
    (0x05D8A4, i_05d8a4),
    (0x05D8A6, i_05d8a6),
    (0x05D8AC, i_05d8ac),
    (0x05D8B0, i_05d8b0),
    (0x05D8B2, i_05d8b2),
    (0x05D8B4, i_05d8b4),
    (0x05D8B8, i_05d8b8),
    (0x05D8BA, i_05d8ba),
    (0x05D8BC, i_05d8bc),
    (0x05D8BE, i_05d8be),
    (0x05D8C4, i_05d8c4),
    (0x05D8C8, i_05d8c8),
    (0x05D8CA, i_05d8ca),
    (0x05D8CC, i_05d8cc),
    (0x05D8D0, i_05d8d0),
    (0x05D8D2, i_05d8d2),
    (0x05D8D4, i_05d8d4),
    (0x05D8D6, i_05d8d6),
    (0x05D8DC, i_05d8dc),
    (0x05D8DE, i_05d8de),
    (0x05D8E0, i_05d8e0),
    (0x05D8E4, i_05d8e4),
    (0x05D8E6, i_05d8e6),
    (0x05D8E8, i_05d8e8),
    (0x05D8EA, i_05d8ea),
    (0x05D8EC, i_05d8ec),
    (0x05D8F0, i_05d8f0),
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
    (0x05D946, i_05d946),
    (0x05D94C, i_05d94c),
    (0x05D950, i_05d950),
    (0x05D954, i_05d954),
    (0x05D956, i_05d956),
    (0x05D95A, i_05d95a),
    (0x05D95C, i_05d95c),
    (0x05D95E, i_05d95e),
    (0x05D964, i_05d964),
    (0x05D966, i_05d966),
    (0x05D96A, i_05d96a),
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
    (0x05DB1A, i_05db1a),
    (0x05DB1C, i_05db1c),
    (0x05DB1E, i_05db1e),
    (0x05DB22, i_05db22),
    (0x05DB26, i_05db26),
    (0x05DB28, i_05db28),
    (0x05DB2A, i_05db2a),
    (0x05DB2E, i_05db2e),
    (0x05DB32, i_05db32),
    (0x05DB36, i_05db36),
    (0x05DB3A, i_05db3a),
    (0x05DB3C, i_05db3c),
    (0x05DB40, i_05db40),
    (0x05DB44, i_05db44),
    (0x05DB48, i_05db48),
    (0x05DB4C, i_05db4c),
    (0x05DB50, i_05db50),
    (0x05DB52, i_05db52),
    (0x05DB56, i_05db56),
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
    (0x05DC00, i_05dc00),
    (0x05DC04, i_05dc04),
    (0x05DC08, i_05dc08),
    (0x05DC0C, i_05dc0c),
    (0x05DC10, i_05dc10),
    (0x05DC14, i_05dc14),
    (0x05DC16, i_05dc16),
    (0x05DC1A, i_05dc1a),
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
    (0x05DCB6, i_05dcb6),
    (0x05DCBA, i_05dcba),
    (0x05DCC0, i_05dcc0),
    (0x05DCC4, i_05dcc4),
    (0x05DCC8, i_05dcc8),
    (0x05DCCC, i_05dccc),
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
    (0x05DD22, i_05dd22),
    (0x05DD28, i_05dd28),
    (0x05DD2A, i_05dd2a),
    (0x05DD2E, i_05dd2e),
    (0x05DD34, i_05dd34),
    (0x05DD3A, i_05dd3a),
    (0x05DD40, i_05dd40),
    (0x05DD46, i_05dd46),
    (0x05DD4A, i_05dd4a),
    (0x05DD4C, i_05dd4c),
    (0x05DD50, i_05dd50),
    (0x05DD54, i_05dd54),
    (0x05DD56, i_05dd56),
    (0x05DD5C, i_05dd5c),
    (0x05DD62, i_05dd62),
    (0x05DD66, i_05dd66),
    (0x05DD6A, i_05dd6a),
    (0x05DD70, i_05dd70),
    (0x05DD74, i_05dd74),
    (0x05DD78, i_05dd78),
    (0x05DD7C, i_05dd7c),
    (0x05DD80, i_05dd80),
    (0x05DD84, i_05dd84),
    (0x05DD86, i_05dd86),
    (0x05DD8A, i_05dd8a),
    (0x05DD8E, i_05dd8e),
    (0x05DD92, i_05dd92),
    (0x05DD96, i_05dd96),
    (0x05DD9A, i_05dd9a),
    (0x05DD9E, i_05dd9e),
    (0x05DDA2, i_05dda2),
    (0x05DDA6, i_05dda6),
    (0x05DDAA, i_05ddaa),
    (0x05DDAE, i_05ddae),
    (0x05DDB2, i_05ddb2),
    (0x05DDB6, i_05ddb6),
    (0x05DDB8, i_05ddb8),
    (0x05DDBC, i_05ddbc),
    (0x05DDBE, i_05ddbe),
    (0x05DDC4, i_05ddc4),
    (0x05DDC8, i_05ddc8),
    (0x05DDCE, i_05ddce),
    (0x05DDD2, i_05ddd2),
    (0x05DDD8, i_05ddd8),
    (0x05DDDC, i_05dddc),
    (0x05DDE2, i_05dde2),
    (0x05DDE6, i_05dde6),
    (0x05DDEC, i_05ddec),
    (0x05DDF0, i_05ddf0),
    (0x05DDF2, i_05ddf2),
    (0x05DDF8, i_05ddf8),
    (0x05DDFA, i_05ddfa),
    (0x05DE00, i_05de00),
    (0x05DE02, i_05de02),
    (0x05DE08, i_05de08),
    (0x05DE0A, i_05de0a),
    (0x05DE10, i_05de10),
    (0x05DE12, i_05de12),
    (0x05DE16, i_05de16),
    (0x05E018, i_05e018),
    (0x05E01A, i_05e01a),
    (0x05E01C, i_05e01c),
    (0x05E01E, i_05e01e),
    (0x05E020, i_05e020),
    (0x05E022, i_05e022),
    (0x05E024, i_05e024),
    (0x05E028, i_05e028),
    (0x05E02A, i_05e02a),
    (0x05E02C, i_05e02c),
    (0x05E02E, i_05e02e),
    (0x05E030, i_05e030),
    (0x05E034, i_05e034),
    (0x05E036, i_05e036),
    (0x05E038, i_05e038),
    (0x05E03C, i_05e03c),
    (0x05E03E, i_05e03e),
    (0x05E042, i_05e042),
    (0x05E046, i_05e046),
    (0x05E04A, i_05e04a),
    (0x05E04E, i_05e04e),
    (0x05E052, i_05e052),
    (0x05E056, i_05e056),
    (0x05E05A, i_05e05a),
    (0x05E05C, i_05e05c),
    (0x05E05E, i_05e05e),
    (0x05E062, i_05e062),
    (0x05E064, i_05e064),
    (0x05E066, i_05e066),
    (0x05E068, i_05e068),
    (0x05E06C, i_05e06c),
    (0x05E06E, i_05e06e),
    (0x05E070, i_05e070),
    (0x05E074, i_05e074),
    (0x05E078, i_05e078),
    (0x05E07C, i_05e07c),
    (0x05E080, i_05e080),
    (0x05E084, i_05e084),
    (0x05E086, i_05e086),
    (0x05E088, i_05e088),
    (0x05E08C, i_05e08c),
    (0x05E090, i_05e090),
    (0x05E096, i_05e096),
    (0x05E09C, i_05e09c),
    (0x05E09E, i_05e09e),
    (0x05E0A2, i_05e0a2),
    (0x05E0A6, i_05e0a6),
    (0x05E0A8, i_05e0a8),
    (0x05E0AC, i_05e0ac),
    (0x05E0B0, i_05e0b0),
    (0x05E0B4, i_05e0b4),
    (0x05E0B8, i_05e0b8),
    (0x05E0BC, i_05e0bc),
    (0x05E0BE, i_05e0be),
    (0x05E0C2, i_05e0c2),
    (0x05E0C6, i_05e0c6),
    (0x05E0CA, i_05e0ca),
    (0x05E0CE, i_05e0ce),
    (0x05E0D0, i_05e0d0),
    (0x05E0D4, i_05e0d4),
    (0x05E0D8, i_05e0d8),
    (0x05E0DA, i_05e0da),
    (0x05E0DE, i_05e0de),
    (0x05E0E4, i_05e0e4),
    (0x05E0EA, i_05e0ea),
    (0x05E0EE, i_05e0ee),
    (0x05E0F2, i_05e0f2),
    (0x05E0F6, i_05e0f6),
    (0x05E0FA, i_05e0fa),
    (0x05E0FE, i_05e0fe),
    (0x05E102, i_05e102),
    (0x05E106, i_05e106),
    (0x05E108, i_05e108),
    (0x05E10C, i_05e10c),
    (0x05E10E, i_05e10e),
    (0x05E110, i_05e110),
    (0x05E114, i_05e114),
    (0x05E116, i_05e116),
    (0x05E118, i_05e118),
    (0x05E11C, i_05e11c),
    (0x05E120, i_05e120),
    (0x05E122, i_05e122),
    (0x05E124, i_05e124),
    (0x05E128, i_05e128),
    (0x05E12A, i_05e12a),
    (0x05E130, i_05e130),
    (0x05E134, i_05e134),
    (0x05E136, i_05e136),
    (0x05E13A, i_05e13a),
    (0x05E13E, i_05e13e),
    (0x05E142, i_05e142),
    (0x05E146, i_05e146),
    (0x05E14A, i_05e14a),
    (0x05E14E, i_05e14e),
    (0x05E152, i_05e152),
    (0x05E156, i_05e156),
    (0x05E15A, i_05e15a),
    (0x05E15E, i_05e15e),
    (0x05E160, i_05e160),
    (0x05E164, i_05e164),
    (0x05E166, i_05e166),
    (0x05E172, i_05e172),
    (0x05E176, i_05e176),
    (0x05E17A, i_05e17a),
    (0x05E17E, i_05e17e),
    (0x05E182, i_05e182),
    (0x05E186, i_05e186),
    (0x05E18A, i_05e18a),
    (0x05E18E, i_05e18e),
    (0x05E192, i_05e192),
    (0x05E196, i_05e196),
    (0x05E19A, i_05e19a),
    (0x05E19E, i_05e19e),
    (0x05E1A2, i_05e1a2),
    (0x05E1A4, i_05e1a4),
    (0x05E1A8, i_05e1a8),
    (0x05E1AA, i_05e1aa),
    (0x05E1AC, i_05e1ac),
    (0x05E1B0, i_05e1b0),
    (0x05E1B4, i_05e1b4),
    (0x05E1B8, i_05e1b8),
    (0x05E1BC, i_05e1bc),
    (0x05E1BE, i_05e1be),
    (0x05E1C0, i_05e1c0),
    (0x05E1C2, i_05e1c2),
    (0x05E1C6, i_05e1c6),
    (0x05E1CA, i_05e1ca),
    (0x05E1CE, i_05e1ce),
    (0x05E1D2, i_05e1d2),
    (0x05E1D4, i_05e1d4),
    (0x05E1D6, i_05e1d6),
    (0x05E1D8, i_05e1d8),
    (0x05E1DA, i_05e1da),
    (0x05E1DE, i_05e1de),
    (0x05E1E2, i_05e1e2),
    (0x05E1E4, i_05e1e4),
    (0x05E1E8, i_05e1e8),
    (0x05E1EA, i_05e1ea),
    (0x05E1EE, i_05e1ee),
    (0x05E1F2, i_05e1f2),
    (0x05E1F6, i_05e1f6),
    (0x05E1FC, i_05e1fc),
    (0x05E200, i_05e200),
    (0x05E202, i_05e202),
    (0x05E208, i_05e208),
    (0x05E20C, i_05e20c),
    (0x05E210, i_05e210),
    (0x05E216, i_05e216),
    (0x05E21A, i_05e21a),
    (0x05E21E, i_05e21e),
    (0x05E222, i_05e222),
    (0x05E224, i_05e224),
    (0x05E228, i_05e228),
    (0x05E22A, i_05e22a),
    (0x05E22E, i_05e22e),
    (0x05E232, i_05e232),
    (0x05E236, i_05e236),
    (0x05E23A, i_05e23a),
    (0x05E23C, i_05e23c),
    (0x05E23E, i_05e23e),
    (0x05E242, i_05e242),
    (0x05E246, i_05e246),
    (0x05E248, i_05e248),
    (0x05E24C, i_05e24c),
    (0x05E250, i_05e250),
    (0x05E252, i_05e252),
    (0x05E254, i_05e254),
    (0x05E258, i_05e258),
    (0x05E25A, i_05e25a),
    (0x05E25C, i_05e25c),
    (0x05E25E, i_05e25e),
    (0x05E260, i_05e260),
    (0x05E264, i_05e264),
    (0x05E268, i_05e268),
    (0x05E26E, i_05e26e),
    (0x05E272, i_05e272),
    (0x05E274, i_05e274),
    (0x05E278, i_05e278),
    (0x05E27C, i_05e27c),
    (0x05E280, i_05e280),
    (0x05E284, i_05e284),
    (0x05E286, i_05e286),
    (0x05E288, i_05e288),
    (0x05E28C, i_05e28c),
    (0x05E290, i_05e290),
    (0x05E294, i_05e294),
    (0x05E298, i_05e298),
    (0x05E29C, i_05e29c),
    (0x05E2A2, i_05e2a2),
    (0x05E2A6, i_05e2a6),
    (0x05E2AA, i_05e2aa),
    (0x05E2AE, i_05e2ae),
    (0x05E2B2, i_05e2b2),
    (0x05E2B6, i_05e2b6),
    (0x05E2BA, i_05e2ba),
    (0x05E2BC, i_05e2bc),
    (0x05E2C0, i_05e2c0),
    (0x05E2C4, i_05e2c4),
    (0x05E2C8, i_05e2c8),
    (0x05E2CC, i_05e2cc),
    (0x05E2D0, i_05e2d0),
    (0x05E2D2, i_05e2d2),
    (0x05E2D6, i_05e2d6),
    (0x05E2D8, i_05e2d8),
    (0x05E2DA, i_05e2da),
    (0x05E2DC, i_05e2dc),
    (0x05E2E2, i_05e2e2),
    (0x05E2E6, i_05e2e6),
    (0x05E2EC, i_05e2ec),
    (0x05E2F0, i_05e2f0),
    (0x05E2F6, i_05e2f6),
    (0x05E2FA, i_05e2fa),
    (0x05E302, i_05e302),
    (0x05E306, i_05e306),
    (0x05E308, i_05e308),
    (0x05E30C, i_05e30c),
    (0x05E312, i_05e312),
    (0x05E316, i_05e316),
    (0x05E318, i_05e318),
    (0x05E31C, i_05e31c),
    (0x05E320, i_05e320),
    (0x05E322, i_05e322),
    (0x05E326, i_05e326),
    (0x05E32C, i_05e32c),
    (0x05E330, i_05e330),
    (0x05E332, i_05e332),
    (0x05E336, i_05e336),
    (0x05E338, i_05e338),
    (0x05E33A, i_05e33a),
    (0x05E33C, i_05e33c),
    (0x05E342, i_05e342),
    (0x05E346, i_05e346),
    (0x05E348, i_05e348),
    (0x05E34E, i_05e34e),
    (0x05E352, i_05e352),
    (0x05E356, i_05e356),
    (0x05E35A, i_05e35a),
    (0x05E35E, i_05e35e),
    (0x05E360, i_05e360),
    (0x05E364, i_05e364),
    (0x05E366, i_05e366),
    (0x05E368, i_05e368),
    (0x05E36C, i_05e36c),
    (0x05E370, i_05e370),
    (0x05E376, i_05e376),
    (0x05E37A, i_05e37a),
    (0x05E37E, i_05e37e),
    (0x05E380, i_05e380),
    (0x05E384, i_05e384),
    (0x05E388, i_05e388),
    (0x05E38C, i_05e38c),
    (0x05E392, i_05e392),
    (0x05E396, i_05e396),
    (0x05E39A, i_05e39a),
    (0x05E39C, i_05e39c),
    (0x05E3A0, i_05e3a0),
    (0x05E3A2, i_05e3a2),
    (0x05E3A6, i_05e3a6),
    (0x05E3AA, i_05e3aa),
    (0x05E3AC, i_05e3ac),
    (0x05E3B0, i_05e3b0),
    (0x05E3B6, i_05e3b6),
    (0x05E3BA, i_05e3ba),
    (0x05E3C0, i_05e3c0),
    (0x05E3C4, i_05e3c4),
    (0x05E3CA, i_05e3ca),
    (0x05E3CE, i_05e3ce),
    (0x05E3D4, i_05e3d4),
    (0x05E3D8, i_05e3d8),
    (0x05E3DE, i_05e3de),
    (0x05E3E2, i_05e3e2),
    (0x05E3E6, i_05e3e6),
    (0x05E3E8, i_05e3e8),
    (0x05E3EE, i_05e3ee),
    (0x05E3F2, i_05e3f2),
    (0x05E3FC, i_05e3fc),
    (0x05E3FE, i_05e3fe),
    (0x05E402, i_05e402),
    (0x05E406, i_05e406),
    (0x05E408, i_05e408),
    (0x05E40C, i_05e40c),
    (0x05E410, i_05e410),
    (0x05E414, i_05e414),
    (0x05E418, i_05e418),
    (0x05E41A, i_05e41a),
    (0x05E41E, i_05e41e),
    (0x05E422, i_05e422),
    (0x05E424, i_05e424),
    (0x05E428, i_05e428),
    (0x05E42A, i_05e42a),
    (0x05E42E, i_05e42e),
    (0x05E432, i_05e432),
    (0x05E438, i_05e438),
    (0x05E43C, i_05e43c),
    (0x05E43E, i_05e43e),
    (0x05E442, i_05e442),
    (0x05E446, i_05e446),
    (0x05E44A, i_05e44a),
    (0x05E44C, i_05e44c),
    (0x05E450, i_05e450),
    (0x05E452, i_05e452),
    (0x05E456, i_05e456),
    (0x05E45A, i_05e45a),
    (0x05E45E, i_05e45e),
    (0x05E464, i_05e464),
    (0x05E468, i_05e468),
    (0x05E46E, i_05e46e),
    (0x05E472, i_05e472),
    (0x05E478, i_05e478),
    (0x05E47C, i_05e47c),
    (0x05E480, i_05e480),
    (0x05E482, i_05e482),
    (0x05E486, i_05e486),
    (0x05E488, i_05e488),
    (0x05E48E, i_05e48e),
    (0x05E492, i_05e492),
    (0x05E498, i_05e498),
    (0x05E49C, i_05e49c),
    (0x05E4A2, i_05e4a2),
    (0x05E4A6, i_05e4a6),
    (0x05E4AA, i_05e4aa),
    (0x05E4AC, i_05e4ac),
    (0x05E4B0, i_05e4b0),
    (0x05E4B2, i_05e4b2),
    (0x05E4B8, i_05e4b8),
    (0x05E4BC, i_05e4bc),
    (0x05E4C0, i_05e4c0),
    (0x05E4C4, i_05e4c4),
    (0x05E4C8, i_05e4c8),
    (0x05E4CA, i_05e4ca),
    (0x05E4CE, i_05e4ce),
    (0x05E4D2, i_05e4d2),
    (0x05E4D6, i_05e4d6),
    (0x05E4DA, i_05e4da),
    (0x05E4DC, i_05e4dc),
    (0x05E4E0, i_05e4e0),
    (0x05E4E6, i_05e4e6),
    (0x05E4EC, i_05e4ec),
    (0x05E4EE, i_05e4ee),
    (0x05E4F2, i_05e4f2),
    (0x05E4F8, i_05e4f8),
    (0x05E4FE, i_05e4fe),
    (0x05E504, i_05e504),
    (0x05E506, i_05e506),
    (0x05E50A, i_05e50a),
    (0x05E510, i_05e510),
    (0x05E516, i_05e516),
    (0x05E51C, i_05e51c),
    (0x05E522, i_05e522),
    (0x05E528, i_05e528),
    (0x05E52E, i_05e52e),
    (0x05E530, i_05e530),
    (0x05E536, i_05e536),
    (0x05E538, i_05e538),
    (0x05E53C, i_05e53c),
    (0x05E542, i_05e542),
    (0x05E54C, i_05e54c),
    (0x05E550, i_05e550),
    (0x05E55C, i_05e55c),
    (0x05E55E, i_05e55e),
    (0x05E560, i_05e560),
    (0x05E564, i_05e564),
    (0x05E568, i_05e568),
    (0x05E56E, i_05e56e),
    (0x05E572, i_05e572),
    (0x05E578, i_05e578),
    (0x05E57C, i_05e57c),
    (0x05E580, i_05e580),
    (0x05E584, i_05e584),
    (0x05E588, i_05e588),
    (0x05E58C, i_05e58c),
    (0x05E592, i_05e592),
    (0x05E596, i_05e596),
    (0x05E59C, i_05e59c),
    (0x05E5A0, i_05e5a0),
    (0x05E5A4, i_05e5a4),
    (0x05E5A6, i_05e5a6),
    (0x05E5A8, i_05e5a8),
    (0x05E5AA, i_05e5aa),
    (0x05E5AE, i_05e5ae),
    (0x05E5B2, i_05e5b2),
    (0x05E5B6, i_05e5b6),
    (0x05E5BA, i_05e5ba),
    (0x05E5BE, i_05e5be),
    (0x05E5C0, i_05e5c0),
    (0x05E5C4, i_05e5c4),
    (0x05E5C8, i_05e5c8),
    (0x05E5CC, i_05e5cc),
    (0x05E5D0, i_05e5d0),
    (0x05E5D4, i_05e5d4),
    (0x05E5D8, i_05e5d8),
    (0x05E5DA, i_05e5da),
    (0x05E5DE, i_05e5de),
    (0x05E5E0, i_05e5e0),
    (0x05E5E2, i_05e5e2),
    (0x05E5E6, i_05e5e6),
    (0x05E5EA, i_05e5ea),
    (0x05E5EE, i_05e5ee),
    (0x05E5F2, i_05e5f2),
    (0x05E5F6, i_05e5f6),
    (0x05E5F8, i_05e5f8),
    (0x05E5FC, i_05e5fc),
    (0x05E600, i_05e600),
    (0x05E604, i_05e604),
    (0x05E608, i_05e608),
    (0x05E60C, i_05e60c),
    (0x05E610, i_05e610),
    (0x05E612, i_05e612),
    (0x05E616, i_05e616),
    (0x05E618, i_05e618),
    (0x05E61C, i_05e61c),
    (0x05E620, i_05e620),
    (0x05E626, i_05e626),
    (0x05E62A, i_05e62a),
    (0x05E62E, i_05e62e),
    (0x05E632, i_05e632),
    (0x05E636, i_05e636),
    (0x05E638, i_05e638),
    (0x05E63C, i_05e63c),
    (0x05E640, i_05e640),
    (0x05E644, i_05e644),
    (0x05E646, i_05e646),
    (0x05E64A, i_05e64a),
    (0x05E64C, i_05e64c),
    (0x05E650, i_05e650),
    (0x05E656, i_05e656),
    (0x05E65C, i_05e65c),
    (0x05E660, i_05e660),
    (0x05E666, i_05e666),
    (0x05E66A, i_05e66a),
    (0x05E66C, i_05e66c),
    (0x05E670, i_05e670),
    (0x05E676, i_05e676),
    (0x05E67A, i_05e67a),
    (0x05E680, i_05e680),
    (0x05E684, i_05e684),
    (0x05E68A, i_05e68a),
    (0x05E68E, i_05e68e),
    (0x05E690, i_05e690),
    (0x05E6A4, i_05e6a4),
    (0x05E6A8, i_05e6a8),
    (0x05E6AC, i_05e6ac),
    (0x05E6B0, i_05e6b0),
    (0x05E6B4, i_05e6b4),
    (0x05E6B8, i_05e6b8),
    (0x05E6BC, i_05e6bc),
    (0x05E6C0, i_05e6c0),
    (0x05E6C2, i_05e6c2),
    (0x05E6C6, i_05e6c6),
    (0x05E6CC, i_05e6cc),
    (0x05E6CE, i_05e6ce),
    (0x05E6D2, i_05e6d2),
    (0x05E6D6, i_05e6d6),
    (0x05E6DA, i_05e6da),
    (0x05E6DE, i_05e6de),
    (0x05E6E2, i_05e6e2),
    (0x05E6E4, i_05e6e4),
    (0x05E6E6, i_05e6e6),
    (0x05E6EA, i_05e6ea),
    (0x05E6EC, i_05e6ec),
    (0x05E6EE, i_05e6ee),
    (0x05E6F2, i_05e6f2),
    (0x05E6F6, i_05e6f6),
    (0x05E6FA, i_05e6fa),
    (0x05E6FE, i_05e6fe),
    (0x05E700, i_05e700),
    (0x05E702, i_05e702),
    (0x05E706, i_05e706),
    (0x05E708, i_05e708),
    (0x05E70A, i_05e70a),
    (0x05E70E, i_05e70e),
    (0x05E710, i_05e710),
    (0x05E712, i_05e712),
    (0x05E714, i_05e714),
    (0x05E718, i_05e718),
    (0x05E71A, i_05e71a),
    (0x05E720, i_05e720),
    (0x05E722, i_05e722),
    (0x05E726, i_05e726),
    (0x05E728, i_05e728),
    (0x05E72E, i_05e72e),
    (0x05E732, i_05e732),
    (0x05E738, i_05e738),
    (0x05E73C, i_05e73c),
    (0x05E740, i_05e740),
    (0x05E744, i_05e744),
    (0x05E74A, i_05e74a),
    (0x05E74C, i_05e74c),
    (0x05E752, i_05e752),
    (0x05E756, i_05e756),
    (0x05E75A, i_05e75a),
    (0x05E75E, i_05e75e),
    (0x05E764, i_05e764),
    (0x05E770, i_05e770),
    (0x05E776, i_05e776),
    (0x05E77A, i_05e77a),
    (0x05E77E, i_05e77e),
    (0x05E782, i_05e782),
    (0x05E784, i_05e784),
    (0x05E788, i_05e788),
    (0x05E78C, i_05e78c),
    (0x05E78E, i_05e78e),
    (0x05E792, i_05e792),
    (0x05E796, i_05e796),
    (0x05E798, i_05e798),
    (0x05E79E, i_05e79e),
    (0x05E7A2, i_05e7a2),
    (0x05E7A6, i_05e7a6),
    (0x05E7AA, i_05e7aa),
    (0x05E7AC, i_05e7ac),
    (0x05E7B0, i_05e7b0),
    (0x05E7B4, i_05e7b4),
    (0x05E7B6, i_05e7b6),
    (0x05E7BA, i_05e7ba),
    (0x05E7BE, i_05e7be),
    (0x05E7C0, i_05e7c0),
    (0x05E7C6, i_05e7c6),
    (0x05E7CC, i_05e7cc),
    (0x05E7D0, i_05e7d0),
    (0x05E7D6, i_05e7d6),
    (0x05E7D8, i_05e7d8),
    (0x05E7DC, i_05e7dc),
    (0x05E7DE, i_05e7de),
    (0x05E7E4, i_05e7e4),
    (0x05E7EA, i_05e7ea),
    (0x05E7EE, i_05e7ee),
    (0x05E7F0, i_05e7f0),
    (0x05E7F6, i_05e7f6),
    (0x05E7FC, i_05e7fc),
    (0x05E802, i_05e802),
    (0x05E804, i_05e804),
    (0x05E80A, i_05e80a),
    (0x05E810, i_05e810),
    (0x05E816, i_05e816),
    (0x05E818, i_05e818),
    (0x05E81A, i_05e81a),
    (0x05E81C, i_05e81c),
    (0x05E81E, i_05e81e),
    (0x05E822, i_05e822),
    (0x05E826, i_05e826),
    (0x05E828, i_05e828),
    (0x05E82E, i_05e82e),
    (0x05E832, i_05e832),
    (0x05E834, i_05e834),
    (0x05E838, i_05e838),
    (0x05E83C, i_05e83c),
    (0x05E842, i_05e842),
    (0x05E844, i_05e844),
    (0x05E84A, i_05e84a),
    (0x05E84E, i_05e84e),
    (0x05E854, i_05e854),
    (0x05E858, i_05e858),
    (0x05E85E, i_05e85e),
    (0x05E862, i_05e862),
    (0x05E868, i_05e868),
    (0x05E86C, i_05e86c),
    (0x05E872, i_05e872),
    (0x05E876, i_05e876),
    (0x05E87C, i_05e87c),
    (0x05E880, i_05e880),
    (0x05E886, i_05e886),
    (0x05E88A, i_05e88a),
    (0x05E88E, i_05e88e),
    (0x05E890, i_05e890),
    (0x05E894, i_05e894),
    (0x05E896, i_05e896),
    (0x05E89C, i_05e89c),
    (0x05E8A2, i_05e8a2),
    (0x05E8A6, i_05e8a6),
    (0x05E8AC, i_05e8ac),
    (0x05E8B2, i_05e8b2),
    (0x05E8B6, i_05e8b6),
    (0x05E8B8, i_05e8b8),
    (0x05E8BA, i_05e8ba),
    (0x05E8C0, i_05e8c0),
    (0x05E8C4, i_05e8c4),
    (0x05E8CA, i_05e8ca),
    (0x05E8CE, i_05e8ce),
    (0x05E8D2, i_05e8d2),
    (0x05E8D4, i_05e8d4),
    (0x05E8D8, i_05e8d8),
    (0x05E8DA, i_05e8da),
    (0x05E8E0, i_05e8e0),
    (0x05E8E6, i_05e8e6),
    (0x05E8EA, i_05e8ea),
    (0x05E8EE, i_05e8ee),
    (0x05E8F4, i_05e8f4),
    (0x05E8F6, i_05e8f6),
    (0x05E8FA, i_05e8fa),
    (0x05E8FE, i_05e8fe),
    (0x05E902, i_05e902),
    (0x05E906, i_05e906),
    (0x05E90A, i_05e90a),
    (0x05E90C, i_05e90c),
    (0x05E910, i_05e910),
    (0x05E912, i_05e912),
    (0x05E918, i_05e918),
    (0x05E91A, i_05e91a),
    (0x05E91E, i_05e91e),
    (0x05E924, i_05e924),
    (0x05E928, i_05e928),
    (0x05E92A, i_05e92a),
    (0x05E92E, i_05e92e),
    (0x05E932, i_05e932),
    (0x05E934, i_05e934),
    (0x05E938, i_05e938),
    (0x05E93C, i_05e93c),
    (0x05E940, i_05e940),
    (0x05E946, i_05e946),
    (0x05E94A, i_05e94a),
    (0x05E94E, i_05e94e),
    (0x05E952, i_05e952),
    (0x05E956, i_05e956),
    (0x05E95C, i_05e95c),
    (0x05E960, i_05e960),
    (0x05E964, i_05e964),
    (0x05E96A, i_05e96a),
    (0x05E970, i_05e970),
    (0x05E976, i_05e976),
    (0x05E97C, i_05e97c),
    (0x05E982, i_05e982),
    (0x05E988, i_05e988),
    (0x05E98A, i_05e98a),
    (0x05E990, i_05e990),
    (0x05E996, i_05e996),
    (0x05E998, i_05e998),
    (0x05E99E, i_05e99e),
    (0x05E9A0, i_05e9a0),
    (0x05E9A2, i_05e9a2),
    (0x05E9A6, i_05e9a6),
    (0x05E9AA, i_05e9aa),
];
