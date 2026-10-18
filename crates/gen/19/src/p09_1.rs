// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p09_1) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $092962: lea.l $92968(pc), a1
pub fn i_092962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92966;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x92968;
    16
}
/// $092966: rts
pub fn i_092966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92968;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $092994: cmpi.w #$a50, $106f50.l
pub fn i_092994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9299C;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0xA50, Size::Word, false);
    20
}
/// $09299C: scs.b d0
pub fn i_09299c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9299E;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $09299E: lea.l $929a4(pc), a1
pub fn i_09299e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x929A2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x929A4;
    16
}
/// $0929A2: rts
pub fn i_0929a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x929A4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0929A6: cmpi.b #$1, $10e39a.l
pub fn i_0929a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x929AE;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10E39A) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $0929AE: sne.b d0
pub fn i_0929ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x929B0;
    cpu.ir = 0x56C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Ne) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $0929B0: lea.l $929b6(pc), a1
pub fn i_0929b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x929B4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x929B6;
    16
}
/// $0929B4: rts
pub fn i_0929b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x929B6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0929CA: cmpi.w #$a80, $106f50.l
pub fn i_0929ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x929D2;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0xA80, Size::Word, false);
    20
}
/// $0929D2: scs.b d0
pub fn i_0929d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x929D4;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $0929D4: lea.l $929da(pc), a1
pub fn i_0929d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x929D8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x929DA;
    16
}
/// $0929D8: rts
pub fn i_0929d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x929DA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $092A06: cmpi.w #$b10, $106f50.l
pub fn i_092a06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92A0E;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0xB10, Size::Word, false);
    20
}
/// $092A0E: scs.b d0
pub fn i_092a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92A10;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $092A10: lea.l $92a16(pc), a1
pub fn i_092a10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92A14;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x92A16;
    16
}
/// $092A14: rts
pub fn i_092a14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92A16;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $092A18: cmpi.b #$1, $10e39a.l
pub fn i_092a18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92A20;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10E39A) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $092A20: sne.b d0
pub fn i_092a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92A22;
    cpu.ir = 0x56C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Ne) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $092A22: lea.l $92a28(pc), a1
pub fn i_092a22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92A26;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x92A28;
    16
}
/// $092A26: rts
pub fn i_092a26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92A28;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $092A36: cmpi.w #$b70, $106f50.l
pub fn i_092a36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92A3E;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0xB70, Size::Word, false);
    20
}
/// $092A3E: scs.b d0
pub fn i_092a3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92A40;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $092A40: lea.l $92a46(pc), a1
pub fn i_092a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92A44;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x92A46;
    16
}
/// $092A44: rts
pub fn i_092a44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92A46;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $092A48: cmpi.b #$1, $10e39a.l
pub fn i_092a48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92A50;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10E39A) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $092A50: sne.b d0
pub fn i_092a50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92A52;
    cpu.ir = 0x56C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Ne) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $092A52: lea.l $92a58(pc), a1
pub fn i_092a52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92A56;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x92A58;
    16
}
/// $092A56: rts
pub fn i_092a56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92A58;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $092A66: cmpi.w #$c58, $106f50.l
pub fn i_092a66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92A6E;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0xC58, Size::Word, false);
    20
}
/// $092A6E: scs.b d0
pub fn i_092a6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92A70;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $092A70: lea.l $92a76(pc), a1
pub fn i_092a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92A74;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x92A76;
    16
}
/// $092A74: rts
pub fn i_092a74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92A76;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $092A78: cmpi.b #$1, $10e39a.l
pub fn i_092a78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92A80;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10E39A) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $092A80: sne.b d0
pub fn i_092a80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92A82;
    cpu.ir = 0x56C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Ne) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $092A82: lea.l $92a88(pc), a1
pub fn i_092a82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92A86;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x92A88;
    16
}
/// $092A86: rts
pub fn i_092a86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92A88;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $092A96: cmpi.w #$cd8, $106f50.l
pub fn i_092a96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92A9E;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0xCD8, Size::Word, false);
    20
}
/// $092A9E: scs.b d0
pub fn i_092a9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92AA0;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $092AA0: lea.l $92aa6(pc), a1
pub fn i_092aa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92AA4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x92AA6;
    16
}
/// $092AA4: rts
pub fn i_092aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92AA6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $092AA8: cmpi.b #$1, $10e39a.l
pub fn i_092aa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92AB0;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10E39A) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $092AB0: sne.b d0
pub fn i_092ab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92AB2;
    cpu.ir = 0x56C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Ne) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $092AB2: lea.l $92ab8(pc), a1
pub fn i_092ab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92AB6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x92AB8;
    16
}
/// $092AB6: rts
pub fn i_092ab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92AB8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $092DF0: cmpi.w #$368, $106f54.l
pub fn i_092df0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92DF8;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F54) as u32;
    sub_with_flags(cpu, v2, 0x368, Size::Word, false);
    20
}
/// $092DF8: shi.b d0
pub fn i_092df8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92DFA;
    cpu.ir = 0x52C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Hi) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $092DFA: lea.l $92e00(pc), a1
pub fn i_092dfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92DFE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x92E00;
    16
}
/// $092DFE: rts
pub fn i_092dfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92E00;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $092E08: cmpi.w #$340, $106f54.l
pub fn i_092e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92E10;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F54) as u32;
    sub_with_flags(cpu, v2, 0x340, Size::Word, false);
    20
}
/// $092E10: shi.b d0
pub fn i_092e10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92E12;
    cpu.ir = 0x52C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Hi) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $092E12: lea.l $92e18(pc), a1
pub fn i_092e12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92E16;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x92E18;
    16
}
/// $092E16: rts
pub fn i_092e16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92E18;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $092E20: cmpi.w #$300, $106f54.l
pub fn i_092e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92E28;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F54) as u32;
    sub_with_flags(cpu, v2, 0x300, Size::Word, false);
    20
}
/// $092E28: shi.b d0
pub fn i_092e28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92E2A;
    cpu.ir = 0x52C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Hi) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $092E2A: lea.l $92e30(pc), a1
pub fn i_092e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92E2E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x92E30;
    16
}
/// $092E2E: rts
pub fn i_092e2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92E30;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $092E32: cmpi.w #$270, $106f54.l
pub fn i_092e32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92E3A;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F54) as u32;
    sub_with_flags(cpu, v2, 0x270, Size::Word, false);
    20
}
/// $092E3A: shi.b d0
pub fn i_092e3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92E3C;
    cpu.ir = 0x52C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Hi) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $092E3C: lea.l $92e42(pc), a1
pub fn i_092e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92E40;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x92E42;
    16
}
/// $092E40: rts
pub fn i_092e40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92E42;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $092E56: cmpi.w #$230, $106f54.l
pub fn i_092e56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92E5E;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F54) as u32;
    sub_with_flags(cpu, v2, 0x230, Size::Word, false);
    20
}
/// $092E5E: shi.b d0
pub fn i_092e5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92E60;
    cpu.ir = 0x52C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Hi) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $092E60: lea.l $92e66(pc), a1
pub fn i_092e60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92E64;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x92E66;
    16
}
/// $092E64: rts
pub fn i_092e64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92E66;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $092E6E: cmpi.b #$1, $10e39a.l
pub fn i_092e6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92E76;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10E39A) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $092E76: sne.b d0
pub fn i_092e76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92E78;
    cpu.ir = 0x56C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Ne) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $092E78: lea.l $92e7e(pc), a1
pub fn i_092e78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92E7C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x92E7E;
    16
}
/// $092E7C: rts
pub fn i_092e7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92E7E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $092E80: cmpi.w #$60, $106f54.l
pub fn i_092e80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92E88;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F54) as u32;
    sub_with_flags(cpu, v2, 0x60, Size::Word, false);
    20
}
/// $092E88: shi.b d0
pub fn i_092e88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92E8A;
    cpu.ir = 0x52C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Hi) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $092E8A: lea.l $92e90(pc), a1
pub fn i_092e8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92E8E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x92E90;
    16
}
/// $092E8E: rts
pub fn i_092e8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92E90;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $092E98: cmpi.w #$10, $106f54.l
pub fn i_092e98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92EA0;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F54) as u32;
    sub_with_flags(cpu, v2, 0x10, Size::Word, false);
    20
}
/// $092EA0: shi.b d0
pub fn i_092ea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92EA2;
    cpu.ir = 0x52C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Hi) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $092EA2: lea.l $92ea8(pc), a1
pub fn i_092ea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92EA6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x92EA8;
    16
}
/// $092EA6: rts
pub fn i_092ea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92EA8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $092F00: cmpi.w #$20, $106f50.l
pub fn i_092f00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92F08;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x20, Size::Word, false);
    20
}
/// $092F08: scs.b d0
pub fn i_092f08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92F0A;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $092F0A: lea.l $92f10(pc), a1
pub fn i_092f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92F0E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x92F10;
    16
}
/// $092F0E: rts
pub fn i_092f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92F10;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $092F44: cmpi.w #$150, $106f50.l
pub fn i_092f44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92F4C;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x150, Size::Word, false);
    20
}
/// $092F4C: scs.b d0
pub fn i_092f4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92F4E;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $092F4E: lea.l $92f54(pc), a1
pub fn i_092f4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92F52;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x92F54;
    16
}
/// $092F52: rts
pub fn i_092f52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92F54;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $092F88: cmpi.w #$180, $106f50.l
pub fn i_092f88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92F90;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x180, Size::Word, false);
    20
}
/// $092F90: scs.b d0
pub fn i_092f90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92F92;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $092F92: lea.l $92f98(pc), a1
pub fn i_092f92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92F96;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x92F98;
    16
}
/// $092F96: rts
pub fn i_092f96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92F98;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $092F9A: cmpi.b #$1, $10e39a.l
pub fn i_092f9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92FA2;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10E39A) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $092FA2: sne.b d0
pub fn i_092fa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92FA4;
    cpu.ir = 0x56C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Ne) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $092FA4: lea.l $92faa(pc), a1
pub fn i_092fa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92FA8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x92FAA;
    16
}
/// $092FA8: rts
pub fn i_092fa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92FAA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $092FB8: cmpi.w #$220, $106f50.l
pub fn i_092fb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92FC0;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x220, Size::Word, false);
    20
}
/// $092FC0: scs.b d0
pub fn i_092fc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92FC2;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $092FC2: lea.l $92fc8(pc), a1
pub fn i_092fc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92FC6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x92FC8;
    16
}
/// $092FC6: rts
pub fn i_092fc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x92FC8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $092FFA: cmpi.w #$350, $106f50.l
pub fn i_092ffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93002;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x350, Size::Word, false);
    20
}
/// $093002: scs.b d0
pub fn i_093002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93004;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $093004: lea.l $9300a(pc), a1
pub fn i_093004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93008;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9300A;
    16
}
/// $093008: rts
pub fn i_093008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9300A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $093012: cmpi.w #$3e0, $106f50.l
pub fn i_093012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9301A;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x3E0, Size::Word, false);
    20
}
/// $09301A: scs.b d0
pub fn i_09301a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9301C;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $09301C: lea.l $93022(pc), a1
pub fn i_09301c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93020;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x93022;
    16
}
/// $093020: rts
pub fn i_093020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93022;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $09302A: cmpi.w #$4a0, $106f50.l
pub fn i_09302a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93032;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x4A0, Size::Word, false);
    20
}
/// $093032: scs.b d0
pub fn i_093032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93034;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $093034: lea.l $9303a(pc), a1
pub fn i_093034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93038;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9303A;
    16
}
/// $093038: rts
pub fn i_093038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9303A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $093042: cmpi.w #$4b0, $106f50.l
pub fn i_093042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9304A;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x4B0, Size::Word, false);
    20
}
/// $09304A: scs.b d0
pub fn i_09304a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9304C;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $09304C: lea.l $93052(pc), a1
pub fn i_09304c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93050;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x93052;
    16
}
/// $093050: rts
pub fn i_093050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93052;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $09305A: cmpi.w #$4f0, $106f50.l
pub fn i_09305a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93062;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x4F0, Size::Word, false);
    20
}
/// $093062: scs.b d0
pub fn i_093062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93064;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $093064: lea.l $9306a(pc), a1
pub fn i_093064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93068;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9306A;
    16
}
/// $093068: rts
pub fn i_093068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9306A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0930B6: cmpi.b #$2, $10e39a.l
pub fn i_0930b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x930BE;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10E39A) as u32;
    sub_with_flags(cpu, v2, 0x2, Size::Byte, false);
    20
}
/// $0930BE: sne.b d0
pub fn i_0930be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x930C0;
    cpu.ir = 0x56C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Ne) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $0930C0: lea.l $930c6(pc), a1
pub fn i_0930c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x930C4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x930C6;
    16
}
/// $0930C4: rts
pub fn i_0930c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x930C6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0930FA: cmpi.b #$1, $10e39a.l
pub fn i_0930fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93102;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10E39A) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $093102: sne.b d0
pub fn i_093102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93104;
    cpu.ir = 0x56C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Ne) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $093104: lea.l $9310a(pc), a1
pub fn i_093104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93108;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9310A;
    16
}
/// $093108: rts
pub fn i_093108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9310A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $093118: cmpi.w #$6c0, $106f50.l
pub fn i_093118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93120;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x6C0, Size::Word, false);
    20
}
/// $093120: scs.b d0
pub fn i_093120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93122;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $093122: lea.l $93128(pc), a1
pub fn i_093122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93126;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x93128;
    16
}
/// $093126: rts
pub fn i_093126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93128;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $093130: cmpi.w #$7d0, $106f50.l
pub fn i_093130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93138;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x7D0, Size::Word, false);
    20
}
/// $093138: scs.b d0
pub fn i_093138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9313A;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $09313A: lea.l $93140(pc), a1
pub fn i_09313a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9313E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x93140;
    16
}
/// $09313E: rts
pub fn i_09313e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93140;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $093148: cmpi.w #$8e8, $106f50.l
pub fn i_093148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93150;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x8E8, Size::Word, false);
    20
}
/// $093150: scs.b d0
pub fn i_093150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93152;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $093152: lea.l $93158(pc), a1
pub fn i_093152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93156;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x93158;
    16
}
/// $093156: rts
pub fn i_093156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93158;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $09315A: cmpi.w #$0, $106e88.l
pub fn i_09315a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93162;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106E88) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Word, false);
    20
}
/// $093162: sne.b d0
pub fn i_093162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93164;
    cpu.ir = 0x56C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Ne) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $093164: lea.l $9316a(pc), a1
pub fn i_093164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93168;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9316A;
    16
}
/// $093168: rts
pub fn i_093168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9316A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $09317E: cmpi.w #$a48, $106f50.l
pub fn i_09317e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93186;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0xA48, Size::Word, false);
    20
}
/// $093186: scs.b d0
pub fn i_093186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93188;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $093188: lea.l $9318e(pc), a1
pub fn i_093188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9318C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9318E;
    16
}
/// $09318C: rts
pub fn i_09318c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9318E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $093190: cmpi.b #$1, $10e39a.l
pub fn i_093190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93198;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10E39A) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $093198: sne.b d0
pub fn i_093198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9319A;
    cpu.ir = 0x56C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Ne) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $09319A: lea.l $931a0(pc), a1
pub fn i_09319a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9319E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x931A0;
    16
}
/// $09319E: rts
pub fn i_09319e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x931A0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0934E2: cmpi.w #$b20, $106f50.l
pub fn i_0934e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x934EA;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0xB20, Size::Word, false);
    20
}
/// $0934EA: scs.b d0
pub fn i_0934ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x934EC;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $0934EC: lea.l $934f2(pc), a1
pub fn i_0934ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x934F0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x934F2;
    16
}
/// $0934F0: rts
pub fn i_0934f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x934F2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0934F4: cmpi.b #$1, $10e39a.l
pub fn i_0934f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x934FC;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10E39A) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $0934FC: sne.b d0
pub fn i_0934fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x934FE;
    cpu.ir = 0x56C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Ne) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $0934FE: lea.l $93504(pc), a1
pub fn i_0934fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93502;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x93504;
    16
}
/// $093502: rts
pub fn i_093502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93504;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $093512: cmpi.w #$ca0, $106f50.l
pub fn i_093512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9351A;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0xCA0, Size::Word, false);
    20
}
/// $09351A: scs.b d0
pub fn i_09351a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9351C;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $09351C: lea.l $93522(pc), a1
pub fn i_09351c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93520;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x93522;
    16
}
/// $093520: rts
pub fn i_093520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93522;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $093524: cmpi.b #$1, $10e39a.l
pub fn i_093524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9352C;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10E39A) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $09352C: sne.b d0
pub fn i_09352c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9352E;
    cpu.ir = 0x56C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Ne) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $09352E: lea.l $93534(pc), a1
pub fn i_09352e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93532;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x93534;
    16
}
/// $093532: rts
pub fn i_093532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93534;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $093798: cmpi.w #$b0, $106f50.l
pub fn i_093798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x937A0;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0xB0, Size::Word, false);
    20
}
/// $0937A0: scs.b d0
pub fn i_0937a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x937A2;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $0937A2: lea.l $937a8(pc), a1
pub fn i_0937a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x937A6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x937A8;
    16
}
/// $0937A6: rts
pub fn i_0937a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x937A8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0937B0: cmpi.w #$100, $106f50.l
pub fn i_0937b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x937B8;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x100, Size::Word, false);
    20
}
/// $0937B8: scs.b d0
pub fn i_0937b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x937BA;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $0937BA: lea.l $937c0(pc), a1
pub fn i_0937ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x937BE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x937C0;
    16
}
/// $0937BE: rts
pub fn i_0937be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x937C0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0937C8: cmpi.w #$100, $106f50.l
pub fn i_0937c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x937D0;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x100, Size::Word, false);
    20
}
/// $0937D0: scs.b d0
pub fn i_0937d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x937D2;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $0937D2: lea.l $937d8(pc), a1
pub fn i_0937d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x937D6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x937D8;
    16
}
/// $0937D6: rts
pub fn i_0937d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x937D8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0937EE: cmpi.w #$140, $106f50.l
pub fn i_0937ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x937F6;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x140, Size::Word, false);
    20
}
/// $0937F6: scs.b d0
pub fn i_0937f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x937F8;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $0937F8: lea.l $937fe(pc), a1
pub fn i_0937f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x937FC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x937FE;
    16
}
/// $0937FC: rts
pub fn i_0937fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x937FE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $093830: cmpi.w #$300, $106f50.l
pub fn i_093830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93838;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x300, Size::Word, false);
    20
}
/// $093838: scs.b d0
pub fn i_093838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9383A;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $09383A: lea.l $93840(pc), a1
pub fn i_09383a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9383E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x93840;
    16
}
/// $09383E: rts
pub fn i_09383e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93840;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $09384E: cmpi.w #$508, $106f50.l
pub fn i_09384e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93856;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x508, Size::Word, false);
    20
}
/// $093856: scs.b d0
pub fn i_093856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93858;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $093858: lea.l $9385e(pc), a1
pub fn i_093858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9385C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9385E;
    16
}
/// $09385C: rts
pub fn i_09385c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9385E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $093866: cmpi.w #$558, $106f50.l
pub fn i_093866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9386E;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x558, Size::Word, false);
    20
}
/// $09386E: scs.b d0
pub fn i_09386e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93870;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $093870: lea.l $93876(pc), a1
pub fn i_093870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93874;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x93876;
    16
}
/// $093874: rts
pub fn i_093874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93876;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $09387E: cmpi.w #$640, $106f50.l
pub fn i_09387e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93886;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x640, Size::Word, false);
    20
}
/// $093886: scs.b d0
pub fn i_093886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93888;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $093888: lea.l $9388e(pc), a1
pub fn i_093888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9388C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9388E;
    16
}
/// $09388C: rts
pub fn i_09388c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9388E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $093896: cmpi.w #$670, $106f50.l
pub fn i_093896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9389E;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x670, Size::Word, false);
    20
}
/// $09389E: scs.b d0
pub fn i_09389e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x938A0;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $0938A0: lea.l $938a6(pc), a1
pub fn i_0938a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x938A4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x938A6;
    16
}
/// $0938A4: rts
pub fn i_0938a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x938A6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0938AE: move.b #$1, $10e39c.l
pub fn i_0938ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x938B6;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E39C, v1 as u8);
    16
}
/// $0938B6: lea.l $938bc(pc), a0
pub fn i_0938b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x938BA;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x938BC;
    16
}
/// $0938BA: rts
pub fn i_0938ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x938BC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0938BE: cmpi.b #$1, $10e39a.l
pub fn i_0938be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x938C6;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10E39A) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $0938C6: sne.b d0
pub fn i_0938c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x938C8;
    cpu.ir = 0x56C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Ne) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $0938C8: lea.l $938ce(pc), a1
pub fn i_0938c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x938CC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x938CE;
    16
}
/// $0938CC: rts
pub fn i_0938cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x938CE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $09393C: cmpi.w #$200, $106f54.l
pub fn i_09393c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93944;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F54) as u32;
    sub_with_flags(cpu, v2, 0x200, Size::Word, false);
    20
}
/// $093944: scs.b d0
pub fn i_093944(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93946;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $093946: lea.l $9394c(pc), a1
pub fn i_093946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9394A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9394C;
    16
}
/// $09394A: rts
pub fn i_09394a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9394C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $093954: cmpi.w #$150, $106f54.l
pub fn i_093954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9395C;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F54) as u32;
    sub_with_flags(cpu, v2, 0x150, Size::Word, false);
    20
}
/// $09395C: scs.b d0
pub fn i_09395c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9395E;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $09395E: lea.l $93964(pc), a1
pub fn i_09395e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93962;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x93964;
    16
}
/// $093962: rts
pub fn i_093962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93964;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $093998: cmpi.w #$230, $106f54.l
pub fn i_093998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x939A0;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F54) as u32;
    sub_with_flags(cpu, v2, 0x230, Size::Word, false);
    20
}
/// $0939A0: scs.b d0
pub fn i_0939a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x939A2;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $0939A2: lea.l $939a8(pc), a1
pub fn i_0939a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x939A6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x939A8;
    16
}
/// $0939A6: rts
pub fn i_0939a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x939A8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0939DA: cmpi.w #$280, $106f54.l
pub fn i_0939da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x939E2;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F54) as u32;
    sub_with_flags(cpu, v2, 0x280, Size::Word, false);
    20
}
/// $0939E2: scs.b d0
pub fn i_0939e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x939E4;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $0939E4: lea.l $939ea(pc), a1
pub fn i_0939e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x939E8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x939EA;
    16
}
/// $0939E8: rts
pub fn i_0939e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x939EA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0939EC: cmpi.b #$1, $10e39a.l
pub fn i_0939ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x939F4;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10E39A) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $0939F4: sne.b d0
pub fn i_0939f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x939F6;
    cpu.ir = 0x56C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Ne) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $0939F6: lea.l $939fc(pc), a1
pub fn i_0939f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x939FA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x939FC;
    16
}
/// $0939FA: rts
pub fn i_0939fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x939FC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $093A10: cmpi.w #$671, $106f50.l
pub fn i_093a10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93A18;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x671, Size::Word, false);
    20
}
/// $093A18: scs.b d0
pub fn i_093a18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93A1A;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $093A1A: lea.l $93a20(pc), a1
pub fn i_093a1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93A1E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x93A20;
    16
}
/// $093A1E: rts
pub fn i_093a1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93A20;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $093A22: cmpi.w #$0, $106e88.l
pub fn i_093a22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93A2A;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106E88) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Word, false);
    20
}
/// $093A2A: sne.b d0
pub fn i_093a2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93A2C;
    cpu.ir = 0x56C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Ne) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $093A2C: lea.l $93a32(pc), a1
pub fn i_093a2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93A30;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x93A32;
    16
}
/// $093A30: rts
pub fn i_093a30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93A32;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $093A40: cmpi.w #$672, $106f50.l
pub fn i_093a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93A48;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x672, Size::Word, false);
    20
}
/// $093A48: scs.b d0
pub fn i_093a48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93A4A;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $093A4A: lea.l $93a50(pc), a1
pub fn i_093a4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93A4E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x93A50;
    16
}
/// $093A4E: rts
pub fn i_093a4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93A50;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $093A52: cmpi.w #$0, $106e88.l
pub fn i_093a52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93A5A;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106E88) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Word, false);
    20
}
/// $093A5A: sne.b d0
pub fn i_093a5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93A5C;
    cpu.ir = 0x56C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Ne) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $093A5C: lea.l $93a62(pc), a1
pub fn i_093a5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93A60;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x93A62;
    16
}
/// $093A60: rts
pub fn i_093a60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93A62;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $093A70: cmpi.w #$7d0, $106f50.l
pub fn i_093a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93A78;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x7D0, Size::Word, false);
    20
}
/// $093A78: scs.b d0
pub fn i_093a78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93A7A;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $093A7A: lea.l $93a80(pc), a1
pub fn i_093a7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93A7E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x93A80;
    16
}
/// $093A7E: rts
pub fn i_093a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93A80;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $093A82: cmpi.b #$1, $10e39a.l
pub fn i_093a82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93A8A;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10E39A) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $093A8A: sne.b d0
pub fn i_093a8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93A8C;
    cpu.ir = 0x56C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Ne) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $093A8C: lea.l $93a92(pc), a1
pub fn i_093a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93A90;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x93A92;
    16
}
/// $093A90: rts
pub fn i_093a90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93A92;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $093AA0: cmpi.w #$950, $106f50.l
pub fn i_093aa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93AA8;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x950, Size::Word, false);
    20
}
/// $093AA8: scs.b d0
pub fn i_093aa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93AAA;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $093AAA: lea.l $93ab0(pc), a1
pub fn i_093aaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93AAE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x93AB0;
    16
}
/// $093AAE: rts
pub fn i_093aae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93AB0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $093AB2: cmpi.b #$1, $10e39a.l
pub fn i_093ab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93ABA;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10E39A) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $093ABA: sne.b d0
pub fn i_093aba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93ABC;
    cpu.ir = 0x56C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Ne) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $093ABC: lea.l $93ac2(pc), a1
pub fn i_093abc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93AC0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x93AC2;
    16
}
/// $093AC0: rts
pub fn i_093ac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93AC2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $093AD0: cmpi.w #$950, $106f50.l
pub fn i_093ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93AD8;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x950, Size::Word, false);
    20
}
/// $093AD8: scs.b d0
pub fn i_093ad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93ADA;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $093ADA: lea.l $93ae0(pc), a1
pub fn i_093ada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93ADE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x93AE0;
    16
}
/// $093ADE: rts
pub fn i_093ade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93AE0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $093B14: cmpi.w #$af0, $106f50.l
pub fn i_093b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93B1C;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0xAF0, Size::Word, false);
    20
}
/// $093B1C: scs.b d0
pub fn i_093b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93B1E;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $093B1E: lea.l $93b24(pc), a1
pub fn i_093b1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93B22;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x93B24;
    16
}
/// $093B22: rts
pub fn i_093b22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93B24;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $093B32: cmpi.w #$b40, $106f50.l
pub fn i_093b32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93B3A;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0xB40, Size::Word, false);
    20
}
/// $093B3A: scs.b d0
pub fn i_093b3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93B3C;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $093B3C: lea.l $93b42(pc), a1
pub fn i_093b3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93B40;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x93B42;
    16
}
/// $093B40: rts
pub fn i_093b40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93B42;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $093B4A: cmpi.w #$c40, $106f50.l
pub fn i_093b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93B52;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0xC40, Size::Word, false);
    20
}
/// $093B52: scs.b d0
pub fn i_093b52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93B54;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $093B54: lea.l $93b5a(pc), a1
pub fn i_093b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93B58;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x93B5A;
    16
}
/// $093B58: rts
pub fn i_093b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93B5A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $093B70: cmpi.w #$cb0, $106f50.l
pub fn i_093b70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93B78;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0xCB0, Size::Word, false);
    20
}
/// $093B78: scs.b d0
pub fn i_093b78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93B7A;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $093B7A: lea.l $93b80(pc), a1
pub fn i_093b7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93B7E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x93B80;
    16
}
/// $093B7E: rts
pub fn i_093b7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93B80;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $093BB2: cmpi.w #$d60, $106f50.l
pub fn i_093bb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93BBA;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0xD60, Size::Word, false);
    20
}
/// $093BBA: scs.b d0
pub fn i_093bba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93BBC;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $093BBC: lea.l $93bc2(pc), a1
pub fn i_093bbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93BC0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x93BC2;
    16
}
/// $093BC0: rts
pub fn i_093bc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x93BC2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $094514: move.b #$0, $10e39b.l
pub fn i_094514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9451C;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E39B, v1 as u8);
    16
}
/// $09451C: lea.l $94522(pc), a0
pub fn i_09451c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94520;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x94522;
    16
}
/// $094520: rts
pub fn i_094520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94522;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $09474A: cmpi.w #$320, $106f50.l
pub fn i_09474a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94752;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x320, Size::Word, false);
    20
}
/// $094752: scs.b d0
pub fn i_094752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94754;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $094754: lea.l $9475a(pc), a1
pub fn i_094754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94758;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9475A;
    16
}
/// $094758: rts
pub fn i_094758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9475A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $09476A: cmpi.w #$340, $106f50.l
pub fn i_09476a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94772;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x340, Size::Word, false);
    20
}
/// $094772: scs.b d0
pub fn i_094772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94774;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $094774: lea.l $9477a(pc), a1
pub fn i_094774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94778;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9477A;
    16
}
/// $094778: rts
pub fn i_094778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9477A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $09477C: cmpi.b #$0, $10a2cf.l
pub fn i_09477c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94784;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10A2CF) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    20
}
/// $094784: seq.b d0
pub fn i_094784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94786;
    cpu.ir = 0x57C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Eq) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $094786: lea.l $9478c(pc), a1
pub fn i_094786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9478A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9478C;
    16
}
/// $09478A: rts
pub fn i_09478a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9478C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0949BC: cmpi.w #$580, $106f50.l
pub fn i_0949bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x949C4;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x580, Size::Word, false);
    20
}
/// $0949C4: scs.b d0
pub fn i_0949c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x949C6;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $0949C6: lea.l $949cc(pc), a1
pub fn i_0949c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x949CA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x949CC;
    16
}
/// $0949CA: rts
pub fn i_0949ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x949CC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0949DC: cmpi.w #$590, $106f50.l
pub fn i_0949dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x949E4;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x590, Size::Word, false);
    20
}
/// $0949E4: scs.b d0
pub fn i_0949e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x949E6;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $0949E6: lea.l $949ec(pc), a1
pub fn i_0949e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x949EA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x949EC;
    16
}
/// $0949EA: rts
pub fn i_0949ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x949EC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0949EE: cmpi.b #$0, $10a2cf.l
pub fn i_0949ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x949F6;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10A2CF) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    20
}
/// $0949F6: seq.b d0
pub fn i_0949f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x949F8;
    cpu.ir = 0x57C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Eq) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $0949F8: lea.l $949fe(pc), a1
pub fn i_0949f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x949FC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x949FE;
    16
}
/// $0949FC: rts
pub fn i_0949fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x949FE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $094C30: cmpi.w #$2e0, $106f50.l
pub fn i_094c30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94C38;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x2E0, Size::Word, false);
    20
}
/// $094C38: scs.b d0
pub fn i_094c38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94C3A;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $094C3A: lea.l $94c40(pc), a1
pub fn i_094c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94C3E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x94C40;
    16
}
/// $094C3E: rts
pub fn i_094c3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94C40;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $094C48: cmpi.w #$5e0, $106f50.l
pub fn i_094c48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94C50;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x5E0, Size::Word, false);
    20
}
/// $094C50: scs.b d0
pub fn i_094c50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94C52;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $094C52: lea.l $94c58(pc), a1
pub fn i_094c52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94C56;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x94C58;
    16
}
/// $094C56: rts
pub fn i_094c56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94C58;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $094C8C: cmpi.w #$600, $106f50.l
pub fn i_094c8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94C94;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x600, Size::Word, false);
    20
}
/// $094C94: scs.b d0
pub fn i_094c94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94C96;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $094C96: lea.l $94c9c(pc), a1
pub fn i_094c96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94C9A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x94C9C;
    16
}
/// $094C9A: rts
pub fn i_094c9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94C9C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $094CCE: cmpi.w #$640, $106f50.l
pub fn i_094cce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94CD6;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x640, Size::Word, false);
    20
}
/// $094CD6: scs.b d0
pub fn i_094cd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94CD8;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $094CD8: lea.l $94cde(pc), a1
pub fn i_094cd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94CDC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x94CDE;
    16
}
/// $094CDC: rts
pub fn i_094cdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94CDE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $094D04: cmpi.w #$720, $106f50.l
pub fn i_094d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94D0C;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x720, Size::Word, false);
    20
}
/// $094D0C: scs.b d0
pub fn i_094d0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94D0E;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $094D0E: lea.l $94d14(pc), a1
pub fn i_094d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94D12;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x94D14;
    16
}
/// $094D12: rts
pub fn i_094d12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94D14;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $094D4A: cmpi.w #$9a0, $106f50.l
pub fn i_094d4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94D52;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x9A0, Size::Word, false);
    20
}
/// $094D52: scs.b d0
pub fn i_094d52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94D54;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $094D54: lea.l $94d5a(pc), a1
pub fn i_094d54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94D58;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x94D5A;
    16
}
/// $094D58: rts
pub fn i_094d58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94D5A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $094D82: cmpi.w #$9f0, $106f50.l
pub fn i_094d82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94D8A;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x9F0, Size::Word, false);
    20
}
/// $094D8A: scs.b d0
pub fn i_094d8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94D8C;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $094D8C: lea.l $94d92(pc), a1
pub fn i_094d8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94D90;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x94D92;
    16
}
/// $094D90: rts
pub fn i_094d90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94D92;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $094DC4: cmpi.w #$a20, $106f50.l
pub fn i_094dc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94DCC;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0xA20, Size::Word, false);
    20
}
/// $094DCC: scs.b d0
pub fn i_094dcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94DCE;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $094DCE: lea.l $94dd4(pc), a1
pub fn i_094dce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94DD2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x94DD4;
    16
}
/// $094DD2: rts
pub fn i_094dd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94DD4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $094DFA: cmpi.w #$b00, $106f50.l
pub fn i_094dfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94E02;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0xB00, Size::Word, false);
    20
}
/// $094E02: scs.b d0
pub fn i_094e02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94E04;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $094E04: lea.l $94e0a(pc), a1
pub fn i_094e04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94E08;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x94E0A;
    16
}
/// $094E08: rts
pub fn i_094e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94E0A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $094E3C: cmpi.w #$b40, $106f50.l
pub fn i_094e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94E44;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0xB40, Size::Word, false);
    20
}
/// $094E44: scs.b d0
pub fn i_094e44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94E46;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $094E46: lea.l $94e4c(pc), a1
pub fn i_094e46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94E4A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x94E4C;
    16
}
/// $094E4A: rts
pub fn i_094e4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94E4C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $094E5C: cmpi.w #$ba0, $106f50.l
pub fn i_094e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94E64;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0xBA0, Size::Word, false);
    20
}
/// $094E64: scs.b d0
pub fn i_094e64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94E66;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $094E66: lea.l $94e6c(pc), a1
pub fn i_094e66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94E6A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x94E6C;
    16
}
/// $094E6A: rts
pub fn i_094e6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94E6C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $094EA0: cmpi.w #$ba0, $106f50.l
pub fn i_094ea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94EA8;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0xBA0, Size::Word, false);
    20
}
/// $094EA8: scs.b d0
pub fn i_094ea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94EAA;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $094EAA: lea.l $94eb0(pc), a1
pub fn i_094eaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94EAE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x94EB0;
    16
}
/// $094EAE: rts
pub fn i_094eae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94EB0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $094EB6: cmpi.w #$bc0, $106f50.l
pub fn i_094eb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94EBE;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0xBC0, Size::Word, false);
    20
}
/// $094EBE: scs.b d0
pub fn i_094ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94EC0;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $094EC0: lea.l $94ec6(pc), a1
pub fn i_094ec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94EC4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x94EC6;
    16
}
/// $094EC4: rts
pub fn i_094ec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94EC6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $094EEE: cmpi.w #$ce0, $106f50.l
pub fn i_094eee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94EF6;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0xCE0, Size::Word, false);
    20
}
/// $094EF6: scs.b d0
pub fn i_094ef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94EF8;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $094EF8: lea.l $94efe(pc), a1
pub fn i_094ef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94EFC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x94EFE;
    16
}
/// $094EFC: rts
pub fn i_094efc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94EFE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $094F00: move.l #$ffff0000, $106f64.l
pub fn i_094f00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94F0A;
    cpu.ir = 0x23FC;
    let v1: u32 = 0xFFFF0000;
    bus.write32(0x106F64, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $094F0A: lea.l $94f10(pc), a0
pub fn i_094f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94F0E;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x94F10;
    16
}
/// $094F0E: rts
pub fn i_094f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94F10;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $094F12: cmpi.w #$d20, $106f50.l
pub fn i_094f12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94F1A;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0xD20, Size::Word, false);
    20
}
/// $094F1A: scs.b d0
pub fn i_094f1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94F1C;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $094F1C: lea.l $94f22(pc), a1
pub fn i_094f1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94F20;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x94F22;
    16
}
/// $094F20: rts
pub fn i_094f20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94F22;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $094F56: cmpi.w #$d80, $106f50.l
pub fn i_094f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94F5E;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0xD80, Size::Word, false);
    20
}
/// $094F5E: scs.b d0
pub fn i_094f5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94F60;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $094F60: lea.l $94f66(pc), a1
pub fn i_094f60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94F64;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x94F66;
    16
}
/// $094F64: rts
pub fn i_094f64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94F66;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $094FBE: cmpi.w #$dc0, $106f50.l
pub fn i_094fbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94FC6;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0xDC0, Size::Word, false);
    20
}
/// $094FC6: scs.b d0
pub fn i_094fc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94FC8;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $094FC8: lea.l $94fce(pc), a1
pub fn i_094fc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94FCC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x94FCE;
    16
}
/// $094FCC: rts
pub fn i_094fcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94FCE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $094FD0: move.l #$0, $106f64.l
pub fn i_094fd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94FDA;
    cpu.ir = 0x23FC;
    let v1: u32 = 0x0;
    bus.write32(0x106F64, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $094FDA: lea.l $94fe0(pc), a0
pub fn i_094fda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94FDE;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x94FE0;
    16
}
/// $094FDE: rts
pub fn i_094fde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94FE0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $094FE2: cmpi.w #$dc0, $106f50.l
pub fn i_094fe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94FEA;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0xDC0, Size::Word, false);
    20
}
/// $094FEA: scs.b d0
pub fn i_094fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94FEC;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $094FEC: lea.l $94ff2(pc), a1
pub fn i_094fec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94FF0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x94FF2;
    16
}
/// $094FF0: rts
pub fn i_094ff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x94FF2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $094FF8: cmpi.w #$e10, $106f50.l
pub fn i_094ff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95000;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0xE10, Size::Word, false);
    20
}
/// $095000: scs.b d0
pub fn i_095000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95002;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $095002: lea.l $95008(pc), a1
pub fn i_095002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95006;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x95008;
    16
}
/// $095006: rts
pub fn i_095006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95008;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $095010: cmpi.w #$e80, $106f50.l
pub fn i_095010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95018;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0xE80, Size::Word, false);
    20
}
/// $095018: scs.b d0
pub fn i_095018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9501A;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $09501A: lea.l $95020(pc), a1
pub fn i_09501a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9501E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x95020;
    16
}
/// $09501E: rts
pub fn i_09501e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95020;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $095052: cmpi.w #$ef0, $106f50.l
pub fn i_095052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9505A;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0xEF0, Size::Word, false);
    20
}
/// $09505A: scs.b d0
pub fn i_09505a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9505C;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $09505C: lea.l $95062(pc), a1
pub fn i_09505c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95060;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x95062;
    16
}
/// $095060: rts
pub fn i_095060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95062;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $095072: cmpi.w #$f00, $106f50.l
pub fn i_095072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9507A;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0xF00, Size::Word, false);
    20
}
/// $09507A: scs.b d0
pub fn i_09507a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9507C;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $09507C: lea.l $95082(pc), a1
pub fn i_09507c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95080;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x95082;
    16
}
/// $095080: rts
pub fn i_095080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95082;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $095084: cmpi.b #$0, $10a2cf.l
pub fn i_095084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9508C;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10A2CF) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    20
}
/// $09508C: seq.b d0
pub fn i_09508c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9508E;
    cpu.ir = 0x57C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Eq) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $09508E: lea.l $95094(pc), a1
pub fn i_09508e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95092;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x95094;
    16
}
/// $095092: rts
pub fn i_095092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95094;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $095306: cmpi.w #$2b0, $106f50.l
pub fn i_095306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9530E;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x2B0, Size::Word, false);
    20
}
/// $09530E: scs.b d0
pub fn i_09530e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95310;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $095310: lea.l $95316(pc), a1
pub fn i_095310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95314;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x95316;
    16
}
/// $095314: rts
pub fn i_095314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95316;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $095348: cmpi.w #$480, $106f50.l
pub fn i_095348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95350;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x480, Size::Word, false);
    20
}
/// $095350: scs.b d0
pub fn i_095350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95352;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $095352: lea.l $95358(pc), a1
pub fn i_095352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95356;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x95358;
    16
}
/// $095356: rts
pub fn i_095356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95358;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $095368: cmpi.w #$4f0, $106f50.l
pub fn i_095368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95370;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x4F0, Size::Word, false);
    20
}
/// $095370: scs.b d0
pub fn i_095370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95372;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $095372: lea.l $95378(pc), a1
pub fn i_095372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95376;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x95378;
    16
}
/// $095376: rts
pub fn i_095376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95378;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $095388: cmpi.w #$500, $106f50.l
pub fn i_095388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95390;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x500, Size::Word, false);
    20
}
/// $095390: scs.b d0
pub fn i_095390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95392;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $095392: lea.l $95398(pc), a1
pub fn i_095392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95396;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x95398;
    16
}
/// $095396: rts
pub fn i_095396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95398;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $09539A: cmpi.b #$0, $10a2cf.l
pub fn i_09539a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x953A2;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10A2CF) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    20
}
/// $0953A2: seq.b d0
pub fn i_0953a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x953A4;
    cpu.ir = 0x57C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Eq) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $0953A4: lea.l $953aa(pc), a1
pub fn i_0953a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x953A8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x953AA;
    16
}
/// $0953A8: rts
pub fn i_0953a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x953AA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $095634: cmpi.w #$340, $106f50.l
pub fn i_095634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9563C;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x340, Size::Word, false);
    20
}
/// $09563C: scs.b d0
pub fn i_09563c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9563E;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $09563E: lea.l $95644(pc), a1
pub fn i_09563e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95642;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x95644;
    16
}
/// $095642: rts
pub fn i_095642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95644;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $095646: move.b #$1, $10e39b.l
pub fn i_095646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9564E;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E39B, v1 as u8);
    16
}
/// $09564E: lea.l $95654(pc), a0
pub fn i_09564e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95652;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x95654;
    16
}
/// $095652: rts
pub fn i_095652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95654;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $095692: move.b #$0, $10e39b.l
pub fn i_095692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9569A;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E39B, v1 as u8);
    16
}
/// $09569A: lea.l $956a0(pc), a0
pub fn i_09569a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9569E;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x956A0;
    16
}
/// $09569E: rts
pub fn i_09569e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x956A0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0958C8: cmpi.w #$320, $106f50.l
pub fn i_0958c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x958D0;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x320, Size::Word, false);
    20
}
/// $0958D0: scs.b d0
pub fn i_0958d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x958D2;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $0958D2: lea.l $958d8(pc), a1
pub fn i_0958d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x958D6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x958D8;
    16
}
/// $0958D6: rts
pub fn i_0958d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x958D8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0958E8: cmpi.w #$340, $106f50.l
pub fn i_0958e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x958F0;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x340, Size::Word, false);
    20
}
/// $0958F0: scs.b d0
pub fn i_0958f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x958F2;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $0958F2: lea.l $958f8(pc), a1
pub fn i_0958f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x958F6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x958F8;
    16
}
/// $0958F6: rts
pub fn i_0958f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x958F8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0958FA: cmpi.b #$0, $10a2cf.l
pub fn i_0958fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95902;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10A2CF) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    20
}
/// $095902: seq.b d0
pub fn i_095902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95904;
    cpu.ir = 0x57C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Eq) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $095904: lea.l $9590a(pc), a1
pub fn i_095904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95908;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9590A;
    16
}
/// $095908: rts
pub fn i_095908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9590A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $095B72: cmpi.w #$950, $106f50.l
pub fn i_095b72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95B7A;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x950, Size::Word, false);
    20
}
/// $095B7A: scs.b d0
pub fn i_095b7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95B7C;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $095B7C: lea.l $95b82(pc), a1
pub fn i_095b7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95B80;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x95B82;
    16
}
/// $095B80: rts
pub fn i_095b80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95B82;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $095BB6: cmpi.w #$a60, $106f50.l
pub fn i_095bb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95BBE;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0xA60, Size::Word, false);
    20
}
/// $095BBE: scs.b d0
pub fn i_095bbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95BC0;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $095BC0: lea.l $95bc6(pc), a1
pub fn i_095bc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95BC4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x95BC6;
    16
}
/// $095BC4: rts
pub fn i_095bc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95BC6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $095BD6: cmpi.w #$a70, $106f50.l
pub fn i_095bd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95BDE;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0xA70, Size::Word, false);
    20
}
/// $095BDE: scs.b d0
pub fn i_095bde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95BE0;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $095BE0: lea.l $95be6(pc), a1
pub fn i_095be0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95BE4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x95BE6;
    16
}
/// $095BE4: rts
pub fn i_095be4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95BE6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $095BE8: cmpi.b #$0, $10a2cf.l
pub fn i_095be8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95BF0;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10A2CF) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    20
}
/// $095BF0: seq.b d0
pub fn i_095bf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95BF2;
    cpu.ir = 0x57C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Eq) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $095BF2: lea.l $95bf8(pc), a1
pub fn i_095bf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95BF6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x95BF8;
    16
}
/// $095BF6: rts
pub fn i_095bf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95BF8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $095E2A: cmpi.w #$2e0, $106f50.l
pub fn i_095e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95E32;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x2E0, Size::Word, false);
    20
}
/// $095E32: scs.b d0
pub fn i_095e32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95E34;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $095E34: lea.l $95e3a(pc), a1
pub fn i_095e34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95E38;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x95E3A;
    16
}
/// $095E38: rts
pub fn i_095e38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95E3A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $095E42: cmpi.w #$590, $106f50.l
pub fn i_095e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95E4A;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x590, Size::Word, false);
    20
}
/// $095E4A: scs.b d0
pub fn i_095e4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95E4C;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $095E4C: lea.l $95e52(pc), a1
pub fn i_095e4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95E50;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x95E52;
    16
}
/// $095E50: rts
pub fn i_095e50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95E52;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $095E84: cmpi.w #$5e0, $106f50.l
pub fn i_095e84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95E8C;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x5E0, Size::Word, false);
    20
}
/// $095E8C: scs.b d0
pub fn i_095e8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95E8E;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $095E8E: lea.l $95e94(pc), a1
pub fn i_095e8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95E92;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x95E94;
    16
}
/// $095E92: rts
pub fn i_095e92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95E94;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $095EC8: cmpi.w #$600, $106f50.l
pub fn i_095ec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95ED0;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x600, Size::Word, false);
    20
}
/// $095ED0: scs.b d0
pub fn i_095ed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95ED2;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $095ED2: lea.l $95ed8(pc), a1
pub fn i_095ed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95ED6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x95ED8;
    16
}
/// $095ED6: rts
pub fn i_095ed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95ED8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $095F0A: cmpi.w #$640, $106f50.l
pub fn i_095f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95F12;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x640, Size::Word, false);
    20
}
/// $095F12: scs.b d0
pub fn i_095f12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95F14;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $095F14: lea.l $95f1a(pc), a1
pub fn i_095f14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95F18;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x95F1A;
    16
}
/// $095F18: rts
pub fn i_095f18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95F1A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $095F40: cmpi.w #$980, $106f50.l
pub fn i_095f40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95F48;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x980, Size::Word, false);
    20
}
/// $095F48: scs.b d0
pub fn i_095f48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95F4A;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $095F4A: lea.l $95f50(pc), a1
pub fn i_095f4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95F4E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x95F50;
    16
}
/// $095F4E: rts
pub fn i_095f4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95F50;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $095F86: cmpi.w #$9a0, $106f50.l
pub fn i_095f86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95F8E;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x9A0, Size::Word, false);
    20
}
/// $095F8E: scs.b d0
pub fn i_095f8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95F90;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $095F90: lea.l $95f96(pc), a1
pub fn i_095f90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95F94;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x95F96;
    16
}
/// $095F94: rts
pub fn i_095f94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95F96;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $095FBE: cmpi.w #$a20, $106f50.l
pub fn i_095fbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95FC6;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0xA20, Size::Word, false);
    20
}
/// $095FC6: scs.b d0
pub fn i_095fc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95FC8;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $095FC8: lea.l $95fce(pc), a1
pub fn i_095fc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95FCC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x95FCE;
    16
}
/// $095FCC: rts
pub fn i_095fcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95FCE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $095FF4: cmpi.w #$a10, $106f50.l
pub fn i_095ff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95FFC;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0xA10, Size::Word, false);
    20
}
/// $095FFC: scs.b d0
pub fn i_095ffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x95FFE;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $095FFE: lea.l $96004(pc), a1
pub fn i_095ffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96002;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x96004;
    16
}
/// $096002: rts
pub fn i_096002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96004;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $096014: cmpi.w #$a30, $106f50.l
pub fn i_096014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9601C;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0xA30, Size::Word, false);
    20
}
/// $09601C: scs.b d0
pub fn i_09601c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9601E;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $09601E: lea.l $96024(pc), a1
pub fn i_09601e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96022;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x96024;
    16
}
/// $096022: rts
pub fn i_096022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96024;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $096026: cmpi.b #$0, $10a2cf.l
pub fn i_096026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9602E;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10A2CF) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    20
}
/// $09602E: seq.b d0
pub fn i_09602e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96030;
    cpu.ir = 0x57C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Eq) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $096030: lea.l $96036(pc), a1
pub fn i_096030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96034;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x96036;
    16
}
/// $096034: rts
pub fn i_096034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96036;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $096266: cmpi.w #$520, $106f50.l
pub fn i_096266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9626E;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x520, Size::Word, false);
    20
}
/// $09626E: scs.b d0
pub fn i_09626e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96270;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $096270: lea.l $96276(pc), a1
pub fn i_096270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96274;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x96276;
    16
}
/// $096274: rts
pub fn i_096274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96276;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $096286: cmpi.w #$540, $106f50.l
pub fn i_096286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9628E;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x540, Size::Word, false);
    20
}
/// $09628E: scs.b d0
pub fn i_09628e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96290;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $096290: lea.l $96296(pc), a1
pub fn i_096290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96294;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x96296;
    16
}
/// $096294: rts
pub fn i_096294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96296;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $096298: cmpi.b #$0, $10a2cf.l
pub fn i_096298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x962A0;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10A2CF) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    20
}
/// $0962A0: seq.b d0
pub fn i_0962a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x962A2;
    cpu.ir = 0x57C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Eq) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $0962A2: lea.l $962a8(pc), a1
pub fn i_0962a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x962A6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x962A8;
    16
}
/// $0962A6: rts
pub fn i_0962a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x962A8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $096532: cmpi.w #$340, $106f50.l
pub fn i_096532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9653A;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x340, Size::Word, false);
    20
}
/// $09653A: scs.b d0
pub fn i_09653a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9653C;
    cpu.ir = 0x55C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $09653C: lea.l $96542(pc), a1
pub fn i_09653c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96540;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x96542;
    16
}
/// $096540: rts
pub fn i_096540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96542;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $096544: move.b #$1, $10e39b.l
pub fn i_096544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9654C;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E39B, v1 as u8);
    16
}
/// $09654C: lea.l $96552(pc), a0
pub fn i_09654c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96550;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x96552;
    16
}
/// $096550: rts
pub fn i_096550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96552;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ChildRank_CmpByte10_0967A4  ($0967A4, 16 B, entity/childrank_cmp_islands.s)
/// $0967A4: movea.l $8(a6), a1
pub fn i_0967a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x967A8;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0967A8: move.b $10(a6), d0
pub fn i_0967a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x967AC;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0967AC: cmp.b $10(a1), d0
pub fn i_0967ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x967B0;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $0967B0: bcs.w $967ba
pub fn i_0967b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x967B4;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x967BA; return 10; }
    8
}
// ===== ClearXN_0967b4  ($0967B4, 6 B, thunks/ccr_helpers.c)
/// $0967B4: andi.b #$ee, ccr
pub fn i_0967b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x967B8;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0967B8: rts
pub fn i_0967b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x967BA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_0967ba  ($0967BA, 6 B, thunks/ccr_helpers.c)
/// $0967BA: ori.b #$11, ccr
pub fn i_0967ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x967BE;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0967BE: rts
pub fn i_0967be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x967C0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SelectPositive_TwoSlots_0967C0  ($0967C0, 62 B, frontend/attract_sub_helpers_0967c0.s)
/// $0967C0: lea.l $100440.l, a0
pub fn i_0967c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x967C6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100440;
    20
}
/// $0967C6: jsr $5e2d8.l
pub fn i_0967c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x967CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x967CC);
    cpu.pc = 0x5E2D8;
    16
}
/// $0967CC: move.l d0, d2
pub fn i_0967cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x967CE;
    cpu.ir = 0x2400;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[2] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0967CE: move.l d1, d3
pub fn i_0967ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x967D0;
    cpu.ir = 0x2601;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.d[3] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0967D0: lea.l $1004e0.l, a0
pub fn i_0967d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x967D6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1004E0;
    20
}
/// $0967D6: jsr $5e2d8.l
pub fn i_0967d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x967DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x967DC);
    cpu.pc = 0x5E2D8;
    16
}
/// $0967DC: tst.l d0
pub fn i_0967dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x967DE;
    cpu.ir = 0x4A80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0967DE: bpl.w $967e4
pub fn i_0967de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x967E2;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x967E4; return 10; }
    8
}
/// $0967E2: move.l d2, d0
pub fn i_0967e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x967E4;
    cpu.ir = 0x2002;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0967E4: tst.l d2
pub fn i_0967e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x967E6;
    cpu.ir = 0x4A82;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0967E6: bpl.w $967ec
pub fn i_0967e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x967EA;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x967EC; return 10; }
    8
}
/// $0967EA: move.l d0, d2
pub fn i_0967ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x967EC;
    cpu.ir = 0x2400;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[2] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0967EC: tst.l d1
pub fn i_0967ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x967EE;
    cpu.ir = 0x4A81;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0967EE: bpl.w $967f4
pub fn i_0967ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x967F2;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x967F4; return 10; }
    8
}
/// $0967F2: move.l d3, d1
pub fn i_0967f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x967F4;
    cpu.ir = 0x2203;
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.d[1] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0967F4: tst.l d3
pub fn i_0967f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x967F6;
    cpu.ir = 0x4A83;
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0967F6: bpl.w $967fc
pub fn i_0967f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x967FA;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x967FC; return 10; }
    8
}
/// $0967FA: move.l d1, d3
pub fn i_0967fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x967FC;
    cpu.ir = 0x2601;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.d[3] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0967FC: rts
pub fn i_0967fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x967FE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Attract_State0_Handler_0967FE  ($0967FE, 66 B, frontend/attract_state_machines_0967fe.s)
/// $0967FE: move.b #$0, d0
pub fn i_0967fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96802;
    cpu.ir = 0x103C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $096802: move.b d0, $21(a6)
pub fn i_096802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96806;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $096806: jsr $43568.l
pub fn i_096806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9680C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9680C);
    cpu.pc = 0x43568;
    16
}
/// $09680C: move.b #$0, $10e39e.l
pub fn i_09680c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96814;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E39E, v1 as u8);
    16
}
/// $096814: move.w #$0, $70(a6)
pub fn i_096814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9681A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09681A: lea.l $96820(pc), a1
pub fn i_09681a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9681E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x96820;
    16
}
/// $09681E: move.l a1, (a6)
pub fn i_09681e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96820;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $096820: jsr $967c0(pc)
pub fn i_096820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96824;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x96824);
    cpu.pc = 0x967C0;
    16
}
/// $096824: jsr $436de.l
pub fn i_096824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9682A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9682A);
    cpu.pc = 0x436DE;
    16
}
/// $09682A: jsr $969c2(pc)
pub fn i_09682a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9682E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9682E);
    cpu.pc = 0x969C2;
    16
}
/// $09682E: jsr $96a80(pc)
pub fn i_09682e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96832;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x96832);
    cpu.pc = 0x96A80;
    16
}
/// $096832: jsr $96b24(pc)
pub fn i_096832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96836;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x96836);
    cpu.pc = 0x96B24;
    16
}
/// $096836: move.b #$1, $10e39a.l
pub fn i_096836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9683E;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E39A, v1 as u8);
    16
}
/// $09683E: rts
pub fn i_09683e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96840;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Attract_State1_Handler_096840  ($096840, 66 B, frontend/attract_state_machines_0967fe.s)
/// $096840: move.b #$1, d0
pub fn i_096840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96844;
    cpu.ir = 0x103C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $096844: move.b d0, $21(a6)
pub fn i_096844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96848;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $096848: jsr $43568.l
pub fn i_096848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9684E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9684E);
    cpu.pc = 0x43568;
    16
}
/// $09684E: move.b #$0, $10e39e.l
pub fn i_09684e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96856;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E39E, v1 as u8);
    16
}
/// $096856: move.w #$0, $70(a6)
pub fn i_096856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9685C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09685C: lea.l $96862(pc), a1
pub fn i_09685c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96860;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x96862;
    16
}
/// $096860: move.l a1, (a6)
pub fn i_096860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96862;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $096862: jsr $967c0(pc)
pub fn i_096862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96866;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x96866);
    cpu.pc = 0x967C0;
    16
}
/// $096866: jsr $436de.l
pub fn i_096866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9686C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9686C);
    cpu.pc = 0x436DE;
    16
}
/// $09686C: jsr $969c2(pc)
pub fn i_09686c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96870;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x96870);
    cpu.pc = 0x969C2;
    16
}
/// $096870: jsr $96a80(pc)
pub fn i_096870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96874;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x96874);
    cpu.pc = 0x96A80;
    16
}
/// $096874: jsr $96b24(pc)
pub fn i_096874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96878;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x96878);
    cpu.pc = 0x96B24;
    16
}
/// $096878: move.b #$1, $10e39a.l
pub fn i_096878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96880;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E39A, v1 as u8);
    16
}
/// $096880: rts
pub fn i_096880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96882;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Attract_State2_Handler_096882  ($096882, 66 B, frontend/attract_state_machines_0967fe.s)
/// $096882: move.b #$2, d0
pub fn i_096882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96886;
    cpu.ir = 0x103C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $096886: move.b d0, $21(a6)
pub fn i_096886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9688A;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09688A: jsr $43568.l
pub fn i_09688a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96890;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x96890);
    cpu.pc = 0x43568;
    16
}
/// $096890: move.b #$0, $10e39e.l
pub fn i_096890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96898;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E39E, v1 as u8);
    16
}
/// $096898: move.w #$0, $70(a6)
pub fn i_096898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9689E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09689E: lea.l $968a4(pc), a1
pub fn i_09689e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x968A2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x968A4;
    16
}
/// $0968A2: move.l a1, (a6)
pub fn i_0968a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x968A4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0968A4: jsr $967c0(pc)
pub fn i_0968a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x968A8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x968A8);
    cpu.pc = 0x967C0;
    16
}
/// $0968A8: jsr $436de.l
pub fn i_0968a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x968AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x968AE);
    cpu.pc = 0x436DE;
    16
}
/// $0968AE: jsr $969c2(pc)
pub fn i_0968ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x968B2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x968B2);
    cpu.pc = 0x969C2;
    16
}
/// $0968B2: jsr $96a80(pc)
pub fn i_0968b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x968B6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x968B6);
    cpu.pc = 0x96A80;
    16
}
/// $0968B6: jsr $96b24(pc)
pub fn i_0968b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x968BA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x968BA);
    cpu.pc = 0x96B24;
    16
}
/// $0968BA: move.b #$1, $10e39a.l
pub fn i_0968ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x968C2;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E39A, v1 as u8);
    16
}
/// $0968C2: rts
pub fn i_0968c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x968C4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Attract_State3_Handler_0968C4  ($0968C4, 66 B, frontend/attract_state_machines_0967fe.s)
/// $0968C4: move.b #$3, d0
pub fn i_0968c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x968C8;
    cpu.ir = 0x103C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0968C8: move.b d0, $21(a6)
pub fn i_0968c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x968CC;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0968CC: jsr $43568.l
pub fn i_0968cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x968D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x968D2);
    cpu.pc = 0x43568;
    16
}
/// $0968D2: move.b #$0, $10e39e.l
pub fn i_0968d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x968DA;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E39E, v1 as u8);
    16
}
/// $0968DA: move.w #$0, $70(a6)
pub fn i_0968da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x968E0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0968E0: lea.l $968e6(pc), a1
pub fn i_0968e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x968E4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x968E6;
    16
}
/// $0968E4: move.l a1, (a6)
pub fn i_0968e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x968E6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0968E6: jsr $967c0(pc)
pub fn i_0968e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x968EA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x968EA);
    cpu.pc = 0x967C0;
    16
}
/// $0968EA: jsr $436de.l
pub fn i_0968ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x968F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x968F0);
    cpu.pc = 0x436DE;
    16
}
/// $0968F0: jsr $969c2(pc)
pub fn i_0968f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x968F4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x968F4);
    cpu.pc = 0x969C2;
    16
}
/// $0968F4: jsr $96a80(pc)
pub fn i_0968f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x968F8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x968F8);
    cpu.pc = 0x96A80;
    16
}
/// $0968F8: jsr $96b24(pc)
pub fn i_0968f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x968FC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x968FC);
    cpu.pc = 0x96B24;
    16
}
/// $0968FC: move.b #$1, $10e39a.l
pub fn i_0968fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96904;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E39A, v1 as u8);
    16
}
/// $096904: rts
pub fn i_096904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96906;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Attract_State4_Handler_096906  ($096906, 66 B, frontend/attract_state_machines_0967fe.s)
/// $096906: move.b #$4, d0
pub fn i_096906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9690A;
    cpu.ir = 0x103C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $09690A: move.b d0, $21(a6)
pub fn i_09690a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9690E;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09690E: jsr $43568.l
pub fn i_09690e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96914;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x96914);
    cpu.pc = 0x43568;
    16
}
/// $096914: move.b #$0, $10e39e.l
pub fn i_096914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9691C;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E39E, v1 as u8);
    16
}
/// $09691C: move.w #$0, $70(a6)
pub fn i_09691c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96922;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $096922: lea.l $96928(pc), a1
pub fn i_096922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96926;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x96928;
    16
}
/// $096926: move.l a1, (a6)
pub fn i_096926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96928;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $096928: jsr $967c0(pc)
pub fn i_096928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9692C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9692C);
    cpu.pc = 0x967C0;
    16
}
/// $09692C: jsr $436de.l
pub fn i_09692c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96932;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x96932);
    cpu.pc = 0x436DE;
    16
}
/// $096932: jsr $969c2(pc)
pub fn i_096932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96936;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x96936);
    cpu.pc = 0x969C2;
    16
}
/// $096936: jsr $96a80(pc)
pub fn i_096936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9693A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9693A);
    cpu.pc = 0x96A80;
    16
}
/// $09693A: jsr $96b24(pc)
pub fn i_09693a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9693E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9693E);
    cpu.pc = 0x96B24;
    16
}
/// $09693E: move.b #$1, $10e39a.l
pub fn i_09693e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96946;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E39A, v1 as u8);
    16
}
/// $096946: rts
pub fn i_096946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96948;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Attract_State5_Handler_096948  ($096948, 66 B, frontend/attract_state_machines_0967fe.s)
/// $096948: move.b #$5, d0
pub fn i_096948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9694C;
    cpu.ir = 0x103C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $09694C: move.b d0, $21(a6)
pub fn i_09694c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96950;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $096950: jsr $43568.l
pub fn i_096950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96956;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x96956);
    cpu.pc = 0x43568;
    16
}
/// $096956: move.b #$0, $10e39e.l
pub fn i_096956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9695E;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E39E, v1 as u8);
    16
}
/// $09695E: move.w #$0, $70(a6)
pub fn i_09695e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96964;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $096964: lea.l $9696a(pc), a1
pub fn i_096964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96968;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9696A;
    16
}
/// $096968: move.l a1, (a6)
pub fn i_096968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9696A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09696A: jsr $967c0(pc)
pub fn i_09696a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9696E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9696E);
    cpu.pc = 0x967C0;
    16
}
/// $09696E: jsr $436de.l
pub fn i_09696e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96974;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x96974);
    cpu.pc = 0x436DE;
    16
}
/// $096974: jsr $969c2(pc)
pub fn i_096974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96978;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x96978);
    cpu.pc = 0x969C2;
    16
}
/// $096978: jsr $96a80(pc)
pub fn i_096978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9697C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9697C);
    cpu.pc = 0x96A80;
    16
}
/// $09697C: jsr $96b24(pc)
pub fn i_09697c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96980;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x96980);
    cpu.pc = 0x96B24;
    16
}
/// $096980: move.b #$1, $10e39a.l
pub fn i_096980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96988;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E39A, v1 as u8);
    16
}
/// $096988: rts
pub fn i_096988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9698A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Attract_State7_Handler_09698A  ($09698A, 56 B, frontend/attract_state_machines_0967fe.s)
/// $09698A: move.b #$7, d0
pub fn i_09698a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9698E;
    cpu.ir = 0x103C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $09698E: move.b d0, $21(a6)
pub fn i_09698e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x96992;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
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
// ===== HiScore_Tpl_StaticLogo_0977c2  ($0977C2, 20 B, frontend/hiscore_memcard_options_09773c.s)
/// $0977C2: lea.l $97962(pc), a1
pub fn i_0977c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x977C6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x97962;
    16
}
/// $0977C6: jsr $4ae.l
pub fn i_0977c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x977CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x977CC);
    cpu.pc = 0x4AE;
    16
}
/// $0977CC: move.w #$3c, $80(a6)
pub fn i_0977cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x977D2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0977D2: bra.w $97816
pub fn i_0977d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x977D6;
    cpu.ir = 0x6000;
    cpu.pc = 0x97816;
    10
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
/// $097962: move.b #$7f, $21(a6)
pub fn i_097962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97968;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x7F;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $097968: move.b #$ff, $32(a6)
pub fn i_097968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9796E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09796E: move.w #$ff, $70(a6)
pub fn i_09796e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97974;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
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
/// $097D24: clr.b $21(a6)
pub fn i_097d24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97D28;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $097D28: moveq #$0, d0
pub fn i_097d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97D2A;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $097D2A: move.w d0, $7a(a6)
pub fn i_097d2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97D2E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $097D2E: move.w d0, $7c(a6)
pub fn i_097d2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97D32;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $097D32: move.w d0, $7e(a6)
pub fn i_097d32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97D36;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $097D36: move.w d0, $72(a6)
pub fn i_097d36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97D3A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $097D3A: move.w d0, $70(a6)
pub fn i_097d3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97D3E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $097D3E: move.b d0, $8c(a6)
pub fn i_097d3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97D42;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $097D42: move.b d0, $8d(a6)
pub fn i_097d42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97D46;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $097D46: move.l #$c0, $84(a6)
pub fn i_097d46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97D4E;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xC0;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $097D4E: move.w #$b40, d1
pub fn i_097d4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97D52;
    cpu.ir = 0x323C;
    let v1: u32 = 0xB40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $097D52: move.w d1, $74(a6)
pub fn i_097d52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97D56;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $097D56: move.w d1, $76(a6)
pub fn i_097d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97D5A;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $097D5A: move.w d1, $78(a6)
pub fn i_097d5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97D5E;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad2, v1 as u16);
    12
}
/// $097D5E: movea.l $80(a6), a1
pub fn i_097d5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97D62;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $097D62: move.w #$b40, d0
pub fn i_097d62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97D66;
    cpu.ir = 0x303C;
    let v1: u32 = 0xB40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $097D66: move.w #$e, d1
pub fn i_097d66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97D6A;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $097D6A: moveq #$2, d2
pub fn i_097d6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97D6C;
    cpu.ir = 0x7402;
    cpu.d[2] = 0x2;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $097D6C: jsr $5da9c.l
pub fn i_097d6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97D72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x97D72);
    cpu.pc = 0x5DA9C;
    16
}
/// $097D72: lea.l $97d78(pc), a1
pub fn i_097d72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97D76;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x97D78;
    16
}
/// $097D76: move.l a1, (a6)
pub fn i_097d76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x97D78;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
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
/// $098068: rts
pub fn i_098068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9806A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
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

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x092962, i_092962),
    (0x092966, i_092966),
    (0x092994, i_092994),
    (0x09299C, i_09299c),
    (0x09299E, i_09299e),
    (0x0929A2, i_0929a2),
    (0x0929A6, i_0929a6),
    (0x0929AE, i_0929ae),
    (0x0929B0, i_0929b0),
    (0x0929B4, i_0929b4),
    (0x0929CA, i_0929ca),
    (0x0929D2, i_0929d2),
    (0x0929D4, i_0929d4),
    (0x0929D8, i_0929d8),
    (0x092A06, i_092a06),
    (0x092A0E, i_092a0e),
    (0x092A10, i_092a10),
    (0x092A14, i_092a14),
    (0x092A18, i_092a18),
    (0x092A20, i_092a20),
    (0x092A22, i_092a22),
    (0x092A26, i_092a26),
    (0x092A36, i_092a36),
    (0x092A3E, i_092a3e),
    (0x092A40, i_092a40),
    (0x092A44, i_092a44),
    (0x092A48, i_092a48),
    (0x092A50, i_092a50),
    (0x092A52, i_092a52),
    (0x092A56, i_092a56),
    (0x092A66, i_092a66),
    (0x092A6E, i_092a6e),
    (0x092A70, i_092a70),
    (0x092A74, i_092a74),
    (0x092A78, i_092a78),
    (0x092A80, i_092a80),
    (0x092A82, i_092a82),
    (0x092A86, i_092a86),
    (0x092A96, i_092a96),
    (0x092A9E, i_092a9e),
    (0x092AA0, i_092aa0),
    (0x092AA4, i_092aa4),
    (0x092AA8, i_092aa8),
    (0x092AB0, i_092ab0),
    (0x092AB2, i_092ab2),
    (0x092AB6, i_092ab6),
    (0x092DF0, i_092df0),
    (0x092DF8, i_092df8),
    (0x092DFA, i_092dfa),
    (0x092DFE, i_092dfe),
    (0x092E08, i_092e08),
    (0x092E10, i_092e10),
    (0x092E12, i_092e12),
    (0x092E16, i_092e16),
    (0x092E20, i_092e20),
    (0x092E28, i_092e28),
    (0x092E2A, i_092e2a),
    (0x092E2E, i_092e2e),
    (0x092E32, i_092e32),
    (0x092E3A, i_092e3a),
    (0x092E3C, i_092e3c),
    (0x092E40, i_092e40),
    (0x092E56, i_092e56),
    (0x092E5E, i_092e5e),
    (0x092E60, i_092e60),
    (0x092E64, i_092e64),
    (0x092E6E, i_092e6e),
    (0x092E76, i_092e76),
    (0x092E78, i_092e78),
    (0x092E7C, i_092e7c),
    (0x092E80, i_092e80),
    (0x092E88, i_092e88),
    (0x092E8A, i_092e8a),
    (0x092E8E, i_092e8e),
    (0x092E98, i_092e98),
    (0x092EA0, i_092ea0),
    (0x092EA2, i_092ea2),
    (0x092EA6, i_092ea6),
    (0x092F00, i_092f00),
    (0x092F08, i_092f08),
    (0x092F0A, i_092f0a),
    (0x092F0E, i_092f0e),
    (0x092F44, i_092f44),
    (0x092F4C, i_092f4c),
    (0x092F4E, i_092f4e),
    (0x092F52, i_092f52),
    (0x092F88, i_092f88),
    (0x092F90, i_092f90),
    (0x092F92, i_092f92),
    (0x092F96, i_092f96),
    (0x092F9A, i_092f9a),
    (0x092FA2, i_092fa2),
    (0x092FA4, i_092fa4),
    (0x092FA8, i_092fa8),
    (0x092FB8, i_092fb8),
    (0x092FC0, i_092fc0),
    (0x092FC2, i_092fc2),
    (0x092FC6, i_092fc6),
    (0x092FFA, i_092ffa),
    (0x093002, i_093002),
    (0x093004, i_093004),
    (0x093008, i_093008),
    (0x093012, i_093012),
    (0x09301A, i_09301a),
    (0x09301C, i_09301c),
    (0x093020, i_093020),
    (0x09302A, i_09302a),
    (0x093032, i_093032),
    (0x093034, i_093034),
    (0x093038, i_093038),
    (0x093042, i_093042),
    (0x09304A, i_09304a),
    (0x09304C, i_09304c),
    (0x093050, i_093050),
    (0x09305A, i_09305a),
    (0x093062, i_093062),
    (0x093064, i_093064),
    (0x093068, i_093068),
    (0x0930B6, i_0930b6),
    (0x0930BE, i_0930be),
    (0x0930C0, i_0930c0),
    (0x0930C4, i_0930c4),
    (0x0930FA, i_0930fa),
    (0x093102, i_093102),
    (0x093104, i_093104),
    (0x093108, i_093108),
    (0x093118, i_093118),
    (0x093120, i_093120),
    (0x093122, i_093122),
    (0x093126, i_093126),
    (0x093130, i_093130),
    (0x093138, i_093138),
    (0x09313A, i_09313a),
    (0x09313E, i_09313e),
    (0x093148, i_093148),
    (0x093150, i_093150),
    (0x093152, i_093152),
    (0x093156, i_093156),
    (0x09315A, i_09315a),
    (0x093162, i_093162),
    (0x093164, i_093164),
    (0x093168, i_093168),
    (0x09317E, i_09317e),
    (0x093186, i_093186),
    (0x093188, i_093188),
    (0x09318C, i_09318c),
    (0x093190, i_093190),
    (0x093198, i_093198),
    (0x09319A, i_09319a),
    (0x09319E, i_09319e),
    (0x0934E2, i_0934e2),
    (0x0934EA, i_0934ea),
    (0x0934EC, i_0934ec),
    (0x0934F0, i_0934f0),
    (0x0934F4, i_0934f4),
    (0x0934FC, i_0934fc),
    (0x0934FE, i_0934fe),
    (0x093502, i_093502),
    (0x093512, i_093512),
    (0x09351A, i_09351a),
    (0x09351C, i_09351c),
    (0x093520, i_093520),
    (0x093524, i_093524),
    (0x09352C, i_09352c),
    (0x09352E, i_09352e),
    (0x093532, i_093532),
    (0x093798, i_093798),
    (0x0937A0, i_0937a0),
    (0x0937A2, i_0937a2),
    (0x0937A6, i_0937a6),
    (0x0937B0, i_0937b0),
    (0x0937B8, i_0937b8),
    (0x0937BA, i_0937ba),
    (0x0937BE, i_0937be),
    (0x0937C8, i_0937c8),
    (0x0937D0, i_0937d0),
    (0x0937D2, i_0937d2),
    (0x0937D6, i_0937d6),
    (0x0937EE, i_0937ee),
    (0x0937F6, i_0937f6),
    (0x0937F8, i_0937f8),
    (0x0937FC, i_0937fc),
    (0x093830, i_093830),
    (0x093838, i_093838),
    (0x09383A, i_09383a),
    (0x09383E, i_09383e),
    (0x09384E, i_09384e),
    (0x093856, i_093856),
    (0x093858, i_093858),
    (0x09385C, i_09385c),
    (0x093866, i_093866),
    (0x09386E, i_09386e),
    (0x093870, i_093870),
    (0x093874, i_093874),
    (0x09387E, i_09387e),
    (0x093886, i_093886),
    (0x093888, i_093888),
    (0x09388C, i_09388c),
    (0x093896, i_093896),
    (0x09389E, i_09389e),
    (0x0938A0, i_0938a0),
    (0x0938A4, i_0938a4),
    (0x0938AE, i_0938ae),
    (0x0938B6, i_0938b6),
    (0x0938BA, i_0938ba),
    (0x0938BE, i_0938be),
    (0x0938C6, i_0938c6),
    (0x0938C8, i_0938c8),
    (0x0938CC, i_0938cc),
    (0x09393C, i_09393c),
    (0x093944, i_093944),
    (0x093946, i_093946),
    (0x09394A, i_09394a),
    (0x093954, i_093954),
    (0x09395C, i_09395c),
    (0x09395E, i_09395e),
    (0x093962, i_093962),
    (0x093998, i_093998),
    (0x0939A0, i_0939a0),
    (0x0939A2, i_0939a2),
    (0x0939A6, i_0939a6),
    (0x0939DA, i_0939da),
    (0x0939E2, i_0939e2),
    (0x0939E4, i_0939e4),
    (0x0939E8, i_0939e8),
    (0x0939EC, i_0939ec),
    (0x0939F4, i_0939f4),
    (0x0939F6, i_0939f6),
    (0x0939FA, i_0939fa),
    (0x093A10, i_093a10),
    (0x093A18, i_093a18),
    (0x093A1A, i_093a1a),
    (0x093A1E, i_093a1e),
    (0x093A22, i_093a22),
    (0x093A2A, i_093a2a),
    (0x093A2C, i_093a2c),
    (0x093A30, i_093a30),
    (0x093A40, i_093a40),
    (0x093A48, i_093a48),
    (0x093A4A, i_093a4a),
    (0x093A4E, i_093a4e),
    (0x093A52, i_093a52),
    (0x093A5A, i_093a5a),
    (0x093A5C, i_093a5c),
    (0x093A60, i_093a60),
    (0x093A70, i_093a70),
    (0x093A78, i_093a78),
    (0x093A7A, i_093a7a),
    (0x093A7E, i_093a7e),
    (0x093A82, i_093a82),
    (0x093A8A, i_093a8a),
    (0x093A8C, i_093a8c),
    (0x093A90, i_093a90),
    (0x093AA0, i_093aa0),
    (0x093AA8, i_093aa8),
    (0x093AAA, i_093aaa),
    (0x093AAE, i_093aae),
    (0x093AB2, i_093ab2),
    (0x093ABA, i_093aba),
    (0x093ABC, i_093abc),
    (0x093AC0, i_093ac0),
    (0x093AD0, i_093ad0),
    (0x093AD8, i_093ad8),
    (0x093ADA, i_093ada),
    (0x093ADE, i_093ade),
    (0x093B14, i_093b14),
    (0x093B1C, i_093b1c),
    (0x093B1E, i_093b1e),
    (0x093B22, i_093b22),
    (0x093B32, i_093b32),
    (0x093B3A, i_093b3a),
    (0x093B3C, i_093b3c),
    (0x093B40, i_093b40),
    (0x093B4A, i_093b4a),
    (0x093B52, i_093b52),
    (0x093B54, i_093b54),
    (0x093B58, i_093b58),
    (0x093B70, i_093b70),
    (0x093B78, i_093b78),
    (0x093B7A, i_093b7a),
    (0x093B7E, i_093b7e),
    (0x093BB2, i_093bb2),
    (0x093BBA, i_093bba),
    (0x093BBC, i_093bbc),
    (0x093BC0, i_093bc0),
    (0x094514, i_094514),
    (0x09451C, i_09451c),
    (0x094520, i_094520),
    (0x09474A, i_09474a),
    (0x094752, i_094752),
    (0x094754, i_094754),
    (0x094758, i_094758),
    (0x09476A, i_09476a),
    (0x094772, i_094772),
    (0x094774, i_094774),
    (0x094778, i_094778),
    (0x09477C, i_09477c),
    (0x094784, i_094784),
    (0x094786, i_094786),
    (0x09478A, i_09478a),
    (0x0949BC, i_0949bc),
    (0x0949C4, i_0949c4),
    (0x0949C6, i_0949c6),
    (0x0949CA, i_0949ca),
    (0x0949DC, i_0949dc),
    (0x0949E4, i_0949e4),
    (0x0949E6, i_0949e6),
    (0x0949EA, i_0949ea),
    (0x0949EE, i_0949ee),
    (0x0949F6, i_0949f6),
    (0x0949F8, i_0949f8),
    (0x0949FC, i_0949fc),
    (0x094C30, i_094c30),
    (0x094C38, i_094c38),
    (0x094C3A, i_094c3a),
    (0x094C3E, i_094c3e),
    (0x094C48, i_094c48),
    (0x094C50, i_094c50),
    (0x094C52, i_094c52),
    (0x094C56, i_094c56),
    (0x094C8C, i_094c8c),
    (0x094C94, i_094c94),
    (0x094C96, i_094c96),
    (0x094C9A, i_094c9a),
    (0x094CCE, i_094cce),
    (0x094CD6, i_094cd6),
    (0x094CD8, i_094cd8),
    (0x094CDC, i_094cdc),
    (0x094D04, i_094d04),
    (0x094D0C, i_094d0c),
    (0x094D0E, i_094d0e),
    (0x094D12, i_094d12),
    (0x094D4A, i_094d4a),
    (0x094D52, i_094d52),
    (0x094D54, i_094d54),
    (0x094D58, i_094d58),
    (0x094D82, i_094d82),
    (0x094D8A, i_094d8a),
    (0x094D8C, i_094d8c),
    (0x094D90, i_094d90),
    (0x094DC4, i_094dc4),
    (0x094DCC, i_094dcc),
    (0x094DCE, i_094dce),
    (0x094DD2, i_094dd2),
    (0x094DFA, i_094dfa),
    (0x094E02, i_094e02),
    (0x094E04, i_094e04),
    (0x094E08, i_094e08),
    (0x094E3C, i_094e3c),
    (0x094E44, i_094e44),
    (0x094E46, i_094e46),
    (0x094E4A, i_094e4a),
    (0x094E5C, i_094e5c),
    (0x094E64, i_094e64),
    (0x094E66, i_094e66),
    (0x094E6A, i_094e6a),
    (0x094EA0, i_094ea0),
    (0x094EA8, i_094ea8),
    (0x094EAA, i_094eaa),
    (0x094EAE, i_094eae),
    (0x094EB6, i_094eb6),
    (0x094EBE, i_094ebe),
    (0x094EC0, i_094ec0),
    (0x094EC4, i_094ec4),
    (0x094EEE, i_094eee),
    (0x094EF6, i_094ef6),
    (0x094EF8, i_094ef8),
    (0x094EFC, i_094efc),
    (0x094F00, i_094f00),
    (0x094F0A, i_094f0a),
    (0x094F0E, i_094f0e),
    (0x094F12, i_094f12),
    (0x094F1A, i_094f1a),
    (0x094F1C, i_094f1c),
    (0x094F20, i_094f20),
    (0x094F56, i_094f56),
    (0x094F5E, i_094f5e),
    (0x094F60, i_094f60),
    (0x094F64, i_094f64),
    (0x094FBE, i_094fbe),
    (0x094FC6, i_094fc6),
    (0x094FC8, i_094fc8),
    (0x094FCC, i_094fcc),
    (0x094FD0, i_094fd0),
    (0x094FDA, i_094fda),
    (0x094FDE, i_094fde),
    (0x094FE2, i_094fe2),
    (0x094FEA, i_094fea),
    (0x094FEC, i_094fec),
    (0x094FF0, i_094ff0),
    (0x094FF8, i_094ff8),
    (0x095000, i_095000),
    (0x095002, i_095002),
    (0x095006, i_095006),
    (0x095010, i_095010),
    (0x095018, i_095018),
    (0x09501A, i_09501a),
    (0x09501E, i_09501e),
    (0x095052, i_095052),
    (0x09505A, i_09505a),
    (0x09505C, i_09505c),
    (0x095060, i_095060),
    (0x095072, i_095072),
    (0x09507A, i_09507a),
    (0x09507C, i_09507c),
    (0x095080, i_095080),
    (0x095084, i_095084),
    (0x09508C, i_09508c),
    (0x09508E, i_09508e),
    (0x095092, i_095092),
    (0x095306, i_095306),
    (0x09530E, i_09530e),
    (0x095310, i_095310),
    (0x095314, i_095314),
    (0x095348, i_095348),
    (0x095350, i_095350),
    (0x095352, i_095352),
    (0x095356, i_095356),
    (0x095368, i_095368),
    (0x095370, i_095370),
    (0x095372, i_095372),
    (0x095376, i_095376),
    (0x095388, i_095388),
    (0x095390, i_095390),
    (0x095392, i_095392),
    (0x095396, i_095396),
    (0x09539A, i_09539a),
    (0x0953A2, i_0953a2),
    (0x0953A4, i_0953a4),
    (0x0953A8, i_0953a8),
    (0x095634, i_095634),
    (0x09563C, i_09563c),
    (0x09563E, i_09563e),
    (0x095642, i_095642),
    (0x095646, i_095646),
    (0x09564E, i_09564e),
    (0x095652, i_095652),
    (0x095692, i_095692),
    (0x09569A, i_09569a),
    (0x09569E, i_09569e),
    (0x0958C8, i_0958c8),
    (0x0958D0, i_0958d0),
    (0x0958D2, i_0958d2),
    (0x0958D6, i_0958d6),
    (0x0958E8, i_0958e8),
    (0x0958F0, i_0958f0),
    (0x0958F2, i_0958f2),
    (0x0958F6, i_0958f6),
    (0x0958FA, i_0958fa),
    (0x095902, i_095902),
    (0x095904, i_095904),
    (0x095908, i_095908),
    (0x095B72, i_095b72),
    (0x095B7A, i_095b7a),
    (0x095B7C, i_095b7c),
    (0x095B80, i_095b80),
    (0x095BB6, i_095bb6),
    (0x095BBE, i_095bbe),
    (0x095BC0, i_095bc0),
    (0x095BC4, i_095bc4),
    (0x095BD6, i_095bd6),
    (0x095BDE, i_095bde),
    (0x095BE0, i_095be0),
    (0x095BE4, i_095be4),
    (0x095BE8, i_095be8),
    (0x095BF0, i_095bf0),
    (0x095BF2, i_095bf2),
    (0x095BF6, i_095bf6),
    (0x095E2A, i_095e2a),
    (0x095E32, i_095e32),
    (0x095E34, i_095e34),
    (0x095E38, i_095e38),
    (0x095E42, i_095e42),
    (0x095E4A, i_095e4a),
    (0x095E4C, i_095e4c),
    (0x095E50, i_095e50),
    (0x095E84, i_095e84),
    (0x095E8C, i_095e8c),
    (0x095E8E, i_095e8e),
    (0x095E92, i_095e92),
    (0x095EC8, i_095ec8),
    (0x095ED0, i_095ed0),
    (0x095ED2, i_095ed2),
    (0x095ED6, i_095ed6),
    (0x095F0A, i_095f0a),
    (0x095F12, i_095f12),
    (0x095F14, i_095f14),
    (0x095F18, i_095f18),
    (0x095F40, i_095f40),
    (0x095F48, i_095f48),
    (0x095F4A, i_095f4a),
    (0x095F4E, i_095f4e),
    (0x095F86, i_095f86),
    (0x095F8E, i_095f8e),
    (0x095F90, i_095f90),
    (0x095F94, i_095f94),
    (0x095FBE, i_095fbe),
    (0x095FC6, i_095fc6),
    (0x095FC8, i_095fc8),
    (0x095FCC, i_095fcc),
    (0x095FF4, i_095ff4),
    (0x095FFC, i_095ffc),
    (0x095FFE, i_095ffe),
    (0x096002, i_096002),
    (0x096014, i_096014),
    (0x09601C, i_09601c),
    (0x09601E, i_09601e),
    (0x096022, i_096022),
    (0x096026, i_096026),
    (0x09602E, i_09602e),
    (0x096030, i_096030),
    (0x096034, i_096034),
    (0x096266, i_096266),
    (0x09626E, i_09626e),
    (0x096270, i_096270),
    (0x096274, i_096274),
    (0x096286, i_096286),
    (0x09628E, i_09628e),
    (0x096290, i_096290),
    (0x096294, i_096294),
    (0x096298, i_096298),
    (0x0962A0, i_0962a0),
    (0x0962A2, i_0962a2),
    (0x0962A6, i_0962a6),
    (0x096532, i_096532),
    (0x09653A, i_09653a),
    (0x09653C, i_09653c),
    (0x096540, i_096540),
    (0x096544, i_096544),
    (0x09654C, i_09654c),
    (0x096550, i_096550),
    (0x0967A4, i_0967a4),
    (0x0967A8, i_0967a8),
    (0x0967AC, i_0967ac),
    (0x0967B0, i_0967b0),
    (0x0967B4, i_0967b4),
    (0x0967B8, i_0967b8),
    (0x0967BA, i_0967ba),
    (0x0967BE, i_0967be),
    (0x0967C0, i_0967c0),
    (0x0967C6, i_0967c6),
    (0x0967CC, i_0967cc),
    (0x0967CE, i_0967ce),
    (0x0967D0, i_0967d0),
    (0x0967D6, i_0967d6),
    (0x0967DC, i_0967dc),
    (0x0967DE, i_0967de),
    (0x0967E2, i_0967e2),
    (0x0967E4, i_0967e4),
    (0x0967E6, i_0967e6),
    (0x0967EA, i_0967ea),
    (0x0967EC, i_0967ec),
    (0x0967EE, i_0967ee),
    (0x0967F2, i_0967f2),
    (0x0967F4, i_0967f4),
    (0x0967F6, i_0967f6),
    (0x0967FA, i_0967fa),
    (0x0967FC, i_0967fc),
    (0x0967FE, i_0967fe),
    (0x096802, i_096802),
    (0x096806, i_096806),
    (0x09680C, i_09680c),
    (0x096814, i_096814),
    (0x09681A, i_09681a),
    (0x09681E, i_09681e),
    (0x096820, i_096820),
    (0x096824, i_096824),
    (0x09682A, i_09682a),
    (0x09682E, i_09682e),
    (0x096832, i_096832),
    (0x096836, i_096836),
    (0x09683E, i_09683e),
    (0x096840, i_096840),
    (0x096844, i_096844),
    (0x096848, i_096848),
    (0x09684E, i_09684e),
    (0x096856, i_096856),
    (0x09685C, i_09685c),
    (0x096860, i_096860),
    (0x096862, i_096862),
    (0x096866, i_096866),
    (0x09686C, i_09686c),
    (0x096870, i_096870),
    (0x096874, i_096874),
    (0x096878, i_096878),
    (0x096880, i_096880),
    (0x096882, i_096882),
    (0x096886, i_096886),
    (0x09688A, i_09688a),
    (0x096890, i_096890),
    (0x096898, i_096898),
    (0x09689E, i_09689e),
    (0x0968A2, i_0968a2),
    (0x0968A4, i_0968a4),
    (0x0968A8, i_0968a8),
    (0x0968AE, i_0968ae),
    (0x0968B2, i_0968b2),
    (0x0968B6, i_0968b6),
    (0x0968BA, i_0968ba),
    (0x0968C2, i_0968c2),
    (0x0968C4, i_0968c4),
    (0x0968C8, i_0968c8),
    (0x0968CC, i_0968cc),
    (0x0968D2, i_0968d2),
    (0x0968DA, i_0968da),
    (0x0968E0, i_0968e0),
    (0x0968E4, i_0968e4),
    (0x0968E6, i_0968e6),
    (0x0968EA, i_0968ea),
    (0x0968F0, i_0968f0),
    (0x0968F4, i_0968f4),
    (0x0968F8, i_0968f8),
    (0x0968FC, i_0968fc),
    (0x096904, i_096904),
    (0x096906, i_096906),
    (0x09690A, i_09690a),
    (0x09690E, i_09690e),
    (0x096914, i_096914),
    (0x09691C, i_09691c),
    (0x096922, i_096922),
    (0x096926, i_096926),
    (0x096928, i_096928),
    (0x09692C, i_09692c),
    (0x096932, i_096932),
    (0x096936, i_096936),
    (0x09693A, i_09693a),
    (0x09693E, i_09693e),
    (0x096946, i_096946),
    (0x096948, i_096948),
    (0x09694C, i_09694c),
    (0x096950, i_096950),
    (0x096956, i_096956),
    (0x09695E, i_09695e),
    (0x096964, i_096964),
    (0x096968, i_096968),
    (0x09696A, i_09696a),
    (0x09696E, i_09696e),
    (0x096974, i_096974),
    (0x096978, i_096978),
    (0x09697C, i_09697c),
    (0x096980, i_096980),
    (0x096988, i_096988),
    (0x09698A, i_09698a),
    (0x09698E, i_09698e),
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
    (0x0977C2, i_0977c2),
    (0x0977C6, i_0977c6),
    (0x0977CC, i_0977cc),
    (0x0977D2, i_0977d2),
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
    (0x097962, i_097962),
    (0x097968, i_097968),
    (0x09796E, i_09796e),
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
    (0x097D24, i_097d24),
    (0x097D28, i_097d28),
    (0x097D2A, i_097d2a),
    (0x097D2E, i_097d2e),
    (0x097D32, i_097d32),
    (0x097D36, i_097d36),
    (0x097D3A, i_097d3a),
    (0x097D3E, i_097d3e),
    (0x097D42, i_097d42),
    (0x097D46, i_097d46),
    (0x097D4E, i_097d4e),
    (0x097D52, i_097d52),
    (0x097D56, i_097d56),
    (0x097D5A, i_097d5a),
    (0x097D5E, i_097d5e),
    (0x097D62, i_097d62),
    (0x097D66, i_097d66),
    (0x097D6A, i_097d6a),
    (0x097D6C, i_097d6c),
    (0x097D72, i_097d72),
    (0x097D76, i_097d76),
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
    (0x098068, i_098068),
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
];
