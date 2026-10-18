// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región bios_0) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $C00402: jmp $c10002.l
pub fn i_c00402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00408;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC10002;
    8
}
/// $C00408: jmp $c10002.l
pub fn i_c00408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0040E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC10002;
    8
}
/// $C0040E: jmp $c10002.l
pub fn i_c0040e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00414;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC10002;
    8
}
/// $C00414: jmp $c10002.l
pub fn i_c00414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0041A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC10002;
    8
}
/// $C0041A: jmp $c10002.l
pub fn i_c0041a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00420;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC10002;
    8
}
/// $C00420: jmp $c10002.l
pub fn i_c00420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00426;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC10002;
    8
}
/// $C00426: jmp $c10002.l
pub fn i_c00426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0042C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC10002;
    8
}
/// $C0042C: jmp $c10002.l
pub fn i_c0042c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00432;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC10002;
    8
}
/// $C00432: jmp $c10002.l
pub fn i_c00432(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00438;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC10002;
    8
}
/// $C00438: jmp $c10646.l
pub fn i_c00438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0043E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC10646;
    8
}
/// $C0043E: jmp $c108ca.l
pub fn i_c0043e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC00444;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC108CA;
    8
}
/// $C00444: jmp $c102d2.l
pub fn i_c00444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0044A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC102D2;
    8
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
/// $C004C2: jmp $c16ed0.l
pub fn i_c004c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC004C8;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC16ED0;
    8
}
/// $C004CE: jmp $c17290.l
pub fn i_c004ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC004D4;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC17290;
    8
}
/// $C10002: move.w #$2700, sr
pub fn i_c10002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10006;
    cpu.ir = 0x46FC;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let v1: u32 = 0x2700;
    write_sr(cpu, v1 as u16);
    12
}
/// $C10006: move.b d0, $300001.l
pub fn i_c10006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1000C;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C1000C: move.w #$7, $3c000c.l
pub fn i_c1000c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10014;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C000C, v1 as u16);
    16
}
/// $C10014: move.w #$4000, $3c0006.l
pub fn i_c10014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1001C;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0006, v1 as u16);
    16
}
/// $C1001C: lea.l $10f300.l, a5
pub fn i_c1001c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10022;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x10F300;
    20
}
/// $C10022: jmp $c10a84.l
pub fn i_c10022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10028;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC10A84;
    8
}
/// $C10028: bra.w $c10038
pub fn i_c10028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1002C;
    cpu.ir = 0x6000;
    cpu.pc = 0xC10038;
    10
}
/// $C1002C: move.w $c00400.l, $a82(a5)
pub fn i_c1002c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10034;
    cpu.ir = 0x3B79;
    let v2 = bus.read16(0xC00400) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[5].wrapping_add(0xA82);
    bus.write16(ad3, v2 as u16);
    24
}
/// $C10034: bra.w $c10136
pub fn i_c10034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10038;
    cpu.ir = 0x6000;
    cpu.pc = 0xC10136;
    10
}
/// $C10038: moveq #$0, d0
pub fn i_c10038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1003A;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1003A: move.l d0, d1
pub fn i_c1003a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1003C;
    cpu.ir = 0x2200;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[1] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1003C: move.l d0, d2
pub fn i_c1003c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1003E;
    cpu.ir = 0x2400;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[2] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1003E: move.l d0, d3
pub fn i_c1003e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10040;
    cpu.ir = 0x2600;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[3] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C10040: lea.l $110000.l, a0
pub fn i_c10040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10046;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x110000;
    20
}
/// $C10046: move.w #$1ff, d7
pub fn i_c10046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1004A;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x1FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1004A: move.b d0, $300001.l
pub fn i_c1004a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10050;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C10050: movem.l d0-d3, -(a0)
pub fn i_c10050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10054;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C10054: movem.l d0-d3, -(a0)
pub fn i_c10054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10058;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C10058: movem.l d0-d3, -(a0)
pub fn i_c10058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1005C;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C1005C: movem.l d0-d3, -(a0)
pub fn i_c1005c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10060;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C10060: movem.l d0-d3, -(a0)
pub fn i_c10060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10064;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C10064: movem.l d0-d3, -(a0)
pub fn i_c10064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10068;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C10068: movem.l d0-d3, -(a0)
pub fn i_c10068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1006C;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C1006C: movem.l d0-d3, -(a0)
pub fn i_c1006c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10070;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C10070: dbra d7, $c1004a
pub fn i_c10070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10074;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC10074; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC10074; return 14; }
    cpu.pc = 0xC1004A;
    10
}
/// $C10074: move.b d0, $3a000f.l
pub fn i_c10074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1007A;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A000F, v1 as u8);
    16
}
/// $C1007A: lea.l $402000.l, a0
pub fn i_c1007a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10080;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x402000;
    20
}
/// $C10080: move.w #$7f, d7
pub fn i_c10080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10084;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x7F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C10084: movem.l d0-d3, -(a0)
pub fn i_c10084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10088;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C10088: movem.l d0-d3, -(a0)
pub fn i_c10088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1008C;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C1008C: movem.l d0-d3, -(a0)
pub fn i_c1008c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10090;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C10090: movem.l d0-d3, -(a0)
pub fn i_c10090(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10094;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C10094: dbra d7, $c10084
pub fn i_c10094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10098;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC10098; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC10098; return 14; }
    cpu.pc = 0xC10084;
    10
}
/// $C10098: move.b d0, $300001.l
pub fn i_c10098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1009E;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C1009E: move.b d0, $3a001f.l
pub fn i_c1009e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC100A4;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001F, v1 as u8);
    16
}
/// $C100A4: lea.l $402000.l, a0
pub fn i_c100a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC100AA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x402000;
    20
}
/// $C100AA: move.w #$7f, d7
pub fn i_c100aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC100AE;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x7F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C100AE: movem.l d0-d3, -(a0)
pub fn i_c100ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC100B2;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C100B2: movem.l d0-d3, -(a0)
pub fn i_c100b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC100B6;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C100B6: movem.l d0-d3, -(a0)
pub fn i_c100b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC100BA;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C100BA: movem.l d0-d3, -(a0)
pub fn i_c100ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC100BE;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C100BE: dbra d7, $c100ae
pub fn i_c100be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC100C2;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC100C2; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC100C2; return 14; }
    cpu.pc = 0xC100AE;
    10
}
/// $C100C2: move.b d0, $300001.l
pub fn i_c100c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC100C8;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C100C8: jsr $c16ed0(pc)
pub fn i_c100c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC100CC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC100CC);
    cpu.pc = 0xC16ED0;
    16
}
/// $C100CC: lea.l $3c0002.l, a1
pub fn i_c100cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC100D2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x3C0002;
    20
}
/// $C100D2: move.w #$1, $2(a1)
pub fn i_c100d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC100D8;
    cpu.ir = 0x337C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C100D8: move.w d0, (a1)
pub fn i_c100d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC100DA;
    cpu.ir = 0x3280;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v1 as u16);
    8
}
/// $C100DA: move.b #$ff, d1
pub fn i_c100da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC100DE;
    cpu.ir = 0x123C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C100DE: move.w #$5f3, d7
pub fn i_c100de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC100E2;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x5F3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C100E2: move.b d0, $300001.l
pub fn i_c100e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC100E8;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C100E8: move.w d1, (a1)
pub fn i_c100e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC100EA;
    cpu.ir = 0x3281;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v1 as u16);
    8
}
/// $C100EA: swap d1
pub fn i_c100ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC100EC;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C100EC: move.w d1, (a1)
pub fn i_c100ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC100EE;
    cpu.ir = 0x3281;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v1 as u16);
    8
}
/// $C100EE: swap d1
pub fn i_c100ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC100F0;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C100F0: move.w d1, (a1)
pub fn i_c100f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC100F2;
    cpu.ir = 0x3281;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v1 as u16);
    8
}
/// $C100F2: swap d1
pub fn i_c100f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC100F4;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C100F4: move.w d1, (a1)
pub fn i_c100f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC100F6;
    cpu.ir = 0x3281;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v1 as u16);
    8
}
/// $C100F6: swap d1
pub fn i_c100f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC100F8;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C100F8: move.w d1, (a1)
pub fn i_c100f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC100FA;
    cpu.ir = 0x3281;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v1 as u16);
    8
}
/// $C100FA: swap d1
pub fn i_c100fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC100FC;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C100FC: move.w d1, (a1)
pub fn i_c100fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC100FE;
    cpu.ir = 0x3281;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v1 as u16);
    8
}
/// $C100FE: swap d1
pub fn i_c100fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10100;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C10100: move.w d1, (a1)
pub fn i_c10100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10102;
    cpu.ir = 0x3281;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v1 as u16);
    8
}
/// $C10102: swap d1
pub fn i_c10102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10104;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C10104: move.w d1, (a1)
pub fn i_c10104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10106;
    cpu.ir = 0x3281;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v1 as u16);
    8
}
/// $C10106: swap d1
pub fn i_c10106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10108;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C10108: move.w d1, (a1)
pub fn i_c10108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1010A;
    cpu.ir = 0x3281;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v1 as u16);
    8
}
/// $C1010A: swap d1
pub fn i_c1010a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1010C;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1010C: move.w d1, (a1)
pub fn i_c1010c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1010E;
    cpu.ir = 0x3281;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v1 as u16);
    8
}
/// $C1010E: swap d1
pub fn i_c1010e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10110;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C10110: move.w d1, (a1)
pub fn i_c10110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10112;
    cpu.ir = 0x3281;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v1 as u16);
    8
}
/// $C10112: swap d1
pub fn i_c10112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10114;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C10114: move.w d1, (a1)
pub fn i_c10114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10116;
    cpu.ir = 0x3281;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v1 as u16);
    8
}
/// $C10116: swap d1
pub fn i_c10116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10118;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C10118: move.w d1, (a1)
pub fn i_c10118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1011A;
    cpu.ir = 0x3281;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v1 as u16);
    8
}
/// $C1011A: swap d1
pub fn i_c1011a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1011C;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1011C: move.w d1, (a1)
pub fn i_c1011c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1011E;
    cpu.ir = 0x3281;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v1 as u16);
    8
}
/// $C1011E: swap d1
pub fn i_c1011e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10120;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C10120: move.w d1, (a1)
pub fn i_c10120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10122;
    cpu.ir = 0x3281;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v1 as u16);
    8
}
/// $C10122: swap d1
pub fn i_c10122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10124;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C10124: move.w d1, (a1)
pub fn i_c10124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10126;
    cpu.ir = 0x3281;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v1 as u16);
    8
}
/// $C10126: swap d1
pub fn i_c10126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10128;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C10128: dbra d7, $c100e2
pub fn i_c10128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1012C;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC1012C; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC1012C; return 14; }
    cpu.pc = 0xC100E2;
    10
}
/// $C1012C: jsr $c16f0c.l
pub fn i_c1012c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10132;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0xC10132);
    cpu.pc = 0xC16F0C;
    16
}
/// $C10132: bra.w $c1002c
pub fn i_c10132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10136;
    cpu.ir = 0x6000;
    cpu.pc = 0xC1002C;
    10
}
/// $C10136: move.l #$10ff00, $abe(a5)
pub fn i_c10136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1013E;
    cpu.ir = 0x2B7C;
    let v1: u32 = 0x10FF00;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1013E: jsr $c16fd4(pc)
pub fn i_c1013e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10142;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10142);
    cpu.pc = 0xC16FD4;
    16
}
/// $C10142: move.b d0, $300001.l
pub fn i_c10142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10148;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C10148: movea.l #$10f300, a7
pub fn i_c10148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1014E;
    cpu.ir = 0x2E7C;
    let v1: u32 = 0x10F300;
    cpu.a[7] = v1;
    4
}
/// $C1014E: lea.l $10f300.l, a5
pub fn i_c1014e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10154;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x10F300;
    20
}
/// $C10154: bclr.b #$7, $a80(a5)
pub fn i_c10154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1015A;
    cpu.ir = 0x8AD;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad1 = cpu.a[5].wrapping_add(0xA80);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $C1015A: moveq #$0, d0
pub fn i_c1015a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1015C;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1015C: move.b $a81(a5), d0
pub fn i_c1015c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10160;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xA81);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C10160: lsl.w #$2, d0
pub fn i_c10160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10162;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $C10162: movea.l $c10168(pc, d0.w), a0
pub fn i_c10162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10166;
    cpu.ir = 0x207B;
    let ad2 = 0xC10168_u32.wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $C10166: jmp (a0)
pub fn i_c10166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10168;
    cpu.ir = 0x4ED0;
    cpu.jump_to(cpu.a[0]);
    8
}
/// $C10174: move.b #$1, $bbf(a5)
pub fn i_c10174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1017A;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xBBF);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C1017A: jsr $c111a0(pc)
pub fn i_c1017a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1017E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1017E);
    cpu.pc = 0xC111A0;
    16
}
/// $C1017E: clr.b $bbf(a5)
pub fn i_c1017e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10182;
    cpu.ir = 0x422D;
    let ad1 = cpu.a[5].wrapping_add(0xBBF);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C10182: cmpi.b #$8, $d00109.l
pub fn i_c10182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1018A;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0xD00109) as u32;
    sub_with_flags(cpu, v2, 0x8, Size::Byte, false);
    20
}
/// $C1018A: beq.w $c10368
pub fn i_c1018a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1018E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC10368; return 10; }
    8
}
/// $C1018E: jsr $c10612(pc)
pub fn i_c1018e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10192;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10192);
    cpu.pc = 0xC10612;
    16
}
/// $C10192: clr.b $aaf(a5)
pub fn i_c10192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10196;
    cpu.ir = 0x422D;
    let ad1 = cpu.a[5].wrapping_add(0xAAF);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C10196: move.b #$0, d0
pub fn i_c10196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1019A;
    cpu.ir = 0x103C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C1019A: move.b d0, d1
pub fn i_c1019a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1019C;
    cpu.ir = 0x1200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C1019C: bra.w $c10286
pub fn i_c1019c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC101A0;
    cpu.ir = 0x6000;
    cpu.pc = 0xC10286;
    10
}
/// $C101A0: move.b #$a, $be1(a5)
pub fn i_c101a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC101A6;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xBE1);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C101A6: clr.b $aaf(a5)
pub fn i_c101a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC101AA;
    cpu.ir = 0x422D;
    let ad1 = cpu.a[5].wrapping_add(0xAAF);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C101AA: clr.b $9ec(a5)
pub fn i_c101aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC101AE;
    cpu.ir = 0x422D;
    let ad1 = cpu.a[5].wrapping_add(0x9EC);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C101AE: move.b $114.w, d0
pub fn i_c101ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC101B2;
    cpu.ir = 0x1038;
    let v2 = bus.read8(0x114) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    12
}
/// $C101B2: beq.b $c101c2
pub fn i_c101b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC101B4;
    cpu.ir = 0x670E;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC101C2; return 10; }
    8
}
/// $C101B4: subq.b #$1, d0
pub fn i_c101b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC101B6;
    cpu.ir = 0x5300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C101B6: bne.b $c101d2
pub fn i_c101b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC101B8;
    cpu.ir = 0x661A;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC101D2; return 10; }
    8
}
/// $C101B8: move.b #$1, d0
pub fn i_c101b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC101BC;
    cpu.ir = 0x103C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C101BC: move.b d0, d1
pub fn i_c101bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC101BE;
    cpu.ir = 0x1200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C101BE: bra.w $c10286
pub fn i_c101be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC101C2;
    cpu.ir = 0x6000;
    cpu.pc = 0xC10286;
    10
}
/// $C101C2: move.b #$1, $a80(a5)
pub fn i_c101c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC101C8;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xA80);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C101C8: clr.b $bbf(a5)
pub fn i_c101c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC101CC;
    cpu.ir = 0x422D;
    let ad1 = cpu.a[5].wrapping_add(0xBBF);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C101CC: jmp $c169b0.l
pub fn i_c101cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC101D2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC169B0;
    8
}
/// $C101D2: move.w #$2, $a80(a5)
pub fn i_c101d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC101D8;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xA80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C101D8: bra.w $c10142
pub fn i_c101d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC101DC;
    cpu.ir = 0x6000;
    cpu.pc = 0xC10142;
    10
}
/// $C101EA: tst.w $d00060.l
pub fn i_c101ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC101F0;
    cpu.ir = 0x4A79;
    let v2 = bus.read16(0xD00060) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    16
}
/// $C101F0: beq.w $c10260
pub fn i_c101f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC101F4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC10260; return 10; }
    8
}
/// $C101F4: move.b #$ff, $10fee1.l
pub fn i_c101f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC101FC;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10FEE1, v1 as u8);
    16
}
/// $C101FC: move.b d0, $3a0003.l
pub fn i_c101fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10202;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0003, v1 as u8);
    16
}
/// $C10202: move.w #$12c, $9d8(a5)
pub fn i_c10202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10208;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x12C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C10208: move.l #$eee0000, $400002.l
pub fn i_c10208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10212;
    cpu.ir = 0x23FC;
    let v1: u32 = 0xEEE0000;
    bus.write32(0x400002, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C10212: move.b #$1, $ac2(a5)
pub fn i_c10212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10218;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAC2);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C10218: lea.l $c00(a5), a0
pub fn i_c10218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1021C;
    cpu.ir = 0x41ED;
    let ad1 = cpu.a[5].wrapping_add(0xC00);
    cpu.a[0] = ad1;
    16
}
/// $C1021C: move.l #$c101dc, (a0)+
pub fn i_c1021c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10222;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC101DC;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C10222: clr.l (a0)+
pub fn i_c10222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10224;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C10224: move.w #$5, (a0)+
pub fn i_c10224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10228;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C10228: move.w #$9f, (a0)+
pub fn i_c10228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1022C;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x9F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C1022C: move.w #$108, (a0)+
pub fn i_c1022c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10230;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x108;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C10230: move.b $d00108.l, d0
pub fn i_c10230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10236;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0xD00108) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C10236: addi.b #$31, d0
pub fn i_c10236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1023A;
    cpu.ir = 0x600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x31, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1023A: move.b d0, (a0)+
pub fn i_c1023a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1023C;
    cpu.ir = 0x10C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C1023C: move.b #$ff, (a0)+
pub fn i_c1023c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10240;
    cpu.ir = 0x10FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C10240: clr.w (a0)+
pub fn i_c10240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10242;
    cpu.ir = 0x4258;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C10242: move.l a0, $abe(a5)
pub fn i_c10242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10246;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C10246: clr.b $ac2(a5)
pub fn i_c10246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1024A;
    cpu.ir = 0x422D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1024A: move.w #$2000, sr
pub fn i_c1024a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1024E;
    cpu.ir = 0x46FC;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let v1: u32 = 0x2000;
    write_sr(cpu, v1 as u16);
    12
}
/// $C1024E: jsr $c1727e(pc)
pub fn i_c1024e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10252;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10252);
    cpu.pc = 0xC1727E;
    16
}
/// $C10252: btst.b #$1, $bdd(a5)
pub fn i_c10252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10258;
    cpu.ir = 0x82D;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[5].wrapping_add(0xBDD);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $C10258: bne.b $c10260
pub fn i_c10258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1025A;
    cpu.ir = 0x6606;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC10260; return 10; }
    8
}
/// $C1025A: subq.w #$1, $9d8(a5)
pub fn i_c1025a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1025E;
    cpu.ir = 0x536D;
    let ad1 = cpu.a[5].wrapping_add(0x9D8);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $C1025E: bne.b $c1024e
pub fn i_c1025e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10260;
    cpu.ir = 0x66EE;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1024E; return 10; }
    8
}
/// $C10260: move.b #$a, $be1(a5)
pub fn i_c10260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10266;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xBE1);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C10266: move.b #$1, $aaf(a5)
pub fn i_c10266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1026C;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAAF);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C1026C: move.b #$2, d1
pub fn i_c1026c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10270;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C10270: move.b d1, d0
pub fn i_c10270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10272;
    cpu.ir = 0x1001;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C10272: tst.b $d00043.l
pub fn i_c10272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10278;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0xD00043) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $C10278: bne.b $c10286
pub fn i_c10278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1027A;
    cpu.ir = 0x660C;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC10286; return 10; }
    8
}
/// $C1027A: tst.w $d00034.l
pub fn i_c1027a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10280;
    cpu.ir = 0x4A79;
    let v2 = bus.read16(0xD00034) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    16
}
/// $C10280: beq.b $c10286
pub fn i_c10280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10282;
    cpu.ir = 0x6704;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC10286; return 10; }
    8
}
/// $C10282: move.b #$3, d0
pub fn i_c10282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10286;
    cpu.ir = 0x103C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C10286: clr.l $ab6(a5)
pub fn i_c10286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1028A;
    cpu.ir = 0x42AD;
    let ad1 = cpu.a[5].wrapping_add(0xAB6);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $C1028A: clr.l $aba(a5)
pub fn i_c1028a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1028E;
    cpu.ir = 0x42AD;
    let ad1 = cpu.a[5].wrapping_add(0xABA);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $C1028E: move.w #$7, $3c000c.l
pub fn i_c1028e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10296;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C000C, v1 as u16);
    16
}
/// $C10296: move.w #$2000, sr
pub fn i_c10296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1029A;
    cpu.ir = 0x46FC;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let v1: u32 = 0x2000;
    write_sr(cpu, v1 as u16);
    12
}
/// $C1029A: move.b #$3, $320000.l
pub fn i_c1029a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC102A2;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x320000, v1 as u8);
    16
}
/// $C102A2: jsr $c1727e(pc)
pub fn i_c102a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC102A6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC102A6);
    cpu.pc = 0xC1727E;
    16
}
/// $C102A6: move.w #$2700, sr
pub fn i_c102a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC102AA;
    cpu.ir = 0x46FC;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let v1: u32 = 0x2700;
    write_sr(cpu, v1 as u16);
    12
}
/// $C102AA: move.w #$4000, $3c0006.l
pub fn i_c102aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC102B2;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0006, v1 as u16);
    16
}
/// $C102B2: move.b d0, $aae(a5)
pub fn i_c102b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC102B6;
    cpu.ir = 0x1B40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAAE);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C102B6: ori.b #$80, d1
pub fn i_c102b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC102BA;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x80) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C102BA: move.b d1, $a80(a5)
pub fn i_c102ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC102BE;
    cpu.ir = 0x1B41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xA80);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C102BE: clr.b $bbf(a5)
pub fn i_c102be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC102C2;
    cpu.ir = 0x422D;
    let ad1 = cpu.a[5].wrapping_add(0xBBF);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C102C2: move.b d0, $3a0013.l
pub fn i_c102c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC102C8;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0013, v1 as u8);
    16
}
/// $C102C8: move.b d0, $3a000d.l
pub fn i_c102c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC102CE;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A000D, v1 as u8);
    16
}
/// $C102CE: jmp $122.w
pub fn i_c102ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC102D2;
    cpu.ir = 0x4EF8;
    cpu.pc = 0x122;
    8
}
/// $C102D2: move.b #$ff, $10fee1.l
pub fn i_c102d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC102DA;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10FEE1, v1 as u8);
    16
}
/// $C102DA: move.w #$4000, $3c0006.l
pub fn i_c102da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC102E2;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0006, v1 as u16);
    16
}
/// $C102E2: bclr.b #$7, $10fd80.l
pub fn i_c102e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC102EA;
    cpu.ir = 0x8B9;
    let mk: u32 = 1u32 << ((7) % 8);
    let v1 = bus.read8(0x10FD80) as u32;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 & !mk;
    bus.write8(0x10FD80, r2 as u8);
    20
}
/// $C102EA: move.b d0, $300001.l
pub fn i_c102ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC102F0;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C102F0: move.w #$7, $3c000c.l
pub fn i_c102f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC102F8;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C000C, v1 as u16);
    16
}
/// $C102F8: move.w #$2000, sr
pub fn i_c102f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC102FC;
    cpu.ir = 0x46FC;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let v1: u32 = 0x2000;
    write_sr(cpu, v1 as u16);
    12
}
/// $C102FC: jsr $c1727e(pc)
pub fn i_c102fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10300;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10300);
    cpu.pc = 0xC1727E;
    16
}
/// $C10300: move.b d0, $3a0003.l
pub fn i_c10300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10306;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0003, v1 as u8);
    16
}
/// $C10306: lea.l $10f300.l, a7
pub fn i_c10306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1030C;
    cpu.ir = 0x4FF9;
    cpu.a[7] = 0x10F300;
    20
}
/// $C1030C: moveq #$0, d0
pub fn i_c1030c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1030E;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1030E: move.l d0, d1
pub fn i_c1030e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10310;
    cpu.ir = 0x2200;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[1] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C10310: move.l d0, d2
pub fn i_c10310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10312;
    cpu.ir = 0x2400;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[2] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C10312: move.l d0, d3
pub fn i_c10312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10314;
    cpu.ir = 0x2600;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[3] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C10314: move.l d0, d4
pub fn i_c10314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10316;
    cpu.ir = 0x2800;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[4] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C10316: move.l d0, d5
pub fn i_c10316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10318;
    cpu.ir = 0x2A00;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[5] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C10318: move.l d0, d6
pub fn i_c10318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1031A;
    cpu.ir = 0x2C00;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[6] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1031A: movea.l d0, a0
pub fn i_c1031a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1031C;
    cpu.ir = 0x2040;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C1031C: move.w #$1f, d7
pub fn i_c1031c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10320;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x1F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C10320: lea.l $402000.l, a1
pub fn i_c10320(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10326;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x402000;
    20
}
/// $C10326: movem.l d0-d6/a0, -(a1)
pub fn i_c10326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1032A;
    cpu.ir = 0x48E1;
    let mut a = cpu.a[1];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[1] = a;
    20
}
/// $C1032A: movem.l d0-d6/a0, -(a1)
pub fn i_c1032a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1032E;
    cpu.ir = 0x48E1;
    let mut a = cpu.a[1];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[1] = a;
    20
}
/// $C1032E: movem.l d0-d6/a0, -(a1)
pub fn i_c1032e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10332;
    cpu.ir = 0x48E1;
    let mut a = cpu.a[1];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[1] = a;
    20
}
/// $C10332: movem.l d0-d6/a0, -(a1)
pub fn i_c10332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10336;
    cpu.ir = 0x48E1;
    let mut a = cpu.a[1];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[1] = a;
    20
}
/// $C10336: movem.l d0-d6/a0, -(a1)
pub fn i_c10336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1033A;
    cpu.ir = 0x48E1;
    let mut a = cpu.a[1];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[1] = a;
    20
}
/// $C1033A: movem.l d0-d6/a0, -(a1)
pub fn i_c1033a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1033E;
    cpu.ir = 0x48E1;
    let mut a = cpu.a[1];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[1] = a;
    20
}
/// $C1033E: movem.l d0-d6/a0, -(a1)
pub fn i_c1033e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10342;
    cpu.ir = 0x48E1;
    let mut a = cpu.a[1];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[1] = a;
    20
}
/// $C10342: movem.l d0-d6/a0, -(a1)
pub fn i_c10342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10346;
    cpu.ir = 0x48E1;
    let mut a = cpu.a[1];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[1] = a;
    20
}
/// $C10346: dbra d7, $c10326
pub fn i_c10346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1034A;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC1034A; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC1034A; return 14; }
    cpu.pc = 0xC10326;
    10
}
/// $C1034A: jsr $c16fd4(pc)
pub fn i_c1034a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1034E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1034E);
    cpu.pc = 0xC16FD4;
    16
}
/// $C1034E: lea.l $10f300.l, a5
pub fn i_c1034e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10354;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x10F300;
    20
}
/// $C10354: clr.b $ac2(a5)
pub fn i_c10354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10358;
    cpu.ir = 0x422D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C10358: move.l #$10ff00, $abe(a5)
pub fn i_c10358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10360;
    cpu.ir = 0x2B7C;
    let v1: u32 = 0x10FF00;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C10360: jsr $c12a0e(pc)
pub fn i_c10360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10364;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10364);
    cpu.pc = 0xC12A0E;
    16
}
/// $C10364: jsr $c16f0c(pc)
pub fn i_c10364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10368;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10368);
    cpu.pc = 0xC16F0C;
    16
}
/// $C10368: pea.l $c10142(pc)
pub fn i_c10368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1036C;
    cpu.ir = 0x487A;
    cpu.push32(bus, 0xC10142);
    24
}
/// $C1036C: moveq #$0, d0
pub fn i_c1036c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1036E;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1036E: move.b $a81(a5), d0
pub fn i_c1036e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10372;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xA81);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C10372: lsl.w #$2, d0
pub fn i_c10372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10374;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $C10374: movea.l $c1037a(pc, d0.w), a0
pub fn i_c10374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10378;
    cpu.ir = 0x207B;
    let ad2 = 0xC1037A_u32.wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $C10378: jmp (a0)
pub fn i_c10378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1037A;
    cpu.ir = 0x4ED0;
    cpu.jump_to(cpu.a[0]);
    8
}
/// $C10396: move.b #$1, $bbf(a5)
pub fn i_c10396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1039C;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xBBF);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C1039C: move.b $d00109.l, d0
pub fn i_c1039c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC103A2;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0xD00109) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C103A2: cmpi.b #$8, d0
pub fn i_c103a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC103A6;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x8, Size::Byte, false);
    8
}
/// $C103A6: beq.b $c103de
pub fn i_c103a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC103A8;
    cpu.ir = 0x6736;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC103DE; return 10; }
    8
}
/// $C103A8: jsr $c112a2(pc)
pub fn i_c103a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC103AC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC103AC);
    cpu.pc = 0xC112A2;
    16
}
/// $C103AC: jsr $c112ec(pc)
pub fn i_c103ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC103B0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC103B0);
    cpu.pc = 0xC112EC;
    16
}
/// $C103B0: move.b d0, $3a001d.l
pub fn i_c103b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC103B6;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001D, v1 as u8);
    16
}
/// $C103B6: move.b $d00109.l, d0
pub fn i_c103b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC103BC;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0xD00109) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C103BC: addq.b #$1, d0
pub fn i_c103bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC103BE;
    cpu.ir = 0x5200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C103BE: move.b d0, $d00109.l
pub fn i_c103be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC103C4;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0xD00109, v1 as u8);
    16
}
/// $C103C4: move.b d0, $3a000d.l
pub fn i_c103c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC103CA;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A000D, v1 as u8);
    16
}
/// $C103CA: cmpi.b #$8, d0
pub fn i_c103ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC103CE;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x8, Size::Byte, false);
    8
}
/// $C103CE: bcc.b $c103de
pub fn i_c103ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC103D0;
    cpu.ir = 0x640E;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC103DE; return 10; }
    8
}
/// $C103D0: bra.b $c103d8
pub fn i_c103d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC103D2;
    cpu.ir = 0x6006;
    cpu.pc = 0xC103D8;
    10
}
/// $C103D2: move.b #$1, $a81(a5)
pub fn i_c103d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC103D8;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xA81);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C103D8: clr.b $bbf(a5)
pub fn i_c103d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC103DC;
    cpu.ir = 0x422D;
    let ad1 = cpu.a[5].wrapping_add(0xBBF);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C103DC: rts
pub fn i_c103dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC103DE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C103DE: move.b $300001.l, d0
pub fn i_c103de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC103E4;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x300001) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C103E4: andi.b #$1, d0
pub fn i_c103e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC103E8;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C103E8: beq.w $c17ae6
pub fn i_c103e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC103EC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC17AE6; return 10; }
    8
}
/// $C103EC: tst.w $d00060.l
pub fn i_c103ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC103F2;
    cpu.ir = 0x4A79;
    let v2 = bus.read16(0xD00060) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    16
}
/// $C103F2: bne.w $c10418
pub fn i_c103f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC103F6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC10418; return 10; }
    8
}
/// $C103F6: move.b $d00058.l, d5
pub fn i_c103f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC103FC;
    cpu.ir = 0x1A39;
    let v2 = bus.read8(0xD00058) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C103FC: bmi.b $c10418
pub fn i_c103fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC103FE;
    cpu.ir = 0x6B1A;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0xC10418; return 10; }
    8
}
/// $C103FE: move.b d0, $3a001d.l
pub fn i_c103fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10404;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001D, v1 as u8);
    16
}
/// $C10404: jsr $c10f90(pc)
pub fn i_c10404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10408;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10408);
    cpu.pc = 0xC10F90;
    16
}
/// $C10408: move.b #$3, $320000.l
pub fn i_c10408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10410;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x320000, v1 as u8);
    16
}
/// $C10410: move.b d0, $3a000d.l
pub fn i_c10410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10416;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A000D, v1 as u8);
    16
}
/// $C10416: bra.b $c103d2
pub fn i_c10416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10418;
    cpu.ir = 0x60BA;
    cpu.pc = 0xC103D2;
    10
}
/// $C10418: move.b #$2, $a81(a5)
pub fn i_c10418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1041E;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xA81);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C1041E: jsr $c1138e(pc)
pub fn i_c1041e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10422;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10422);
    cpu.pc = 0xC1138E;
    16
}
/// $C10422: move.b $d00108.l, d5
pub fn i_c10422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10428;
    cpu.ir = 0x1A39;
    let v2 = bus.read8(0xD00108) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C10428: clr.b $bbf(a5)
pub fn i_c10428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1042C;
    cpu.ir = 0x422D;
    let ad1 = cpu.a[5].wrapping_add(0xBBF);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1042C: jsr $c11654(pc)
pub fn i_c1042c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10430;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10430);
    cpu.pc = 0xC11654;
    16
}
/// $C10430: rts
pub fn i_c10430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10432;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C10432: move.b #$2, $a81(a5)
pub fn i_c10432(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10438;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xA81);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C10438: move.b $d00108.l, d5
pub fn i_c10438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1043E;
    cpu.ir = 0x1A39;
    let v2 = bus.read8(0xD00108) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C1043E: jsr $c11654(pc)
pub fn i_c1043e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10442;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10442);
    cpu.pc = 0xC11654;
    16
}
/// $C10442: move.b #$1, $bbf(a5)
pub fn i_c10442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10448;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xBBF);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C10448: jsr $c11372(pc)
pub fn i_c10448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1044C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1044C);
    cpu.pc = 0xC11372;
    16
}
/// $C1044C: clr.b $bbf(a5)
pub fn i_c1044c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10450;
    cpu.ir = 0x422D;
    let ad1 = cpu.a[5].wrapping_add(0xBBF);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C10450: rts
pub fn i_c10450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10452;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C10452: clr.b $bc5(a5)
pub fn i_c10452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10456;
    cpu.ir = 0x422D;
    let ad1 = cpu.a[5].wrapping_add(0xBC5);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C10456: jsr $c11bee(pc)
pub fn i_c10456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1045A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1045A);
    cpu.pc = 0xC11BEE;
    16
}
/// $C1045A: move.b #$1, $bbf(a5)
pub fn i_c1045a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10460;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xBBF);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C10460: move.b #$2, $a81(a5)
pub fn i_c10460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10466;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xA81);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C10466: jsr $c112ec(pc)
pub fn i_c10466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1046A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1046A);
    cpu.pc = 0xC112EC;
    16
}
/// $C1046A: clr.b $bbf(a5)
pub fn i_c1046a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1046E;
    cpu.ir = 0x422D;
    let ad1 = cpu.a[5].wrapping_add(0xBBF);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1046E: tst.b $d00043.l
pub fn i_c1046e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10474;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0xD00043) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $C10474: bne.b $c10486
pub fn i_c10474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10476;
    cpu.ir = 0x6610;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC10486; return 10; }
    8
}
/// $C10476: tst.w $d00034.l
pub fn i_c10476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1047C;
    cpu.ir = 0x4A79;
    let v2 = bus.read16(0xD00034) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    16
}
/// $C1047C: beq.b $c10486
pub fn i_c1047c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1047E;
    cpu.ir = 0x6708;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC10486; return 10; }
    8
}
/// $C1047E: move.w $d00044.l, $ada(a5)
pub fn i_c1047e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10486;
    cpu.ir = 0x3B79;
    let v2 = bus.read16(0xD00044) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[5].wrapping_add(0xADA);
    bus.write16(ad3, v2 as u16);
    24
}
/// $C10486: move.b $380000.l, d0
pub fn i_c10486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1048C;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x380000) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C1048C: andi.b #$30, d0
pub fn i_c1048c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10490;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x30) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C10490: bne.w $c10498
pub fn i_c10490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10494;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC10498; return 10; }
    8
}
/// $C10494: jsr $c10504(pc)
pub fn i_c10494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10498;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10498);
    cpu.pc = 0xC10504;
    16
}
/// $C10498: rts
pub fn i_c10498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1049A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C104FC: move.b #$2, $a81(a5)
pub fn i_c104fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10502;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xA81);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C10502: rts
pub fn i_c10502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10504;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C10504: move.w #$2700, sr
pub fn i_c10504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10508;
    cpu.ir = 0x46FC;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let v1: u32 = 0x2700;
    write_sr(cpu, v1 as u16);
    12
}
/// $C10508: jsr $c10fae(pc)
pub fn i_c10508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1050C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1050C);
    cpu.pc = 0xC10FAE;
    16
}
/// $C1050C: move.b #$1, $bdb(a5)
pub fn i_c1050c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10512;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xBDB);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C10512: move.w #$2000, sr
pub fn i_c10512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10516;
    cpu.ir = 0x46FC;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let v1: u32 = 0x2000;
    write_sr(cpu, v1 as u16);
    12
}
/// $C10516: jsr $c1727e(pc)
pub fn i_c10516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1051A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1051A);
    cpu.pc = 0xC1727E;
    16
}
/// $C1051A: move.b d0, $3a0003.l
pub fn i_c1051a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10520;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0003, v1 as u8);
    16
}
/// $C10520: move.b d0, $3a000b.l
pub fn i_c10520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10526;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A000B, v1 as u8);
    16
}
/// $C10526: move.b #$3, $320000.l
pub fn i_c10526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1052E;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x320000, v1 as u8);
    16
}
/// $C1052E: move.b #$ff, $b8d(a5)
pub fn i_c1052e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10534;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xB8D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C10534: clr.b $bdb(a5)
pub fn i_c10534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10538;
    cpu.ir = 0x422D;
    let ad1 = cpu.a[5].wrapping_add(0xBDB);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C10538: addq.b #$1, $ac2(a5)
pub fn i_c10538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1053C;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1053C: movea.l $abe(a5), a0
pub fn i_c1053c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10540;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C10540: lea.l $c105d2(pc), a1
pub fn i_c10540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10544;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0xC105D2;
    16
}
/// $C10544: tst.b $a83(a5)
pub fn i_c10544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10548;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xA83);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C10548: beq.b $c1054e
pub fn i_c10548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1054A;
    cpu.ir = 0x6704;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1054E; return 10; }
    8
}
/// $C1054A: lea.l $c105ec(pc), a1
pub fn i_c1054a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1054E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0xC105EC;
    16
}
/// $C1054E: move.l a1, (a0)+
pub fn i_c1054e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10550;
    cpu.ir = 0x20C9;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C10550: move.l a0, $abe(a5)
pub fn i_c10550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10554;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C10554: subq.b #$1, $ac2(a5)
pub fn i_c10554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10558;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C10558: move.w #$100, $9dc(a5)
pub fn i_c10558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1055E;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9DC);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C1055E: jsr $c1727e(pc)
pub fn i_c1055e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10562;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10562);
    cpu.pc = 0xC1727E;
    16
}
/// $C10562: move.b $380000.l, d0
pub fn i_c10562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10568;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x380000) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C10568: andi.b #$30, d0
pub fn i_c10568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1056C;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x30) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1056C: bne.w $c10596
pub fn i_c1056c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10570;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC10596; return 10; }
    8
}
/// $C10570: move.w $9dc(a5), d0
pub fn i_c10570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10574;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9DC);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C10574: subq.w #$1, d0
pub fn i_c10574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10576;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C10576: move.w d0, $9dc(a5)
pub fn i_c10576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1057A;
    cpu.ir = 0x3B40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9DC);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C1057A: beq.w $c10596
pub fn i_c1057a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1057E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC10596; return 10; }
    8
}
/// $C1057E: moveq #$0, d1
pub fn i_c1057e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10580;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C10580: andi.w #$1f, d0
pub fn i_c10580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10584;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C10584: cmpi.b #$16, d0
pub fn i_c10584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10588;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x16, Size::Byte, false);
    8
}
/// $C10588: bcc.b $c1058e
pub fn i_c10588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1058A;
    cpu.ir = 0x6404;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC1058E; return 10; }
    8
}
/// $C1058A: move.w #$eee, d1
pub fn i_c1058a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1058E;
    cpu.ir = 0x323C;
    let v1: u32 = 0xEEE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1058E: move.w d1, $400002.l
pub fn i_c1058e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10594;
    cpu.ir = 0x33C1;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x400002, v1 as u16);
    16
}
/// $C10594: bra.b $c1055e
pub fn i_c10594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10596;
    cpu.ir = 0x60C8;
    cpu.pc = 0xC1055E;
    10
}
/// $C10596: move.w #$0, $400002.l
pub fn i_c10596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1059E;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x400002, v1 as u16);
    16
}
/// $C1059E: move.w #$2700, sr
pub fn i_c1059e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC105A2;
    cpu.ir = 0x46FC;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let v1: u32 = 0x2700;
    write_sr(cpu, v1 as u16);
    12
}
/// $C105A2: jsr $c10fae(pc)
pub fn i_c105a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC105A6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC105A6);
    cpu.pc = 0xC10FAE;
    16
}
/// $C105A6: move.b #$1, $bdb(a5)
pub fn i_c105a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC105AC;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xBDB);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C105AC: move.w #$2000, sr
pub fn i_c105ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC105B0;
    cpu.ir = 0x46FC;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let v1: u32 = 0x2000;
    write_sr(cpu, v1 as u16);
    12
}
/// $C105B0: jsr $c1727e(pc)
pub fn i_c105b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC105B4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC105B4);
    cpu.pc = 0xC1727E;
    16
}
/// $C105B4: move.b d0, $3a001b.l
pub fn i_c105b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC105BA;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001B, v1 as u8);
    16
}
/// $C105BA: move.b d0, $3a0013.l
pub fn i_c105ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC105C0;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0013, v1 as u8);
    16
}
/// $C105C0: move.b #$3, $320000.l
pub fn i_c105c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC105C8;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x320000, v1 as u8);
    16
}
/// $C105C8: clr.b $b8d(a5)
pub fn i_c105c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC105CC;
    cpu.ir = 0x422D;
    let ad1 = cpu.a[5].wrapping_add(0xB8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C105CC: clr.b $bdb(a5)
pub fn i_c105cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC105D0;
    cpu.ir = 0x422D;
    let ad1 = cpu.a[5].wrapping_add(0xBDB);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C105D0: rts
pub fn i_c105d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC105D2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C10612: jsr $c10632(pc)
pub fn i_c10612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10616;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10616);
    cpu.pc = 0xC10632;
    16
}
/// $C10616: lea.l $10(a0), a0
pub fn i_c10616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1061A;
    cpu.ir = 0x41E8;
    let ad1 = cpu.a[0].wrapping_add(0x10);
    cpu.a[0] = ad1;
    16
}
/// $C1061A: lea.l $10fd84.l, a1
pub fn i_c1061a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10620;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x10FD84;
    20
}
/// $C10620: move.l (a0)+, (a1)+
pub fn i_c10620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10622;
    cpu.ir = 0x22D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C10622: move.w (a0)+, (a1)+
pub fn i_c10622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10624;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $C10624: moveq #$b, d7
pub fn i_c10624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10626;
    cpu.ir = 0x7E0B;
    cpu.d[7] = 0xB;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C10626: move.b (a0)+, d0
pub fn i_c10626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10628;
    cpu.ir = 0x1018;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C10628: lsr.b #$4, d0
pub fn i_c10628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1062A;
    cpu.ir = 0xE808;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    14
}
/// $C1062A: move.b d0, (a1)+
pub fn i_c1062a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1062C;
    cpu.ir = 0x12C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C1062C: dbra d7, $c10626
pub fn i_c1062c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10630;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC10630; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC10630; return 14; }
    cpu.pc = 0xC10626;
    10
}
/// $C10630: rts
pub fn i_c10630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10632;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C10632: moveq #$0, d0
pub fn i_c10632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10634;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C10634: move.b $c00401.l, d0
pub fn i_c10634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1063A;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0xC00401) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C1063A: lsl.w #$2, d0
pub fn i_c1063a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1063C;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $C1063C: lea.l $116.w, a0
pub fn i_c1063c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10640;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0x116;
    16
}
/// $C10640: movea.l (a0, d0.w), a0
pub fn i_c10640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10644;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $C10644: rts
pub fn i_c10644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10646;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C10646: move.w #$4, $3c000c.l
pub fn i_c10646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1064E;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C000C, v1 as u16);
    16
}
/// $C1064E: move.b d0, $300001.l
pub fn i_c1064e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10654;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C10654: tst.b $10fee3.l
pub fn i_c10654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1065A;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FEE3) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $C1065A: bne.b $c10672
pub fn i_c1065a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1065C;
    cpu.ir = 0x6616;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC10672; return 10; }
    8
}
/// $C1065C: movem.l d0-d7/a0-a6, -(a7)
pub fn i_c1065c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10660;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[7]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $C10660: jsr $c17290(pc)
pub fn i_c10660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10664;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10664);
    cpu.pc = 0xC17290;
    16
}
/// $C10664: jsr $c1067a(pc)
pub fn i_c10664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10668;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10668);
    cpu.pc = 0xC1067A;
    16
}
/// $C10668: clr.b $10fe8c.l
pub fn i_c10668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1066E;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10FE8C) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10FE8C, r2 as u8);
    16
}
/// $C1066E: movem.l (a7)+, d0-d7/a0-a6
pub fn i_c1066e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10672;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[7] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $C10672: addq.b #$1, $10fee4.l
pub fn i_c10672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10678;
    cpu.ir = 0x5239;
    let v1 = bus.read8(0x10FEE4) as u32;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    bus.write8(0x10FEE4, r2 as u8);
    16
}
/// $C10678: rte
pub fn i_c10678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1067A;
    cpu.ir = 0x4E73;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let s = cpu.pop16(bus); let r = cpu.pop32(bus); write_sr(cpu, s); cpu.jump_to(r);
    20
}
/// $C1067A: lea.l $10f300.l, a5
pub fn i_c1067a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10680;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x10F300;
    20
}
/// $C10680: addq.l #$1, $b88(a5)
pub fn i_c10680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10684;
    cpu.ir = 0x52AD;
    let ad1 = cpu.a[5].wrapping_add(0xB88);
    let v2 = bus.read32(ad1);
    let r3 = add_with_flags(cpu, v2, 1, Size::Long, true);
    bus.write32(ad1, r3);
    16
}
/// $C10684: jsr $c1167e(pc)
pub fn i_c10684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10688;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10688);
    cpu.pc = 0xC1167E;
    16
}
/// $C10688: jsr $c17080(pc)
pub fn i_c10688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1068C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1068C);
    cpu.pc = 0xC17080;
    16
}
/// $C1068C: tst.w $c106c6.l
pub fn i_c1068c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10692;
    cpu.ir = 0x4A79;
    let v2 = bus.read16(0xC106C6) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    16
}
/// $C10692: bne.w $c10696
pub fn i_c10692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10696;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC10696; return 10; }
    8
}
/// $C10696: tst.b $be1(a5)
pub fn i_c10696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1069A;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xBE1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C1069A: bmi.b $c106b2
pub fn i_c1069a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1069C;
    cpu.ir = 0x6B16;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0xC106B2; return 10; }
    8
}
/// $C1069C: beq.b $c106a4
pub fn i_c1069c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1069E;
    cpu.ir = 0x6706;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC106A4; return 10; }
    8
}
/// $C1069E: subq.b #$1, $be1(a5)
pub fn i_c1069e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC106A2;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xBE1);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C106A2: bra.b $c106b2
pub fn i_c106a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC106A4;
    cpu.ir = 0x600E;
    cpu.pc = 0xC106B2;
    10
}
/// $C106A4: tst.b $a80(a5)
pub fn i_c106a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC106A8;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xA80);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C106A8: bpl.b $c106b2
pub fn i_c106a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC106AA;
    cpu.ir = 0x6A08;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0xC106B2; return 10; }
    8
}
/// $C106AA: jsr $c106c8(pc)
pub fn i_c106aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC106AE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC106AE);
    cpu.pc = 0xC106C8;
    16
}
/// $C106AE: jsr $c1083c(pc)
pub fn i_c106ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC106B2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC106B2);
    cpu.pc = 0xC1083C;
    16
}
/// $C106B2: tst.b $a82(a5)
pub fn i_c106b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC106B6;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xA82);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C106B6: beq.b $c106c4
pub fn i_c106b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC106B8;
    cpu.ir = 0x670C;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC106C4; return 10; }
    8
}
/// $C106B8: cmpi.b #$2, $aaf(a5)
pub fn i_c106b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC106BE;
    cpu.ir = 0xC2D;
    let ad2 = cpu.a[5].wrapping_add(0xAAF);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $C106BE: bne.b $c106c4
pub fn i_c106be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC106C0;
    cpu.ir = 0x6604;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC106C4; return 10; }
    8
}
/// $C106C0: jsr $c11bb0(pc)
pub fn i_c106c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC106C4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC106C4);
    cpu.pc = 0xC11BB0;
    16
}
/// $C106C4: rts
pub fn i_c106c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC106C6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C106C8: tst.b $aaf(a5)
pub fn i_c106c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC106CC;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xAAF);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C106CC: beq.w $c10778
pub fn i_c106cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC106D0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC10778; return 10; }
    8
}
/// $C106D0: moveq #$0, d0
pub fn i_c106d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC106D2;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C106D2: move.b $aad(a5), d1
pub fn i_c106d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC106D6;
    cpu.ir = 0x122D;
    let ad2 = cpu.a[5].wrapping_add(0xAAD);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C106D6: moveq #$3, d7
pub fn i_c106d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC106D8;
    cpu.ir = 0x7E03;
    cpu.d[7] = 0x3;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C106D8: add.b d1, d1
pub fn i_c106d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC106DA;
    cpu.ir = 0xD201;
    let v1 = cpu.d[1] & 0xFF;
    let h1 = cpu.d[1] & 0xFF;
    let r = add_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    4
}
/// $C106DA: add.b d1, d1
pub fn i_c106da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC106DC;
    cpu.ir = 0xD201;
    let v1 = cpu.d[1] & 0xFF;
    let h1 = cpu.d[1] & 0xFF;
    let r = add_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    4
}
/// $C106DC: addx.b d0, d0
pub fn i_c106dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC106DE;
    cpu.ir = 0xD100;
    let x = cpu.sr.get(StatusRegister::X) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let h2 = cpu.d[0] & 0xFF;
    let r = addx_with_flags(cpu, h1, h2, x, Size::Byte);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    4
}
/// $C106DE: dbra d7, $c106d8
pub fn i_c106de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC106E2;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC106E2; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC106E2; return 14; }
    cpu.pc = 0xC106D8;
    10
}
/// $C106E2: tst.b d0
pub fn i_c106e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC106E4;
    cpu.ir = 0x4A00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C106E4: bne.b $c1073a
pub fn i_c106e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC106E6;
    cpu.ir = 0x6654;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1073A; return 10; }
    8
}
/// $C106E6: cmpi.b #$1, $aaf(a5)
pub fn i_c106e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC106EC;
    cpu.ir = 0xC2D;
    let ad2 = cpu.a[5].wrapping_add(0xAAF);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $C106EC: bne.w $c10778
pub fn i_c106ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC106F0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC10778; return 10; }
    8
}
/// $C106F0: tst.w $d00034.l
pub fn i_c106f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC106F6;
    cpu.ir = 0x4A79;
    let v2 = bus.read16(0xD00034) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    16
}
/// $C106F6: beq.w $c10778
pub fn i_c106f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC106FA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC10778; return 10; }
    8
}
/// $C106FA: tst.b $d00043.l
pub fn i_c106fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10700;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0xD00043) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $C10700: bne.w $c10778
pub fn i_c10700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10704;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC10778; return 10; }
    8
}
/// $C10704: move.b $ada(a5), d1
pub fn i_c10704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10708;
    cpu.ir = 0x122D;
    let ad2 = cpu.a[5].wrapping_add(0xADA);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C10708: beq.w $c10728
pub fn i_c10708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1070C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC10728; return 10; }
    8
}
/// $C1070C: subq.b #$1, $adb(a5)
pub fn i_c1070c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10710;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xADB);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C10710: bpl.w $c10778
pub fn i_c10710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10714;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0xC10778; return 10; }
    8
}
/// $C10714: move.b #$3b, $adb(a5)
pub fn i_c10714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1071A;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x3B;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xADB);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C1071A: sub.b d2, d2
pub fn i_c1071a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1071C;
    cpu.ir = 0x9402;
    let v1 = cpu.d[2] & 0xFF;
    let h1 = cpu.d[2] & 0xFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r & 0xFF);
    4
}
/// $C1071C: moveq #$1, d2
pub fn i_c1071c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1071E;
    cpu.ir = 0x7401;
    cpu.d[2] = 0x1;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1071E: sbcd.b d2, d1
pub fn i_c1071e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10720;
    cpu.ir = 0x8302;
    let s = cpu.d[2] as u8; let d = cpu.d[1] as u8;
    let x = cpu.sr.get(StatusRegister::X) as u8;
    let (r, carry, vf) = sbcd_calc(d, s, x);
    if r != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::N, r & 0x80 != 0); cpu.sr.set(StatusRegister::V, vf);
    cpu.sr.set(StatusRegister::C, carry); cpu.sr.set(StatusRegister::X, carry);
    cpu.d[1] = (cpu.d[1] & !0xFF) | r as u32;
    6
}
/// $C10720: move.b d1, $ada(a5)
pub fn i_c10720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10724;
    cpu.ir = 0x1B41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xADA);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C10724: bne.w $c10778
pub fn i_c10724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10728;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC10778; return 10; }
    8
}
/// $C10728: move.b #$1, $be0(a5)
pub fn i_c10728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1072E;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xBE0);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C1072E: moveq #$1, d0
pub fn i_c1072e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10730;
    cpu.ir = 0x7001;
    cpu.d[0] = 0x1;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C10730: tst.b $d00034.l
pub fn i_c10730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10736;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0xD00034) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $C10736: bne.b $c1073a
pub fn i_c10736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10738;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1073A; return 10; }
    8
}
/// $C10738: moveq #$2, d0
pub fn i_c10738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1073A;
    cpu.ir = 0x7002;
    cpu.d[0] = 0x2;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1073A: move.b d0, $ab4(a5)
pub fn i_c1073a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1073E;
    cpu.ir = 0x1B40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAB4);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C1073E: jsr $c1077a(pc)
pub fn i_c1073e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10742;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10742);
    cpu.pc = 0xC1077A;
    16
}
/// $C10742: tst.b $ab4(a5)
pub fn i_c10742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10746;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xAB4);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C10746: beq.w $c10778
pub fn i_c10746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1074A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC10778; return 10; }
    8
}
/// $C1074A: movem.l a5, -(a7)
pub fn i_c1074a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1074E;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $C1074E: jsr $128.w
pub fn i_c1074e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10752;
    cpu.ir = 0x4EB8;
    cpu.push32(bus, 0xC10752);
    cpu.pc = 0x128;
    16
}
/// $C10752: movem.l (a7)+, a5
pub fn i_c10752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10756;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $C10756: move.b $ab4(a5), d0
pub fn i_c10756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1075A;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAB4);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C1075A: andi.b #$f, d0
pub fn i_c1075a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1075E;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1075E: beq.b $c10778
pub fn i_c1075e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10760;
    cpu.ir = 0x6718;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC10778; return 10; }
    8
}
/// $C10760: move.b #$1, $be1(a5)
pub fn i_c10760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10766;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xBE1);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C10766: clr.w $ada(a5)
pub fn i_c10766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1076A;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0xADA);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $C1076A: clr.b $be0(a5)
pub fn i_c1076a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1076E;
    cpu.ir = 0x422D;
    let ad1 = cpu.a[5].wrapping_add(0xBE0);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1076E: move.b #$3, $a81(a5)
pub fn i_c1076e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10774;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xA81);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C10774: jsr $c10836(pc)
pub fn i_c10774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10778;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10778);
    cpu.pc = 0xC10836;
    16
}
/// $C10778: rts
pub fn i_c10778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1077A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1077A: tst.b $a82(a5)
pub fn i_c1077a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1077E;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xA82);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C1077E: beq.w $c107c0
pub fn i_c1077e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10782;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC107C0; return 10; }
    8
}
/// $C10782: andi.w #$3, d0
pub fn i_c10782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10786;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C10786: beq.w $c107bc
pub fn i_c10786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1078A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC107BC; return 10; }
    8
}
/// $C1078A: btst.b #$1, d0
pub fn i_c1078a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1078E;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((1) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C1078E: beq.b $c1079a
pub fn i_c1078e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10790;
    cpu.ir = 0x670A;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1079A; return 10; }
    8
}
/// $C10790: cmpi.b #$1, $aaf(a5)
pub fn i_c10790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10796;
    cpu.ir = 0xC2D;
    let ad2 = cpu.a[5].wrapping_add(0xAAF);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $C10796: bne.b $c1079a
pub fn i_c10796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10798;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1079A; return 10; }
    8
}
/// $C10798: moveq #$0, d0
pub fn i_c10798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1079A;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1079A: move.b $300001.l, d1
pub fn i_c1079a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC107A0;
    cpu.ir = 0x1239;
    let v2 = bus.read8(0x300001) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C107A0: andi.b #$40, d1
pub fn i_c107a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC107A4;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0x40) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C107A4: beq.w $c107b8
pub fn i_c107a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC107A8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC107B8; return 10; }
    8
}
/// $C107A8: move.b $c107c2(pc, d0.w), d1
pub fn i_c107a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC107AC;
    cpu.ir = 0x123B;
    let ad2 = 0xC107C2_u32.wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    14
}
/// $C107AC: move.b $d00034.l, d2
pub fn i_c107ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC107B2;
    cpu.ir = 0x1439;
    let v2 = bus.read8(0xD00034) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C107B2: cmp.b d1, d2
pub fn i_c107b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC107B4;
    cpu.ir = 0xB401;
    let v1 = cpu.d[1] & 0xFF;
    let h1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $C107B4: bcc.b $c107b8
pub fn i_c107b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC107B6;
    cpu.ir = 0x6402;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC107B8; return 10; }
    8
}
/// $C107B6: addq.b #$4, d0
pub fn i_c107b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC107B8;
    cpu.ir = 0x5800;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C107B8: move.b $c107c6(pc, d0.w), d0
pub fn i_c107b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC107BC;
    cpu.ir = 0x103B;
    let ad2 = 0xC107C6_u32.wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $C107BC: move.b d0, $ab4(a5)
pub fn i_c107bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC107C0;
    cpu.ir = 0x1B40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAB4);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C107C0: rts
pub fn i_c107c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC107C2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C107CE: lea.l $10f300.l, a5
pub fn i_c107ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC107D4;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x10F300;
    20
}
/// $C107D4: tst.b $a82(a5)
pub fn i_c107d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC107D8;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xA82);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C107D8: beq.w $c10814
pub fn i_c107d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC107DC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC10814; return 10; }
    8
}
/// $C107DC: clr.w $ab2(a5)
pub fn i_c107dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC107E0;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0xAB2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $C107E0: move.b $300001.l, d0
pub fn i_c107e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC107E6;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x300001) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C107E6: andi.b #$40, d0
pub fn i_c107e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC107EA;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x40) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C107EA: beq.b $c10814
pub fn i_c107ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC107EC;
    cpu.ir = 0x6728;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC10814; return 10; }
    8
}
/// $C107EC: move.b $d00034.l, d0
pub fn i_c107ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC107F2;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0xD00034) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C107F2: lea.l $ab0(a5), a0
pub fn i_c107f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC107F6;
    cpu.ir = 0x41ED;
    let ad1 = cpu.a[5].wrapping_add(0xAB0);
    cpu.a[0] = ad1;
    16
}
/// $C107F6: cmpi.b #$1, $aaf(a5)
pub fn i_c107f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC107FC;
    cpu.ir = 0xC2D;
    let ad2 = cpu.a[5].wrapping_add(0xAAF);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $C107FC: bne.b $c10802
pub fn i_c107fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC107FE;
    cpu.ir = 0x6604;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC10802; return 10; }
    8
}
/// $C107FE: tst.b (a0)
pub fn i_c107fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10800;
    cpu.ir = 0x4A10;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    8
}
/// $C10800: beq.b $c10816
pub fn i_c10800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10802;
    cpu.ir = 0x6714;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC10816; return 10; }
    8
}
/// $C10802: jsr $c10808(pc)
pub fn i_c10802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10806;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10806);
    cpu.pc = 0xC10808;
    16
}
/// $C10806: addq.l #$1, a0
pub fn i_c10806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10808;
    cpu.ir = 0x5288;
    cpu.a[0] = cpu.a[0].wrapping_add(1);
    8
}
/// $C10808: move.b (a0), d1
pub fn i_c10808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1080A;
    cpu.ir = 0x1210;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C1080A: sub.b d2, d2
pub fn i_c1080a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1080C;
    cpu.ir = 0x9402;
    let v1 = cpu.d[2] & 0xFF;
    let h1 = cpu.d[2] & 0xFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r & 0xFF);
    4
}
/// $C1080C: sbcd.b d1, d0
pub fn i_c1080c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1080E;
    cpu.ir = 0x8101;
    let s = cpu.d[1] as u8; let d = cpu.d[0] as u8;
    let x = cpu.sr.get(StatusRegister::X) as u8;
    let (r, carry, vf) = sbcd_calc(d, s, x);
    if r != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::N, r & 0x80 != 0); cpu.sr.set(StatusRegister::V, vf);
    cpu.sr.set(StatusRegister::C, carry); cpu.sr.set(StatusRegister::X, carry);
    cpu.d[0] = (cpu.d[0] & !0xFF) | r as u32;
    6
}
/// $C1080E: bcc.b $c10814
pub fn i_c1080e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10810;
    cpu.ir = 0x6404;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC10814; return 10; }
    8
}
/// $C10810: moveq #$0, d0
pub fn i_c10810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10812;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C10812: move.b d0, (a0)
pub fn i_c10812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10814;
    cpu.ir = 0x1080;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[0], v1 as u8);
    8
}
/// $C10814: rts
pub fn i_c10814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10816;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C10816: addq.l #$1, a0
pub fn i_c10816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10818;
    cpu.ir = 0x5288;
    cpu.a[0] = cpu.a[0].wrapping_add(1);
    8
}
/// $C10818: sub.b d1, d1
pub fn i_c10818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1081A;
    cpu.ir = 0x9201;
    let v1 = cpu.d[1] & 0xFF;
    let h1 = cpu.d[1] & 0xFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    4
}
/// $C1081A: move.b (a0), d1
pub fn i_c1081a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1081C;
    cpu.ir = 0x1210;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C1081C: abcd.b d1, d1
pub fn i_c1081c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1081E;
    cpu.ir = 0xC301;
    let s = cpu.d[1] as u8; let d = cpu.d[1] as u8;
    let x = cpu.sr.get(StatusRegister::X) as u8;
    let (r, carry, vf) = abcd_calc(d, s, x);
    if r != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::N, r & 0x80 != 0); cpu.sr.set(StatusRegister::V, vf);
    cpu.sr.set(StatusRegister::C, carry); cpu.sr.set(StatusRegister::X, carry);
    cpu.d[1] = (cpu.d[1] & !0xFF) | r as u32;
    6
}
/// $C1081E: bra.b $c1080a
pub fn i_c1081e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10820;
    cpu.ir = 0x60EA;
    cpu.pc = 0xC1080A;
    10
}
/// $C10836: jsr $c118dc(pc)
pub fn i_c10836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1083A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1083A);
    cpu.pc = 0xC118DC;
    16
}
/// $C1083A: rts
pub fn i_c1083a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1083C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1083C: tst.b $bbb(a5)
pub fn i_c1083c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10840;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xBBB);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C10840: bne.b $c1084a
pub fn i_c10840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10842;
    cpu.ir = 0x6608;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1084A; return 10; }
    8
}
/// $C10842: move.b #$ff, $380031.l
pub fn i_c10842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1084A;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x380031, v1 as u8);
    16
}
/// $C1084A: tst.b $be1(a5)
pub fn i_c1084a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1084E;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xBE1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C1084E: bne.w $c108c8
pub fn i_c1084e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10852;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC108C8; return 10; }
    8
}
/// $C10852: move.b $aaf(a5), d0
pub fn i_c10852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10856;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAAF);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C10856: cmpi.b #$1, d0
pub fn i_c10856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1085A;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $C1085A: bne.w $c108c8
pub fn i_c1085a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1085E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC108C8; return 10; }
    8
}
/// $C1085E: tst.b $be0(a5)
pub fn i_c1085e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10862;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xBE0);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C10862: bne.w $c108ae
pub fn i_c10862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10866;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC108AE; return 10; }
    8
}
/// $C10866: tst.w $d00034.l
pub fn i_c10866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1086C;
    cpu.ir = 0x4A79;
    let v2 = bus.read16(0xD00034) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    16
}
/// $C1086C: beq.b $c10888
pub fn i_c1086c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1086E;
    cpu.ir = 0x671A;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC10888; return 10; }
    8
}
/// $C1086E: tst.b $bc5(a5)
pub fn i_c1086e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10872;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xBC5);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C10872: bne.b $c10888
pub fn i_c10872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10874;
    cpu.ir = 0x6614;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC10888; return 10; }
    8
}
/// $C10874: tst.b $d00043.l
pub fn i_c10874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1087A;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0xD00043) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $C1087A: bne.b $c10888
pub fn i_c1087a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1087C;
    cpu.ir = 0x660C;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC10888; return 10; }
    8
}
/// $C1087C: moveq #$6, d1
pub fn i_c1087c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1087E;
    cpu.ir = 0x7206;
    cpu.d[1] = 0x6;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1087E: move.b #$1, $10fec5.l
pub fn i_c1087e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10886;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10FEC5, v1 as u8);
    16
}
/// $C10886: bra.b $c108b8
pub fn i_c10886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10888;
    cpu.ir = 0x6030;
    cpu.pc = 0xC108B8;
    10
}
/// $C10888: tst.w $d00060.l
pub fn i_c10888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1088E;
    cpu.ir = 0x4A79;
    let v2 = bus.read16(0xD00060) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    16
}
/// $C1088E: bne.b $c108a2
pub fn i_c1088e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10890;
    cpu.ir = 0x6612;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC108A2; return 10; }
    8
}
/// $C10890: tst.b $d00042.l
pub fn i_c10890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10896;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0xD00042) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $C10896: bne.b $c108a2
pub fn i_c10896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10898;
    cpu.ir = 0x660A;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC108A2; return 10; }
    8
}
/// $C10898: tst.w $d00034.l
pub fn i_c10898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1089E;
    cpu.ir = 0x4A79;
    let v2 = bus.read16(0xD00034) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    16
}
/// $C1089E: beq.w $c108c8
pub fn i_c1089e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC108A2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC108C8; return 10; }
    8
}
/// $C108A2: move.b $bdd(a5), d0
pub fn i_c108a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC108A6;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xBDD);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C108A6: andi.b #$a, d0
pub fn i_c108a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC108AA;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xA) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C108AA: beq.w $c108c8
pub fn i_c108aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC108AE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC108C8; return 10; }
    8
}
/// $C108AE: moveq #$4, d1
pub fn i_c108ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC108B0;
    cpu.ir = 0x7204;
    cpu.d[1] = 0x4;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C108B0: andi.b #$8, d0
pub fn i_c108b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC108B4;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x8) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C108B4: beq.b $c108b8
pub fn i_c108b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC108B6;
    cpu.ir = 0x6702;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC108B8; return 10; }
    8
}
/// $C108B6: moveq #$5, d1
pub fn i_c108b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC108B8;
    cpu.ir = 0x7205;
    cpu.d[1] = 0x5;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C108B8: clr.b $be0(a5)
pub fn i_c108b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC108BC;
    cpu.ir = 0x422D;
    let ad1 = cpu.a[5].wrapping_add(0xBE0);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C108BC: move.b d1, $a81(a5)
pub fn i_c108bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC108C0;
    cpu.ir = 0x1B41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xA81);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C108C0: jsr $12e.w
pub fn i_c108c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC108C4;
    cpu.ir = 0x4EB8;
    cpu.push32(bus, 0xC108C4);
    cpu.pc = 0x12E;
    16
}
/// $C108C4: jmp $c102d2(pc)
pub fn i_c108c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC108C8;
    cpu.ir = 0x4EFA;
    cpu.pc = 0xC102D2;
    8
}
/// $C108C8: rts
pub fn i_c108c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC108CA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C108CA: move.w #$2, $3c000c.l
pub fn i_c108ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC108D2;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C000C, v1 as u16);
    16
}
/// $C108D2: move.w #$4000, $3c0006.l
pub fn i_c108d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC108DA;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0006, v1 as u16);
    16
}
/// $C108DA: rte
pub fn i_c108da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC108DC;
    cpu.ir = 0x4E73;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let s = cpu.pop16(bus); let r = cpu.pop32(bus); write_sr(cpu, s); cpu.jump_to(r);
    20
}
/// $C10A84: move.b #$ff, $380031.l
pub fn i_c10a84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10A8C;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x380031, v1 as u8);
    16
}
/// $C10A8C: bsr.w $c17062
pub fn i_c10a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10A90;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0xC10A90);
    cpu.pc = 0xC17062;
    18
}
/// $C10A90: move.b #$ff, $380041.l
pub fn i_c10a90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10A98;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x380041, v1 as u8);
    16
}
/// $C10A98: move.b #$f7, $380031.l
pub fn i_c10a98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10AA0;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xF7;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x380031, v1 as u8);
    16
}
/// $C10AA0: bsr.w $c17062
pub fn i_c10aa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10AA4;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0xC10AA4);
    cpu.pc = 0xC17062;
    18
}
/// $C10AA4: move.b #$ff, $380031.l
pub fn i_c10aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10AAC;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x380031, v1 as u8);
    16
}
/// $C10AAC: bsr.w $c17062
pub fn i_c10aac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10AB0;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0xC10AB0);
    cpu.pc = 0xC17062;
    18
}
/// $C10AB0: move.b #$ef, $380031.l
pub fn i_c10ab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10AB8;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xEF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x380031, v1 as u8);
    16
}
/// $C10AB8: bsr.w $c17062
pub fn i_c10ab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10ABC;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0xC10ABC);
    cpu.pc = 0xC17062;
    18
}
/// $C10ABC: move.b #$ff, $380031.l
pub fn i_c10abc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10AC4;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x380031, v1 as u8);
    16
}
/// $C10AC4: bsr.w $c17062
pub fn i_c10ac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10AC8;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0xC10AC8);
    cpu.pc = 0xC17062;
    18
}
/// $C10AC8: move.b #$df, $380031.l
pub fn i_c10ac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10AD0;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xDF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x380031, v1 as u8);
    16
}
/// $C10AD0: bsr.w $c17062
pub fn i_c10ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10AD4;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0xC10AD4);
    cpu.pc = 0xC17062;
    18
}
/// $C10AD4: move.b #$ff, $380031.l
pub fn i_c10ad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10ADC;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x380031, v1 as u8);
    16
}
/// $C10ADC: move.b d0, $3a0003.l
pub fn i_c10adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10AE2;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0003, v1 as u8);
    16
}
/// $C10AE2: move.b d0, $3a000b.l
pub fn i_c10ae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10AE8;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A000B, v1 as u8);
    16
}
/// $C10AE8: move.b d0, $380065.l
pub fn i_c10ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10AEE;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x380065, v1 as u8);
    16
}
/// $C10AEE: move.b d0, $380067.l
pub fn i_c10aee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10AF4;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x380067, v1 as u8);
    16
}
/// $C10AF4: moveq #$0, d6
pub fn i_c10af4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10AF6;
    cpu.ir = 0x7C00;
    cpu.d[6] = 0x0;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C10AF6: lea.l $100000.l, a0
pub fn i_c10af6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10AFC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100000;
    20
}
/// $C10AFC: move.w #$7fff, d5
pub fn i_c10afc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10B00;
    cpu.ir = 0x3A3C;
    let v1: u32 = 0x7FFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C10B00: jsr $c12106(pc)
pub fn i_c10b00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10B04;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10B04);
    cpu.pc = 0xC12106;
    16
}
/// $C10B04: move.b d0, $3a001d.l
pub fn i_c10b04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10B0A;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001D, v1 as u8);
    16
}
/// $C10B0A: addq.b #$1, d6
pub fn i_c10b0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10B0C;
    cpu.ir = 0x5206;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C10B0C: lea.l $d00000.l, a0
pub fn i_c10b0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10B12;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0xD00000;
    20
}
/// $C10B12: jsr $c11592(pc)
pub fn i_c10b12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10B16;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10B16);
    cpu.pc = 0xC11592;
    16
}
/// $C10B16: moveq #$2, d6
pub fn i_c10b16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10B18;
    cpu.ir = 0x7C02;
    cpu.d[6] = 0x2;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C10B18: lea.l $400000.l, a0
pub fn i_c10b18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10B1E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x400000;
    20
}
/// $C10B1E: move.w #$fff, d5
pub fn i_c10b1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10B22;
    cpu.ir = 0x3A3C;
    let v1: u32 = 0xFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C10B22: move.b d0, $3a001f.l
pub fn i_c10b22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10B28;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001F, v1 as u8);
    16
}
/// $C10B28: jsr $c12106(pc)
pub fn i_c10b28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10B2C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10B2C);
    cpu.pc = 0xC12106;
    16
}
/// $C10B2C: addq.b #$1, d6
pub fn i_c10b2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10B2E;
    cpu.ir = 0x5206;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C10B2E: lea.l $400000.l, a0
pub fn i_c10b2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10B34;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x400000;
    20
}
/// $C10B34: move.b d0, $3a000f.l
pub fn i_c10b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10B3A;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A000F, v1 as u8);
    16
}
/// $C10B3A: jsr $c12106(pc)
pub fn i_c10b3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10B3E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10B3E);
    cpu.pc = 0xC12106;
    16
}
/// $C10B3E: move.b d0, $3a001f.l
pub fn i_c10b3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10B44;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001F, v1 as u8);
    16
}
/// $C10B44: addq.b #$1, d6
pub fn i_c10b44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10B46;
    cpu.ir = 0x5206;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C10B46: lea.l $3c0002.l, a1
pub fn i_c10b46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10B4C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x3C0002;
    20
}
/// $C10B4C: move.w #$1, $2(a1)
pub fn i_c10b4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10B52;
    cpu.ir = 0x337C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C10B52: moveq #$0, d4
pub fn i_c10b52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10B54;
    cpu.ir = 0x7800;
    cpu.d[4] = 0x0;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C10B54: move.w #$7fff, d5
pub fn i_c10b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10B58;
    cpu.ir = 0x3A3C;
    let v1: u32 = 0x7FFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C10B58: jsr $c1207c(pc)
pub fn i_c10b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10B5C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10B5C);
    cpu.pc = 0xC1207C;
    16
}
/// $C10B5C: move.l #$8000, d4
pub fn i_c10b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10B62;
    cpu.ir = 0x283C;
    let v1: u32 = 0x8000;
    cpu.d[4] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C10B62: move.w #$5ff, d5
pub fn i_c10b62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10B66;
    cpu.ir = 0x3A3C;
    let v1: u32 = 0x5FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C10B66: jsr $c1207c(pc)
pub fn i_c10b66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10B6A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10B6A);
    cpu.pc = 0xC1207C;
    16
}
/// $C10B6A: move.b d0, $3a000d.l
pub fn i_c10b6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10B70;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A000D, v1 as u8);
    16
}
/// $C10B70: move.b $300001.l, d0
pub fn i_c10b70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10B76;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x300001) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C10B76: bne.w $c10bd6
pub fn i_c10b76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10B7A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC10BD6; return 10; }
    8
}
/// $C10B7A: lea.l $10f300.l, a5
pub fn i_c10b7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10B80;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x10F300;
    20
}
/// $C10B80: jsr $c12a0e.l
pub fn i_c10b80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10B86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0xC10B86);
    cpu.pc = 0xC12A0E;
    16
}
/// $C10B86: move.l #$eee0000, $400002.l
pub fn i_c10b86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10B90;
    cpu.ir = 0x23FC;
    let v1: u32 = 0xEEE0000;
    bus.write32(0x400002, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C10B90: lea.l $c00(a5), a0
pub fn i_c10b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10B94;
    cpu.ir = 0x41ED;
    let ad1 = cpu.a[5].wrapping_add(0xC00);
    cpu.a[0] = ad1;
    16
}
/// $C10B94: move.l #$c10a6e, (a0)+
pub fn i_c10b94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10B9A;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC10A6E;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C10B9A: move.l a0, $abe(a5)
pub fn i_c10b9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10B9E;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C10B9E: clr.b $ac2(a5)
pub fn i_c10b9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10BA2;
    cpu.ir = 0x422D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C10BA2: jsr $c17290.l
pub fn i_c10ba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10BA8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0xC10BA8);
    cpu.pc = 0xC17290;
    16
}
/// $C10BA8: moveq #$0, d6
pub fn i_c10ba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10BAA;
    cpu.ir = 0x7C00;
    cpu.d[6] = 0x0;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C10BAA: lea.l $100000.l, a0
pub fn i_c10baa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10BB0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100000;
    20
}
/// $C10BB0: move.w #$7fff, d5
pub fn i_c10bb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10BB4;
    cpu.ir = 0x3A3C;
    let v1: u32 = 0x7FFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C10BB4: jsr $c12106(pc)
pub fn i_c10bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10BB8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10BB8);
    cpu.pc = 0xC12106;
    16
}
/// $C10BB8: move.b d0, $3a001d.l
pub fn i_c10bb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10BBE;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001D, v1 as u8);
    16
}
/// $C10BBE: addq.b #$1, d6
pub fn i_c10bbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10BC0;
    cpu.ir = 0x5206;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C10BC0: lea.l $d00000.l, a0
pub fn i_c10bc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10BC6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0xD00000;
    20
}
/// $C10BC6: jsr $c11592(pc)
pub fn i_c10bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10BCA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10BCA);
    cpu.pc = 0xC11592;
    16
}
/// $C10BCA: move.b $300001.l, d0
pub fn i_c10bca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10BD0;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x300001) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C10BD0: beq.b $c10ba8
pub fn i_c10bd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10BD2;
    cpu.ir = 0x67D6;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC10BA8; return 10; }
    8
}
/// $C10BD2: jsr $c115be(pc)
pub fn i_c10bd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10BD6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10BD6);
    cpu.pc = 0xC115BE;
    16
}
/// $C10BD6: moveq #$8, d0
pub fn i_c10bd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10BD8;
    cpu.ir = 0x7008;
    cpu.d[0] = 0x8;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C10BD8: jsr $c11d3a(pc)
pub fn i_c10bd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10BDC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10BDC);
    cpu.pc = 0xC11D3A;
    16
}
/// $C10BDC: move.b #$40, d0
pub fn i_c10bdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10BE0;
    cpu.ir = 0x103C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C10BE0: jsr $c10d94(pc)
pub fn i_c10be0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10BE4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10BE4);
    cpu.pc = 0xC10D94;
    16
}
/// $C10BE4: clr.b $10fee4.l
pub fn i_c10be4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10BEA;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10FEE4) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10FEE4, r2 as u8);
    16
}
/// $C10BEA: move.b #$1, $10fee3.l
pub fn i_c10bea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10BF2;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10FEE3, v1 as u8);
    16
}
/// $C10BF2: move.w #$2000, sr
pub fn i_c10bf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10BF6;
    cpu.ir = 0x46FC;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let v1: u32 = 0x2000;
    write_sr(cpu, v1 as u16);
    12
}
/// $C10BF6: jsr $c10d94(pc)
pub fn i_c10bf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10BFA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10BFA);
    cpu.pc = 0xC10D94;
    16
}
/// $C10BFA: move.w #$2700, sr
pub fn i_c10bfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10BFE;
    cpu.ir = 0x46FC;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let v1: u32 = 0x2700;
    write_sr(cpu, v1 as u16);
    12
}
/// $C10BFE: clr.b $10fee3.l
pub fn i_c10bfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10C04;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10FEE3) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10FEE3, r2 as u8);
    16
}
/// $C10C04: moveq #$7, d0
pub fn i_c10c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10C06;
    cpu.ir = 0x7007;
    cpu.d[0] = 0x7;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C10C06: jsr $c11d3a(pc)
pub fn i_c10c06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10C0A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10C0A);
    cpu.pc = 0xC11D3A;
    16
}
/// $C10C0A: move.b $10fee4.l, d0
pub fn i_c10c0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10C10;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10FEE4) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C10C10: subi.b #$39, d0
pub fn i_c10c10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10C14;
    cpu.ir = 0x400;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x39, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C10C14: bcs.w $c10d8c
pub fn i_c10c14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10C18;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC10D8C; return 10; }
    8
}
/// $C10C18: cmpi.b #$7, d0
pub fn i_c10c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10C1C;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x7, Size::Byte, false);
    8
}
/// $C10C1C: bcc.w $c10d8c
pub fn i_c10c1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10C20;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC10D8C; return 10; }
    8
}
/// $C10C20: cmpi.l #$c10002, $c0040a.l
pub fn i_c10c20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10C2A;
    cpu.ir = 0xCB9;
    let v2 = bus.read32(0xC0040A);
    sub_with_flags(cpu, v2, 0xC10002, Size::Long, false);
    24
}
/// $C10C2A: bne.w $c10c66
pub fn i_c10c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10C2E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC10C66; return 10; }
    8
}
/// $C10C2E: lea.l $0.w, a0
pub fn i_c10c2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10C32;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0x0;
    16
}
/// $C10C32: moveq #$0, d0
pub fn i_c10c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10C34;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C10C34: moveq #$7f, d7
pub fn i_c10c34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10C36;
    cpu.ir = 0x7E7F;
    cpu.d[7] = 0x7F;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C10C36: move.b d0, $300001.l
pub fn i_c10c36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10C3C;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C10C3C: add.b (a0)+, d0
pub fn i_c10c3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10C3E;
    cpu.ir = 0xD018;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r = add_with_flags(cpu, h1, v3, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    8
}
/// $C10C3E: dbra d7, $c10c36
pub fn i_c10c3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10C42;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC10C42; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC10C42; return 14; }
    cpu.pc = 0xC10C36;
    10
}
/// $C10C42: lea.l $c00082.l, a0
pub fn i_c10c42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10C48;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0xC00082;
    20
}
/// $C10C48: move.l #$1ff7e, d7
pub fn i_c10c48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10C4E;
    cpu.ir = 0x2E3C;
    let v1: u32 = 0x1FF7E;
    cpu.d[7] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C10C4E: move.b d0, $300001.l
pub fn i_c10c4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10C54;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C10C54: add.b (a0)+, d0
pub fn i_c10c54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10C56;
    cpu.ir = 0xD018;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r = add_with_flags(cpu, h1, v3, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    8
}
/// $C10C56: subq.l #$1, d7
pub fn i_c10c56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10C58;
    cpu.ir = 0x5387;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Long, true);
    cpu.d[7] = r2;
    4
}
/// $C10C58: bne.b $c10c4e
pub fn i_c10c58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10C5A;
    cpu.ir = 0x66F4;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC10C4E; return 10; }
    8
}
/// $C10C5A: moveq #$6, d6
pub fn i_c10c5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10C5C;
    cpu.ir = 0x7C06;
    cpu.d[6] = 0x6;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C10C5C: cmp.b $c00080.l, d0
pub fn i_c10c5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10C62;
    cpu.ir = 0xB039;
    let v2 = bus.read8(0xC00080) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v2, Size::Byte, false);
    16
}
/// $C10C62: bne.w $c10d46
pub fn i_c10c62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10C66;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC10D46; return 10; }
    8
}
/// $C10C66: move.b #$5, $10fdc4.l
pub fn i_c10c66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10C6E;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10FDC4, v1 as u8);
    16
}
/// $C10C6E: move.l #$10f300, $10fdc8.l
pub fn i_c10c6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10C78;
    cpu.ir = 0x23FC;
    let v1: u32 = 0x10F300;
    bus.write32(0x10FDC8, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C10C78: clr.w $10fdcc.l
pub fn i_c10c78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10C7E;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x10FDCC) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x10FDCC, r2 as u16);
    16
}
/// $C10C7E: jsr $c1231e(pc)
pub fn i_c10c7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10C82;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10C82);
    cpu.pc = 0xC1231E;
    16
}
/// $C10C82: cmpi.b #$81, $10fdc6.l
pub fn i_c10c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10C8A;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10FDC6) as u32;
    sub_with_flags(cpu, v2, 0x81, Size::Byte, false);
    20
}
/// $C10C8A: bne.w $c10d4a
pub fn i_c10c8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10C8E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC10D4A; return 10; }
    8
}
/// $C10C8E: move.b d0, $3a0005.l
pub fn i_c10c8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10C94;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0005, v1 as u8);
    16
}
/// $C10C94: move.b d0, $3a0017.l
pub fn i_c10c94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10C9A;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0017, v1 as u8);
    16
}
/// $C10C9A: move.b d0, $3a0019.l
pub fn i_c10c9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10CA0;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0019, v1 as u8);
    16
}
/// $C10CA0: lea.l $800000.l, a0
pub fn i_c10ca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10CA6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x800000;
    20
}
/// $C10CA6: movea.l a0, a1
pub fn i_c10ca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10CA8;
    cpu.ir = 0x2248;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $C10CA8: move.w #$7ff, d7
pub fn i_c10ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10CAC;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x7FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C10CAC: move.b #$55, d0
pub fn i_c10cac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10CB0;
    cpu.ir = 0x103C;
    let v1: u32 = 0x55;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C10CB0: move.b d0, $300001.l
pub fn i_c10cb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10CB6;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C10CB6: move.w d0, (a0)+
pub fn i_c10cb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10CB8;
    cpu.ir = 0x30C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C10CB8: dbra d7, $c10cb0
pub fn i_c10cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10CBC;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC10CBC; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC10CBC; return 14; }
    cpu.pc = 0xC10CB0;
    10
}
/// $C10CBC: movea.l a1, a0
pub fn i_c10cbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10CBE;
    cpu.ir = 0x2049;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C10CBE: move.w #$7ff, d7
pub fn i_c10cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10CC2;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x7FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C10CC2: move.b d0, $300001.l
pub fn i_c10cc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10CC8;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C10CC8: move.w (a0)+, d1
pub fn i_c10cc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10CCA;
    cpu.ir = 0x3218;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $C10CCA: cmp.b d1, d0
pub fn i_c10cca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10CCC;
    cpu.ir = 0xB001;
    let v1 = cpu.d[1] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $C10CCC: bne.w $c10d44
pub fn i_c10ccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10CD0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC10D44; return 10; }
    8
}
/// $C10CD0: dbra d7, $c10cc2
pub fn i_c10cd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10CD4;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC10CD4; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC10CD4; return 14; }
    cpu.pc = 0xC10CC2;
    10
}
/// $C10CD4: move.w #$7ff, d7
pub fn i_c10cd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10CD8;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x7FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C10CD8: movea.l a1, a0
pub fn i_c10cd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10CDA;
    cpu.ir = 0x2049;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C10CDA: move.b #$aa, d0
pub fn i_c10cda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10CDE;
    cpu.ir = 0x103C;
    let v1: u32 = 0xAA;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C10CDE: move.b d0, $300001.l
pub fn i_c10cde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10CE4;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C10CE4: move.w d0, (a0)+
pub fn i_c10ce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10CE6;
    cpu.ir = 0x30C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C10CE6: dbra d7, $c10cde
pub fn i_c10ce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10CEA;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC10CEA; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC10CEA; return 14; }
    cpu.pc = 0xC10CDE;
    10
}
/// $C10CEA: movea.l a1, a0
pub fn i_c10cea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10CEC;
    cpu.ir = 0x2049;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C10CEC: move.w #$7ff, d7
pub fn i_c10cec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10CF0;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x7FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C10CF0: move.b d0, $300001.l
pub fn i_c10cf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10CF6;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C10CF6: move.w (a0)+, d1
pub fn i_c10cf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10CF8;
    cpu.ir = 0x3218;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $C10CF8: cmp.b d1, d0
pub fn i_c10cf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10CFA;
    cpu.ir = 0xB001;
    let v1 = cpu.d[1] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $C10CFA: bne.w $c10d44
pub fn i_c10cfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10CFE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC10D44; return 10; }
    8
}
/// $C10CFE: dbra d7, $c10cf0
pub fn i_c10cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10D02;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC10D02; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC10D02; return 14; }
    cpu.pc = 0xC10CF0;
    10
}
/// $C10D02: movea.l a1, a0
pub fn i_c10d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10D04;
    cpu.ir = 0x2049;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C10D04: move.w #$3ff, d7
pub fn i_c10d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10D08;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x3FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C10D08: move.b d0, $300001.l
pub fn i_c10d08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10D0E;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C10D0E: move.w a0, d0
pub fn i_c10d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10D10;
    cpu.ir = 0x3008;
    let v1 = cpu.a[0] as i16 as i32 as u32;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C10D10: move.w a0, d1
pub fn i_c10d10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10D12;
    cpu.ir = 0x3208;
    let v1 = cpu.a[0] as i16 as i32 as u32;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C10D12: lsr.w #$8, d0
pub fn i_c10d12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10D14;
    cpu.ir = 0xE048;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C10D14: move.w d0, (a0)+
pub fn i_c10d14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10D16;
    cpu.ir = 0x30C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C10D16: move.w d1, (a0)+
pub fn i_c10d16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10D18;
    cpu.ir = 0x30C1;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C10D18: dbra d7, $c10d08
pub fn i_c10d18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10D1C;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC10D1C; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC10D1C; return 14; }
    cpu.pc = 0xC10D08;
    10
}
/// $C10D1C: movea.l a1, a0
pub fn i_c10d1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10D1E;
    cpu.ir = 0x2049;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C10D1E: move.w #$3ff, d7
pub fn i_c10d1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10D22;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x3FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C10D22: move.b d0, $300001.l
pub fn i_c10d22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10D28;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C10D28: move.w a0, d2
pub fn i_c10d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10D2A;
    cpu.ir = 0x3408;
    let v1 = cpu.a[0] as i16 as i32 as u32;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C10D2A: move.w d2, d3
pub fn i_c10d2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10D2C;
    cpu.ir = 0x3602;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C10D2C: lsr.w #$8, d2
pub fn i_c10d2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10D2E;
    cpu.ir = 0xE04A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C10D2E: move.w (a0)+, d0
pub fn i_c10d2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10D30;
    cpu.ir = 0x3018;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $C10D30: cmp.b d0, d2
pub fn i_c10d30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10D32;
    cpu.ir = 0xB400;
    let v1 = cpu.d[0] & 0xFF;
    let h1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $C10D32: bne.w $c10d44
pub fn i_c10d32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10D36;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC10D44; return 10; }
    8
}
/// $C10D36: move.w (a0)+, d0
pub fn i_c10d36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10D38;
    cpu.ir = 0x3018;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $C10D38: cmp.b d0, d3
pub fn i_c10d38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10D3A;
    cpu.ir = 0xB600;
    let v1 = cpu.d[0] & 0xFF;
    let h1 = cpu.d[3] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $C10D3A: bne.w $c10d44
pub fn i_c10d3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10D3E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC10D44; return 10; }
    8
}
/// $C10D3E: dbra d7, $c10d22
pub fn i_c10d3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10D42;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC10D42; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC10D42; return 14; }
    cpu.pc = 0xC10D22;
    10
}
/// $C10D42: bra.b $c10d4a
pub fn i_c10d42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10D44;
    cpu.ir = 0x6006;
    cpu.pc = 0xC10D4A;
    10
}
/// $C10D44: moveq #$7, d6
pub fn i_c10d44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10D46;
    cpu.ir = 0x7C07;
    cpu.d[6] = 0x7;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C10D46: jmp $c15aac(pc)
pub fn i_c10d46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10D4A;
    cpu.ir = 0x4EFA;
    cpu.pc = 0xC15AAC;
    8
}
/// $C10D4A: jsr $c10db0(pc)
pub fn i_c10d4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10D4E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10D4E);
    cpu.pc = 0xC10DB0;
    16
}
/// $C10D4E: lea.l $d00124.l, a0
pub fn i_c10d4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10D54;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0xD00124;
    20
}
/// $C10D54: moveq #$0, d0
pub fn i_c10d54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10D56;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C10D56: tst.w (a0)
pub fn i_c10d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10D58;
    cpu.ir = 0x4A50;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    8
}
/// $C10D58: bne.b $c10d88
pub fn i_c10d58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10D5A;
    cpu.ir = 0x662E;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC10D88; return 10; }
    8
}
/// $C10D5A: addq.l #$4, a0
pub fn i_c10d5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10D5C;
    cpu.ir = 0x5888;
    cpu.a[0] = cpu.a[0].wrapping_add(4);
    8
}
/// $C10D5C: addq.b #$1, d0
pub fn i_c10d5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10D5E;
    cpu.ir = 0x5200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C10D5E: cmp.b $d00047.l, d0
pub fn i_c10d5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10D64;
    cpu.ir = 0xB039;
    let v2 = bus.read8(0xD00047) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v2, Size::Byte, false);
    16
}
/// $C10D64: bne.b $c10d56
pub fn i_c10d64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10D66;
    cpu.ir = 0x66F0;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC10D56; return 10; }
    8
}
/// $C10D66: move.b $320001.l, d0
pub fn i_c10d66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10D6C;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x320001) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C10D6C: btst.b #$2, d0
pub fn i_c10d6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10D70;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((2) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C10D70: bne.b $c10d84
pub fn i_c10d70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10D72;
    cpu.ir = 0x6612;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC10D84; return 10; }
    8
}
/// $C10D72: jsr $c1a536.l
pub fn i_c10d72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10D78;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0xC10D78);
    cpu.pc = 0xC1A536;
    16
}
/// $C10D78: tst.b d5
pub fn i_c10d78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10D7A;
    cpu.ir = 0x4A05;
    let v1 = cpu.d[5] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C10D7A: beq.w $c10d84
pub fn i_c10d7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10D7E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC10D84; return 10; }
    8
}
/// $C10D7E: jmp $c1a328.l
pub fn i_c10d7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10D84;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC1A328;
    8
}
/// $C10D84: jmp $c16034(pc)
pub fn i_c10d84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10D88;
    cpu.ir = 0x4EFA;
    cpu.pc = 0xC16034;
    8
}
/// $C10D88: jmp $c10028(pc)
pub fn i_c10d88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10D8C;
    cpu.ir = 0x4EFA;
    cpu.pc = 0xC10028;
    8
}
/// $C10D8C: moveq #$5, d6
pub fn i_c10d8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10D8E;
    cpu.ir = 0x7C05;
    cpu.d[6] = 0x5;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C10D8E: jmp $c15aac.l
pub fn i_c10d8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10D94;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC15AAC;
    8
}
/// $C10D94: move.b d0, $300001.l
pub fn i_c10d94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10D9A;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C10D9A: move.b d0, d2
pub fn i_c10d9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10D9C;
    cpu.ir = 0x1400;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C10D9C: move.b $320001.l, d0
pub fn i_c10d9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10DA2;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x320001) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C10DA2: move.b d0, d1
pub fn i_c10da2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10DA4;
    cpu.ir = 0x1200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C10DA4: eor.b d1, d2
pub fn i_c10da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10DA6;
    cpu.ir = 0xB302;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 ^ cpu.d[1]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C10DA6: and.b d1, d2
pub fn i_c10da6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10DA8;
    cpu.ir = 0xC401;
    let v1 = cpu.d[1] & 0xFF;
    let r = (v1 & cpu.d[2]) & 0xFF;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C10DA8: andi.b #$40, d2
pub fn i_c10da8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10DAC;
    cpu.ir = 0x202;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 & 0x40) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C10DAC: beq.b $c10d94
pub fn i_c10dac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10DAE;
    cpu.ir = 0x67E6;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC10D94; return 10; }
    8
}
/// $C10DAE: rts
pub fn i_c10dae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10DB0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C10DB0: lea.l $d08000.l, a4
pub fn i_c10db0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10DB6;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0xD08000;
    20
}
/// $C10DB6: lea.l $10f300.l, a5
pub fn i_c10db6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10DBC;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x10F300;
    20
}
/// $C10DBC: move.b #$ff, $be1(a5)
pub fn i_c10dbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10DC2;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xBE1);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C10DC2: move.b d0, $3a001d.l
pub fn i_c10dc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10DC8;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001D, v1 as u8);
    16
}
/// $C10DC8: move.w #$ffff, -$7fa2(a4)
pub fn i_c10dc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10DCE;
    cpu.ir = 0x397C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF805E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C10DCE: moveq #$0, d0
pub fn i_c10dce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10DD0;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C10DD0: move.w d0, -$7fa0(a4)
pub fn i_c10dd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10DD4;
    cpu.ir = 0x3940;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8060);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C10DD4: move.l d0, -$7fb8(a4)
pub fn i_c10dd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10DD8;
    cpu.ir = 0x2940;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8048);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C10DD8: move.l d0, -$7fe0(a4)
pub fn i_c10dd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10DDC;
    cpu.ir = 0x2940;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8020);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C10DDC: move.b #$3b, -$7fb4(a4)
pub fn i_c10ddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10DE2;
    cpu.ir = 0x197C;
    let v1: u32 = 0x3B;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF804C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C10DE2: lea.l -$7fdc(a4), a0
pub fn i_c10de2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10DE6;
    cpu.ir = 0x41EC;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF8024);
    cpu.a[0] = ad1;
    16
}
/// $C10DE6: move.w #$15, d7
pub fn i_c10de6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10DEA;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x15;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C10DEA: move.b d0, (a0)+
pub fn i_c10dea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10DEC;
    cpu.ir = 0x10C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C10DEC: dbra d7, $c10dea
pub fn i_c10dec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10DF0;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC10DF0; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC10DF0; return 14; }
    cpu.pc = 0xC10DEA;
    10
}
/// $C10DF0: jsr $c11c42(pc)
pub fn i_c10df0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10DF4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10DF4);
    cpu.pc = 0xC11C42;
    16
}
/// $C10DF4: move.l $ad6(a5), -$7fa6(a4)
pub fn i_c10df4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10DFA;
    cpu.ir = 0x296D;
    let ad2 = cpu.a[5].wrapping_add(0xAD6);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[4].wrapping_add(0xFFFF805A);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $C10DFA: move.b #$1, $a4e(a5)
pub fn i_c10dfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10E00;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xA4E);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C10E00: lea.l $ad2(a5), a0
pub fn i_c10e00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10E04;
    cpu.ir = 0x41ED;
    let ad1 = cpu.a[5].wrapping_add(0xAD2);
    cpu.a[0] = ad1;
    16
}
/// $C10E04: lea.l -$7efc(a4), a1
pub fn i_c10e04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10E08;
    cpu.ir = 0x43EC;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF8104);
    cpu.a[1] = ad1;
    16
}
/// $C10E08: moveq #$2, d7
pub fn i_c10e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10E0A;
    cpu.ir = 0x7E02;
    cpu.d[7] = 0x2;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C10E0A: cmpm.b (a0)+, (a1)+
pub fn i_c10e0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10E0C;
    cpu.ir = 0xB308;
    let ay = cpu.a[0]; cpu.a[0] = ay.wrapping_add(1);
    let v1 = bus.read8(ay) as u32;
    let ax = cpu.a[1];
    let v2 = bus.read8(ax) as u32;
    cpu.a[1] = cpu.a[1].wrapping_add(1);
    sub_with_flags(cpu, v2, v1, Size::Byte, false);
    12
}
/// $C10E0C: bne.b $c10e1a
pub fn i_c10e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10E0E;
    cpu.ir = 0x660C;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC10E1A; return 10; }
    8
}
/// $C10E0E: dbra d7, $c10e0a
pub fn i_c10e0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10E12;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC10E12; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC10E12; return 14; }
    cpu.pc = 0xC10E0A;
    10
}
/// $C10E12: clr.b $a4e(a5)
pub fn i_c10e12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10E16;
    cpu.ir = 0x422D;
    let ad1 = cpu.a[5].wrapping_add(0xA4E);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C10E16: bra.w $c10e8c
pub fn i_c10e16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10E1A;
    cpu.ir = 0x6000;
    cpu.pc = 0xC10E8C;
    10
}
/// $C10E1A: move.b -$7efb(a4), (a5)
pub fn i_c10e1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10E1E;
    cpu.ir = 0x1AAC;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8105);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[5], v3 as u8);
    16
}
/// $C10E1E: move.l $ad2(a5), -$7efc(a4)
pub fn i_c10e1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10E24;
    cpu.ir = 0x296D;
    let ad2 = cpu.a[5].wrapping_add(0xAD2);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[4].wrapping_add(0xFFFF8104);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $C10E24: move.b -$7ede(a4), d0
pub fn i_c10e24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10E28;
    cpu.ir = 0x102C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8122);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C10E28: addq.b #$1, d0
pub fn i_c10e28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10E2A;
    cpu.ir = 0x5200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C10E2A: cmpi.b #$24, d0
pub fn i_c10e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10E2E;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x24, Size::Byte, false);
    8
}
/// $C10E2E: bcs.b $c10e32
pub fn i_c10e2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10E30;
    cpu.ir = 0x6502;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC10E32; return 10; }
    8
}
/// $C10E30: moveq #$0, d0
pub fn i_c10e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10E32;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C10E32: move.b d0, -$7ede(a4)
pub fn i_c10e32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10E36;
    cpu.ir = 0x1940;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8122);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C10E36: lea.l $1de0(a4), a0
pub fn i_c10e36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10E3A;
    cpu.ir = 0x41EC;
    let ad1 = cpu.a[4].wrapping_add(0x1DE0);
    cpu.a[0] = ad1;
    16
}
/// $C10E3A: jsr $c1155c(pc)
pub fn i_c10e3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10E3E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10E3E);
    cpu.pc = 0xC1155C;
    16
}
/// $C10E3E: lea.l $1ba0(a4), a0
pub fn i_c10e3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10E42;
    cpu.ir = 0x41EC;
    let ad1 = cpu.a[4].wrapping_add(0x1BA0);
    cpu.a[0] = ad1;
    16
}
/// $C10E42: move.b -$7ede(a4), d0
pub fn i_c10e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10E46;
    cpu.ir = 0x102C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8122);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C10E46: jsr $c1155c(pc)
pub fn i_c10e46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10E4A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10E4A);
    cpu.pc = 0xC1155C;
    16
}
/// $C10E4A: lea.l (a0, d0.w), a0
pub fn i_c10e4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10E4E;
    cpu.ir = 0x41F0;
    let ad1 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    cpu.a[0] = ad1;
    18
}
/// $C10E4E: move.l a0, -$7eee(a4)
pub fn i_c10e4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10E52;
    cpu.ir = 0x2948;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8112);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C10E52: move.b -$7efb(a4), d0
pub fn i_c10e52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10E56;
    cpu.ir = 0x102C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8105);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C10E56: cmp.b (a5), d0
pub fn i_c10e56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10E58;
    cpu.ir = 0xB015;
    let v2 = bus.read8(cpu.a[5]) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v2, Size::Byte, false);
    8
}
/// $C10E58: beq.b $c10e8c
pub fn i_c10e58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10E5A;
    cpu.ir = 0x6732;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC10E8C; return 10; }
    8
}
/// $C10E5A: jsr $c1217c(pc)
pub fn i_c10e5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10E5E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10E5E);
    cpu.pc = 0xC1217C;
    16
}
/// $C10E5E: subq.b #$1, d0
pub fn i_c10e5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10E60;
    cpu.ir = 0x5300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C10E60: lsl.w #$4, d0
pub fn i_c10e60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10E62;
    cpu.ir = 0xE948;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $C10E62: lea.l $20e0(a4), a0
pub fn i_c10e62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10E66;
    cpu.ir = 0x41EC;
    let ad1 = cpu.a[4].wrapping_add(0x20E0);
    cpu.a[0] = ad1;
    16
}
/// $C10E66: jsr $c1156e(pc)
pub fn i_c10e66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10E6A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10E6A);
    cpu.pc = 0xC1156E;
    16
}
/// $C10E6A: lea.l $2020(a4), a0
pub fn i_c10e6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10E6E;
    cpu.ir = 0x41EC;
    let ad1 = cpu.a[4].wrapping_add(0x2020);
    cpu.a[0] = ad1;
    16
}
/// $C10E6E: jsr $c1156e(pc)
pub fn i_c10e6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10E72;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10E72);
    cpu.pc = 0xC1156E;
    16
}
/// $C10E72: lea.l (a0, d0.w), a0
pub fn i_c10e72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10E76;
    cpu.ir = 0x41F0;
    let ad1 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    cpu.a[0] = ad1;
    18
}
/// $C10E76: move.l a0, -$7eea(a4)
pub fn i_c10e76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10E7A;
    cpu.ir = 0x2948;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8116);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C10E7A: lea.l $1520(a4), a0
pub fn i_c10e7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10E7E;
    cpu.ir = 0x41EC;
    let ad1 = cpu.a[4].wrapping_add(0x1520);
    cpu.a[0] = ad1;
    16
}
/// $C10E7E: moveq #$7, d7
pub fn i_c10e7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10E80;
    cpu.ir = 0x7E07;
    cpu.d[7] = 0x7;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C10E80: jsr $c1156e(pc)
pub fn i_c10e80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10E84;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10E84);
    cpu.pc = 0xC1156E;
    16
}
/// $C10E84: addi.w #$c0, d0
pub fn i_c10e84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10E88;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xC0, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C10E88: dbra d7, $c10e80
pub fn i_c10e88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10E8C;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC10E8C; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC10E8C; return 14; }
    cpu.pc = 0xC10E80;
    10
}
/// $C10E8C: lea.l $a00(a5), a3
pub fn i_c10e8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10E90;
    cpu.ir = 0x47ED;
    let ad1 = cpu.a[5].wrapping_add(0xA00);
    cpu.a[3] = ad1;
    16
}
/// $C10E90: movea.l a3, a0
pub fn i_c10e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10E92;
    cpu.ir = 0x204B;
    let v1 = cpu.a[3] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C10E92: move.l #$ffff, d0
pub fn i_c10e92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10E98;
    cpu.ir = 0x203C;
    let v1: u32 = 0xFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C10E98: moveq #$7, d7
pub fn i_c10e98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10E9A;
    cpu.ir = 0x7E07;
    cpu.d[7] = 0x7;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C10E9A: clr.l $20(a0)
pub fn i_c10e9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10E9E;
    cpu.ir = 0x42A8;
    let ad1 = cpu.a[0].wrapping_add(0x20);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $C10E9E: move.l d0, (a0)+
pub fn i_c10e9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10EA0;
    cpu.ir = 0x20C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C10EA0: dbra d7, $c10e9a
pub fn i_c10ea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10EA4;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC10EA4; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC10EA4; return 14; }
    cpu.pc = 0xC10E9A;
    10
}
/// $C10EA4: moveq #$2, d4
pub fn i_c10ea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10EA6;
    cpu.ir = 0x7802;
    cpu.d[4] = 0x2;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C10EA6: move.b $300081.l, d0
pub fn i_c10ea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10EAC;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x300081) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C10EAC: btst.b #$6, d0
pub fn i_c10eac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10EB0;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((6) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C10EB0: beq.b $c10ec2
pub fn i_c10eb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10EB2;
    cpu.ir = 0x6710;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC10EC2; return 10; }
    8
}
/// $C10EB2: moveq #$6, d4
pub fn i_c10eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10EB4;
    cpu.ir = 0x7806;
    cpu.d[4] = 0x6;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C10EB4: move.b $320001.l, d0
pub fn i_c10eb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10EBA;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x320001) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C10EBA: andi.b #$20, d0
pub fn i_c10eba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10EBE;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x20) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C10EBE: bne.b $c10ec2
pub fn i_c10ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10EC0;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC10EC2; return 10; }
    8
}
/// $C10EC0: moveq #$4, d4
pub fn i_c10ec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10EC2;
    cpu.ir = 0x7804;
    cpu.d[4] = 0x4;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C10EC2: move.b d4, -$7fb9(a4)
pub fn i_c10ec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10EC6;
    cpu.ir = 0x1944;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8047);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C10EC6: lsl.w #$2, d4
pub fn i_c10ec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10EC8;
    cpu.ir = 0xE54C;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $C10EC8: move.b #$0, $380021.l
pub fn i_c10ec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10ED0;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x380021, v1 as u8);
    16
}
/// $C10ED0: move.b d0, $3a001b.l
pub fn i_c10ed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10ED6;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001B, v1 as u8);
    16
}
/// $C10ED6: jsr $c10f56(pc)
pub fn i_c10ed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10EDA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10EDA);
    cpu.pc = 0xC10F56;
    16
}
/// $C10EDA: move.b d0, $3a000b.l
pub fn i_c10eda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10EE0;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A000B, v1 as u8);
    16
}
/// $C10EE0: lea.l -$7edc(a4), a2
pub fn i_c10ee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10EE4;
    cpu.ir = 0x45EC;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF8124);
    cpu.a[2] = ad1;
    16
}
/// $C10EE4: moveq #$0, d4
pub fn i_c10ee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10EE6;
    cpu.ir = 0x7800;
    cpu.d[4] = 0x0;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C10EE6: move.b -$7fb9(a4), d4
pub fn i_c10ee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10EEA;
    cpu.ir = 0x182C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8047);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C10EEA: add.w d4, d4
pub fn i_c10eea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10EEC;
    cpu.ir = 0xD844;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C10EEC: add.w d4, d4
pub fn i_c10eec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10EEE;
    cpu.ir = 0xD844;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C10EEE: jsr $c1100e(pc)
pub fn i_c10eee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10EF2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10EF2);
    cpu.pc = 0xC1100E;
    16
}
/// $C10EF2: jsr $c1103e(pc)
pub fn i_c10ef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10EF6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10EF6);
    cpu.pc = 0xC1103E;
    16
}
/// $C10EF6: jsr $c1107e(pc)
pub fn i_c10ef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10EFA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10EFA);
    cpu.pc = 0xC1107E;
    16
}
/// $C10EFA: jsr $c110b6(pc)
pub fn i_c10efa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10EFE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10EFE);
    cpu.pc = 0xC110B6;
    16
}
/// $C10EFE: jsr $c110e2(pc)
pub fn i_c10efe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10F02;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10F02);
    cpu.pc = 0xC110E2;
    16
}
/// $C10F02: jsr $c1110c(pc)
pub fn i_c10f02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10F06;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10F06);
    cpu.pc = 0xC1110C;
    16
}
/// $C10F06: tst.b $a4e(a5)
pub fn i_c10f06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10F0A;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xA4E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C10F0A: beq.b $c10f10
pub fn i_c10f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10F0C;
    cpu.ir = 0x6704;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC10F10; return 10; }
    8
}
/// $C10F0C: jsr $c1112a(pc)
pub fn i_c10f0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10F10;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10F10);
    cpu.pc = 0xC1112A;
    16
}
/// $C10F10: move.b -$7fa5(a4), d0
pub fn i_c10f10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10F14;
    cpu.ir = 0x102C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF805B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C10F14: jsr $c1217c(pc)
pub fn i_c10f14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10F18;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10F18);
    cpu.pc = 0xC1217C;
    16
}
/// $C10F18: move.b d0, -$7fa5(a4)
pub fn i_c10f18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10F1C;
    cpu.ir = 0x1940;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF805B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C10F1C: move.b -$7fa4(a4), d0
pub fn i_c10f1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10F20;
    cpu.ir = 0x102C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF805C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C10F20: jsr $c1217c(pc)
pub fn i_c10f20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10F24;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10F24);
    cpu.pc = 0xC1217C;
    16
}
/// $C10F24: move.b d0, -$7fa4(a4)
pub fn i_c10f24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10F28;
    cpu.ir = 0x1940;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF805C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C10F28: move.b -$7fa6(a4), d0
pub fn i_c10f28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10F2C;
    cpu.ir = 0x102C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF805A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C10F2C: jsr $c1217c(pc)
pub fn i_c10f2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10F30;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10F30);
    cpu.pc = 0xC1217C;
    16
}
/// $C10F30: lsl.b #$2, d0
pub fn i_c10f30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10F32;
    cpu.ir = 0xE508;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    10
}
/// $C10F32: move.b d0, -$7fa6(a4)
pub fn i_c10f32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10F36;
    cpu.ir = 0x1940;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF805A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C10F36: move.b #$3c, -$7fa3(a4)
pub fn i_c10f36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10F3C;
    cpu.ir = 0x197C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF805D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C10F3C: move.b #$0, -$7ef7(a4)
pub fn i_c10f3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10F42;
    cpu.ir = 0x197C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8109);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C10F42: move.b d0, $3a000d.l
pub fn i_c10f42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10F48;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A000D, v1 as u8);
    16
}
/// $C10F48: move.b d0, $3a000b.l
pub fn i_c10f48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10F4E;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A000B, v1 as u8);
    16
}
/// $C10F4E: move.b d0, $3a0003.l
pub fn i_c10f4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10F54;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0003, v1 as u8);
    16
}
/// $C10F54: rts
pub fn i_c10f54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10F56;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C10F56: move.b #$ff, -$7fa8(a4)
pub fn i_c10f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10F5C;
    cpu.ir = 0x197C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8058);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C10F5C: moveq #$0, d6
pub fn i_c10f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10F5E;
    cpu.ir = 0x7C00;
    cpu.d[6] = 0x0;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C10F5E: move.w d6, d5
pub fn i_c10f5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10F60;
    cpu.ir = 0x3A06;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C10F60: lsr.w #$2, d5
pub fn i_c10f60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10F62;
    cpu.ir = 0xE44D;
    let h1 = cpu.d[5] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $C10F62: move.b d5, -$7ef8(a4)
pub fn i_c10f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10F66;
    cpu.ir = 0x1945;
    let v1 = cpu.d[5] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8108);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C10F66: move.b d5, $380021.l
pub fn i_c10f66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10F6C;
    cpu.ir = 0x13C5;
    let v1 = cpu.d[5] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x380021, v1 as u8);
    16
}
/// $C10F6C: jsr $c11fcc(pc)
pub fn i_c10f6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10F70;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10F70);
    cpu.pc = 0xC11FCC;
    16
}
/// $C10F70: bne.b $c10f88
pub fn i_c10f70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10F72;
    cpu.ir = 0x6616;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC10F88; return 10; }
    8
}
/// $C10F72: move.w $108.w, (a3, d6.w)
pub fn i_c10f72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10F78;
    cpu.ir = 0x37B8;
    let v2 = bus.read16(0x108) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[3].wrapping_add(0x0).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    bus.write16(ad3, v2 as u16);
    22
}
/// $C10F78: tst.b $114.w
pub fn i_c10f78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10F7C;
    cpu.ir = 0x4A38;
    let v2 = bus.read8(0x114) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C10F7C: bne.b $c10f88
pub fn i_c10f7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10F7E;
    cpu.ir = 0x660A;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC10F88; return 10; }
    8
}
/// $C10F7E: tst.b -$7fa8(a4)
pub fn i_c10f7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10F82;
    cpu.ir = 0x4A2C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8058);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C10F82: bpl.b $c10f88
pub fn i_c10f82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10F84;
    cpu.ir = 0x6A04;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0xC10F88; return 10; }
    8
}
/// $C10F84: move.b d5, -$7fa8(a4)
pub fn i_c10f84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10F88;
    cpu.ir = 0x1945;
    let v1 = cpu.d[5] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8058);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C10F88: addq.w #$4, d6
pub fn i_c10f88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10F8A;
    cpu.ir = 0x5846;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C10F8A: cmp.b d6, d4
pub fn i_c10f8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10F8C;
    cpu.ir = 0xB806;
    let v1 = cpu.d[6] & 0xFF;
    let h1 = cpu.d[4] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $C10F8C: bne.b $c10f5e
pub fn i_c10f8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10F8E;
    cpu.ir = 0x66D0;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC10F5E; return 10; }
    8
}
/// $C10F8E: rts
pub fn i_c10f8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10F90;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C10F90: jsr $c10fae(pc)
pub fn i_c10f90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10F94;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC10F94);
    cpu.pc = 0xC10FAE;
    16
}
/// $C10F94: move.b d5, $d00108.l
pub fn i_c10f94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10F9A;
    cpu.ir = 0x13C5;
    let v1 = cpu.d[5] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0xD00108, v1 as u8);
    16
}
/// $C10F9A: move.b d5, $380021.l
pub fn i_c10f9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10FA0;
    cpu.ir = 0x13C5;
    let v1 = cpu.d[5] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x380021, v1 as u8);
    16
}
/// $C10FA0: move.b d0, $3a001b.l
pub fn i_c10fa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10FA6;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001B, v1 as u8);
    16
}
/// $C10FA6: move.b d0, $3a0013.l
pub fn i_c10fa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10FAC;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0013, v1 as u8);
    16
}
/// $C10FAC: rts
pub fn i_c10fac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10FAE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C10FAE: move.b $320000.l, d0
pub fn i_c10fae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10FB4;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x320000) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C10FB4: subq.b #$1, d0
pub fn i_c10fb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10FB6;
    cpu.ir = 0x5300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C10FB6: beq.w $c1100c
pub fn i_c10fb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10FBA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1100C; return 10; }
    8
}
/// $C10FBA: move.w sr, -(a7)
pub fn i_c10fba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10FBC;
    cpu.ir = 0x40E7;
    let ad1 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad1;
    let v2 = bus.read16(ad1) as u32;
    bus.write16(ad1, (cpu.sr.0 as u32) as u16);
    12
}
/// $C10FBC: move.l d1, -(a7)
pub fn i_c10fbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10FBE;
    cpu.ir = 0x2F01;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C10FBE: move.b #$1, $10fedb.l
pub fn i_c10fbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10FC6;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10FEDB, v1 as u8);
    16
}
/// $C10FC6: move.w #$2000, sr
pub fn i_c10fc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10FCA;
    cpu.ir = 0x46FC;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let v1: u32 = 0x2000;
    write_sr(cpu, v1 as u16);
    12
}
/// $C10FCA: move.w #$7, d1
pub fn i_c10fca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10FCE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C10FCE: swap d1
pub fn i_c10fce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10FD0;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C10FD0: move.w $10fe8a.l, d1
pub fn i_c10fd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10FD6;
    cpu.ir = 0x3239;
    let v2 = bus.read16(0x10FE8A) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $C10FD6: addq.w #$4, d1
pub fn i_c10fd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10FD8;
    cpu.ir = 0x5841;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C10FD8: move.b #$1, $320000.l
pub fn i_c10fd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10FE0;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x320000, v1 as u8);
    16
}
/// $C10FE0: move.b d0, $300001.l
pub fn i_c10fe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10FE6;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C10FE6: move.b $320000.l, d0
pub fn i_c10fe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10FEC;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x320000) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C10FEC: subq.b #$1, d0
pub fn i_c10fec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10FEE;
    cpu.ir = 0x5300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C10FEE: beq.b $c10ffe
pub fn i_c10fee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10FF0;
    cpu.ir = 0x670E;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC10FFE; return 10; }
    8
}
/// $C10FF0: cmp.w $10fe8a.l, d1
pub fn i_c10ff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10FF6;
    cpu.ir = 0xB279;
    let v2 = bus.read16(0x10FE8A) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    16
}
/// $C10FF6: bne.b $c10fe0
pub fn i_c10ff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10FF8;
    cpu.ir = 0x66E8;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC10FE0; return 10; }
    8
}
/// $C10FF8: swap d1
pub fn i_c10ff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10FFA;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C10FFA: dbra d1, $c10fce
pub fn i_c10ffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10FFE;
    cpu.ir = 0x51C9;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC10FFE; return 12; }
    let n = (cpu.d[1] as u16).wrapping_sub(1);
    cpu.d[1] = (cpu.d[1] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC10FFE; return 14; }
    cpu.pc = 0xC10FCE;
    10
}
/// $C10FFE: move.l (a7)+, d1
pub fn i_c10ffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11000;
    cpu.ir = 0x221F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C11000: move.w (a7)+, sr
pub fn i_c11000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11002;
    cpu.ir = 0x46DF;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    write_sr(cpu, v3 as u16);
    16
}
/// $C11002: bsr.w $c1705e
pub fn i_c11002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11006;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0xC11006);
    cpu.pc = 0xC1705E;
    18
}
/// $C11006: clr.b $10fedb.l
pub fn i_c11006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1100C;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10FEDB) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10FEDB, r2 as u8);
    16
}
/// $C1100C: rts
pub fn i_c1100c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1100E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1100E: tst.w -$7fa0(a4)
pub fn i_c1100e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11012;
    cpu.ir = 0x4A6C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8060);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $C11012: bne.w $c1103c
pub fn i_c11012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11016;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1103C; return 10; }
    8
}
/// $C11016: moveq #$0, d7
pub fn i_c11016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11018;
    cpu.ir = 0x7E00;
    cpu.d[7] = 0x0;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11018: move.w d4, d2
pub fn i_c11018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1101A;
    cpu.ir = 0x3404;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1101A: subq.w #$4, d2
pub fn i_c1101a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1101C;
    cpu.ir = 0x5942;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C1101C: move.w (a3, d7.w), d0
pub fn i_c1101c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11020;
    cpu.ir = 0x3033;
    let ad2 = cpu.a[3].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C11020: beq.w $c11036
pub fn i_c11020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11024;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC11036; return 10; }
    8
}
/// $C11024: move.w d7, d1
pub fn i_c11024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11026;
    cpu.ir = 0x3207;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C11026: cmp.w $4(a3, d1.w), d0
pub fn i_c11026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1102A;
    cpu.ir = 0xB073;
    let ad2 = cpu.a[3].wrapping_add(0x4).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    14
}
/// $C1102A: bne.b $c11030
pub fn i_c1102a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1102C;
    cpu.ir = 0x6604;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC11030; return 10; }
    8
}
/// $C1102C: clr.w $4(a3, d1.w)
pub fn i_c1102c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11030;
    cpu.ir = 0x4273;
    let ad1 = cpu.a[3].wrapping_add(0x4).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    14
}
/// $C11030: addq.w #$4, d1
pub fn i_c11030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11032;
    cpu.ir = 0x5841;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C11032: cmp.w d1, d2
pub fn i_c11032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11034;
    cpu.ir = 0xB441;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $C11034: bne.b $c11026
pub fn i_c11034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11036;
    cpu.ir = 0x66F0;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC11026; return 10; }
    8
}
/// $C11036: addq.w #$4, d7
pub fn i_c11036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11038;
    cpu.ir = 0x5847;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C11038: cmp.w d7, d2
pub fn i_c11038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1103A;
    cpu.ir = 0xB447;
    let v1 = cpu.d[7] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $C1103A: bne.b $c1101c
pub fn i_c1103a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1103C;
    cpu.ir = 0x66E0;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1101C; return 10; }
    8
}
/// $C1103C: rts
pub fn i_c1103c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1103E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1103E: lea.l -$7e9c(a4), a0
pub fn i_c1103e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11042;
    cpu.ir = 0x41EC;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF8164);
    cpu.a[0] = ad1;
    16
}
/// $C11042: moveq #$7, d7
pub fn i_c11042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11044;
    cpu.ir = 0x7E07;
    cpu.d[7] = 0x7;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11044: move.l #$ffff, (a0)+
pub fn i_c11044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1104A;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xFFFF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C1104A: clr.l (a0)+
pub fn i_c1104a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1104C;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C1104C: dbra d7, $c11044
pub fn i_c1104c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11050;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC11050; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC11050; return 14; }
    cpu.pc = 0xC11044;
    10
}
/// $C11050: lea.l -$7e9c(a4), a0
pub fn i_c11050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11054;
    cpu.ir = 0x41EC;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF8164);
    cpu.a[0] = ad1;
    16
}
/// $C11054: moveq #$0, d7
pub fn i_c11054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11056;
    cpu.ir = 0x7E00;
    cpu.d[7] = 0x0;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11056: move.w (a2, d7.w), d0
pub fn i_c11056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1105A;
    cpu.ir = 0x3032;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C1105A: beq.w $c11076
pub fn i_c1105a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1105E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC11076; return 10; }
    8
}
/// $C1105E: moveq #$0, d6
pub fn i_c1105e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11060;
    cpu.ir = 0x7C00;
    cpu.d[6] = 0x0;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11060: cmp.w (a3, d6.w), d0
pub fn i_c11060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11064;
    cpu.ir = 0xB073;
    let ad2 = cpu.a[3].wrapping_add(0x0).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    14
}
/// $C11064: beq.w $c11076
pub fn i_c11064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11068;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC11076; return 10; }
    8
}
/// $C11068: addq.w #$4, d6
pub fn i_c11068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1106A;
    cpu.ir = 0x5846;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C1106A: cmp.w d4, d6
pub fn i_c1106a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1106C;
    cpu.ir = 0xBC44;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $C1106C: bne.b $c11060
pub fn i_c1106c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1106E;
    cpu.ir = 0x66F2;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC11060; return 10; }
    8
}
/// $C1106E: move.l (a2, d7.w), (a0)+
pub fn i_c1106e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11072;
    cpu.ir = 0x20F2;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C11072: move.l $20(a2, d7.w), (a0)+
pub fn i_c11072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11076;
    cpu.ir = 0x20F2;
    let ad2 = cpu.a[2].wrapping_add(0x20).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C11076: addq.w #$4, d7
pub fn i_c11076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11078;
    cpu.ir = 0x5847;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C11078: cmp.w d4, d7
pub fn i_c11078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1107A;
    cpu.ir = 0xBE44;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $C1107A: bne.b $c11056
pub fn i_c1107a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1107C;
    cpu.ir = 0x66DA;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC11056; return 10; }
    8
}
/// $C1107C: rts
pub fn i_c1107c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1107E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1107E: move.w d4, d6
pub fn i_c1107e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11080;
    cpu.ir = 0x3C04;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C11080: move.w (a2, d6.w), d0
pub fn i_c11080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11084;
    cpu.ir = 0x3032;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C11084: beq.b $c110ac
pub fn i_c11084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11086;
    cpu.ir = 0x6726;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC110AC; return 10; }
    8
}
/// $C11086: moveq #$0, d7
pub fn i_c11086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11088;
    cpu.ir = 0x7E00;
    cpu.d[7] = 0x0;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11088: cmp.w (a3, d7.w), d0
pub fn i_c11088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1108C;
    cpu.ir = 0xB073;
    let ad2 = cpu.a[3].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    14
}
/// $C1108C: beq.b $c110a0
pub fn i_c1108c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1108E;
    cpu.ir = 0x6712;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC110A0; return 10; }
    8
}
/// $C1108E: addq.w #$4, d7
pub fn i_c1108e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11090;
    cpu.ir = 0x5847;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C11090: cmp.w d4, d7
pub fn i_c11090(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11092;
    cpu.ir = 0xBE44;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $C11092: bne.b $c11088
pub fn i_c11092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11094;
    cpu.ir = 0x66F4;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC11088; return 10; }
    8
}
/// $C11094: move.l (a2, d6.w), (a0)+
pub fn i_c11094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11098;
    cpu.ir = 0x20F2;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C11098: move.l $20(a2, d6.w), (a0)+
pub fn i_c11098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1109C;
    cpu.ir = 0x20F2;
    let ad2 = cpu.a[2].wrapping_add(0x20).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C1109C: bra.w $c110ac
pub fn i_c1109c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC110A0;
    cpu.ir = 0x6000;
    cpu.pc = 0xC110AC;
    10
}
/// $C110A0: move.w $2(a2, d6.w), $2(a3, d7.w)
pub fn i_c110a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC110A6;
    cpu.ir = 0x37B2;
    let ad2 = cpu.a[2].wrapping_add(0x2).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[3].wrapping_add(0x2).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    bus.write16(ad4, v3 as u16);
    24
}
/// $C110A6: move.l $20(a2, d6.w), $20(a3, d7.w)
pub fn i_c110a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC110AC;
    cpu.ir = 0x27B2;
    let ad2 = cpu.a[2].wrapping_add(0x20).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[3].wrapping_add(0x20).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    32
}
/// $C110AC: addq.w #$4, d6
pub fn i_c110ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC110AE;
    cpu.ir = 0x5846;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C110AE: cmpi.w #$20, d6
pub fn i_c110ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC110B2;
    cpu.ir = 0xC46;
    let v1 = cpu.d[6] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x20, Size::Word, false);
    8
}
/// $C110B2: bne.b $c11080
pub fn i_c110b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC110B4;
    cpu.ir = 0x66CC;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC11080; return 10; }
    8
}
/// $C110B4: rts
pub fn i_c110b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC110B6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C110B6: moveq #$0, d6
pub fn i_c110b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC110B8;
    cpu.ir = 0x7C00;
    cpu.d[6] = 0x0;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C110B8: move.w (a3, d6.w), d0
pub fn i_c110b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC110BC;
    cpu.ir = 0x3033;
    let ad2 = cpu.a[3].wrapping_add(0x0).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C110BC: beq.b $c110da
pub fn i_c110bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC110BE;
    cpu.ir = 0x671C;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC110DA; return 10; }
    8
}
/// $C110BE: moveq #$0, d7
pub fn i_c110be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC110C0;
    cpu.ir = 0x7E00;
    cpu.d[7] = 0x0;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C110C0: cmp.w (a2, d7.w), d0
pub fn i_c110c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC110C4;
    cpu.ir = 0xB072;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    14
}
/// $C110C4: beq.b $c110ce
pub fn i_c110c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC110C6;
    cpu.ir = 0x6708;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC110CE; return 10; }
    8
}
/// $C110C6: addq.w #$4, d7
pub fn i_c110c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC110C8;
    cpu.ir = 0x5847;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C110C8: cmp.w d4, d7
pub fn i_c110c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC110CA;
    cpu.ir = 0xBE44;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $C110CA: bne.b $c110c0
pub fn i_c110ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC110CC;
    cpu.ir = 0x66F4;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC110C0; return 10; }
    8
}
/// $C110CC: beq.b $c110da
pub fn i_c110cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC110CE;
    cpu.ir = 0x670C;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC110DA; return 10; }
    8
}
/// $C110CE: move.w $2(a2, d7.w), $2(a3, d6.w)
pub fn i_c110ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC110D4;
    cpu.ir = 0x37B2;
    let ad2 = cpu.a[2].wrapping_add(0x2).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[3].wrapping_add(0x2).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    bus.write16(ad4, v3 as u16);
    24
}
/// $C110D4: move.l $20(a2, d7.w), $20(a3, d6.w)
pub fn i_c110d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC110DA;
    cpu.ir = 0x27B2;
    let ad2 = cpu.a[2].wrapping_add(0x20).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[3].wrapping_add(0x20).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    32
}
/// $C110DA: addq.w #$4, d6
pub fn i_c110da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC110DC;
    cpu.ir = 0x5846;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C110DC: cmp.w d4, d6
pub fn i_c110dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC110DE;
    cpu.ir = 0xBC44;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $C110DE: bne.b $c110b8
pub fn i_c110de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC110E0;
    cpu.ir = 0x66D8;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC110B8; return 10; }
    8
}
/// $C110E0: rts
pub fn i_c110e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC110E2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C110E2: moveq #$0, d6
pub fn i_c110e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC110E4;
    cpu.ir = 0x7C00;
    cpu.d[6] = 0x0;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C110E4: move.l (a3, d6.w), (a2, d6.w)
pub fn i_c110e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC110EA;
    cpu.ir = 0x25B3;
    let ad2 = cpu.a[3].wrapping_add(0x0).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    32
}
/// $C110EA: move.l $20(a3, d6.w), $20(a2, d6.w)
pub fn i_c110ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC110F0;
    cpu.ir = 0x25B3;
    let ad2 = cpu.a[3].wrapping_add(0x20).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[2].wrapping_add(0x20).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    32
}
/// $C110F0: addq.w #$4, d6
pub fn i_c110f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC110F2;
    cpu.ir = 0x5846;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C110F2: cmp.w d4, d6
pub fn i_c110f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC110F4;
    cpu.ir = 0xBC44;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $C110F4: bne.b $c110e4
pub fn i_c110f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC110F6;
    cpu.ir = 0x66EE;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC110E4; return 10; }
    8
}
/// $C110F6: lea.l $40(a2), a0
pub fn i_c110f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC110FA;
    cpu.ir = 0x41EA;
    let ad1 = cpu.a[2].wrapping_add(0x40);
    cpu.a[0] = ad1;
    16
}
/// $C110FA: move.l (a0)+, (a2, d6.w)
pub fn i_c110fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC110FE;
    cpu.ir = 0x2598;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C110FE: move.l (a0)+, $20(a2, d6.w)
pub fn i_c110fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11102;
    cpu.ir = 0x2598;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[2].wrapping_add(0x20).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C11102: addq.w #$4, d6
pub fn i_c11102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11104;
    cpu.ir = 0x5846;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C11104: cmpi.w #$20, d6
pub fn i_c11104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11108;
    cpu.ir = 0xC46;
    let v1 = cpu.d[6] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x20, Size::Word, false);
    8
}
/// $C11108: bne.b $c110fa
pub fn i_c11108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1110A;
    cpu.ir = 0x66F0;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC110FA; return 10; }
    8
}
/// $C1110A: rts
pub fn i_c1110a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1110C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1110C: moveq #$0, d6
pub fn i_c1110c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1110E;
    cpu.ir = 0x7C00;
    cpu.d[6] = 0x0;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1110E: move.w (a2, d6.w), d0
pub fn i_c1110e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11112;
    cpu.ir = 0x3032;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C11112: beq.b $c11122
pub fn i_c11112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11114;
    cpu.ir = 0x670E;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC11122; return 10; }
    8
}
/// $C11114: cmpi.w #$ffff, $2(a2, d6.w)
pub fn i_c11114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1111A;
    cpu.ir = 0xC72;
    let ad2 = cpu.a[2].wrapping_add(0x2).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFFF, Size::Word, false);
    18
}
/// $C1111A: bne.b $c11122
pub fn i_c1111a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1111C;
    cpu.ir = 0x6606;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC11122; return 10; }
    8
}
/// $C1111C: move.l -$7efc(a4), $20(a2, d6.w)
pub fn i_c1111c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11122;
    cpu.ir = 0x25AC;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8104);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[2].wrapping_add(0x20).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    30
}
/// $C11122: addq.w #$4, d6
pub fn i_c11122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11124;
    cpu.ir = 0x5846;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C11124: cmp.w d4, d6
pub fn i_c11124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11126;
    cpu.ir = 0xBC44;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $C11126: bne.b $c1110e
pub fn i_c11126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11128;
    cpu.ir = 0x66E6;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1110E; return 10; }
    8
}
/// $C11128: rts
pub fn i_c11128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1112A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1112A: moveq #$0, d6
pub fn i_c1112a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1112C;
    cpu.ir = 0x7C00;
    cpu.d[6] = 0x0;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1112C: move.b d6, -$7ef7(a4)
pub fn i_c1112c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11130;
    cpu.ir = 0x1946;
    let v1 = cpu.d[6] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8109);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C11130: lea.l -$7edc(a4), a2
pub fn i_c11130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11134;
    cpu.ir = 0x45EC;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF8124);
    cpu.a[2] = ad1;
    16
}
/// $C11134: tst.w (a2, d6.w)
pub fn i_c11134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11138;
    cpu.ir = 0x4A72;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    14
}
/// $C11138: beq.w $c1117a
pub fn i_c11138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1113C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1117A; return 10; }
    8
}
/// $C1113C: tst.w $2(a2, d6.w)
pub fn i_c1113c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11140;
    cpu.ir = 0x4A72;
    let ad2 = cpu.a[2].wrapping_add(0x2).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    14
}
/// $C11140: bpl.w $c1114e
pub fn i_c11140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11144;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0xC1114E; return 10; }
    8
}
/// $C11144: cmpi.b #$80, $2(a2, d6.w)
pub fn i_c11144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1114A;
    cpu.ir = 0xC32;
    let ad2 = cpu.a[2].wrapping_add(0x2).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x80, Size::Byte, false);
    18
}
/// $C1114A: bne.w $c1117a
pub fn i_c1114a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1114E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1117A; return 10; }
    8
}
/// $C1114E: jsr $c114d0(pc)
pub fn i_c1114e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11152;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11152);
    cpu.pc = 0xC114D0;
    16
}
/// $C11152: lea.l -$7ee6(a4), a3
pub fn i_c11152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11156;
    cpu.ir = 0x47EC;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF811A);
    cpu.a[3] = ad1;
    16
}
/// $C11156: move.w $2(a2, d6.w), d1
pub fn i_c11156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1115A;
    cpu.ir = 0x3232;
    let ad2 = cpu.a[2].wrapping_add(0x2).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C1115A: andi.w #$7, d1
pub fn i_c1115a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1115E;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C1115E: move.b (a3, d1.w), d0
pub fn i_c1115e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11162;
    cpu.ir = 0x1033;
    let ad2 = cpu.a[3].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $C11162: addq.b #$1, d0
pub fn i_c11162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11164;
    cpu.ir = 0x5200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C11164: cmpi.b #$24, d0
pub fn i_c11164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11168;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x24, Size::Byte, false);
    8
}
/// $C11168: bcs.b $c1116c
pub fn i_c11168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1116A;
    cpu.ir = 0x6502;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC1116C; return 10; }
    8
}
/// $C1116A: moveq #$0, d0
pub fn i_c1116a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1116C;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1116C: move.b d0, (a3, d1.w)
pub fn i_c1116c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11170;
    cpu.ir = 0x1780;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $C11170: jsr $c1155c(pc)
pub fn i_c11170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11174;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11174);
    cpu.pc = 0xC1155C;
    16
}
/// $C11174: jsr $c11508(pc)
pub fn i_c11174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11178;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11178);
    cpu.pc = 0xC11508;
    16
}
/// $C11178: addq.l #$1, (a0)
pub fn i_c11178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1117A;
    cpu.ir = 0x5290;
    let ad1 = cpu.a[0];
    let v2 = bus.read32(ad1);
    let r3 = add_with_flags(cpu, v2, 1, Size::Long, true);
    bus.write32(ad1, r3);
    12
}
/// $C1117A: addq.b #$1, -$7ef7(a4)
pub fn i_c1117a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1117E;
    cpu.ir = 0x522C;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF8109);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1117E: moveq #$0, d6
pub fn i_c1117e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11180;
    cpu.ir = 0x7C00;
    cpu.d[6] = 0x0;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11180: move.b -$7ef7(a4), d6
pub fn i_c11180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11184;
    cpu.ir = 0x1C2C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8109);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C11184: move.b d6, d0
pub fn i_c11184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11186;
    cpu.ir = 0x1006;
    let v1 = cpu.d[6] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C11186: lsl.w #$2, d6
pub fn i_c11186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11188;
    cpu.ir = 0xE54E;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $C11188: cmp.b -$7fb9(a4), d0
pub fn i_c11188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1118C;
    cpu.ir = 0xB02C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8047);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $C1118C: bcs.b $c11130
pub fn i_c1118c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1118E;
    cpu.ir = 0x65A2;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC11130; return 10; }
    8
}
/// $C1118E: rts
pub fn i_c1118e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11190;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C11190: tst.b $10febf.l
pub fn i_c11190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11196;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FEBF) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $C11196: bne.b $c1119e
pub fn i_c11196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11198;
    cpu.ir = 0x6606;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1119E; return 10; }
    8
}
/// $C11198: move.b d0, $3a000d.l
pub fn i_c11198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1119E;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A000D, v1 as u8);
    16
}
/// $C1119E: rts
pub fn i_c1119e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC111A0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C111A0: lea.l $d08000.l, a4
pub fn i_c111a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC111A6;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0xD08000;
    20
}
/// $C111A6: lea.l -$7edc(a4), a2
pub fn i_c111a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC111AA;
    cpu.ir = 0x45EC;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF8124);
    cpu.a[2] = ad1;
    16
}
/// $C111AA: move.b d0, $3a001d.l
pub fn i_c111aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC111B0;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001D, v1 as u8);
    16
}
/// $C111B0: moveq #$0, d5
pub fn i_c111b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC111B2;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C111B2: move.b -$7ef7(a4), d5
pub fn i_c111b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC111B6;
    cpu.ir = 0x1A2C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8109);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C111B6: move.w d5, d6
pub fn i_c111b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC111B8;
    cpu.ir = 0x3C05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C111B8: lsl.w #$2, d6
pub fn i_c111b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC111BA;
    cpu.ir = 0xE54E;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $C111BA: tst.w (a2, d6.w)
pub fn i_c111ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC111BE;
    cpu.ir = 0x4A72;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    14
}
/// $C111BE: beq.w $c1127c
pub fn i_c111be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC111C2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1127C; return 10; }
    8
}
/// $C111C2: tst.w $2(a2, d6.w)
pub fn i_c111c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC111C6;
    cpu.ir = 0x4A72;
    let ad2 = cpu.a[2].wrapping_add(0x2).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    14
}
/// $C111C6: bpl.w $c1127c
pub fn i_c111c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC111CA;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0xC1127C; return 10; }
    8
}
/// $C111CA: cmpi.b #$80, $2(a2, d6.w)
pub fn i_c111ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC111D0;
    cpu.ir = 0xC32;
    let ad2 = cpu.a[2].wrapping_add(0x2).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x80, Size::Byte, false);
    18
}
/// $C111D0: beq.w $c11292
pub fn i_c111d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC111D4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC11292; return 10; }
    8
}
/// $C111D4: moveq #$0, d0
pub fn i_c111d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC111D6;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C111D6: moveq #$0, d7
pub fn i_c111d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC111D8;
    cpu.ir = 0x7E00;
    cpu.d[7] = 0x0;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C111D8: cmp.b $3(a2, d7.w), d0
pub fn i_c111d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC111DC;
    cpu.ir = 0xB032;
    let ad2 = cpu.a[2].wrapping_add(0x3).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    14
}
/// $C111DC: beq.b $c111e8
pub fn i_c111dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC111DE;
    cpu.ir = 0x670A;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC111E8; return 10; }
    8
}
/// $C111DE: addq.w #$4, d7
pub fn i_c111de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC111E0;
    cpu.ir = 0x5847;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C111E0: cmpi.w #$20, d7
pub fn i_c111e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC111E4;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x20, Size::Word, false);
    8
}
/// $C111E4: beq.b $c111ec
pub fn i_c111e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC111E6;
    cpu.ir = 0x6706;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC111EC; return 10; }
    8
}
/// $C111E6: bra.b $c111d8
pub fn i_c111e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC111E8;
    cpu.ir = 0x60F0;
    cpu.pc = 0xC111D8;
    10
}
/// $C111E8: addq.w #$1, d0
pub fn i_c111e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC111EA;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C111EA: bra.b $c111d6
pub fn i_c111ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC111EC;
    cpu.ir = 0x60EA;
    cpu.pc = 0xC111D6;
    10
}
/// $C111EC: move.w d0, $2(a2, d6.w)
pub fn i_c111ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC111F0;
    cpu.ir = 0x3580;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[2].wrapping_add(0x2).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $C111F0: lea.l -$7ee6(a4), a0
pub fn i_c111f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC111F4;
    cpu.ir = 0x41EC;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF811A);
    cpu.a[0] = ad1;
    16
}
/// $C111F4: clr.b (a0, d0.w)
pub fn i_c111f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC111F8;
    cpu.ir = 0x4230;
    let ad1 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    14
}
/// $C111F8: move.w d0, d4
pub fn i_c111f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC111FA;
    cpu.ir = 0x3800;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C111FA: jsr $c114d0(pc)
pub fn i_c111fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC111FE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC111FE);
    cpu.pc = 0xC114D0;
    16
}
/// $C111FE: movea.l a0, a1
pub fn i_c111fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11200;
    cpu.ir = 0x2248;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $C11200: moveq #$0, d0
pub fn i_c11200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11202;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11202: move.w #$8f, d7
pub fn i_c11202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11206;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x8F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C11206: move.l d0, (a1)+
pub fn i_c11206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11208;
    cpu.ir = 0x22C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C11208: dbra d7, $c11206
pub fn i_c11208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1120C;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC1120C; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC1120C; return 14; }
    cpu.pc = 0xC11206;
    10
}
/// $C1120C: jsr $c1155c(pc)
pub fn i_c1120c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11210;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11210);
    cpu.pc = 0xC1155C;
    16
}
/// $C11210: move.w d4, d0
pub fn i_c11210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11212;
    cpu.ir = 0x3004;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C11212: lsl.w #$4, d0
pub fn i_c11212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11214;
    cpu.ir = 0xE948;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $C11214: lea.l $1b20(a4), a0
pub fn i_c11214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11218;
    cpu.ir = 0x41EC;
    let ad1 = cpu.a[4].wrapping_add(0x1B20);
    cpu.a[0] = ad1;
    16
}
/// $C11218: jsr $c1156e(pc)
pub fn i_c11218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1121C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1121C);
    cpu.pc = 0xC1156E;
    16
}
/// $C1121C: move.w d4, d0
pub fn i_c1121c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1121E;
    cpu.ir = 0x3004;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1121E: jsr $c11524(pc)
pub fn i_c1121e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11222;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11222);
    cpu.pc = 0xC11524;
    16
}
/// $C11222: moveq #$17, d7
pub fn i_c11222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11224;
    cpu.ir = 0x7E17;
    cpu.d[7] = 0x17;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11224: clr.l (a0)+
pub fn i_c11224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11226;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C11226: dbra d7, $c11224
pub fn i_c11226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1122A;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC1122A; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC1122A; return 14; }
    cpu.pc = 0xC11224;
    10
}
/// $C1122A: move.w d4, d0
pub fn i_c1122a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1122C;
    cpu.ir = 0x3004;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1122C: jsr $c11508(pc)
pub fn i_c1122c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11230;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11230);
    cpu.pc = 0xC11508;
    16
}
/// $C11230: clr.l (a0)
pub fn i_c11230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11232;
    cpu.ir = 0x4290;
    let ad1 = cpu.a[0];
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C11232: add.w d4, d4
pub fn i_c11232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11234;
    cpu.ir = 0xD844;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C11234: add.w d4, d4
pub fn i_c11234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11236;
    cpu.ir = 0xD844;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C11236: lea.l $c114b0(pc), a0
pub fn i_c11236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1123A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0xC114B0;
    16
}
/// $C1123A: movea.l (a0, d4.w), a0
pub fn i_c1123a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1123E;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $C1123E: moveq #$0, d0
pub fn i_c1123e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11240;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11240: move.w #$2f, d7
pub fn i_c11240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11244;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x2F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C11244: move.l d0, (a0)+
pub fn i_c11244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11246;
    cpu.ir = 0x20C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C11246: dbra d7, $c11244
pub fn i_c11246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1124A;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC1124A; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC1124A; return 14; }
    cpu.pc = 0xC11244;
    10
}
/// $C1124A: jsr $c10f90(pc)
pub fn i_c1124a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1124E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1124E);
    cpu.pc = 0xC10F90;
    16
}
/// $C1124E: moveq #$0, d0
pub fn i_c1124e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11250;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11250: move.b $c00401.l, d0
pub fn i_c11250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11256;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0xC00401) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C11256: add.w d0, d0
pub fn i_c11256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11258;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C11258: add.w d0, d0
pub fn i_c11258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1125A;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C1125A: addi.l #$116, d0
pub fn i_c1125a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11260;
    cpu.ir = 0x680;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = add_with_flags(cpu, v1, 0x116, Size::Long, true);
    cpu.d[0] = r2;
    8
}
/// $C11260: movea.l d0, a1
pub fn i_c11260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11262;
    cpu.ir = 0x2240;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $C11262: movea.l (a1), a0
pub fn i_c11262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11264;
    cpu.ir = 0x2051;
    let v2 = bus.read32(cpu.a[1]);
    cpu.a[0] = v2;
    12
}
/// $C11264: move.w $2(a2, d6.w), d0
pub fn i_c11264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11268;
    cpu.ir = 0x3032;
    let ad2 = cpu.a[2].wrapping_add(0x2).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C11268: lsl.w #$4, d0
pub fn i_c11268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1126A;
    cpu.ir = 0xE948;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $C1126A: lea.l -$7d60(a4), a1
pub fn i_c1126a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1126E;
    cpu.ir = 0x43EC;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF82A0);
    cpu.a[1] = ad1;
    16
}
/// $C1126E: lea.l (a1, d0.w), a1
pub fn i_c1126e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11272;
    cpu.ir = 0x43F1;
    let ad1 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    cpu.a[1] = ad1;
    18
}
/// $C11272: move.l (a0)+, (a1)+
pub fn i_c11272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11274;
    cpu.ir = 0x22D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C11274: move.l (a0)+, (a1)+
pub fn i_c11274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11276;
    cpu.ir = 0x22D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C11276: move.l (a0)+, (a1)+
pub fn i_c11276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11278;
    cpu.ir = 0x22D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C11278: move.l (a0)+, (a1)+
pub fn i_c11278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1127A;
    cpu.ir = 0x22D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C1127A: bra.b $c1128a
pub fn i_c1127a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1127C;
    cpu.ir = 0x600E;
    cpu.pc = 0xC1128A;
    10
}
/// $C1127C: addq.b #$1, d5
pub fn i_c1127c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1127E;
    cpu.ir = 0x5205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C1127E: move.b d5, -$7ef7(a4)
pub fn i_c1127e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11282;
    cpu.ir = 0x1945;
    let v1 = cpu.d[5] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8109);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C11282: cmpi.b #$8, d5
pub fn i_c11282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11286;
    cpu.ir = 0xC05;
    let v1 = cpu.d[5] & 0xFF;
    sub_with_flags(cpu, v1, 0x8, Size::Byte, false);
    8
}
/// $C11286: bcs.w $c111b6
pub fn i_c11286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1128A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC111B6; return 10; }
    8
}
/// $C1128A: move.b d0, $3a000d.l
pub fn i_c1128a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11290;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A000D, v1 as u8);
    16
}
/// $C11290: rts
pub fn i_c11290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11292;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C11292: cmp.b -$7fb9(a4), d5
pub fn i_c11292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11296;
    cpu.ir = 0xBA2C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8047);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[5] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $C11296: bcc.b $c1127c
pub fn i_c11296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11298;
    cpu.ir = 0x64E4;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC1127C; return 10; }
    8
}
/// $C11298: clr.b $2(a2, d6.w)
pub fn i_c11298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1129C;
    cpu.ir = 0x4232;
    let ad1 = cpu.a[2].wrapping_add(0x2).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    14
}
/// $C1129C: jsr $c10f90(pc)
pub fn i_c1129c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC112A0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC112A0);
    cpu.pc = 0xC10F90;
    16
}
/// $C112A0: bra.b $c1128a
pub fn i_c112a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC112A2;
    cpu.ir = 0x60E8;
    cpu.pc = 0xC1128A;
    10
}
/// $C112A2: lea.l $d08000.l, a4
pub fn i_c112a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC112A8;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0xD08000;
    20
}
/// $C112A8: moveq #$0, d0
pub fn i_c112a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC112AA;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C112AA: move.b -$7ef8(a4), d0
pub fn i_c112aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC112AE;
    cpu.ir = 0x102C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8108);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C112AE: cmpi.b #$8, d0
pub fn i_c112ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC112B2;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x8, Size::Byte, false);
    8
}
/// $C112B2: bcc.w $c112ea
pub fn i_c112b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC112B6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC112EA; return 10; }
    8
}
/// $C112B6: add.w d0, d0
pub fn i_c112b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC112B8;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C112B8: add.w d0, d0
pub fn i_c112b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC112BA;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C112BA: lea.l -$7edc(a4), a0
pub fn i_c112ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC112BE;
    cpu.ir = 0x41EC;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF8124);
    cpu.a[0] = ad1;
    16
}
/// $C112BE: move.w $2(a0, d0.w), d0
pub fn i_c112be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC112C2;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C112C2: cmpi.w #$8, d0
pub fn i_c112c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC112C6;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x8, Size::Word, false);
    8
}
/// $C112C6: bcc.b $c112ea
pub fn i_c112c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC112C8;
    cpu.ir = 0x6422;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC112EA; return 10; }
    8
}
/// $C112C8: move.b d0, $3a001d.l
pub fn i_c112c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC112CE;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001D, v1 as u8);
    16
}
/// $C112CE: lsl.w #$4, d0
pub fn i_c112ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC112D0;
    cpu.ir = 0xE948;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $C112D0: lea.l -$7de0(a4), a0
pub fn i_c112d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC112D4;
    cpu.ir = 0x41EC;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF8220);
    cpu.a[0] = ad1;
    16
}
/// $C112D4: lea.l (a0, d0.w), a0
pub fn i_c112d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC112D8;
    cpu.ir = 0x41F0;
    let ad1 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    cpu.a[0] = ad1;
    18
}
/// $C112D8: lea.l $a84(a5), a1
pub fn i_c112d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC112DC;
    cpu.ir = 0x43ED;
    let ad1 = cpu.a[5].wrapping_add(0xA84);
    cpu.a[1] = ad1;
    16
}
/// $C112DC: move.l (a1)+, (a0)+
pub fn i_c112dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC112DE;
    cpu.ir = 0x20D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C112DE: move.l (a1)+, (a0)+
pub fn i_c112de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC112E0;
    cpu.ir = 0x20D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C112E0: move.l (a1)+, (a0)+
pub fn i_c112e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC112E2;
    cpu.ir = 0x20D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C112E2: move.l (a1)+, (a0)+
pub fn i_c112e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC112E4;
    cpu.ir = 0x20D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C112E4: move.b d0, $3a000d.l
pub fn i_c112e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC112EA;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A000D, v1 as u8);
    16
}
/// $C112EA: rts
pub fn i_c112ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC112EC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C112EC: lea.l $d08000.l, a4
pub fn i_c112ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC112F2;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0xD08000;
    20
}
/// $C112F2: moveq #$0, d0
pub fn i_c112f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC112F4;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C112F4: move.b $d00108.l, d0
pub fn i_c112f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC112FA;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0xD00108) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C112FA: cmpi.b #$8, d0
pub fn i_c112fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC112FE;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x8, Size::Byte, false);
    8
}
/// $C112FE: bcc.b $c112ea
pub fn i_c112fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11300;
    cpu.ir = 0x64EA;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC112EA; return 10; }
    8
}
/// $C11300: move.b d0, $3a001d.l
pub fn i_c11300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11306;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001D, v1 as u8);
    16
}
/// $C11306: lsl.w #$2, d0
pub fn i_c11306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11308;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $C11308: lea.l -$7edc(a4), a0
pub fn i_c11308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1130C;
    cpu.ir = 0x41EC;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF8124);
    cpu.a[0] = ad1;
    16
}
/// $C1130C: move.w $2(a0, d0.w), d0
pub fn i_c1130c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11310;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C11310: ror.w #$4, d0
pub fn i_c11310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11312;
    cpu.ir = 0xE858;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 3, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $C11312: addi.l #$d00320, d0
pub fn i_c11312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11318;
    cpu.ir = 0x680;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = add_with_flags(cpu, v1, 0xD00320, Size::Long, true);
    cpu.d[0] = r2;
    8
}
/// $C11318: movea.l d0, a0
pub fn i_c11318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1131A;
    cpu.ir = 0x2040;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C1131A: movea.l $10e.w, a1
pub fn i_c1131a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1131E;
    cpu.ir = 0x2278;
    let v2 = bus.read32(0x10E);
    cpu.a[1] = v2;
    16
}
/// $C1131E: moveq #$0, d0
pub fn i_c1131e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11320;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11320: move.w $112.w, d0
pub fn i_c11320(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11324;
    cpu.ir = 0x3038;
    let v2 = bus.read16(0x112) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    12
}
/// $C11324: movea.l a1, a2
pub fn i_c11324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11326;
    cpu.ir = 0x2449;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.a[2] = v1;
    4
}
/// $C11326: adda.l d0, a2
pub fn i_c11326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11328;
    cpu.ir = 0xD5C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[2] = cpu.a[2].wrapping_add(v1);
    8
}
/// $C11328: move.b d0, $300001.l
pub fn i_c11328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1132E;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C1132E: move.b (a1)+, (a0)+
pub fn i_c1132e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11330;
    cpu.ir = 0x10D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $C11330: cmpa.l a2, a1
pub fn i_c11330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11332;
    cpu.ir = 0xB3CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[1];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $C11332: bcs.b $c11328
pub fn i_c11332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11334;
    cpu.ir = 0x65F4;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC11328; return 10; }
    8
}
/// $C11334: move.b d0, $3a000d.l
pub fn i_c11334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1133A;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A000D, v1 as u8);
    16
}
/// $C1133A: rts
pub fn i_c1133a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1133C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C11372: move.b d0, $3a001d.l
pub fn i_c11372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11378;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001D, v1 as u8);
    16
}
/// $C11378: lea.l $d08000.l, a4
pub fn i_c11378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1137E;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0xD08000;
    20
}
/// $C1137E: lea.l -$7edc(a4), a2
pub fn i_c1137e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11382;
    cpu.ir = 0x45EC;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF8124);
    cpu.a[2] = ad1;
    16
}
/// $C11382: move.b -$7ef8(a4), d5
pub fn i_c11382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11386;
    cpu.ir = 0x1A2C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8108);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C11386: move.b d5, -$7ef7(a4)
pub fn i_c11386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1138A;
    cpu.ir = 0x1945;
    let v1 = cpu.d[5] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8109);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C1138A: bra.w $c113e8
pub fn i_c1138a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1138E;
    cpu.ir = 0x6000;
    cpu.pc = 0xC113E8;
    10
}
/// $C1138E: lea.l $d08000.l, a4
pub fn i_c1138e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11394;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0xD08000;
    20
}
/// $C11394: move.b #$ff, $380031.l
pub fn i_c11394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1139C;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x380031, v1 as u8);
    16
}
/// $C1139C: bsr.w $c1705e
pub fn i_c1139c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC113A0;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0xC113A0);
    cpu.pc = 0xC1705E;
    18
}
/// $C113A0: lea.l -$7edc(a4), a2
pub fn i_c113a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC113A4;
    cpu.ir = 0x45EC;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF8124);
    cpu.a[2] = ad1;
    16
}
/// $C113A4: move.b -$7ef8(a4), d5
pub fn i_c113a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC113A8;
    cpu.ir = 0x1A2C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8108);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C113A8: addq.b #$1, d5
pub fn i_c113a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC113AA;
    cpu.ir = 0x5205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C113AA: cmp.b -$7fb9(a4), d5
pub fn i_c113aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC113AE;
    cpu.ir = 0xBA2C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8047);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[5] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $C113AE: bcs.b $c113b2
pub fn i_c113ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC113B0;
    cpu.ir = 0x6502;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC113B2; return 10; }
    8
}
/// $C113B0: moveq #$0, d5
pub fn i_c113b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC113B2;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C113B2: moveq #$0, d0
pub fn i_c113b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC113B4;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C113B4: move.b d5, d0
pub fn i_c113b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC113B6;
    cpu.ir = 0x1005;
    let v1 = cpu.d[5] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C113B6: add.w d0, d0
pub fn i_c113b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC113B8;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C113B8: add.w d0, d0
pub fn i_c113b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC113BA;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C113BA: tst.w (a2, d0.w)
pub fn i_c113ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC113BE;
    cpu.ir = 0x4A72;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    14
}
/// $C113BE: beq.b $c113a8
pub fn i_c113be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC113C0;
    cpu.ir = 0x67E8;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC113A8; return 10; }
    8
}
/// $C113C0: move.b d0, $3a001d.l
pub fn i_c113c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC113C6;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001D, v1 as u8);
    16
}
/// $C113C6: move.b d5, d0
pub fn i_c113c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC113C8;
    cpu.ir = 0x1005;
    let v1 = cpu.d[5] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C113C8: not.b d0
pub fn i_c113c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC113CA;
    cpu.ir = 0x4600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = !v1 & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C113CA: move.b d0, $380041.l
pub fn i_c113ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC113D0;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x380041, v1 as u8);
    16
}
/// $C113D0: move.b #$f7, $380031.l
pub fn i_c113d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC113D8;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xF7;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x380031, v1 as u8);
    16
}
/// $C113D8: move.b d5, -$7ef7(a4)
pub fn i_c113d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC113DC;
    cpu.ir = 0x1945;
    let v1 = cpu.d[5] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8109);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C113DC: jsr $c10f90(pc)
pub fn i_c113dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC113E0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC113E0);
    cpu.pc = 0xC10F90;
    16
}
/// $C113E0: move.b #$3, $320000.l
pub fn i_c113e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC113E8;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x320000, v1 as u8);
    16
}
/// $C113E8: moveq #$0, d0
pub fn i_c113e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC113EA;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C113EA: move.b -$7ef8(a4), d0
pub fn i_c113ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC113EE;
    cpu.ir = 0x102C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8108);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C113EE: add.w d0, d0
pub fn i_c113ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC113F0;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C113F0: add.w d0, d0
pub fn i_c113f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC113F2;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C113F2: move.w (a2, d0.w), $a84(a5)
pub fn i_c113f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC113F8;
    cpu.ir = 0x3B72;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xA84);
    bus.write16(ad4, v3 as u16);
    22
}
/// $C113F8: move.w $2(a2, d0.w), d0
pub fn i_c113f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC113FC;
    cpu.ir = 0x3032;
    let ad2 = cpu.a[2].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C113FC: andi.w #$ff, d0
pub fn i_c113fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11400;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C11400: move.w d0, d6
pub fn i_c11400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11402;
    cpu.ir = 0x3C00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C11402: ror.w #$4, d0
pub fn i_c11402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11404;
    cpu.ir = 0xE858;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 3, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $C11404: addi.l #$d00320, d0
pub fn i_c11404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1140A;
    cpu.ir = 0x680;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = add_with_flags(cpu, v1, 0xD00320, Size::Long, true);
    cpu.d[0] = r2;
    8
}
/// $C1140A: movea.l d0, a0
pub fn i_c1140a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1140C;
    cpu.ir = 0x2040;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C1140C: movea.l $10e.w, a1
pub fn i_c1140c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11410;
    cpu.ir = 0x2278;
    let v2 = bus.read32(0x10E);
    cpu.a[1] = v2;
    16
}
/// $C11410: moveq #$0, d0
pub fn i_c11410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11412;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11412: move.w $112.w, d0
pub fn i_c11412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11416;
    cpu.ir = 0x3038;
    let v2 = bus.read16(0x112) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    12
}
/// $C11416: movea.l a1, a2
pub fn i_c11416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11418;
    cpu.ir = 0x2449;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.a[2] = v1;
    4
}
/// $C11418: adda.l d0, a2
pub fn i_c11418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1141A;
    cpu.ir = 0xD5C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[2] = cpu.a[2].wrapping_add(v1);
    8
}
/// $C1141A: move.b d0, $300001.l
pub fn i_c1141a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11420;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C11420: move.b (a0)+, (a1)+
pub fn i_c11420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11422;
    cpu.ir = 0x12D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $C11422: cmpa.l a2, a1
pub fn i_c11422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11424;
    cpu.ir = 0xB3CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[1];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $C11424: bcs.b $c1141a
pub fn i_c11424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11426;
    cpu.ir = 0x65F4;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC1141A; return 10; }
    8
}
/// $C11426: move.w d6, d0
pub fn i_c11426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11428;
    cpu.ir = 0x3006;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C11428: lsl.w #$4, d0
pub fn i_c11428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1142A;
    cpu.ir = 0xE948;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $C1142A: lea.l -$7de0(a4), a1
pub fn i_c1142a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1142E;
    cpu.ir = 0x43EC;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF8220);
    cpu.a[1] = ad1;
    16
}
/// $C1142E: lea.l (a1, d0.w), a1
pub fn i_c1142e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11432;
    cpu.ir = 0x43F1;
    let ad1 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    cpu.a[1] = ad1;
    18
}
/// $C11432: lea.l $a84(a5), a0
pub fn i_c11432(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11436;
    cpu.ir = 0x41ED;
    let ad1 = cpu.a[5].wrapping_add(0xA84);
    cpu.a[0] = ad1;
    16
}
/// $C11436: move.w (a0), d0
pub fn i_c11436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11438;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C11438: move.l (a1)+, (a0)+
pub fn i_c11438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1143A;
    cpu.ir = 0x20D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C1143A: move.l (a1)+, (a0)+
pub fn i_c1143a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1143C;
    cpu.ir = 0x20D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C1143C: move.l (a1)+, (a0)+
pub fn i_c1143c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1143E;
    cpu.ir = 0x20D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C1143E: move.l (a1)+, (a0)+
pub fn i_c1143e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11440;
    cpu.ir = 0x20D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C11440: cmpi.w #$5, d0
pub fn i_c11440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11444;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x5, Size::Word, false);
    8
}
/// $C11444: bne.b $c11452
pub fn i_c11444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11446;
    cpu.ir = 0x660C;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC11452; return 10; }
    8
}
/// $C11446: tst.b -$7fba(a4)
pub fn i_c11446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1144A;
    cpu.ir = 0x4A2C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8046);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C1144A: beq.b $c11452
pub fn i_c1144a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1144C;
    cpu.ir = 0x6706;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC11452; return 10; }
    8
}
/// $C1144C: move.b #$1, $a8a(a5)
pub fn i_c1144c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11452;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xA8A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C11452: jsr $c114d0(pc)
pub fn i_c11452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11456;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11456);
    cpu.pc = 0xC114D0;
    16
}
/// $C11456: lea.l -$7ee6(a4), a1
pub fn i_c11456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1145A;
    cpu.ir = 0x43EC;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF811A);
    cpu.a[1] = ad1;
    16
}
/// $C1145A: moveq #$0, d0
pub fn i_c1145a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1145C;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1145C: move.b (a1, d6.w), d0
pub fn i_c1145c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11460;
    cpu.ir = 0x1031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $C11460: lsl.w #$4, d0
pub fn i_c11460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11462;
    cpu.ir = 0xE948;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $C11462: lea.l (a0, d0.w), a0
pub fn i_c11462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11466;
    cpu.ir = 0x41F0;
    let ad1 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    cpu.a[0] = ad1;
    18
}
/// $C11466: move.l a0, -$7ef6(a4)
pub fn i_c11466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1146A;
    cpu.ir = 0x2948;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF810A);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1146A: move.w d6, d0
pub fn i_c1146a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1146C;
    cpu.ir = 0x3006;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1146C: add.w d0, d0
pub fn i_c1146c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1146E;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C1146E: add.w d0, d0
pub fn i_c1146e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11470;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C11470: movea.l $c114b0(pc, d0.w), a0
pub fn i_c11470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11474;
    cpu.ir = 0x207B;
    let ad2 = 0xC114B0_u32.wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $C11474: move.b -$7efb(a4), d0
pub fn i_c11474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11478;
    cpu.ir = 0x102C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8105);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C11478: jsr $c1217c(pc)
pub fn i_c11478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1147C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1147C);
    cpu.pc = 0xC1217C;
    16
}
/// $C1147C: lsl.w #$4, d0
pub fn i_c1147c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1147E;
    cpu.ir = 0xE948;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $C1147E: lea.l -$10(a0, d0.w), a0
pub fn i_c1147e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11482;
    cpu.ir = 0x41F0;
    let ad1 = cpu.a[0].wrapping_add(0xFFFFFFF0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    cpu.a[0] = ad1;
    18
}
/// $C11482: move.l a0, -$7e5c(a4)
pub fn i_c11482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11486;
    cpu.ir = 0x2948;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF81A4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C11486: move.w d6, d0
pub fn i_c11486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11488;
    cpu.ir = 0x3006;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C11488: jsr $c11524(pc)
pub fn i_c11488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1148C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1148C);
    cpu.pc = 0xC11524;
    16
}
/// $C1148C: move.l a0, -$7e58(a4)
pub fn i_c1148c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11490;
    cpu.ir = 0x2948;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF81A8);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C11490: move.w d6, d0
pub fn i_c11490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11492;
    cpu.ir = 0x3006;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C11492: lsl.w #$4, d0
pub fn i_c11492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11494;
    cpu.ir = 0xE948;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $C11494: lea.l $1b20(a4), a0
pub fn i_c11494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11498;
    cpu.ir = 0x41EC;
    let ad1 = cpu.a[4].wrapping_add(0x1B20);
    cpu.a[0] = ad1;
    16
}
/// $C11498: lea.l (a0, d0.w), a0
pub fn i_c11498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1149C;
    cpu.ir = 0x41F0;
    let ad1 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    cpu.a[0] = ad1;
    18
}
/// $C1149C: move.l a0, -$7ef2(a4)
pub fn i_c1149c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC114A0;
    cpu.ir = 0x2948;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF810E);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C114A0: move.b #$ff, $380031.l
pub fn i_c114a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC114A8;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x380031, v1 as u8);
    16
}
/// $C114A8: move.b d0, $3a000d.l
pub fn i_c114a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC114AE;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A000D, v1 as u8);
    16
}
/// $C114AE: rts
pub fn i_c114ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC114B0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C114D0: moveq #$0, d0
pub fn i_c114d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC114D2;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C114D2: move.b -$7ef7(a4), d0
pub fn i_c114d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC114D6;
    cpu.ir = 0x102C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8109);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C114D6: lsl.w #$2, d0
pub fn i_c114d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC114D8;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $C114D8: lea.l -$7edc(a4), a0
pub fn i_c114d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC114DC;
    cpu.ir = 0x41EC;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF8124);
    cpu.a[0] = ad1;
    16
}
/// $C114DC: move.w $2(a0, d0.w), d0
pub fn i_c114dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC114E0;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C114E0: lsl.w #$2, d0
pub fn i_c114e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC114E2;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $C114E2: movea.l $c114e8(pc, d0.w), a0
pub fn i_c114e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC114E6;
    cpu.ir = 0x207B;
    let ad2 = 0xC114E8_u32.wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $C114E6: rts
pub fn i_c114e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC114E8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C11508: moveq #$0, d0
pub fn i_c11508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1150A;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1150A: move.b -$7ef7(a4), d0
pub fn i_c1150a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1150E;
    cpu.ir = 0x102C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8109);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C1150E: lsl.w #$2, d0
pub fn i_c1150e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11510;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $C11510: lea.l -$7edc(a4), a0
pub fn i_c11510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11514;
    cpu.ir = 0x41EC;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF8124);
    cpu.a[0] = ad1;
    16
}
/// $C11514: move.w $2(a0, d0.w), d0
pub fn i_c11514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11518;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C11518: lsl.w #$2, d0
pub fn i_c11518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1151A;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $C1151A: lea.l -$7e00(a4), a0
pub fn i_c1151a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1151E;
    cpu.ir = 0x41EC;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF8200);
    cpu.a[0] = ad1;
    16
}
/// $C1151E: lea.l (a0, d0.w), a0
pub fn i_c1151e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11522;
    cpu.ir = 0x41F0;
    let ad1 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    cpu.a[0] = ad1;
    18
}
/// $C11522: rts
pub fn i_c11522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11524;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C11524: moveq #$0, d0
pub fn i_c11524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11526;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11526: move.b -$7ef7(a4), d0
pub fn i_c11526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1152A;
    cpu.ir = 0x102C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8109);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C1152A: lsl.w #$2, d0
pub fn i_c1152a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1152C;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $C1152C: lea.l -$7edc(a4), a0
pub fn i_c1152c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11530;
    cpu.ir = 0x41EC;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF8124);
    cpu.a[0] = ad1;
    16
}
/// $C11530: move.w $2(a0, d0.w), d0
pub fn i_c11530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11534;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C11534: lsl.w #$2, d0
pub fn i_c11534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11536;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $C11536: movea.l $c1153c(pc, d0.w), a0
pub fn i_c11536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1153A;
    cpu.ir = 0x207B;
    let ad2 = 0xC1153C_u32.wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $C1153A: rts
pub fn i_c1153a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1153C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1155C: andi.w #$ff, d0
pub fn i_c1155c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11560;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C11560: lsl.w #$4, d0
pub fn i_c11560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11562;
    cpu.ir = 0xE948;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $C11562: jsr $c1156e(pc)
pub fn i_c11562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11566;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11566);
    cpu.pc = 0xC1156E;
    16
}
/// $C11566: move.l -$7efc(a4), (a0, d0.w)
pub fn i_c11566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1156C;
    cpu.ir = 0x21AC;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8104);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    30
}
/// $C1156C: rts
pub fn i_c1156c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1156E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1156E: moveq #$0, d1
pub fn i_c1156e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11570;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11570: move.l d1, (a0, d0.w)
pub fn i_c11570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11574;
    cpu.ir = 0x2181;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $C11574: move.l d1, $4(a0, d0.w)
pub fn i_c11574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11578;
    cpu.ir = 0x2181;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x4).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $C11578: move.l d1, $8(a0, d0.w)
pub fn i_c11578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1157C;
    cpu.ir = 0x2181;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x8).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $C1157C: move.l d1, $c(a0, d0.w)
pub fn i_c1157c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11580;
    cpu.ir = 0x2181;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0xC).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $C11580: rts
pub fn i_c11580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11582;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C11592: move.b $300001.l, d0
pub fn i_c11592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11598;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x300001) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C11598: beq.b $c115b0
pub fn i_c11598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1159A;
    cpu.ir = 0x6716;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC115B0; return 10; }
    8
}
/// $C1159A: lea.l $d00010.l, a6
pub fn i_c1159a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC115A0;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0xD00010;
    20
}
/// $C115A0: lea.l $c11582(pc), a4
pub fn i_c115a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC115A4;
    cpu.ir = 0x49FA;
    cpu.a[4] = 0xC11582;
    16
}
/// $C115A4: moveq #$f, d7
pub fn i_c115a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC115A6;
    cpu.ir = 0x7E0F;
    cpu.d[7] = 0xF;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C115A6: cmpm.b (a4)+, (a6)+
pub fn i_c115a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC115A8;
    cpu.ir = 0xBD0C;
    let ay = cpu.a[4]; cpu.a[4] = ay.wrapping_add(1);
    let v1 = bus.read8(ay) as u32;
    let ax = cpu.a[6];
    let v2 = bus.read8(ax) as u32;
    cpu.a[6] = cpu.a[6].wrapping_add(1);
    sub_with_flags(cpu, v2, v1, Size::Byte, false);
    12
}
/// $C115A8: bne.b $c115b0
pub fn i_c115a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC115AA;
    cpu.ir = 0x6606;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC115B0; return 10; }
    8
}
/// $C115AA: dbra d7, $c115a6
pub fn i_c115aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC115AE;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC115AE; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC115AE; return 14; }
    cpu.pc = 0xC115A6;
    10
}
/// $C115AE: rts
pub fn i_c115ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC115B0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C115B0: jsr $c12106(pc)
pub fn i_c115b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC115B4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC115B4);
    cpu.pc = 0xC12106;
    16
}
/// $C115B4: move.b $300001.l, d0
pub fn i_c115b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC115BA;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x300001) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C115BA: bne.b $c115be
pub fn i_c115ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC115BC;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC115BE; return 10; }
    8
}
/// $C115BC: rts
pub fn i_c115bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC115BE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C115BE: move.b #$1, $10febf.l
pub fn i_c115be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC115C6;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10FEBF, v1 as u8);
    16
}
/// $C115C6: move.b d0, $3a001d.l
pub fn i_c115c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC115CC;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001D, v1 as u8);
    16
}
/// $C115CC: lea.l $d10000.l, a0
pub fn i_c115cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC115D2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0xD10000;
    20
}
/// $C115D2: moveq #$0, d0
pub fn i_c115d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC115D4;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C115D4: move.l d0, d1
pub fn i_c115d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC115D6;
    cpu.ir = 0x2200;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[1] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C115D6: move.l d0, d2
pub fn i_c115d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC115D8;
    cpu.ir = 0x2400;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[2] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C115D8: move.l d0, d3
pub fn i_c115d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC115DA;
    cpu.ir = 0x2600;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[3] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C115DA: move.w #$3ff, d7
pub fn i_c115da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC115DE;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x3FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C115DE: move.b d0, $300001.l
pub fn i_c115de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC115E4;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C115E4: movem.l d0-d3, -(a0)
pub fn i_c115e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC115E8;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C115E8: movem.l d0-d3, -(a0)
pub fn i_c115e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC115EC;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C115EC: movem.l d0-d3, -(a0)
pub fn i_c115ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC115F0;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C115F0: movem.l d0-d3, -(a0)
pub fn i_c115f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC115F4;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C115F4: dbra d7, $c115de
pub fn i_c115f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC115F8;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC115F8; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC115F8; return 14; }
    cpu.pc = 0xC115DE;
    10
}
/// $C115F8: lea.l $d00010.l, a6
pub fn i_c115f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC115FE;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0xD00010;
    20
}
/// $C115FE: lea.l $c11582(pc), a4
pub fn i_c115fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11602;
    cpu.ir = 0x49FA;
    cpu.a[4] = 0xC11582;
    16
}
/// $C11602: moveq #$f, d7
pub fn i_c11602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11604;
    cpu.ir = 0x7E0F;
    cpu.d[7] = 0xF;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11604: move.b (a4)+, (a6)+
pub fn i_c11604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11606;
    cpu.ir = 0x1CDC;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6]; cpu.a[6] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $C11606: dbra d7, $c11604
pub fn i_c11606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1160A;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC1160A; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC1160A; return 14; }
    cpu.pc = 0xC11604;
    10
}
/// $C1160A: lea.l $c1f800.l, a4
pub fn i_c1160a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11610;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0xC1F800;
    20
}
/// $C11610: lea.l $d0003a.l, a6
pub fn i_c11610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11616;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0xD0003A;
    20
}
/// $C11616: moveq #$c, d7
pub fn i_c11616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11618;
    cpu.ir = 0x7E0C;
    cpu.d[7] = 0xC;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11618: move.b (a4)+, (a6)+
pub fn i_c11618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1161A;
    cpu.ir = 0x1CDC;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6]; cpu.a[6] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $C1161A: dbra d7, $c11618
pub fn i_c1161a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1161E;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC1161E; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC1161E; return 14; }
    cpu.pc = 0xC11618;
    10
}
/// $C1161E: move.b #$23, $d00122.l
pub fn i_c1161e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11626;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x23;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0xD00122, v1 as u8);
    16
}
/// $C11626: lea.l $d00124.l, a0
pub fn i_c11626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1162C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0xD00124;
    20
}
/// $C1162C: move.l #$ffff, d0
pub fn i_c1162c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11632;
    cpu.ir = 0x203C;
    let v1: u32 = 0xFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11632: moveq #$7, d7
pub fn i_c11632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11634;
    cpu.ir = 0x7E07;
    cpu.d[7] = 0x7;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11634: move.l d0, (a0)+
pub fn i_c11634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11636;
    cpu.ir = 0x20C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C11636: dbra d7, $c11634
pub fn i_c11636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1163A;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC1163A; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC1163A; return 14; }
    cpu.pc = 0xC11634;
    10
}
/// $C1163A: move.l #$f0f0f0f0, $d001ac.l
pub fn i_c1163a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11644;
    cpu.ir = 0x23FC;
    let v1: u32 = 0xF0F0F0F0;
    bus.write32(0xD001AC, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C11644: move.w #$ffff, $d001b0.l
pub fn i_c11644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1164C;
    cpu.ir = 0x33FC;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0xD001B0, v1 as u16);
    16
}
/// $C1164C: move.b d0, $3a000d.l
pub fn i_c1164c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11652;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A000D, v1 as u8);
    16
}
/// $C11652: rts
pub fn i_c11652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11654;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C11654: move.b #$ff, $380031.l
pub fn i_c11654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1165C;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x380031, v1 as u8);
    16
}
/// $C1165C: bsr.w $c17062
pub fn i_c1165c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11660;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0xC11660);
    cpu.pc = 0xC17062;
    18
}
/// $C11660: not.b d5
pub fn i_c11660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11662;
    cpu.ir = 0x4605;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = !v1 & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C11662: move.b d5, $380041.l
pub fn i_c11662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11668;
    cpu.ir = 0x13C5;
    let v1 = cpu.d[5] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x380041, v1 as u8);
    16
}
/// $C11668: move.b #$f7, $380031.l
pub fn i_c11668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11670;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xF7;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x380031, v1 as u8);
    16
}
/// $C11670: bsr.w $c17062
pub fn i_c11670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11674;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0xC11674);
    cpu.pc = 0xC17062;
    18
}
/// $C11674: move.b #$ff, $380031.l
pub fn i_c11674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1167C;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x380031, v1 as u8);
    16
}
/// $C1167C: rts
pub fn i_c1167c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1167E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1167E: tst.b $a82(a5)
pub fn i_c1167e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11682;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xA82);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C11682: bne.b $c11686
pub fn i_c11682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11684;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC11686; return 10; }
    8
}
/// $C11684: rts
pub fn i_c11684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11686;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C11686: cmpi.b #$1, $a81(a5)
pub fn i_c11686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1168C;
    cpu.ir = 0xC2D;
    let ad2 = cpu.a[5].wrapping_add(0xA81);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $C1168C: bra.b $c116b4
pub fn i_c1168c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1168E;
    cpu.ir = 0x6026;
    cpu.pc = 0xC116B4;
    10
}
/// $C116B4: tst.b $bbb(a5)
pub fn i_c116b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC116B8;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xBBB);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C116B8: bne.b $c116c2
pub fn i_c116b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC116BA;
    cpu.ir = 0x6608;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC116C2; return 10; }
    8
}
/// $C116BA: move.b #$ff, $380031.l
pub fn i_c116ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC116C2;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x380031, v1 as u8);
    16
}
/// $C116C2: tst.b $a81(a5)
pub fn i_c116c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC116C6;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xA81);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C116C6: beq.b $c116fc
pub fn i_c116c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC116C8;
    cpu.ir = 0x6734;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC116FC; return 10; }
    8
}
/// $C116C8: btst.b #$4, $d00038.l
pub fn i_c116c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC116D0;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((4) % 8);
    let v2 = bus.read8(0xD00038) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $C116D0: bne.b $c116fc
pub fn i_c116d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC116D2;
    cpu.ir = 0x662A;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC116FC; return 10; }
    8
}
/// $C116D2: jsr $c119e2(pc)
pub fn i_c116d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC116D6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC116D6);
    cpu.pc = 0xC119E2;
    16
}
/// $C116D6: move.b $300001.l, d0
pub fn i_c116d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC116DC;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x300001) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C116DC: andi.b #$80, d0
pub fn i_c116dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC116E0;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x80) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C116E0: bne.b $c116fc
pub fn i_c116e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC116E2;
    cpu.ir = 0x661A;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC116FC; return 10; }
    8
}
/// $C116E2: move.w sr, d1
pub fn i_c116e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC116E4;
    cpu.ir = 0x40C1;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | ((cpu.sr.0 as u32) & 0xFFFF);
    6
}
/// $C116E4: move.w #$2700, sr
pub fn i_c116e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC116E8;
    cpu.ir = 0x46FC;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let v1: u32 = 0x2700;
    write_sr(cpu, v1 as u16);
    12
}
/// $C116E8: move.b d0, $300001.l
pub fn i_c116e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC116EE;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C116EE: move.b $300001.l, d0
pub fn i_c116ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC116F4;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x300001) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C116F4: andi.b #$80, d0
pub fn i_c116f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC116F8;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x80) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C116F8: beq.b $c116e8
pub fn i_c116f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC116FA;
    cpu.ir = 0x67EE;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC116E8; return 10; }
    8
}
/// $C116FA: move.w d1, sr
pub fn i_c116fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC116FC;
    cpu.ir = 0x46C1;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let v1 = cpu.d[1] & 0xFFFF;
    write_sr(cpu, v1 as u16);
    12
}
/// $C116FC: move.b d0, $3a001d.l
pub fn i_c116fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11702;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001D, v1 as u8);
    16
}
/// $C11702: lea.l $d08000.l, a4
pub fn i_c11702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11708;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0xD08000;
    20
}
/// $C11708: move.b $300081.l, d0
pub fn i_c11708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1170E;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x300081) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C1170E: not.b d0
pub fn i_c1170e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11710;
    cpu.ir = 0x4600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = !v1 & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C11710: move.b $bde(a5), d1
pub fn i_c11710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11714;
    cpu.ir = 0x122D;
    let ad2 = cpu.a[5].wrapping_add(0xBDE);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C11714: move.b d0, $bde(a5)
pub fn i_c11714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11718;
    cpu.ir = 0x1B40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xBDE);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C11718: eor.b d0, d1
pub fn i_c11718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1171A;
    cpu.ir = 0xB101;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 ^ cpu.d[0]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C1171A: and.b d0, d1
pub fn i_c1171a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1171C;
    cpu.ir = 0xC200;
    let v1 = cpu.d[0] & 0xFF;
    let r = (v1 & cpu.d[1]) & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C1171C: move.b d1, $bdf(a5)
pub fn i_c1171c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11720;
    cpu.ir = 0x1B41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xBDF);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C11720: tst.b $a81(a5)
pub fn i_c11720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11724;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xA81);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C11724: beq.b $c11732
pub fn i_c11724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11726;
    cpu.ir = 0x670C;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC11732; return 10; }
    8
}
/// $C11726: btst.b #$7, d1
pub fn i_c11726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1172A;
    cpu.ir = 0x801;
    let mk: u32 = 1u32 << ((7) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C1172A: beq.b $c11732
pub fn i_c1172a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1172C;
    cpu.ir = 0x6706;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC11732; return 10; }
    8
}
/// $C1172C: jmp $c17ae6.l
pub fn i_c1172c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11732;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC17AE6;
    8
}
/// $C11732: move.b $320001.l, d0
pub fn i_c11732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11738;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x320001) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C11738: not.b d0
pub fn i_c11738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1173A;
    cpu.ir = 0x4600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = !v1 & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C1173A: move.b -$7fe0(a4), d2
pub fn i_c1173a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1173E;
    cpu.ir = 0x142C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8020);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C1173E: move.b -$7fdf(a4), d1
pub fn i_c1173e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11742;
    cpu.ir = 0x122C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8021);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C11742: move.b d1, -$7fe0(a4)
pub fn i_c11742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11746;
    cpu.ir = 0x1941;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8020);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C11746: move.b d0, -$7fdf(a4)
pub fn i_c11746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1174A;
    cpu.ir = 0x1940;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8021);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C1174A: or.b d1, d2
pub fn i_c1174a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1174C;
    cpu.ir = 0x8401;
    let v1 = cpu.d[1] & 0xFF;
    let r = (v1 | cpu.d[2]) & 0xFF;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C1174C: not.b d2
pub fn i_c1174c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1174E;
    cpu.ir = 0x4602;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = !v1 & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C1174E: eor.b d0, d1
pub fn i_c1174e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11750;
    cpu.ir = 0xB101;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 ^ cpu.d[0]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C11750: and.b d0, d1
pub fn i_c11750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11752;
    cpu.ir = 0xC200;
    let v1 = cpu.d[0] & 0xFF;
    let r = (v1 & cpu.d[1]) & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C11752: move.b d1, -$7fde(a4)
pub fn i_c11752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11756;
    cpu.ir = 0x1941;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8022);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C11756: and.b d0, d2
pub fn i_c11756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11758;
    cpu.ir = 0xC400;
    let v1 = cpu.d[0] & 0xFF;
    let r = (v1 & cpu.d[2]) & 0xFF;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C11758: moveq #$0, d7
pub fn i_c11758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1175A;
    cpu.ir = 0x7E00;
    cpu.d[7] = 0x0;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1175A: moveq #$1, d3
pub fn i_c1175a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1175C;
    cpu.ir = 0x7601;
    cpu.d[3] = 0x1;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1175C: moveq #$0, d4
pub fn i_c1175c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1175E;
    cpu.ir = 0x7800;
    cpu.d[4] = 0x0;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1175E: move.w #$4, d5
pub fn i_c1175e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11762;
    cpu.ir = 0x3A3C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C11762: movea.l -$7eee(a4), a2
pub fn i_c11762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11766;
    cpu.ir = 0x246C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8112);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $C11766: movea.l -$7eea(a4), a1
pub fn i_c11766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1176A;
    cpu.ir = 0x226C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8116);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $C1176A: lea.l -$7fdc(a4), a6
pub fn i_c1176a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1176E;
    cpu.ir = 0x4DEC;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF8024);
    cpu.a[6] = ad1;
    16
}
/// $C1176E: lea.l -$7fc6(a4), a3
pub fn i_c1176e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11772;
    cpu.ir = 0x47EC;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF803A);
    cpu.a[3] = ad1;
    16
}
/// $C11772: jsr $c11870(pc)
pub fn i_c11772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11776;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11776);
    cpu.pc = 0xC11870;
    16
}
/// $C11776: moveq #$1, d4
pub fn i_c11776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11778;
    cpu.ir = 0x7801;
    cpu.d[4] = 0x1;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11778: moveq #$8, d5
pub fn i_c11778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1177A;
    cpu.ir = 0x7A08;
    cpu.d[5] = 0x8;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1177A: moveq #$2, d3
pub fn i_c1177a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1177C;
    cpu.ir = 0x7602;
    cpu.d[3] = 0x2;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1177C: addq.l #$2, a3
pub fn i_c1177c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1177E;
    cpu.ir = 0x548B;
    cpu.a[3] = cpu.a[3].wrapping_add(2);
    8
}
/// $C1177E: jsr $c11870(pc)
pub fn i_c1177e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11782;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11782);
    cpu.pc = 0xC11870;
    16
}
/// $C11782: jsr $c118b0(pc)
pub fn i_c11782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11786;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11786);
    cpu.pc = 0xC118B0;
    16
}
/// $C11786: tst.b d7
pub fn i_c11786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11788;
    cpu.ir = 0x4A07;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C11788: beq.b $c11794
pub fn i_c11788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1178A;
    cpu.ir = 0x670A;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC11794; return 10; }
    8
}
/// $C1178A: move.w -$7fbc(a4), $ada(a5)
pub fn i_c1178a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11790;
    cpu.ir = 0x3B6C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8044);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xADA);
    bus.write16(ad4, v3 as u16);
    20
}
/// $C11790: addq.b #$1, $be2(a5)
pub fn i_c11790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11794;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xBE2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C11794: jsr $c1199e(pc)
pub fn i_c11794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11798;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11798);
    cpu.pc = 0xC1199E;
    16
}
/// $C11798: tst.b $bbc(a5)
pub fn i_c11798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1179C;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xBBC);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C1179C: bne.w $c117f4
pub fn i_c1179c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117A0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC117F4; return 10; }
    8
}
/// $C117A0: move.b -$7fc8(a4), d0
pub fn i_c117a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117A4;
    cpu.ir = 0x102C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8038);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C117A4: beq.w $c117d0
pub fn i_c117a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117A8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC117D0; return 10; }
    8
}
/// $C117A8: lea.l $380061.l, a0
pub fn i_c117a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117AE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x380061;
    20
}
/// $C117AE: moveq #$0, d1
pub fn i_c117ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117B0;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C117B0: move.b -$7fc7(a4), d1
pub fn i_c117b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117B4;
    cpu.ir = 0x122C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8039);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C117B4: add.w d1, d1
pub fn i_c117b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117B6;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C117B6: subq.b #$1, d0
pub fn i_c117b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117B8;
    cpu.ir = 0x5300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C117B8: move.b d0, -$7fc8(a4)
pub fn i_c117b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117BC;
    cpu.ir = 0x1940;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8038);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C117BC: andi.b #$10, d0
pub fn i_c117bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117C0;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x10) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C117C0: beq.b $c117c8
pub fn i_c117c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117C2;
    cpu.ir = 0x6706;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC117C8; return 10; }
    8
}
/// $C117C2: lea.l $3800e1.l, a0
pub fn i_c117c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117C8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x3800E1;
    20
}
/// $C117C8: move.b d0, (a0, d1.w)
pub fn i_c117c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117CC;
    cpu.ir = 0x1180;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $C117CC: bra.w $c117f4
pub fn i_c117cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117D0;
    cpu.ir = 0x6000;
    cpu.pc = 0xC117F4;
    10
}
/// $C117D0: moveq #$0, d7
pub fn i_c117d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117D2;
    cpu.ir = 0x7E00;
    cpu.d[7] = 0x0;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C117D2: lea.l -$7fd4(a4), a6
pub fn i_c117d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117D6;
    cpu.ir = 0x4DEC;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF802C);
    cpu.a[6] = ad1;
    16
}
/// $C117D6: tst.b (a6, d7.w)
pub fn i_c117d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117DA;
    cpu.ir = 0x4A36;
    let ad2 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    14
}
/// $C117DA: beq.b $c117ec
pub fn i_c117da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117DC;
    cpu.ir = 0x6710;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC117EC; return 10; }
    8
}
/// $C117DC: subq.b #$1, (a6, d7.w)
pub fn i_c117dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117E0;
    cpu.ir = 0x5336;
    let ad1 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    14
}
/// $C117E0: move.b d7, -$7fc7(a4)
pub fn i_c117e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117E4;
    cpu.ir = 0x1947;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8039);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C117E4: move.b #$1f, -$7fc8(a4)
pub fn i_c117e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117EA;
    cpu.ir = 0x197C;
    let v1: u32 = 0x1F;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8038);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C117EA: bra.b $c117f4
pub fn i_c117ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117EC;
    cpu.ir = 0x6008;
    cpu.pc = 0xC117F4;
    10
}
/// $C117EC: addq.b #$1, d7
pub fn i_c117ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117EE;
    cpu.ir = 0x5207;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C117EE: cmpi.b #$2, d7
pub fn i_c117ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117F2;
    cpu.ir = 0xC07;
    let v1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $C117F2: bne.b $c117d6
pub fn i_c117f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117F4;
    cpu.ir = 0x66E2;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC117D6; return 10; }
    8
}
/// $C117F4: jsr $c1182a(pc)
pub fn i_c117f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117F8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC117F8);
    cpu.pc = 0xC1182A;
    16
}
/// $C117F8: tst.b $bbb(a5)
pub fn i_c117f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117FC;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xBBB);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C117FC: bne.b $c11828
pub fn i_c117fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC117FE;
    cpu.ir = 0x662A;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC11828; return 10; }
    8
}
/// $C117FE: move.b -$7fcc(a4), d0
pub fn i_c117fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11802;
    cpu.ir = 0x102C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8034);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C11802: move.b #$ef, d1
pub fn i_c11802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11806;
    cpu.ir = 0x123C;
    let v1: u32 = 0xEF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C11806: btst.b #$0, $b8b(a5)
pub fn i_c11806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1180C;
    cpu.ir = 0x82D;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[5].wrapping_add(0xB8B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $C1180C: beq.b $c11816
pub fn i_c1180c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1180E;
    cpu.ir = 0x6708;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC11816; return 10; }
    8
}
/// $C1180E: move.b -$7fcb(a4), d0
pub fn i_c1180e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11812;
    cpu.ir = 0x102C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8035);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C11812: move.b #$df, d1
pub fn i_c11812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11816;
    cpu.ir = 0x123C;
    let v1: u32 = 0xDF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C11816: not.b d0
pub fn i_c11816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11818;
    cpu.ir = 0x4600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = !v1 & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C11818: move.b d0, $380041.l
pub fn i_c11818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1181E;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x380041, v1 as u8);
    16
}
/// $C1181E: jsr $c11190(pc)
pub fn i_c1181e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11822;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11822);
    cpu.pc = 0xC11190;
    16
}
/// $C11822: move.b d1, $380031.l
pub fn i_c11822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11828;
    cpu.ir = 0x13C1;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x380031, v1 as u8);
    16
}
/// $C11828: rts
pub fn i_c11828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1182A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1182A: subq.b #$1, -$7fa3(a4)
pub fn i_c1182a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1182E;
    cpu.ir = 0x532C;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF805D);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1182E: bne.w $c1186e
pub fn i_c1182e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11832;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1186E; return 10; }
    8
}
/// $C11832: move.b #$3c, -$7fa3(a4)
pub fn i_c11832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11838;
    cpu.ir = 0x197C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF805D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C11838: addq.b #$1, -$7fa4(a4)
pub fn i_c11838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1183C;
    cpu.ir = 0x522C;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF805C);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1183C: cmpi.b #$3c, -$7fa4(a4)
pub fn i_c1183c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11842;
    cpu.ir = 0xC2C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF805C);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3C, Size::Byte, false);
    16
}
/// $C11842: bcs.w $c1186e
pub fn i_c11842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11846;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC1186E; return 10; }
    8
}
/// $C11846: clr.b -$7fa4(a4)
pub fn i_c11846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1184A;
    cpu.ir = 0x422C;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF805C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1184A: addq.b #$1, -$7fa5(a4)
pub fn i_c1184a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1184E;
    cpu.ir = 0x522C;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF805B);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1184E: cmpi.b #$3c, -$7fa5(a4)
pub fn i_c1184e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11854;
    cpu.ir = 0xC2C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF805B);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3C, Size::Byte, false);
    16
}
/// $C11854: bcs.w $c1186e
pub fn i_c11854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11858;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC1186E; return 10; }
    8
}
/// $C11858: clr.b -$7fa5(a4)
pub fn i_c11858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1185C;
    cpu.ir = 0x422C;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF805B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1185C: addq.b #$4, -$7fa6(a4)
pub fn i_c1185c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11860;
    cpu.ir = 0x582C;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF805A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 4, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C11860: cmpi.b #$60, -$7fa6(a4)
pub fn i_c11860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11866;
    cpu.ir = 0xC2C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF805A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x60, Size::Byte, false);
    16
}
/// $C11866: bcs.w $c1186e
pub fn i_c11866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1186A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC1186E; return 10; }
    8
}
/// $C1186A: clr.b -$7fa6(a4)
pub fn i_c1186a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1186E;
    cpu.ir = 0x422C;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF805A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1186E: rts
pub fn i_c1186e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11870;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C11870: and.b d2, d3
pub fn i_c11870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11872;
    cpu.ir = 0xC602;
    let v1 = cpu.d[2] & 0xFF;
    let r = (v1 & cpu.d[3]) & 0xFF;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C11872: beq.b $c118ae
pub fn i_c11872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11874;
    cpu.ir = 0x673A;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC118AE; return 10; }
    8
}
/// $C11874: moveq #$1, d7
pub fn i_c11874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11876;
    cpu.ir = 0x7E01;
    cpu.d[7] = 0x1;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11876: add.b d7, (a6, d4.w)
pub fn i_c11876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1187A;
    cpu.ir = 0xDF36;
    let ad1 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[7] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    14
}
/// $C1187A: add.b d7, $8(a6, d4.w)
pub fn i_c1187a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1187E;
    cpu.ir = 0xDF36;
    let ad1 = cpu.a[6].wrapping_add(0x8).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[7] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    14
}
/// $C1187E: tst.b $bbc(a5)
pub fn i_c1187e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11882;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xBBC);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C11882: bne.b $c1188c
pub fn i_c11882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11884;
    cpu.ir = 0x6608;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1188C; return 10; }
    8
}
/// $C11884: add.l d7, (a2, d5.w)
pub fn i_c11884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11888;
    cpu.ir = 0xDFB2;
    let ad1 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    let v2 = bus.read32(ad1);
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Long, true);
    bus.write32(ad1, r3);
    18
}
/// $C11888: add.l d7, (a1, d5.w)
pub fn i_c11888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1188C;
    cpu.ir = 0xDFB1;
    let ad1 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    let v2 = bus.read32(ad1);
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Long, true);
    bus.write32(ad1, r3);
    18
}
/// $C1188C: move.b (a6, d4.w), d0
pub fn i_c1188c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11890;
    cpu.ir = 0x1036;
    let ad2 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $C11890: sub.b (a3), d0
pub fn i_c11890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11892;
    cpu.ir = 0x9013;
    let v2 = bus.read8(cpu.a[3]) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r = sub_with_flags(cpu, h1, v2, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    8
}
/// $C11892: bcs.b $c118ae
pub fn i_c11892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11894;
    cpu.ir = 0x651A;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC118AE; return 10; }
    8
}
/// $C11894: move.b d0, (a6, d4.w)
pub fn i_c11894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11898;
    cpu.ir = 0x1D80;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $C11898: sub.b d0, d0
pub fn i_c11898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1189A;
    cpu.ir = 0x9000;
    let v1 = cpu.d[0] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    4
}
/// $C1189A: move.b $1(a3), d0
pub fn i_c1189a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1189E;
    cpu.ir = 0x102B;
    let ad2 = cpu.a[3].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C1189E: move.b -$7fcc(a4), d1
pub fn i_c1189e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118A2;
    cpu.ir = 0x122C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8034);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C118A2: abcd.b d1, d0
pub fn i_c118a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118A4;
    cpu.ir = 0xC101;
    let s = cpu.d[1] as u8; let d = cpu.d[0] as u8;
    let x = cpu.sr.get(StatusRegister::X) as u8;
    let (r, carry, vf) = abcd_calc(d, s, x);
    if r != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::N, r & 0x80 != 0); cpu.sr.set(StatusRegister::V, vf);
    cpu.sr.set(StatusRegister::C, carry); cpu.sr.set(StatusRegister::X, carry);
    cpu.d[0] = (cpu.d[0] & !0xFF) | r as u32;
    6
}
/// $C118A4: bcc.b $c118aa
pub fn i_c118a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118A6;
    cpu.ir = 0x6404;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC118AA; return 10; }
    8
}
/// $C118A6: move.b #$99, d0
pub fn i_c118a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118AA;
    cpu.ir = 0x103C;
    let v1: u32 = 0x99;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C118AA: move.b d0, -$7fcc(a4)
pub fn i_c118aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118AE;
    cpu.ir = 0x1940;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8034);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C118AE: rts
pub fn i_c118ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118B0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C118B0: btst.b #$2, -$7fde(a4)
pub fn i_c118b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118B6;
    cpu.ir = 0x82C;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8022);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $C118B6: beq.b $c118da
pub fn i_c118b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118B8;
    cpu.ir = 0x6722;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC118DA; return 10; }
    8
}
/// $C118B8: moveq #$1, d7
pub fn i_c118b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118BA;
    cpu.ir = 0x7E01;
    cpu.d[7] = 0x1;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C118BA: tst.b $bbc(a5)
pub fn i_c118ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118BE;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xBBC);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C118BE: bne.b $c118c8
pub fn i_c118be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118C0;
    cpu.ir = 0x6608;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC118C8; return 10; }
    8
}
/// $C118C0: add.l d7, $c(a2)
pub fn i_c118c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118C4;
    cpu.ir = 0xDFAA;
    let ad1 = cpu.a[2].wrapping_add(0xC);
    let v2 = bus.read32(ad1);
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Long, true);
    bus.write32(ad1, r3);
    16
}
/// $C118C4: add.l d7, $c(a1)
pub fn i_c118c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118C8;
    cpu.ir = 0xDFA9;
    let ad1 = cpu.a[1].wrapping_add(0xC);
    let v2 = bus.read32(ad1);
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Long, true);
    bus.write32(ad1, r3);
    16
}
/// $C118C8: sub.b d0, d0
pub fn i_c118c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118CA;
    cpu.ir = 0x9000;
    let v1 = cpu.d[0] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    4
}
/// $C118CA: move.b -$7fcc(a4), d0
pub fn i_c118ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118CE;
    cpu.ir = 0x102C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8034);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C118CE: abcd.b d7, d0
pub fn i_c118ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118D0;
    cpu.ir = 0xC107;
    let s = cpu.d[7] as u8; let d = cpu.d[0] as u8;
    let x = cpu.sr.get(StatusRegister::X) as u8;
    let (r, carry, vf) = abcd_calc(d, s, x);
    if r != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::N, r & 0x80 != 0); cpu.sr.set(StatusRegister::V, vf);
    cpu.sr.set(StatusRegister::C, carry); cpu.sr.set(StatusRegister::X, carry);
    cpu.d[0] = (cpu.d[0] & !0xFF) | r as u32;
    6
}
/// $C118D0: bcc.b $c118d6
pub fn i_c118d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118D2;
    cpu.ir = 0x6404;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC118D6; return 10; }
    8
}
/// $C118D2: move.b #$99, d0
pub fn i_c118d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118D6;
    cpu.ir = 0x103C;
    let v1: u32 = 0x99;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C118D6: move.b d0, -$7fcc(a4)
pub fn i_c118d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118DA;
    cpu.ir = 0x1940;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8034);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C118DA: rts
pub fn i_c118da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118DC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C118DC: move.b d0, $3a001d.l
pub fn i_c118dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118E2;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001D, v1 as u8);
    16
}
/// $C118E2: lea.l $d08000.l, a4
pub fn i_c118e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118E8;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0xD08000;
    20
}
/// $C118E8: lea.l $aba(a5), a1
pub fn i_c118e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118EC;
    cpu.ir = 0x43ED;
    let ad1 = cpu.a[5].wrapping_add(0xABA);
    cpu.a[1] = ad1;
    16
}
/// $C118EC: move.b $ab4(a5), d0
pub fn i_c118ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118F0;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAB4);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C118F0: jsr $c118fe(pc)
pub fn i_c118f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118F4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC118F4);
    cpu.pc = 0xC118FE;
    16
}
/// $C118F4: jsr $c118fe(pc)
pub fn i_c118f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118F8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC118F8);
    cpu.pc = 0xC118FE;
    16
}
/// $C118F8: jsr $c11190(pc)
pub fn i_c118f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118FC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC118FC);
    cpu.pc = 0xC11190;
    16
}
/// $C118FC: rts
pub fn i_c118fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC118FE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C118FE: move.b (a1)+, d2
pub fn i_c118fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11900;
    cpu.ir = 0x1419;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C11900: ror.b #$1, d0
pub fn i_c11900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11902;
    cpu.ir = 0xE218;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Byte, 3, false);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    8
}
/// $C11902: bcc.b $c11960
pub fn i_c11902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11904;
    cpu.ir = 0x645C;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC11960; return 10; }
    8
}
/// $C11904: moveq #$1, d4
pub fn i_c11904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11906;
    cpu.ir = 0x7801;
    cpu.d[4] = 0x1;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C11906: move.b $300001.l, d5
pub fn i_c11906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1190C;
    cpu.ir = 0x1A39;
    let v2 = bus.read8(0x300001) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C1190C: andi.b #$40, d5
pub fn i_c1190c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11910;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x40) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C11910: beq.b $c11922
pub fn i_c11910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11912;
    cpu.ir = 0x6710;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC11922; return 10; }
    8
}
/// $C11912: move.b -$7fcc(a4), d3
pub fn i_c11912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11916;
    cpu.ir = 0x162C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8034);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C11916: sub.b d5, d5
pub fn i_c11916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11918;
    cpu.ir = 0x9A05;
    let v1 = cpu.d[5] & 0xFF;
    let h1 = cpu.d[5] & 0xFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r & 0xFF);
    4
}
/// $C11918: sbcd.b d4, d3
pub fn i_c11918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1191A;
    cpu.ir = 0x8704;
    let s = cpu.d[4] as u8; let d = cpu.d[3] as u8;
    let x = cpu.sr.get(StatusRegister::X) as u8;
    let (r, carry, vf) = sbcd_calc(d, s, x);
    if r != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::N, r & 0x80 != 0); cpu.sr.set(StatusRegister::V, vf);
    cpu.sr.set(StatusRegister::C, carry); cpu.sr.set(StatusRegister::X, carry);
    cpu.d[3] = (cpu.d[3] & !0xFF) | r as u32;
    6
}
/// $C1191A: bcc.b $c1191e
pub fn i_c1191a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1191C;
    cpu.ir = 0x6402;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC1191E; return 10; }
    8
}
/// $C1191C: moveq #$0, d3
pub fn i_c1191c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1191E;
    cpu.ir = 0x7600;
    cpu.d[3] = 0x0;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1191E: move.b d3, -$7fcc(a4)
pub fn i_c1191e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11922;
    cpu.ir = 0x1943;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8034);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C11922: movea.l -$7ef6(a4), a2
pub fn i_c11922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11926;
    cpu.ir = 0x246C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF810A);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $C11926: jsr $c11990(pc)
pub fn i_c11926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1192A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1192A);
    cpu.pc = 0xC11990;
    16
}
/// $C1192A: movea.l -$7ef2(a4), a2
pub fn i_c1192a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1192E;
    cpu.ir = 0x246C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF810E);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $C1192E: jsr $c11990(pc)
pub fn i_c1192e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11932;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11932);
    cpu.pc = 0xC11990;
    16
}
/// $C11932: movea.l -$7e5c(a4), a2
pub fn i_c11932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11936;
    cpu.ir = 0x246C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF81A4);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $C11936: jsr $c11990(pc)
pub fn i_c11936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1193A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1193A);
    cpu.pc = 0xC11990;
    16
}
/// $C1193A: movea.l -$7eee(a4), a2
pub fn i_c1193a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1193E;
    cpu.ir = 0x246C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8112);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $C1193E: lea.l $240(a2), a2
pub fn i_c1193e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11942;
    cpu.ir = 0x45EA;
    let ad1 = cpu.a[2].wrapping_add(0x240);
    cpu.a[2] = ad1;
    16
}
/// $C11942: jsr $c11990(pc)
pub fn i_c11942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC11946;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC11946);
    cpu.pc = 0xC11990;
    16
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0xC00402, i_c00402),
    (0xC00408, i_c00408),
    (0xC0040E, i_c0040e),
    (0xC00414, i_c00414),
    (0xC0041A, i_c0041a),
    (0xC00420, i_c00420),
    (0xC00426, i_c00426),
    (0xC0042C, i_c0042c),
    (0xC00432, i_c00432),
    (0xC00438, i_c00438),
    (0xC0043E, i_c0043e),
    (0xC00444, i_c00444),
    (0xC0044A, i_c0044a),
    (0xC00450, i_c00450),
    (0xC00468, i_c00468),
    (0xC0046E, i_c0046e),
    (0xC004C2, i_c004c2),
    (0xC004CE, i_c004ce),
    (0xC10002, i_c10002),
    (0xC10006, i_c10006),
    (0xC1000C, i_c1000c),
    (0xC10014, i_c10014),
    (0xC1001C, i_c1001c),
    (0xC10022, i_c10022),
    (0xC10028, i_c10028),
    (0xC1002C, i_c1002c),
    (0xC10034, i_c10034),
    (0xC10038, i_c10038),
    (0xC1003A, i_c1003a),
    (0xC1003C, i_c1003c),
    (0xC1003E, i_c1003e),
    (0xC10040, i_c10040),
    (0xC10046, i_c10046),
    (0xC1004A, i_c1004a),
    (0xC10050, i_c10050),
    (0xC10054, i_c10054),
    (0xC10058, i_c10058),
    (0xC1005C, i_c1005c),
    (0xC10060, i_c10060),
    (0xC10064, i_c10064),
    (0xC10068, i_c10068),
    (0xC1006C, i_c1006c),
    (0xC10070, i_c10070),
    (0xC10074, i_c10074),
    (0xC1007A, i_c1007a),
    (0xC10080, i_c10080),
    (0xC10084, i_c10084),
    (0xC10088, i_c10088),
    (0xC1008C, i_c1008c),
    (0xC10090, i_c10090),
    (0xC10094, i_c10094),
    (0xC10098, i_c10098),
    (0xC1009E, i_c1009e),
    (0xC100A4, i_c100a4),
    (0xC100AA, i_c100aa),
    (0xC100AE, i_c100ae),
    (0xC100B2, i_c100b2),
    (0xC100B6, i_c100b6),
    (0xC100BA, i_c100ba),
    (0xC100BE, i_c100be),
    (0xC100C2, i_c100c2),
    (0xC100C8, i_c100c8),
    (0xC100CC, i_c100cc),
    (0xC100D2, i_c100d2),
    (0xC100D8, i_c100d8),
    (0xC100DA, i_c100da),
    (0xC100DE, i_c100de),
    (0xC100E2, i_c100e2),
    (0xC100E8, i_c100e8),
    (0xC100EA, i_c100ea),
    (0xC100EC, i_c100ec),
    (0xC100EE, i_c100ee),
    (0xC100F0, i_c100f0),
    (0xC100F2, i_c100f2),
    (0xC100F4, i_c100f4),
    (0xC100F6, i_c100f6),
    (0xC100F8, i_c100f8),
    (0xC100FA, i_c100fa),
    (0xC100FC, i_c100fc),
    (0xC100FE, i_c100fe),
    (0xC10100, i_c10100),
    (0xC10102, i_c10102),
    (0xC10104, i_c10104),
    (0xC10106, i_c10106),
    (0xC10108, i_c10108),
    (0xC1010A, i_c1010a),
    (0xC1010C, i_c1010c),
    (0xC1010E, i_c1010e),
    (0xC10110, i_c10110),
    (0xC10112, i_c10112),
    (0xC10114, i_c10114),
    (0xC10116, i_c10116),
    (0xC10118, i_c10118),
    (0xC1011A, i_c1011a),
    (0xC1011C, i_c1011c),
    (0xC1011E, i_c1011e),
    (0xC10120, i_c10120),
    (0xC10122, i_c10122),
    (0xC10124, i_c10124),
    (0xC10126, i_c10126),
    (0xC10128, i_c10128),
    (0xC1012C, i_c1012c),
    (0xC10132, i_c10132),
    (0xC10136, i_c10136),
    (0xC1013E, i_c1013e),
    (0xC10142, i_c10142),
    (0xC10148, i_c10148),
    (0xC1014E, i_c1014e),
    (0xC10154, i_c10154),
    (0xC1015A, i_c1015a),
    (0xC1015C, i_c1015c),
    (0xC10160, i_c10160),
    (0xC10162, i_c10162),
    (0xC10166, i_c10166),
    (0xC10174, i_c10174),
    (0xC1017A, i_c1017a),
    (0xC1017E, i_c1017e),
    (0xC10182, i_c10182),
    (0xC1018A, i_c1018a),
    (0xC1018E, i_c1018e),
    (0xC10192, i_c10192),
    (0xC10196, i_c10196),
    (0xC1019A, i_c1019a),
    (0xC1019C, i_c1019c),
    (0xC101A0, i_c101a0),
    (0xC101A6, i_c101a6),
    (0xC101AA, i_c101aa),
    (0xC101AE, i_c101ae),
    (0xC101B2, i_c101b2),
    (0xC101B4, i_c101b4),
    (0xC101B6, i_c101b6),
    (0xC101B8, i_c101b8),
    (0xC101BC, i_c101bc),
    (0xC101BE, i_c101be),
    (0xC101C2, i_c101c2),
    (0xC101C8, i_c101c8),
    (0xC101CC, i_c101cc),
    (0xC101D2, i_c101d2),
    (0xC101D8, i_c101d8),
    (0xC101EA, i_c101ea),
    (0xC101F0, i_c101f0),
    (0xC101F4, i_c101f4),
    (0xC101FC, i_c101fc),
    (0xC10202, i_c10202),
    (0xC10208, i_c10208),
    (0xC10212, i_c10212),
    (0xC10218, i_c10218),
    (0xC1021C, i_c1021c),
    (0xC10222, i_c10222),
    (0xC10224, i_c10224),
    (0xC10228, i_c10228),
    (0xC1022C, i_c1022c),
    (0xC10230, i_c10230),
    (0xC10236, i_c10236),
    (0xC1023A, i_c1023a),
    (0xC1023C, i_c1023c),
    (0xC10240, i_c10240),
    (0xC10242, i_c10242),
    (0xC10246, i_c10246),
    (0xC1024A, i_c1024a),
    (0xC1024E, i_c1024e),
    (0xC10252, i_c10252),
    (0xC10258, i_c10258),
    (0xC1025A, i_c1025a),
    (0xC1025E, i_c1025e),
    (0xC10260, i_c10260),
    (0xC10266, i_c10266),
    (0xC1026C, i_c1026c),
    (0xC10270, i_c10270),
    (0xC10272, i_c10272),
    (0xC10278, i_c10278),
    (0xC1027A, i_c1027a),
    (0xC10280, i_c10280),
    (0xC10282, i_c10282),
    (0xC10286, i_c10286),
    (0xC1028A, i_c1028a),
    (0xC1028E, i_c1028e),
    (0xC10296, i_c10296),
    (0xC1029A, i_c1029a),
    (0xC102A2, i_c102a2),
    (0xC102A6, i_c102a6),
    (0xC102AA, i_c102aa),
    (0xC102B2, i_c102b2),
    (0xC102B6, i_c102b6),
    (0xC102BA, i_c102ba),
    (0xC102BE, i_c102be),
    (0xC102C2, i_c102c2),
    (0xC102C8, i_c102c8),
    (0xC102CE, i_c102ce),
    (0xC102D2, i_c102d2),
    (0xC102DA, i_c102da),
    (0xC102E2, i_c102e2),
    (0xC102EA, i_c102ea),
    (0xC102F0, i_c102f0),
    (0xC102F8, i_c102f8),
    (0xC102FC, i_c102fc),
    (0xC10300, i_c10300),
    (0xC10306, i_c10306),
    (0xC1030C, i_c1030c),
    (0xC1030E, i_c1030e),
    (0xC10310, i_c10310),
    (0xC10312, i_c10312),
    (0xC10314, i_c10314),
    (0xC10316, i_c10316),
    (0xC10318, i_c10318),
    (0xC1031A, i_c1031a),
    (0xC1031C, i_c1031c),
    (0xC10320, i_c10320),
    (0xC10326, i_c10326),
    (0xC1032A, i_c1032a),
    (0xC1032E, i_c1032e),
    (0xC10332, i_c10332),
    (0xC10336, i_c10336),
    (0xC1033A, i_c1033a),
    (0xC1033E, i_c1033e),
    (0xC10342, i_c10342),
    (0xC10346, i_c10346),
    (0xC1034A, i_c1034a),
    (0xC1034E, i_c1034e),
    (0xC10354, i_c10354),
    (0xC10358, i_c10358),
    (0xC10360, i_c10360),
    (0xC10364, i_c10364),
    (0xC10368, i_c10368),
    (0xC1036C, i_c1036c),
    (0xC1036E, i_c1036e),
    (0xC10372, i_c10372),
    (0xC10374, i_c10374),
    (0xC10378, i_c10378),
    (0xC10396, i_c10396),
    (0xC1039C, i_c1039c),
    (0xC103A2, i_c103a2),
    (0xC103A6, i_c103a6),
    (0xC103A8, i_c103a8),
    (0xC103AC, i_c103ac),
    (0xC103B0, i_c103b0),
    (0xC103B6, i_c103b6),
    (0xC103BC, i_c103bc),
    (0xC103BE, i_c103be),
    (0xC103C4, i_c103c4),
    (0xC103CA, i_c103ca),
    (0xC103CE, i_c103ce),
    (0xC103D0, i_c103d0),
    (0xC103D2, i_c103d2),
    (0xC103D8, i_c103d8),
    (0xC103DC, i_c103dc),
    (0xC103DE, i_c103de),
    (0xC103E4, i_c103e4),
    (0xC103E8, i_c103e8),
    (0xC103EC, i_c103ec),
    (0xC103F2, i_c103f2),
    (0xC103F6, i_c103f6),
    (0xC103FC, i_c103fc),
    (0xC103FE, i_c103fe),
    (0xC10404, i_c10404),
    (0xC10408, i_c10408),
    (0xC10410, i_c10410),
    (0xC10416, i_c10416),
    (0xC10418, i_c10418),
    (0xC1041E, i_c1041e),
    (0xC10422, i_c10422),
    (0xC10428, i_c10428),
    (0xC1042C, i_c1042c),
    (0xC10430, i_c10430),
    (0xC10432, i_c10432),
    (0xC10438, i_c10438),
    (0xC1043E, i_c1043e),
    (0xC10442, i_c10442),
    (0xC10448, i_c10448),
    (0xC1044C, i_c1044c),
    (0xC10450, i_c10450),
    (0xC10452, i_c10452),
    (0xC10456, i_c10456),
    (0xC1045A, i_c1045a),
    (0xC10460, i_c10460),
    (0xC10466, i_c10466),
    (0xC1046A, i_c1046a),
    (0xC1046E, i_c1046e),
    (0xC10474, i_c10474),
    (0xC10476, i_c10476),
    (0xC1047C, i_c1047c),
    (0xC1047E, i_c1047e),
    (0xC10486, i_c10486),
    (0xC1048C, i_c1048c),
    (0xC10490, i_c10490),
    (0xC10494, i_c10494),
    (0xC10498, i_c10498),
    (0xC104FC, i_c104fc),
    (0xC10502, i_c10502),
    (0xC10504, i_c10504),
    (0xC10508, i_c10508),
    (0xC1050C, i_c1050c),
    (0xC10512, i_c10512),
    (0xC10516, i_c10516),
    (0xC1051A, i_c1051a),
    (0xC10520, i_c10520),
    (0xC10526, i_c10526),
    (0xC1052E, i_c1052e),
    (0xC10534, i_c10534),
    (0xC10538, i_c10538),
    (0xC1053C, i_c1053c),
    (0xC10540, i_c10540),
    (0xC10544, i_c10544),
    (0xC10548, i_c10548),
    (0xC1054A, i_c1054a),
    (0xC1054E, i_c1054e),
    (0xC10550, i_c10550),
    (0xC10554, i_c10554),
    (0xC10558, i_c10558),
    (0xC1055E, i_c1055e),
    (0xC10562, i_c10562),
    (0xC10568, i_c10568),
    (0xC1056C, i_c1056c),
    (0xC10570, i_c10570),
    (0xC10574, i_c10574),
    (0xC10576, i_c10576),
    (0xC1057A, i_c1057a),
    (0xC1057E, i_c1057e),
    (0xC10580, i_c10580),
    (0xC10584, i_c10584),
    (0xC10588, i_c10588),
    (0xC1058A, i_c1058a),
    (0xC1058E, i_c1058e),
    (0xC10594, i_c10594),
    (0xC10596, i_c10596),
    (0xC1059E, i_c1059e),
    (0xC105A2, i_c105a2),
    (0xC105A6, i_c105a6),
    (0xC105AC, i_c105ac),
    (0xC105B0, i_c105b0),
    (0xC105B4, i_c105b4),
    (0xC105BA, i_c105ba),
    (0xC105C0, i_c105c0),
    (0xC105C8, i_c105c8),
    (0xC105CC, i_c105cc),
    (0xC105D0, i_c105d0),
    (0xC10612, i_c10612),
    (0xC10616, i_c10616),
    (0xC1061A, i_c1061a),
    (0xC10620, i_c10620),
    (0xC10622, i_c10622),
    (0xC10624, i_c10624),
    (0xC10626, i_c10626),
    (0xC10628, i_c10628),
    (0xC1062A, i_c1062a),
    (0xC1062C, i_c1062c),
    (0xC10630, i_c10630),
    (0xC10632, i_c10632),
    (0xC10634, i_c10634),
    (0xC1063A, i_c1063a),
    (0xC1063C, i_c1063c),
    (0xC10640, i_c10640),
    (0xC10644, i_c10644),
    (0xC10646, i_c10646),
    (0xC1064E, i_c1064e),
    (0xC10654, i_c10654),
    (0xC1065A, i_c1065a),
    (0xC1065C, i_c1065c),
    (0xC10660, i_c10660),
    (0xC10664, i_c10664),
    (0xC10668, i_c10668),
    (0xC1066E, i_c1066e),
    (0xC10672, i_c10672),
    (0xC10678, i_c10678),
    (0xC1067A, i_c1067a),
    (0xC10680, i_c10680),
    (0xC10684, i_c10684),
    (0xC10688, i_c10688),
    (0xC1068C, i_c1068c),
    (0xC10692, i_c10692),
    (0xC10696, i_c10696),
    (0xC1069A, i_c1069a),
    (0xC1069C, i_c1069c),
    (0xC1069E, i_c1069e),
    (0xC106A2, i_c106a2),
    (0xC106A4, i_c106a4),
    (0xC106A8, i_c106a8),
    (0xC106AA, i_c106aa),
    (0xC106AE, i_c106ae),
    (0xC106B2, i_c106b2),
    (0xC106B6, i_c106b6),
    (0xC106B8, i_c106b8),
    (0xC106BE, i_c106be),
    (0xC106C0, i_c106c0),
    (0xC106C4, i_c106c4),
    (0xC106C8, i_c106c8),
    (0xC106CC, i_c106cc),
    (0xC106D0, i_c106d0),
    (0xC106D2, i_c106d2),
    (0xC106D6, i_c106d6),
    (0xC106D8, i_c106d8),
    (0xC106DA, i_c106da),
    (0xC106DC, i_c106dc),
    (0xC106DE, i_c106de),
    (0xC106E2, i_c106e2),
    (0xC106E4, i_c106e4),
    (0xC106E6, i_c106e6),
    (0xC106EC, i_c106ec),
    (0xC106F0, i_c106f0),
    (0xC106F6, i_c106f6),
    (0xC106FA, i_c106fa),
    (0xC10700, i_c10700),
    (0xC10704, i_c10704),
    (0xC10708, i_c10708),
    (0xC1070C, i_c1070c),
    (0xC10710, i_c10710),
    (0xC10714, i_c10714),
    (0xC1071A, i_c1071a),
    (0xC1071C, i_c1071c),
    (0xC1071E, i_c1071e),
    (0xC10720, i_c10720),
    (0xC10724, i_c10724),
    (0xC10728, i_c10728),
    (0xC1072E, i_c1072e),
    (0xC10730, i_c10730),
    (0xC10736, i_c10736),
    (0xC10738, i_c10738),
    (0xC1073A, i_c1073a),
    (0xC1073E, i_c1073e),
    (0xC10742, i_c10742),
    (0xC10746, i_c10746),
    (0xC1074A, i_c1074a),
    (0xC1074E, i_c1074e),
    (0xC10752, i_c10752),
    (0xC10756, i_c10756),
    (0xC1075A, i_c1075a),
    (0xC1075E, i_c1075e),
    (0xC10760, i_c10760),
    (0xC10766, i_c10766),
    (0xC1076A, i_c1076a),
    (0xC1076E, i_c1076e),
    (0xC10774, i_c10774),
    (0xC10778, i_c10778),
    (0xC1077A, i_c1077a),
    (0xC1077E, i_c1077e),
    (0xC10782, i_c10782),
    (0xC10786, i_c10786),
    (0xC1078A, i_c1078a),
    (0xC1078E, i_c1078e),
    (0xC10790, i_c10790),
    (0xC10796, i_c10796),
    (0xC10798, i_c10798),
    (0xC1079A, i_c1079a),
    (0xC107A0, i_c107a0),
    (0xC107A4, i_c107a4),
    (0xC107A8, i_c107a8),
    (0xC107AC, i_c107ac),
    (0xC107B2, i_c107b2),
    (0xC107B4, i_c107b4),
    (0xC107B6, i_c107b6),
    (0xC107B8, i_c107b8),
    (0xC107BC, i_c107bc),
    (0xC107C0, i_c107c0),
    (0xC107CE, i_c107ce),
    (0xC107D4, i_c107d4),
    (0xC107D8, i_c107d8),
    (0xC107DC, i_c107dc),
    (0xC107E0, i_c107e0),
    (0xC107E6, i_c107e6),
    (0xC107EA, i_c107ea),
    (0xC107EC, i_c107ec),
    (0xC107F2, i_c107f2),
    (0xC107F6, i_c107f6),
    (0xC107FC, i_c107fc),
    (0xC107FE, i_c107fe),
    (0xC10800, i_c10800),
    (0xC10802, i_c10802),
    (0xC10806, i_c10806),
    (0xC10808, i_c10808),
    (0xC1080A, i_c1080a),
    (0xC1080C, i_c1080c),
    (0xC1080E, i_c1080e),
    (0xC10810, i_c10810),
    (0xC10812, i_c10812),
    (0xC10814, i_c10814),
    (0xC10816, i_c10816),
    (0xC10818, i_c10818),
    (0xC1081A, i_c1081a),
    (0xC1081C, i_c1081c),
    (0xC1081E, i_c1081e),
    (0xC10836, i_c10836),
    (0xC1083A, i_c1083a),
    (0xC1083C, i_c1083c),
    (0xC10840, i_c10840),
    (0xC10842, i_c10842),
    (0xC1084A, i_c1084a),
    (0xC1084E, i_c1084e),
    (0xC10852, i_c10852),
    (0xC10856, i_c10856),
    (0xC1085A, i_c1085a),
    (0xC1085E, i_c1085e),
    (0xC10862, i_c10862),
    (0xC10866, i_c10866),
    (0xC1086C, i_c1086c),
    (0xC1086E, i_c1086e),
    (0xC10872, i_c10872),
    (0xC10874, i_c10874),
    (0xC1087A, i_c1087a),
    (0xC1087C, i_c1087c),
    (0xC1087E, i_c1087e),
    (0xC10886, i_c10886),
    (0xC10888, i_c10888),
    (0xC1088E, i_c1088e),
    (0xC10890, i_c10890),
    (0xC10896, i_c10896),
    (0xC10898, i_c10898),
    (0xC1089E, i_c1089e),
    (0xC108A2, i_c108a2),
    (0xC108A6, i_c108a6),
    (0xC108AA, i_c108aa),
    (0xC108AE, i_c108ae),
    (0xC108B0, i_c108b0),
    (0xC108B4, i_c108b4),
    (0xC108B6, i_c108b6),
    (0xC108B8, i_c108b8),
    (0xC108BC, i_c108bc),
    (0xC108C0, i_c108c0),
    (0xC108C4, i_c108c4),
    (0xC108C8, i_c108c8),
    (0xC108CA, i_c108ca),
    (0xC108D2, i_c108d2),
    (0xC108DA, i_c108da),
    (0xC10A84, i_c10a84),
    (0xC10A8C, i_c10a8c),
    (0xC10A90, i_c10a90),
    (0xC10A98, i_c10a98),
    (0xC10AA0, i_c10aa0),
    (0xC10AA4, i_c10aa4),
    (0xC10AAC, i_c10aac),
    (0xC10AB0, i_c10ab0),
    (0xC10AB8, i_c10ab8),
    (0xC10ABC, i_c10abc),
    (0xC10AC4, i_c10ac4),
    (0xC10AC8, i_c10ac8),
    (0xC10AD0, i_c10ad0),
    (0xC10AD4, i_c10ad4),
    (0xC10ADC, i_c10adc),
    (0xC10AE2, i_c10ae2),
    (0xC10AE8, i_c10ae8),
    (0xC10AEE, i_c10aee),
    (0xC10AF4, i_c10af4),
    (0xC10AF6, i_c10af6),
    (0xC10AFC, i_c10afc),
    (0xC10B00, i_c10b00),
    (0xC10B04, i_c10b04),
    (0xC10B0A, i_c10b0a),
    (0xC10B0C, i_c10b0c),
    (0xC10B12, i_c10b12),
    (0xC10B16, i_c10b16),
    (0xC10B18, i_c10b18),
    (0xC10B1E, i_c10b1e),
    (0xC10B22, i_c10b22),
    (0xC10B28, i_c10b28),
    (0xC10B2C, i_c10b2c),
    (0xC10B2E, i_c10b2e),
    (0xC10B34, i_c10b34),
    (0xC10B3A, i_c10b3a),
    (0xC10B3E, i_c10b3e),
    (0xC10B44, i_c10b44),
    (0xC10B46, i_c10b46),
    (0xC10B4C, i_c10b4c),
    (0xC10B52, i_c10b52),
    (0xC10B54, i_c10b54),
    (0xC10B58, i_c10b58),
    (0xC10B5C, i_c10b5c),
    (0xC10B62, i_c10b62),
    (0xC10B66, i_c10b66),
    (0xC10B6A, i_c10b6a),
    (0xC10B70, i_c10b70),
    (0xC10B76, i_c10b76),
    (0xC10B7A, i_c10b7a),
    (0xC10B80, i_c10b80),
    (0xC10B86, i_c10b86),
    (0xC10B90, i_c10b90),
    (0xC10B94, i_c10b94),
    (0xC10B9A, i_c10b9a),
    (0xC10B9E, i_c10b9e),
    (0xC10BA2, i_c10ba2),
    (0xC10BA8, i_c10ba8),
    (0xC10BAA, i_c10baa),
    (0xC10BB0, i_c10bb0),
    (0xC10BB4, i_c10bb4),
    (0xC10BB8, i_c10bb8),
    (0xC10BBE, i_c10bbe),
    (0xC10BC0, i_c10bc0),
    (0xC10BC6, i_c10bc6),
    (0xC10BCA, i_c10bca),
    (0xC10BD0, i_c10bd0),
    (0xC10BD2, i_c10bd2),
    (0xC10BD6, i_c10bd6),
    (0xC10BD8, i_c10bd8),
    (0xC10BDC, i_c10bdc),
    (0xC10BE0, i_c10be0),
    (0xC10BE4, i_c10be4),
    (0xC10BEA, i_c10bea),
    (0xC10BF2, i_c10bf2),
    (0xC10BF6, i_c10bf6),
    (0xC10BFA, i_c10bfa),
    (0xC10BFE, i_c10bfe),
    (0xC10C04, i_c10c04),
    (0xC10C06, i_c10c06),
    (0xC10C0A, i_c10c0a),
    (0xC10C10, i_c10c10),
    (0xC10C14, i_c10c14),
    (0xC10C18, i_c10c18),
    (0xC10C1C, i_c10c1c),
    (0xC10C20, i_c10c20),
    (0xC10C2A, i_c10c2a),
    (0xC10C2E, i_c10c2e),
    (0xC10C32, i_c10c32),
    (0xC10C34, i_c10c34),
    (0xC10C36, i_c10c36),
    (0xC10C3C, i_c10c3c),
    (0xC10C3E, i_c10c3e),
    (0xC10C42, i_c10c42),
    (0xC10C48, i_c10c48),
    (0xC10C4E, i_c10c4e),
    (0xC10C54, i_c10c54),
    (0xC10C56, i_c10c56),
    (0xC10C58, i_c10c58),
    (0xC10C5A, i_c10c5a),
    (0xC10C5C, i_c10c5c),
    (0xC10C62, i_c10c62),
    (0xC10C66, i_c10c66),
    (0xC10C6E, i_c10c6e),
    (0xC10C78, i_c10c78),
    (0xC10C7E, i_c10c7e),
    (0xC10C82, i_c10c82),
    (0xC10C8A, i_c10c8a),
    (0xC10C8E, i_c10c8e),
    (0xC10C94, i_c10c94),
    (0xC10C9A, i_c10c9a),
    (0xC10CA0, i_c10ca0),
    (0xC10CA6, i_c10ca6),
    (0xC10CA8, i_c10ca8),
    (0xC10CAC, i_c10cac),
    (0xC10CB0, i_c10cb0),
    (0xC10CB6, i_c10cb6),
    (0xC10CB8, i_c10cb8),
    (0xC10CBC, i_c10cbc),
    (0xC10CBE, i_c10cbe),
    (0xC10CC2, i_c10cc2),
    (0xC10CC8, i_c10cc8),
    (0xC10CCA, i_c10cca),
    (0xC10CCC, i_c10ccc),
    (0xC10CD0, i_c10cd0),
    (0xC10CD4, i_c10cd4),
    (0xC10CD8, i_c10cd8),
    (0xC10CDA, i_c10cda),
    (0xC10CDE, i_c10cde),
    (0xC10CE4, i_c10ce4),
    (0xC10CE6, i_c10ce6),
    (0xC10CEA, i_c10cea),
    (0xC10CEC, i_c10cec),
    (0xC10CF0, i_c10cf0),
    (0xC10CF6, i_c10cf6),
    (0xC10CF8, i_c10cf8),
    (0xC10CFA, i_c10cfa),
    (0xC10CFE, i_c10cfe),
    (0xC10D02, i_c10d02),
    (0xC10D04, i_c10d04),
    (0xC10D08, i_c10d08),
    (0xC10D0E, i_c10d0e),
    (0xC10D10, i_c10d10),
    (0xC10D12, i_c10d12),
    (0xC10D14, i_c10d14),
    (0xC10D16, i_c10d16),
    (0xC10D18, i_c10d18),
    (0xC10D1C, i_c10d1c),
    (0xC10D1E, i_c10d1e),
    (0xC10D22, i_c10d22),
    (0xC10D28, i_c10d28),
    (0xC10D2A, i_c10d2a),
    (0xC10D2C, i_c10d2c),
    (0xC10D2E, i_c10d2e),
    (0xC10D30, i_c10d30),
    (0xC10D32, i_c10d32),
    (0xC10D36, i_c10d36),
    (0xC10D38, i_c10d38),
    (0xC10D3A, i_c10d3a),
    (0xC10D3E, i_c10d3e),
    (0xC10D42, i_c10d42),
    (0xC10D44, i_c10d44),
    (0xC10D46, i_c10d46),
    (0xC10D4A, i_c10d4a),
    (0xC10D4E, i_c10d4e),
    (0xC10D54, i_c10d54),
    (0xC10D56, i_c10d56),
    (0xC10D58, i_c10d58),
    (0xC10D5A, i_c10d5a),
    (0xC10D5C, i_c10d5c),
    (0xC10D5E, i_c10d5e),
    (0xC10D64, i_c10d64),
    (0xC10D66, i_c10d66),
    (0xC10D6C, i_c10d6c),
    (0xC10D70, i_c10d70),
    (0xC10D72, i_c10d72),
    (0xC10D78, i_c10d78),
    (0xC10D7A, i_c10d7a),
    (0xC10D7E, i_c10d7e),
    (0xC10D84, i_c10d84),
    (0xC10D88, i_c10d88),
    (0xC10D8C, i_c10d8c),
    (0xC10D8E, i_c10d8e),
    (0xC10D94, i_c10d94),
    (0xC10D9A, i_c10d9a),
    (0xC10D9C, i_c10d9c),
    (0xC10DA2, i_c10da2),
    (0xC10DA4, i_c10da4),
    (0xC10DA6, i_c10da6),
    (0xC10DA8, i_c10da8),
    (0xC10DAC, i_c10dac),
    (0xC10DAE, i_c10dae),
    (0xC10DB0, i_c10db0),
    (0xC10DB6, i_c10db6),
    (0xC10DBC, i_c10dbc),
    (0xC10DC2, i_c10dc2),
    (0xC10DC8, i_c10dc8),
    (0xC10DCE, i_c10dce),
    (0xC10DD0, i_c10dd0),
    (0xC10DD4, i_c10dd4),
    (0xC10DD8, i_c10dd8),
    (0xC10DDC, i_c10ddc),
    (0xC10DE2, i_c10de2),
    (0xC10DE6, i_c10de6),
    (0xC10DEA, i_c10dea),
    (0xC10DEC, i_c10dec),
    (0xC10DF0, i_c10df0),
    (0xC10DF4, i_c10df4),
    (0xC10DFA, i_c10dfa),
    (0xC10E00, i_c10e00),
    (0xC10E04, i_c10e04),
    (0xC10E08, i_c10e08),
    (0xC10E0A, i_c10e0a),
    (0xC10E0C, i_c10e0c),
    (0xC10E0E, i_c10e0e),
    (0xC10E12, i_c10e12),
    (0xC10E16, i_c10e16),
    (0xC10E1A, i_c10e1a),
    (0xC10E1E, i_c10e1e),
    (0xC10E24, i_c10e24),
    (0xC10E28, i_c10e28),
    (0xC10E2A, i_c10e2a),
    (0xC10E2E, i_c10e2e),
    (0xC10E30, i_c10e30),
    (0xC10E32, i_c10e32),
    (0xC10E36, i_c10e36),
    (0xC10E3A, i_c10e3a),
    (0xC10E3E, i_c10e3e),
    (0xC10E42, i_c10e42),
    (0xC10E46, i_c10e46),
    (0xC10E4A, i_c10e4a),
    (0xC10E4E, i_c10e4e),
    (0xC10E52, i_c10e52),
    (0xC10E56, i_c10e56),
    (0xC10E58, i_c10e58),
    (0xC10E5A, i_c10e5a),
    (0xC10E5E, i_c10e5e),
    (0xC10E60, i_c10e60),
    (0xC10E62, i_c10e62),
    (0xC10E66, i_c10e66),
    (0xC10E6A, i_c10e6a),
    (0xC10E6E, i_c10e6e),
    (0xC10E72, i_c10e72),
    (0xC10E76, i_c10e76),
    (0xC10E7A, i_c10e7a),
    (0xC10E7E, i_c10e7e),
    (0xC10E80, i_c10e80),
    (0xC10E84, i_c10e84),
    (0xC10E88, i_c10e88),
    (0xC10E8C, i_c10e8c),
    (0xC10E90, i_c10e90),
    (0xC10E92, i_c10e92),
    (0xC10E98, i_c10e98),
    (0xC10E9A, i_c10e9a),
    (0xC10E9E, i_c10e9e),
    (0xC10EA0, i_c10ea0),
    (0xC10EA4, i_c10ea4),
    (0xC10EA6, i_c10ea6),
    (0xC10EAC, i_c10eac),
    (0xC10EB0, i_c10eb0),
    (0xC10EB2, i_c10eb2),
    (0xC10EB4, i_c10eb4),
    (0xC10EBA, i_c10eba),
    (0xC10EBE, i_c10ebe),
    (0xC10EC0, i_c10ec0),
    (0xC10EC2, i_c10ec2),
    (0xC10EC6, i_c10ec6),
    (0xC10EC8, i_c10ec8),
    (0xC10ED0, i_c10ed0),
    (0xC10ED6, i_c10ed6),
    (0xC10EDA, i_c10eda),
    (0xC10EE0, i_c10ee0),
    (0xC10EE4, i_c10ee4),
    (0xC10EE6, i_c10ee6),
    (0xC10EEA, i_c10eea),
    (0xC10EEC, i_c10eec),
    (0xC10EEE, i_c10eee),
    (0xC10EF2, i_c10ef2),
    (0xC10EF6, i_c10ef6),
    (0xC10EFA, i_c10efa),
    (0xC10EFE, i_c10efe),
    (0xC10F02, i_c10f02),
    (0xC10F06, i_c10f06),
    (0xC10F0A, i_c10f0a),
    (0xC10F0C, i_c10f0c),
    (0xC10F10, i_c10f10),
    (0xC10F14, i_c10f14),
    (0xC10F18, i_c10f18),
    (0xC10F1C, i_c10f1c),
    (0xC10F20, i_c10f20),
    (0xC10F24, i_c10f24),
    (0xC10F28, i_c10f28),
    (0xC10F2C, i_c10f2c),
    (0xC10F30, i_c10f30),
    (0xC10F32, i_c10f32),
    (0xC10F36, i_c10f36),
    (0xC10F3C, i_c10f3c),
    (0xC10F42, i_c10f42),
    (0xC10F48, i_c10f48),
    (0xC10F4E, i_c10f4e),
    (0xC10F54, i_c10f54),
    (0xC10F56, i_c10f56),
    (0xC10F5C, i_c10f5c),
    (0xC10F5E, i_c10f5e),
    (0xC10F60, i_c10f60),
    (0xC10F62, i_c10f62),
    (0xC10F66, i_c10f66),
    (0xC10F6C, i_c10f6c),
    (0xC10F70, i_c10f70),
    (0xC10F72, i_c10f72),
    (0xC10F78, i_c10f78),
    (0xC10F7C, i_c10f7c),
    (0xC10F7E, i_c10f7e),
    (0xC10F82, i_c10f82),
    (0xC10F84, i_c10f84),
    (0xC10F88, i_c10f88),
    (0xC10F8A, i_c10f8a),
    (0xC10F8C, i_c10f8c),
    (0xC10F8E, i_c10f8e),
    (0xC10F90, i_c10f90),
    (0xC10F94, i_c10f94),
    (0xC10F9A, i_c10f9a),
    (0xC10FA0, i_c10fa0),
    (0xC10FA6, i_c10fa6),
    (0xC10FAC, i_c10fac),
    (0xC10FAE, i_c10fae),
    (0xC10FB4, i_c10fb4),
    (0xC10FB6, i_c10fb6),
    (0xC10FBA, i_c10fba),
    (0xC10FBC, i_c10fbc),
    (0xC10FBE, i_c10fbe),
    (0xC10FC6, i_c10fc6),
    (0xC10FCA, i_c10fca),
    (0xC10FCE, i_c10fce),
    (0xC10FD0, i_c10fd0),
    (0xC10FD6, i_c10fd6),
    (0xC10FD8, i_c10fd8),
    (0xC10FE0, i_c10fe0),
    (0xC10FE6, i_c10fe6),
    (0xC10FEC, i_c10fec),
    (0xC10FEE, i_c10fee),
    (0xC10FF0, i_c10ff0),
    (0xC10FF6, i_c10ff6),
    (0xC10FF8, i_c10ff8),
    (0xC10FFA, i_c10ffa),
    (0xC10FFE, i_c10ffe),
    (0xC11000, i_c11000),
    (0xC11002, i_c11002),
    (0xC11006, i_c11006),
    (0xC1100C, i_c1100c),
    (0xC1100E, i_c1100e),
    (0xC11012, i_c11012),
    (0xC11016, i_c11016),
    (0xC11018, i_c11018),
    (0xC1101A, i_c1101a),
    (0xC1101C, i_c1101c),
    (0xC11020, i_c11020),
    (0xC11024, i_c11024),
    (0xC11026, i_c11026),
    (0xC1102A, i_c1102a),
    (0xC1102C, i_c1102c),
    (0xC11030, i_c11030),
    (0xC11032, i_c11032),
    (0xC11034, i_c11034),
    (0xC11036, i_c11036),
    (0xC11038, i_c11038),
    (0xC1103A, i_c1103a),
    (0xC1103C, i_c1103c),
    (0xC1103E, i_c1103e),
    (0xC11042, i_c11042),
    (0xC11044, i_c11044),
    (0xC1104A, i_c1104a),
    (0xC1104C, i_c1104c),
    (0xC11050, i_c11050),
    (0xC11054, i_c11054),
    (0xC11056, i_c11056),
    (0xC1105A, i_c1105a),
    (0xC1105E, i_c1105e),
    (0xC11060, i_c11060),
    (0xC11064, i_c11064),
    (0xC11068, i_c11068),
    (0xC1106A, i_c1106a),
    (0xC1106C, i_c1106c),
    (0xC1106E, i_c1106e),
    (0xC11072, i_c11072),
    (0xC11076, i_c11076),
    (0xC11078, i_c11078),
    (0xC1107A, i_c1107a),
    (0xC1107C, i_c1107c),
    (0xC1107E, i_c1107e),
    (0xC11080, i_c11080),
    (0xC11084, i_c11084),
    (0xC11086, i_c11086),
    (0xC11088, i_c11088),
    (0xC1108C, i_c1108c),
    (0xC1108E, i_c1108e),
    (0xC11090, i_c11090),
    (0xC11092, i_c11092),
    (0xC11094, i_c11094),
    (0xC11098, i_c11098),
    (0xC1109C, i_c1109c),
    (0xC110A0, i_c110a0),
    (0xC110A6, i_c110a6),
    (0xC110AC, i_c110ac),
    (0xC110AE, i_c110ae),
    (0xC110B2, i_c110b2),
    (0xC110B4, i_c110b4),
    (0xC110B6, i_c110b6),
    (0xC110B8, i_c110b8),
    (0xC110BC, i_c110bc),
    (0xC110BE, i_c110be),
    (0xC110C0, i_c110c0),
    (0xC110C4, i_c110c4),
    (0xC110C6, i_c110c6),
    (0xC110C8, i_c110c8),
    (0xC110CA, i_c110ca),
    (0xC110CC, i_c110cc),
    (0xC110CE, i_c110ce),
    (0xC110D4, i_c110d4),
    (0xC110DA, i_c110da),
    (0xC110DC, i_c110dc),
    (0xC110DE, i_c110de),
    (0xC110E0, i_c110e0),
    (0xC110E2, i_c110e2),
    (0xC110E4, i_c110e4),
    (0xC110EA, i_c110ea),
    (0xC110F0, i_c110f0),
    (0xC110F2, i_c110f2),
    (0xC110F4, i_c110f4),
    (0xC110F6, i_c110f6),
    (0xC110FA, i_c110fa),
    (0xC110FE, i_c110fe),
    (0xC11102, i_c11102),
    (0xC11104, i_c11104),
    (0xC11108, i_c11108),
    (0xC1110A, i_c1110a),
    (0xC1110C, i_c1110c),
    (0xC1110E, i_c1110e),
    (0xC11112, i_c11112),
    (0xC11114, i_c11114),
    (0xC1111A, i_c1111a),
    (0xC1111C, i_c1111c),
    (0xC11122, i_c11122),
    (0xC11124, i_c11124),
    (0xC11126, i_c11126),
    (0xC11128, i_c11128),
    (0xC1112A, i_c1112a),
    (0xC1112C, i_c1112c),
    (0xC11130, i_c11130),
    (0xC11134, i_c11134),
    (0xC11138, i_c11138),
    (0xC1113C, i_c1113c),
    (0xC11140, i_c11140),
    (0xC11144, i_c11144),
    (0xC1114A, i_c1114a),
    (0xC1114E, i_c1114e),
    (0xC11152, i_c11152),
    (0xC11156, i_c11156),
    (0xC1115A, i_c1115a),
    (0xC1115E, i_c1115e),
    (0xC11162, i_c11162),
    (0xC11164, i_c11164),
    (0xC11168, i_c11168),
    (0xC1116A, i_c1116a),
    (0xC1116C, i_c1116c),
    (0xC11170, i_c11170),
    (0xC11174, i_c11174),
    (0xC11178, i_c11178),
    (0xC1117A, i_c1117a),
    (0xC1117E, i_c1117e),
    (0xC11180, i_c11180),
    (0xC11184, i_c11184),
    (0xC11186, i_c11186),
    (0xC11188, i_c11188),
    (0xC1118C, i_c1118c),
    (0xC1118E, i_c1118e),
    (0xC11190, i_c11190),
    (0xC11196, i_c11196),
    (0xC11198, i_c11198),
    (0xC1119E, i_c1119e),
    (0xC111A0, i_c111a0),
    (0xC111A6, i_c111a6),
    (0xC111AA, i_c111aa),
    (0xC111B0, i_c111b0),
    (0xC111B2, i_c111b2),
    (0xC111B6, i_c111b6),
    (0xC111B8, i_c111b8),
    (0xC111BA, i_c111ba),
    (0xC111BE, i_c111be),
    (0xC111C2, i_c111c2),
    (0xC111C6, i_c111c6),
    (0xC111CA, i_c111ca),
    (0xC111D0, i_c111d0),
    (0xC111D4, i_c111d4),
    (0xC111D6, i_c111d6),
    (0xC111D8, i_c111d8),
    (0xC111DC, i_c111dc),
    (0xC111DE, i_c111de),
    (0xC111E0, i_c111e0),
    (0xC111E4, i_c111e4),
    (0xC111E6, i_c111e6),
    (0xC111E8, i_c111e8),
    (0xC111EA, i_c111ea),
    (0xC111EC, i_c111ec),
    (0xC111F0, i_c111f0),
    (0xC111F4, i_c111f4),
    (0xC111F8, i_c111f8),
    (0xC111FA, i_c111fa),
    (0xC111FE, i_c111fe),
    (0xC11200, i_c11200),
    (0xC11202, i_c11202),
    (0xC11206, i_c11206),
    (0xC11208, i_c11208),
    (0xC1120C, i_c1120c),
    (0xC11210, i_c11210),
    (0xC11212, i_c11212),
    (0xC11214, i_c11214),
    (0xC11218, i_c11218),
    (0xC1121C, i_c1121c),
    (0xC1121E, i_c1121e),
    (0xC11222, i_c11222),
    (0xC11224, i_c11224),
    (0xC11226, i_c11226),
    (0xC1122A, i_c1122a),
    (0xC1122C, i_c1122c),
    (0xC11230, i_c11230),
    (0xC11232, i_c11232),
    (0xC11234, i_c11234),
    (0xC11236, i_c11236),
    (0xC1123A, i_c1123a),
    (0xC1123E, i_c1123e),
    (0xC11240, i_c11240),
    (0xC11244, i_c11244),
    (0xC11246, i_c11246),
    (0xC1124A, i_c1124a),
    (0xC1124E, i_c1124e),
    (0xC11250, i_c11250),
    (0xC11256, i_c11256),
    (0xC11258, i_c11258),
    (0xC1125A, i_c1125a),
    (0xC11260, i_c11260),
    (0xC11262, i_c11262),
    (0xC11264, i_c11264),
    (0xC11268, i_c11268),
    (0xC1126A, i_c1126a),
    (0xC1126E, i_c1126e),
    (0xC11272, i_c11272),
    (0xC11274, i_c11274),
    (0xC11276, i_c11276),
    (0xC11278, i_c11278),
    (0xC1127A, i_c1127a),
    (0xC1127C, i_c1127c),
    (0xC1127E, i_c1127e),
    (0xC11282, i_c11282),
    (0xC11286, i_c11286),
    (0xC1128A, i_c1128a),
    (0xC11290, i_c11290),
    (0xC11292, i_c11292),
    (0xC11296, i_c11296),
    (0xC11298, i_c11298),
    (0xC1129C, i_c1129c),
    (0xC112A0, i_c112a0),
    (0xC112A2, i_c112a2),
    (0xC112A8, i_c112a8),
    (0xC112AA, i_c112aa),
    (0xC112AE, i_c112ae),
    (0xC112B2, i_c112b2),
    (0xC112B6, i_c112b6),
    (0xC112B8, i_c112b8),
    (0xC112BA, i_c112ba),
    (0xC112BE, i_c112be),
    (0xC112C2, i_c112c2),
    (0xC112C6, i_c112c6),
    (0xC112C8, i_c112c8),
    (0xC112CE, i_c112ce),
    (0xC112D0, i_c112d0),
    (0xC112D4, i_c112d4),
    (0xC112D8, i_c112d8),
    (0xC112DC, i_c112dc),
    (0xC112DE, i_c112de),
    (0xC112E0, i_c112e0),
    (0xC112E2, i_c112e2),
    (0xC112E4, i_c112e4),
    (0xC112EA, i_c112ea),
    (0xC112EC, i_c112ec),
    (0xC112F2, i_c112f2),
    (0xC112F4, i_c112f4),
    (0xC112FA, i_c112fa),
    (0xC112FE, i_c112fe),
    (0xC11300, i_c11300),
    (0xC11306, i_c11306),
    (0xC11308, i_c11308),
    (0xC1130C, i_c1130c),
    (0xC11310, i_c11310),
    (0xC11312, i_c11312),
    (0xC11318, i_c11318),
    (0xC1131A, i_c1131a),
    (0xC1131E, i_c1131e),
    (0xC11320, i_c11320),
    (0xC11324, i_c11324),
    (0xC11326, i_c11326),
    (0xC11328, i_c11328),
    (0xC1132E, i_c1132e),
    (0xC11330, i_c11330),
    (0xC11332, i_c11332),
    (0xC11334, i_c11334),
    (0xC1133A, i_c1133a),
    (0xC11372, i_c11372),
    (0xC11378, i_c11378),
    (0xC1137E, i_c1137e),
    (0xC11382, i_c11382),
    (0xC11386, i_c11386),
    (0xC1138A, i_c1138a),
    (0xC1138E, i_c1138e),
    (0xC11394, i_c11394),
    (0xC1139C, i_c1139c),
    (0xC113A0, i_c113a0),
    (0xC113A4, i_c113a4),
    (0xC113A8, i_c113a8),
    (0xC113AA, i_c113aa),
    (0xC113AE, i_c113ae),
    (0xC113B0, i_c113b0),
    (0xC113B2, i_c113b2),
    (0xC113B4, i_c113b4),
    (0xC113B6, i_c113b6),
    (0xC113B8, i_c113b8),
    (0xC113BA, i_c113ba),
    (0xC113BE, i_c113be),
    (0xC113C0, i_c113c0),
    (0xC113C6, i_c113c6),
    (0xC113C8, i_c113c8),
    (0xC113CA, i_c113ca),
    (0xC113D0, i_c113d0),
    (0xC113D8, i_c113d8),
    (0xC113DC, i_c113dc),
    (0xC113E0, i_c113e0),
    (0xC113E8, i_c113e8),
    (0xC113EA, i_c113ea),
    (0xC113EE, i_c113ee),
    (0xC113F0, i_c113f0),
    (0xC113F2, i_c113f2),
    (0xC113F8, i_c113f8),
    (0xC113FC, i_c113fc),
    (0xC11400, i_c11400),
    (0xC11402, i_c11402),
    (0xC11404, i_c11404),
    (0xC1140A, i_c1140a),
    (0xC1140C, i_c1140c),
    (0xC11410, i_c11410),
    (0xC11412, i_c11412),
    (0xC11416, i_c11416),
    (0xC11418, i_c11418),
    (0xC1141A, i_c1141a),
    (0xC11420, i_c11420),
    (0xC11422, i_c11422),
    (0xC11424, i_c11424),
    (0xC11426, i_c11426),
    (0xC11428, i_c11428),
    (0xC1142A, i_c1142a),
    (0xC1142E, i_c1142e),
    (0xC11432, i_c11432),
    (0xC11436, i_c11436),
    (0xC11438, i_c11438),
    (0xC1143A, i_c1143a),
    (0xC1143C, i_c1143c),
    (0xC1143E, i_c1143e),
    (0xC11440, i_c11440),
    (0xC11444, i_c11444),
    (0xC11446, i_c11446),
    (0xC1144A, i_c1144a),
    (0xC1144C, i_c1144c),
    (0xC11452, i_c11452),
    (0xC11456, i_c11456),
    (0xC1145A, i_c1145a),
    (0xC1145C, i_c1145c),
    (0xC11460, i_c11460),
    (0xC11462, i_c11462),
    (0xC11466, i_c11466),
    (0xC1146A, i_c1146a),
    (0xC1146C, i_c1146c),
    (0xC1146E, i_c1146e),
    (0xC11470, i_c11470),
    (0xC11474, i_c11474),
    (0xC11478, i_c11478),
    (0xC1147C, i_c1147c),
    (0xC1147E, i_c1147e),
    (0xC11482, i_c11482),
    (0xC11486, i_c11486),
    (0xC11488, i_c11488),
    (0xC1148C, i_c1148c),
    (0xC11490, i_c11490),
    (0xC11492, i_c11492),
    (0xC11494, i_c11494),
    (0xC11498, i_c11498),
    (0xC1149C, i_c1149c),
    (0xC114A0, i_c114a0),
    (0xC114A8, i_c114a8),
    (0xC114AE, i_c114ae),
    (0xC114D0, i_c114d0),
    (0xC114D2, i_c114d2),
    (0xC114D6, i_c114d6),
    (0xC114D8, i_c114d8),
    (0xC114DC, i_c114dc),
    (0xC114E0, i_c114e0),
    (0xC114E2, i_c114e2),
    (0xC114E6, i_c114e6),
    (0xC11508, i_c11508),
    (0xC1150A, i_c1150a),
    (0xC1150E, i_c1150e),
    (0xC11510, i_c11510),
    (0xC11514, i_c11514),
    (0xC11518, i_c11518),
    (0xC1151A, i_c1151a),
    (0xC1151E, i_c1151e),
    (0xC11522, i_c11522),
    (0xC11524, i_c11524),
    (0xC11526, i_c11526),
    (0xC1152A, i_c1152a),
    (0xC1152C, i_c1152c),
    (0xC11530, i_c11530),
    (0xC11534, i_c11534),
    (0xC11536, i_c11536),
    (0xC1153A, i_c1153a),
    (0xC1155C, i_c1155c),
    (0xC11560, i_c11560),
    (0xC11562, i_c11562),
    (0xC11566, i_c11566),
    (0xC1156C, i_c1156c),
    (0xC1156E, i_c1156e),
    (0xC11570, i_c11570),
    (0xC11574, i_c11574),
    (0xC11578, i_c11578),
    (0xC1157C, i_c1157c),
    (0xC11580, i_c11580),
    (0xC11592, i_c11592),
    (0xC11598, i_c11598),
    (0xC1159A, i_c1159a),
    (0xC115A0, i_c115a0),
    (0xC115A4, i_c115a4),
    (0xC115A6, i_c115a6),
    (0xC115A8, i_c115a8),
    (0xC115AA, i_c115aa),
    (0xC115AE, i_c115ae),
    (0xC115B0, i_c115b0),
    (0xC115B4, i_c115b4),
    (0xC115BA, i_c115ba),
    (0xC115BC, i_c115bc),
    (0xC115BE, i_c115be),
    (0xC115C6, i_c115c6),
    (0xC115CC, i_c115cc),
    (0xC115D2, i_c115d2),
    (0xC115D4, i_c115d4),
    (0xC115D6, i_c115d6),
    (0xC115D8, i_c115d8),
    (0xC115DA, i_c115da),
    (0xC115DE, i_c115de),
    (0xC115E4, i_c115e4),
    (0xC115E8, i_c115e8),
    (0xC115EC, i_c115ec),
    (0xC115F0, i_c115f0),
    (0xC115F4, i_c115f4),
    (0xC115F8, i_c115f8),
    (0xC115FE, i_c115fe),
    (0xC11602, i_c11602),
    (0xC11604, i_c11604),
    (0xC11606, i_c11606),
    (0xC1160A, i_c1160a),
    (0xC11610, i_c11610),
    (0xC11616, i_c11616),
    (0xC11618, i_c11618),
    (0xC1161A, i_c1161a),
    (0xC1161E, i_c1161e),
    (0xC11626, i_c11626),
    (0xC1162C, i_c1162c),
    (0xC11632, i_c11632),
    (0xC11634, i_c11634),
    (0xC11636, i_c11636),
    (0xC1163A, i_c1163a),
    (0xC11644, i_c11644),
    (0xC1164C, i_c1164c),
    (0xC11652, i_c11652),
    (0xC11654, i_c11654),
    (0xC1165C, i_c1165c),
    (0xC11660, i_c11660),
    (0xC11662, i_c11662),
    (0xC11668, i_c11668),
    (0xC11670, i_c11670),
    (0xC11674, i_c11674),
    (0xC1167C, i_c1167c),
    (0xC1167E, i_c1167e),
    (0xC11682, i_c11682),
    (0xC11684, i_c11684),
    (0xC11686, i_c11686),
    (0xC1168C, i_c1168c),
    (0xC116B4, i_c116b4),
    (0xC116B8, i_c116b8),
    (0xC116BA, i_c116ba),
    (0xC116C2, i_c116c2),
    (0xC116C6, i_c116c6),
    (0xC116C8, i_c116c8),
    (0xC116D0, i_c116d0),
    (0xC116D2, i_c116d2),
    (0xC116D6, i_c116d6),
    (0xC116DC, i_c116dc),
    (0xC116E0, i_c116e0),
    (0xC116E2, i_c116e2),
    (0xC116E4, i_c116e4),
    (0xC116E8, i_c116e8),
    (0xC116EE, i_c116ee),
    (0xC116F4, i_c116f4),
    (0xC116F8, i_c116f8),
    (0xC116FA, i_c116fa),
    (0xC116FC, i_c116fc),
    (0xC11702, i_c11702),
    (0xC11708, i_c11708),
    (0xC1170E, i_c1170e),
    (0xC11710, i_c11710),
    (0xC11714, i_c11714),
    (0xC11718, i_c11718),
    (0xC1171A, i_c1171a),
    (0xC1171C, i_c1171c),
    (0xC11720, i_c11720),
    (0xC11724, i_c11724),
    (0xC11726, i_c11726),
    (0xC1172A, i_c1172a),
    (0xC1172C, i_c1172c),
    (0xC11732, i_c11732),
    (0xC11738, i_c11738),
    (0xC1173A, i_c1173a),
    (0xC1173E, i_c1173e),
    (0xC11742, i_c11742),
    (0xC11746, i_c11746),
    (0xC1174A, i_c1174a),
    (0xC1174C, i_c1174c),
    (0xC1174E, i_c1174e),
    (0xC11750, i_c11750),
    (0xC11752, i_c11752),
    (0xC11756, i_c11756),
    (0xC11758, i_c11758),
    (0xC1175A, i_c1175a),
    (0xC1175C, i_c1175c),
    (0xC1175E, i_c1175e),
    (0xC11762, i_c11762),
    (0xC11766, i_c11766),
    (0xC1176A, i_c1176a),
    (0xC1176E, i_c1176e),
    (0xC11772, i_c11772),
    (0xC11776, i_c11776),
    (0xC11778, i_c11778),
    (0xC1177A, i_c1177a),
    (0xC1177C, i_c1177c),
    (0xC1177E, i_c1177e),
    (0xC11782, i_c11782),
    (0xC11786, i_c11786),
    (0xC11788, i_c11788),
    (0xC1178A, i_c1178a),
    (0xC11790, i_c11790),
    (0xC11794, i_c11794),
    (0xC11798, i_c11798),
    (0xC1179C, i_c1179c),
    (0xC117A0, i_c117a0),
    (0xC117A4, i_c117a4),
    (0xC117A8, i_c117a8),
    (0xC117AE, i_c117ae),
    (0xC117B0, i_c117b0),
    (0xC117B4, i_c117b4),
    (0xC117B6, i_c117b6),
    (0xC117B8, i_c117b8),
    (0xC117BC, i_c117bc),
    (0xC117C0, i_c117c0),
    (0xC117C2, i_c117c2),
    (0xC117C8, i_c117c8),
    (0xC117CC, i_c117cc),
    (0xC117D0, i_c117d0),
    (0xC117D2, i_c117d2),
    (0xC117D6, i_c117d6),
    (0xC117DA, i_c117da),
    (0xC117DC, i_c117dc),
    (0xC117E0, i_c117e0),
    (0xC117E4, i_c117e4),
    (0xC117EA, i_c117ea),
    (0xC117EC, i_c117ec),
    (0xC117EE, i_c117ee),
    (0xC117F2, i_c117f2),
    (0xC117F4, i_c117f4),
    (0xC117F8, i_c117f8),
    (0xC117FC, i_c117fc),
    (0xC117FE, i_c117fe),
    (0xC11802, i_c11802),
    (0xC11806, i_c11806),
    (0xC1180C, i_c1180c),
    (0xC1180E, i_c1180e),
    (0xC11812, i_c11812),
    (0xC11816, i_c11816),
    (0xC11818, i_c11818),
    (0xC1181E, i_c1181e),
    (0xC11822, i_c11822),
    (0xC11828, i_c11828),
    (0xC1182A, i_c1182a),
    (0xC1182E, i_c1182e),
    (0xC11832, i_c11832),
    (0xC11838, i_c11838),
    (0xC1183C, i_c1183c),
    (0xC11842, i_c11842),
    (0xC11846, i_c11846),
    (0xC1184A, i_c1184a),
    (0xC1184E, i_c1184e),
    (0xC11854, i_c11854),
    (0xC11858, i_c11858),
    (0xC1185C, i_c1185c),
    (0xC11860, i_c11860),
    (0xC11866, i_c11866),
    (0xC1186A, i_c1186a),
    (0xC1186E, i_c1186e),
    (0xC11870, i_c11870),
    (0xC11872, i_c11872),
    (0xC11874, i_c11874),
    (0xC11876, i_c11876),
    (0xC1187A, i_c1187a),
    (0xC1187E, i_c1187e),
    (0xC11882, i_c11882),
    (0xC11884, i_c11884),
    (0xC11888, i_c11888),
    (0xC1188C, i_c1188c),
    (0xC11890, i_c11890),
    (0xC11892, i_c11892),
    (0xC11894, i_c11894),
    (0xC11898, i_c11898),
    (0xC1189A, i_c1189a),
    (0xC1189E, i_c1189e),
    (0xC118A2, i_c118a2),
    (0xC118A4, i_c118a4),
    (0xC118A6, i_c118a6),
    (0xC118AA, i_c118aa),
    (0xC118AE, i_c118ae),
    (0xC118B0, i_c118b0),
    (0xC118B6, i_c118b6),
    (0xC118B8, i_c118b8),
    (0xC118BA, i_c118ba),
    (0xC118BE, i_c118be),
    (0xC118C0, i_c118c0),
    (0xC118C4, i_c118c4),
    (0xC118C8, i_c118c8),
    (0xC118CA, i_c118ca),
    (0xC118CE, i_c118ce),
    (0xC118D0, i_c118d0),
    (0xC118D2, i_c118d2),
    (0xC118D6, i_c118d6),
    (0xC118DA, i_c118da),
    (0xC118DC, i_c118dc),
    (0xC118E2, i_c118e2),
    (0xC118E8, i_c118e8),
    (0xC118EC, i_c118ec),
    (0xC118F0, i_c118f0),
    (0xC118F4, i_c118f4),
    (0xC118F8, i_c118f8),
    (0xC118FC, i_c118fc),
    (0xC118FE, i_c118fe),
    (0xC11900, i_c11900),
    (0xC11902, i_c11902),
    (0xC11904, i_c11904),
    (0xC11906, i_c11906),
    (0xC1190C, i_c1190c),
    (0xC11910, i_c11910),
    (0xC11912, i_c11912),
    (0xC11916, i_c11916),
    (0xC11918, i_c11918),
    (0xC1191A, i_c1191a),
    (0xC1191C, i_c1191c),
    (0xC1191E, i_c1191e),
    (0xC11922, i_c11922),
    (0xC11926, i_c11926),
    (0xC1192A, i_c1192a),
    (0xC1192E, i_c1192e),
    (0xC11932, i_c11932),
    (0xC11936, i_c11936),
    (0xC1193A, i_c1193a),
    (0xC1193E, i_c1193e),
    (0xC11942, i_c11942),
];
