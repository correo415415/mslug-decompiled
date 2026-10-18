// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región bios_0) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $C00000: ori.b #$0, (a0)
pub fn i_c00000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00004;
    cpu.ir = 0x10;
    let ad1 = cpu.a[0];
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C00002: dc.w $f300
pub fn i_c00002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00004;
    cpu.ir = 0xF300;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00004: ?
pub fn i_c00004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00006;
    cpu.ir = 0xC0;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C00006: subi.b #$c0, d2
pub fn i_c00006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0000A;
    cpu.ir = 0x402;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0xC0, Size::Byte, true);
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C00008: ?
pub fn i_c00008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0000A;
    cpu.ir = 0xC0;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C0000A: ?
pub fn i_c0000a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0000E;
    cpu.ir = 0x408;
    let v1 = cpu.a[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0xC0, Size::Byte, true);
    cpu.a[0] = r2;
    8
}
/// $C0000E: ?
pub fn i_c0000e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00012;
    cpu.ir = 0x40E;
    let v1 = cpu.a[6] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0xC0, Size::Byte, true);
    cpu.a[6] = r2;
    8
}
/// $C00012: subi.b #$c0, (a4)
pub fn i_c00012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00016;
    cpu.ir = 0x414;
    let ad1 = cpu.a[4];
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0xC0, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C00014: ?
pub fn i_c00014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00016;
    cpu.ir = 0xC0;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C00016: subi.b #$c0, -(a6)
pub fn i_c00016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0001A;
    cpu.ir = 0x426;
    let ad1 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0xC0, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    14
}
/// $C00018: ?
pub fn i_c00018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0001A;
    cpu.ir = 0xC0;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C0001A: subi.b #$c0, -(a6)
pub fn i_c0001a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0001E;
    cpu.ir = 0x426;
    let ad1 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0xC0, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    14
}
/// $C0001C: ?
pub fn i_c0001c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0001E;
    cpu.ir = 0xC0;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C0001E: subi.b #$c0, -(a6)
pub fn i_c0001e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00022;
    cpu.ir = 0x426;
    let ad1 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0xC0, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    14
}
/// $C00020: ?
pub fn i_c00020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00022;
    cpu.ir = 0xC0;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C00022: subi.b #$c0, (a2)+
pub fn i_c00022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00026;
    cpu.ir = 0x41A;
    let ad1 = cpu.a[2]; cpu.a[2] = ad1.wrapping_add(1);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0xC0, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C00024: ?
pub fn i_c00024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00026;
    cpu.ir = 0xC0;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C00026: subi.b #$c0, -(a0)
pub fn i_c00026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0002A;
    cpu.ir = 0x420;
    let ad1 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0xC0, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    14
}
/// $C00028: ?
pub fn i_c00028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0002A;
    cpu.ir = 0xC0;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C0002A: subi.b #$c0, -(a6)
pub fn i_c0002a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0002E;
    cpu.ir = 0x426;
    let ad1 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0xC0, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    14
}
/// $C0002E: subi.b #$c0, -(a6)
pub fn i_c0002e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00032;
    cpu.ir = 0x426;
    let ad1 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0xC0, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    14
}
/// $C00032: subi.b #$c0, -(a6)
pub fn i_c00032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00036;
    cpu.ir = 0x426;
    let ad1 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0xC0, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    14
}
/// $C00034: ?
pub fn i_c00034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00036;
    cpu.ir = 0xC0;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C00036: subi.b #$c0, -(a6)
pub fn i_c00036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0003A;
    cpu.ir = 0x426;
    let ad1 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0xC0, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    14
}
/// $C0003A: subi.b #$c0, -(a6)
pub fn i_c0003a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0003E;
    cpu.ir = 0x426;
    let ad1 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0xC0, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    14
}
/// $C0003C: ?
pub fn i_c0003c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0003E;
    cpu.ir = 0xC0;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C0003E: subi.b #$c0, $426(a4)
pub fn i_c0003e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00044;
    cpu.ir = 0x42C;
    let ad1 = cpu.a[4].wrapping_add(0x426);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0xC0, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $C00040: ?
pub fn i_c00040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00042;
    cpu.ir = 0xC0;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C00042: subi.b #$c0, -(a6)
pub fn i_c00042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00046;
    cpu.ir = 0x426;
    let ad1 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0xC0, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    14
}
/// $C00044: ?
pub fn i_c00044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00046;
    cpu.ir = 0xC0;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C00046: subi.b #$c0, -(a6)
pub fn i_c00046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0004A;
    cpu.ir = 0x426;
    let ad1 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0xC0, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    14
}
/// $C0004A: subi.b #$c0, -(a6)
pub fn i_c0004a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0004E;
    cpu.ir = 0x426;
    let ad1 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0xC0, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    14
}
/// $C0004E: subi.b #$c0, -(a6)
pub fn i_c0004e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00052;
    cpu.ir = 0x426;
    let ad1 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0xC0, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    14
}
/// $C00050: ?
pub fn i_c00050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00052;
    cpu.ir = 0xC0;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C00052: subi.b #$c0, -(a6)
pub fn i_c00052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00056;
    cpu.ir = 0x426;
    let ad1 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0xC0, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    14
}
/// $C00056: subi.b #$c0, -(a6)
pub fn i_c00056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0005A;
    cpu.ir = 0x426;
    let ad1 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0xC0, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    14
}
/// $C0005A: subi.b #$c0, -(a6)
pub fn i_c0005a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0005E;
    cpu.ir = 0x426;
    let ad1 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0xC0, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    14
}
/// $C0005E: subi.b #$c0, -(a6)
pub fn i_c0005e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00062;
    cpu.ir = 0x426;
    let ad1 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0xC0, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    14
}
/// $C00060: ?
pub fn i_c00060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00062;
    cpu.ir = 0xC0;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C00062: subi.b #$c0, $38(a2, d0.w)
pub fn i_c00062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00068;
    cpu.ir = 0x432;
    let ad1 = cpu.a[2].wrapping_add(0x38).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0xC0, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    18
}
/// $C00068: ?
pub fn i_c00068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0006A;
    cpu.ir = 0xC0;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C0006A: ?
pub fn i_c0006a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0006E;
    cpu.ir = 0x43E;
    let v1: u32 = 0x0;
    let r2 = sub_with_flags(cpu, v1, 0xC0, Size::Byte, true);
    8
}
/// $C0006E: subi.b #$c0, -(a6)
pub fn i_c0006e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00072;
    cpu.ir = 0x426;
    let ad1 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0xC0, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    14
}
/// $C00070: ?
pub fn i_c00070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00072;
    cpu.ir = 0xC0;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C00072: subi.b #$c0, -(a6)
pub fn i_c00072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00076;
    cpu.ir = 0x426;
    let ad1 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0xC0, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    14
}
/// $C00074: ?
pub fn i_c00074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00076;
    cpu.ir = 0xC0;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C00076: subi.b #$c0, -(a6)
pub fn i_c00076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0007A;
    cpu.ir = 0x426;
    let ad1 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0xC0, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    14
}
/// $C00078: ?
pub fn i_c00078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0007A;
    cpu.ir = 0xC0;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C0007A: subi.b #$c0, -(a6)
pub fn i_c0007a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0007E;
    cpu.ir = 0x426;
    let ad1 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0xC0, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    14
}
/// $C0007E: subi.b #$6f, -(a6)
pub fn i_c0007e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00082;
    cpu.ir = 0x426;
    let ad1 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x6F, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    14
}
/// $C00082: trap #$5
pub fn i_c00082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00084;
    cpu.ir = 0x4E45;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0x94);
    34
}
/// $C00084: dc.w $4f2d
pub fn i_c00084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00086;
    cpu.ir = 0x4F2D;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C00086: ?
pub fn i_c00086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00088;
    cpu.ir = 0x4745;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C00088: dc.w $4f20
pub fn i_c00088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0008A;
    cpu.ir = 0x4F20;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C0008A: subq.w #$1, (a1)+
pub fn i_c0008a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0008C;
    cpu.ir = 0x5359;
    let ad1 = cpu.a[1]; cpu.a[1] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C0008C: subq.w #$1, (a4)
pub fn i_c0008c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0008E;
    cpu.ir = 0x5354;
    let ad1 = cpu.a[4];
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C0008E: ?
pub fn i_c0008e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00090;
    cpu.ir = 0x454D;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C00090: movea.l (a0), a0
pub fn i_c00090(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00092;
    cpu.ir = 0x2050;
    let v2 = bus.read32(cpu.a[0]);
    cpu.a[0] = v2;
    12
}
/// $C00092: addq.w #$1, a7
pub fn i_c00092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00094;
    cpu.ir = 0x524F;
    cpu.a[7] = cpu.a[7].wrapping_add(1);
    8
}
/// $C00094: ?
pub fn i_c00094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00096;
    cpu.ir = 0x4752;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C00096: ?
pub fn i_c00096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00098;
    cpu.ir = 0x414D;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C00098: movea.l (a6), a0
pub fn i_c00098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0009A;
    cpu.ir = 0x2056;
    let v2 = bus.read32(cpu.a[6]);
    cpu.a[0] = v2;
    12
}
/// $C0009A: ?
pub fn i_c0009a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0009C;
    cpu.ir = 0x4552;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C0009C: subq.w #$1, a1
pub fn i_c0009c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0009E;
    cpu.ir = 0x5349;
    cpu.a[1] = cpu.a[1].wrapping_sub(1);
    8
}
/// $C0009E: ?
pub fn i_c0009e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC000A0;
    cpu.ir = 0x4F4E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C000A0: move.l $43(a0, d2.w), d0
pub fn i_c000a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC000A4;
    cpu.ir = 0x2030;
    let ad2 = cpu.a[0].wrapping_add(0x43).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $C000A4: ?
pub fn i_c000a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC000A6;
    cpu.ir = 0x4F50;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C000A6: subq.w #$4, (a2)
pub fn i_c000a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC000A8;
    cpu.ir = 0x5952;
    let ad1 = cpu.a[2];
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 4, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C000A8: ?
pub fn i_c000a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC000AA;
    cpu.ir = 0x4947;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C000AA: pea.l (a4)
pub fn i_c000aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC000AC;
    cpu.ir = 0x4854;
    cpu.push32(bus, cpu.a[4]);
    20
}
/// $C000AC: movea.l d2, a0
pub fn i_c000ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC000AE;
    cpu.ir = 0x2042;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C000AE: subq.b #$4, -(a0)
pub fn i_c000ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC000B0;
    cpu.ir = 0x5920;
    let ad1 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 4, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    10
}
/// $C000B0: subq.w #$1, a6
pub fn i_c000b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC000B2;
    cpu.ir = 0x534E;
    cpu.a[6] = cpu.a[6].wrapping_sub(1);
    8
}
/// $C000B2: dc.w $4b20
pub fn i_c000b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC000B4;
    cpu.ir = 0x4B20;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C000B4: ?
pub fn i_c000b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC000B6;
    cpu.ir = 0x434F;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C000B6: addq.w #$1, (a0)
pub fn i_c000b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC000B8;
    cpu.ir = 0x5250;
    let ad1 = cpu.a[0];
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C000B8: ?
pub fn i_c000b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC000BA;
    cpu.ir = 0x4F52;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C000BA: ?
pub fn i_c000ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC000BC;
    cpu.ir = 0x4154;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C000BC: ?
pub fn i_c000bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC000BE;
    cpu.ir = 0x494F;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C000BE: ?
pub fn i_c000be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC000C0;
    cpu.ir = 0x4EFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C000C0: dc.w $ffff
pub fn i_c000c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC000C2;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C000C2: dc.w $ffff
pub fn i_c000c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC000C4;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C000C4: dc.w $ffff
pub fn i_c000c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC000C6;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C000C6: dc.w $ffff
pub fn i_c000c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC000C8;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C000C8: dc.w $ffff
pub fn i_c000c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC000CA;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C000CA: dc.w $ffff
pub fn i_c000ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC000CC;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C000CC: dc.w $ffff
pub fn i_c000cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC000CE;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C000CE: dc.w $ffff
pub fn i_c000ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC000D0;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C000D0: dc.w $ffff
pub fn i_c000d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC000D2;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C000D2: dc.w $ffff
pub fn i_c000d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC000D4;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C000D4: dc.w $ffff
pub fn i_c000d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC000D6;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C000D6: dc.w $ffff
pub fn i_c000d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC000D8;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C000D8: dc.w $ffff
pub fn i_c000d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC000DA;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C000DA: dc.w $ffff
pub fn i_c000da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC000DC;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C000DC: dc.w $ffff
pub fn i_c000dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC000DE;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C000DE: dc.w $ffff
pub fn i_c000de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC000E0;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C000E0: dc.w $ffff
pub fn i_c000e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC000E2;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C000E2: dc.w $ffff
pub fn i_c000e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC000E4;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C000E4: dc.w $ffff
pub fn i_c000e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC000E6;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C000E6: dc.w $ffff
pub fn i_c000e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC000E8;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C000E8: dc.w $ffff
pub fn i_c000e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC000EA;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C000EA: dc.w $ffff
pub fn i_c000ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC000EC;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C000EC: dc.w $ffff
pub fn i_c000ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC000EE;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C000EE: dc.w $ffff
pub fn i_c000ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC000F0;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C000F0: dc.w $ffff
pub fn i_c000f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC000F2;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00100: dc.w $ffff
pub fn i_c00100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00102;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00102: dc.w $ffff
pub fn i_c00102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00104;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00104: dc.w $ffff
pub fn i_c00104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00106;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00106: dc.w $ffff
pub fn i_c00106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00108;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00108: dc.w $ffff
pub fn i_c00108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0010A;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0010A: dc.w $ffff
pub fn i_c0010a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0010C;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0010C: dc.w $ffff
pub fn i_c0010c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0010E;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0010E: dc.w $ffff
pub fn i_c0010e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00110;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00110: dc.w $ffff
pub fn i_c00110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00112;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00112: dc.w $ffff
pub fn i_c00112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00114;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00114: dc.w $ffff
pub fn i_c00114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00116;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00116: dc.w $ffff
pub fn i_c00116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00118;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00118: dc.w $ffff
pub fn i_c00118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0011A;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0011A: dc.w $ffff
pub fn i_c0011a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0011C;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0011C: dc.w $ffff
pub fn i_c0011c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0011E;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0011E: dc.w $ffff
pub fn i_c0011e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00120;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00120: dc.w $ffff
pub fn i_c00120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00122;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00122: dc.w $ffff
pub fn i_c00122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00124;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00124: dc.w $ffff
pub fn i_c00124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00126;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00126: dc.w $ffff
pub fn i_c00126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00128;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00128: dc.w $ffff
pub fn i_c00128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0012A;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0012A: dc.w $ffff
pub fn i_c0012a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0012C;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0012C: dc.w $ffff
pub fn i_c0012c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0012E;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0012E: dc.w $ffff
pub fn i_c0012e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00130;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00130: dc.w $ffff
pub fn i_c00130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00132;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00132: dc.w $ffff
pub fn i_c00132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00134;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00134: dc.w $ffff
pub fn i_c00134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00136;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00136: dc.w $ffff
pub fn i_c00136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00138;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00138: dc.w $ffff
pub fn i_c00138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0013A;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0013A: dc.w $ffff
pub fn i_c0013a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0013C;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0013C: dc.w $ffff
pub fn i_c0013c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0013E;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0013E: dc.w $ffff
pub fn i_c0013e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00140;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00140: dc.w $ffff
pub fn i_c00140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00142;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00142: dc.w $ffff
pub fn i_c00142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00144;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00144: dc.w $ffff
pub fn i_c00144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00146;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00146: dc.w $ffff
pub fn i_c00146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00148;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00148: dc.w $ffff
pub fn i_c00148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0014A;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0014A: dc.w $ffff
pub fn i_c0014a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0014C;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0014C: dc.w $ffff
pub fn i_c0014c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0014E;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0014E: dc.w $ffff
pub fn i_c0014e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00150;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00150: dc.w $ffff
pub fn i_c00150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00152;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00158: dc.w $ffff
pub fn i_c00158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0015A;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0015A: dc.w $ffff
pub fn i_c0015a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0015C;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0015C: dc.w $ffff
pub fn i_c0015c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0015E;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0015E: dc.w $ffff
pub fn i_c0015e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00160;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00160: dc.w $ffff
pub fn i_c00160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00162;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00162: dc.w $ffff
pub fn i_c00162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00164;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00164: dc.w $ffff
pub fn i_c00164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00166;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00166: dc.w $ffff
pub fn i_c00166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00168;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00168: dc.w $ffff
pub fn i_c00168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0016A;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0016A: dc.w $ffff
pub fn i_c0016a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0016C;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0016C: dc.w $ffff
pub fn i_c0016c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0016E;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0016E: dc.w $ffff
pub fn i_c0016e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00170;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00170: dc.w $ffff
pub fn i_c00170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00172;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00172: dc.w $ffff
pub fn i_c00172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00174;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00174: dc.w $ffff
pub fn i_c00174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00176;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00180: dc.w $ffff
pub fn i_c00180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00182;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00182: dc.w $ffff
pub fn i_c00182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00184;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00184: dc.w $ffff
pub fn i_c00184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00186;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00186: dc.w $ffff
pub fn i_c00186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00188;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00188: dc.w $ffff
pub fn i_c00188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0018A;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0018A: dc.w $ffff
pub fn i_c0018a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0018C;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0018C: dc.w $ffff
pub fn i_c0018c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0018E;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0018E: dc.w $ffff
pub fn i_c0018e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00190;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00190: dc.w $ffff
pub fn i_c00190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00192;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00198: dc.w $ffff
pub fn i_c00198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0019A;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0019A: dc.w $ffff
pub fn i_c0019a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0019C;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0019C: dc.w $ffff
pub fn i_c0019c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0019E;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0019E: dc.w $ffff
pub fn i_c0019e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC001A0;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C001A0: dc.w $ffff
pub fn i_c001a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC001A2;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C001A2: dc.w $ffff
pub fn i_c001a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC001A4;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C001A4: dc.w $ffff
pub fn i_c001a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC001A6;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C001A6: dc.w $ffff
pub fn i_c001a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC001A8;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C001A8: dc.w $ffff
pub fn i_c001a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC001AA;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C001B0: dc.w $ffff
pub fn i_c001b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC001B2;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C001B2: dc.w $ffff
pub fn i_c001b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC001B4;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C001B4: dc.w $ffff
pub fn i_c001b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC001B6;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C001B6: dc.w $ffff
pub fn i_c001b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC001B8;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C001B8: dc.w $ffff
pub fn i_c001b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC001BA;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C001BA: dc.w $ffff
pub fn i_c001ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC001BC;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C001BC: dc.w $ffff
pub fn i_c001bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC001BE;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C001BE: dc.w $ffff
pub fn i_c001be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC001C0;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C001C0: dc.w $ffff
pub fn i_c001c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC001C2;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00200: dc.w $ffff
pub fn i_c00200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00202;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00202: dc.w $ffff
pub fn i_c00202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00204;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00204: dc.w $ffff
pub fn i_c00204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00206;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00206: dc.w $ffff
pub fn i_c00206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00208;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00208: dc.w $ffff
pub fn i_c00208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0020A;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0020A: dc.w $ffff
pub fn i_c0020a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0020C;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0020C: dc.w $ffff
pub fn i_c0020c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0020E;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0020E: dc.w $ffff
pub fn i_c0020e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00210;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00210: dc.w $ffff
pub fn i_c00210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00212;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00212: dc.w $ffff
pub fn i_c00212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00214;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00214: dc.w $ffff
pub fn i_c00214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00216;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00216: dc.w $ffff
pub fn i_c00216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00218;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00218: dc.w $ffff
pub fn i_c00218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0021A;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0021A: dc.w $ffff
pub fn i_c0021a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0021C;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0021C: dc.w $ffff
pub fn i_c0021c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0021E;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0021E: dc.w $ffff
pub fn i_c0021e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00220;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00220: dc.w $ffff
pub fn i_c00220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00222;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00222: dc.w $ffff
pub fn i_c00222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00224;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00224: dc.w $ffff
pub fn i_c00224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00226;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00228: dc.w $ffff
pub fn i_c00228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0022A;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0022A: dc.w $ffff
pub fn i_c0022a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0022C;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0022C: dc.w $ffff
pub fn i_c0022c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0022E;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0022E: dc.w $ffff
pub fn i_c0022e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00230;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00230: dc.w $ffff
pub fn i_c00230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00232;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00232: dc.w $ffff
pub fn i_c00232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00234;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00234: dc.w $ffff
pub fn i_c00234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00236;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00236: dc.w $ffff
pub fn i_c00236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00238;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00238: dc.w $ffff
pub fn i_c00238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0023A;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0023A: dc.w $ffff
pub fn i_c0023a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0023C;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0023C: dc.w $ffff
pub fn i_c0023c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0023E;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0023E: dc.w $ffff
pub fn i_c0023e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00240;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00240: dc.w $ffff
pub fn i_c00240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00242;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00242: dc.w $ffff
pub fn i_c00242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00244;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00244: dc.w $ffff
pub fn i_c00244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00246;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00246: dc.w $ffff
pub fn i_c00246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00248;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00248: dc.w $ffff
pub fn i_c00248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0024A;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0024A: dc.w $ffff
pub fn i_c0024a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0024C;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0024C: dc.w $ffff
pub fn i_c0024c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0024E;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0024E: dc.w $ffff
pub fn i_c0024e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00250;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00250: dc.w $ffff
pub fn i_c00250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00252;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00252: dc.w $ffff
pub fn i_c00252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00254;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00254: dc.w $ffff
pub fn i_c00254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00256;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00256: dc.w $ffff
pub fn i_c00256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00258;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00258: dc.w $ffff
pub fn i_c00258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0025A;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0025A: dc.w $ffff
pub fn i_c0025a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0025C;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0025C: dc.w $ffff
pub fn i_c0025c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0025E;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0025E: dc.w $ffff
pub fn i_c0025e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00260;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00260: dc.w $ffff
pub fn i_c00260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00262;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00278: dc.w $ffff
pub fn i_c00278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0027A;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0027A: dc.w $ffff
pub fn i_c0027a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0027C;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0027C: dc.w $ffff
pub fn i_c0027c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0027E;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0027E: dc.w $ffff
pub fn i_c0027e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00280;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00280: dc.w $ffff
pub fn i_c00280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00282;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00282: dc.w $ffff
pub fn i_c00282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00284;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00284: dc.w $ffff
pub fn i_c00284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00286;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00286: dc.w $ffff
pub fn i_c00286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00288;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00288: dc.w $ffff
pub fn i_c00288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0028A;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00296: dc.w $ffff
pub fn i_c00296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00298;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00298: dc.w $ffff
pub fn i_c00298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0029A;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0029A: dc.w $ffff
pub fn i_c0029a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0029C;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0029C: dc.w $ffff
pub fn i_c0029c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0029E;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0029E: dc.w $ffff
pub fn i_c0029e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC002A0;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C002A0: dc.w $ffff
pub fn i_c002a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC002A2;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C002A2: dc.w $ffff
pub fn i_c002a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC002A4;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C002A4: dc.w $ffff
pub fn i_c002a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC002A6;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C002A6: dc.w $ffff
pub fn i_c002a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC002A8;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00300: dc.w $ffff
pub fn i_c00300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00302;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00302: dc.w $ffff
pub fn i_c00302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00304;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00304: dc.w $ffff
pub fn i_c00304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00306;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00306: dc.w $ffff
pub fn i_c00306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00308;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00308: dc.w $ffff
pub fn i_c00308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0030A;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0030A: dc.w $ffff
pub fn i_c0030a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0030C;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0030C: dc.w $ffff
pub fn i_c0030c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0030E;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0030E: dc.w $ffff
pub fn i_c0030e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00310;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00310: dc.w $ffff
pub fn i_c00310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00312;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00312: dc.w $ffff
pub fn i_c00312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00314;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00314: dc.w $ffff
pub fn i_c00314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00316;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00316: dc.w $ffff
pub fn i_c00316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00318;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00318: dc.w $ffff
pub fn i_c00318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0031A;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0031A: dc.w $ffff
pub fn i_c0031a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0031C;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0031C: dc.w $ffff
pub fn i_c0031c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0031E;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0031E: dc.w $ffff
pub fn i_c0031e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00320;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00320: dc.w $ffff
pub fn i_c00320(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00322;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00322: dc.w $ffff
pub fn i_c00322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00324;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00324: dc.w $ffff
pub fn i_c00324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00326;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00326: dc.w $ffff
pub fn i_c00326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00328;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00328: dc.w $ffff
pub fn i_c00328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0032A;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0032A: dc.w $ffff
pub fn i_c0032a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0032C;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0032C: dc.w $ffff
pub fn i_c0032c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0032E;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0032E: dc.w $ffff
pub fn i_c0032e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00330;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00400: or.b d2, d0
pub fn i_c00400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00402;
    cpu.ir = 0x8002;
    let v1 = cpu.d[2] & 0xFF;
    let r = (v1 | cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C00402: jmp $c10002.l
pub fn i_c00402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00408;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC10002;
    8
}
/// $C00404: ?
pub fn i_c00404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00406;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C00406: ori.b #$f9, d2
pub fn i_c00406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0040A;
    cpu.ir = 0x2;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 | 0xF9) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C00408: jmp $c10002.l
pub fn i_c00408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0040E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC10002;
    8
}
/// $C0040A: ?
pub fn i_c0040a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0040C;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C0040C: ori.b #$f9, d2
pub fn i_c0040c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00410;
    cpu.ir = 0x2;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 | 0xF9) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C0040E: jmp $c10002.l
pub fn i_c0040e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00414;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC10002;
    8
}
/// $C00410: ?
pub fn i_c00410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00412;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C00412: ori.b #$f9, d2
pub fn i_c00412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00416;
    cpu.ir = 0x2;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 | 0xF9) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C00414: jmp $c10002.l
pub fn i_c00414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0041A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC10002;
    8
}
/// $C00416: ?
pub fn i_c00416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00418;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C00418: ori.b #$f9, d2
pub fn i_c00418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0041C;
    cpu.ir = 0x2;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 | 0xF9) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C0041A: jmp $c10002.l
pub fn i_c0041a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00420;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC10002;
    8
}
/// $C0041C: ?
pub fn i_c0041c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0041E;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C0041E: ori.b #$f9, d2
pub fn i_c0041e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00422;
    cpu.ir = 0x2;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 | 0xF9) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C00420: jmp $c10002.l
pub fn i_c00420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00426;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC10002;
    8
}
/// $C00422: ?
pub fn i_c00422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00424;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C00424: ori.b #$f9, d2
pub fn i_c00424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00428;
    cpu.ir = 0x2;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 | 0xF9) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C00426: jmp $c10002.l
pub fn i_c00426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0042C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC10002;
    8
}
/// $C00428: ?
pub fn i_c00428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0042A;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C0042A: ori.b #$f9, d2
pub fn i_c0042a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0042E;
    cpu.ir = 0x2;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 | 0xF9) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C0042C: jmp $c10002.l
pub fn i_c0042c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00432;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC10002;
    8
}
/// $C0042E: ?
pub fn i_c0042e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00430;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C00430: ori.b #$f9, d2
pub fn i_c00430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00434;
    cpu.ir = 0x2;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 | 0xF9) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C00432: jmp $c10002.l
pub fn i_c00432(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00438;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC10002;
    8
}
/// $C00434: ?
pub fn i_c00434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00436;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C00436: ori.b #$f9, d2
pub fn i_c00436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0043A;
    cpu.ir = 0x2;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 | 0xF9) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C00438: jmp $c10646.l
pub fn i_c00438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0043E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC10646;
    8
}
/// $C0043A: ?
pub fn i_c0043a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0043C;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C0043C: addi.w #$4ef9, d6
pub fn i_c0043c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00440;
    cpu.ir = 0x646;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x4EF9, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C0043E: jmp $c108ca.l
pub fn i_c0043e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00444;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC108CA;
    8
}
/// $C00440: ?
pub fn i_c00440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00442;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C00442: ?
pub fn i_c00442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00446;
    cpu.ir = 0x8CA;
    let mk: u32 = 1u32 << ((249) % 32);
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 | mk;
    cpu.a[2] = r2;
    8
}
/// $C00444: jmp $c102d2.l
pub fn i_c00444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0044A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC102D2;
    8
}
/// $C00446: ?
pub fn i_c00446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00448;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C00448: ?
pub fn i_c00448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0044A;
    cpu.ir = 0x2D2;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C0044A: jmp $c1067a.l
pub fn i_c0044a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00450;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC1067A;
    8
}
/// $C00450: jmp $c107ce.l
pub fn i_c00450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00456;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC107CE;
    8
}
/// $C00456: jmp $c10820.l
pub fn i_c00456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0045C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC10820;
    8
}
/// $C0045C: jmp $c11c42.l
pub fn i_c0045c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00462;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC11C42;
    8
}
/// $C00462: jmp $c11cc0.l
pub fn i_c00462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00468;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC11CC0;
    8
}
/// $C00468: jmp $c1231e.l
pub fn i_c00468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0046E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC1231E;
    8
}
/// $C0046E: jmp $c12392.l
pub fn i_c0046e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00474;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC12392;
    8
}
/// $C00474: jmp $c17486.l
pub fn i_c00474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0047A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC17486;
    8
}
/// $C0047A: jmp $c15be0.l
pub fn i_c0047a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00480;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC15BE0;
    8
}
/// $C00480: jmp $c004da(pc)
pub fn i_c00480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00484;
    cpu.ir = 0x4EFA;
    cpu.pc = 0xC004DA;
    8
}
/// $C00484: nop
pub fn i_c00484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00486;
    cpu.ir = 0x4E71;
    4
}
/// $C00486: jmp $c004da(pc)
pub fn i_c00486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0048A;
    cpu.ir = 0x4EFA;
    cpu.pc = 0xC004DA;
    8
}
/// $C0048A: nop
pub fn i_c0048a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0048C;
    cpu.ir = 0x4E71;
    4
}
/// $C0048C: jmp $c004da(pc)
pub fn i_c0048c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00490;
    cpu.ir = 0x4EFA;
    cpu.pc = 0xC004DA;
    8
}
/// $C00490: nop
pub fn i_c00490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00492;
    cpu.ir = 0x4E71;
    4
}
/// $C00492: jmp $c004da(pc)
pub fn i_c00492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00496;
    cpu.ir = 0x4EFA;
    cpu.pc = 0xC004DA;
    8
}
/// $C00496: nop
pub fn i_c00496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00498;
    cpu.ir = 0x4E71;
    4
}
/// $C00498: jmp $c004da(pc)
pub fn i_c00498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0049C;
    cpu.ir = 0x4EFA;
    cpu.pc = 0xC004DA;
    8
}
/// $C0049C: nop
pub fn i_c0049c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0049E;
    cpu.ir = 0x4E71;
    4
}
/// $C0049E: jmp $c004da(pc)
pub fn i_c0049e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC004A2;
    cpu.ir = 0x4EFA;
    cpu.pc = 0xC004DA;
    8
}
/// $C004C2: jmp $c16ed0.l
pub fn i_c004c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC004C8;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC16ED0;
    8
}
/// $C004C8: jmp $c16f0c.l
pub fn i_c004c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC004CE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC16F0C;
    8
}
/// $C004CE: jmp $c17290.l
pub fn i_c004ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC004D4;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC17290;
    8
}
/// $C004D4: jmp $c16fd4.l
pub fn i_c004d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC004DA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC16FD4;
    8
}
/// $C004DA: rts
pub fn i_c004da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC004DC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C004DC: dc.w $ffff
pub fn i_c004dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC004DE;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C004DE: dc.w $ffff
pub fn i_c004de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC004E0;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C004E0: dc.w $ffff
pub fn i_c004e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC004E2;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C004E2: dc.w $ffff
pub fn i_c004e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC004E4;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C004E4: dc.w $ffff
pub fn i_c004e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC004E6;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C004E6: dc.w $ffff
pub fn i_c004e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC004E8;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00506: dc.w $ffff
pub fn i_c00506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00508;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00508: dc.w $ffff
pub fn i_c00508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0050A;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0050A: dc.w $ffff
pub fn i_c0050a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0050C;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0050C: dc.w $ffff
pub fn i_c0050c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0050E;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0050E: dc.w $ffff
pub fn i_c0050e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00510;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00510: dc.w $ffff
pub fn i_c00510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00512;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00512: dc.w $ffff
pub fn i_c00512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00514;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00514: dc.w $ffff
pub fn i_c00514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00516;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00516: dc.w $ffff
pub fn i_c00516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00518;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C005E8: dc.w $ffff
pub fn i_c005e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC005EA;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C005EA: dc.w $ffff
pub fn i_c005ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC005EC;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C005EC: dc.w $ffff
pub fn i_c005ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC005EE;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C005EE: dc.w $ffff
pub fn i_c005ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC005F0;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C005F0: dc.w $ffff
pub fn i_c005f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC005F2;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C005F2: dc.w $ffff
pub fn i_c005f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC005F4;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C005F4: dc.w $ffff
pub fn i_c005f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC005F6;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C005F6: dc.w $ffff
pub fn i_c005f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC005F8;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C005F8: dc.w $ffff
pub fn i_c005f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC005FA;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C005FA: dc.w $ffff
pub fn i_c005fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC005FC;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C005FC: dc.w $ffff
pub fn i_c005fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC005FE;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C005FE: dc.w $ffff
pub fn i_c005fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00600;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00600: dc.w $ffff
pub fn i_c00600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00602;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00602: dc.w $ffff
pub fn i_c00602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00604;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00604: dc.w $ffff
pub fn i_c00604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00606;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00606: dc.w $ffff
pub fn i_c00606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00608;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00608: dc.w $ffff
pub fn i_c00608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0060A;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0060A: dc.w $ffff
pub fn i_c0060a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0060C;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0060C: dc.w $ffff
pub fn i_c0060c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0060E;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0060E: dc.w $ffff
pub fn i_c0060e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00610;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00610: dc.w $ffff
pub fn i_c00610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00612;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00612: dc.w $ffff
pub fn i_c00612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00614;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00614: dc.w $ffff
pub fn i_c00614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00616;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00616: dc.w $ffff
pub fn i_c00616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00618;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00618: dc.w $ffff
pub fn i_c00618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0061A;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0061A: dc.w $ffff
pub fn i_c0061a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0061C;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00A00: dc.w $ffff
pub fn i_c00a00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00A02;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00A02: dc.w $ffff
pub fn i_c00a02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00A04;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00A04: dc.w $ffff
pub fn i_c00a04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00A06;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00A06: dc.w $ffff
pub fn i_c00a06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00A08;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00A08: dc.w $ffff
pub fn i_c00a08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00A0A;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00A0A: dc.w $ffff
pub fn i_c00a0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00A0C;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00A0C: dc.w $ffff
pub fn i_c00a0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00A0E;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00A0E: dc.w $ffff
pub fn i_c00a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00A10;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00A10: dc.w $ffff
pub fn i_c00a10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00A12;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00C00: dc.w $ffff
pub fn i_c00c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00C02;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00C02: dc.w $ffff
pub fn i_c00c02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00C04;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00C04: dc.w $ffff
pub fn i_c00c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00C06;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00C06: dc.w $ffff
pub fn i_c00c06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00C08;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00C08: dc.w $ffff
pub fn i_c00c08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00C0A;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00C0A: dc.w $ffff
pub fn i_c00c0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00C0C;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00C0C: dc.w $ffff
pub fn i_c00c0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00C0E;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00C0E: dc.w $ffff
pub fn i_c00c0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00C10;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00C10: dc.w $ffff
pub fn i_c00c10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00C12;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00C6E: dc.w $ffff
pub fn i_c00c6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00C70;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00C70: dc.w $ffff
pub fn i_c00c70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00C72;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00C72: dc.w $ffff
pub fn i_c00c72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00C74;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00C74: dc.w $ffff
pub fn i_c00c74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00C76;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00C76: dc.w $ffff
pub fn i_c00c76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00C78;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00C78: dc.w $ffff
pub fn i_c00c78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00C7A;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00C7A: dc.w $ffff
pub fn i_c00c7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00C7C;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00C7C: dc.w $ffff
pub fn i_c00c7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00C7E;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00C7E: dc.w $ffff
pub fn i_c00c7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00C80;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00E02: dc.w $ffff
pub fn i_c00e02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00E04;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00E04: dc.w $ffff
pub fn i_c00e04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00E06;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00E06: dc.w $ffff
pub fn i_c00e06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00E08;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00E08: dc.w $ffff
pub fn i_c00e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00E0A;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00E0A: dc.w $ffff
pub fn i_c00e0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00E0C;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00E0C: dc.w $ffff
pub fn i_c00e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00E0E;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00E0E: dc.w $ffff
pub fn i_c00e0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00E10;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00E10: dc.w $ffff
pub fn i_c00e10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00E12;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C00E12: dc.w $ffff
pub fn i_c00e12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00E14;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C010C0: dc.w $ffff
pub fn i_c010c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC010C2;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C010C2: dc.w $ffff
pub fn i_c010c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC010C4;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C010C4: dc.w $ffff
pub fn i_c010c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC010C6;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C010C6: dc.w $ffff
pub fn i_c010c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC010C8;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C010C8: dc.w $ffff
pub fn i_c010c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC010CA;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C010CA: dc.w $ffff
pub fn i_c010ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC010CC;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C010CC: dc.w $ffff
pub fn i_c010cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC010CE;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C010CE: dc.w $ffff
pub fn i_c010ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC010D0;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C010D0: dc.w $ffff
pub fn i_c010d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC010D2;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C010FC: dc.w $ffff
pub fn i_c010fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC010FE;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C010FE: dc.w $ffff
pub fn i_c010fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC01100;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C01100: dc.w $ffff
pub fn i_c01100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC01102;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C01102: dc.w $ffff
pub fn i_c01102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC01104;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C01104: dc.w $ffff
pub fn i_c01104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC01106;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C01106: dc.w $ffff
pub fn i_c01106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC01108;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C01108: dc.w $ffff
pub fn i_c01108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0110A;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0110A: dc.w $ffff
pub fn i_c0110a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0110C;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0110C: dc.w $ffff
pub fn i_c0110c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0110E;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0206E: btst.l d0, d1
pub fn i_c0206e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02070;
    cpu.ir = 0x101;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C02070: btst.l d0, d1
pub fn i_c02070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02072;
    cpu.ir = 0x101;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C02072: btst.l d0, d1
pub fn i_c02072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02074;
    cpu.ir = 0x101;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C02074: btst.l d0, d1
pub fn i_c02074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02076;
    cpu.ir = 0x101;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C02076: btst.l d0, d1
pub fn i_c02076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02078;
    cpu.ir = 0x101;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C02078: btst.l d0, d1
pub fn i_c02078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0207A;
    cpu.ir = 0x101;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C0207A: btst.l d0, d1
pub fn i_c0207a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0207C;
    cpu.ir = 0x101;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C0207C: btst.l d0, d1
pub fn i_c0207c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0207E;
    cpu.ir = 0x101;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C0207E: btst.l d0, d1
pub fn i_c0207e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02080;
    cpu.ir = 0x101;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C02080: negx.b $2018(a5)
pub fn i_c02080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02084;
    cpu.ir = 0x402D;
    let ad1 = cpu.a[5].wrapping_add(0x2018);
    let v2 = bus.read8(ad1) as u32;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r3 = 0u32.wrapping_sub(v2).wrapping_sub(x) & 0xFF;
    let bw = v2 != 0 || x != 0; let vf = (v2 & 0x80) != 0 && (r3 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r3 & 0x80 != 0); if r3 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C02084: move.b (a0), -(a1)
pub fn i_c02084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02086;
    cpu.ir = 0x1310;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[1].wrapping_sub(1); cpu.a[1] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C02086: movep.w $a09(a3), d6
pub fn i_c02086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0208A;
    cpu.ir = 0xD0B;
    let a = cpu.a[3].wrapping_add(0xA09);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C0208A: ?
pub fn i_c0208a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0208E;
    cpu.ir = 0x807;
    let mk: u32 = 1u32 << ((6) % 32);
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C0208E: addi.b #$5, d5
pub fn i_c0208e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02092;
    cpu.ir = 0x605;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x5, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C02092: btst.l d2, d4
pub fn i_c02092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02094;
    cpu.ir = 0x504;
    let mk: u32 = 1u32 << ((cpu.d[2]) % 32);
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C02094: subi.b #$4, d4
pub fn i_c02094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02098;
    cpu.ir = 0x404;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x4, Size::Byte, true);
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C02098: btst.l d1, d3
pub fn i_c02098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0209A;
    cpu.ir = 0x303;
    let mk: u32 = 1u32 << ((cpu.d[1]) % 32);
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C0209A: btst.l d1, d3
pub fn i_c0209a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0209C;
    cpu.ir = 0x303;
    let mk: u32 = 1u32 << ((cpu.d[1]) % 32);
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C0209C: btst.l d1, d3
pub fn i_c0209c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0209E;
    cpu.ir = 0x303;
    let mk: u32 = 1u32 << ((cpu.d[1]) % 32);
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C0209E: btst.l d1, d3
pub fn i_c0209e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC020A0;
    cpu.ir = 0x303;
    let mk: u32 = 1u32 << ((cpu.d[1]) % 32);
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C020A0: btst.l d1, d2
pub fn i_c020a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC020A2;
    cpu.ir = 0x302;
    let mk: u32 = 1u32 << ((cpu.d[1]) % 32);
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C020A2: andi.b #$2, d2
pub fn i_c020a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC020A6;
    cpu.ir = 0x202;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 & 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C020A6: andi.b #$2, d2
pub fn i_c020a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC020AA;
    cpu.ir = 0x202;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 & 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C020AA: andi.b #$2, d2
pub fn i_c020aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC020AE;
    cpu.ir = 0x202;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 & 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C020AE: andi.b #$2, d2
pub fn i_c020ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC020B2;
    cpu.ir = 0x202;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 & 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C020B2: andi.b #$2, d2
pub fn i_c020b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC020B6;
    cpu.ir = 0x202;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 & 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C020B6: andi.b #$1, d1
pub fn i_c020b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC020BA;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C020BA: btst.l d0, d1
pub fn i_c020ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC020BC;
    cpu.ir = 0x101;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C020BC: btst.l d0, d1
pub fn i_c020bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC020BE;
    cpu.ir = 0x101;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C020BE: btst.l d0, d1
pub fn i_c020be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC020C0;
    cpu.ir = 0x101;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C020C0: negx.b $20(a3, d2.l)
pub fn i_c020c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC020C4;
    cpu.ir = 0x4033;
    let ad1 = cpu.a[3].wrapping_add(0x20).wrapping_add(cpu.d[2]);
    let v2 = bus.read8(ad1) as u32;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r3 = 0u32.wrapping_sub(v2).wrapping_sub(x) & 0xFF;
    let bw = v2 != 0 || x != 0; let vf = (v2 & 0x80) != 0 && (r3 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r3 & 0x80 != 0); if r3 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    bus.write8(ad1, r3 as u8);
    14
}
/// $C020C4: move.b (a6), d5
pub fn i_c020c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC020C6;
    cpu.ir = 0x1A16;
    let v2 = bus.read8(cpu.a[6]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C020C6: move.b (a0), -(a1)
pub fn i_c020c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC020C8;
    cpu.ir = 0x1310;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[1].wrapping_sub(1); cpu.a[1] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C020C8: movep.w $c0b(a5), d7
pub fn i_c020c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC020CC;
    cpu.ir = 0xF0D;
    let a = cpu.a[5].wrapping_add(0xC0B);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C020CC: ?
pub fn i_c020cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC020D0;
    cpu.ir = 0xA09;
    let v1 = cpu.a[1] & 0xFF;
    let r2 = (v1 ^ 0x8) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.a[1] = r2;
    8
}
/// $C020D0: ?
pub fn i_c020d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC020D4;
    cpu.ir = 0x807;
    let mk: u32 = 1u32 << ((6) % 32);
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C020D4: addi.b #$5, d6
pub fn i_c020d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC020D8;
    cpu.ir = 0x606;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x5, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C020D8: btst.l d2, d5
pub fn i_c020d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC020DA;
    cpu.ir = 0x505;
    let mk: u32 = 1u32 << ((cpu.d[2]) % 32);
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C020DA: btst.l d2, d5
pub fn i_c020da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC020DC;
    cpu.ir = 0x505;
    let mk: u32 = 1u32 << ((cpu.d[2]) % 32);
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C020DC: subi.b #$4, d4
pub fn i_c020dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC020E0;
    cpu.ir = 0x404;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x4, Size::Byte, true);
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C020E0: subi.b #$3, d4
pub fn i_c020e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC020E4;
    cpu.ir = 0x404;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x3, Size::Byte, true);
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C020E4: btst.l d1, d3
pub fn i_c020e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC020E6;
    cpu.ir = 0x303;
    let mk: u32 = 1u32 << ((cpu.d[1]) % 32);
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C020E6: btst.l d1, d3
pub fn i_c020e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC020E8;
    cpu.ir = 0x303;
    let mk: u32 = 1u32 << ((cpu.d[1]) % 32);
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C020E8: btst.l d1, d3
pub fn i_c020e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC020EA;
    cpu.ir = 0x303;
    let mk: u32 = 1u32 << ((cpu.d[1]) % 32);
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C020EA: btst.l d1, d3
pub fn i_c020ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC020EC;
    cpu.ir = 0x303;
    let mk: u32 = 1u32 << ((cpu.d[1]) % 32);
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C020EC: btst.l d1, d3
pub fn i_c020ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC020EE;
    cpu.ir = 0x303;
    let mk: u32 = 1u32 << ((cpu.d[1]) % 32);
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C020EE: btst.l d1, d3
pub fn i_c020ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC020F0;
    cpu.ir = 0x303;
    let mk: u32 = 1u32 << ((cpu.d[1]) % 32);
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C020F0: btst.l d1, d2
pub fn i_c020f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC020F2;
    cpu.ir = 0x302;
    let mk: u32 = 1u32 << ((cpu.d[1]) % 32);
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C020F2: andi.b #$2, d2
pub fn i_c020f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC020F6;
    cpu.ir = 0x202;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 & 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C020F6: andi.b #$2, d2
pub fn i_c020f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC020FA;
    cpu.ir = 0x202;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 & 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C020FA: andi.b #$2, d2
pub fn i_c020fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC020FE;
    cpu.ir = 0x202;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 & 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C020FE: andi.b #$36, d2
pub fn i_c020fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02102;
    cpu.ir = 0x202;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 & 0x36) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C02102: move.l -(a6), -(a6)
pub fn i_c02102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02104;
    cpu.ir = 0x2D26;
    let ad2 = cpu.a[6].wrapping_sub(4); cpu.a[6] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[6];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[6] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C02104: move.l (a3)+, d0
pub fn i_c02104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02106;
    cpu.ir = 0x201B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C02106: move.b (a5), d4
pub fn i_c02106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02108;
    cpu.ir = 0x1815;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C02108: move.b (a1), -(a1)
pub fn i_c02108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0210A;
    cpu.ir = 0x1311;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[1].wrapping_sub(1); cpu.a[1] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C0210A: ?
pub fn i_c0210a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0210C;
    cpu.ir = 0x100E;
    let v1 = cpu.a[6] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C0210C: movep.w $b0b(a4), d6
pub fn i_c0210c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02110;
    cpu.ir = 0xD0C;
    let a = cpu.a[4].wrapping_add(0xB0B);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C02110: ?
pub fn i_c02110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02114;
    cpu.ir = 0xA09;
    let v1 = cpu.a[1] & 0xFF;
    let r2 = (v1 ^ 0x8) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.a[1] = r2;
    8
}
/// $C02114: ?
pub fn i_c02114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02118;
    cpu.ir = 0x808;
    let mk: u32 = 1u32 << ((7) % 32);
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C02118: btst.l d3, d6
pub fn i_c02118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0211A;
    cpu.ir = 0x706;
    let mk: u32 = 1u32 << ((cpu.d[3]) % 32);
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C0211A: addi.b #$6, d6
pub fn i_c0211a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0211E;
    cpu.ir = 0x606;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x6, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C0211E: btst.l d2, d5
pub fn i_c0211e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02120;
    cpu.ir = 0x505;
    let mk: u32 = 1u32 << ((cpu.d[2]) % 32);
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C02120: btst.l d2, d5
pub fn i_c02120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02122;
    cpu.ir = 0x505;
    let mk: u32 = 1u32 << ((cpu.d[2]) % 32);
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C02122: btst.l d2, d5
pub fn i_c02122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02124;
    cpu.ir = 0x505;
    let mk: u32 = 1u32 << ((cpu.d[2]) % 32);
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C02124: btst.l d2, d4
pub fn i_c02124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02126;
    cpu.ir = 0x504;
    let mk: u32 = 1u32 << ((cpu.d[2]) % 32);
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C02126: subi.b #$4, d4
pub fn i_c02126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0212A;
    cpu.ir = 0x404;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x4, Size::Byte, true);
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C0212A: subi.b #$4, d4
pub fn i_c0212a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0212E;
    cpu.ir = 0x404;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x4, Size::Byte, true);
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C0212E: subi.b #$3, d3
pub fn i_c0212e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02132;
    cpu.ir = 0x403;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x3, Size::Byte, true);
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C02132: btst.l d1, d3
pub fn i_c02132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02134;
    cpu.ir = 0x303;
    let mk: u32 = 1u32 << ((cpu.d[1]) % 32);
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C02134: btst.l d1, d3
pub fn i_c02134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02136;
    cpu.ir = 0x303;
    let mk: u32 = 1u32 << ((cpu.d[1]) % 32);
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C02136: btst.l d1, d3
pub fn i_c02136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02138;
    cpu.ir = 0x303;
    let mk: u32 = 1u32 << ((cpu.d[1]) % 32);
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C02138: btst.l d1, d3
pub fn i_c02138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0213A;
    cpu.ir = 0x303;
    let mk: u32 = 1u32 << ((cpu.d[1]) % 32);
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C0213A: btst.l d1, d3
pub fn i_c0213a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0213C;
    cpu.ir = 0x303;
    let mk: u32 = 1u32 << ((cpu.d[1]) % 32);
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C0213C: btst.l d1, d3
pub fn i_c0213c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0213E;
    cpu.ir = 0x303;
    let mk: u32 = 1u32 << ((cpu.d[1]) % 32);
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C0213E: btst.l d1, d3
pub fn i_c0213e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02140;
    cpu.ir = 0x303;
    let mk: u32 = 1u32 << ((cpu.d[1]) % 32);
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C02140: negx.b $302a.w
pub fn i_c02140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02144;
    cpu.ir = 0x4038;
    let v1 = bus.read8(0x302A) as u32;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x80) != 0 && (r2 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x80 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    bus.write8(0x302A, r2 as u8);
    12
}
/// $C02144: move.l -(a0), -(a2)
pub fn i_c02144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02146;
    cpu.ir = 0x2520;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[2];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[2] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C02146: move.b (a1)+, d6
pub fn i_c02146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02148;
    cpu.ir = 0x1C19;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02148: move.b (a5), -(a3)
pub fn i_c02148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0214A;
    cpu.ir = 0x1715;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[3].wrapping_sub(1); cpu.a[3] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C0214A: move.b (a1), -(a1)
pub fn i_c0214a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0214C;
    cpu.ir = 0x1311;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[1].wrapping_sub(1); cpu.a[1] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C0214C: ?
pub fn i_c0214c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0214E;
    cpu.ir = 0x100F;
    let v1 = cpu.a[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C0214E: ?
pub fn i_c0214e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02152;
    cpu.ir = 0xE0D;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C02152: movep.w $a0a(a2), d5
pub fn i_c02152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02156;
    cpu.ir = 0xB0A;
    let a = cpu.a[2].wrapping_add(0xA0A);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C02156: movep.w $808(a1), d4
pub fn i_c02156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0215A;
    cpu.ir = 0x909;
    let a = cpu.a[1].wrapping_add(0x808);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[4] = (cpu.d[4] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C0215A: ?
pub fn i_c0215a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0215E;
    cpu.ir = 0x807;
    let mk: u32 = 1u32 << ((7) % 32);
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C0215E: btst.l d3, d7
pub fn i_c0215e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02160;
    cpu.ir = 0x707;
    let mk: u32 = 1u32 << ((cpu.d[3]) % 32);
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C02160: addi.b #$6, d6
pub fn i_c02160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02164;
    cpu.ir = 0x606;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x6, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C02164: addi.b #$5, d5
pub fn i_c02164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02168;
    cpu.ir = 0x605;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x5, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C02168: btst.l d2, d5
pub fn i_c02168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0216A;
    cpu.ir = 0x505;
    let mk: u32 = 1u32 << ((cpu.d[2]) % 32);
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C0216A: btst.l d2, d5
pub fn i_c0216a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0216C;
    cpu.ir = 0x505;
    let mk: u32 = 1u32 << ((cpu.d[2]) % 32);
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C0216C: btst.l d2, d5
pub fn i_c0216c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0216E;
    cpu.ir = 0x505;
    let mk: u32 = 1u32 << ((cpu.d[2]) % 32);
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C0216E: subi.b #$4, d4
pub fn i_c0216e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02172;
    cpu.ir = 0x404;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x4, Size::Byte, true);
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C02172: subi.b #$4, d4
pub fn i_c02172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02176;
    cpu.ir = 0x404;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x4, Size::Byte, true);
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C02176: subi.b #$4, d4
pub fn i_c02176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0217A;
    cpu.ir = 0x404;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x4, Size::Byte, true);
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C0217A: subi.b #$3, d3
pub fn i_c0217a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0217E;
    cpu.ir = 0x403;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x3, Size::Byte, true);
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C0217E: btst.l d1, d3
pub fn i_c0217e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02180;
    cpu.ir = 0x303;
    let mk: u32 = 1u32 << ((cpu.d[1]) % 32);
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C02180: negx.b $332d2824.l
pub fn i_c02180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02186;
    cpu.ir = 0x4039;
    let v1 = bus.read8(0x332D2824) as u32;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x80) != 0 && (r2 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x80 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    bus.write8(0x332D2824, r2 as u8);
    16
}
/// $C02186: move.l (a5)+, d0
pub fn i_c02186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02188;
    cpu.ir = 0x201D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C02188: move.b (a0)+, d5
pub fn i_c02188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0218A;
    cpu.ir = 0x1A18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C0218A: move.b (a4), d3
pub fn i_c0218a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0218C;
    cpu.ir = 0x1614;
    let v2 = bus.read8(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C0218C: move.b (a2), -(a1)
pub fn i_c0218c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0218E;
    cpu.ir = 0x1312;
    let v2 = bus.read8(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[1].wrapping_sub(1); cpu.a[1] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C0218E: move.b (a0), d0
pub fn i_c0218e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02190;
    cpu.ir = 0x1010;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C02190: movep.w $d0c(a6), d7
pub fn i_c02190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02194;
    cpu.ir = 0xF0E;
    let a = cpu.a[6].wrapping_add(0xD0C);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C02194: ?
pub fn i_c02194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02198;
    cpu.ir = 0xC0B;
    let v1 = cpu.a[3] & 0xFF;
    sub_with_flags(cpu, v1, 0xA, Size::Byte, false);
    8
}
/// $C02198: ?
pub fn i_c02198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0219C;
    cpu.ir = 0xA0A;
    let v1 = cpu.a[2] & 0xFF;
    let r2 = (v1 ^ 0x9) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.a[2] = r2;
    8
}
/// $C0219C: movep.w $808(a0), d4
pub fn i_c0219c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC021A0;
    cpu.ir = 0x908;
    let a = cpu.a[0].wrapping_add(0x808);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[4] = (cpu.d[4] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C021A0: ?
pub fn i_c021a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC021A4;
    cpu.ir = 0x807;
    let mk: u32 = 1u32 << ((7) % 32);
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C021A4: btst.l d3, d7
pub fn i_c021a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC021A6;
    cpu.ir = 0x707;
    let mk: u32 = 1u32 << ((cpu.d[3]) % 32);
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C021A6: addi.b #$6, d6
pub fn i_c021a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC021AA;
    cpu.ir = 0x606;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x6, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C021AA: addi.b #$5, d6
pub fn i_c021aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC021AE;
    cpu.ir = 0x606;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x5, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C021AE: btst.l d2, d5
pub fn i_c021ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC021B0;
    cpu.ir = 0x505;
    let mk: u32 = 1u32 << ((cpu.d[2]) % 32);
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C021B0: btst.l d2, d5
pub fn i_c021b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC021B2;
    cpu.ir = 0x505;
    let mk: u32 = 1u32 << ((cpu.d[2]) % 32);
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C021B2: btst.l d2, d5
pub fn i_c021b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC021B4;
    cpu.ir = 0x505;
    let mk: u32 = 1u32 << ((cpu.d[2]) % 32);
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C021B4: btst.l d2, d5
pub fn i_c021b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC021B6;
    cpu.ir = 0x505;
    let mk: u32 = 1u32 << ((cpu.d[2]) % 32);
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C021B6: btst.l d2, d4
pub fn i_c021b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC021B8;
    cpu.ir = 0x504;
    let mk: u32 = 1u32 << ((cpu.d[2]) % 32);
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C021B8: subi.b #$4, d4
pub fn i_c021b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC021BC;
    cpu.ir = 0x404;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x4, Size::Byte, true);
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C021BC: subi.b #$4, d4
pub fn i_c021bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC021C0;
    cpu.ir = 0x404;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x4, Size::Byte, true);
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C021C0: ?
pub fn i_c021c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC021C4;
    cpu.ir = 0x403A;
    let v2 = bus.read8(0xC056F2) as u32;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r3 = 0u32.wrapping_sub(v2).wrapping_sub(x) & 0xFF;
    let bw = v2 != 0 || x != 0; let vf = (v2 & 0x80) != 0 && (r3 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r3 & 0x80 != 0); if r3 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    12
}
/// $C021C4: move.l -(a7), -(a5)
pub fn i_c021c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC021C6;
    cpu.ir = 0x2B27;
    let ad2 = cpu.a[7].wrapping_sub(4); cpu.a[7] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[5];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[5] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C021C6: move.l -(a0), -(a1)
pub fn i_c021c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC021C8;
    cpu.ir = 0x2320;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[1];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[1] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C021C8: move.b (a3)+, -(a6)
pub fn i_c021c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC021CA;
    cpu.ir = 0x1D1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C021CA: move.b (a7), -(a4)
pub fn i_c021ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC021CC;
    cpu.ir = 0x1917;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C021CC: move.b (a4), d3
pub fn i_c021cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC021CE;
    cpu.ir = 0x1614;
    let v2 = bus.read8(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C021CE: move.b (a2), -(a1)
pub fn i_c021ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC021D0;
    cpu.ir = 0x1312;
    let v2 = bus.read8(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[1].wrapping_sub(1); cpu.a[1] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C021D0: move.b (a0), -(a0)
pub fn i_c021d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC021D2;
    cpu.ir = 0x1110;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C021D2: movep.w $e0d(a6), d7
pub fn i_c021d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC021D6;
    cpu.ir = 0xF0E;
    let a = cpu.a[6].wrapping_add(0xE0D);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C021D6: movep.w $c0b(a4), d6
pub fn i_c021d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC021DA;
    cpu.ir = 0xD0C;
    let a = cpu.a[4].wrapping_add(0xC0B);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C021DA: movep.w $a0a(a2), d5
pub fn i_c021da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC021DE;
    cpu.ir = 0xB0A;
    let a = cpu.a[2].wrapping_add(0xA0A);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C021DE: movep.w $909(a1), d4
pub fn i_c021de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC021E2;
    cpu.ir = 0x909;
    let a = cpu.a[1].wrapping_add(0x909);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[4] = (cpu.d[4] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C021E2: ?
pub fn i_c021e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC021E6;
    cpu.ir = 0x808;
    let mk: u32 = 1u32 << ((8) % 32);
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C021E6: btst.l d3, d7
pub fn i_c021e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC021E8;
    cpu.ir = 0x707;
    let mk: u32 = 1u32 << ((cpu.d[3]) % 32);
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C021E8: btst.l d3, d7
pub fn i_c021e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC021EA;
    cpu.ir = 0x707;
    let mk: u32 = 1u32 << ((cpu.d[3]) % 32);
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C021EA: btst.l d3, d7
pub fn i_c021ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC021EC;
    cpu.ir = 0x707;
    let mk: u32 = 1u32 << ((cpu.d[3]) % 32);
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C021EC: addi.b #$6, d6
pub fn i_c021ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC021F0;
    cpu.ir = 0x606;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x6, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C021F0: addi.b #$6, d6
pub fn i_c021f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC021F4;
    cpu.ir = 0x606;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x6, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C021F4: btst.l d2, d5
pub fn i_c021f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC021F6;
    cpu.ir = 0x505;
    let mk: u32 = 1u32 << ((cpu.d[2]) % 32);
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C021F6: btst.l d2, d5
pub fn i_c021f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC021F8;
    cpu.ir = 0x505;
    let mk: u32 = 1u32 << ((cpu.d[2]) % 32);
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C021F8: btst.l d2, d5
pub fn i_c021f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC021FA;
    cpu.ir = 0x505;
    let mk: u32 = 1u32 << ((cpu.d[2]) % 32);
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C021FA: btst.l d2, d5
pub fn i_c021fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC021FC;
    cpu.ir = 0x505;
    let mk: u32 = 1u32 << ((cpu.d[2]) % 32);
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C021FC: btst.l d2, d5
pub fn i_c021fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC021FE;
    cpu.ir = 0x505;
    let mk: u32 = 1u32 << ((cpu.d[2]) % 32);
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C021FE: btst.l d2, d5
pub fn i_c021fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02200;
    cpu.ir = 0x505;
    let mk: u32 = 1u32 << ((cpu.d[2]) % 32);
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C02200: ?
pub fn i_c02200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02204;
    cpu.ir = 0x403B;
    let ad2 = 0xC02233_u32.wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r4 = 0u32.wrapping_sub(v3).wrapping_sub(x) & 0xFF;
    let bw = v3 != 0 || x != 0; let vf = (v3 & 0x80) != 0 && (r4 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r4 & 0x80 != 0); if r4 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    14
}
/// $C02204: move.l $2623(a1), -(a6)
pub fn i_c02204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02208;
    cpu.ir = 0x2D29;
    let ad2 = cpu.a[1].wrapping_add(0x2623);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[6];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[6] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C02208: move.l (a6)+, d0
pub fn i_c02208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0220A;
    cpu.ir = 0x201E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C0220A: move.b (a2)+, -(a5)
pub fn i_c0220a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0220C;
    cpu.ir = 0x1B1A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C0220C: move.b (a6), d4
pub fn i_c0220c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0220E;
    cpu.ir = 0x1816;
    let v2 = bus.read8(cpu.a[6]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C0220E: move.b (a4), -(a2)
pub fn i_c0220e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02210;
    cpu.ir = 0x1514;
    let v2 = bus.read8(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[2].wrapping_sub(1); cpu.a[2] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C02210: move.b (a2), -(a1)
pub fn i_c02210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02212;
    cpu.ir = 0x1312;
    let v2 = bus.read8(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[1].wrapping_sub(1); cpu.a[1] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C02212: move.b (a0), -(a0)
pub fn i_c02212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02214;
    cpu.ir = 0x1110;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C02214: ?
pub fn i_c02214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02216;
    cpu.ir = 0x100F;
    let v1 = cpu.a[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C02216: ?
pub fn i_c02216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0221A;
    cpu.ir = 0xE0E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C0221A: ?
pub fn i_c0221a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0221E;
    cpu.ir = 0xC0C;
    let v1 = cpu.a[4] & 0xFF;
    sub_with_flags(cpu, v1, 0xB, Size::Byte, false);
    8
}
/// $C0221E: movep.w $a0a(a2), d5
pub fn i_c0221e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02222;
    cpu.ir = 0xB0A;
    let a = cpu.a[2].wrapping_add(0xA0A);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C02222: movep.w $909(a1), d4
pub fn i_c02222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02226;
    cpu.ir = 0x909;
    let a = cpu.a[1].wrapping_add(0x909);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[4] = (cpu.d[4] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C02226: ?
pub fn i_c02226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0222A;
    cpu.ir = 0x808;
    let mk: u32 = 1u32 << ((8) % 32);
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C0222A: ?
pub fn i_c0222a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0222E;
    cpu.ir = 0x807;
    let mk: u32 = 1u32 << ((7) % 32);
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C0222E: btst.l d3, d7
pub fn i_c0222e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02230;
    cpu.ir = 0x707;
    let mk: u32 = 1u32 << ((cpu.d[3]) % 32);
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C02230: btst.l d3, d7
pub fn i_c02230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02232;
    cpu.ir = 0x707;
    let mk: u32 = 1u32 << ((cpu.d[3]) % 32);
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C02232: addi.b #$6, d6
pub fn i_c02232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02236;
    cpu.ir = 0x606;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x6, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C02236: addi.b #$6, d6
pub fn i_c02236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0223A;
    cpu.ir = 0x606;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x6, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C0223A: addi.b #$5, d5
pub fn i_c0223a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0223E;
    cpu.ir = 0x605;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x5, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C0223E: btst.l d2, d5
pub fn i_c0223e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02240;
    cpu.ir = 0x505;
    let mk: u32 = 1u32 << ((cpu.d[2]) % 32);
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C02240: ?
pub fn i_c02240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02244;
    cpu.ir = 0x403B;
    let ad2 = 0xC02275_u32.wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r4 = 0u32.wrapping_sub(v3).wrapping_sub(x) & 0xFF;
    let bw = v3 != 0 || x != 0; let vf = (v3 & 0x80) != 0 && (r4 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r4 & 0x80 != 0); if r4 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    14
}
/// $C02244: move.l $2825(a3), -(a7)
pub fn i_c02244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02248;
    cpu.ir = 0x2F2B;
    let ad2 = cpu.a[3].wrapping_add(0x2825);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[7];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[7] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C02248: move.l -(a0), d1
pub fn i_c02248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0224A;
    cpu.ir = 0x2220;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C0224A: move.b (a4)+, d7
pub fn i_c0224a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0224C;
    cpu.ir = 0x1E1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C0224C: move.b (a1)+, d5
pub fn i_c0224c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0224E;
    cpu.ir = 0x1A19;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C0224E: move.b (a6), -(a3)
pub fn i_c0224e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02250;
    cpu.ir = 0x1716;
    let v2 = bus.read8(cpu.a[6]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[3].wrapping_sub(1); cpu.a[3] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C02250: move.b (a4), -(a2)
pub fn i_c02250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02252;
    cpu.ir = 0x1514;
    let v2 = bus.read8(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[2].wrapping_sub(1); cpu.a[2] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C02252: move.b (a2), -(a1)
pub fn i_c02252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02254;
    cpu.ir = 0x1312;
    let v2 = bus.read8(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[1].wrapping_sub(1); cpu.a[1] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C02254: move.b (a0), -(a0)
pub fn i_c02254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02256;
    cpu.ir = 0x1110;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C02256: ?
pub fn i_c02256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02258;
    cpu.ir = 0x100F;
    let v1 = cpu.a[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C02258: movep.w $e0d(a6), d7
pub fn i_c02258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0225C;
    cpu.ir = 0xF0E;
    let a = cpu.a[6].wrapping_add(0xE0D);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C0225C: movep.w $c0c(a4), d6
pub fn i_c0225c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02260;
    cpu.ir = 0xD0C;
    let a = cpu.a[4].wrapping_add(0xC0C);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C02260: movep.w $b0a(a3), d5
pub fn i_c02260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02264;
    cpu.ir = 0xB0B;
    let a = cpu.a[3].wrapping_add(0xB0A);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C02264: ?
pub fn i_c02264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02268;
    cpu.ir = 0xA0A;
    let v1 = cpu.a[2] & 0xFF;
    let r2 = (v1 ^ 0x9) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.a[2] = r2;
    8
}
/// $C02268: movep.w $908(a1), d4
pub fn i_c02268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0226C;
    cpu.ir = 0x909;
    let a = cpu.a[1].wrapping_add(0x908);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[4] = (cpu.d[4] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C0226C: ?
pub fn i_c0226c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02270;
    cpu.ir = 0x808;
    let mk: u32 = 1u32 << ((8) % 32);
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C02270: ?
pub fn i_c02270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02274;
    cpu.ir = 0x807;
    let mk: u32 = 1u32 << ((7) % 32);
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C02274: btst.l d3, d7
pub fn i_c02274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02276;
    cpu.ir = 0x707;
    let mk: u32 = 1u32 << ((cpu.d[3]) % 32);
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C02276: btst.l d3, d7
pub fn i_c02276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02278;
    cpu.ir = 0x707;
    let mk: u32 = 1u32 << ((cpu.d[3]) % 32);
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C02278: addi.b #$6, d6
pub fn i_c02278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0227C;
    cpu.ir = 0x606;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x6, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C0227C: addi.b #$6, d6
pub fn i_c0227c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02280;
    cpu.ir = 0x606;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x6, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C02280: ?
pub fn i_c02280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02284;
    cpu.ir = 0x403C;
    let v1: u32 = 0x34;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x80) != 0 && (r2 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x80 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    4
}
/// $C02284: move.w $2a27(a5), d0
pub fn i_c02284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02288;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x2A27);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C02288: move.l -(a2), -(a2)
pub fn i_c02288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0228A;
    cpu.ir = 0x2522;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[2];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[2] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C0228A: move.l (a6)+, d0
pub fn i_c0228a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0228C;
    cpu.ir = 0x201E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C0228C: move.b (a3)+, d6
pub fn i_c0228c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0228E;
    cpu.ir = 0x1C1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C0228E: move.b (a0)+, -(a4)
pub fn i_c0228e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02290;
    cpu.ir = 0x1918;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02290: move.b (a6), -(a3)
pub fn i_c02290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02292;
    cpu.ir = 0x1716;
    let v2 = bus.read8(cpu.a[6]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[3].wrapping_sub(1); cpu.a[3] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C02292: move.b (a4), -(a2)
pub fn i_c02292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02294;
    cpu.ir = 0x1514;
    let v2 = bus.read8(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[2].wrapping_sub(1); cpu.a[2] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C02294: move.b (a2), -(a1)
pub fn i_c02294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02296;
    cpu.ir = 0x1312;
    let v2 = bus.read8(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[1].wrapping_sub(1); cpu.a[1] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C02296: move.b (a1), -(a0)
pub fn i_c02296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02298;
    cpu.ir = 0x1111;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C02298: move.b (a0), d0
pub fn i_c02298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0229A;
    cpu.ir = 0x1010;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C0229A: movep.w $e0e(a6), d7
pub fn i_c0229a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0229E;
    cpu.ir = 0xF0E;
    let a = cpu.a[6].wrapping_add(0xE0E);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C0229E: movep.w $c0c(a5), d6
pub fn i_c0229e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC022A2;
    cpu.ir = 0xD0D;
    let a = cpu.a[5].wrapping_add(0xC0C);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C022A2: ?
pub fn i_c022a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC022A6;
    cpu.ir = 0xC0B;
    let v1 = cpu.a[3] & 0xFF;
    sub_with_flags(cpu, v1, 0xB, Size::Byte, false);
    8
}
/// $C022A6: ?
pub fn i_c022a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC022AA;
    cpu.ir = 0xA0A;
    let v1 = cpu.a[2] & 0xFF;
    let r2 = (v1 ^ 0xA) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.a[2] = r2;
    8
}
/// $C022AA: ?
pub fn i_c022aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC022AE;
    cpu.ir = 0xA09;
    let v1 = cpu.a[1] & 0xFF;
    let r2 = (v1 ^ 0x9) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.a[1] = r2;
    8
}
/// $C022AE: movep.w $808(a1), d4
pub fn i_c022ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC022B2;
    cpu.ir = 0x909;
    let a = cpu.a[1].wrapping_add(0x808);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[4] = (cpu.d[4] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C022B2: ?
pub fn i_c022b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC022B6;
    cpu.ir = 0x808;
    let mk: u32 = 1u32 << ((8) % 32);
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C022B6: btst.l d3, d7
pub fn i_c022b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC022B8;
    cpu.ir = 0x707;
    let mk: u32 = 1u32 << ((cpu.d[3]) % 32);
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C022B8: btst.l d3, d7
pub fn i_c022b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC022BA;
    cpu.ir = 0x707;
    let mk: u32 = 1u32 << ((cpu.d[3]) % 32);
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C022BA: btst.l d3, d7
pub fn i_c022ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC022BC;
    cpu.ir = 0x707;
    let mk: u32 = 1u32 << ((cpu.d[3]) % 32);
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C022BC: btst.l d3, d7
pub fn i_c022bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC022BE;
    cpu.ir = 0x707;
    let mk: u32 = 1u32 << ((cpu.d[3]) % 32);
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C022BE: btst.l d3, d6
pub fn i_c022be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC022C0;
    cpu.ir = 0x706;
    let mk: u32 = 1u32 << ((cpu.d[3]) % 32);
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C022C0: ?
pub fn i_c022c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC022C4;
    cpu.ir = 0x403C;
    let v1: u32 = 0x35;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x80) != 0 && (r2 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x80 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    4
}
/// $C022C4: move.w $2c29(a7), d1
pub fn i_c022c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC022C8;
    cpu.ir = 0x322F;
    let ad2 = cpu.a[7].wrapping_add(0x2C29);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C022C8: move.l -(a4), d3
pub fn i_c022c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC022CA;
    cpu.ir = 0x2624;
    let ad2 = cpu.a[4].wrapping_sub(4); cpu.a[4] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C022CA: move.l -(a0), d1
pub fn i_c022ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC022CC;
    cpu.ir = 0x2220;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C022CC: move.b (a5)+, d7
pub fn i_c022cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC022CE;
    cpu.ir = 0x1E1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C022CE: move.b (a2)+, -(a5)
pub fn i_c022ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC022D0;
    cpu.ir = 0x1B1A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C022D0: move.b (a7), -(a4)
pub fn i_c022d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC022D2;
    cpu.ir = 0x1917;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C022D2: move.b (a5), d3
pub fn i_c022d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC022D4;
    cpu.ir = 0x1615;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C022D4: move.b (a4), d2
pub fn i_c022d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC022D6;
    cpu.ir = 0x1414;
    let v2 = bus.read8(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C022D6: move.b (a2), -(a1)
pub fn i_c022d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC022D8;
    cpu.ir = 0x1312;
    let v2 = bus.read8(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[1].wrapping_sub(1); cpu.a[1] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C022D8: move.b (a1), d1
pub fn i_c022d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC022DA;
    cpu.ir = 0x1211;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C022DA: move.b (a0), d0
pub fn i_c022da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC022DC;
    cpu.ir = 0x1010;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C022DC: movep.w $e0e(a7), d7
pub fn i_c022dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC022E0;
    cpu.ir = 0xF0F;
    let a = cpu.a[7].wrapping_add(0xE0E);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C022E0: movep.w $d0c(a5), d6
pub fn i_c022e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC022E4;
    cpu.ir = 0xD0D;
    let a = cpu.a[5].wrapping_add(0xD0C);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C022E4: ?
pub fn i_c022e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC022E8;
    cpu.ir = 0xC0C;
    let v1 = cpu.a[4] & 0xFF;
    sub_with_flags(cpu, v1, 0xB, Size::Byte, false);
    8
}
/// $C022E8: movep.w $a0a(a3), d5
pub fn i_c022e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC022EC;
    cpu.ir = 0xB0B;
    let a = cpu.a[3].wrapping_add(0xA0A);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C022EC: ?
pub fn i_c022ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC022F0;
    cpu.ir = 0xA0A;
    let v1 = cpu.a[2] & 0xFF;
    let r2 = (v1 ^ 0x9) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.a[2] = r2;
    8
}
/// $C022F0: movep.w $909(a1), d4
pub fn i_c022f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC022F4;
    cpu.ir = 0x909;
    let a = cpu.a[1].wrapping_add(0x909);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[4] = (cpu.d[4] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C022F4: ?
pub fn i_c022f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC022F8;
    cpu.ir = 0x808;
    let mk: u32 = 1u32 << ((8) % 32);
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C022F8: ?
pub fn i_c022f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC022FC;
    cpu.ir = 0x808;
    let mk: u32 = 1u32 << ((8) % 32);
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C022FC: btst.l d3, d7
pub fn i_c022fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC022FE;
    cpu.ir = 0x707;
    let mk: u32 = 1u32 << ((cpu.d[3]) % 32);
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C022FE: btst.l d3, d7
pub fn i_c022fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02300;
    cpu.ir = 0x707;
    let mk: u32 = 1u32 << ((cpu.d[3]) % 32);
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C02300: ?
pub fn i_c02300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02302;
    cpu.ir = 0x403D;
    let v1: u32 = 0x0;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x80) != 0 && (r2 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x80 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    4
}
/// $C02302: move.w $2d2a2826(a6, d3.w * 2), -(a4)
pub fn i_c02302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02306;
    cpu.ir = 0x3936;
    let ad2 = cpu.a[6].wrapping_add(0x30).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(2); cpu.a[4] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C02306: move.l $2826(a2), -(a6)
pub fn i_c02306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0230A;
    cpu.ir = 0x2D2A;
    let ad2 = cpu.a[2].wrapping_add(0x2826);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[6];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[6] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C0230A: move.l -(a2), d2
pub fn i_c0230a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0230C;
    cpu.ir = 0x2422;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C0230C: move.l (a6)+, d0
pub fn i_c0230c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0230E;
    cpu.ir = 0x201E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C0230E: move.b (a3)+, -(a6)
pub fn i_c0230e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02310;
    cpu.ir = 0x1D1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02310: move.b (a1)+, d5
pub fn i_c02310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02312;
    cpu.ir = 0x1A19;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02312: move.b (a7), d4
pub fn i_c02312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02314;
    cpu.ir = 0x1817;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C02314: move.b (a5), d3
pub fn i_c02314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02316;
    cpu.ir = 0x1615;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C02316: move.b (a4), d2
pub fn i_c02316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02318;
    cpu.ir = 0x1414;
    let v2 = bus.read8(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C02318: move.b (a2), -(a1)
pub fn i_c02318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0231A;
    cpu.ir = 0x1312;
    let v2 = bus.read8(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[1].wrapping_sub(1); cpu.a[1] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C0231A: move.b (a1), d1
pub fn i_c0231a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0231C;
    cpu.ir = 0x1211;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C0231C: move.b (a0), d0
pub fn i_c0231c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0231E;
    cpu.ir = 0x1010;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C0231E: ?
pub fn i_c0231e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02320;
    cpu.ir = 0x100F;
    let v1 = cpu.a[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C02320: movep.w $e0d(a6), d7
pub fn i_c02320(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02324;
    cpu.ir = 0xF0E;
    let a = cpu.a[6].wrapping_add(0xE0D);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C02324: movep.w $c0c(a5), d6
pub fn i_c02324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02328;
    cpu.ir = 0xD0D;
    let a = cpu.a[5].wrapping_add(0xC0C);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C02328: ?
pub fn i_c02328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0232C;
    cpu.ir = 0xC0C;
    let v1 = cpu.a[4] & 0xFF;
    sub_with_flags(cpu, v1, 0xB, Size::Byte, false);
    8
}
/// $C0232C: movep.w $a0a(a3), d5
pub fn i_c0232c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02330;
    cpu.ir = 0xB0B;
    let a = cpu.a[3].wrapping_add(0xA0A);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C02330: ?
pub fn i_c02330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02334;
    cpu.ir = 0xA0A;
    let v1 = cpu.a[2] & 0xFF;
    let r2 = (v1 ^ 0x9) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.a[2] = r2;
    8
}
/// $C02334: movep.w $909(a1), d4
pub fn i_c02334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02338;
    cpu.ir = 0x909;
    let a = cpu.a[1].wrapping_add(0x909);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[4] = (cpu.d[4] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C02338: movep.w $808(a0), d4
pub fn i_c02338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0233C;
    cpu.ir = 0x908;
    let a = cpu.a[0].wrapping_add(0x808);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[4] = (cpu.d[4] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C0233C: ?
pub fn i_c0233c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02340;
    cpu.ir = 0x808;
    let mk: u32 = 1u32 << ((8) % 32);
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C02340: ?
pub fn i_c02340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02342;
    cpu.ir = 0x403D;
    let v1: u32 = 0x0;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x80) != 0 && (r2 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x80 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    4
}
/// $C02342: move.w $31(a7, d3.w), d5
pub fn i_c02342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02346;
    cpu.ir = 0x3A37;
    let ad2 = cpu.a[7].wrapping_add(0x31).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C02346: move.l $2a27(a4), d7
pub fn i_c02346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0234A;
    cpu.ir = 0x2E2C;
    let ad2 = cpu.a[4].wrapping_add(0x2A27);
    let v3 = bus.read32(ad2);
    cpu.d[7] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C0234A: move.l -(a3), -(a2)
pub fn i_c0234a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0234C;
    cpu.ir = 0x2523;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[2];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[2] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C0234C: move.l -(a0), d1
pub fn i_c0234c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0234E;
    cpu.ir = 0x2220;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C0234E: move.b (a5)+, d7
pub fn i_c0234e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02350;
    cpu.ir = 0x1E1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02350: move.b (a3)+, d6
pub fn i_c02350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02352;
    cpu.ir = 0x1C1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02352: move.b (a0)+, -(a4)
pub fn i_c02352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02354;
    cpu.ir = 0x1918;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02354: move.b (a7), -(a3)
pub fn i_c02354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02356;
    cpu.ir = 0x1717;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[3].wrapping_sub(1); cpu.a[3] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C02356: move.b (a5), d3
pub fn i_c02356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02358;
    cpu.ir = 0x1615;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C02358: move.b (a4), d2
pub fn i_c02358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0235A;
    cpu.ir = 0x1414;
    let v2 = bus.read8(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C0235A: move.b (a2), -(a1)
pub fn i_c0235a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0235C;
    cpu.ir = 0x1312;
    let v2 = bus.read8(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[1].wrapping_sub(1); cpu.a[1] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C0235C: move.b (a1), d1
pub fn i_c0235c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0235E;
    cpu.ir = 0x1211;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C0235E: move.b (a0), -(a0)
pub fn i_c0235e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02360;
    cpu.ir = 0x1110;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C02360: ?
pub fn i_c02360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02362;
    cpu.ir = 0x100F;
    let v1 = cpu.a[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C02362: movep.w $e0e(a6), d7
pub fn i_c02362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02366;
    cpu.ir = 0xF0E;
    let a = cpu.a[6].wrapping_add(0xE0E);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C02366: movep.w $d0d(a5), d6
pub fn i_c02366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0236A;
    cpu.ir = 0xD0D;
    let a = cpu.a[5].wrapping_add(0xD0D);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C0236A: ?
pub fn i_c0236a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0236E;
    cpu.ir = 0xC0C;
    let v1 = cpu.a[4] & 0xFF;
    sub_with_flags(cpu, v1, 0xB, Size::Byte, false);
    8
}
/// $C0236E: movep.w $b0b(a3), d5
pub fn i_c0236e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02372;
    cpu.ir = 0xB0B;
    let a = cpu.a[3].wrapping_add(0xB0B);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C02372: ?
pub fn i_c02372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02376;
    cpu.ir = 0xA0A;
    let v1 = cpu.a[2] & 0xFF;
    let r2 = (v1 ^ 0xA) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.a[2] = r2;
    8
}
/// $C02376: ?
pub fn i_c02376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0237A;
    cpu.ir = 0xA09;
    let v1 = cpu.a[1] & 0xFF;
    let r2 = (v1 ^ 0x9) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.a[1] = r2;
    8
}
/// $C0237A: movep.w $909(a1), d4
pub fn i_c0237a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0237E;
    cpu.ir = 0x909;
    let a = cpu.a[1].wrapping_add(0x909);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[4] = (cpu.d[4] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C0237E: ?
pub fn i_c0237e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02382;
    cpu.ir = 0x808;
    let mk: u32 = 1u32 << ((61) % 32);
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C02382: move.w ([$302d2b29, a7, d3.w * 4], $2725), d5
pub fn i_c02382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02386;
    cpu.ir = 0x3A37;
    let ad2 = cpu.a[7].wrapping_add(0x32).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C02386: move.w $2b29(a5), d0
pub fn i_c02386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0238A;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x2B29);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C0238A: move.l -(a5), -(a3)
pub fn i_c0238a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0238C;
    cpu.ir = 0x2725;
    let ad2 = cpu.a[5].wrapping_sub(4); cpu.a[5] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[3];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[3] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C0238C: move.l -(a2), -(a1)
pub fn i_c0238c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0238E;
    cpu.ir = 0x2322;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[1];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[1] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C0238E: move.l (a7)+, d0
pub fn i_c0238e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02390;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C02390: move.b (a4)+, -(a6)
pub fn i_c02390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02392;
    cpu.ir = 0x1D1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02392: move.b (a2)+, -(a5)
pub fn i_c02392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02394;
    cpu.ir = 0x1B1A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02394: move.b (a0)+, -(a4)
pub fn i_c02394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02396;
    cpu.ir = 0x1918;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02396: move.b (a6), -(a3)
pub fn i_c02396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02398;
    cpu.ir = 0x1716;
    let v2 = bus.read8(cpu.a[6]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[3].wrapping_sub(1); cpu.a[3] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C02398: move.b (a5), d3
pub fn i_c02398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0239A;
    cpu.ir = 0x1615;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C0239A: move.b (a3), d2
pub fn i_c0239a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0239C;
    cpu.ir = 0x1413;
    let v2 = bus.read8(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C0239C: move.b (a2), -(a1)
pub fn i_c0239c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0239E;
    cpu.ir = 0x1312;
    let v2 = bus.read8(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[1].wrapping_sub(1); cpu.a[1] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C0239E: move.b (a1), d1
pub fn i_c0239e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC023A0;
    cpu.ir = 0x1211;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C023A0: move.b (a0), -(a0)
pub fn i_c023a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC023A2;
    cpu.ir = 0x1110;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C023A2: move.b (a0), d0
pub fn i_c023a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC023A4;
    cpu.ir = 0x1010;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C023A4: movep.w $e0e(a7), d7
pub fn i_c023a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC023A8;
    cpu.ir = 0xF0F;
    let a = cpu.a[7].wrapping_add(0xE0E);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C023A8: ?
pub fn i_c023a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC023AC;
    cpu.ir = 0xE0D;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C023AC: movep.w $c0c(a4), d6
pub fn i_c023ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC023B0;
    cpu.ir = 0xD0C;
    let a = cpu.a[4].wrapping_add(0xC0C);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C023B0: ?
pub fn i_c023b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC023B4;
    cpu.ir = 0xC0B;
    let v1 = cpu.a[3] & 0xFF;
    sub_with_flags(cpu, v1, 0xB, Size::Byte, false);
    8
}
/// $C023B4: movep.w $a0a(a3), d5
pub fn i_c023b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC023B8;
    cpu.ir = 0xB0B;
    let a = cpu.a[3].wrapping_add(0xA0A);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C023B8: ?
pub fn i_c023b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC023BC;
    cpu.ir = 0xA0A;
    let v1 = cpu.a[2] & 0xFF;
    let r2 = (v1 ^ 0x9) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.a[2] = r2;
    8
}
/// $C023BC: movep.w $909(a1), d4
pub fn i_c023bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC023C0;
    cpu.ir = 0x909;
    let a = cpu.a[1].wrapping_add(0x909);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[4] = (cpu.d[4] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C023C0: ?
pub fn i_c023c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC023C2;
    cpu.ir = 0x403D;
    let v1: u32 = 0x0;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x80) != 0 && (r2 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x80 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    4
}
/// $C023C2: move.w $3533.w, -(a5)
pub fn i_c023c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC023C6;
    cpu.ir = 0x3B38;
    let v2 = bus.read16(0x3533) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[5].wrapping_sub(2); cpu.a[5] = ad3;
    bus.write16(ad3, v2 as u16);
    18
}
/// $C023C6: move.w $2c2a(a6), d0
pub fn i_c023c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC023CA;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2C2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C023CA: move.l -(a6), d4
pub fn i_c023ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC023CC;
    cpu.ir = 0x2826;
    let ad2 = cpu.a[6].wrapping_sub(4); cpu.a[6] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[4] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C023CC: move.l -(a3), -(a2)
pub fn i_c023cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC023CE;
    cpu.ir = 0x2523;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[2];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[2] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C023CE: move.l -(a0), -(a0)
pub fn i_c023ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC023D0;
    cpu.ir = 0x2120;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[0];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[0] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C023D0: move.b (a5)+, -(a7)
pub fn i_c023d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC023D2;
    cpu.ir = 0x1F1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C023D2: move.b (a3)+, d6
pub fn i_c023d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC023D4;
    cpu.ir = 0x1C1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C023D4: move.b (a1)+, d5
pub fn i_c023d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC023D6;
    cpu.ir = 0x1A19;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C023D6: move.b (a0)+, d4
pub fn i_c023d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC023D8;
    cpu.ir = 0x1818;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C023D8: move.b (a6), -(a3)
pub fn i_c023d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC023DA;
    cpu.ir = 0x1716;
    let v2 = bus.read8(cpu.a[6]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[3].wrapping_sub(1); cpu.a[3] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C023DA: move.b (a5), -(a2)
pub fn i_c023da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC023DC;
    cpu.ir = 0x1515;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[2].wrapping_sub(1); cpu.a[2] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C023DC: move.b (a3), d2
pub fn i_c023dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC023DE;
    cpu.ir = 0x1413;
    let v2 = bus.read8(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C023DE: move.b (a2), -(a1)
pub fn i_c023de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC023E0;
    cpu.ir = 0x1312;
    let v2 = bus.read8(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[1].wrapping_sub(1); cpu.a[1] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C023E0: move.b (a1), d1
pub fn i_c023e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC023E2;
    cpu.ir = 0x1211;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C023E2: move.b (a0), -(a0)
pub fn i_c023e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC023E4;
    cpu.ir = 0x1110;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C023E4: move.b (a0), d0
pub fn i_c023e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC023E6;
    cpu.ir = 0x1010;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C023E6: movep.w $f0e(a7), d7
pub fn i_c023e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC023EA;
    cpu.ir = 0xF0F;
    let a = cpu.a[7].wrapping_add(0xF0E);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C023EA: ?
pub fn i_c023ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC023EE;
    cpu.ir = 0xE0E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C023EE: movep.w $c0c(a5), d6
pub fn i_c023ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC023F2;
    cpu.ir = 0xD0D;
    let a = cpu.a[5].wrapping_add(0xC0C);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C023F2: ?
pub fn i_c023f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC023F6;
    cpu.ir = 0xC0C;
    let v1 = cpu.a[4] & 0xFF;
    sub_with_flags(cpu, v1, 0xB, Size::Byte, false);
    8
}
/// $C023F6: movep.w $b0a(a3), d5
pub fn i_c023f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC023FA;
    cpu.ir = 0xB0B;
    let a = cpu.a[3].wrapping_add(0xB0A);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C023FA: ?
pub fn i_c023fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC023FE;
    cpu.ir = 0xA0A;
    let v1 = cpu.a[2] & 0xFF;
    let r2 = (v1 ^ 0xA) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.a[2] = r2;
    8
}
/// $C023FE: ?
pub fn i_c023fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02402;
    cpu.ir = 0xA0A;
    let v1 = cpu.a[2] & 0xFF;
    let r2 = (v1 ^ 0x3D) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.a[2] = r2;
    8
}
/// $C02402: move.w $3634.w, -(a5)
pub fn i_c02402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02406;
    cpu.ir = 0x3B38;
    let v2 = bus.read16(0x3634) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[5].wrapping_sub(2); cpu.a[5] = ad3;
    bus.write16(ad3, v2 as u16);
    18
}
/// $C02406: move.w $2d2b(a7), -(a0)
pub fn i_c02406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0240A;
    cpu.ir = 0x312F;
    let ad2 = cpu.a[7].wrapping_add(0x2D2B);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_sub(2); cpu.a[0] = ad4;
    bus.write16(ad4, v3 as u16);
    18
}
/// $C0240A: move.l -(a7), -(a4)
pub fn i_c0240a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0240C;
    cpu.ir = 0x2927;
    let ad2 = cpu.a[7].wrapping_sub(4); cpu.a[7] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[4];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[4] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C0240C: move.l -(a4), d3
pub fn i_c0240c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0240E;
    cpu.ir = 0x2624;
    let ad2 = cpu.a[4].wrapping_sub(4); cpu.a[4] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C0240E: move.l -(a1), -(a1)
pub fn i_c0240e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02410;
    cpu.ir = 0x2321;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[1];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[1] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C02410: move.l (a7)+, d0
pub fn i_c02410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02412;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C02412: move.b (a5)+, d7
pub fn i_c02412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02414;
    cpu.ir = 0x1E1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02414: move.b (a3)+, -(a5)
pub fn i_c02414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02416;
    cpu.ir = 0x1B1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02416: move.b (a1)+, d5
pub fn i_c02416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02418;
    cpu.ir = 0x1A19;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02418: move.b (a7), d4
pub fn i_c02418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0241A;
    cpu.ir = 0x1817;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C0241A: move.b (a6), d3
pub fn i_c0241a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0241C;
    cpu.ir = 0x1616;
    let v2 = bus.read8(cpu.a[6]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C0241C: move.b (a5), -(a2)
pub fn i_c0241c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0241E;
    cpu.ir = 0x1515;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[2].wrapping_sub(1); cpu.a[2] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C0241E: move.b (a3), d2
pub fn i_c0241e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02420;
    cpu.ir = 0x1413;
    let v2 = bus.read8(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C02420: move.b (a2), -(a1)
pub fn i_c02420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02422;
    cpu.ir = 0x1312;
    let v2 = bus.read8(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[1].wrapping_sub(1); cpu.a[1] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C02422: move.b (a1), d1
pub fn i_c02422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02424;
    cpu.ir = 0x1211;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C02424: move.b (a1), -(a0)
pub fn i_c02424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02426;
    cpu.ir = 0x1111;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C02426: move.b (a0), d0
pub fn i_c02426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02428;
    cpu.ir = 0x1010;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C02428: ?
pub fn i_c02428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0242A;
    cpu.ir = 0x100F;
    let v1 = cpu.a[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C0242A: movep.w $e0e(a7), d7
pub fn i_c0242a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0242E;
    cpu.ir = 0xF0F;
    let a = cpu.a[7].wrapping_add(0xE0E);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C0242E: ?
pub fn i_c0242e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02432;
    cpu.ir = 0xE0D;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C02432: movep.w $c0c(a4), d6
pub fn i_c02432(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02436;
    cpu.ir = 0xD0C;
    let a = cpu.a[4].wrapping_add(0xC0C);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C02436: ?
pub fn i_c02436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0243A;
    cpu.ir = 0xC0C;
    let v1 = cpu.a[4] & 0xFF;
    sub_with_flags(cpu, v1, 0xB, Size::Byte, false);
    8
}
/// $C0243A: movep.w $b0a(a3), d5
pub fn i_c0243a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0243E;
    cpu.ir = 0xB0B;
    let a = cpu.a[3].wrapping_add(0xB0A);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C0243E: ?
pub fn i_c0243e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02442;
    cpu.ir = 0xA0A;
    let v1 = cpu.a[2] & 0xFF;
    let r2 = (v1 ^ 0x3E) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.a[2] = r2;
    8
}
/// $C02442: move.w $37343230.l, -(a5)
pub fn i_c02442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02448;
    cpu.ir = 0x3B39;
    let v2 = bus.read16(0x37343230) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[5].wrapping_sub(2); cpu.a[5] = ad3;
    bus.write16(ad3, v2 as u16);
    22
}
/// $C02448: move.l $2a29(a4), d7
pub fn i_c02448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0244C;
    cpu.ir = 0x2E2C;
    let ad2 = cpu.a[4].wrapping_add(0x2A29);
    let v3 = bus.read32(ad2);
    cpu.d[7] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C0244C: move.l -(a5), -(a3)
pub fn i_c0244c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0244E;
    cpu.ir = 0x2725;
    let ad2 = cpu.a[5].wrapping_sub(4); cpu.a[5] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[3];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[3] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C0244E: move.l -(a3), d2
pub fn i_c0244e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02450;
    cpu.ir = 0x2423;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02450: move.l -(a0), -(a0)
pub fn i_c02450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02452;
    cpu.ir = 0x2120;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[0];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[0] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C02452: move.b (a6)+, -(a7)
pub fn i_c02452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02454;
    cpu.ir = 0x1F1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02454: move.b (a4)+, -(a6)
pub fn i_c02454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02456;
    cpu.ir = 0x1D1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02456: move.b (a2)+, -(a5)
pub fn i_c02456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02458;
    cpu.ir = 0x1B1A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02458: move.b (a0)+, -(a4)
pub fn i_c02458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0245A;
    cpu.ir = 0x1918;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C0245A: move.b (a7), d4
pub fn i_c0245a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0245C;
    cpu.ir = 0x1817;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C0245C: move.b (a6), d3
pub fn i_c0245c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0245E;
    cpu.ir = 0x1616;
    let v2 = bus.read8(cpu.a[6]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C0245E: move.b (a4), -(a2)
pub fn i_c0245e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02460;
    cpu.ir = 0x1514;
    let v2 = bus.read8(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[2].wrapping_sub(1); cpu.a[2] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C02460: move.b (a3), d2
pub fn i_c02460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02462;
    cpu.ir = 0x1413;
    let v2 = bus.read8(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C02462: move.b (a2), -(a1)
pub fn i_c02462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02464;
    cpu.ir = 0x1312;
    let v2 = bus.read8(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[1].wrapping_sub(1); cpu.a[1] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C02464: move.b (a2), d1
pub fn i_c02464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02466;
    cpu.ir = 0x1212;
    let v2 = bus.read8(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C02466: move.b (a1), -(a0)
pub fn i_c02466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02468;
    cpu.ir = 0x1111;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C02468: move.b (a0), d0
pub fn i_c02468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0246A;
    cpu.ir = 0x1010;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C0246A: ?
pub fn i_c0246a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0246C;
    cpu.ir = 0x100F;
    let v1 = cpu.a[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C0246C: movep.w $e0e(a7), d7
pub fn i_c0246c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02470;
    cpu.ir = 0xF0F;
    let a = cpu.a[7].wrapping_add(0xE0E);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C02470: ?
pub fn i_c02470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02474;
    cpu.ir = 0xE0E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C02474: movep.w $c0c(a5), d6
pub fn i_c02474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02478;
    cpu.ir = 0xD0D;
    let a = cpu.a[5].wrapping_add(0xC0C);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C02478: ?
pub fn i_c02478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0247C;
    cpu.ir = 0xC0C;
    let v1 = cpu.a[4] & 0xFF;
    sub_with_flags(cpu, v1, 0xB, Size::Byte, false);
    8
}
/// $C0247C: movep.w $b0b(a3), d5
pub fn i_c0247c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02480;
    cpu.ir = 0xB0B;
    let a = cpu.a[3].wrapping_add(0xB0B);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C02480: ?
pub fn i_c02480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02482;
    cpu.ir = 0x403E;
    let v1: u32 = 0x0;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x80) != 0 && (r2 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x80 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    4
}
/// $C02482: move.w $37353331.l, -(a5)
pub fn i_c02482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02488;
    cpu.ir = 0x3B39;
    let v2 = bus.read16(0x37353331) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[5].wrapping_sub(2); cpu.a[5] = ad3;
    bus.write16(ad3, v2 as u16);
    22
}
/// $C02488: move.l $2b2a(a5), -(a7)
pub fn i_c02488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0248C;
    cpu.ir = 0x2F2D;
    let ad2 = cpu.a[5].wrapping_add(0x2B2A);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[7];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[7] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C0248C: move.l -(a7), d4
pub fn i_c0248c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0248E;
    cpu.ir = 0x2827;
    let ad2 = cpu.a[7].wrapping_sub(4); cpu.a[7] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[4] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C0248E: move.l -(a4), -(a2)
pub fn i_c0248e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02490;
    cpu.ir = 0x2524;
    let ad2 = cpu.a[4].wrapping_sub(4); cpu.a[4] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[2];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[2] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C02490: move.l -(a1), d1
pub fn i_c02490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02492;
    cpu.ir = 0x2221;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02492: move.l (a7)+, d0
pub fn i_c02492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02494;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C02494: move.b (a5)+, d7
pub fn i_c02494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02496;
    cpu.ir = 0x1E1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02496: move.b (a3)+, d6
pub fn i_c02496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02498;
    cpu.ir = 0x1C1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02498: move.b (a1)+, d5
pub fn i_c02498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0249A;
    cpu.ir = 0x1A19;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C0249A: move.b (a0)+, -(a4)
pub fn i_c0249a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0249C;
    cpu.ir = 0x1918;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C0249C: move.b (a7), -(a3)
pub fn i_c0249c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0249E;
    cpu.ir = 0x1717;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[3].wrapping_sub(1); cpu.a[3] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C0249E: move.b (a5), d3
pub fn i_c0249e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC024A0;
    cpu.ir = 0x1615;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C024A0: move.b (a4), -(a2)
pub fn i_c024a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC024A2;
    cpu.ir = 0x1514;
    let v2 = bus.read8(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[2].wrapping_sub(1); cpu.a[2] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C024A2: move.b (a3), d2
pub fn i_c024a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC024A4;
    cpu.ir = 0x1413;
    let v2 = bus.read8(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C024A4: move.b (a2), -(a1)
pub fn i_c024a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC024A6;
    cpu.ir = 0x1312;
    let v2 = bus.read8(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[1].wrapping_sub(1); cpu.a[1] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C024A6: move.b (a2), d1
pub fn i_c024a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC024A8;
    cpu.ir = 0x1212;
    let v2 = bus.read8(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C024A8: move.b (a1), -(a0)
pub fn i_c024a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC024AA;
    cpu.ir = 0x1111;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C024AA: move.b (a0), d0
pub fn i_c024aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC024AC;
    cpu.ir = 0x1010;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C024AC: move.b (a0), d0
pub fn i_c024ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC024AE;
    cpu.ir = 0x1010;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C024AE: movep.w $f0e(a7), d7
pub fn i_c024ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC024B2;
    cpu.ir = 0xF0F;
    let a = cpu.a[7].wrapping_add(0xF0E);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C024B2: ?
pub fn i_c024b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC024B6;
    cpu.ir = 0xE0E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C024B6: movep.w $d0c(a5), d6
pub fn i_c024b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC024BA;
    cpu.ir = 0xD0D;
    let a = cpu.a[5].wrapping_add(0xD0C);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C024BA: ?
pub fn i_c024ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC024BE;
    cpu.ir = 0xC0C;
    let v1 = cpu.a[4] & 0xFF;
    sub_with_flags(cpu, v1, 0xC, Size::Byte, false);
    8
}
/// $C024BE: ?
pub fn i_c024be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC024C2;
    cpu.ir = 0xC0B;
    let v1 = cpu.a[3] & 0xFF;
    sub_with_flags(cpu, v1, 0x3E, Size::Byte, false);
    8
}
/// $C024C2: move.w $c05cfa(pc), d6
pub fn i_c024c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC024C6;
    cpu.ir = 0x3C3A;
    let v2 = bus.read16(0xC05CFA) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v2 & 0xFFFF);
    12
}
/// $C024C6: move.w $2e(a2, d3.w), d2
pub fn i_c024c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC024CA;
    cpu.ir = 0x3432;
    let ad2 = cpu.a[2].wrapping_add(0x2E).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C024CA: move.l $2928(a3), d6
pub fn i_c024ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC024CE;
    cpu.ir = 0x2C2B;
    let ad2 = cpu.a[3].wrapping_add(0x2928);
    let v3 = bus.read32(ad2);
    cpu.d[6] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C024CE: move.l -(a5), d3
pub fn i_c024ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC024D0;
    cpu.ir = 0x2625;
    let ad2 = cpu.a[5].wrapping_sub(4); cpu.a[5] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C024D0: move.l -(a2), -(a1)
pub fn i_c024d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC024D2;
    cpu.ir = 0x2322;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[1];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[1] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C024D2: move.l -(a0), -(a0)
pub fn i_c024d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC024D4;
    cpu.ir = 0x2120;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[0];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[0] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C024D4: move.b (a6)+, -(a7)
pub fn i_c024d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC024D6;
    cpu.ir = 0x1F1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C024D6: move.b (a4)+, -(a6)
pub fn i_c024d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC024D8;
    cpu.ir = 0x1D1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C024D8: move.b (a2)+, -(a5)
pub fn i_c024d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC024DA;
    cpu.ir = 0x1B1A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C024DA: move.b (a1)+, d5
pub fn i_c024da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC024DC;
    cpu.ir = 0x1A19;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C024DC: move.b (a0)+, d4
pub fn i_c024dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC024DE;
    cpu.ir = 0x1818;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C024DE: move.b (a6), -(a3)
pub fn i_c024de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC024E0;
    cpu.ir = 0x1716;
    let v2 = bus.read8(cpu.a[6]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[3].wrapping_sub(1); cpu.a[3] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C024E0: move.b (a5), d3
pub fn i_c024e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC024E2;
    cpu.ir = 0x1615;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C024E2: move.b (a4), -(a2)
pub fn i_c024e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC024E4;
    cpu.ir = 0x1514;
    let v2 = bus.read8(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[2].wrapping_sub(1); cpu.a[2] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C024E4: move.b (a3), d2
pub fn i_c024e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC024E6;
    cpu.ir = 0x1413;
    let v2 = bus.read8(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C024E6: move.b (a2), -(a1)
pub fn i_c024e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC024E8;
    cpu.ir = 0x1312;
    let v2 = bus.read8(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[1].wrapping_sub(1); cpu.a[1] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C024E8: move.b (a2), d1
pub fn i_c024e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC024EA;
    cpu.ir = 0x1212;
    let v2 = bus.read8(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C024EA: move.b (a1), -(a0)
pub fn i_c024ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC024EC;
    cpu.ir = 0x1111;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C024EC: move.b (a0), -(a0)
pub fn i_c024ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC024EE;
    cpu.ir = 0x1110;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C024EE: move.b (a0), d0
pub fn i_c024ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC024F0;
    cpu.ir = 0x1010;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C024F0: movep.w $f0f(a7), d7
pub fn i_c024f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC024F4;
    cpu.ir = 0xF0F;
    let a = cpu.a[7].wrapping_add(0xF0F);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C024F4: ?
pub fn i_c024f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC024F8;
    cpu.ir = 0xE0E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C024F8: movep.w $d0d(a5), d6
pub fn i_c024f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC024FC;
    cpu.ir = 0xD0D;
    let a = cpu.a[5].wrapping_add(0xD0D);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C024FC: ?
pub fn i_c024fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02500;
    cpu.ir = 0xC0C;
    let v1 = cpu.a[4] & 0xFF;
    sub_with_flags(cpu, v1, 0xC, Size::Byte, false);
    8
}
/// $C02500: ?
pub fn i_c02500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02502;
    cpu.ir = 0x403E;
    let v1: u32 = 0x0;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x80) != 0 && (r2 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x80 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    4
}
/// $C02502: move.w $c05d3a(pc), d6
pub fn i_c02502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02506;
    cpu.ir = 0x3C3A;
    let v2 = bus.read16(0xC05D3A) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v2 & 0xFFFF);
    12
}
/// $C02506: move.w $2f(a2, d3.w), d2
pub fn i_c02506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0250A;
    cpu.ir = 0x3432;
    let ad2 = cpu.a[2].wrapping_add(0x2F).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C0250A: move.l $2a29(a4), -(a6)
pub fn i_c0250a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0250E;
    cpu.ir = 0x2D2C;
    let ad2 = cpu.a[4].wrapping_add(0x2A29);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[6];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[6] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C0250E: move.l -(a6), -(a3)
pub fn i_c0250e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02510;
    cpu.ir = 0x2726;
    let ad2 = cpu.a[6].wrapping_sub(4); cpu.a[6] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[3];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[3] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C02510: move.l -(a3), -(a2)
pub fn i_c02510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02512;
    cpu.ir = 0x2523;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[2];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[2] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C02512: move.l -(a1), d1
pub fn i_c02512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02514;
    cpu.ir = 0x2221;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02514: move.l (a7)+, d0
pub fn i_c02514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02516;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C02516: move.b (a5)+, d7
pub fn i_c02516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02518;
    cpu.ir = 0x1E1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02518: move.b (a3)+, d6
pub fn i_c02518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0251A;
    cpu.ir = 0x1C1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C0251A: move.b (a2)+, -(a5)
pub fn i_c0251a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0251C;
    cpu.ir = 0x1B1A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C0251C: move.b (a1)+, -(a4)
pub fn i_c0251c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0251E;
    cpu.ir = 0x1919;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C0251E: move.b (a7), d4
pub fn i_c0251e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02520;
    cpu.ir = 0x1817;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C02520: move.b (a6), -(a3)
pub fn i_c02520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02522;
    cpu.ir = 0x1716;
    let v2 = bus.read8(cpu.a[6]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[3].wrapping_sub(1); cpu.a[3] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C02522: move.b (a5), d3
pub fn i_c02522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02524;
    cpu.ir = 0x1615;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C02524: move.b (a4), -(a2)
pub fn i_c02524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02526;
    cpu.ir = 0x1514;
    let v2 = bus.read8(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[2].wrapping_sub(1); cpu.a[2] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C02526: move.b (a3), d2
pub fn i_c02526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02528;
    cpu.ir = 0x1413;
    let v2 = bus.read8(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C02528: move.b (a2), -(a1)
pub fn i_c02528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0252A;
    cpu.ir = 0x1312;
    let v2 = bus.read8(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[1].wrapping_sub(1); cpu.a[1] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C0252A: move.b (a2), d1
pub fn i_c0252a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0252C;
    cpu.ir = 0x1212;
    let v2 = bus.read8(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C0252C: move.b (a1), -(a0)
pub fn i_c0252c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0252E;
    cpu.ir = 0x1111;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C0252E: move.b (a0), -(a0)
pub fn i_c0252e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02530;
    cpu.ir = 0x1110;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C02530: move.b (a0), d0
pub fn i_c02530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02532;
    cpu.ir = 0x1010;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C02532: ?
pub fn i_c02532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02534;
    cpu.ir = 0x100F;
    let v1 = cpu.a[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C02534: movep.w $e0e(a7), d7
pub fn i_c02534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02538;
    cpu.ir = 0xF0F;
    let a = cpu.a[7].wrapping_add(0xE0E);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C02538: ?
pub fn i_c02538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0253C;
    cpu.ir = 0xE0E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C0253C: movep.w $d0d(a5), d6
pub fn i_c0253c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02540;
    cpu.ir = 0xD0D;
    let a = cpu.a[5].wrapping_add(0xD0D);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C02540: ?
pub fn i_c02540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02542;
    cpu.ir = 0x403E;
    let v1: u32 = 0x0;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x80) != 0 && (r2 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x80 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    4
}
/// $C02542: move.w $c05d7a(pc), d6
pub fn i_c02542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02546;
    cpu.ir = 0x3C3A;
    let v2 = bus.read16(0xC05D7A) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v2 & 0xFFFF);
    12
}
/// $C02546: move.w $2e2c2b29(a3, d3.w), -(a2)
pub fn i_c02546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0254A;
    cpu.ir = 0x3533;
    let ad2 = cpu.a[3].wrapping_add(0x30).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2].wrapping_sub(2); cpu.a[2] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C0254A: move.l $2b29(a4), d7
pub fn i_c0254a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0254E;
    cpu.ir = 0x2E2C;
    let ad2 = cpu.a[4].wrapping_add(0x2B29);
    let v3 = bus.read32(ad2);
    cpu.d[7] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C0254E: move.l -(a7), d4
pub fn i_c0254e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02550;
    cpu.ir = 0x2827;
    let ad2 = cpu.a[7].wrapping_sub(4); cpu.a[7] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[4] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02550: move.l -(a4), -(a2)
pub fn i_c02550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02552;
    cpu.ir = 0x2524;
    let ad2 = cpu.a[4].wrapping_sub(4); cpu.a[4] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[2];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[2] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C02552: move.l -(a2), -(a1)
pub fn i_c02552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02554;
    cpu.ir = 0x2322;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[1];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[1] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C02554: move.l -(a0), -(a0)
pub fn i_c02554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02556;
    cpu.ir = 0x2120;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[0];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[0] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C02556: move.b (a6)+, -(a7)
pub fn i_c02556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02558;
    cpu.ir = 0x1F1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02558: move.b (a4)+, -(a6)
pub fn i_c02558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0255A;
    cpu.ir = 0x1D1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C0255A: move.b (a3)+, d6
pub fn i_c0255a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0255C;
    cpu.ir = 0x1C1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C0255C: move.b (a2)+, d5
pub fn i_c0255c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0255E;
    cpu.ir = 0x1A1A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C0255E: move.b (a0)+, -(a4)
pub fn i_c0255e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02560;
    cpu.ir = 0x1918;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02560: move.b (a7), d4
pub fn i_c02560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02562;
    cpu.ir = 0x1817;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C02562: move.b (a6), -(a3)
pub fn i_c02562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02564;
    cpu.ir = 0x1716;
    let v2 = bus.read8(cpu.a[6]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[3].wrapping_sub(1); cpu.a[3] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C02564: move.b (a5), d3
pub fn i_c02564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02566;
    cpu.ir = 0x1615;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C02566: move.b (a4), -(a2)
pub fn i_c02566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02568;
    cpu.ir = 0x1514;
    let v2 = bus.read8(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[2].wrapping_sub(1); cpu.a[2] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C02568: move.b (a3), d2
pub fn i_c02568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0256A;
    cpu.ir = 0x1413;
    let v2 = bus.read8(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C0256A: move.b (a3), -(a1)
pub fn i_c0256a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0256C;
    cpu.ir = 0x1313;
    let v2 = bus.read8(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[1].wrapping_sub(1); cpu.a[1] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C0256C: move.b (a2), d1
pub fn i_c0256c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0256E;
    cpu.ir = 0x1212;
    let v2 = bus.read8(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C0256E: move.b (a1), -(a0)
pub fn i_c0256e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02570;
    cpu.ir = 0x1111;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C02570: move.b (a0), -(a0)
pub fn i_c02570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02572;
    cpu.ir = 0x1110;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C02572: move.b (a0), d0
pub fn i_c02572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02574;
    cpu.ir = 0x1010;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C02574: ?
pub fn i_c02574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02576;
    cpu.ir = 0x100F;
    let v1 = cpu.a[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C02576: movep.w $f0e(a7), d7
pub fn i_c02576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0257A;
    cpu.ir = 0xF0F;
    let a = cpu.a[7].wrapping_add(0xF0E);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C0257A: ?
pub fn i_c0257a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0257E;
    cpu.ir = 0xE0E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C0257E: movep.w $403e(a5), d6
pub fn i_c0257e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02582;
    cpu.ir = 0xD0D;
    let a = cpu.a[5].wrapping_add(0x403E);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C02582: move.w $c05ebb(pc), d6
pub fn i_c02582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02586;
    cpu.ir = 0x3C3A;
    let v2 = bus.read16(0xC05EBB) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v2 & 0xFFFF);
    12
}
/// $C02586: move.w $30(a3, d3.w), -(a2)
pub fn i_c02586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0258A;
    cpu.ir = 0x3533;
    let ad2 = cpu.a[3].wrapping_add(0x30).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2].wrapping_sub(2); cpu.a[2] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C0258A: move.l $2c2a(a5), -(a7)
pub fn i_c0258a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0258E;
    cpu.ir = 0x2F2D;
    let ad2 = cpu.a[5].wrapping_add(0x2C2A);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[7];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[7] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C0258E: move.l $2625(a0), -(a4)
pub fn i_c0258e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02592;
    cpu.ir = 0x2928;
    let ad2 = cpu.a[0].wrapping_add(0x2625);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[4];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[4] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C02590: move.l -(a5), d3
pub fn i_c02590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02592;
    cpu.ir = 0x2625;
    let ad2 = cpu.a[5].wrapping_sub(4); cpu.a[5] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02592: move.l -(a3), d2
pub fn i_c02592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02594;
    cpu.ir = 0x2423;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02594: move.l -(a1), d1
pub fn i_c02594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02596;
    cpu.ir = 0x2221;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02596: move.l (a7)+, d0
pub fn i_c02596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02598;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C02598: move.b (a5)+, d7
pub fn i_c02598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0259A;
    cpu.ir = 0x1E1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C0259A: move.b (a4)+, -(a6)
pub fn i_c0259a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0259C;
    cpu.ir = 0x1D1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C0259C: move.b (a2)+, -(a5)
pub fn i_c0259c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0259E;
    cpu.ir = 0x1B1A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C0259E: move.b (a1)+, d5
pub fn i_c0259e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC025A0;
    cpu.ir = 0x1A19;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C025A0: move.b (a0)+, -(a4)
pub fn i_c025a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC025A2;
    cpu.ir = 0x1918;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C025A2: move.b (a7), -(a3)
pub fn i_c025a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC025A4;
    cpu.ir = 0x1717;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[3].wrapping_sub(1); cpu.a[3] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C025A4: move.b (a6), d3
pub fn i_c025a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC025A6;
    cpu.ir = 0x1616;
    let v2 = bus.read8(cpu.a[6]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C025A6: move.b (a5), -(a2)
pub fn i_c025a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC025A8;
    cpu.ir = 0x1515;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[2].wrapping_sub(1); cpu.a[2] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C025A8: move.b (a4), d2
pub fn i_c025a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC025AA;
    cpu.ir = 0x1414;
    let v2 = bus.read8(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C025AA: move.b (a3), d2
pub fn i_c025aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC025AC;
    cpu.ir = 0x1413;
    let v2 = bus.read8(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C025AC: move.b (a3), -(a1)
pub fn i_c025ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC025AE;
    cpu.ir = 0x1313;
    let v2 = bus.read8(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[1].wrapping_sub(1); cpu.a[1] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C025AE: move.b (a2), d1
pub fn i_c025ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC025B0;
    cpu.ir = 0x1212;
    let v2 = bus.read8(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C025B0: move.b (a1), d1
pub fn i_c025b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC025B2;
    cpu.ir = 0x1211;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C025B2: move.b (a1), -(a0)
pub fn i_c025b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC025B4;
    cpu.ir = 0x1111;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C025B4: move.b (a0), d0
pub fn i_c025b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC025B6;
    cpu.ir = 0x1010;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C025B6: move.b (a0), d0
pub fn i_c025b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC025B8;
    cpu.ir = 0x1010;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C025B8: movep.w $f0f(a7), d7
pub fn i_c025b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC025BC;
    cpu.ir = 0xF0F;
    let a = cpu.a[7].wrapping_add(0xF0F);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C025BC: ?
pub fn i_c025bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC025C0;
    cpu.ir = 0xE0E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C025C0: ?
pub fn i_c025c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC025C2;
    cpu.ir = 0x403E;
    let v1: u32 = 0x0;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x80) != 0 && (r2 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x80 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    4
}
/// $C025C2: move.w ([$36f457f5, pc], d3.l, $aaaaaaaa), d6
pub fn i_c025c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC025C6;
    cpu.ir = 0x3C3B;
    let ad2 = 0xC025FB_u32.wrapping_add(cpu.d[3]);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C025C6: move.w $31(a4, d3.w), d3
pub fn i_c025c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC025CA;
    cpu.ir = 0x3634;
    let ad2 = cpu.a[4].wrapping_add(0x31).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C025CA: move.l $2c2b(a6), -(a7)
pub fn i_c025ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC025CE;
    cpu.ir = 0x2F2E;
    let ad2 = cpu.a[6].wrapping_add(0x2C2B);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[7];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[7] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C025CE: move.l $2726(a0), d5
pub fn i_c025ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC025D2;
    cpu.ir = 0x2A28;
    let ad2 = cpu.a[0].wrapping_add(0x2726);
    let v3 = bus.read32(ad2);
    cpu.d[5] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C025D2: move.l -(a4), -(a2)
pub fn i_c025d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC025D4;
    cpu.ir = 0x2524;
    let ad2 = cpu.a[4].wrapping_sub(4); cpu.a[4] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[2];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[2] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C025D4: move.l -(a2), -(a1)
pub fn i_c025d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC025D6;
    cpu.ir = 0x2322;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[1];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[1] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C025D6: move.l -(a0), -(a0)
pub fn i_c025d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC025D8;
    cpu.ir = 0x2120;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[0];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[0] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C025D8: move.b (a6)+, -(a7)
pub fn i_c025d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC025DA;
    cpu.ir = 0x1F1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C025DA: move.b (a5)+, d7
pub fn i_c025da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC025DC;
    cpu.ir = 0x1E1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C025DC: move.b (a3)+, d6
pub fn i_c025dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC025DE;
    cpu.ir = 0x1C1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C025DE: move.b (a2)+, -(a5)
pub fn i_c025de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC025E0;
    cpu.ir = 0x1B1A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C025E0: move.b (a1)+, -(a4)
pub fn i_c025e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC025E2;
    cpu.ir = 0x1919;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C025E2: move.b (a0)+, d4
pub fn i_c025e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC025E4;
    cpu.ir = 0x1818;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C025E4: move.b (a7), -(a3)
pub fn i_c025e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC025E6;
    cpu.ir = 0x1717;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[3].wrapping_sub(1); cpu.a[3] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C025E6: move.b (a6), d3
pub fn i_c025e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC025E8;
    cpu.ir = 0x1616;
    let v2 = bus.read8(cpu.a[6]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C025E8: move.b (a5), -(a2)
pub fn i_c025e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC025EA;
    cpu.ir = 0x1515;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[2].wrapping_sub(1); cpu.a[2] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C025EA: move.b (a4), d2
pub fn i_c025ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC025EC;
    cpu.ir = 0x1414;
    let v2 = bus.read8(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C025EC: move.b (a3), d2
pub fn i_c025ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC025EE;
    cpu.ir = 0x1413;
    let v2 = bus.read8(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C025EE: move.b (a3), -(a1)
pub fn i_c025ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC025F0;
    cpu.ir = 0x1313;
    let v2 = bus.read8(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[1].wrapping_sub(1); cpu.a[1] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C025F0: move.b (a2), d1
pub fn i_c025f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC025F2;
    cpu.ir = 0x1212;
    let v2 = bus.read8(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C025F2: move.b (a1), d1
pub fn i_c025f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC025F4;
    cpu.ir = 0x1211;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C025F4: move.b (a1), -(a0)
pub fn i_c025f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC025F6;
    cpu.ir = 0x1111;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C025F6: move.b (a0), d0
pub fn i_c025f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC025F8;
    cpu.ir = 0x1010;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C025F8: move.b (a0), d0
pub fn i_c025f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC025FA;
    cpu.ir = 0x1010;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C025FA: movep.w $f0f(a7), d7
pub fn i_c025fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC025FE;
    cpu.ir = 0xF0F;
    let a = cpu.a[7].wrapping_add(0xF0F);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C025FE: ?
pub fn i_c025fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02602;
    cpu.ir = 0xE0E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C02600: ?
pub fn i_c02600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02602;
    cpu.ir = 0x403E;
    let v1: u32 = 0x0;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x80) != 0 && (r2 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x80 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    4
}
/// $C02602: move.w $36343331(a16, d3.l), -(a6)
pub fn i_c02602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02606;
    cpu.ir = 0x3D3B;
    let ad2 = 0xC0263C_u32.wrapping_add(cpu.d[3]);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C02606: move.w ([$302e2d2c, a4, d3.w * 2]), d3
pub fn i_c02606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0260A;
    cpu.ir = 0x3634;
    let ad2 = cpu.a[4].wrapping_add(0x31).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C0260A: move.w $2d2c(a6), d0
pub fn i_c0260a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0260E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2D2C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C0260E: move.l $2827(a1), d5
pub fn i_c0260e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02612;
    cpu.ir = 0x2A29;
    let ad2 = cpu.a[1].wrapping_add(0x2827);
    let v3 = bus.read32(ad2);
    cpu.d[5] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C02612: move.l -(a5), d3
pub fn i_c02612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02614;
    cpu.ir = 0x2625;
    let ad2 = cpu.a[5].wrapping_sub(4); cpu.a[5] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02614: move.l -(a3), d2
pub fn i_c02614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02616;
    cpu.ir = 0x2423;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02616: move.l -(a1), d1
pub fn i_c02616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02618;
    cpu.ir = 0x2221;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02618: move.l (a7)+, d0
pub fn i_c02618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0261A;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C0261A: move.b (a6)+, d7
pub fn i_c0261a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0261C;
    cpu.ir = 0x1E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C0261C: move.b (a4)+, -(a6)
pub fn i_c0261c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0261E;
    cpu.ir = 0x1D1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C0261E: move.b (a3)+, -(a5)
pub fn i_c0261e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02620;
    cpu.ir = 0x1B1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02620: move.b (a2)+, d5
pub fn i_c02620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02622;
    cpu.ir = 0x1A1A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02622: move.b (a0)+, -(a4)
pub fn i_c02622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02624;
    cpu.ir = 0x1918;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02624: move.b (a7), d4
pub fn i_c02624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02626;
    cpu.ir = 0x1817;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C02626: move.b (a6), -(a3)
pub fn i_c02626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02628;
    cpu.ir = 0x1716;
    let v2 = bus.read8(cpu.a[6]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[3].wrapping_sub(1); cpu.a[3] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C02628: move.b (a6), d3
pub fn i_c02628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0262A;
    cpu.ir = 0x1616;
    let v2 = bus.read8(cpu.a[6]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C0262A: move.b (a5), -(a2)
pub fn i_c0262a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0262C;
    cpu.ir = 0x1515;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[2].wrapping_sub(1); cpu.a[2] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C0262C: move.b (a4), d2
pub fn i_c0262c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0262E;
    cpu.ir = 0x1414;
    let v2 = bus.read8(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C0262E: move.b (a3), d2
pub fn i_c0262e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02630;
    cpu.ir = 0x1413;
    let v2 = bus.read8(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C02630: move.b (a3), -(a1)
pub fn i_c02630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02632;
    cpu.ir = 0x1313;
    let v2 = bus.read8(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[1].wrapping_sub(1); cpu.a[1] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C02632: move.b (a2), d1
pub fn i_c02632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02634;
    cpu.ir = 0x1212;
    let v2 = bus.read8(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C02634: move.b (a1), d1
pub fn i_c02634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02636;
    cpu.ir = 0x1211;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C02636: move.b (a1), -(a0)
pub fn i_c02636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02638;
    cpu.ir = 0x1111;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C02638: move.b (a0), d0
pub fn i_c02638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0263A;
    cpu.ir = 0x1010;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C0263A: move.b (a0), d0
pub fn i_c0263a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0263C;
    cpu.ir = 0x1010;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C0263C: ?
pub fn i_c0263c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0263E;
    cpu.ir = 0x100F;
    let v1 = cpu.a[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C0263E: movep.w $403e(a7), d7
pub fn i_c0263e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02642;
    cpu.ir = 0xF0F;
    let a = cpu.a[7].wrapping_add(0x403E);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C02642: move.w $c0267c(pc, d3.l), -(a6)
pub fn i_c02642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02646;
    cpu.ir = 0x3D3B;
    let ad2 = 0xC0267C_u32.wrapping_add(cpu.d[3]);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C02646: move.w ([$302f2e2c, a5, d3.w * 2], $2b2a), d3
pub fn i_c02646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0264A;
    cpu.ir = 0x3635;
    let ad2 = cpu.a[5].wrapping_add(0x32).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C0264A: move.w $2e2c(a7), d0
pub fn i_c0264a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0264E;
    cpu.ir = 0x302F;
    let ad2 = cpu.a[7].wrapping_add(0x2E2C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C0264E: move.l $2928(a2), -(a5)
pub fn i_c0264e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02652;
    cpu.ir = 0x2B2A;
    let ad2 = cpu.a[2].wrapping_add(0x2928);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[5];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[5] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C02652: move.l -(a6), -(a3)
pub fn i_c02652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02654;
    cpu.ir = 0x2726;
    let ad2 = cpu.a[6].wrapping_sub(4); cpu.a[6] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[3];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[3] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C02654: move.l -(a4), -(a2)
pub fn i_c02654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02656;
    cpu.ir = 0x2524;
    let ad2 = cpu.a[4].wrapping_sub(4); cpu.a[4] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[2];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[2] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C02656: move.l -(a2), -(a1)
pub fn i_c02656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02658;
    cpu.ir = 0x2322;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[1];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[1] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C02658: move.l -(a0), -(a0)
pub fn i_c02658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0265A;
    cpu.ir = 0x2120;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[0];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[0] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C0265A: move.b (a6)+, -(a7)
pub fn i_c0265a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0265C;
    cpu.ir = 0x1F1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C0265C: move.b (a5)+, d7
pub fn i_c0265c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0265E;
    cpu.ir = 0x1E1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C0265E: move.b (a4)+, d6
pub fn i_c0265e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02660;
    cpu.ir = 0x1C1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02660: move.b (a2)+, -(a5)
pub fn i_c02660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02662;
    cpu.ir = 0x1B1A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02662: move.b (a1)+, d5
pub fn i_c02662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02664;
    cpu.ir = 0x1A19;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02664: move.b (a0)+, -(a4)
pub fn i_c02664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02666;
    cpu.ir = 0x1918;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02666: move.b (a7), d4
pub fn i_c02666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02668;
    cpu.ir = 0x1817;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C02668: move.b (a6), -(a3)
pub fn i_c02668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0266A;
    cpu.ir = 0x1716;
    let v2 = bus.read8(cpu.a[6]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[3].wrapping_sub(1); cpu.a[3] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C0266A: move.b (a5), d3
pub fn i_c0266a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0266C;
    cpu.ir = 0x1615;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C0266C: move.b (a5), -(a2)
pub fn i_c0266c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0266E;
    cpu.ir = 0x1515;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[2].wrapping_sub(1); cpu.a[2] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C0266E: move.b (a4), d2
pub fn i_c0266e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02670;
    cpu.ir = 0x1414;
    let v2 = bus.read8(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C02670: move.b (a3), d2
pub fn i_c02670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02672;
    cpu.ir = 0x1413;
    let v2 = bus.read8(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C02672: move.b (a3), -(a1)
pub fn i_c02672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02674;
    cpu.ir = 0x1313;
    let v2 = bus.read8(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[1].wrapping_sub(1); cpu.a[1] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C02674: move.b (a2), d1
pub fn i_c02674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02676;
    cpu.ir = 0x1212;
    let v2 = bus.read8(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C02676: move.b (a1), d1
pub fn i_c02676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02678;
    cpu.ir = 0x1211;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C02678: move.b (a1), -(a0)
pub fn i_c02678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0267A;
    cpu.ir = 0x1111;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C0267A: move.b (a0), -(a0)
pub fn i_c0267a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0267C;
    cpu.ir = 0x1110;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C0267C: move.b (a0), d0
pub fn i_c0267c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0267E;
    cpu.ir = 0x1010;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C0267E: ?
pub fn i_c0267e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02680;
    cpu.ir = 0x100F;
    let v1 = cpu.a[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C02680: ?
pub fn i_c02680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02682;
    cpu.ir = 0x403E;
    let v1: u32 = 0x0;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x80) != 0 && (r2 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x80 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    4
}
/// $C02682: move.w $c026bc(pc, d3.l), -(a6)
pub fn i_c02682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02686;
    cpu.ir = 0x3D3B;
    let ad2 = 0xC026BC_u32.wrapping_add(cpu.d[3]);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C02686: move.w $32(a5, d3.w), -(a3)
pub fn i_c02686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0268A;
    cpu.ir = 0x3735;
    let ad2 = cpu.a[5].wrapping_add(0x32).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[3].wrapping_sub(2); cpu.a[3] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C0268A: move.w $2d(a0, d2.l), -(a0)
pub fn i_c0268a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0268E;
    cpu.ir = 0x3130;
    let ad2 = cpu.a[0].wrapping_add(0x2D).wrapping_add(cpu.d[2]);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_sub(2); cpu.a[0] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C0268E: move.l $2a28(a3), d6
pub fn i_c0268e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02692;
    cpu.ir = 0x2C2B;
    let ad2 = cpu.a[3].wrapping_add(0x2A28);
    let v3 = bus.read32(ad2);
    cpu.d[6] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C02692: move.l -(a6), -(a3)
pub fn i_c02692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02694;
    cpu.ir = 0x2726;
    let ad2 = cpu.a[6].wrapping_sub(4); cpu.a[6] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[3];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[3] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C02694: move.l -(a4), -(a2)
pub fn i_c02694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02696;
    cpu.ir = 0x2524;
    let ad2 = cpu.a[4].wrapping_sub(4); cpu.a[4] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[2];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[2] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C02696: move.l -(a2), -(a1)
pub fn i_c02696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02698;
    cpu.ir = 0x2322;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[1];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[1] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C02698: move.l -(a1), d1
pub fn i_c02698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0269A;
    cpu.ir = 0x2221;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C0269A: move.l (a7)+, d0
pub fn i_c0269a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0269C;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C0269C: move.b (a6)+, d7
pub fn i_c0269c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0269E;
    cpu.ir = 0x1E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C0269E: move.b (a4)+, -(a6)
pub fn i_c0269e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC026A0;
    cpu.ir = 0x1D1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C026A0: move.b (a3)+, d6
pub fn i_c026a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC026A2;
    cpu.ir = 0x1C1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C026A2: move.b (a2)+, -(a5)
pub fn i_c026a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC026A4;
    cpu.ir = 0x1B1A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C026A4: move.b (a1)+, -(a4)
pub fn i_c026a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC026A6;
    cpu.ir = 0x1919;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C026A6: move.b (a0)+, d4
pub fn i_c026a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC026A8;
    cpu.ir = 0x1818;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C026A8: move.b (a7), -(a3)
pub fn i_c026a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC026AA;
    cpu.ir = 0x1717;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[3].wrapping_sub(1); cpu.a[3] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C026AA: move.b (a6), -(a3)
pub fn i_c026aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC026AC;
    cpu.ir = 0x1716;
    let v2 = bus.read8(cpu.a[6]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[3].wrapping_sub(1); cpu.a[3] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C026AC: move.b (a5), d3
pub fn i_c026ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC026AE;
    cpu.ir = 0x1615;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C026AE: move.b (a5), -(a2)
pub fn i_c026ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC026B0;
    cpu.ir = 0x1515;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[2].wrapping_sub(1); cpu.a[2] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C026B0: move.b (a4), d2
pub fn i_c026b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC026B2;
    cpu.ir = 0x1414;
    let v2 = bus.read8(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C026B2: move.b (a3), d2
pub fn i_c026b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC026B4;
    cpu.ir = 0x1413;
    let v2 = bus.read8(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C026B4: move.b (a3), -(a1)
pub fn i_c026b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC026B6;
    cpu.ir = 0x1313;
    let v2 = bus.read8(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[1].wrapping_sub(1); cpu.a[1] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C026B6: move.b (a2), d1
pub fn i_c026b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC026B8;
    cpu.ir = 0x1212;
    let v2 = bus.read8(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C026B8: move.b (a1), d1
pub fn i_c026b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC026BA;
    cpu.ir = 0x1211;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C026BA: move.b (a1), -(a0)
pub fn i_c026ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC026BC;
    cpu.ir = 0x1111;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C026BC: move.b (a0), -(a0)
pub fn i_c026bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC026BE;
    cpu.ir = 0x1110;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C026BE: move.b (a0), d0
pub fn i_c026be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC026C0;
    cpu.ir = 0x1010;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C026C0: ?
pub fn i_c026c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC026C2;
    cpu.ir = 0x403E;
    let v1: u32 = 0x0;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x80) != 0 && (r2 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x80 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    4
}
/// $C026C2: move.w $c026fd(pc, d3.l), -(a6)
pub fn i_c026c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC026C6;
    cpu.ir = 0x3D3B;
    let ad2 = 0xC026FD_u32.wrapping_add(cpu.d[3]);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C026C6: move.w $33(a6, d3.w), -(a3)
pub fn i_c026c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC026CA;
    cpu.ir = 0x3736;
    let ad2 = cpu.a[6].wrapping_add(0x33).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[3].wrapping_sub(2); cpu.a[3] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C026CA: move.w ([$2d2b, a0], d2.l * 8, $2a29), d1
pub fn i_c026ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC026CE;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x2E).wrapping_add(cpu.d[2]);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C026CE: move.l $2a29(a3), -(a6)
pub fn i_c026ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC026D2;
    cpu.ir = 0x2D2B;
    let ad2 = cpu.a[3].wrapping_add(0x2A29);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[6];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[6] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C026D2: move.l -(a7), d4
pub fn i_c026d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC026D4;
    cpu.ir = 0x2827;
    let ad2 = cpu.a[7].wrapping_sub(4); cpu.a[7] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[4] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C026D4: move.l -(a5), d3
pub fn i_c026d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC026D6;
    cpu.ir = 0x2625;
    let ad2 = cpu.a[5].wrapping_sub(4); cpu.a[5] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C026D6: move.l -(a3), d2
pub fn i_c026d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC026D8;
    cpu.ir = 0x2423;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C026D8: move.l -(a2), d1
pub fn i_c026d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC026DA;
    cpu.ir = 0x2222;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C026DA: move.l -(a0), -(a0)
pub fn i_c026da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC026DC;
    cpu.ir = 0x2120;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[0];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[0] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C026DC: move.b (a7)+, -(a7)
pub fn i_c026dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC026DE;
    cpu.ir = 0x1F1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C026DE: move.b (a5)+, d7
pub fn i_c026de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC026E0;
    cpu.ir = 0x1E1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C026E0: move.b (a4)+, -(a6)
pub fn i_c026e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC026E2;
    cpu.ir = 0x1D1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C026E2: move.b (a3)+, -(a5)
pub fn i_c026e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC026E4;
    cpu.ir = 0x1B1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C026E4: move.b (a2)+, d5
pub fn i_c026e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC026E6;
    cpu.ir = 0x1A1A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C026E6: move.b (a1)+, -(a4)
pub fn i_c026e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC026E8;
    cpu.ir = 0x1919;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C026E8: move.b (a0)+, d4
pub fn i_c026e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC026EA;
    cpu.ir = 0x1818;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C026EA: move.b (a7), -(a3)
pub fn i_c026ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC026EC;
    cpu.ir = 0x1717;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[3].wrapping_sub(1); cpu.a[3] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C026EC: move.b (a6), d3
pub fn i_c026ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC026EE;
    cpu.ir = 0x1616;
    let v2 = bus.read8(cpu.a[6]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C026EE: move.b (a5), d3
pub fn i_c026ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC026F0;
    cpu.ir = 0x1615;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C026F0: move.b (a5), -(a2)
pub fn i_c026f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC026F2;
    cpu.ir = 0x1515;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[2].wrapping_sub(1); cpu.a[2] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C026F2: move.b (a4), d2
pub fn i_c026f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC026F4;
    cpu.ir = 0x1414;
    let v2 = bus.read8(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C026F4: move.b (a3), d2
pub fn i_c026f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC026F6;
    cpu.ir = 0x1413;
    let v2 = bus.read8(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C026F6: move.b (a3), -(a1)
pub fn i_c026f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC026F8;
    cpu.ir = 0x1313;
    let v2 = bus.read8(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[1].wrapping_sub(1); cpu.a[1] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C026F8: move.b (a2), d1
pub fn i_c026f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC026FA;
    cpu.ir = 0x1212;
    let v2 = bus.read8(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C026FA: move.b (a1), d1
pub fn i_c026fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC026FC;
    cpu.ir = 0x1211;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C026FC: move.b (a1), -(a0)
pub fn i_c026fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC026FE;
    cpu.ir = 0x1111;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C026FE: move.b (a0), -(a0)
pub fn i_c026fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02700;
    cpu.ir = 0x1110;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C02700: ?
pub fn i_c02700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02702;
    cpu.ir = 0x403F;
    let v1: u32 = 0x0;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x80) != 0 && (r2 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x80 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    4
}
/// $C02702: move.w #$3a39, -(a6)
pub fn i_c02702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02706;
    cpu.ir = 0x3D3C;
    let v1: u32 = 0x3A39;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $C02706: move.w ([$3231302e, a6, d3.w * 4], $aaaaaaaa), -(a3)
pub fn i_c02706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0270A;
    cpu.ir = 0x3736;
    let ad2 = cpu.a[6].wrapping_add(0x33).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[3].wrapping_sub(2); cpu.a[3] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C0270A: move.w $2e(a1, d3.w), d1
pub fn i_c0270a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0270E;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x2E).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C0270E: move.l $2b2a(a4), -(a6)
pub fn i_c0270e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02712;
    cpu.ir = 0x2D2C;
    let ad2 = cpu.a[4].wrapping_add(0x2B2A);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[6];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[6] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C02710: move.l $2928(a2), -(a5)
pub fn i_c02710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02714;
    cpu.ir = 0x2B2A;
    let ad2 = cpu.a[2].wrapping_add(0x2928);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[5];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[5] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C02712: move.l $2726(a0), -(a4)
pub fn i_c02712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02716;
    cpu.ir = 0x2928;
    let ad2 = cpu.a[0].wrapping_add(0x2726);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[4];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[4] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C02714: move.l -(a6), -(a3)
pub fn i_c02714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02716;
    cpu.ir = 0x2726;
    let ad2 = cpu.a[6].wrapping_sub(4); cpu.a[6] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[3];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[3] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C02716: move.l -(a4), -(a2)
pub fn i_c02716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02718;
    cpu.ir = 0x2524;
    let ad2 = cpu.a[4].wrapping_sub(4); cpu.a[4] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[2];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[2] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C02718: move.l -(a2), -(a1)
pub fn i_c02718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0271A;
    cpu.ir = 0x2322;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[1];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[1] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C0271A: move.l -(a1), d1
pub fn i_c0271a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0271C;
    cpu.ir = 0x2221;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C0271C: move.l (a7)+, d0
pub fn i_c0271c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0271E;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C0271E: move.b (a6)+, -(a7)
pub fn i_c0271e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02720;
    cpu.ir = 0x1F1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02720: move.b (a5)+, -(a6)
pub fn i_c02720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02722;
    cpu.ir = 0x1D1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02722: move.b (a3)+, d6
pub fn i_c02722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02724;
    cpu.ir = 0x1C1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02724: move.b (a2)+, -(a5)
pub fn i_c02724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02726;
    cpu.ir = 0x1B1A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02726: move.b (a1)+, d5
pub fn i_c02726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02728;
    cpu.ir = 0x1A19;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02728: move.b (a0)+, -(a4)
pub fn i_c02728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0272A;
    cpu.ir = 0x1918;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C0272A: move.b (a0)+, d4
pub fn i_c0272a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0272C;
    cpu.ir = 0x1818;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C0272C: move.b (a7), -(a3)
pub fn i_c0272c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0272E;
    cpu.ir = 0x1717;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[3].wrapping_sub(1); cpu.a[3] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C0272E: move.b (a6), d3
pub fn i_c0272e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02730;
    cpu.ir = 0x1616;
    let v2 = bus.read8(cpu.a[6]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C02730: move.b (a5), d3
pub fn i_c02730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02732;
    cpu.ir = 0x1615;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C02732: move.b (a4), -(a2)
pub fn i_c02732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02734;
    cpu.ir = 0x1514;
    let v2 = bus.read8(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[2].wrapping_sub(1); cpu.a[2] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C02734: move.b (a4), d2
pub fn i_c02734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02736;
    cpu.ir = 0x1414;
    let v2 = bus.read8(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C02736: move.b (a3), -(a1)
pub fn i_c02736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02738;
    cpu.ir = 0x1313;
    let v2 = bus.read8(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[1].wrapping_sub(1); cpu.a[1] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C02738: move.b (a3), -(a1)
pub fn i_c02738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0273A;
    cpu.ir = 0x1313;
    let v2 = bus.read8(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[1].wrapping_sub(1); cpu.a[1] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C0273A: move.b (a2), d1
pub fn i_c0273a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0273C;
    cpu.ir = 0x1212;
    let v2 = bus.read8(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C0273C: move.b (a2), d1
pub fn i_c0273c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0273E;
    cpu.ir = 0x1212;
    let v2 = bus.read8(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C0273E: move.b (a1), -(a0)
pub fn i_c0273e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02740;
    cpu.ir = 0x1111;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C02740: ?
pub fn i_c02740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02742;
    cpu.ir = 0x403F;
    let v1: u32 = 0x0;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x80) != 0 && (r2 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x80 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    4
}
/// $C02742: move.w #$3a39, -(a6)
pub fn i_c02742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02746;
    cpu.ir = 0x3D3C;
    let v1: u32 = 0x3A39;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $C02746: move.w $3231302f(a6, d3.w * 4), d4
pub fn i_c02746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0274A;
    cpu.ir = 0x3836;
    let ad2 = cpu.a[6].wrapping_add(0x34).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C0274A: move.w $2f(a1, d3.w), d1
pub fn i_c0274a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0274E;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x2F).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C0274E: move.l $2b2a(a5), d7
pub fn i_c0274e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02752;
    cpu.ir = 0x2E2D;
    let ad2 = cpu.a[5].wrapping_add(0x2B2A);
    let v3 = bus.read32(ad2);
    cpu.d[7] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C02752: move.l $2726(a0), -(a4)
pub fn i_c02752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02756;
    cpu.ir = 0x2928;
    let ad2 = cpu.a[0].wrapping_add(0x2726);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[4];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[4] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C02756: move.l -(a5), d3
pub fn i_c02756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02758;
    cpu.ir = 0x2625;
    let ad2 = cpu.a[5].wrapping_sub(4); cpu.a[5] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02758: move.l -(a3), d2
pub fn i_c02758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0275A;
    cpu.ir = 0x2423;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C0275A: move.l -(a1), d1
pub fn i_c0275a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0275C;
    cpu.ir = 0x2221;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C0275C: move.l -(a0), -(a0)
pub fn i_c0275c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0275E;
    cpu.ir = 0x2120;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[0];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[0] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C0275E: move.b (a7)+, -(a7)
pub fn i_c0275e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02760;
    cpu.ir = 0x1F1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02760: move.b (a5)+, d7
pub fn i_c02760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02762;
    cpu.ir = 0x1E1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02762: move.b (a4)+, -(a6)
pub fn i_c02762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02764;
    cpu.ir = 0x1D1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02764: move.b (a3)+, d6
pub fn i_c02764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02766;
    cpu.ir = 0x1C1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02766: move.b (a2)+, -(a5)
pub fn i_c02766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02768;
    cpu.ir = 0x1B1A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02768: move.b (a1)+, d5
pub fn i_c02768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0276A;
    cpu.ir = 0x1A19;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C0276A: move.b (a0)+, -(a4)
pub fn i_c0276a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0276C;
    cpu.ir = 0x1918;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C0276C: move.b (a7), d4
pub fn i_c0276c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0276E;
    cpu.ir = 0x1817;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C0276E: move.b (a7), -(a3)
pub fn i_c0276e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02770;
    cpu.ir = 0x1717;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[3].wrapping_sub(1); cpu.a[3] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C02770: move.b (a6), d3
pub fn i_c02770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02772;
    cpu.ir = 0x1616;
    let v2 = bus.read8(cpu.a[6]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C02772: move.b (a5), -(a2)
pub fn i_c02772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02774;
    cpu.ir = 0x1515;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[2].wrapping_sub(1); cpu.a[2] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C02774: move.b (a4), -(a2)
pub fn i_c02774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02776;
    cpu.ir = 0x1514;
    let v2 = bus.read8(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[2].wrapping_sub(1); cpu.a[2] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C02776: move.b (a4), d2
pub fn i_c02776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02778;
    cpu.ir = 0x1414;
    let v2 = bus.read8(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C02778: move.b (a3), -(a1)
pub fn i_c02778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0277A;
    cpu.ir = 0x1313;
    let v2 = bus.read8(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[1].wrapping_sub(1); cpu.a[1] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C0277A: move.b (a3), -(a1)
pub fn i_c0277a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0277C;
    cpu.ir = 0x1313;
    let v2 = bus.read8(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[1].wrapping_sub(1); cpu.a[1] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C0277C: move.b (a2), d1
pub fn i_c0277c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0277E;
    cpu.ir = 0x1212;
    let v2 = bus.read8(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C0277E: move.b (a2), d1
pub fn i_c0277e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02780;
    cpu.ir = 0x1212;
    let v2 = bus.read8(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C02780: ?
pub fn i_c02780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02782;
    cpu.ir = 0x403F;
    let v1: u32 = 0x0;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x80) != 0 && (r2 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x80 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    4
}
/// $C02782: move.w #$3b39, -(a6)
pub fn i_c02782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02786;
    cpu.ir = 0x3D3C;
    let v1: u32 = 0x3B39;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $C02786: move.w $3332302f(a7, d3.w * 4), d4
pub fn i_c02786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0278A;
    cpu.ir = 0x3837;
    let ad2 = cpu.a[7].wrapping_add(0x34).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C0278A: move.w $2f(a2, d3.w), -(a1)
pub fn i_c0278a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0278E;
    cpu.ir = 0x3332;
    let ad2 = cpu.a[2].wrapping_add(0x2F).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1].wrapping_sub(2); cpu.a[1] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C0278E: move.l $2c2b(a5), d7
pub fn i_c0278e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02792;
    cpu.ir = 0x2E2D;
    let ad2 = cpu.a[5].wrapping_add(0x2C2B);
    let v3 = bus.read32(ad2);
    cpu.d[7] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C02792: move.l $2827(a1), d5
pub fn i_c02792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02796;
    cpu.ir = 0x2A29;
    let ad2 = cpu.a[1].wrapping_add(0x2827);
    let v3 = bus.read32(ad2);
    cpu.d[5] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C02796: move.l -(a5), d3
pub fn i_c02796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02798;
    cpu.ir = 0x2625;
    let ad2 = cpu.a[5].wrapping_sub(4); cpu.a[5] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02798: move.l -(a4), -(a2)
pub fn i_c02798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0279A;
    cpu.ir = 0x2524;
    let ad2 = cpu.a[4].wrapping_sub(4); cpu.a[4] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[2];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[2] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C0279A: move.l -(a2), -(a1)
pub fn i_c0279a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0279C;
    cpu.ir = 0x2322;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[1];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[1] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C0279C: move.l -(a1), -(a0)
pub fn i_c0279c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0279E;
    cpu.ir = 0x2121;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[0];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[0] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C0279E: move.l (a7)+, d0
pub fn i_c0279e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC027A0;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C027A0: move.b (a6)+, -(a7)
pub fn i_c027a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC027A2;
    cpu.ir = 0x1F1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C027A2: move.b (a5)+, -(a6)
pub fn i_c027a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC027A4;
    cpu.ir = 0x1D1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C027A4: move.b (a4)+, d6
pub fn i_c027a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC027A6;
    cpu.ir = 0x1C1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C027A6: move.b (a3)+, -(a5)
pub fn i_c027a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC027A8;
    cpu.ir = 0x1B1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C027A8: move.b (a2)+, d5
pub fn i_c027a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC027AA;
    cpu.ir = 0x1A1A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C027AA: move.b (a1)+, -(a4)
pub fn i_c027aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC027AC;
    cpu.ir = 0x1919;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C027AC: move.b (a0)+, d4
pub fn i_c027ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC027AE;
    cpu.ir = 0x1818;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C027AE: move.b (a7), d4
pub fn i_c027ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC027B0;
    cpu.ir = 0x1817;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C027B0: move.b (a6), -(a3)
pub fn i_c027b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC027B2;
    cpu.ir = 0x1716;
    let v2 = bus.read8(cpu.a[6]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[3].wrapping_sub(1); cpu.a[3] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C027B2: move.b (a6), d3
pub fn i_c027b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC027B4;
    cpu.ir = 0x1616;
    let v2 = bus.read8(cpu.a[6]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C027B4: move.b (a5), -(a2)
pub fn i_c027b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC027B6;
    cpu.ir = 0x1515;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[2].wrapping_sub(1); cpu.a[2] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C027B6: move.b (a4), -(a2)
pub fn i_c027b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC027B8;
    cpu.ir = 0x1514;
    let v2 = bus.read8(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[2].wrapping_sub(1); cpu.a[2] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C027B8: move.b (a4), d2
pub fn i_c027b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC027BA;
    cpu.ir = 0x1414;
    let v2 = bus.read8(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C027BA: move.b (a3), -(a1)
pub fn i_c027ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC027BC;
    cpu.ir = 0x1313;
    let v2 = bus.read8(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[1].wrapping_sub(1); cpu.a[1] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C027BC: move.b (a3), -(a1)
pub fn i_c027bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC027BE;
    cpu.ir = 0x1313;
    let v2 = bus.read8(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[1].wrapping_sub(1); cpu.a[1] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C027BE: move.b (a2), d1
pub fn i_c027be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC027C0;
    cpu.ir = 0x1212;
    let v2 = bus.read8(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C027C0: ?
pub fn i_c027c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC027C2;
    cpu.ir = 0x403F;
    let v1: u32 = 0x0;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x80) != 0 && (r2 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x80 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    4
}
/// $C027C2: move.w #$3b39, -(a6)
pub fn i_c027c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC027C6;
    cpu.ir = 0x3D3C;
    let v1: u32 = 0x3B39;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $C027C6: move.w $34(a7, d3.w), d4
pub fn i_c027c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC027CA;
    cpu.ir = 0x3837;
    let ad2 = cpu.a[7].wrapping_add(0x34).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C027CA: move.w $2f2e2d2c(a2, d3.w), -(a1)
pub fn i_c027ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC027CE;
    cpu.ir = 0x3332;
    let ad2 = cpu.a[2].wrapping_add(0x30).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1].wrapping_sub(2); cpu.a[1] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C027CE: move.l $2d2c(a6), -(a7)
pub fn i_c027ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC027D2;
    cpu.ir = 0x2F2E;
    let ad2 = cpu.a[6].wrapping_add(0x2D2C);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[7];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[7] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C027D2: move.l $2928(a2), -(a5)
pub fn i_c027d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC027D6;
    cpu.ir = 0x2B2A;
    let ad2 = cpu.a[2].wrapping_add(0x2928);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[5];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[5] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C027D6: move.l -(a6), -(a3)
pub fn i_c027d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC027D8;
    cpu.ir = 0x2726;
    let ad2 = cpu.a[6].wrapping_sub(4); cpu.a[6] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[3];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[3] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C027D8: move.l -(a4), -(a2)
pub fn i_c027d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC027DA;
    cpu.ir = 0x2524;
    let ad2 = cpu.a[4].wrapping_sub(4); cpu.a[4] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[2];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[2] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C027DA: move.l -(a3), d2
pub fn i_c027da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC027DC;
    cpu.ir = 0x2423;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C027DC: move.l -(a1), d1
pub fn i_c027dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC027DE;
    cpu.ir = 0x2221;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C027DE: move.l -(a0), -(a0)
pub fn i_c027de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC027E0;
    cpu.ir = 0x2120;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[0];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[0] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C027E0: move.b (a7)+, -(a7)
pub fn i_c027e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC027E2;
    cpu.ir = 0x1F1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C027E2: move.b (a6)+, d7
pub fn i_c027e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC027E4;
    cpu.ir = 0x1E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C027E4: move.b (a4)+, -(a6)
pub fn i_c027e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC027E6;
    cpu.ir = 0x1D1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C027E6: move.b (a3)+, d6
pub fn i_c027e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC027E8;
    cpu.ir = 0x1C1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C027E8: move.b (a2)+, -(a5)
pub fn i_c027e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC027EA;
    cpu.ir = 0x1B1A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C027EA: move.b (a1)+, d5
pub fn i_c027ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC027EC;
    cpu.ir = 0x1A19;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C027EC: move.b (a1)+, -(a4)
pub fn i_c027ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC027EE;
    cpu.ir = 0x1919;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C027EE: move.b (a0)+, d4
pub fn i_c027ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC027F0;
    cpu.ir = 0x1818;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C027F0: move.b (a7), -(a3)
pub fn i_c027f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC027F2;
    cpu.ir = 0x1717;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[3].wrapping_sub(1); cpu.a[3] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C027F2: move.b (a6), -(a3)
pub fn i_c027f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC027F4;
    cpu.ir = 0x1716;
    let v2 = bus.read8(cpu.a[6]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[3].wrapping_sub(1); cpu.a[3] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C027F4: move.b (a6), d3
pub fn i_c027f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC027F6;
    cpu.ir = 0x1616;
    let v2 = bus.read8(cpu.a[6]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C027F6: move.b (a5), -(a2)
pub fn i_c027f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC027F8;
    cpu.ir = 0x1515;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[2].wrapping_sub(1); cpu.a[2] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C027F8: move.b (a4), -(a2)
pub fn i_c027f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC027FA;
    cpu.ir = 0x1514;
    let v2 = bus.read8(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[2].wrapping_sub(1); cpu.a[2] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C027FA: move.b (a4), d2
pub fn i_c027fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC027FC;
    cpu.ir = 0x1414;
    let v2 = bus.read8(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C027FC: move.b (a3), -(a1)
pub fn i_c027fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC027FE;
    cpu.ir = 0x1313;
    let v2 = bus.read8(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[1].wrapping_sub(1); cpu.a[1] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C027FE: move.b (a3), -(a1)
pub fn i_c027fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02800;
    cpu.ir = 0x1313;
    let v2 = bus.read8(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[1].wrapping_sub(1); cpu.a[1] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C02800: ?
pub fn i_c02800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02802;
    cpu.ir = 0x403F;
    let v1: u32 = 0x0;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x80) != 0 && (r2 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x80 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    4
}
/// $C02802: move.w #$3b3a, -(a6)
pub fn i_c02802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02806;
    cpu.ir = 0x3D3C;
    let v1: u32 = 0x3B3A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $C02806: move.w $35(a7, d3.w), d4
pub fn i_c02806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0280A;
    cpu.ir = 0x3837;
    let ad2 = cpu.a[7].wrapping_add(0x35).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C0280A: move.w $2f2e2d2c(a3, d3.w), d2
pub fn i_c0280a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0280E;
    cpu.ir = 0x3433;
    let ad2 = cpu.a[3].wrapping_add(0x30).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C0280E: move.l $2d2c(a6), -(a7)
pub fn i_c0280e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02812;
    cpu.ir = 0x2F2E;
    let ad2 = cpu.a[6].wrapping_add(0x2D2C);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[7];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[7] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C02812: move.l $2928(a2), -(a5)
pub fn i_c02812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02816;
    cpu.ir = 0x2B2A;
    let ad2 = cpu.a[2].wrapping_add(0x2928);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[5];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[5] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C02816: move.l -(a7), -(a3)
pub fn i_c02816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02818;
    cpu.ir = 0x2727;
    let ad2 = cpu.a[7].wrapping_sub(4); cpu.a[7] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[3];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[3] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C02818: move.l -(a5), d3
pub fn i_c02818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0281A;
    cpu.ir = 0x2625;
    let ad2 = cpu.a[5].wrapping_sub(4); cpu.a[5] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C0281A: move.l -(a3), d2
pub fn i_c0281a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0281C;
    cpu.ir = 0x2423;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C0281C: move.l -(a2), -(a1)
pub fn i_c0281c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0281E;
    cpu.ir = 0x2322;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[1];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[1] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C0281E: move.l -(a1), -(a0)
pub fn i_c0281e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02820;
    cpu.ir = 0x2121;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[0];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[0] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C02820: move.l (a7)+, d0
pub fn i_c02820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02822;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C02822: move.b (a6)+, -(a7)
pub fn i_c02822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02824;
    cpu.ir = 0x1F1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02824: move.b (a5)+, d7
pub fn i_c02824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02826;
    cpu.ir = 0x1E1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02826: move.b (a4)+, -(a6)
pub fn i_c02826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02828;
    cpu.ir = 0x1D1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02828: move.b (a3)+, -(a5)
pub fn i_c02828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0282A;
    cpu.ir = 0x1B1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C0282A: move.b (a2)+, -(a5)
pub fn i_c0282a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0282C;
    cpu.ir = 0x1B1A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C0282C: move.b (a1)+, d5
pub fn i_c0282c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0282E;
    cpu.ir = 0x1A19;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C0282E: move.b (a0)+, -(a4)
pub fn i_c0282e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02830;
    cpu.ir = 0x1918;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02830: move.b (a0)+, d4
pub fn i_c02830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02832;
    cpu.ir = 0x1818;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02832: move.b (a7), -(a3)
pub fn i_c02832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02834;
    cpu.ir = 0x1717;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[3].wrapping_sub(1); cpu.a[3] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C02834: move.b (a6), d3
pub fn i_c02834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02836;
    cpu.ir = 0x1616;
    let v2 = bus.read8(cpu.a[6]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C02836: move.b (a5), d3
pub fn i_c02836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02838;
    cpu.ir = 0x1615;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C02838: move.b (a5), -(a2)
pub fn i_c02838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0283A;
    cpu.ir = 0x1515;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[2].wrapping_sub(1); cpu.a[2] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C0283A: move.b (a4), -(a2)
pub fn i_c0283a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0283C;
    cpu.ir = 0x1514;
    let v2 = bus.read8(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[2].wrapping_sub(1); cpu.a[2] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C0283C: move.b (a4), d2
pub fn i_c0283c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0283E;
    cpu.ir = 0x1414;
    let v2 = bus.read8(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C0283E: move.b (a3), -(a1)
pub fn i_c0283e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02840;
    cpu.ir = 0x1313;
    let v2 = bus.read8(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[1].wrapping_sub(1); cpu.a[1] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C02840: ?
pub fn i_c02840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02842;
    cpu.ir = 0x403F;
    let v1: u32 = 0x0;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x80) != 0 && (r2 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x80 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    4
}
/// $C02842: move.w #$3b3a, d7
pub fn i_c02842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02846;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x3B3A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C02846: move.w $35(a7, d3.w), -(a4)
pub fn i_c02846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0284A;
    cpu.ir = 0x3937;
    let ad2 = cpu.a[7].wrapping_add(0x35).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(2); cpu.a[4] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C0284A: move.w $31(a3, d3.w), d2
pub fn i_c0284a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0284E;
    cpu.ir = 0x3433;
    let ad2 = cpu.a[3].wrapping_add(0x31).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C0284E: move.w $2e2d(a7), d0
pub fn i_c0284e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02852;
    cpu.ir = 0x302F;
    let ad2 = cpu.a[7].wrapping_add(0x2E2D);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C02852: move.l $2a29(a3), d6
pub fn i_c02852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02856;
    cpu.ir = 0x2C2B;
    let ad2 = cpu.a[3].wrapping_add(0x2A29);
    let v3 = bus.read32(ad2);
    cpu.d[6] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C02856: move.l -(a7), d4
pub fn i_c02856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02858;
    cpu.ir = 0x2827;
    let ad2 = cpu.a[7].wrapping_sub(4); cpu.a[7] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[4] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02858: move.l -(a6), d3
pub fn i_c02858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0285A;
    cpu.ir = 0x2626;
    let ad2 = cpu.a[6].wrapping_sub(4); cpu.a[6] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C0285A: move.l -(a4), -(a2)
pub fn i_c0285a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0285C;
    cpu.ir = 0x2524;
    let ad2 = cpu.a[4].wrapping_sub(4); cpu.a[4] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[2];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[2] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C0285C: move.l -(a3), -(a1)
pub fn i_c0285c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0285E;
    cpu.ir = 0x2323;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[1];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[1] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C0285E: move.l -(a1), d1
pub fn i_c0285e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02860;
    cpu.ir = 0x2221;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02860: move.l -(a0), -(a0)
pub fn i_c02860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02862;
    cpu.ir = 0x2120;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[0];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[0] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C02862: move.b (a7)+, -(a7)
pub fn i_c02862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02864;
    cpu.ir = 0x1F1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02864: move.b (a6)+, d7
pub fn i_c02864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02866;
    cpu.ir = 0x1E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02866: move.b (a5)+, -(a6)
pub fn i_c02866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02868;
    cpu.ir = 0x1D1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02868: move.b (a4)+, d6
pub fn i_c02868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0286A;
    cpu.ir = 0x1C1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C0286A: move.b (a3)+, -(a5)
pub fn i_c0286a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0286C;
    cpu.ir = 0x1B1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C0286C: move.b (a2)+, d5
pub fn i_c0286c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0286E;
    cpu.ir = 0x1A1A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C0286E: move.b (a1)+, -(a4)
pub fn i_c0286e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02870;
    cpu.ir = 0x1919;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02870: move.b (a0)+, -(a4)
pub fn i_c02870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02872;
    cpu.ir = 0x1918;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02872: move.b (a7), d4
pub fn i_c02872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02874;
    cpu.ir = 0x1817;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C02874: move.b (a7), -(a3)
pub fn i_c02874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02876;
    cpu.ir = 0x1717;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[3].wrapping_sub(1); cpu.a[3] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C02876: move.b (a6), d3
pub fn i_c02876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02878;
    cpu.ir = 0x1616;
    let v2 = bus.read8(cpu.a[6]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C02878: move.b (a5), d3
pub fn i_c02878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0287A;
    cpu.ir = 0x1615;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C0287A: move.b (a5), -(a2)
pub fn i_c0287a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0287C;
    cpu.ir = 0x1515;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[2].wrapping_sub(1); cpu.a[2] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C0287C: move.b (a4), d2
pub fn i_c0287c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0287E;
    cpu.ir = 0x1414;
    let v2 = bus.read8(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C0287E: move.b (a4), d2
pub fn i_c0287e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02880;
    cpu.ir = 0x1414;
    let v2 = bus.read8(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C02880: ?
pub fn i_c02880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02882;
    cpu.ir = 0x403F;
    let v1: u32 = 0x0;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x80) != 0 && (r2 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x80 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    4
}
/// $C02882: move.w #$3b3a, d7
pub fn i_c02882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02886;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x3B3A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C02886: move.w $3735.w, -(a4)
pub fn i_c02886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0288A;
    cpu.ir = 0x3938;
    let v2 = bus.read16(0x3735) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[4].wrapping_sub(2); cpu.a[4] = ad3;
    bus.write16(ad3, v2 as u16);
    18
}
/// $C0288A: move.w $31(a3, d3.w), d2
pub fn i_c0288a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0288E;
    cpu.ir = 0x3433;
    let ad2 = cpu.a[3].wrapping_add(0x31).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C0288E: move.w $2e2d(a7), d0
pub fn i_c0288e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02892;
    cpu.ir = 0x302F;
    let ad2 = cpu.a[7].wrapping_add(0x2E2D);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C02892: move.l $2a29(a3), d6
pub fn i_c02892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02896;
    cpu.ir = 0x2C2B;
    let ad2 = cpu.a[3].wrapping_add(0x2A29);
    let v3 = bus.read32(ad2);
    cpu.d[6] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C02896: move.l $2726(a0), -(a4)
pub fn i_c02896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0289A;
    cpu.ir = 0x2928;
    let ad2 = cpu.a[0].wrapping_add(0x2726);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[4];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[4] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C0289A: move.l -(a5), -(a2)
pub fn i_c0289a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0289C;
    cpu.ir = 0x2525;
    let ad2 = cpu.a[5].wrapping_sub(4); cpu.a[5] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[2];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[2] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C0289C: move.l -(a3), d2
pub fn i_c0289c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0289E;
    cpu.ir = 0x2423;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C0289E: move.l -(a2), -(a1)
pub fn i_c0289e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC028A0;
    cpu.ir = 0x2322;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[1];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[1] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C028A0: move.l -(a1), -(a0)
pub fn i_c028a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC028A2;
    cpu.ir = 0x2121;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[0];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[0] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C028A2: move.l (a7)+, d0
pub fn i_c028a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC028A4;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C028A4: move.b (a6)+, -(a7)
pub fn i_c028a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC028A6;
    cpu.ir = 0x1F1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C028A6: move.b (a5)+, d7
pub fn i_c028a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC028A8;
    cpu.ir = 0x1E1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C028A8: move.b (a4)+, -(a6)
pub fn i_c028a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC028AA;
    cpu.ir = 0x1D1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C028AA: move.b (a3)+, d6
pub fn i_c028aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC028AC;
    cpu.ir = 0x1C1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C028AC: move.b (a2)+, -(a5)
pub fn i_c028ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC028AE;
    cpu.ir = 0x1B1A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C028AE: move.b (a2)+, d5
pub fn i_c028ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC028B0;
    cpu.ir = 0x1A1A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C028B0: move.b (a1)+, -(a4)
pub fn i_c028b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC028B2;
    cpu.ir = 0x1919;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C028B2: move.b (a0)+, d4
pub fn i_c028b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC028B4;
    cpu.ir = 0x1818;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C028B4: move.b (a7), d4
pub fn i_c028b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC028B6;
    cpu.ir = 0x1817;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C028B6: move.b (a7), -(a3)
pub fn i_c028b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC028B8;
    cpu.ir = 0x1717;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[3].wrapping_sub(1); cpu.a[3] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C028B8: move.b (a6), d3
pub fn i_c028b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC028BA;
    cpu.ir = 0x1616;
    let v2 = bus.read8(cpu.a[6]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C028BA: move.b (a5), d3
pub fn i_c028ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC028BC;
    cpu.ir = 0x1615;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C028BC: move.b (a5), -(a2)
pub fn i_c028bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC028BE;
    cpu.ir = 0x1515;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[2].wrapping_sub(1); cpu.a[2] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C028BE: move.b (a4), d2
pub fn i_c028be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC028C0;
    cpu.ir = 0x1414;
    let v2 = bus.read8(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C028C0: ?
pub fn i_c028c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC028C2;
    cpu.ir = 0x403F;
    let v1: u32 = 0x0;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x80) != 0 && (r2 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x80 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    4
}
/// $C028C2: ?
pub fn i_c028c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC028C4;
    cpu.ir = 0x3E3D;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C028C4: move.w $c061fe(pc), -(a5)
pub fn i_c028c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC028C8;
    cpu.ir = 0x3B3A;
    let v2 = bus.read16(0xC061FE) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[5].wrapping_sub(2); cpu.a[5] = ad3;
    bus.write16(ad3, v2 as u16);
    18
}
/// $C028C8: move.w $33323030(a6, d3.w * 4), -(a3)
pub fn i_c028c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC028CC;
    cpu.ir = 0x3736;
    let ad2 = cpu.a[6].wrapping_add(0x34).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[3].wrapping_sub(2); cpu.a[3] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C028CC: move.w $30(a2, d3.w), -(a1)
pub fn i_c028cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC028D0;
    cpu.ir = 0x3332;
    let ad2 = cpu.a[2].wrapping_add(0x30).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1].wrapping_sub(2); cpu.a[1] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C028D0: move.l $2d2c(a6), -(a7)
pub fn i_c028d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC028D4;
    cpu.ir = 0x2F2E;
    let ad2 = cpu.a[6].wrapping_add(0x2D2C);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[7];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[7] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C028D4: move.l $2928(a2), -(a5)
pub fn i_c028d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC028D8;
    cpu.ir = 0x2B2A;
    let ad2 = cpu.a[2].wrapping_add(0x2928);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[5];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[5] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C028D8: move.l -(a7), d4
pub fn i_c028d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC028DA;
    cpu.ir = 0x2827;
    let ad2 = cpu.a[7].wrapping_sub(4); cpu.a[7] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[4] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C028DA: move.l -(a5), d3
pub fn i_c028da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC028DC;
    cpu.ir = 0x2625;
    let ad2 = cpu.a[5].wrapping_sub(4); cpu.a[5] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C028DC: move.l -(a4), -(a2)
pub fn i_c028dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC028DE;
    cpu.ir = 0x2524;
    let ad2 = cpu.a[4].wrapping_sub(4); cpu.a[4] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[2];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[2] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C028DE: move.l -(a2), -(a1)
pub fn i_c028de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC028E0;
    cpu.ir = 0x2322;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[1];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[1] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C028E0: move.l -(a1), d1
pub fn i_c028e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC028E2;
    cpu.ir = 0x2221;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C028E2: move.l -(a0), -(a0)
pub fn i_c028e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC028E4;
    cpu.ir = 0x2120;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[0];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[0] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C028E4: move.b (a7)+, -(a7)
pub fn i_c028e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC028E6;
    cpu.ir = 0x1F1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C028E6: move.b (a6)+, d7
pub fn i_c028e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC028E8;
    cpu.ir = 0x1E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C028E8: move.b (a5)+, -(a6)
pub fn i_c028e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC028EA;
    cpu.ir = 0x1D1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C028EA: move.b (a4)+, d6
pub fn i_c028ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC028EC;
    cpu.ir = 0x1C1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C028EC: move.b (a3)+, -(a5)
pub fn i_c028ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC028EE;
    cpu.ir = 0x1B1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C028EE: move.b (a2)+, -(a5)
pub fn i_c028ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC028F0;
    cpu.ir = 0x1B1A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C028F0: move.b (a1)+, d5
pub fn i_c028f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC028F2;
    cpu.ir = 0x1A19;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C028F2: move.b (a1)+, -(a4)
pub fn i_c028f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC028F4;
    cpu.ir = 0x1919;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C028F4: move.b (a0)+, d4
pub fn i_c028f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC028F6;
    cpu.ir = 0x1818;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C028F6: move.b (a7), -(a3)
pub fn i_c028f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC028F8;
    cpu.ir = 0x1717;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[3].wrapping_sub(1); cpu.a[3] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C028F8: move.b (a6), -(a3)
pub fn i_c028f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC028FA;
    cpu.ir = 0x1716;
    let v2 = bus.read8(cpu.a[6]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[3].wrapping_sub(1); cpu.a[3] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C028FA: move.b (a6), d3
pub fn i_c028fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC028FC;
    cpu.ir = 0x1616;
    let v2 = bus.read8(cpu.a[6]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C028FC: move.b (a5), d3
pub fn i_c028fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC028FE;
    cpu.ir = 0x1615;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C028FE: move.b (a5), -(a2)
pub fn i_c028fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02900;
    cpu.ir = 0x1515;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[2].wrapping_sub(1); cpu.a[2] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C02900: ?
pub fn i_c02900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02902;
    cpu.ir = 0x403F;
    let v1: u32 = 0x0;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x80) != 0 && (r2 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x80 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    4
}
/// $C02902: ?
pub fn i_c02902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02904;
    cpu.ir = 0x3E3D;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C02904: move.w $c0623e(pc), -(a5)
pub fn i_c02904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02908;
    cpu.ir = 0x3B3A;
    let v2 = bus.read16(0xC0623E) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[5].wrapping_sub(2); cpu.a[5] = ad3;
    bus.write16(ad3, v2 as u16);
    18
}
/// $C02908: move.w $33323130(a6, d3.w * 4), -(a3)
pub fn i_c02908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0290C;
    cpu.ir = 0x3736;
    let ad2 = cpu.a[6].wrapping_add(0x34).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[3].wrapping_sub(2); cpu.a[3] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C0290C: move.w $2f2e2d2c(a2, d3.w), -(a1)
pub fn i_c0290c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02910;
    cpu.ir = 0x3332;
    let ad2 = cpu.a[2].wrapping_add(0x30).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1].wrapping_sub(2); cpu.a[1] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C02910: move.l $2d2c(a6), -(a7)
pub fn i_c02910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02914;
    cpu.ir = 0x2F2E;
    let ad2 = cpu.a[6].wrapping_add(0x2D2C);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[7];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[7] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C02914: move.l $2a29(a2), -(a5)
pub fn i_c02914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02918;
    cpu.ir = 0x2B2A;
    let ad2 = cpu.a[2].wrapping_add(0x2A29);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[5];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[5] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C02918: move.l -(a7), d4
pub fn i_c02918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0291A;
    cpu.ir = 0x2827;
    let ad2 = cpu.a[7].wrapping_sub(4); cpu.a[7] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[4] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C0291A: move.l -(a6), -(a3)
pub fn i_c0291a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0291C;
    cpu.ir = 0x2726;
    let ad2 = cpu.a[6].wrapping_sub(4); cpu.a[6] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[3];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[3] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C0291C: move.l -(a4), -(a2)
pub fn i_c0291c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0291E;
    cpu.ir = 0x2524;
    let ad2 = cpu.a[4].wrapping_sub(4); cpu.a[4] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[2];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[2] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C0291E: move.l -(a3), d2
pub fn i_c0291e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02920;
    cpu.ir = 0x2423;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02920: move.l -(a2), d1
pub fn i_c02920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02922;
    cpu.ir = 0x2222;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02922: move.l -(a1), -(a0)
pub fn i_c02922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02924;
    cpu.ir = 0x2121;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[0];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[0] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C02924: move.l (a7)+, d0
pub fn i_c02924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02926;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C02926: move.b (a6)+, -(a7)
pub fn i_c02926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02928;
    cpu.ir = 0x1F1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02928: move.b (a5)+, d7
pub fn i_c02928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0292A;
    cpu.ir = 0x1E1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C0292A: move.b (a4)+, -(a6)
pub fn i_c0292a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0292C;
    cpu.ir = 0x1D1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C0292C: move.b (a3)+, d6
pub fn i_c0292c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0292E;
    cpu.ir = 0x1C1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C0292E: move.b (a3)+, -(a5)
pub fn i_c0292e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02930;
    cpu.ir = 0x1B1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02930: move.b (a2)+, d5
pub fn i_c02930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02932;
    cpu.ir = 0x1A1A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02932: move.b (a1)+, -(a4)
pub fn i_c02932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02934;
    cpu.ir = 0x1919;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02934: move.b (a0)+, -(a4)
pub fn i_c02934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02936;
    cpu.ir = 0x1918;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02936: move.b (a0)+, d4
pub fn i_c02936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02938;
    cpu.ir = 0x1818;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02938: move.b (a7), -(a3)
pub fn i_c02938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0293A;
    cpu.ir = 0x1717;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[3].wrapping_sub(1); cpu.a[3] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C0293A: move.b (a6), -(a3)
pub fn i_c0293a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0293C;
    cpu.ir = 0x1716;
    let v2 = bus.read8(cpu.a[6]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[3].wrapping_sub(1); cpu.a[3] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C0293C: move.b (a6), d3
pub fn i_c0293c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0293E;
    cpu.ir = 0x1616;
    let v2 = bus.read8(cpu.a[6]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C0293E: move.b (a5), -(a2)
pub fn i_c0293e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02940;
    cpu.ir = 0x1515;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[2].wrapping_sub(1); cpu.a[2] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C02940: ?
pub fn i_c02940(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02942;
    cpu.ir = 0x403F;
    let v1: u32 = 0x0;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x80) != 0 && (r2 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x80 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    4
}
/// $C02942: ?
pub fn i_c02942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02944;
    cpu.ir = 0x3E3D;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C02944: move.w $37363534(a16, d3.l), d6
pub fn i_c02944(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02948;
    cpu.ir = 0x3C3B;
    let ad2 = 0xC0297E_u32.wrapping_add(cpu.d[3]);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C02948: move.w $33323130(a6, d3.w * 4), -(a3)
pub fn i_c02948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0294C;
    cpu.ir = 0x3736;
    let ad2 = cpu.a[6].wrapping_add(0x34).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[3].wrapping_sub(2); cpu.a[3] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C0294C: move.w $2f2e2e2d(a2, d3.w), -(a1)
pub fn i_c0294c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02950;
    cpu.ir = 0x3332;
    let ad2 = cpu.a[2].wrapping_add(0x30).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1].wrapping_sub(2); cpu.a[1] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C02950: move.l $2e2d(a6), -(a7)
pub fn i_c02950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02954;
    cpu.ir = 0x2F2E;
    let ad2 = cpu.a[6].wrapping_add(0x2E2D);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[7];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[7] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C02954: move.l $2a29(a3), d6
pub fn i_c02954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02958;
    cpu.ir = 0x2C2B;
    let ad2 = cpu.a[3].wrapping_add(0x2A29);
    let v3 = bus.read32(ad2);
    cpu.d[6] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C02958: move.l $2726(a0), -(a4)
pub fn i_c02958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0295C;
    cpu.ir = 0x2928;
    let ad2 = cpu.a[0].wrapping_add(0x2726);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[4];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[4] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C0295C: move.l -(a5), d3
pub fn i_c0295c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0295E;
    cpu.ir = 0x2625;
    let ad2 = cpu.a[5].wrapping_sub(4); cpu.a[5] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C0295E: move.l -(a4), d2
pub fn i_c0295e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02960;
    cpu.ir = 0x2424;
    let ad2 = cpu.a[4].wrapping_sub(4); cpu.a[4] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02960: move.l -(a2), -(a1)
pub fn i_c02960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02962;
    cpu.ir = 0x2322;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[1];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[1] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C02962: move.l -(a1), d1
pub fn i_c02962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02964;
    cpu.ir = 0x2221;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02964: move.l -(a0), -(a0)
pub fn i_c02964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02966;
    cpu.ir = 0x2120;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[0];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[0] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C02966: move.b (a7)+, -(a7)
pub fn i_c02966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02968;
    cpu.ir = 0x1F1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02968: move.b (a6)+, d7
pub fn i_c02968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0296A;
    cpu.ir = 0x1E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C0296A: move.b (a5)+, -(a6)
pub fn i_c0296a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0296C;
    cpu.ir = 0x1D1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C0296C: move.b (a4)+, d6
pub fn i_c0296c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0296E;
    cpu.ir = 0x1C1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C0296E: move.b (a3)+, d6
pub fn i_c0296e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02970;
    cpu.ir = 0x1C1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02970: move.b (a2)+, -(a5)
pub fn i_c02970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02972;
    cpu.ir = 0x1B1A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02972: move.b (a2)+, d5
pub fn i_c02972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02974;
    cpu.ir = 0x1A1A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02974: move.b (a1)+, -(a4)
pub fn i_c02974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02976;
    cpu.ir = 0x1919;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02976: move.b (a0)+, d4
pub fn i_c02976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02978;
    cpu.ir = 0x1818;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02978: move.b (a7), d4
pub fn i_c02978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0297A;
    cpu.ir = 0x1817;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C0297A: move.b (a7), -(a3)
pub fn i_c0297a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0297C;
    cpu.ir = 0x1717;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[3].wrapping_sub(1); cpu.a[3] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C0297C: move.b (a6), -(a3)
pub fn i_c0297c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0297E;
    cpu.ir = 0x1716;
    let v2 = bus.read8(cpu.a[6]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[3].wrapping_sub(1); cpu.a[3] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C0297E: move.b (a6), d3
pub fn i_c0297e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02980;
    cpu.ir = 0x1616;
    let v2 = bus.read8(cpu.a[6]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C02980: ?
pub fn i_c02980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02982;
    cpu.ir = 0x403F;
    let v1: u32 = 0x0;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x80) != 0 && (r2 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x80 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    4
}
/// $C02982: ?
pub fn i_c02982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02984;
    cpu.ir = 0x3E3D;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C02984: move.w $c029bf(pc, d3.l), d6
pub fn i_c02984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02988;
    cpu.ir = 0x3C3B;
    let ad2 = 0xC029BF_u32.wrapping_add(cpu.d[3]);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C02988: move.w $35(a7, d3.w), d4
pub fn i_c02988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0298C;
    cpu.ir = 0x3837;
    let ad2 = cpu.a[7].wrapping_add(0x35).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C0298C: move.w $31(a3, d3.w), d2
pub fn i_c0298c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02990;
    cpu.ir = 0x3433;
    let ad2 = cpu.a[3].wrapping_add(0x31).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C02990: move.w $2e2d(a7), d0
pub fn i_c02990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02994;
    cpu.ir = 0x302F;
    let ad2 = cpu.a[7].wrapping_add(0x2E2D);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C02994: move.l $2b2a(a3), d6
pub fn i_c02994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02998;
    cpu.ir = 0x2C2B;
    let ad2 = cpu.a[3].wrapping_add(0x2B2A);
    let v3 = bus.read32(ad2);
    cpu.d[6] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C02998: move.l $2827(a0), -(a4)
pub fn i_c02998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0299C;
    cpu.ir = 0x2928;
    let ad2 = cpu.a[0].wrapping_add(0x2827);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[4];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[4] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C0299C: move.l -(a5), d3
pub fn i_c0299c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0299E;
    cpu.ir = 0x2625;
    let ad2 = cpu.a[5].wrapping_sub(4); cpu.a[5] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C0299E: move.l -(a4), -(a2)
pub fn i_c0299e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC029A0;
    cpu.ir = 0x2524;
    let ad2 = cpu.a[4].wrapping_sub(4); cpu.a[4] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[2];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[2] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C029A0: move.l -(a3), -(a1)
pub fn i_c029a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC029A2;
    cpu.ir = 0x2323;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[1];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[1] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C029A2: move.l -(a2), d1
pub fn i_c029a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC029A4;
    cpu.ir = 0x2222;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C029A4: move.l -(a1), -(a0)
pub fn i_c029a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC029A6;
    cpu.ir = 0x2121;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[0];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[0] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C029A6: move.l (a7)+, d0
pub fn i_c029a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC029A8;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C029A8: move.b (a6)+, -(a7)
pub fn i_c029a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC029AA;
    cpu.ir = 0x1F1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C029AA: move.b (a5)+, d7
pub fn i_c029aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC029AC;
    cpu.ir = 0x1E1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C029AC: move.b (a5)+, -(a6)
pub fn i_c029ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC029AE;
    cpu.ir = 0x1D1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C029AE: move.b (a4)+, d6
pub fn i_c029ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC029B0;
    cpu.ir = 0x1C1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C029B0: move.b (a3)+, -(a5)
pub fn i_c029b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC029B2;
    cpu.ir = 0x1B1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C029B2: move.b (a2)+, d5
pub fn i_c029b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC029B4;
    cpu.ir = 0x1A1A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C029B4: move.b (a1)+, d5
pub fn i_c029b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC029B6;
    cpu.ir = 0x1A19;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C029B6: move.b (a1)+, -(a4)
pub fn i_c029b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC029B8;
    cpu.ir = 0x1919;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C029B8: move.b (a0)+, d4
pub fn i_c029b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC029BA;
    cpu.ir = 0x1818;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C029BA: move.b (a7), d4
pub fn i_c029ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC029BC;
    cpu.ir = 0x1817;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C029BC: move.b (a7), -(a3)
pub fn i_c029bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC029BE;
    cpu.ir = 0x1717;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[3].wrapping_sub(1); cpu.a[3] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C029BE: move.b (a6), d3
pub fn i_c029be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC029C0;
    cpu.ir = 0x1616;
    let v2 = bus.read8(cpu.a[6]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C029C0: ?
pub fn i_c029c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC029C2;
    cpu.ir = 0x403F;
    let v1: u32 = 0x0;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x80) != 0 && (r2 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x80 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    4
}
/// $C029C2: ?
pub fn i_c029c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC029C4;
    cpu.ir = 0x3E3D;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C029C4: move.w $c029ff(pc, d3.l), d6
pub fn i_c029c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC029C8;
    cpu.ir = 0x3C3B;
    let ad2 = 0xC029FF_u32.wrapping_add(cpu.d[3]);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C029C8: move.w $35(a7, d3.w), d4
pub fn i_c029c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC029CC;
    cpu.ir = 0x3837;
    let ad2 = cpu.a[7].wrapping_add(0x35).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C029CC: move.w $31(a3, d3.w), d2
pub fn i_c029cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC029D0;
    cpu.ir = 0x3433;
    let ad2 = cpu.a[3].wrapping_add(0x31).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C029D0: move.w $2e2e(a7), d0
pub fn i_c029d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC029D4;
    cpu.ir = 0x302F;
    let ad2 = cpu.a[7].wrapping_add(0x2E2E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C029D4: move.l $2b2a(a4), -(a6)
pub fn i_c029d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC029D8;
    cpu.ir = 0x2D2C;
    let ad2 = cpu.a[4].wrapping_add(0x2B2A);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[6];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[6] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C029D8: move.l $2827(a1), d5
pub fn i_c029d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC029DC;
    cpu.ir = 0x2A29;
    let ad2 = cpu.a[1].wrapping_add(0x2827);
    let v3 = bus.read32(ad2);
    cpu.d[5] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C029DC: move.l -(a6), -(a3)
pub fn i_c029dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC029DE;
    cpu.ir = 0x2726;
    let ad2 = cpu.a[6].wrapping_sub(4); cpu.a[6] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[3];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[3] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C029DE: move.l -(a5), -(a2)
pub fn i_c029de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC029E0;
    cpu.ir = 0x2525;
    let ad2 = cpu.a[5].wrapping_sub(4); cpu.a[5] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[2];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[2] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C029E0: move.l -(a3), d2
pub fn i_c029e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC029E2;
    cpu.ir = 0x2423;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C029E2: move.l -(a2), -(a1)
pub fn i_c029e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC029E4;
    cpu.ir = 0x2322;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[1];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[1] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C029E4: move.l -(a1), d1
pub fn i_c029e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC029E6;
    cpu.ir = 0x2221;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C029E6: move.l -(a0), -(a0)
pub fn i_c029e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC029E8;
    cpu.ir = 0x2120;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[0];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[0] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C029E8: move.b (a7)+, -(a7)
pub fn i_c029e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC029EA;
    cpu.ir = 0x1F1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C029EA: move.b (a6)+, d7
pub fn i_c029ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC029EC;
    cpu.ir = 0x1E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C029EC: move.b (a5)+, d7
pub fn i_c029ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC029EE;
    cpu.ir = 0x1E1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C029EE: move.b (a4)+, -(a6)
pub fn i_c029ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC029F0;
    cpu.ir = 0x1D1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C029F0: move.b (a3)+, d6
pub fn i_c029f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC029F2;
    cpu.ir = 0x1C1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C029F2: move.b (a3)+, -(a5)
pub fn i_c029f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC029F4;
    cpu.ir = 0x1B1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C029F4: move.b (a2)+, d5
pub fn i_c029f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC029F6;
    cpu.ir = 0x1A1A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C029F6: move.b (a1)+, -(a4)
pub fn i_c029f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC029F8;
    cpu.ir = 0x1919;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C029F8: move.b (a0)+, -(a4)
pub fn i_c029f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC029FA;
    cpu.ir = 0x1918;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C029FA: move.b (a0)+, d4
pub fn i_c029fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC029FC;
    cpu.ir = 0x1818;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C029FC: move.b (a7), -(a3)
pub fn i_c029fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC029FE;
    cpu.ir = 0x1717;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[3].wrapping_sub(1); cpu.a[3] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C029FE: move.b (a7), -(a3)
pub fn i_c029fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A00;
    cpu.ir = 0x1717;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[3].wrapping_sub(1); cpu.a[3] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C02A00: ?
pub fn i_c02a00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A02;
    cpu.ir = 0x403F;
    let v1: u32 = 0x0;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x80) != 0 && (r2 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x80 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    4
}
/// $C02A02: ?
pub fn i_c02a02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A04;
    cpu.ir = 0x3E3D;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C02A04: move.w $c02a3f(pc, d3.l), d6
pub fn i_c02a04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A08;
    cpu.ir = 0x3C3B;
    let ad2 = 0xC02A3F_u32.wrapping_add(cpu.d[3]);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C02A08: move.w $35(a7, d3.w), d4
pub fn i_c02a08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A0C;
    cpu.ir = 0x3837;
    let ad2 = cpu.a[7].wrapping_add(0x35).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C02A0C: move.w $31(a3, d3.w), d2
pub fn i_c02a0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A10;
    cpu.ir = 0x3433;
    let ad2 = cpu.a[3].wrapping_add(0x31).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C02A10: move.w ([$2d2c, a0], d2.l * 8, $2c2b), d0
pub fn i_c02a10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A14;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x2E).wrapping_add(cpu.d[2]);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C02A14: move.l $2c2b(a4), -(a6)
pub fn i_c02a14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A18;
    cpu.ir = 0x2D2C;
    let ad2 = cpu.a[4].wrapping_add(0x2C2B);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[6];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[6] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C02A18: move.l $2928(a1), d5
pub fn i_c02a18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A1C;
    cpu.ir = 0x2A29;
    let ad2 = cpu.a[1].wrapping_add(0x2928);
    let v3 = bus.read32(ad2);
    cpu.d[5] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C02A1C: move.l -(a6), -(a3)
pub fn i_c02a1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A1E;
    cpu.ir = 0x2726;
    let ad2 = cpu.a[6].wrapping_sub(4); cpu.a[6] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[3];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[3] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C02A1E: move.l -(a5), d3
pub fn i_c02a1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A20;
    cpu.ir = 0x2625;
    let ad2 = cpu.a[5].wrapping_sub(4); cpu.a[5] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02A20: move.l -(a4), -(a2)
pub fn i_c02a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A22;
    cpu.ir = 0x2524;
    let ad2 = cpu.a[4].wrapping_sub(4); cpu.a[4] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[2];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[2] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C02A22: move.l -(a3), -(a1)
pub fn i_c02a22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A24;
    cpu.ir = 0x2323;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[1];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[1] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C02A24: move.l -(a2), d1
pub fn i_c02a24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A26;
    cpu.ir = 0x2222;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02A26: move.l -(a1), -(a0)
pub fn i_c02a26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A28;
    cpu.ir = 0x2121;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[0];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[0] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C02A28: move.l (a7)+, d0
pub fn i_c02a28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A2A;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C02A2A: move.b (a7)+, -(a7)
pub fn i_c02a2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A2C;
    cpu.ir = 0x1F1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02A2C: move.b (a6)+, d7
pub fn i_c02a2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A2E;
    cpu.ir = 0x1E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02A2E: move.b (a5)+, -(a6)
pub fn i_c02a2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A30;
    cpu.ir = 0x1D1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02A30: move.b (a4)+, d6
pub fn i_c02a30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A32;
    cpu.ir = 0x1C1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02A32: move.b (a3)+, -(a5)
pub fn i_c02a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A34;
    cpu.ir = 0x1B1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02A34: move.b (a2)+, -(a5)
pub fn i_c02a34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A36;
    cpu.ir = 0x1B1A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02A36: move.b (a2)+, d5
pub fn i_c02a36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A38;
    cpu.ir = 0x1A1A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02A38: move.b (a1)+, -(a4)
pub fn i_c02a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A3A;
    cpu.ir = 0x1919;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02A3A: move.b (a0)+, -(a4)
pub fn i_c02a3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A3C;
    cpu.ir = 0x1918;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02A3C: move.b (a0)+, d4
pub fn i_c02a3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A3E;
    cpu.ir = 0x1818;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02A3E: move.b (a7), -(a3)
pub fn i_c02a3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A40;
    cpu.ir = 0x1717;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[3].wrapping_sub(1); cpu.a[3] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C02A40: ?
pub fn i_c02a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A42;
    cpu.ir = 0x403F;
    let v1: u32 = 0x0;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x80) != 0 && (r2 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x80 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    4
}
/// $C02A42: ?
pub fn i_c02a42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A44;
    cpu.ir = 0x3E3D;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C02A44: move.w $c02a7f(pc, d3.l), d6
pub fn i_c02a44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A48;
    cpu.ir = 0x3C3B;
    let ad2 = 0xC02A7F_u32.wrapping_add(cpu.d[3]);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C02A48: move.w $35(a7, d3.w), d4
pub fn i_c02a48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A4C;
    cpu.ir = 0x3837;
    let ad2 = cpu.a[7].wrapping_add(0x35).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C02A4C: move.w ([$31302f2e, a3, d3.w * 2], $2e2d), d2
pub fn i_c02a4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A50;
    cpu.ir = 0x3433;
    let ad2 = cpu.a[3].wrapping_add(0x32).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C02A50: move.w ([$2e2d, a0], d2.l * 8, $2c2b), -(a0)
pub fn i_c02a50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A54;
    cpu.ir = 0x3130;
    let ad2 = cpu.a[0].wrapping_add(0x2E).wrapping_add(cpu.d[2]);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_sub(2); cpu.a[0] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C02A54: move.l $2c2b(a5), d7
pub fn i_c02a54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A58;
    cpu.ir = 0x2E2D;
    let ad2 = cpu.a[5].wrapping_add(0x2C2B);
    let v3 = bus.read32(ad2);
    cpu.d[7] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C02A58: move.l $2928(a2), d5
pub fn i_c02a58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A5C;
    cpu.ir = 0x2A2A;
    let ad2 = cpu.a[2].wrapping_add(0x2928);
    let v3 = bus.read32(ad2);
    cpu.d[5] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C02A5C: move.l -(a7), d4
pub fn i_c02a5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A5E;
    cpu.ir = 0x2827;
    let ad2 = cpu.a[7].wrapping_sub(4); cpu.a[7] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[4] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02A5E: move.l -(a6), d3
pub fn i_c02a5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A60;
    cpu.ir = 0x2626;
    let ad2 = cpu.a[6].wrapping_sub(4); cpu.a[6] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02A60: move.l -(a4), -(a2)
pub fn i_c02a60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A62;
    cpu.ir = 0x2524;
    let ad2 = cpu.a[4].wrapping_sub(4); cpu.a[4] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[2];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[2] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C02A62: move.l -(a3), d2
pub fn i_c02a62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A64;
    cpu.ir = 0x2423;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02A64: move.l -(a2), -(a1)
pub fn i_c02a64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A66;
    cpu.ir = 0x2322;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[1];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[1] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C02A66: move.l -(a1), d1
pub fn i_c02a66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A68;
    cpu.ir = 0x2221;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02A68: move.l -(a0), -(a0)
pub fn i_c02a68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A6A;
    cpu.ir = 0x2120;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[0];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[0] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C02A6A: move.l (a7)+, d0
pub fn i_c02a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A6C;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C02A6C: move.b (a6)+, -(a7)
pub fn i_c02a6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A6E;
    cpu.ir = 0x1F1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02A6E: move.b (a5)+, d7
pub fn i_c02a6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A70;
    cpu.ir = 0x1E1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02A70: move.b (a4)+, -(a6)
pub fn i_c02a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A72;
    cpu.ir = 0x1D1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02A72: move.b (a4)+, d6
pub fn i_c02a72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A74;
    cpu.ir = 0x1C1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02A74: move.b (a3)+, -(a5)
pub fn i_c02a74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A76;
    cpu.ir = 0x1B1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02A76: move.b (a2)+, d5
pub fn i_c02a76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A78;
    cpu.ir = 0x1A1A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02A78: move.b (a1)+, d5
pub fn i_c02a78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A7A;
    cpu.ir = 0x1A19;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02A7A: move.b (a1)+, -(a4)
pub fn i_c02a7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A7C;
    cpu.ir = 0x1919;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02A7C: move.b (a0)+, d4
pub fn i_c02a7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A7E;
    cpu.ir = 0x1818;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02A7E: move.b (a0)+, d4
pub fn i_c02a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A80;
    cpu.ir = 0x1818;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02A80: ?
pub fn i_c02a80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A82;
    cpu.ir = 0x403F;
    let v1: u32 = 0x0;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x80) != 0 && (r2 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x80 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    4
}
/// $C02A82: ?
pub fn i_c02a82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A84;
    cpu.ir = 0x3E3D;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C02A84: move.w $c02abf(pc, d3.l), d6
pub fn i_c02a84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A88;
    cpu.ir = 0x3C3B;
    let ad2 = 0xC02ABF_u32.wrapping_add(cpu.d[3]);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C02A88: move.w $36(a7, d3.w), d4
pub fn i_c02a88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A8C;
    cpu.ir = 0x3837;
    let ad2 = cpu.a[7].wrapping_add(0x36).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C02A8C: move.w ([$3130302f, a4, d3.w * 2], $2e2d), -(a2)
pub fn i_c02a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A90;
    cpu.ir = 0x3534;
    let ad2 = cpu.a[4].wrapping_add(0x32).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2].wrapping_sub(2); cpu.a[2] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C02A90: move.w $2f(a0, d3.w), -(a0)
pub fn i_c02a90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A94;
    cpu.ir = 0x3130;
    let ad2 = cpu.a[0].wrapping_add(0x2F).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_sub(2); cpu.a[0] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C02A94: move.l $2c2c(a5), d7
pub fn i_c02a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A98;
    cpu.ir = 0x2E2D;
    let ad2 = cpu.a[5].wrapping_add(0x2C2C);
    let v3 = bus.read32(ad2);
    cpu.d[7] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C02A98: move.l $2929(a2), -(a5)
pub fn i_c02a98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A9C;
    cpu.ir = 0x2B2A;
    let ad2 = cpu.a[2].wrapping_add(0x2929);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[5];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[5] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C02A9C: move.l -(a7), d4
pub fn i_c02a9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02A9E;
    cpu.ir = 0x2827;
    let ad2 = cpu.a[7].wrapping_sub(4); cpu.a[7] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[4] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02A9E: move.l -(a6), -(a3)
pub fn i_c02a9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02AA0;
    cpu.ir = 0x2726;
    let ad2 = cpu.a[6].wrapping_sub(4); cpu.a[6] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[3];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[3] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C02AA0: move.l -(a5), -(a2)
pub fn i_c02aa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02AA2;
    cpu.ir = 0x2525;
    let ad2 = cpu.a[5].wrapping_sub(4); cpu.a[5] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[2];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[2] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C02AA2: move.l -(a4), d2
pub fn i_c02aa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02AA4;
    cpu.ir = 0x2424;
    let ad2 = cpu.a[4].wrapping_sub(4); cpu.a[4] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02AA4: move.l -(a3), -(a1)
pub fn i_c02aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02AA6;
    cpu.ir = 0x2323;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[1];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[1] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C02AA6: move.l -(a2), d1
pub fn i_c02aa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02AA8;
    cpu.ir = 0x2222;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02AA8: move.l -(a0), -(a0)
pub fn i_c02aa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02AAA;
    cpu.ir = 0x2120;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[0];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[0] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C02AAA: move.l -(a0), d0
pub fn i_c02aaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02AAC;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02AAC: move.b (a7)+, -(a7)
pub fn i_c02aac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02AAE;
    cpu.ir = 0x1F1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02AAE: move.b (a6)+, d7
pub fn i_c02aae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02AB0;
    cpu.ir = 0x1E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02AB0: move.b (a5)+, -(a6)
pub fn i_c02ab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02AB2;
    cpu.ir = 0x1D1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02AB2: move.b (a4)+, d6
pub fn i_c02ab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02AB4;
    cpu.ir = 0x1C1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02AB4: move.b (a3)+, d6
pub fn i_c02ab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02AB6;
    cpu.ir = 0x1C1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02AB6: move.b (a3)+, -(a5)
pub fn i_c02ab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02AB8;
    cpu.ir = 0x1B1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02AB8: move.b (a2)+, d5
pub fn i_c02ab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02ABA;
    cpu.ir = 0x1A1A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02ABA: move.b (a1)+, d5
pub fn i_c02aba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02ABC;
    cpu.ir = 0x1A19;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02ABC: move.b (a1)+, -(a4)
pub fn i_c02abc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02ABE;
    cpu.ir = 0x1919;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02ABE: move.b (a0)+, d4
pub fn i_c02abe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02AC0;
    cpu.ir = 0x1818;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02AC0: ?
pub fn i_c02ac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02AC2;
    cpu.ir = 0x403F;
    let v1: u32 = 0x0;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x80) != 0 && (r2 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x80 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    4
}
/// $C02AC2: ?
pub fn i_c02ac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02AC4;
    cpu.ir = 0x3E3D;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C02AC4: move.w $c02aff(pc, d3.l), d6
pub fn i_c02ac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02AC8;
    cpu.ir = 0x3C3B;
    let ad2 = 0xC02AFF_u32.wrapping_add(cpu.d[3]);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C02AC8: move.w $3736.w, -(a4)
pub fn i_c02ac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02ACC;
    cpu.ir = 0x3938;
    let v2 = bus.read16(0x3736) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[4].wrapping_sub(2); cpu.a[4] = ad3;
    bus.write16(ad3, v2 as u16);
    18
}
/// $C02ACC: move.w ([$3131302f, a4, d3.w * 2], $2e2d), -(a2)
pub fn i_c02acc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02AD0;
    cpu.ir = 0x3534;
    let ad2 = cpu.a[4].wrapping_add(0x32).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2].wrapping_sub(2); cpu.a[2] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C02AD0: move.w $2f(a1, d3.w), -(a0)
pub fn i_c02ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02AD4;
    cpu.ir = 0x3131;
    let ad2 = cpu.a[1].wrapping_add(0x2F).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_sub(2); cpu.a[0] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C02AD4: move.l $2d2c(a5), d7
pub fn i_c02ad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02AD8;
    cpu.ir = 0x2E2D;
    let ad2 = cpu.a[5].wrapping_add(0x2D2C);
    let v3 = bus.read32(ad2);
    cpu.d[7] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C02AD8: move.l $2a29(a3), -(a5)
pub fn i_c02ad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02ADC;
    cpu.ir = 0x2B2B;
    let ad2 = cpu.a[3].wrapping_add(0x2A29);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[5];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[5] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C02ADC: move.l $2727(a0), d4
pub fn i_c02adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02AE0;
    cpu.ir = 0x2828;
    let ad2 = cpu.a[0].wrapping_add(0x2727);
    let v3 = bus.read32(ad2);
    cpu.d[4] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C02AE0: move.l -(a5), d3
pub fn i_c02ae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02AE2;
    cpu.ir = 0x2625;
    let ad2 = cpu.a[5].wrapping_sub(4); cpu.a[5] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02AE2: move.l -(a4), -(a2)
pub fn i_c02ae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02AE4;
    cpu.ir = 0x2524;
    let ad2 = cpu.a[4].wrapping_sub(4); cpu.a[4] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[2];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[2] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C02AE4: move.l -(a3), d2
pub fn i_c02ae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02AE6;
    cpu.ir = 0x2423;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02AE6: move.l -(a2), -(a1)
pub fn i_c02ae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02AE8;
    cpu.ir = 0x2322;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[1];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[1] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C02AE8: move.l -(a1), -(a0)
pub fn i_c02ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02AEA;
    cpu.ir = 0x2121;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[0];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[0] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C02AEA: move.l -(a0), d0
pub fn i_c02aea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02AEC;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02AEC: move.l (a7)+, d0
pub fn i_c02aec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02AEE;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C02AEE: move.b (a6)+, -(a7)
pub fn i_c02aee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02AF0;
    cpu.ir = 0x1F1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02AF0: move.b (a5)+, d7
pub fn i_c02af0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02AF2;
    cpu.ir = 0x1E1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02AF2: move.b (a5)+, -(a6)
pub fn i_c02af2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02AF4;
    cpu.ir = 0x1D1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02AF4: move.b (a4)+, d6
pub fn i_c02af4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02AF6;
    cpu.ir = 0x1C1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02AF6: move.b (a3)+, -(a5)
pub fn i_c02af6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02AF8;
    cpu.ir = 0x1B1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02AF8: move.b (a2)+, -(a5)
pub fn i_c02af8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02AFA;
    cpu.ir = 0x1B1A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02AFA: move.b (a2)+, d5
pub fn i_c02afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02AFC;
    cpu.ir = 0x1A1A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02AFC: move.b (a1)+, -(a4)
pub fn i_c02afc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02AFE;
    cpu.ir = 0x1919;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02AFE: move.b (a0)+, -(a4)
pub fn i_c02afe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B00;
    cpu.ir = 0x1918;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02B00: ?
pub fn i_c02b00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B02;
    cpu.ir = 0x403F;
    let v1: u32 = 0x0;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x80) != 0 && (r2 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x80 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    4
}
/// $C02B02: ?
pub fn i_c02b02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B04;
    cpu.ir = 0x3E3D;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C02B04: move.w $c02b40(pc, d3.l), d6
pub fn i_c02b04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B08;
    cpu.ir = 0x3C3B;
    let ad2 = 0xC02B40_u32.wrapping_add(cpu.d[3]);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C02B08: move.w $3736.w, -(a4)
pub fn i_c02b08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B0C;
    cpu.ir = 0x3938;
    let v2 = bus.read16(0x3736) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[4].wrapping_sub(2); cpu.a[4] = ad3;
    bus.write16(ad3, v2 as u16);
    18
}
/// $C02B0C: move.w ([$3231302f, a4, d3.w * 2], $aaaaaaaa), -(a2)
pub fn i_c02b0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B10;
    cpu.ir = 0x3534;
    let ad2 = cpu.a[4].wrapping_add(0x33).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2].wrapping_sub(2); cpu.a[2] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C02B10: move.w $2f(a1, d3.w), d1
pub fn i_c02b10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B14;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x2F).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C02B14: move.l $2d2c(a6), -(a7)
pub fn i_c02b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B18;
    cpu.ir = 0x2F2E;
    let ad2 = cpu.a[6].wrapping_add(0x2D2C);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[7];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[7] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0xC00000, i_c00000),
    (0xC00002, i_c00002),
    (0xC00004, i_c00004),
    (0xC00006, i_c00006),
    (0xC00008, i_c00008),
    (0xC0000A, i_c0000a),
    (0xC0000E, i_c0000e),
    (0xC00012, i_c00012),
    (0xC00014, i_c00014),
    (0xC00016, i_c00016),
    (0xC00018, i_c00018),
    (0xC0001A, i_c0001a),
    (0xC0001C, i_c0001c),
    (0xC0001E, i_c0001e),
    (0xC00020, i_c00020),
    (0xC00022, i_c00022),
    (0xC00024, i_c00024),
    (0xC00026, i_c00026),
    (0xC00028, i_c00028),
    (0xC0002A, i_c0002a),
    (0xC0002E, i_c0002e),
    (0xC00032, i_c00032),
    (0xC00034, i_c00034),
    (0xC00036, i_c00036),
    (0xC0003A, i_c0003a),
    (0xC0003C, i_c0003c),
    (0xC0003E, i_c0003e),
    (0xC00040, i_c00040),
    (0xC00042, i_c00042),
    (0xC00044, i_c00044),
    (0xC00046, i_c00046),
    (0xC0004A, i_c0004a),
    (0xC0004E, i_c0004e),
    (0xC00050, i_c00050),
    (0xC00052, i_c00052),
    (0xC00056, i_c00056),
    (0xC0005A, i_c0005a),
    (0xC0005E, i_c0005e),
    (0xC00060, i_c00060),
    (0xC00062, i_c00062),
    (0xC00068, i_c00068),
    (0xC0006A, i_c0006a),
    (0xC0006E, i_c0006e),
    (0xC00070, i_c00070),
    (0xC00072, i_c00072),
    (0xC00074, i_c00074),
    (0xC00076, i_c00076),
    (0xC00078, i_c00078),
    (0xC0007A, i_c0007a),
    (0xC0007E, i_c0007e),
    (0xC00082, i_c00082),
    (0xC00084, i_c00084),
    (0xC00086, i_c00086),
    (0xC00088, i_c00088),
    (0xC0008A, i_c0008a),
    (0xC0008C, i_c0008c),
    (0xC0008E, i_c0008e),
    (0xC00090, i_c00090),
    (0xC00092, i_c00092),
    (0xC00094, i_c00094),
    (0xC00096, i_c00096),
    (0xC00098, i_c00098),
    (0xC0009A, i_c0009a),
    (0xC0009C, i_c0009c),
    (0xC0009E, i_c0009e),
    (0xC000A0, i_c000a0),
    (0xC000A4, i_c000a4),
    (0xC000A6, i_c000a6),
    (0xC000A8, i_c000a8),
    (0xC000AA, i_c000aa),
    (0xC000AC, i_c000ac),
    (0xC000AE, i_c000ae),
    (0xC000B0, i_c000b0),
    (0xC000B2, i_c000b2),
    (0xC000B4, i_c000b4),
    (0xC000B6, i_c000b6),
    (0xC000B8, i_c000b8),
    (0xC000BA, i_c000ba),
    (0xC000BC, i_c000bc),
    (0xC000BE, i_c000be),
    (0xC000C0, i_c000c0),
    (0xC000C2, i_c000c2),
    (0xC000C4, i_c000c4),
    (0xC000C6, i_c000c6),
    (0xC000C8, i_c000c8),
    (0xC000CA, i_c000ca),
    (0xC000CC, i_c000cc),
    (0xC000CE, i_c000ce),
    (0xC000D0, i_c000d0),
    (0xC000D2, i_c000d2),
    (0xC000D4, i_c000d4),
    (0xC000D6, i_c000d6),
    (0xC000D8, i_c000d8),
    (0xC000DA, i_c000da),
    (0xC000DC, i_c000dc),
    (0xC000DE, i_c000de),
    (0xC000E0, i_c000e0),
    (0xC000E2, i_c000e2),
    (0xC000E4, i_c000e4),
    (0xC000E6, i_c000e6),
    (0xC000E8, i_c000e8),
    (0xC000EA, i_c000ea),
    (0xC000EC, i_c000ec),
    (0xC000EE, i_c000ee),
    (0xC000F0, i_c000f0),
    (0xC00100, i_c00100),
    (0xC00102, i_c00102),
    (0xC00104, i_c00104),
    (0xC00106, i_c00106),
    (0xC00108, i_c00108),
    (0xC0010A, i_c0010a),
    (0xC0010C, i_c0010c),
    (0xC0010E, i_c0010e),
    (0xC00110, i_c00110),
    (0xC00112, i_c00112),
    (0xC00114, i_c00114),
    (0xC00116, i_c00116),
    (0xC00118, i_c00118),
    (0xC0011A, i_c0011a),
    (0xC0011C, i_c0011c),
    (0xC0011E, i_c0011e),
    (0xC00120, i_c00120),
    (0xC00122, i_c00122),
    (0xC00124, i_c00124),
    (0xC00126, i_c00126),
    (0xC00128, i_c00128),
    (0xC0012A, i_c0012a),
    (0xC0012C, i_c0012c),
    (0xC0012E, i_c0012e),
    (0xC00130, i_c00130),
    (0xC00132, i_c00132),
    (0xC00134, i_c00134),
    (0xC00136, i_c00136),
    (0xC00138, i_c00138),
    (0xC0013A, i_c0013a),
    (0xC0013C, i_c0013c),
    (0xC0013E, i_c0013e),
    (0xC00140, i_c00140),
    (0xC00142, i_c00142),
    (0xC00144, i_c00144),
    (0xC00146, i_c00146),
    (0xC00148, i_c00148),
    (0xC0014A, i_c0014a),
    (0xC0014C, i_c0014c),
    (0xC0014E, i_c0014e),
    (0xC00150, i_c00150),
    (0xC00158, i_c00158),
    (0xC0015A, i_c0015a),
    (0xC0015C, i_c0015c),
    (0xC0015E, i_c0015e),
    (0xC00160, i_c00160),
    (0xC00162, i_c00162),
    (0xC00164, i_c00164),
    (0xC00166, i_c00166),
    (0xC00168, i_c00168),
    (0xC0016A, i_c0016a),
    (0xC0016C, i_c0016c),
    (0xC0016E, i_c0016e),
    (0xC00170, i_c00170),
    (0xC00172, i_c00172),
    (0xC00174, i_c00174),
    (0xC00180, i_c00180),
    (0xC00182, i_c00182),
    (0xC00184, i_c00184),
    (0xC00186, i_c00186),
    (0xC00188, i_c00188),
    (0xC0018A, i_c0018a),
    (0xC0018C, i_c0018c),
    (0xC0018E, i_c0018e),
    (0xC00190, i_c00190),
    (0xC00198, i_c00198),
    (0xC0019A, i_c0019a),
    (0xC0019C, i_c0019c),
    (0xC0019E, i_c0019e),
    (0xC001A0, i_c001a0),
    (0xC001A2, i_c001a2),
    (0xC001A4, i_c001a4),
    (0xC001A6, i_c001a6),
    (0xC001A8, i_c001a8),
    (0xC001B0, i_c001b0),
    (0xC001B2, i_c001b2),
    (0xC001B4, i_c001b4),
    (0xC001B6, i_c001b6),
    (0xC001B8, i_c001b8),
    (0xC001BA, i_c001ba),
    (0xC001BC, i_c001bc),
    (0xC001BE, i_c001be),
    (0xC001C0, i_c001c0),
    (0xC00200, i_c00200),
    (0xC00202, i_c00202),
    (0xC00204, i_c00204),
    (0xC00206, i_c00206),
    (0xC00208, i_c00208),
    (0xC0020A, i_c0020a),
    (0xC0020C, i_c0020c),
    (0xC0020E, i_c0020e),
    (0xC00210, i_c00210),
    (0xC00212, i_c00212),
    (0xC00214, i_c00214),
    (0xC00216, i_c00216),
    (0xC00218, i_c00218),
    (0xC0021A, i_c0021a),
    (0xC0021C, i_c0021c),
    (0xC0021E, i_c0021e),
    (0xC00220, i_c00220),
    (0xC00222, i_c00222),
    (0xC00224, i_c00224),
    (0xC00228, i_c00228),
    (0xC0022A, i_c0022a),
    (0xC0022C, i_c0022c),
    (0xC0022E, i_c0022e),
    (0xC00230, i_c00230),
    (0xC00232, i_c00232),
    (0xC00234, i_c00234),
    (0xC00236, i_c00236),
    (0xC00238, i_c00238),
    (0xC0023A, i_c0023a),
    (0xC0023C, i_c0023c),
    (0xC0023E, i_c0023e),
    (0xC00240, i_c00240),
    (0xC00242, i_c00242),
    (0xC00244, i_c00244),
    (0xC00246, i_c00246),
    (0xC00248, i_c00248),
    (0xC0024A, i_c0024a),
    (0xC0024C, i_c0024c),
    (0xC0024E, i_c0024e),
    (0xC00250, i_c00250),
    (0xC00252, i_c00252),
    (0xC00254, i_c00254),
    (0xC00256, i_c00256),
    (0xC00258, i_c00258),
    (0xC0025A, i_c0025a),
    (0xC0025C, i_c0025c),
    (0xC0025E, i_c0025e),
    (0xC00260, i_c00260),
    (0xC00278, i_c00278),
    (0xC0027A, i_c0027a),
    (0xC0027C, i_c0027c),
    (0xC0027E, i_c0027e),
    (0xC00280, i_c00280),
    (0xC00282, i_c00282),
    (0xC00284, i_c00284),
    (0xC00286, i_c00286),
    (0xC00288, i_c00288),
    (0xC00296, i_c00296),
    (0xC00298, i_c00298),
    (0xC0029A, i_c0029a),
    (0xC0029C, i_c0029c),
    (0xC0029E, i_c0029e),
    (0xC002A0, i_c002a0),
    (0xC002A2, i_c002a2),
    (0xC002A4, i_c002a4),
    (0xC002A6, i_c002a6),
    (0xC00300, i_c00300),
    (0xC00302, i_c00302),
    (0xC00304, i_c00304),
    (0xC00306, i_c00306),
    (0xC00308, i_c00308),
    (0xC0030A, i_c0030a),
    (0xC0030C, i_c0030c),
    (0xC0030E, i_c0030e),
    (0xC00310, i_c00310),
    (0xC00312, i_c00312),
    (0xC00314, i_c00314),
    (0xC00316, i_c00316),
    (0xC00318, i_c00318),
    (0xC0031A, i_c0031a),
    (0xC0031C, i_c0031c),
    (0xC0031E, i_c0031e),
    (0xC00320, i_c00320),
    (0xC00322, i_c00322),
    (0xC00324, i_c00324),
    (0xC00326, i_c00326),
    (0xC00328, i_c00328),
    (0xC0032A, i_c0032a),
    (0xC0032C, i_c0032c),
    (0xC0032E, i_c0032e),
    (0xC00400, i_c00400),
    (0xC00402, i_c00402),
    (0xC00404, i_c00404),
    (0xC00406, i_c00406),
    (0xC00408, i_c00408),
    (0xC0040A, i_c0040a),
    (0xC0040C, i_c0040c),
    (0xC0040E, i_c0040e),
    (0xC00410, i_c00410),
    (0xC00412, i_c00412),
    (0xC00414, i_c00414),
    (0xC00416, i_c00416),
    (0xC00418, i_c00418),
    (0xC0041A, i_c0041a),
    (0xC0041C, i_c0041c),
    (0xC0041E, i_c0041e),
    (0xC00420, i_c00420),
    (0xC00422, i_c00422),
    (0xC00424, i_c00424),
    (0xC00426, i_c00426),
    (0xC00428, i_c00428),
    (0xC0042A, i_c0042a),
    (0xC0042C, i_c0042c),
    (0xC0042E, i_c0042e),
    (0xC00430, i_c00430),
    (0xC00432, i_c00432),
    (0xC00434, i_c00434),
    (0xC00436, i_c00436),
    (0xC00438, i_c00438),
    (0xC0043A, i_c0043a),
    (0xC0043C, i_c0043c),
    (0xC0043E, i_c0043e),
    (0xC00440, i_c00440),
    (0xC00442, i_c00442),
    (0xC00444, i_c00444),
    (0xC00446, i_c00446),
    (0xC00448, i_c00448),
    (0xC0044A, i_c0044a),
    (0xC00450, i_c00450),
    (0xC00456, i_c00456),
    (0xC0045C, i_c0045c),
    (0xC00462, i_c00462),
    (0xC00468, i_c00468),
    (0xC0046E, i_c0046e),
    (0xC00474, i_c00474),
    (0xC0047A, i_c0047a),
    (0xC00480, i_c00480),
    (0xC00484, i_c00484),
    (0xC00486, i_c00486),
    (0xC0048A, i_c0048a),
    (0xC0048C, i_c0048c),
    (0xC00490, i_c00490),
    (0xC00492, i_c00492),
    (0xC00496, i_c00496),
    (0xC00498, i_c00498),
    (0xC0049C, i_c0049c),
    (0xC0049E, i_c0049e),
    (0xC004C2, i_c004c2),
    (0xC004C8, i_c004c8),
    (0xC004CE, i_c004ce),
    (0xC004D4, i_c004d4),
    (0xC004DA, i_c004da),
    (0xC004DC, i_c004dc),
    (0xC004DE, i_c004de),
    (0xC004E0, i_c004e0),
    (0xC004E2, i_c004e2),
    (0xC004E4, i_c004e4),
    (0xC004E6, i_c004e6),
    (0xC00506, i_c00506),
    (0xC00508, i_c00508),
    (0xC0050A, i_c0050a),
    (0xC0050C, i_c0050c),
    (0xC0050E, i_c0050e),
    (0xC00510, i_c00510),
    (0xC00512, i_c00512),
    (0xC00514, i_c00514),
    (0xC00516, i_c00516),
    (0xC005E8, i_c005e8),
    (0xC005EA, i_c005ea),
    (0xC005EC, i_c005ec),
    (0xC005EE, i_c005ee),
    (0xC005F0, i_c005f0),
    (0xC005F2, i_c005f2),
    (0xC005F4, i_c005f4),
    (0xC005F6, i_c005f6),
    (0xC005F8, i_c005f8),
    (0xC005FA, i_c005fa),
    (0xC005FC, i_c005fc),
    (0xC005FE, i_c005fe),
    (0xC00600, i_c00600),
    (0xC00602, i_c00602),
    (0xC00604, i_c00604),
    (0xC00606, i_c00606),
    (0xC00608, i_c00608),
    (0xC0060A, i_c0060a),
    (0xC0060C, i_c0060c),
    (0xC0060E, i_c0060e),
    (0xC00610, i_c00610),
    (0xC00612, i_c00612),
    (0xC00614, i_c00614),
    (0xC00616, i_c00616),
    (0xC00618, i_c00618),
    (0xC0061A, i_c0061a),
    (0xC00A00, i_c00a00),
    (0xC00A02, i_c00a02),
    (0xC00A04, i_c00a04),
    (0xC00A06, i_c00a06),
    (0xC00A08, i_c00a08),
    (0xC00A0A, i_c00a0a),
    (0xC00A0C, i_c00a0c),
    (0xC00A0E, i_c00a0e),
    (0xC00A10, i_c00a10),
    (0xC00C00, i_c00c00),
    (0xC00C02, i_c00c02),
    (0xC00C04, i_c00c04),
    (0xC00C06, i_c00c06),
    (0xC00C08, i_c00c08),
    (0xC00C0A, i_c00c0a),
    (0xC00C0C, i_c00c0c),
    (0xC00C0E, i_c00c0e),
    (0xC00C10, i_c00c10),
    (0xC00C6E, i_c00c6e),
    (0xC00C70, i_c00c70),
    (0xC00C72, i_c00c72),
    (0xC00C74, i_c00c74),
    (0xC00C76, i_c00c76),
    (0xC00C78, i_c00c78),
    (0xC00C7A, i_c00c7a),
    (0xC00C7C, i_c00c7c),
    (0xC00C7E, i_c00c7e),
    (0xC00E02, i_c00e02),
    (0xC00E04, i_c00e04),
    (0xC00E06, i_c00e06),
    (0xC00E08, i_c00e08),
    (0xC00E0A, i_c00e0a),
    (0xC00E0C, i_c00e0c),
    (0xC00E0E, i_c00e0e),
    (0xC00E10, i_c00e10),
    (0xC00E12, i_c00e12),
    (0xC010C0, i_c010c0),
    (0xC010C2, i_c010c2),
    (0xC010C4, i_c010c4),
    (0xC010C6, i_c010c6),
    (0xC010C8, i_c010c8),
    (0xC010CA, i_c010ca),
    (0xC010CC, i_c010cc),
    (0xC010CE, i_c010ce),
    (0xC010D0, i_c010d0),
    (0xC010FC, i_c010fc),
    (0xC010FE, i_c010fe),
    (0xC01100, i_c01100),
    (0xC01102, i_c01102),
    (0xC01104, i_c01104),
    (0xC01106, i_c01106),
    (0xC01108, i_c01108),
    (0xC0110A, i_c0110a),
    (0xC0110C, i_c0110c),
    (0xC0206E, i_c0206e),
    (0xC02070, i_c02070),
    (0xC02072, i_c02072),
    (0xC02074, i_c02074),
    (0xC02076, i_c02076),
    (0xC02078, i_c02078),
    (0xC0207A, i_c0207a),
    (0xC0207C, i_c0207c),
    (0xC0207E, i_c0207e),
    (0xC02080, i_c02080),
    (0xC02084, i_c02084),
    (0xC02086, i_c02086),
    (0xC0208A, i_c0208a),
    (0xC0208E, i_c0208e),
    (0xC02092, i_c02092),
    (0xC02094, i_c02094),
    (0xC02098, i_c02098),
    (0xC0209A, i_c0209a),
    (0xC0209C, i_c0209c),
    (0xC0209E, i_c0209e),
    (0xC020A0, i_c020a0),
    (0xC020A2, i_c020a2),
    (0xC020A6, i_c020a6),
    (0xC020AA, i_c020aa),
    (0xC020AE, i_c020ae),
    (0xC020B2, i_c020b2),
    (0xC020B6, i_c020b6),
    (0xC020BA, i_c020ba),
    (0xC020BC, i_c020bc),
    (0xC020BE, i_c020be),
    (0xC020C0, i_c020c0),
    (0xC020C4, i_c020c4),
    (0xC020C6, i_c020c6),
    (0xC020C8, i_c020c8),
    (0xC020CC, i_c020cc),
    (0xC020D0, i_c020d0),
    (0xC020D4, i_c020d4),
    (0xC020D8, i_c020d8),
    (0xC020DA, i_c020da),
    (0xC020DC, i_c020dc),
    (0xC020E0, i_c020e0),
    (0xC020E4, i_c020e4),
    (0xC020E6, i_c020e6),
    (0xC020E8, i_c020e8),
    (0xC020EA, i_c020ea),
    (0xC020EC, i_c020ec),
    (0xC020EE, i_c020ee),
    (0xC020F0, i_c020f0),
    (0xC020F2, i_c020f2),
    (0xC020F6, i_c020f6),
    (0xC020FA, i_c020fa),
    (0xC020FE, i_c020fe),
    (0xC02102, i_c02102),
    (0xC02104, i_c02104),
    (0xC02106, i_c02106),
    (0xC02108, i_c02108),
    (0xC0210A, i_c0210a),
    (0xC0210C, i_c0210c),
    (0xC02110, i_c02110),
    (0xC02114, i_c02114),
    (0xC02118, i_c02118),
    (0xC0211A, i_c0211a),
    (0xC0211E, i_c0211e),
    (0xC02120, i_c02120),
    (0xC02122, i_c02122),
    (0xC02124, i_c02124),
    (0xC02126, i_c02126),
    (0xC0212A, i_c0212a),
    (0xC0212E, i_c0212e),
    (0xC02132, i_c02132),
    (0xC02134, i_c02134),
    (0xC02136, i_c02136),
    (0xC02138, i_c02138),
    (0xC0213A, i_c0213a),
    (0xC0213C, i_c0213c),
    (0xC0213E, i_c0213e),
    (0xC02140, i_c02140),
    (0xC02144, i_c02144),
    (0xC02146, i_c02146),
    (0xC02148, i_c02148),
    (0xC0214A, i_c0214a),
    (0xC0214C, i_c0214c),
    (0xC0214E, i_c0214e),
    (0xC02152, i_c02152),
    (0xC02156, i_c02156),
    (0xC0215A, i_c0215a),
    (0xC0215E, i_c0215e),
    (0xC02160, i_c02160),
    (0xC02164, i_c02164),
    (0xC02168, i_c02168),
    (0xC0216A, i_c0216a),
    (0xC0216C, i_c0216c),
    (0xC0216E, i_c0216e),
    (0xC02172, i_c02172),
    (0xC02176, i_c02176),
    (0xC0217A, i_c0217a),
    (0xC0217E, i_c0217e),
    (0xC02180, i_c02180),
    (0xC02186, i_c02186),
    (0xC02188, i_c02188),
    (0xC0218A, i_c0218a),
    (0xC0218C, i_c0218c),
    (0xC0218E, i_c0218e),
    (0xC02190, i_c02190),
    (0xC02194, i_c02194),
    (0xC02198, i_c02198),
    (0xC0219C, i_c0219c),
    (0xC021A0, i_c021a0),
    (0xC021A4, i_c021a4),
    (0xC021A6, i_c021a6),
    (0xC021AA, i_c021aa),
    (0xC021AE, i_c021ae),
    (0xC021B0, i_c021b0),
    (0xC021B2, i_c021b2),
    (0xC021B4, i_c021b4),
    (0xC021B6, i_c021b6),
    (0xC021B8, i_c021b8),
    (0xC021BC, i_c021bc),
    (0xC021C0, i_c021c0),
    (0xC021C4, i_c021c4),
    (0xC021C6, i_c021c6),
    (0xC021C8, i_c021c8),
    (0xC021CA, i_c021ca),
    (0xC021CC, i_c021cc),
    (0xC021CE, i_c021ce),
    (0xC021D0, i_c021d0),
    (0xC021D2, i_c021d2),
    (0xC021D6, i_c021d6),
    (0xC021DA, i_c021da),
    (0xC021DE, i_c021de),
    (0xC021E2, i_c021e2),
    (0xC021E6, i_c021e6),
    (0xC021E8, i_c021e8),
    (0xC021EA, i_c021ea),
    (0xC021EC, i_c021ec),
    (0xC021F0, i_c021f0),
    (0xC021F4, i_c021f4),
    (0xC021F6, i_c021f6),
    (0xC021F8, i_c021f8),
    (0xC021FA, i_c021fa),
    (0xC021FC, i_c021fc),
    (0xC021FE, i_c021fe),
    (0xC02200, i_c02200),
    (0xC02204, i_c02204),
    (0xC02208, i_c02208),
    (0xC0220A, i_c0220a),
    (0xC0220C, i_c0220c),
    (0xC0220E, i_c0220e),
    (0xC02210, i_c02210),
    (0xC02212, i_c02212),
    (0xC02214, i_c02214),
    (0xC02216, i_c02216),
    (0xC0221A, i_c0221a),
    (0xC0221E, i_c0221e),
    (0xC02222, i_c02222),
    (0xC02226, i_c02226),
    (0xC0222A, i_c0222a),
    (0xC0222E, i_c0222e),
    (0xC02230, i_c02230),
    (0xC02232, i_c02232),
    (0xC02236, i_c02236),
    (0xC0223A, i_c0223a),
    (0xC0223E, i_c0223e),
    (0xC02240, i_c02240),
    (0xC02244, i_c02244),
    (0xC02248, i_c02248),
    (0xC0224A, i_c0224a),
    (0xC0224C, i_c0224c),
    (0xC0224E, i_c0224e),
    (0xC02250, i_c02250),
    (0xC02252, i_c02252),
    (0xC02254, i_c02254),
    (0xC02256, i_c02256),
    (0xC02258, i_c02258),
    (0xC0225C, i_c0225c),
    (0xC02260, i_c02260),
    (0xC02264, i_c02264),
    (0xC02268, i_c02268),
    (0xC0226C, i_c0226c),
    (0xC02270, i_c02270),
    (0xC02274, i_c02274),
    (0xC02276, i_c02276),
    (0xC02278, i_c02278),
    (0xC0227C, i_c0227c),
    (0xC02280, i_c02280),
    (0xC02284, i_c02284),
    (0xC02288, i_c02288),
    (0xC0228A, i_c0228a),
    (0xC0228C, i_c0228c),
    (0xC0228E, i_c0228e),
    (0xC02290, i_c02290),
    (0xC02292, i_c02292),
    (0xC02294, i_c02294),
    (0xC02296, i_c02296),
    (0xC02298, i_c02298),
    (0xC0229A, i_c0229a),
    (0xC0229E, i_c0229e),
    (0xC022A2, i_c022a2),
    (0xC022A6, i_c022a6),
    (0xC022AA, i_c022aa),
    (0xC022AE, i_c022ae),
    (0xC022B2, i_c022b2),
    (0xC022B6, i_c022b6),
    (0xC022B8, i_c022b8),
    (0xC022BA, i_c022ba),
    (0xC022BC, i_c022bc),
    (0xC022BE, i_c022be),
    (0xC022C0, i_c022c0),
    (0xC022C4, i_c022c4),
    (0xC022C8, i_c022c8),
    (0xC022CA, i_c022ca),
    (0xC022CC, i_c022cc),
    (0xC022CE, i_c022ce),
    (0xC022D0, i_c022d0),
    (0xC022D2, i_c022d2),
    (0xC022D4, i_c022d4),
    (0xC022D6, i_c022d6),
    (0xC022D8, i_c022d8),
    (0xC022DA, i_c022da),
    (0xC022DC, i_c022dc),
    (0xC022E0, i_c022e0),
    (0xC022E4, i_c022e4),
    (0xC022E8, i_c022e8),
    (0xC022EC, i_c022ec),
    (0xC022F0, i_c022f0),
    (0xC022F4, i_c022f4),
    (0xC022F8, i_c022f8),
    (0xC022FC, i_c022fc),
    (0xC022FE, i_c022fe),
    (0xC02300, i_c02300),
    (0xC02302, i_c02302),
    (0xC02306, i_c02306),
    (0xC0230A, i_c0230a),
    (0xC0230C, i_c0230c),
    (0xC0230E, i_c0230e),
    (0xC02310, i_c02310),
    (0xC02312, i_c02312),
    (0xC02314, i_c02314),
    (0xC02316, i_c02316),
    (0xC02318, i_c02318),
    (0xC0231A, i_c0231a),
    (0xC0231C, i_c0231c),
    (0xC0231E, i_c0231e),
    (0xC02320, i_c02320),
    (0xC02324, i_c02324),
    (0xC02328, i_c02328),
    (0xC0232C, i_c0232c),
    (0xC02330, i_c02330),
    (0xC02334, i_c02334),
    (0xC02338, i_c02338),
    (0xC0233C, i_c0233c),
    (0xC02340, i_c02340),
    (0xC02342, i_c02342),
    (0xC02346, i_c02346),
    (0xC0234A, i_c0234a),
    (0xC0234C, i_c0234c),
    (0xC0234E, i_c0234e),
    (0xC02350, i_c02350),
    (0xC02352, i_c02352),
    (0xC02354, i_c02354),
    (0xC02356, i_c02356),
    (0xC02358, i_c02358),
    (0xC0235A, i_c0235a),
    (0xC0235C, i_c0235c),
    (0xC0235E, i_c0235e),
    (0xC02360, i_c02360),
    (0xC02362, i_c02362),
    (0xC02366, i_c02366),
    (0xC0236A, i_c0236a),
    (0xC0236E, i_c0236e),
    (0xC02372, i_c02372),
    (0xC02376, i_c02376),
    (0xC0237A, i_c0237a),
    (0xC0237E, i_c0237e),
    (0xC02382, i_c02382),
    (0xC02386, i_c02386),
    (0xC0238A, i_c0238a),
    (0xC0238C, i_c0238c),
    (0xC0238E, i_c0238e),
    (0xC02390, i_c02390),
    (0xC02392, i_c02392),
    (0xC02394, i_c02394),
    (0xC02396, i_c02396),
    (0xC02398, i_c02398),
    (0xC0239A, i_c0239a),
    (0xC0239C, i_c0239c),
    (0xC0239E, i_c0239e),
    (0xC023A0, i_c023a0),
    (0xC023A2, i_c023a2),
    (0xC023A4, i_c023a4),
    (0xC023A8, i_c023a8),
    (0xC023AC, i_c023ac),
    (0xC023B0, i_c023b0),
    (0xC023B4, i_c023b4),
    (0xC023B8, i_c023b8),
    (0xC023BC, i_c023bc),
    (0xC023C0, i_c023c0),
    (0xC023C2, i_c023c2),
    (0xC023C6, i_c023c6),
    (0xC023CA, i_c023ca),
    (0xC023CC, i_c023cc),
    (0xC023CE, i_c023ce),
    (0xC023D0, i_c023d0),
    (0xC023D2, i_c023d2),
    (0xC023D4, i_c023d4),
    (0xC023D6, i_c023d6),
    (0xC023D8, i_c023d8),
    (0xC023DA, i_c023da),
    (0xC023DC, i_c023dc),
    (0xC023DE, i_c023de),
    (0xC023E0, i_c023e0),
    (0xC023E2, i_c023e2),
    (0xC023E4, i_c023e4),
    (0xC023E6, i_c023e6),
    (0xC023EA, i_c023ea),
    (0xC023EE, i_c023ee),
    (0xC023F2, i_c023f2),
    (0xC023F6, i_c023f6),
    (0xC023FA, i_c023fa),
    (0xC023FE, i_c023fe),
    (0xC02402, i_c02402),
    (0xC02406, i_c02406),
    (0xC0240A, i_c0240a),
    (0xC0240C, i_c0240c),
    (0xC0240E, i_c0240e),
    (0xC02410, i_c02410),
    (0xC02412, i_c02412),
    (0xC02414, i_c02414),
    (0xC02416, i_c02416),
    (0xC02418, i_c02418),
    (0xC0241A, i_c0241a),
    (0xC0241C, i_c0241c),
    (0xC0241E, i_c0241e),
    (0xC02420, i_c02420),
    (0xC02422, i_c02422),
    (0xC02424, i_c02424),
    (0xC02426, i_c02426),
    (0xC02428, i_c02428),
    (0xC0242A, i_c0242a),
    (0xC0242E, i_c0242e),
    (0xC02432, i_c02432),
    (0xC02436, i_c02436),
    (0xC0243A, i_c0243a),
    (0xC0243E, i_c0243e),
    (0xC02442, i_c02442),
    (0xC02448, i_c02448),
    (0xC0244C, i_c0244c),
    (0xC0244E, i_c0244e),
    (0xC02450, i_c02450),
    (0xC02452, i_c02452),
    (0xC02454, i_c02454),
    (0xC02456, i_c02456),
    (0xC02458, i_c02458),
    (0xC0245A, i_c0245a),
    (0xC0245C, i_c0245c),
    (0xC0245E, i_c0245e),
    (0xC02460, i_c02460),
    (0xC02462, i_c02462),
    (0xC02464, i_c02464),
    (0xC02466, i_c02466),
    (0xC02468, i_c02468),
    (0xC0246A, i_c0246a),
    (0xC0246C, i_c0246c),
    (0xC02470, i_c02470),
    (0xC02474, i_c02474),
    (0xC02478, i_c02478),
    (0xC0247C, i_c0247c),
    (0xC02480, i_c02480),
    (0xC02482, i_c02482),
    (0xC02488, i_c02488),
    (0xC0248C, i_c0248c),
    (0xC0248E, i_c0248e),
    (0xC02490, i_c02490),
    (0xC02492, i_c02492),
    (0xC02494, i_c02494),
    (0xC02496, i_c02496),
    (0xC02498, i_c02498),
    (0xC0249A, i_c0249a),
    (0xC0249C, i_c0249c),
    (0xC0249E, i_c0249e),
    (0xC024A0, i_c024a0),
    (0xC024A2, i_c024a2),
    (0xC024A4, i_c024a4),
    (0xC024A6, i_c024a6),
    (0xC024A8, i_c024a8),
    (0xC024AA, i_c024aa),
    (0xC024AC, i_c024ac),
    (0xC024AE, i_c024ae),
    (0xC024B2, i_c024b2),
    (0xC024B6, i_c024b6),
    (0xC024BA, i_c024ba),
    (0xC024BE, i_c024be),
    (0xC024C2, i_c024c2),
    (0xC024C6, i_c024c6),
    (0xC024CA, i_c024ca),
    (0xC024CE, i_c024ce),
    (0xC024D0, i_c024d0),
    (0xC024D2, i_c024d2),
    (0xC024D4, i_c024d4),
    (0xC024D6, i_c024d6),
    (0xC024D8, i_c024d8),
    (0xC024DA, i_c024da),
    (0xC024DC, i_c024dc),
    (0xC024DE, i_c024de),
    (0xC024E0, i_c024e0),
    (0xC024E2, i_c024e2),
    (0xC024E4, i_c024e4),
    (0xC024E6, i_c024e6),
    (0xC024E8, i_c024e8),
    (0xC024EA, i_c024ea),
    (0xC024EC, i_c024ec),
    (0xC024EE, i_c024ee),
    (0xC024F0, i_c024f0),
    (0xC024F4, i_c024f4),
    (0xC024F8, i_c024f8),
    (0xC024FC, i_c024fc),
    (0xC02500, i_c02500),
    (0xC02502, i_c02502),
    (0xC02506, i_c02506),
    (0xC0250A, i_c0250a),
    (0xC0250E, i_c0250e),
    (0xC02510, i_c02510),
    (0xC02512, i_c02512),
    (0xC02514, i_c02514),
    (0xC02516, i_c02516),
    (0xC02518, i_c02518),
    (0xC0251A, i_c0251a),
    (0xC0251C, i_c0251c),
    (0xC0251E, i_c0251e),
    (0xC02520, i_c02520),
    (0xC02522, i_c02522),
    (0xC02524, i_c02524),
    (0xC02526, i_c02526),
    (0xC02528, i_c02528),
    (0xC0252A, i_c0252a),
    (0xC0252C, i_c0252c),
    (0xC0252E, i_c0252e),
    (0xC02530, i_c02530),
    (0xC02532, i_c02532),
    (0xC02534, i_c02534),
    (0xC02538, i_c02538),
    (0xC0253C, i_c0253c),
    (0xC02540, i_c02540),
    (0xC02542, i_c02542),
    (0xC02546, i_c02546),
    (0xC0254A, i_c0254a),
    (0xC0254E, i_c0254e),
    (0xC02550, i_c02550),
    (0xC02552, i_c02552),
    (0xC02554, i_c02554),
    (0xC02556, i_c02556),
    (0xC02558, i_c02558),
    (0xC0255A, i_c0255a),
    (0xC0255C, i_c0255c),
    (0xC0255E, i_c0255e),
    (0xC02560, i_c02560),
    (0xC02562, i_c02562),
    (0xC02564, i_c02564),
    (0xC02566, i_c02566),
    (0xC02568, i_c02568),
    (0xC0256A, i_c0256a),
    (0xC0256C, i_c0256c),
    (0xC0256E, i_c0256e),
    (0xC02570, i_c02570),
    (0xC02572, i_c02572),
    (0xC02574, i_c02574),
    (0xC02576, i_c02576),
    (0xC0257A, i_c0257a),
    (0xC0257E, i_c0257e),
    (0xC02582, i_c02582),
    (0xC02586, i_c02586),
    (0xC0258A, i_c0258a),
    (0xC0258E, i_c0258e),
    (0xC02590, i_c02590),
    (0xC02592, i_c02592),
    (0xC02594, i_c02594),
    (0xC02596, i_c02596),
    (0xC02598, i_c02598),
    (0xC0259A, i_c0259a),
    (0xC0259C, i_c0259c),
    (0xC0259E, i_c0259e),
    (0xC025A0, i_c025a0),
    (0xC025A2, i_c025a2),
    (0xC025A4, i_c025a4),
    (0xC025A6, i_c025a6),
    (0xC025A8, i_c025a8),
    (0xC025AA, i_c025aa),
    (0xC025AC, i_c025ac),
    (0xC025AE, i_c025ae),
    (0xC025B0, i_c025b0),
    (0xC025B2, i_c025b2),
    (0xC025B4, i_c025b4),
    (0xC025B6, i_c025b6),
    (0xC025B8, i_c025b8),
    (0xC025BC, i_c025bc),
    (0xC025C0, i_c025c0),
    (0xC025C2, i_c025c2),
    (0xC025C6, i_c025c6),
    (0xC025CA, i_c025ca),
    (0xC025CE, i_c025ce),
    (0xC025D2, i_c025d2),
    (0xC025D4, i_c025d4),
    (0xC025D6, i_c025d6),
    (0xC025D8, i_c025d8),
    (0xC025DA, i_c025da),
    (0xC025DC, i_c025dc),
    (0xC025DE, i_c025de),
    (0xC025E0, i_c025e0),
    (0xC025E2, i_c025e2),
    (0xC025E4, i_c025e4),
    (0xC025E6, i_c025e6),
    (0xC025E8, i_c025e8),
    (0xC025EA, i_c025ea),
    (0xC025EC, i_c025ec),
    (0xC025EE, i_c025ee),
    (0xC025F0, i_c025f0),
    (0xC025F2, i_c025f2),
    (0xC025F4, i_c025f4),
    (0xC025F6, i_c025f6),
    (0xC025F8, i_c025f8),
    (0xC025FA, i_c025fa),
    (0xC025FE, i_c025fe),
    (0xC02600, i_c02600),
    (0xC02602, i_c02602),
    (0xC02606, i_c02606),
    (0xC0260A, i_c0260a),
    (0xC0260E, i_c0260e),
    (0xC02612, i_c02612),
    (0xC02614, i_c02614),
    (0xC02616, i_c02616),
    (0xC02618, i_c02618),
    (0xC0261A, i_c0261a),
    (0xC0261C, i_c0261c),
    (0xC0261E, i_c0261e),
    (0xC02620, i_c02620),
    (0xC02622, i_c02622),
    (0xC02624, i_c02624),
    (0xC02626, i_c02626),
    (0xC02628, i_c02628),
    (0xC0262A, i_c0262a),
    (0xC0262C, i_c0262c),
    (0xC0262E, i_c0262e),
    (0xC02630, i_c02630),
    (0xC02632, i_c02632),
    (0xC02634, i_c02634),
    (0xC02636, i_c02636),
    (0xC02638, i_c02638),
    (0xC0263A, i_c0263a),
    (0xC0263C, i_c0263c),
    (0xC0263E, i_c0263e),
    (0xC02642, i_c02642),
    (0xC02646, i_c02646),
    (0xC0264A, i_c0264a),
    (0xC0264E, i_c0264e),
    (0xC02652, i_c02652),
    (0xC02654, i_c02654),
    (0xC02656, i_c02656),
    (0xC02658, i_c02658),
    (0xC0265A, i_c0265a),
    (0xC0265C, i_c0265c),
    (0xC0265E, i_c0265e),
    (0xC02660, i_c02660),
    (0xC02662, i_c02662),
    (0xC02664, i_c02664),
    (0xC02666, i_c02666),
    (0xC02668, i_c02668),
    (0xC0266A, i_c0266a),
    (0xC0266C, i_c0266c),
    (0xC0266E, i_c0266e),
    (0xC02670, i_c02670),
    (0xC02672, i_c02672),
    (0xC02674, i_c02674),
    (0xC02676, i_c02676),
    (0xC02678, i_c02678),
    (0xC0267A, i_c0267a),
    (0xC0267C, i_c0267c),
    (0xC0267E, i_c0267e),
    (0xC02680, i_c02680),
    (0xC02682, i_c02682),
    (0xC02686, i_c02686),
    (0xC0268A, i_c0268a),
    (0xC0268E, i_c0268e),
    (0xC02692, i_c02692),
    (0xC02694, i_c02694),
    (0xC02696, i_c02696),
    (0xC02698, i_c02698),
    (0xC0269A, i_c0269a),
    (0xC0269C, i_c0269c),
    (0xC0269E, i_c0269e),
    (0xC026A0, i_c026a0),
    (0xC026A2, i_c026a2),
    (0xC026A4, i_c026a4),
    (0xC026A6, i_c026a6),
    (0xC026A8, i_c026a8),
    (0xC026AA, i_c026aa),
    (0xC026AC, i_c026ac),
    (0xC026AE, i_c026ae),
    (0xC026B0, i_c026b0),
    (0xC026B2, i_c026b2),
    (0xC026B4, i_c026b4),
    (0xC026B6, i_c026b6),
    (0xC026B8, i_c026b8),
    (0xC026BA, i_c026ba),
    (0xC026BC, i_c026bc),
    (0xC026BE, i_c026be),
    (0xC026C0, i_c026c0),
    (0xC026C2, i_c026c2),
    (0xC026C6, i_c026c6),
    (0xC026CA, i_c026ca),
    (0xC026CE, i_c026ce),
    (0xC026D2, i_c026d2),
    (0xC026D4, i_c026d4),
    (0xC026D6, i_c026d6),
    (0xC026D8, i_c026d8),
    (0xC026DA, i_c026da),
    (0xC026DC, i_c026dc),
    (0xC026DE, i_c026de),
    (0xC026E0, i_c026e0),
    (0xC026E2, i_c026e2),
    (0xC026E4, i_c026e4),
    (0xC026E6, i_c026e6),
    (0xC026E8, i_c026e8),
    (0xC026EA, i_c026ea),
    (0xC026EC, i_c026ec),
    (0xC026EE, i_c026ee),
    (0xC026F0, i_c026f0),
    (0xC026F2, i_c026f2),
    (0xC026F4, i_c026f4),
    (0xC026F6, i_c026f6),
    (0xC026F8, i_c026f8),
    (0xC026FA, i_c026fa),
    (0xC026FC, i_c026fc),
    (0xC026FE, i_c026fe),
    (0xC02700, i_c02700),
    (0xC02702, i_c02702),
    (0xC02706, i_c02706),
    (0xC0270A, i_c0270a),
    (0xC0270E, i_c0270e),
    (0xC02710, i_c02710),
    (0xC02712, i_c02712),
    (0xC02714, i_c02714),
    (0xC02716, i_c02716),
    (0xC02718, i_c02718),
    (0xC0271A, i_c0271a),
    (0xC0271C, i_c0271c),
    (0xC0271E, i_c0271e),
    (0xC02720, i_c02720),
    (0xC02722, i_c02722),
    (0xC02724, i_c02724),
    (0xC02726, i_c02726),
    (0xC02728, i_c02728),
    (0xC0272A, i_c0272a),
    (0xC0272C, i_c0272c),
    (0xC0272E, i_c0272e),
    (0xC02730, i_c02730),
    (0xC02732, i_c02732),
    (0xC02734, i_c02734),
    (0xC02736, i_c02736),
    (0xC02738, i_c02738),
    (0xC0273A, i_c0273a),
    (0xC0273C, i_c0273c),
    (0xC0273E, i_c0273e),
    (0xC02740, i_c02740),
    (0xC02742, i_c02742),
    (0xC02746, i_c02746),
    (0xC0274A, i_c0274a),
    (0xC0274E, i_c0274e),
    (0xC02752, i_c02752),
    (0xC02756, i_c02756),
    (0xC02758, i_c02758),
    (0xC0275A, i_c0275a),
    (0xC0275C, i_c0275c),
    (0xC0275E, i_c0275e),
    (0xC02760, i_c02760),
    (0xC02762, i_c02762),
    (0xC02764, i_c02764),
    (0xC02766, i_c02766),
    (0xC02768, i_c02768),
    (0xC0276A, i_c0276a),
    (0xC0276C, i_c0276c),
    (0xC0276E, i_c0276e),
    (0xC02770, i_c02770),
    (0xC02772, i_c02772),
    (0xC02774, i_c02774),
    (0xC02776, i_c02776),
    (0xC02778, i_c02778),
    (0xC0277A, i_c0277a),
    (0xC0277C, i_c0277c),
    (0xC0277E, i_c0277e),
    (0xC02780, i_c02780),
    (0xC02782, i_c02782),
    (0xC02786, i_c02786),
    (0xC0278A, i_c0278a),
    (0xC0278E, i_c0278e),
    (0xC02792, i_c02792),
    (0xC02796, i_c02796),
    (0xC02798, i_c02798),
    (0xC0279A, i_c0279a),
    (0xC0279C, i_c0279c),
    (0xC0279E, i_c0279e),
    (0xC027A0, i_c027a0),
    (0xC027A2, i_c027a2),
    (0xC027A4, i_c027a4),
    (0xC027A6, i_c027a6),
    (0xC027A8, i_c027a8),
    (0xC027AA, i_c027aa),
    (0xC027AC, i_c027ac),
    (0xC027AE, i_c027ae),
    (0xC027B0, i_c027b0),
    (0xC027B2, i_c027b2),
    (0xC027B4, i_c027b4),
    (0xC027B6, i_c027b6),
    (0xC027B8, i_c027b8),
    (0xC027BA, i_c027ba),
    (0xC027BC, i_c027bc),
    (0xC027BE, i_c027be),
    (0xC027C0, i_c027c0),
    (0xC027C2, i_c027c2),
    (0xC027C6, i_c027c6),
    (0xC027CA, i_c027ca),
    (0xC027CE, i_c027ce),
    (0xC027D2, i_c027d2),
    (0xC027D6, i_c027d6),
    (0xC027D8, i_c027d8),
    (0xC027DA, i_c027da),
    (0xC027DC, i_c027dc),
    (0xC027DE, i_c027de),
    (0xC027E0, i_c027e0),
    (0xC027E2, i_c027e2),
    (0xC027E4, i_c027e4),
    (0xC027E6, i_c027e6),
    (0xC027E8, i_c027e8),
    (0xC027EA, i_c027ea),
    (0xC027EC, i_c027ec),
    (0xC027EE, i_c027ee),
    (0xC027F0, i_c027f0),
    (0xC027F2, i_c027f2),
    (0xC027F4, i_c027f4),
    (0xC027F6, i_c027f6),
    (0xC027F8, i_c027f8),
    (0xC027FA, i_c027fa),
    (0xC027FC, i_c027fc),
    (0xC027FE, i_c027fe),
    (0xC02800, i_c02800),
    (0xC02802, i_c02802),
    (0xC02806, i_c02806),
    (0xC0280A, i_c0280a),
    (0xC0280E, i_c0280e),
    (0xC02812, i_c02812),
    (0xC02816, i_c02816),
    (0xC02818, i_c02818),
    (0xC0281A, i_c0281a),
    (0xC0281C, i_c0281c),
    (0xC0281E, i_c0281e),
    (0xC02820, i_c02820),
    (0xC02822, i_c02822),
    (0xC02824, i_c02824),
    (0xC02826, i_c02826),
    (0xC02828, i_c02828),
    (0xC0282A, i_c0282a),
    (0xC0282C, i_c0282c),
    (0xC0282E, i_c0282e),
    (0xC02830, i_c02830),
    (0xC02832, i_c02832),
    (0xC02834, i_c02834),
    (0xC02836, i_c02836),
    (0xC02838, i_c02838),
    (0xC0283A, i_c0283a),
    (0xC0283C, i_c0283c),
    (0xC0283E, i_c0283e),
    (0xC02840, i_c02840),
    (0xC02842, i_c02842),
    (0xC02846, i_c02846),
    (0xC0284A, i_c0284a),
    (0xC0284E, i_c0284e),
    (0xC02852, i_c02852),
    (0xC02856, i_c02856),
    (0xC02858, i_c02858),
    (0xC0285A, i_c0285a),
    (0xC0285C, i_c0285c),
    (0xC0285E, i_c0285e),
    (0xC02860, i_c02860),
    (0xC02862, i_c02862),
    (0xC02864, i_c02864),
    (0xC02866, i_c02866),
    (0xC02868, i_c02868),
    (0xC0286A, i_c0286a),
    (0xC0286C, i_c0286c),
    (0xC0286E, i_c0286e),
    (0xC02870, i_c02870),
    (0xC02872, i_c02872),
    (0xC02874, i_c02874),
    (0xC02876, i_c02876),
    (0xC02878, i_c02878),
    (0xC0287A, i_c0287a),
    (0xC0287C, i_c0287c),
    (0xC0287E, i_c0287e),
    (0xC02880, i_c02880),
    (0xC02882, i_c02882),
    (0xC02886, i_c02886),
    (0xC0288A, i_c0288a),
    (0xC0288E, i_c0288e),
    (0xC02892, i_c02892),
    (0xC02896, i_c02896),
    (0xC0289A, i_c0289a),
    (0xC0289C, i_c0289c),
    (0xC0289E, i_c0289e),
    (0xC028A0, i_c028a0),
    (0xC028A2, i_c028a2),
    (0xC028A4, i_c028a4),
    (0xC028A6, i_c028a6),
    (0xC028A8, i_c028a8),
    (0xC028AA, i_c028aa),
    (0xC028AC, i_c028ac),
    (0xC028AE, i_c028ae),
    (0xC028B0, i_c028b0),
    (0xC028B2, i_c028b2),
    (0xC028B4, i_c028b4),
    (0xC028B6, i_c028b6),
    (0xC028B8, i_c028b8),
    (0xC028BA, i_c028ba),
    (0xC028BC, i_c028bc),
    (0xC028BE, i_c028be),
    (0xC028C0, i_c028c0),
    (0xC028C2, i_c028c2),
    (0xC028C4, i_c028c4),
    (0xC028C8, i_c028c8),
    (0xC028CC, i_c028cc),
    (0xC028D0, i_c028d0),
    (0xC028D4, i_c028d4),
    (0xC028D8, i_c028d8),
    (0xC028DA, i_c028da),
    (0xC028DC, i_c028dc),
    (0xC028DE, i_c028de),
    (0xC028E0, i_c028e0),
    (0xC028E2, i_c028e2),
    (0xC028E4, i_c028e4),
    (0xC028E6, i_c028e6),
    (0xC028E8, i_c028e8),
    (0xC028EA, i_c028ea),
    (0xC028EC, i_c028ec),
    (0xC028EE, i_c028ee),
    (0xC028F0, i_c028f0),
    (0xC028F2, i_c028f2),
    (0xC028F4, i_c028f4),
    (0xC028F6, i_c028f6),
    (0xC028F8, i_c028f8),
    (0xC028FA, i_c028fa),
    (0xC028FC, i_c028fc),
    (0xC028FE, i_c028fe),
    (0xC02900, i_c02900),
    (0xC02902, i_c02902),
    (0xC02904, i_c02904),
    (0xC02908, i_c02908),
    (0xC0290C, i_c0290c),
    (0xC02910, i_c02910),
    (0xC02914, i_c02914),
    (0xC02918, i_c02918),
    (0xC0291A, i_c0291a),
    (0xC0291C, i_c0291c),
    (0xC0291E, i_c0291e),
    (0xC02920, i_c02920),
    (0xC02922, i_c02922),
    (0xC02924, i_c02924),
    (0xC02926, i_c02926),
    (0xC02928, i_c02928),
    (0xC0292A, i_c0292a),
    (0xC0292C, i_c0292c),
    (0xC0292E, i_c0292e),
    (0xC02930, i_c02930),
    (0xC02932, i_c02932),
    (0xC02934, i_c02934),
    (0xC02936, i_c02936),
    (0xC02938, i_c02938),
    (0xC0293A, i_c0293a),
    (0xC0293C, i_c0293c),
    (0xC0293E, i_c0293e),
    (0xC02940, i_c02940),
    (0xC02942, i_c02942),
    (0xC02944, i_c02944),
    (0xC02948, i_c02948),
    (0xC0294C, i_c0294c),
    (0xC02950, i_c02950),
    (0xC02954, i_c02954),
    (0xC02958, i_c02958),
    (0xC0295C, i_c0295c),
    (0xC0295E, i_c0295e),
    (0xC02960, i_c02960),
    (0xC02962, i_c02962),
    (0xC02964, i_c02964),
    (0xC02966, i_c02966),
    (0xC02968, i_c02968),
    (0xC0296A, i_c0296a),
    (0xC0296C, i_c0296c),
    (0xC0296E, i_c0296e),
    (0xC02970, i_c02970),
    (0xC02972, i_c02972),
    (0xC02974, i_c02974),
    (0xC02976, i_c02976),
    (0xC02978, i_c02978),
    (0xC0297A, i_c0297a),
    (0xC0297C, i_c0297c),
    (0xC0297E, i_c0297e),
    (0xC02980, i_c02980),
    (0xC02982, i_c02982),
    (0xC02984, i_c02984),
    (0xC02988, i_c02988),
    (0xC0298C, i_c0298c),
    (0xC02990, i_c02990),
    (0xC02994, i_c02994),
    (0xC02998, i_c02998),
    (0xC0299C, i_c0299c),
    (0xC0299E, i_c0299e),
    (0xC029A0, i_c029a0),
    (0xC029A2, i_c029a2),
    (0xC029A4, i_c029a4),
    (0xC029A6, i_c029a6),
    (0xC029A8, i_c029a8),
    (0xC029AA, i_c029aa),
    (0xC029AC, i_c029ac),
    (0xC029AE, i_c029ae),
    (0xC029B0, i_c029b0),
    (0xC029B2, i_c029b2),
    (0xC029B4, i_c029b4),
    (0xC029B6, i_c029b6),
    (0xC029B8, i_c029b8),
    (0xC029BA, i_c029ba),
    (0xC029BC, i_c029bc),
    (0xC029BE, i_c029be),
    (0xC029C0, i_c029c0),
    (0xC029C2, i_c029c2),
    (0xC029C4, i_c029c4),
    (0xC029C8, i_c029c8),
    (0xC029CC, i_c029cc),
    (0xC029D0, i_c029d0),
    (0xC029D4, i_c029d4),
    (0xC029D8, i_c029d8),
    (0xC029DC, i_c029dc),
    (0xC029DE, i_c029de),
    (0xC029E0, i_c029e0),
    (0xC029E2, i_c029e2),
    (0xC029E4, i_c029e4),
    (0xC029E6, i_c029e6),
    (0xC029E8, i_c029e8),
    (0xC029EA, i_c029ea),
    (0xC029EC, i_c029ec),
    (0xC029EE, i_c029ee),
    (0xC029F0, i_c029f0),
    (0xC029F2, i_c029f2),
    (0xC029F4, i_c029f4),
    (0xC029F6, i_c029f6),
    (0xC029F8, i_c029f8),
    (0xC029FA, i_c029fa),
    (0xC029FC, i_c029fc),
    (0xC029FE, i_c029fe),
    (0xC02A00, i_c02a00),
    (0xC02A02, i_c02a02),
    (0xC02A04, i_c02a04),
    (0xC02A08, i_c02a08),
    (0xC02A0C, i_c02a0c),
    (0xC02A10, i_c02a10),
    (0xC02A14, i_c02a14),
    (0xC02A18, i_c02a18),
    (0xC02A1C, i_c02a1c),
    (0xC02A1E, i_c02a1e),
    (0xC02A20, i_c02a20),
    (0xC02A22, i_c02a22),
    (0xC02A24, i_c02a24),
    (0xC02A26, i_c02a26),
    (0xC02A28, i_c02a28),
    (0xC02A2A, i_c02a2a),
    (0xC02A2C, i_c02a2c),
    (0xC02A2E, i_c02a2e),
    (0xC02A30, i_c02a30),
    (0xC02A32, i_c02a32),
    (0xC02A34, i_c02a34),
    (0xC02A36, i_c02a36),
    (0xC02A38, i_c02a38),
    (0xC02A3A, i_c02a3a),
    (0xC02A3C, i_c02a3c),
    (0xC02A3E, i_c02a3e),
    (0xC02A40, i_c02a40),
    (0xC02A42, i_c02a42),
    (0xC02A44, i_c02a44),
    (0xC02A48, i_c02a48),
    (0xC02A4C, i_c02a4c),
    (0xC02A50, i_c02a50),
    (0xC02A54, i_c02a54),
    (0xC02A58, i_c02a58),
    (0xC02A5C, i_c02a5c),
    (0xC02A5E, i_c02a5e),
    (0xC02A60, i_c02a60),
    (0xC02A62, i_c02a62),
    (0xC02A64, i_c02a64),
    (0xC02A66, i_c02a66),
    (0xC02A68, i_c02a68),
    (0xC02A6A, i_c02a6a),
    (0xC02A6C, i_c02a6c),
    (0xC02A6E, i_c02a6e),
    (0xC02A70, i_c02a70),
    (0xC02A72, i_c02a72),
    (0xC02A74, i_c02a74),
    (0xC02A76, i_c02a76),
    (0xC02A78, i_c02a78),
    (0xC02A7A, i_c02a7a),
    (0xC02A7C, i_c02a7c),
    (0xC02A7E, i_c02a7e),
    (0xC02A80, i_c02a80),
    (0xC02A82, i_c02a82),
    (0xC02A84, i_c02a84),
    (0xC02A88, i_c02a88),
    (0xC02A8C, i_c02a8c),
    (0xC02A90, i_c02a90),
    (0xC02A94, i_c02a94),
    (0xC02A98, i_c02a98),
    (0xC02A9C, i_c02a9c),
    (0xC02A9E, i_c02a9e),
    (0xC02AA0, i_c02aa0),
    (0xC02AA2, i_c02aa2),
    (0xC02AA4, i_c02aa4),
    (0xC02AA6, i_c02aa6),
    (0xC02AA8, i_c02aa8),
    (0xC02AAA, i_c02aaa),
    (0xC02AAC, i_c02aac),
    (0xC02AAE, i_c02aae),
    (0xC02AB0, i_c02ab0),
    (0xC02AB2, i_c02ab2),
    (0xC02AB4, i_c02ab4),
    (0xC02AB6, i_c02ab6),
    (0xC02AB8, i_c02ab8),
    (0xC02ABA, i_c02aba),
    (0xC02ABC, i_c02abc),
    (0xC02ABE, i_c02abe),
    (0xC02AC0, i_c02ac0),
    (0xC02AC2, i_c02ac2),
    (0xC02AC4, i_c02ac4),
    (0xC02AC8, i_c02ac8),
    (0xC02ACC, i_c02acc),
    (0xC02AD0, i_c02ad0),
    (0xC02AD4, i_c02ad4),
    (0xC02AD8, i_c02ad8),
    (0xC02ADC, i_c02adc),
    (0xC02AE0, i_c02ae0),
    (0xC02AE2, i_c02ae2),
    (0xC02AE4, i_c02ae4),
    (0xC02AE6, i_c02ae6),
    (0xC02AE8, i_c02ae8),
    (0xC02AEA, i_c02aea),
    (0xC02AEC, i_c02aec),
    (0xC02AEE, i_c02aee),
    (0xC02AF0, i_c02af0),
    (0xC02AF2, i_c02af2),
    (0xC02AF4, i_c02af4),
    (0xC02AF6, i_c02af6),
    (0xC02AF8, i_c02af8),
    (0xC02AFA, i_c02afa),
    (0xC02AFC, i_c02afc),
    (0xC02AFE, i_c02afe),
    (0xC02B00, i_c02b00),
    (0xC02B02, i_c02b02),
    (0xC02B04, i_c02b04),
    (0xC02B08, i_c02b08),
    (0xC02B0C, i_c02b0c),
    (0xC02B10, i_c02b10),
    (0xC02B14, i_c02b14),
];
