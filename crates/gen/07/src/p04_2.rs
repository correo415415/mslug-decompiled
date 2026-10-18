// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p04_2) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $043958: movea.l $6(a1), a1
pub fn i_043958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4395C;
    cpu.ir = 0x2269;
    let ad2 = cpu.a[1].wrapping_add(0x6);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $04395C: jsr $51b38.l
pub fn i_04395c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43962;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x43962);
    cpu.pc = 0x51B38;
    16
}
/// $043962: bra.w $4380c
pub fn i_043962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43966;
    cpu.ir = 0x6000;
    cpu.pc = 0x4380C;
    10
}
/// $043966: lea.l $6(a1), a0
pub fn i_043966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4396A;
    cpu.ir = 0x41E9;
    let ad1 = cpu.a[1].wrapping_add(0x6);
    cpu.a[0] = ad1;
    16
}
/// $04396A: move.l a0, $10815c.l
pub fn i_04396a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43970;
    cpu.ir = 0x23C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(0x10815C, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043970: move.w $2(a1), $108164.l
pub fn i_043970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43978;
    cpu.ir = 0x33E9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x108164, v3 as u16);
    24
}
/// $043978: move.w $4(a1), $108166.l
pub fn i_043978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43980;
    cpu.ir = 0x33E9;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x108166, v3 as u16);
    24
}
/// $043980: bra.w $4380c
pub fn i_043980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43984;
    cpu.ir = 0x6000;
    cpu.pc = 0x4380C;
    10
}
/// $043984: lea.l $c(a1), a0
pub fn i_043984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43988;
    cpu.ir = 0x41E9;
    let ad1 = cpu.a[1].wrapping_add(0xC);
    cpu.a[0] = ad1;
    16
}
/// $043988: move.l a0, $10815c.l
pub fn i_043988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4398E;
    cpu.ir = 0x23C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(0x10815C, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $04398E: move.w $2(a1), $108168.l
pub fn i_04398e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43996;
    cpu.ir = 0x33E9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x108168, v3 as u16);
    24
}
/// $043996: move.w $4(a1), $10816a.l
pub fn i_043996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4399E;
    cpu.ir = 0x33E9;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10816A, v3 as u16);
    24
}
/// $04399E: move.w $6(a1), $10816c.l
pub fn i_04399e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x439A6;
    cpu.ir = 0x33E9;
    let ad2 = cpu.a[1].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10816C, v3 as u16);
    24
}
/// $0439A6: move.w $8(a1), $10816e.l
pub fn i_0439a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x439AE;
    cpu.ir = 0x33E9;
    let ad2 = cpu.a[1].wrapping_add(0x8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10816E, v3 as u16);
    24
}
/// $0439AE: move.w $a(a1), $108170.l
pub fn i_0439ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x439B6;
    cpu.ir = 0x33E9;
    let ad2 = cpu.a[1].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x108170, v3 as u16);
    24
}
/// $0439B6: bra.w $4380c
pub fn i_0439b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x439BA;
    cpu.ir = 0x6000;
    cpu.pc = 0x4380C;
    10
}
/// $0439BA: lea.l $6(a1), a0
pub fn i_0439ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x439BE;
    cpu.ir = 0x41E9;
    let ad1 = cpu.a[1].wrapping_add(0x6);
    cpu.a[0] = ad1;
    16
}
/// $0439BE: move.l a0, $10815c.l
pub fn i_0439be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x439C4;
    cpu.ir = 0x23C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(0x10815C, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $0439C4: movea.l $2(a1), a1
pub fn i_0439c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x439C8;
    cpu.ir = 0x2269;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0439C8: move.l a1, -(a7)
pub fn i_0439c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x439CA;
    cpu.ir = 0x2F09;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $0439CA: jsr $4ae.l
pub fn i_0439ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x439D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x439D0);
    cpu.pc = 0x4AE;
    16
}
/// $0439D0: movea.l (a7)+, a1
pub fn i_0439d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x439D2;
    cpu.ir = 0x225F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    12
}
/// $0439D2: move.l a1, (a0)
pub fn i_0439d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x439D4;
    cpu.ir = 0x2089;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0439D4: bra.w $4380c
pub fn i_0439d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x439D8;
    cpu.ir = 0x6000;
    cpu.pc = 0x4380C;
    10
}
/// $0439D8: lea.l $4(a1), a0
pub fn i_0439d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x439DC;
    cpu.ir = 0x41E9;
    let ad1 = cpu.a[1].wrapping_add(0x4);
    cpu.a[0] = ad1;
    16
}
/// $0439DC: move.l a0, $10815c.l
pub fn i_0439dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x439E2;
    cpu.ir = 0x23C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(0x10815C, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $0439E2: move.w $2(a1), d0
pub fn i_0439e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x439E6;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0439E6: jsr $219c.l
pub fn i_0439e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x439EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x439EC);
    cpu.pc = 0x219C;
    16
}
/// $0439EC: bra.w $4380c
pub fn i_0439ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x439F0;
    cpu.ir = 0x6000;
    cpu.pc = 0x4380C;
    10
}
/// $0439F0: lea.l $e(a1), a0
pub fn i_0439f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x439F4;
    cpu.ir = 0x41E9;
    let ad1 = cpu.a[1].wrapping_add(0xE);
    cpu.a[0] = ad1;
    16
}
/// $0439F4: move.l a0, $10815c.l
pub fn i_0439f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x439FA;
    cpu.ir = 0x23C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(0x10815C, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $0439FA: lea.l $6(a1), a0
pub fn i_0439fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x439FE;
    cpu.ir = 0x41E9;
    let ad1 = cpu.a[1].wrapping_add(0x6);
    cpu.a[0] = ad1;
    16
}
/// $0439FE: movea.l $2(a1), a1
pub fn i_0439fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A02;
    cpu.ir = 0x2269;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $043A02: jsr (a1)
pub fn i_043a02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A04;
    cpu.ir = 0x4E91;
    cpu.push32(bus, 0x43A04);
    cpu.jump_to(cpu.a[1]);
    16
}
/// $043A04: bra.w $4380c
pub fn i_043a04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A08;
    cpu.ir = 0x6000;
    cpu.pc = 0x4380C;
    10
}
/// $043A08: lea.l $4(a1), a0
pub fn i_043a08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A0C;
    cpu.ir = 0x41E9;
    let ad1 = cpu.a[1].wrapping_add(0x4);
    cpu.a[0] = ad1;
    16
}
/// $043A0C: move.l a0, $10815c.l
pub fn i_043a0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A12;
    cpu.ir = 0x23C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(0x10815C, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043A12: move.b $2(a1), $106eac.l
pub fn i_043a12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A1A;
    cpu.ir = 0x13E9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106EAC, v3 as u8);
    24
}
/// $043A1A: bra.w $4380c
pub fn i_043a1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A1E;
    cpu.ir = 0x6000;
    cpu.pc = 0x4380C;
    10
}
/// $043A1E: lea.l $6(a1), a0
pub fn i_043a1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A22;
    cpu.ir = 0x41E9;
    let ad1 = cpu.a[1].wrapping_add(0x6);
    cpu.a[0] = ad1;
    16
}
/// $043A22: move.l a0, $10815c.l
pub fn i_043a22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A28;
    cpu.ir = 0x23C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(0x10815C, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043A28: movea.l $2(a1), a0
pub fn i_043a28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A2C;
    cpu.ir = 0x2069;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $043A2C: jsr $51f02.l
pub fn i_043a2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x43A32);
    cpu.pc = 0x51F02;
    16
}
/// $043A32: tst.l $78(a0)
pub fn i_043a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A36;
    cpu.ir = 0x4AA8;
    let ad2 = cpu.a[0].wrapping_add(0x78);
    let v3 = bus.read32(ad2);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $043A36: beq.w $4380c
pub fn i_043a36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A3A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4380C; return 10; }
    8
}
/// $043A3A: bsr.w $43eda
pub fn i_043a3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A3E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x43A3E);
    cpu.pc = 0x43EDA;
    18
}
/// $043A3E: bra.w $4380c
pub fn i_043a3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A42;
    cpu.ir = 0x6000;
    cpu.pc = 0x4380C;
    10
}
/// $043A42: lea.l $2(a1), a0
pub fn i_043a42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A46;
    cpu.ir = 0x41E9;
    let ad1 = cpu.a[1].wrapping_add(0x2);
    cpu.a[0] = ad1;
    16
}
/// $043A46: jsr $2b58.l
pub fn i_043a46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x43A4C);
    cpu.pc = 0x2B58;
    16
}
/// $043A4C: addq.w #$2, a0
pub fn i_043a4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A4E;
    cpu.ir = 0x5448;
    cpu.a[0] = cpu.a[0].wrapping_add(2);
    8
}
/// $043A4E: move.l a0, $10815c.l
pub fn i_043a4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A54;
    cpu.ir = 0x23C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(0x10815C, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043A54: bra.w $4380c
pub fn i_043a54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A58;
    cpu.ir = 0x6000;
    cpu.pc = 0x4380C;
    10
}
/// $043A58: lea.l $6(a1), a0
pub fn i_043a58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A5C;
    cpu.ir = 0x41E9;
    let ad1 = cpu.a[1].wrapping_add(0x6);
    cpu.a[0] = ad1;
    16
}
/// $043A5C: move.l a0, $10815c.l
pub fn i_043a5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A62;
    cpu.ir = 0x23C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(0x10815C, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043A62: movea.l $2(a1), a0
pub fn i_043a62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A66;
    cpu.ir = 0x2069;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $043A66: bsr.w $43df4
pub fn i_043a66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A6A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x43A6A);
    cpu.pc = 0x43DF4;
    18
}
/// $043A6A: jsr $51ece.l
pub fn i_043a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A70;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x43A70);
    cpu.pc = 0x51ECE;
    16
}
/// $043A70: bra.w $4380c
pub fn i_043a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A74;
    cpu.ir = 0x6000;
    cpu.pc = 0x4380C;
    10
}
/// $043A74: lea.l $6(a1), a0
pub fn i_043a74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A78;
    cpu.ir = 0x41E9;
    let ad1 = cpu.a[1].wrapping_add(0x6);
    cpu.a[0] = ad1;
    16
}
/// $043A78: move.l a0, $10815c.l
pub fn i_043a78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A7E;
    cpu.ir = 0x23C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(0x10815C, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043A7E: movea.l $2(a1), a0
pub fn i_043a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A82;
    cpu.ir = 0x2069;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $043A82: jsr $51ece.l
pub fn i_043a82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x43A88);
    cpu.pc = 0x51ECE;
    16
}
/// $043A88: bra.w $4380c
pub fn i_043a88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A8C;
    cpu.ir = 0x6000;
    cpu.pc = 0x4380C;
    10
}
/// $043A8C: lea.l $6(a1), a0
pub fn i_043a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A90;
    cpu.ir = 0x41E9;
    let ad1 = cpu.a[1].wrapping_add(0x6);
    cpu.a[0] = ad1;
    16
}
/// $043A90: move.l a0, $10815c.l
pub fn i_043a90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A96;
    cpu.ir = 0x23C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(0x10815C, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043A96: movea.l $2(a1), a0
pub fn i_043a96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A9A;
    cpu.ir = 0x2069;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $043A9A: jsr $51ed6.l
pub fn i_043a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43AA0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x43AA0);
    cpu.pc = 0x51ED6;
    16
}
/// $043AA0: bra.w $4380c
pub fn i_043aa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43AA4;
    cpu.ir = 0x6000;
    cpu.pc = 0x4380C;
    10
}
/// $043AA4: jsr $2(a1)
pub fn i_043aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43AA8;
    cpu.ir = 0x4EA9;
    let ad1 = cpu.a[1].wrapping_add(0x2);
    cpu.push32(bus, 0x43AA8);
    cpu.jump_to(ad1);
    16
}
/// $043AA8: tst.b d0
pub fn i_043aa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43AAA;
    cpu.ir = 0x4A00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $043AAA: bne.w $43bc6
pub fn i_043aaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43AAE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x43BC6; return 10; }
    8
}
/// $043AAE: move.l a1, $10815c.l
pub fn i_043aae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43AB4;
    cpu.ir = 0x23C9;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(0x10815C, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043AB4: bra.w $4380c
pub fn i_043ab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43AB8;
    cpu.ir = 0x6000;
    cpu.pc = 0x4380C;
    10
}
/// $043AB8: lea.l $6(a1), a0
pub fn i_043ab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43ABC;
    cpu.ir = 0x41E9;
    let ad1 = cpu.a[1].wrapping_add(0x6);
    cpu.a[0] = ad1;
    16
}
/// $043ABC: move.l a0, $10815c.l
pub fn i_043abc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43AC2;
    cpu.ir = 0x23C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(0x10815C, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043AC2: movea.l $2(a1), a0
pub fn i_043ac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43AC6;
    cpu.ir = 0x2069;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $043AC6: clr.l $e(a0)
pub fn i_043ac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43ACA;
    cpu.ir = 0x42A8;
    let ad1 = cpu.a[0].wrapping_add(0xE);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $043ACA: jsr $51ed6.l
pub fn i_043aca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43AD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x43AD0);
    cpu.pc = 0x51ED6;
    16
}
/// $043AD0: jsr $51f30.l
pub fn i_043ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43AD6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x43AD6);
    cpu.pc = 0x51F30;
    16
}
/// $043AD6: bra.w $4380c
pub fn i_043ad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43ADA;
    cpu.ir = 0x6000;
    cpu.pc = 0x4380C;
    10
}
/// $043ADA: lea.l $8(a1), a0
pub fn i_043ada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43ADE;
    cpu.ir = 0x41E9;
    let ad1 = cpu.a[1].wrapping_add(0x8);
    cpu.a[0] = ad1;
    16
}
/// $043ADE: move.l a0, $10815c.l
pub fn i_043ade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43AE4;
    cpu.ir = 0x23C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(0x10815C, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043AE4: move.b $6(a1), d0
pub fn i_043ae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43AE8;
    cpu.ir = 0x1029;
    let ad2 = cpu.a[1].wrapping_add(0x6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $043AE8: move.b $7(a1), d1
pub fn i_043ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43AEC;
    cpu.ir = 0x1229;
    let ad2 = cpu.a[1].wrapping_add(0x7);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $043AEC: movea.l $2(a1), a0
pub fn i_043aec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43AF0;
    cpu.ir = 0x2069;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $043AF0: jsr $51b1c.l
pub fn i_043af0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43AF6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x43AF6);
    cpu.pc = 0x51B1C;
    16
}
/// $043AF6: bra.w $4380c
pub fn i_043af6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43AFA;
    cpu.ir = 0x6000;
    cpu.pc = 0x4380C;
    10
}
/// $043AFA: lea.l $6(a1), a0
pub fn i_043afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43AFE;
    cpu.ir = 0x41E9;
    let ad1 = cpu.a[1].wrapping_add(0x6);
    cpu.a[0] = ad1;
    16
}
/// $043AFE: move.l a0, $10815c.l
pub fn i_043afe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B04;
    cpu.ir = 0x23C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(0x10815C, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043B04: move.w $2(a1), d0
pub fn i_043b04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B08;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $043B08: move.w $4(a1), d1
pub fn i_043b08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B0C;
    cpu.ir = 0x3229;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $043B0C: move.b #$1, $108178.l
pub fn i_043b0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B14;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x108178, v1 as u8);
    16
}
/// $043B14: clr.w $106f5c.l
pub fn i_043b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B1A;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x106F5C) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x106F5C, r2 as u16);
    16
}
/// $043B1A: swap d0
pub fn i_043b1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B1C;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043B1C: swap d1
pub fn i_043b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B1E;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043B1E: clr.w d0
pub fn i_043b1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B20;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $043B20: clr.w d1
pub fn i_043b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B22;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $043B22: move.l d0, $106f50.l
pub fn i_043b22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B28;
    cpu.ir = 0x23C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    bus.write32(0x106F50, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043B28: move.l d1, $106f54.l
pub fn i_043b28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B2E;
    cpu.ir = 0x23C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    bus.write32(0x106F54, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043B2E: move.l d1, $106f58.l
pub fn i_043b2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B34;
    cpu.ir = 0x23C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    bus.write32(0x106F58, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043B34: clr.w $108160.l
pub fn i_043b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B3A;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x108160) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x108160, r2 as u16);
    16
}
/// $043B3A: clr.w $108162.l
pub fn i_043b3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B40;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x108162) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x108162, r2 as u16);
    16
}
/// $043B40: move.w $106f5c.l, $108172.l
pub fn i_043b40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B4A;
    cpu.ir = 0x33F9;
    let v2 = bus.read16(0x106F5C) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x108172, v2 as u16);
    28
}
/// $043B4A: move.w $106f50.l, $108174.l
pub fn i_043b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B54;
    cpu.ir = 0x33F9;
    let v2 = bus.read16(0x106F50) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x108174, v2 as u16);
    28
}
/// $043B54: move.w $106f58.l, $108176.l
pub fn i_043b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B5E;
    cpu.ir = 0x33F9;
    let v2 = bus.read16(0x106F58) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x108176, v2 as u16);
    28
}
/// $043B5E: bsr.w $434ea
pub fn i_043b5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B62;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x43B62);
    cpu.pc = 0x434EA;
    18
}
/// $043B62: bra.w $4380c
pub fn i_043b62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B66;
    cpu.ir = 0x6000;
    cpu.pc = 0x4380C;
    10
}
/// $043B66: lea.l $16(a1), a0
pub fn i_043b66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B6A;
    cpu.ir = 0x41E9;
    let ad1 = cpu.a[1].wrapping_add(0x16);
    cpu.a[0] = ad1;
    16
}
/// $043B6A: move.l a0, $10815c.l
pub fn i_043b6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B70;
    cpu.ir = 0x23C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(0x10815C, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043B70: movea.l $2(a1), a0
pub fn i_043b70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B74;
    cpu.ir = 0x2069;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $043B74: move.l $e(a1), $78(a0)
pub fn i_043b74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B7A;
    cpu.ir = 0x2169;
    let ad2 = cpu.a[1].wrapping_add(0xE);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0].wrapping_add(0x78);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $043B7A: move.w $106f50.l, d0
pub fn i_043b7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B80;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106F50) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043B80: move.w $106f58.l, d1
pub fn i_043b80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B86;
    cpu.ir = 0x3239;
    let v2 = bus.read16(0x106F58) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043B86: add.w $10817a.l, d1
pub fn i_043b86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B8C;
    cpu.ir = 0xD279;
    let v2 = bus.read16(0x10817A) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $043B8C: sub.w $12(a1), d0
pub fn i_043b8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B90;
    cpu.ir = 0x9069;
    let ad2 = cpu.a[1].wrapping_add(0x12);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $043B90: sub.w $14(a1), d1
pub fn i_043b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B94;
    cpu.ir = 0x9269;
    let ad2 = cpu.a[1].wrapping_add(0x14);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $043B94: lea.l $6(a1), a1
pub fn i_043b94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B98;
    cpu.ir = 0x43E9;
    let ad1 = cpu.a[1].wrapping_add(0x6);
    cpu.a[1] = ad1;
    16
}
/// $043B98: jsr $51b3e.l
pub fn i_043b98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B9E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x43B9E);
    cpu.pc = 0x51B3E;
    16
}
/// $043B9E: bra.w $4380c
pub fn i_043b9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BA2;
    cpu.ir = 0x6000;
    cpu.pc = 0x4380C;
    10
}
/// $043BA2: lea.l $c(a1), a0
pub fn i_043ba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BA6;
    cpu.ir = 0x41E9;
    let ad1 = cpu.a[1].wrapping_add(0xC);
    cpu.a[0] = ad1;
    16
}
/// $043BA6: move.l a0, $10815c.l
pub fn i_043ba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BAC;
    cpu.ir = 0x23C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(0x10815C, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043BAC: movea.l $2(a1), a0
pub fn i_043bac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BB0;
    cpu.ir = 0x2069;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $043BB0: move.b $6(a1), $72(a0)
pub fn i_043bb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BB6;
    cpu.ir = 0x1169;
    let ad2 = cpu.a[1].wrapping_add(0x6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x72);
    bus.write8(ad4, v3 as u8);
    20
}
/// $043BB6: move.w $8(a1), $74(a0)
pub fn i_043bb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BBC;
    cpu.ir = 0x3169;
    let ad2 = cpu.a[1].wrapping_add(0x8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad4, v3 as u16);
    20
}
/// $043BBC: move.w $a(a1), $76(a0)
pub fn i_043bbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BC2;
    cpu.ir = 0x3169;
    let ad2 = cpu.a[1].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x76);
    bus.write16(ad4, v3 as u16);
    20
}
/// $043BC2: bra.w $4380c
pub fn i_043bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BC6;
    cpu.ir = 0x6000;
    cpu.pc = 0x4380C;
    10
}
/// $043BC6: move.l (a7)+, d0
pub fn i_043bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BC8;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $043BC8: move.l $106f50.l, d1
pub fn i_043bc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BCE;
    cpu.ir = 0x2239;
    let v2 = bus.read32(0x106F50);
    cpu.d[1] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043BCE: move.w $108168.l, d2
pub fn i_043bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BD4;
    cpu.ir = 0x3439;
    let v2 = bus.read16(0x108168) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043BD4: move.w $10816c.l, d3
pub fn i_043bd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BDA;
    cpu.ir = 0x3639;
    let v2 = bus.read16(0x10816C) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043BDA: bsr.w $43e3a
pub fn i_043bda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BDE;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x43BDE);
    cpu.pc = 0x43E3A;
    18
}
/// $043BDE: move.w d0, $108160.l
pub fn i_043bde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BE4;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x108160, v1 as u16);
    16
}
/// $043BE4: move.l d1, $106f50.l
pub fn i_043be4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BEA;
    cpu.ir = 0x23C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    bus.write32(0x106F50, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043BEA: move.l (a7)+, d0
pub fn i_043bea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BEC;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $043BEC: move.l $106f58.l, d1
pub fn i_043bec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BF2;
    cpu.ir = 0x2239;
    let v2 = bus.read32(0x106F58);
    cpu.d[1] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043BF2: move.w $10816a.l, d2
pub fn i_043bf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BF8;
    cpu.ir = 0x3439;
    let v2 = bus.read16(0x10816A) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043BF8: move.w $10816e.l, d3
pub fn i_043bf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BFE;
    cpu.ir = 0x3639;
    let v2 = bus.read16(0x10816E) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043BFE: move.w $108170.l, d4
pub fn i_043bfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C04;
    cpu.ir = 0x3839;
    let v2 = bus.read16(0x108170) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043C04: beq.w $43c36
pub fn i_043c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C08;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x43C36; return 10; }
    8
}
/// $043C08: bmi.w $43c18
pub fn i_043c08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C0C;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x43C18; return 10; }
    8
}
/// $043C0C: move.w $108160.l, d0
pub fn i_043c0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C12;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x108160) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043C12: neg.w d0
pub fn i_043c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C14;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $043C14: bra.w $43c1e
pub fn i_043c14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C18;
    cpu.ir = 0x6000;
    cpu.pc = 0x43C1E;
    10
}
/// $043C18: move.w $108160.l, d0
pub fn i_043c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C1E;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x108160) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043C1E: swap d0
pub fn i_043c1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C20;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043C20: clr.w d0
pub fn i_043c20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C22;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $043C22: move.w $10816c.l, d5
pub fn i_043c22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C28;
    cpu.ir = 0x3A39;
    let v2 = bus.read16(0x10816C) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043C28: sub.w $106f50.l, d5
pub fn i_043c28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C2E;
    cpu.ir = 0x9A79;
    let v2 = bus.read16(0x106F50) as u32;
    let h1 = cpu.d[5] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $043C2E: muls.w d5, d4
pub fn i_043c2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C30;
    cpu.ir = 0xC9C5;
    let v1 = cpu.d[5] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[4] as u16 as i16 as i32)) as u32;
    cpu.d[4] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $043C30: asr.l #$8, d4
pub fn i_043c30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C32;
    cpu.ir = 0xE084;
    let h1 = cpu.d[4] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[4] = r;
    22
}
/// $043C32: add.w d4, d2
pub fn i_043c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C34;
    cpu.ir = 0xD444;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $043C34: add.w d4, d3
pub fn i_043c34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C36;
    cpu.ir = 0xD644;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $043C36: bsr.w $43e3a
pub fn i_043c36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C3A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x43C3A);
    cpu.pc = 0x43E3A;
    18
}
/// $043C3A: move.w d0, $108162.l
pub fn i_043c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C40;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x108162, v1 as u16);
    16
}
/// $043C40: move.l d1, $106f58.l
pub fn i_043c40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C46;
    cpu.ir = 0x23C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    bus.write32(0x106F58, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043C46: move.b $108178.l, d0
pub fn i_043c46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C4C;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x108178) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $043C4C: cmpi.b #$3, d0
pub fn i_043c4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C50;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $043C50: bne.w $43c64
pub fn i_043c50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C54;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x43C64; return 10; }
    8
}
/// $043C54: move.w $106f58.l, d1
pub fn i_043c54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C5A;
    cpu.ir = 0x3239;
    let v2 = bus.read16(0x106F58) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043C5A: sub.w $108176.l, d1
pub fn i_043c5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C60;
    cpu.ir = 0x9279;
    let v2 = bus.read16(0x108176) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $043C60: bra.w $43ca0
pub fn i_043c60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C64;
    cpu.ir = 0x6000;
    cpu.pc = 0x43CA0;
    10
}
/// $043C64: cmpi.b #$2, d0
pub fn i_043c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C68;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $043C68: bne.w $43c7c
pub fn i_043c68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C6C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x43C7C; return 10; }
    8
}
/// $043C6C: move.w $108176.l, d1
pub fn i_043c6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C72;
    cpu.ir = 0x3239;
    let v2 = bus.read16(0x108176) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043C72: sub.w $106f58.l, d1
pub fn i_043c72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C78;
    cpu.ir = 0x9279;
    let v2 = bus.read16(0x106F58) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $043C78: bra.w $43ca0
pub fn i_043c78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C7C;
    cpu.ir = 0x6000;
    cpu.pc = 0x43CA0;
    10
}
/// $043C7C: cmpi.b #$0, d0
pub fn i_043c7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C80;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $043C80: bne.w $43c94
pub fn i_043c80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C84;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x43C94; return 10; }
    8
}
/// $043C84: move.w $108174.l, d1
pub fn i_043c84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C8A;
    cpu.ir = 0x3239;
    let v2 = bus.read16(0x108174) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043C8A: sub.w $106f50.l, d1
pub fn i_043c8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C90;
    cpu.ir = 0x9279;
    let v2 = bus.read16(0x106F50) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $043C90: bra.w $43ca0
pub fn i_043c90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C94;
    cpu.ir = 0x6000;
    cpu.pc = 0x43CA0;
    10
}
/// $043C94: move.w $106f50.l, d1
pub fn i_043c94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C9A;
    cpu.ir = 0x3239;
    let v2 = bus.read16(0x106F50) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043C9A: sub.w $108174.l, d1
pub fn i_043c9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CA0;
    cpu.ir = 0x9279;
    let v2 = bus.read16(0x108174) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $043CA0: add.w $108172.l, d1
pub fn i_043ca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CA6;
    cpu.ir = 0xD279;
    let v2 = bus.read16(0x108172) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $043CA6: cmp.w $106f5c.l, d1
pub fn i_043ca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CAC;
    cpu.ir = 0xB279;
    let v2 = bus.read16(0x106F5C) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    16
}
/// $043CAC: bls.w $43cb6
pub fn i_043cac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CB0;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x43CB6; return 10; }
    8
}
/// $043CB0: move.w d1, $106f5c.l
pub fn i_043cb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CB6;
    cpu.ir = 0x33C1;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x106F5C, v1 as u16);
    16
}
/// $043CB6: bsr.w $434f8
pub fn i_043cb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CBA;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x43CBA);
    cpu.pc = 0x434F8;
    18
}
/// $043CBA: move.l $10817a.l, d0
pub fn i_043cba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CC0;
    cpu.ir = 0x2039;
    let v2 = bus.read32(0x10817A);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043CC0: add.l $106f58.l, d0
pub fn i_043cc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CC6;
    cpu.ir = 0xD0B9;
    let v2 = bus.read32(0x106F58);
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Long, true);
    cpu.d[0] = r;
    20
}
/// $043CC6: move.l d0, $106f54.l
pub fn i_043cc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CCC;
    cpu.ir = 0x23C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    bus.write32(0x106F54, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043CCC: move.w $108182.l, d0
pub fn i_043ccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CD2;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x108182) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043CD2: add.w d0, $108162.l
pub fn i_043cd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CD8;
    cpu.ir = 0xD179;
    let v1 = bus.read16(0x108162) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, h1, Size::Word, true);
    bus.write16(0x108162, r2 as u16);
    16
}
/// $043CD8: jsr $4cb88.l
pub fn i_043cd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CDE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x43CDE);
    cpu.pc = 0x4CB88;
    16
}
/// $043CDE: bra.w $43d86
pub fn i_043cde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CE2;
    cpu.ir = 0x6000;
    cpu.pc = 0x43D86;
    10
}
// ===== SceneScript_EdgeArrivalTest_043CE2  ($043CE2, 126 B, mission/scene_script_vm_0437da.s)
/// $043CE2: tst.b $108179.l
pub fn i_043ce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CE8;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x108179) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $043CE8: bne.w $43d66
pub fn i_043ce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CEC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x43D66; return 10; }
    8
}
/// $043CEC: tst.b $10e39a.l
pub fn i_043cec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CF2;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10E39A) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $043CF2: beq.w $43d66
pub fn i_043cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CF6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x43D66; return 10; }
    8
}
/// $043CF6: move.b $108178.l, d0
pub fn i_043cf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CFC;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x108178) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $043CFC: cmpi.b #$3, d0
pub fn i_043cfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D00;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $043D00: bne.w $43d18
pub fn i_043d00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D04;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x43D18; return 10; }
    8
}
/// $043D04: move.w $106f58.l, d0
pub fn i_043d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D0A;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106F58) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043D0A: cmp.w $10816e.l, d0
pub fn i_043d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D10;
    cpu.ir = 0xB079;
    let v2 = bus.read16(0x10816E) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    16
}
/// $043D10: beq.w $43d66
pub fn i_043d10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D14;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x43D66; return 10; }
    8
}
/// $043D14: bra.w $43d60
pub fn i_043d14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D18;
    cpu.ir = 0x6000;
    cpu.pc = 0x43D60;
    10
}
/// $043D18: cmpi.b #$2, d0
pub fn i_043d18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D1C;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $043D1C: bne.w $43d34
pub fn i_043d1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D20;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x43D34; return 10; }
    8
}
/// $043D20: move.w $106f58.l, d0
pub fn i_043d20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D26;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106F58) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043D26: cmp.w $10816a.l, d0
pub fn i_043d26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D2C;
    cpu.ir = 0xB079;
    let v2 = bus.read16(0x10816A) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    16
}
/// $043D2C: beq.w $43d66
pub fn i_043d2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D30;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x43D66; return 10; }
    8
}
/// $043D30: bra.w $43d60
pub fn i_043d30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D34;
    cpu.ir = 0x6000;
    cpu.pc = 0x43D60;
    10
}
/// $043D34: cmpi.b #$0, d0
pub fn i_043d34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D38;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $043D38: bne.w $43d50
pub fn i_043d38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D3C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x43D50; return 10; }
    8
}
/// $043D3C: move.w $106f50.l, d0
pub fn i_043d3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D42;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106F50) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043D42: cmp.w $108168.l, d0
pub fn i_043d42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D48;
    cpu.ir = 0xB079;
    let v2 = bus.read16(0x108168) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    16
}
/// $043D48: beq.w $43d66
pub fn i_043d48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D4C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x43D66; return 10; }
    8
}
/// $043D4C: bra.w $43d60
pub fn i_043d4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D50;
    cpu.ir = 0x6000;
    cpu.pc = 0x43D60;
    10
}
/// $043D50: move.w $106f50.l, d0
pub fn i_043d50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D56;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106F50) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043D56: cmp.w $10816c.l, d0
pub fn i_043d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D5C;
    cpu.ir = 0xB079;
    let v2 = bus.read16(0x10816C) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    16
}
/// $043D5C: beq.w $43d66
pub fn i_043d5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D60;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x43D66; return 10; }
    8
}
// ===== ClearC_043d60  ($043D60, 6 B, thunks/ccr_helpers.c)
/// $043D60: andi.b #$fe, ccr
pub fn i_043d60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D64;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $043D64: rts
pub fn i_043d64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D66;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetC_043d66  ($043D66, 6 B, thunks/ccr_helpers.c)
/// $043D66: ori.b #$1, ccr
pub fn i_043d66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D6A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $043D6A: rts
pub fn i_043d6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D6C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Reset4CameraLongs_043D6C  ($043D6C, 26 B, video/camera_list_ctx_helpers_025012.s)
/// $043D6C: clr.l $106f7a.l
pub fn i_043d6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D72;
    cpu.ir = 0x42B9;
    let v1 = bus.read32(0x106F7A);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write32(0x106F7A, r2);
    20
}
/// $043D72: clr.l $107ff6.l
pub fn i_043d72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D78;
    cpu.ir = 0x42B9;
    let v1 = bus.read32(0x107FF6);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write32(0x107FF6, r2);
    20
}
/// $043D78: clr.l $108072.l
pub fn i_043d78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D7E;
    cpu.ir = 0x42B9;
    let v1 = bus.read32(0x108072);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write32(0x108072, r2);
    20
}
/// $043D7E: clr.l $1080ee.l
pub fn i_043d7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D84;
    cpu.ir = 0x42B9;
    let v1 = bus.read32(0x1080EE);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write32(0x1080EE, r2);
    20
}
/// $043D84: rts
pub fn i_043d84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D86;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== CameraApplyAll4_043D86  ($043D86, 36 B, video/camera_list_ctx_helpers_025012.s)
/// $043D86: lea.l $106f6c.l, a0
pub fn i_043d86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D8C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x106F6C;
    20
}
/// $043D8C: bsr.w $43daa
pub fn i_043d8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D90;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x43D90);
    cpu.pc = 0x43DAA;
    18
}
/// $043D90: lea.l $107fe8.l, a0
pub fn i_043d90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D96;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x107FE8;
    20
}
/// $043D96: bsr.w $43daa
pub fn i_043d96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D9A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x43D9A);
    cpu.pc = 0x43DAA;
    18
}
/// $043D9A: lea.l $108064.l, a0
pub fn i_043d9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43DA0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x108064;
    20
}
/// $043DA0: bsr.w $43daa
pub fn i_043da0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43DA4;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x43DA4);
    cpu.pc = 0x43DAA;
    18
}
/// $043DA4: lea.l $1080e0.l, a0
pub fn i_043da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43DAA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1080E0;
    20
}
// ===== CameraApplyOne_043DAA  ($043DAA, 74 B, video/camera_apply_043daa.s)
/// $043DAA: tst.l $e(a0)
pub fn i_043daa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43DAE;
    cpu.ir = 0x4AA8;
    let ad2 = cpu.a[0].wrapping_add(0xE);
    let v3 = bus.read32(ad2);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $043DAE: beq.w $43df2
pub fn i_043dae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43DB2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x43DF2; return 10; }
    8
}
/// $043DB2: move.w $108160.l, d0
pub fn i_043db2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43DB8;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x108160) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043DB8: move.w $108162.l, d1
pub fn i_043db8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43DBE;
    cpu.ir = 0x3239;
    let v2 = bus.read16(0x108162) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043DBE: muls.w $74(a0), d0
pub fn i_043dbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43DC2;
    cpu.ir = 0xC1E8;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    let r = ((v3 as u16 as i16 as i32).wrapping_mul(cpu.d[0] as u16 as i16 as i32)) as u32;
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    78
}
/// $043DC2: asl.l #$8, d0
pub fn i_043dc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43DC4;
    cpu.ir = 0xE180;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, true);
    cpu.d[0] = r;
    22
}
/// $043DC4: muls.w $76(a0), d1
pub fn i_043dc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43DC8;
    cpu.ir = 0xC3E8;
    let ad2 = cpu.a[0].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    let r = ((v3 as u16 as i16 as i32).wrapping_mul(cpu.d[1] as u16 as i16 as i32)) as u32;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    78
}
/// $043DC8: asl.l #$8, d1
pub fn i_043dc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43DCA;
    cpu.ir = 0xE181;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, true);
    cpu.d[1] = r;
    22
}
/// $043DCA: jsr $51b80.l
pub fn i_043dca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43DD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x43DD0);
    cpu.pc = 0x51B80;
    16
}
/// $043DD0: btst.b #$0, $72(a0)
pub fn i_043dd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43DD6;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $043DD6: beq.w $43dde
pub fn i_043dd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43DDA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x43DDE; return 10; }
    8
}
/// $043DDA: bsr.w $43e0e
pub fn i_043dda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43DDE;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x43DDE);
    cpu.pc = 0x43E0E;
    18
}
/// $043DDE: btst.b #$1, $72(a0)
pub fn i_043dde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43DE4;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $043DE4: beq.w $43dec
pub fn i_043de4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43DE8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x43DEC; return 10; }
    8
}
/// $043DE8: bsr.w $43e24
pub fn i_043de8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43DEC;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x43DEC);
    cpu.pc = 0x43E24;
    18
}
/// $043DEC: jsr $51f30.l
pub fn i_043dec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43DF2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x43DF2);
    cpu.pc = 0x51F30;
    16
}
/// $043DF2: rts
pub fn i_043df2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43DF4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== CameraHook_Probe08_043DF4  ($043DF4, 26 B, video/camera_apply_043daa.s)
/// $043DF4: jsr $51c08.l
pub fn i_043df4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43DFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x43DFA);
    cpu.pc = 0x51C08;
    16
}
/// $043DFA: bcc.w $43e0c
pub fn i_043dfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43DFE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x43E0C; return 10; }
    8
}
/// $043DFE: movea.l $78(a0), a1
pub fn i_043dfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E02;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0x78);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $043E02: move.l a1, d7
pub fn i_043e02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E04;
    cpu.ir = 0x2E09;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.d[7] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043E04: beq.w $43e0c
pub fn i_043e04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E08;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x43E0C; return 10; }
    8
}
/// $043E08: bra.w $43e8c
pub fn i_043e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E0C;
    cpu.ir = 0x6000;
    cpu.pc = 0x43E8C;
    10
}
/// $043E0C: rts
pub fn i_043e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E0E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== CameraHook_Probe82_043E0E  ($043E0E, 22 B, video/camera_apply_043daa.s)
/// $043E0E: jsr $51c82.l
pub fn i_043e0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x43E14);
    cpu.pc = 0x51C82;
    16
}
/// $043E14: bcc.b $43e0c
pub fn i_043e14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E16;
    cpu.ir = 0x64F6;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x43E0C; return 10; }
    8
}
/// $043E16: movea.l $78(a0), a1
pub fn i_043e16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E1A;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0x78);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $043E1A: move.l a1, d7
pub fn i_043e1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E1C;
    cpu.ir = 0x2E09;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.d[7] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043E1C: beq.b $43e0c
pub fn i_043e1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E1E;
    cpu.ir = 0x67EE;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x43E0C; return 10; }
    8
}
/// $043E1E: bra.w $43e8c
pub fn i_043e1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E22;
    cpu.ir = 0x6000;
    cpu.pc = 0x43E8C;
    10
}
/// $043E22: rts
pub fn i_043e22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E24;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== CameraHook_ProbeF6_043E24  ($043E24, 22 B, video/camera_apply_043daa.s)
/// $043E24: jsr $51cf6.l
pub fn i_043e24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x43E2A);
    cpu.pc = 0x51CF6;
    16
}
/// $043E2A: bcc.b $43e0c
pub fn i_043e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E2C;
    cpu.ir = 0x64E0;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x43E0C; return 10; }
    8
}
/// $043E2C: movea.l $78(a0), a1
pub fn i_043e2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E30;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0x78);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $043E30: move.l a1, d7
pub fn i_043e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E32;
    cpu.ir = 0x2E09;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.d[7] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043E32: beq.b $43e0c
pub fn i_043e32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E34;
    cpu.ir = 0x67D8;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x43E0C; return 10; }
    8
}
/// $043E34: bra.w $43e8c
pub fn i_043e34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E38;
    cpu.ir = 0x6000;
    cpu.pc = 0x43E8C;
    10
}
/// $043E38: rts
pub fn i_043e38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E3A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Scroll_ClampToRange_043E3A  ($043E3A, 78 B, mission/scene_script_vm_0437da.s)
/// $043E3A: swap d2
pub fn i_043e3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E3C;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043E3C: swap d3
pub fn i_043e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E3E;
    cpu.ir = 0x4843;
    cpu.d[3] = cpu.d[3].rotate_left(16);
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043E3E: clr.w d2
pub fn i_043e3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E40;
    cpu.ir = 0x4242;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $043E40: move.w #$ffff, d3
pub fn i_043e40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E44;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $043E44: tst.l d0
pub fn i_043e44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E46;
    cpu.ir = 0x4A80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043E46: beq.w $43e88
pub fn i_043e46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E4A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x43E88; return 10; }
    8
}
/// $043E4A: bmi.w $43e5a
pub fn i_043e4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E4E;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x43E5A; return 10; }
    8
}
/// $043E4E: cmp.l d1, d3
pub fn i_043e4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E50;
    cpu.ir = 0xB681;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let h1 = cpu.d[3] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    4
}
/// $043E50: bcs.w $43e88
pub fn i_043e50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E54;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x43E88; return 10; }
    8
}
/// $043E54: move.l d3, d2
pub fn i_043e54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E56;
    cpu.ir = 0x2403;
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.d[2] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043E56: bra.w $43e60
pub fn i_043e56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E5A;
    cpu.ir = 0x6000;
    cpu.pc = 0x43E60;
    10
}
/// $043E5A: cmp.l d1, d2
pub fn i_043e5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E5C;
    cpu.ir = 0xB481;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    4
}
/// $043E5C: bcc.w $43e88
pub fn i_043e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E60;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x43E88; return 10; }
    8
}
/// $043E60: add.l d1, d0
pub fn i_043e60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E62;
    cpu.ir = 0xD081;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[0] = r;
    4
}
/// $043E62: cmpi.l #$ffffffff, d2
pub fn i_043e62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E68;
    cpu.ir = 0xC82;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0xFFFFFFFF, Size::Long, false);
    8
}
/// $043E68: beq.w $43e7c
pub fn i_043e68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E6C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x43E7C; return 10; }
    8
}
/// $043E6C: move.l d2, d3
pub fn i_043e6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E6E;
    cpu.ir = 0x2602;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[3] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043E6E: move.l d2, d4
pub fn i_043e6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E70;
    cpu.ir = 0x2802;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[4] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043E70: sub.l d0, d3
pub fn i_043e70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E72;
    cpu.ir = 0x9680;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let h1 = cpu.d[3] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[3] = r;
    4
}
/// $043E72: sub.l d1, d4
pub fn i_043e72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E74;
    cpu.ir = 0x9881;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let h1 = cpu.d[4] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[4] = r;
    4
}
/// $043E74: eor.l d3, d4
pub fn i_043e74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E76;
    cpu.ir = 0xB784;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let r2 = (v1 ^ cpu.d[3]) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[4] = r2;
    4
}
/// $043E76: bpl.w $43e7c
pub fn i_043e76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E7A;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x43E7C; return 10; }
    8
}
/// $043E7A: move.l d2, d0
pub fn i_043e7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E7C;
    cpu.ir = 0x2002;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043E7C: move.l d0, d2
pub fn i_043e7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E7E;
    cpu.ir = 0x2400;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[2] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043E7E: swap d0
pub fn i_043e7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E80;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043E80: swap d1
pub fn i_043e80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E82;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043E82: sub.w d1, d0
pub fn i_043e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E84;
    cpu.ir = 0x9041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $043E84: move.l d2, d1
pub fn i_043e84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E86;
    cpu.ir = 0x2202;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[1] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043E86: rts
pub fn i_043e86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E88;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClrWD0_00043E88  ($043E88, 4 B, thunks/trivial_returns.c)
/// $043E88: clr.w d0
pub fn i_043e88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E8A;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $043E8A: rts
pub fn i_043e8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E8C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== BlitterTile_2D_043E8C  ($043E8C, 78 B, video/camera_sprite_helpers_043e8c.s)
/// $043E8C: add.w $22(a0), d0
pub fn i_043e8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E90;
    cpu.ir = 0xD068;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $043E90: add.w $24(a0), d1
pub fn i_043e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E94;
    cpu.ir = 0xD268;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $043E94: mulu.w d6, d2
pub fn i_043e94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E96;
    cpu.ir = 0xC4C6;
    let v1 = cpu.d[6] & 0xFFFF;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[2] & 0xFFFF);
    cpu.d[2] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $043E96: add.w d3, d2
pub fn i_043e96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E98;
    cpu.ir = 0xD443;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $043E98: sub.w d5, d6
pub fn i_043e98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E9A;
    cpu.ir = 0x9C45;
    let v1 = cpu.d[5] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $043E9A: add.w d2, d2
pub fn i_043e9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E9C;
    cpu.ir = 0xD442;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $043E9C: add.w d6, d6
pub fn i_043e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E9E;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $043E9E: add.w d2, d2
pub fn i_043e9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43EA0;
    cpu.ir = 0xD442;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $043EA0: add.w d6, d6
pub fn i_043ea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43EA2;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $043EA2: lea.l (a1, d2.w), a1
pub fn i_043ea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43EA6;
    cpu.ir = 0x43F1;
    let ad1 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    cpu.a[1] = ad1;
    18
}
/// $043EA6: lea.l $7c(a0), a2
pub fn i_043ea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43EAA;
    cpu.ir = 0x45E8;
    let ad1 = cpu.a[0].wrapping_add(0x7C);
    cpu.a[2] = ad1;
    16
}
/// $043EAA: lsl.w #$7, d0
pub fn i_043eaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43EAC;
    cpu.ir = 0xEF48;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 7, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    20
}
/// $043EAC: lsl.w #$2, d1
pub fn i_043eac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43EAE;
    cpu.ir = 0xE549;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $043EAE: subq.w #$1, d4
pub fn i_043eae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43EB0;
    cpu.ir = 0x5344;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $043EB0: subq.w #$1, d5
pub fn i_043eb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43EB2;
    cpu.ir = 0x5345;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $043EB2: move.w d5, d2
pub fn i_043eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43EB4;
    cpu.ir = 0x3405;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $043EB4: move.w d1, d3
pub fn i_043eb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43EB6;
    cpu.ir = 0x3601;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $043EB6: andi.w #$f80, d0
pub fn i_043eb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43EBA;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF80) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $043EBA: lea.l (a2, d0.w), a3
pub fn i_043eba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43EBE;
    cpu.ir = 0x47F2;
    let ad1 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    cpu.a[3] = ad1;
    18
}
/// $043EBE: andi.w #$7c, d3
pub fn i_043ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43EC2;
    cpu.ir = 0x243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = (v1 & 0x7C) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $043EC2: move.l (a1)+, (a3, d3.w)
pub fn i_043ec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43EC6;
    cpu.ir = 0x2799;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[3].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $043EC6: addq.w #$4, d3
pub fn i_043ec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43EC8;
    cpu.ir = 0x5843;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $043EC8: dbra d2, $43ebe
pub fn i_043ec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43ECC;
    cpu.ir = 0x51CA;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x43ECC; return 12; }
    let n = (cpu.d[2] as u16).wrapping_sub(1);
    cpu.d[2] = (cpu.d[2] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x43ECC; return 14; }
    cpu.pc = 0x43EBE;
    10
}
/// $043ECC: lea.l (a1, d6.w), a1
pub fn i_043ecc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43ED0;
    cpu.ir = 0x43F1;
    let ad1 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    cpu.a[1] = ad1;
    18
}
/// $043ED0: addi.w #$80, d0
pub fn i_043ed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43ED4;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x80, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $043ED4: dbra d4, $43eb2
pub fn i_043ed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43ED8;
    cpu.ir = 0x51CC;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x43ED8; return 12; }
    let n = (cpu.d[4] as u16).wrapping_sub(1);
    cpu.d[4] = (cpu.d[4] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x43ED8; return 14; }
    cpu.pc = 0x43EB2;
    10
}
/// $043ED8: rts
pub fn i_043ed8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43EDA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Buffer_ClearBlock1024L_043EDA  ($043EDA, 18 B, video/coord_camera_043eda.s)
/// $043EDA: lea.l $7c(a0), a1
pub fn i_043eda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43EDE;
    cpu.ir = 0x43E8;
    let ad1 = cpu.a[0].wrapping_add(0x7C);
    cpu.a[1] = ad1;
    16
}
/// $043EDE: move.l #$3ff, d7
pub fn i_043ede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43EE4;
    cpu.ir = 0x2E3C;
    let v1: u32 = 0x3FF;
    cpu.d[7] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043EE4: clr.l (a1)+
pub fn i_043ee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43EE6;
    cpu.ir = 0x4299;
    let ad1 = cpu.a[1]; cpu.a[1] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $043EE6: dbra d7, $43ee4
pub fn i_043ee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43EEA;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x43EEA; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x43EEA; return 14; }
    cpu.pc = 0x43EE4;
    10
}
/// $043EEA: rts
pub fn i_043eea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43EEC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== CollMap_RowToLocalY_043EEC  ($043EEC, 22 B, mission/scene_script_vm_0437da.s)
/// $043EEC: lea.l $106f6c.l, a0
pub fn i_043eec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43EF2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x106F6C;
    20
}
/// $043EF2: move.w $24(a0), d2
pub fn i_043ef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43EF6;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $043EF6: add.w d2, d0
pub fn i_043ef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43EF8;
    cpu.ir = 0xD042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $043EF8: lsl.w #$4, d0
pub fn i_043ef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43EFA;
    cpu.ir = 0xE948;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $043EFA: sub.w $8(a0), d0
pub fn i_043efa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43EFE;
    cpu.ir = 0x9068;
    let ad2 = cpu.a[0].wrapping_add(0x8);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $043EFE: neg.w d0
pub fn i_043efe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F00;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $043F00: rts
pub fn i_043f00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F02;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== CollMap_TestPoint_043F02  ($043F02, 80 B, mission/scene_script_vm_0437da.s)
/// $043F02: lea.l $106f6c.l, a0
pub fn i_043f02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F08;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x106F6C;
    20
}
/// $043F08: add.w $4(a0), d0
pub fn i_043f08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F0C;
    cpu.ir = 0xD068;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $043F0C: neg.w d1
pub fn i_043f0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F0E;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $043F0E: add.w $8(a0), d1
pub fn i_043f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F12;
    cpu.ir = 0xD268;
    let ad2 = cpu.a[0].wrapping_add(0x8);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $043F12: move.w $22(a0), d2
pub fn i_043f12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F16;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $043F16: lsl.w #$4, d2
pub fn i_043f16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F18;
    cpu.ir = 0xE94A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $043F18: add.w d2, d0
pub fn i_043f18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F1A;
    cpu.ir = 0xD042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $043F1A: move.w d0, d3
pub fn i_043f1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F1C;
    cpu.ir = 0x3600;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $043F1C: andi.w #$7, d3
pub fn i_043f1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F20;
    cpu.ir = 0x243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $043F20: move.w $24(a0), d2
pub fn i_043f20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F24;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $043F24: lsl.w #$4, d2
pub fn i_043f24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F26;
    cpu.ir = 0xE94A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $043F26: add.w d2, d1
pub fn i_043f26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F28;
    cpu.ir = 0xD242;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $043F28: move.w d1, d4
pub fn i_043f28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F2A;
    cpu.ir = 0x3801;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $043F2A: andi.w #$7, d4
pub fn i_043f2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F2E;
    cpu.ir = 0x244;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $043F2E: move.w d0, d2
pub fn i_043f2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F30;
    cpu.ir = 0x3400;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $043F30: andi.w #$1f0, d0
pub fn i_043f30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F34;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F0) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $043F34: andi.w #$8, d2
pub fn i_043f34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F38;
    cpu.ir = 0x242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = (v1 & 0x8) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $043F38: andi.w #$1f8, d1
pub fn i_043f38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F3C;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1F8) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $043F3C: ror.w #$2, d1
pub fn i_043f3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F3E;
    cpu.ir = 0xE459;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 3, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $043F3E: ror.w #$3, d2
pub fn i_043f3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F40;
    cpu.ir = 0xE65A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 3, false);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $043F40: rol.w #$3, d0
pub fn i_043f40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F42;
    cpu.ir = 0xE758;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 3, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $043F42: add.w d1, d0
pub fn i_043f42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F44;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $043F44: add.w d2, d0
pub fn i_043f44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F46;
    cpu.ir = 0xD042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $043F46: lea.l $7c(a0), a0
pub fn i_043f46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F4A;
    cpu.ir = 0x41E8;
    let ad1 = cpu.a[0].wrapping_add(0x7C);
    cpu.a[0] = ad1;
    16
}
/// $043F4A: move.b (a0, d0.w), d0
pub fn i_043f4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F4E;
    cpu.ir = 0x1030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $043F4E: bne.w $43f58
pub fn i_043f4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F52;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x43F58; return 10; }
    8
}
// ===== ClearC_043f52  ($043F52, 6 B, thunks/ccr_helpers.c)
/// $043F52: andi.b #$fe, ccr
pub fn i_043f52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F56;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $043F56: rts
pub fn i_043f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F58;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetC_043f58  ($043F58, 6 B, thunks/ccr_helpers.c)
/// $043F58: ori.b #$1, ccr
pub fn i_043f58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F5C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $043F5C: rts
pub fn i_043f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F5E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FixBlit_TileByCoord_043F5E  ($043F5E, 78 B, video/coord_camera_043eda.s)
/// $043F5E: lea.l $106f6c.l, a0
pub fn i_043f5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F64;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x106F6C;
    20
}
/// $043F64: add.w $4(a0), d0
pub fn i_043f64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F68;
    cpu.ir = 0xD068;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $043F68: neg.w d1
pub fn i_043f68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F6A;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $043F6A: add.w $8(a0), d1
pub fn i_043f6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F6E;
    cpu.ir = 0xD268;
    let ad2 = cpu.a[0].wrapping_add(0x8);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $043F6E: move.w $22(a0), d2
pub fn i_043f6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F72;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $043F72: lsl.w #$4, d2
pub fn i_043f72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F74;
    cpu.ir = 0xE94A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $043F74: add.w d2, d0
pub fn i_043f74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F76;
    cpu.ir = 0xD042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $043F76: move.w d0, d3
pub fn i_043f76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F78;
    cpu.ir = 0x3600;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $043F78: andi.w #$7, d3
pub fn i_043f78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F7C;
    cpu.ir = 0x243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $043F7C: move.w $24(a0), d2
pub fn i_043f7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F80;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $043F80: lsl.w #$4, d2
pub fn i_043f80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F82;
    cpu.ir = 0xE94A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $043F82: add.w d2, d1
pub fn i_043f82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F84;
    cpu.ir = 0xD242;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $043F84: move.w d1, d4
pub fn i_043f84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F86;
    cpu.ir = 0x3801;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $043F86: andi.w #$7, d4
pub fn i_043f86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F8A;
    cpu.ir = 0x244;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $043F8A: move.w d0, d2
pub fn i_043f8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F8C;
    cpu.ir = 0x3400;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $043F8C: andi.w #$1f0, d0
pub fn i_043f8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F90;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F0) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $043F90: andi.w #$8, d2
pub fn i_043f90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F94;
    cpu.ir = 0x242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = (v1 & 0x8) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $043F94: andi.w #$1f8, d1
pub fn i_043f94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F98;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1F8) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $043F98: ror.w #$2, d1
pub fn i_043f98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F9A;
    cpu.ir = 0xE459;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 3, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $043F9A: ror.w #$3, d2
pub fn i_043f9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F9C;
    cpu.ir = 0xE65A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 3, false);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $043F9C: rol.w #$3, d0
pub fn i_043f9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F9E;
    cpu.ir = 0xE758;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 3, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $043F9E: add.w d1, d0
pub fn i_043f9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FA0;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $043FA0: add.w d2, d0
pub fn i_043fa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FA2;
    cpu.ir = 0xD042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $043FA2: lea.l $7c(a0), a0
pub fn i_043fa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FA6;
    cpu.ir = 0x41E8;
    let ad1 = cpu.a[0].wrapping_add(0x7C);
    cpu.a[0] = ad1;
    16
}
/// $043FA6: move.b d7, (a0, d0.w)
pub fn i_043fa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FAA;
    cpu.ir = 0x1187;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $043FAA: rts
pub fn i_043faa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FAC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sprite_MultiBlitClippedX  ($043FAC, 98 B, video/sprite_multi_blit_043fac.s)
/// $043FAC: move.w $22(a6), d0
pub fn i_043fac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FB0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $043FB0: move.w $24(a6), d1
pub fn i_043fb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FB4;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $043FB4: move.w (a1), d2
pub fn i_043fb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FB6;
    cpu.ir = 0x3411;
    let v2 = bus.read16(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $043FB6: btst.b #$0, $3a(a6)
pub fn i_043fb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FBC;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $043FBC: beq.w $43fc0
pub fn i_043fbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FC0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x43FC0; return 10; }
    8
}
/// $043FC0: add.w d2, d0
pub fn i_043fc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FC2;
    cpu.ir = 0xD042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $043FC2: add.w $2(a1), d1
pub fn i_043fc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FC6;
    cpu.ir = 0xD269;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $043FC6: move.w $4(a1), d2
pub fn i_043fc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FCA;
    cpu.ir = 0x3429;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $043FCA: move.w $6(a1), d3
pub fn i_043fca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FCE;
    cpu.ir = 0x3629;
    let ad2 = cpu.a[1].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $043FCE: addq.l #$8, a1
pub fn i_043fce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FD0;
    cpu.ir = 0x5089;
    cpu.a[1] = cpu.a[1].wrapping_add(8);
    8
}
/// $043FD0: movem.l d1/d3, -(a7)
pub fn i_043fd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FD4;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $043FD4: cmpi.w #$ffb0, d0
pub fn i_043fd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FD8;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFB0, Size::Word, false);
    8
}
/// $043FD8: blt.w $43ff2
pub fn i_043fd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FDC;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x43FF2; return 10; }
    8
}
/// $043FDC: cmpi.w #$188, d0
pub fn i_043fdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FE0;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x188, Size::Word, false);
    8
}
/// $043FE0: bgt.w $43ff2
pub fn i_043fe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FE4;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x43FF2; return 10; }
    8
}
/// $043FE4: move.b (a1)+, d7
pub fn i_043fe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FE6;
    cpu.ir = 0x1E19;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $043FE6: movem.l d0-d3/a1, -(a7)
pub fn i_043fe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FEA;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $043FEA: jsr $43f5e(pc)
pub fn i_043fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FEE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x43FEE);
    cpu.pc = 0x43F5E;
    16
}
/// $043FEE: movem.l (a7)+, d0-d3/a1
pub fn i_043fee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FF2;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $043FF2: subq.w #$8, d1
pub fn i_043ff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FF4;
    cpu.ir = 0x5141;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $043FF4: subq.w #$1, d3
pub fn i_043ff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FF6;
    cpu.ir = 0x5343;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $043FF6: bne.b $43fd4
pub fn i_043ff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FF8;
    cpu.ir = 0x66DC;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x43FD4; return 10; }
    8
}
/// $043FF8: movem.l (a7)+, d1/d3
pub fn i_043ff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FFC;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $043FFC: addq.w #$8, d0
pub fn i_043ffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FFE;
    cpu.ir = 0x5040;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $043FFE: btst.b #$0, $3a(a6)
pub fn i_043ffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44004;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $044004: beq.w $44008
pub fn i_044004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44008;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x44008; return 10; }
    8
}
/// $044008: subq.w #$1, d2
pub fn i_044008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4400A;
    cpu.ir = 0x5342;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $04400A: bne.b $43fd0
pub fn i_04400a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4400C;
    cpu.ir = 0x66C4;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x43FD0; return 10; }
    8
}
/// $04400C: rts
pub fn i_04400c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4400E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Coord_LocalToScreen_04400E  ($04400E, 20 B, video/coord_camera_043eda.s)
/// $04400E: add.w $106f70.l, d0
pub fn i_04400e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44014;
    cpu.ir = 0xD079;
    let v2 = bus.read16(0x106F70) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $044014: neg.w d1
pub fn i_044014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44016;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $044016: addi.w #$200, d1
pub fn i_044016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4401A;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x200, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04401A: add.w $106f74.l, d1
pub fn i_04401a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44020;
    cpu.ir = 0xD279;
    let v2 = bus.read16(0x106F74) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $044020: rts
pub fn i_044020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44022;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Coord_ScreenToLocal_044022  ($044022, 20 B, video/coord_camera_043eda.s)
/// $044022: sub.w $106f70.l, d0
pub fn i_044022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44028;
    cpu.ir = 0x9079;
    let v2 = bus.read16(0x106F70) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $044028: sub.w $106f74.l, d1
pub fn i_044028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4402E;
    cpu.ir = 0x9279;
    let v2 = bus.read16(0x106F74) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $04402E: subi.w #$200, d1
pub fn i_04402e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44032;
    cpu.ir = 0x441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x200, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $044032: neg.w d1
pub fn i_044032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44034;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $044034: rts
pub fn i_044034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44036;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Coord_ResetAndClearBuf_044036  ($044036, 18 B, video/coord_camera_043eda.s)
/// $044036: lea.l $106f6c.l, a0
pub fn i_044036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4403C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x106F6C;
    20
}
/// $04403C: bsr.w $43eda
pub fn i_04403c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44040;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x44040);
    cpu.pc = 0x43EDA;
    18
}
/// $044040: clr.w (a0)
pub fn i_044040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44042;
    cpu.ir = 0x4250;
    let ad1 = cpu.a[0];
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $044042: clr.w $2(a0)
pub fn i_044042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44046;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $044046: rts
pub fn i_044046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44048;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Coord_ApplyCameraDeltaToSelf_044048  ($044048, 22 B, video/coord_camera_043eda.s)
/// $044048: move.w $106f6c.l, d0
pub fn i_044048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4404E;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106F6C) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $04404E: sub.w d0, $22(a6)
pub fn i_04404e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44052;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $044052: move.w $106f6e.l, d0
pub fn i_044052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44058;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106F6E) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $044058: add.w d0, $24(a6)
pub fn i_044058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4405C;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04405C: rts
pub fn i_04405c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4405E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Coord_ApplyCameraDeltaToA0Mark_04405E  ($04405E, 28 B, video/coord_camera_043eda.s)
/// $04405E: move.w $106f6c.l, d0
pub fn i_04405e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44064;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106F6C) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $044064: sub.w d0, $22(a0)
pub fn i_044064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44068;
    cpu.ir = 0x9168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $044068: move.w $106f6e.l, d0
pub fn i_044068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4406E;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106F6E) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $04406E: add.w d0, $24(a0)
pub fn i_04406e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44072;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $044072: bset.b #$6, $13(a0)
pub fn i_044072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44078;
    cpu.ir = 0x8E8;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $044078: rts
pub fn i_044078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4407A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Coord_ApplyCameraTerciaryToSelf_04407A  ($04407A, 22 B, video/coord_camera_043eda.s)
/// $04407A: move.w $108064.l, d0
pub fn i_04407a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44080;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x108064) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $044080: sub.w d0, $22(a6)
pub fn i_044080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44084;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $044084: move.w $108066.l, d0
pub fn i_044084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4408A;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x108066) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $04408A: add.w d0, $24(a6)
pub fn i_04408a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4408E;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04408E: rts
pub fn i_04408e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44090;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Coord_ApplyCameraDeltaToGlobals_044090  ($044090, 26 B, video/coord_camera_043eda.s)
/// $044090: move.w $106f6c.l, d0
pub fn i_044090(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44096;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106F6C) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $044096: sub.w d0, $106f38.l
pub fn i_044096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4409C;
    cpu.ir = 0x9179;
    let v1 = bus.read16(0x106F38) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, h1, Size::Word, true);
    bus.write16(0x106F38, r2 as u16);
    16
}
/// $04409C: move.w $106f6e.l, d0
pub fn i_04409c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x440A2;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106F6E) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $0440A2: add.w d0, $106f3a.l
pub fn i_0440a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x440A8;
    cpu.ir = 0xD179;
    let v1 = bus.read16(0x106F3A) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, h1, Size::Word, true);
    bus.write16(0x106F3A, r2 as u16);
    16
}
/// $0440A8: rts
pub fn i_0440a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x440AA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Coord_ApplyCameraDeltaToD1D2_0440AA  ($0440AA, 18 B, video/coord_camera_043eda.s)
/// $0440AA: move.w $106f6c.l, d0
pub fn i_0440aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x440B0;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106F6C) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $0440B0: add.w d0, d1
pub fn i_0440b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x440B2;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0440B2: move.w $106f6e.l, d0
pub fn i_0440b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x440B8;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106F6E) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $0440B8: sub.w d0, d2
pub fn i_0440b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x440BA;
    cpu.ir = 0x9440;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0440BA: rts
pub fn i_0440ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x440BC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Coord_LocalToScreenSecondary_0440BC  ($0440BC, 20 B, video/coord_camera_043eda.s)
/// $0440BC: add.w $106f50.l, d0
pub fn i_0440bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x440C2;
    cpu.ir = 0xD079;
    let v2 = bus.read16(0x106F50) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $0440C2: subi.w #$200, d1
pub fn i_0440c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x440C6;
    cpu.ir = 0x441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x200, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0440C6: neg.w d1
pub fn i_0440c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x440C8;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0440C8: add.w $106f54.l, d1
pub fn i_0440c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x440CE;
    cpu.ir = 0xD279;
    let v2 = bus.read16(0x106F54) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $0440CE: rts
pub fn i_0440ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x440D0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Coord_ScreenToLocalSecondary_0440D0  ($0440D0, 20 B, video/coord_camera_043eda.s)
/// $0440D0: sub.w $106f50.l, d0
pub fn i_0440d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x440D6;
    cpu.ir = 0x9079;
    let v2 = bus.read16(0x106F50) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $0440D6: sub.w $106f54.l, d1
pub fn i_0440d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x440DC;
    cpu.ir = 0x9279;
    let v2 = bus.read16(0x106F54) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $0440DC: neg.w d1
pub fn i_0440dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x440DE;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0440DE: addi.w #$200, d1
pub fn i_0440de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x440E2;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x200, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0440E2: rts
pub fn i_0440e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x440E4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Handler_ApplyCameraSelf_0440E4  ($0440E4, 158 B, video/coord_transform_handlers_0440e4.s)
/// $0440E4: btst.b #$6, $13(a6)
pub fn i_0440e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x440EA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0440EA: bne.w $44180
pub fn i_0440ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x440EE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x44180; return 10; }
    8
}
/// $0440EE: move.w $22(a6), d0
pub fn i_0440ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x440F2;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0440F2: move.w $24(a6), d1
pub fn i_0440f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x440F6;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0440F6: bset.b #$6, $13(a6)
pub fn i_0440f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x440FC;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0440FC: jsr $999de.l
pub fn i_0440fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44102;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44102);
    cpu.pc = 0x999DE;
    16
}
/// $044102: bcs.w $44116
pub fn i_044102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44106;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x44116; return 10; }
    8
}
/// $044106: sub.w $106f6c.l, d0
pub fn i_044106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4410C;
    cpu.ir = 0x9079;
    let v2 = bus.read16(0x106F6C) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $04410C: add.w $106f6e.l, d1
pub fn i_04410c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44112;
    cpu.ir = 0xD279;
    let v2 = bus.read16(0x106F6E) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $044112: bra.w $4415e
pub fn i_044112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44116;
    cpu.ir = 0x6000;
    cpu.pc = 0x4415E;
    10
}
/// $044116: btst.b #$7, $13(a6)
pub fn i_044116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4411C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04411C: beq.w $44128
pub fn i_04411c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44120;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x44128; return 10; }
    8
}
/// $044120: add.w d2, d0
pub fn i_044120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44122;
    cpu.ir = 0xD042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $044122: add.w d3, d1
pub fn i_044122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44124;
    cpu.ir = 0xD243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $044124: bra.w $4415e
pub fn i_044124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44128;
    cpu.ir = 0x6000;
    cpu.pc = 0x4415E;
    10
}
/// $044128: btst.b #$6, $6b(a6)
pub fn i_044128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4412E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x6B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04412E: bne.w $44148
pub fn i_04412e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44132;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x44148; return 10; }
    8
}
/// $044132: btst.b #$1, $6b(a6)
pub fn i_044132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44138;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x6B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $044138: bne.w $44148
pub fn i_044138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4413C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x44148; return 10; }
    8
}
/// $04413C: btst.b #$0, $6b(a6)
pub fn i_04413c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44142;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x6B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $044142: bne.w $44148
pub fn i_044142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44146;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x44148; return 10; }
    8
}
/// $044146: bra.b $44120
pub fn i_044146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44148;
    cpu.ir = 0x60D8;
    cpu.pc = 0x44120;
    10
}
/// $044148: cmpi.w #$1e, $22(a6)
pub fn i_044148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4414E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1E, Size::Word, false);
    16
}
/// $04414E: sgt.b d4
pub fn i_04414e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44150;
    cpu.ir = 0x5EC4;
    let c1: u32 = if cpu.sr.evaluate(Condition::Gt) { 0xFF } else { 0 };
    let v2 = cpu.d[4] & 0xFF;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (c1 & 0xFF);
    4
}
/// $044150: tst.w d2
pub fn i_044150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44152;
    cpu.ir = 0x4A42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $044152: spl.b d5
pub fn i_044152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44154;
    cpu.ir = 0x5AC5;
    let c1: u32 = if cpu.sr.evaluate(Condition::Pl) { 0xFF } else { 0 };
    let v2 = cpu.d[5] & 0xFF;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (c1 & 0xFF);
    4
}
/// $044154: eor.b d4, d5
pub fn i_044154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44156;
    cpu.ir = 0xB905;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 ^ cpu.d[4]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    4
}
/// $044156: beq.w $4415c
pub fn i_044156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4415A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4415C; return 10; }
    8
}
/// $04415A: add.w d2, d0
pub fn i_04415a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4415C;
    cpu.ir = 0xD042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $04415C: add.w d3, d1
pub fn i_04415c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4415E;
    cpu.ir = 0xD243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $04415E: cmpi.b #$1, $106ece.l
pub fn i_04415e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44166;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $044166: bne.w $44178
pub fn i_044166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4416A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x44178; return 10; }
    8
}
/// $04416A: tst.w $106f5e.l
pub fn i_04416a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44170;
    cpu.ir = 0x4A79;
    let v2 = bus.read16(0x106F5E) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    16
}
/// $044170: beq.w $44178
pub fn i_044170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44174;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x44178; return 10; }
    8
}
/// $044174: move.w $22(a6), d0
pub fn i_044174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44178;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $044178: move.w d0, $22(a6)
pub fn i_044178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4417C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04417C: move.w d1, $24(a6)
pub fn i_04417c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44180;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $044180: rts
pub fn i_044180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44182;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Handler_ApplyCameraGlobals_044182  ($044182, 168 B, video/coord_transform_handlers_0440e4.s)
/// $044182: btst.b #$6, $13(a6)
pub fn i_044182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44188;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $044188: bne.w $44228
pub fn i_044188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4418C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x44228; return 10; }
    8
}
/// $04418C: move.w $106f38.l, d0
pub fn i_04418c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44192;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106F38) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $044192: move.w $106f3a.l, d1
pub fn i_044192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44198;
    cpu.ir = 0x3239;
    let v2 = bus.read16(0x106F3A) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $044198: bset.b #$6, $13(a6)
pub fn i_044198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4419E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04419E: jsr $999de.l
pub fn i_04419e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x441A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x441A4);
    cpu.pc = 0x999DE;
    16
}
/// $0441A4: bcs.w $441b8
pub fn i_0441a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x441A8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x441B8; return 10; }
    8
}
/// $0441A8: sub.w $106f6c.l, d0
pub fn i_0441a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x441AE;
    cpu.ir = 0x9079;
    let v2 = bus.read16(0x106F6C) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $0441AE: add.w $106f6e.l, d1
pub fn i_0441ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x441B4;
    cpu.ir = 0xD279;
    let v2 = bus.read16(0x106F6E) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $0441B4: bra.w $44200
pub fn i_0441b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x441B8;
    cpu.ir = 0x6000;
    cpu.pc = 0x44200;
    10
}
/// $0441B8: btst.b #$7, $13(a6)
pub fn i_0441b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x441BE;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0441BE: beq.w $441ca
pub fn i_0441be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x441C2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x441CA; return 10; }
    8
}
/// $0441C2: add.w d2, d0
pub fn i_0441c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x441C4;
    cpu.ir = 0xD042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0441C4: add.w d3, d1
pub fn i_0441c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x441C6;
    cpu.ir = 0xD243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0441C6: bra.w $44200
pub fn i_0441c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x441CA;
    cpu.ir = 0x6000;
    cpu.pc = 0x44200;
    10
}
/// $0441CA: btst.b #$6, $6b(a6)
pub fn i_0441ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x441D0;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x6B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0441D0: bne.w $441ea
pub fn i_0441d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x441D4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x441EA; return 10; }
    8
}
/// $0441D4: btst.b #$1, $6b(a6)
pub fn i_0441d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x441DA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x6B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0441DA: bne.w $441ea
pub fn i_0441da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x441DE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x441EA; return 10; }
    8
}
/// $0441DE: btst.b #$0, $6b(a6)
pub fn i_0441de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x441E4;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x6B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0441E4: bne.w $441ea
pub fn i_0441e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x441E8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x441EA; return 10; }
    8
}
/// $0441E8: bra.b $441c2
pub fn i_0441e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x441EA;
    cpu.ir = 0x60D8;
    cpu.pc = 0x441C2;
    10
}
/// $0441EA: cmpi.w #$1e, $22(a6)
pub fn i_0441ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x441F0;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1E, Size::Word, false);
    16
}
/// $0441F0: sgt.b d4
pub fn i_0441f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x441F2;
    cpu.ir = 0x5EC4;
    let c1: u32 = if cpu.sr.evaluate(Condition::Gt) { 0xFF } else { 0 };
    let v2 = cpu.d[4] & 0xFF;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (c1 & 0xFF);
    4
}
/// $0441F2: tst.w d2
pub fn i_0441f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x441F4;
    cpu.ir = 0x4A42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $0441F4: spl.b d5
pub fn i_0441f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x441F6;
    cpu.ir = 0x5AC5;
    let c1: u32 = if cpu.sr.evaluate(Condition::Pl) { 0xFF } else { 0 };
    let v2 = cpu.d[5] & 0xFF;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (c1 & 0xFF);
    4
}
/// $0441F6: eor.b d4, d5
pub fn i_0441f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x441F8;
    cpu.ir = 0xB905;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 ^ cpu.d[4]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    4
}
/// $0441F8: beq.w $441fe
pub fn i_0441f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x441FC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x441FE; return 10; }
    8
}
/// $0441FC: add.w d2, d0
pub fn i_0441fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x441FE;
    cpu.ir = 0xD042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0441FE: add.w d3, d1
pub fn i_0441fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44200;
    cpu.ir = 0xD243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $044200: cmpi.b #$1, $106ece.l
pub fn i_044200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44208;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $044208: bne.w $4421c
pub fn i_044208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4420C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4421C; return 10; }
    8
}
/// $04420C: tst.w $106f5e.l
pub fn i_04420c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44212;
    cpu.ir = 0x4A79;
    let v2 = bus.read16(0x106F5E) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    16
}
/// $044212: beq.w $4421c
pub fn i_044212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44216;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4421C; return 10; }
    8
}
/// $044216: move.w $106f38.l, d0
pub fn i_044216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4421C;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106F38) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $04421C: move.w d0, $106f38.l
pub fn i_04421c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44222;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x106F38, v1 as u16);
    16
}
/// $044222: move.w d1, $106f3a.l
pub fn i_044222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44228;
    cpu.ir = 0x33C1;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x106F3A, v1 as u16);
    16
}
/// $044228: rts
pub fn i_044228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4422A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionBoot_Run_044236  ($044236, 16 B, mission/event_vm_04422a.s)
/// $044236: lea.l $4422e(pc), a0
pub fn i_044236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4423A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4422E;
    16
}
/// $04423A: jsr $43562(pc)
pub fn i_04423a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4423E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4423E);
    cpu.pc = 0x43562;
    16
}
/// $04423E: moveq #$0, d0
pub fn i_04423e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44240;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $044240: moveq #$0, d1
pub fn i_044240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44242;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $044242: jmp $437da(pc)
pub fn i_044242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44246;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x437DA;
    8
}
// ===== Entity_CmpDepthToParent_044246  ($044246, 16 B, mission/event_vm_04422a.s)
/// $044246: movea.l $8(a6), a1
pub fn i_044246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4424A;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $04424A: move.b $10(a6), d0
pub fn i_04424a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4424E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04424E: cmp.b $10(a1), d0
pub fn i_04424e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44252;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $044252: bcs.w $4425c
pub fn i_044252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44256;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x4425C; return 10; }
    8
}
// ===== ClearXN_044256  ($044256, 6 B, thunks/ccr_helpers.c)
/// $044256: andi.b #$ee, ccr
pub fn i_044256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4425A;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $04425A: rts
pub fn i_04425a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4425C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_04425c  ($04425C, 6 B, thunks/ccr_helpers.c)
/// $04425C: ori.b #$11, ccr
pub fn i_04425c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44260;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $044260: rts
pub fn i_044260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44262;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionWatch_Spawn_04429E  ($04429E, 38 B, mission/event_vm_04422a.s)
/// $04429E: move.l a1, -(a7)
pub fn i_04429e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x442A0;
    cpu.ir = 0x2F09;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $0442A0: move.w d0, -(a7)
pub fn i_0442a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x442A2;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $0442A2: lea.l $442c4(pc), a1
pub fn i_0442a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x442A6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x442C4;
    16
}
/// $0442A6: jsr $4ae.l
pub fn i_0442a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x442AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x442AC);
    cpu.pc = 0x4AE;
    16
}
/// $0442AC: move.w (a7)+, $78(a0)
pub fn i_0442ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x442B0;
    cpu.ir = 0x315F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x78);
    bus.write16(ad4, v3 as u16);
    16
}
/// $0442B0: move.l (a7)+, $70(a0)
pub fn i_0442b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x442B4;
    cpu.ir = 0x215F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0].wrapping_add(0x70);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $0442B4: move.b #$0, $74(a0)
pub fn i_0442b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x442BA;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0442BA: clr.w $76(a0)
pub fn i_0442ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x442BE;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x76);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0442BE: clr.w $7a(a0)
pub fn i_0442be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x442C2;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x7A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0442C2: rts
pub fn i_0442c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x442C4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionWatch_Handler_0442C4  ($0442C4, 16 B, mission/event_vm_04422a.s)
/// $0442C4: movea.l $c(a6), a0
pub fn i_0442c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x442C8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0442C8: move.w $78(a6), d0
pub fn i_0442c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x442CC;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0442CC: tst.b (a0, d0.w)
pub fn i_0442cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x442D0;
    cpu.ir = 0x4A30;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    14
}
/// $0442D0: beq.w $442e2
pub fn i_0442d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x442D4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x442E2; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_0442d4  ($0442D4, 14 B, kernel/jsr5b6_jmp518.c)
/// $0442D4: jsr $5b6.l
pub fn i_0442d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x442DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x442DA);
    cpu.pc = 0x5B6;
    16
}
/// $0442DA: jmp $518.l
pub fn i_0442da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x442E0;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0442E0: rts
pub fn i_0442e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x442E2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionWatch_Resume_0442E2  ($0442E2, 4 B, mission/event_vm_04422a.s)
/// $0442E2: bra.w $44368
pub fn i_0442e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x442E6;
    cpu.ir = 0x6000;
    cpu.pc = 0x44368;
    10
}
// ===== MissionDriver_Init_0442E6  ($0442E6, 130 B, mission/event_vm_04422a.s)
/// $0442E6: moveq #$0, d0
pub fn i_0442e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x442E8;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0442E8: move.b $106ece.l, d0
pub fn i_0442e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x442EE;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106ECE) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $0442EE: bpl.w $44302
pub fn i_0442ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x442F2;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x44302; return 10; }
    8
}
/// $0442F2: jsr $5b6.l
pub fn i_0442f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x442F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x442F8);
    cpu.pc = 0x5B6;
    16
}
/// $0442F8: lea.l $400.l, a1
pub fn i_0442f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x442FE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x400;
    20
}
/// $0442FE: move.l a1, (a6)
pub fn i_0442fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44300;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $044300: rts
pub fn i_044300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44302;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $044302: add.w d0, d0
pub fn i_044302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44304;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $044304: add.w d0, d0
pub fn i_044304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44306;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $044306: lea.l $44266(pc), a0
pub fn i_044306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4430A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x44266;
    16
}
/// $04430A: move.l (a0, d0.w), d0
pub fn i_04430a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4430E;
    cpu.ir = 0x2030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $04430E: cmpi.l #$f1cd4, d0
pub fn i_04430e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44314;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0xF1CD4, Size::Long, false);
    8
}
/// $044314: bne.w $44328
pub fn i_044314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44318;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x44328; return 10; }
    8
}
/// $044318: tst.b $10fd83.l
pub fn i_044318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4431E;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FD83) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $04431E: beq.w $44328
pub fn i_04431e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44322;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x44328; return 10; }
    8
}
/// $044322: move.l #$f260e, d0
pub fn i_044322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44328;
    cpu.ir = 0x203C;
    let v1: u32 = 0xF260E;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $044328: move.l d0, $70(a6)
pub fn i_044328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4432C;
    cpu.ir = 0x2D40;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04432C: move.l a6, -(a7)
pub fn i_04432c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4432E;
    cpu.ir = 0x2F0E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $04432E: lea.l $1008a0.l, a6
pub fn i_04432e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44334;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0x1008A0;
    20
}
/// $044334: jsr $8c85c.l
pub fn i_044334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4433A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4433A);
    cpu.pc = 0x8C85C;
    16
}
/// $04433A: lea.l $8c864.l, a1
pub fn i_04433a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44340;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x8C864;
    20
}
/// $044340: jsr $4ae.l
pub fn i_044340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44346;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44346);
    cpu.pc = 0x4AE;
    16
}
/// $044346: lea.l $79298.l, a1
pub fn i_044346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4434C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x79298;
    20
}
/// $04434C: jsr $4ae.l
pub fn i_04434c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44352;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44352);
    cpu.pc = 0x4AE;
    16
}
/// $044352: movea.l (a7)+, a6
pub fn i_044352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44354;
    cpu.ir = 0x2C5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    12
}
/// $044354: move.b #$0, $74(a6)
pub fn i_044354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4435A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04435A: clr.w $76(a6)
pub fn i_04435a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4435E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $04435E: clr.w $7a(a6)
pub fn i_04435e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44362;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x7A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $044362: lea.l $44368(pc), a0
pub fn i_044362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44366;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x44368;
    16
}
/// $044366: move.l a0, (a6)
pub fn i_044366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44368;
    cpu.ir = 0x2C88;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== MissionDriver_Loop_044368  ($044368, 88 B, mission/event_vm_04422a.s)
/// $044368: movea.l $70(a6), a1
pub fn i_044368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4436C;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $04436C: cmpi.b #$0, $74(a6)
pub fn i_04436c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44372;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $044372: bne.w $44396
pub fn i_044372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44376;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x44396; return 10; }
    8
}
/// $044376: move.w $2(a1), d0
pub fn i_044376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4437A;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04437A: cmpi.w #$ffff, d0
pub fn i_04437a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4437E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFFF, Size::Word, false);
    8
}
/// $04437E: beq.w $44390
pub fn i_04437e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44382;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x44390; return 10; }
    8
}
/// $044382: move.w d0, $7a(a6)
pub fn i_044382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44386;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $044386: cmp.w $106f5c.l, d0
pub fn i_044386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4438C;
    cpu.ir = 0xB079;
    let v2 = bus.read16(0x106F5C) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    16
}
/// $04438C: bls.w $443b6
pub fn i_04438c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44390;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x443B6; return 10; }
    8
}
/// $044390: rts
pub fn i_044390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44392;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $044392: bra.w $443b6
pub fn i_044392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44396;
    cpu.ir = 0x6000;
    cpu.pc = 0x443B6;
    10
}
/// $044396: move.w $76(a6), d0
pub fn i_044396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4439A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04439A: beq.w $443a8
pub fn i_04439a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4439E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x443A8; return 10; }
    8
}
/// $04439E: subq.w #$1, d0
pub fn i_04439e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443A0;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0443A0: move.w d0, $76(a6)
pub fn i_0443a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443A4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0443A4: bra.w $443ac
pub fn i_0443a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443A8;
    cpu.ir = 0x6000;
    cpu.pc = 0x443AC;
    10
}
/// $0443A8: move.w $2(a1), d0
pub fn i_0443a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443AC;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0443AC: move.w d0, $76(a6)
pub fn i_0443ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443B0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0443B0: beq.w $443b6
pub fn i_0443b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443B4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x443B6; return 10; }
    8
}
/// $0443B4: rts
pub fn i_0443b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443B6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0443B6: bsr.w $443c0
pub fn i_0443b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443BA;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x443BA);
    cpu.pc = 0x443C0;
    18
}
/// $0443BA: move.l a1, $70(a6)
pub fn i_0443ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443BE;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0443BE: bra.b $4436c
pub fn i_0443be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443C0;
    cpu.ir = 0x60AC;
    cpu.pc = 0x4436C;
    10
}
// ===== MissionVM_ExecOp_0443C0  ($0443C0, 96 B, mission/event_vm_04422a.s)
/// $0443C0: moveq #$0, d0
pub fn i_0443c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443C2;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0443C2: move.b (a1), d0
pub fn i_0443c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443C4;
    cpu.ir = 0x1011;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $0443C4: cmpi.b #$c, d0
pub fn i_0443c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443C8;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xC, Size::Byte, false);
    8
}
/// $0443C8: bls.w $443d8
pub fn i_0443c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443CC;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x443D8; return 10; }
    8
}
/// $0443CC: nop
pub fn i_0443cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443CE;
    cpu.ir = 0x4E71;
    4
}
/// $0443CE: nop
pub fn i_0443ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443D0;
    cpu.ir = 0x4E71;
    4
}
/// $0443D0: cmpi.b #$c, d0
pub fn i_0443d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443D4;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xC, Size::Byte, false);
    8
}
/// $0443D4: nop
pub fn i_0443d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443D6;
    cpu.ir = 0x4E71;
    4
}
/// $0443D6: trap #$f
pub fn i_0443d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443D8;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $0443D8: add.w d0, d0
pub fn i_0443d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443DA;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0443DA: add.w d0, d0
pub fn i_0443da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443DC;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0443DC: lea.l $443e4(pc), a0
pub fn i_0443dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443E0;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x443E4;
    16
}
/// $0443E0: jmp (a0, d0.w)
pub fn i_0443e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443E4;
    cpu.ir = 0x4EF0;
    let ad1 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    cpu.jump_to(ad1);
    8
}
/// $0443E4: bra.w $446a8
pub fn i_0443e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443E8;
    cpu.ir = 0x6000;
    cpu.pc = 0x446A8;
    10
}
/// $0443E8: bra.w $4469e
pub fn i_0443e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443EC;
    cpu.ir = 0x6000;
    cpu.pc = 0x4469E;
    10
}
/// $0443EC: bra.w $4460a
pub fn i_0443ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443F0;
    cpu.ir = 0x6000;
    cpu.pc = 0x4460A;
    10
}
/// $0443F0: bra.w $44638
pub fn i_0443f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443F4;
    cpu.ir = 0x6000;
    cpu.pc = 0x44638;
    10
}
/// $0443F4: bra.w $4446c
pub fn i_0443f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443F8;
    cpu.ir = 0x6000;
    cpu.pc = 0x4446C;
    10
}
/// $0443F8: bra.w $44426
pub fn i_0443f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443FC;
    cpu.ir = 0x6000;
    cpu.pc = 0x44426;
    10
}
/// $0443FC: bra.w $4443a
pub fn i_0443fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44400;
    cpu.ir = 0x6000;
    cpu.pc = 0x4443A;
    10
}
/// $044400: bra.w $44446
pub fn i_044400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44404;
    cpu.ir = 0x6000;
    cpu.pc = 0x44446;
    10
}
/// $044404: bra.w $4445a
pub fn i_044404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44408;
    cpu.ir = 0x6000;
    cpu.pc = 0x4445A;
    10
}
/// $044408: bra.w $44420
pub fn i_044408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4440C;
    cpu.ir = 0x6000;
    cpu.pc = 0x44420;
    10
}
/// $04440C: bra.w $444e8
pub fn i_04440c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44410;
    cpu.ir = 0x6000;
    cpu.pc = 0x444E8;
    10
}
/// $044410: bra.w $4455a
pub fn i_044410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44414;
    cpu.ir = 0x6000;
    cpu.pc = 0x4455A;
    10
}
/// $044414: bra.w $44418
pub fn i_044414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44418;
    cpu.ir = 0x6000;
    cpu.pc = 0x44418;
    10
}
/// $044418: jmp $518.l
pub fn i_044418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4441E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04441E: rts
pub fn i_04441e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44420;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== LeaA1Plus4_044420  ($044420, 6 B, thunks/lea_a1_plus4.c)
/// $044420: lea.l $4(a1), a1
pub fn i_044420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44424;
    cpu.ir = 0x43E9;
    let ad1 = cpu.a[1].wrapping_add(0x4);
    cpu.a[1] = ad1;
    16
}
/// $044424: rts
pub fn i_044424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44426;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionOp05_PauseOn_044426  ($044426, 14 B, mission/event_vm_04422a.s)
/// $044426: move.b #$1, $74(a6)
pub fn i_044426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4442C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04442C: move.b #$1, $108179.l
pub fn i_04442c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44434;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x108179, v1 as u8);
    16
}
// ===== LeaA1Plus4_044434  ($044434, 6 B, thunks/lea_a1_plus4.c)
/// $044434: lea.l $4(a1), a1
pub fn i_044434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44438;
    cpu.ir = 0x43E9;
    let ad1 = cpu.a[1].wrapping_add(0x4);
    cpu.a[1] = ad1;
    16
}
/// $044438: rts
pub fn i_044438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4443A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionOp06_Pause_04443A  ($04443A, 6 B, mission/event_vm_04422a.s)
/// $04443A: move.b #$1, $74(a6)
pub fn i_04443a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44440;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== LeaA1Plus4_044440  ($044440, 6 B, thunks/lea_a1_plus4.c)
/// $044440: lea.l $4(a1), a1
pub fn i_044440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44444;
    cpu.ir = 0x43E9;
    let ad1 = cpu.a[1].wrapping_add(0x4);
    cpu.a[1] = ad1;
    16
}
/// $044444: rts
pub fn i_044444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44446;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionOp07_WaitEnemies_044446  ($044446, 14 B, mission/event_vm_04422a.s)
/// $044446: move.w $106e88.l, d0
pub fn i_044446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4444C;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106E88) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $04444C: cmp.b $1(a1), d0
pub fn i_04444c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44450;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $044450: bhi.w $44458
pub fn i_044450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44454;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x44458; return 10; }
    8
}
// ===== LeaA1Plus4_044454  ($044454, 6 B, thunks/lea_a1_plus4.c)
/// $044454: lea.l $4(a1), a1
pub fn i_044454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44458;
    cpu.ir = 0x43E9;
    let ad1 = cpu.a[1].wrapping_add(0x4);
    cpu.a[1] = ad1;
    16
}
/// $044458: rts
pub fn i_044458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4445A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionOp08_Resume_04445A  ($04445A, 12 B, mission/event_vm_04422a.s)
/// $04445A: move.b #$0, $74(a6)
pub fn i_04445a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44460;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write8(ad2, v1 as u8);
    12
}
/// $044460: clr.b $108179.l
pub fn i_044460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44466;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x108179) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x108179, r2 as u8);
    16
}
// ===== LeaA1Plus4_044466  ($044466, 6 B, thunks/lea_a1_plus4.c)
/// $044466: lea.l $4(a1), a1
pub fn i_044466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4446A;
    cpu.ir = 0x43E9;
    let ad1 = cpu.a[1].wrapping_add(0x4);
    cpu.a[1] = ad1;
    16
}
/// $04446A: rts
pub fn i_04446a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4446C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionOp04_PlayersPastY_04446C  ($04446C, 124 B, mission/event_vm_04422a.s)
/// $04446C: clr.w d0
pub fn i_04446c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4446E;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $04446E: clr.w d1
pub fn i_04446e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44470;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $044470: clr.w d2
pub fn i_044470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44472;
    cpu.ir = 0x4242;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $044472: clr.w d3
pub fn i_044472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44474;
    cpu.ir = 0x4243;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $044474: cmpi.b #$1, $10fdb6.l
pub fn i_044474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4447C;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10FDB6) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $04447C: bne.w $4448e
pub fn i_04447c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44480;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4448E; return 10; }
    8
}
/// $044480: lea.l $100440.l, a0
pub fn i_044480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44486;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100440;
    20
}
/// $044486: move.w $22(a0), d0
pub fn i_044486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4448A;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04448A: move.w $24(a0), d1
pub fn i_04448a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4448E;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04448E: cmpi.b #$1, $10fdb7.l
pub fn i_04448e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44496;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10FDB7) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $044496: bne.w $444a8
pub fn i_044496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4449A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x444A8; return 10; }
    8
}
/// $04449A: lea.l $1004e0.l, a0
pub fn i_04449a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x444A0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1004E0;
    20
}
/// $0444A0: move.w $22(a0), d2
pub fn i_0444a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x444A4;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0444A4: move.w $24(a0), d3
pub fn i_0444a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x444A8;
    cpu.ir = 0x3628;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0444A8: cmp.w d2, d0
pub fn i_0444a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x444AA;
    cpu.ir = 0xB042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $0444AA: bge.w $444b0
pub fn i_0444aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x444AE;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x444B0; return 10; }
    8
}
/// $0444AE: exg.l d1, d3
pub fn i_0444ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x444B0;
    cpu.ir = 0xC343;
    cpu.d.swap(1, 3);
    6
}
/// $0444B0: subi.w #$200, d1
pub fn i_0444b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x444B4;
    cpu.ir = 0x441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x200, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0444B4: neg.w d1
pub fn i_0444b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x444B6;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0444B6: cmpi.b #$0, $1(a1)
pub fn i_0444b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x444BC;
    cpu.ir = 0xC29;
    let ad2 = cpu.a[1].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $0444BC: bne.w $444c6
pub fn i_0444bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x444C0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x444C6; return 10; }
    8
}
/// $0444C0: add.w $106f54.l, d1
pub fn i_0444c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x444C6;
    cpu.ir = 0xD279;
    let v2 = bus.read16(0x106F54) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $0444C6: cmp.w $4(a1), d1
pub fn i_0444c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x444CA;
    cpu.ir = 0xB269;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $0444CA: lea.l $6(a1), a1
pub fn i_0444ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x444CE;
    cpu.ir = 0x43E9;
    let ad1 = cpu.a[1].wrapping_add(0x6);
    cpu.a[1] = ad1;
    16
}
/// $0444CE: bpl.w $444de
pub fn i_0444ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x444D2;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x444DE; return 10; }
    8
}
/// $0444D2: bsr.w $443c0
pub fn i_0444d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x444D6;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x444D6);
    cpu.pc = 0x443C0;
    18
}
/// $0444D6: bsr.w $446b6
pub fn i_0444d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x444DA;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x444DA);
    cpu.pc = 0x446B6;
    18
}
/// $0444DA: bra.w $444e6
pub fn i_0444da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x444DE;
    cpu.ir = 0x6000;
    cpu.pc = 0x444E6;
    10
}
/// $0444DE: bsr.w $446b6
pub fn i_0444de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x444E2;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x444E2);
    cpu.pc = 0x446B6;
    18
}
/// $0444E2: bsr.w $443c0
pub fn i_0444e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x444E6;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x444E6);
    cpu.pc = 0x443C0;
    18
}
/// $0444E6: rts
pub fn i_0444e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x444E8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionOp0A_PlayersPastX_0444E8  ($0444E8, 112 B, mission/event_vm_04422a.s)
/// $0444E8: move.w #$ffff, d0
pub fn i_0444e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x444EC;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0444EC: move.w #$ffff, d1
pub fn i_0444ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x444F0;
    cpu.ir = 0x323C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0444F0: cmpi.b #$1, $10fdb6.l
pub fn i_0444f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x444F8;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10FDB6) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $0444F8: bne.w $44506
pub fn i_0444f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x444FC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x44506; return 10; }
    8
}
/// $0444FC: lea.l $100440.l, a0
pub fn i_0444fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44502;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100440;
    20
}
/// $044502: move.w $22(a0), d0
pub fn i_044502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44506;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $044506: cmpi.b #$1, $10fdb7.l
pub fn i_044506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4450E;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10FDB7) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $04450E: bne.w $4451c
pub fn i_04450e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44512;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4451C; return 10; }
    8
}
/// $044512: lea.l $1004e0.l, a0
pub fn i_044512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44518;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1004E0;
    20
}
/// $044518: move.w $22(a0), d1
pub fn i_044518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4451C;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04451C: cmp.w d1, d0
pub fn i_04451c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4451E;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $04451E: bls.w $44524
pub fn i_04451e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44522;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x44524; return 10; }
    8
}
/// $044522: exg.l d1, d0
pub fn i_044522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44524;
    cpu.ir = 0xC340;
    cpu.d.swap(1, 0);
    6
}
/// $044524: cmpi.b #$0, $1(a1)
pub fn i_044524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4452A;
    cpu.ir = 0xC29;
    let ad2 = cpu.a[1].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $04452A: bne.w $44534
pub fn i_04452a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4452E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x44534; return 10; }
    8
}
/// $04452E: add.w $106f50.l, d0
pub fn i_04452e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44534;
    cpu.ir = 0xD079;
    let v2 = bus.read16(0x106F50) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $044534: cmp.w $4(a1), d0
pub fn i_044534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44538;
    cpu.ir = 0xB069;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $044538: lea.l $6(a1), a1
pub fn i_044538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4453C;
    cpu.ir = 0x43E9;
    let ad1 = cpu.a[1].wrapping_add(0x6);
    cpu.a[1] = ad1;
    16
}
/// $04453C: bmi.w $4454e
pub fn i_04453c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44540;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x4454E; return 10; }
    8
}
/// $044540: bsr.w $446b6
pub fn i_044540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44544;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x44544);
    cpu.pc = 0x446B6;
    18
}
/// $044544: bsr.w $443c0
pub fn i_044544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44548;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x44548);
    cpu.pc = 0x443C0;
    18
}
/// $044548: rts
pub fn i_044548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4454A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $04454A: bra.w $44558
pub fn i_04454a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4454E;
    cpu.ir = 0x6000;
    cpu.pc = 0x44558;
    10
}
/// $04454E: bsr.w $443c0
pub fn i_04454e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44552;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x44552);
    cpu.pc = 0x443C0;
    18
}
/// $044552: bsr.w $446b6
pub fn i_044552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44556;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x44556);
    cpu.pc = 0x446B6;
    18
}
/// $044556: rts
pub fn i_044556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44558;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Stub_00044558  ($044558, 2 B, thunks/trivial_rts.c)
/// $044558: rts
pub fn i_044558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4455A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionOp0B_BothPastY_04455A  ($04455A, 176 B, mission/event_vm_04422a.s)
/// $04455A: clr.w d0
pub fn i_04455a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4455C;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $04455C: clr.w d1
pub fn i_04455c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4455E;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $04455E: cmpi.b #$1, $10fdb6.l
pub fn i_04455e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44566;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10FDB6) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $044566: bne.w $44574
pub fn i_044566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4456A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x44574; return 10; }
    8
}
/// $04456A: lea.l $100440.l, a0
pub fn i_04456a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44570;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100440;
    20
}
/// $044570: move.w $24(a0), d0
pub fn i_044570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44574;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $044574: cmpi.b #$1, $10fdb7.l
pub fn i_044574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4457C;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10FDB7) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $04457C: bne.w $4458a
pub fn i_04457c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44580;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4458A; return 10; }
    8
}
/// $044580: lea.l $1004e0.l, a0
pub fn i_044580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44586;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1004E0;
    20
}
/// $044586: move.w $24(a0), d1
pub fn i_044586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4458A;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04458A: tst.w d0
pub fn i_04458a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4458C;
    cpu.ir = 0x4A40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $04458C: bne.w $44592
pub fn i_04458c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44590;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x44592; return 10; }
    8
}
/// $044590: move.w d1, d0
pub fn i_044590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44592;
    cpu.ir = 0x3001;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $044592: tst.w d1
pub fn i_044592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44594;
    cpu.ir = 0x4A41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $044594: bne.w $4459a
pub fn i_044594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44598;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4459A; return 10; }
    8
}
/// $044598: move.w d0, d1
pub fn i_044598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4459A;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04459A: subi.w #$200, d0
pub fn i_04459a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4459E;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x200, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04459E: neg.w d0
pub fn i_04459e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x445A0;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0445A0: subi.w #$200, d1
pub fn i_0445a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x445A4;
    cpu.ir = 0x441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x200, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0445A4: neg.w d1
pub fn i_0445a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x445A6;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0445A6: cmpi.b #$0, $1(a1)
pub fn i_0445a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x445AC;
    cpu.ir = 0xC29;
    let ad2 = cpu.a[1].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $0445AC: bne.w $445bc
pub fn i_0445ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x445B0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x445BC; return 10; }
    8
}
/// $0445B0: add.w $106f54.l, d0
pub fn i_0445b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x445B6;
    cpu.ir = 0xD079;
    let v2 = bus.read16(0x106F54) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $0445B6: add.w $106f54.l, d1
pub fn i_0445b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x445BC;
    cpu.ir = 0xD279;
    let v2 = bus.read16(0x106F54) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $0445BC: cmp.w $4(a1), d0
pub fn i_0445bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x445C0;
    cpu.ir = 0xB069;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $0445C0: smi.b d0
pub fn i_0445c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x445C2;
    cpu.ir = 0x5BC0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Mi) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $0445C2: cmp.w $4(a1), d1
pub fn i_0445c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x445C6;
    cpu.ir = 0xB269;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $0445C6: smi.b d1
pub fn i_0445c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x445C8;
    cpu.ir = 0x5BC1;
    let c1: u32 = if cpu.sr.evaluate(Condition::Mi) { 0xFF } else { 0 };
    let v2 = cpu.d[1] & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (c1 & 0xFF);
    4
}
/// $0445C8: lea.l $6(a1), a1
pub fn i_0445c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x445CC;
    cpu.ir = 0x43E9;
    let ad1 = cpu.a[1].wrapping_add(0x6);
    cpu.a[1] = ad1;
    16
}
/// $0445CC: move.b d0, d7
pub fn i_0445cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x445CE;
    cpu.ir = 0x1E00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0445CE: and.b d1, d7
pub fn i_0445ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x445D0;
    cpu.ir = 0xCE01;
    let v1 = cpu.d[1] & 0xFF;
    let r = (v1 & cpu.d[7]) & 0xFF;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $0445D0: beq.w $445e4
pub fn i_0445d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x445D4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x445E4; return 10; }
    8
}
/// $0445D4: bsr.w $443c0
pub fn i_0445d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x445D8;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x445D8);
    cpu.pc = 0x443C0;
    18
}
/// $0445D8: bsr.w $446b6
pub fn i_0445d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x445DC;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x445DC);
    cpu.pc = 0x446B6;
    18
}
/// $0445DC: bsr.w $446b6
pub fn i_0445dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x445E0;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x445E0);
    cpu.pc = 0x446B6;
    18
}
/// $0445E0: bra.w $44608
pub fn i_0445e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x445E4;
    cpu.ir = 0x6000;
    cpu.pc = 0x44608;
    10
}
/// $0445E4: move.b d0, d7
pub fn i_0445e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x445E6;
    cpu.ir = 0x1E00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0445E6: or.b d0, d7
pub fn i_0445e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x445E8;
    cpu.ir = 0x8E00;
    let v1 = cpu.d[0] & 0xFF;
    let r = (v1 | cpu.d[7]) & 0xFF;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $0445E8: beq.w $445fc
pub fn i_0445e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x445EC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x445FC; return 10; }
    8
}
/// $0445EC: bsr.w $446b6
pub fn i_0445ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x445F0;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x445F0);
    cpu.pc = 0x446B6;
    18
}
/// $0445F0: bsr.w $443c0
pub fn i_0445f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x445F4;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x445F4);
    cpu.pc = 0x443C0;
    18
}
/// $0445F4: bsr.w $446b6
pub fn i_0445f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x445F8;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x445F8);
    cpu.pc = 0x446B6;
    18
}
/// $0445F8: bra.w $44608
pub fn i_0445f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x445FC;
    cpu.ir = 0x6000;
    cpu.pc = 0x44608;
    10
}
/// $0445FC: bsr.w $446b6
pub fn i_0445fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44600;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x44600);
    cpu.pc = 0x446B6;
    18
}
/// $044600: bsr.w $446b6
pub fn i_044600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44604;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x44604);
    cpu.pc = 0x446B6;
    18
}
/// $044604: bsr.w $443c0
pub fn i_044604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44608;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x44608);
    cpu.pc = 0x443C0;
    18
}
/// $044608: rts
pub fn i_044608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4460A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionOp02_SkipTo0D_04460A  ($04460A, 46 B, mission/event_vm_04422a.s)
/// $04460A: lea.l $4(a1), a1
pub fn i_04460a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4460E;
    cpu.ir = 0x43E9;
    let ad1 = cpu.a[1].wrapping_add(0x4);
    cpu.a[1] = ad1;
    16
}
/// $04460E: move.b (a1), d0
pub fn i_04460e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44610;
    cpu.ir = 0x1011;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $044610: cmpi.b #$c, d0
pub fn i_044610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44614;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xC, Size::Byte, false);
    8
}
/// $044614: bne.w $44624
pub fn i_044614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44618;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x44624; return 10; }
    8
}
/// $044618: nop
pub fn i_044618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4461A;
    cpu.ir = 0x4E71;
    4
}
/// $04461A: nop
pub fn i_04461a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4461C;
    cpu.ir = 0x4E71;
    4
}
/// $04461C: cmpi.b #$c, d0
pub fn i_04461c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44620;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xC, Size::Byte, false);
    8
}
/// $044620: nop
pub fn i_044620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44622;
    cpu.ir = 0x4E71;
    4
}
/// $044622: trap #$f
pub fn i_044622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44624;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $044624: cmpi.b #$d, d0
pub fn i_044624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44628;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xD, Size::Byte, false);
    8
}
/// $044628: beq.w $44632
pub fn i_044628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4462C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x44632; return 10; }
    8
}
/// $04462C: bsr.w $443c0
pub fn i_04462c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44630;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x44630);
    cpu.pc = 0x443C0;
    18
}
/// $044630: bra.b $4460e
pub fn i_044630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44632;
    cpu.ir = 0x60DC;
    cpu.pc = 0x4460E;
    10
}
/// $044632: lea.l $2(a1), a1
pub fn i_044632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44636;
    cpu.ir = 0x43E9;
    let ad1 = cpu.a[1].wrapping_add(0x2);
    cpu.a[1] = ad1;
    16
}
/// $044636: rts
pub fn i_044636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44638;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionOp03_CondSelect_044638  ($044638, 102 B, mission/event_vm_04422a.s)
/// $044638: moveq #$0, d7
pub fn i_044638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4463A;
    cpu.ir = 0x7E00;
    cpu.d[7] = 0x0;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04463A: lea.l $4(a1), a1
pub fn i_04463a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4463E;
    cpu.ir = 0x43E9;
    let ad1 = cpu.a[1].wrapping_add(0x4);
    cpu.a[1] = ad1;
    16
}
/// $04463E: move.b (a1), d0
pub fn i_04463e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44640;
    cpu.ir = 0x1011;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $044640: cmpi.b #$c, d0
pub fn i_044640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44644;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xC, Size::Byte, false);
    8
}
/// $044644: bne.w $44654
pub fn i_044644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44648;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x44654; return 10; }
    8
}
/// $044648: nop
pub fn i_044648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4464A;
    cpu.ir = 0x4E71;
    4
}
/// $04464A: nop
pub fn i_04464a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4464C;
    cpu.ir = 0x4E71;
    4
}
/// $04464C: cmpi.b #$c, d0
pub fn i_04464c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44650;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xC, Size::Byte, false);
    8
}
/// $044650: nop
pub fn i_044650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44652;
    cpu.ir = 0x4E71;
    4
}
/// $044652: trap #$f
pub fn i_044652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44654;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $044654: cmpi.b #$e, d0
pub fn i_044654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44658;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xE, Size::Byte, false);
    8
}
/// $044658: beq.w $44666
pub fn i_044658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4465C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x44666; return 10; }
    8
}
/// $04465C: move.l a1, -(a7)
pub fn i_04465c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4465E;
    cpu.ir = 0x2F09;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $04465E: addq.w #$1, d7
pub fn i_04465e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44660;
    cpu.ir = 0x5247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $044660: bsr.w $446b6
pub fn i_044660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44664;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x44664);
    cpu.pc = 0x446B6;
    18
}
/// $044664: bra.b $4463e
pub fn i_044664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44666;
    cpu.ir = 0x60D8;
    cpu.pc = 0x4463E;
    10
}
/// $044666: lea.l $2(a1), a2
pub fn i_044666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4466A;
    cpu.ir = 0x45E9;
    let ad1 = cpu.a[1].wrapping_add(0x2);
    cpu.a[2] = ad1;
    16
}
/// $04466A: move.w d7, d0
pub fn i_04466a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4466C;
    cpu.ir = 0x3007;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04466C: beq.w $4469a
pub fn i_04466c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44670;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4469A; return 10; }
    8
}
/// $044670: move.l a4, -(a7)
pub fn i_044670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44672;
    cpu.ir = 0x2F0C;
    let v1 = cpu.a[4] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $044672: movem.w d6-d7, -(a7)
pub fn i_044672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44676;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[7] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[6] as u16);
    cpu.a[7] = a;
    20
}
/// $044676: jsr $5e9e4.l
pub fn i_044676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4467C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4467C);
    cpu.pc = 0x5E9E4;
    16
}
/// $04467C: movem.w (a7)+, d6-d7
pub fn i_04467c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44680;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[6] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[7] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $044680: movea.l (a7)+, a4
pub fn i_044680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44682;
    cpu.ir = 0x285F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[4] = v3;
    12
}
/// $044682: add.w d0, d0
pub fn i_044682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44684;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $044684: add.w d0, d0
pub fn i_044684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44686;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $044686: add.w d7, d7
pub fn i_044686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44688;
    cpu.ir = 0xDE47;
    let v1 = cpu.d[7] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $044688: add.w d7, d7
pub fn i_044688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4468A;
    cpu.ir = 0xDE47;
    let v1 = cpu.d[7] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $04468A: movea.l (a7, d0.w), a1
pub fn i_04468a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4468E;
    cpu.ir = 0x2277;
    let ad2 = cpu.a[7].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    18
}
/// $04468E: lea.l (a7, d7.w), a7
pub fn i_04468e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44692;
    cpu.ir = 0x4FF7;
    let ad1 = cpu.a[7].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    cpu.a[7] = ad1;
    18
}
/// $044692: move.l a2, -(a7)
pub fn i_044692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44694;
    cpu.ir = 0x2F0A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $044694: bsr.w $443c0
pub fn i_044694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44698;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x44698);
    cpu.pc = 0x443C0;
    18
}
/// $044698: movea.l (a7)+, a2
pub fn i_044698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4469A;
    cpu.ir = 0x245F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    12
}
/// $04469A: movea.l a2, a1
pub fn i_04469a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4469C;
    cpu.ir = 0x224A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $04469C: rts
pub fn i_04469c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4469E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionOp01_PeriodicSpawn_04469E  ($04469E, 10 B, mission/event_vm_04422a.s)
/// $04469E: bsr.w $447c6
pub fn i_04469e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446A2;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x446A2);
    cpu.pc = 0x447C6;
    18
}
/// $0446A2: bsr.w $446b6
pub fn i_0446a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446A6;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x446A6);
    cpu.pc = 0x446B6;
    18
}
/// $0446A6: rts
pub fn i_0446a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446A8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionOp00_SpawnDirect_0446A8  ($0446A8, 14 B, mission/event_vm_04422a.s)
/// $0446A8: move.l a1, -(a7)
pub fn i_0446a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446AA;
    cpu.ir = 0x2F09;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $0446AA: bsr.w $448a6
pub fn i_0446aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446AE;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x446AE);
    cpu.pc = 0x448A6;
    18
}
/// $0446AE: movea.l (a7)+, a1
pub fn i_0446ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446B0;
    cpu.ir = 0x225F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    12
}
/// $0446B0: lea.l $12(a1), a1
pub fn i_0446b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446B4;
    cpu.ir = 0x43E9;
    let ad1 = cpu.a[1].wrapping_add(0x12);
    cpu.a[1] = ad1;
    16
}
/// $0446B4: rts
pub fn i_0446b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446B6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionVM_SkipOp_0446B6  ($0446B6, 88 B, mission/event_vm_04422a.s)
/// $0446B6: moveq #$0, d0
pub fn i_0446b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446B8;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0446B8: move.b (a1), d0
pub fn i_0446b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446BA;
    cpu.ir = 0x1011;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $0446BA: cmpi.b #$c, d0
pub fn i_0446ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446BE;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xC, Size::Byte, false);
    8
}
/// $0446BE: bls.w $446ce
pub fn i_0446be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446C2;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x446CE; return 10; }
    8
}
/// $0446C2: nop
pub fn i_0446c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446C4;
    cpu.ir = 0x4E71;
    4
}
/// $0446C4: nop
pub fn i_0446c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446C6;
    cpu.ir = 0x4E71;
    4
}
/// $0446C6: cmpi.b #$c, d0
pub fn i_0446c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446CA;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xC, Size::Byte, false);
    8
}
/// $0446CA: nop
pub fn i_0446ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446CC;
    cpu.ir = 0x4E71;
    4
}
/// $0446CC: trap #$f
pub fn i_0446cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446CE;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $0446CE: add.w d0, d0
pub fn i_0446ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446D0;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0446D0: add.w d0, d0
pub fn i_0446d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446D2;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0446D2: lea.l $446da(pc), a0
pub fn i_0446d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446D6;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x446DA;
    16
}
/// $0446D6: jmp (a0, d0.w)
pub fn i_0446d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446DA;
    cpu.ir = 0x4EF0;
    let ad1 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    cpu.jump_to(ad1);
    8
}
/// $0446DA: bra.w $4472c
pub fn i_0446da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446DE;
    cpu.ir = 0x6000;
    cpu.pc = 0x4472C;
    10
}
/// $0446DE: bra.w $44732
pub fn i_0446de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446E2;
    cpu.ir = 0x6000;
    cpu.pc = 0x44732;
    10
}
/// $0446E2: bra.w $4473c
pub fn i_0446e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446E6;
    cpu.ir = 0x6000;
    cpu.pc = 0x4473C;
    10
}
/// $0446E6: bra.w $4476a
pub fn i_0446e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446EA;
    cpu.ir = 0x6000;
    cpu.pc = 0x4476A;
    10
}
/// $0446EA: bra.w $44798
pub fn i_0446ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446EE;
    cpu.ir = 0x6000;
    cpu.pc = 0x44798;
    10
}
/// $0446EE: bra.w $44720
pub fn i_0446ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446F2;
    cpu.ir = 0x6000;
    cpu.pc = 0x44720;
    10
}
/// $0446F2: bra.w $44726
pub fn i_0446f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446F6;
    cpu.ir = 0x6000;
    cpu.pc = 0x44726;
    10
}
/// $0446F6: bra.w $4471a
pub fn i_0446f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446FA;
    cpu.ir = 0x6000;
    cpu.pc = 0x4471A;
    10
}
/// $0446FA: bra.w $44714
pub fn i_0446fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446FE;
    cpu.ir = 0x6000;
    cpu.pc = 0x44714;
    10
}
/// $0446FE: bra.w $4470e
pub fn i_0446fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44702;
    cpu.ir = 0x6000;
    cpu.pc = 0x4470E;
    10
}
/// $044702: bra.w $447a6
pub fn i_044702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44706;
    cpu.ir = 0x6000;
    cpu.pc = 0x447A6;
    10
}
/// $044706: bra.w $447b4
pub fn i_044706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4470A;
    cpu.ir = 0x6000;
    cpu.pc = 0x447B4;
    10
}
/// $04470A: bra.w $447c4
pub fn i_04470a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4470E;
    cpu.ir = 0x6000;
    cpu.pc = 0x447C4;
    10
}
// ===== LeaA1Plus4_04470e  ($04470E, 6 B, thunks/lea_a1_plus4.c)
/// $04470E: lea.l $4(a1), a1
pub fn i_04470e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44712;
    cpu.ir = 0x43E9;
    let ad1 = cpu.a[1].wrapping_add(0x4);
    cpu.a[1] = ad1;
    16
}
/// $044712: rts
pub fn i_044712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44714;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== LeaA1Plus4_044714  ($044714, 6 B, thunks/lea_a1_plus4.c)
/// $044714: lea.l $4(a1), a1
pub fn i_044714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44718;
    cpu.ir = 0x43E9;
    let ad1 = cpu.a[1].wrapping_add(0x4);
    cpu.a[1] = ad1;
    16
}
/// $044718: rts
pub fn i_044718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4471A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== LeaA1Plus4_04471a  ($04471A, 6 B, thunks/lea_a1_plus4.c)
/// $04471A: lea.l $4(a1), a1
pub fn i_04471a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4471E;
    cpu.ir = 0x43E9;
    let ad1 = cpu.a[1].wrapping_add(0x4);
    cpu.a[1] = ad1;
    16
}
/// $04471E: rts
pub fn i_04471e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44720;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== LeaA1Plus4_044720  ($044720, 6 B, thunks/lea_a1_plus4.c)
/// $044720: lea.l $4(a1), a1
pub fn i_044720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44724;
    cpu.ir = 0x43E9;
    let ad1 = cpu.a[1].wrapping_add(0x4);
    cpu.a[1] = ad1;
    16
}
/// $044724: rts
pub fn i_044724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44726;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== LeaA1Plus4_044726  ($044726, 6 B, thunks/lea_a1_plus4.c)
/// $044726: lea.l $4(a1), a1
pub fn i_044726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4472A;
    cpu.ir = 0x43E9;
    let ad1 = cpu.a[1].wrapping_add(0x4);
    cpu.a[1] = ad1;
    16
}
/// $04472A: rts
pub fn i_04472a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4472C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionSkip_Op00_04472C  ($04472C, 6 B, mission/event_vm_04422a.s)
/// $04472C: lea.l $12(a1), a1
pub fn i_04472c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44730;
    cpu.ir = 0x43E9;
    let ad1 = cpu.a[1].wrapping_add(0x12);
    cpu.a[1] = ad1;
    16
}
/// $044730: rts
pub fn i_044730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44732;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionSkip_Op01_044732  ($044732, 10 B, mission/event_vm_04422a.s)
/// $044732: lea.l $a(a1), a1
pub fn i_044732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44736;
    cpu.ir = 0x43E9;
    let ad1 = cpu.a[1].wrapping_add(0xA);
    cpu.a[1] = ad1;
    16
}
/// $044736: bsr.w $446b6
pub fn i_044736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4473A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4473A);
    cpu.pc = 0x446B6;
    18
}
/// $04473A: rts
pub fn i_04473a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4473C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionSkip_Op02_04473C  ($04473C, 46 B, mission/event_vm_04422a.s)
/// $04473C: lea.l $4(a1), a1
pub fn i_04473c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44740;
    cpu.ir = 0x43E9;
    let ad1 = cpu.a[1].wrapping_add(0x4);
    cpu.a[1] = ad1;
    16
}
/// $044740: move.b (a1), d0
pub fn i_044740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44742;
    cpu.ir = 0x1011;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $044742: cmpi.b #$c, d0
pub fn i_044742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44746;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xC, Size::Byte, false);
    8
}
/// $044746: bne.w $44756
pub fn i_044746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4474A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x44756; return 10; }
    8
}
/// $04474A: nop
pub fn i_04474a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4474C;
    cpu.ir = 0x4E71;
    4
}
/// $04474C: nop
pub fn i_04474c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4474E;
    cpu.ir = 0x4E71;
    4
}
/// $04474E: cmpi.b #$c, d0
pub fn i_04474e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44752;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xC, Size::Byte, false);
    8
}
/// $044752: nop
pub fn i_044752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44754;
    cpu.ir = 0x4E71;
    4
}
/// $044754: trap #$f
pub fn i_044754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44756;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $044756: cmpi.b #$d, d0
pub fn i_044756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4475A;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xD, Size::Byte, false);
    8
}
/// $04475A: beq.w $44764
pub fn i_04475a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4475E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x44764; return 10; }
    8
}
/// $04475E: bsr.w $446b6
pub fn i_04475e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44762;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x44762);
    cpu.pc = 0x446B6;
    18
}
/// $044762: bra.b $44740
pub fn i_044762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44764;
    cpu.ir = 0x60DC;
    cpu.pc = 0x44740;
    10
}
/// $044764: lea.l $2(a1), a1
pub fn i_044764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44768;
    cpu.ir = 0x43E9;
    let ad1 = cpu.a[1].wrapping_add(0x2);
    cpu.a[1] = ad1;
    16
}
/// $044768: rts
pub fn i_044768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4476A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionSkip_Op03_04476A  ($04476A, 46 B, mission/event_vm_04422a.s)
/// $04476A: lea.l $4(a1), a1
pub fn i_04476a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4476E;
    cpu.ir = 0x43E9;
    let ad1 = cpu.a[1].wrapping_add(0x4);
    cpu.a[1] = ad1;
    16
}
/// $04476E: move.b (a1), d0
pub fn i_04476e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44770;
    cpu.ir = 0x1011;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $044770: cmpi.b #$c, d0
pub fn i_044770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44774;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xC, Size::Byte, false);
    8
}
/// $044774: bne.w $44784
pub fn i_044774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44778;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x44784; return 10; }
    8
}
/// $044778: nop
pub fn i_044778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4477A;
    cpu.ir = 0x4E71;
    4
}
/// $04477A: nop
pub fn i_04477a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4477C;
    cpu.ir = 0x4E71;
    4
}
/// $04477C: cmpi.b #$c, d0
pub fn i_04477c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44780;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xC, Size::Byte, false);
    8
}
/// $044780: nop
pub fn i_044780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44782;
    cpu.ir = 0x4E71;
    4
}
/// $044782: trap #$f
pub fn i_044782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44784;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $044784: cmpi.b #$e, d0
pub fn i_044784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44788;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xE, Size::Byte, false);
    8
}
/// $044788: beq.w $44792
pub fn i_044788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4478C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x44792; return 10; }
    8
}
/// $04478C: bsr.w $446b6
pub fn i_04478c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44790;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x44790);
    cpu.pc = 0x446B6;
    18
}
/// $044790: bra.b $4476e
pub fn i_044790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44792;
    cpu.ir = 0x60DC;
    cpu.pc = 0x4476E;
    10
}
/// $044792: lea.l $2(a1), a1
pub fn i_044792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44796;
    cpu.ir = 0x43E9;
    let ad1 = cpu.a[1].wrapping_add(0x2);
    cpu.a[1] = ad1;
    16
}
/// $044796: rts
pub fn i_044796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44798;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionSkip_Op04_044798  ($044798, 14 B, mission/event_vm_04422a.s)
/// $044798: lea.l $6(a1), a1
pub fn i_044798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4479C;
    cpu.ir = 0x43E9;
    let ad1 = cpu.a[1].wrapping_add(0x6);
    cpu.a[1] = ad1;
    16
}
/// $04479C: bsr.w $446b6
pub fn i_04479c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x447A0;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x447A0);
    cpu.pc = 0x446B6;
    18
}
/// $0447A0: bsr.w $446b6
pub fn i_0447a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x447A4;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x447A4);
    cpu.pc = 0x446B6;
    18
}
/// $0447A4: rts
pub fn i_0447a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x447A6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionSkip_Op0A_0447A6  ($0447A6, 14 B, mission/event_vm_04422a.s)
/// $0447A6: lea.l $6(a1), a1
pub fn i_0447a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x447AA;
    cpu.ir = 0x43E9;
    let ad1 = cpu.a[1].wrapping_add(0x6);
    cpu.a[1] = ad1;
    16
}
/// $0447AA: bsr.w $446b6
pub fn i_0447aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x447AE;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x447AE);
    cpu.pc = 0x446B6;
    18
}
/// $0447AE: bsr.w $446b6
pub fn i_0447ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x447B2;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x447B2);
    cpu.pc = 0x446B6;
    18
}
/// $0447B2: rts
pub fn i_0447b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x447B4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionSkip_Op0B_0447B4  ($0447B4, 18 B, mission/event_vm_04422a.s)
/// $0447B4: lea.l $6(a1), a1
pub fn i_0447b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x447B8;
    cpu.ir = 0x43E9;
    let ad1 = cpu.a[1].wrapping_add(0x6);
    cpu.a[1] = ad1;
    16
}
/// $0447B8: bsr.w $446b6
pub fn i_0447b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x447BC;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x447BC);
    cpu.pc = 0x446B6;
    18
}
/// $0447BC: bsr.w $446b6
pub fn i_0447bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x447C0;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x447C0);
    cpu.pc = 0x446B6;
    18
}
/// $0447C0: bsr.w $446b6
pub fn i_0447c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x447C4;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x447C4);
    cpu.pc = 0x446B6;
    18
}
/// $0447C4: rts
pub fn i_0447c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x447C6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PeriodicSpawner_Setup_0447C6  ($0447C6, 100 B, mission/event_vm_04422a.s)
/// $0447C6: movem.l a1, -(a7)
pub fn i_0447c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x447CA;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $0447CA: lea.l $4482a(pc), a1
pub fn i_0447ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x447CE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4482A;
    16
}
/// $0447CE: jsr $4ae.l
pub fn i_0447ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x447D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x447D4);
    cpu.pc = 0x4AE;
    16
}
/// $0447D4: movem.l (a7)+, a1
pub fn i_0447d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x447D8;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $0447D8: move.b $5(a1), d0
pub fn i_0447d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x447DC;
    cpu.ir = 0x1029;
    let ad2 = cpu.a[1].wrapping_add(0x5);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0447DC: andi.w #$ff, d0
pub fn i_0447dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x447E0;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0447E0: tst.b $8(a1)
pub fn i_0447e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x447E4;
    cpu.ir = 0x4A29;
    let ad2 = cpu.a[1].wrapping_add(0x8);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0447E4: beq.w $447ea
pub fn i_0447e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x447E8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x447EA; return 10; }
    8
}
/// $0447E8: lsl.w #$5, d0
pub fn i_0447e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x447EA;
    cpu.ir = 0xEB48;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $0447EA: move.w d0, $80(a0)
pub fn i_0447ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x447EE;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0447EE: move.b $1(a1), $7c(a0)
pub fn i_0447ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x447F4;
    cpu.ir = 0x1169;
    let ad2 = cpu.a[1].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x7C);
    bus.write8(ad4, v3 as u8);
    20
}
/// $0447F4: moveq #$0, d0
pub fn i_0447f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x447F6;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0447F6: move.b $6(a1), d0
pub fn i_0447f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x447FA;
    cpu.ir = 0x1029;
    let ad2 = cpu.a[1].wrapping_add(0x6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0447FA: tst.b $9(a1)
pub fn i_0447fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x447FE;
    cpu.ir = 0x4A29;
    let ad2 = cpu.a[1].wrapping_add(0x9);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0447FE: beq.w $44804
pub fn i_0447fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44802;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x44804; return 10; }
    8
}
/// $044802: lsl.w #$5, d0
pub fn i_044802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44804;
    cpu.ir = 0xEB48;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $044804: move.w d0, $7e(a0)
pub fn i_044804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44808;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $044808: move.b $7(a1), $84(a0)
pub fn i_044808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4480E;
    cpu.ir = 0x1169;
    let ad2 = cpu.a[1].wrapping_add(0x7);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x84);
    bus.write8(ad4, v3 as u8);
    20
}
/// $04480E: move.b $4(a1), d0
pub fn i_04480e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44812;
    cpu.ir = 0x1029;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $044812: andi.w #$ff, d0
pub fn i_044812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44816;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $044816: lsl.w #$4, d0
pub fn i_044816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44818;
    cpu.ir = 0xE948;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $044818: add.w $7a(a6), d0
pub fn i_044818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4481C;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $04481C: move.w d0, $82(a0)
pub fn i_04481c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44820;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $044820: lea.l $a(a1), a1
pub fn i_044820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44824;
    cpu.ir = 0x43E9;
    let ad1 = cpu.a[1].wrapping_add(0xA);
    cpu.a[1] = ad1;
    16
}
/// $044824: move.l a1, $70(a0)
pub fn i_044824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44828;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $044828: rts
pub fn i_044828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4482A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PeriodicSpawner_Task_04482A  ($04482A, 24 B, mission/event_vm_04422a.s)
/// $04482A: tst.b $106ed3.l
pub fn i_04482a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44830;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x106ED3) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $044830: beq.w $44842
pub fn i_044830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44834;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x44842; return 10; }
    8
}
/// $044834: move.w $106f5c.l, d0
pub fn i_044834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4483A;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106F5C) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $04483A: cmp.w $82(a6), d0
pub fn i_04483a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4483E;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $04483E: bcs.w $44850
pub fn i_04483e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44842;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x44850; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_044842  ($044842, 14 B, kernel/jsr5b6_jmp518.c)
/// $044842: jsr $5b6.l
pub fn i_044842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44848;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44848);
    cpu.pc = 0x5B6;
    16
}
/// $044848: jmp $518.l
pub fn i_044848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4484E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04484E: rts
pub fn i_04484e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44850;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PeriodicSpawner_Tick_044850  ($044850, 10 B, mission/event_vm_04422a.s)
/// $044850: move.w $80(a6), d0
pub fn i_044850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44854;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $044854: beq.w $44860
pub fn i_044854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44858;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x44860; return 10; }
    8
}
/// $044858: subq.w #$1, d0
pub fn i_044858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4485A;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
// ===== SetTaskW_04485a  ($04485A, 6 B, kernel/task_setters_w.c)
/// $04485A: move.w d0, $80(a6)
pub fn i_04485a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4485E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04485E: rts
pub fn i_04485e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44860;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PeriodicSpawner_Fire_044860  ($044860, 54 B, mission/event_vm_04422a.s)
/// $044860: clr.w d0
pub fn i_044860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44862;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $044862: move.b $84(a6), d0
pub fn i_044862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44866;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $044866: cmpi.b #$ff, d0
pub fn i_044866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4486A;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $04486A: beq.b $44876
pub fn i_04486a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4486C;
    cpu.ir = 0x670A;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x44876; return 10; }
    8
}
/// $04486C: cmp.w $106e88.l, d0
pub fn i_04486c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44872;
    cpu.ir = 0xB079;
    let v2 = bus.read16(0x106E88) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    16
}
/// $044872: bcs.w $448a4
pub fn i_044872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44876;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x448A4; return 10; }
    8
}
/// $044876: movea.l $70(a6), a1
pub fn i_044876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4487A;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $04487A: bsr.w $443c0
pub fn i_04487a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4487E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4487E);
    cpu.pc = 0x443C0;
    18
}
/// $04487E: move.w $7e(a6), $80(a6)
pub fn i_04487e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44884;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad4, v3 as u16);
    20
}
/// $044884: move.b $7c(a6), d0
pub fn i_044884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44888;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $044888: beq.w $448a4
pub fn i_044888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4488C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x448A4; return 10; }
    8
}
/// $04488C: subq.b #$1, d0
pub fn i_04488c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4488E;
    cpu.ir = 0x5300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $04488E: move.b d0, $7c(a6)
pub fn i_04488e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44892;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $044892: bne.w $448a4
pub fn i_044892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44896;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x448A4; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_044896  ($044896, 14 B, kernel/jsr5b6_jmp518.c)
/// $044896: jsr $5b6.l
pub fn i_044896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4489C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4489C);
    cpu.pc = 0x5B6;
    16
}
/// $04489C: jmp $518.l
pub fn i_04489c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x448A2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0448A2: rts
pub fn i_0448a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x448A4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Stub_000448A4  ($0448A4, 2 B, thunks/trivial_rts.c)
/// $0448A4: rts
pub fn i_0448a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x448A6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Spawn_FromStream_0448A6  ($0448A6, 232 B, mission/spawn_boss_0448a6.s)
/// $0448A6: cmpi.b #$0, (a1)
pub fn i_0448a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x448AA;
    cpu.ir = 0xC11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    12
}
/// $0448AA: beq.w $448ba
pub fn i_0448aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x448AE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x448BA; return 10; }
    8
}
/// $0448AE: nop
pub fn i_0448ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x448B0;
    cpu.ir = 0x4E71;
    4
}
/// $0448B0: nop
pub fn i_0448b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x448B2;
    cpu.ir = 0x4E71;
    4
}
/// $0448B2: cmpi.b #$0, (a1)
pub fn i_0448b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x448B6;
    cpu.ir = 0xC11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    12
}
/// $0448B6: nop
pub fn i_0448b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x448B8;
    cpu.ir = 0x4E71;
    4
}
/// $0448B8: trap #$f
pub fn i_0448b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x448BA;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $0448BA: move.b $1(a1), d7
pub fn i_0448ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x448BE;
    cpu.ir = 0x1E29;
    let ad2 = cpu.a[1].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0448BE: andi.b #$3, d7
pub fn i_0448be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x448C2;
    cpu.ir = 0x207;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0448C2: beq.w $448e6
pub fn i_0448c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x448C6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x448E6; return 10; }
    8
}
/// $0448C6: cmpi.b #$3, d7
pub fn i_0448c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x448CA;
    cpu.ir = 0xC07;
    let v1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $0448CA: bne.w $448dc
pub fn i_0448ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x448CE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x448DC; return 10; }
    8
}
/// $0448CE: jsr $2ac6a.l
pub fn i_0448ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x448D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x448D4);
    cpu.pc = 0x2AC6A;
    16
}
/// $0448D4: bcc.w $4498c
pub fn i_0448d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x448D8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4498C; return 10; }
    8
}
/// $0448D8: bra.w $448e6
pub fn i_0448d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x448DC;
    cpu.ir = 0x6000;
    cpu.pc = 0x448E6;
    10
}
/// $0448DC: cmp.b $106ed1.l, d7
pub fn i_0448dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x448E2;
    cpu.ir = 0xBE39;
    let v2 = bus.read8(0x106ED1) as u32;
    let h1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, h1, v2, Size::Byte, false);
    16
}
/// $0448E2: bne.w $4498c
pub fn i_0448e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x448E6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4498C; return 10; }
    8
}
/// $0448E6: moveq #$0, d0
pub fn i_0448e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x448E8;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0448E8: move.w $4(a1), d0
pub fn i_0448e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x448EC;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0448EC: add.w d0, d0
pub fn i_0448ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x448EE;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0448EE: add.w d0, d0
pub fn i_0448ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x448F0;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0448F0: lea.l $e8000.l, a0
pub fn i_0448f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x448F6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0xE8000;
    20
}
/// $0448F6: movem.l a1/a6, -(a7)
pub fn i_0448f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x448FA;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $0448FA: movea.l (a0, d0.w), a1
pub fn i_0448fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x448FE;
    cpu.ir = 0x2270;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    18
}
/// $0448FE: lea.l $100800.l, a6
pub fn i_0448fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44904;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0x100800;
    20
}
/// $044904: jsr $4ae.l
pub fn i_044904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4490A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4490A);
    cpu.pc = 0x4AE;
    16
}
/// $04490A: movem.l (a7)+, a1/a6
pub fn i_04490a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4490E;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $04490E: move.b $1(a1), d7
pub fn i_04490e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44912;
    cpu.ir = 0x1E29;
    let ad2 = cpu.a[1].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    12
}
/// $044912: btst.b #$4, d7
pub fn i_044912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44916;
    cpu.ir = 0x807;
    let mk: u32 = 1u32 << ((4) % 32);
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $044916: bne.w $44920
pub fn i_044916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4491A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x44920; return 10; }
    8
}
/// $04491A: bset.b #$1, $12(a0)
pub fn i_04491a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44920;
    cpu.ir = 0x8E8;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $044920: move.w $6(a1), d0
pub fn i_044920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44924;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $044924: btst.b #$7, d7
pub fn i_044924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44928;
    cpu.ir = 0x807;
    let mk: u32 = 1u32 << ((7) % 32);
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $044928: bne.w $44932
pub fn i_044928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4492C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x44932; return 10; }
    8
}
/// $04492C: sub.w $106f50.l, d0
pub fn i_04492c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44932;
    cpu.ir = 0x9079;
    let v2 = bus.read16(0x106F50) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $044932: move.w d0, $22(a0)
pub fn i_044932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44936;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $044936: move.w $8(a1), d0
pub fn i_044936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4493A;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04493A: btst.b #$6, d7
pub fn i_04493a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4493E;
    cpu.ir = 0x807;
    let mk: u32 = 1u32 << ((6) % 32);
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $04493E: bne.w $44948
pub fn i_04493e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44942;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x44948; return 10; }
    8
}
/// $044942: sub.w $106f54.l, d0
pub fn i_044942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44948;
    cpu.ir = 0x9079;
    let v2 = bus.read16(0x106F54) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $044948: neg.w d0
pub fn i_044948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4494A;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $04494A: addi.w #$200, d0
pub fn i_04494a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4494E;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x200, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04494E: move.w d0, $24(a0)
pub fn i_04494e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44952;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $044952: cmpa.l #$100800, a6
pub fn i_044952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44958;
    cpu.ir = 0xBDFC;
    let v1: u32 = 0x100800;
    let h1 = cpu.a[6];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $044958: beq.w $44972
pub fn i_044958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4495C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x44972; return 10; }
    8
}
/// $04495C: btst.b #$1, $12(a0)
pub fn i_04495c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44962;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x12);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $044962: beq.w $4496c
pub fn i_044962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44966;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4496C; return 10; }
    8
}
/// $044966: addq.w #$1, $106e8a.l
pub fn i_044966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4496C;
    cpu.ir = 0x5279;
    let v1 = bus.read16(0x106E8A) as u32;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    bus.write16(0x106E8A, r2 as u16);
    16
}
/// $04496C: jsr $4405e.l
pub fn i_04496c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44972;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44972);
    cpu.pc = 0x4405E;
    16
}
/// $044972: andi.b #$c, d7
pub fn i_044972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44976;
    cpu.ir = 0x207;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = (v1 & 0xC) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $044976: lsr.w #$2, d7
pub fn i_044976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44978;
    cpu.ir = 0xE44F;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $044978: move.b d7, $11(a0)
pub fn i_044978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4497C;
    cpu.ir = 0x1147;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x11);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04497C: lea.l $a(a1), a1
pub fn i_04497c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44980;
    cpu.ir = 0x43E9;
    let ad1 = cpu.a[1].wrapping_add(0xA);
    cpu.a[1] = ad1;
    16
}
/// $044980: lea.l $98(a0), a2
pub fn i_044980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44984;
    cpu.ir = 0x45E8;
    let ad1 = cpu.a[0].wrapping_add(0x98);
    cpu.a[2] = ad1;
    16
}
/// $044984: moveq #$7, d7
pub fn i_044984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44986;
    cpu.ir = 0x7E07;
    cpu.d[7] = 0x7;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $044986: move.b (a1)+, (a2)+
pub fn i_044986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44988;
    cpu.ir = 0x14D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2]; cpu.a[2] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $044988: dbra d7, $44986
pub fn i_044988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4498C;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x4498C; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x4498C; return 14; }
    cpu.pc = 0x44986;
    10
}
/// $04498C: rts
pub fn i_04498c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4498E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Spawn_MarkPending_04498E  ($04498E, 24 B, mission/spawn_boss_0448a6.s)
/// $04498E: move.l a6, -(a7)
pub fn i_04498e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44990;
    cpu.ir = 0x2F0E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $044990: lea.l $100800.l, a6
pub fn i_044990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44996;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0x100800;
    20
}
/// $044996: jsr $4ae.l
pub fn i_044996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4499C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4499C);
    cpu.pc = 0x4AE;
    16
}
/// $04499C: bset.b #$1, $12(a0)
pub fn i_04499c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x449A2;
    cpu.ir = 0x8E8;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0449A2: movea.l (a7)+, a6
pub fn i_0449a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x449A4;
    cpu.ir = 0x2C5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    12
}
/// $0449A4: rts
pub fn i_0449a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x449A6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CmpDepthToParent_0449A6  ($0449A6, 16 B, mission/spawn_boss_0448a6.s)
/// $0449A6: movea.l $8(a6), a1
pub fn i_0449a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x449AA;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0449AA: move.b $10(a6), d0
pub fn i_0449aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x449AE;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0449AE: cmp.b $10(a1), d0
pub fn i_0449ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x449B2;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $0449B2: bcs.w $449bc
pub fn i_0449b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x449B6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x449BC; return 10; }
    8
}
// ===== ClearXN_0449b6  ($0449B6, 6 B, thunks/ccr_helpers.c)
/// $0449B6: andi.b #$ee, ccr
pub fn i_0449b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x449BA;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0449BA: rts
pub fn i_0449ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x449BC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_0449bc  ($0449BC, 6 B, thunks/ccr_helpers.c)
/// $0449BC: ori.b #$11, ccr
pub fn i_0449bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x449C0;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0449C0: rts
pub fn i_0449c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x449C2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Boss_Intro_0449C2  ($0449C2, 80 B, mission/spawn_boss_0448a6.s)
/// $0449C2: move.w #$a0, $22(a6)
pub fn i_0449c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x449C8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0449C8: move.w #$148, $24(a6)
pub fn i_0449c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x449CE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x148;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0449CE: move.w #$3, d1
pub fn i_0449ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x449D2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0449D2: jsr $2a1aa.l
pub fn i_0449d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x449D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x449D8);
    cpu.pc = 0x2A1AA;
    16
}
/// $0449D8: lea.l $44a2c(pc), a1
pub fn i_0449d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x449DC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x44A2C;
    16
}
/// $0449DC: jsr $4ae.l
pub fn i_0449dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x449E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x449E2);
    cpu.pc = 0x4AE;
    16
}
/// $0449E2: jsr $5dd02.l
pub fn i_0449e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x449E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x449E8);
    cpu.pc = 0x5DD02;
    16
}
/// $0449E8: lea.l $2891ea.l, a0
pub fn i_0449e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x449EE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2891EA;
    20
}
/// $0449EE: jsr $28cd4.l
pub fn i_0449ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x449F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x449F4);
    cpu.pc = 0x28CD4;
    16
}
/// $0449F4: clr.w $28(a6)
pub fn i_0449f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x449F8;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0449F8: clr.w $2a(a6)
pub fn i_0449f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x449FC;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0449FC: clr.w $2c(a6)
pub fn i_0449fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44A00;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $044A00: clr.w $2e(a6)
pub fn i_044a00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44A04;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $044A04: clr.b $26(a6)
pub fn i_044a04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44A08;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x26);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $044A08: clr.b $27(a6)
pub fn i_044a08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44A0C;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x27);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $044A0C: lea.l $44a12(pc), a1
pub fn i_044a0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44A10;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x44A12;
    16
}
/// $044A10: move.l a1, (a6)
pub fn i_044a10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44A12;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Boss_WaitOne_044A12  ($044A12, 18 B, mission/spawn_boss_0448a6.s)
/// $044A12: move.w #$1, d1
pub fn i_044a12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44A16;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $044A16: cmpi.w #$1, d1
pub fn i_044a16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44A1A;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x1, Size::Word, false);
    8
}
/// $044A1A: beq.w $44a24
pub fn i_044a1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44A1E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x44A24; return 10; }
    8
}
/// $044A1E: jsr $5e912.l
pub fn i_044a1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44A24;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44A24);
    cpu.pc = 0x5E912;
    16
}
// ===== JsrAbsThunk_044a24  ($044A24, 8 B, thunks/jsr_abs_thunks.c)
/// $044A24: jsr $28d70.l
pub fn i_044a24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44A2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44A2A);
    cpu.pc = 0x28D70;
    16
}
/// $044A2A: rts
pub fn i_044a2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44A2C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Boss_EngageA_044A2C  ($044A2C, 36 B, mission/spawn_boss_0448a6.s)
/// $044A2C: move.b #$0, $7a(a6)
pub fn i_044a2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44A32;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $044A32: move.w #$2, d1
pub fn i_044a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44A36;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $044A36: jsr $236e.l
pub fn i_044a36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44A3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44A3C);
    cpu.pc = 0x236E;
    16
}
/// $044A3C: bra.w $44a50
pub fn i_044a3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44A40;
    cpu.ir = 0x6000;
    cpu.pc = 0x44A50;
    10
}
/// $044A40: move.b #$1, $7a(a6)
pub fn i_044a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44A46;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $044A46: move.w #$4e, d1
pub fn i_044a46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44A4A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $044A4A: jsr $236e.l
pub fn i_044a4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44A50;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44A50);
    cpu.pc = 0x236E;
    16
}
// ===== Boss_Rearm_044A50  ($044A50, 46 B, mission/spawn_boss_0448a6.s)
/// $044A50: bsr.w $45412
pub fn i_044a50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44A54;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x44A54);
    cpu.pc = 0x45412;
    18
}
/// $044A54: move.w #$6000, d0
pub fn i_044a54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44A58;
    cpu.ir = 0x303C;
    let v1: u32 = 0x6000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $044A58: move.w d0, $7c(a6)
pub fn i_044a58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44A5C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $044A5C: move.b $7c(a6), d0
pub fn i_044a5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44A60;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $044A60: addq.b #$4, d0
pub fn i_044a60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44A62;
    cpu.ir = 0x5800;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $044A62: lsr.b #$3, d0
pub fn i_044a62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44A64;
    cpu.ir = 0xE608;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Byte, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $044A64: andi.w #$1f, d0
pub fn i_044a64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44A68;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $044A68: move.w d0, $34(a6)
pub fn i_044a68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44A6C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $044A6C: lea.l $29f840.l, a0
pub fn i_044a6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44A72;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29F840;
    20
}
/// $044A72: jsr $28cd4.l
pub fn i_044a72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44A78;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44A78);
    cpu.pc = 0x28CD4;
    16
}
/// $044A78: lea.l $44a7e(pc), a1
pub fn i_044a78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44A7C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x44A7E;
    16
}
/// $044A7C: move.l a1, (a6)
pub fn i_044a7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44A7E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Boss_Active_044A7E  ($044A7E, 120 B, mission/spawn_boss_0448a6.s)
/// $044A7E: bsr.w $4546e
pub fn i_044a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44A82;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x44A82);
    cpu.pc = 0x4546E;
    18
}
/// $044A82: bsr.w $44fa0
pub fn i_044a82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44A86;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x44A86);
    cpu.pc = 0x44FA0;
    18
}
/// $044A86: movea.l $c(a6), a0
pub fn i_044a86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44A8A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $044A8A: bclr.b #$2, $8c(a0)
pub fn i_044a8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44A90;
    cpu.ir = 0x8A8;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $044A90: jsr $45022(pc)
pub fn i_044a90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44A94;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x44A94);
    cpu.pc = 0x45022;
    16
}
/// $044A94: addq.b #$4, d0
pub fn i_044a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44A96;
    cpu.ir = 0x5800;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $044A96: lsr.b #$3, d0
pub fn i_044a96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44A98;
    cpu.ir = 0xE608;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Byte, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $044A98: andi.w #$1f, d0
pub fn i_044a98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44A9C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $044A9C: move.w d0, $34(a6)
pub fn i_044a9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44AA0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $044AA0: jsr $28d70.l
pub fn i_044aa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44AA6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44AA6);
    cpu.pc = 0x28D70;
    16
}
/// $044AA6: movea.l $c(a6), a0
pub fn i_044aa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44AAA;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $044AAA: bclr.b #$3, $8c(a0)
pub fn i_044aaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44AB0;
    cpu.ir = 0x8A8;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $044AB0: jsr $45648(pc)
pub fn i_044ab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44AB4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x44AB4);
    cpu.pc = 0x45648;
    16
}
/// $044AB4: bcc.w $44abe
pub fn i_044ab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44AB8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x44ABE; return 10; }
    8
}
/// $044AB8: lea.l $44afe(pc), a1
pub fn i_044ab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44ABC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x44AFE;
    16
}
/// $044ABC: move.l a1, (a6)
pub fn i_044abc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44ABE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $044ABE: movem.l a6, -(a7)
pub fn i_044abe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44AC2;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $044AC2: movea.l $c(a6), a6
pub fn i_044ac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44AC6;
    cpu.ir = 0x2C6E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    16
}
/// $044AC6: move.b #$8, d1
pub fn i_044ac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44ACA;
    cpu.ir = 0x123C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $044ACA: jsr $8f714.l
pub fn i_044aca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44AD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44AD0);
    cpu.pc = 0x8F714;
    16
}
/// $044AD0: movem.l (a7)+, a6
pub fn i_044ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44AD4;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $044AD4: bcc.w $44ade
pub fn i_044ad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44AD8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x44ADE; return 10; }
    8
}
/// $044AD8: lea.l $44c28(pc), a1
pub fn i_044ad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44ADC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x44C28;
    16
}
/// $044ADC: move.l a1, (a6)
pub fn i_044adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44ADE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $044ADE: jsr $5e452.l
pub fn i_044ade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44AE4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44AE4);
    cpu.pc = 0x5E452;
    16
}
/// $044AE4: bcc.w $44af6
pub fn i_044ae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44AE8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x44AF6; return 10; }
    8
}
/// $044AE8: movea.l $c(a6), a0
pub fn i_044ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44AEC;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $044AEC: btst.b #$0, $13(a0)
pub fn i_044aec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44AF2;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $044AF2: beq.w $44afc
pub fn i_044af2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44AF6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x44AFC; return 10; }
    8
}
// ===== SetTaskHandler_044af6  ($044AF6, 8 B, kernel/task_handlers.c)
/// $044AF6: lea.l $44c1a(pc), a1
pub fn i_044af6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44AFA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x44C1A;
    16
}
/// $044AFA: move.l a1, (a6)
pub fn i_044afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44AFC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $044AFC: rts
pub fn i_044afc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44AFE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Boss_PhaseFire_044AFE  ($044AFE, 284 B, mission/spawn_boss_0448a6.s)
/// $044AFE: jsr $4572c(pc)
pub fn i_044afe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44B02;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x44B02);
    cpu.pc = 0x4572C;
    16
}
/// $044B02: bcc.w $44b12
pub fn i_044b02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44B06;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x44B12; return 10; }
    8
}
/// $044B06: lea.l $29e864.l, a0
pub fn i_044b06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44B0C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29E864;
    20
}
/// $044B0C: jsr $28cd4.l
pub fn i_044b0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44B12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44B12);
    cpu.pc = 0x28CD4;
    16
}
/// $044B12: addq.b #$1, $78(a6)
pub fn i_044b12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44B16;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $044B16: move.w #$0, d0
pub fn i_044b16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44B1A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $044B1A: jsr $5e3a2.l
pub fn i_044b1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44B20;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44B20);
    cpu.pc = 0x5E3A2;
    16
}
/// $044B20: bcc.w $44b3c
pub fn i_044b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44B24;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x44B3C; return 10; }
    8
}
/// $044B24: move.w #$1, d0
pub fn i_044b24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44B28;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $044B28: jsr $5e3a2.l
pub fn i_044b28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44B2E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44B2E);
    cpu.pc = 0x5E3A2;
    16
}
/// $044B2E: bcc.w $44b3c
pub fn i_044b2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44B32;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x44B3C; return 10; }
    8
}
/// $044B32: andi.b #$3, $78(a6)
pub fn i_044b32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44B38;
    cpu.ir = 0x22E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0x3) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $044B38: bra.w $44b42
pub fn i_044b38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44B3C;
    cpu.ir = 0x6000;
    cpu.pc = 0x44B42;
    10
}
/// $044B3C: andi.b #$1, $78(a6)
pub fn i_044b3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44B42;
    cpu.ir = 0x22E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $044B42: tst.b $78(a6)
pub fn i_044b42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44B46;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $044B46: bne.w $44b5c
pub fn i_044b46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44B4A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x44B5C; return 10; }
    8
}
/// $044B4A: lea.l $30c70.l, a1
pub fn i_044b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44B50;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x30C70;
    20
}
/// $044B50: jsr $4ae.l
pub fn i_044b50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44B56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44B56);
    cpu.pc = 0x4AE;
    16
}
/// $044B56: jsr $5dd02.l
pub fn i_044b56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44B5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44B5C);
    cpu.pc = 0x5DD02;
    16
}
/// $044B5C: move.w #$108c, d0
pub fn i_044b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44B60;
    cpu.ir = 0x303C;
    let v1: u32 = 0x108C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $044B60: jsr $2352.l
pub fn i_044b60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44B66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44B66);
    cpu.pc = 0x2352;
    16
}
/// $044B66: lea.l $308c2.l, a1
pub fn i_044b66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44B6C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x308C2;
    20
}
/// $044B6C: jsr $6fe.l
pub fn i_044b6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44B72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44B72);
    cpu.pc = 0x6FE;
    16
}
/// $044B72: jsr $5dd02.l
pub fn i_044b72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44B78;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44B78);
    cpu.pc = 0x5DD02;
    16
}
/// $044B78: jsr $517fe.l
pub fn i_044b78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44B7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44B7E);
    cpu.pc = 0x517FE;
    16
}
/// $044B7E: move.b $35(a6), d0
pub fn i_044b7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44B82;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x35);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $044B82: lsl.b #$3, d0
pub fn i_044b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44B84;
    cpu.ir = 0xE708;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $044B84: move.b d0, $98(a0)
pub fn i_044b84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44B88;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $044B88: move.b $79(a6), $9a(a0)
pub fn i_044b88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44B8E;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x79);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $044B8E: move.b $82(a6), $9b(a0)
pub fn i_044b8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44B94;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9B);
    bus.write8(ad4, v3 as u8);
    20
}
/// $044B94: addi.b #$1, $82(a6)
pub fn i_044b94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44B9A;
    cpu.ir = 0x62E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $044B9A: movea.l $c(a6), a0
pub fn i_044b9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44B9E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $044B9E: bset.b #$2, $8c(a0)
pub fn i_044b9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44BA4;
    cpu.ir = 0x8E8;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $044BA4: lea.l $44bae(pc), a1
pub fn i_044ba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44BA8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x44BAE;
    16
}
/// $044BA8: move.l a1, (a6)
pub fn i_044ba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44BAA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $044BAA: bra.w $44bbe
pub fn i_044baa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44BAE;
    cpu.ir = 0x6000;
    cpu.pc = 0x44BBE;
    10
}
/// $044BAE: movea.l $c(a6), a0
pub fn i_044bae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44BB2;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $044BB2: bclr.b #$2, $8c(a0)
pub fn i_044bb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44BB8;
    cpu.ir = 0x8A8;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $044BB8: lea.l $44bbe(pc), a1
pub fn i_044bb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44BBC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x44BBE;
    16
}
/// $044BBC: move.l a1, (a6)
pub fn i_044bbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44BBE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $044BBE: jsr $4546e(pc)
pub fn i_044bbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44BC2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x44BC2);
    cpu.pc = 0x4546E;
    16
}
/// $044BC2: bsr.w $44fa0
pub fn i_044bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44BC6;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x44BC6);
    cpu.pc = 0x44FA0;
    18
}
/// $044BC6: jsr $45022(pc)
pub fn i_044bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44BCA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x44BCA);
    cpu.pc = 0x45022;
    16
}
/// $044BCA: move.w $34(a6), d1
pub fn i_044bca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44BCE;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $044BCE: addq.b #$4, d0
pub fn i_044bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44BD0;
    cpu.ir = 0x5800;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $044BD0: lsr.b #$3, d0
pub fn i_044bd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44BD2;
    cpu.ir = 0xE608;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Byte, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $044BD2: andi.w #$1f, d0
pub fn i_044bd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44BD6;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $044BD6: move.w d0, $34(a6)
pub fn i_044bd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44BDA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $044BDA: jsr $4572c(pc)
pub fn i_044bda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44BDE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x44BDE);
    cpu.pc = 0x4572C;
    16
}
/// $044BDE: bcs.w $44c10
pub fn i_044bde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44BE2;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x44C10; return 10; }
    8
}
/// $044BE2: cmpi.w #$0, $88(a6)
pub fn i_044be2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44BE8;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $044BE8: bne.w $44c10
pub fn i_044be8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44BEC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x44C10; return 10; }
    8
}
/// $044BEC: lea.l $30c14.l, a1
pub fn i_044bec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44BF2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x30C14;
    20
}
/// $044BF2: jsr $4ae.l
pub fn i_044bf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44BF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44BF8);
    cpu.pc = 0x4AE;
    16
}
/// $044BF8: jsr $5dd02.l
pub fn i_044bf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44BFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44BFE);
    cpu.pc = 0x5DD02;
    16
}
/// $044BFE: move.b $35(a6), $98(a0)
pub fn i_044bfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44C04;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x35);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad4, v3 as u8);
    20
}
/// $044C04: lea.l $29f840.l, a0
pub fn i_044c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44C0A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29F840;
    20
}
/// $044C0A: jsr $28cd4.l
pub fn i_044c0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44C10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44C10);
    cpu.pc = 0x28CD4;
    16
}
/// $044C10: jsr $28d70.l
pub fn i_044c10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44C16;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44C16);
    cpu.pc = 0x28D70;
    16
}
/// $044C16: bra.w $44aa6
pub fn i_044c16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44C1A;
    cpu.ir = 0x6000;
    cpu.pc = 0x44AA6;
    10
}
// ===== Jsr5B6ThenJmpScheduler_044c1a  ($044C1A, 14 B, kernel/jsr5b6_jmp518.c)
/// $044C1A: jsr $5b6.l
pub fn i_044c1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44C20;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44C20);
    cpu.pc = 0x5B6;
    16
}
/// $044C20: jmp $518.l
pub fn i_044c20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44C26;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $044C26: rts
pub fn i_044c26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44C28;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Boss_Descend_044C28  ($044C28, 256 B, mission/spawn_boss_0448a6.s)
/// $044C28: move.w #$106a, d0
pub fn i_044c28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44C2C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x106A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $044C2C: jsr $2352.l
pub fn i_044c2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44C32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44C32);
    cpu.pc = 0x2352;
    16
}
/// $044C32: clr.w $80(a6)
pub fn i_044c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44C36;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x80);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $044C36: move.l a6, -(a7)
pub fn i_044c36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44C38;
    cpu.ir = 0x2F0E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $044C38: lea.l $100800.l, a6
pub fn i_044c38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44C3E;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0x100800;
    20
}
/// $044C3E: lea.l $44d40(pc), a1
pub fn i_044c3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44C42;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x44D40;
    16
}
/// $044C42: jsr $4ae.l
pub fn i_044c42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44C48;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44C48);
    cpu.pc = 0x4AE;
    16
}
/// $044C48: movea.l (a7)+, a6
pub fn i_044c48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44C4A;
    cpu.ir = 0x2C5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    12
}
/// $044C4A: move.w $22(a6), $22(a0)
pub fn i_044c4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44C50;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $044C50: move.w $24(a6), $24(a0)
pub fn i_044c50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44C56;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $044C56: move.b $7a(a6), $98(a0)
pub fn i_044c56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44C5C;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad4, v3 as u8);
    20
}
/// $044C5C: lea.l $77f6a.l, a1
pub fn i_044c5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44C62;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77F6A;
    20
}
/// $044C62: jsr $6fe.l
pub fn i_044c62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44C68;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44C68);
    cpu.pc = 0x6FE;
    16
}
/// $044C68: jsr $5dd02.l
pub fn i_044c68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44C6E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44C6E);
    cpu.pc = 0x5DD02;
    16
}
/// $044C6E: lea.l $44c74(pc), a1
pub fn i_044c6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44C72;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x44C74;
    16
}
/// $044C72: move.l a1, (a6)
pub fn i_044c72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44C74;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $044C74: move.w $80(a6), d0
pub fn i_044c74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44C78;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $044C78: add.w $106f6c.l, d0
pub fn i_044c78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44C7E;
    cpu.ir = 0xD079;
    let v2 = bus.read16(0x106F6C) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $044C7E: move.w d0, $80(a6)
pub fn i_044c7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44C82;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $044C82: cmpi.w #$1e0, d0
pub fn i_044c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44C86;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x1E0, Size::Word, false);
    8
}
/// $044C86: blt.w $44c90
pub fn i_044c86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44C8A;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x44C90; return 10; }
    8
}
/// $044C8A: lea.l $44ca8(pc), a1
pub fn i_044c8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44C8E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x44CA8;
    16
}
/// $044C8E: move.l a1, (a6)
pub fn i_044c8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44C90;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $044C90: movem.l a6, -(a7)
pub fn i_044c90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44C94;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $044C94: movea.l $c(a6), a6
pub fn i_044c94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44C98;
    cpu.ir = 0x2C6E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    16
}
/// $044C98: move.b #$1, d1
pub fn i_044c98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44C9C;
    cpu.ir = 0x123C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $044C9C: jsr $8f6f2.l
pub fn i_044c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44CA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44CA2);
    cpu.pc = 0x8F6F2;
    16
}
/// $044CA2: movem.l (a7)+, a6
pub fn i_044ca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44CA6;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $044CA6: rts
pub fn i_044ca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44CA8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $044CA8: movea.l $c(a6), a0
pub fn i_044ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44CAC;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $044CAC: bset.b #$3, $8c(a0)
pub fn i_044cac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44CB2;
    cpu.ir = 0x8E8;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $044CB2: bclr.b #$4, $8c(a0)
pub fn i_044cb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44CB8;
    cpu.ir = 0x8A8;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $044CB8: lea.l $44cbe(pc), a1
pub fn i_044cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44CBC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x44CBE;
    16
}
/// $044CBC: move.l a1, (a6)
pub fn i_044cbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44CBE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $044CBE: movea.l $c(a6), a0
pub fn i_044cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44CC2;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $044CC2: btst.b #$4, $8c(a0)
pub fn i_044cc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44CC8;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $044CC8: beq.w $44cd8
pub fn i_044cc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44CCC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x44CD8; return 10; }
    8
}
/// $044CCC: bclr.b #$3, $8c(a0)
pub fn i_044ccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44CD2;
    cpu.ir = 0x8A8;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $044CD2: lea.l $44cda(pc), a1
pub fn i_044cd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44CD6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x44CDA;
    16
}
/// $044CD6: move.l a1, (a6)
pub fn i_044cd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44CD8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $044CD8: bra.b $44c90
pub fn i_044cd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44CDA;
    cpu.ir = 0x60B6;
    cpu.pc = 0x44C90;
    10
}
/// $044CDA: move.w #$106a, d0
pub fn i_044cda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44CDE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x106A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $044CDE: jsr $2352.l
pub fn i_044cde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44CE4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44CE4);
    cpu.pc = 0x2352;
    16
}
/// $044CE4: lea.l $29d3a8.l, a0
pub fn i_044ce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44CEA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29D3A8;
    20
}
/// $044CEA: jsr $28cd4.l
pub fn i_044cea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44CF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44CF0);
    cpu.pc = 0x28CD4;
    16
}
/// $044CF0: lea.l $44cf6(pc), a1
pub fn i_044cf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44CF4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x44CF6;
    16
}
/// $044CF4: move.l a1, (a6)
pub fn i_044cf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44CF6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $044CF6: bsr.w $44fa0
pub fn i_044cf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44CFA;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x44CFA);
    cpu.pc = 0x44FA0;
    18
}
/// $044CFA: jsr $28d70.l
pub fn i_044cfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44D00;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44D00);
    cpu.pc = 0x28D70;
    16
}
/// $044D00: bcc.w $44d0a
pub fn i_044d00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44D04;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x44D0A; return 10; }
    8
}
/// $044D04: lea.l $44a50(pc), a1
pub fn i_044d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44D08;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x44A50;
    16
}
/// $044D08: move.l a1, (a6)
pub fn i_044d08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44D0A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $044D0A: jsr $5e452.l
pub fn i_044d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44D10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44D10);
    cpu.pc = 0x5E452;
    16
}
/// $044D10: bcs.w $44d1a
pub fn i_044d10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44D14;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x44D1A; return 10; }
    8
}
/// $044D14: lea.l $44c1a(pc), a1
pub fn i_044d14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44D18;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x44C1A;
    16
}
/// $044D18: move.l a1, (a6)
pub fn i_044d18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44D1A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $044D1A: movea.l $c(a6), a0
pub fn i_044d1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44D1E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $044D1E: cmpi.l #$2dc5c, (a0)
pub fn i_044d1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44D24;
    cpu.ir = 0xC90;
    let v2 = bus.read32(cpu.a[0]);
    sub_with_flags(cpu, v2, 0x2DC5C, Size::Long, false);
    16
}
/// $044D24: bne.w $44d2e
pub fn i_044d24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44D28;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x44D2E; return 10; }
    8
}
// ===== SetTaskHandler_044d28  ($044D28, 8 B, kernel/task_handlers.c)
/// $044D28: lea.l $44c1a(pc), a1
pub fn i_044d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44D2C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x44C1A;
    16
}
/// $044D2C: move.l a1, (a6)
pub fn i_044d2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44D2E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $044D2E: rts
pub fn i_044d2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44D30;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== BossShot_Init_044D40  ($044D40, 100 B, mission/spawn_boss_0448a6.s)
/// $044D40: move.w #$d000, d0
pub fn i_044d40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44D44;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $044D44: jsr $28134.l
pub fn i_044d44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44D4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44D4A);
    cpu.pc = 0x28134;
    16
}
/// $044D4A: andi.w #$ffe3, $38(a6)
pub fn i_044d4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44D50;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $044D50: ori.w #$10, $38(a6)
pub fn i_044d50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44D56;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x10) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $044D56: move.w #$2, d1
pub fn i_044d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44D5A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $044D5A: tst.b $98(a6)
pub fn i_044d5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44D5E;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $044D5E: beq.w $44d66
pub fn i_044d5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44D62;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x44D66; return 10; }
    8
}
/// $044D62: move.w #$4e, d1
pub fn i_044d62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44D66;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $044D66: jsr $236e.l
pub fn i_044d66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44D6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44D6C);
    cpu.pc = 0x236E;
    16
}
/// $044D6C: move.w #$38, d0
pub fn i_044d6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44D70;
    cpu.ir = 0x303C;
    let v1: u32 = 0x38;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $044D70: jsr $5dca4.l
pub fn i_044d70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44D76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44D76);
    cpu.pc = 0x5DCA4;
    16
}
/// $044D76: move.w d0, $28(a6)
pub fn i_044d76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44D7A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $044D7A: move.w #$384, $2a(a6)
pub fn i_044d7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44D80;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x384;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $044D80: move.w #$ffec, $2e(a6)
pub fn i_044d80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44D86;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFEC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $044D86: move.w #$0, $2c(a6)
pub fn i_044d86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44D8C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $044D8C: move.b #$3c, $45(a6)
pub fn i_044d8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44D92;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $044D92: lea.l $29d090.l, a0
pub fn i_044d92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44D98;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29D090;
    20
}
/// $044D98: jsr $28cd4.l
pub fn i_044d98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44D9E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44D9E);
    cpu.pc = 0x28CD4;
    16
}
/// $044D9E: lea.l $44da4(pc), a1
pub fn i_044d9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44DA2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x44DA4;
    16
}
/// $044DA2: move.l a1, (a6)
pub fn i_044da2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44DA4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== BossShot_Fly_044DA4  ($044DA4, 70 B, mission/spawn_boss_0448a6.s)
/// $044DA4: jsr $27d50.l
pub fn i_044da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44DAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44DAA);
    cpu.pc = 0x27D50;
    16
}
/// $044DAA: jsr $28d70.l
pub fn i_044daa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44DB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44DB0);
    cpu.pc = 0x28D70;
    16
}
/// $044DB0: tst.b $45(a6)
pub fn i_044db0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44DB4;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $044DB4: bgt.w $44dda
pub fn i_044db4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44DB8;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x44DDA; return 10; }
    8
}
/// $044DB8: move.w #$1022, d0
pub fn i_044db8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44DBC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1022;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $044DBC: jsr $2352.l
pub fn i_044dbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44DC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44DC2);
    cpu.pc = 0x2352;
    16
}
/// $044DC2: lea.l $77f6a.l, a1
pub fn i_044dc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44DC8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77F6A;
    20
}
/// $044DC8: jsr $6fe.l
pub fn i_044dc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44DCE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44DCE);
    cpu.pc = 0x6FE;
    16
}
/// $044DCE: jsr $5dd02.l
pub fn i_044dce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44DD4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44DD4);
    cpu.pc = 0x5DD02;
    16
}
/// $044DD4: lea.l $44df2(pc), a1
pub fn i_044dd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44DD8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x44DF2;
    16
}
/// $044DD8: move.l a1, (a6)
pub fn i_044dd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44DDA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $044DDA: movea.l #$ffffffff, a0
pub fn i_044dda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44DE0;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $044DE0: jsr $5dd56.l
pub fn i_044de0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44DE6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44DE6);
    cpu.pc = 0x5DD56;
    16
}
/// $044DE6: bcc.w $44df0
pub fn i_044de6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44DEA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x44DF0; return 10; }
    8
}
// ===== SetTaskHandler_044dea  ($044DEA, 8 B, kernel/task_handlers.c)
/// $044DEA: lea.l $44df2(pc), a1
pub fn i_044dea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44DEE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x44DF2;
    16
}
/// $044DEE: move.l a1, (a6)
pub fn i_044dee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44DF0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $044DF0: rts
pub fn i_044df0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44DF2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpAbsThunk_044df2  ($044DF2, 6 B, thunks/jmp_abs_thunks.c)
/// $044DF2: jmp $518.l
pub fn i_044df2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44DF8;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Miniboss_SpawnP1_044DF8  ($044DF8, 26 B, mission/spawn_boss_0448a6.s)
/// $044DF8: move.w #$2, d1
pub fn i_044df8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44DFC;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $044DFC: jsr $236e.l
pub fn i_044dfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44E02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44E02);
    cpu.pc = 0x236E;
    16
}
/// $044E02: move.l #$100440, $70(a6)
pub fn i_044e02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44E0A;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x100440;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $044E0A: move.w #$0, d0
pub fn i_044e0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44E0E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $044E0E: bra.w $44e42
pub fn i_044e0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44E12;
    cpu.ir = 0x6000;
    cpu.pc = 0x44E42;
    10
}
// ===== Miniboss_SpawnP2_044E12  ($044E12, 26 B, mission/spawn_boss_0448a6.s)
/// $044E12: move.w #$4e, d1
pub fn i_044e12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44E16;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $044E16: jsr $236e.l
pub fn i_044e16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44E1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44E1C);
    cpu.pc = 0x236E;
    16
}
/// $044E1C: move.l #$1004e0, $70(a6)
pub fn i_044e1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44E24;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x1004E0;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $044E24: move.w #$1, d0
pub fn i_044e24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44E28;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $044E28: bra.w $44e42
pub fn i_044e28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44E2C;
    cpu.ir = 0x6000;
    cpu.pc = 0x44E42;
    10
}
// ===== Miniboss_SpawnAlt_044E2C  ($044E2C, 22 B, mission/spawn_boss_0448a6.s)
/// $044E2C: move.w #$2, d1
pub fn i_044e2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44E30;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $044E30: jsr $236e.l
pub fn i_044e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44E36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44E36);
    cpu.pc = 0x236E;
    16
}
/// $044E36: move.l #$100580, $70(a6)
pub fn i_044e36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44E3E;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x100580;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $044E3E: move.w #$0, d0
pub fn i_044e3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44E42;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== Miniboss_Attach_044E42  ($044E42, 96 B, mission/spawn_boss_0448a6.s)
/// $044E42: jsr $5e3a2.l
pub fn i_044e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44E48;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44E48);
    cpu.pc = 0x5E3A2;
    16
}
/// $044E48: bcs.w $44e50
pub fn i_044e48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44E4C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x44E50; return 10; }
    8
}
/// $044E4C: bra.w $44f9a
pub fn i_044e4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44E50;
    cpu.ir = 0x6000;
    cpu.pc = 0x44F9A;
    10
}
/// $044E50: btst.b #$0, $6b(a0)
pub fn i_044e50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44E56;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x6B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $044E56: bne.w $44e5e
pub fn i_044e56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44E5A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x44E5E; return 10; }
    8
}
/// $044E5A: bra.w $44f9a
pub fn i_044e5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44E5E;
    cpu.ir = 0x6000;
    cpu.pc = 0x44F9A;
    10
}
/// $044E5E: btst.b #$3, $8c(a0)
pub fn i_044e5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44E64;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $044E64: bne.w $44e6c
pub fn i_044e64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44E68;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x44E6C; return 10; }
    8
}
/// $044E68: bra.w $44f9a
pub fn i_044e68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44E6C;
    cpu.ir = 0x6000;
    cpu.pc = 0x44F9A;
    10
}
/// $044E6C: tst.b $98(a6)
pub fn i_044e6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44E70;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $044E70: bne.w $44e7a
pub fn i_044e70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44E74;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x44E7A; return 10; }
    8
}
/// $044E74: bset.b #$2, $5b(a6)
pub fn i_044e74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44E7A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $044E7A: move.w #$c000, d0
pub fn i_044e7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44E7E;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $044E7E: jsr $28134.l
pub fn i_044e7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44E84;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44E84);
    cpu.pc = 0x28134;
    16
}
/// $044E84: andi.w #$ffe3, $38(a6)
pub fn i_044e84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44E8A;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $044E8A: ori.w #$10, $38(a6)
pub fn i_044e8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44E90;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x10) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $044E90: lea.l $29d090.l, a0
pub fn i_044e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44E96;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29D090;
    20
}
/// $044E96: jsr $28cd4.l
pub fn i_044e96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44E9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44E9C);
    cpu.pc = 0x28CD4;
    16
}
/// $044E9C: lea.l $44ea2(pc), a1
pub fn i_044e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44EA0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x44EA2;
    16
}
/// $044EA0: move.l a1, (a6)
pub fn i_044ea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44EA2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Miniboss_Ride_044EA2  ($044EA2, 68 B, mission/spawn_boss_0448a6.s)
/// $044EA2: jsr $27c8c.l
pub fn i_044ea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44EA8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44EA8);
    cpu.pc = 0x27C8C;
    16
}
/// $044EA8: bcc.w $44ed0
pub fn i_044ea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44EAC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x44ED0; return 10; }
    8
}
/// $044EAC: lea.l $44f04(pc), a1
pub fn i_044eac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44EB0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x44F04;
    16
}
/// $044EB0: move.l a1, (a6)
pub fn i_044eb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44EB2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $044EB2: cmpi.b #$0, $106ece.l
pub fn i_044eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44EBA;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    20
}
/// $044EBA: bne.w $44ed0
pub fn i_044eba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44EBE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x44ED0; return 10; }
    8
}
/// $044EBE: andi.b #$c0, d7
pub fn i_044ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44EC2;
    cpu.ir = 0x207;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = (v1 & 0xC0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $044EC2: cmpi.b #$40, d7
pub fn i_044ec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44EC6;
    cpu.ir = 0xC07;
    let v1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, v1, 0x40, Size::Byte, false);
    8
}
/// $044EC6: bne.w $44ed0
pub fn i_044ec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44ECA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x44ED0; return 10; }
    8
}
/// $044ECA: lea.l $44eee(pc), a1
pub fn i_044eca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44ECE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x44EEE;
    16
}
/// $044ECE: move.l a1, (a6)
pub fn i_044ece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44ED0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $044ED0: jsr $28d70.l
pub fn i_044ed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44ED6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44ED6);
    cpu.pc = 0x28D70;
    16
}
/// $044ED6: movea.l #$ffffffff, a0
pub fn i_044ed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44EDC;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $044EDC: jsr $5dd5c.l
pub fn i_044edc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44EE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44EE2);
    cpu.pc = 0x5DD5C;
    16
}
/// $044EE2: bcc.w $44eec
pub fn i_044ee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44EE6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x44EEC; return 10; }
    8
}
// ===== SetTaskHandler_044ee6  ($044EE6, 8 B, kernel/task_handlers.c)
/// $044EE6: lea.l $44f9a(pc), a1
pub fn i_044ee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44EEA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x44F9A;
    16
}
/// $044EEA: move.l a1, (a6)
pub fn i_044eea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44EEC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $044EEC: rts
pub fn i_044eec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44EEE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Miniboss_Dead_044EEE  ($044EEE, 22 B, mission/spawn_boss_0448a6.s)
/// $044EEE: lea.l $29d234.l, a0
pub fn i_044eee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44EF4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29D234;
    20
}
/// $044EF4: jsr $28cd4.l
pub fn i_044ef4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44EFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44EFA);
    cpu.pc = 0x28CD4;
    16
}
/// $044EFA: lea.l $44f4a(pc), a1
pub fn i_044efa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44EFE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x44F4A;
    16
}
/// $044EFE: move.l a1, (a6)
pub fn i_044efe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44F00;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $044F00: bra.w $44f4a
pub fn i_044f00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44F04;
    cpu.ir = 0x6000;
    cpu.pc = 0x44F4A;
    10
}
// ===== Miniboss_Hop_044F04  ($044F04, 126 B, mission/spawn_boss_0448a6.s)
/// $044F04: lea.l $29d178.l, a0
pub fn i_044f04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44F0A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29D178;
    20
}
/// $044F0A: jsr $28cd4.l
pub fn i_044f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44F10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44F10);
    cpu.pc = 0x28CD4;
    16
}
/// $044F10: move.w #$10, d0
pub fn i_044f10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44F14;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $044F14: jsr $5dca4.l
pub fn i_044f14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44F1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44F1A);
    cpu.pc = 0x5DCA4;
    16
}
/// $044F1A: move.w d0, $28(a6)
pub fn i_044f1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44F1E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $044F1E: move.w #$480, $2a(a6)
pub fn i_044f1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44F24;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x480;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $044F24: move.w #$ff70, $2e(a6)
pub fn i_044f24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44F2A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF70;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $044F2A: move.w #$0, $2c(a6)
pub fn i_044f2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44F30;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $044F30: lea.l $44f36(pc), a1
pub fn i_044f30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44F34;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x44F36;
    16
}
/// $044F34: move.l a1, (a6)
pub fn i_044f34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44F36;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $044F36: jsr $27bc8.l
pub fn i_044f36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44F3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44F3C);
    cpu.pc = 0x27BC8;
    16
}
/// $044F3C: bcc.w $44f46
pub fn i_044f3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44F40;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x44F46; return 10; }
    8
}
/// $044F40: lea.l $44f4a(pc), a1
pub fn i_044f40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44F44;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x44F4A;
    16
}
/// $044F44: move.l a1, (a6)
pub fn i_044f44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44F46;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $044F46: bra.w $44f50
pub fn i_044f46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44F4A;
    cpu.ir = 0x6000;
    cpu.pc = 0x44F50;
    10
}
/// $044F4A: jsr $2783a.l
pub fn i_044f4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44F50;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44F50);
    cpu.pc = 0x2783A;
    16
}
/// $044F50: jsr $28d70.l
pub fn i_044f50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44F56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44F56);
    cpu.pc = 0x28D70;
    16
}
/// $044F56: movea.l #$ffffffff, a0
pub fn i_044f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44F5C;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $044F5C: jsr $5dd5c.l
pub fn i_044f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44F62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44F62);
    cpu.pc = 0x5DD5C;
    16
}
/// $044F62: bcc.w $44f6c
pub fn i_044f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44F66;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x44F6C; return 10; }
    8
}
/// $044F66: lea.l $44f9a(pc), a1
pub fn i_044f66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44F6A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x44F9A;
    16
}
/// $044F6A: move.l a1, (a6)
pub fn i_044f6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44F6C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $044F6C: movea.l $70(a6), a1
pub fn i_044f6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44F70;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $044F70: lea.l $44d30(pc), a2
pub fn i_044f70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44F74;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0x44D30;
    16
}
/// $044F74: lea.l $44d38(pc), a0
pub fn i_044f74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44F78;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x44D38;
    16
}
/// $044F78: jsr $28c20.l
pub fn i_044f78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44F7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44F7E);
    cpu.pc = 0x28C20;
    16
}
/// $044F7E: bcc.w $44f88
pub fn i_044f7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44F82;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x44F88; return 10; }
    8
}
// ===== SetTaskHandler_044f82  ($044F82, 8 B, kernel/task_handlers.c)
/// $044F82: lea.l $44f8a(pc), a1
pub fn i_044f82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44F86;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x44F8A;
    16
}
/// $044F86: move.l a1, (a6)
pub fn i_044f86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44F88;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $044F88: rts
pub fn i_044f88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44F8A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Task_KillFlagParent_044F8A  ($044F8A, 16 B, input/entity_aim_input_044f8a.s)
/// $044F8A: movea.l $70(a6), a0
pub fn i_044f8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44F8E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $044F8E: bset.b #$4, $8c(a0)
pub fn i_044f8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44F94;
    cpu.ir = 0x8E8;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $044F94: jmp $518.l
pub fn i_044f94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44F9A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Task_Kill_044F9A  ($044F9A, 6 B, input/entity_aim_input_044f8a.s)
/// $044F9A: jmp $518.l
pub fn i_044f9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44FA0;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Ent_FaceTarget_044FA0  ($044FA0, 130 B, input/entity_aim_input_044f8a.s)
/// $044FA0: jsr $5e4ca.l
pub fn i_044fa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44FA6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44FA6);
    cpu.pc = 0x5E4CA;
    16
}
/// $044FA6: move.b $59(a0), $59(a6)
pub fn i_044fa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44FAC;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x59);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x59);
    bus.write8(ad4, v3 as u8);
    20
}
/// $044FAC: btst.b #$6, $6b(a0)
pub fn i_044fac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44FB2;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x6B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $044FB2: beq.w $44ffc
pub fn i_044fb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44FB6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x44FFC; return 10; }
    8
}
/// $044FB6: cmpi.b #$0, $106f2a.l
pub fn i_044fb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44FBE;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106F2A) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    20
}
/// $044FBE: bne.w $44fd0
pub fn i_044fbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44FC2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x44FD0; return 10; }
    8
}
/// $044FC2: move.b #$1, d1
pub fn i_044fc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44FC6;
    cpu.ir = 0x123C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $044FC6: not.b d1
pub fn i_044fc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44FC8;
    cpu.ir = 0x4601;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = !v1 & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $044FC8: and.b d1, $3a(a6)
pub fn i_044fc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44FCC;
    cpu.ir = 0xC32E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & cpu.d[1]) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $044FCC: bra.w $44fdc
pub fn i_044fcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44FD0;
    cpu.ir = 0x6000;
    cpu.pc = 0x44FDC;
    10
}
/// $044FD0: move.b $3a(a0), d1
pub fn i_044fd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44FD4;
    cpu.ir = 0x1228;
    let ad2 = cpu.a[0].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $044FD4: eori.b #$1, d1
pub fn i_044fd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44FD8;
    cpu.ir = 0xA01;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $044FD8: move.b d1, $3a(a6)
pub fn i_044fd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44FDC;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $044FDC: move.w $22(a0), d1
pub fn i_044fdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44FE0;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $044FE0: move.w $24(a0), d2
pub fn i_044fe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44FE4;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $044FE4: addi.w #$10, d2
pub fn i_044fe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44FE8;
    cpu.ir = 0x642;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $044FE8: ori.w #$1, d0
pub fn i_044fe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44FEC;
    cpu.ir = 0x40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 | 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $044FEC: move.w d0, $38(a6)
pub fn i_044fec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44FF0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $044FF0: move.w d1, $22(a6)
pub fn i_044ff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44FF4;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $044FF4: move.w d2, $24(a6)
pub fn i_044ff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44FF8;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $044FF8: bra.w $45020
pub fn i_044ff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44FFC;
    cpu.ir = 0x6000;
    cpu.pc = 0x45020;
    10
}
/// $044FFC: movea.l $70(a0), a3
pub fn i_044ffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45000;
    cpu.ir = 0x2668;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[3] = v3;
    16
}
/// $045000: add.w $4(a3), d1
pub fn i_045000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45004;
    cpu.ir = 0xD26B;
    let ad2 = cpu.a[3].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $045004: add.w $6(a3), d2
pub fn i_045004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45008;
    cpu.ir = 0xD46B;
    let ad2 = cpu.a[3].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $045008: movea.l (a3), a3
pub fn i_045008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4500A;
    cpu.ir = 0x2653;
    let v2 = bus.read32(cpu.a[3]);
    cpu.a[3] = v2;
    12
}
/// $04500A: add.w (a3), d1
pub fn i_04500a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4500C;
    cpu.ir = 0xD253;
    let v2 = bus.read16(cpu.a[3]) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $04500C: add.w $2(a3), d2
pub fn i_04500c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45010;
    cpu.ir = 0xD46B;
    let ad2 = cpu.a[3].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $045010: ori.w #$1, d0
pub fn i_045010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45014;
    cpu.ir = 0x40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 | 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $045014: move.w d0, $38(a6)
pub fn i_045014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45018;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $045018: move.w d1, $22(a6)
pub fn i_045018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4501C;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04501C: move.w d2, $24(a6)
pub fn i_04501c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45020;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $045020: rts
pub fn i_045020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45022;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Ent_AimUpdate_045022  ($045022, 1008 B, input/entity_aim_input_044f8a.s)
/// $045022: cmpi.b #$3, $106f2a.l
pub fn i_045022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4502A;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106F2A) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    20
}
/// $04502A: beq.w $45036
pub fn i_04502a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4502E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x45036; return 10; }
    8
}
/// $04502E: tst.b $74(a6)
pub fn i_04502e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45032;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $045032: beq.w $45408
pub fn i_045032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45036;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x45408; return 10; }
    8
}
/// $045036: cmpi.b #$2, $106f2a.l
pub fn i_045036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4503E;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106F2A) as u32;
    sub_with_flags(cpu, v2, 0x2, Size::Byte, false);
    20
}
/// $04503E: beq.w $4504a
pub fn i_04503e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45042;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4504A; return 10; }
    8
}
/// $045042: tst.b $74(a6)
pub fn i_045042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45046;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $045046: beq.w $45408
pub fn i_045046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4504A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x45408; return 10; }
    8
}
/// $04504A: move.b $71(a6), d0
pub fn i_04504a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4504E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04504E: tst.b $106f2a.l
pub fn i_04504e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45054;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x106F2A) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $045054: beq.w $4505a
pub fn i_045054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45058;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4505A; return 10; }
    8
}
/// $045058: clr.b d0
pub fn i_045058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4505A;
    cpu.ir = 0x4200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $04505A: tst.b $106f2a.l
pub fn i_04505a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45060;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x106F2A) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $045060: beq.w $45370
pub fn i_045060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45064;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x45370; return 10; }
    8
}
/// $045064: move.b $106f2a.l, d1
pub fn i_045064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4506A;
    cpu.ir = 0x1239;
    let v2 = bus.read8(0x106F2A) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    16
}
/// $04506A: cmpi.b #$1, d1
pub fn i_04506a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4506E;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $04506E: bne.w $45090
pub fn i_04506e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45072;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x45090; return 10; }
    8
}
/// $045072: btst.b #$5, $7b(a6)
pub fn i_045072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45078;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $045078: beq.w $45086
pub fn i_045078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4507C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x45086; return 10; }
    8
}
/// $04507C: lea.l $5d3a6.l, a0
pub fn i_04507c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45082;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5D3A6;
    20
}
/// $045082: bra.w $4508c
pub fn i_045082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45086;
    cpu.ir = 0x6000;
    cpu.pc = 0x4508C;
    10
}
/// $045086: lea.l $5d4c6.l, a0
pub fn i_045086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4508C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5D4C6;
    20
}
/// $04508C: bra.w $4537a
pub fn i_04508c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45090;
    cpu.ir = 0x6000;
    cpu.pc = 0x4537A;
    10
}
/// $045090: move.b $106f2a.l, d1
pub fn i_045090(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45096;
    cpu.ir = 0x1239;
    let v2 = bus.read8(0x106F2A) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    16
}
/// $045096: cmpi.b #$5, d1
pub fn i_045096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4509A;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x5, Size::Byte, false);
    8
}
/// $04509A: bne.w $45184
pub fn i_04509a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4509E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x45184; return 10; }
    8
}
/// $04509E: cmpi.w #$180, $24(a6)
pub fn i_04509e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x450A4;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x180, Size::Word, false);
    16
}
/// $0450A4: bgt.w $45116
pub fn i_0450a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x450A8;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x45116; return 10; }
    8
}
/// $0450A8: movem.l a6, -(a7)
pub fn i_0450a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x450AC;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $0450AC: movea.l $c(a6), a6
pub fn i_0450ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x450B0;
    cpu.ir = 0x2C6E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    16
}
/// $0450B0: jsr $5cef8.l
pub fn i_0450b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x450B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x450B6);
    cpu.pc = 0x5CEF8;
    16
}
/// $0450B6: movem.l (a7)+, a6
pub fn i_0450b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x450BA;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $0450BA: bcs.w $450ea
pub fn i_0450ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x450BE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x450EA; return 10; }
    8
}
/// $0450BE: btst.b #$4, $7b(a6)
pub fn i_0450be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x450C4;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0450C4: beq.w $450d2
pub fn i_0450c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x450C8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x450D2; return 10; }
    8
}
/// $0450C8: lea.l $5d3c6.l, a0
pub fn i_0450c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x450CE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5D3C6;
    20
}
/// $0450CE: bra.w $45180
pub fn i_0450ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x450D2;
    cpu.ir = 0x6000;
    cpu.pc = 0x45180;
    10
}
/// $0450D2: btst.b #$5, $7b(a6)
pub fn i_0450d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x450D8;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0450D8: beq.w $450e6
pub fn i_0450d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x450DC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x450E6; return 10; }
    8
}
/// $0450DC: lea.l $5d426.l, a0
pub fn i_0450dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x450E2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5D426;
    20
}
/// $0450E2: bra.w $45180
pub fn i_0450e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x450E6;
    cpu.ir = 0x6000;
    cpu.pc = 0x45180;
    10
}
/// $0450E6: bra.w $45112
pub fn i_0450e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x450EA;
    cpu.ir = 0x6000;
    cpu.pc = 0x45112;
    10
}
/// $0450EA: btst.b #$4, $7b(a6)
pub fn i_0450ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x450F0;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0450F0: beq.w $450fe
pub fn i_0450f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x450F4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x450FE; return 10; }
    8
}
/// $0450F4: lea.l $5d4c6.l, a0
pub fn i_0450f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x450FA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5D4C6;
    20
}
/// $0450FA: bra.w $45180
pub fn i_0450fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x450FE;
    cpu.ir = 0x6000;
    cpu.pc = 0x45180;
    10
}
/// $0450FE: btst.b #$5, $7b(a6)
pub fn i_0450fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45104;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $045104: beq.w $45112
pub fn i_045104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45108;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x45112; return 10; }
    8
}
/// $045108: lea.l $5d4e6.l, a0
pub fn i_045108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4510E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5D4E6;
    20
}
/// $04510E: bra.w $45180
pub fn i_04510e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45112;
    cpu.ir = 0x6000;
    cpu.pc = 0x45180;
    10
}
/// $045112: bra.w $45180
pub fn i_045112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45116;
    cpu.ir = 0x6000;
    cpu.pc = 0x45180;
    10
}
/// $045116: movem.l a6, -(a7)
pub fn i_045116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4511A;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $04511A: movea.l $c(a6), a6
pub fn i_04511a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4511E;
    cpu.ir = 0x2C6E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    16
}
/// $04511E: jsr $5cef8.l
pub fn i_04511e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45124;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45124);
    cpu.pc = 0x5CEF8;
    16
}
/// $045124: movem.l (a7)+, a6
pub fn i_045124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45128;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $045128: bcs.w $45158
pub fn i_045128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4512C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x45158; return 10; }
    8
}
/// $04512C: btst.b #$4, $7b(a6)
pub fn i_04512c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45132;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $045132: beq.w $45140
pub fn i_045132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45136;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x45140; return 10; }
    8
}
/// $045136: lea.l $5d446.l, a0
pub fn i_045136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4513C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5D446;
    20
}
/// $04513C: bra.w $45180
pub fn i_04513c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45140;
    cpu.ir = 0x6000;
    cpu.pc = 0x45180;
    10
}
/// $045140: btst.b #$5, $7b(a6)
pub fn i_045140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45146;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $045146: beq.w $45154
pub fn i_045146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4514A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x45154; return 10; }
    8
}
/// $04514A: lea.l $5d486.l, a0
pub fn i_04514a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45150;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5D486;
    20
}
/// $045150: bra.w $45180
pub fn i_045150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45154;
    cpu.ir = 0x6000;
    cpu.pc = 0x45180;
    10
}
/// $045154: bra.w $45180
pub fn i_045154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45158;
    cpu.ir = 0x6000;
    cpu.pc = 0x45180;
    10
}
/// $045158: btst.b #$4, $7b(a6)
pub fn i_045158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4515E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04515E: beq.w $4516c
pub fn i_04515e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45162;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4516C; return 10; }
    8
}
/// $045162: lea.l $5d506.l, a0
pub fn i_045162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45168;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5D506;
    20
}
/// $045168: bra.w $45180
pub fn i_045168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4516C;
    cpu.ir = 0x6000;
    cpu.pc = 0x45180;
    10
}
/// $04516C: btst.b #$5, $7b(a6)
pub fn i_04516c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45172;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $045172: beq.w $45180
pub fn i_045172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45176;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x45180; return 10; }
    8
}
/// $045176: lea.l $5d526.l, a0
pub fn i_045176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4517C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5D526;
    20
}
/// $04517C: bra.w $45180
pub fn i_04517c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45180;
    cpu.ir = 0x6000;
    cpu.pc = 0x45180;
    10
}
/// $045180: bra.w $4537a
pub fn i_045180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45184;
    cpu.ir = 0x6000;
    cpu.pc = 0x4537A;
    10
}
/// $045184: cmpi.b #$4, d1
pub fn i_045184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45188;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x4, Size::Byte, false);
    8
}
/// $045188: bne.w $45196
pub fn i_045188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4518C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x45196; return 10; }
    8
}
/// $04518C: lea.l $5d346.l, a0
pub fn i_04518c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45192;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5D346;
    20
}
/// $045192: bra.w $4537a
pub fn i_045192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45196;
    cpu.ir = 0x6000;
    cpu.pc = 0x4537A;
    10
}
/// $045196: cmpi.b #$3, d1
pub fn i_045196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4519A;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $04519A: bne.w $45292
pub fn i_04519a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4519E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x45292; return 10; }
    8
}
/// $04519E: movea.l $84(a6), a0
pub fn i_04519e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x451A2;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0451A2: move.b $83(a6), d1
pub fn i_0451a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x451A6;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x83);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0451A6: andi.b #$30, d1
pub fn i_0451a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x451AA;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0x30) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0451AA: cmpi.b #$30, d1
pub fn i_0451aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x451AE;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x30, Size::Byte, false);
    8
}
/// $0451AE: bne.w $451c0
pub fn i_0451ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x451B2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x451C0; return 10; }
    8
}
/// $0451B2: lea.l $5d3c6.l, a0
pub fn i_0451b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x451B8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5D3C6;
    20
}
/// $0451B8: move.l a0, $84(a6)
pub fn i_0451b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x451BC;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x043958, i_043958),
    (0x04395C, i_04395c),
    (0x043962, i_043962),
    (0x043966, i_043966),
    (0x04396A, i_04396a),
    (0x043970, i_043970),
    (0x043978, i_043978),
    (0x043980, i_043980),
    (0x043984, i_043984),
    (0x043988, i_043988),
    (0x04398E, i_04398e),
    (0x043996, i_043996),
    (0x04399E, i_04399e),
    (0x0439A6, i_0439a6),
    (0x0439AE, i_0439ae),
    (0x0439B6, i_0439b6),
    (0x0439BA, i_0439ba),
    (0x0439BE, i_0439be),
    (0x0439C4, i_0439c4),
    (0x0439C8, i_0439c8),
    (0x0439CA, i_0439ca),
    (0x0439D0, i_0439d0),
    (0x0439D2, i_0439d2),
    (0x0439D4, i_0439d4),
    (0x0439D8, i_0439d8),
    (0x0439DC, i_0439dc),
    (0x0439E2, i_0439e2),
    (0x0439E6, i_0439e6),
    (0x0439EC, i_0439ec),
    (0x0439F0, i_0439f0),
    (0x0439F4, i_0439f4),
    (0x0439FA, i_0439fa),
    (0x0439FE, i_0439fe),
    (0x043A02, i_043a02),
    (0x043A04, i_043a04),
    (0x043A08, i_043a08),
    (0x043A0C, i_043a0c),
    (0x043A12, i_043a12),
    (0x043A1A, i_043a1a),
    (0x043A1E, i_043a1e),
    (0x043A22, i_043a22),
    (0x043A28, i_043a28),
    (0x043A2C, i_043a2c),
    (0x043A32, i_043a32),
    (0x043A36, i_043a36),
    (0x043A3A, i_043a3a),
    (0x043A3E, i_043a3e),
    (0x043A42, i_043a42),
    (0x043A46, i_043a46),
    (0x043A4C, i_043a4c),
    (0x043A4E, i_043a4e),
    (0x043A54, i_043a54),
    (0x043A58, i_043a58),
    (0x043A5C, i_043a5c),
    (0x043A62, i_043a62),
    (0x043A66, i_043a66),
    (0x043A6A, i_043a6a),
    (0x043A70, i_043a70),
    (0x043A74, i_043a74),
    (0x043A78, i_043a78),
    (0x043A7E, i_043a7e),
    (0x043A82, i_043a82),
    (0x043A88, i_043a88),
    (0x043A8C, i_043a8c),
    (0x043A90, i_043a90),
    (0x043A96, i_043a96),
    (0x043A9A, i_043a9a),
    (0x043AA0, i_043aa0),
    (0x043AA4, i_043aa4),
    (0x043AA8, i_043aa8),
    (0x043AAA, i_043aaa),
    (0x043AAE, i_043aae),
    (0x043AB4, i_043ab4),
    (0x043AB8, i_043ab8),
    (0x043ABC, i_043abc),
    (0x043AC2, i_043ac2),
    (0x043AC6, i_043ac6),
    (0x043ACA, i_043aca),
    (0x043AD0, i_043ad0),
    (0x043AD6, i_043ad6),
    (0x043ADA, i_043ada),
    (0x043ADE, i_043ade),
    (0x043AE4, i_043ae4),
    (0x043AE8, i_043ae8),
    (0x043AEC, i_043aec),
    (0x043AF0, i_043af0),
    (0x043AF6, i_043af6),
    (0x043AFA, i_043afa),
    (0x043AFE, i_043afe),
    (0x043B04, i_043b04),
    (0x043B08, i_043b08),
    (0x043B0C, i_043b0c),
    (0x043B14, i_043b14),
    (0x043B1A, i_043b1a),
    (0x043B1C, i_043b1c),
    (0x043B1E, i_043b1e),
    (0x043B20, i_043b20),
    (0x043B22, i_043b22),
    (0x043B28, i_043b28),
    (0x043B2E, i_043b2e),
    (0x043B34, i_043b34),
    (0x043B3A, i_043b3a),
    (0x043B40, i_043b40),
    (0x043B4A, i_043b4a),
    (0x043B54, i_043b54),
    (0x043B5E, i_043b5e),
    (0x043B62, i_043b62),
    (0x043B66, i_043b66),
    (0x043B6A, i_043b6a),
    (0x043B70, i_043b70),
    (0x043B74, i_043b74),
    (0x043B7A, i_043b7a),
    (0x043B80, i_043b80),
    (0x043B86, i_043b86),
    (0x043B8C, i_043b8c),
    (0x043B90, i_043b90),
    (0x043B94, i_043b94),
    (0x043B98, i_043b98),
    (0x043B9E, i_043b9e),
    (0x043BA2, i_043ba2),
    (0x043BA6, i_043ba6),
    (0x043BAC, i_043bac),
    (0x043BB0, i_043bb0),
    (0x043BB6, i_043bb6),
    (0x043BBC, i_043bbc),
    (0x043BC2, i_043bc2),
    (0x043BC6, i_043bc6),
    (0x043BC8, i_043bc8),
    (0x043BCE, i_043bce),
    (0x043BD4, i_043bd4),
    (0x043BDA, i_043bda),
    (0x043BDE, i_043bde),
    (0x043BE4, i_043be4),
    (0x043BEA, i_043bea),
    (0x043BEC, i_043bec),
    (0x043BF2, i_043bf2),
    (0x043BF8, i_043bf8),
    (0x043BFE, i_043bfe),
    (0x043C04, i_043c04),
    (0x043C08, i_043c08),
    (0x043C0C, i_043c0c),
    (0x043C12, i_043c12),
    (0x043C14, i_043c14),
    (0x043C18, i_043c18),
    (0x043C1E, i_043c1e),
    (0x043C20, i_043c20),
    (0x043C22, i_043c22),
    (0x043C28, i_043c28),
    (0x043C2E, i_043c2e),
    (0x043C30, i_043c30),
    (0x043C32, i_043c32),
    (0x043C34, i_043c34),
    (0x043C36, i_043c36),
    (0x043C3A, i_043c3a),
    (0x043C40, i_043c40),
    (0x043C46, i_043c46),
    (0x043C4C, i_043c4c),
    (0x043C50, i_043c50),
    (0x043C54, i_043c54),
    (0x043C5A, i_043c5a),
    (0x043C60, i_043c60),
    (0x043C64, i_043c64),
    (0x043C68, i_043c68),
    (0x043C6C, i_043c6c),
    (0x043C72, i_043c72),
    (0x043C78, i_043c78),
    (0x043C7C, i_043c7c),
    (0x043C80, i_043c80),
    (0x043C84, i_043c84),
    (0x043C8A, i_043c8a),
    (0x043C90, i_043c90),
    (0x043C94, i_043c94),
    (0x043C9A, i_043c9a),
    (0x043CA0, i_043ca0),
    (0x043CA6, i_043ca6),
    (0x043CAC, i_043cac),
    (0x043CB0, i_043cb0),
    (0x043CB6, i_043cb6),
    (0x043CBA, i_043cba),
    (0x043CC0, i_043cc0),
    (0x043CC6, i_043cc6),
    (0x043CCC, i_043ccc),
    (0x043CD2, i_043cd2),
    (0x043CD8, i_043cd8),
    (0x043CDE, i_043cde),
    (0x043CE2, i_043ce2),
    (0x043CE8, i_043ce8),
    (0x043CEC, i_043cec),
    (0x043CF2, i_043cf2),
    (0x043CF6, i_043cf6),
    (0x043CFC, i_043cfc),
    (0x043D00, i_043d00),
    (0x043D04, i_043d04),
    (0x043D0A, i_043d0a),
    (0x043D10, i_043d10),
    (0x043D14, i_043d14),
    (0x043D18, i_043d18),
    (0x043D1C, i_043d1c),
    (0x043D20, i_043d20),
    (0x043D26, i_043d26),
    (0x043D2C, i_043d2c),
    (0x043D30, i_043d30),
    (0x043D34, i_043d34),
    (0x043D38, i_043d38),
    (0x043D3C, i_043d3c),
    (0x043D42, i_043d42),
    (0x043D48, i_043d48),
    (0x043D4C, i_043d4c),
    (0x043D50, i_043d50),
    (0x043D56, i_043d56),
    (0x043D5C, i_043d5c),
    (0x043D60, i_043d60),
    (0x043D64, i_043d64),
    (0x043D66, i_043d66),
    (0x043D6A, i_043d6a),
    (0x043D6C, i_043d6c),
    (0x043D72, i_043d72),
    (0x043D78, i_043d78),
    (0x043D7E, i_043d7e),
    (0x043D84, i_043d84),
    (0x043D86, i_043d86),
    (0x043D8C, i_043d8c),
    (0x043D90, i_043d90),
    (0x043D96, i_043d96),
    (0x043D9A, i_043d9a),
    (0x043DA0, i_043da0),
    (0x043DA4, i_043da4),
    (0x043DAA, i_043daa),
    (0x043DAE, i_043dae),
    (0x043DB2, i_043db2),
    (0x043DB8, i_043db8),
    (0x043DBE, i_043dbe),
    (0x043DC2, i_043dc2),
    (0x043DC4, i_043dc4),
    (0x043DC8, i_043dc8),
    (0x043DCA, i_043dca),
    (0x043DD0, i_043dd0),
    (0x043DD6, i_043dd6),
    (0x043DDA, i_043dda),
    (0x043DDE, i_043dde),
    (0x043DE4, i_043de4),
    (0x043DE8, i_043de8),
    (0x043DEC, i_043dec),
    (0x043DF2, i_043df2),
    (0x043DF4, i_043df4),
    (0x043DFA, i_043dfa),
    (0x043DFE, i_043dfe),
    (0x043E02, i_043e02),
    (0x043E04, i_043e04),
    (0x043E08, i_043e08),
    (0x043E0C, i_043e0c),
    (0x043E0E, i_043e0e),
    (0x043E14, i_043e14),
    (0x043E16, i_043e16),
    (0x043E1A, i_043e1a),
    (0x043E1C, i_043e1c),
    (0x043E1E, i_043e1e),
    (0x043E22, i_043e22),
    (0x043E24, i_043e24),
    (0x043E2A, i_043e2a),
    (0x043E2C, i_043e2c),
    (0x043E30, i_043e30),
    (0x043E32, i_043e32),
    (0x043E34, i_043e34),
    (0x043E38, i_043e38),
    (0x043E3A, i_043e3a),
    (0x043E3C, i_043e3c),
    (0x043E3E, i_043e3e),
    (0x043E40, i_043e40),
    (0x043E44, i_043e44),
    (0x043E46, i_043e46),
    (0x043E4A, i_043e4a),
    (0x043E4E, i_043e4e),
    (0x043E50, i_043e50),
    (0x043E54, i_043e54),
    (0x043E56, i_043e56),
    (0x043E5A, i_043e5a),
    (0x043E5C, i_043e5c),
    (0x043E60, i_043e60),
    (0x043E62, i_043e62),
    (0x043E68, i_043e68),
    (0x043E6C, i_043e6c),
    (0x043E6E, i_043e6e),
    (0x043E70, i_043e70),
    (0x043E72, i_043e72),
    (0x043E74, i_043e74),
    (0x043E76, i_043e76),
    (0x043E7A, i_043e7a),
    (0x043E7C, i_043e7c),
    (0x043E7E, i_043e7e),
    (0x043E80, i_043e80),
    (0x043E82, i_043e82),
    (0x043E84, i_043e84),
    (0x043E86, i_043e86),
    (0x043E88, i_043e88),
    (0x043E8A, i_043e8a),
    (0x043E8C, i_043e8c),
    (0x043E90, i_043e90),
    (0x043E94, i_043e94),
    (0x043E96, i_043e96),
    (0x043E98, i_043e98),
    (0x043E9A, i_043e9a),
    (0x043E9C, i_043e9c),
    (0x043E9E, i_043e9e),
    (0x043EA0, i_043ea0),
    (0x043EA2, i_043ea2),
    (0x043EA6, i_043ea6),
    (0x043EAA, i_043eaa),
    (0x043EAC, i_043eac),
    (0x043EAE, i_043eae),
    (0x043EB0, i_043eb0),
    (0x043EB2, i_043eb2),
    (0x043EB4, i_043eb4),
    (0x043EB6, i_043eb6),
    (0x043EBA, i_043eba),
    (0x043EBE, i_043ebe),
    (0x043EC2, i_043ec2),
    (0x043EC6, i_043ec6),
    (0x043EC8, i_043ec8),
    (0x043ECC, i_043ecc),
    (0x043ED0, i_043ed0),
    (0x043ED4, i_043ed4),
    (0x043ED8, i_043ed8),
    (0x043EDA, i_043eda),
    (0x043EDE, i_043ede),
    (0x043EE4, i_043ee4),
    (0x043EE6, i_043ee6),
    (0x043EEA, i_043eea),
    (0x043EEC, i_043eec),
    (0x043EF2, i_043ef2),
    (0x043EF6, i_043ef6),
    (0x043EF8, i_043ef8),
    (0x043EFA, i_043efa),
    (0x043EFE, i_043efe),
    (0x043F00, i_043f00),
    (0x043F02, i_043f02),
    (0x043F08, i_043f08),
    (0x043F0C, i_043f0c),
    (0x043F0E, i_043f0e),
    (0x043F12, i_043f12),
    (0x043F16, i_043f16),
    (0x043F18, i_043f18),
    (0x043F1A, i_043f1a),
    (0x043F1C, i_043f1c),
    (0x043F20, i_043f20),
    (0x043F24, i_043f24),
    (0x043F26, i_043f26),
    (0x043F28, i_043f28),
    (0x043F2A, i_043f2a),
    (0x043F2E, i_043f2e),
    (0x043F30, i_043f30),
    (0x043F34, i_043f34),
    (0x043F38, i_043f38),
    (0x043F3C, i_043f3c),
    (0x043F3E, i_043f3e),
    (0x043F40, i_043f40),
    (0x043F42, i_043f42),
    (0x043F44, i_043f44),
    (0x043F46, i_043f46),
    (0x043F4A, i_043f4a),
    (0x043F4E, i_043f4e),
    (0x043F52, i_043f52),
    (0x043F56, i_043f56),
    (0x043F58, i_043f58),
    (0x043F5C, i_043f5c),
    (0x043F5E, i_043f5e),
    (0x043F64, i_043f64),
    (0x043F68, i_043f68),
    (0x043F6A, i_043f6a),
    (0x043F6E, i_043f6e),
    (0x043F72, i_043f72),
    (0x043F74, i_043f74),
    (0x043F76, i_043f76),
    (0x043F78, i_043f78),
    (0x043F7C, i_043f7c),
    (0x043F80, i_043f80),
    (0x043F82, i_043f82),
    (0x043F84, i_043f84),
    (0x043F86, i_043f86),
    (0x043F8A, i_043f8a),
    (0x043F8C, i_043f8c),
    (0x043F90, i_043f90),
    (0x043F94, i_043f94),
    (0x043F98, i_043f98),
    (0x043F9A, i_043f9a),
    (0x043F9C, i_043f9c),
    (0x043F9E, i_043f9e),
    (0x043FA0, i_043fa0),
    (0x043FA2, i_043fa2),
    (0x043FA6, i_043fa6),
    (0x043FAA, i_043faa),
    (0x043FAC, i_043fac),
    (0x043FB0, i_043fb0),
    (0x043FB4, i_043fb4),
    (0x043FB6, i_043fb6),
    (0x043FBC, i_043fbc),
    (0x043FC0, i_043fc0),
    (0x043FC2, i_043fc2),
    (0x043FC6, i_043fc6),
    (0x043FCA, i_043fca),
    (0x043FCE, i_043fce),
    (0x043FD0, i_043fd0),
    (0x043FD4, i_043fd4),
    (0x043FD8, i_043fd8),
    (0x043FDC, i_043fdc),
    (0x043FE0, i_043fe0),
    (0x043FE4, i_043fe4),
    (0x043FE6, i_043fe6),
    (0x043FEA, i_043fea),
    (0x043FEE, i_043fee),
    (0x043FF2, i_043ff2),
    (0x043FF4, i_043ff4),
    (0x043FF6, i_043ff6),
    (0x043FF8, i_043ff8),
    (0x043FFC, i_043ffc),
    (0x043FFE, i_043ffe),
    (0x044004, i_044004),
    (0x044008, i_044008),
    (0x04400A, i_04400a),
    (0x04400C, i_04400c),
    (0x04400E, i_04400e),
    (0x044014, i_044014),
    (0x044016, i_044016),
    (0x04401A, i_04401a),
    (0x044020, i_044020),
    (0x044022, i_044022),
    (0x044028, i_044028),
    (0x04402E, i_04402e),
    (0x044032, i_044032),
    (0x044034, i_044034),
    (0x044036, i_044036),
    (0x04403C, i_04403c),
    (0x044040, i_044040),
    (0x044042, i_044042),
    (0x044046, i_044046),
    (0x044048, i_044048),
    (0x04404E, i_04404e),
    (0x044052, i_044052),
    (0x044058, i_044058),
    (0x04405C, i_04405c),
    (0x04405E, i_04405e),
    (0x044064, i_044064),
    (0x044068, i_044068),
    (0x04406E, i_04406e),
    (0x044072, i_044072),
    (0x044078, i_044078),
    (0x04407A, i_04407a),
    (0x044080, i_044080),
    (0x044084, i_044084),
    (0x04408A, i_04408a),
    (0x04408E, i_04408e),
    (0x044090, i_044090),
    (0x044096, i_044096),
    (0x04409C, i_04409c),
    (0x0440A2, i_0440a2),
    (0x0440A8, i_0440a8),
    (0x0440AA, i_0440aa),
    (0x0440B0, i_0440b0),
    (0x0440B2, i_0440b2),
    (0x0440B8, i_0440b8),
    (0x0440BA, i_0440ba),
    (0x0440BC, i_0440bc),
    (0x0440C2, i_0440c2),
    (0x0440C6, i_0440c6),
    (0x0440C8, i_0440c8),
    (0x0440CE, i_0440ce),
    (0x0440D0, i_0440d0),
    (0x0440D6, i_0440d6),
    (0x0440DC, i_0440dc),
    (0x0440DE, i_0440de),
    (0x0440E2, i_0440e2),
    (0x0440E4, i_0440e4),
    (0x0440EA, i_0440ea),
    (0x0440EE, i_0440ee),
    (0x0440F2, i_0440f2),
    (0x0440F6, i_0440f6),
    (0x0440FC, i_0440fc),
    (0x044102, i_044102),
    (0x044106, i_044106),
    (0x04410C, i_04410c),
    (0x044112, i_044112),
    (0x044116, i_044116),
    (0x04411C, i_04411c),
    (0x044120, i_044120),
    (0x044122, i_044122),
    (0x044124, i_044124),
    (0x044128, i_044128),
    (0x04412E, i_04412e),
    (0x044132, i_044132),
    (0x044138, i_044138),
    (0x04413C, i_04413c),
    (0x044142, i_044142),
    (0x044146, i_044146),
    (0x044148, i_044148),
    (0x04414E, i_04414e),
    (0x044150, i_044150),
    (0x044152, i_044152),
    (0x044154, i_044154),
    (0x044156, i_044156),
    (0x04415A, i_04415a),
    (0x04415C, i_04415c),
    (0x04415E, i_04415e),
    (0x044166, i_044166),
    (0x04416A, i_04416a),
    (0x044170, i_044170),
    (0x044174, i_044174),
    (0x044178, i_044178),
    (0x04417C, i_04417c),
    (0x044180, i_044180),
    (0x044182, i_044182),
    (0x044188, i_044188),
    (0x04418C, i_04418c),
    (0x044192, i_044192),
    (0x044198, i_044198),
    (0x04419E, i_04419e),
    (0x0441A4, i_0441a4),
    (0x0441A8, i_0441a8),
    (0x0441AE, i_0441ae),
    (0x0441B4, i_0441b4),
    (0x0441B8, i_0441b8),
    (0x0441BE, i_0441be),
    (0x0441C2, i_0441c2),
    (0x0441C4, i_0441c4),
    (0x0441C6, i_0441c6),
    (0x0441CA, i_0441ca),
    (0x0441D0, i_0441d0),
    (0x0441D4, i_0441d4),
    (0x0441DA, i_0441da),
    (0x0441DE, i_0441de),
    (0x0441E4, i_0441e4),
    (0x0441E8, i_0441e8),
    (0x0441EA, i_0441ea),
    (0x0441F0, i_0441f0),
    (0x0441F2, i_0441f2),
    (0x0441F4, i_0441f4),
    (0x0441F6, i_0441f6),
    (0x0441F8, i_0441f8),
    (0x0441FC, i_0441fc),
    (0x0441FE, i_0441fe),
    (0x044200, i_044200),
    (0x044208, i_044208),
    (0x04420C, i_04420c),
    (0x044212, i_044212),
    (0x044216, i_044216),
    (0x04421C, i_04421c),
    (0x044222, i_044222),
    (0x044228, i_044228),
    (0x044236, i_044236),
    (0x04423A, i_04423a),
    (0x04423E, i_04423e),
    (0x044240, i_044240),
    (0x044242, i_044242),
    (0x044246, i_044246),
    (0x04424A, i_04424a),
    (0x04424E, i_04424e),
    (0x044252, i_044252),
    (0x044256, i_044256),
    (0x04425A, i_04425a),
    (0x04425C, i_04425c),
    (0x044260, i_044260),
    (0x04429E, i_04429e),
    (0x0442A0, i_0442a0),
    (0x0442A2, i_0442a2),
    (0x0442A6, i_0442a6),
    (0x0442AC, i_0442ac),
    (0x0442B0, i_0442b0),
    (0x0442B4, i_0442b4),
    (0x0442BA, i_0442ba),
    (0x0442BE, i_0442be),
    (0x0442C2, i_0442c2),
    (0x0442C4, i_0442c4),
    (0x0442C8, i_0442c8),
    (0x0442CC, i_0442cc),
    (0x0442D0, i_0442d0),
    (0x0442D4, i_0442d4),
    (0x0442DA, i_0442da),
    (0x0442E0, i_0442e0),
    (0x0442E2, i_0442e2),
    (0x0442E6, i_0442e6),
    (0x0442E8, i_0442e8),
    (0x0442EE, i_0442ee),
    (0x0442F2, i_0442f2),
    (0x0442F8, i_0442f8),
    (0x0442FE, i_0442fe),
    (0x044300, i_044300),
    (0x044302, i_044302),
    (0x044304, i_044304),
    (0x044306, i_044306),
    (0x04430A, i_04430a),
    (0x04430E, i_04430e),
    (0x044314, i_044314),
    (0x044318, i_044318),
    (0x04431E, i_04431e),
    (0x044322, i_044322),
    (0x044328, i_044328),
    (0x04432C, i_04432c),
    (0x04432E, i_04432e),
    (0x044334, i_044334),
    (0x04433A, i_04433a),
    (0x044340, i_044340),
    (0x044346, i_044346),
    (0x04434C, i_04434c),
    (0x044352, i_044352),
    (0x044354, i_044354),
    (0x04435A, i_04435a),
    (0x04435E, i_04435e),
    (0x044362, i_044362),
    (0x044366, i_044366),
    (0x044368, i_044368),
    (0x04436C, i_04436c),
    (0x044372, i_044372),
    (0x044376, i_044376),
    (0x04437A, i_04437a),
    (0x04437E, i_04437e),
    (0x044382, i_044382),
    (0x044386, i_044386),
    (0x04438C, i_04438c),
    (0x044390, i_044390),
    (0x044392, i_044392),
    (0x044396, i_044396),
    (0x04439A, i_04439a),
    (0x04439E, i_04439e),
    (0x0443A0, i_0443a0),
    (0x0443A4, i_0443a4),
    (0x0443A8, i_0443a8),
    (0x0443AC, i_0443ac),
    (0x0443B0, i_0443b0),
    (0x0443B4, i_0443b4),
    (0x0443B6, i_0443b6),
    (0x0443BA, i_0443ba),
    (0x0443BE, i_0443be),
    (0x0443C0, i_0443c0),
    (0x0443C2, i_0443c2),
    (0x0443C4, i_0443c4),
    (0x0443C8, i_0443c8),
    (0x0443CC, i_0443cc),
    (0x0443CE, i_0443ce),
    (0x0443D0, i_0443d0),
    (0x0443D4, i_0443d4),
    (0x0443D6, i_0443d6),
    (0x0443D8, i_0443d8),
    (0x0443DA, i_0443da),
    (0x0443DC, i_0443dc),
    (0x0443E0, i_0443e0),
    (0x0443E4, i_0443e4),
    (0x0443E8, i_0443e8),
    (0x0443EC, i_0443ec),
    (0x0443F0, i_0443f0),
    (0x0443F4, i_0443f4),
    (0x0443F8, i_0443f8),
    (0x0443FC, i_0443fc),
    (0x044400, i_044400),
    (0x044404, i_044404),
    (0x044408, i_044408),
    (0x04440C, i_04440c),
    (0x044410, i_044410),
    (0x044414, i_044414),
    (0x044418, i_044418),
    (0x04441E, i_04441e),
    (0x044420, i_044420),
    (0x044424, i_044424),
    (0x044426, i_044426),
    (0x04442C, i_04442c),
    (0x044434, i_044434),
    (0x044438, i_044438),
    (0x04443A, i_04443a),
    (0x044440, i_044440),
    (0x044444, i_044444),
    (0x044446, i_044446),
    (0x04444C, i_04444c),
    (0x044450, i_044450),
    (0x044454, i_044454),
    (0x044458, i_044458),
    (0x04445A, i_04445a),
    (0x044460, i_044460),
    (0x044466, i_044466),
    (0x04446A, i_04446a),
    (0x04446C, i_04446c),
    (0x04446E, i_04446e),
    (0x044470, i_044470),
    (0x044472, i_044472),
    (0x044474, i_044474),
    (0x04447C, i_04447c),
    (0x044480, i_044480),
    (0x044486, i_044486),
    (0x04448A, i_04448a),
    (0x04448E, i_04448e),
    (0x044496, i_044496),
    (0x04449A, i_04449a),
    (0x0444A0, i_0444a0),
    (0x0444A4, i_0444a4),
    (0x0444A8, i_0444a8),
    (0x0444AA, i_0444aa),
    (0x0444AE, i_0444ae),
    (0x0444B0, i_0444b0),
    (0x0444B4, i_0444b4),
    (0x0444B6, i_0444b6),
    (0x0444BC, i_0444bc),
    (0x0444C0, i_0444c0),
    (0x0444C6, i_0444c6),
    (0x0444CA, i_0444ca),
    (0x0444CE, i_0444ce),
    (0x0444D2, i_0444d2),
    (0x0444D6, i_0444d6),
    (0x0444DA, i_0444da),
    (0x0444DE, i_0444de),
    (0x0444E2, i_0444e2),
    (0x0444E6, i_0444e6),
    (0x0444E8, i_0444e8),
    (0x0444EC, i_0444ec),
    (0x0444F0, i_0444f0),
    (0x0444F8, i_0444f8),
    (0x0444FC, i_0444fc),
    (0x044502, i_044502),
    (0x044506, i_044506),
    (0x04450E, i_04450e),
    (0x044512, i_044512),
    (0x044518, i_044518),
    (0x04451C, i_04451c),
    (0x04451E, i_04451e),
    (0x044522, i_044522),
    (0x044524, i_044524),
    (0x04452A, i_04452a),
    (0x04452E, i_04452e),
    (0x044534, i_044534),
    (0x044538, i_044538),
    (0x04453C, i_04453c),
    (0x044540, i_044540),
    (0x044544, i_044544),
    (0x044548, i_044548),
    (0x04454A, i_04454a),
    (0x04454E, i_04454e),
    (0x044552, i_044552),
    (0x044556, i_044556),
    (0x044558, i_044558),
    (0x04455A, i_04455a),
    (0x04455C, i_04455c),
    (0x04455E, i_04455e),
    (0x044566, i_044566),
    (0x04456A, i_04456a),
    (0x044570, i_044570),
    (0x044574, i_044574),
    (0x04457C, i_04457c),
    (0x044580, i_044580),
    (0x044586, i_044586),
    (0x04458A, i_04458a),
    (0x04458C, i_04458c),
    (0x044590, i_044590),
    (0x044592, i_044592),
    (0x044594, i_044594),
    (0x044598, i_044598),
    (0x04459A, i_04459a),
    (0x04459E, i_04459e),
    (0x0445A0, i_0445a0),
    (0x0445A4, i_0445a4),
    (0x0445A6, i_0445a6),
    (0x0445AC, i_0445ac),
    (0x0445B0, i_0445b0),
    (0x0445B6, i_0445b6),
    (0x0445BC, i_0445bc),
    (0x0445C0, i_0445c0),
    (0x0445C2, i_0445c2),
    (0x0445C6, i_0445c6),
    (0x0445C8, i_0445c8),
    (0x0445CC, i_0445cc),
    (0x0445CE, i_0445ce),
    (0x0445D0, i_0445d0),
    (0x0445D4, i_0445d4),
    (0x0445D8, i_0445d8),
    (0x0445DC, i_0445dc),
    (0x0445E0, i_0445e0),
    (0x0445E4, i_0445e4),
    (0x0445E6, i_0445e6),
    (0x0445E8, i_0445e8),
    (0x0445EC, i_0445ec),
    (0x0445F0, i_0445f0),
    (0x0445F4, i_0445f4),
    (0x0445F8, i_0445f8),
    (0x0445FC, i_0445fc),
    (0x044600, i_044600),
    (0x044604, i_044604),
    (0x044608, i_044608),
    (0x04460A, i_04460a),
    (0x04460E, i_04460e),
    (0x044610, i_044610),
    (0x044614, i_044614),
    (0x044618, i_044618),
    (0x04461A, i_04461a),
    (0x04461C, i_04461c),
    (0x044620, i_044620),
    (0x044622, i_044622),
    (0x044624, i_044624),
    (0x044628, i_044628),
    (0x04462C, i_04462c),
    (0x044630, i_044630),
    (0x044632, i_044632),
    (0x044636, i_044636),
    (0x044638, i_044638),
    (0x04463A, i_04463a),
    (0x04463E, i_04463e),
    (0x044640, i_044640),
    (0x044644, i_044644),
    (0x044648, i_044648),
    (0x04464A, i_04464a),
    (0x04464C, i_04464c),
    (0x044650, i_044650),
    (0x044652, i_044652),
    (0x044654, i_044654),
    (0x044658, i_044658),
    (0x04465C, i_04465c),
    (0x04465E, i_04465e),
    (0x044660, i_044660),
    (0x044664, i_044664),
    (0x044666, i_044666),
    (0x04466A, i_04466a),
    (0x04466C, i_04466c),
    (0x044670, i_044670),
    (0x044672, i_044672),
    (0x044676, i_044676),
    (0x04467C, i_04467c),
    (0x044680, i_044680),
    (0x044682, i_044682),
    (0x044684, i_044684),
    (0x044686, i_044686),
    (0x044688, i_044688),
    (0x04468A, i_04468a),
    (0x04468E, i_04468e),
    (0x044692, i_044692),
    (0x044694, i_044694),
    (0x044698, i_044698),
    (0x04469A, i_04469a),
    (0x04469C, i_04469c),
    (0x04469E, i_04469e),
    (0x0446A2, i_0446a2),
    (0x0446A6, i_0446a6),
    (0x0446A8, i_0446a8),
    (0x0446AA, i_0446aa),
    (0x0446AE, i_0446ae),
    (0x0446B0, i_0446b0),
    (0x0446B4, i_0446b4),
    (0x0446B6, i_0446b6),
    (0x0446B8, i_0446b8),
    (0x0446BA, i_0446ba),
    (0x0446BE, i_0446be),
    (0x0446C2, i_0446c2),
    (0x0446C4, i_0446c4),
    (0x0446C6, i_0446c6),
    (0x0446CA, i_0446ca),
    (0x0446CC, i_0446cc),
    (0x0446CE, i_0446ce),
    (0x0446D0, i_0446d0),
    (0x0446D2, i_0446d2),
    (0x0446D6, i_0446d6),
    (0x0446DA, i_0446da),
    (0x0446DE, i_0446de),
    (0x0446E2, i_0446e2),
    (0x0446E6, i_0446e6),
    (0x0446EA, i_0446ea),
    (0x0446EE, i_0446ee),
    (0x0446F2, i_0446f2),
    (0x0446F6, i_0446f6),
    (0x0446FA, i_0446fa),
    (0x0446FE, i_0446fe),
    (0x044702, i_044702),
    (0x044706, i_044706),
    (0x04470A, i_04470a),
    (0x04470E, i_04470e),
    (0x044712, i_044712),
    (0x044714, i_044714),
    (0x044718, i_044718),
    (0x04471A, i_04471a),
    (0x04471E, i_04471e),
    (0x044720, i_044720),
    (0x044724, i_044724),
    (0x044726, i_044726),
    (0x04472A, i_04472a),
    (0x04472C, i_04472c),
    (0x044730, i_044730),
    (0x044732, i_044732),
    (0x044736, i_044736),
    (0x04473A, i_04473a),
    (0x04473C, i_04473c),
    (0x044740, i_044740),
    (0x044742, i_044742),
    (0x044746, i_044746),
    (0x04474A, i_04474a),
    (0x04474C, i_04474c),
    (0x04474E, i_04474e),
    (0x044752, i_044752),
    (0x044754, i_044754),
    (0x044756, i_044756),
    (0x04475A, i_04475a),
    (0x04475E, i_04475e),
    (0x044762, i_044762),
    (0x044764, i_044764),
    (0x044768, i_044768),
    (0x04476A, i_04476a),
    (0x04476E, i_04476e),
    (0x044770, i_044770),
    (0x044774, i_044774),
    (0x044778, i_044778),
    (0x04477A, i_04477a),
    (0x04477C, i_04477c),
    (0x044780, i_044780),
    (0x044782, i_044782),
    (0x044784, i_044784),
    (0x044788, i_044788),
    (0x04478C, i_04478c),
    (0x044790, i_044790),
    (0x044792, i_044792),
    (0x044796, i_044796),
    (0x044798, i_044798),
    (0x04479C, i_04479c),
    (0x0447A0, i_0447a0),
    (0x0447A4, i_0447a4),
    (0x0447A6, i_0447a6),
    (0x0447AA, i_0447aa),
    (0x0447AE, i_0447ae),
    (0x0447B2, i_0447b2),
    (0x0447B4, i_0447b4),
    (0x0447B8, i_0447b8),
    (0x0447BC, i_0447bc),
    (0x0447C0, i_0447c0),
    (0x0447C4, i_0447c4),
    (0x0447C6, i_0447c6),
    (0x0447CA, i_0447ca),
    (0x0447CE, i_0447ce),
    (0x0447D4, i_0447d4),
    (0x0447D8, i_0447d8),
    (0x0447DC, i_0447dc),
    (0x0447E0, i_0447e0),
    (0x0447E4, i_0447e4),
    (0x0447E8, i_0447e8),
    (0x0447EA, i_0447ea),
    (0x0447EE, i_0447ee),
    (0x0447F4, i_0447f4),
    (0x0447F6, i_0447f6),
    (0x0447FA, i_0447fa),
    (0x0447FE, i_0447fe),
    (0x044802, i_044802),
    (0x044804, i_044804),
    (0x044808, i_044808),
    (0x04480E, i_04480e),
    (0x044812, i_044812),
    (0x044816, i_044816),
    (0x044818, i_044818),
    (0x04481C, i_04481c),
    (0x044820, i_044820),
    (0x044824, i_044824),
    (0x044828, i_044828),
    (0x04482A, i_04482a),
    (0x044830, i_044830),
    (0x044834, i_044834),
    (0x04483A, i_04483a),
    (0x04483E, i_04483e),
    (0x044842, i_044842),
    (0x044848, i_044848),
    (0x04484E, i_04484e),
    (0x044850, i_044850),
    (0x044854, i_044854),
    (0x044858, i_044858),
    (0x04485A, i_04485a),
    (0x04485E, i_04485e),
    (0x044860, i_044860),
    (0x044862, i_044862),
    (0x044866, i_044866),
    (0x04486A, i_04486a),
    (0x04486C, i_04486c),
    (0x044872, i_044872),
    (0x044876, i_044876),
    (0x04487A, i_04487a),
    (0x04487E, i_04487e),
    (0x044884, i_044884),
    (0x044888, i_044888),
    (0x04488C, i_04488c),
    (0x04488E, i_04488e),
    (0x044892, i_044892),
    (0x044896, i_044896),
    (0x04489C, i_04489c),
    (0x0448A2, i_0448a2),
    (0x0448A4, i_0448a4),
    (0x0448A6, i_0448a6),
    (0x0448AA, i_0448aa),
    (0x0448AE, i_0448ae),
    (0x0448B0, i_0448b0),
    (0x0448B2, i_0448b2),
    (0x0448B6, i_0448b6),
    (0x0448B8, i_0448b8),
    (0x0448BA, i_0448ba),
    (0x0448BE, i_0448be),
    (0x0448C2, i_0448c2),
    (0x0448C6, i_0448c6),
    (0x0448CA, i_0448ca),
    (0x0448CE, i_0448ce),
    (0x0448D4, i_0448d4),
    (0x0448D8, i_0448d8),
    (0x0448DC, i_0448dc),
    (0x0448E2, i_0448e2),
    (0x0448E6, i_0448e6),
    (0x0448E8, i_0448e8),
    (0x0448EC, i_0448ec),
    (0x0448EE, i_0448ee),
    (0x0448F0, i_0448f0),
    (0x0448F6, i_0448f6),
    (0x0448FA, i_0448fa),
    (0x0448FE, i_0448fe),
    (0x044904, i_044904),
    (0x04490A, i_04490a),
    (0x04490E, i_04490e),
    (0x044912, i_044912),
    (0x044916, i_044916),
    (0x04491A, i_04491a),
    (0x044920, i_044920),
    (0x044924, i_044924),
    (0x044928, i_044928),
    (0x04492C, i_04492c),
    (0x044932, i_044932),
    (0x044936, i_044936),
    (0x04493A, i_04493a),
    (0x04493E, i_04493e),
    (0x044942, i_044942),
    (0x044948, i_044948),
    (0x04494A, i_04494a),
    (0x04494E, i_04494e),
    (0x044952, i_044952),
    (0x044958, i_044958),
    (0x04495C, i_04495c),
    (0x044962, i_044962),
    (0x044966, i_044966),
    (0x04496C, i_04496c),
    (0x044972, i_044972),
    (0x044976, i_044976),
    (0x044978, i_044978),
    (0x04497C, i_04497c),
    (0x044980, i_044980),
    (0x044984, i_044984),
    (0x044986, i_044986),
    (0x044988, i_044988),
    (0x04498C, i_04498c),
    (0x04498E, i_04498e),
    (0x044990, i_044990),
    (0x044996, i_044996),
    (0x04499C, i_04499c),
    (0x0449A2, i_0449a2),
    (0x0449A4, i_0449a4),
    (0x0449A6, i_0449a6),
    (0x0449AA, i_0449aa),
    (0x0449AE, i_0449ae),
    (0x0449B2, i_0449b2),
    (0x0449B6, i_0449b6),
    (0x0449BA, i_0449ba),
    (0x0449BC, i_0449bc),
    (0x0449C0, i_0449c0),
    (0x0449C2, i_0449c2),
    (0x0449C8, i_0449c8),
    (0x0449CE, i_0449ce),
    (0x0449D2, i_0449d2),
    (0x0449D8, i_0449d8),
    (0x0449DC, i_0449dc),
    (0x0449E2, i_0449e2),
    (0x0449E8, i_0449e8),
    (0x0449EE, i_0449ee),
    (0x0449F4, i_0449f4),
    (0x0449F8, i_0449f8),
    (0x0449FC, i_0449fc),
    (0x044A00, i_044a00),
    (0x044A04, i_044a04),
    (0x044A08, i_044a08),
    (0x044A0C, i_044a0c),
    (0x044A10, i_044a10),
    (0x044A12, i_044a12),
    (0x044A16, i_044a16),
    (0x044A1A, i_044a1a),
    (0x044A1E, i_044a1e),
    (0x044A24, i_044a24),
    (0x044A2A, i_044a2a),
    (0x044A2C, i_044a2c),
    (0x044A32, i_044a32),
    (0x044A36, i_044a36),
    (0x044A3C, i_044a3c),
    (0x044A40, i_044a40),
    (0x044A46, i_044a46),
    (0x044A4A, i_044a4a),
    (0x044A50, i_044a50),
    (0x044A54, i_044a54),
    (0x044A58, i_044a58),
    (0x044A5C, i_044a5c),
    (0x044A60, i_044a60),
    (0x044A62, i_044a62),
    (0x044A64, i_044a64),
    (0x044A68, i_044a68),
    (0x044A6C, i_044a6c),
    (0x044A72, i_044a72),
    (0x044A78, i_044a78),
    (0x044A7C, i_044a7c),
    (0x044A7E, i_044a7e),
    (0x044A82, i_044a82),
    (0x044A86, i_044a86),
    (0x044A8A, i_044a8a),
    (0x044A90, i_044a90),
    (0x044A94, i_044a94),
    (0x044A96, i_044a96),
    (0x044A98, i_044a98),
    (0x044A9C, i_044a9c),
    (0x044AA0, i_044aa0),
    (0x044AA6, i_044aa6),
    (0x044AAA, i_044aaa),
    (0x044AB0, i_044ab0),
    (0x044AB4, i_044ab4),
    (0x044AB8, i_044ab8),
    (0x044ABC, i_044abc),
    (0x044ABE, i_044abe),
    (0x044AC2, i_044ac2),
    (0x044AC6, i_044ac6),
    (0x044ACA, i_044aca),
    (0x044AD0, i_044ad0),
    (0x044AD4, i_044ad4),
    (0x044AD8, i_044ad8),
    (0x044ADC, i_044adc),
    (0x044ADE, i_044ade),
    (0x044AE4, i_044ae4),
    (0x044AE8, i_044ae8),
    (0x044AEC, i_044aec),
    (0x044AF2, i_044af2),
    (0x044AF6, i_044af6),
    (0x044AFA, i_044afa),
    (0x044AFC, i_044afc),
    (0x044AFE, i_044afe),
    (0x044B02, i_044b02),
    (0x044B06, i_044b06),
    (0x044B0C, i_044b0c),
    (0x044B12, i_044b12),
    (0x044B16, i_044b16),
    (0x044B1A, i_044b1a),
    (0x044B20, i_044b20),
    (0x044B24, i_044b24),
    (0x044B28, i_044b28),
    (0x044B2E, i_044b2e),
    (0x044B32, i_044b32),
    (0x044B38, i_044b38),
    (0x044B3C, i_044b3c),
    (0x044B42, i_044b42),
    (0x044B46, i_044b46),
    (0x044B4A, i_044b4a),
    (0x044B50, i_044b50),
    (0x044B56, i_044b56),
    (0x044B5C, i_044b5c),
    (0x044B60, i_044b60),
    (0x044B66, i_044b66),
    (0x044B6C, i_044b6c),
    (0x044B72, i_044b72),
    (0x044B78, i_044b78),
    (0x044B7E, i_044b7e),
    (0x044B82, i_044b82),
    (0x044B84, i_044b84),
    (0x044B88, i_044b88),
    (0x044B8E, i_044b8e),
    (0x044B94, i_044b94),
    (0x044B9A, i_044b9a),
    (0x044B9E, i_044b9e),
    (0x044BA4, i_044ba4),
    (0x044BA8, i_044ba8),
    (0x044BAA, i_044baa),
    (0x044BAE, i_044bae),
    (0x044BB2, i_044bb2),
    (0x044BB8, i_044bb8),
    (0x044BBC, i_044bbc),
    (0x044BBE, i_044bbe),
    (0x044BC2, i_044bc2),
    (0x044BC6, i_044bc6),
    (0x044BCA, i_044bca),
    (0x044BCE, i_044bce),
    (0x044BD0, i_044bd0),
    (0x044BD2, i_044bd2),
    (0x044BD6, i_044bd6),
    (0x044BDA, i_044bda),
    (0x044BDE, i_044bde),
    (0x044BE2, i_044be2),
    (0x044BE8, i_044be8),
    (0x044BEC, i_044bec),
    (0x044BF2, i_044bf2),
    (0x044BF8, i_044bf8),
    (0x044BFE, i_044bfe),
    (0x044C04, i_044c04),
    (0x044C0A, i_044c0a),
    (0x044C10, i_044c10),
    (0x044C16, i_044c16),
    (0x044C1A, i_044c1a),
    (0x044C20, i_044c20),
    (0x044C26, i_044c26),
    (0x044C28, i_044c28),
    (0x044C2C, i_044c2c),
    (0x044C32, i_044c32),
    (0x044C36, i_044c36),
    (0x044C38, i_044c38),
    (0x044C3E, i_044c3e),
    (0x044C42, i_044c42),
    (0x044C48, i_044c48),
    (0x044C4A, i_044c4a),
    (0x044C50, i_044c50),
    (0x044C56, i_044c56),
    (0x044C5C, i_044c5c),
    (0x044C62, i_044c62),
    (0x044C68, i_044c68),
    (0x044C6E, i_044c6e),
    (0x044C72, i_044c72),
    (0x044C74, i_044c74),
    (0x044C78, i_044c78),
    (0x044C7E, i_044c7e),
    (0x044C82, i_044c82),
    (0x044C86, i_044c86),
    (0x044C8A, i_044c8a),
    (0x044C8E, i_044c8e),
    (0x044C90, i_044c90),
    (0x044C94, i_044c94),
    (0x044C98, i_044c98),
    (0x044C9C, i_044c9c),
    (0x044CA2, i_044ca2),
    (0x044CA6, i_044ca6),
    (0x044CA8, i_044ca8),
    (0x044CAC, i_044cac),
    (0x044CB2, i_044cb2),
    (0x044CB8, i_044cb8),
    (0x044CBC, i_044cbc),
    (0x044CBE, i_044cbe),
    (0x044CC2, i_044cc2),
    (0x044CC8, i_044cc8),
    (0x044CCC, i_044ccc),
    (0x044CD2, i_044cd2),
    (0x044CD6, i_044cd6),
    (0x044CD8, i_044cd8),
    (0x044CDA, i_044cda),
    (0x044CDE, i_044cde),
    (0x044CE4, i_044ce4),
    (0x044CEA, i_044cea),
    (0x044CF0, i_044cf0),
    (0x044CF4, i_044cf4),
    (0x044CF6, i_044cf6),
    (0x044CFA, i_044cfa),
    (0x044D00, i_044d00),
    (0x044D04, i_044d04),
    (0x044D08, i_044d08),
    (0x044D0A, i_044d0a),
    (0x044D10, i_044d10),
    (0x044D14, i_044d14),
    (0x044D18, i_044d18),
    (0x044D1A, i_044d1a),
    (0x044D1E, i_044d1e),
    (0x044D24, i_044d24),
    (0x044D28, i_044d28),
    (0x044D2C, i_044d2c),
    (0x044D2E, i_044d2e),
    (0x044D40, i_044d40),
    (0x044D44, i_044d44),
    (0x044D4A, i_044d4a),
    (0x044D50, i_044d50),
    (0x044D56, i_044d56),
    (0x044D5A, i_044d5a),
    (0x044D5E, i_044d5e),
    (0x044D62, i_044d62),
    (0x044D66, i_044d66),
    (0x044D6C, i_044d6c),
    (0x044D70, i_044d70),
    (0x044D76, i_044d76),
    (0x044D7A, i_044d7a),
    (0x044D80, i_044d80),
    (0x044D86, i_044d86),
    (0x044D8C, i_044d8c),
    (0x044D92, i_044d92),
    (0x044D98, i_044d98),
    (0x044D9E, i_044d9e),
    (0x044DA2, i_044da2),
    (0x044DA4, i_044da4),
    (0x044DAA, i_044daa),
    (0x044DB0, i_044db0),
    (0x044DB4, i_044db4),
    (0x044DB8, i_044db8),
    (0x044DBC, i_044dbc),
    (0x044DC2, i_044dc2),
    (0x044DC8, i_044dc8),
    (0x044DCE, i_044dce),
    (0x044DD4, i_044dd4),
    (0x044DD8, i_044dd8),
    (0x044DDA, i_044dda),
    (0x044DE0, i_044de0),
    (0x044DE6, i_044de6),
    (0x044DEA, i_044dea),
    (0x044DEE, i_044dee),
    (0x044DF0, i_044df0),
    (0x044DF2, i_044df2),
    (0x044DF8, i_044df8),
    (0x044DFC, i_044dfc),
    (0x044E02, i_044e02),
    (0x044E0A, i_044e0a),
    (0x044E0E, i_044e0e),
    (0x044E12, i_044e12),
    (0x044E16, i_044e16),
    (0x044E1C, i_044e1c),
    (0x044E24, i_044e24),
    (0x044E28, i_044e28),
    (0x044E2C, i_044e2c),
    (0x044E30, i_044e30),
    (0x044E36, i_044e36),
    (0x044E3E, i_044e3e),
    (0x044E42, i_044e42),
    (0x044E48, i_044e48),
    (0x044E4C, i_044e4c),
    (0x044E50, i_044e50),
    (0x044E56, i_044e56),
    (0x044E5A, i_044e5a),
    (0x044E5E, i_044e5e),
    (0x044E64, i_044e64),
    (0x044E68, i_044e68),
    (0x044E6C, i_044e6c),
    (0x044E70, i_044e70),
    (0x044E74, i_044e74),
    (0x044E7A, i_044e7a),
    (0x044E7E, i_044e7e),
    (0x044E84, i_044e84),
    (0x044E8A, i_044e8a),
    (0x044E90, i_044e90),
    (0x044E96, i_044e96),
    (0x044E9C, i_044e9c),
    (0x044EA0, i_044ea0),
    (0x044EA2, i_044ea2),
    (0x044EA8, i_044ea8),
    (0x044EAC, i_044eac),
    (0x044EB0, i_044eb0),
    (0x044EB2, i_044eb2),
    (0x044EBA, i_044eba),
    (0x044EBE, i_044ebe),
    (0x044EC2, i_044ec2),
    (0x044EC6, i_044ec6),
    (0x044ECA, i_044eca),
    (0x044ECE, i_044ece),
    (0x044ED0, i_044ed0),
    (0x044ED6, i_044ed6),
    (0x044EDC, i_044edc),
    (0x044EE2, i_044ee2),
    (0x044EE6, i_044ee6),
    (0x044EEA, i_044eea),
    (0x044EEC, i_044eec),
    (0x044EEE, i_044eee),
    (0x044EF4, i_044ef4),
    (0x044EFA, i_044efa),
    (0x044EFE, i_044efe),
    (0x044F00, i_044f00),
    (0x044F04, i_044f04),
    (0x044F0A, i_044f0a),
    (0x044F10, i_044f10),
    (0x044F14, i_044f14),
    (0x044F1A, i_044f1a),
    (0x044F1E, i_044f1e),
    (0x044F24, i_044f24),
    (0x044F2A, i_044f2a),
    (0x044F30, i_044f30),
    (0x044F34, i_044f34),
    (0x044F36, i_044f36),
    (0x044F3C, i_044f3c),
    (0x044F40, i_044f40),
    (0x044F44, i_044f44),
    (0x044F46, i_044f46),
    (0x044F4A, i_044f4a),
    (0x044F50, i_044f50),
    (0x044F56, i_044f56),
    (0x044F5C, i_044f5c),
    (0x044F62, i_044f62),
    (0x044F66, i_044f66),
    (0x044F6A, i_044f6a),
    (0x044F6C, i_044f6c),
    (0x044F70, i_044f70),
    (0x044F74, i_044f74),
    (0x044F78, i_044f78),
    (0x044F7E, i_044f7e),
    (0x044F82, i_044f82),
    (0x044F86, i_044f86),
    (0x044F88, i_044f88),
    (0x044F8A, i_044f8a),
    (0x044F8E, i_044f8e),
    (0x044F94, i_044f94),
    (0x044F9A, i_044f9a),
    (0x044FA0, i_044fa0),
    (0x044FA6, i_044fa6),
    (0x044FAC, i_044fac),
    (0x044FB2, i_044fb2),
    (0x044FB6, i_044fb6),
    (0x044FBE, i_044fbe),
    (0x044FC2, i_044fc2),
    (0x044FC6, i_044fc6),
    (0x044FC8, i_044fc8),
    (0x044FCC, i_044fcc),
    (0x044FD0, i_044fd0),
    (0x044FD4, i_044fd4),
    (0x044FD8, i_044fd8),
    (0x044FDC, i_044fdc),
    (0x044FE0, i_044fe0),
    (0x044FE4, i_044fe4),
    (0x044FE8, i_044fe8),
    (0x044FEC, i_044fec),
    (0x044FF0, i_044ff0),
    (0x044FF4, i_044ff4),
    (0x044FF8, i_044ff8),
    (0x044FFC, i_044ffc),
    (0x045000, i_045000),
    (0x045004, i_045004),
    (0x045008, i_045008),
    (0x04500A, i_04500a),
    (0x04500C, i_04500c),
    (0x045010, i_045010),
    (0x045014, i_045014),
    (0x045018, i_045018),
    (0x04501C, i_04501c),
    (0x045020, i_045020),
    (0x045022, i_045022),
    (0x04502A, i_04502a),
    (0x04502E, i_04502e),
    (0x045032, i_045032),
    (0x045036, i_045036),
    (0x04503E, i_04503e),
    (0x045042, i_045042),
    (0x045046, i_045046),
    (0x04504A, i_04504a),
    (0x04504E, i_04504e),
    (0x045054, i_045054),
    (0x045058, i_045058),
    (0x04505A, i_04505a),
    (0x045060, i_045060),
    (0x045064, i_045064),
    (0x04506A, i_04506a),
    (0x04506E, i_04506e),
    (0x045072, i_045072),
    (0x045078, i_045078),
    (0x04507C, i_04507c),
    (0x045082, i_045082),
    (0x045086, i_045086),
    (0x04508C, i_04508c),
    (0x045090, i_045090),
    (0x045096, i_045096),
    (0x04509A, i_04509a),
    (0x04509E, i_04509e),
    (0x0450A4, i_0450a4),
    (0x0450A8, i_0450a8),
    (0x0450AC, i_0450ac),
    (0x0450B0, i_0450b0),
    (0x0450B6, i_0450b6),
    (0x0450BA, i_0450ba),
    (0x0450BE, i_0450be),
    (0x0450C4, i_0450c4),
    (0x0450C8, i_0450c8),
    (0x0450CE, i_0450ce),
    (0x0450D2, i_0450d2),
    (0x0450D8, i_0450d8),
    (0x0450DC, i_0450dc),
    (0x0450E2, i_0450e2),
    (0x0450E6, i_0450e6),
    (0x0450EA, i_0450ea),
    (0x0450F0, i_0450f0),
    (0x0450F4, i_0450f4),
    (0x0450FA, i_0450fa),
    (0x0450FE, i_0450fe),
    (0x045104, i_045104),
    (0x045108, i_045108),
    (0x04510E, i_04510e),
    (0x045112, i_045112),
    (0x045116, i_045116),
    (0x04511A, i_04511a),
    (0x04511E, i_04511e),
    (0x045124, i_045124),
    (0x045128, i_045128),
    (0x04512C, i_04512c),
    (0x045132, i_045132),
    (0x045136, i_045136),
    (0x04513C, i_04513c),
    (0x045140, i_045140),
    (0x045146, i_045146),
    (0x04514A, i_04514a),
    (0x045150, i_045150),
    (0x045154, i_045154),
    (0x045158, i_045158),
    (0x04515E, i_04515e),
    (0x045162, i_045162),
    (0x045168, i_045168),
    (0x04516C, i_04516c),
    (0x045172, i_045172),
    (0x045176, i_045176),
    (0x04517C, i_04517c),
    (0x045180, i_045180),
    (0x045184, i_045184),
    (0x045188, i_045188),
    (0x04518C, i_04518c),
    (0x045192, i_045192),
    (0x045196, i_045196),
    (0x04519A, i_04519a),
    (0x04519E, i_04519e),
    (0x0451A2, i_0451a2),
    (0x0451A6, i_0451a6),
    (0x0451AA, i_0451aa),
    (0x0451AE, i_0451ae),
    (0x0451B2, i_0451b2),
    (0x0451B8, i_0451b8),
];
