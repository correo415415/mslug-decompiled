// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región bios_6) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $C15CFC: neg.w d5
pub fn i_c15cfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15CFE;
    cpu.ir = 0x4445;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C15CFE: dc.w $4f20
pub fn i_c15cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D00;
    cpu.ir = 0x4F20;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15D00: addq.w #$1, d1
pub fn i_c15d00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D02;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C15D02: dc.w $4d20
pub fn i_c15d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D04;
    cpu.ir = 0x4D20;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15D04: ?
pub fn i_c15d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D06;
    cpu.ir = 0x4552;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15D06: addq.w #$1, a7
pub fn i_c15d06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D08;
    cpu.ir = 0x524F;
    cpu.a[7] = cpu.a[7].wrapping_add(1);
    8
}
/// $C15D16: ori.b #$ff, d1
pub fn i_c15d16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D1A;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0xFF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15D1A: move.l d2, d0
pub fn i_c15d1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D1C;
    cpu.ir = 0x2002;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C15D1C: ori.b #$8a, d3
pub fn i_c15d1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D20;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x8A) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15D20: ori.b #$41, d7
pub fn i_c15d20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D24;
    cpu.ir = 0x7;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = (v1 | 0x41) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15D24: ?
pub fn i_c15d24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D26;
    cpu.ir = 0x4C45;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15D26: trap #$4
pub fn i_c15d26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D28;
    cpu.ir = 0x4E44;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0x90);
    34
}
/// $C15D28: ?
pub fn i_c15d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D2A;
    cpu.ir = 0x4152;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15D2A: movea.l d5, a0
pub fn i_c15d2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D2C;
    cpu.ir = 0x2045;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C15D2C: addq.w #$1, (a2)
pub fn i_c15d2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D2E;
    cpu.ir = 0x5252;
    let ad1 = cpu.a[2];
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C15D2E: ?
pub fn i_c15d2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D30;
    cpu.ir = 0x4F52;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15D30: dc.w $ffff
pub fn i_c15d30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D32;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C15D32: ori.b #$1, d0
pub fn i_c15d32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D36;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15D34: ori.b #$ff, d1
pub fn i_c15d34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D38;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0xFF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15D36: dc.w $f0ff
pub fn i_c15d36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D38;
    cpu.ir = 0xF0FF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C15D38: move.l d2, d0
pub fn i_c15d38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D3A;
    cpu.ir = 0x2002;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C15D3A: ori.b #$8a, d3
pub fn i_c15d3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D3E;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x8A) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15D3E: ori.b #$59, d7
pub fn i_c15d3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D42;
    cpu.ir = 0x7;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = (v1 | 0x59) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15D42: subq.w #$1, (a4)
pub fn i_c15d42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D44;
    cpu.ir = 0x5354;
    let ad1 = cpu.a[4];
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C15D44: ?
pub fn i_c15d44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D46;
    cpu.ir = 0x454D;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15D46: movea.l (a2), a0
pub fn i_c15d46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D48;
    cpu.ir = 0x2052;
    let v2 = bus.read32(cpu.a[2]);
    cpu.a[0] = v2;
    12
}
/// $C15D48: ?
pub fn i_c15d48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D4A;
    cpu.ir = 0x4F4D;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15D4A: movea.l d5, a0
pub fn i_c15d4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D4C;
    cpu.ir = 0x2045;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C15D4C: addq.w #$1, (a2)
pub fn i_c15d4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D4E;
    cpu.ir = 0x5252;
    let ad1 = cpu.a[2];
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C15D4E: ?
pub fn i_c15d4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D50;
    cpu.ir = 0x4F52;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15D50: dc.w $ffff
pub fn i_c15d50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D52;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C15D52: ori.b #$1, d0
pub fn i_c15d52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D56;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15D54: ori.b #$ff, d1
pub fn i_c15d54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D58;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0xFF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15D56: dc.w $f0ff
pub fn i_c15d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D58;
    cpu.ir = 0xF0FF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C15D58: move.l d2, d0
pub fn i_c15d58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D5A;
    cpu.ir = 0x2002;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C15D5A: ori.b #$8a, d3
pub fn i_c15d5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D5E;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x8A) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15D5E: ori.b #$45, d7
pub fn i_c15d5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D62;
    cpu.ir = 0x7;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = (v1 | 0x45) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15D62: ?
pub fn i_c15d62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D64;
    cpu.ir = 0x4D4F;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15D64: addq.w #$1, (a1)+
pub fn i_c15d64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D66;
    cpu.ir = 0x5259;
    let ad1 = cpu.a[1]; cpu.a[1] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C15D66: movea.l d3, a0
pub fn i_c15d66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D68;
    cpu.ir = 0x2043;
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C15D68: ?
pub fn i_c15d68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D6A;
    cpu.ir = 0x4152;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15D6A: neg.b -(a0)
pub fn i_c15d6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D6C;
    cpu.ir = 0x4420;
    let ad1 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.set(StatusRegister::V, v2 == 0x80); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write8(ad1, r3 as u8);
    10
}
/// $C15D6C: ?
pub fn i_c15d6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D6E;
    cpu.ir = 0x4552;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15D6E: addq.w #$1, a7
pub fn i_c15d6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D70;
    cpu.ir = 0x524F;
    cpu.a[7] = cpu.a[7].wrapping_add(1);
    8
}
/// $C15D74: ori.b #$ff, d1
pub fn i_c15d74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D78;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0xFF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15D78: move.l d2, d0
pub fn i_c15d78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D7A;
    cpu.ir = 0x2002;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C15D7A: ori.b #$8a, d3
pub fn i_c15d7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D7E;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x8A) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15D7E: ori.b #$38, d7
pub fn i_c15d7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D82;
    cpu.ir = 0x7;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = (v1 | 0x38) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15D82: move.w -(a0), d0
pub fn i_c15d82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D84;
    cpu.ir = 0x3020;
    let ad2 = cpu.a[0].wrapping_sub(2); cpu.a[0] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $C15D84: ?
pub fn i_c15d84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D86;
    cpu.ir = 0x4552;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15D86: addq.w #$1, a7
pub fn i_c15d86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D88;
    cpu.ir = 0x524F;
    cpu.a[7] = cpu.a[7].wrapping_add(1);
    8
}
/// $C15D8C: ?
pub fn i_c15d8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D8E;
    cpu.ir = 0x4144;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15D8E: neg.w (a2)
pub fn i_c15d8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D90;
    cpu.ir = 0x4452;
    let ad1 = cpu.a[2];
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C15D90: ?
pub fn i_c15d90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D92;
    cpu.ir = 0x4553;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15D92: subq.b #$1, -(a0)
pub fn i_c15d92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D94;
    cpu.ir = 0x5320;
    let ad1 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    10
}
/// $C15D94: movea.l (a7), a0
pub fn i_c15d94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D96;
    cpu.ir = 0x2057;
    let v2 = bus.read32(cpu.a[7]);
    cpu.a[0] = v2;
    12
}
/// $C15D96: addq.w #$1, a1
pub fn i_c15d96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D98;
    cpu.ir = 0x5249;
    cpu.a[1] = cpu.a[1].wrapping_add(1);
    8
}
/// $C15D98: addq.w #$2, d5
pub fn i_c15d98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D9A;
    cpu.ir = 0x5445;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 2, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C15D9A: movea.l (a2), a0
pub fn i_c15d9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D9C;
    cpu.ir = 0x2052;
    let v2 = bus.read32(cpu.a[2]);
    cpu.a[0] = v2;
    12
}
/// $C15D9C: ?
pub fn i_c15d9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15D9E;
    cpu.ir = 0x4541;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15D9E: ?
pub fn i_c15d9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15DA0;
    cpu.ir = 0x44FF;
    let v1: u32 = 0x0;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | (v1 as u16 & 0x1F);
    12
}
/// $C15DA0: ?
pub fn i_c15da0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15DA2;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15DA2: slt.b d0
pub fn i_c15da2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15DA4;
    cpu.ir = 0x5DC0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Lt) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $C15DA4: ?
pub fn i_c15da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15DA6;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15DA6: slt.b (a4)+
pub fn i_c15da6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15DA8;
    cpu.ir = 0x5DDC;
    let c1: u32 = if cpu.sr.evaluate(Condition::Lt) { 0xFF } else { 0 };
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    8
}
/// $C15DA8: ?
pub fn i_c15da8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15DAA;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15DAA: slt.b $c1.w
pub fn i_c15daa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15DAE;
    cpu.ir = 0x5DF8;
    let c1: u32 = if cpu.sr.evaluate(Condition::Lt) { 0xFF } else { 0 };
    let v2 = bus.read8(0xC1) as u32;
    bus.write8(0xC1, c1 as u8);
    12
}
/// $C15DAE: addq.b #$7, (a2)+
pub fn i_c15dae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15DB0;
    cpu.ir = 0x5E1A;
    let ad1 = cpu.a[2]; cpu.a[2] = ad1.wrapping_add(1);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 7, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    8
}
/// $C15DB0: ?
pub fn i_c15db0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15DB2;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15DB2: ?
pub fn i_c15db2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15DB6;
    cpu.ir = 0x5E3C;
    let v1: u32 = 0xC1;
    let r2 = add_with_flags(cpu, v1, 7, Size::Byte, true);
    4
}
/// $C15DB6: addq.w #$7, (a0)
pub fn i_c15db6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15DB8;
    cpu.ir = 0x5E50;
    let ad1 = cpu.a[0];
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 7, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C15DB8: ?
pub fn i_c15db8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15DBA;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15DBA: addq.w #$7, $c1(a2)
pub fn i_c15dba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15DBE;
    cpu.ir = 0x5E6A;
    let ad1 = cpu.a[2].wrapping_add(0xC1);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 7, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $C15DBE: addq.l #$7, (a0)
pub fn i_c15dbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15DC0;
    cpu.ir = 0x5E90;
    let ad1 = cpu.a[0];
    let v2 = bus.read32(ad1);
    let r3 = add_with_flags(cpu, v2, 7, Size::Long, true);
    bus.write32(ad1, r3);
    12
}
/// $C15DC0: ori.b #$ff, d1
pub fn i_c15dc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15DC4;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0xFF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15DC4: move.l d2, d0
pub fn i_c15dc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15DC6;
    cpu.ir = 0x2002;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C15DC6: ori.b #$84, d3
pub fn i_c15dc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15DCA;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15DCA: ori.b #$4f, d7
pub fn i_c15dca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15DCE;
    cpu.ir = 0x7;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = (v1 | 0x4F) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15DCE: trap #$9
pub fn i_c15dce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15DD0;
    cpu.ir = 0x4E49;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xA4);
    34
}
/// $C15DD0: addq.w #$2, a7
pub fn i_c15dd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15DD2;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $C15DD2: addq.b #$1, -(a0)
pub fn i_c15dd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15DD4;
    cpu.ir = 0x5220;
    let ad1 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    10
}
/// $C15DD4: ?
pub fn i_c15dd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15DD6;
    cpu.ir = 0x454E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15DD6: addq.w #$2, (a2)
pub fn i_c15dd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15DD8;
    cpu.ir = 0x5452;
    let ad1 = cpu.a[2];
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C15DDC: ori.b #$ff, d1
pub fn i_c15ddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15DE0;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0xFF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15DE0: move.l d2, d0
pub fn i_c15de0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15DE2;
    cpu.ir = 0x2002;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C15DE2: ori.b #$84, d3
pub fn i_c15de2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15DE6;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15DE6: ori.b #$44, d7
pub fn i_c15de6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15DEA;
    cpu.ir = 0x7;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = (v1 | 0x44) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15DEA: neg.w (a2)
pub fn i_c15dea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15DEC;
    cpu.ir = 0x4452;
    let ad1 = cpu.a[2];
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C15DEC: ?
pub fn i_c15dec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15DEE;
    cpu.ir = 0x4553;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15DEE: subq.b #$1, -(a0)
pub fn i_c15dee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15DF0;
    cpu.ir = 0x5320;
    let ad1 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    10
}
/// $C15DF0: ?
pub fn i_c15df0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15DF2;
    cpu.ir = 0x4552;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15DF2: addq.w #$1, a7
pub fn i_c15df2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15DF4;
    cpu.ir = 0x524F;
    cpu.a[7] = cpu.a[7].wrapping_add(1);
    8
}
/// $C15DF8: ori.b #$ff, d1
pub fn i_c15df8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15DFC;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0xFF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15DFC: move.l d2, d0
pub fn i_c15dfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15DFE;
    cpu.ir = 0x2002;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C15DFE: ori.b #$84, d3
pub fn i_c15dfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E02;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15E02: ori.b #$4c, d7
pub fn i_c15e02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E06;
    cpu.ir = 0x7;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = (v1 | 0x4C) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15E06: ?
pub fn i_c15e06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E08;
    cpu.ir = 0x4C45;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15E08: ?
pub fn i_c15e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E0A;
    cpu.ir = 0x4741;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15E0A: ?
pub fn i_c15e0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E0C;
    cpu.ir = 0x4C20;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15E0C: ?
pub fn i_c15e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E0E;
    cpu.ir = 0x494E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15E0E: subq.w #$1, (a4)
pub fn i_c15e0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E10;
    cpu.ir = 0x5354;
    let ad1 = cpu.a[4];
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C15E10: addq.w #$1, (a5)
pub fn i_c15e10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E12;
    cpu.ir = 0x5255;
    let ad1 = cpu.a[5];
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C15E12: ?
pub fn i_c15e12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E14;
    cpu.ir = 0x4354;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15E14: ?
pub fn i_c15e14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E16;
    cpu.ir = 0x494F;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15E16: ?
pub fn i_c15e16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E18;
    cpu.ir = 0x4EFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15E18: ori.b #$1, d0
pub fn i_c15e18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E1C;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15E1A: ori.b #$ff, d1
pub fn i_c15e1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E1E;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0xFF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15E1C: dc.w $f0ff
pub fn i_c15e1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E1E;
    cpu.ir = 0xF0FF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C15E1E: move.l d2, d0
pub fn i_c15e1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E20;
    cpu.ir = 0x2002;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C15E20: ori.b #$84, d3
pub fn i_c15e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E24;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15E24: ori.b #$52, d7
pub fn i_c15e24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E28;
    cpu.ir = 0x7;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = (v1 | 0x52) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15E28: ?
pub fn i_c15e28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E2A;
    cpu.ir = 0x4956;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15E2A: ?
pub fn i_c15e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E2C;
    cpu.ir = 0x494C;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15E2C: ?
pub fn i_c15e2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E2E;
    cpu.ir = 0x4547;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15E2E: dc.w $4520
pub fn i_c15e2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E30;
    cpu.ir = 0x4520;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15E30: addq.w #$3, a1
pub fn i_c15e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E32;
    cpu.ir = 0x5649;
    cpu.a[1] = cpu.a[1].wrapping_add(3);
    8
}
/// $C15E32: ?
pub fn i_c15e32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E34;
    cpu.ir = 0x4F4C;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15E34: ?
pub fn i_c15e34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E36;
    cpu.ir = 0x4154;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15E36: ?
pub fn i_c15e36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E38;
    cpu.ir = 0x494F;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15E38: ?
pub fn i_c15e38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E3A;
    cpu.ir = 0x4EFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15E3A: ori.b #$1, d0
pub fn i_c15e3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E3E;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15E3C: ori.b #$ff, d1
pub fn i_c15e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E40;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0xFF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15E3E: dc.w $f0ff
pub fn i_c15e3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E40;
    cpu.ir = 0xF0FF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C15E40: move.l d2, d0
pub fn i_c15e40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E42;
    cpu.ir = 0x2002;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C15E42: ori.b #$84, d3
pub fn i_c15e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E46;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15E46: ori.b #$52, d7
pub fn i_c15e46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E4A;
    cpu.ir = 0x7;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = (v1 | 0x52) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15E4A: ?
pub fn i_c15e4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E4C;
    cpu.ir = 0x4143;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15E50: ori.b #$ff, d1
pub fn i_c15e50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E54;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0xFF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15E54: move.l d2, d0
pub fn i_c15e54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E56;
    cpu.ir = 0x2002;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C15E56: ori.b #$84, d3
pub fn i_c15e56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E5A;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15E5A: ori.b #$4e, d7
pub fn i_c15e5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E5E;
    cpu.ir = 0x7;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = (v1 | 0x4E) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15E5E: ?
pub fn i_c15e5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E60;
    cpu.ir = 0x4153;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15E60: subq.w #$1, a1
pub fn i_c15e60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E62;
    cpu.ir = 0x5349;
    cpu.a[1] = cpu.a[1].wrapping_sub(1);
    8
}
/// $C15E62: ?
pub fn i_c15e62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E64;
    cpu.ir = 0x474E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15E64: ?
pub fn i_c15e64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E66;
    cpu.ir = 0x4544;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15E66: dc.w $ffff
pub fn i_c15e66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E68;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C15E68: ori.b #$1, d0
pub fn i_c15e68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E6C;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15E6A: ori.b #$ff, d1
pub fn i_c15e6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E6E;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0xFF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15E6C: dc.w $f0ff
pub fn i_c15e6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E6E;
    cpu.ir = 0xF0FF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C15E6E: move.l d2, d0
pub fn i_c15e6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E70;
    cpu.ir = 0x2002;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C15E70: ori.b #$84, d3
pub fn i_c15e70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E74;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15E74: ori.b #$4e, d7
pub fn i_c15e74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E78;
    cpu.ir = 0x7;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = (v1 | 0x4E) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15E78: ?
pub fn i_c15e78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E7A;
    cpu.ir = 0x494E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15E7A: ?
pub fn i_c15e7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E7C;
    cpu.ir = 0x4954;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15E7C: ?
pub fn i_c15e7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E7E;
    cpu.ir = 0x4941;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15E7E: ?
pub fn i_c15e7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E80;
    cpu.ir = 0x4C49;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15E80: addq.w #$5, d5
pub fn i_c15e80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E82;
    cpu.ir = 0x5A45;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 5, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C15E82: neg.b -(a0)
pub fn i_c15e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E84;
    cpu.ir = 0x4420;
    let ad1 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.set(StatusRegister::V, v2 == 0x80); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write8(ad1, r3 as u8);
    10
}
/// $C15E84: ?
pub fn i_c15e84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E86;
    cpu.ir = 0x494E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15E86: addq.w #$2, d5
pub fn i_c15e86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E88;
    cpu.ir = 0x5445;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 2, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C15E88: addq.w #$1, (a2)
pub fn i_c15e88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E8A;
    cpu.ir = 0x5252;
    let ad1 = cpu.a[2];
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C15E8A: subq.w #$2, (a0)
pub fn i_c15e8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E8C;
    cpu.ir = 0x5550;
    let ad1 = cpu.a[0];
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C15E90: ori.b #$ff, d1
pub fn i_c15e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E94;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0xFF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15E94: move.l d2, d0
pub fn i_c15e94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E96;
    cpu.ir = 0x2002;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C15E96: ori.b #$84, d3
pub fn i_c15e96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E9A;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x84) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15E9A: ori.b #$50, d7
pub fn i_c15e9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15E9E;
    cpu.ir = 0x7;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = (v1 | 0x50) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15E9E: subq.w #$2, (a2)
pub fn i_c15e9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15EA0;
    cpu.ir = 0x5552;
    let ad1 = cpu.a[2];
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C15EA0: ?
pub fn i_c15ea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15EA2;
    cpu.ir = 0x494F;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15EA2: subq.w #$2, (a3)
pub fn i_c15ea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15EA4;
    cpu.ir = 0x5553;
    let ad1 = cpu.a[3];
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C15EA4: movea.l a1, a0
pub fn i_c15ea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15EA6;
    cpu.ir = 0x2049;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C15EA6: link.w a4, #$4552
pub fn i_c15ea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15EAA;
    cpu.ir = 0x4E54;
    cpu.push32(bus, cpu.a[4]); cpu.a[4] = cpu.a[7]; cpu.a[7] = cpu.a[7].wrapping_add(0x4552);
    16
}
/// $C15EAA: addq.w #$1, (a5)
pub fn i_c15eaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15EAC;
    cpu.ir = 0x5255;
    let ad1 = cpu.a[5];
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C15EAC: addq.w #$8, (a4)
pub fn i_c15eac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15EAE;
    cpu.ir = 0x5054;
    let ad1 = cpu.a[4];
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C15EAE: dc.w $ffff
pub fn i_c15eae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15EB0;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C15EB0: ori.b #$1, d0
pub fn i_c15eb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15EB4;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15EB2: ori.b #$ff, d1
pub fn i_c15eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15EB6;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0xFF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15EB4: dc.w $f0ff
pub fn i_c15eb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15EB6;
    cpu.ir = 0xF0FF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C15EB6: move.l d2, d0
pub fn i_c15eb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15EB8;
    cpu.ir = 0x2002;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C15EB8: ori.b #$8d, d3
pub fn i_c15eb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15EBC;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x8D) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15EBC: ori.b #$49, d7
pub fn i_c15ebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15EC0;
    cpu.ir = 0x7;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = (v1 | 0x49) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15EC0: subq.w #$1, (a0)
pub fn i_c15ec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15EC2;
    cpu.ir = 0x5350;
    let ad1 = cpu.a[0];
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C15EC2: ?
pub fn i_c15ec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15EC4;
    cpu.ir = 0x4C41;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15EC4: subq.b #$4, -(a0)
pub fn i_c15ec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15EC6;
    cpu.ir = 0x5920;
    let ad1 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 4, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    10
}
/// $C15EC6: ?
pub fn i_c15ec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15EC8;
    cpu.ir = 0x4D45;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15EC8: ?
pub fn i_c15ec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15ECA;
    cpu.ir = 0x4D4F;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15ECA: addq.w #$1, (a1)+
pub fn i_c15eca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15ECC;
    cpu.ir = 0x5259;
    let ad1 = cpu.a[1]; cpu.a[1] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C15ECC: dc.w $ffff
pub fn i_c15ecc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15ECE;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C15ECE: ori.b #$1, d0
pub fn i_c15ece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15ED2;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15ED0: ori.b #$0, d1
pub fn i_c15ed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15ED4;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15ED2: dc.w $f000
pub fn i_c15ed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15ED4;
    cpu.ir = 0xF000;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C15ED4: move.l d2, d0
pub fn i_c15ed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15ED6;
    cpu.ir = 0x2002;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C15ED6: ori.b #$8d, d3
pub fn i_c15ed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15EDA;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x8D) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15EDA: ori.b #$49, d7
pub fn i_c15eda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15EDE;
    cpu.ir = 0x7;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = (v1 | 0x49) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15EDE: subq.w #$1, (a0)
pub fn i_c15ede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15EE0;
    cpu.ir = 0x5350;
    let ad1 = cpu.a[0];
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C15EE0: ?
pub fn i_c15ee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15EE2;
    cpu.ir = 0x4C41;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15EE2: subq.b #$4, -(a0)
pub fn i_c15ee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15EE4;
    cpu.ir = 0x5920;
    let ad1 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 4, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    10
}
/// $C15EE4: addq.b #$3, $5241(a5)
pub fn i_c15ee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15EE8;
    cpu.ir = 0x562D;
    let ad1 = cpu.a[5].wrapping_add(0x5241);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 3, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C15EF8: ori.b #$0, d1
pub fn i_c15ef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15EFC;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15EFC: move.l d2, d0
pub fn i_c15efc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15EFE;
    cpu.ir = 0x2002;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C15EFE: ori.b #$8d, d3
pub fn i_c15efe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F02;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x8D) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15F02: ori.b #$4f, d7
pub fn i_c15f02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F06;
    cpu.ir = 0x7;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = (v1 | 0x4F) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15F06: ?
pub fn i_c15f06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F08;
    cpu.ir = 0x4449;
    let v1 = cpu.a[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.a[1] = r2 as i16 as i32 as u32;
    4
}
/// $C15F08: not.w (a1)+
pub fn i_c15f08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F0A;
    cpu.ir = 0x4659;
    let ad1 = cpu.a[1]; cpu.a[1] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    let r3 = !v2 & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C15F0A: movea.l a5, a0
pub fn i_c15f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F0C;
    cpu.ir = 0x204D;
    let v1 = cpu.a[5] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C15F0C: ?
pub fn i_c15f0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F0E;
    cpu.ir = 0x454D;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15F0E: ?
pub fn i_c15f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F10;
    cpu.ir = 0x4F52;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15F10: subq.b #$4, d0
pub fn i_c15f10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F12;
    cpu.ir = 0x5900;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 4, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C15F12: ori.b #$4e, d3
pub fn i_c15f12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F16;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x4E) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15F16: ori.b #$54, d7
pub fn i_c15f16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F1A;
    cpu.ir = 0x7;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = (v1 | 0x54) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15F1A: dc.w $4f20
pub fn i_c15f1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F1C;
    cpu.ir = 0x4F20;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15F1C: ?
pub fn i_c15f1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F1E;
    cpu.ir = 0xFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15F1E: ori.b #$1, d0
pub fn i_c15f1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F22;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15F20: ori.b #$0, d1
pub fn i_c15f20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F24;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15F22: dc.w $f000
pub fn i_c15f22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F24;
    cpu.ir = 0xF000;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C15F24: move.l d2, d0
pub fn i_c15f24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F26;
    cpu.ir = 0x2002;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C15F26: ori.b #$8d, d3
pub fn i_c15f26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F2A;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x8D) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15F2A: ori.b #$4f, d7
pub fn i_c15f2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F2E;
    cpu.ir = 0x7;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = (v1 | 0x4F) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15F2E: ?
pub fn i_c15f2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F30;
    cpu.ir = 0x4449;
    let v1 = cpu.a[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.a[1] = r2 as i16 as i32 as u32;
    4
}
/// $C15F30: not.w (a1)+
pub fn i_c15f30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F32;
    cpu.ir = 0x4659;
    let ad1 = cpu.a[1]; cpu.a[1] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    let r3 = !v2 & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C15F32: movea.l (a6), a0
pub fn i_c15f32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F34;
    cpu.ir = 0x2056;
    let v2 = bus.read32(cpu.a[6]);
    cpu.a[0] = v2;
    12
}
/// $C15F34: move.l (a2), $414d(a6)
pub fn i_c15f34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F38;
    cpu.ir = 0x2D52;
    let v2 = bus.read32(cpu.a[2]);
    let ad3 = cpu.a[6].wrapping_add(0x414D);
    bus.write32(ad3, v2);
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C15F38: dc.w $ff00
pub fn i_c15f38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F3A;
    cpu.ir = 0xFF00;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C15F3A: ori.b #$ce, d3
pub fn i_c15f3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F3E;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0xCE) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15F3E: ori.b #$ff, d7
pub fn i_c15f3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F42;
    cpu.ir = 0x7;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = (v1 | 0xFF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15F42: ?
pub fn i_c15f42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F44;
    cpu.ir = 0xFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15F44: ori.b #$1, d0
pub fn i_c15f44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F48;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15F46: ori.b #$0, d1
pub fn i_c15f46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F4A;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15F48: dc.w $f000
pub fn i_c15f48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F4A;
    cpu.ir = 0xF000;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C15F4A: move.l d2, d0
pub fn i_c15f4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F4C;
    cpu.ir = 0x2002;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C15F4C: ori.b #$8d, d3
pub fn i_c15f4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F50;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x8D) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15F50: ori.b #$4f, d7
pub fn i_c15f50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F54;
    cpu.ir = 0x7;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = (v1 | 0x4F) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15F54: ?
pub fn i_c15f54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F56;
    cpu.ir = 0x4449;
    let v1 = cpu.a[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.a[1] = r2 as i16 as i32 as u32;
    4
}
/// $C15F56: not.w (a1)+
pub fn i_c15f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F58;
    cpu.ir = 0x4659;
    let ad1 = cpu.a[1]; cpu.a[1] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    let r3 = !v2 & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C15F58: movea.l (a2), a0
pub fn i_c15f58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F5A;
    cpu.ir = 0x2052;
    let v2 = bus.read32(cpu.a[2]);
    cpu.a[0] = v2;
    12
}
/// $C15F5A: ?
pub fn i_c15f5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F5C;
    cpu.ir = 0x4547;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15F5C: ?
pub fn i_c15f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F5E;
    cpu.ir = 0x4953;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15F5E: addq.w #$2, d5
pub fn i_c15f5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F60;
    cpu.ir = 0x5445;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 2, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C15F60: addq.b #$1, d0
pub fn i_c15f60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F62;
    cpu.ir = 0x5200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C15F62: ori.b #$8e, d3
pub fn i_c15f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F66;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x8E) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15F66: ori.b #$20, d7
pub fn i_c15f66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F6A;
    cpu.ir = 0x7;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = (v1 | 0x20) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15F6A: move.l -(a0), d0
pub fn i_c15f6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F6C;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C15F6C: move.l -(a0), d0
pub fn i_c15f6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F6E;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C15F6E: addq.w #$2, a7
pub fn i_c15f6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F70;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $C15F70: move.l -(a0), d0
pub fn i_c15f70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F72;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C15F72: move.l -(a0), d0
pub fn i_c15f72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F74;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C15F74: move.l -(a0), d0
pub fn i_c15f74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F76;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C15F76: ?
pub fn i_c15f76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F78;
    cpu.ir = 0xFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15F78: ori.b #$1, d0
pub fn i_c15f78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F7C;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15F7A: ori.b #$0, d1
pub fn i_c15f7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F7E;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15F7C: dc.w $f000
pub fn i_c15f7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F7E;
    cpu.ir = 0xF000;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C15F7E: move.l d2, d0
pub fn i_c15f7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F80;
    cpu.ir = 0x2002;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C15F80: ori.b #$8d, d3
pub fn i_c15f80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F84;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x8D) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15F84: ori.b #$52, d7
pub fn i_c15f84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F88;
    cpu.ir = 0x7;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = (v1 | 0x52) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15F88: ?
pub fn i_c15f88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15F8A;
    cpu.ir = 0x4143;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15FB2: ori.b #$0, d1
pub fn i_c15fb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15FB6;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15FB6: move.l d2, d0
pub fn i_c15fb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15FB8;
    cpu.ir = 0x2002;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C15FB8: ori.b #$8d, d3
pub fn i_c15fb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15FBC;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x8D) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15FBC: ori.b #$55, d7
pub fn i_c15fbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15FC0;
    cpu.ir = 0x7;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = (v1 | 0x55) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15FC0: ?
pub fn i_c15fc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15FC2;
    cpu.ir = 0x4EFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15FC2: dc.w $ff00
pub fn i_c15fc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15FC4;
    cpu.ir = 0xFF00;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C15FC4: ori.b #$1, d0
pub fn i_c15fc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15FC8;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15FC6: ori.b #$0, d1
pub fn i_c15fc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15FCA;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15FC8: dc.w $f000
pub fn i_c15fc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15FCA;
    cpu.ir = 0xF000;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C15FCA: move.l d2, d0
pub fn i_c15fca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15FCC;
    cpu.ir = 0x2002;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C15FCC: ori.b #$70, d3
pub fn i_c15fcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15FD0;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x70) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15FD0: ori.b #$c1, d4
pub fn i_c15fd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15FD4;
    cpu.ir = 0x4;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 | 0xC1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15FD4: bra.b $c15ff4
pub fn i_c15fd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15FD6;
    cpu.ir = 0x601E;
    cpu.pc = 0xC15FF4;
    10
}
/// $C15FD6: ori.b #$1, d5
pub fn i_c15fd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15FDA;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15FDA: ori.b #$c1, d4
pub fn i_c15fda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15FDE;
    cpu.ir = 0x4;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 | 0xC1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15FDE: bra.b $c15ffe
pub fn i_c15fde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15FE0;
    cpu.ir = 0x601E;
    cpu.pc = 0xC15FFE;
    10
}
/// $C15FE0: ori.b #$1, d5
pub fn i_c15fe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15FE4;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15FE4: ori.b #$c1, d4
pub fn i_c15fe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15FE8;
    cpu.ir = 0x4;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 | 0xC1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15FE8: bra.b $c16008
pub fn i_c15fe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15FEA;
    cpu.ir = 0x601E;
    cpu.pc = 0xC16008;
    10
}
/// $C15FEA: ori.b #$1, d5
pub fn i_c15fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15FEE;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15FEE: ori.b #$c1, d4
pub fn i_c15fee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15FF2;
    cpu.ir = 0x4;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 | 0xC1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15FF2: bra.b $c16012
pub fn i_c15ff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15FF4;
    cpu.ir = 0x601E;
    cpu.pc = 0xC16012;
    10
}
/// $C15FF4: ori.b #$1, d5
pub fn i_c15ff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15FF8;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15FF8: ori.b #$c1, d4
pub fn i_c15ff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15FFC;
    cpu.ir = 0x4;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 | 0xC1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15FFC: bra.b $c1601c
pub fn i_c15ffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15FFE;
    cpu.ir = 0x601E;
    cpu.pc = 0xC1601C;
    10
}
/// $C15FFE: ori.b #$1, d5
pub fn i_c15ffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16002;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C16002: ori.b #$c1, d4
pub fn i_c16002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16006;
    cpu.ir = 0x4;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 | 0xC1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C16006: bra.b $c16026
pub fn i_c16006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16008;
    cpu.ir = 0x601E;
    cpu.pc = 0xC16026;
    10
}
/// $C16008: ori.b #$1, d5
pub fn i_c16008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1600C;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1600C: ori.b #$c1, d4
pub fn i_c1600c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16010;
    cpu.ir = 0x4;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 | 0xC1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C16010: bra.b $c16030
pub fn i_c16010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16012;
    cpu.ir = 0x601E;
    cpu.pc = 0xC16030;
    10
}
/// $C16012: ori.b #$1, d5
pub fn i_c16012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16016;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C16016: ori.b #$c1, d4
pub fn i_c16016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1601A;
    cpu.ir = 0x4;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 | 0xC1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1601A: bra.b $c1603a
pub fn i_c1601a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1601C;
    cpu.ir = 0x601E;
    cpu.pc = 0xC1603A;
    10
}
/// $C1601C: ori.b #$ff, d0
pub fn i_c1601c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16020;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0xFF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1601E: dc.w $ffff
pub fn i_c1601e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16020;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C16020: dc.w $ffff
pub fn i_c16020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16022;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C16022: dc.w $ffff
pub fn i_c16022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16024;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C16024: dc.w $ffff
pub fn i_c16024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16026;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C16026: dc.w $ffff
pub fn i_c16026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16028;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C16028: dc.w $ffff
pub fn i_c16028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1602A;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C1602A: dc.w $ffff
pub fn i_c1602a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1602C;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C1602C: dc.w $ffff
pub fn i_c1602c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1602E;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C1602E: dc.w $ffff
pub fn i_c1602e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16030;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C16030: dc.w $ff00
pub fn i_c16030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16032;
    cpu.ir = 0xFF00;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C16032: ori.b #$fc, (a6, d2.w * 8)
pub fn i_c16032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16038;
    cpu.ir = 0x36;
    let ad1 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0xFC) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    18
}
/// $C16034: move.w #$2700, sr
pub fn i_c16034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16038;
    cpu.ir = 0x46FC;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let v1: u32 = 0x2700;
    write_sr(cpu, v1 as u16);
    12
}
/// $C16038: move.b d0, $3a0003.l
pub fn i_c16038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1603E;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0003, v1 as u8);
    16
}
/// $C1603A: ?
pub fn i_c1603a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16040;
    cpu.ir = 0x3A;
    let v2 = bus.read8(0xC173FE) as u32;
    let r3 = (v2 | 0x3) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $C1603E: move.b d0, $3a000b.l
pub fn i_c1603e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16044;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A000B, v1 as u8);
    16
}
/// $C16040: ?
pub fn i_c16040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16046;
    cpu.ir = 0x3A;
    let v2 = bus.read8(0xC1D044) as u32;
    let r3 = (v2 | 0xB) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $C16044: moveq #$0, d0
pub fn i_c16044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16046;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C16046: move.l d0, d1
pub fn i_c16046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16048;
    cpu.ir = 0x2200;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[1] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C16048: move.l d0, d2
pub fn i_c16048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1604A;
    cpu.ir = 0x2400;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[2] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1604A: move.l d0, d3
pub fn i_c1604a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1604C;
    cpu.ir = 0x2600;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[3] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1604C: lea.l $110000.l, a0
pub fn i_c1604c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16052;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x110000;
    20
}
/// $C16052: move.w #$1ff, d7
pub fn i_c16052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16056;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x1FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16056: move.b d0, $300001.l
pub fn i_c16056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1605C;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C1605C: movem.l d0-d3, -(a0)
pub fn i_c1605c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16060;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C16060: movem.l d0-d3, -(a0)
pub fn i_c16060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16064;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C16064: movem.l d0-d3, -(a0)
pub fn i_c16064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16068;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C16068: movem.l d0-d3, -(a0)
pub fn i_c16068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1606C;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C1606C: movem.l d0-d3, -(a0)
pub fn i_c1606c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16070;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C16070: movem.l d0-d3, -(a0)
pub fn i_c16070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16074;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C16074: movem.l d0-d3, -(a0)
pub fn i_c16074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16078;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C16078: movem.l d0-d3, -(a0)
pub fn i_c16078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1607C;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C1607C: dbra d7, $c16056
pub fn i_c1607c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16080;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC16080; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC16080; return 14; }
    cpu.pc = 0xC16056;
    10
}
/// $C16080: move.b d0, $3a000f.l
pub fn i_c16080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16086;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A000F, v1 as u8);
    16
}
/// $C16086: lea.l $402000.l, a0
pub fn i_c16086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1608C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x402000;
    20
}
/// $C1608C: move.w #$7f, d7
pub fn i_c1608c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16090;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x7F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16090: movem.l d0-d3, -(a0)
pub fn i_c16090(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16094;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C16094: movem.l d0-d3, -(a0)
pub fn i_c16094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16098;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C16098: movem.l d0-d3, -(a0)
pub fn i_c16098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1609C;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C1609C: movem.l d0-d3, -(a0)
pub fn i_c1609c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC160A0;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C160A0: dbra d7, $c16090
pub fn i_c160a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC160A4;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC160A4; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC160A4; return 14; }
    cpu.pc = 0xC16090;
    10
}
/// $C160A4: move.b d0, $300001.l
pub fn i_c160a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC160AA;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C160AA: move.b d0, $3a001f.l
pub fn i_c160aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC160B0;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001F, v1 as u8);
    16
}
/// $C160B0: lea.l $402000.l, a0
pub fn i_c160b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC160B6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x402000;
    20
}
/// $C160B6: move.w #$7f, d7
pub fn i_c160b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC160BA;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x7F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C160BA: movem.l d0-d3, -(a0)
pub fn i_c160ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC160BE;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C160BE: movem.l d0-d3, -(a0)
pub fn i_c160be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC160C2;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C160C2: movem.l d0-d3, -(a0)
pub fn i_c160c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC160C6;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C160C6: movem.l d0-d3, -(a0)
pub fn i_c160c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC160CA;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C160CA: dbra d7, $c160ba
pub fn i_c160ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC160CE;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC160CE; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC160CE; return 14; }
    cpu.pc = 0xC160BA;
    10
}
/// $C160CE: lea.l $10f300.l, a5
pub fn i_c160ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC160D4;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x10F300;
    20
}
/// $C160D4: move.b #$ff, $b8d(a5)
pub fn i_c160d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC160DA;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xB8D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C160DA: move.b #$1, $bbc(a5)
pub fn i_c160da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC160E0;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xBBC);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C160E0: move.b $c00401.l, $a83(a5)
pub fn i_c160e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC160E8;
    cpu.ir = 0x1B79;
    let v2 = bus.read8(0xC00401) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[5].wrapping_add(0xA83);
    bus.write8(ad3, v2 as u8);
    24
}
/// $C160E8: move.b #$80, $10fd82.l
pub fn i_c160e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC160F0;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10FD82, v1 as u8);
    16
}
/// $C160F0: move.l #$10ff00, $abe(a5)
pub fn i_c160f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC160F8;
    cpu.ir = 0x2B7C;
    let v1: u32 = 0x10FF00;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C160F8: move.b d0, $3a0001.l
pub fn i_c160f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC160FE;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0001, v1 as u8);
    16
}
/// $C160FE: move.w #$7, $3c000c.l
pub fn i_c160fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16106;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C000C, v1 as u16);
    16
}
/// $C16106: move.w #$2000, sr
pub fn i_c16106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1610A;
    cpu.ir = 0x46FC;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let v1: u32 = 0x2000;
    write_sr(cpu, v1 as u16);
    12
}
/// $C1610A: tst.b $d0005e.l
pub fn i_c1610a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16110;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0xD0005E) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $C16110: bmi.b $c1611c
pub fn i_c16110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16112;
    cpu.ir = 0x6B0A;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0xC1611C; return 10; }
    8
}
/// $C16112: move.w #$c, $9d8(a5)
pub fn i_c16112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16118;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16118: clr.w $ae4(a5)
pub fn i_c16118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1611C;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0xAE4);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $C1611C: jsr $c1727e(pc)
pub fn i_c1611c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16120;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16120);
    cpu.pc = 0xC1727E;
    16
}
/// $C16120: cmpi.w #$2, $9d8(a5)
pub fn i_c16120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16126;
    cpu.ir = 0xC6D;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Word, false);
    16
}
/// $C16126: bcs.w $c16130
pub fn i_c16126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1612A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC16130; return 10; }
    8
}
/// $C1612A: jsr $c17f40.l
pub fn i_c1612a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16130;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0xC16130);
    cpu.pc = 0xC17F40;
    16
}
/// $C16130: lea.l $10f300.l, a5
pub fn i_c16130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16136;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x10F300;
    20
}
/// $C16136: move.w $9d8(a5), d0
pub fn i_c16136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1613A;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C1613A: add.w d0, d0
pub fn i_c1613a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1613C;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C1613C: add.w d0, d0
pub fn i_c1613c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1613E;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C1613E: movea.l $c16146(pc, d0.w), a0
pub fn i_c1613e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16142;
    cpu.ir = 0x207B;
    let ad2 = 0xC16146_u32.wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $C16142: jsr (a0)
pub fn i_c16142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16144;
    cpu.ir = 0x4E90;
    cpu.push32(bus, 0xC16144);
    cpu.jump_to(cpu.a[0]);
    16
}
/// $C16144: bra.b $c1611c
pub fn i_c16144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16146;
    cpu.ir = 0x60D6;
    cpu.pc = 0xC1611C;
    10
}
/// $C16146: ?
pub fn i_c16146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16148;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C16148: bsr.b $c161c8
pub fn i_c16148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1614A;
    cpu.ir = 0x617E;
    cpu.push32(bus, 0xC1614A);
    cpu.pc = 0xC161C8;
    18
}
/// $C1614A: ?
pub fn i_c1614a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1614C;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1614C: bhi.b $c1618a
pub fn i_c1614c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1614E;
    cpu.ir = 0x623C;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0xC1618A; return 10; }
    8
}
/// $C1614E: ?
pub fn i_c1614e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16150;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C16150: bhi.b $c1619e
pub fn i_c16150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16152;
    cpu.ir = 0x624C;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0xC1619E; return 10; }
    8
}
/// $C16152: ?
pub fn i_c16152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16154;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C16154: bhi.b $c160f0
pub fn i_c16154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16156;
    cpu.ir = 0x629A;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0xC160F0; return 10; }
    8
}
/// $C16156: ?
pub fn i_c16156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16158;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C16158: bls.b $c161d6
pub fn i_c16158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1615A;
    cpu.ir = 0x637C;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0xC161D6; return 10; }
    8
}
/// $C1615A: ?
pub fn i_c1615a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1615C;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1615C: bls.b $c1614e
pub fn i_c1615c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1615E;
    cpu.ir = 0x63F0;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0xC1614E; return 10; }
    8
}
/// $C1615E: ?
pub fn i_c1615e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16160;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1617E: move.l #$eee0000, $400002.l
pub fn i_c1617e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16188;
    cpu.ir = 0x23FC;
    let v1: u32 = 0xEEE0000;
    bus.write32(0x400002, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C16188: move.l #$e0000, $400022.l
pub fn i_c16188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16192;
    cpu.ir = 0x23FC;
    let v1: u32 = 0xE0000;
    bus.write32(0x400022, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C1618A: ?
pub fn i_c1618a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1618E;
    cpu.ir = 0xE;
    let v1 = cpu.a[6] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.a[6] = r2;
    8
}
/// $C1618E: ori.w #$22, d0
pub fn i_c1618e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16192;
    cpu.ir = 0x40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 | 0x22) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C16192: move.l #$e0000, $4001ea.l
pub fn i_c16192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1619C;
    cpu.ir = 0x23FC;
    let v1: u32 = 0xE0000;
    bus.write32(0x4001EA, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C1619C: addq.b #$1, $ac2(a5)
pub fn i_c1619c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC161A0;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1619E: ?
pub fn i_c1619e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC161A0;
    cpu.ir = 0xAC2;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C161A0: lea.l $3c0002.l, a2
pub fn i_c161a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC161A6;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x3C0002;
    20
}
/// $C161A6: move.w #$1, $2(a2)
pub fn i_c161a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC161AC;
    cpu.ir = 0x357C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[2].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C161AC: moveq #$1, d1
pub fn i_c161ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC161AE;
    cpu.ir = 0x7201;
    cpu.d[1] = 0x1;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C161AE: moveq #$2, d2
pub fn i_c161ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC161B0;
    cpu.ir = 0x7402;
    cpu.d[2] = 0x2;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C161B0: moveq #$3, d3
pub fn i_c161b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC161B2;
    cpu.ir = 0x7603;
    cpu.d[3] = 0x3;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C161B2: moveq #$4, d4
pub fn i_c161b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC161B4;
    cpu.ir = 0x7804;
    cpu.d[4] = 0x4;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C161B4: move.w #$7000, -$2(a2)
pub fn i_c161b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC161BA;
    cpu.ir = 0x357C;
    let v1: u32 = 0x7000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[2].wrapping_add(0xFFFFFFFE);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C161BA: move.w #$13, d7
pub fn i_c161ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC161BE;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x13;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C161BE: move.w #$f, d6
pub fn i_c161be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC161C2;
    cpu.ir = 0x3C3C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C161C2: move.w d1, (a2)
pub fn i_c161c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC161C4;
    cpu.ir = 0x3481;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[2], v1 as u16);
    8
}
/// $C161C4: nop
pub fn i_c161c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC161C6;
    cpu.ir = 0x4E71;
    4
}
/// $C161C6: move.w d3, (a2)
pub fn i_c161c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC161C8;
    cpu.ir = 0x3483;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[2], v1 as u16);
    8
}
/// $C161C8: dbra d6, $c161c2
pub fn i_c161c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC161CC;
    cpu.ir = 0x51CE;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC161CC; return 12; }
    let n = (cpu.d[6] as u16).wrapping_sub(1);
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC161CC; return 14; }
    cpu.pc = 0xC161C2;
    10
}
/// $C161CC: move.w #$f, d6
pub fn i_c161cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC161D0;
    cpu.ir = 0x3C3C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C161D0: move.w d2, (a2)
pub fn i_c161d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC161D2;
    cpu.ir = 0x3482;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[2], v1 as u16);
    8
}
/// $C161D2: nop
pub fn i_c161d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC161D4;
    cpu.ir = 0x4E71;
    4
}
/// $C161D4: move.w d4, (a2)
pub fn i_c161d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC161D6;
    cpu.ir = 0x3484;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[2], v1 as u16);
    8
}
/// $C161D6: dbra d6, $c161d0
pub fn i_c161d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC161DA;
    cpu.ir = 0x51CE;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC161DA; return 12; }
    let n = (cpu.d[6] as u16).wrapping_sub(1);
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC161DA; return 14; }
    cpu.pc = 0xC161D0;
    10
}
/// $C161DA: dbra d7, $c161be
pub fn i_c161da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC161DE;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC161DE; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC161DE; return 14; }
    cpu.pc = 0xC161BE;
    10
}
/// $C161DE: move.w #$7000, -$2(a2)
pub fn i_c161de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC161E4;
    cpu.ir = 0x357C;
    let v1: u32 = 0x7000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[2].wrapping_add(0xFFFFFFFE);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C161E4: move.w #$5f, d7
pub fn i_c161e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC161E8;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x5F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C161E8: jsr $c16228(pc)
pub fn i_c161e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC161EC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC161EC);
    cpu.pc = 0xC16228;
    16
}
/// $C161EC: move.w #$74a0, -$2(a2)
pub fn i_c161ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC161F2;
    cpu.ir = 0x357C;
    let v1: u32 = 0x74A0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[2].wrapping_add(0xFFFFFFFE);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C161F2: move.w #$5f, d7
pub fn i_c161f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC161F6;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x5F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C161F6: jsr $c16228(pc)
pub fn i_c161f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC161FA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC161FA);
    cpu.pc = 0xC16228;
    16
}
/// $C161FA: move.w #$7002, -$2(a2)
pub fn i_c161fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16200;
    cpu.ir = 0x357C;
    let v1: u32 = 0x7002;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[2].wrapping_add(0xFFFFFFFE);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16200: move.w #$27, d7
pub fn i_c16200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16204;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x27;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16204: move.w #$20, $2(a2)
pub fn i_c16204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1620A;
    cpu.ir = 0x357C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[2].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C1620A: jsr $c16228(pc)
pub fn i_c1620a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1620E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1620E);
    cpu.pc = 0xC16228;
    16
}
/// $C1620E: move.w #$701d, -$2(a2)
pub fn i_c1620e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16214;
    cpu.ir = 0x357C;
    let v1: u32 = 0x701D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[2].wrapping_add(0xFFFFFFFE);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16214: move.w #$27, d7
pub fn i_c16214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16218;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x27;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16218: jsr $c16228(pc)
pub fn i_c16218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1621C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1621C);
    cpu.pc = 0xC16228;
    16
}
/// $C1621C: subq.b #$1, $ac2(a5)
pub fn i_c1621c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16220;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C16220: move.w #$1, $9d8(a5)
pub fn i_c16220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16226;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16226: rts
pub fn i_c16226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16228;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C16228: nop
pub fn i_c16228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1622A;
    cpu.ir = 0x4E71;
    4
}
/// $C1622A: nop
pub fn i_c1622a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1622C;
    cpu.ir = 0x4E71;
    4
}
/// $C1622C: nop
pub fn i_c1622c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1622E;
    cpu.ir = 0x4E71;
    4
}
/// $C1622E: move.w (a2), d0
pub fn i_c1622e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16230;
    cpu.ir = 0x3012;
    let v2 = bus.read16(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C16230: ori.w #$1000, d0
pub fn i_c16230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16234;
    cpu.ir = 0x40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 | 0x1000) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C16234: move.w d0, (a2)
pub fn i_c16234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16236;
    cpu.ir = 0x3480;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[2], v1 as u16);
    8
}
/// $C16236: dbra d7, $c16228
pub fn i_c16236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1623A;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC1623A; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC1623A; return 14; }
    cpu.pc = 0xC16228;
    10
}
/// $C1623A: rts
pub fn i_c1623a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1623C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1623C: btst.b #$0, $bdd(a5)
pub fn i_c1623c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16242;
    cpu.ir = 0x82D;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[5].wrapping_add(0xBDD);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $C16242: beq.b $c1624a
pub fn i_c16242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16244;
    cpu.ir = 0x6706;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1624A; return 10; }
    8
}
/// $C16244: move.w #$2, $9d8(a5)
pub fn i_c16244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1624A;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C1624A: rts
pub fn i_c1624a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1624C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1624C: move.w #$e00, $400024.l
pub fn i_c1624c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16254;
    cpu.ir = 0x33FC;
    let v1: u32 = 0xE00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x400024, v1 as u16);
    16
}
/// $C16254: move.w #$e0, $400044.l
pub fn i_c16254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1625C;
    cpu.ir = 0x33FC;
    let v1: u32 = 0xE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x400044, v1 as u16);
    16
}
/// $C1625C: move.w #$e, $400064.l
pub fn i_c1625c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16264;
    cpu.ir = 0x33FC;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x400064, v1 as u16);
    16
}
/// $C16264: move.w #$eee, $400084.l
pub fn i_c16264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1626C;
    cpu.ir = 0x33FC;
    let v1: u32 = 0xEEE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x400084, v1 as u16);
    16
}
/// $C1626C: jsr $c12a0e(pc)
pub fn i_c1626c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16270;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16270);
    cpu.pc = 0xC12A0E;
    16
}
/// $C16270: addq.b #$1, $ac2(a5)
pub fn i_c16270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16274;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C16274: movea.l $abe(a5), a0
pub fn i_c16274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16278;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C16278: move.l #$c17e20, (a0)+
pub fn i_c16278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1627E;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC17E20;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C1627E: jsr $c17eaa.l
pub fn i_c1627e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16284;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0xC16284);
    cpu.pc = 0xC17EAA;
    16
}
/// $C16284: move.l #$c162aa, (a0)+
pub fn i_c16284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1628A;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC162AA;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C1628A: move.l a0, $abe(a5)
pub fn i_c1628a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1628E;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1628E: subq.b #$1, $ac2(a5)
pub fn i_c1628e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16292;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C16292: move.w #$3, $9d8(a5)
pub fn i_c16292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16298;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16298: rts
pub fn i_c16298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1629A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1629A: btst.b #$0, $bdd(a5)
pub fn i_c1629a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC162A0;
    cpu.ir = 0x82D;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[5].wrapping_add(0xBDD);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $C162A0: beq.b $c162a8
pub fn i_c162a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC162A2;
    cpu.ir = 0x6706;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC162A8; return 10; }
    8
}
/// $C162A2: move.w #$4, $9d8(a5)
pub fn i_c162a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC162A8;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C162A8: rts
pub fn i_c162a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC162AA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C162AA: ori.b #$69, d3
pub fn i_c162aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC162AE;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x69) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C162AE: movep.w $2020(a0), d0
pub fn i_c162ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC162B2;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x2020);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C162B2: addq.w #$1, d5
pub fn i_c162b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC162B4;
    cpu.ir = 0x5245;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C162B4: neg.b -(a0)
pub fn i_c162b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC162B6;
    cpu.ir = 0x4420;
    let ad1 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.set(StatusRegister::V, v2 == 0x80); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write8(ad1, r3 as u8);
    10
}
/// $C162B6: move.l -(a0), d0
pub fn i_c162b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC162B8;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C162B8: movea.l d7, a0
pub fn i_c162b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC162BA;
    cpu.ir = 0x2047;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C162BA: addq.w #$1, d5
pub fn i_c162ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC162BC;
    cpu.ir = 0x5245;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C162BC: ?
pub fn i_c162bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC162BE;
    cpu.ir = 0x454E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C162BE: move.l -(a0), d0
pub fn i_c162be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC162C0;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C162C0: move.l -(a0), d0
pub fn i_c162c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC162C2;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C162C2: movea.l d2, a0
pub fn i_c162c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC162C4;
    cpu.ir = 0x2042;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C162C4: ?
pub fn i_c162c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC162C6;
    cpu.ir = 0x4C55;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C162C6: dc.w $4520
pub fn i_c162c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC162C8;
    cpu.ir = 0x4520;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C162C8: move.l -(a0), d0
pub fn i_c162c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC162CA;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C162CA: movea.l (a7), a0
pub fn i_c162ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC162CC;
    cpu.ir = 0x2057;
    let v2 = bus.read32(cpu.a[7]);
    cpu.a[0] = v2;
    12
}
/// $C1637C: jsr $c12a0e(pc)
pub fn i_c1637c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16380;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16380);
    cpu.pc = 0xC12A0E;
    16
}
/// $C16380: move.l #$eee0000, $400002.l
pub fn i_c16380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1638A;
    cpu.ir = 0x23FC;
    let v1: u32 = 0xEEE0000;
    bus.write32(0x400002, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C1638A: move.l #$e000000, $400022.l
pub fn i_c1638a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16394;
    cpu.ir = 0x23FC;
    let v1: u32 = 0xE000000;
    bus.write32(0x400022, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C16394: move.b #$1, $10febb.l
pub fn i_c16394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1639C;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10FEBB, v1 as u8);
    16
}
/// $C1639C: clr.b $9de(a5)
pub fn i_c1639c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC163A0;
    cpu.ir = 0x422D;
    let ad1 = cpu.a[5].wrapping_add(0x9DE);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C163A0: clr.b $9da(a5)
pub fn i_c163a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC163A4;
    cpu.ir = 0x422D;
    let ad1 = cpu.a[5].wrapping_add(0x9DA);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C163A4: moveq #$2, d0
pub fn i_c163a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC163A6;
    cpu.ir = 0x7002;
    cpu.d[0] = 0x2;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C163A6: move.b $300081.l, d1
pub fn i_c163a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC163AC;
    cpu.ir = 0x1239;
    let v2 = bus.read8(0x300081) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C163AC: btst.b #$6, d1
pub fn i_c163ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC163B0;
    cpu.ir = 0x801;
    let mk: u32 = 1u32 << ((6) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C163B0: beq.b $c163c2
pub fn i_c163b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC163B2;
    cpu.ir = 0x6710;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC163C2; return 10; }
    8
}
/// $C163B2: moveq #$6, d0
pub fn i_c163b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC163B4;
    cpu.ir = 0x7006;
    cpu.d[0] = 0x6;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C163B4: move.b $320001.l, d1
pub fn i_c163b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC163BA;
    cpu.ir = 0x1239;
    let v2 = bus.read8(0x320001) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C163BA: andi.b #$20, d1
pub fn i_c163ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC163BE;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0x20) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C163BE: bne.b $c163c2
pub fn i_c163be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC163C0;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC163C2; return 10; }
    8
}
/// $C163C0: moveq #$4, d0
pub fn i_c163c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC163C2;
    cpu.ir = 0x7004;
    cpu.d[0] = 0x4;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C163C2: move.b d0, $9db(a5)
pub fn i_c163c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC163C6;
    cpu.ir = 0x1B40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9DB);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C163C6: addq.b #$1, $ac2(a5)
pub fn i_c163c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC163CA;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C163CA: movea.l $abe(a5), a0
pub fn i_c163ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC163CE;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C163CE: move.l #$c17e20, (a0)+
pub fn i_c163ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC163D4;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC17E20;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C163D4: jsr $c17eaa.l
pub fn i_c163d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC163DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0xC163DA);
    cpu.pc = 0xC17EAA;
    16
}
/// $C163DA: move.l #$c16510, (a0)+
pub fn i_c163da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC163E0;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC16510;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C163E0: move.l a0, $abe(a5)
pub fn i_c163e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC163E4;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C163E4: subq.b #$1, $ac2(a5)
pub fn i_c163e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC163E8;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C163E8: move.w #$5, $9d8(a5)
pub fn i_c163e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC163EE;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C163EE: rts
pub fn i_c163ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC163F0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C163F0: btst.b #$0, $bdd(a5)
pub fn i_c163f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC163F6;
    cpu.ir = 0x82D;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[5].wrapping_add(0xBDD);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $C163F6: beq.b $c16404
pub fn i_c163f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC163F8;
    cpu.ir = 0x670C;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC16404; return 10; }
    8
}
/// $C163F8: clr.b $bbb(a5)
pub fn i_c163f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC163FC;
    cpu.ir = 0x422D;
    let ad1 = cpu.a[5].wrapping_add(0xBBB);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C163FC: move.w #$6, $9d8(a5)
pub fn i_c163fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16402;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16402: rts
pub fn i_c16402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16404;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C16404: addq.b #$1, $ac2(a5)
pub fn i_c16404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16408;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C16408: movea.l $abe(a5), a0
pub fn i_c16408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1640C;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C1640C: clr.l (a0)+
pub fn i_c1640c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1640E;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C1640E: move.b $bdc(a5), d0
pub fn i_c1640e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16412;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xBDC);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C16412: move.b $10fd96.l, d1
pub fn i_c16412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16418;
    cpu.ir = 0x1239;
    let v2 = bus.read8(0x10FD96) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C16418: move.w #$71a9, d2
pub fn i_c16418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1641C;
    cpu.ir = 0x343C;
    let v1: u32 = 0x71A9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1641C: jsr $c164dc(pc)
pub fn i_c1641c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16420;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16420);
    cpu.pc = 0xC164DC;
    16
}
/// $C16420: move.b $bdc(a5), d0
pub fn i_c16420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16424;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xBDC);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C16424: lsr.b #$2, d0
pub fn i_c16424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16426;
    cpu.ir = 0xE408;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Byte, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    10
}
/// $C16426: move.b $10fd9c.l, d1
pub fn i_c16426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1642C;
    cpu.ir = 0x1239;
    let v2 = bus.read8(0x10FD9C) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C1642C: move.w #$7209, d2
pub fn i_c1642c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16430;
    cpu.ir = 0x343C;
    let v1: u32 = 0x7209;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16430: jsr $c164dc(pc)
pub fn i_c16430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16434;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16434);
    cpu.pc = 0xC164DC;
    16
}
/// $C16434: move.b $300001.l, d0
pub fn i_c16434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1643A;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x300001) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C1643A: move.w #$7328, d1
pub fn i_c1643a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1643E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x7328;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1643E: jsr $c164aa(pc)
pub fn i_c1643e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16442;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16442);
    cpu.pc = 0xC164AA;
    16
}
/// $C16442: jsr $c16452(pc)
pub fn i_c16442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16446;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16446);
    cpu.pc = 0xC16452;
    16
}
/// $C16446: clr.w (a0)+
pub fn i_c16446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16448;
    cpu.ir = 0x4258;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C16448: move.l a0, $abe(a5)
pub fn i_c16448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1644C;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1644C: subq.b #$1, $ac2(a5)
pub fn i_c1644c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16450;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C16450: rts
pub fn i_c16450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16452;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C16452: move.l #$3010004, (a0)+
pub fn i_c16452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16458;
    cpu.ir = 0x20FC;
    let v1: u32 = 0x3010004;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C16458: move.w #$102, (a0)+
pub fn i_c16458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1645C;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x102;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C1645C: move.l #$373ab, (a0)+
pub fn i_c1645c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16462;
    cpu.ir = 0x20FC;
    let v1: u32 = 0x373AB;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C16462: move.w #$7, (a0)+
pub fn i_c16462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16466;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C16466: move.b $300081.l, d0
pub fn i_c16466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1646C;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x300081) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C1646C: not.b d0
pub fn i_c1646c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1646E;
    cpu.ir = 0x4600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = !v1 & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C1646E: btst.b #$7, d0
pub fn i_c1646e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16472;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((7) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C16472: jsr $c1649c(pc)
pub fn i_c16472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16476;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16476);
    cpu.pc = 0xC1649C;
    16
}
/// $C16476: btst.b #$0, $d00021.l
pub fn i_c16476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1647E;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((0) % 8);
    let v2 = bus.read8(0xD00021) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $C1647E: jsr $c1649c(pc)
pub fn i_c1647e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16482;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16482);
    cpu.pc = 0xC1649C;
    16
}
/// $C16482: btst.b #$1, $d00021.l
pub fn i_c16482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1648A;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((1) % 8);
    let v2 = bus.read8(0xD00021) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $C1648A: jsr $c1649c(pc)
pub fn i_c1648a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1648E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1648E);
    cpu.pc = 0xC1649C;
    16
}
/// $C1648E: btst.b #$2, $d00021.l
pub fn i_c1648e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16496;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((2) % 8);
    let v2 = bus.read8(0xD00021) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $C16496: jsr $c1649c(pc)
pub fn i_c16496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1649A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1649A);
    cpu.pc = 0xC1649C;
    16
}
/// $C1649A: rts
pub fn i_c1649a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1649C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1649C: bne.b $c164a4
pub fn i_c1649c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1649E;
    cpu.ir = 0x6606;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC164A4; return 10; }
    8
}
/// $C1649E: move.w #$30, (a0)+
pub fn i_c1649e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC164A2;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C164A2: rts
pub fn i_c164a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC164A4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C164A4: move.w #$1031, (a0)+
pub fn i_c164a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC164A8;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x1031;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C164A8: rts
pub fn i_c164a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC164AA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C164AA: not.b d0
pub fn i_c164aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC164AC;
    cpu.ir = 0x4600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = !v1 & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C164AC: move.l #$3010008, (a0)+
pub fn i_c164ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC164B2;
    cpu.ir = 0x20FC;
    let v1: u32 = 0x3010008;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C164B2: move.w #$2002, (a0)+
pub fn i_c164b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC164B6;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x2002;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C164B6: move.w #$3, (a0)+
pub fn i_c164b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC164BA;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C164BA: move.w d1, (a0)+
pub fn i_c164ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC164BC;
    cpu.ir = 0x30C1;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C164BC: move.w #$7, (a0)+
pub fn i_c164bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC164C0;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C164C0: moveq #$7, d7
pub fn i_c164c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC164C2;
    cpu.ir = 0x7E07;
    cpu.d[7] = 0x7;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C164C2: move.w #$0, d1
pub fn i_c164c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC164C6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C164C6: move.b #$30, d1
pub fn i_c164c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC164CA;
    cpu.ir = 0x123C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C164CA: lsr.b #$1, d0
pub fn i_c164ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC164CC;
    cpu.ir = 0xE208;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Byte, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    8
}
/// $C164CC: bcc.b $c164d4
pub fn i_c164cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC164CE;
    cpu.ir = 0x6406;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC164D4; return 10; }
    8
}
/// $C164CE: ori.w #$1000, d1
pub fn i_c164ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC164D2;
    cpu.ir = 0x41;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 | 0x1000) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C164D2: addq.b #$1, d1
pub fn i_c164d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC164D4;
    cpu.ir = 0x5201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C164D4: move.w d1, (a0)+
pub fn i_c164d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC164D6;
    cpu.ir = 0x30C1;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C164D6: dbra d7, $c164c2
pub fn i_c164d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC164DA;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC164DA; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC164DA; return 14; }
    cpu.pc = 0xC164C2;
    10
}
/// $C164DA: rts
pub fn i_c164da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC164DC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C164DC: lsl.w #$8, d0
pub fn i_c164dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC164DE;
    cpu.ir = 0xE148;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C164DE: move.b d1, d0
pub fn i_c164de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC164E0;
    cpu.ir = 0x1001;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C164E0: move.l #$301000a, (a0)+
pub fn i_c164e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC164E6;
    cpu.ir = 0x20FC;
    let v1: u32 = 0x301000A;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C164E6: move.w #$102, (a0)+
pub fn i_c164e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC164EA;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x102;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C164EA: move.w #$3, (a0)+
pub fn i_c164ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC164EE;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C164EE: move.w d2, (a0)+
pub fn i_c164ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC164F0;
    cpu.ir = 0x30C2;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C164F0: move.w #$7, (a0)+
pub fn i_c164f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC164F4;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C164F4: moveq #$9, d7
pub fn i_c164f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC164F6;
    cpu.ir = 0x7E09;
    cpu.d[7] = 0x9;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C164F6: move.w #$0, d1
pub fn i_c164f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC164FA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C164FA: move.b #$30, d1
pub fn i_c164fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC164FE;
    cpu.ir = 0x123C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C164FE: lsr.w #$1, d0
pub fn i_c164fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16500;
    cpu.ir = 0xE248;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $C16500: bcc.b $c16508
pub fn i_c16500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16502;
    cpu.ir = 0x6406;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC16508; return 10; }
    8
}
/// $C16502: ori.w #$1000, d1
pub fn i_c16502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16506;
    cpu.ir = 0x41;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 | 0x1000) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C16506: addq.b #$1, d1
pub fn i_c16506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16508;
    cpu.ir = 0x5201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C16508: move.w d1, (a0)+
pub fn i_c16508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1650A;
    cpu.ir = 0x30C1;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C1650A: dbra d7, $c164f6
pub fn i_c1650a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1650E;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC1650E; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC1650E; return 14; }
    cpu.pc = 0xC164F6;
    10
}
/// $C1650E: rts
pub fn i_c1650e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16510;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C16510: ori.b #$ff, d1
pub fn i_c16510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16514;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0xFF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C16514: move.l d2, d0
pub fn i_c16514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16516;
    cpu.ir = 0x2002;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C16516: ori.b #$e5, d3
pub fn i_c16516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1651A;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0xE5) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1651A: ori.b #$2f, d7
pub fn i_c1651a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1651E;
    cpu.ir = 0x7;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = (v1 | 0x2F) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1651E: dc.w $4f20
pub fn i_c1651e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16520;
    cpu.ir = 0x4F20;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C16520: ?
pub fn i_c16520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16522;
    cpu.ir = 0x4348;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C16522: ?
pub fn i_c16522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16524;
    cpu.ir = 0x4543;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C16600: addq.w #$1, $9d8(a5)
pub fn i_c16600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16604;
    cpu.ir = 0x526D;
    let ad1 = cpu.a[5].wrapping_add(0x9D8);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $C16604: rts
pub fn i_c16604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16606;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C16606: addq.w #$1, $9d8(a5)
pub fn i_c16606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1660A;
    cpu.ir = 0x526D;
    let ad1 = cpu.a[5].wrapping_add(0x9D8);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $C1660A: rts
pub fn i_c1660a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1660C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1660C: move.w #$9, $9d8(a5)
pub fn i_c1660c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16612;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16612: move.w #$0, $9da(a5)
pub fn i_c16612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16618;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9DA);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16618: rts
pub fn i_c16618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1661A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1661A: move.w #$a, $9d8(a5)
pub fn i_c1661a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16620;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16620: rts
pub fn i_c16620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16622;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C16622: jsr $c12a0e(pc)
pub fn i_c16622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16626;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16626);
    cpu.pc = 0xC12A0E;
    16
}
/// $C16626: addq.b #$1, $ac2(a5)
pub fn i_c16626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1662A;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1662A: movea.l $abe(a5), a0
pub fn i_c1662a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1662E;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C1662E: move.l #$c17e20, (a0)+
pub fn i_c1662e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16634;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC17E20;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C16634: jsr $c17eaa.l
pub fn i_c16634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1663A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0xC1663A);
    cpu.pc = 0xC17EAA;
    16
}
/// $C1663A: move.l #$c16688, (a0)+
pub fn i_c1663a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16640;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC16688;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C16640: move.l a0, $abe(a5)
pub fn i_c16640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16644;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C16644: subq.b #$1, $ac2(a5)
pub fn i_c16644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16648;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C16648: move.w #$b, $9d8(a5)
pub fn i_c16648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1664E;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0xB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C1664E: rts
pub fn i_c1664e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16650;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C16650: btst.b #$0, $bdd(a5)
pub fn i_c16650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16656;
    cpu.ir = 0x82D;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[5].wrapping_add(0xBDD);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $C16656: bne.b $c16680
pub fn i_c16656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16658;
    cpu.ir = 0x6628;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC16680; return 10; }
    8
}
/// $C16658: move.b $a96(a5), d0
pub fn i_c16658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1665C;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xA96);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C1665C: andi.b #$70, d0
pub fn i_c1665c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16660;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x70) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C16660: cmpi.b #$70, d0
pub fn i_c16660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16664;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x70, Size::Byte, false);
    8
}
/// $C16664: bne.b $c16686
pub fn i_c16664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16666;
    cpu.ir = 0x6620;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC16686; return 10; }
    8
}
/// $C16666: move.w #$2700, sr
pub fn i_c16666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1666A;
    cpu.ir = 0x46FC;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let v1: u32 = 0x2700;
    write_sr(cpu, v1 as u16);
    12
}
/// $C1666A: jsr $c115be(pc)
pub fn i_c1666a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1666E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1666E);
    cpu.pc = 0xC115BE;
    16
}
/// $C1666E: move.w #$2000, sr
pub fn i_c1666e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16672;
    cpu.ir = 0x46FC;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let v1: u32 = 0x2000;
    write_sr(cpu, v1 as u16);
    12
}
/// $C16672: jsr $c1727e(pc)
pub fn i_c16672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16676;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16676);
    cpu.pc = 0xC1727E;
    16
}
/// $C16676: move.b $a95(a5), d0
pub fn i_c16676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1667A;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xA95);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C1667A: andi.b #$70, d0
pub fn i_c1667a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1667E;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x70) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1667E: bne.b $c16672
pub fn i_c1667e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16680;
    cpu.ir = 0x66F2;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC16672; return 10; }
    8
}
/// $C16680: move.w #$c, $9d8(a5)
pub fn i_c16680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16686;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16686: rts
pub fn i_c16686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16688;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C16688: ori.b #$49, d3
pub fn i_c16688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1668C;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x49) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1668C: movep.w $4241(a0), d0
pub fn i_c1668c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16690;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x4241);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C16690: ?
pub fn i_c16690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16692;
    cpu.ir = 0x434B;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C16692: subq.w #$2, (a0)
pub fn i_c16692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16694;
    cpu.ir = 0x5550;
    let ad1 = cpu.a[0];
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C16694: movea.l d3, a0
pub fn i_c16694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16696;
    cpu.ir = 0x2043;
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C16696: ?
pub fn i_c16696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16698;
    cpu.ir = 0x4C45;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C16698: ?
pub fn i_c16698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1669A;
    cpu.ir = 0x4152;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1669A: dc.w $ffff
pub fn i_c1669a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1669C;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C1669C: ori.b #$c, d3
pub fn i_c1669c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC166A0;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0xC) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C166A0: movep.w $4f4b(a0), d0
pub fn i_c166a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC166A4;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x4F4B);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C166A4: ?
pub fn i_c166a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC166A6;
    cpu.ir = 0x203D;
    let v1: u32 = 0x0;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C166A6: movea.l (a0), a0
pub fn i_c166a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC166A8;
    cpu.ir = 0x2050;
    let v2 = bus.read32(cpu.a[0]);
    cpu.a[0] = v2;
    12
}
/// $C166A8: addq.w #$1, d5
pub fn i_c166a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC166AA;
    cpu.ir = 0x5245;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C166AA: subq.w #$1, (a3)
pub fn i_c166aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC166AC;
    cpu.ir = 0x5353;
    let ad1 = cpu.a[3];
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C166AC: movea.l d1, a0
pub fn i_c166ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC166AE;
    cpu.ir = 0x2041;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C166AE: movea.l d2, a6
pub fn i_c166ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC166B0;
    cpu.ir = 0x2C42;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.a[6] = v1;
    4
}
/// $C166B0: movea.l d3, a6
pub fn i_c166b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC166B2;
    cpu.ir = 0x2C43;
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.a[6] = v1;
    4
}
/// $C166B2: movea.l d2, a0
pub fn i_c166b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC166B4;
    cpu.ir = 0x2042;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C166B4: subq.w #$2, (a4)
pub fn i_c166b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC166B6;
    cpu.ir = 0x5554;
    let ad1 = cpu.a[4];
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C166B6: addq.w #$2, a7
pub fn i_c166b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC166B8;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $C166B8: link.w a3, #$ffff
pub fn i_c166b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC166BC;
    cpu.ir = 0x4E53;
    cpu.push32(bus, cpu.a[3]); cpu.a[3] = cpu.a[7]; cpu.a[7] = cpu.a[7].wrapping_add(0xFFFFFFFF);
    16
}
/// $C166BC: ori.b #$2, d5
pub fn i_c166bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC166C0;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C166C0: movep.w $2020(a0), d0
pub fn i_c166c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC166C4;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x2020);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C166C4: move.l -(a0), d0
pub fn i_c166c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC166C6;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C166C6: movea.l d1, a0
pub fn i_c166c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC166C8;
    cpu.ir = 0x2041;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C166C8: addq.b #$2, -(a0)
pub fn i_c166c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC166CA;
    cpu.ir = 0x5420;
    let ad1 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    10
}
/// $C166CA: ?
pub fn i_c166ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC166CC;
    cpu.ir = 0x4F4E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C166CC: dc.w $4520
pub fn i_c166cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC166CE;
    cpu.ir = 0x4520;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C166CE: addq.w #$2, a1
pub fn i_c166ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC166D0;
    cpu.ir = 0x5449;
    cpu.a[1] = cpu.a[1].wrapping_add(2);
    8
}
/// $C166D0: ?
pub fn i_c166d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC166D2;
    cpu.ir = 0x4D45;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C166D2: dc.w $ffff
pub fn i_c166d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC166D4;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C166D4: ori.b #$ba, d0
pub fn i_c166d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC166D8;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0xBA) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C166D6: jsr $c12a0e(pc)
pub fn i_c166d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC166DA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC166DA);
    cpu.pc = 0xC12A0E;
    16
}
/// $C166D8: and.b d1, $eee0000(invalid.w)
pub fn i_c166d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC166DC;
    cpu.ir = 0xC336;
    let ad1 = cpu.a[6].wrapping_add(0xFFFFFFFC).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & cpu.d[1]) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    14
}
/// $C166DA: move.l #$eee0000, $400002.l
pub fn i_c166da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC166E4;
    cpu.ir = 0x23FC;
    let v1: u32 = 0xEEE0000;
    bus.write32(0x400002, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C166DC: dc.w $eee
pub fn i_c166dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC166DE;
    cpu.ir = 0xEEE;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C166DE: ori.b #$40, d0
pub fn i_c166de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC166E2;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x40) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C166E2: ori.b #$fc, d2
pub fn i_c166e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC166E6;
    cpu.ir = 0x2;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 | 0xFC) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C166E4: move.l #$e000000, $400022.l
pub fn i_c166e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC166EE;
    cpu.ir = 0x23FC;
    let v1: u32 = 0xE000000;
    bus.write32(0x400022, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C166E6: ?
pub fn i_c166e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC166EA;
    cpu.ir = 0xE00;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C166EA: ori.w #$22, d0
pub fn i_c166ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC166EE;
    cpu.ir = 0x40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 | 0x22) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C166EE: jsr $c11c42(pc)
pub fn i_c166ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC166F2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC166F2);
    cpu.pc = 0xC11C42;
    16
}
/// $C166F2: moveq #$7, d6
pub fn i_c166f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC166F4;
    cpu.ir = 0x7C07;
    cpu.d[6] = 0x7;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C166F4: addq.b #$1, $ac2(a5)
pub fn i_c166f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC166F8;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C166F8: movea.l $abe(a5), a0
pub fn i_c166f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC166FC;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C166FC: move.l #$c17e20, (a0)+
pub fn i_c166fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16702;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC17E20;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C16702: jsr $c17eaa.l
pub fn i_c16702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16708;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0xC16708);
    cpu.pc = 0xC17EAA;
    16
}
/// $C16708: move.l #$c168fc, (a0)+
pub fn i_c16708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1670E;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC168FC;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C1670E: clr.l (a0)+
pub fn i_c1670e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16710;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C16710: move.w #$720a, d2
pub fn i_c16710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16714;
    cpu.ir = 0x343C;
    let v1: u32 = 0x720A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16714: move.b $ad2(a5), d0
pub fn i_c16714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16718;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C16718: jsr $c168be(pc)
pub fn i_c16718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1671C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1671C);
    cpu.pc = 0xC168BE;
    16
}
/// $C1671C: move.w #$714a, d2
pub fn i_c1671c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16720;
    cpu.ir = 0x343C;
    let v1: u32 = 0x714A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16720: move.b $ad3(a5), d0
pub fn i_c16720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16724;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD3);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C16724: jsr $c168be(pc)
pub fn i_c16724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16728;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16728);
    cpu.pc = 0xC168BE;
    16
}
/// $C16728: move.w #$71aa, d2
pub fn i_c16728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1672C;
    cpu.ir = 0x343C;
    let v1: u32 = 0x71AA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1672C: move.b $ad4(a5), d0
pub fn i_c1672c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16730;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD4);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C16730: jsr $c168be(pc)
pub fn i_c16730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16734;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16734);
    cpu.pc = 0xC168BE;
    16
}
/// $C16734: move.w #$714d, d2
pub fn i_c16734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16738;
    cpu.ir = 0x343C;
    let v1: u32 = 0x714D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16738: move.b $ad6(a5), d0
pub fn i_c16738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1673C;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C1673C: jsr $c168be(pc)
pub fn i_c1673c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16740;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16740);
    cpu.pc = 0xC168BE;
    16
}
/// $C16740: move.w #$71ad, d2
pub fn i_c16740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16744;
    cpu.ir = 0x343C;
    let v1: u32 = 0x71AD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16744: move.b $ad7(a5), d0
pub fn i_c16744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16748;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD7);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C16748: jsr $c168be(pc)
pub fn i_c16748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1674C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1674C);
    cpu.pc = 0xC168BE;
    16
}
/// $C1674C: move.w #$720d, d2
pub fn i_c1674c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16750;
    cpu.ir = 0x343C;
    let v1: u32 = 0x720D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16750: move.b $ad8(a5), d0
pub fn i_c16750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16754;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD8);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C16754: jsr $c168be(pc)
pub fn i_c16754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16758;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16758);
    cpu.pc = 0xC168BE;
    16
}
/// $C16758: move.w #$3, (a0)+
pub fn i_c16758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1675C;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C1675C: move.w #$728a, (a0)+
pub fn i_c1675c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16760;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x728A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C16760: jsr $c17e7a(pc)
pub fn i_c16760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16764;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16764);
    cpu.pc = 0xC17E7A;
    16
}
/// $C16764: clr.w (a0)+
pub fn i_c16764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16766;
    cpu.ir = 0x4258;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C16766: move.l a0, $abe(a5)
pub fn i_c16766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1676A;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1676A: subq.b #$1, $ac2(a5)
pub fn i_c1676a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1676E;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1676E: jsr $c1685e(pc)
pub fn i_c1676e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16772;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16772);
    cpu.pc = 0xC1685E;
    16
}
/// $C16772: clr.w $9da(a5)
pub fn i_c16772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16776;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0x9DA);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $C16776: move.w #$d, $9d8(a5)
pub fn i_c16776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1677C;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0xD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C1677C: rts
pub fn i_c1677c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1677E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1678A: btst.b #$0, $bdd(a5)
pub fn i_c1678a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16790;
    cpu.ir = 0x82D;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[5].wrapping_add(0xBDD);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $C16790: beq.b $c1679a
pub fn i_c16790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16792;
    cpu.ir = 0x6708;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1679A; return 10; }
    8
}
/// $C16792: move.w #$0, $9d8(a5)
pub fn i_c16792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16798;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16798: rts
pub fn i_c16798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1679A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1679A: btst.b #$7, $a97(a5)
pub fn i_c1679a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC167A0;
    cpu.ir = 0x82D;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[5].wrapping_add(0xA97);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $C167A0: bne.w $c1682e
pub fn i_c167a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC167A4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1682E; return 10; }
    8
}
/// $C167A4: jsr $c1685e(pc)
pub fn i_c167a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC167A8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC167A8);
    cpu.pc = 0xC1685E;
    16
}
/// $C167A8: move.b $a97(a5), d0
pub fn i_c167a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC167AC;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xA97);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C167AC: andi.b #$c, d0
pub fn i_c167ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC167B0;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xC) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C167B0: beq.w $c167dc
pub fn i_c167b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC167B4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC167DC; return 10; }
    8
}
/// $C167B4: moveq #$1, d1
pub fn i_c167b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC167B6;
    cpu.ir = 0x7201;
    cpu.d[1] = 0x1;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C167B6: andi.b #$8, d0
pub fn i_c167b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC167BA;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x8) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C167BA: bne.b $c167be
pub fn i_c167ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC167BC;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC167BE; return 10; }
    8
}
/// $C167BC: moveq #$ff, d1
pub fn i_c167bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC167BE;
    cpu.ir = 0x72FF;
    cpu.d[1] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C167BE: move.w $9da(a5), d0
pub fn i_c167be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC167C2;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9DA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C167C2: add.b d1, d0
pub fn i_c167c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC167C4;
    cpu.ir = 0xD001;
    let v1 = cpu.d[1] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    let r = add_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    4
}
/// $C167C4: cmpi.b #$6, d0
pub fn i_c167c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC167C8;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x6, Size::Byte, false);
    8
}
/// $C167C8: bcc.b $c167c2
pub fn i_c167c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC167CA;
    cpu.ir = 0x64F8;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC167C2; return 10; }
    8
}
/// $C167CA: move.w d0, $9da(a5)
pub fn i_c167ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC167CE;
    cpu.ir = 0x3B40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9DA);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C167CE: jsr $c1872e.l
pub fn i_c167ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC167D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0xC167D4);
    cpu.pc = 0xC1872E;
    16
}
/// $C167D4: rts
pub fn i_c167d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC167D6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C167D6: btst.l d0, d2
pub fn i_c167d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC167D8;
    cpu.ir = 0x102;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C167D8: ori.b #$6, d4
pub fn i_c167d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC167DC;
    cpu.ir = 0x4;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 | 0x6) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C167DC: move.b $a98(a5), d0
pub fn i_c167dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC167E0;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xA98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C167E0: andi.b #$30, d0
pub fn i_c167e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC167E4;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x30) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C167E4: beq.b $c167ce
pub fn i_c167e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC167E6;
    cpu.ir = 0x67E8;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC167CE; return 10; }
    8
}
/// $C167E6: moveq #$1, d1
pub fn i_c167e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC167E8;
    cpu.ir = 0x7201;
    cpu.d[1] = 0x1;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C167E8: andi.b #$10, d0
pub fn i_c167e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC167EC;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x10) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C167EC: bne.b $c167f2
pub fn i_c167ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC167EE;
    cpu.ir = 0x6604;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC167F2; return 10; }
    8
}
/// $C167EE: move.b #$99, d1
pub fn i_c167ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC167F2;
    cpu.ir = 0x123C;
    let v1: u32 = 0x99;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C167F2: moveq #$0, d2
pub fn i_c167f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC167F4;
    cpu.ir = 0x7400;
    cpu.d[2] = 0x0;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C167F4: move.w $9da(a5), d0
pub fn i_c167f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC167F8;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9DA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C167F8: lea.l $ad2(a5), a1
pub fn i_c167f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC167FC;
    cpu.ir = 0x43ED;
    let ad1 = cpu.a[5].wrapping_add(0xAD2);
    cpu.a[1] = ad1;
    16
}
/// $C167FC: move.b $c167d6(pc, d0.w), d2
pub fn i_c167fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16800;
    cpu.ir = 0x143B;
    let ad2 = 0xC167D6_u32.wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    14
}
/// $C16800: move.b (a1, d2.w), d3
pub fn i_c16800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16804;
    cpu.ir = 0x1631;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    14
}
/// $C16804: sub.b d4, d4
pub fn i_c16804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16806;
    cpu.ir = 0x9804;
    let v1 = cpu.d[4] & 0xFF;
    let h1 = cpu.d[4] & 0xFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r & 0xFF);
    4
}
/// $C16806: abcd.b d1, d3
pub fn i_c16806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16808;
    cpu.ir = 0xC701;
    let s = cpu.d[1] as u8; let d = cpu.d[3] as u8;
    let x = cpu.sr.get(StatusRegister::X) as u8;
    let (r, carry, vf) = abcd_calc(d, s, x);
    if r != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::N, r & 0x80 != 0); cpu.sr.set(StatusRegister::V, vf);
    cpu.sr.set(StatusRegister::C, carry); cpu.sr.set(StatusRegister::X, carry);
    cpu.d[3] = (cpu.d[3] & !0xFF) | r as u32;
    6
}
/// $C16808: tst.b d2
pub fn i_c16808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1680A;
    cpu.ir = 0x4A02;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C1680A: beq.b $c16818
pub fn i_c1680a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1680C;
    cpu.ir = 0x670C;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC16818; return 10; }
    8
}
/// $C1680C: cmp.b $c1681e(pc, d2.w), d3
pub fn i_c1680c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16810;
    cpu.ir = 0xB63B;
    let ad2 = 0xC1681E_u32.wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[3] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    14
}
/// $C16810: bhi.b $c16804
pub fn i_c16810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16812;
    cpu.ir = 0x62F2;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0xC16804; return 10; }
    8
}
/// $C16812: cmp.b $c16826(pc, d2.w), d3
pub fn i_c16812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16816;
    cpu.ir = 0xB63B;
    let ad2 = 0xC16826_u32.wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[3] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    14
}
/// $C16816: bcs.b $c16804
pub fn i_c16816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16818;
    cpu.ir = 0x65EC;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC16804; return 10; }
    8
}
/// $C16818: move.b d3, (a1, d2.w)
pub fn i_c16818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1681C;
    cpu.ir = 0x1383;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $C1681C: rts
pub fn i_c1681c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1681E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1681E: ori.b #$0, (a2)
pub fn i_c1681e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16822;
    cpu.ir = 0x12;
    let ad1 = cpu.a[2];
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C16822: move.l (a1)+, $5900(a1)
pub fn i_c16822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16826;
    cpu.ir = 0x2359;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1].wrapping_add(0x5900);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C16826: ori.b #$0, d1
pub fn i_c16826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1682A;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1682A: ori.b #$0, d0
pub fn i_c1682a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1682E;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1682E: move.b $d0005e.l, d5
pub fn i_c1682e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16834;
    cpu.ir = 0x1A39;
    let v2 = bus.read8(0xD0005E) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C16834: bmi.b $c16852
pub fn i_c16834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16836;
    cpu.ir = 0x6B1C;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0xC16852; return 10; }
    8
}
/// $C16836: move.w #$2700, sr
pub fn i_c16836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1683A;
    cpu.ir = 0x46FC;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let v1: u32 = 0x2700;
    write_sr(cpu, v1 as u16);
    12
}
/// $C1683A: jsr $c10f90(pc)
pub fn i_c1683a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1683E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1683E);
    cpu.pc = 0xC10F90;
    16
}
/// $C1683E: move.b d0, $3a0003.l
pub fn i_c1683e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16844;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0003, v1 as u8);
    16
}
/// $C16844: move.w #$2000, sr
pub fn i_c16844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16848;
    cpu.ir = 0x46FC;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let v1: u32 = 0x2000;
    write_sr(cpu, v1 as u16);
    12
}
/// $C16848: jsr $c11e04(pc)
pub fn i_c16848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1684C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1684C);
    cpu.pc = 0xC11E04;
    16
}
/// $C1684C: move.b d0, $3a000b.l
pub fn i_c1684c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16852;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A000B, v1 as u8);
    16
}
/// $C16852: jsr $c11cc0(pc)
pub fn i_c16852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16856;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16856);
    cpu.pc = 0xC11CC0;
    16
}
/// $C16856: tst.b d7
pub fn i_c16856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16858;
    cpu.ir = 0x4A07;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C16858: beq.w $c166d6
pub fn i_c16858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1685C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC166D6; return 10; }
    8
}
/// $C1685C: rts
pub fn i_c1685c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1685E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1685E: moveq #$0, d6
pub fn i_c1685e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16860;
    cpu.ir = 0x7C00;
    cpu.d[6] = 0x0;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C16860: addq.b #$1, $ac2(a5)
pub fn i_c16860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16864;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C16864: movea.l $abe(a5), a0
pub fn i_c16864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16868;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C16868: clr.l (a0)+
pub fn i_c16868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1686A;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C1686A: move.w #$7119, d2
pub fn i_c1686a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1686E;
    cpu.ir = 0x343C;
    let v1: u32 = 0x7119;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1686E: move.b $ad3(a5), d0
pub fn i_c1686e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16872;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD3);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C16872: jsr $c168be(pc)
pub fn i_c16872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16876;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16876);
    cpu.pc = 0xC168BE;
    16
}
/// $C16876: move.w #$7179, d2
pub fn i_c16876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1687A;
    cpu.ir = 0x343C;
    let v1: u32 = 0x7179;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1687A: move.b $ad4(a5), d0
pub fn i_c1687a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1687E;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD4);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C1687E: jsr $c168be(pc)
pub fn i_c1687e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16882;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16882);
    cpu.pc = 0xC168BE;
    16
}
/// $C16882: move.w #$71d9, d2
pub fn i_c16882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16886;
    cpu.ir = 0x343C;
    let v1: u32 = 0x71D9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16886: move.b $ad2(a5), d0
pub fn i_c16886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1688A;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C1688A: jsr $c168be(pc)
pub fn i_c1688a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1688E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1688E);
    cpu.pc = 0xC168BE;
    16
}
/// $C1688E: move.w #$7239, d2
pub fn i_c1688e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16892;
    cpu.ir = 0x343C;
    let v1: u32 = 0x7239;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16892: move.b $ad6(a5), d0
pub fn i_c16892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16896;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C16896: jsr $c168be(pc)
pub fn i_c16896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1689A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1689A);
    cpu.pc = 0xC168BE;
    16
}
/// $C1689A: move.w #$7299, d2
pub fn i_c1689a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1689E;
    cpu.ir = 0x343C;
    let v1: u32 = 0x7299;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1689E: move.b $ad7(a5), d0
pub fn i_c1689e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC168A2;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD7);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C168A2: jsr $c168be(pc)
pub fn i_c168a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC168A6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC168A6);
    cpu.pc = 0xC168BE;
    16
}
/// $C168A6: move.w #$72f9, d2
pub fn i_c168a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC168AA;
    cpu.ir = 0x343C;
    let v1: u32 = 0x72F9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C168AA: move.b $ad8(a5), d0
pub fn i_c168aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC168AE;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD8);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C168AE: jsr $c168be(pc)
pub fn i_c168ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC168B2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC168B2);
    cpu.pc = 0xC168BE;
    16
}
/// $C168B2: clr.w (a0)+
pub fn i_c168b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC168B4;
    cpu.ir = 0x4258;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C168B4: move.l a0, $abe(a5)
pub fn i_c168b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC168B8;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C168B8: subq.b #$1, $ac2(a5)
pub fn i_c168b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC168BC;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C168BC: rts
pub fn i_c168bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC168BE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C168BE: move.w #$3, (a0)+
pub fn i_c168be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC168C2;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C168C2: move.w d2, (a0)+
pub fn i_c168c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC168C4;
    cpu.ir = 0x30C2;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C168C4: move.w #$108, d5
pub fn i_c168c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC168C8;
    cpu.ir = 0x3A3C;
    let v1: u32 = 0x108;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C168C8: cmp.w $9da(a5), d6
pub fn i_c168c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC168CC;
    cpu.ir = 0xBC6D;
    let ad2 = cpu.a[5].wrapping_add(0x9DA);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[6] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $C168CC: bne.b $c168d2
pub fn i_c168cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC168CE;
    cpu.ir = 0x6604;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC168D2; return 10; }
    8
}
/// $C168CE: move.w #$1108, d5
pub fn i_c168ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC168D2;
    cpu.ir = 0x3A3C;
    let v1: u32 = 0x1108;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C168D2: move.w d5, (a0)+
pub fn i_c168d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC168D4;
    cpu.ir = 0x30C5;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C168D4: move.b #$20, (a0)+
pub fn i_c168d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC168D8;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C168D8: move.b d0, d1
pub fn i_c168d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC168DA;
    cpu.ir = 0x1200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C168DA: lsr.b #$4, d0
pub fn i_c168da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC168DC;
    cpu.ir = 0xE808;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    14
}
/// $C168DC: andi.b #$f, d0
pub fn i_c168dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC168E0;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C168E0: beq.b $c168ea
pub fn i_c168e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC168E2;
    cpu.ir = 0x6708;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC168EA; return 10; }
    8
}
/// $C168E2: addi.b #$30, d0
pub fn i_c168e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC168E6;
    cpu.ir = 0x600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x30, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C168E6: move.b d0, -$1(a0)
pub fn i_c168e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC168EA;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFF);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C168EA: andi.b #$f, d1
pub fn i_c168ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC168EE;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C168EE: addi.b #$30, d1
pub fn i_c168ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC168F2;
    cpu.ir = 0x601;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x30, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C168F2: move.b d1, (a0)+
pub fn i_c168f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC168F4;
    cpu.ir = 0x10C1;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C168F4: move.w #$ff00, (a0)+
pub fn i_c168f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC168F8;
    cpu.ir = 0x30FC;
    let v1: u32 = 0xFF00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C168F8: addq.w #$1, d6
pub fn i_c168f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC168FA;
    cpu.ir = 0x5246;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C168FA: rts
pub fn i_c168fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC168FC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C168FC: ori.b #$4, d3
pub fn i_c168fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16900;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x4) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C16900: movep.w $5345(a0), d0
pub fn i_c16900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16904;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x5345);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C16904: addq.w #$2, (a4)
pub fn i_c16904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16906;
    cpu.ir = 0x5454;
    let ad1 = cpu.a[4];
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C16906: ?
pub fn i_c16906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16908;
    cpu.ir = 0x494E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C16908: dc.w $4720
pub fn i_c16908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1690A;
    cpu.ir = 0x4720;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1690A: subq.w #$2, (a0)
pub fn i_c1690a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1690C;
    cpu.ir = 0x5550;
    let ad1 = cpu.a[0];
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C1690C: movea.l (a4), a0
pub fn i_c1690c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1690E;
    cpu.ir = 0x2054;
    let v2 = bus.read32(cpu.a[4]);
    cpu.a[0] = v2;
    12
}
/// $C1690E: swap d5
pub fn i_c1690e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16910;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C16910: movea.l d3, a0
pub fn i_c16910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16912;
    cpu.ir = 0x2043;
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C16912: ?
pub fn i_c16912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16914;
    cpu.ir = 0x414C;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C16914: ?
pub fn i_c16914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16916;
    cpu.ir = 0x454E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C16916: neg.w d1
pub fn i_c16916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16918;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C169B0: move.b #$3, $320000.l
pub fn i_c169b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC169B8;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x320000, v1 as u8);
    16
}
/// $C169B8: lea.l $10f300.l, a5
pub fn i_c169b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC169BE;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x10F300;
    20
}
/// $C169BE: moveq #$0, d0
pub fn i_c169be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC169C0;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C169C0: move.w d0, $9d8(a5)
pub fn i_c169c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC169C4;
    cpu.ir = 0x3B40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C169C4: move.w d0, $9e6(a5)
pub fn i_c169c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC169C8;
    cpu.ir = 0x3B40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9E6);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C169C8: move.b d0, $3a0001.l
pub fn i_c169c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC169CE;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0001, v1 as u8);
    16
}
/// $C169CE: andi.w #$f0ff, sr
pub fn i_c169ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC169D2;
    cpu.ir = 0x27C;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let h1 = cpu.sr.0 & 0xF0FF;
    write_sr(cpu, h1);
    20
}
/// $C169D2: jsr $c1727e(pc)
pub fn i_c169d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC169D6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC169D6);
    cpu.pc = 0xC1727E;
    16
}
/// $C169D6: addq.b #$1, $ac2(a5)
pub fn i_c169d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC169DA;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C169DA: movea.l $abe(a5), a0
pub fn i_c169da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC169DE;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C169DE: move.l #$c16a7a, (a0)+
pub fn i_c169de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC169E4;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC16A7A;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C169E4: move.l a0, $abe(a5)
pub fn i_c169e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC169E8;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C169E8: subq.b #$1, $ac2(a5)
pub fn i_c169e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC169EC;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C169EC: jsr $c1727e(pc)
pub fn i_c169ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC169F0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC169F0);
    cpu.pc = 0xC1727E;
    16
}
/// $C169EE: ?
pub fn i_c169ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC169F2;
    cpu.ir = 0x890;
    let mk: u32 = 1u32 << ((249) % 8);
    let ad1 = cpu.a[0];
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C169F0: lea.l $10f300.l, a5
pub fn i_c169f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC169F6;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x10F300;
    20
}
/// $C169F2: ori.b #$0, (a0)
pub fn i_c169f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC169F6;
    cpu.ir = 0x10;
    let ad1 = cpu.a[0];
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C169F6: tst.b $a82(a5)
pub fn i_c169f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC169FA;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xA82);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C169FA: beq.b $c16a0e
pub fn i_c169fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC169FC;
    cpu.ir = 0x6712;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC16A0E; return 10; }
    8
}
/// $C169FC: tst.w $d00034.l
pub fn i_c169fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A02;
    cpu.ir = 0x4A79;
    let v2 = bus.read16(0xD00034) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    16
}
/// $C16A02: beq.b $c16a20
pub fn i_c16a02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A04;
    cpu.ir = 0x671C;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC16A20; return 10; }
    8
}
/// $C16A04: bra.w $c16e20
pub fn i_c16a04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A08;
    cpu.ir = 0x6000;
    cpu.pc = 0xC16E20;
    10
}
/// $C16A08: jmp $c102d2.l
pub fn i_c16a08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A0E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC102D2;
    8
}
/// $C16A0E: move.b $beb(a5), d0
pub fn i_c16a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A12;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xBEB);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C16A12: andi.b #$f0, d0
pub fn i_c16a12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A16;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C16A16: bne.b $c16a08
pub fn i_c16a16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A18;
    cpu.ir = 0x66F0;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC16A08; return 10; }
    8
}
/// $C16A18: btst.b #$0, $aad(a5)
pub fn i_c16a18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A1E;
    cpu.ir = 0x82D;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[5].wrapping_add(0xAAD);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $C16A1E: bne.b $c16a08
pub fn i_c16a1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A20;
    cpu.ir = 0x66E8;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC16A08; return 10; }
    8
}
/// $C16A20: tst.w $9e6(a5)
pub fn i_c16a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A24;
    cpu.ir = 0x4A6D;
    let ad2 = cpu.a[5].wrapping_add(0x9E6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $C16A24: beq.b $c16a44
pub fn i_c16a24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A26;
    cpu.ir = 0x671E;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC16A44; return 10; }
    8
}
/// $C16A26: movea.l $9e8(a5), a0
pub fn i_c16a26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A2A;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0x9E8);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C16A2A: lea.l $4001e2.l, a1
pub fn i_c16a2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A30;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x4001E2;
    20
}
/// $C16A30: moveq #$4, d6
pub fn i_c16a30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A32;
    cpu.ir = 0x7C04;
    cpu.d[6] = 0x4;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C16A32: move.w (a0)+, (a1)+
pub fn i_c16a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A34;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $C16A34: dbra d6, $c16a32
pub fn i_c16a34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A38;
    cpu.ir = 0x51CE;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC16A38; return 12; }
    let n = (cpu.d[6] as u16).wrapping_sub(1);
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC16A38; return 14; }
    cpu.pc = 0xC16A32;
    10
}
/// $C16A38: move.w (a0), $401ffe.l
pub fn i_c16a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A3E;
    cpu.ir = 0x33D0;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x401FFE, v2 as u16);
    20
}
/// $C16A3E: clr.w $10fce6.l
pub fn i_c16a3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A44;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x10FCE6) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x10FCE6, r2 as u16);
    16
}
/// $C16A44: move.w $9d8(a5), d0
pub fn i_c16a44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A48;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C16A48: lsl.w #$2, d0
pub fn i_c16a48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A4A;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $C16A4A: movea.l $c16a52(pc, d0.w), a0
pub fn i_c16a4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A4E;
    cpu.ir = 0x207B;
    let ad2 = 0xC16A52_u32.wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $C16A4E: jsr (a0)
pub fn i_c16a4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A50;
    cpu.ir = 0x4E90;
    cpu.push32(bus, 0xC16A50);
    cpu.jump_to(cpu.a[0]);
    16
}
/// $C16A50: bra.b $c169d6
pub fn i_c16a50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A52;
    cpu.ir = 0x6084;
    cpu.pc = 0xC169D6;
    10
}
/// $C16A52: ?
pub fn i_c16a52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A54;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C16A54: bpl.b $c169ee
pub fn i_c16a54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A56;
    cpu.ir = 0x6A98;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0xC169EE; return 10; }
    8
}
/// $C16A56: ?
pub fn i_c16a56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A58;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C16A58: bmi.b $c169f6
pub fn i_c16a58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A5A;
    cpu.ir = 0x6B9C;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0xC169F6; return 10; }
    8
}
/// $C16A5A: ?
pub fn i_c16a5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A5C;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C16A5C: bmi.b $c16a5c
pub fn i_c16a5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A5E;
    cpu.ir = 0x6BFE;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0xC16A5C; return 10; }
    8
}
/// $C16A5E: ?
pub fn i_c16a5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A60;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C16A60: bge.b $c16a6e
pub fn i_c16a60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A62;
    cpu.ir = 0x6C0C;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0xC16A6E; return 10; }
    8
}
/// $C16A62: ?
pub fn i_c16a62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A64;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C16A64: bge.b $c16acc
pub fn i_c16a64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A66;
    cpu.ir = 0x6C66;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0xC16ACC; return 10; }
    8
}
/// $C16A66: ?
pub fn i_c16a66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A68;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C16A68: bge.b $c16a0e
pub fn i_c16a68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A6A;
    cpu.ir = 0x6CA4;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0xC16A0E; return 10; }
    8
}
/// $C16A6A: ?
pub fn i_c16a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A6C;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C16A6C: blt.b $c16a8c
pub fn i_c16a6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A6E;
    cpu.ir = 0x6D1E;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0xC16A8C; return 10; }
    8
}
/// $C16A6E: ?
pub fn i_c16a6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A70;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C16A70: blt.b $c16aaa
pub fn i_c16a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A72;
    cpu.ir = 0x6D38;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0xC16AAA; return 10; }
    8
}
/// $C16A72: ?
pub fn i_c16a72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A74;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C16A74: blt.b $c16af0
pub fn i_c16a74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A76;
    cpu.ir = 0x6D7A;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0xC16AF0; return 10; }
    8
}
/// $C16A76: ?
pub fn i_c16a76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A78;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C16A78: bgt.b $c16a92
pub fn i_c16a78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A7A;
    cpu.ir = 0x6E18;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0xC16A92; return 10; }
    8
}
/// $C16A7A: btst.l d1, d1
pub fn i_c16a7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A7C;
    cpu.ir = 0x301;
    let mk: u32 = 1u32 << ((cpu.d[1]) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C16A7C: ori.b #$2, -(a4)
pub fn i_c16a7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A80;
    cpu.ir = 0x24;
    let ad1 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x2) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    14
}
/// $C16A80: ori.b #$1, d3
pub fn i_c16a80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A84;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C16A84: ori.b #$10, d4
pub fn i_c16a84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A88;
    cpu.ir = 0x4;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 | 0x10) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C16A88: dc.w $fc00
pub fn i_c16a88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A8A;
    cpu.ir = 0xFC00;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C16A8A: ori.b #$1, d3
pub fn i_c16a8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A8E;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C16A8C: or.b d1, d1
pub fn i_c16a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A8E;
    cpu.ir = 0x8201;
    let v1 = cpu.d[1] & 0xFF;
    let r = (v1 | cpu.d[1]) & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C16A8E: ori.b #$3, d6
pub fn i_c16a8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A92;
    cpu.ir = 0x6;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = (v1 | 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C16A92: or.b d1, d0
pub fn i_c16a92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A94;
    cpu.ir = 0x8001;
    let v1 = cpu.d[1] & 0xFF;
    let r = (v1 | cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C16A94: ori.b #$0, d6
pub fn i_c16a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A98;
    cpu.ir = 0x6;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C16A98: moveq #$0, d0
pub fn i_c16a98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A9A;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C16A9A: move.l d0, d1
pub fn i_c16a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A9C;
    cpu.ir = 0x2200;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[1] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C16A9C: move.l d0, d2
pub fn i_c16a9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16A9E;
    cpu.ir = 0x2400;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[2] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C16A9E: move.l d0, d3
pub fn i_c16a9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16AA0;
    cpu.ir = 0x2600;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[3] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C16AA0: move.b d0, $3a000f.l
pub fn i_c16aa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16AA6;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A000F, v1 as u8);
    16
}
/// $C16AA6: move.w #$7f, d7
pub fn i_c16aa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16AAA;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x7F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16AAA: lea.l $402000.l, a0
pub fn i_c16aaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16AB0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x402000;
    20
}
/// $C16AB0: movem.l d0-d3, -(a0)
pub fn i_c16ab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16AB4;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C16AB4: movem.l d0-d3, -(a0)
pub fn i_c16ab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16AB8;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C16AB8: movem.l d0-d3, -(a0)
pub fn i_c16ab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16ABC;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C16ABC: movem.l d0-d3, -(a0)
pub fn i_c16abc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16AC0;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C16AC0: dbra d7, $c16ab0
pub fn i_c16ac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16AC4;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC16AC4; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC16AC4; return 14; }
    cpu.pc = 0xC16AB0;
    10
}
/// $C16AC4: move.b d0, $3a001f.l
pub fn i_c16ac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16ACA;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001F, v1 as u8);
    16
}
/// $C16ACA: move.w #$7f, d7
pub fn i_c16aca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16ACE;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x7F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16ACC: ?
pub fn i_c16acc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16AD0;
    cpu.ir = 0x7F;
    let v1: u32 = 0x0;
    let r2 = (v1 | 0x41F9) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    8
}
/// $C16ACE: lea.l $402000.l, a0
pub fn i_c16ace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16AD4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x402000;
    20
}
/// $C16AD0: ori.w #$2000, d0
pub fn i_c16ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16AD4;
    cpu.ir = 0x40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 | 0x2000) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C16AD4: movem.l d0-d3, -(a0)
pub fn i_c16ad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16AD8;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C16AD8: movem.l d0-d3, -(a0)
pub fn i_c16ad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16ADC;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C16ADC: movem.l d0-d3, -(a0)
pub fn i_c16adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16AE0;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C16AE0: movem.l d0-d3, -(a0)
pub fn i_c16ae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16AE4;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C16AE4: dbra d7, $c16ad4
pub fn i_c16ae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16AE8;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC16AE8; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC16AE8; return 14; }
    cpu.pc = 0xC16AD4;
    10
}
/// $C16AE8: addq.b #$1, $ac2(a5)
pub fn i_c16ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16AEC;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C16AEC: jsr $c16ed0(pc)
pub fn i_c16aec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16AF0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16AF0);
    cpu.pc = 0xC16ED0;
    16
}
/// $C16AF0: jsr $c16f0c(pc)
pub fn i_c16af0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16AF4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16AF4);
    cpu.pc = 0xC16F0C;
    16
}
/// $C16AF4: move.l #$ddd0000, $400002.l
pub fn i_c16af4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16AFE;
    cpu.ir = 0x23FC;
    let v1: u32 = 0xDDD0000;
    bus.write32(0x400002, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C16AFE: lea.l $c1fa7e.l, a0
pub fn i_c16afe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B04;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0xC1FA7E;
    20
}
/// $C16B04: move.w #$40, d0
pub fn i_c16b04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B08;
    cpu.ir = 0x303C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16B08: move.b $115.w, d2
pub fn i_c16b08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B0C;
    cpu.ir = 0x1438;
    let v2 = bus.read8(0x115) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    12
}
/// $C16B0C: lsl.w #$8, d2
pub fn i_c16b0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B0E;
    cpu.ir = 0xE14A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C16B0E: move.w #$f01, d1
pub fn i_c16b0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B12;
    cpu.ir = 0x323C;
    let v1: u32 = 0xF01;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16B12: jsr $c16e7c(pc)
pub fn i_c16b12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B16;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16B16);
    cpu.pc = 0xC16E7C;
    16
}
/// $C16B16: lea.l $c1fac6.l, a0
pub fn i_c16b16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B1C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0xC1FAC6;
    20
}
/// $C16B1C: move.w #$4c0, d0
pub fn i_c16b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B20;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4C0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16B20: move.w #$f00, d1
pub fn i_c16b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B24;
    cpu.ir = 0x323C;
    let v1: u32 = 0xF00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16B24: jsr $c16e7c(pc)
pub fn i_c16b24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B28;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16B28);
    cpu.pc = 0xC16E7C;
    16
}
/// $C16B28: move.w #$0, -$2(a1)
pub fn i_c16b28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B2E;
    cpu.ir = 0x337C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0xFFFFFFFE);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16B2E: move.w #$2, $2(a1)
pub fn i_c16b2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B34;
    cpu.ir = 0x337C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16B34: moveq #$1f, d6
pub fn i_c16b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B36;
    cpu.ir = 0x7C1F;
    cpu.d[6] = 0x1F;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C16B36: move.w d2, (a1)
pub fn i_c16b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B38;
    cpu.ir = 0x3282;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v1 as u16);
    8
}
/// $C16B38: dbra d6, $c16b36
pub fn i_c16b38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B3C;
    cpu.ir = 0x51CE;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC16B3C; return 12; }
    let n = (cpu.d[6] as u16).wrapping_sub(1);
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC16B3C; return 14; }
    cpu.pc = 0xC16B36;
    10
}
/// $C16B3C: movea.l $abe(a5), a0
pub fn i_c16b3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B40;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C16B40: move.l #$c1fb0e, (a0)+
pub fn i_c16b40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B46;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC1FB0E;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C16B46: move.l a0, $abe(a5)
pub fn i_c16b46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B4A;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C16B4A: lea.l $948(a5), a0
pub fn i_c16b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B4E;
    cpu.ir = 0x41ED;
    let ad1 = cpu.a[5].wrapping_add(0x948);
    cpu.a[0] = ad1;
    16
}
/// $C16B4E: move.w #$e404, (a0)+
pub fn i_c16b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B52;
    cpu.ir = 0x30FC;
    let v1: u32 = 0xE404;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C16B52: moveq #$44, d0
pub fn i_c16b52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B54;
    cpu.ir = 0x7044;
    cpu.d[0] = 0x44;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C16B54: moveq #$22, d7
pub fn i_c16b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B56;
    cpu.ir = 0x7E22;
    cpu.d[7] = 0x22;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C16B56: move.w d0, (a0)+
pub fn i_c16b56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B58;
    cpu.ir = 0x30C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C16B58: dbra d7, $c16b56
pub fn i_c16b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B5C;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC16B5C; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC16B5C; return 14; }
    cpu.pc = 0xC16B56;
    10
}
/// $C16B5C: clr.w $96c(a5)
pub fn i_c16b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B60;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0x96C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $C16B60: move.w #$4b80, $900(a5)
pub fn i_c16b60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B66;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x4B80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x900);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16B66: move.w #$ff, d0
pub fn i_c16b66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B6A;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16B6A: lea.l $990(a5), a0
pub fn i_c16b6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B6E;
    cpu.ir = 0x41ED;
    let ad1 = cpu.a[5].wrapping_add(0x990);
    cpu.a[0] = ad1;
    16
}
/// $C16B6E: moveq #$11, d7
pub fn i_c16b6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B70;
    cpu.ir = 0x7E11;
    cpu.d[7] = 0x11;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C16B70: move.w d0, (a0)+
pub fn i_c16b70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B72;
    cpu.ir = 0x30C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C16B72: dbra d7, $c16b70
pub fn i_c16b72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B76;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC16B76; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC16B76; return 14; }
    cpu.pc = 0xC16B70;
    10
}
/// $C16B76: move.w #$fff, d0
pub fn i_c16b76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B7A;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16B7A: moveq #$11, d7
pub fn i_c16b7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B7C;
    cpu.ir = 0x7E11;
    cpu.d[7] = 0x11;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C16B7C: move.w d0, (a0)+
pub fn i_c16b7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B7E;
    cpu.ir = 0x30C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C16B7E: dbra d7, $c16b7c
pub fn i_c16b7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B82;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC16B82; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC16B82; return 14; }
    cpu.pc = 0xC16B7C;
    10
}
/// $C16B82: move.w #$0, d0
pub fn i_c16b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B86;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16B86: bsr.w $c16eb8
pub fn i_c16b86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B8A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0xC16B8A);
    cpu.pc = 0xC16EB8;
    18
}
/// $C16B8A: move.w #$0, $9dc(a5)
pub fn i_c16b8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B90;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9DC);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16B90: move.w #$1, $9d8(a5)
pub fn i_c16b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B96;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16B96: subq.b #$1, $ac2(a5)
pub fn i_c16b96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B9A;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C16B9A: rts
pub fn i_c16b9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16B9C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C16B9C: addq.w #$1, $9dc(a5)
pub fn i_c16b9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16BA0;
    cpu.ir = 0x526D;
    let ad1 = cpu.a[5].wrapping_add(0x9DC);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $C16BA0: move.w $9dc(a5), d0
pub fn i_c16ba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16BA4;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9DC);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C16BA4: cmpi.w #$5, d0
pub fn i_c16ba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16BA8;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x5, Size::Word, false);
    8
}
/// $C16BA8: bne.b $c16bc0
pub fn i_c16ba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16BAA;
    cpu.ir = 0x6616;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC16BC0; return 10; }
    8
}
/// $C16BAA: tst.b $a82(a5)
pub fn i_c16baa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16BAE;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xA82);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C16BAE: beq.b $c16bb8
pub fn i_c16bae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16BB0;
    cpu.ir = 0x6708;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC16BB8; return 10; }
    8
}
/// $C16BB0: tst.b $d00046.l
pub fn i_c16bb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16BB6;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0xD00046) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $C16BB6: bne.b $c16bc0
pub fn i_c16bb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16BB8;
    cpu.ir = 0x6608;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC16BC0; return 10; }
    8
}
/// $C16BB8: move.b #$2, $320000.l
pub fn i_c16bb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16BC0;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x320000, v1 as u8);
    16
}
/// $C16BC0: andi.w #$1, d0
pub fn i_c16bc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16BC4;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C16BC4: bne.b $c16bfc
pub fn i_c16bc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16BC6;
    cpu.ir = 0x6636;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC16BFC; return 10; }
    8
}
/// $C16BC6: lea.l $990(a5), a0
pub fn i_c16bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16BCA;
    cpu.ir = 0x41ED;
    let ad1 = cpu.a[5].wrapping_add(0x990);
    cpu.a[0] = ad1;
    16
}
/// $C16BCA: move.w $900(a5), d1
pub fn i_c16bca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16BCE;
    cpu.ir = 0x322D;
    let ad2 = cpu.a[5].wrapping_add(0x900);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C16BCE: subi.w #$480, d1
pub fn i_c16bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16BD2;
    cpu.ir = 0x441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x480, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C16BD2: move.w d1, $900(a5)
pub fn i_c16bd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16BD6;
    cpu.ir = 0x3B41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x900);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16BD6: move.w (a0), d0
pub fn i_c16bd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16BD8;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C16BD8: addi.w #$100, d0
pub fn i_c16bd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16BDC;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C16BDC: moveq #$11, d7
pub fn i_c16bdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16BDE;
    cpu.ir = 0x7E11;
    cpu.d[7] = 0x11;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C16BDE: move.w d0, (a0)+
pub fn i_c16bde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16BE0;
    cpu.ir = 0x30C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C16BE0: dbra d7, $c16bde
pub fn i_c16be0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16BE4;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC16BE4; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC16BE4; return 14; }
    cpu.pc = 0xC16BDE;
    10
}
/// $C16BE4: cmpi.w #$fff, d0
pub fn i_c16be4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16BE8;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFF, Size::Word, false);
    8
}
/// $C16BE8: bne.b $c16bfc
pub fn i_c16be8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16BEA;
    cpu.ir = 0x6612;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC16BFC; return 10; }
    8
}
/// $C16BEA: move.w #$0, $9da(a5)
pub fn i_c16bea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16BF0;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9DA);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16BF0: move.w #$a, $9dc(a5)
pub fn i_c16bf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16BF6;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9DC);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16BF6: move.w #$2, $9d8(a5)
pub fn i_c16bf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16BFC;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16BFC: rts
pub fn i_c16bfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16BFE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C16BFE: subq.w #$1, $9dc(a5)
pub fn i_c16bfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C02;
    cpu.ir = 0x536D;
    let ad1 = cpu.a[5].wrapping_add(0x9DC);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $C16C02: bne.b $c16c0a
pub fn i_c16c02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C04;
    cpu.ir = 0x6606;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC16C0A; return 10; }
    8
}
/// $C16C04: move.w #$3, $9d8(a5)
pub fn i_c16c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C0A;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16C0A: rts
pub fn i_c16c0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C0C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C16C0C: addq.w #$1, $9dc(a5)
pub fn i_c16c0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C10;
    cpu.ir = 0x526D;
    let ad1 = cpu.a[5].wrapping_add(0x9DC);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $C16C10: move.w $9dc(a5), d0
pub fn i_c16c10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C14;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9DC);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C16C14: andi.w #$1, d0
pub fn i_c16c14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C18;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C16C18: bne.b $c16c64
pub fn i_c16c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C1A;
    cpu.ir = 0x664A;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC16C64; return 10; }
    8
}
/// $C16C1A: jsr $c16e62(pc)
pub fn i_c16c1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C1E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16C1E);
    cpu.pc = 0xC16E62;
    16
}
/// $C16C1E: subi.w #$100, $948(a5)
pub fn i_c16c1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C24;
    cpu.ir = 0x46D;
    let ad1 = cpu.a[5].wrapping_add(0x948);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x100, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $C16C24: subi.b #$10, $991(a5)
pub fn i_c16c24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C2A;
    cpu.ir = 0x42D;
    let ad1 = cpu.a[5].wrapping_add(0x991);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x10, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $C16C2A: bcc.b $c16c64
pub fn i_c16c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C2C;
    cpu.ir = 0x6438;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC16C64; return 10; }
    8
}
/// $C16C2C: move.w #$f00, $9b4(a5)
pub fn i_c16c2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C32;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0xF00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9B4);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16C32: move.w $948(a5), $96c(a5)
pub fn i_c16c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C38;
    cpu.ir = 0x3B6D;
    let ad2 = cpu.a[5].wrapping_add(0x948);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0x96C);
    bus.write16(ad4, v3 as u16);
    20
}
/// $C16C38: move.w #$6404, $948(a5)
pub fn i_c16c38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C3E;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x6404;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x948);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16C3E: move.w #$800, $924(a5)
pub fn i_c16c3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C44;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x800;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x924);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16C44: move.w #$4, $9d8(a5)
pub fn i_c16c44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C4A;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16C4A: move.w #$0, $9dc(a5)
pub fn i_c16c4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C50;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9DC);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16C50: move.w #$0, $9de(a5)
pub fn i_c16c50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C56;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9DE);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16C56: move.w #$71b0, $9e0(a5)
pub fn i_c16c56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C5C;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x71B0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9E0);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16C5C: move.l #$c1fb4a, $9e2(a5)
pub fn i_c16c5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C64;
    cpu.ir = 0x2B7C;
    let v1: u32 = 0xC1FB4A;
    let ad2 = cpu.a[5].wrapping_add(0x9E2);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C16C64: rts
pub fn i_c16c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C66;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C16C66: addq.w #$1, $9dc(a5)
pub fn i_c16c66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C6A;
    cpu.ir = 0x526D;
    let ad1 = cpu.a[5].wrapping_add(0x9DC);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $C16C6A: move.w $9dc(a5), d0
pub fn i_c16c6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C6E;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9DC);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C16C6E: andi.w #$1, d0
pub fn i_c16c6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C72;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C16C72: bne.b $c16ca2
pub fn i_c16c72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C74;
    cpu.ir = 0x662E;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC16CA2; return 10; }
    8
}
/// $C16C74: jsr $c16e62(pc)
pub fn i_c16c74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C78;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16C78);
    cpu.pc = 0xC16E62;
    16
}
/// $C16C78: addi.w #$100, $96c(a5)
pub fn i_c16c78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C7E;
    cpu.ir = 0x66D;
    let ad1 = cpu.a[5].wrapping_add(0x96C);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x100, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $C16C7E: move.b $9b5(a5), d0
pub fn i_c16c7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C82;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0x9B5);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C16C82: bne.b $c16c88
pub fn i_c16c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C84;
    cpu.ir = 0x6604;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC16C88; return 10; }
    8
}
/// $C16C84: move.b #$ff, d0
pub fn i_c16c84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C88;
    cpu.ir = 0x103C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C16C88: addi.b #$10, d0
pub fn i_c16c88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C8C;
    cpu.ir = 0x600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C16C8C: move.b d0, $9b5(a5)
pub fn i_c16c8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C90;
    cpu.ir = 0x1B40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9B5);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C16C90: cmpi.b #$ff, d0
pub fn i_c16c90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C94;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $C16C94: bne.b $c16ca2
pub fn i_c16c94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C96;
    cpu.ir = 0x660C;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC16CA2; return 10; }
    8
}
/// $C16C96: move.w #$0, $9dc(a5)
pub fn i_c16c96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16C9C;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9DC);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16C9C: move.w #$5, $9d8(a5)
pub fn i_c16c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16CA2;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16CA2: rts
pub fn i_c16ca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16CA4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C16CA4: addq.w #$1, $9dc(a5)
pub fn i_c16ca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16CA8;
    cpu.ir = 0x526D;
    let ad1 = cpu.a[5].wrapping_add(0x9DC);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $C16CA8: move.w $9dc(a5), d0
pub fn i_c16ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16CAC;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9DC);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C16CAC: andi.w #$3, d0
pub fn i_c16cac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16CB0;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C16CB0: bne.w $c16d1c
pub fn i_c16cb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16CB4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC16D1C; return 10; }
    8
}
/// $C16CB4: addq.b #$1, $ac2(a5)
pub fn i_c16cb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16CB8;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C16CB8: movea.l $abe(a5), a0
pub fn i_c16cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16CBC;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C16CBC: move.l #$0, (a0)+
pub fn i_c16cbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16CC2;
    cpu.ir = 0x20FC;
    let v1: u32 = 0x0;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C16CC2: move.l #$3010002, (a0)+
pub fn i_c16cc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16CC8;
    cpu.ir = 0x20FC;
    let v1: u32 = 0x3010002;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C16CC8: move.w #$102, (a0)+
pub fn i_c16cc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16CCC;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x102;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C16CCC: move.w #$3, (a0)+
pub fn i_c16ccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16CD0;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C16CD0: move.w $9e0(a5), (a0)+
pub fn i_c16cd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16CD4;
    cpu.ir = 0x30ED;
    let ad2 = cpu.a[5].wrapping_add(0x9E0);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    16
}
/// $C16CD4: move.w #$4, (a0)+
pub fn i_c16cd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16CD8;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C16CD8: move.l $9e2(a5), (a0)+
pub fn i_c16cd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16CDC;
    cpu.ir = 0x20ED;
    let ad2 = cpu.a[5].wrapping_add(0x9E2);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C16CDC: move.w #$0, (a0)+
pub fn i_c16cdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16CE0;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C16CE0: move.l a0, $abe(a5)
pub fn i_c16ce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16CE4;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C16CE4: subq.b #$1, $ac2(a5)
pub fn i_c16ce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16CE8;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C16CE8: addq.l #$4, $9e2(a5)
pub fn i_c16ce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16CEC;
    cpu.ir = 0x58AD;
    let ad1 = cpu.a[5].wrapping_add(0x9E2);
    let v2 = bus.read32(ad1);
    let r3 = add_with_flags(cpu, v2, 4, Size::Long, true);
    bus.write32(ad1, r3);
    16
}
/// $C16CEC: addi.w #$20, $9e0(a5)
pub fn i_c16cec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16CF2;
    cpu.ir = 0x66D;
    let ad1 = cpu.a[5].wrapping_add(0x9E0);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $C16CF2: addq.w #$1, $9de(a5)
pub fn i_c16cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16CF6;
    cpu.ir = 0x526D;
    let ad1 = cpu.a[5].wrapping_add(0x9DE);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $C16CF6: move.w $9de(a5), d0
pub fn i_c16cf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16CFA;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9DE);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C16CFA: cmpi.w #$f, d0
pub fn i_c16cfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16CFE;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xF, Size::Word, false);
    8
}
/// $C16CFE: bne.b $c16d06
pub fn i_c16cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16D00;
    cpu.ir = 0x6606;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC16D06; return 10; }
    8
}
/// $C16D00: move.w #$7192, $9e0(a5)
pub fn i_c16d00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16D06;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x7192;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9E0);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16D06: move.w $9de(a5), d0
pub fn i_c16d06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16D0A;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9DE);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C16D0A: cmpi.w #$20, d0
pub fn i_c16d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16D0E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x20, Size::Word, false);
    8
}
/// $C16D0E: bne.b $c16d1c
pub fn i_c16d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16D10;
    cpu.ir = 0x660C;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC16D1C; return 10; }
    8
}
/// $C16D10: move.w #$6, $9d8(a5)
pub fn i_c16d10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16D16;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16D16: move.w #$14, $9dc(a5)
pub fn i_c16d16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16D1C;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9DC);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16D1C: rts
pub fn i_c16d1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16D1E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C16D1E: subq.w #$1, $9dc(a5)
pub fn i_c16d1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16D22;
    cpu.ir = 0x536D;
    let ad1 = cpu.a[5].wrapping_add(0x9DC);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $C16D22: bne.b $c16d36
pub fn i_c16d22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16D24;
    cpu.ir = 0x6612;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC16D36; return 10; }
    8
}
/// $C16D24: move.w #$7, $9d8(a5)
pub fn i_c16d24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16D2A;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16D2A: move.w #$0, $9dc(a5)
pub fn i_c16d2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16D30;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9DC);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16D30: move.w #$0, $9da(a5)
pub fn i_c16d30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16D36;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9DA);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16D36: rts
pub fn i_c16d36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16D38;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C16D38: move.w $9dc(a5), d0
pub fn i_c16d38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16D3C;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9DC);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C16D3C: addq.w #$1, d0
pub fn i_c16d3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16D3E;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C16D3E: move.w d0, $9dc(a5)
pub fn i_c16d3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16D42;
    cpu.ir = 0x3B40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9DC);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16D42: andi.w #$1, d0
pub fn i_c16d42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16D46;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C16D46: bne.b $c16d78
pub fn i_c16d46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16D48;
    cpu.ir = 0x6630;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC16D78; return 10; }
    8
}
/// $C16D48: move.w $9da(a5), d1
pub fn i_c16d48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16D4C;
    cpu.ir = 0x322D;
    let ad2 = cpu.a[5].wrapping_add(0x9DA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C16D4C: lea.l $c1f82e.l, a0
pub fn i_c16d4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16D52;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0xC1F82E;
    20
}
/// $C16D52: moveq #$0, d0
pub fn i_c16d52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16D54;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C16D54: move.b (a0, d1.w), d0
pub fn i_c16d54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16D58;
    cpu.ir = 0x1030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $C16D58: bsr.w $c16eb8
pub fn i_c16d58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16D5C;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0xC16D5C);
    cpu.pc = 0xC16EB8;
    18
}
/// $C16D5C: addq.w #$1, $9da(a5)
pub fn i_c16d5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16D60;
    cpu.ir = 0x526D;
    let ad1 = cpu.a[5].wrapping_add(0x9DA);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $C16D60: move.w $9da(a5), d0
pub fn i_c16d60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16D64;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9DA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C16D64: cmpi.w #$10, d0
pub fn i_c16d64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16D68;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x10, Size::Word, false);
    8
}
/// $C16D68: bne.w $c16d78
pub fn i_c16d68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16D6C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC16D78; return 10; }
    8
}
/// $C16D6C: move.w #$b4, $9dc(a5)
pub fn i_c16d6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16D72;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0xB4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9DC);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16D72: move.w #$8, $9d8(a5)
pub fn i_c16d72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16D78;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16D78: rts
pub fn i_c16d78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16D7A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C16D7A: subq.w #$1, $9dc(a5)
pub fn i_c16d7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16D7E;
    cpu.ir = 0x536D;
    let ad1 = cpu.a[5].wrapping_add(0x9DC);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $C16D7E: bne.b $c16d9c
pub fn i_c16d7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16D80;
    cpu.ir = 0x661C;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC16D9C; return 10; }
    8
}
/// $C16D80: tst.b $a82(a5)
pub fn i_c16d80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16D84;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xA82);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C16D84: beq.b $c16d8e
pub fn i_c16d84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16D86;
    cpu.ir = 0x6708;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC16D8E; return 10; }
    8
}
/// $C16D86: cmpi.b #$1, $a83(a5)
pub fn i_c16d86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16D8C;
    cpu.ir = 0xC2D;
    let ad2 = cpu.a[5].wrapping_add(0xA83);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $C16D8C: beq.b $c16d9e
pub fn i_c16d8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16D8E;
    cpu.ir = 0x6710;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC16D9E; return 10; }
    8
}
/// $C16D8E: tst.b $a83(a5)
pub fn i_c16d8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16D92;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xA83);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C16D92: beq.w $c16dfa
pub fn i_c16d92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16D96;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC16DFA; return 10; }
    8
}
/// $C16D96: jmp $c102d2.l
pub fn i_c16d96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16D9C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC102D2;
    8
}
/// $C16D9C: rts
pub fn i_c16d9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16D9E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C16D9E: jsr $c10fae(pc)
pub fn i_c16d9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16DA2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16DA2);
    cpu.pc = 0xC10FAE;
    16
}
/// $C16DA2: jsr $c1727e(pc)
pub fn i_c16da2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16DA6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16DA6);
    cpu.pc = 0xC1727E;
    16
}
/// $C16DA6: move.b #$ff, $b8d(a5)
pub fn i_c16da6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16DAC;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xB8D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C16DAC: move.b d0, $3a0003.l
pub fn i_c16dac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16DB2;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0003, v1 as u8);
    16
}
/// $C16DB2: move.b d0, $3a000b.l
pub fn i_c16db2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16DB8;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A000B, v1 as u8);
    16
}
/// $C16DB8: clr.l $400002.l
pub fn i_c16db8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16DBE;
    cpu.ir = 0x42B9;
    let v1 = bus.read32(0x400002);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write32(0x400002, r2);
    20
}
/// $C16DBE: lea.l $400020.l, a0
pub fn i_c16dbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16DC4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x400020;
    20
}
/// $C16DC4: lea.l $c1fd8a.l, a1
pub fn i_c16dc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16DCA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xC1FD8A;
    20
}
/// $C16DCA: moveq #$7, d7
pub fn i_c16dca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16DCC;
    cpu.ir = 0x7E07;
    cpu.d[7] = 0x7;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C16DCC: move.l (a1)+, (a0)+
pub fn i_c16dcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16DCE;
    cpu.ir = 0x20D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C16DCE: dbra d7, $c16dcc
pub fn i_c16dce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16DD2;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC16DD2; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC16DD2; return 14; }
    cpu.pc = 0xC16DCC;
    10
}
/// $C16DD2: jsr $c12a0e(pc)
pub fn i_c16dd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16DD6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16DD6);
    cpu.pc = 0xC12A0E;
    16
}
/// $C16DD6: addq.b #$1, $ac2(a5)
pub fn i_c16dd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16DDA;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C16DDA: movea.l $abe(a5), a0
pub fn i_c16dda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16DDE;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C16DDE: move.l #$c1fdaa, (a0)+
pub fn i_c16dde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16DE4;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC1FDAA;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C16DE4: move.l a0, $abe(a5)
pub fn i_c16de4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16DE8;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C16DE8: subq.b #$1, $ac2(a5)
pub fn i_c16de8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16DEC;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C16DEC: move.w #$12c, $9dc(a5)
pub fn i_c16dec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16DF2;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x12C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9DC);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16DF2: move.w #$9, $9d8(a5)
pub fn i_c16df2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16DF8;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16DF8: rts
pub fn i_c16df8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16DFA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C16DFA: move.l #$eee0000, $400002.l
pub fn i_c16dfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E04;
    cpu.ir = 0x23FC;
    let v1: u32 = 0xEEE0000;
    bus.write32(0x400002, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C16E04: jsr $c12a0e(pc)
pub fn i_c16e04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E08;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16E08);
    cpu.pc = 0xC12A0E;
    16
}
/// $C16E08: addq.b #$1, $ac2(a5)
pub fn i_c16e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E0C;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C16E0C: movea.l $abe(a5), a0
pub fn i_c16e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E10;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C16E10: move.l #$c1fe52, (a0)+
pub fn i_c16e10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E16;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC1FE52;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C16E16: bra.b $c16de4
pub fn i_c16e16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E18;
    cpu.ir = 0x60CC;
    cpu.pc = 0xC16DE4;
    10
}
/// $C16E18: subq.w #$1, $9dc(a5)
pub fn i_c16e18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E1C;
    cpu.ir = 0x536D;
    let ad1 = cpu.a[5].wrapping_add(0x9DC);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $C16E1C: beq.b $c16e20
pub fn i_c16e1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E1E;
    cpu.ir = 0x6702;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC16E20; return 10; }
    8
}
/// $C16E1E: rts
pub fn i_c16e1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E20;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C16E20: cmpi.w #$9, $9d8(a5)
pub fn i_c16e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E26;
    cpu.ir = 0xC6D;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x9, Size::Word, false);
    16
}
/// $C16E26: bne.w $c16e5c
pub fn i_c16e26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E2A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC16E5C; return 10; }
    8
}
/// $C16E2A: tst.b $a83(a5)
pub fn i_c16e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E2E;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xA83);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C16E2E: beq.w $c16e5c
pub fn i_c16e2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E32;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC16E5C; return 10; }
    8
}
/// $C16E32: clr.l $400002.l
pub fn i_c16e32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E38;
    cpu.ir = 0x42B9;
    let v1 = bus.read32(0x400002);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write32(0x400002, r2);
    20
}
/// $C16E38: jsr $c12a0e(pc)
pub fn i_c16e38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E3C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16E3C);
    cpu.pc = 0xC12A0E;
    16
}
/// $C16E3C: jsr $c10fae(pc)
pub fn i_c16e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E40;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16E40);
    cpu.pc = 0xC10FAE;
    16
}
/// $C16E40: jsr $c1727e(pc)
pub fn i_c16e40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E44;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16E44);
    cpu.pc = 0xC1727E;
    16
}
/// $C16E44: move.b #$3, $320000.l
pub fn i_c16e44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E4C;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x320000, v1 as u8);
    16
}
/// $C16E4C: move.b d0, $3a001b.l
pub fn i_c16e4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E52;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001B, v1 as u8);
    16
}
/// $C16E52: move.b d0, $3a0013.l
pub fn i_c16e52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E58;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0013, v1 as u8);
    16
}
/// $C16E58: clr.b $b8d(a5)
pub fn i_c16e58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E5C;
    cpu.ir = 0x422D;
    let ad1 = cpu.a[5].wrapping_add(0xB8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C16E5C: jmp $c102d2.l
pub fn i_c16e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E62;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC102D2;
    8
}
/// $C16E62: move.w $9da(a5), d1
pub fn i_c16e62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E66;
    cpu.ir = 0x322D;
    let ad2 = cpu.a[5].wrapping_add(0x9DA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C16E66: lea.l $c1f80e.l, a0
pub fn i_c16e66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E6C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0xC1F80E;
    20
}
/// $C16E6C: moveq #$0, d0
pub fn i_c16e6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E6E;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C16E6E: move.b (a0, d1.w), d0
pub fn i_c16e6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E72;
    cpu.ir = 0x1030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $C16E72: jsr $c16eb8(pc)
pub fn i_c16e72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E76;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16E76);
    cpu.pc = 0xC16EB8;
    16
}
/// $C16E76: addq.w #$1, $9da(a5)
pub fn i_c16e76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E7A;
    cpu.ir = 0x526D;
    let ad1 = cpu.a[5].wrapping_add(0x9DA);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $C16E7A: rts
pub fn i_c16e7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E7C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C16E7C: lea.l $3c0002.l, a1
pub fn i_c16e7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E82;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x3C0002;
    20
}
/// $C16E82: move.w #$1, $2(a1)
pub fn i_c16e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E88;
    cpu.ir = 0x337C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16E88: moveq #$11, d7
pub fn i_c16e88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E8A;
    cpu.ir = 0x7E11;
    cpu.d[7] = 0x11;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C16E8A: move.w d0, -$2(a1)
pub fn i_c16e8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E8E;
    cpu.ir = 0x3340;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0xFFFFFFFE);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16E8E: moveq #$3, d6
pub fn i_c16e8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E90;
    cpu.ir = 0x7C03;
    cpu.d[6] = 0x3;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C16E90: move.b (a0)+, d2
pub fn i_c16e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E92;
    cpu.ir = 0x1418;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C16E92: move.w d2, (a1)
pub fn i_c16e92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E94;
    cpu.ir = 0x3282;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v1 as u16);
    8
}
/// $C16E94: nop
pub fn i_c16e94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E96;
    cpu.ir = 0x4E71;
    4
}
/// $C16E96: move.w d1, (a1)
pub fn i_c16e96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E98;
    cpu.ir = 0x3281;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v1 as u16);
    8
}
/// $C16E98: dbra d6, $c16e90
pub fn i_c16e98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E9C;
    cpu.ir = 0x51CE;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC16E9C; return 12; }
    let n = (cpu.d[6] as u16).wrapping_sub(1);
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC16E9C; return 14; }
    cpu.pc = 0xC16E90;
    10
}
/// $C16E9C: moveq #$1b, d6
pub fn i_c16e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E9E;
    cpu.ir = 0x7C1B;
    cpu.d[6] = 0x1B;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C16E9E: move.b #$ff, d2
pub fn i_c16e9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16EA2;
    cpu.ir = 0x143C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C16EA2: move.w d2, (a1)
pub fn i_c16ea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16EA4;
    cpu.ir = 0x3282;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v1 as u16);
    8
}
/// $C16EA4: nop
pub fn i_c16ea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16EA6;
    cpu.ir = 0x4E71;
    4
}
/// $C16EA6: move.w #$0, (a1)
pub fn i_c16ea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16EAA;
    cpu.ir = 0x32BC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v1 as u16);
    8
}
/// $C16EAA: dbra d6, $c16ea2
pub fn i_c16eaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16EAE;
    cpu.ir = 0x51CE;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC16EAE; return 12; }
    let n = (cpu.d[6] as u16).wrapping_sub(1);
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC16EAE; return 14; }
    cpu.pc = 0xC16EA2;
    10
}
/// $C16EAE: addi.w #$40, d0
pub fn i_c16eae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16EB2;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x40, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C16EB2: dbra d7, $c16e8a
pub fn i_c16eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16EB6;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC16EB6; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC16EB6; return 14; }
    cpu.pc = 0xC16E8A;
    10
}
/// $C16EB6: rts
pub fn i_c16eb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16EB8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C16EB8: move.w #$c, d1
pub fn i_c16eb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16EBC;
    cpu.ir = 0x323C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16EBC: mulu.w d1, d0
pub fn i_c16ebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16EBE;
    cpu.ir = 0xC0C1;
    let v1 = cpu.d[1] & 0xFFFF;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[0] & 0xFFFF);
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $C16EBE: addi.l #$c1f83e, d0
pub fn i_c16ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16EC4;
    cpu.ir = 0x680;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = add_with_flags(cpu, v1, 0xC1F83E, Size::Long, true);
    cpu.d[0] = r2;
    8
}
/// $C16EC4: move.l d0, $9e8(a5)
pub fn i_c16ec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16EC8;
    cpu.ir = 0x2B40;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0x9E8);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C16EC8: move.w #$1, $9e6(a5)
pub fn i_c16ec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16ECE;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9E6);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16ECE: rts
pub fn i_c16ece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16ED0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C16ED0: lea.l $3c0002.l, a0
pub fn i_c16ed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16ED6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x3C0002;
    20
}
/// $C16ED6: move.w #$1, $2(a0)
pub fn i_c16ed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16EDC;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16EDC: move.w #$7020, -$2(a0)
pub fn i_c16edc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16EE2;
    cpu.ir = 0x317C;
    let v1: u32 = 0x7020;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFE);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16EE2: move.w #$ff, d0
pub fn i_c16ee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16EE6;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16EE6: move.w #$4bf, d1
pub fn i_c16ee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16EEA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4BF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16EEA: jsr $c16f04(pc)
pub fn i_c16eea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16EEE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16EEE);
    cpu.pc = 0xC16F04;
    16
}
/// $C16EEE: move.w #$20, d0
pub fn i_c16eee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16EF2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16EF2: move.w #$1f, d1
pub fn i_c16ef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16EF6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16EF6: jsr $c16f04(pc)
pub fn i_c16ef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16EFA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16EFA);
    cpu.pc = 0xC16F04;
    16
}
/// $C16EFA: move.w #$7000, -$2(a0)
pub fn i_c16efa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F00;
    cpu.ir = 0x317C;
    let v1: u32 = 0x7000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFE);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16F00: move.w #$1f, d1
pub fn i_c16f00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F04;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16F04: move.w d0, (a0)
pub fn i_c16f04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F06;
    cpu.ir = 0x3080;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v1 as u16);
    8
}
/// $C16F06: dbra d1, $c16f04
pub fn i_c16f06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F0A;
    cpu.ir = 0x51C9;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC16F0A; return 12; }
    let n = (cpu.d[1] as u16).wrapping_sub(1);
    cpu.d[1] = (cpu.d[1] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC16F0A; return 14; }
    cpu.pc = 0xC16F04;
    10
}
/// $C16F0A: rts
pub fn i_c16f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F0C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C16F0C: lea.l $3c0002.l, a0
pub fn i_c16f0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F12;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x3C0002;
    20
}
/// $C16F12: move.w #$1, $2(a0)
pub fn i_c16f12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F18;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16F18: move.w #$8000, -$2(a0)
pub fn i_c16f18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F1E;
    cpu.ir = 0x317C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFE);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16F1E: move.w #$fff, d0
pub fn i_c16f1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F22;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16F22: move.w #$1ff, d1
pub fn i_c16f22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F26;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16F26: jsr $c16f04(pc)
pub fn i_c16f26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F2A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16F2A);
    cpu.pc = 0xC16F04;
    16
}
/// $C16F2A: moveq #$0, d0
pub fn i_c16f2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F2C;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C16F2C: move.w #$1ff, d1
pub fn i_c16f2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F30;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16F30: jsr $c16f04(pc)
pub fn i_c16f30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F34;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16F34);
    cpu.pc = 0xC16F04;
    16
}
/// $C16F34: move.w #$be00, d0
pub fn i_c16f34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F38;
    cpu.ir = 0x303C;
    let v1: u32 = 0xBE00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16F38: move.w #$1ff, d1
pub fn i_c16f38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F3C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16F3C: jsr $c16f04(pc)
pub fn i_c16f3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F40;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16F40);
    cpu.pc = 0xC16F04;
    16
}
/// $C16F40: move.w #$0, -$2(a0)
pub fn i_c16f40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F46;
    cpu.ir = 0x317C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFE);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16F46: move.w #$ff, d0
pub fn i_c16f46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F4A;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16F4A: move.w #$2, $2(a0)
pub fn i_c16f4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F50;
    cpu.ir = 0x317C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16F50: moveq #$1f, d1
pub fn i_c16f50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F52;
    cpu.ir = 0x721F;
    cpu.d[1] = 0x1F;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C16F52: bra.b $c16f04
pub fn i_c16f52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F54;
    cpu.ir = 0x60B0;
    cpu.pc = 0xC16F04;
    10
}
/// $C16F54: move.w d0, d1
pub fn i_c16f54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F56;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16F56: lsr.b #$4, d1
pub fn i_c16f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F58;
    cpu.ir = 0xE809;
    let h1 = cpu.d[1] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, false);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    14
}
/// $C16F58: jsr $c16f62(pc)
pub fn i_c16f58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F5C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16F5C);
    cpu.pc = 0xC16F62;
    16
}
/// $C16F5C: exg.l d0, d1
pub fn i_c16f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F5E;
    cpu.ir = 0xC141;
    cpu.d.swap(0, 1);
    6
}
/// $C16F5E: andi.b #$f, d1
pub fn i_c16f5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F62;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C16F62: cmpi.b #$a, d1
pub fn i_c16f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F66;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0xA, Size::Byte, false);
    8
}
/// $C16F66: bcc.b $c16f6e
pub fn i_c16f66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F68;
    cpu.ir = 0x6406;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC16F6E; return 10; }
    8
}
/// $C16F68: addi.b #$30, d1
pub fn i_c16f68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F6C;
    cpu.ir = 0x601;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x30, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C16F6C: rts
pub fn i_c16f6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F6E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C16F6E: addi.b #$37, d1
pub fn i_c16f6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F72;
    cpu.ir = 0x601;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x37, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C16F72: rts
pub fn i_c16f72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F74;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C16F74: andi.b #$0, d2
pub fn i_c16f74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F78;
    cpu.ir = 0x202;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 & 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C16F78: ori.b #$0, d2
pub fn i_c16f78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F7C;
    cpu.ir = 0x2;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C16F7C: andi.b #$0, d0
pub fn i_c16f7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F80;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C16F80: ori.b #$0, d0
pub fn i_c16f80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F84;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C16F84: ori.b #$0, d0
pub fn i_c16f84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F88;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C16F88: btst.l d0, d0
pub fn i_c16f88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F8A;
    cpu.ir = 0x100;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C16F8A: btst.l d0, d0
pub fn i_c16f8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F8C;
    cpu.ir = 0x100;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C16F8C: ori.b #$1, d1
pub fn i_c16f8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F90;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C16F90: btst.l d0, d1
pub fn i_c16f90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F92;
    cpu.ir = 0x101;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C16F92: btst.l d0, d1
pub fn i_c16f92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F94;
    cpu.ir = 0x101;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C16F94: btst.l d1, d3
pub fn i_c16f94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F96;
    cpu.ir = 0x303;
    let mk: u32 = 1u32 << ((cpu.d[1]) % 32);
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C16F96: ori.b #$3, d0
pub fn i_c16f96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F9A;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C16F9A: ori.b #$0, d0
pub fn i_c16f9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F9E;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C16F9E: ori.b #$0, d0
pub fn i_c16f9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16FA2;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C16FA2: ori.b #$1, d0
pub fn i_c16fa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16FA6;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C16FA6: btst.l d1, d3
pub fn i_c16fa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16FA8;
    cpu.ir = 0x303;
    let mk: u32 = 1u32 << ((cpu.d[1]) % 32);
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C16FA8: btst.l d0, d1
pub fn i_c16fa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16FAA;
    cpu.ir = 0x101;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C16FAA: btst.l d0, d3
pub fn i_c16faa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16FAC;
    cpu.ir = 0x103;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C16FAC: btst.l d0, d1
pub fn i_c16fac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16FAE;
    cpu.ir = 0x101;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C16FAE: btst.l d1, d1
pub fn i_c16fae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16FB0;
    cpu.ir = 0x301;
    let mk: u32 = 1u32 << ((cpu.d[1]) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C16FB0: btst.l d0, d1
pub fn i_c16fb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16FB2;
    cpu.ir = 0x101;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C16FB2: btst.l d0, d1
pub fn i_c16fb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16FB4;
    cpu.ir = 0x101;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C16FB4: btst.l d1, d3
pub fn i_c16fb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16FB6;
    cpu.ir = 0x303;
    let mk: u32 = 1u32 << ((cpu.d[1]) % 32);
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C16FB6: ori.b #$3, d0
pub fn i_c16fb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16FBA;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C16FBA: ori.b #$0, d0
pub fn i_c16fba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16FBE;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C16FBE: ori.b #$0, d0
pub fn i_c16fbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16FC2;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C16FC2: ori.b #$4, d0
pub fn i_c16fc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16FC6;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x4) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C16FC6: ori.b #$4, d0
pub fn i_c16fc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16FCA;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x4) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C16FCA: ori.b #$0, d0
pub fn i_c16fca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16FCE;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C16FCE: ori.b #$0, d0
pub fn i_c16fce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16FD2;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C16FD2: ori.b #$f9, d0
pub fn i_c16fd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16FD6;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0xF9) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C16FD4: lea.l $10f300.l, a5
pub fn i_c16fd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16FDA;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x10F300;
    20
}
/// $C16FD6: ori.b #$0, (a0)
pub fn i_c16fd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16FDA;
    cpu.ir = 0x10;
    let ad1 = cpu.a[0];
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C16FDA: lea.l $c16f74(pc), a4
pub fn i_c16fda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16FDE;
    cpu.ir = 0x49FA;
    cpu.a[4] = 0xC16F74;
    16
}
/// $C16FDE: lea.l $380001.l, a2
pub fn i_c16fde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16FE4;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x380001;
    20
}
/// $C16FE4: moveq #$0, d0
pub fn i_c16fe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16FE6;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C16FE6: move.b d0, $a94(a5)
pub fn i_c16fe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16FEA;
    cpu.ir = 0x1B40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xA94);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C16FEA: move.b d0, $a9a(a5)
pub fn i_c16fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16FEE;
    cpu.ir = 0x1B40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xA9A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C16FEE: move.b d0, $aa0(a5)
pub fn i_c16fee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16FF2;
    cpu.ir = 0x1B40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAA0);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C16FF2: move.b d0, $aa6(a5)
pub fn i_c16ff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16FF6;
    cpu.ir = 0x1B40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAA6);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C16FF6: move.b $300001.l, d0
pub fn i_c16ff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16FFC;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x300001) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C16FFC: andi.b #$4, d0
pub fn i_c16ffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17000;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x4) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17000: bne.b $c17008
pub fn i_c17000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17002;
    cpu.ir = 0x6606;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC17008; return 10; }
    8
}
/// $C17002: move.b #$3, $a94(a5)
pub fn i_c17002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17008;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xA94);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C17008: move.b #$0, (a2)
pub fn i_c17008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1700C;
    cpu.ir = 0x14BC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[2], v1 as u8);
    8
}
/// $C1700C: rts
pub fn i_c1700c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1700E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1700E: move.b $300000.l, d4
pub fn i_c1700e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17014;
    cpu.ir = 0x1839;
    let v2 = bus.read8(0x300000) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C17014: move.b $340000.l, d5
pub fn i_c17014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1701A;
    cpu.ir = 0x1A39;
    let v2 = bus.read8(0x340000) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C1701A: move.b $380000.l, d6
pub fn i_c1701a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17020;
    cpu.ir = 0x1C39;
    let v2 = bus.read8(0x380000) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C17020: andi.b #$c0, d4
pub fn i_c17020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17024;
    cpu.ir = 0x204;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 & 0xC0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17024: andi.b #$c0, d5
pub fn i_c17024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17028;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0xC0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17028: andi.b #$f, d6
pub fn i_c17028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1702C;
    cpu.ir = 0x206;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1702C: or.b d6, d5
pub fn i_c1702c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1702E;
    cpu.ir = 0x8A06;
    let v1 = cpu.d[6] & 0xFF;
    let r = (v1 | cpu.d[5]) & 0xFF;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C1702E: lsl.b #$2, d6
pub fn i_c1702e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17030;
    cpu.ir = 0xE50E;
    let h1 = cpu.d[6] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Byte, 1, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r & 0xFF);
    10
}
/// $C17030: or.b d6, d4
pub fn i_c17030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17032;
    cpu.ir = 0x8806;
    let v1 = cpu.d[6] & 0xFF;
    let r = (v1 | cpu.d[4]) & 0xFF;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C17032: moveq #$0, d0
pub fn i_c17032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17034;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C17034: and.b d2, d4
pub fn i_c17034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17036;
    cpu.ir = 0xC802;
    let v1 = cpu.d[2] & 0xFF;
    let r = (v1 & cpu.d[4]) & 0xFF;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C17036: beq.b $c1703a
pub fn i_c17036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17038;
    cpu.ir = 0x6702;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1703A; return 10; }
    8
}
/// $C17038: moveq #$4, d0
pub fn i_c17038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1703A;
    cpu.ir = 0x7004;
    cpu.d[0] = 0x4;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1703A: and.b d2, d5
pub fn i_c1703a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1703C;
    cpu.ir = 0xCA02;
    let v1 = cpu.d[2] & 0xFF;
    let r = (v1 & cpu.d[5]) & 0xFF;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C1703C: beq.b $c17042
pub fn i_c1703c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1703E;
    cpu.ir = 0x6704;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC17042; return 10; }
    8
}
/// $C1703E: ori.b #$8, d0
pub fn i_c1703e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17042;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x8) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17042: lea.l $a94(a5), a3
pub fn i_c17042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17046;
    cpu.ir = 0x47ED;
    let ad1 = cpu.a[5].wrapping_add(0xA94);
    cpu.a[3] = ad1;
    16
}
/// $C17046: moveq #$3, d7
pub fn i_c17046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17048;
    cpu.ir = 0x7E03;
    cpu.d[7] = 0x3;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C17048: tst.b (a3)
pub fn i_c17048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1704A;
    cpu.ir = 0x4A13;
    let v2 = bus.read8(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    8
}
/// $C1704A: bne.b $c17050
pub fn i_c1704a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1704C;
    cpu.ir = 0x6604;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC17050; return 10; }
    8
}
/// $C1704C: move.b (a4, d0.w), (a3)
pub fn i_c1704c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17050;
    cpu.ir = 0x16B4;
    let ad2 = cpu.a[4].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[3], v3 as u8);
    18
}
/// $C17050: addq.w #$1, d0
pub fn i_c17050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17052;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C17052: addq.l #$6, a3
pub fn i_c17052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17054;
    cpu.ir = 0x5C8B;
    cpu.a[3] = cpu.a[3].wrapping_add(6);
    8
}
/// $C17054: dbra d7, $c17048
pub fn i_c17054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17058;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC17058; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC17058; return 14; }
    cpu.pc = 0xC17048;
    10
}
/// $C17058: lea.l $10(a4), a4
pub fn i_c17058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1705C;
    cpu.ir = 0x49EC;
    let ad1 = cpu.a[4].wrapping_add(0x10);
    cpu.a[4] = ad1;
    16
}
/// $C1705C: rts
pub fn i_c1705c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1705E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1705E: bsr.w $c1706a
pub fn i_c1705e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17062;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0xC17062);
    cpu.pc = 0xC1706A;
    18
}
/// $C17062: bsr.w $c1706a
pub fn i_c17062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17066;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0xC17066);
    cpu.pc = 0xC1706A;
    18
}
/// $C17066: bsr.w $c17072
pub fn i_c17066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1706A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0xC1706A);
    cpu.pc = 0xC17072;
    18
}
/// $C1706A: bsr.w $c17072
pub fn i_c1706a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1706E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0xC1706E);
    cpu.pc = 0xC17072;
    18
}
/// $C1706E: bsr.w $c17072
pub fn i_c1706e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17072;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0xC17072);
    cpu.pc = 0xC17072;
    18
}
/// $C17072: nop
pub fn i_c17072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17074;
    cpu.ir = 0x4E71;
    4
}
/// $C17074: nop
pub fn i_c17074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17076;
    cpu.ir = 0x4E71;
    4
}
/// $C17076: nop
pub fn i_c17076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17078;
    cpu.ir = 0x4E71;
    4
}
/// $C17078: nop
pub fn i_c17078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1707A;
    cpu.ir = 0x4E71;
    4
}
/// $C1707A: nop
pub fn i_c1707a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1707C;
    cpu.ir = 0x4E71;
    4
}
/// $C1707C: nop
pub fn i_c1707c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1707E;
    cpu.ir = 0x4E71;
    4
}
/// $C1707E: rts
pub fn i_c1707e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17080;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C17080: lea.l $10f300.l, a5
pub fn i_c17080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17086;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x10F300;
    20
}
/// $C17086: lea.l $a94(a5), a0
pub fn i_c17086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1708A;
    cpu.ir = 0x41ED;
    let ad1 = cpu.a[5].wrapping_add(0xA94);
    cpu.a[0] = ad1;
    16
}
/// $C1708A: lea.l $380001.l, a2
pub fn i_c1708a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17090;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x380001;
    20
}
/// $C17090: moveq #$0, d1
pub fn i_c17090(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17092;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C17092: move.b #$12, (a2)
pub fn i_c17092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17096;
    cpu.ir = 0x14BC;
    let v1: u32 = 0x12;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[2], v1 as u8);
    8
}
/// $C17096: bsr.b $c17066
pub fn i_c17096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17098;
    cpu.ir = 0x61CE;
    cpu.push32(bus, 0xC17098);
    cpu.pc = 0xC17066;
    18
}
/// $C17098: move.b $380000.l, d1
pub fn i_c17098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1709E;
    cpu.ir = 0x1239;
    let v2 = bus.read8(0x380000) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C1709E: move.b $340000.l, d5
pub fn i_c1709e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC170A4;
    cpu.ir = 0x1A39;
    let v2 = bus.read8(0x340000) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C170A4: swap d5
pub fn i_c170a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC170A6;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C170A6: move.b $300000.l, d5
pub fn i_c170a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC170AC;
    cpu.ir = 0x1A39;
    let v2 = bus.read8(0x300000) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C170AC: lsl.b #$4, d1
pub fn i_c170ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC170AE;
    cpu.ir = 0xE909;
    let h1 = cpu.d[1] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    14
}
/// $C170AE: move.b #$1b, (a2)
pub fn i_c170ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC170B2;
    cpu.ir = 0x14BC;
    let v1: u32 = 0x1B;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[2], v1 as u8);
    8
}
/// $C170B2: bsr.b $c17066
pub fn i_c170b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC170B4;
    cpu.ir = 0x61B2;
    cpu.push32(bus, 0xC170B4);
    cpu.pc = 0xC17066;
    18
}
/// $C170B4: move.b $380000.l, d2
pub fn i_c170b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC170BA;
    cpu.ir = 0x1439;
    let v2 = bus.read8(0x380000) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C170BA: move.b $340000.l, d4
pub fn i_c170ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC170C0;
    cpu.ir = 0x1839;
    let v2 = bus.read8(0x340000) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C170C0: swap d4
pub fn i_c170c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC170C2;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C170C2: move.b $300000.l, d4
pub fn i_c170c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC170C8;
    cpu.ir = 0x1839;
    let v2 = bus.read8(0x300000) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C170C8: andi.b #$f, d2
pub fn i_c170c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC170CC;
    cpu.ir = 0x202;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C170CC: or.b d1, d2
pub fn i_c170cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC170CE;
    cpu.ir = 0x8401;
    let v1 = cpu.d[1] & 0xFF;
    let r = (v1 | cpu.d[2]) & 0xFF;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C170CE: not.b d2
pub fn i_c170ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC170D0;
    cpu.ir = 0x4602;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = !v1 & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C170D0: move.b $10fedc.l, d3
pub fn i_c170d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC170D6;
    cpu.ir = 0x1639;
    let v2 = bus.read8(0x10FEDC) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C170D6: eor.b d2, d3
pub fn i_c170d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC170D8;
    cpu.ir = 0xB503;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 ^ cpu.d[2]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C170D8: and.b d2, d3
pub fn i_c170d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC170DA;
    cpu.ir = 0xC602;
    let v1 = cpu.d[2] & 0xFF;
    let r = (v1 & cpu.d[3]) & 0xFF;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C170DA: cmpi.b #$3, $a94(a5)
pub fn i_c170da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC170E0;
    cpu.ir = 0xC2D;
    let ad2 = cpu.a[5].wrapping_add(0xA94);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $C170E0: beq.b $c170ea
pub fn i_c170e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC170E2;
    cpu.ir = 0x6708;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC170EA; return 10; }
    8
}
/// $C170E2: cmpi.b #$3, $a9a(a5)
pub fn i_c170e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC170E8;
    cpu.ir = 0xC2D;
    let ad2 = cpu.a[5].wrapping_add(0xA9A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $C170E8: bne.b $c170ee
pub fn i_c170e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC170EA;
    cpu.ir = 0x6604;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC170EE; return 10; }
    8
}
/// $C170EA: jsr $c1724c(pc)
pub fn i_c170ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC170EE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC170EE);
    cpu.pc = 0xC1724C;
    16
}
/// $C170EE: move.b #$cf, d7
pub fn i_c170ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC170F2;
    cpu.ir = 0x1E3C;
    let v1: u32 = 0xCF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C170F2: jsr $c171c8(pc)
pub fn i_c170f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC170F6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC170F6);
    cpu.pc = 0xC171C8;
    16
}
/// $C170F6: swap d4
pub fn i_c170f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC170F8;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C170F8: swap d5
pub fn i_c170f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC170FA;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C170FA: swap d6
pub fn i_c170fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC170FC;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C170FC: lea.l $a9a(a5), a0
pub fn i_c170fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17100;
    cpu.ir = 0x41ED;
    let ad1 = cpu.a[5].wrapping_add(0xA9A);
    cpu.a[0] = ad1;
    16
}
/// $C17100: move.b #$3f, d7
pub fn i_c17100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17104;
    cpu.ir = 0x1E3C;
    let v1: u32 = 0x3F;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C17104: jsr $c171c8(pc)
pub fn i_c17104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17108;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC17108);
    cpu.pc = 0xC171C8;
    16
}
/// $C17108: lea.l $a94(a5), a0
pub fn i_c17108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1710C;
    cpu.ir = 0x41ED;
    let ad1 = cpu.a[5].wrapping_add(0xA94);
    cpu.a[0] = ad1;
    16
}
/// $C1710C: moveq #$1, d0
pub fn i_c1710c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1710E;
    cpu.ir = 0x7001;
    cpu.d[0] = 0x1;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1710E: jsr $c171a0(pc)
pub fn i_c1710e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17112;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC17112);
    cpu.pc = 0xC171A0;
    16
}
/// $C17112: lea.l $a9a(a5), a0
pub fn i_c17112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17116;
    cpu.ir = 0x41ED;
    let ad1 = cpu.a[5].wrapping_add(0xA9A);
    cpu.a[0] = ad1;
    16
}
/// $C17116: moveq #$3, d0
pub fn i_c17116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17118;
    cpu.ir = 0x7003;
    cpu.d[0] = 0x3;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C17118: jsr $c171a0(pc)
pub fn i_c17118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1711C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1711C);
    cpu.pc = 0xC171A0;
    16
}
/// $C1711C: move.b d2, $bdc(a5)
pub fn i_c1711c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17120;
    cpu.ir = 0x1B42;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xBDC);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C17120: move.b d3, $bdd(a5)
pub fn i_c17120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17124;
    cpu.ir = 0x1B43;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xBDD);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C17124: andi.b #$55, d2
pub fn i_c17124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17128;
    cpu.ir = 0x202;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 & 0x55) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17128: andi.b #$55, d3
pub fn i_c17128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1712C;
    cpu.ir = 0x203;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 & 0x55) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1712C: move.b d2, $aac(a5)
pub fn i_c1712c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17130;
    cpu.ir = 0x1B42;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAAC);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C17130: move.b d3, $aad(a5)
pub fn i_c17130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17134;
    cpu.ir = 0x1B43;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAAD);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C17134: move.b #$0, $380001.l
pub fn i_c17134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1713C;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x380001, v1 as u8);
    16
}
/// $C1713C: lea.l $a94(a5), a0
pub fn i_c1713c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17140;
    cpu.ir = 0x41ED;
    let ad1 = cpu.a[5].wrapping_add(0xA94);
    cpu.a[0] = ad1;
    16
}
/// $C17140: lea.l $be8(a5), a1
pub fn i_c17140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17144;
    cpu.ir = 0x43ED;
    let ad1 = cpu.a[5].wrapping_add(0xBE8);
    cpu.a[1] = ad1;
    16
}
/// $C17144: jsr $c17150(pc)
pub fn i_c17144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17148;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC17148);
    cpu.pc = 0xC17150;
    16
}
/// $C17148: lea.l $a9a(a5), a0
pub fn i_c17148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1714C;
    cpu.ir = 0x41ED;
    let ad1 = cpu.a[5].wrapping_add(0xA9A);
    cpu.a[0] = ad1;
    16
}
/// $C1714C: lea.l $bee(a5), a1
pub fn i_c1714c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17150;
    cpu.ir = 0x43ED;
    let ad1 = cpu.a[5].wrapping_add(0xBEE);
    cpu.a[1] = ad1;
    16
}
/// $C17150: cmpi.b #$3, (a0)
pub fn i_c17150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17154;
    cpu.ir = 0xC10;
    let v2 = bus.read8(cpu.a[0]) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    12
}
/// $C17154: beq.b $c17160
pub fn i_c17154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17156;
    cpu.ir = 0x670A;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC17160; return 10; }
    8
}
/// $C17156: move.l (a0), (a1)
pub fn i_c17156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17158;
    cpu.ir = 0x2290;
    let v2 = bus.read32(cpu.a[0]);
    bus.write32(cpu.a[1], v2);
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C17158: move.w $4(a0), $4(a1)
pub fn i_c17158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1715E;
    cpu.ir = 0x3368;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1].wrapping_add(0x4);
    bus.write16(ad4, v3 as u16);
    20
}
/// $C1715E: rts
pub fn i_c1715e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17160;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C17160: move.b #$1, (a1)
pub fn i_c17160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17164;
    cpu.ir = 0x12BC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[1], v1 as u8);
    8
}
/// $C17164: move.b $1(a0), d0
pub fn i_c17164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17168;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C17168: move.b $2(a0), d1
pub fn i_c17168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1716C;
    cpu.ir = 0x1228;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C1716C: jsr $c1718e(pc)
pub fn i_c1716c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17170;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC17170);
    cpu.pc = 0xC1718E;
    16
}
/// $C17170: move.b d0, $1(a1)
pub fn i_c17170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17174;
    cpu.ir = 0x1340;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x1);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C17174: move.b d0, $2(a1)
pub fn i_c17174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17178;
    cpu.ir = 0x1340;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C17178: move.b $d(a0), d0
pub fn i_c17178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1717C;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0xD);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C1717C: move.b $e(a0), d1
pub fn i_c1717c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17180;
    cpu.ir = 0x1228;
    let ad2 = cpu.a[0].wrapping_add(0xE);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C17180: jsr $c1718e(pc)
pub fn i_c17180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17184;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC17184);
    cpu.pc = 0xC1718E;
    16
}
/// $C17184: move.b d0, $3(a1)
pub fn i_c17184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17188;
    cpu.ir = 0x1340;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x3);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C17188: move.b d0, $4(a1)
pub fn i_c17188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1718C;
    cpu.ir = 0x1340;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C1718C: rts
pub fn i_c1718c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1718E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1718E: bclr.b #$7, d0
pub fn i_c1718e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17192;
    cpu.ir = 0x880;
    let mk: u32 = 1u32 << ((7) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 & !mk;
    cpu.d[0] = r2;
    8
}
/// $C17192: btst.b #$0, d1
pub fn i_c17192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17196;
    cpu.ir = 0x801;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C17196: beq.b $c1719c
pub fn i_c17196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17198;
    cpu.ir = 0x6704;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1719C; return 10; }
    8
}
/// $C17198: bset.b #$7, d0
pub fn i_c17198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1719C;
    cpu.ir = 0x8C0;
    let mk: u32 = 1u32 << ((7) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 | mk;
    cpu.d[0] = r2;
    8
}
/// $C1719C: ror.b #$4, d0
pub fn i_c1719c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1719E;
    cpu.ir = 0xE818;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 3, false);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    14
}
/// $C1719E: rts
pub fn i_c1719e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171A0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C171A0: cmpi.b #$3, (a0)
pub fn i_c171a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171A4;
    cpu.ir = 0xC10;
    let v2 = bus.read8(cpu.a[0]) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    12
}
/// $C171A4: bne.b $c171c6
pub fn i_c171a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171A6;
    cpu.ir = 0x6620;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC171C6; return 10; }
    8
}
/// $C171A6: btst.l d0, d2
pub fn i_c171a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171A8;
    cpu.ir = 0x102;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C171A8: beq.b $c171c6
pub fn i_c171a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171AA;
    cpu.ir = 0x671C;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC171C6; return 10; }
    8
}
/// $C171AA: move.b $3(a0), d5
pub fn i_c171aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171AE;
    cpu.ir = 0x1A28;
    let ad2 = cpu.a[0].wrapping_add(0x3);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C171AE: andi.b #$1f, d5
pub fn i_c171ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171B2;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x1F) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C171B2: move.b $1(a0), d4
pub fn i_c171b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171B6;
    cpu.ir = 0x1828;
    let ad2 = cpu.a[0].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C171B6: and.b d5, d4
pub fn i_c171b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171B8;
    cpu.ir = 0xC805;
    let v1 = cpu.d[5] & 0xFF;
    let r = (v1 & cpu.d[4]) & 0xFF;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C171B8: bne.b $c171c2
pub fn i_c171b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171BA;
    cpu.ir = 0x6608;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC171C2; return 10; }
    8
}
/// $C171BA: move.b $2(a0), d4
pub fn i_c171ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171BE;
    cpu.ir = 0x1828;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C171BE: and.b d5, d4
pub fn i_c171be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171C0;
    cpu.ir = 0xC805;
    let v1 = cpu.d[5] & 0xFF;
    let r = (v1 & cpu.d[4]) & 0xFF;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C171C0: beq.b $c171c6
pub fn i_c171c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171C2;
    cpu.ir = 0x6704;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC171C6; return 10; }
    8
}
/// $C171C2: bclr.b d0, d2
pub fn i_c171c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171C4;
    cpu.ir = 0x182;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 & !mk;
    cpu.d[2] = r2;
    8
}
/// $C171C4: bclr.b d0, d3
pub fn i_c171c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171C6;
    cpu.ir = 0x183;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 & !mk;
    cpu.d[3] = r2;
    8
}
/// $C171C6: rts
pub fn i_c171c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171C8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C171C8: cmpi.b #$3, (a0)
pub fn i_c171c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171CC;
    cpu.ir = 0xC10;
    let v2 = bus.read8(cpu.a[0]) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    12
}
/// $C171CC: beq.b $c17226
pub fn i_c171cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171CE;
    cpu.ir = 0x6758;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC17226; return 10; }
    8
}
/// $C171CE: move.b d4, d0
pub fn i_c171ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171D0;
    cpu.ir = 0x1004;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C171D0: jsr $c171de(pc)
pub fn i_c171d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171D4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC171D4);
    cpu.pc = 0xC171DE;
    16
}
/// $C171D4: lea.l $c(a0), a0
pub fn i_c171d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171D8;
    cpu.ir = 0x41E8;
    let ad1 = cpu.a[0].wrapping_add(0xC);
    cpu.a[0] = ad1;
    16
}
/// $C171D8: tst.b (a0)
pub fn i_c171d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171DA;
    cpu.ir = 0x4A10;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    8
}
/// $C171DA: beq.b $c17218
pub fn i_c171da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171DC;
    cpu.ir = 0x673C;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC17218; return 10; }
    8
}
/// $C171DC: move.b d5, d0
pub fn i_c171dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171DE;
    cpu.ir = 0x1005;
    let v1 = cpu.d[5] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C171DE: not.b d0
pub fn i_c171de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171E0;
    cpu.ir = 0x4600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = !v1 & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C171E0: move.b $2(a0), d1
pub fn i_c171e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171E4;
    cpu.ir = 0x1228;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C171E4: cmp.b d0, d1
pub fn i_c171e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171E6;
    cpu.ir = 0xB200;
    let v1 = cpu.d[0] & 0xFF;
    let h1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $C171E6: bne.b $c171f6
pub fn i_c171e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171E8;
    cpu.ir = 0x660E;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC171F6; return 10; }
    8
}
/// $C171E8: subq.b #$1, $5(a0)
pub fn i_c171e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171EC;
    cpu.ir = 0x5328;
    let ad1 = cpu.a[0].wrapping_add(0x5);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C171EC: bpl.b $c17202
pub fn i_c171ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171EE;
    cpu.ir = 0x6A14;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0xC17202; return 10; }
    8
}
/// $C171EE: move.b #$8, $5(a0)
pub fn i_c171ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171F4;
    cpu.ir = 0x117C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x5);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C171F4: bra.b $c171fc
pub fn i_c171f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171F6;
    cpu.ir = 0x6006;
    cpu.pc = 0xC171FC;
    10
}
/// $C171F6: move.b #$10, $5(a0)
pub fn i_c171f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171FC;
    cpu.ir = 0x117C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x5);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C171FC: move.b d0, $4(a0)
pub fn i_c171fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17200;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C17200: bra.b $c17206
pub fn i_c17200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17202;
    cpu.ir = 0x6004;
    cpu.pc = 0xC17206;
    10
}
/// $C17202: clr.b $4(a0)
pub fn i_c17202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17206;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x4);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C17206: move.b d1, $1(a0)
pub fn i_c17206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1720A;
    cpu.ir = 0x1141;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x1);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C1720A: move.b d0, $2(a0)
pub fn i_c1720a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1720E;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C1720E: eor.b d0, d1
pub fn i_c1720e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17210;
    cpu.ir = 0xB101;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 ^ cpu.d[0]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C17210: and.b d0, d1
pub fn i_c17210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17212;
    cpu.ir = 0xC200;
    let v1 = cpu.d[0] & 0xFF;
    let r = (v1 & cpu.d[1]) & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C17212: move.b d1, $3(a0)
pub fn i_c17212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17216;
    cpu.ir = 0x1141;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x3);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C17216: rts
pub fn i_c17216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17218;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C17218: clr.b $1(a0)
pub fn i_c17218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1721C;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x1);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1721C: clr.l $2(a0)
pub fn i_c1721c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17220;
    cpu.ir = 0x42A8;
    let ad1 = cpu.a[0].wrapping_add(0x2);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $C17220: and.b d7, d2
pub fn i_c17220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17222;
    cpu.ir = 0xC407;
    let v1 = cpu.d[7] & 0xFF;
    let r = (v1 & cpu.d[2]) & 0xFF;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C17222: and.b d7, d3
pub fn i_c17222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17224;
    cpu.ir = 0xC607;
    let v1 = cpu.d[7] & 0xFF;
    let r = (v1 & cpu.d[3]) & 0xFF;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C17224: rts
pub fn i_c17224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17226;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C17226: and.b d7, d2
pub fn i_c17226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17228;
    cpu.ir = 0xC407;
    let v1 = cpu.d[7] & 0xFF;
    let r = (v1 & cpu.d[2]) & 0xFF;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C17228: and.b d7, d3
pub fn i_c17228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1722A;
    cpu.ir = 0xC607;
    let v1 = cpu.d[7] & 0xFF;
    let r = (v1 & cpu.d[3]) & 0xFF;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C1722A: move.b d6, d0
pub fn i_c1722a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1722C;
    cpu.ir = 0x1006;
    let v1 = cpu.d[6] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C1722C: jsr $c1723a(pc)
pub fn i_c1722c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17230;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC17230);
    cpu.pc = 0xC1723A;
    16
}
/// $C17230: move.b d5, d0
pub fn i_c17230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17232;
    cpu.ir = 0x1005;
    let v1 = cpu.d[5] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C17232: jsr $c1723a(pc)
pub fn i_c17232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17236;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC17236);
    cpu.pc = 0xC1723A;
    16
}
/// $C17236: lsr.w #$8, d6
pub fn i_c17236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17238;
    cpu.ir = 0xE04E;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C17238: move.b d6, d0
pub fn i_c17238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1723A;
    cpu.ir = 0x1006;
    let v1 = cpu.d[6] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C1723A: addq.l #$1, a0
pub fn i_c1723a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1723C;
    cpu.ir = 0x5288;
    cpu.a[0] = cpu.a[0].wrapping_add(1);
    8
}
/// $C1723C: not.b d0
pub fn i_c1723c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1723E;
    cpu.ir = 0x4600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = !v1 & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C1723E: move.b (a0), d1
pub fn i_c1723e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17240;
    cpu.ir = 0x1210;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C17240: move.b d0, (a0)
pub fn i_c17240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17242;
    cpu.ir = 0x1080;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[0], v1 as u8);
    8
}
/// $C17242: eor.b d0, d1
pub fn i_c17242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17244;
    cpu.ir = 0xB101;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 ^ cpu.d[0]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C17244: and.b d0, d1
pub fn i_c17244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17246;
    cpu.ir = 0xC200;
    let v1 = cpu.d[0] & 0xFF;
    let r = (v1 & cpu.d[1]) & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C17246: move.b d1, $c(a0)
pub fn i_c17246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1724A;
    cpu.ir = 0x1141;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C1724A: rts
pub fn i_c1724a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1724C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1724C: move.b #$24, (a2)
pub fn i_c1724c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17250;
    cpu.ir = 0x14BC;
    let v1: u32 = 0x24;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[2], v1 as u8);
    8
}
/// $C17250: bsr.w $c17066
pub fn i_c17250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17254;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0xC17254);
    cpu.pc = 0xC17066;
    18
}
/// $C17254: move.b $340000.l, d6
pub fn i_c17254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1725A;
    cpu.ir = 0x1C39;
    let v2 = bus.read8(0x340000) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C1725A: swap d6
pub fn i_c1725a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1725C;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1725C: move.b $300000.l, d6
pub fn i_c1725c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17262;
    cpu.ir = 0x1C39;
    let v2 = bus.read8(0x300000) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C17262: lsl.l #$8, d6
pub fn i_c17262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17264;
    cpu.ir = 0xE18E;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 1, true);
    cpu.d[6] = r;
    22
}
/// $C17264: move.b #$9, (a2)
pub fn i_c17264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17268;
    cpu.ir = 0x14BC;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[2], v1 as u8);
    8
}
/// $C17268: bsr.w $c17066
pub fn i_c17268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1726C;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0xC1726C);
    cpu.pc = 0xC17066;
    18
}
/// $C1726C: move.b $300000.l, d6
pub fn i_c1726c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17272;
    cpu.ir = 0x1C39;
    let v2 = bus.read8(0x300000) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C17272: swap d6
pub fn i_c17272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17274;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C17274: move.b $340000.l, d6
pub fn i_c17274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1727A;
    cpu.ir = 0x1C39;
    let v2 = bus.read8(0x340000) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C1727A: swap d6
pub fn i_c1727a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1727C;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1727C: rts
pub fn i_c1727c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1727E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1727E: move.b #$1, $10fe8c.l
pub fn i_c1727e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17286;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10FE8C, v1 as u8);
    16
}
/// $C17286: tst.b $10fe8c.l
pub fn i_c17286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1728C;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FE8C) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $C1728C: bne.b $c17286
pub fn i_c1728c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1728E;
    cpu.ir = 0x66F8;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC17286; return 10; }
    8
}
/// $C1728E: rts
pub fn i_c1728e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17290;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C17290: tst.b $10fdc2.l
pub fn i_c17290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17296;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FDC2) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $C17296: bne.b $c172d0
pub fn i_c17296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17298;
    cpu.ir = 0x6638;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC172D0; return 10; }
    8
}
/// $C17298: addq.b #$1, $10fdc2.l
pub fn i_c17298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1729E;
    cpu.ir = 0x5239;
    let v1 = bus.read8(0x10FDC2) as u32;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    bus.write8(0x10FDC2, r2 as u8);
    16
}
/// $C1729E: move.l #$10feca, $10fec6.l
pub fn i_c1729e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC172A8;
    cpu.ir = 0x23FC;
    let v1: u32 = 0x10FECA;
    bus.write32(0x10FEC6, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C172A8: lea.l $10ff00.l, a0
pub fn i_c172a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC172AE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10FF00;
    20
}
/// $C172AE: lea.l $3c0000.l, a6
pub fn i_c172ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC172B4;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0x3C0000;
    20
}
/// $C172B4: lea.l $2(a6), a4
pub fn i_c172b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC172B8;
    cpu.ir = 0x49EE;
    let ad1 = cpu.a[6].wrapping_add(0x2);
    cpu.a[4] = ad1;
    16
}
/// $C172B8: cmpa.l $10fdbe.l, a0
pub fn i_c172b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC172BE;
    cpu.ir = 0xB1F9;
    let v2 = bus.read32(0x10FDBE);
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v2, Size::Long, false);
    22
}
/// $C172BE: bcs.b $c172d2
pub fn i_c172be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC172C0;
    cpu.ir = 0x6512;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC172D2; return 10; }
    8
}
/// $C172C0: move.l #$10ff00, $10fdbe.l
pub fn i_c172c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC172CA;
    cpu.ir = 0x23FC;
    let v1: u32 = 0x10FF00;
    bus.write32(0x10FDBE, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C172CA: clr.b $10fdc2.l
pub fn i_c172ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC172D0;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10FDC2) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10FDC2, r2 as u8);
    16
}
/// $C172D0: rts
pub fn i_c172d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC172D2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C172D2: movea.l (a0)+, a1
pub fn i_c172d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC172D4;
    cpu.ir = 0x2258;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    12
}
/// $C172D4: move.l a1, d4
pub fn i_c172d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC172D6;
    cpu.ir = 0x2809;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.d[4] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C172D6: bne.b $c172da
pub fn i_c172d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC172D8;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC172DA; return 10; }
    8
}
/// $C172D8: movea.l a0, a1
pub fn i_c172d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC172DA;
    cpu.ir = 0x2248;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $C172DA: move.b (a1)+, d1
pub fn i_c172da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC172DC;
    cpu.ir = 0x1219;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C172DC: moveq #$0, d0
pub fn i_c172dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC172DE;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C172DE: move.b (a1)+, d0
pub fn i_c172de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC172E0;
    cpu.ir = 0x1019;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C172E0: cmpi.b #$e, d0
pub fn i_c172e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC172E4;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xE, Size::Byte, false);
    8
}
/// $C172E4: bcc.b $c172c0
pub fn i_c172e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC172E6;
    cpu.ir = 0x64DA;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC172C0; return 10; }
    8
}
/// $C172E6: lsl.w #$2, d0
pub fn i_c172e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC172E8;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $C172E8: movea.l $c172f0(pc, d0.w), a3
pub fn i_c172e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC172EC;
    cpu.ir = 0x267B;
    let ad2 = 0xC172F0_u32.wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[3] = v3;
    18
}
/// $C172EC: jsr (a3)
pub fn i_c172ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC172EE;
    cpu.ir = 0x4E93;
    cpu.push32(bus, 0xC172EE);
    cpu.jump_to(cpu.a[3]);
    16
}
/// $C172EE: bra.b $c172da
pub fn i_c172ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC172F0;
    cpu.ir = 0x60EA;
    cpu.pc = 0xC172DA;
    10
}
/// $C172F0: ?
pub fn i_c172f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC172F2;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C17328: or.l d4, d4
pub fn i_c17328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1732A;
    cpu.ir = 0x8884;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let r = (v1 | cpu.d[4]) & 0xFFFFFFFF;
    cpu.d[4] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1732A: bne.b $c1732e
pub fn i_c1732a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1732C;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1732E; return 10; }
    8
}
/// $C1732C: movea.l a1, a0
pub fn i_c1732c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1732E;
    cpu.ir = 0x2049;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C1732E: move.l #$c172b8, (a7)
pub fn i_c1732e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17334;
    cpu.ir = 0x2EBC;
    let v1: u32 = 0xC172B8;
    bus.write32(cpu.a[7], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C17334: rts
pub fn i_c17334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17336;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C17336: swap d1
pub fn i_c17336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17338;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C17338: move.l d1, d5
pub fn i_c17338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1733A;
    cpu.ir = 0x2A01;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.d[5] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1733A: move.w (a1)+, d5
pub fn i_c1733a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1733C;
    cpu.ir = 0x3A19;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $C1733C: rts
pub fn i_c1733c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1733E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1733E: ext.w d1
pub fn i_c1733e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17340;
    cpu.ir = 0x4881;
    let v = cpu.d[1] as i8 as i16 as u16 as u32; cpu.d[1] = (cpu.d[1] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $C17340: move.w d1, $2(a4)
pub fn i_c17340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17344;
    cpu.ir = 0x3941;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C17344: rts
pub fn i_c17344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17346;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C17346: move.w (a1)+, d6
pub fn i_c17346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17348;
    cpu.ir = 0x3C19;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $C17348: move.w d6, (a6)
pub fn i_c17348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1734A;
    cpu.ir = 0x3C86;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[6], v1 as u16);
    8
}
/// $C1734A: rts
pub fn i_c1734a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1734C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1734C: move.l (a1)+, d0
pub fn i_c1734c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1734E;
    cpu.ir = 0x2019;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C1734E: movea.l $10fec6.l, a3
pub fn i_c1734e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17354;
    cpu.ir = 0x2679;
    let v2 = bus.read32(0x10FEC6);
    cpu.a[3] = v2;
    20
}
/// $C17354: move.l a1, (a3)+
pub fn i_c17354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17356;
    cpu.ir = 0x26C9;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C17356: move.l a3, $10fec6.l
pub fn i_c17356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1735C;
    cpu.ir = 0x23CB;
    let v1 = cpu.a[3] & 0xFFFFFFFF;
    bus.write32(0x10FEC6, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C1735C: movea.l d0, a1
pub fn i_c1735c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1735E;
    cpu.ir = 0x2240;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $C1735E: rts
pub fn i_c1735e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17360;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C17360: movea.l $10fec6.l, a3
pub fn i_c17360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17366;
    cpu.ir = 0x2679;
    let v2 = bus.read32(0x10FEC6);
    cpu.a[3] = v2;
    20
}
/// $C17366: movea.l -(a3), a1
pub fn i_c17366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17368;
    cpu.ir = 0x2263;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    14
}
/// $C17368: move.l a3, $10fec6.l
pub fn i_c17368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1736E;
    cpu.ir = 0x23CB;
    let v1 = cpu.a[3] & 0xFFFFFFFF;
    bus.write32(0x10FEC6, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C1736E: rts
pub fn i_c1736e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17370;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C17370: move.w (a1)+, d0
pub fn i_c17370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17372;
    cpu.ir = 0x3019;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $C17372: move.w d0, (a4)
pub fn i_c17372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17374;
    cpu.ir = 0x3880;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[4], v1 as u16);
    8
}
/// $C17374: subq.b #$1, d1
pub fn i_c17374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17376;
    cpu.ir = 0x5301;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C17376: bne.b $c17372
pub fn i_c17376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17378;
    cpu.ir = 0x66FA;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC17372; return 10; }
    8
}
/// $C17378: rts
pub fn i_c17378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1737A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1737A: move.w (a1)+, d0
pub fn i_c1737a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1737C;
    cpu.ir = 0x3019;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $C1737C: move.w d0, (a4)
pub fn i_c1737c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1737E;
    cpu.ir = 0x3880;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[4], v1 as u16);
    8
}
/// $C1737E: addq.b #$1, d0
pub fn i_c1737e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17380;
    cpu.ir = 0x5200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C17380: subq.b #$1, d1
pub fn i_c17380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17382;
    cpu.ir = 0x5301;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C17382: bne.b $c1737c
pub fn i_c17382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17384;
    cpu.ir = 0x66F8;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1737C; return 10; }
    8
}
/// $C17384: rts
pub fn i_c17384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17386;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C17386: movea.l a1, a2
pub fn i_c17386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17388;
    cpu.ir = 0x2449;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.a[2] = v1;
    4
}
/// $C17388: moveq #$0, d3
pub fn i_c17388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1738A;
    cpu.ir = 0x7600;
    cpu.d[3] = 0x0;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1738A: bra.b $c17390
pub fn i_c1738a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1738C;
    cpu.ir = 0x6004;
    cpu.pc = 0xC17390;
    10
}
/// $C1738C: movea.l (a1)+, a2
pub fn i_c1738c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1738E;
    cpu.ir = 0x2459;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    12
}
/// $C1738E: moveq #$1, d3
pub fn i_c1738e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17390;
    cpu.ir = 0x7601;
    cpu.d[3] = 0x1;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C17390: bsr.w $c173aa
pub fn i_c17390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17394;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0xC17394);
    cpu.pc = 0xC173AA;
    18
}
/// $C17394: or.b d3, d3
pub fn i_c17394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17396;
    cpu.ir = 0x8603;
    let v1 = cpu.d[3] & 0xFF;
    let r = (v1 | cpu.d[3]) & 0xFF;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C17396: bne.b $c173a2
pub fn i_c17396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17398;
    cpu.ir = 0x660A;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC173A2; return 10; }
    8
}
/// $C17398: move.l a2, d0
pub fn i_c17398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1739A;
    cpu.ir = 0x200A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1739A: addq.l #$1, d0
pub fn i_c1739a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1739C;
    cpu.ir = 0x5280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Long, true);
    cpu.d[0] = r2;
    4
}
/// $C1739C: andi.w #$fffe, d0
pub fn i_c1739c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173A0;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFFFE) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C173A0: movea.l d0, a1
pub fn i_c173a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173A2;
    cpu.ir = 0x2240;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $C173A2: rts
pub fn i_c173a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173A4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C173A4: add.w (a1)+, d6
pub fn i_c173a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173A6;
    cpu.ir = 0xDC59;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $C173A6: move.w d6, (a6)
pub fn i_c173a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173A8;
    cpu.ir = 0x3C86;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[6], v1 as u16);
    8
}
/// $C173A8: rts
pub fn i_c173a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173AA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C173AA: btst.b #$11, d5
pub fn i_c173aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173AE;
    cpu.ir = 0x805;
    let mk: u32 = 1u32 << ((17) % 32);
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C173AE: bne.b $c173ce
pub fn i_c173ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173B0;
    cpu.ir = 0x661E;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC173CE; return 10; }
    8
}
/// $C173B0: move.w d5, d2
pub fn i_c173b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173B2;
    cpu.ir = 0x3405;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C173B2: move.b d5, d7
pub fn i_c173b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173B4;
    cpu.ir = 0x1E05;
    let v1 = cpu.d[5] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C173B4: btst.b #$10, d5
pub fn i_c173b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173B8;
    cpu.ir = 0x805;
    let mk: u32 = 1u32 << ((16) % 32);
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C173B8: bne.b $c173c4
pub fn i_c173b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173BA;
    cpu.ir = 0x660A;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC173C4; return 10; }
    8
}
/// $C173BA: move.b (a2)+, d2
pub fn i_c173ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173BC;
    cpu.ir = 0x141A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C173BC: cmp.b d2, d7
pub fn i_c173bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173BE;
    cpu.ir = 0xBE02;
    let v1 = cpu.d[2] & 0xFF;
    let h1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $C173BE: beq.b $c173cc
pub fn i_c173be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173C0;
    cpu.ir = 0x670C;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC173CC; return 10; }
    8
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0xC15CFC, i_c15cfc),
    (0xC15CFE, i_c15cfe),
    (0xC15D00, i_c15d00),
    (0xC15D02, i_c15d02),
    (0xC15D04, i_c15d04),
    (0xC15D06, i_c15d06),
    (0xC15D16, i_c15d16),
    (0xC15D1A, i_c15d1a),
    (0xC15D1C, i_c15d1c),
    (0xC15D20, i_c15d20),
    (0xC15D24, i_c15d24),
    (0xC15D26, i_c15d26),
    (0xC15D28, i_c15d28),
    (0xC15D2A, i_c15d2a),
    (0xC15D2C, i_c15d2c),
    (0xC15D2E, i_c15d2e),
    (0xC15D30, i_c15d30),
    (0xC15D32, i_c15d32),
    (0xC15D34, i_c15d34),
    (0xC15D36, i_c15d36),
    (0xC15D38, i_c15d38),
    (0xC15D3A, i_c15d3a),
    (0xC15D3E, i_c15d3e),
    (0xC15D42, i_c15d42),
    (0xC15D44, i_c15d44),
    (0xC15D46, i_c15d46),
    (0xC15D48, i_c15d48),
    (0xC15D4A, i_c15d4a),
    (0xC15D4C, i_c15d4c),
    (0xC15D4E, i_c15d4e),
    (0xC15D50, i_c15d50),
    (0xC15D52, i_c15d52),
    (0xC15D54, i_c15d54),
    (0xC15D56, i_c15d56),
    (0xC15D58, i_c15d58),
    (0xC15D5A, i_c15d5a),
    (0xC15D5E, i_c15d5e),
    (0xC15D62, i_c15d62),
    (0xC15D64, i_c15d64),
    (0xC15D66, i_c15d66),
    (0xC15D68, i_c15d68),
    (0xC15D6A, i_c15d6a),
    (0xC15D6C, i_c15d6c),
    (0xC15D6E, i_c15d6e),
    (0xC15D74, i_c15d74),
    (0xC15D78, i_c15d78),
    (0xC15D7A, i_c15d7a),
    (0xC15D7E, i_c15d7e),
    (0xC15D82, i_c15d82),
    (0xC15D84, i_c15d84),
    (0xC15D86, i_c15d86),
    (0xC15D8C, i_c15d8c),
    (0xC15D8E, i_c15d8e),
    (0xC15D90, i_c15d90),
    (0xC15D92, i_c15d92),
    (0xC15D94, i_c15d94),
    (0xC15D96, i_c15d96),
    (0xC15D98, i_c15d98),
    (0xC15D9A, i_c15d9a),
    (0xC15D9C, i_c15d9c),
    (0xC15D9E, i_c15d9e),
    (0xC15DA0, i_c15da0),
    (0xC15DA2, i_c15da2),
    (0xC15DA4, i_c15da4),
    (0xC15DA6, i_c15da6),
    (0xC15DA8, i_c15da8),
    (0xC15DAA, i_c15daa),
    (0xC15DAE, i_c15dae),
    (0xC15DB0, i_c15db0),
    (0xC15DB2, i_c15db2),
    (0xC15DB6, i_c15db6),
    (0xC15DB8, i_c15db8),
    (0xC15DBA, i_c15dba),
    (0xC15DBE, i_c15dbe),
    (0xC15DC0, i_c15dc0),
    (0xC15DC4, i_c15dc4),
    (0xC15DC6, i_c15dc6),
    (0xC15DCA, i_c15dca),
    (0xC15DCE, i_c15dce),
    (0xC15DD0, i_c15dd0),
    (0xC15DD2, i_c15dd2),
    (0xC15DD4, i_c15dd4),
    (0xC15DD6, i_c15dd6),
    (0xC15DDC, i_c15ddc),
    (0xC15DE0, i_c15de0),
    (0xC15DE2, i_c15de2),
    (0xC15DE6, i_c15de6),
    (0xC15DEA, i_c15dea),
    (0xC15DEC, i_c15dec),
    (0xC15DEE, i_c15dee),
    (0xC15DF0, i_c15df0),
    (0xC15DF2, i_c15df2),
    (0xC15DF8, i_c15df8),
    (0xC15DFC, i_c15dfc),
    (0xC15DFE, i_c15dfe),
    (0xC15E02, i_c15e02),
    (0xC15E06, i_c15e06),
    (0xC15E08, i_c15e08),
    (0xC15E0A, i_c15e0a),
    (0xC15E0C, i_c15e0c),
    (0xC15E0E, i_c15e0e),
    (0xC15E10, i_c15e10),
    (0xC15E12, i_c15e12),
    (0xC15E14, i_c15e14),
    (0xC15E16, i_c15e16),
    (0xC15E18, i_c15e18),
    (0xC15E1A, i_c15e1a),
    (0xC15E1C, i_c15e1c),
    (0xC15E1E, i_c15e1e),
    (0xC15E20, i_c15e20),
    (0xC15E24, i_c15e24),
    (0xC15E28, i_c15e28),
    (0xC15E2A, i_c15e2a),
    (0xC15E2C, i_c15e2c),
    (0xC15E2E, i_c15e2e),
    (0xC15E30, i_c15e30),
    (0xC15E32, i_c15e32),
    (0xC15E34, i_c15e34),
    (0xC15E36, i_c15e36),
    (0xC15E38, i_c15e38),
    (0xC15E3A, i_c15e3a),
    (0xC15E3C, i_c15e3c),
    (0xC15E3E, i_c15e3e),
    (0xC15E40, i_c15e40),
    (0xC15E42, i_c15e42),
    (0xC15E46, i_c15e46),
    (0xC15E4A, i_c15e4a),
    (0xC15E50, i_c15e50),
    (0xC15E54, i_c15e54),
    (0xC15E56, i_c15e56),
    (0xC15E5A, i_c15e5a),
    (0xC15E5E, i_c15e5e),
    (0xC15E60, i_c15e60),
    (0xC15E62, i_c15e62),
    (0xC15E64, i_c15e64),
    (0xC15E66, i_c15e66),
    (0xC15E68, i_c15e68),
    (0xC15E6A, i_c15e6a),
    (0xC15E6C, i_c15e6c),
    (0xC15E6E, i_c15e6e),
    (0xC15E70, i_c15e70),
    (0xC15E74, i_c15e74),
    (0xC15E78, i_c15e78),
    (0xC15E7A, i_c15e7a),
    (0xC15E7C, i_c15e7c),
    (0xC15E7E, i_c15e7e),
    (0xC15E80, i_c15e80),
    (0xC15E82, i_c15e82),
    (0xC15E84, i_c15e84),
    (0xC15E86, i_c15e86),
    (0xC15E88, i_c15e88),
    (0xC15E8A, i_c15e8a),
    (0xC15E90, i_c15e90),
    (0xC15E94, i_c15e94),
    (0xC15E96, i_c15e96),
    (0xC15E9A, i_c15e9a),
    (0xC15E9E, i_c15e9e),
    (0xC15EA0, i_c15ea0),
    (0xC15EA2, i_c15ea2),
    (0xC15EA4, i_c15ea4),
    (0xC15EA6, i_c15ea6),
    (0xC15EAA, i_c15eaa),
    (0xC15EAC, i_c15eac),
    (0xC15EAE, i_c15eae),
    (0xC15EB0, i_c15eb0),
    (0xC15EB2, i_c15eb2),
    (0xC15EB4, i_c15eb4),
    (0xC15EB6, i_c15eb6),
    (0xC15EB8, i_c15eb8),
    (0xC15EBC, i_c15ebc),
    (0xC15EC0, i_c15ec0),
    (0xC15EC2, i_c15ec2),
    (0xC15EC4, i_c15ec4),
    (0xC15EC6, i_c15ec6),
    (0xC15EC8, i_c15ec8),
    (0xC15ECA, i_c15eca),
    (0xC15ECC, i_c15ecc),
    (0xC15ECE, i_c15ece),
    (0xC15ED0, i_c15ed0),
    (0xC15ED2, i_c15ed2),
    (0xC15ED4, i_c15ed4),
    (0xC15ED6, i_c15ed6),
    (0xC15EDA, i_c15eda),
    (0xC15EDE, i_c15ede),
    (0xC15EE0, i_c15ee0),
    (0xC15EE2, i_c15ee2),
    (0xC15EE4, i_c15ee4),
    (0xC15EF8, i_c15ef8),
    (0xC15EFC, i_c15efc),
    (0xC15EFE, i_c15efe),
    (0xC15F02, i_c15f02),
    (0xC15F06, i_c15f06),
    (0xC15F08, i_c15f08),
    (0xC15F0A, i_c15f0a),
    (0xC15F0C, i_c15f0c),
    (0xC15F0E, i_c15f0e),
    (0xC15F10, i_c15f10),
    (0xC15F12, i_c15f12),
    (0xC15F16, i_c15f16),
    (0xC15F1A, i_c15f1a),
    (0xC15F1C, i_c15f1c),
    (0xC15F1E, i_c15f1e),
    (0xC15F20, i_c15f20),
    (0xC15F22, i_c15f22),
    (0xC15F24, i_c15f24),
    (0xC15F26, i_c15f26),
    (0xC15F2A, i_c15f2a),
    (0xC15F2E, i_c15f2e),
    (0xC15F30, i_c15f30),
    (0xC15F32, i_c15f32),
    (0xC15F34, i_c15f34),
    (0xC15F38, i_c15f38),
    (0xC15F3A, i_c15f3a),
    (0xC15F3E, i_c15f3e),
    (0xC15F42, i_c15f42),
    (0xC15F44, i_c15f44),
    (0xC15F46, i_c15f46),
    (0xC15F48, i_c15f48),
    (0xC15F4A, i_c15f4a),
    (0xC15F4C, i_c15f4c),
    (0xC15F50, i_c15f50),
    (0xC15F54, i_c15f54),
    (0xC15F56, i_c15f56),
    (0xC15F58, i_c15f58),
    (0xC15F5A, i_c15f5a),
    (0xC15F5C, i_c15f5c),
    (0xC15F5E, i_c15f5e),
    (0xC15F60, i_c15f60),
    (0xC15F62, i_c15f62),
    (0xC15F66, i_c15f66),
    (0xC15F6A, i_c15f6a),
    (0xC15F6C, i_c15f6c),
    (0xC15F6E, i_c15f6e),
    (0xC15F70, i_c15f70),
    (0xC15F72, i_c15f72),
    (0xC15F74, i_c15f74),
    (0xC15F76, i_c15f76),
    (0xC15F78, i_c15f78),
    (0xC15F7A, i_c15f7a),
    (0xC15F7C, i_c15f7c),
    (0xC15F7E, i_c15f7e),
    (0xC15F80, i_c15f80),
    (0xC15F84, i_c15f84),
    (0xC15F88, i_c15f88),
    (0xC15FB2, i_c15fb2),
    (0xC15FB6, i_c15fb6),
    (0xC15FB8, i_c15fb8),
    (0xC15FBC, i_c15fbc),
    (0xC15FC0, i_c15fc0),
    (0xC15FC2, i_c15fc2),
    (0xC15FC4, i_c15fc4),
    (0xC15FC6, i_c15fc6),
    (0xC15FC8, i_c15fc8),
    (0xC15FCA, i_c15fca),
    (0xC15FCC, i_c15fcc),
    (0xC15FD0, i_c15fd0),
    (0xC15FD4, i_c15fd4),
    (0xC15FD6, i_c15fd6),
    (0xC15FDA, i_c15fda),
    (0xC15FDE, i_c15fde),
    (0xC15FE0, i_c15fe0),
    (0xC15FE4, i_c15fe4),
    (0xC15FE8, i_c15fe8),
    (0xC15FEA, i_c15fea),
    (0xC15FEE, i_c15fee),
    (0xC15FF2, i_c15ff2),
    (0xC15FF4, i_c15ff4),
    (0xC15FF8, i_c15ff8),
    (0xC15FFC, i_c15ffc),
    (0xC15FFE, i_c15ffe),
    (0xC16002, i_c16002),
    (0xC16006, i_c16006),
    (0xC16008, i_c16008),
    (0xC1600C, i_c1600c),
    (0xC16010, i_c16010),
    (0xC16012, i_c16012),
    (0xC16016, i_c16016),
    (0xC1601A, i_c1601a),
    (0xC1601C, i_c1601c),
    (0xC1601E, i_c1601e),
    (0xC16020, i_c16020),
    (0xC16022, i_c16022),
    (0xC16024, i_c16024),
    (0xC16026, i_c16026),
    (0xC16028, i_c16028),
    (0xC1602A, i_c1602a),
    (0xC1602C, i_c1602c),
    (0xC1602E, i_c1602e),
    (0xC16030, i_c16030),
    (0xC16032, i_c16032),
    (0xC16034, i_c16034),
    (0xC16038, i_c16038),
    (0xC1603A, i_c1603a),
    (0xC1603E, i_c1603e),
    (0xC16040, i_c16040),
    (0xC16044, i_c16044),
    (0xC16046, i_c16046),
    (0xC16048, i_c16048),
    (0xC1604A, i_c1604a),
    (0xC1604C, i_c1604c),
    (0xC16052, i_c16052),
    (0xC16056, i_c16056),
    (0xC1605C, i_c1605c),
    (0xC16060, i_c16060),
    (0xC16064, i_c16064),
    (0xC16068, i_c16068),
    (0xC1606C, i_c1606c),
    (0xC16070, i_c16070),
    (0xC16074, i_c16074),
    (0xC16078, i_c16078),
    (0xC1607C, i_c1607c),
    (0xC16080, i_c16080),
    (0xC16086, i_c16086),
    (0xC1608C, i_c1608c),
    (0xC16090, i_c16090),
    (0xC16094, i_c16094),
    (0xC16098, i_c16098),
    (0xC1609C, i_c1609c),
    (0xC160A0, i_c160a0),
    (0xC160A4, i_c160a4),
    (0xC160AA, i_c160aa),
    (0xC160B0, i_c160b0),
    (0xC160B6, i_c160b6),
    (0xC160BA, i_c160ba),
    (0xC160BE, i_c160be),
    (0xC160C2, i_c160c2),
    (0xC160C6, i_c160c6),
    (0xC160CA, i_c160ca),
    (0xC160CE, i_c160ce),
    (0xC160D4, i_c160d4),
    (0xC160DA, i_c160da),
    (0xC160E0, i_c160e0),
    (0xC160E8, i_c160e8),
    (0xC160F0, i_c160f0),
    (0xC160F8, i_c160f8),
    (0xC160FE, i_c160fe),
    (0xC16106, i_c16106),
    (0xC1610A, i_c1610a),
    (0xC16110, i_c16110),
    (0xC16112, i_c16112),
    (0xC16118, i_c16118),
    (0xC1611C, i_c1611c),
    (0xC16120, i_c16120),
    (0xC16126, i_c16126),
    (0xC1612A, i_c1612a),
    (0xC16130, i_c16130),
    (0xC16136, i_c16136),
    (0xC1613A, i_c1613a),
    (0xC1613C, i_c1613c),
    (0xC1613E, i_c1613e),
    (0xC16142, i_c16142),
    (0xC16144, i_c16144),
    (0xC16146, i_c16146),
    (0xC16148, i_c16148),
    (0xC1614A, i_c1614a),
    (0xC1614C, i_c1614c),
    (0xC1614E, i_c1614e),
    (0xC16150, i_c16150),
    (0xC16152, i_c16152),
    (0xC16154, i_c16154),
    (0xC16156, i_c16156),
    (0xC16158, i_c16158),
    (0xC1615A, i_c1615a),
    (0xC1615C, i_c1615c),
    (0xC1615E, i_c1615e),
    (0xC1617E, i_c1617e),
    (0xC16188, i_c16188),
    (0xC1618A, i_c1618a),
    (0xC1618E, i_c1618e),
    (0xC16192, i_c16192),
    (0xC1619C, i_c1619c),
    (0xC1619E, i_c1619e),
    (0xC161A0, i_c161a0),
    (0xC161A6, i_c161a6),
    (0xC161AC, i_c161ac),
    (0xC161AE, i_c161ae),
    (0xC161B0, i_c161b0),
    (0xC161B2, i_c161b2),
    (0xC161B4, i_c161b4),
    (0xC161BA, i_c161ba),
    (0xC161BE, i_c161be),
    (0xC161C2, i_c161c2),
    (0xC161C4, i_c161c4),
    (0xC161C6, i_c161c6),
    (0xC161C8, i_c161c8),
    (0xC161CC, i_c161cc),
    (0xC161D0, i_c161d0),
    (0xC161D2, i_c161d2),
    (0xC161D4, i_c161d4),
    (0xC161D6, i_c161d6),
    (0xC161DA, i_c161da),
    (0xC161DE, i_c161de),
    (0xC161E4, i_c161e4),
    (0xC161E8, i_c161e8),
    (0xC161EC, i_c161ec),
    (0xC161F2, i_c161f2),
    (0xC161F6, i_c161f6),
    (0xC161FA, i_c161fa),
    (0xC16200, i_c16200),
    (0xC16204, i_c16204),
    (0xC1620A, i_c1620a),
    (0xC1620E, i_c1620e),
    (0xC16214, i_c16214),
    (0xC16218, i_c16218),
    (0xC1621C, i_c1621c),
    (0xC16220, i_c16220),
    (0xC16226, i_c16226),
    (0xC16228, i_c16228),
    (0xC1622A, i_c1622a),
    (0xC1622C, i_c1622c),
    (0xC1622E, i_c1622e),
    (0xC16230, i_c16230),
    (0xC16234, i_c16234),
    (0xC16236, i_c16236),
    (0xC1623A, i_c1623a),
    (0xC1623C, i_c1623c),
    (0xC16242, i_c16242),
    (0xC16244, i_c16244),
    (0xC1624A, i_c1624a),
    (0xC1624C, i_c1624c),
    (0xC16254, i_c16254),
    (0xC1625C, i_c1625c),
    (0xC16264, i_c16264),
    (0xC1626C, i_c1626c),
    (0xC16270, i_c16270),
    (0xC16274, i_c16274),
    (0xC16278, i_c16278),
    (0xC1627E, i_c1627e),
    (0xC16284, i_c16284),
    (0xC1628A, i_c1628a),
    (0xC1628E, i_c1628e),
    (0xC16292, i_c16292),
    (0xC16298, i_c16298),
    (0xC1629A, i_c1629a),
    (0xC162A0, i_c162a0),
    (0xC162A2, i_c162a2),
    (0xC162A8, i_c162a8),
    (0xC162AA, i_c162aa),
    (0xC162AE, i_c162ae),
    (0xC162B2, i_c162b2),
    (0xC162B4, i_c162b4),
    (0xC162B6, i_c162b6),
    (0xC162B8, i_c162b8),
    (0xC162BA, i_c162ba),
    (0xC162BC, i_c162bc),
    (0xC162BE, i_c162be),
    (0xC162C0, i_c162c0),
    (0xC162C2, i_c162c2),
    (0xC162C4, i_c162c4),
    (0xC162C6, i_c162c6),
    (0xC162C8, i_c162c8),
    (0xC162CA, i_c162ca),
    (0xC1637C, i_c1637c),
    (0xC16380, i_c16380),
    (0xC1638A, i_c1638a),
    (0xC16394, i_c16394),
    (0xC1639C, i_c1639c),
    (0xC163A0, i_c163a0),
    (0xC163A4, i_c163a4),
    (0xC163A6, i_c163a6),
    (0xC163AC, i_c163ac),
    (0xC163B0, i_c163b0),
    (0xC163B2, i_c163b2),
    (0xC163B4, i_c163b4),
    (0xC163BA, i_c163ba),
    (0xC163BE, i_c163be),
    (0xC163C0, i_c163c0),
    (0xC163C2, i_c163c2),
    (0xC163C6, i_c163c6),
    (0xC163CA, i_c163ca),
    (0xC163CE, i_c163ce),
    (0xC163D4, i_c163d4),
    (0xC163DA, i_c163da),
    (0xC163E0, i_c163e0),
    (0xC163E4, i_c163e4),
    (0xC163E8, i_c163e8),
    (0xC163EE, i_c163ee),
    (0xC163F0, i_c163f0),
    (0xC163F6, i_c163f6),
    (0xC163F8, i_c163f8),
    (0xC163FC, i_c163fc),
    (0xC16402, i_c16402),
    (0xC16404, i_c16404),
    (0xC16408, i_c16408),
    (0xC1640C, i_c1640c),
    (0xC1640E, i_c1640e),
    (0xC16412, i_c16412),
    (0xC16418, i_c16418),
    (0xC1641C, i_c1641c),
    (0xC16420, i_c16420),
    (0xC16424, i_c16424),
    (0xC16426, i_c16426),
    (0xC1642C, i_c1642c),
    (0xC16430, i_c16430),
    (0xC16434, i_c16434),
    (0xC1643A, i_c1643a),
    (0xC1643E, i_c1643e),
    (0xC16442, i_c16442),
    (0xC16446, i_c16446),
    (0xC16448, i_c16448),
    (0xC1644C, i_c1644c),
    (0xC16450, i_c16450),
    (0xC16452, i_c16452),
    (0xC16458, i_c16458),
    (0xC1645C, i_c1645c),
    (0xC16462, i_c16462),
    (0xC16466, i_c16466),
    (0xC1646C, i_c1646c),
    (0xC1646E, i_c1646e),
    (0xC16472, i_c16472),
    (0xC16476, i_c16476),
    (0xC1647E, i_c1647e),
    (0xC16482, i_c16482),
    (0xC1648A, i_c1648a),
    (0xC1648E, i_c1648e),
    (0xC16496, i_c16496),
    (0xC1649A, i_c1649a),
    (0xC1649C, i_c1649c),
    (0xC1649E, i_c1649e),
    (0xC164A2, i_c164a2),
    (0xC164A4, i_c164a4),
    (0xC164A8, i_c164a8),
    (0xC164AA, i_c164aa),
    (0xC164AC, i_c164ac),
    (0xC164B2, i_c164b2),
    (0xC164B6, i_c164b6),
    (0xC164BA, i_c164ba),
    (0xC164BC, i_c164bc),
    (0xC164C0, i_c164c0),
    (0xC164C2, i_c164c2),
    (0xC164C6, i_c164c6),
    (0xC164CA, i_c164ca),
    (0xC164CC, i_c164cc),
    (0xC164CE, i_c164ce),
    (0xC164D2, i_c164d2),
    (0xC164D4, i_c164d4),
    (0xC164D6, i_c164d6),
    (0xC164DA, i_c164da),
    (0xC164DC, i_c164dc),
    (0xC164DE, i_c164de),
    (0xC164E0, i_c164e0),
    (0xC164E6, i_c164e6),
    (0xC164EA, i_c164ea),
    (0xC164EE, i_c164ee),
    (0xC164F0, i_c164f0),
    (0xC164F4, i_c164f4),
    (0xC164F6, i_c164f6),
    (0xC164FA, i_c164fa),
    (0xC164FE, i_c164fe),
    (0xC16500, i_c16500),
    (0xC16502, i_c16502),
    (0xC16506, i_c16506),
    (0xC16508, i_c16508),
    (0xC1650A, i_c1650a),
    (0xC1650E, i_c1650e),
    (0xC16510, i_c16510),
    (0xC16514, i_c16514),
    (0xC16516, i_c16516),
    (0xC1651A, i_c1651a),
    (0xC1651E, i_c1651e),
    (0xC16520, i_c16520),
    (0xC16522, i_c16522),
    (0xC16600, i_c16600),
    (0xC16604, i_c16604),
    (0xC16606, i_c16606),
    (0xC1660A, i_c1660a),
    (0xC1660C, i_c1660c),
    (0xC16612, i_c16612),
    (0xC16618, i_c16618),
    (0xC1661A, i_c1661a),
    (0xC16620, i_c16620),
    (0xC16622, i_c16622),
    (0xC16626, i_c16626),
    (0xC1662A, i_c1662a),
    (0xC1662E, i_c1662e),
    (0xC16634, i_c16634),
    (0xC1663A, i_c1663a),
    (0xC16640, i_c16640),
    (0xC16644, i_c16644),
    (0xC16648, i_c16648),
    (0xC1664E, i_c1664e),
    (0xC16650, i_c16650),
    (0xC16656, i_c16656),
    (0xC16658, i_c16658),
    (0xC1665C, i_c1665c),
    (0xC16660, i_c16660),
    (0xC16664, i_c16664),
    (0xC16666, i_c16666),
    (0xC1666A, i_c1666a),
    (0xC1666E, i_c1666e),
    (0xC16672, i_c16672),
    (0xC16676, i_c16676),
    (0xC1667A, i_c1667a),
    (0xC1667E, i_c1667e),
    (0xC16680, i_c16680),
    (0xC16686, i_c16686),
    (0xC16688, i_c16688),
    (0xC1668C, i_c1668c),
    (0xC16690, i_c16690),
    (0xC16692, i_c16692),
    (0xC16694, i_c16694),
    (0xC16696, i_c16696),
    (0xC16698, i_c16698),
    (0xC1669A, i_c1669a),
    (0xC1669C, i_c1669c),
    (0xC166A0, i_c166a0),
    (0xC166A4, i_c166a4),
    (0xC166A6, i_c166a6),
    (0xC166A8, i_c166a8),
    (0xC166AA, i_c166aa),
    (0xC166AC, i_c166ac),
    (0xC166AE, i_c166ae),
    (0xC166B0, i_c166b0),
    (0xC166B2, i_c166b2),
    (0xC166B4, i_c166b4),
    (0xC166B6, i_c166b6),
    (0xC166B8, i_c166b8),
    (0xC166BC, i_c166bc),
    (0xC166C0, i_c166c0),
    (0xC166C4, i_c166c4),
    (0xC166C6, i_c166c6),
    (0xC166C8, i_c166c8),
    (0xC166CA, i_c166ca),
    (0xC166CC, i_c166cc),
    (0xC166CE, i_c166ce),
    (0xC166D0, i_c166d0),
    (0xC166D2, i_c166d2),
    (0xC166D4, i_c166d4),
    (0xC166D6, i_c166d6),
    (0xC166D8, i_c166d8),
    (0xC166DA, i_c166da),
    (0xC166DC, i_c166dc),
    (0xC166DE, i_c166de),
    (0xC166E2, i_c166e2),
    (0xC166E4, i_c166e4),
    (0xC166E6, i_c166e6),
    (0xC166EA, i_c166ea),
    (0xC166EE, i_c166ee),
    (0xC166F2, i_c166f2),
    (0xC166F4, i_c166f4),
    (0xC166F8, i_c166f8),
    (0xC166FC, i_c166fc),
    (0xC16702, i_c16702),
    (0xC16708, i_c16708),
    (0xC1670E, i_c1670e),
    (0xC16710, i_c16710),
    (0xC16714, i_c16714),
    (0xC16718, i_c16718),
    (0xC1671C, i_c1671c),
    (0xC16720, i_c16720),
    (0xC16724, i_c16724),
    (0xC16728, i_c16728),
    (0xC1672C, i_c1672c),
    (0xC16730, i_c16730),
    (0xC16734, i_c16734),
    (0xC16738, i_c16738),
    (0xC1673C, i_c1673c),
    (0xC16740, i_c16740),
    (0xC16744, i_c16744),
    (0xC16748, i_c16748),
    (0xC1674C, i_c1674c),
    (0xC16750, i_c16750),
    (0xC16754, i_c16754),
    (0xC16758, i_c16758),
    (0xC1675C, i_c1675c),
    (0xC16760, i_c16760),
    (0xC16764, i_c16764),
    (0xC16766, i_c16766),
    (0xC1676A, i_c1676a),
    (0xC1676E, i_c1676e),
    (0xC16772, i_c16772),
    (0xC16776, i_c16776),
    (0xC1677C, i_c1677c),
    (0xC1678A, i_c1678a),
    (0xC16790, i_c16790),
    (0xC16792, i_c16792),
    (0xC16798, i_c16798),
    (0xC1679A, i_c1679a),
    (0xC167A0, i_c167a0),
    (0xC167A4, i_c167a4),
    (0xC167A8, i_c167a8),
    (0xC167AC, i_c167ac),
    (0xC167B0, i_c167b0),
    (0xC167B4, i_c167b4),
    (0xC167B6, i_c167b6),
    (0xC167BA, i_c167ba),
    (0xC167BC, i_c167bc),
    (0xC167BE, i_c167be),
    (0xC167C2, i_c167c2),
    (0xC167C4, i_c167c4),
    (0xC167C8, i_c167c8),
    (0xC167CA, i_c167ca),
    (0xC167CE, i_c167ce),
    (0xC167D4, i_c167d4),
    (0xC167D6, i_c167d6),
    (0xC167D8, i_c167d8),
    (0xC167DC, i_c167dc),
    (0xC167E0, i_c167e0),
    (0xC167E4, i_c167e4),
    (0xC167E6, i_c167e6),
    (0xC167E8, i_c167e8),
    (0xC167EC, i_c167ec),
    (0xC167EE, i_c167ee),
    (0xC167F2, i_c167f2),
    (0xC167F4, i_c167f4),
    (0xC167F8, i_c167f8),
    (0xC167FC, i_c167fc),
    (0xC16800, i_c16800),
    (0xC16804, i_c16804),
    (0xC16806, i_c16806),
    (0xC16808, i_c16808),
    (0xC1680A, i_c1680a),
    (0xC1680C, i_c1680c),
    (0xC16810, i_c16810),
    (0xC16812, i_c16812),
    (0xC16816, i_c16816),
    (0xC16818, i_c16818),
    (0xC1681C, i_c1681c),
    (0xC1681E, i_c1681e),
    (0xC16822, i_c16822),
    (0xC16826, i_c16826),
    (0xC1682A, i_c1682a),
    (0xC1682E, i_c1682e),
    (0xC16834, i_c16834),
    (0xC16836, i_c16836),
    (0xC1683A, i_c1683a),
    (0xC1683E, i_c1683e),
    (0xC16844, i_c16844),
    (0xC16848, i_c16848),
    (0xC1684C, i_c1684c),
    (0xC16852, i_c16852),
    (0xC16856, i_c16856),
    (0xC16858, i_c16858),
    (0xC1685C, i_c1685c),
    (0xC1685E, i_c1685e),
    (0xC16860, i_c16860),
    (0xC16864, i_c16864),
    (0xC16868, i_c16868),
    (0xC1686A, i_c1686a),
    (0xC1686E, i_c1686e),
    (0xC16872, i_c16872),
    (0xC16876, i_c16876),
    (0xC1687A, i_c1687a),
    (0xC1687E, i_c1687e),
    (0xC16882, i_c16882),
    (0xC16886, i_c16886),
    (0xC1688A, i_c1688a),
    (0xC1688E, i_c1688e),
    (0xC16892, i_c16892),
    (0xC16896, i_c16896),
    (0xC1689A, i_c1689a),
    (0xC1689E, i_c1689e),
    (0xC168A2, i_c168a2),
    (0xC168A6, i_c168a6),
    (0xC168AA, i_c168aa),
    (0xC168AE, i_c168ae),
    (0xC168B2, i_c168b2),
    (0xC168B4, i_c168b4),
    (0xC168B8, i_c168b8),
    (0xC168BC, i_c168bc),
    (0xC168BE, i_c168be),
    (0xC168C2, i_c168c2),
    (0xC168C4, i_c168c4),
    (0xC168C8, i_c168c8),
    (0xC168CC, i_c168cc),
    (0xC168CE, i_c168ce),
    (0xC168D2, i_c168d2),
    (0xC168D4, i_c168d4),
    (0xC168D8, i_c168d8),
    (0xC168DA, i_c168da),
    (0xC168DC, i_c168dc),
    (0xC168E0, i_c168e0),
    (0xC168E2, i_c168e2),
    (0xC168E6, i_c168e6),
    (0xC168EA, i_c168ea),
    (0xC168EE, i_c168ee),
    (0xC168F2, i_c168f2),
    (0xC168F4, i_c168f4),
    (0xC168F8, i_c168f8),
    (0xC168FA, i_c168fa),
    (0xC168FC, i_c168fc),
    (0xC16900, i_c16900),
    (0xC16904, i_c16904),
    (0xC16906, i_c16906),
    (0xC16908, i_c16908),
    (0xC1690A, i_c1690a),
    (0xC1690C, i_c1690c),
    (0xC1690E, i_c1690e),
    (0xC16910, i_c16910),
    (0xC16912, i_c16912),
    (0xC16914, i_c16914),
    (0xC16916, i_c16916),
    (0xC169B0, i_c169b0),
    (0xC169B8, i_c169b8),
    (0xC169BE, i_c169be),
    (0xC169C0, i_c169c0),
    (0xC169C4, i_c169c4),
    (0xC169C8, i_c169c8),
    (0xC169CE, i_c169ce),
    (0xC169D2, i_c169d2),
    (0xC169D6, i_c169d6),
    (0xC169DA, i_c169da),
    (0xC169DE, i_c169de),
    (0xC169E4, i_c169e4),
    (0xC169E8, i_c169e8),
    (0xC169EC, i_c169ec),
    (0xC169EE, i_c169ee),
    (0xC169F0, i_c169f0),
    (0xC169F2, i_c169f2),
    (0xC169F6, i_c169f6),
    (0xC169FA, i_c169fa),
    (0xC169FC, i_c169fc),
    (0xC16A02, i_c16a02),
    (0xC16A04, i_c16a04),
    (0xC16A08, i_c16a08),
    (0xC16A0E, i_c16a0e),
    (0xC16A12, i_c16a12),
    (0xC16A16, i_c16a16),
    (0xC16A18, i_c16a18),
    (0xC16A1E, i_c16a1e),
    (0xC16A20, i_c16a20),
    (0xC16A24, i_c16a24),
    (0xC16A26, i_c16a26),
    (0xC16A2A, i_c16a2a),
    (0xC16A30, i_c16a30),
    (0xC16A32, i_c16a32),
    (0xC16A34, i_c16a34),
    (0xC16A38, i_c16a38),
    (0xC16A3E, i_c16a3e),
    (0xC16A44, i_c16a44),
    (0xC16A48, i_c16a48),
    (0xC16A4A, i_c16a4a),
    (0xC16A4E, i_c16a4e),
    (0xC16A50, i_c16a50),
    (0xC16A52, i_c16a52),
    (0xC16A54, i_c16a54),
    (0xC16A56, i_c16a56),
    (0xC16A58, i_c16a58),
    (0xC16A5A, i_c16a5a),
    (0xC16A5C, i_c16a5c),
    (0xC16A5E, i_c16a5e),
    (0xC16A60, i_c16a60),
    (0xC16A62, i_c16a62),
    (0xC16A64, i_c16a64),
    (0xC16A66, i_c16a66),
    (0xC16A68, i_c16a68),
    (0xC16A6A, i_c16a6a),
    (0xC16A6C, i_c16a6c),
    (0xC16A6E, i_c16a6e),
    (0xC16A70, i_c16a70),
    (0xC16A72, i_c16a72),
    (0xC16A74, i_c16a74),
    (0xC16A76, i_c16a76),
    (0xC16A78, i_c16a78),
    (0xC16A7A, i_c16a7a),
    (0xC16A7C, i_c16a7c),
    (0xC16A80, i_c16a80),
    (0xC16A84, i_c16a84),
    (0xC16A88, i_c16a88),
    (0xC16A8A, i_c16a8a),
    (0xC16A8C, i_c16a8c),
    (0xC16A8E, i_c16a8e),
    (0xC16A92, i_c16a92),
    (0xC16A94, i_c16a94),
    (0xC16A98, i_c16a98),
    (0xC16A9A, i_c16a9a),
    (0xC16A9C, i_c16a9c),
    (0xC16A9E, i_c16a9e),
    (0xC16AA0, i_c16aa0),
    (0xC16AA6, i_c16aa6),
    (0xC16AAA, i_c16aaa),
    (0xC16AB0, i_c16ab0),
    (0xC16AB4, i_c16ab4),
    (0xC16AB8, i_c16ab8),
    (0xC16ABC, i_c16abc),
    (0xC16AC0, i_c16ac0),
    (0xC16AC4, i_c16ac4),
    (0xC16ACA, i_c16aca),
    (0xC16ACC, i_c16acc),
    (0xC16ACE, i_c16ace),
    (0xC16AD0, i_c16ad0),
    (0xC16AD4, i_c16ad4),
    (0xC16AD8, i_c16ad8),
    (0xC16ADC, i_c16adc),
    (0xC16AE0, i_c16ae0),
    (0xC16AE4, i_c16ae4),
    (0xC16AE8, i_c16ae8),
    (0xC16AEC, i_c16aec),
    (0xC16AF0, i_c16af0),
    (0xC16AF4, i_c16af4),
    (0xC16AFE, i_c16afe),
    (0xC16B04, i_c16b04),
    (0xC16B08, i_c16b08),
    (0xC16B0C, i_c16b0c),
    (0xC16B0E, i_c16b0e),
    (0xC16B12, i_c16b12),
    (0xC16B16, i_c16b16),
    (0xC16B1C, i_c16b1c),
    (0xC16B20, i_c16b20),
    (0xC16B24, i_c16b24),
    (0xC16B28, i_c16b28),
    (0xC16B2E, i_c16b2e),
    (0xC16B34, i_c16b34),
    (0xC16B36, i_c16b36),
    (0xC16B38, i_c16b38),
    (0xC16B3C, i_c16b3c),
    (0xC16B40, i_c16b40),
    (0xC16B46, i_c16b46),
    (0xC16B4A, i_c16b4a),
    (0xC16B4E, i_c16b4e),
    (0xC16B52, i_c16b52),
    (0xC16B54, i_c16b54),
    (0xC16B56, i_c16b56),
    (0xC16B58, i_c16b58),
    (0xC16B5C, i_c16b5c),
    (0xC16B60, i_c16b60),
    (0xC16B66, i_c16b66),
    (0xC16B6A, i_c16b6a),
    (0xC16B6E, i_c16b6e),
    (0xC16B70, i_c16b70),
    (0xC16B72, i_c16b72),
    (0xC16B76, i_c16b76),
    (0xC16B7A, i_c16b7a),
    (0xC16B7C, i_c16b7c),
    (0xC16B7E, i_c16b7e),
    (0xC16B82, i_c16b82),
    (0xC16B86, i_c16b86),
    (0xC16B8A, i_c16b8a),
    (0xC16B90, i_c16b90),
    (0xC16B96, i_c16b96),
    (0xC16B9A, i_c16b9a),
    (0xC16B9C, i_c16b9c),
    (0xC16BA0, i_c16ba0),
    (0xC16BA4, i_c16ba4),
    (0xC16BA8, i_c16ba8),
    (0xC16BAA, i_c16baa),
    (0xC16BAE, i_c16bae),
    (0xC16BB0, i_c16bb0),
    (0xC16BB6, i_c16bb6),
    (0xC16BB8, i_c16bb8),
    (0xC16BC0, i_c16bc0),
    (0xC16BC4, i_c16bc4),
    (0xC16BC6, i_c16bc6),
    (0xC16BCA, i_c16bca),
    (0xC16BCE, i_c16bce),
    (0xC16BD2, i_c16bd2),
    (0xC16BD6, i_c16bd6),
    (0xC16BD8, i_c16bd8),
    (0xC16BDC, i_c16bdc),
    (0xC16BDE, i_c16bde),
    (0xC16BE0, i_c16be0),
    (0xC16BE4, i_c16be4),
    (0xC16BE8, i_c16be8),
    (0xC16BEA, i_c16bea),
    (0xC16BF0, i_c16bf0),
    (0xC16BF6, i_c16bf6),
    (0xC16BFC, i_c16bfc),
    (0xC16BFE, i_c16bfe),
    (0xC16C02, i_c16c02),
    (0xC16C04, i_c16c04),
    (0xC16C0A, i_c16c0a),
    (0xC16C0C, i_c16c0c),
    (0xC16C10, i_c16c10),
    (0xC16C14, i_c16c14),
    (0xC16C18, i_c16c18),
    (0xC16C1A, i_c16c1a),
    (0xC16C1E, i_c16c1e),
    (0xC16C24, i_c16c24),
    (0xC16C2A, i_c16c2a),
    (0xC16C2C, i_c16c2c),
    (0xC16C32, i_c16c32),
    (0xC16C38, i_c16c38),
    (0xC16C3E, i_c16c3e),
    (0xC16C44, i_c16c44),
    (0xC16C4A, i_c16c4a),
    (0xC16C50, i_c16c50),
    (0xC16C56, i_c16c56),
    (0xC16C5C, i_c16c5c),
    (0xC16C64, i_c16c64),
    (0xC16C66, i_c16c66),
    (0xC16C6A, i_c16c6a),
    (0xC16C6E, i_c16c6e),
    (0xC16C72, i_c16c72),
    (0xC16C74, i_c16c74),
    (0xC16C78, i_c16c78),
    (0xC16C7E, i_c16c7e),
    (0xC16C82, i_c16c82),
    (0xC16C84, i_c16c84),
    (0xC16C88, i_c16c88),
    (0xC16C8C, i_c16c8c),
    (0xC16C90, i_c16c90),
    (0xC16C94, i_c16c94),
    (0xC16C96, i_c16c96),
    (0xC16C9C, i_c16c9c),
    (0xC16CA2, i_c16ca2),
    (0xC16CA4, i_c16ca4),
    (0xC16CA8, i_c16ca8),
    (0xC16CAC, i_c16cac),
    (0xC16CB0, i_c16cb0),
    (0xC16CB4, i_c16cb4),
    (0xC16CB8, i_c16cb8),
    (0xC16CBC, i_c16cbc),
    (0xC16CC2, i_c16cc2),
    (0xC16CC8, i_c16cc8),
    (0xC16CCC, i_c16ccc),
    (0xC16CD0, i_c16cd0),
    (0xC16CD4, i_c16cd4),
    (0xC16CD8, i_c16cd8),
    (0xC16CDC, i_c16cdc),
    (0xC16CE0, i_c16ce0),
    (0xC16CE4, i_c16ce4),
    (0xC16CE8, i_c16ce8),
    (0xC16CEC, i_c16cec),
    (0xC16CF2, i_c16cf2),
    (0xC16CF6, i_c16cf6),
    (0xC16CFA, i_c16cfa),
    (0xC16CFE, i_c16cfe),
    (0xC16D00, i_c16d00),
    (0xC16D06, i_c16d06),
    (0xC16D0A, i_c16d0a),
    (0xC16D0E, i_c16d0e),
    (0xC16D10, i_c16d10),
    (0xC16D16, i_c16d16),
    (0xC16D1C, i_c16d1c),
    (0xC16D1E, i_c16d1e),
    (0xC16D22, i_c16d22),
    (0xC16D24, i_c16d24),
    (0xC16D2A, i_c16d2a),
    (0xC16D30, i_c16d30),
    (0xC16D36, i_c16d36),
    (0xC16D38, i_c16d38),
    (0xC16D3C, i_c16d3c),
    (0xC16D3E, i_c16d3e),
    (0xC16D42, i_c16d42),
    (0xC16D46, i_c16d46),
    (0xC16D48, i_c16d48),
    (0xC16D4C, i_c16d4c),
    (0xC16D52, i_c16d52),
    (0xC16D54, i_c16d54),
    (0xC16D58, i_c16d58),
    (0xC16D5C, i_c16d5c),
    (0xC16D60, i_c16d60),
    (0xC16D64, i_c16d64),
    (0xC16D68, i_c16d68),
    (0xC16D6C, i_c16d6c),
    (0xC16D72, i_c16d72),
    (0xC16D78, i_c16d78),
    (0xC16D7A, i_c16d7a),
    (0xC16D7E, i_c16d7e),
    (0xC16D80, i_c16d80),
    (0xC16D84, i_c16d84),
    (0xC16D86, i_c16d86),
    (0xC16D8C, i_c16d8c),
    (0xC16D8E, i_c16d8e),
    (0xC16D92, i_c16d92),
    (0xC16D96, i_c16d96),
    (0xC16D9C, i_c16d9c),
    (0xC16D9E, i_c16d9e),
    (0xC16DA2, i_c16da2),
    (0xC16DA6, i_c16da6),
    (0xC16DAC, i_c16dac),
    (0xC16DB2, i_c16db2),
    (0xC16DB8, i_c16db8),
    (0xC16DBE, i_c16dbe),
    (0xC16DC4, i_c16dc4),
    (0xC16DCA, i_c16dca),
    (0xC16DCC, i_c16dcc),
    (0xC16DCE, i_c16dce),
    (0xC16DD2, i_c16dd2),
    (0xC16DD6, i_c16dd6),
    (0xC16DDA, i_c16dda),
    (0xC16DDE, i_c16dde),
    (0xC16DE4, i_c16de4),
    (0xC16DE8, i_c16de8),
    (0xC16DEC, i_c16dec),
    (0xC16DF2, i_c16df2),
    (0xC16DF8, i_c16df8),
    (0xC16DFA, i_c16dfa),
    (0xC16E04, i_c16e04),
    (0xC16E08, i_c16e08),
    (0xC16E0C, i_c16e0c),
    (0xC16E10, i_c16e10),
    (0xC16E16, i_c16e16),
    (0xC16E18, i_c16e18),
    (0xC16E1C, i_c16e1c),
    (0xC16E1E, i_c16e1e),
    (0xC16E20, i_c16e20),
    (0xC16E26, i_c16e26),
    (0xC16E2A, i_c16e2a),
    (0xC16E2E, i_c16e2e),
    (0xC16E32, i_c16e32),
    (0xC16E38, i_c16e38),
    (0xC16E3C, i_c16e3c),
    (0xC16E40, i_c16e40),
    (0xC16E44, i_c16e44),
    (0xC16E4C, i_c16e4c),
    (0xC16E52, i_c16e52),
    (0xC16E58, i_c16e58),
    (0xC16E5C, i_c16e5c),
    (0xC16E62, i_c16e62),
    (0xC16E66, i_c16e66),
    (0xC16E6C, i_c16e6c),
    (0xC16E6E, i_c16e6e),
    (0xC16E72, i_c16e72),
    (0xC16E76, i_c16e76),
    (0xC16E7A, i_c16e7a),
    (0xC16E7C, i_c16e7c),
    (0xC16E82, i_c16e82),
    (0xC16E88, i_c16e88),
    (0xC16E8A, i_c16e8a),
    (0xC16E8E, i_c16e8e),
    (0xC16E90, i_c16e90),
    (0xC16E92, i_c16e92),
    (0xC16E94, i_c16e94),
    (0xC16E96, i_c16e96),
    (0xC16E98, i_c16e98),
    (0xC16E9C, i_c16e9c),
    (0xC16E9E, i_c16e9e),
    (0xC16EA2, i_c16ea2),
    (0xC16EA4, i_c16ea4),
    (0xC16EA6, i_c16ea6),
    (0xC16EAA, i_c16eaa),
    (0xC16EAE, i_c16eae),
    (0xC16EB2, i_c16eb2),
    (0xC16EB6, i_c16eb6),
    (0xC16EB8, i_c16eb8),
    (0xC16EBC, i_c16ebc),
    (0xC16EBE, i_c16ebe),
    (0xC16EC4, i_c16ec4),
    (0xC16EC8, i_c16ec8),
    (0xC16ECE, i_c16ece),
    (0xC16ED0, i_c16ed0),
    (0xC16ED6, i_c16ed6),
    (0xC16EDC, i_c16edc),
    (0xC16EE2, i_c16ee2),
    (0xC16EE6, i_c16ee6),
    (0xC16EEA, i_c16eea),
    (0xC16EEE, i_c16eee),
    (0xC16EF2, i_c16ef2),
    (0xC16EF6, i_c16ef6),
    (0xC16EFA, i_c16efa),
    (0xC16F00, i_c16f00),
    (0xC16F04, i_c16f04),
    (0xC16F06, i_c16f06),
    (0xC16F0A, i_c16f0a),
    (0xC16F0C, i_c16f0c),
    (0xC16F12, i_c16f12),
    (0xC16F18, i_c16f18),
    (0xC16F1E, i_c16f1e),
    (0xC16F22, i_c16f22),
    (0xC16F26, i_c16f26),
    (0xC16F2A, i_c16f2a),
    (0xC16F2C, i_c16f2c),
    (0xC16F30, i_c16f30),
    (0xC16F34, i_c16f34),
    (0xC16F38, i_c16f38),
    (0xC16F3C, i_c16f3c),
    (0xC16F40, i_c16f40),
    (0xC16F46, i_c16f46),
    (0xC16F4A, i_c16f4a),
    (0xC16F50, i_c16f50),
    (0xC16F52, i_c16f52),
    (0xC16F54, i_c16f54),
    (0xC16F56, i_c16f56),
    (0xC16F58, i_c16f58),
    (0xC16F5C, i_c16f5c),
    (0xC16F5E, i_c16f5e),
    (0xC16F62, i_c16f62),
    (0xC16F66, i_c16f66),
    (0xC16F68, i_c16f68),
    (0xC16F6C, i_c16f6c),
    (0xC16F6E, i_c16f6e),
    (0xC16F72, i_c16f72),
    (0xC16F74, i_c16f74),
    (0xC16F78, i_c16f78),
    (0xC16F7C, i_c16f7c),
    (0xC16F80, i_c16f80),
    (0xC16F84, i_c16f84),
    (0xC16F88, i_c16f88),
    (0xC16F8A, i_c16f8a),
    (0xC16F8C, i_c16f8c),
    (0xC16F90, i_c16f90),
    (0xC16F92, i_c16f92),
    (0xC16F94, i_c16f94),
    (0xC16F96, i_c16f96),
    (0xC16F9A, i_c16f9a),
    (0xC16F9E, i_c16f9e),
    (0xC16FA2, i_c16fa2),
    (0xC16FA6, i_c16fa6),
    (0xC16FA8, i_c16fa8),
    (0xC16FAA, i_c16faa),
    (0xC16FAC, i_c16fac),
    (0xC16FAE, i_c16fae),
    (0xC16FB0, i_c16fb0),
    (0xC16FB2, i_c16fb2),
    (0xC16FB4, i_c16fb4),
    (0xC16FB6, i_c16fb6),
    (0xC16FBA, i_c16fba),
    (0xC16FBE, i_c16fbe),
    (0xC16FC2, i_c16fc2),
    (0xC16FC6, i_c16fc6),
    (0xC16FCA, i_c16fca),
    (0xC16FCE, i_c16fce),
    (0xC16FD2, i_c16fd2),
    (0xC16FD4, i_c16fd4),
    (0xC16FD6, i_c16fd6),
    (0xC16FDA, i_c16fda),
    (0xC16FDE, i_c16fde),
    (0xC16FE4, i_c16fe4),
    (0xC16FE6, i_c16fe6),
    (0xC16FEA, i_c16fea),
    (0xC16FEE, i_c16fee),
    (0xC16FF2, i_c16ff2),
    (0xC16FF6, i_c16ff6),
    (0xC16FFC, i_c16ffc),
    (0xC17000, i_c17000),
    (0xC17002, i_c17002),
    (0xC17008, i_c17008),
    (0xC1700C, i_c1700c),
    (0xC1700E, i_c1700e),
    (0xC17014, i_c17014),
    (0xC1701A, i_c1701a),
    (0xC17020, i_c17020),
    (0xC17024, i_c17024),
    (0xC17028, i_c17028),
    (0xC1702C, i_c1702c),
    (0xC1702E, i_c1702e),
    (0xC17030, i_c17030),
    (0xC17032, i_c17032),
    (0xC17034, i_c17034),
    (0xC17036, i_c17036),
    (0xC17038, i_c17038),
    (0xC1703A, i_c1703a),
    (0xC1703C, i_c1703c),
    (0xC1703E, i_c1703e),
    (0xC17042, i_c17042),
    (0xC17046, i_c17046),
    (0xC17048, i_c17048),
    (0xC1704A, i_c1704a),
    (0xC1704C, i_c1704c),
    (0xC17050, i_c17050),
    (0xC17052, i_c17052),
    (0xC17054, i_c17054),
    (0xC17058, i_c17058),
    (0xC1705C, i_c1705c),
    (0xC1705E, i_c1705e),
    (0xC17062, i_c17062),
    (0xC17066, i_c17066),
    (0xC1706A, i_c1706a),
    (0xC1706E, i_c1706e),
    (0xC17072, i_c17072),
    (0xC17074, i_c17074),
    (0xC17076, i_c17076),
    (0xC17078, i_c17078),
    (0xC1707A, i_c1707a),
    (0xC1707C, i_c1707c),
    (0xC1707E, i_c1707e),
    (0xC17080, i_c17080),
    (0xC17086, i_c17086),
    (0xC1708A, i_c1708a),
    (0xC17090, i_c17090),
    (0xC17092, i_c17092),
    (0xC17096, i_c17096),
    (0xC17098, i_c17098),
    (0xC1709E, i_c1709e),
    (0xC170A4, i_c170a4),
    (0xC170A6, i_c170a6),
    (0xC170AC, i_c170ac),
    (0xC170AE, i_c170ae),
    (0xC170B2, i_c170b2),
    (0xC170B4, i_c170b4),
    (0xC170BA, i_c170ba),
    (0xC170C0, i_c170c0),
    (0xC170C2, i_c170c2),
    (0xC170C8, i_c170c8),
    (0xC170CC, i_c170cc),
    (0xC170CE, i_c170ce),
    (0xC170D0, i_c170d0),
    (0xC170D6, i_c170d6),
    (0xC170D8, i_c170d8),
    (0xC170DA, i_c170da),
    (0xC170E0, i_c170e0),
    (0xC170E2, i_c170e2),
    (0xC170E8, i_c170e8),
    (0xC170EA, i_c170ea),
    (0xC170EE, i_c170ee),
    (0xC170F2, i_c170f2),
    (0xC170F6, i_c170f6),
    (0xC170F8, i_c170f8),
    (0xC170FA, i_c170fa),
    (0xC170FC, i_c170fc),
    (0xC17100, i_c17100),
    (0xC17104, i_c17104),
    (0xC17108, i_c17108),
    (0xC1710C, i_c1710c),
    (0xC1710E, i_c1710e),
    (0xC17112, i_c17112),
    (0xC17116, i_c17116),
    (0xC17118, i_c17118),
    (0xC1711C, i_c1711c),
    (0xC17120, i_c17120),
    (0xC17124, i_c17124),
    (0xC17128, i_c17128),
    (0xC1712C, i_c1712c),
    (0xC17130, i_c17130),
    (0xC17134, i_c17134),
    (0xC1713C, i_c1713c),
    (0xC17140, i_c17140),
    (0xC17144, i_c17144),
    (0xC17148, i_c17148),
    (0xC1714C, i_c1714c),
    (0xC17150, i_c17150),
    (0xC17154, i_c17154),
    (0xC17156, i_c17156),
    (0xC17158, i_c17158),
    (0xC1715E, i_c1715e),
    (0xC17160, i_c17160),
    (0xC17164, i_c17164),
    (0xC17168, i_c17168),
    (0xC1716C, i_c1716c),
    (0xC17170, i_c17170),
    (0xC17174, i_c17174),
    (0xC17178, i_c17178),
    (0xC1717C, i_c1717c),
    (0xC17180, i_c17180),
    (0xC17184, i_c17184),
    (0xC17188, i_c17188),
    (0xC1718C, i_c1718c),
    (0xC1718E, i_c1718e),
    (0xC17192, i_c17192),
    (0xC17196, i_c17196),
    (0xC17198, i_c17198),
    (0xC1719C, i_c1719c),
    (0xC1719E, i_c1719e),
    (0xC171A0, i_c171a0),
    (0xC171A4, i_c171a4),
    (0xC171A6, i_c171a6),
    (0xC171A8, i_c171a8),
    (0xC171AA, i_c171aa),
    (0xC171AE, i_c171ae),
    (0xC171B2, i_c171b2),
    (0xC171B6, i_c171b6),
    (0xC171B8, i_c171b8),
    (0xC171BA, i_c171ba),
    (0xC171BE, i_c171be),
    (0xC171C0, i_c171c0),
    (0xC171C2, i_c171c2),
    (0xC171C4, i_c171c4),
    (0xC171C6, i_c171c6),
    (0xC171C8, i_c171c8),
    (0xC171CC, i_c171cc),
    (0xC171CE, i_c171ce),
    (0xC171D0, i_c171d0),
    (0xC171D4, i_c171d4),
    (0xC171D8, i_c171d8),
    (0xC171DA, i_c171da),
    (0xC171DC, i_c171dc),
    (0xC171DE, i_c171de),
    (0xC171E0, i_c171e0),
    (0xC171E4, i_c171e4),
    (0xC171E6, i_c171e6),
    (0xC171E8, i_c171e8),
    (0xC171EC, i_c171ec),
    (0xC171EE, i_c171ee),
    (0xC171F4, i_c171f4),
    (0xC171F6, i_c171f6),
    (0xC171FC, i_c171fc),
    (0xC17200, i_c17200),
    (0xC17202, i_c17202),
    (0xC17206, i_c17206),
    (0xC1720A, i_c1720a),
    (0xC1720E, i_c1720e),
    (0xC17210, i_c17210),
    (0xC17212, i_c17212),
    (0xC17216, i_c17216),
    (0xC17218, i_c17218),
    (0xC1721C, i_c1721c),
    (0xC17220, i_c17220),
    (0xC17222, i_c17222),
    (0xC17224, i_c17224),
    (0xC17226, i_c17226),
    (0xC17228, i_c17228),
    (0xC1722A, i_c1722a),
    (0xC1722C, i_c1722c),
    (0xC17230, i_c17230),
    (0xC17232, i_c17232),
    (0xC17236, i_c17236),
    (0xC17238, i_c17238),
    (0xC1723A, i_c1723a),
    (0xC1723C, i_c1723c),
    (0xC1723E, i_c1723e),
    (0xC17240, i_c17240),
    (0xC17242, i_c17242),
    (0xC17244, i_c17244),
    (0xC17246, i_c17246),
    (0xC1724A, i_c1724a),
    (0xC1724C, i_c1724c),
    (0xC17250, i_c17250),
    (0xC17254, i_c17254),
    (0xC1725A, i_c1725a),
    (0xC1725C, i_c1725c),
    (0xC17262, i_c17262),
    (0xC17264, i_c17264),
    (0xC17268, i_c17268),
    (0xC1726C, i_c1726c),
    (0xC17272, i_c17272),
    (0xC17274, i_c17274),
    (0xC1727A, i_c1727a),
    (0xC1727C, i_c1727c),
    (0xC1727E, i_c1727e),
    (0xC17286, i_c17286),
    (0xC1728C, i_c1728c),
    (0xC1728E, i_c1728e),
    (0xC17290, i_c17290),
    (0xC17296, i_c17296),
    (0xC17298, i_c17298),
    (0xC1729E, i_c1729e),
    (0xC172A8, i_c172a8),
    (0xC172AE, i_c172ae),
    (0xC172B4, i_c172b4),
    (0xC172B8, i_c172b8),
    (0xC172BE, i_c172be),
    (0xC172C0, i_c172c0),
    (0xC172CA, i_c172ca),
    (0xC172D0, i_c172d0),
    (0xC172D2, i_c172d2),
    (0xC172D4, i_c172d4),
    (0xC172D6, i_c172d6),
    (0xC172D8, i_c172d8),
    (0xC172DA, i_c172da),
    (0xC172DC, i_c172dc),
    (0xC172DE, i_c172de),
    (0xC172E0, i_c172e0),
    (0xC172E4, i_c172e4),
    (0xC172E6, i_c172e6),
    (0xC172E8, i_c172e8),
    (0xC172EC, i_c172ec),
    (0xC172EE, i_c172ee),
    (0xC172F0, i_c172f0),
    (0xC17328, i_c17328),
    (0xC1732A, i_c1732a),
    (0xC1732C, i_c1732c),
    (0xC1732E, i_c1732e),
    (0xC17334, i_c17334),
    (0xC17336, i_c17336),
    (0xC17338, i_c17338),
    (0xC1733A, i_c1733a),
    (0xC1733C, i_c1733c),
    (0xC1733E, i_c1733e),
    (0xC17340, i_c17340),
    (0xC17344, i_c17344),
    (0xC17346, i_c17346),
    (0xC17348, i_c17348),
    (0xC1734A, i_c1734a),
    (0xC1734C, i_c1734c),
    (0xC1734E, i_c1734e),
    (0xC17354, i_c17354),
    (0xC17356, i_c17356),
    (0xC1735C, i_c1735c),
    (0xC1735E, i_c1735e),
    (0xC17360, i_c17360),
    (0xC17366, i_c17366),
    (0xC17368, i_c17368),
    (0xC1736E, i_c1736e),
    (0xC17370, i_c17370),
    (0xC17372, i_c17372),
    (0xC17374, i_c17374),
    (0xC17376, i_c17376),
    (0xC17378, i_c17378),
    (0xC1737A, i_c1737a),
    (0xC1737C, i_c1737c),
    (0xC1737E, i_c1737e),
    (0xC17380, i_c17380),
    (0xC17382, i_c17382),
    (0xC17384, i_c17384),
    (0xC17386, i_c17386),
    (0xC17388, i_c17388),
    (0xC1738A, i_c1738a),
    (0xC1738C, i_c1738c),
    (0xC1738E, i_c1738e),
    (0xC17390, i_c17390),
    (0xC17394, i_c17394),
    (0xC17396, i_c17396),
    (0xC17398, i_c17398),
    (0xC1739A, i_c1739a),
    (0xC1739C, i_c1739c),
    (0xC173A0, i_c173a0),
    (0xC173A2, i_c173a2),
    (0xC173A4, i_c173a4),
    (0xC173A6, i_c173a6),
    (0xC173A8, i_c173a8),
    (0xC173AA, i_c173aa),
    (0xC173AE, i_c173ae),
    (0xC173B0, i_c173b0),
    (0xC173B2, i_c173b2),
    (0xC173B4, i_c173b4),
    (0xC173B8, i_c173b8),
    (0xC173BA, i_c173ba),
    (0xC173BC, i_c173bc),
    (0xC173BE, i_c173be),
];
