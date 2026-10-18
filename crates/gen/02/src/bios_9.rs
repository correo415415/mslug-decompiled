// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región bios_9) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $C1A132: bne.b $c1a13a
pub fn i_c1a132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A134;
    cpu.ir = 0x6606;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1A13A; return 10; }
    8
}
/// $C1A134: move.b #$1, -$7fbd(a4)
pub fn i_c1a134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A13A;
    cpu.ir = 0x197C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8043);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C1A13A: addq.b #$1, $ac2(a5)
pub fn i_c1a13a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A13E;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1A13E: movea.l $abe(a5), a0
pub fn i_c1a13e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A142;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C1A142: tst.b -$7fbd(a4)
pub fn i_c1a142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A146;
    cpu.ir = 0x4A2C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8043);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C1A146: bne.w $c1a182
pub fn i_c1a146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A14A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1A182; return 10; }
    8
}
/// $C1A14A: clr.l (a0)+
pub fn i_c1a14a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A14C;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C1A14C: move.w #$3, (a0)+
pub fn i_c1a14c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A150;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C1A150: move.w #$7391, (a0)+
pub fn i_c1a150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A154;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x7391;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C1A154: move.w #$108, (a0)+
pub fn i_c1a154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A158;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x108;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C1A158: move.b -$7fbc(a4), d0
pub fn i_c1a158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A15C;
    cpu.ir = 0x102C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8044);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C1A15C: jsr $c16f54(pc)
pub fn i_c1a15c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A160;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1A160);
    cpu.pc = 0xC16F54;
    16
}
/// $C1A160: move.b d0, (a0)+
pub fn i_c1a160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A162;
    cpu.ir = 0x10C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C1A162: move.b d1, (a0)+
pub fn i_c1a162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A164;
    cpu.ir = 0x10C1;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C1A164: move.b #$20, (a0)+
pub fn i_c1a164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A168;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C1A168: move.b #$73, (a0)+
pub fn i_c1a168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A16C;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x73;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C1A16C: move.b #$65, (a0)+
pub fn i_c1a16c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A170;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x65;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C1A170: move.b #$63, (a0)+
pub fn i_c1a170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A174;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x63;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C1A174: move.b #$20, (a0)+
pub fn i_c1a174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A178;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C1A178: move.b #$ff, (a0)+
pub fn i_c1a178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A17C;
    cpu.ir = 0x10FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C1A17C: clr.w (a0)+
pub fn i_c1a17c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A17E;
    cpu.ir = 0x4258;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C1A17E: bra.w $c1a188
pub fn i_c1a17e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A182;
    cpu.ir = 0x6000;
    cpu.pc = 0xC1A188;
    10
}
/// $C1A182: move.l #$c1a192, (a0)+
pub fn i_c1a182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A188;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC1A192;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C1A188: move.l a0, $abe(a5)
pub fn i_c1a188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A18C;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1A18C: subq.b #$1, $ac2(a5)
pub fn i_c1a18c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A190;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1A190: rts
pub fn i_c1a190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A192;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1A192: ori.b #$91, d3
pub fn i_c1a192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A196;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x91) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1A196: movep.w $5749(a0), d0
pub fn i_c1a196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A19A;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x5749);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C1A19A: addq.w #$2, a0
pub fn i_c1a19a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A19C;
    cpu.ir = 0x5448;
    cpu.a[0] = cpu.a[0].wrapping_add(2);
    8
}
/// $C1A19C: ?
pub fn i_c1a19c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A19E;
    cpu.ir = 0x4F55;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1A1A2: move.b $a97(a5), d0
pub fn i_c1a1a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A1A6;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xA97);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C1A1A6: andi.b #$30, d0
pub fn i_c1a1a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A1AA;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x30) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1A1AA: beq.w $c1a1e4
pub fn i_c1a1aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A1AE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1A1E4; return 10; }
    8
}
/// $C1A1AE: move.b d0, $3a001d.l
pub fn i_c1a1ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A1B4;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001D, v1 as u8);
    16
}
/// $C1A1B4: move.b -$7fba(a4), d0
pub fn i_c1a1b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A1B8;
    cpu.ir = 0x102C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8046);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C1A1B8: beq.b $c1a1be
pub fn i_c1a1b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A1BA;
    cpu.ir = 0x6704;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1A1BE; return 10; }
    8
}
/// $C1A1BA: moveq #$0, d0
pub fn i_c1a1ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A1BC;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1A1BC: bra.b $c1a1c0
pub fn i_c1a1bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A1BE;
    cpu.ir = 0x6002;
    cpu.pc = 0xC1A1C0;
    10
}
/// $C1A1BE: moveq #$1, d0
pub fn i_c1a1be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A1C0;
    cpu.ir = 0x7001;
    cpu.d[0] = 0x1;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1A1C0: move.b d0, -$7fba(a4)
pub fn i_c1a1c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A1C4;
    cpu.ir = 0x1940;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8046);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C1A1C4: addq.b #$1, $ac2(a5)
pub fn i_c1a1c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A1C8;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1A1C8: movea.l $abe(a5), a0
pub fn i_c1a1c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A1CC;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C1A1CC: lea.l $c1a202(pc), a1
pub fn i_c1a1cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A1D0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0xC1A202;
    16
}
/// $C1A1D0: tst.b -$7fba(a4)
pub fn i_c1a1d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A1D4;
    cpu.ir = 0x4A2C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8046);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C1A1D4: beq.b $c1a1da
pub fn i_c1a1d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A1D6;
    cpu.ir = 0x6704;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1A1DA; return 10; }
    8
}
/// $C1A1D6: lea.l $c1a1e6(pc), a1
pub fn i_c1a1d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A1DA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0xC1A1E6;
    16
}
/// $C1A1DA: move.l a1, (a0)+
pub fn i_c1a1da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A1DC;
    cpu.ir = 0x20C9;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C1A1DC: move.l a0, $abe(a5)
pub fn i_c1a1dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A1E0;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1A1E0: subq.b #$1, $ac2(a5)
pub fn i_c1a1e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A1E4;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1A1E4: rts
pub fn i_c1a1e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A1E6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1A1E6: ori.b #$34, d3
pub fn i_c1a1e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A1EA;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x34) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1A1EA: movep.w $5749(a0), d0
pub fn i_c1a1ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A1EE;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x5749);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C1A1EE: addq.w #$2, a0
pub fn i_c1a1ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A1F0;
    cpu.ir = 0x5448;
    cpu.a[0] = cpu.a[0].wrapping_add(2);
    8
}
/// $C1A1F0: ?
pub fn i_c1a1f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A1F2;
    cpu.ir = 0x4F55;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1A1F2: addq.b #$2, -(a0)
pub fn i_c1a1f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A1F4;
    cpu.ir = 0x5420;
    let ad1 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    10
}
/// $C1A1F4: move.l -(a0), d0
pub fn i_c1a1f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A1F6;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C1A1F6: move.l -(a0), d0
pub fn i_c1a1f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A1F8;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C1A1F8: move.l -(a0), d0
pub fn i_c1a1f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A1FA;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C1A1FA: move.l -(a0), d0
pub fn i_c1a1fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A1FC;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C1A1FC: move.l -(a0), d0
pub fn i_c1a1fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A1FE;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C1A1FE: ?
pub fn i_c1a1fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A200;
    cpu.ir = 0x20FF;
    let v1: u32 = 0x0;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C1A200: ori.b #$3, d0
pub fn i_c1a200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A204;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1A204: moveq #$34, d1
pub fn i_c1a204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A206;
    cpu.ir = 0x7234;
    cpu.d[1] = 0x34;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1A206: movep.w $5345(a0), d0
pub fn i_c1a206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A20A;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x5345);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C1A20A: addq.b #$2, -(a0)
pub fn i_c1a20a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A20C;
    cpu.ir = 0x5420;
    let ad1 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    10
}
/// $C1A20C: subq.w #$2, (a0)
pub fn i_c1a20c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A20E;
    cpu.ir = 0x5550;
    let ad1 = cpu.a[0];
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C1A20E: movea.l a1, a0
pub fn i_c1a20e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A210;
    cpu.ir = 0x2049;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C1A210: ?
pub fn i_c1a210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A212;
    cpu.ir = 0x4E20;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1A212: ?
pub fn i_c1a212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A214;
    cpu.ir = 0x4541;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1A214: ?
pub fn i_c1a214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A216;
    cpu.ir = 0x4348;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1A216: movea.l d7, a0
pub fn i_c1a216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A218;
    cpu.ir = 0x2047;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C1A218: ?
pub fn i_c1a218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A21A;
    cpu.ir = 0x414D;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1A21E: move.w $9e2(a5), d0
pub fn i_c1a21e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A222;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9E2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C1A222: add.w d0, d0
pub fn i_c1a222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A224;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C1A224: add.w d0, d0
pub fn i_c1a224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A226;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C1A226: movea.l $c1a22c(pc, d0.w), a0
pub fn i_c1a226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A22A;
    cpu.ir = 0x207B;
    let ad2 = 0xC1A22C_u32.wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $C1A22A: jmp (a0)
pub fn i_c1a22a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A22C;
    cpu.ir = 0x4ED0;
    cpu.jump_to(cpu.a[0]);
    8
}
/// $C1A22C: ?
pub fn i_c1a22c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A22E;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1A22E: suba.l -$3f(a2, d0.w), a7
pub fn i_c1a22e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A232;
    cpu.ir = 0x9FF2;
    let ad2 = cpu.a[2].wrapping_add(0xFFFFFFC1).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[7] = cpu.a[7].wrapping_sub(v3);
    22
}
/// $C1A232: suba.l -(a6), a7
pub fn i_c1a232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A234;
    cpu.ir = 0x9FE6;
    let ad2 = cpu.a[6].wrapping_sub(4); cpu.a[6] = ad2;
    let v3 = bus.read32(ad2);
    cpu.a[7] = cpu.a[7].wrapping_sub(v3);
    18
}
/// $C1A234: ?
pub fn i_c1a234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A236;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1A236: dc.w $a09c
pub fn i_c1a236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A238;
    cpu.ir = 0xA09C;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineA);
    return 34;
}
/// $C1A238: ?
pub fn i_c1a238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A23A;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1A23A: dc.w $a13a
pub fn i_c1a23a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A23C;
    cpu.ir = 0xA13A;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineA);
    return 34;
}
/// $C1A23C: ?
pub fn i_c1a23c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A23E;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1A23E: dc.w $a1c4
pub fn i_c1a23e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A240;
    cpu.ir = 0xA1C4;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineA);
    return 34;
}
/// $C1A240: addq.b #$1, $ac2(a5)
pub fn i_c1a240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A244;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1A244: movea.l $abe(a5), a0
pub fn i_c1a244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A248;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C1A248: clr.l (a0)+
pub fn i_c1a248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A24A;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C1A24A: move.w #$20ff, d1
pub fn i_c1a24a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A24E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x20FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1A24E: jsr $c1a26c(pc)
pub fn i_c1a24e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A252;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1A252);
    cpu.pc = 0xC1A26C;
    16
}
/// $C1A252: move.w $9e4(a5), $9e2(a5)
pub fn i_c1a252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A258;
    cpu.ir = 0x3B6D;
    let ad2 = cpu.a[5].wrapping_add(0x9E4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0x9E2);
    bus.write16(ad4, v3 as u16);
    20
}
/// $C1A258: move.w #$11ff, d1
pub fn i_c1a258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A25C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x11FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1A25C: jsr $c1a26c(pc)
pub fn i_c1a25c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A260;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1A260);
    cpu.pc = 0xC1A26C;
    16
}
/// $C1A260: clr.w (a0)+
pub fn i_c1a260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A262;
    cpu.ir = 0x4258;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C1A262: move.l a0, $abe(a5)
pub fn i_c1a262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A266;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1A266: subq.b #$1, $ac2(a5)
pub fn i_c1a266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A26A;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1A26A: rts
pub fn i_c1a26a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A26C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1A26C: move.w #$3, (a0)+
pub fn i_c1a26c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A270;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C1A270: move.w $9e2(a5), d0
pub fn i_c1a270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A274;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9E2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C1A274: move.w d0, d2
pub fn i_c1a274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A276;
    cpu.ir = 0x3400;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1A276: add.w d0, d0
pub fn i_c1a276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A278;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C1A278: add.w d0, d2
pub fn i_c1a278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A27A;
    cpu.ir = 0xD440;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C1A27A: addi.w #$7068, d2
pub fn i_c1a27a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A27E;
    cpu.ir = 0x642;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x7068, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C1A27E: move.w d2, (a0)+
pub fn i_c1a27e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A280;
    cpu.ir = 0x30C2;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C1A280: move.w #$1108, (a0)+
pub fn i_c1a280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A284;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x1108;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C1A284: move.w d1, (a0)+
pub fn i_c1a284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A286;
    cpu.ir = 0x30C1;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C1A286: rts
pub fn i_c1a286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A288;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1A288: ori.b #$45, d3
pub fn i_c1a288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A28C;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x45) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1A28C: movep.w $5345(a0), d0
pub fn i_c1a28c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A290;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x5345);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C1A290: addq.w #$2, (a4)
pub fn i_c1a290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A292;
    cpu.ir = 0x5454;
    let ad1 = cpu.a[4];
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C1A292: ?
pub fn i_c1a292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A294;
    cpu.ir = 0x494E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1A294: dc.w $4720
pub fn i_c1a294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A296;
    cpu.ir = 0x4720;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1A296: subq.w #$2, (a0)
pub fn i_c1a296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A298;
    cpu.ir = 0x5550;
    let ad1 = cpu.a[0];
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C1A298: movea.l (a4), a0
pub fn i_c1a298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A29A;
    cpu.ir = 0x2054;
    let v2 = bus.read32(cpu.a[4]);
    cpu.a[0] = v2;
    12
}
/// $C1A29A: swap d5
pub fn i_c1a29a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A29C;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1A29C: movea.l d3, a0
pub fn i_c1a29c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A29E;
    cpu.ir = 0x2043;
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C1A29E: ?
pub fn i_c1a29e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A2A0;
    cpu.ir = 0x4142;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1A2A0: ?
pub fn i_c1a2a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A2A2;
    cpu.ir = 0x494E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1A2A2: ?
pub fn i_c1a2a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A2A4;
    cpu.ir = 0x4554;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1A2A4: dc.w $ffff
pub fn i_c1a2a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A2A6;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C1A2A6: ori.b #$a8, d3
pub fn i_c1a2a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A2AA;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0xA8) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1A2AA: movep.w $434f(a0), d0
pub fn i_c1a2aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A2AE;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x434F);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C1A2AE: ?
pub fn i_c1a2ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A2B0;
    cpu.ir = 0x494E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1A2B0: move.w -(a0), -(a0)
pub fn i_c1a2b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A2B2;
    cpu.ir = 0x3120;
    let ad2 = cpu.a[0].wrapping_sub(2); cpu.a[0] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_sub(2); cpu.a[0] = ad4;
    bus.write16(ad4, v3 as u16);
    16
}
/// $C1A2B2: move.l -(a0), d0
pub fn i_c1a2b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A2B4;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C1A2B4: move.l -(a0), d0
pub fn i_c1a2b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A2B6;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C1A2B6: movea.l d3, a0
pub fn i_c1a2b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A2B8;
    cpu.ir = 0x2043;
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C1A2B8: ?
pub fn i_c1a2b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A2BA;
    cpu.ir = 0x4F49;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1A2BA: ?
pub fn i_c1a2ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A2BC;
    cpu.ir = 0x4E20;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1A328: move.b d0, $3a0003.l
pub fn i_c1a328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A32E;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0003, v1 as u8);
    16
}
/// $C1A32E: move.b d0, $3a000b.l
pub fn i_c1a32e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A334;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A000B, v1 as u8);
    16
}
/// $C1A334: moveq #$0, d0
pub fn i_c1a334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A336;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1A336: move.l d0, d1
pub fn i_c1a336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A338;
    cpu.ir = 0x2200;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[1] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1A338: move.l d0, d2
pub fn i_c1a338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A33A;
    cpu.ir = 0x2400;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[2] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1A33A: move.l d0, d3
pub fn i_c1a33a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A33C;
    cpu.ir = 0x2600;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[3] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1A33C: lea.l $110000.l, a0
pub fn i_c1a33c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A342;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x110000;
    20
}
/// $C1A342: move.w #$1ff, d7
pub fn i_c1a342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A346;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x1FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1A346: move.b d0, $300001.l
pub fn i_c1a346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A34C;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C1A34C: movem.l d0-d3, -(a0)
pub fn i_c1a34c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A350;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C1A350: movem.l d0-d3, -(a0)
pub fn i_c1a350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A354;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C1A354: movem.l d0-d3, -(a0)
pub fn i_c1a354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A358;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C1A358: movem.l d0-d3, -(a0)
pub fn i_c1a358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A35C;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C1A35C: movem.l d0-d3, -(a0)
pub fn i_c1a35c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A360;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C1A360: movem.l d0-d3, -(a0)
pub fn i_c1a360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A364;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C1A364: movem.l d0-d3, -(a0)
pub fn i_c1a364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A368;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C1A368: movem.l d0-d3, -(a0)
pub fn i_c1a368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A36C;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C1A36C: dbra d7, $c1a346
pub fn i_c1a36c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A370;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC1A370; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC1A370; return 14; }
    cpu.pc = 0xC1A346;
    10
}
/// $C1A370: move.b d0, $3a000f.l
pub fn i_c1a370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A376;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A000F, v1 as u8);
    16
}
/// $C1A376: lea.l $402000.l, a0
pub fn i_c1a376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A37C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x402000;
    20
}
/// $C1A37C: move.w #$7f, d7
pub fn i_c1a37c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A380;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x7F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1A380: movem.l d0-d3, -(a0)
pub fn i_c1a380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A384;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C1A384: movem.l d0-d3, -(a0)
pub fn i_c1a384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A388;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C1A388: movem.l d0-d3, -(a0)
pub fn i_c1a388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A38C;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C1A38C: movem.l d0-d3, -(a0)
pub fn i_c1a38c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A390;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C1A390: dbra d7, $c1a380
pub fn i_c1a390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A394;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC1A394; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC1A394; return 14; }
    cpu.pc = 0xC1A380;
    10
}
/// $C1A394: move.b d0, $300001.l
pub fn i_c1a394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A39A;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C1A39A: move.b d0, $3a001f.l
pub fn i_c1a39a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A3A0;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001F, v1 as u8);
    16
}
/// $C1A3A0: lea.l $402000.l, a0
pub fn i_c1a3a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A3A6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x402000;
    20
}
/// $C1A3A6: move.w #$7f, d7
pub fn i_c1a3a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A3AA;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x7F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1A3AA: movem.l d0-d3, -(a0)
pub fn i_c1a3aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A3AE;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C1A3AE: movem.l d0-d3, -(a0)
pub fn i_c1a3ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A3B2;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C1A3B2: movem.l d0-d3, -(a0)
pub fn i_c1a3b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A3B6;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C1A3B6: movem.l d0-d3, -(a0)
pub fn i_c1a3b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A3BA;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C1A3BA: dbra d7, $c1a3aa
pub fn i_c1a3ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A3BE;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC1A3BE; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC1A3BE; return 14; }
    cpu.pc = 0xC1A3AA;
    10
}
/// $C1A3BE: move.l #$eee0000, $400002.l
pub fn i_c1a3be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A3C8;
    cpu.ir = 0x23FC;
    let v1: u32 = 0xEEE0000;
    bus.write32(0x400002, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C1A3C8: move.l #$e000000, $400022.l
pub fn i_c1a3c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A3D2;
    cpu.ir = 0x23FC;
    let v1: u32 = 0xE000000;
    bus.write32(0x400022, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C1A3D2: lea.l $10f300.l, a5
pub fn i_c1a3d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A3D8;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x10F300;
    20
}
/// $C1A3D8: jsr $c16fd4.l
pub fn i_c1a3d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A3DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0xC1A3DE);
    cpu.pc = 0xC16FD4;
    16
}
/// $C1A3DE: move.b #$1, $be6(a5)
pub fn i_c1a3de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A3E4;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xBE6);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C1A3E4: move.b #$ff, $b8d(a5)
pub fn i_c1a3e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A3EA;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xB8D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C1A3EA: move.b #$1, $bbc(a5)
pub fn i_c1a3ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A3F0;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xBBC);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C1A3F0: move.b $c00401.l, $a83(a5)
pub fn i_c1a3f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A3F8;
    cpu.ir = 0x1B79;
    let v2 = bus.read8(0xC00401) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[5].wrapping_add(0xA83);
    bus.write8(ad3, v2 as u8);
    24
}
/// $C1A3F8: move.b #$80, $10fd82.l
pub fn i_c1a3f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A400;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10FD82, v1 as u8);
    16
}
/// $C1A400: move.l #$10ff00, $abe(a5)
pub fn i_c1a400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A408;
    cpu.ir = 0x2B7C;
    let v1: u32 = 0x10FF00;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1A408: move.b d0, $3a0001.l
pub fn i_c1a408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A40E;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0001, v1 as u8);
    16
}
/// $C1A40E: move.w #$7, $3c000c.l
pub fn i_c1a40e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A416;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C000C, v1 as u16);
    16
}
/// $C1A416: move.w #$2000, sr
pub fn i_c1a416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A41A;
    cpu.ir = 0x46FC;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let v1: u32 = 0x2000;
    write_sr(cpu, v1 as u16);
    12
}
/// $C1A41A: move.w #$0, $9d8(a5)
pub fn i_c1a41a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A420;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C1A420: jsr $c12a0e.l
pub fn i_c1a420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A426;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0xC1A426);
    cpu.pc = 0xC12A0E;
    16
}
/// $C1A426: addq.b #$1, $ac2(a5)
pub fn i_c1a426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A42A;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1A42A: movea.l $abe(a5), a0
pub fn i_c1a42a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A42E;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C1A42E: move.l #$c1a504, (a0)+
pub fn i_c1a42e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A434;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC1A504;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C1A434: move.l a0, $abe(a5)
pub fn i_c1a434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A438;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1A438: subq.b #$1, $ac2(a5)
pub fn i_c1a438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A43C;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1A43C: jsr $c1727e.l
pub fn i_c1a43c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A442;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0xC1A442);
    cpu.pc = 0xC1727E;
    16
}
/// $C1A442: jsr $c1a536(pc)
pub fn i_c1a442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A446;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1A446);
    cpu.pc = 0xC1A536;
    16
}
/// $C1A446: tst.b d5
pub fn i_c1a446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A448;
    cpu.ir = 0x4A05;
    let v1 = cpu.d[5] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C1A448: beq.b $c1a43c
pub fn i_c1a448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A44A;
    cpu.ir = 0x67F2;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1A43C; return 10; }
    8
}
/// $C1A44A: clr.w $9d8(a5)
pub fn i_c1a44a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A44E;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0x9D8);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $C1A44E: jsr $c12a0e.l
pub fn i_c1a44e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A454;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0xC1A454);
    cpu.pc = 0xC12A0E;
    16
}
/// $C1A454: jsr $c1727e.l
pub fn i_c1a454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A45A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0xC1A45A);
    cpu.pc = 0xC1727E;
    16
}
/// $C1A45A: jsr $c1a4a8(pc)
pub fn i_c1a45a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A45E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1A45E);
    cpu.pc = 0xC1A4A8;
    16
}
/// $C1A45E: jsr $c1a536(pc)
pub fn i_c1a45e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A462;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1A462);
    cpu.pc = 0xC1A536;
    16
}
/// $C1A462: tst.b d5
pub fn i_c1a462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A464;
    cpu.ir = 0x4A05;
    let v1 = cpu.d[5] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C1A464: beq.b $c1a420
pub fn i_c1a464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A466;
    cpu.ir = 0x67BA;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1A420; return 10; }
    8
}
/// $C1A466: move.b $bea(a5), d0
pub fn i_c1a466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A46A;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xBEA);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C1A46A: andi.b #$f0, d0
pub fn i_c1a46a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A46E;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1A46E: cmpi.b #$f0, d0
pub fn i_c1a46e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A472;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xF0, Size::Byte, false);
    8
}
/// $C1A472: beq.w $c1a5e2
pub fn i_c1a472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A476;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1A5E2; return 10; }
    8
}
/// $C1A476: move.b $beb(a5), d0
pub fn i_c1a476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A47A;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xBEB);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C1A47A: andi.b #$3, d0
pub fn i_c1a47a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A47E;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1A47E: beq.b $c1a454
pub fn i_c1a47e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A480;
    cpu.ir = 0x67D4;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1A454; return 10; }
    8
}
/// $C1A480: lea.l $800000.l, a6
pub fn i_c1a480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A486;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0x800000;
    20
}
/// $C1A486: move.w $9d8(a5), d2
pub fn i_c1a486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A48A;
    cpu.ir = 0x342D;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C1A48A: moveq #$1, d1
pub fn i_c1a48a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A48C;
    cpu.ir = 0x7201;
    cpu.d[1] = 0x1;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1A48C: andi.b #$1, d0
pub fn i_c1a48c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A490;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1A490: bne.b $c1a494
pub fn i_c1a490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A492;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1A494; return 10; }
    8
}
/// $C1A492: moveq #$ff, d1
pub fn i_c1a492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A494;
    cpu.ir = 0x72FF;
    cpu.d[1] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1A494: add.b d1, d2
pub fn i_c1a494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A496;
    cpu.ir = 0xD401;
    let v1 = cpu.d[1] & 0xFF;
    let h1 = cpu.d[2] & 0xFF;
    let r = add_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r & 0xFF);
    4
}
/// $C1A496: cmp.w $74(a6), d2
pub fn i_c1a496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A49A;
    cpu.ir = 0xB46E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $C1A49A: bcc.b $c1a494
pub fn i_c1a49a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A49C;
    cpu.ir = 0x64F8;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC1A494; return 10; }
    8
}
/// $C1A49C: move.w d2, $9d8(a5)
pub fn i_c1a49c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A4A0;
    cpu.ir = 0x3B42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C1A4A0: jsr $c12a0e.l
pub fn i_c1a4a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A4A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0xC1A4A6);
    cpu.pc = 0xC12A0E;
    16
}
/// $C1A4A6: bra.b $c1a454
pub fn i_c1a4a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A4A8;
    cpu.ir = 0x60AC;
    cpu.pc = 0xC1A454;
    10
}
/// $C1A4A8: addq.b #$1, $ac2(a5)
pub fn i_c1a4a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A4AC;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1A4AC: movea.l $abe(a5), a0
pub fn i_c1a4ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A4B0;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C1A4B0: move.l #$c1a522, (a0)+
pub fn i_c1a4b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A4B6;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC1A522;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C1A4B6: clr.l (a0)+
pub fn i_c1a4b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A4B8;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C1A4B8: move.w #$3, (a0)+
pub fn i_c1a4b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A4BC;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C1A4BC: move.w #$72c6, (a0)+
pub fn i_c1a4bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A4C0;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x72C6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C1A4C0: move.w #$108, (a0)+
pub fn i_c1a4c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A4C4;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x108;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C1A4C4: move.b $9d9(a5), d0
pub fn i_c1a4c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A4C8;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0x9D9);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C1A4C8: ori.b #$30, d0
pub fn i_c1a4c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A4CC;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x30) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1A4CC: move.b d0, (a0)+
pub fn i_c1a4cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A4CE;
    cpu.ir = 0x10C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C1A4CE: move.b #$ff, (a0)+
pub fn i_c1a4ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A4D2;
    cpu.ir = 0x10FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C1A4D2: clr.w (a0)+
pub fn i_c1a4d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A4D4;
    cpu.ir = 0x4258;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C1A4D4: move.l a0, $abe(a5)
pub fn i_c1a4d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A4D8;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1A4D8: subq.b #$1, $ac2(a5)
pub fn i_c1a4d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A4DC;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1A4DC: jsr $c17290.l
pub fn i_c1a4dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A4E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0xC1A4E2);
    cpu.pc = 0xC17290;
    16
}
/// $C1A4E2: addq.b #$1, $ac2(a5)
pub fn i_c1a4e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A4E6;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1A4E6: movea.l $abe(a5), a0
pub fn i_c1a4e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A4EA;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C1A4EA: clr.l (a0)+
pub fn i_c1a4ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A4EC;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C1A4EC: jsr $c1a57a(pc)
pub fn i_c1a4ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A4F0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1A4F0);
    cpu.pc = 0xC1A57A;
    16
}
/// $C1A4F0: clr.w (a0)+
pub fn i_c1a4f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A4F2;
    cpu.ir = 0x4258;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C1A4F2: move.l a0, $abe(a5)
pub fn i_c1a4f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A4F6;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1A4F6: subq.b #$1, $ac2(a5)
pub fn i_c1a4f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A4FA;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1A4FA: rts
pub fn i_c1a4fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A4FC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1A4FC: trap #$5
pub fn i_c1a4fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A4FE;
    cpu.ir = 0x4E45;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0x94);
    34
}
/// $C1A4FE: dc.w $4f2d
pub fn i_c1a4fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A500;
    cpu.ir = 0x4F2D;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1A500: ?
pub fn i_c1a500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A502;
    cpu.ir = 0x4745;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1A502: chk.w d0, d7
pub fn i_c1a502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A504;
    cpu.ir = 0x4F80;
    let v1 = cpu.d[0] & 0xFFFF;
    let bd = v1 as u16 as i16 as i32; let vv = cpu.d[7] as u16 as i16 as i32;
    cpu.sr.set(StatusRegister::Z, false); cpu.sr.set(StatusRegister::V, false); cpu.sr.set(StatusRegister::C, false);
    if vv < 0 { cpu.sr.set(StatusRegister::N, true); cpu.enter_exception(bus, Exception::Chk); return 40; }
    if vv > bd { cpu.sr.set(StatusRegister::N, false); cpu.enter_exception(bus, Exception::Chk); return 40; }
    cpu.sr.set(StatusRegister::N, false);
    10
}
/// $C1A504: ori.b #$4c, d3
pub fn i_c1a504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A508;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x4C) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1A508: movep.w $494e(a0), d0
pub fn i_c1a508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A50C;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x494E);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C1A50C: subq.w #$1, d5
pub fn i_c1a50c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A50E;
    cpu.ir = 0x5345;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C1A50E: addq.w #$1, (a4)
pub fn i_c1a50e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A510;
    cpu.ir = 0x5254;
    let ad1 = cpu.a[4];
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C1A510: move.l $4d45(a1, d4.l * 4), d0
pub fn i_c1a510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A514;
    cpu.ir = 0x2031;
    let ad2 = cpu.a[1].wrapping_add(0x20).wrapping_add(cpu.d[4]);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $C1A514: ?
pub fn i_c1a514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A516;
    cpu.ir = 0x4D45;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1A516: ?
pub fn i_c1a516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A518;
    cpu.ir = 0x4D4F;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1A518: addq.w #$1, (a1)+
pub fn i_c1a518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A51A;
    cpu.ir = 0x5259;
    let ad1 = cpu.a[1]; cpu.a[1] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C1A51A: movea.l d3, a0
pub fn i_c1a51a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A51C;
    cpu.ir = 0x2043;
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C1A51C: ?
pub fn i_c1a51c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A51E;
    cpu.ir = 0x4152;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1A51E: ?
pub fn i_c1a51e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A520;
    cpu.ir = 0x44FF;
    let v1: u32 = 0x0;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | (v1 as u16 & 0x1F);
    12
}
/// $C1A520: ori.b #$3, d0
pub fn i_c1a520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A524;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1A522: ori.b #$46, d3
pub fn i_c1a522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A526;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x46) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1A526: movep.w $4341(a0), d0
pub fn i_c1a526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A52A;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x4341);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C1A52A: ?
pub fn i_c1a52a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A52C;
    cpu.ir = 0x4249;
    let v1 = cpu.a[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.a[1] = r2 as i16 as i32 as u32;
    4
}
/// $C1A52C: trap #$5
pub fn i_c1a52c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A52E;
    cpu.ir = 0x4E45;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0x94);
    34
}
/// $C1A52E: addq.b #$2, -(a0)
pub fn i_c1a52e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A530;
    cpu.ir = 0x5420;
    let ad1 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    10
}
/// $C1A530: move usp, a7
pub fn i_c1a530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A532;
    cpu.ir = 0x4E6F;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    cpu.a[7] = cpu.usp;
    4
}
/// $C1A532: dc.w $ffff
pub fn i_c1a532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A534;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C1A534: ori.b #$0, d0
pub fn i_c1a534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A538;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1A536: moveq #$0, d5
pub fn i_c1a536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A538;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1A538: move.b $380000.l, d0
pub fn i_c1a538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A53E;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x380000) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C1A53E: andi.b #$30, d0
pub fn i_c1a53e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A542;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x30) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1A542: beq.b $c1a546
pub fn i_c1a542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A544;
    cpu.ir = 0x6702;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1A546; return 10; }
    8
}
/// $C1A544: rts
pub fn i_c1a544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A546;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1A546: lea.l $800000.l, a6
pub fn i_c1a546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A54C;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0x800000;
    20
}
/// $C1A54C: lea.l $40(a6), a0
pub fn i_c1a54c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A550;
    cpu.ir = 0x41EE;
    let ad1 = cpu.a[6].wrapping_add(0x40);
    cpu.a[0] = ad1;
    16
}
/// $C1A550: lea.l $c1a4fc(pc), a1
pub fn i_c1a550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A554;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0xC1A4FC;
    16
}
/// $C1A554: moveq #$7, d7
pub fn i_c1a554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A556;
    cpu.ir = 0x7E07;
    cpu.d[7] = 0x7;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1A556: move.w (a0), d0
pub fn i_c1a556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A558;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C1A558: cmp.b (a1)+, d0
pub fn i_c1a558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A55A;
    cpu.ir = 0xB019;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    8
}
/// $C1A55A: bne.b $c1a544
pub fn i_c1a55a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A55C;
    cpu.ir = 0x66E8;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1A544; return 10; }
    8
}
/// $C1A55C: addq.l #$4, a0
pub fn i_c1a55c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A55E;
    cpu.ir = 0x5888;
    cpu.a[0] = cpu.a[0].wrapping_add(4);
    8
}
/// $C1A55E: dbra d7, $c1a556
pub fn i_c1a55e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A562;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC1A562; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC1A562; return 14; }
    cpu.pc = 0xC1A556;
    10
}
/// $C1A562: move.w $10(a6), d0
pub fn i_c1a562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A566;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C1A566: andi.b #$f0, d0
pub fn i_c1a566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A56A;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1A56A: cmpi.b #$20, d0
pub fn i_c1a56a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A56E;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x20, Size::Byte, false);
    8
}
/// $C1A56E: bne.b $c1a544
pub fn i_c1a56e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A570;
    cpu.ir = 0x66D4;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1A544; return 10; }
    8
}
/// $C1A570: tst.w $74(a6)
pub fn i_c1a570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A574;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $C1A574: beq.b $c1a544
pub fn i_c1a574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A576;
    cpu.ir = 0x67CE;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1A544; return 10; }
    8
}
/// $C1A576: moveq #$1, d5
pub fn i_c1a576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A578;
    cpu.ir = 0x7A01;
    cpu.d[5] = 0x1;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1A578: rts
pub fn i_c1a578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A57A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1A57A: lea.l $800100.l, a6
pub fn i_c1a57a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A580;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0x800100;
    20
}
/// $C1A580: move.w $9d8(a5), d0
pub fn i_c1a580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A584;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C1A584: move.w #$2800, d1
pub fn i_c1a584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A588;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2800;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1A588: mulu.w d0, d1
pub fn i_c1a588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A58A;
    cpu.ir = 0xC2C0;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[1] & 0xFFFF);
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $C1A58A: adda.l d1, a6
pub fn i_c1a58a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A58C;
    cpu.ir = 0xDDC1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.a[6] = cpu.a[6].wrapping_add(v1);
    8
}
/// $C1A58C: move.l #$370ca, (a0)+
pub fn i_c1a58c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A592;
    cpu.ir = 0x20FC;
    let v1: u32 = 0x370CA;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C1A592: moveq #$0, d5
pub fn i_c1a592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A594;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1A594: tst.w $12(a6, d5.w)
pub fn i_c1a594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A598;
    cpu.ir = 0x4A76;
    let ad2 = cpu.a[6].wrapping_add(0x12).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    14
}
/// $C1A598: beq.w $c1a5d8
pub fn i_c1a598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A59C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1A5D8; return 10; }
    8
}
/// $C1A59C: move.l #$50001, (a0)+
pub fn i_c1a59c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5A2;
    cpu.ir = 0x20FC;
    let v1: u32 = 0x50001;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C1A5A2: move.w #$108, (a0)+
pub fn i_c1a5a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5A6;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x108;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C1A5A6: move.l #$534c4f54, (a0)+
pub fn i_c1a5a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5AC;
    cpu.ir = 0x20FC;
    let v1: u32 = 0x534C4F54;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C1A5AC: move.w d5, d0
pub fn i_c1a5ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5AE;
    cpu.ir = 0x3005;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1A5AE: lsr.w #$2, d0
pub fn i_c1a5ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5B0;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $C1A5B0: addi.b #$31, d0
pub fn i_c1a5b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5B4;
    cpu.ir = 0x600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x31, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1A5B4: move.b d0, (a0)+
pub fn i_c1a5b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5B6;
    cpu.ir = 0x10C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C1A5B6: move.b #$20, (a0)+
pub fn i_c1a5b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5BA;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C1A5BA: move.w $14(a6, d5.w), d0
pub fn i_c1a5ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5BE;
    cpu.ir = 0x3036;
    let ad2 = cpu.a[6].wrapping_add(0x14).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C1A5BE: andi.w #$ff, d0
pub fn i_c1a5be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5C2;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C1A5C2: lsl.w #$4, d0
pub fn i_c1a5c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5C4;
    cpu.ir = 0xE948;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $C1A5C4: addi.w #$f2, d0
pub fn i_c1a5c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5C8;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xF2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C1A5C8: lea.l (a6, d0.w), a1
pub fn i_c1a5c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5CC;
    cpu.ir = 0x43F6;
    let ad1 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    cpu.a[1] = ad1;
    18
}
/// $C1A5CC: move.l (a1)+, (a0)+
pub fn i_c1a5cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5CE;
    cpu.ir = 0x20D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C1A5CE: move.l (a1)+, (a0)+
pub fn i_c1a5ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5D0;
    cpu.ir = 0x20D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C1A5D0: move.l (a1)+, (a0)+
pub fn i_c1a5d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5D2;
    cpu.ir = 0x20D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C1A5D2: move.l (a1)+, (a0)+
pub fn i_c1a5d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5D4;
    cpu.ir = 0x20D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C1A5D4: move.w #$ff00, (a0)+
pub fn i_c1a5d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5D8;
    cpu.ir = 0x30FC;
    let v1: u32 = 0xFF00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C1A5D8: addq.b #$4, d5
pub fn i_c1a5d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5DA;
    cpu.ir = 0x5805;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C1A5DA: cmpi.w #$20, d5
pub fn i_c1a5da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5DE;
    cpu.ir = 0xC45;
    let v1 = cpu.d[5] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x20, Size::Word, false);
    8
}
/// $C1A5DE: bne.b $c1a594
pub fn i_c1a5de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5E0;
    cpu.ir = 0x66B4;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1A594; return 10; }
    8
}
/// $C1A5E0: rts
pub fn i_c1a5e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5E2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1A5E2: lea.l $d08000.l, a4
pub fn i_c1a5e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5E8;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0xD08000;
    20
}
/// $C1A5E8: move.w #$2700, sr
pub fn i_c1a5e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5EC;
    cpu.ir = 0x46FC;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let v1: u32 = 0x2700;
    write_sr(cpu, v1 as u16);
    12
}
/// $C1A5EC: move.b d0, $3a001d.l
pub fn i_c1a5ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5F2;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001D, v1 as u8);
    16
}
/// $C1A5F2: move.w $9d8(a5), d0
pub fn i_c1a5f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5F6;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C1A5F6: move.w #$2800, d1
pub fn i_c1a5f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5FA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2800;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1A5FA: mulu.w d0, d1
pub fn i_c1a5fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5FC;
    cpu.ir = 0xC2C0;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[1] & 0xFFFF);
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $C1A5FC: lea.l $800100.l, a0
pub fn i_c1a5fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A602;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x800100;
    20
}
/// $C1A602: adda.l d1, a0
pub fn i_c1a602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A604;
    cpu.ir = 0xD1C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add(v1);
    8
}
/// $C1A604: lea.l $300001.l, a3
pub fn i_c1a604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A60A;
    cpu.ir = 0x47F9;
    cpu.a[3] = 0x300001;
    20
}
/// $C1A60A: move.l $800070.l, -$7e54(a4)
pub fn i_c1a60a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A612;
    cpu.ir = 0x2979;
    let v2 = bus.read32(0x800070);
    let ad3 = cpu.a[4].wrapping_add(0xFFFF81AC);
    bus.write32(ad3, v2);
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    32
}
/// $C1A612: move.w $9d8(a5), -$7e50(a4)
pub fn i_c1a612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A618;
    cpu.ir = 0x396D;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_add(0xFFFF81B0);
    bus.write16(ad4, v3 as u16);
    20
}
/// $C1A618: move.l (a0)+, -$7efc(a4)
pub fn i_c1a618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A61C;
    cpu.ir = 0x2958;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[4].wrapping_add(0xFFFF8104);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C1A61C: move.l (a0)+, -$7ef8(a4)
pub fn i_c1a61c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A620;
    cpu.ir = 0x2958;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[4].wrapping_add(0xFFFF8108);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C1A620: move.l #$25, d7
pub fn i_c1a620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A626;
    cpu.ir = 0x2E3C;
    let v1: u32 = 0x25;
    cpu.d[7] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1A626: lea.l -$7ee6(a4), a1
pub fn i_c1a626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A62A;
    cpu.ir = 0x43EC;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF811A);
    cpu.a[1] = ad1;
    16
}
/// $C1A62A: jsr $c1a688(pc)
pub fn i_c1a62a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A62E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1A62E);
    cpu.pc = 0xC1A688;
    16
}
/// $C1A62E: move.l #$90, d7
pub fn i_c1a62e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A634;
    cpu.ir = 0x2E3C;
    let v1: u32 = 0x90;
    cpu.d[7] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1A634: lea.l -$7e00(a4), a1
pub fn i_c1a634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A638;
    cpu.ir = 0x43EC;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF8200);
    cpu.a[1] = ad1;
    16
}
/// $C1A638: jsr $c1a688(pc)
pub fn i_c1a638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A63C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1A63C);
    cpu.pc = 0xC1A688;
    16
}
/// $C1A63C: move.l #$10f0, d7
pub fn i_c1a63c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A642;
    cpu.ir = 0x2E3C;
    let v1: u32 = 0x10F0;
    cpu.d[7] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1A642: lea.l $320(a4), a1
pub fn i_c1a642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A646;
    cpu.ir = 0x43EC;
    let ad1 = cpu.a[4].wrapping_add(0x320);
    cpu.a[1] = ad1;
    16
}
/// $C1A646: jsr $c1a688(pc)
pub fn i_c1a646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A64A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1A64A);
    cpu.pc = 0xC1A688;
    16
}
/// $C1A64A: lea.l -$7edc(a4), a0
pub fn i_c1a64a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A64E;
    cpu.ir = 0x41EC;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF8124);
    cpu.a[0] = ad1;
    16
}
/// $C1A64E: moveq #$0, d0
pub fn i_c1a64e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A650;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1A650: tst.w (a0, d0.w)
pub fn i_c1a650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A654;
    cpu.ir = 0x4A70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    14
}
/// $C1A654: beq.w $c1a65e
pub fn i_c1a654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A658;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1A65E; return 10; }
    8
}
/// $C1A658: move.b #$80, $2(a0, d0.w)
pub fn i_c1a658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A65E;
    cpu.ir = 0x11BC;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $C1A65E: addq.w #$4, d0
pub fn i_c1a65e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A660;
    cpu.ir = 0x5840;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C1A660: cmpi.w #$20, d0
pub fn i_c1a660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A664;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x20, Size::Word, false);
    8
}
/// $C1A664: bne.b $c1a650
pub fn i_c1a664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A666;
    cpu.ir = 0x66EA;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1A650; return 10; }
    8
}
/// $C1A666: move.b d0, $3a000d.l
pub fn i_c1a666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A66C;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A000D, v1 as u8);
    16
}
/// $C1A66C: movea.l $abe(a5), a0
pub fn i_c1a66c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A670;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C1A670: move.l #$c1a692, (a0)+
pub fn i_c1a670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A676;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC1A692;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C1A676: move.l a0, $abe(a5)
pub fn i_c1a676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A67A;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1A67A: jsr $c17290.l
pub fn i_c1a67a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A680;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0xC1A680);
    cpu.pc = 0xC17290;
    16
}
/// $C1A680: move.b d0, $300001.l
pub fn i_c1a680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A686;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C1A686: bra.b $c1a680
pub fn i_c1a686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A688;
    cpu.ir = 0x60F8;
    cpu.pc = 0xC1A680;
    10
}
/// $C1A688: move.b d0, (a3)
pub fn i_c1a688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A68A;
    cpu.ir = 0x1680;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[3], v1 as u8);
    8
}
/// $C1A68A: move.w (a0)+, (a1)+
pub fn i_c1a68a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A68C;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $C1A68C: subq.l #$1, d7
pub fn i_c1a68c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A68E;
    cpu.ir = 0x5387;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Long, true);
    cpu.d[7] = r2;
    4
}
/// $C1A68E: bne.b $c1a688
pub fn i_c1a68e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A690;
    cpu.ir = 0x66F8;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1A688; return 10; }
    8
}
/// $C1A690: rts
pub fn i_c1a690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A692;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1A692: ori.b #$5a, d3
pub fn i_c1a692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A696;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x5A) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1A696: movep.w $5245(a0), d0
pub fn i_c1a696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A69A;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x5245);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C1A69A: ?
pub fn i_c1a69a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A69C;
    cpu.ir = 0x434F;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1A69C: addq.w #$3, d5
pub fn i_c1a69c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A69E;
    cpu.ir = 0x5645;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 3, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C1A69E: addq.b #$1, -(a0)
pub fn i_c1a69e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A6A0;
    cpu.ir = 0x5220;
    let ad1 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    10
}
/// $C1A6A0: ?
pub fn i_c1a6a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A6A2;
    cpu.ir = 0x4F4B;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1A6A2: dc.w $ffff
pub fn i_c1a6a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A6A4;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C1A6A4: ori.b #$ff, d0
pub fn i_c1a6a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A6A8;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0xFF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1A6A8: dc.w $ffff
pub fn i_c1a6a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A6AA;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C1A6AA: dc.w $ffff
pub fn i_c1a6aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A6AC;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C1A6AC: dc.w $ffff
pub fn i_c1a6ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A6AE;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C1A6AE: dc.w $ffff
pub fn i_c1a6ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A6B0;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C1A6B0: dc.w $ffff
pub fn i_c1a6b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A6B2;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C1A6B2: dc.w $ffff
pub fn i_c1a6b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A6B4;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C1F800: btst.l d0, d1
pub fn i_c1f800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1F802;
    cpu.ir = 0x101;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C1F802: btst.l d0, d2
pub fn i_c1f802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1F804;
    cpu.ir = 0x102;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C1F804: btst.l d0, d1
pub fn i_c1f804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1F806;
    cpu.ir = 0x101;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C1F806: btst.l d0, d3
pub fn i_c1f806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1F808;
    cpu.ir = 0x103;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C1F808: ori.b #$3b, d0
pub fn i_c1f808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1F80C;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x3B) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1F80C: ori.b #$1, d0
pub fn i_c1f80c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1F810;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1F80E: ori.b #$3, d1
pub fn i_c1f80e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1F812;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1F810: andi.b #$5, d3
pub fn i_c1f810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1F814;
    cpu.ir = 0x203;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 & 0x5) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1F812: subi.b #$7, d5
pub fn i_c1f812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1F816;
    cpu.ir = 0x405;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x7, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1F814: addi.b #$9, d7
pub fn i_c1f814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1F818;
    cpu.ir = 0x607;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x9, Size::Byte, true);
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1F816: ?
pub fn i_c1f816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1F81A;
    cpu.ir = 0x809;
    let mk: u32 = 1u32 << ((11) % 32);
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C1F818: ?
pub fn i_c1f818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1F81C;
    cpu.ir = 0xA0B;
    let v1 = cpu.a[3] & 0xFF;
    let r2 = (v1 ^ 0xD) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.a[3] = r2;
    8
}
/// $C1F81A: ?
pub fn i_c1f81a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1F81E;
    cpu.ir = 0xC0D;
    let v1 = cpu.a[5] & 0xFF;
    sub_with_flags(cpu, v1, 0xF, Size::Byte, false);
    8
}
/// $C1F81C: ?
pub fn i_c1f81c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1F820;
    cpu.ir = 0xE0F;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1F81E: move.b (a1), d0
pub fn i_c1f81e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1F820;
    cpu.ir = 0x1011;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C1F820: move.b (a3), d1
pub fn i_c1f820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1F822;
    cpu.ir = 0x1213;
    let v2 = bus.read8(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C1F822: move.b (a5), d2
pub fn i_c1f822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1F824;
    cpu.ir = 0x1415;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C1F824: move.b (a7), d3
pub fn i_c1f824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1F826;
    cpu.ir = 0x1617;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C1F826: move.b (a1)+, d4
pub fn i_c1f826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1F828;
    cpu.ir = 0x1819;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C1F828: move.b (a3)+, d5
pub fn i_c1f828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1F82A;
    cpu.ir = 0x1A1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C1F82A: move.b (a5)+, d6
pub fn i_c1f82a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1F82C;
    cpu.ir = 0x1C1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C1F82C: move.b (a7)+, d7
pub fn i_c1f82c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1F82E;
    cpu.ir = 0x1E1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C1F82E: move.l -(a1), d0
pub fn i_c1f82e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1F830;
    cpu.ir = 0x2021;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C1F830: move.l -(a3), d1
pub fn i_c1f830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1F832;
    cpu.ir = 0x2223;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C1F832: move.l -(a5), d2
pub fn i_c1f832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1F834;
    cpu.ir = 0x2425;
    let ad2 = cpu.a[5].wrapping_sub(4); cpu.a[5] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C1F834: move.l -(a7), d3
pub fn i_c1f834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1F836;
    cpu.ir = 0x2627;
    let ad2 = cpu.a[7].wrapping_sub(4); cpu.a[7] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C1F836: move.l $2a2b(a1), d4
pub fn i_c1f836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1F83A;
    cpu.ir = 0x2829;
    let ad2 = cpu.a[1].wrapping_add(0x2A2B);
    let v3 = bus.read32(ad2);
    cpu.d[4] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1F83A: move.l $2e2f(a5), d6
pub fn i_c1f83a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1F83E;
    cpu.ir = 0x2C2D;
    let ad2 = cpu.a[5].wrapping_add(0x2E2F);
    let v3 = bus.read32(ad2);
    cpu.d[6] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1F83E: ori.b #$0, d0
pub fn i_c1f83e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1F842;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1F842: ?
pub fn i_c1f842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1F844;
    cpu.ir = 0xFFF;
    let mk: u32 = 1u32 << ((cpu.d[7]) % 8);
    let v1: u32 = 0x0;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 | mk;
    8
}
/// $C1F844: ?
pub fn i_c1f844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1F846;
    cpu.ir = 0xFFF;
    let mk: u32 = 1u32 << ((cpu.d[7]) % 8);
    let v1: u32 = 0x0;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 | mk;
    8
}
/// $C1F846: ?
pub fn i_c1f846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1F848;
    cpu.ir = 0xFFF;
    let mk: u32 = 1u32 << ((cpu.d[7]) % 8);
    let v1: u32 = 0x0;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 | mk;
    8
}
/// $C1F848: ?
pub fn i_c1f848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1F84A;
    cpu.ir = 0xFFF;
    let mk: u32 = 1u32 << ((cpu.d[7]) % 8);
    let v1: u32 = 0x0;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 | mk;
    8
}
/// $C1F84A: moveq #$0, d0
pub fn i_c1f84a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1F84C;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1FA7E: dc.w $ffff
pub fn i_c1fa7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FA80;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C1FA80: dc.w $ffff
pub fn i_c1fa80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FA82;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C1FA82: dc.w $ff1c
pub fn i_c1fa82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FA84;
    cpu.ir = 0xFF1C;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C1FAC6: dc.w $ffff
pub fn i_c1fac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FAC8;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C1FAC8: dc.w $ffff
pub fn i_c1fac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FACA;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C1FACA: dc.w $ffff
pub fn i_c1faca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FACC;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C1FACC: dc.w $ffff
pub fn i_c1facc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FACE;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C1FACE: ?
pub fn i_c1face(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FAD2;
    cpu.ir = 0xE;
    let v1 = cpu.a[6] & 0xFF;
    let r2 = (v1 | 0x2C) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.a[6] = r2;
    8
}
/// $C1FAD2: movep.w $1e2d(a7), d0
pub fn i_c1fad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FAD6;
    cpu.ir = 0x10F;
    let a = cpu.a[7].wrapping_add(0x1E2D);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C1FAD6: andi.b #$2e, (a0)
pub fn i_c1fad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FADA;
    cpu.ir = 0x210;
    let ad1 = cpu.a[0];
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0x2E) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1FADA: btst.l d1, (a1)
pub fn i_c1fada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FADC;
    cpu.ir = 0x311;
    let mk: u32 = 1u32 << ((cpu.d[1]) % 8);
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    12
}
/// $C1FADC: move.l $412(a7), d0
pub fn i_c1fadc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FAE0;
    cpu.ir = 0x202F;
    let ad2 = cpu.a[7].wrapping_add(0x412);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1FAE0: move.l ([a0, d0.w * 4], $22310614), -(a0)
pub fn i_c1fae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FAE4;
    cpu.ir = 0x2130;
    let ad2 = cpu.a[0].wrapping_add(0x13).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[0];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[0] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $C1FAE4: move.l $14(a1, d0.w), d1
pub fn i_c1fae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FAE8;
    cpu.ir = 0x2231;
    let ad2 = cpu.a[1].wrapping_add(0x14).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $C1FAE8: move.l ([a2], d0.w * 8), -(a1)
pub fn i_c1fae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FAEC;
    cpu.ir = 0x2332;
    let ad2 = cpu.a[2].wrapping_add(0x15).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[1];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[1] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $C1FAEC: move.l $16(a3, d0.l), d2
pub fn i_c1faec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FAF0;
    cpu.ir = 0x2433;
    let ad2 = cpu.a[3].wrapping_add(0x16).wrapping_add(cpu.d[0]);
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $C1FAF0: move.l ([a4], d0.l, $26350a18), -(a2)
pub fn i_c1faf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FAF4;
    cpu.ir = 0x2534;
    let ad2 = cpu.a[4].wrapping_add(0x17).wrapping_add(cpu.d[0]);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[2];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[2] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $C1FAF4: move.l $18(a5, d0.l), d3
pub fn i_c1faf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FAF8;
    cpu.ir = 0x2635;
    let ad2 = cpu.a[5].wrapping_add(0x18).wrapping_add(cpu.d[0]);
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $C1FAF8: move.l ([a6, d0.l * 2]), -(a3)
pub fn i_c1faf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FAFC;
    cpu.ir = 0x2736;
    let ad2 = cpu.a[6].wrapping_add(0x19).wrapping_add(cpu.d[0]);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[3];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[3] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $C1FAFC: move.l $1a(a7, d0.l), d4
pub fn i_c1fafc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FB00;
    cpu.ir = 0x2837;
    let ad2 = cpu.a[7].wrapping_add(0x1A).wrapping_add(cpu.d[0]);
    let v3 = bus.read32(ad2);
    cpu.d[4] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $C1FB00: move.l $d1b.w, -(a4)
pub fn i_c1fb00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FB04;
    cpu.ir = 0x2938;
    let v2 = bus.read32(0xD1B);
    let o3 = cpu.a[4];
    bus.write16(o3.wrapping_sub(2), v2 as u16);
    cpu.a[4] = o3.wrapping_sub(4);
    bus.write16(o3.wrapping_sub(4), (v2 >> 16) as u16);
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C1FB04: move.l $ff1c2bff.l, d5
pub fn i_c1fb04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FB0A;
    cpu.ir = 0x2A39;
    let v2 = bus.read32(0xFF1C2BFF);
    cpu.d[5] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C1FB0A: dc.w $ffff
pub fn i_c1fb0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FB0C;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C1FB0C: dc.w $ffff
pub fn i_c1fb0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FB0E;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C1FB0E: btst.l d0, d1
pub fn i_c1fb0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FB10;
    cpu.ir = 0x101;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C1FB10: dc.w $f20a
pub fn i_c1fb10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FB12;
    cpu.ir = 0xF20A;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C1FB12: move.l d2, d0
pub fn i_c1fb12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FB14;
    cpu.ir = 0x2002;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1FB14: ori.b #$f6, d3
pub fn i_c1fb14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FB18;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0xF6) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1FB18: ori.b #$c1, d4
pub fn i_c1fb18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FB1C;
    cpu.ir = 0x4;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 | 0xC1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1FB1C: dc.w $fb2c
pub fn i_c1fb1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FB1E;
    cpu.ir = 0xFB2C;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C1FB1E: ori.b #$1, d5
pub fn i_c1fb1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FB22;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1FB22: ori.b #$5, d6
pub fn i_c1fb22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FB26;
    cpu.ir = 0x6;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = (v1 | 0x5) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1FB26: ori.b #$6, d1
pub fn i_c1fb26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FB2A;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x6) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1FB2A: ori.b #$1, d0
pub fn i_c1fb2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FB2E;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1FB2C: ori.b #$3, d1
pub fn i_c1fb2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FB30;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1FB2E: andi.b #$5, d3
pub fn i_c1fb2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FB32;
    cpu.ir = 0x203;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 & 0x5) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1FB30: subi.b #$7, d5
pub fn i_c1fb30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FB34;
    cpu.ir = 0x405;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x7, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1FB32: addi.b #$9, d7
pub fn i_c1fb32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FB36;
    cpu.ir = 0x607;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x9, Size::Byte, true);
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1FB34: ?
pub fn i_c1fb34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FB38;
    cpu.ir = 0x809;
    let mk: u32 = 1u32 << ((11) % 32);
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C1FB36: ?
pub fn i_c1fb36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FB3A;
    cpu.ir = 0xA0B;
    let v1 = cpu.a[3] & 0xFF;
    let r2 = (v1 ^ 0xD) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.a[3] = r2;
    8
}
/// $C1FB38: ?
pub fn i_c1fb38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FB3C;
    cpu.ir = 0xC0D;
    let v1 = cpu.a[5] & 0xFF;
    sub_with_flags(cpu, v1, 0xF, Size::Byte, false);
    8
}
/// $C1FB3A: ?
pub fn i_c1fb3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FB3E;
    cpu.ir = 0xE0F;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1FB3C: move.b (a5), d2
pub fn i_c1fb3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FB3E;
    cpu.ir = 0x1415;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C1FB3E: move.b (a7), d3
pub fn i_c1fb3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FB40;
    cpu.ir = 0x1617;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C1FB40: move.b (a1)+, d4
pub fn i_c1fb40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FB42;
    cpu.ir = 0x1819;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C1FB42: move.b (a3)+, d5
pub fn i_c1fb42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FB44;
    cpu.ir = 0x1A1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C1FB44: move.b (a5)+, d6
pub fn i_c1fb44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FB46;
    cpu.ir = 0x1C1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C1FB46: move.b (a7)+, d7
pub fn i_c1fb46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FB48;
    cpu.ir = 0x1E1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C1FB48: negx.w (a6)+
pub fn i_c1fb48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FB4A;
    cpu.ir = 0x405E;
    let ad1 = cpu.a[6]; cpu.a[6] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r3 = 0u32.wrapping_sub(v2).wrapping_sub(x) & 0xFFFF;
    let bw = v2 != 0 || x != 0; let vf = (v2 & 0x8000) != 0 && (r3 & 0x8000) != 0;
    cpu.sr.set(StatusRegister::N, r3 & 0x8000 != 0); if r3 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C1FB4A: dc.w $f005
pub fn i_c1fb4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FB4C;
    cpu.ir = 0xF005;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C1FB4C: dc.w $f006
pub fn i_c1fb4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FB4E;
    cpu.ir = 0xF006;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C1FB4E: dc.w $f007
pub fn i_c1fb4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FB50;
    cpu.ir = 0xF007;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C1FB50: dc.w $f008
pub fn i_c1fb50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FB52;
    cpu.ir = 0xF008;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C1FB52: dc.w $f009
pub fn i_c1fb52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FB54;
    cpu.ir = 0xF009;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C1FB54: dc.w $f00a
pub fn i_c1fb54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FB56;
    cpu.ir = 0xF00A;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C1FB56: dc.w $f00b
pub fn i_c1fb56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FB58;
    cpu.ir = 0xF00B;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C1FB58: dc.w $f00c
pub fn i_c1fb58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FB5A;
    cpu.ir = 0xF00C;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C1FB5A: dc.w $f00d
pub fn i_c1fb5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FB5C;
    cpu.ir = 0xF00D;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C1FBCA: ori.b #$0, d0
pub fn i_c1fbca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FBCE;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1FBCE: btst.l d7, -(a1)
pub fn i_c1fbce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FBD0;
    cpu.ir = 0xF21;
    let mk: u32 = 1u32 << ((cpu.d[7]) % 8);
    let ad2 = cpu.a[1].wrapping_sub(1); cpu.a[1] = ad2;
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    14
}
/// $C1FBD0: moveq #$10, d6
pub fn i_c1fbd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FBD2;
    cpu.ir = 0x7C10;
    cpu.d[6] = 0x10;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1FBD2: moveq #$ee, d7
pub fn i_c1fbd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FBD4;
    cpu.ir = 0x7EEE;
    cpu.d[7] = 0xFFFFFFEE;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1FBD4: move.l $202e(a6), d0
pub fn i_c1fbd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FBD8;
    cpu.ir = 0x202E;
    let ad2 = cpu.a[6].wrapping_add(0x202E);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1FBD8: move.l $202e(a6), d0
pub fn i_c1fbd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FBDC;
    cpu.ir = 0x202E;
    let ad2 = cpu.a[6].wrapping_add(0x202E);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1FBDC: move.l $202e(a6), d0
pub fn i_c1fbdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FBE0;
    cpu.ir = 0x202E;
    let ad2 = cpu.a[6].wrapping_add(0x202E);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1FBE0: move.l $202e(a6), d0
pub fn i_c1fbe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FBE4;
    cpu.ir = 0x202E;
    let ad2 = cpu.a[6].wrapping_add(0x202E);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1FD8A: ori.b #$e0, d0
pub fn i_c1fd8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FD8E;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0xE0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1FD8E: bset.b d7, d0
pub fn i_c1fd8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FD90;
    cpu.ir = 0xFC0;
    let mk: u32 = 1u32 << ((cpu.d[7]) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 | mk;
    cpu.d[0] = r2;
    8
}
/// $C1FD90: bclr.b d6, -(a0)
pub fn i_c1fd90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FD92;
    cpu.ir = 0xDA0;
    let mk: u32 = 1u32 << ((cpu.d[6]) % 8);
    let ad1 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad1;
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    14
}
/// $C1FD92: bclr.b d5, d0
pub fn i_c1fd92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FD94;
    cpu.ir = 0xB80;
    let mk: u32 = 1u32 << ((cpu.d[5]) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 & !mk;
    cpu.d[0] = r2;
    8
}
/// $C1FD94: bchg.b d4, (a0)
pub fn i_c1fd94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FD96;
    cpu.ir = 0x950;
    let mk: u32 = 1u32 << ((cpu.d[4]) % 8);
    let ad1 = cpu.a[0];
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 ^ mk;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1FD96: ?
pub fn i_c1fd96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FD98;
    cpu.ir = 0xEEF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1FD98: bclr.b d4, (a1)+
pub fn i_c1fd98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FD9A;
    cpu.ir = 0x999;
    let mk: u32 = 1u32 << ((cpu.d[4]) % 8);
    let ad1 = cpu.a[1]; cpu.a[1] = ad1.wrapping_add(1);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1FD9A: addi.w #$d00, -(a6)
pub fn i_c1fd9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FD9E;
    cpu.ir = 0x666;
    let ad1 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad1;
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xD00, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    14
}
/// $C1FD9E: btst.l d5, d0
pub fn i_c1fd9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FDA0;
    cpu.ir = 0xB00;
    let mk: u32 = 1u32 << ((cpu.d[5]) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C1FDA0: bset.b d2, d0
pub fn i_c1fda0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FDA2;
    cpu.ir = 0x5C0;
    let mk: u32 = 1u32 << ((cpu.d[2]) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 | mk;
    cpu.d[0] = r2;
    8
}
/// $C1FDA2: ori.l #$2e000a, d0
pub fn i_c1fda2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FDA8;
    cpu.ir = 0x80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 | 0x2E000A) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $C1FDA8: ori.b #$2, d0
pub fn i_c1fda8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FDAC;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1FDAA: move.l d2, d0
pub fn i_c1fdaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FDAC;
    cpu.ir = 0x2002;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1FDAC: ori.b #$67, d3
pub fn i_c1fdac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FDB0;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x67) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1FDB0: movep.w $1b01(a5), d0
pub fn i_c1fdb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FDB4;
    cpu.ir = 0x10D;
    let a = cpu.a[5].wrapping_add(0x1B01);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C1FDB4: ori.b #$a1, d5
pub fn i_c1fdb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FDB8;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0xA1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1FDB8: movep.w $1b02(a5), d3
pub fn i_c1fdb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FDBC;
    cpu.ir = 0x70D;
    let a = cpu.a[5].wrapping_add(0x1B02);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[3] = (cpu.d[3] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C1FDBC: ori.b #$1, d5
pub fn i_c1fdbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FDC0;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1FDC0: movep.w $1b09(a5), d3
pub fn i_c1fdc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FDC4;
    cpu.ir = 0x70D;
    let a = cpu.a[5].wrapping_add(0x1B09);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[3] = (cpu.d[3] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C1FDC4: ori.b #$e1, d5
pub fn i_c1fdc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FDC8;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0xE1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1FDC8: movep.w $1b10(a5), d4
pub fn i_c1fdc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FDCC;
    cpu.ir = 0x90D;
    let a = cpu.a[5].wrapping_add(0x1B10);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[4] = (cpu.d[4] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C1FDCC: ori.b #$1, d5
pub fn i_c1fdcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FDD0;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1FDD0: movep.w $1b19(a5), d4
pub fn i_c1fdd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FDD4;
    cpu.ir = 0x90D;
    let a = cpu.a[5].wrapping_add(0x1B19);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[4] = (cpu.d[4] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C1FDD4: ori.b #$1, d5
pub fn i_c1fdd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FDD8;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1FDD8: movep.w $1b22(a5), d4
pub fn i_c1fdd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FDDC;
    cpu.ir = 0x90D;
    let a = cpu.a[5].wrapping_add(0x1B22);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[4] = (cpu.d[4] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C1FDDC: ori.b #$1, d5
pub fn i_c1fddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FDE0;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1FDE0: movep.w $1b2b(a5), d4
pub fn i_c1fde0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FDE4;
    cpu.ir = 0x90D;
    let a = cpu.a[5].wrapping_add(0x1B2B);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[4] = (cpu.d[4] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C1FDE4: ori.b #$1, d5
pub fn i_c1fde4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FDE8;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1FDE8: movep.w $1b34(a5), d4
pub fn i_c1fde8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FDEC;
    cpu.ir = 0x90D;
    let a = cpu.a[5].wrapping_add(0x1B34);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[4] = (cpu.d[4] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C1FDEC: ori.b #$21, d5
pub fn i_c1fdec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FDF0;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0x21) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1FDF0: movep.w $1b3d(a5), d3
pub fn i_c1fdf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FDF4;
    cpu.ir = 0x70D;
    let a = cpu.a[5].wrapping_add(0x1B3D);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[3] = (cpu.d[3] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C1FDF4: ori.b #$1, d5
pub fn i_c1fdf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FDF8;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1FDF8: movep.w $1b44(a5), d3
pub fn i_c1fdf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FDFC;
    cpu.ir = 0x70D;
    let a = cpu.a[5].wrapping_add(0x1B44);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[3] = (cpu.d[3] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C1FDFC: ori.b #$61, d5
pub fn i_c1fdfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FE00;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0x61) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1FE00: movep.w $1b4b(a5), d0
pub fn i_c1fe00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FE04;
    cpu.ir = 0x10D;
    let a = cpu.a[5].wrapping_add(0x1B4B);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C1FE04: ori.b #$ff, d1
pub fn i_c1fe04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FE08;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0xFF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1FE08: ori.b #$f6, d3
pub fn i_c1fe08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FE0C;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0xF6) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1FE0C: ori.b #$4d, d7
pub fn i_c1fe0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FE10;
    cpu.ir = 0x7;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = (v1 | 0x4D) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1FE10: trap #$f
pub fn i_c1fe10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FE12;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $C1FE12: ?
pub fn i_c1fe12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FE14;
    cpu.ir = 0x4F50;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1FE14: subq.w #$8, (a7)
pub fn i_c1fe14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FE16;
    cpu.ir = 0x5157;
    let ad1 = cpu.a[7];
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C1FE16: ori.w #$534f, (a2)
pub fn i_c1fe16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FE1A;
    cpu.ir = 0x52;
    let ad1 = cpu.a[2];
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x534F) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    12
}
/// $C1FE1A: addq.w #$2, (a5)
pub fn i_c1fe1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FE1C;
    cpu.ir = 0x5455;
    let ad1 = cpu.a[5];
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C1FE1C: ori.w #$5750, (a6)
pub fn i_c1fe1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FE20;
    cpu.ir = 0x56;
    let ad1 = cpu.a[6];
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x5750) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    12
}
/// $C1FE20: ori.w #$5156, (a2)
pub fn i_c1fe20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FE24;
    cpu.ir = 0x52;
    let ad1 = cpu.a[2];
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x5156) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    12
}
/// $C1FE24: addq.w #$4, (a7)
pub fn i_c1fe24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FE26;
    cpu.ir = 0x5857;
    let ad1 = cpu.a[7];
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 4, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C1FE52: ori.b #$4, d3
pub fn i_c1fe52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FE56;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x4) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1FE56: movep.w $5741(a0), d0
pub fn i_c1fe56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FE5A;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x5741);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C1FE5A: addq.w #$1, a6
pub fn i_c1fe5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FE5C;
    cpu.ir = 0x524E;
    cpu.a[6] = cpu.a[6].wrapping_add(1);
    8
}
/// $C1FE5C: ?
pub fn i_c1fe5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1FE5E;
    cpu.ir = 0x494E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0xC1A132, i_c1a132),
    (0xC1A134, i_c1a134),
    (0xC1A13A, i_c1a13a),
    (0xC1A13E, i_c1a13e),
    (0xC1A142, i_c1a142),
    (0xC1A146, i_c1a146),
    (0xC1A14A, i_c1a14a),
    (0xC1A14C, i_c1a14c),
    (0xC1A150, i_c1a150),
    (0xC1A154, i_c1a154),
    (0xC1A158, i_c1a158),
    (0xC1A15C, i_c1a15c),
    (0xC1A160, i_c1a160),
    (0xC1A162, i_c1a162),
    (0xC1A164, i_c1a164),
    (0xC1A168, i_c1a168),
    (0xC1A16C, i_c1a16c),
    (0xC1A170, i_c1a170),
    (0xC1A174, i_c1a174),
    (0xC1A178, i_c1a178),
    (0xC1A17C, i_c1a17c),
    (0xC1A17E, i_c1a17e),
    (0xC1A182, i_c1a182),
    (0xC1A188, i_c1a188),
    (0xC1A18C, i_c1a18c),
    (0xC1A190, i_c1a190),
    (0xC1A192, i_c1a192),
    (0xC1A196, i_c1a196),
    (0xC1A19A, i_c1a19a),
    (0xC1A19C, i_c1a19c),
    (0xC1A1A2, i_c1a1a2),
    (0xC1A1A6, i_c1a1a6),
    (0xC1A1AA, i_c1a1aa),
    (0xC1A1AE, i_c1a1ae),
    (0xC1A1B4, i_c1a1b4),
    (0xC1A1B8, i_c1a1b8),
    (0xC1A1BA, i_c1a1ba),
    (0xC1A1BC, i_c1a1bc),
    (0xC1A1BE, i_c1a1be),
    (0xC1A1C0, i_c1a1c0),
    (0xC1A1C4, i_c1a1c4),
    (0xC1A1C8, i_c1a1c8),
    (0xC1A1CC, i_c1a1cc),
    (0xC1A1D0, i_c1a1d0),
    (0xC1A1D4, i_c1a1d4),
    (0xC1A1D6, i_c1a1d6),
    (0xC1A1DA, i_c1a1da),
    (0xC1A1DC, i_c1a1dc),
    (0xC1A1E0, i_c1a1e0),
    (0xC1A1E4, i_c1a1e4),
    (0xC1A1E6, i_c1a1e6),
    (0xC1A1EA, i_c1a1ea),
    (0xC1A1EE, i_c1a1ee),
    (0xC1A1F0, i_c1a1f0),
    (0xC1A1F2, i_c1a1f2),
    (0xC1A1F4, i_c1a1f4),
    (0xC1A1F6, i_c1a1f6),
    (0xC1A1F8, i_c1a1f8),
    (0xC1A1FA, i_c1a1fa),
    (0xC1A1FC, i_c1a1fc),
    (0xC1A1FE, i_c1a1fe),
    (0xC1A200, i_c1a200),
    (0xC1A204, i_c1a204),
    (0xC1A206, i_c1a206),
    (0xC1A20A, i_c1a20a),
    (0xC1A20C, i_c1a20c),
    (0xC1A20E, i_c1a20e),
    (0xC1A210, i_c1a210),
    (0xC1A212, i_c1a212),
    (0xC1A214, i_c1a214),
    (0xC1A216, i_c1a216),
    (0xC1A218, i_c1a218),
    (0xC1A21E, i_c1a21e),
    (0xC1A222, i_c1a222),
    (0xC1A224, i_c1a224),
    (0xC1A226, i_c1a226),
    (0xC1A22A, i_c1a22a),
    (0xC1A22C, i_c1a22c),
    (0xC1A22E, i_c1a22e),
    (0xC1A232, i_c1a232),
    (0xC1A234, i_c1a234),
    (0xC1A236, i_c1a236),
    (0xC1A238, i_c1a238),
    (0xC1A23A, i_c1a23a),
    (0xC1A23C, i_c1a23c),
    (0xC1A23E, i_c1a23e),
    (0xC1A240, i_c1a240),
    (0xC1A244, i_c1a244),
    (0xC1A248, i_c1a248),
    (0xC1A24A, i_c1a24a),
    (0xC1A24E, i_c1a24e),
    (0xC1A252, i_c1a252),
    (0xC1A258, i_c1a258),
    (0xC1A25C, i_c1a25c),
    (0xC1A260, i_c1a260),
    (0xC1A262, i_c1a262),
    (0xC1A266, i_c1a266),
    (0xC1A26A, i_c1a26a),
    (0xC1A26C, i_c1a26c),
    (0xC1A270, i_c1a270),
    (0xC1A274, i_c1a274),
    (0xC1A276, i_c1a276),
    (0xC1A278, i_c1a278),
    (0xC1A27A, i_c1a27a),
    (0xC1A27E, i_c1a27e),
    (0xC1A280, i_c1a280),
    (0xC1A284, i_c1a284),
    (0xC1A286, i_c1a286),
    (0xC1A288, i_c1a288),
    (0xC1A28C, i_c1a28c),
    (0xC1A290, i_c1a290),
    (0xC1A292, i_c1a292),
    (0xC1A294, i_c1a294),
    (0xC1A296, i_c1a296),
    (0xC1A298, i_c1a298),
    (0xC1A29A, i_c1a29a),
    (0xC1A29C, i_c1a29c),
    (0xC1A29E, i_c1a29e),
    (0xC1A2A0, i_c1a2a0),
    (0xC1A2A2, i_c1a2a2),
    (0xC1A2A4, i_c1a2a4),
    (0xC1A2A6, i_c1a2a6),
    (0xC1A2AA, i_c1a2aa),
    (0xC1A2AE, i_c1a2ae),
    (0xC1A2B0, i_c1a2b0),
    (0xC1A2B2, i_c1a2b2),
    (0xC1A2B4, i_c1a2b4),
    (0xC1A2B6, i_c1a2b6),
    (0xC1A2B8, i_c1a2b8),
    (0xC1A2BA, i_c1a2ba),
    (0xC1A328, i_c1a328),
    (0xC1A32E, i_c1a32e),
    (0xC1A334, i_c1a334),
    (0xC1A336, i_c1a336),
    (0xC1A338, i_c1a338),
    (0xC1A33A, i_c1a33a),
    (0xC1A33C, i_c1a33c),
    (0xC1A342, i_c1a342),
    (0xC1A346, i_c1a346),
    (0xC1A34C, i_c1a34c),
    (0xC1A350, i_c1a350),
    (0xC1A354, i_c1a354),
    (0xC1A358, i_c1a358),
    (0xC1A35C, i_c1a35c),
    (0xC1A360, i_c1a360),
    (0xC1A364, i_c1a364),
    (0xC1A368, i_c1a368),
    (0xC1A36C, i_c1a36c),
    (0xC1A370, i_c1a370),
    (0xC1A376, i_c1a376),
    (0xC1A37C, i_c1a37c),
    (0xC1A380, i_c1a380),
    (0xC1A384, i_c1a384),
    (0xC1A388, i_c1a388),
    (0xC1A38C, i_c1a38c),
    (0xC1A390, i_c1a390),
    (0xC1A394, i_c1a394),
    (0xC1A39A, i_c1a39a),
    (0xC1A3A0, i_c1a3a0),
    (0xC1A3A6, i_c1a3a6),
    (0xC1A3AA, i_c1a3aa),
    (0xC1A3AE, i_c1a3ae),
    (0xC1A3B2, i_c1a3b2),
    (0xC1A3B6, i_c1a3b6),
    (0xC1A3BA, i_c1a3ba),
    (0xC1A3BE, i_c1a3be),
    (0xC1A3C8, i_c1a3c8),
    (0xC1A3D2, i_c1a3d2),
    (0xC1A3D8, i_c1a3d8),
    (0xC1A3DE, i_c1a3de),
    (0xC1A3E4, i_c1a3e4),
    (0xC1A3EA, i_c1a3ea),
    (0xC1A3F0, i_c1a3f0),
    (0xC1A3F8, i_c1a3f8),
    (0xC1A400, i_c1a400),
    (0xC1A408, i_c1a408),
    (0xC1A40E, i_c1a40e),
    (0xC1A416, i_c1a416),
    (0xC1A41A, i_c1a41a),
    (0xC1A420, i_c1a420),
    (0xC1A426, i_c1a426),
    (0xC1A42A, i_c1a42a),
    (0xC1A42E, i_c1a42e),
    (0xC1A434, i_c1a434),
    (0xC1A438, i_c1a438),
    (0xC1A43C, i_c1a43c),
    (0xC1A442, i_c1a442),
    (0xC1A446, i_c1a446),
    (0xC1A448, i_c1a448),
    (0xC1A44A, i_c1a44a),
    (0xC1A44E, i_c1a44e),
    (0xC1A454, i_c1a454),
    (0xC1A45A, i_c1a45a),
    (0xC1A45E, i_c1a45e),
    (0xC1A462, i_c1a462),
    (0xC1A464, i_c1a464),
    (0xC1A466, i_c1a466),
    (0xC1A46A, i_c1a46a),
    (0xC1A46E, i_c1a46e),
    (0xC1A472, i_c1a472),
    (0xC1A476, i_c1a476),
    (0xC1A47A, i_c1a47a),
    (0xC1A47E, i_c1a47e),
    (0xC1A480, i_c1a480),
    (0xC1A486, i_c1a486),
    (0xC1A48A, i_c1a48a),
    (0xC1A48C, i_c1a48c),
    (0xC1A490, i_c1a490),
    (0xC1A492, i_c1a492),
    (0xC1A494, i_c1a494),
    (0xC1A496, i_c1a496),
    (0xC1A49A, i_c1a49a),
    (0xC1A49C, i_c1a49c),
    (0xC1A4A0, i_c1a4a0),
    (0xC1A4A6, i_c1a4a6),
    (0xC1A4A8, i_c1a4a8),
    (0xC1A4AC, i_c1a4ac),
    (0xC1A4B0, i_c1a4b0),
    (0xC1A4B6, i_c1a4b6),
    (0xC1A4B8, i_c1a4b8),
    (0xC1A4BC, i_c1a4bc),
    (0xC1A4C0, i_c1a4c0),
    (0xC1A4C4, i_c1a4c4),
    (0xC1A4C8, i_c1a4c8),
    (0xC1A4CC, i_c1a4cc),
    (0xC1A4CE, i_c1a4ce),
    (0xC1A4D2, i_c1a4d2),
    (0xC1A4D4, i_c1a4d4),
    (0xC1A4D8, i_c1a4d8),
    (0xC1A4DC, i_c1a4dc),
    (0xC1A4E2, i_c1a4e2),
    (0xC1A4E6, i_c1a4e6),
    (0xC1A4EA, i_c1a4ea),
    (0xC1A4EC, i_c1a4ec),
    (0xC1A4F0, i_c1a4f0),
    (0xC1A4F2, i_c1a4f2),
    (0xC1A4F6, i_c1a4f6),
    (0xC1A4FA, i_c1a4fa),
    (0xC1A4FC, i_c1a4fc),
    (0xC1A4FE, i_c1a4fe),
    (0xC1A500, i_c1a500),
    (0xC1A502, i_c1a502),
    (0xC1A504, i_c1a504),
    (0xC1A508, i_c1a508),
    (0xC1A50C, i_c1a50c),
    (0xC1A50E, i_c1a50e),
    (0xC1A510, i_c1a510),
    (0xC1A514, i_c1a514),
    (0xC1A516, i_c1a516),
    (0xC1A518, i_c1a518),
    (0xC1A51A, i_c1a51a),
    (0xC1A51C, i_c1a51c),
    (0xC1A51E, i_c1a51e),
    (0xC1A520, i_c1a520),
    (0xC1A522, i_c1a522),
    (0xC1A526, i_c1a526),
    (0xC1A52A, i_c1a52a),
    (0xC1A52C, i_c1a52c),
    (0xC1A52E, i_c1a52e),
    (0xC1A530, i_c1a530),
    (0xC1A532, i_c1a532),
    (0xC1A534, i_c1a534),
    (0xC1A536, i_c1a536),
    (0xC1A538, i_c1a538),
    (0xC1A53E, i_c1a53e),
    (0xC1A542, i_c1a542),
    (0xC1A544, i_c1a544),
    (0xC1A546, i_c1a546),
    (0xC1A54C, i_c1a54c),
    (0xC1A550, i_c1a550),
    (0xC1A554, i_c1a554),
    (0xC1A556, i_c1a556),
    (0xC1A558, i_c1a558),
    (0xC1A55A, i_c1a55a),
    (0xC1A55C, i_c1a55c),
    (0xC1A55E, i_c1a55e),
    (0xC1A562, i_c1a562),
    (0xC1A566, i_c1a566),
    (0xC1A56A, i_c1a56a),
    (0xC1A56E, i_c1a56e),
    (0xC1A570, i_c1a570),
    (0xC1A574, i_c1a574),
    (0xC1A576, i_c1a576),
    (0xC1A578, i_c1a578),
    (0xC1A57A, i_c1a57a),
    (0xC1A580, i_c1a580),
    (0xC1A584, i_c1a584),
    (0xC1A588, i_c1a588),
    (0xC1A58A, i_c1a58a),
    (0xC1A58C, i_c1a58c),
    (0xC1A592, i_c1a592),
    (0xC1A594, i_c1a594),
    (0xC1A598, i_c1a598),
    (0xC1A59C, i_c1a59c),
    (0xC1A5A2, i_c1a5a2),
    (0xC1A5A6, i_c1a5a6),
    (0xC1A5AC, i_c1a5ac),
    (0xC1A5AE, i_c1a5ae),
    (0xC1A5B0, i_c1a5b0),
    (0xC1A5B4, i_c1a5b4),
    (0xC1A5B6, i_c1a5b6),
    (0xC1A5BA, i_c1a5ba),
    (0xC1A5BE, i_c1a5be),
    (0xC1A5C2, i_c1a5c2),
    (0xC1A5C4, i_c1a5c4),
    (0xC1A5C8, i_c1a5c8),
    (0xC1A5CC, i_c1a5cc),
    (0xC1A5CE, i_c1a5ce),
    (0xC1A5D0, i_c1a5d0),
    (0xC1A5D2, i_c1a5d2),
    (0xC1A5D4, i_c1a5d4),
    (0xC1A5D8, i_c1a5d8),
    (0xC1A5DA, i_c1a5da),
    (0xC1A5DE, i_c1a5de),
    (0xC1A5E0, i_c1a5e0),
    (0xC1A5E2, i_c1a5e2),
    (0xC1A5E8, i_c1a5e8),
    (0xC1A5EC, i_c1a5ec),
    (0xC1A5F2, i_c1a5f2),
    (0xC1A5F6, i_c1a5f6),
    (0xC1A5FA, i_c1a5fa),
    (0xC1A5FC, i_c1a5fc),
    (0xC1A602, i_c1a602),
    (0xC1A604, i_c1a604),
    (0xC1A60A, i_c1a60a),
    (0xC1A612, i_c1a612),
    (0xC1A618, i_c1a618),
    (0xC1A61C, i_c1a61c),
    (0xC1A620, i_c1a620),
    (0xC1A626, i_c1a626),
    (0xC1A62A, i_c1a62a),
    (0xC1A62E, i_c1a62e),
    (0xC1A634, i_c1a634),
    (0xC1A638, i_c1a638),
    (0xC1A63C, i_c1a63c),
    (0xC1A642, i_c1a642),
    (0xC1A646, i_c1a646),
    (0xC1A64A, i_c1a64a),
    (0xC1A64E, i_c1a64e),
    (0xC1A650, i_c1a650),
    (0xC1A654, i_c1a654),
    (0xC1A658, i_c1a658),
    (0xC1A65E, i_c1a65e),
    (0xC1A660, i_c1a660),
    (0xC1A664, i_c1a664),
    (0xC1A666, i_c1a666),
    (0xC1A66C, i_c1a66c),
    (0xC1A670, i_c1a670),
    (0xC1A676, i_c1a676),
    (0xC1A67A, i_c1a67a),
    (0xC1A680, i_c1a680),
    (0xC1A686, i_c1a686),
    (0xC1A688, i_c1a688),
    (0xC1A68A, i_c1a68a),
    (0xC1A68C, i_c1a68c),
    (0xC1A68E, i_c1a68e),
    (0xC1A690, i_c1a690),
    (0xC1A692, i_c1a692),
    (0xC1A696, i_c1a696),
    (0xC1A69A, i_c1a69a),
    (0xC1A69C, i_c1a69c),
    (0xC1A69E, i_c1a69e),
    (0xC1A6A0, i_c1a6a0),
    (0xC1A6A2, i_c1a6a2),
    (0xC1A6A4, i_c1a6a4),
    (0xC1A6A8, i_c1a6a8),
    (0xC1A6AA, i_c1a6aa),
    (0xC1A6AC, i_c1a6ac),
    (0xC1A6AE, i_c1a6ae),
    (0xC1A6B0, i_c1a6b0),
    (0xC1A6B2, i_c1a6b2),
    (0xC1F800, i_c1f800),
    (0xC1F802, i_c1f802),
    (0xC1F804, i_c1f804),
    (0xC1F806, i_c1f806),
    (0xC1F808, i_c1f808),
    (0xC1F80C, i_c1f80c),
    (0xC1F80E, i_c1f80e),
    (0xC1F810, i_c1f810),
    (0xC1F812, i_c1f812),
    (0xC1F814, i_c1f814),
    (0xC1F816, i_c1f816),
    (0xC1F818, i_c1f818),
    (0xC1F81A, i_c1f81a),
    (0xC1F81C, i_c1f81c),
    (0xC1F81E, i_c1f81e),
    (0xC1F820, i_c1f820),
    (0xC1F822, i_c1f822),
    (0xC1F824, i_c1f824),
    (0xC1F826, i_c1f826),
    (0xC1F828, i_c1f828),
    (0xC1F82A, i_c1f82a),
    (0xC1F82C, i_c1f82c),
    (0xC1F82E, i_c1f82e),
    (0xC1F830, i_c1f830),
    (0xC1F832, i_c1f832),
    (0xC1F834, i_c1f834),
    (0xC1F836, i_c1f836),
    (0xC1F83A, i_c1f83a),
    (0xC1F83E, i_c1f83e),
    (0xC1F842, i_c1f842),
    (0xC1F844, i_c1f844),
    (0xC1F846, i_c1f846),
    (0xC1F848, i_c1f848),
    (0xC1F84A, i_c1f84a),
    (0xC1FA7E, i_c1fa7e),
    (0xC1FA80, i_c1fa80),
    (0xC1FA82, i_c1fa82),
    (0xC1FAC6, i_c1fac6),
    (0xC1FAC8, i_c1fac8),
    (0xC1FACA, i_c1faca),
    (0xC1FACC, i_c1facc),
    (0xC1FACE, i_c1face),
    (0xC1FAD2, i_c1fad2),
    (0xC1FAD6, i_c1fad6),
    (0xC1FADA, i_c1fada),
    (0xC1FADC, i_c1fadc),
    (0xC1FAE0, i_c1fae0),
    (0xC1FAE4, i_c1fae4),
    (0xC1FAE8, i_c1fae8),
    (0xC1FAEC, i_c1faec),
    (0xC1FAF0, i_c1faf0),
    (0xC1FAF4, i_c1faf4),
    (0xC1FAF8, i_c1faf8),
    (0xC1FAFC, i_c1fafc),
    (0xC1FB00, i_c1fb00),
    (0xC1FB04, i_c1fb04),
    (0xC1FB0A, i_c1fb0a),
    (0xC1FB0C, i_c1fb0c),
    (0xC1FB0E, i_c1fb0e),
    (0xC1FB10, i_c1fb10),
    (0xC1FB12, i_c1fb12),
    (0xC1FB14, i_c1fb14),
    (0xC1FB18, i_c1fb18),
    (0xC1FB1C, i_c1fb1c),
    (0xC1FB1E, i_c1fb1e),
    (0xC1FB22, i_c1fb22),
    (0xC1FB26, i_c1fb26),
    (0xC1FB2A, i_c1fb2a),
    (0xC1FB2C, i_c1fb2c),
    (0xC1FB2E, i_c1fb2e),
    (0xC1FB30, i_c1fb30),
    (0xC1FB32, i_c1fb32),
    (0xC1FB34, i_c1fb34),
    (0xC1FB36, i_c1fb36),
    (0xC1FB38, i_c1fb38),
    (0xC1FB3A, i_c1fb3a),
    (0xC1FB3C, i_c1fb3c),
    (0xC1FB3E, i_c1fb3e),
    (0xC1FB40, i_c1fb40),
    (0xC1FB42, i_c1fb42),
    (0xC1FB44, i_c1fb44),
    (0xC1FB46, i_c1fb46),
    (0xC1FB48, i_c1fb48),
    (0xC1FB4A, i_c1fb4a),
    (0xC1FB4C, i_c1fb4c),
    (0xC1FB4E, i_c1fb4e),
    (0xC1FB50, i_c1fb50),
    (0xC1FB52, i_c1fb52),
    (0xC1FB54, i_c1fb54),
    (0xC1FB56, i_c1fb56),
    (0xC1FB58, i_c1fb58),
    (0xC1FB5A, i_c1fb5a),
    (0xC1FBCA, i_c1fbca),
    (0xC1FBCE, i_c1fbce),
    (0xC1FBD0, i_c1fbd0),
    (0xC1FBD2, i_c1fbd2),
    (0xC1FBD4, i_c1fbd4),
    (0xC1FBD8, i_c1fbd8),
    (0xC1FBDC, i_c1fbdc),
    (0xC1FBE0, i_c1fbe0),
    (0xC1FD8A, i_c1fd8a),
    (0xC1FD8E, i_c1fd8e),
    (0xC1FD90, i_c1fd90),
    (0xC1FD92, i_c1fd92),
    (0xC1FD94, i_c1fd94),
    (0xC1FD96, i_c1fd96),
    (0xC1FD98, i_c1fd98),
    (0xC1FD9A, i_c1fd9a),
    (0xC1FD9E, i_c1fd9e),
    (0xC1FDA0, i_c1fda0),
    (0xC1FDA2, i_c1fda2),
    (0xC1FDA8, i_c1fda8),
    (0xC1FDAA, i_c1fdaa),
    (0xC1FDAC, i_c1fdac),
    (0xC1FDB0, i_c1fdb0),
    (0xC1FDB4, i_c1fdb4),
    (0xC1FDB8, i_c1fdb8),
    (0xC1FDBC, i_c1fdbc),
    (0xC1FDC0, i_c1fdc0),
    (0xC1FDC4, i_c1fdc4),
    (0xC1FDC8, i_c1fdc8),
    (0xC1FDCC, i_c1fdcc),
    (0xC1FDD0, i_c1fdd0),
    (0xC1FDD4, i_c1fdd4),
    (0xC1FDD8, i_c1fdd8),
    (0xC1FDDC, i_c1fddc),
    (0xC1FDE0, i_c1fde0),
    (0xC1FDE4, i_c1fde4),
    (0xC1FDE8, i_c1fde8),
    (0xC1FDEC, i_c1fdec),
    (0xC1FDF0, i_c1fdf0),
    (0xC1FDF4, i_c1fdf4),
    (0xC1FDF8, i_c1fdf8),
    (0xC1FDFC, i_c1fdfc),
    (0xC1FE00, i_c1fe00),
    (0xC1FE04, i_c1fe04),
    (0xC1FE08, i_c1fe08),
    (0xC1FE0C, i_c1fe0c),
    (0xC1FE10, i_c1fe10),
    (0xC1FE12, i_c1fe12),
    (0xC1FE14, i_c1fe14),
    (0xC1FE16, i_c1fe16),
    (0xC1FE1A, i_c1fe1a),
    (0xC1FE1C, i_c1fe1c),
    (0xC1FE20, i_c1fe20),
    (0xC1FE24, i_c1fe24),
    (0xC1FE52, i_c1fe52),
    (0xC1FE56, i_c1fe56),
    (0xC1FE5A, i_c1fe5a),
    (0xC1FE5C, i_c1fe5c),
];
