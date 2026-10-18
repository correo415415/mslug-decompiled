// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región bios_1) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $C02B18: move.l $2a2a(a3), d6
pub fn i_c02b18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B1C;
    cpu.ir = 0x2C2B;
    let ad2 = cpu.a[3].wrapping_add(0x2A2A);
    let v3 = bus.read32(ad2);
    cpu.d[6] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C02B1C: move.l $2827(a0), -(a4)
pub fn i_c02b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B20;
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
/// $C02B20: move.l -(a6), d3
pub fn i_c02b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B22;
    cpu.ir = 0x2626;
    let ad2 = cpu.a[6].wrapping_sub(4); cpu.a[6] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02B22: move.l -(a5), -(a2)
pub fn i_c02b22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B24;
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
/// $C02B24: move.l -(a4), d2
pub fn i_c02b24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B26;
    cpu.ir = 0x2424;
    let ad2 = cpu.a[4].wrapping_sub(4); cpu.a[4] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02B26: move.l -(a2), -(a1)
pub fn i_c02b26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B28;
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
/// $C02B28: move.l -(a1), d1
pub fn i_c02b28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B2A;
    cpu.ir = 0x2221;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02B2A: move.l -(a0), -(a0)
pub fn i_c02b2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B2C;
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
/// $C02B2C: move.l -(a0), d0
pub fn i_c02b2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B2E;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02B2E: move.b (a7)+, -(a7)
pub fn i_c02b2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B30;
    cpu.ir = 0x1F1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02B30: move.b (a6)+, d7
pub fn i_c02b30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B32;
    cpu.ir = 0x1E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02B32: move.b (a5)+, -(a6)
pub fn i_c02b32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B34;
    cpu.ir = 0x1D1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02B34: move.b (a4)+, -(a6)
pub fn i_c02b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B36;
    cpu.ir = 0x1D1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02B36: move.b (a3)+, d6
pub fn i_c02b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B38;
    cpu.ir = 0x1C1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02B38: move.b (a3)+, -(a5)
pub fn i_c02b38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B3A;
    cpu.ir = 0x1B1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02B3A: move.b (a2)+, d5
pub fn i_c02b3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B3C;
    cpu.ir = 0x1A1A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02B3C: move.b (a1)+, d5
pub fn i_c02b3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B3E;
    cpu.ir = 0x1A19;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02B3E: move.b (a1)+, -(a4)
pub fn i_c02b3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B40;
    cpu.ir = 0x1919;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02B40: ?
pub fn i_c02b40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B42;
    cpu.ir = 0x403F;
    let v1: u32 = 0x0;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x80) != 0 && (r2 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x80 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    4
}
/// $C02B42: ?
pub fn i_c02b42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B44;
    cpu.ir = 0x3E3D;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C02B44: move.w ([$39f8627c, pc, d3.l * 2], $3535), d6
pub fn i_c02b44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B48;
    cpu.ir = 0x3C3B;
    let ad2 = 0xC02B80_u32.wrapping_add(cpu.d[3]);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C02B48: move.w $3736.w, -(a4)
pub fn i_c02b48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B4C;
    cpu.ir = 0x3938;
    let v2 = bus.read16(0x3736) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[4].wrapping_sub(2); cpu.a[4] = ad3;
    bus.write16(ad3, v2 as u16);
    18
}
/// $C02B4C: move.w $33(a5, d3.w), -(a2)
pub fn i_c02b4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B50;
    cpu.ir = 0x3535;
    let ad2 = cpu.a[5].wrapping_add(0x33).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2].wrapping_sub(2); cpu.a[2] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C02B50: move.w $30(a1, d3.w), d1
pub fn i_c02b50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B54;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x30).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C02B54: move.l $2d2d(a6), -(a7)
pub fn i_c02b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B58;
    cpu.ir = 0x2F2E;
    let ad2 = cpu.a[6].wrapping_add(0x2D2D);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[7];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[7] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C02B58: move.l $2b2a(a3), d6
pub fn i_c02b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B5C;
    cpu.ir = 0x2C2B;
    let ad2 = cpu.a[3].wrapping_add(0x2B2A);
    let v3 = bus.read32(ad2);
    cpu.d[6] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C02B5C: move.l $2827(a1), -(a4)
pub fn i_c02b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B60;
    cpu.ir = 0x2929;
    let ad2 = cpu.a[1].wrapping_add(0x2827);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[4];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[4] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C02B60: move.l -(a6), -(a3)
pub fn i_c02b60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B62;
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
/// $C02B62: move.l -(a5), d3
pub fn i_c02b62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B64;
    cpu.ir = 0x2625;
    let ad2 = cpu.a[5].wrapping_sub(4); cpu.a[5] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02B64: move.l -(a4), -(a2)
pub fn i_c02b64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B66;
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
/// $C02B66: move.l -(a3), -(a1)
pub fn i_c02b66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B68;
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
/// $C02B68: move.l -(a2), d1
pub fn i_c02b68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B6A;
    cpu.ir = 0x2222;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02B6A: move.l -(a1), -(a0)
pub fn i_c02b6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B6C;
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
/// $C02B6C: move.l -(a0), d0
pub fn i_c02b6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B6E;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02B6E: move.l (a7)+, d0
pub fn i_c02b6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B70;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C02B70: move.b (a6)+, -(a7)
pub fn i_c02b70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B72;
    cpu.ir = 0x1F1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02B72: move.b (a5)+, d7
pub fn i_c02b72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B74;
    cpu.ir = 0x1E1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02B74: move.b (a5)+, -(a6)
pub fn i_c02b74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B76;
    cpu.ir = 0x1D1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02B76: move.b (a4)+, d6
pub fn i_c02b76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B78;
    cpu.ir = 0x1C1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02B78: move.b (a3)+, d6
pub fn i_c02b78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B7A;
    cpu.ir = 0x1C1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02B7A: move.b (a3)+, -(a5)
pub fn i_c02b7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B7C;
    cpu.ir = 0x1B1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02B7C: move.b (a2)+, d5
pub fn i_c02b7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B7E;
    cpu.ir = 0x1A1A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02B7E: move.b (a1)+, d5
pub fn i_c02b7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B80;
    cpu.ir = 0x1A19;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02B80: ?
pub fn i_c02b80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B82;
    cpu.ir = 0x403F;
    let v1: u32 = 0x0;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x80) != 0 && (r2 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x80 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    4
}
/// $C02B82: ?
pub fn i_c02b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B84;
    cpu.ir = 0x3E3D;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C02B84: move.w #$3b3a, d6
pub fn i_c02b84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B88;
    cpu.ir = 0x3C3C;
    let v1: u32 = 0x3B3A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C02B88: move.w $3736.w, -(a4)
pub fn i_c02b88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B8C;
    cpu.ir = 0x3938;
    let v2 = bus.read16(0x3736) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[4].wrapping_sub(2); cpu.a[4] = ad3;
    bus.write16(ad3, v2 as u16);
    18
}
/// $C02B8C: move.w $33(a5, d3.w), d3
pub fn i_c02b8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B90;
    cpu.ir = 0x3635;
    let ad2 = cpu.a[5].wrapping_add(0x33).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C02B90: move.w $2f2f2e2d(a2, d3.w), d1
pub fn i_c02b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B94;
    cpu.ir = 0x3232;
    let ad2 = cpu.a[2].wrapping_add(0x30).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C02B94: move.l $2e2d(a7), -(a7)
pub fn i_c02b94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B98;
    cpu.ir = 0x2F2F;
    let ad2 = cpu.a[7].wrapping_add(0x2E2D);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[7];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[7] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C02B98: move.l $2b2a(a4), d6
pub fn i_c02b98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02B9C;
    cpu.ir = 0x2C2C;
    let ad2 = cpu.a[4].wrapping_add(0x2B2A);
    let v3 = bus.read32(ad2);
    cpu.d[6] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C02B9C: move.l $2828(a1), d5
pub fn i_c02b9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02BA0;
    cpu.ir = 0x2A29;
    let ad2 = cpu.a[1].wrapping_add(0x2828);
    let v3 = bus.read32(ad2);
    cpu.d[5] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C02BA0: move.l -(a7), -(a3)
pub fn i_c02ba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02BA2;
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
/// $C02BA2: move.l -(a5), d3
pub fn i_c02ba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02BA4;
    cpu.ir = 0x2625;
    let ad2 = cpu.a[5].wrapping_sub(4); cpu.a[5] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02BA4: move.l -(a4), -(a2)
pub fn i_c02ba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02BA6;
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
/// $C02BA6: move.l -(a3), d2
pub fn i_c02ba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02BA8;
    cpu.ir = 0x2423;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02BA8: move.l -(a2), -(a1)
pub fn i_c02ba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02BAA;
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
/// $C02BAA: move.l -(a1), d1
pub fn i_c02baa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02BAC;
    cpu.ir = 0x2221;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02BAC: move.l -(a0), -(a0)
pub fn i_c02bac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02BAE;
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
/// $C02BAE: move.l -(a0), d0
pub fn i_c02bae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02BB0;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02BB0: move.b (a7)+, -(a7)
pub fn i_c02bb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02BB2;
    cpu.ir = 0x1F1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02BB2: move.b (a6)+, d7
pub fn i_c02bb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02BB4;
    cpu.ir = 0x1E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02BB4: move.b (a5)+, d7
pub fn i_c02bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02BB6;
    cpu.ir = 0x1E1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02BB6: move.b (a4)+, -(a6)
pub fn i_c02bb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02BB8;
    cpu.ir = 0x1D1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02BB8: move.b (a4)+, d6
pub fn i_c02bb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02BBA;
    cpu.ir = 0x1C1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02BBA: move.b (a3)+, -(a5)
pub fn i_c02bba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02BBC;
    cpu.ir = 0x1B1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02BBC: move.b (a2)+, -(a5)
pub fn i_c02bbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02BBE;
    cpu.ir = 0x1B1A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02BBE: move.b (a2)+, d5
pub fn i_c02bbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02BC0;
    cpu.ir = 0x1A1A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02BC0: ?
pub fn i_c02bc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02BC2;
    cpu.ir = 0x403F;
    let v1: u32 = 0x0;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x80) != 0 && (r2 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x80 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    4
}
/// $C02BC2: ?
pub fn i_c02bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02BC4;
    cpu.ir = 0x3E3D;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C02BC4: move.w #$3b3a, -(a6)
pub fn i_c02bc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02BC8;
    cpu.ir = 0x3D3C;
    let v1: u32 = 0x3B3A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $C02BC8: move.w $3737.w, -(a4)
pub fn i_c02bc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02BCC;
    cpu.ir = 0x3938;
    let v2 = bus.read16(0x3737) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[4].wrapping_sub(2); cpu.a[4] = ad3;
    bus.write16(ad3, v2 as u16);
    18
}
/// $C02BCC: move.w $33(a5, d3.w), d3
pub fn i_c02bcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02BD0;
    cpu.ir = 0x3635;
    let ad2 = cpu.a[5].wrapping_add(0x33).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C02BD0: move.w $302f2e2d(a2, d3.w), -(a1)
pub fn i_c02bd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02BD4;
    cpu.ir = 0x3332;
    let ad2 = cpu.a[2].wrapping_add(0x30).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1].wrapping_sub(2); cpu.a[1] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C02BD4: move.w $2e2d(a7), d0
pub fn i_c02bd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02BD8;
    cpu.ir = 0x302F;
    let ad2 = cpu.a[7].wrapping_add(0x2E2D);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C02BD8: move.l $2b2b(a4), -(a6)
pub fn i_c02bd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02BDC;
    cpu.ir = 0x2D2C;
    let ad2 = cpu.a[4].wrapping_add(0x2B2B);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[6];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[6] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C02BDC: move.l $2928(a1), d5
pub fn i_c02bdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02BE0;
    cpu.ir = 0x2A29;
    let ad2 = cpu.a[1].wrapping_add(0x2928);
    let v3 = bus.read32(ad2);
    cpu.d[5] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C02BE0: move.l -(a7), d4
pub fn i_c02be0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02BE2;
    cpu.ir = 0x2827;
    let ad2 = cpu.a[7].wrapping_sub(4); cpu.a[7] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[4] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02BE2: move.l -(a6), d3
pub fn i_c02be2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02BE4;
    cpu.ir = 0x2626;
    let ad2 = cpu.a[6].wrapping_sub(4); cpu.a[6] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02BE4: move.l -(a5), -(a2)
pub fn i_c02be4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02BE6;
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
/// $C02BE6: move.l -(a4), d2
pub fn i_c02be6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02BE8;
    cpu.ir = 0x2424;
    let ad2 = cpu.a[4].wrapping_sub(4); cpu.a[4] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02BE8: move.l -(a3), -(a1)
pub fn i_c02be8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02BEA;
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
/// $C02BEA: move.l -(a2), d1
pub fn i_c02bea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02BEC;
    cpu.ir = 0x2222;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02BEC: move.l -(a1), -(a0)
pub fn i_c02bec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02BEE;
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
/// $C02BEE: move.l -(a0), d0
pub fn i_c02bee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02BF0;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02BF0: move.l (a7)+, d0
pub fn i_c02bf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02BF2;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C02BF2: move.b (a6)+, -(a7)
pub fn i_c02bf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02BF4;
    cpu.ir = 0x1F1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02BF4: move.b (a6)+, d7
pub fn i_c02bf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02BF6;
    cpu.ir = 0x1E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02BF6: move.b (a5)+, -(a6)
pub fn i_c02bf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02BF8;
    cpu.ir = 0x1D1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02BF8: move.b (a4)+, d6
pub fn i_c02bf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02BFA;
    cpu.ir = 0x1C1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02BFA: move.b (a3)+, d6
pub fn i_c02bfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02BFC;
    cpu.ir = 0x1C1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02BFC: move.b (a3)+, -(a5)
pub fn i_c02bfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02BFE;
    cpu.ir = 0x1B1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02BFE: move.b (a2)+, d5
pub fn i_c02bfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C00;
    cpu.ir = 0x1A1A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02C00: ?
pub fn i_c02c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C02;
    cpu.ir = 0x403F;
    let v1: u32 = 0x0;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x80) != 0 && (r2 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x80 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    4
}
/// $C02C02: ?
pub fn i_c02c02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C04;
    cpu.ir = 0x3E3D;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C02C04: move.w #$3b3a, -(a6)
pub fn i_c02c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C08;
    cpu.ir = 0x3D3C;
    let v1: u32 = 0x3B3A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $C02C08: move.w $3837.w, -(a4)
pub fn i_c02c08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C0C;
    cpu.ir = 0x3938;
    let v2 = bus.read16(0x3837) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[4].wrapping_sub(2); cpu.a[4] = ad3;
    bus.write16(ad3, v2 as u16);
    18
}
/// $C02C0C: move.w $34(a5, d3.w), d3
pub fn i_c02c0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C10;
    cpu.ir = 0x3635;
    let ad2 = cpu.a[5].wrapping_add(0x34).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C02C10: move.w ([$302f2e2e, a2, d3.w]), -(a1)
pub fn i_c02c10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C14;
    cpu.ir = 0x3332;
    let ad2 = cpu.a[2].wrapping_add(0x31).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1].wrapping_sub(2); cpu.a[1] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C02C14: move.w $2e2e(a7), d0
pub fn i_c02c14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C18;
    cpu.ir = 0x302F;
    let ad2 = cpu.a[7].wrapping_add(0x2E2E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C02C18: move.l $2c2b(a4), -(a6)
pub fn i_c02c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C1C;
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
/// $C02C1C: move.l $2929(a2), d5
pub fn i_c02c1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C20;
    cpu.ir = 0x2A2A;
    let ad2 = cpu.a[2].wrapping_add(0x2929);
    let v3 = bus.read32(ad2);
    cpu.d[5] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C02C20: move.l -(a7), d4
pub fn i_c02c20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C22;
    cpu.ir = 0x2827;
    let ad2 = cpu.a[7].wrapping_sub(4); cpu.a[7] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[4] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02C22: move.l -(a6), -(a3)
pub fn i_c02c22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C24;
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
/// $C02C24: move.l -(a5), d3
pub fn i_c02c24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C26;
    cpu.ir = 0x2625;
    let ad2 = cpu.a[5].wrapping_sub(4); cpu.a[5] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02C26: move.l -(a4), -(a2)
pub fn i_c02c26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C28;
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
/// $C02C28: move.l -(a3), d2
pub fn i_c02c28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C2A;
    cpu.ir = 0x2423;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02C2A: move.l -(a2), -(a1)
pub fn i_c02c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C2C;
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
/// $C02C2C: move.l -(a1), d1
pub fn i_c02c2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C2E;
    cpu.ir = 0x2221;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02C2E: move.l -(a0), -(a0)
pub fn i_c02c2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C30;
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
/// $C02C30: move.l -(a0), d0
pub fn i_c02c30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C32;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02C32: move.b (a7)+, -(a7)
pub fn i_c02c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C34;
    cpu.ir = 0x1F1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02C34: move.b (a6)+, d7
pub fn i_c02c34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C36;
    cpu.ir = 0x1E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02C36: move.b (a5)+, d7
pub fn i_c02c36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C38;
    cpu.ir = 0x1E1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02C38: move.b (a5)+, -(a6)
pub fn i_c02c38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C3A;
    cpu.ir = 0x1D1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02C3A: move.b (a4)+, d6
pub fn i_c02c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C3C;
    cpu.ir = 0x1C1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02C3C: move.b (a3)+, -(a5)
pub fn i_c02c3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C3E;
    cpu.ir = 0x1B1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02C3E: move.b (a3)+, -(a5)
pub fn i_c02c3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C40;
    cpu.ir = 0x1B1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02C40: ?
pub fn i_c02c40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C42;
    cpu.ir = 0x403F;
    let v1: u32 = 0x0;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x80) != 0 && (r2 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x80 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    4
}
/// $C02C42: ?
pub fn i_c02c42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C44;
    cpu.ir = 0x3E3E;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C02C44: move.w #$3b3a, -(a6)
pub fn i_c02c44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C48;
    cpu.ir = 0x3D3C;
    let v1: u32 = 0x3B3A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $C02C48: move.w $38373635.l, -(a4)
pub fn i_c02c48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C4E;
    cpu.ir = 0x3939;
    let v2 = bus.read16(0x38373635) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[4].wrapping_sub(2); cpu.a[4] = ad3;
    bus.write16(ad3, v2 as u16);
    22
}
/// $C02C4E: move.w ([$32313030, a4, d3.w * 2], $2f2e), -(a2)
pub fn i_c02c4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C52;
    cpu.ir = 0x3534;
    let ad2 = cpu.a[4].wrapping_add(0x32).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2].wrapping_sub(2); cpu.a[2] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C02C52: move.w $30(a1, d3.w), d1
pub fn i_c02c52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C56;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x30).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C02C56: move.l $2d2d(a6), -(a7)
pub fn i_c02c56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C5A;
    cpu.ir = 0x2F2E;
    let ad2 = cpu.a[6].wrapping_add(0x2D2D);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[7];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[7] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C02C5A: move.l $2b2a(a3), d6
pub fn i_c02c5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C5E;
    cpu.ir = 0x2C2B;
    let ad2 = cpu.a[3].wrapping_add(0x2B2A);
    let v3 = bus.read32(ad2);
    cpu.d[6] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C02C5E: move.l $2828(a1), d5
pub fn i_c02c5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C62;
    cpu.ir = 0x2A29;
    let ad2 = cpu.a[1].wrapping_add(0x2828);
    let v3 = bus.read32(ad2);
    cpu.d[5] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C02C62: move.l -(a7), -(a3)
pub fn i_c02c62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C64;
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
/// $C02C64: move.l -(a6), d3
pub fn i_c02c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C66;
    cpu.ir = 0x2626;
    let ad2 = cpu.a[6].wrapping_sub(4); cpu.a[6] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02C66: move.l -(a5), -(a2)
pub fn i_c02c66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C68;
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
/// $C02C68: move.l -(a4), d2
pub fn i_c02c68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C6A;
    cpu.ir = 0x2424;
    let ad2 = cpu.a[4].wrapping_sub(4); cpu.a[4] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02C6A: move.l -(a3), -(a1)
pub fn i_c02c6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C6C;
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
/// $C02C6C: move.l -(a2), d1
pub fn i_c02c6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C6E;
    cpu.ir = 0x2222;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02C6E: move.l -(a1), -(a0)
pub fn i_c02c6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C70;
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
/// $C02C70: move.l -(a0), d0
pub fn i_c02c70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C72;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02C72: move.l (a7)+, d0
pub fn i_c02c72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C74;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C02C74: move.b (a6)+, -(a7)
pub fn i_c02c74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C76;
    cpu.ir = 0x1F1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02C76: move.b (a6)+, d7
pub fn i_c02c76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C78;
    cpu.ir = 0x1E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02C78: move.b (a5)+, -(a6)
pub fn i_c02c78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C7A;
    cpu.ir = 0x1D1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02C7A: move.b (a4)+, -(a6)
pub fn i_c02c7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C7C;
    cpu.ir = 0x1D1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02C7C: move.b (a4)+, d6
pub fn i_c02c7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C7E;
    cpu.ir = 0x1C1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02C7E: move.b (a3)+, -(a5)
pub fn i_c02c7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C80;
    cpu.ir = 0x1B1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02C80: ?
pub fn i_c02c80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C82;
    cpu.ir = 0x403F;
    let v1: u32 = 0x0;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x80) != 0 && (r2 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x80 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    4
}
/// $C02C82: ?
pub fn i_c02c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C84;
    cpu.ir = 0x3E3E;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C02C84: move.w #$3b3a, -(a6)
pub fn i_c02c84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C88;
    cpu.ir = 0x3D3C;
    let v1: u32 = 0x3B3A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $C02C88: move.w $38373636.l, d5
pub fn i_c02c88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C8E;
    cpu.ir = 0x3A39;
    let v2 = bus.read16(0x38373636) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $C02C8E: move.w ([$32313030, a4, d3.w * 2], $aaaaaaaa), -(a2)
pub fn i_c02c8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C92;
    cpu.ir = 0x3534;
    let ad2 = cpu.a[4].wrapping_add(0x33).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2].wrapping_sub(2); cpu.a[2] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C02C92: move.w $30(a1, d3.w), d1
pub fn i_c02c92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C96;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x30).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C02C96: move.l $2e2d(a6), -(a7)
pub fn i_c02c96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C9A;
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
/// $C02C9A: move.l $2b2b(a4), d6
pub fn i_c02c9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02C9E;
    cpu.ir = 0x2C2C;
    let ad2 = cpu.a[4].wrapping_add(0x2B2B);
    let v3 = bus.read32(ad2);
    cpu.d[6] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C02C9E: move.l $2928(a1), d5
pub fn i_c02c9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02CA2;
    cpu.ir = 0x2A29;
    let ad2 = cpu.a[1].wrapping_add(0x2928);
    let v3 = bus.read32(ad2);
    cpu.d[5] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C02CA2: move.l -(a7), d4
pub fn i_c02ca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02CA4;
    cpu.ir = 0x2827;
    let ad2 = cpu.a[7].wrapping_sub(4); cpu.a[7] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[4] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02CA4: move.l -(a6), -(a3)
pub fn i_c02ca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02CA6;
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
/// $C02CA6: move.l -(a5), d3
pub fn i_c02ca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02CA8;
    cpu.ir = 0x2625;
    let ad2 = cpu.a[5].wrapping_sub(4); cpu.a[5] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02CA8: move.l -(a4), -(a2)
pub fn i_c02ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02CAA;
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
/// $C02CAA: move.l -(a3), d2
pub fn i_c02caa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02CAC;
    cpu.ir = 0x2423;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02CAC: move.l -(a2), -(a1)
pub fn i_c02cac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02CAE;
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
/// $C02CAE: move.l -(a1), d1
pub fn i_c02cae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02CB0;
    cpu.ir = 0x2221;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02CB0: move.l -(a0), -(a0)
pub fn i_c02cb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02CB2;
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
/// $C02CB2: move.l -(a0), d0
pub fn i_c02cb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02CB4;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02CB4: move.b (a7)+, -(a7)
pub fn i_c02cb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02CB6;
    cpu.ir = 0x1F1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02CB6: move.b (a6)+, d7
pub fn i_c02cb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02CB8;
    cpu.ir = 0x1E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02CB8: move.b (a5)+, d7
pub fn i_c02cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02CBA;
    cpu.ir = 0x1E1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02CBA: move.b (a5)+, -(a6)
pub fn i_c02cba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02CBC;
    cpu.ir = 0x1D1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02CBC: move.b (a4)+, d6
pub fn i_c02cbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02CBE;
    cpu.ir = 0x1C1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02CBE: move.b (a3)+, d6
pub fn i_c02cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02CC0;
    cpu.ir = 0x1C1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02CC0: ?
pub fn i_c02cc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02CC2;
    cpu.ir = 0x403F;
    let v1: u32 = 0x0;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x80) != 0 && (r2 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x80 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    4
}
/// $C02CC2: ?
pub fn i_c02cc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02CC4;
    cpu.ir = 0x3E3E;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C02CC4: move.w #$3b3a, -(a6)
pub fn i_c02cc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02CC8;
    cpu.ir = 0x3D3C;
    let v1: u32 = 0x3B3A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $C02CC8: move.w $38373736.l, d5
pub fn i_c02cc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02CCE;
    cpu.ir = 0x3A39;
    let v2 = bus.read16(0x38373736) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $C02CCE: move.w $33(a4, d3.w), -(a2)
pub fn i_c02cce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02CD2;
    cpu.ir = 0x3534;
    let ad2 = cpu.a[4].wrapping_add(0x33).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2].wrapping_sub(2); cpu.a[2] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C02CD2: move.w $2f2f2e2d(a1, d3.w), d1
pub fn i_c02cd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02CD6;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x30).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C02CD6: move.l $2e2d(a7), -(a7)
pub fn i_c02cd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02CDA;
    cpu.ir = 0x2F2F;
    let ad2 = cpu.a[7].wrapping_add(0x2E2D);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[7];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[7] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C02CDA: move.l $2c2b(a4), -(a6)
pub fn i_c02cda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02CDE;
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
/// $C02CDE: move.l $2929(a2), d5
pub fn i_c02cde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02CE2;
    cpu.ir = 0x2A2A;
    let ad2 = cpu.a[2].wrapping_add(0x2929);
    let v3 = bus.read32(ad2);
    cpu.d[5] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C02CE2: move.l -(a7), d4
pub fn i_c02ce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02CE4;
    cpu.ir = 0x2827;
    let ad2 = cpu.a[7].wrapping_sub(4); cpu.a[7] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[4] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02CE4: move.l -(a6), -(a3)
pub fn i_c02ce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02CE6;
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
/// $C02CE6: move.l -(a5), d3
pub fn i_c02ce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02CE8;
    cpu.ir = 0x2625;
    let ad2 = cpu.a[5].wrapping_sub(4); cpu.a[5] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02CE8: move.l -(a4), -(a2)
pub fn i_c02ce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02CEA;
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
/// $C02CEA: move.l -(a3), d2
pub fn i_c02cea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02CEC;
    cpu.ir = 0x2423;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02CEC: move.l -(a3), -(a1)
pub fn i_c02cec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02CEE;
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
/// $C02CEE: move.l -(a2), d1
pub fn i_c02cee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02CF0;
    cpu.ir = 0x2222;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02CF0: move.l -(a1), -(a0)
pub fn i_c02cf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02CF2;
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
/// $C02CF2: move.l -(a0), d0
pub fn i_c02cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02CF4;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02CF4: move.l (a7)+, d0
pub fn i_c02cf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02CF6;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C02CF6: move.b (a6)+, -(a7)
pub fn i_c02cf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02CF8;
    cpu.ir = 0x1F1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02CF8: move.b (a6)+, d7
pub fn i_c02cf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02CFA;
    cpu.ir = 0x1E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02CFA: move.b (a5)+, -(a6)
pub fn i_c02cfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02CFC;
    cpu.ir = 0x1D1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02CFC: move.b (a4)+, -(a6)
pub fn i_c02cfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02CFE;
    cpu.ir = 0x1D1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02CFE: move.b (a4)+, d6
pub fn i_c02cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D00;
    cpu.ir = 0x1C1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02D00: ?
pub fn i_c02d00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D02;
    cpu.ir = 0x403F;
    let v1: u32 = 0x0;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x80) != 0 && (r2 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x80 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    4
}
/// $C02D02: ?
pub fn i_c02d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D04;
    cpu.ir = 0x3E3E;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C02D04: move.w #$3b3b, -(a6)
pub fn i_c02d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D08;
    cpu.ir = 0x3D3C;
    let v1: u32 = 0x3B3B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $C02D08: move.w $38383736.l, d5
pub fn i_c02d08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D0E;
    cpu.ir = 0x3A39;
    let v2 = bus.read16(0x38383736) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $C02D0E: move.w $33(a5, d3.w), -(a2)
pub fn i_c02d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D12;
    cpu.ir = 0x3535;
    let ad2 = cpu.a[5].wrapping_add(0x33).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2].wrapping_sub(2); cpu.a[2] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C02D12: move.w $302f2e2e(a2, d3.w), d1
pub fn i_c02d12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D16;
    cpu.ir = 0x3232;
    let ad2 = cpu.a[2].wrapping_add(0x30).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C02D16: move.w $2e2e(a7), d0
pub fn i_c02d16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D1A;
    cpu.ir = 0x302F;
    let ad2 = cpu.a[7].wrapping_add(0x2E2E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C02D1A: move.l $2c2b(a4), -(a6)
pub fn i_c02d1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D1E;
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
/// $C02D1E: move.l $2a29(a2), -(a5)
pub fn i_c02d1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D22;
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
/// $C02D22: move.l $2727(a0), d4
pub fn i_c02d22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D26;
    cpu.ir = 0x2828;
    let ad2 = cpu.a[0].wrapping_add(0x2727);
    let v3 = bus.read32(ad2);
    cpu.d[4] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C02D26: move.l -(a6), d3
pub fn i_c02d26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D28;
    cpu.ir = 0x2626;
    let ad2 = cpu.a[6].wrapping_sub(4); cpu.a[6] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02D28: move.l -(a5), -(a2)
pub fn i_c02d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D2A;
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
/// $C02D2A: move.l -(a4), d2
pub fn i_c02d2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D2C;
    cpu.ir = 0x2424;
    let ad2 = cpu.a[4].wrapping_sub(4); cpu.a[4] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02D2C: move.l -(a3), -(a1)
pub fn i_c02d2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D2E;
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
/// $C02D2E: move.l -(a2), d1
pub fn i_c02d2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D30;
    cpu.ir = 0x2222;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02D30: move.l -(a1), d1
pub fn i_c02d30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D32;
    cpu.ir = 0x2221;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02D32: move.l -(a0), -(a0)
pub fn i_c02d32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D34;
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
/// $C02D34: move.l -(a0), d0
pub fn i_c02d34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D36;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02D36: move.b (a7)+, -(a7)
pub fn i_c02d36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D38;
    cpu.ir = 0x1F1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02D38: move.b (a6)+, d7
pub fn i_c02d38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D3A;
    cpu.ir = 0x1E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02D3A: move.b (a5)+, d7
pub fn i_c02d3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D3C;
    cpu.ir = 0x1E1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02D3C: move.b (a5)+, -(a6)
pub fn i_c02d3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D3E;
    cpu.ir = 0x1D1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02D3E: move.b (a4)+, d6
pub fn i_c02d3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D40;
    cpu.ir = 0x1C1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02D40: ?
pub fn i_c02d40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D42;
    cpu.ir = 0x403F;
    let v1: u32 = 0x0;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x80) != 0 && (r2 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x80 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    4
}
/// $C02D42: ?
pub fn i_c02d42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D44;
    cpu.ir = 0x3E3E;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C02D44: move.w #$3b3b, -(a6)
pub fn i_c02d44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D48;
    cpu.ir = 0x3D3C;
    let v1: u32 = 0x3B3B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $C02D48: move.w $38383736.l, d5
pub fn i_c02d48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D4E;
    cpu.ir = 0x3A39;
    let v2 = bus.read16(0x38383736) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $C02D4E: move.w $33(a5, d3.w), -(a2)
pub fn i_c02d4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D52;
    cpu.ir = 0x3535;
    let ad2 = cpu.a[5].wrapping_add(0x33).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2].wrapping_sub(2); cpu.a[2] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C02D52: move.w ([$302f2f2e, a2, d3.w]), -(a1)
pub fn i_c02d52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D56;
    cpu.ir = 0x3332;
    let ad2 = cpu.a[2].wrapping_add(0x31).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1].wrapping_sub(2); cpu.a[1] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C02D56: move.w $2f2e(a7), d0
pub fn i_c02d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D5A;
    cpu.ir = 0x302F;
    let ad2 = cpu.a[7].wrapping_add(0x2F2E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C02D5A: move.l $2c2c(a5), -(a6)
pub fn i_c02d5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D5E;
    cpu.ir = 0x2D2D;
    let ad2 = cpu.a[5].wrapping_add(0x2C2C);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[6];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[6] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C02D5E: move.l $2a29(a2), -(a5)
pub fn i_c02d5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D62;
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
/// $C02D62: move.l $2827(a0), -(a4)
pub fn i_c02d62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D66;
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
/// $C02D66: move.l -(a6), -(a3)
pub fn i_c02d66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D68;
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
/// $C02D68: move.l -(a5), d3
pub fn i_c02d68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D6A;
    cpu.ir = 0x2625;
    let ad2 = cpu.a[5].wrapping_sub(4); cpu.a[5] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02D6A: move.l -(a4), -(a2)
pub fn i_c02d6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D6C;
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
/// $C02D6C: move.l -(a3), d2
pub fn i_c02d6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D6E;
    cpu.ir = 0x2423;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02D6E: move.l -(a2), -(a1)
pub fn i_c02d6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D70;
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
/// $C02D70: move.l -(a2), d1
pub fn i_c02d70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D72;
    cpu.ir = 0x2222;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02D72: move.l -(a1), -(a0)
pub fn i_c02d72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D74;
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
/// $C02D74: move.l -(a0), d0
pub fn i_c02d74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D76;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02D76: move.l (a7)+, d0
pub fn i_c02d76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D78;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C02D78: move.b (a7)+, -(a7)
pub fn i_c02d78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D7A;
    cpu.ir = 0x1F1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02D7A: move.b (a6)+, d7
pub fn i_c02d7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D7C;
    cpu.ir = 0x1E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02D7C: move.b (a5)+, -(a6)
pub fn i_c02d7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D7E;
    cpu.ir = 0x1D1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02D7E: move.b (a4)+, -(a6)
pub fn i_c02d7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D80;
    cpu.ir = 0x1D1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02D80: ?
pub fn i_c02d80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D82;
    cpu.ir = 0x403F;
    let v1: u32 = 0x0;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x80) != 0 && (r2 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x80 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    4
}
/// $C02D82: ?
pub fn i_c02d82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D84;
    cpu.ir = 0x3E3E;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C02D84: move.w #$3b3b, -(a6)
pub fn i_c02d84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D88;
    cpu.ir = 0x3D3C;
    let v1: u32 = 0x3B3B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $C02D88: move.w $39383736.l, d5
pub fn i_c02d88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D8E;
    cpu.ir = 0x3A39;
    let v2 = bus.read16(0x39383736) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $C02D8E: move.w $34(a5, d3.w), d3
pub fn i_c02d8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D92;
    cpu.ir = 0x3635;
    let ad2 = cpu.a[5].wrapping_add(0x34).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C02D92: move.w $31(a2, d3.w), -(a1)
pub fn i_c02d92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D96;
    cpu.ir = 0x3332;
    let ad2 = cpu.a[2].wrapping_add(0x31).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1].wrapping_sub(2); cpu.a[1] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C02D96: move.w ([$2e2d, a0], d2.l * 8, $2d2c), d0
pub fn i_c02d96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D9A;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x2E).wrapping_add(cpu.d[2]);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C02D9A: move.l $2d2c(a5), d7
pub fn i_c02d9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02D9E;
    cpu.ir = 0x2E2D;
    let ad2 = cpu.a[5].wrapping_add(0x2D2C);
    let v3 = bus.read32(ad2);
    cpu.d[7] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C02D9E: move.l $2a2a(a3), -(a5)
pub fn i_c02d9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02DA2;
    cpu.ir = 0x2B2B;
    let ad2 = cpu.a[3].wrapping_add(0x2A2A);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[5];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[5] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C02DA2: move.l $2828(a1), -(a4)
pub fn i_c02da2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02DA6;
    cpu.ir = 0x2929;
    let ad2 = cpu.a[1].wrapping_add(0x2828);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[4];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[4] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C02DA6: move.l -(a7), -(a3)
pub fn i_c02da6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02DA8;
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
/// $C02DA8: move.l -(a6), d3
pub fn i_c02da8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02DAA;
    cpu.ir = 0x2626;
    let ad2 = cpu.a[6].wrapping_sub(4); cpu.a[6] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02DAA: move.l -(a5), -(a2)
pub fn i_c02daa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02DAC;
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
/// $C02DAC: move.l -(a4), d2
pub fn i_c02dac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02DAE;
    cpu.ir = 0x2424;
    let ad2 = cpu.a[4].wrapping_sub(4); cpu.a[4] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02DAE: move.l -(a3), -(a1)
pub fn i_c02dae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02DB0;
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
/// $C02DB0: move.l -(a2), d1
pub fn i_c02db0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02DB2;
    cpu.ir = 0x2222;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02DB2: move.l -(a1), d1
pub fn i_c02db2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02DB4;
    cpu.ir = 0x2221;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02DB4: move.l -(a0), -(a0)
pub fn i_c02db4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02DB6;
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
/// $C02DB6: move.l -(a0), d0
pub fn i_c02db6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02DB8;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02DB8: move.b (a7)+, -(a7)
pub fn i_c02db8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02DBA;
    cpu.ir = 0x1F1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02DBA: move.b (a6)+, -(a7)
pub fn i_c02dba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02DBC;
    cpu.ir = 0x1F1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02DBC: move.b (a6)+, d7
pub fn i_c02dbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02DBE;
    cpu.ir = 0x1E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02DBE: move.b (a5)+, -(a6)
pub fn i_c02dbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02DC0;
    cpu.ir = 0x1D1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02DC0: ?
pub fn i_c02dc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02DC2;
    cpu.ir = 0x403F;
    let v1: u32 = 0x0;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x80) != 0 && (r2 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x80 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    4
}
/// $C02DC2: ?
pub fn i_c02dc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02DC4;
    cpu.ir = 0x3F3E;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $C02DC4: move.w #$3c3b, -(a6)
pub fn i_c02dc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02DC8;
    cpu.ir = 0x3D3C;
    let v1: u32 = 0x3C3B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $C02DC8: move.w $39383737.l, d5
pub fn i_c02dc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02DCE;
    cpu.ir = 0x3A39;
    let v2 = bus.read16(0x39383737) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $C02DCE: move.w $34(a5, d3.w), d3
pub fn i_c02dce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02DD2;
    cpu.ir = 0x3635;
    let ad2 = cpu.a[5].wrapping_add(0x34).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C02DD2: move.w $31(a2, d3.w), -(a1)
pub fn i_c02dd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02DD6;
    cpu.ir = 0x3332;
    let ad2 = cpu.a[2].wrapping_add(0x31).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1].wrapping_sub(2); cpu.a[1] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C02DD6: move.w ([$2e2d, a0], d2.l * 8, $2d2c2c2b), d0
pub fn i_c02dd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02DDA;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x2F).wrapping_add(cpu.d[2]);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C02DDA: move.l $2d2c(a5), d7
pub fn i_c02dda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02DDE;
    cpu.ir = 0x2E2D;
    let ad2 = cpu.a[5].wrapping_add(0x2D2C);
    let v3 = bus.read32(ad2);
    cpu.d[7] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C02DDE: move.l $2b2a(a3), d6
pub fn i_c02dde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02DE2;
    cpu.ir = 0x2C2B;
    let ad2 = cpu.a[3].wrapping_add(0x2B2A);
    let v3 = bus.read32(ad2);
    cpu.d[6] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C02DE2: move.l $2828(a1), -(a4)
pub fn i_c02de2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02DE6;
    cpu.ir = 0x2929;
    let ad2 = cpu.a[1].wrapping_add(0x2828);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[4];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[4] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C02DE6: move.l -(a7), -(a3)
pub fn i_c02de6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02DE8;
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
/// $C02DE8: move.l -(a6), d3
pub fn i_c02de8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02DEA;
    cpu.ir = 0x2626;
    let ad2 = cpu.a[6].wrapping_sub(4); cpu.a[6] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02DEA: move.l -(a5), -(a2)
pub fn i_c02dea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02DEC;
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
/// $C02DEC: move.l -(a4), -(a2)
pub fn i_c02dec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02DEE;
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
/// $C02DEE: move.l -(a3), d2
pub fn i_c02dee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02DF0;
    cpu.ir = 0x2423;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02DF0: move.l -(a2), -(a1)
pub fn i_c02df0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02DF2;
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
/// $C02DF2: move.l -(a2), d1
pub fn i_c02df2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02DF4;
    cpu.ir = 0x2222;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02DF4: move.l -(a1), -(a0)
pub fn i_c02df4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02DF6;
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
/// $C02DF6: move.l -(a0), d0
pub fn i_c02df6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02DF8;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02DF8: move.l (a7)+, d0
pub fn i_c02df8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02DFA;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C02DFA: move.b (a7)+, -(a7)
pub fn i_c02dfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02DFC;
    cpu.ir = 0x1F1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02DFC: move.b (a6)+, d7
pub fn i_c02dfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02DFE;
    cpu.ir = 0x1E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02DFE: move.b (a5)+, d7
pub fn i_c02dfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E00;
    cpu.ir = 0x1E1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02E00: ?
pub fn i_c02e00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E02;
    cpu.ir = 0x403F;
    let v1: u32 = 0x0;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x80) != 0 && (r2 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x80 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    4
}
/// $C02E02: ?
pub fn i_c02e02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E04;
    cpu.ir = 0x3F3E;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $C02E04: move.w #$3c3b, -(a6)
pub fn i_c02e04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E08;
    cpu.ir = 0x3D3C;
    let v1: u32 = 0x3C3B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $C02E08: move.w $c06742(pc), d5
pub fn i_c02e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E0C;
    cpu.ir = 0x3A3A;
    let v2 = bus.read16(0xC06742) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v2 & 0xFFFF);
    12
}
/// $C02E0C: move.w $35(a7, d3.w), -(a3)
pub fn i_c02e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E10;
    cpu.ir = 0x3737;
    let ad2 = cpu.a[7].wrapping_add(0x35).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[3].wrapping_sub(2); cpu.a[3] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C02E10: move.w ([$32313130, a4, d3.w * 2], $aaaaaaaa), -(a2)
pub fn i_c02e10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E14;
    cpu.ir = 0x3534;
    let ad2 = cpu.a[4].wrapping_add(0x33).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2].wrapping_sub(2); cpu.a[2] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C02E14: move.w $302f2e2e(a1, d3.w), d1
pub fn i_c02e14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E18;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x30).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C02E18: move.w $2e2e(a7), d0
pub fn i_c02e18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E1C;
    cpu.ir = 0x302F;
    let ad2 = cpu.a[7].wrapping_add(0x2E2E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C02E1C: move.l $2c2b(a5), -(a6)
pub fn i_c02e1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E20;
    cpu.ir = 0x2D2D;
    let ad2 = cpu.a[5].wrapping_add(0x2C2B);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[6];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[6] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C02E20: move.l $2a29(a2), -(a5)
pub fn i_c02e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E24;
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
/// $C02E24: move.l $2827(a0), -(a4)
pub fn i_c02e24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E28;
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
/// $C02E28: move.l -(a6), -(a3)
pub fn i_c02e28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E2A;
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
/// $C02E2A: move.l -(a5), d3
pub fn i_c02e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E2C;
    cpu.ir = 0x2625;
    let ad2 = cpu.a[5].wrapping_sub(4); cpu.a[5] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02E2C: move.l -(a4), -(a2)
pub fn i_c02e2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E2E;
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
/// $C02E2E: move.l -(a4), d2
pub fn i_c02e2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E30;
    cpu.ir = 0x2424;
    let ad2 = cpu.a[4].wrapping_sub(4); cpu.a[4] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02E30: move.l -(a3), -(a1)
pub fn i_c02e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E32;
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
/// $C02E32: move.l -(a2), d1
pub fn i_c02e32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E34;
    cpu.ir = 0x2222;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02E34: move.l -(a1), d1
pub fn i_c02e34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E36;
    cpu.ir = 0x2221;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02E36: move.l -(a0), -(a0)
pub fn i_c02e36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E38;
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
/// $C02E38: move.l -(a0), d0
pub fn i_c02e38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E3A;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02E3A: move.b (a7)+, -(a7)
pub fn i_c02e3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E3C;
    cpu.ir = 0x1F1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02E3C: move.b (a6)+, -(a7)
pub fn i_c02e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E3E;
    cpu.ir = 0x1F1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02E3E: move.b (a6)+, d7
pub fn i_c02e3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E40;
    cpu.ir = 0x1E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02E40: ?
pub fn i_c02e40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E42;
    cpu.ir = 0x403F;
    let v1: u32 = 0x0;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x80) != 0 && (r2 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x80 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    4
}
/// $C02E42: ?
pub fn i_c02e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E44;
    cpu.ir = 0x3F3E;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $C02E44: move.w #$3c3b, -(a6)
pub fn i_c02e44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E48;
    cpu.ir = 0x3D3C;
    let v1: u32 = 0x3C3B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $C02E48: move.w $c06782(pc), d5
pub fn i_c02e48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E4C;
    cpu.ir = 0x3A3A;
    let v2 = bus.read16(0xC06782) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v2 & 0xFFFF);
    12
}
/// $C02E4C: move.w $36(a7, d3.w), d4
pub fn i_c02e4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E50;
    cpu.ir = 0x3837;
    let ad2 = cpu.a[7].wrapping_add(0x36).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C02E50: move.w $33(a4, d3.w), -(a2)
pub fn i_c02e50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E54;
    cpu.ir = 0x3534;
    let ad2 = cpu.a[4].wrapping_add(0x33).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2].wrapping_sub(2); cpu.a[2] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C02E54: move.w $302f2f2e(a2, d3.w), d1
pub fn i_c02e54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E58;
    cpu.ir = 0x3232;
    let ad2 = cpu.a[2].wrapping_add(0x30).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C02E58: move.w $2f2e(a7), d0
pub fn i_c02e58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E5C;
    cpu.ir = 0x302F;
    let ad2 = cpu.a[7].wrapping_add(0x2F2E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C02E5C: move.l $2c2c(a5), -(a6)
pub fn i_c02e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E60;
    cpu.ir = 0x2D2D;
    let ad2 = cpu.a[5].wrapping_add(0x2C2C);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[6];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[6] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C02E60: move.l $2a2a(a3), -(a5)
pub fn i_c02e60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E64;
    cpu.ir = 0x2B2B;
    let ad2 = cpu.a[3].wrapping_add(0x2A2A);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[5];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[5] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C02E64: move.l $2828(a1), -(a4)
pub fn i_c02e64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E68;
    cpu.ir = 0x2929;
    let ad2 = cpu.a[1].wrapping_add(0x2828);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[4];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[4] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C02E68: move.l -(a7), -(a3)
pub fn i_c02e68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E6A;
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
/// $C02E6A: move.l -(a6), d3
pub fn i_c02e6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E6C;
    cpu.ir = 0x2626;
    let ad2 = cpu.a[6].wrapping_sub(4); cpu.a[6] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02E6C: move.l -(a5), -(a2)
pub fn i_c02e6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E6E;
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
/// $C02E6E: move.l -(a4), d2
pub fn i_c02e6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E70;
    cpu.ir = 0x2424;
    let ad2 = cpu.a[4].wrapping_sub(4); cpu.a[4] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02E70: move.l -(a3), -(a1)
pub fn i_c02e70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E72;
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
/// $C02E72: move.l -(a2), -(a1)
pub fn i_c02e72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E74;
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
/// $C02E74: move.l -(a1), d1
pub fn i_c02e74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E76;
    cpu.ir = 0x2221;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02E76: move.l -(a1), -(a0)
pub fn i_c02e76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E78;
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
/// $C02E78: move.l -(a0), d0
pub fn i_c02e78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E7A;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02E7A: move.l (a7)+, d0
pub fn i_c02e7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E7C;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C02E7C: move.b (a7)+, -(a7)
pub fn i_c02e7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E7E;
    cpu.ir = 0x1F1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02E7E: move.b (a6)+, d7
pub fn i_c02e7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E80;
    cpu.ir = 0x1E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C02E80: ?
pub fn i_c02e80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E82;
    cpu.ir = 0x403F;
    let v1: u32 = 0x0;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x80) != 0 && (r2 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x80 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    4
}
/// $C02E82: ?
pub fn i_c02e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E84;
    cpu.ir = 0x3F3E;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $C02E84: move.w #$3c3b, -(a6)
pub fn i_c02e84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E88;
    cpu.ir = 0x3D3C;
    let v1: u32 = 0x3C3B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $C02E88: move.w $c067c2(pc), d5
pub fn i_c02e88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E8C;
    cpu.ir = 0x3A3A;
    let v2 = bus.read16(0xC067C2) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v2 & 0xFFFF);
    12
}
/// $C02E8C: move.w $36(a7, d3.w), d4
pub fn i_c02e8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E90;
    cpu.ir = 0x3837;
    let ad2 = cpu.a[7].wrapping_add(0x36).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C02E90: move.w $33(a4, d3.w), -(a2)
pub fn i_c02e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E94;
    cpu.ir = 0x3534;
    let ad2 = cpu.a[4].wrapping_add(0x33).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2].wrapping_sub(2); cpu.a[2] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C02E94: move.w ([$302f2f2e, a2, d3.w]), d1
pub fn i_c02e94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E98;
    cpu.ir = 0x3232;
    let ad2 = cpu.a[2].wrapping_add(0x31).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C02E98: move.w $2f2e(a7), d0
pub fn i_c02e98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02E9C;
    cpu.ir = 0x302F;
    let ad2 = cpu.a[7].wrapping_add(0x2F2E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C02E9C: move.l $2d2c(a5), d7
pub fn i_c02e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02EA0;
    cpu.ir = 0x2E2D;
    let ad2 = cpu.a[5].wrapping_add(0x2D2C);
    let v3 = bus.read32(ad2);
    cpu.d[7] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C02EA0: move.l $2a2a(a3), -(a5)
pub fn i_c02ea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02EA4;
    cpu.ir = 0x2B2B;
    let ad2 = cpu.a[3].wrapping_add(0x2A2A);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[5];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[5] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C02EA4: move.l $2828(a1), -(a4)
pub fn i_c02ea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02EA8;
    cpu.ir = 0x2929;
    let ad2 = cpu.a[1].wrapping_add(0x2828);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[4];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[4] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C02EA8: move.l -(a7), -(a3)
pub fn i_c02ea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02EAA;
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
/// $C02EAA: move.l -(a6), d3
pub fn i_c02eaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02EAC;
    cpu.ir = 0x2626;
    let ad2 = cpu.a[6].wrapping_sub(4); cpu.a[6] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02EAC: move.l -(a5), d3
pub fn i_c02eac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02EAE;
    cpu.ir = 0x2625;
    let ad2 = cpu.a[5].wrapping_sub(4); cpu.a[5] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02EAE: move.l -(a4), -(a2)
pub fn i_c02eae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02EB0;
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
/// $C02EB0: move.l -(a3), d2
pub fn i_c02eb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02EB2;
    cpu.ir = 0x2423;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02EB2: move.l -(a3), -(a1)
pub fn i_c02eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02EB4;
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
/// $C02EB4: move.l -(a2), d1
pub fn i_c02eb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02EB6;
    cpu.ir = 0x2222;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02EB6: move.l -(a1), -(a0)
pub fn i_c02eb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02EB8;
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
/// $C02EB8: move.l -(a0), -(a0)
pub fn i_c02eb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02EBA;
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
/// $C02EBA: move.l -(a0), d0
pub fn i_c02eba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02EBC;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02EBC: move.b (a7)+, -(a7)
pub fn i_c02ebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02EBE;
    cpu.ir = 0x1F1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02EBE: move.b (a6)+, -(a7)
pub fn i_c02ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02EC0;
    cpu.ir = 0x1F1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02EC0: ?
pub fn i_c02ec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02EC2;
    cpu.ir = 0x403F;
    let v1: u32 = 0x0;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x80) != 0 && (r2 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x80 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    4
}
/// $C02EC2: ?
pub fn i_c02ec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02EC4;
    cpu.ir = 0x3F3E;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $C02EC4: ?
pub fn i_c02ec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02EC6;
    cpu.ir = 0x3D3D;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $C02EC6: move.w ([$39f866ff, pc, d3.l * 2], $3736), d6
pub fn i_c02ec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02ECA;
    cpu.ir = 0x3C3B;
    let ad2 = 0xC02F02_u32.wrapping_add(cpu.d[3]);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C02ECA: move.w $3837.w, -(a4)
pub fn i_c02eca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02ECE;
    cpu.ir = 0x3938;
    let v2 = bus.read16(0x3837) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[4].wrapping_sub(2); cpu.a[4] = ad3;
    bus.write16(ad3, v2 as u16);
    18
}
/// $C02ECE: move.w ([$34333332, a6], d3.w * 4), -(a3)
pub fn i_c02ece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02ED2;
    cpu.ir = 0x3736;
    let ad2 = cpu.a[6].wrapping_add(0x35).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[3].wrapping_sub(2); cpu.a[3] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C02ED2: move.w ([$31313030, a3, d3.w * 2], $2f2f), d2
pub fn i_c02ed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02ED6;
    cpu.ir = 0x3433;
    let ad2 = cpu.a[3].wrapping_add(0x32).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C02ED6: move.w $30(a1, d3.w), -(a0)
pub fn i_c02ed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02EDA;
    cpu.ir = 0x3131;
    let ad2 = cpu.a[1].wrapping_add(0x30).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_sub(2); cpu.a[0] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C02EDA: move.l $2e2d(a7), -(a7)
pub fn i_c02eda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02EDE;
    cpu.ir = 0x2F2F;
    let ad2 = cpu.a[7].wrapping_add(0x2E2D);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[7];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[7] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C02EDE: move.l $2c2b(a4), -(a6)
pub fn i_c02ede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02EE2;
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
/// $C02EE2: move.l $2a29(a2), -(a5)
pub fn i_c02ee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02EE6;
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
/// $C02EE6: move.l $2827(a0), -(a4)
pub fn i_c02ee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02EEA;
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
/// $C02EEA: move.l -(a6), -(a3)
pub fn i_c02eea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02EEC;
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
/// $C02EEC: move.l -(a5), d3
pub fn i_c02eec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02EEE;
    cpu.ir = 0x2625;
    let ad2 = cpu.a[5].wrapping_sub(4); cpu.a[5] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02EEE: move.l -(a5), -(a2)
pub fn i_c02eee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02EF0;
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
/// $C02EF0: move.l -(a4), d2
pub fn i_c02ef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02EF2;
    cpu.ir = 0x2424;
    let ad2 = cpu.a[4].wrapping_sub(4); cpu.a[4] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02EF2: move.l -(a3), -(a1)
pub fn i_c02ef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02EF4;
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
/// $C02EF4: move.l -(a2), -(a1)
pub fn i_c02ef4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02EF6;
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
/// $C02EF6: move.l -(a1), d1
pub fn i_c02ef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02EF8;
    cpu.ir = 0x2221;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02EF8: move.l -(a1), -(a0)
pub fn i_c02ef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02EFA;
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
/// $C02EFA: move.l -(a0), d0
pub fn i_c02efa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02EFC;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02EFC: move.l (a7)+, d0
pub fn i_c02efc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02EFE;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C02EFE: move.b (a7)+, -(a7)
pub fn i_c02efe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02F00;
    cpu.ir = 0x1F1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02F00: ?
pub fn i_c02f00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02F02;
    cpu.ir = 0x403F;
    let v1: u32 = 0x0;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x80) != 0 && (r2 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x80 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    4
}
/// $C02F02: ?
pub fn i_c02f02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02F04;
    cpu.ir = 0x3F3E;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $C02F04: ?
pub fn i_c02f04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02F06;
    cpu.ir = 0x3D3D;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $C02F06: move.w ([$39f9673f, pc, d3.l * 2], $3736), d6
pub fn i_c02f06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02F0A;
    cpu.ir = 0x3C3B;
    let ad2 = 0xC02F42_u32.wrapping_add(cpu.d[3]);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C02F0A: move.w $38373736.l, -(a4)
pub fn i_c02f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02F10;
    cpu.ir = 0x3939;
    let v2 = bus.read16(0x38373736) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[4].wrapping_sub(2); cpu.a[4] = ad3;
    bus.write16(ad3, v2 as u16);
    22
}
/// $C02F10: move.w $34(a5, d3.w), -(a2)
pub fn i_c02f10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02F14;
    cpu.ir = 0x3535;
    let ad2 = cpu.a[5].wrapping_add(0x34).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2].wrapping_sub(2); cpu.a[2] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C02F14: move.w $31(a2, d3.w), -(a1)
pub fn i_c02f14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02F18;
    cpu.ir = 0x3332;
    let ad2 = cpu.a[2].wrapping_add(0x31).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1].wrapping_sub(2); cpu.a[1] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C02F18: move.w ([$2e2e, a0], d2.l * 8, $2d2d2c2c), d0
pub fn i_c02f18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02F1C;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x2F).wrapping_add(cpu.d[2]);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C02F1C: move.l $2d2d(a6), d7
pub fn i_c02f1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02F20;
    cpu.ir = 0x2E2E;
    let ad2 = cpu.a[6].wrapping_add(0x2D2D);
    let v3 = bus.read32(ad2);
    cpu.d[7] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C02F20: move.l $2b2a(a4), d6
pub fn i_c02f20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02F24;
    cpu.ir = 0x2C2C;
    let ad2 = cpu.a[4].wrapping_add(0x2B2A);
    let v3 = bus.read32(ad2);
    cpu.d[6] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C02F24: move.l $2929(a1), d5
pub fn i_c02f24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02F28;
    cpu.ir = 0x2A29;
    let ad2 = cpu.a[1].wrapping_add(0x2929);
    let v3 = bus.read32(ad2);
    cpu.d[5] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C02F28: move.l $2727(a0), d4
pub fn i_c02f28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02F2C;
    cpu.ir = 0x2828;
    let ad2 = cpu.a[0].wrapping_add(0x2727);
    let v3 = bus.read32(ad2);
    cpu.d[4] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C02F2C: move.l -(a6), d3
pub fn i_c02f2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02F2E;
    cpu.ir = 0x2626;
    let ad2 = cpu.a[6].wrapping_sub(4); cpu.a[6] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02F2E: move.l -(a5), -(a2)
pub fn i_c02f2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02F30;
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
/// $C02F30: move.l -(a4), -(a2)
pub fn i_c02f30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02F32;
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
/// $C02F32: move.l -(a3), d2
pub fn i_c02f32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02F34;
    cpu.ir = 0x2423;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02F34: move.l -(a3), -(a1)
pub fn i_c02f34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02F36;
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
/// $C02F36: move.l -(a2), d1
pub fn i_c02f36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02F38;
    cpu.ir = 0x2222;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02F38: move.l -(a1), -(a0)
pub fn i_c02f38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02F3A;
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
/// $C02F3A: move.l -(a0), -(a0)
pub fn i_c02f3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02F3C;
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
/// $C02F3C: move.l -(a0), d0
pub fn i_c02f3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02F3E;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02F3E: move.b (a7)+, -(a7)
pub fn i_c02f3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02F40;
    cpu.ir = 0x1F1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C02F40: ?
pub fn i_c02f40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02F42;
    cpu.ir = 0x403F;
    let v1: u32 = 0x0;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x80) != 0 && (r2 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x80 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    4
}
/// $C02F42: ?
pub fn i_c02f42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02F44;
    cpu.ir = 0x3F3E;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $C02F44: ?
pub fn i_c02f44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02F46;
    cpu.ir = 0x3D3D;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $C02F46: move.w ([$39f9677f, pc, d3.l * 2], $3736), d6
pub fn i_c02f46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02F4A;
    cpu.ir = 0x3C3B;
    let ad2 = 0xC02F82_u32.wrapping_add(cpu.d[3]);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C02F4A: move.w $38373736.l, -(a4)
pub fn i_c02f4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02F50;
    cpu.ir = 0x3939;
    let v2 = bus.read16(0x38373736) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[4].wrapping_sub(2); cpu.a[4] = ad3;
    bus.write16(ad3, v2 as u16);
    22
}
/// $C02F50: move.w $34(a5, d3.w), d3
pub fn i_c02f50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02F54;
    cpu.ir = 0x3635;
    let ad2 = cpu.a[5].wrapping_add(0x34).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C02F54: move.w $31(a2, d3.w), -(a1)
pub fn i_c02f54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02F58;
    cpu.ir = 0x3332;
    let ad2 = cpu.a[2].wrapping_add(0x31).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1].wrapping_sub(2); cpu.a[1] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C02F58: move.w $2f(a0, d3.w), -(a0)
pub fn i_c02f58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02F5C;
    cpu.ir = 0x3130;
    let ad2 = cpu.a[0].wrapping_add(0x2F).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_sub(2); cpu.a[0] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C02F5C: move.l $2d2d(a6), d7
pub fn i_c02f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02F60;
    cpu.ir = 0x2E2E;
    let ad2 = cpu.a[6].wrapping_add(0x2D2D);
    let v3 = bus.read32(ad2);
    cpu.d[7] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C02F60: move.l $2b2b(a4), d6
pub fn i_c02f60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02F64;
    cpu.ir = 0x2C2C;
    let ad2 = cpu.a[4].wrapping_add(0x2B2B);
    let v3 = bus.read32(ad2);
    cpu.d[6] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C02F64: move.l $2929(a2), d5
pub fn i_c02f64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02F68;
    cpu.ir = 0x2A2A;
    let ad2 = cpu.a[2].wrapping_add(0x2929);
    let v3 = bus.read32(ad2);
    cpu.d[5] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C02F68: move.l $2727(a0), d4
pub fn i_c02f68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02F6C;
    cpu.ir = 0x2828;
    let ad2 = cpu.a[0].wrapping_add(0x2727);
    let v3 = bus.read32(ad2);
    cpu.d[4] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C02F6C: move.l -(a6), -(a3)
pub fn i_c02f6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02F6E;
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
/// $C02F6E: move.l -(a5), d3
pub fn i_c02f6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02F70;
    cpu.ir = 0x2625;
    let ad2 = cpu.a[5].wrapping_sub(4); cpu.a[5] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02F70: move.l -(a4), -(a2)
pub fn i_c02f70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02F72;
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
/// $C02F72: move.l -(a4), d2
pub fn i_c02f72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02F74;
    cpu.ir = 0x2424;
    let ad2 = cpu.a[4].wrapping_sub(4); cpu.a[4] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02F74: move.l -(a3), -(a1)
pub fn i_c02f74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02F76;
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
/// $C02F76: move.l -(a2), d1
pub fn i_c02f76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02F78;
    cpu.ir = 0x2222;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02F78: move.l -(a1), d1
pub fn i_c02f78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02F7A;
    cpu.ir = 0x2221;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02F7A: move.l -(a1), -(a0)
pub fn i_c02f7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02F7C;
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
/// $C02F7C: move.l -(a0), d0
pub fn i_c02f7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02F7E;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02F7E: move.l (a7)+, d0
pub fn i_c02f7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02F80;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C02F80: ?
pub fn i_c02f80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02F82;
    cpu.ir = 0x403F;
    let v1: u32 = 0x0;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x80) != 0 && (r2 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x80 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    4
}
/// $C02F82: ?
pub fn i_c02f82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02F84;
    cpu.ir = 0x3F3E;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $C02F84: ?
pub fn i_c02f84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02F86;
    cpu.ir = 0x3D3D;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $C02F86: move.w ([$39f967c0, pc, d3.l * 2], $3736), d6
pub fn i_c02f86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02F8A;
    cpu.ir = 0x3C3B;
    let ad2 = 0xC02FC2_u32.wrapping_add(cpu.d[3]);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C02F8A: move.w $38383736.l, -(a4)
pub fn i_c02f8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02F90;
    cpu.ir = 0x3939;
    let v2 = bus.read16(0x38383736) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[4].wrapping_sub(2); cpu.a[4] = ad3;
    bus.write16(ad3, v2 as u16);
    22
}
/// $C02F90: move.w $34(a5, d3.w), d3
pub fn i_c02f90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02F94;
    cpu.ir = 0x3635;
    let ad2 = cpu.a[5].wrapping_add(0x34).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C02F94: move.w $32(a3, d3.w), -(a1)
pub fn i_c02f94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02F98;
    cpu.ir = 0x3333;
    let ad2 = cpu.a[3].wrapping_add(0x32).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1].wrapping_sub(2); cpu.a[1] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C02F98: move.w $2f(a0, d3.w), -(a0)
pub fn i_c02f98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02F9C;
    cpu.ir = 0x3130;
    let ad2 = cpu.a[0].wrapping_add(0x2F).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_sub(2); cpu.a[0] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C02F9C: move.l $2e2d(a6), -(a7)
pub fn i_c02f9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02FA0;
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
/// $C02FA0: move.l $2c2b(a4), -(a6)
pub fn i_c02fa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02FA4;
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
/// $C02FA4: move.l $2a29(a2), -(a5)
pub fn i_c02fa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02FA8;
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
/// $C02FA8: move.l $2827(a0), -(a4)
pub fn i_c02fa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02FAC;
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
/// $C02FAC: move.l -(a6), -(a3)
pub fn i_c02fac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02FAE;
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
/// $C02FAE: move.l -(a6), d3
pub fn i_c02fae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02FB0;
    cpu.ir = 0x2626;
    let ad2 = cpu.a[6].wrapping_sub(4); cpu.a[6] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02FB0: move.l -(a5), -(a2)
pub fn i_c02fb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02FB2;
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
/// $C02FB2: move.l -(a4), d2
pub fn i_c02fb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02FB4;
    cpu.ir = 0x2424;
    let ad2 = cpu.a[4].wrapping_sub(4); cpu.a[4] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02FB4: move.l -(a3), d2
pub fn i_c02fb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02FB6;
    cpu.ir = 0x2423;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02FB6: move.l -(a2), -(a1)
pub fn i_c02fb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02FB8;
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
/// $C02FB8: move.l -(a2), d1
pub fn i_c02fb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02FBA;
    cpu.ir = 0x2222;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02FBA: move.l -(a1), -(a0)
pub fn i_c02fba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02FBC;
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
/// $C02FBC: move.l -(a0), -(a0)
pub fn i_c02fbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02FBE;
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
/// $C02FBE: move.l -(a0), d0
pub fn i_c02fbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02FC0;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02FC0: ?
pub fn i_c02fc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02FC2;
    cpu.ir = 0x403F;
    let v1: u32 = 0x0;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x80) != 0 && (r2 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x80 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    4
}
/// $C02FC2: ?
pub fn i_c02fc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02FC4;
    cpu.ir = 0x3F3E;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $C02FC4: ?
pub fn i_c02fc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02FC6;
    cpu.ir = 0x3D3D;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $C02FC6: move.w ([$3af96800, pc, d3.l * 2], $3736), d6
pub fn i_c02fc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02FCA;
    cpu.ir = 0x3C3B;
    let ad2 = 0xC03002_u32.wrapping_add(cpu.d[3]);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C02FCA: move.w $38383736.l, d5
pub fn i_c02fca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02FD0;
    cpu.ir = 0x3A39;
    let v2 = bus.read16(0x38383736) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $C02FD0: move.w $33333232(a5, d3.w * 4), d3
pub fn i_c02fd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02FD4;
    cpu.ir = 0x3635;
    let ad2 = cpu.a[5].wrapping_add(0x34).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C02FD4: move.w $32(a3, d3.w), -(a1)
pub fn i_c02fd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02FD8;
    cpu.ir = 0x3333;
    let ad2 = cpu.a[3].wrapping_add(0x32).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1].wrapping_sub(2); cpu.a[1] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C02FD8: move.w $30(a1, d3.w), -(a0)
pub fn i_c02fd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02FDC;
    cpu.ir = 0x3131;
    let ad2 = cpu.a[1].wrapping_add(0x30).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_sub(2); cpu.a[0] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C02FDC: move.l $2e2d(a6), -(a7)
pub fn i_c02fdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02FE0;
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
/// $C02FE0: move.l $2c2b(a4), -(a6)
pub fn i_c02fe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02FE4;
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
/// $C02FE4: move.l $2a29(a2), -(a5)
pub fn i_c02fe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02FE8;
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
/// $C02FE8: move.l $2828(a0), -(a4)
pub fn i_c02fe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02FEC;
    cpu.ir = 0x2928;
    let ad2 = cpu.a[0].wrapping_add(0x2828);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[4];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[4] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C02FEC: move.l -(a7), -(a3)
pub fn i_c02fec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02FEE;
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
/// $C02FEE: move.l -(a6), d3
pub fn i_c02fee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02FF0;
    cpu.ir = 0x2626;
    let ad2 = cpu.a[6].wrapping_sub(4); cpu.a[6] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02FF0: move.l -(a5), -(a2)
pub fn i_c02ff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02FF2;
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
/// $C02FF2: move.l -(a4), -(a2)
pub fn i_c02ff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02FF4;
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
/// $C02FF4: move.l -(a4), d2
pub fn i_c02ff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02FF6;
    cpu.ir = 0x2424;
    let ad2 = cpu.a[4].wrapping_sub(4); cpu.a[4] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02FF6: move.l -(a3), -(a1)
pub fn i_c02ff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02FF8;
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
/// $C02FF8: move.l -(a2), d1
pub fn i_c02ff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02FFA;
    cpu.ir = 0x2222;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02FFA: move.l -(a1), d1
pub fn i_c02ffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02FFC;
    cpu.ir = 0x2221;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C02FFC: move.l -(a1), -(a0)
pub fn i_c02ffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC02FFE;
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
/// $C02FFE: move.l -(a0), d0
pub fn i_c02ffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03000;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C03000: ori.b #$3, d1
pub fn i_c03000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03004;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C03004: subi.b #$7, d5
pub fn i_c03004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03008;
    cpu.ir = 0x405;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x7, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C03008: ?
pub fn i_c03008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0300C;
    cpu.ir = 0x809;
    let mk: u32 = 1u32 << ((11) % 32);
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C0300C: ?
pub fn i_c0300c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03010;
    cpu.ir = 0xC0D;
    let v1 = cpu.a[5] & 0xFF;
    sub_with_flags(cpu, v1, 0xF, Size::Byte, false);
    8
}
/// $C03010: move.b (a1), d0
pub fn i_c03010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03012;
    cpu.ir = 0x1011;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C03012: move.b (a3), d1
pub fn i_c03012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03014;
    cpu.ir = 0x1213;
    let v2 = bus.read8(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C03014: move.b (a5), d2
pub fn i_c03014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03016;
    cpu.ir = 0x1415;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C03016: move.b (a7), d3
pub fn i_c03016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03018;
    cpu.ir = 0x1617;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C03018: move.b (a1)+, d4
pub fn i_c03018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0301A;
    cpu.ir = 0x1819;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C0301A: move.b (a3)+, d5
pub fn i_c0301a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0301C;
    cpu.ir = 0x1A1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C0301C: move.b (a5)+, d6
pub fn i_c0301c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0301E;
    cpu.ir = 0x1C1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C0301E: move.b (a7)+, d7
pub fn i_c0301e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03020;
    cpu.ir = 0x1E1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C03020: move.l -(a1), d0
pub fn i_c03020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03022;
    cpu.ir = 0x2021;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C03022: move.l -(a3), d1
pub fn i_c03022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03024;
    cpu.ir = 0x2223;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C03024: move.l -(a5), d2
pub fn i_c03024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03026;
    cpu.ir = 0x2425;
    let ad2 = cpu.a[5].wrapping_sub(4); cpu.a[5] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C03026: move.l -(a7), d3
pub fn i_c03026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03028;
    cpu.ir = 0x2627;
    let ad2 = cpu.a[7].wrapping_sub(4); cpu.a[7] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C03028: move.l $2a2b(a1), d4
pub fn i_c03028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0302C;
    cpu.ir = 0x2829;
    let ad2 = cpu.a[1].wrapping_add(0x2A2B);
    let v3 = bus.read32(ad2);
    cpu.d[4] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C0302C: move.l $2e2f(a5), d6
pub fn i_c0302c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03030;
    cpu.ir = 0x2C2D;
    let ad2 = cpu.a[5].wrapping_add(0x2E2F);
    let v3 = bus.read32(ad2);
    cpu.d[6] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C03030: move.w $33(a1, d3.w), d0
pub fn i_c03030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03034;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x33).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C03034: move.w $37(a5, d3.w), d2
pub fn i_c03034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03038;
    cpu.ir = 0x3435;
    let ad2 = cpu.a[5].wrapping_add(0x37).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C03038: move.w $3a3b3c3d.l, d4
pub fn i_c03038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0303E;
    cpu.ir = 0x3839;
    let v2 = bus.read16(0x3A3B3C3D) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $C0303C: ?
pub fn i_c0303c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0303E;
    cpu.ir = 0x3C3D;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C0303E: ?
pub fn i_c0303e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03040;
    cpu.ir = 0x3E3F;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C03040: btst.l d0, d1
pub fn i_c03040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03042;
    cpu.ir = 0x101;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C03042: andi.b #$5, d3
pub fn i_c03042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03046;
    cpu.ir = 0x203;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 & 0x5) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C03046: addi.b #$9, d7
pub fn i_c03046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0304A;
    cpu.ir = 0x607;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x9, Size::Byte, true);
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C0304A: ?
pub fn i_c0304a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0304E;
    cpu.ir = 0xA0B;
    let v1 = cpu.a[3] & 0xFF;
    let r2 = (v1 ^ 0xD) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.a[3] = r2;
    8
}
/// $C0304E: ?
pub fn i_c0304e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03052;
    cpu.ir = 0xE0F;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C03052: move.b (a3), d1
pub fn i_c03052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03054;
    cpu.ir = 0x1213;
    let v2 = bus.read8(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C03054: move.b (a5), d2
pub fn i_c03054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03056;
    cpu.ir = 0x1415;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C03056: move.b (a7), d3
pub fn i_c03056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03058;
    cpu.ir = 0x1617;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C03058: move.b (a1)+, d4
pub fn i_c03058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0305A;
    cpu.ir = 0x1819;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C0305A: move.b (a3)+, d5
pub fn i_c0305a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0305C;
    cpu.ir = 0x1A1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C0305C: move.b (a5)+, d6
pub fn i_c0305c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0305E;
    cpu.ir = 0x1C1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C0305E: move.b (a7)+, d7
pub fn i_c0305e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03060;
    cpu.ir = 0x1E1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C03060: move.l -(a1), d0
pub fn i_c03060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03062;
    cpu.ir = 0x2021;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C03062: move.l -(a3), d1
pub fn i_c03062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03064;
    cpu.ir = 0x2223;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C03064: move.l -(a5), d2
pub fn i_c03064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03066;
    cpu.ir = 0x2425;
    let ad2 = cpu.a[5].wrapping_sub(4); cpu.a[5] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C03066: move.l -(a7), d3
pub fn i_c03066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03068;
    cpu.ir = 0x2627;
    let ad2 = cpu.a[7].wrapping_sub(4); cpu.a[7] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C03068: move.l $2a2b(a1), d4
pub fn i_c03068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0306C;
    cpu.ir = 0x2829;
    let ad2 = cpu.a[1].wrapping_add(0x2A2B);
    let v3 = bus.read32(ad2);
    cpu.d[4] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C0306C: move.l $2e2f(a5), d6
pub fn i_c0306c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03070;
    cpu.ir = 0x2C2D;
    let ad2 = cpu.a[5].wrapping_add(0x2E2F);
    let v3 = bus.read32(ad2);
    cpu.d[6] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C03070: move.w $33(a1, d3.w), d0
pub fn i_c03070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03074;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x33).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C03074: move.w $37(a5, d3.w), d2
pub fn i_c03074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03078;
    cpu.ir = 0x3435;
    let ad2 = cpu.a[5].wrapping_add(0x37).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C03078: move.w $3a3b3c3d.l, d4
pub fn i_c03078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0307E;
    cpu.ir = 0x3839;
    let v2 = bus.read16(0x3A3B3C3D) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $C0307E: ?
pub fn i_c0307e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03080;
    cpu.ir = 0x3E3F;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C03080: andi.b #$4, d2
pub fn i_c03080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03084;
    cpu.ir = 0x202;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 & 0x4) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C03084: subi.b #$7, d5
pub fn i_c03084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03088;
    cpu.ir = 0x405;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x7, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C03088: ?
pub fn i_c03088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0308C;
    cpu.ir = 0x809;
    let mk: u32 = 1u32 << ((11) % 32);
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C0308C: ?
pub fn i_c0308c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03090;
    cpu.ir = 0xC0D;
    let v1 = cpu.a[5] & 0xFF;
    sub_with_flags(cpu, v1, 0xF, Size::Byte, false);
    8
}
/// $C03090: move.b (a1), d0
pub fn i_c03090(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03092;
    cpu.ir = 0x1011;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C03092: move.b (a3), d1
pub fn i_c03092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03094;
    cpu.ir = 0x1213;
    let v2 = bus.read8(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C03094: move.b (a5), d2
pub fn i_c03094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03096;
    cpu.ir = 0x1415;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C03096: move.b (a7), d3
pub fn i_c03096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03098;
    cpu.ir = 0x1617;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C03098: move.b (a1)+, d4
pub fn i_c03098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0309A;
    cpu.ir = 0x1819;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C0309A: move.b (a3)+, d5
pub fn i_c0309a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0309C;
    cpu.ir = 0x1A1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C0309C: move.b (a5)+, d6
pub fn i_c0309c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0309E;
    cpu.ir = 0x1C1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C0309E: move.b (a7)+, d7
pub fn i_c0309e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC030A0;
    cpu.ir = 0x1E1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C030A0: move.l -(a1), d0
pub fn i_c030a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC030A2;
    cpu.ir = 0x2021;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C030A2: move.l -(a3), d1
pub fn i_c030a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC030A4;
    cpu.ir = 0x2223;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C030A4: move.l -(a5), d2
pub fn i_c030a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC030A6;
    cpu.ir = 0x2425;
    let ad2 = cpu.a[5].wrapping_sub(4); cpu.a[5] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C030A6: move.l -(a7), d3
pub fn i_c030a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC030A8;
    cpu.ir = 0x2627;
    let ad2 = cpu.a[7].wrapping_sub(4); cpu.a[7] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C030A8: move.l $2a2b(a1), d4
pub fn i_c030a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC030AC;
    cpu.ir = 0x2829;
    let ad2 = cpu.a[1].wrapping_add(0x2A2B);
    let v3 = bus.read32(ad2);
    cpu.d[4] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C030AC: move.l $2e2f(a5), d6
pub fn i_c030ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC030B0;
    cpu.ir = 0x2C2D;
    let ad2 = cpu.a[5].wrapping_add(0x2E2F);
    let v3 = bus.read32(ad2);
    cpu.d[6] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C030B0: move.w $33(a1, d3.w), d0
pub fn i_c030b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC030B4;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x33).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C030B4: move.w $37(a5, d3.w), d2
pub fn i_c030b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC030B8;
    cpu.ir = 0x3435;
    let ad2 = cpu.a[5].wrapping_add(0x37).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C030B8: move.w $3a3b3c3d.l, d4
pub fn i_c030b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC030BE;
    cpu.ir = 0x3839;
    let v2 = bus.read16(0x3A3B3C3D) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $C030BE: ?
pub fn i_c030be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC030C0;
    cpu.ir = 0x3E3F;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C030C0: btst.l d1, d3
pub fn i_c030c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC030C2;
    cpu.ir = 0x303;
    let mk: u32 = 1u32 << ((cpu.d[1]) % 32);
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C030C2: subi.b #$6, d4
pub fn i_c030c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC030C6;
    cpu.ir = 0x404;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x6, Size::Byte, true);
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C030C6: movep.w $909(a0), d3
pub fn i_c030c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC030CA;
    cpu.ir = 0x708;
    let a = cpu.a[0].wrapping_add(0x909);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[3] = (cpu.d[3] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C030CA: ?
pub fn i_c030ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC030CE;
    cpu.ir = 0xA0B;
    let v1 = cpu.a[3] & 0xFF;
    let r2 = (v1 ^ 0xD) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.a[3] = r2;
    8
}
/// $C030CE: ?
pub fn i_c030ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC030D2;
    cpu.ir = 0xE0F;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C030D2: move.b (a3), d1
pub fn i_c030d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC030D4;
    cpu.ir = 0x1213;
    let v2 = bus.read8(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C030D4: move.b (a5), d2
pub fn i_c030d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC030D6;
    cpu.ir = 0x1415;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C030D6: move.b (a7), d3
pub fn i_c030d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC030D8;
    cpu.ir = 0x1617;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C030D8: move.b (a1)+, d4
pub fn i_c030d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC030DA;
    cpu.ir = 0x1819;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C030DA: move.b (a3)+, d5
pub fn i_c030da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC030DC;
    cpu.ir = 0x1A1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C030DC: move.b (a5)+, d6
pub fn i_c030dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC030DE;
    cpu.ir = 0x1C1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C030DE: move.b (a7)+, d7
pub fn i_c030de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC030E0;
    cpu.ir = 0x1E1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C030E0: move.l -(a1), d0
pub fn i_c030e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC030E2;
    cpu.ir = 0x2021;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C030E2: move.l -(a3), d1
pub fn i_c030e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC030E4;
    cpu.ir = 0x2223;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C030E4: move.l -(a5), d2
pub fn i_c030e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC030E6;
    cpu.ir = 0x2425;
    let ad2 = cpu.a[5].wrapping_sub(4); cpu.a[5] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C030E6: move.l -(a7), d3
pub fn i_c030e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC030E8;
    cpu.ir = 0x2627;
    let ad2 = cpu.a[7].wrapping_sub(4); cpu.a[7] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C030E8: move.l $2a2b(a1), d4
pub fn i_c030e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC030EC;
    cpu.ir = 0x2829;
    let ad2 = cpu.a[1].wrapping_add(0x2A2B);
    let v3 = bus.read32(ad2);
    cpu.d[4] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C030EC: move.l $2e2f(a5), d6
pub fn i_c030ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC030F0;
    cpu.ir = 0x2C2D;
    let ad2 = cpu.a[5].wrapping_add(0x2E2F);
    let v3 = bus.read32(ad2);
    cpu.d[6] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C030F0: move.w $33(a1, d3.w), d0
pub fn i_c030f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC030F4;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x33).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C030F4: move.w $37(a5, d3.w), d2
pub fn i_c030f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC030F8;
    cpu.ir = 0x3435;
    let ad2 = cpu.a[5].wrapping_add(0x37).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C030F8: move.w $3a3b3c3d.l, d4
pub fn i_c030f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC030FE;
    cpu.ir = 0x3839;
    let v2 = bus.read16(0x3A3B3C3D) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $C030FE: ?
pub fn i_c030fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03100;
    cpu.ir = 0x3E3F;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C03100: subi.b #$5, d4
pub fn i_c03100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03104;
    cpu.ir = 0x404;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x5, Size::Byte, true);
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C03104: addi.b #$8, d6
pub fn i_c03104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03108;
    cpu.ir = 0x606;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x8, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C03108: movep.w $b0c(a2), d4
pub fn i_c03108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0310C;
    cpu.ir = 0x90A;
    let a = cpu.a[2].wrapping_add(0xB0C);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[4] = (cpu.d[4] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C0310C: movep.w $f10(a6), d6
pub fn i_c0310c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03110;
    cpu.ir = 0xD0E;
    let a = cpu.a[6].wrapping_add(0xF10);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C03110: move.b (a1), d0
pub fn i_c03110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03112;
    cpu.ir = 0x1011;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C03112: move.b (a3), d1
pub fn i_c03112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03114;
    cpu.ir = 0x1213;
    let v2 = bus.read8(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C03114: move.b (a5), d2
pub fn i_c03114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03116;
    cpu.ir = 0x1415;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C03116: move.b (a7), d3
pub fn i_c03116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03118;
    cpu.ir = 0x1617;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C03118: move.b (a1)+, d4
pub fn i_c03118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0311A;
    cpu.ir = 0x1819;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C0311A: move.b (a3)+, d5
pub fn i_c0311a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0311C;
    cpu.ir = 0x1A1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C0311C: move.b (a5)+, d6
pub fn i_c0311c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0311E;
    cpu.ir = 0x1C1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C0311E: move.b (a7)+, d7
pub fn i_c0311e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03120;
    cpu.ir = 0x1E1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C03120: move.l -(a1), d0
pub fn i_c03120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03122;
    cpu.ir = 0x2021;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C03122: move.l -(a3), d1
pub fn i_c03122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03124;
    cpu.ir = 0x2223;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C03124: move.l -(a5), d2
pub fn i_c03124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03126;
    cpu.ir = 0x2425;
    let ad2 = cpu.a[5].wrapping_sub(4); cpu.a[5] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C03126: move.l -(a7), d3
pub fn i_c03126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03128;
    cpu.ir = 0x2627;
    let ad2 = cpu.a[7].wrapping_sub(4); cpu.a[7] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C03128: move.l $2a2b(a1), d4
pub fn i_c03128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0312C;
    cpu.ir = 0x2829;
    let ad2 = cpu.a[1].wrapping_add(0x2A2B);
    let v3 = bus.read32(ad2);
    cpu.d[4] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C0312C: move.l $2e2f(a5), d6
pub fn i_c0312c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03130;
    cpu.ir = 0x2C2D;
    let ad2 = cpu.a[5].wrapping_add(0x2E2F);
    let v3 = bus.read32(ad2);
    cpu.d[6] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C03130: move.w $33(a1, d3.w), d0
pub fn i_c03130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03134;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x33).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C03134: move.w $37(a5, d3.w), d2
pub fn i_c03134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03138;
    cpu.ir = 0x3435;
    let ad2 = cpu.a[5].wrapping_add(0x37).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C03138: move.w $3a3b3c3d.l, d4
pub fn i_c03138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0313E;
    cpu.ir = 0x3839;
    let v2 = bus.read16(0x3A3B3C3D) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $C0313E: ?
pub fn i_c0313e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03140;
    cpu.ir = 0x3E3F;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C03140: btst.l d2, d5
pub fn i_c03140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03142;
    cpu.ir = 0x505;
    let mk: u32 = 1u32 << ((cpu.d[2]) % 32);
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C03142: btst.l d2, d6
pub fn i_c03142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03144;
    cpu.ir = 0x506;
    let mk: u32 = 1u32 << ((cpu.d[2]) % 32);
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C03144: addi.b #$9, d7
pub fn i_c03144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03148;
    cpu.ir = 0x607;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x9, Size::Byte, true);
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C03148: movep.w $b0c(a2), d4
pub fn i_c03148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0314C;
    cpu.ir = 0x90A;
    let a = cpu.a[2].wrapping_add(0xB0C);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[4] = (cpu.d[4] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C0314C: movep.w $f10(a6), d6
pub fn i_c0314c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03150;
    cpu.ir = 0xD0E;
    let a = cpu.a[6].wrapping_add(0xF10);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C03150: move.b (a2), -(a0)
pub fn i_c03150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03152;
    cpu.ir = 0x1112;
    let v2 = bus.read8(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C03152: move.b (a4), -(a1)
pub fn i_c03152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03154;
    cpu.ir = 0x1314;
    let v2 = bus.read8(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[1].wrapping_sub(1); cpu.a[1] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C03154: move.b (a6), -(a2)
pub fn i_c03154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03156;
    cpu.ir = 0x1516;
    let v2 = bus.read8(cpu.a[6]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[2].wrapping_sub(1); cpu.a[2] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C03156: move.b (a0)+, -(a3)
pub fn i_c03156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03158;
    cpu.ir = 0x1718;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[3].wrapping_sub(1); cpu.a[3] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C03158: move.b (a1)+, -(a4)
pub fn i_c03158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0315A;
    cpu.ir = 0x1919;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C0315A: move.b (a3)+, d5
pub fn i_c0315a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0315C;
    cpu.ir = 0x1A1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C0315C: move.b (a5)+, d6
pub fn i_c0315c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0315E;
    cpu.ir = 0x1C1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C0315E: move.b (a7)+, d7
pub fn i_c0315e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03160;
    cpu.ir = 0x1E1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C03160: move.l -(a1), d0
pub fn i_c03160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03162;
    cpu.ir = 0x2021;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C03162: move.l -(a3), d1
pub fn i_c03162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03164;
    cpu.ir = 0x2223;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C03164: move.l -(a5), d2
pub fn i_c03164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03166;
    cpu.ir = 0x2425;
    let ad2 = cpu.a[5].wrapping_sub(4); cpu.a[5] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C03166: move.l -(a7), d3
pub fn i_c03166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03168;
    cpu.ir = 0x2627;
    let ad2 = cpu.a[7].wrapping_sub(4); cpu.a[7] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C03168: move.l $2a2b(a1), d4
pub fn i_c03168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0316C;
    cpu.ir = 0x2829;
    let ad2 = cpu.a[1].wrapping_add(0x2A2B);
    let v3 = bus.read32(ad2);
    cpu.d[4] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C0316C: move.l $2e2f(a5), d6
pub fn i_c0316c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03170;
    cpu.ir = 0x2C2D;
    let ad2 = cpu.a[5].wrapping_add(0x2E2F);
    let v3 = bus.read32(ad2);
    cpu.d[6] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C03170: move.w $33(a1, d3.w), d0
pub fn i_c03170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03174;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x33).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C03174: move.w $37(a5, d3.w), d2
pub fn i_c03174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03178;
    cpu.ir = 0x3435;
    let ad2 = cpu.a[5].wrapping_add(0x37).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C03178: move.w $3a3b3c3d.l, d4
pub fn i_c03178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0317E;
    cpu.ir = 0x3839;
    let v2 = bus.read16(0x3A3B3C3D) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $C0317E: ?
pub fn i_c0317e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03180;
    cpu.ir = 0x3E3F;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C03180: addi.b #$7, d6
pub fn i_c03180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03184;
    cpu.ir = 0x606;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x7, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C03184: movep.w $809(a0), d3
pub fn i_c03184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03188;
    cpu.ir = 0x708;
    let a = cpu.a[0].wrapping_add(0x809);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[3] = (cpu.d[3] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C03188: ?
pub fn i_c03188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0318C;
    cpu.ir = 0xA0B;
    let v1 = cpu.a[3] & 0xFF;
    let r2 = (v1 ^ 0xD) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.a[3] = r2;
    8
}
/// $C0318C: movep.w $f10(a6), d6
pub fn i_c0318c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03190;
    cpu.ir = 0xD0E;
    let a = cpu.a[6].wrapping_add(0xF10);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C03190: move.b (a2), -(a0)
pub fn i_c03190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03192;
    cpu.ir = 0x1112;
    let v2 = bus.read8(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C03192: move.b (a4), -(a1)
pub fn i_c03192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03194;
    cpu.ir = 0x1314;
    let v2 = bus.read8(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[1].wrapping_sub(1); cpu.a[1] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C03194: move.b (a6), -(a2)
pub fn i_c03194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03196;
    cpu.ir = 0x1516;
    let v2 = bus.read8(cpu.a[6]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[2].wrapping_sub(1); cpu.a[2] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C03196: move.b (a0)+, -(a3)
pub fn i_c03196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03198;
    cpu.ir = 0x1718;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[3].wrapping_sub(1); cpu.a[3] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C03198: move.b (a2)+, -(a4)
pub fn i_c03198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0319A;
    cpu.ir = 0x191A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C0319A: move.b (a4)+, -(a5)
pub fn i_c0319a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0319C;
    cpu.ir = 0x1B1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C0319C: move.b (a6)+, -(a6)
pub fn i_c0319c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0319E;
    cpu.ir = 0x1D1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C0319E: move.b -(a0), -(a7)
pub fn i_c0319e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC031A0;
    cpu.ir = 0x1F20;
    let ad2 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad2;
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    16
}
/// $C031A0: move.l -(a2), -(a0)
pub fn i_c031a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC031A2;
    cpu.ir = 0x2122;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[0];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[0] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C031A2: move.l -(a4), -(a1)
pub fn i_c031a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC031A4;
    cpu.ir = 0x2324;
    let ad2 = cpu.a[4].wrapping_sub(4); cpu.a[4] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[1];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[1] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C031A4: move.l -(a5), d2
pub fn i_c031a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC031A6;
    cpu.ir = 0x2425;
    let ad2 = cpu.a[5].wrapping_sub(4); cpu.a[5] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C031A6: move.l -(a7), d3
pub fn i_c031a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC031A8;
    cpu.ir = 0x2627;
    let ad2 = cpu.a[7].wrapping_sub(4); cpu.a[7] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C031A8: move.l $2a2b(a1), d4
pub fn i_c031a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC031AC;
    cpu.ir = 0x2829;
    let ad2 = cpu.a[1].wrapping_add(0x2A2B);
    let v3 = bus.read32(ad2);
    cpu.d[4] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C031AC: move.l $2e2f(a5), d6
pub fn i_c031ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC031B0;
    cpu.ir = 0x2C2D;
    let ad2 = cpu.a[5].wrapping_add(0x2E2F);
    let v3 = bus.read32(ad2);
    cpu.d[6] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C031B0: move.w $33(a1, d3.w), d0
pub fn i_c031b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC031B4;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x33).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C031B4: move.w $37(a5, d3.w), d2
pub fn i_c031b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC031B8;
    cpu.ir = 0x3435;
    let ad2 = cpu.a[5].wrapping_add(0x37).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C031B8: move.w $3a3b3c3d.l, d4
pub fn i_c031b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC031BE;
    cpu.ir = 0x3839;
    let v2 = bus.read16(0x3A3B3C3D) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $C031BE: ?
pub fn i_c031be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC031C0;
    cpu.ir = 0x3E3F;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C031C0: btst.l d3, d7
pub fn i_c031c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC031C2;
    cpu.ir = 0x707;
    let mk: u32 = 1u32 << ((cpu.d[3]) % 32);
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C031C2: movep.w $809(a0), d3
pub fn i_c031c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC031C6;
    cpu.ir = 0x708;
    let a = cpu.a[0].wrapping_add(0x809);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[3] = (cpu.d[3] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C031C6: movep.w $b0b(a2), d4
pub fn i_c031c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC031CA;
    cpu.ir = 0x90A;
    let a = cpu.a[2].wrapping_add(0xB0B);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[4] = (cpu.d[4] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C031CA: ?
pub fn i_c031ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC031CE;
    cpu.ir = 0xC0D;
    let v1 = cpu.a[5] & 0xFF;
    sub_with_flags(cpu, v1, 0xF, Size::Byte, false);
    8
}
/// $C031CE: move.b (a1), d0
pub fn i_c031ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC031D0;
    cpu.ir = 0x1011;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C031D0: move.b (a2), -(a0)
pub fn i_c031d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC031D2;
    cpu.ir = 0x1112;
    let v2 = bus.read8(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C031D2: move.b (a4), -(a1)
pub fn i_c031d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC031D4;
    cpu.ir = 0x1314;
    let v2 = bus.read8(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[1].wrapping_sub(1); cpu.a[1] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C031D4: move.b (a6), -(a2)
pub fn i_c031d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC031D6;
    cpu.ir = 0x1516;
    let v2 = bus.read8(cpu.a[6]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[2].wrapping_sub(1); cpu.a[2] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C031D6: move.b (a0)+, -(a3)
pub fn i_c031d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC031D8;
    cpu.ir = 0x1718;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[3].wrapping_sub(1); cpu.a[3] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C031D8: move.b (a2)+, -(a4)
pub fn i_c031d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC031DA;
    cpu.ir = 0x191A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C031DA: move.b (a4)+, -(a5)
pub fn i_c031da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC031DC;
    cpu.ir = 0x1B1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C031DC: move.b (a6)+, -(a6)
pub fn i_c031dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC031DE;
    cpu.ir = 0x1D1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C031DE: move.b -(a0), -(a7)
pub fn i_c031de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC031E0;
    cpu.ir = 0x1F20;
    let ad2 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad2;
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    16
}
/// $C031E0: move.l -(a2), -(a0)
pub fn i_c031e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC031E2;
    cpu.ir = 0x2122;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[0];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[0] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C031E2: move.l -(a4), -(a1)
pub fn i_c031e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC031E4;
    cpu.ir = 0x2324;
    let ad2 = cpu.a[4].wrapping_sub(4); cpu.a[4] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[1];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[1] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C031E4: move.l -(a6), -(a2)
pub fn i_c031e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC031E6;
    cpu.ir = 0x2526;
    let ad2 = cpu.a[6].wrapping_sub(4); cpu.a[6] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[2];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[2] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C031E6: move.l $292a(a0), -(a3)
pub fn i_c031e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC031EA;
    cpu.ir = 0x2728;
    let ad2 = cpu.a[0].wrapping_add(0x292A);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[3];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[3] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C031EA: move.l $2d2e(a4), -(a5)
pub fn i_c031ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC031EE;
    cpu.ir = 0x2B2C;
    let ad2 = cpu.a[4].wrapping_add(0x2D2E);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[5];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[5] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C031EE: move.l ([$32333435, a0, d3.w]), -(a7)
pub fn i_c031ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC031F2;
    cpu.ir = 0x2F30;
    let ad2 = cpu.a[0].wrapping_add(0x31).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[7];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[7] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $C031F2: move.w $35(a3, d3.w), d1
pub fn i_c031f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC031F6;
    cpu.ir = 0x3233;
    let ad2 = cpu.a[3].wrapping_add(0x35).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C031F6: move.w $39(a7, d3.l), d3
pub fn i_c031f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC031FA;
    cpu.ir = 0x3637;
    let ad2 = cpu.a[7].wrapping_add(0x39).wrapping_add(cpu.d[3]);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C031FA: move.w $c03239(pc, d3.l), d5
pub fn i_c031fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC031FE;
    cpu.ir = 0x3A3B;
    let ad2 = 0xC03239_u32.wrapping_add(cpu.d[3]);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C031FE: ?
pub fn i_c031fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03200;
    cpu.ir = 0x3E3F;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C03200: ?
pub fn i_c03200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03204;
    cpu.ir = 0x808;
    let mk: u32 = 1u32 << ((9) % 32);
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C03204: movep.w $a0b(a1), d4
pub fn i_c03204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03208;
    cpu.ir = 0x909;
    let a = cpu.a[1].wrapping_add(0xA0B);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[4] = (cpu.d[4] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C03208: movep.w $d0e(a4), d5
pub fn i_c03208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0320C;
    cpu.ir = 0xB0C;
    let a = cpu.a[4].wrapping_add(0xD0E);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C0320C: ?
pub fn i_c0320c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03210;
    cpu.ir = 0xE0F;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C03210: move.b (a3), d1
pub fn i_c03210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03212;
    cpu.ir = 0x1213;
    let v2 = bus.read8(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C03212: move.b (a5), d2
pub fn i_c03212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03214;
    cpu.ir = 0x1415;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C03214: move.b (a6), d3
pub fn i_c03214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03216;
    cpu.ir = 0x1616;
    let v2 = bus.read8(cpu.a[6]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C03216: move.b (a0)+, -(a3)
pub fn i_c03216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03218;
    cpu.ir = 0x1718;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[3].wrapping_sub(1); cpu.a[3] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C03218: move.b (a2)+, -(a4)
pub fn i_c03218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0321A;
    cpu.ir = 0x191A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C0321A: move.b (a4)+, -(a5)
pub fn i_c0321a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0321C;
    cpu.ir = 0x1B1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C0321C: move.b (a6)+, -(a6)
pub fn i_c0321c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0321E;
    cpu.ir = 0x1D1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C0321E: move.b -(a0), -(a7)
pub fn i_c0321e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03220;
    cpu.ir = 0x1F20;
    let ad2 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad2;
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    16
}
/// $C03220: move.l -(a2), -(a0)
pub fn i_c03220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03222;
    cpu.ir = 0x2122;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[0];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[0] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C03222: move.l -(a4), -(a1)
pub fn i_c03222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03224;
    cpu.ir = 0x2324;
    let ad2 = cpu.a[4].wrapping_sub(4); cpu.a[4] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[1];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[1] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C03224: move.l -(a6), -(a2)
pub fn i_c03224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03226;
    cpu.ir = 0x2526;
    let ad2 = cpu.a[6].wrapping_sub(4); cpu.a[6] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[2];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[2] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C03226: move.l $292a(a0), -(a3)
pub fn i_c03226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0322A;
    cpu.ir = 0x2728;
    let ad2 = cpu.a[0].wrapping_add(0x292A);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[3];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[3] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C0322A: move.l $2d2e(a4), -(a5)
pub fn i_c0322a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0322E;
    cpu.ir = 0x2B2C;
    let ad2 = cpu.a[4].wrapping_add(0x2D2E);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[5];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[5] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C0322E: move.l ([$33343536, a0, d3.w], $3738), -(a7)
pub fn i_c0322e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03232;
    cpu.ir = 0x2F30;
    let ad2 = cpu.a[0].wrapping_add(0x32).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[7];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[7] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $C03232: move.w ([$3738393a, a4], d3.w * 4, $3b3c), -(a1)
pub fn i_c03232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03236;
    cpu.ir = 0x3334;
    let ad2 = cpu.a[4].wrapping_add(0x36).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1].wrapping_sub(2); cpu.a[1] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C03236: move.w $393a.w, -(a3)
pub fn i_c03236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0323A;
    cpu.ir = 0x3738;
    let v2 = bus.read16(0x393A) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[3].wrapping_sub(2); cpu.a[3] = ad3;
    bus.write16(ad3, v2 as u16);
    18
}
/// $C0323A: move.w #$3d3e, -(a5)
pub fn i_c0323a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0323E;
    cpu.ir = 0x3B3C;
    let v1: u32 = 0x3D3E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_sub(2); cpu.a[5] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $C0323C: ?
pub fn i_c0323c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0323E;
    cpu.ir = 0x3D3E;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $C0323E: move.w d0, $909(a7)
pub fn i_c0323e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03242;
    cpu.ir = 0x3F40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_add(0x909);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C03242: movep.w $a0a(a1), d4
pub fn i_c03242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03246;
    cpu.ir = 0x909;
    let a = cpu.a[1].wrapping_add(0xA0A);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[4] = (cpu.d[4] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C03246: movep.w $c0d(a3), d5
pub fn i_c03246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0324A;
    cpu.ir = 0xB0B;
    let a = cpu.a[3].wrapping_add(0xC0D);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C0324A: movep.w $f10(a6), d6
pub fn i_c0324a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0324E;
    cpu.ir = 0xD0E;
    let a = cpu.a[6].wrapping_add(0xF10);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C0324E: move.b (a1), -(a0)
pub fn i_c0324e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03250;
    cpu.ir = 0x1111;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C03250: move.b (a3), d1
pub fn i_c03250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03252;
    cpu.ir = 0x1213;
    let v2 = bus.read8(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C03252: move.b (a5), d2
pub fn i_c03252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03254;
    cpu.ir = 0x1415;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C03254: move.b (a7), d3
pub fn i_c03254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03256;
    cpu.ir = 0x1617;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C03256: move.b (a1)+, d4
pub fn i_c03256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03258;
    cpu.ir = 0x1819;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C03258: move.b (a3)+, d5
pub fn i_c03258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0325A;
    cpu.ir = 0x1A1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C0325A: move.b (a4)+, d6
pub fn i_c0325a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0325C;
    cpu.ir = 0x1C1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C0325C: move.b (a6)+, -(a6)
pub fn i_c0325c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0325E;
    cpu.ir = 0x1D1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C0325E: move.b -(a0), -(a7)
pub fn i_c0325e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03260;
    cpu.ir = 0x1F20;
    let ad2 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad2;
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    16
}
/// $C03260: move.l -(a2), -(a0)
pub fn i_c03260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03262;
    cpu.ir = 0x2122;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[0];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[0] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C03262: move.l -(a4), -(a1)
pub fn i_c03262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03264;
    cpu.ir = 0x2324;
    let ad2 = cpu.a[4].wrapping_sub(4); cpu.a[4] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[1];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[1] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C03264: move.l -(a6), -(a2)
pub fn i_c03264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03266;
    cpu.ir = 0x2526;
    let ad2 = cpu.a[6].wrapping_sub(4); cpu.a[6] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[2];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[2] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C03266: move.l $292a(a0), -(a3)
pub fn i_c03266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0326A;
    cpu.ir = 0x2728;
    let ad2 = cpu.a[0].wrapping_add(0x292A);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[3];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[3] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C0326A: move.l $2d2e(a4), -(a5)
pub fn i_c0326a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0326E;
    cpu.ir = 0x2B2C;
    let ad2 = cpu.a[4].wrapping_add(0x2D2E);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[5];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[5] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C0326E: move.l ([$33343536, a0, d3.w], $3738), -(a7)
pub fn i_c0326e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03272;
    cpu.ir = 0x2F30;
    let ad2 = cpu.a[0].wrapping_add(0x32).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[7];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[7] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $C03272: move.w ([$3738393a, a4], d3.w * 4, $3b3c), -(a1)
pub fn i_c03272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03276;
    cpu.ir = 0x3334;
    let ad2 = cpu.a[4].wrapping_add(0x36).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1].wrapping_sub(2); cpu.a[1] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C03276: move.w $393a.w, -(a3)
pub fn i_c03276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0327A;
    cpu.ir = 0x3738;
    let v2 = bus.read16(0x393A) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[3].wrapping_sub(2); cpu.a[3] = ad3;
    bus.write16(ad3, v2 as u16);
    18
}
/// $C0327A: move.w #$3d3e, -(a5)
pub fn i_c0327a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0327E;
    cpu.ir = 0x3B3C;
    let v1: u32 = 0x3D3E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_sub(2); cpu.a[5] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $C0327E: move.w d0, $a0a(a7)
pub fn i_c0327e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03282;
    cpu.ir = 0x3F40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_add(0xA0A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C03282: ?
pub fn i_c03282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03286;
    cpu.ir = 0xA0A;
    let v1 = cpu.a[2] & 0xFF;
    let r2 = (v1 ^ 0xB) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.a[2] = r2;
    8
}
/// $C03286: ?
pub fn i_c03286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0328A;
    cpu.ir = 0xC0C;
    let v1 = cpu.a[4] & 0xFF;
    sub_with_flags(cpu, v1, 0xD, Size::Byte, false);
    8
}
/// $C0328A: ?
pub fn i_c0328a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0328E;
    cpu.ir = 0xE0F;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C0328E: move.b (a2), -(a0)
pub fn i_c0328e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03290;
    cpu.ir = 0x1112;
    let v2 = bus.read8(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C03290: move.b (a4), -(a1)
pub fn i_c03290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03292;
    cpu.ir = 0x1314;
    let v2 = bus.read8(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[1].wrapping_sub(1); cpu.a[1] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C03292: move.b (a5), -(a2)
pub fn i_c03292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03294;
    cpu.ir = 0x1515;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[2].wrapping_sub(1); cpu.a[2] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C03294: move.b (a7), d3
pub fn i_c03294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03296;
    cpu.ir = 0x1617;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C03296: move.b (a1)+, d4
pub fn i_c03296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03298;
    cpu.ir = 0x1819;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C03298: move.b (a3)+, d5
pub fn i_c03298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0329A;
    cpu.ir = 0x1A1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C0329A: move.b (a5)+, d6
pub fn i_c0329a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0329C;
    cpu.ir = 0x1C1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C0329C: move.b (a7)+, d7
pub fn i_c0329c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0329E;
    cpu.ir = 0x1E1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C0329E: move.l -(a1), d0
pub fn i_c0329e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC032A0;
    cpu.ir = 0x2021;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C032A0: move.l -(a2), d1
pub fn i_c032a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC032A2;
    cpu.ir = 0x2222;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C032A2: move.l -(a4), -(a1)
pub fn i_c032a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC032A4;
    cpu.ir = 0x2324;
    let ad2 = cpu.a[4].wrapping_sub(4); cpu.a[4] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[1];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[1] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C032A4: move.l -(a6), -(a2)
pub fn i_c032a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC032A6;
    cpu.ir = 0x2526;
    let ad2 = cpu.a[6].wrapping_sub(4); cpu.a[6] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[2];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[2] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C032A6: move.l $292a(a0), -(a3)
pub fn i_c032a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC032AA;
    cpu.ir = 0x2728;
    let ad2 = cpu.a[0].wrapping_add(0x292A);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[3];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[3] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C032AA: move.l $2d2e(a4), -(a5)
pub fn i_c032aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC032AE;
    cpu.ir = 0x2B2C;
    let ad2 = cpu.a[4].wrapping_add(0x2D2E);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[5];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[5] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C032AE: move.l ([$33343536, a0, d3.w], $3738), -(a7)
pub fn i_c032ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC032B2;
    cpu.ir = 0x2F30;
    let ad2 = cpu.a[0].wrapping_add(0x32).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[7];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[7] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $C032B2: move.w ([$3738393a, a4], d3.w * 4, $3b3c), -(a1)
pub fn i_c032b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC032B6;
    cpu.ir = 0x3334;
    let ad2 = cpu.a[4].wrapping_add(0x36).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1].wrapping_sub(2); cpu.a[1] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C032B6: move.w $393a.w, -(a3)
pub fn i_c032b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC032BA;
    cpu.ir = 0x3738;
    let v2 = bus.read16(0x393A) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[3].wrapping_sub(2); cpu.a[3] = ad3;
    bus.write16(ad3, v2 as u16);
    18
}
/// $C032BA: move.w #$3d3e, -(a5)
pub fn i_c032ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC032BE;
    cpu.ir = 0x3B3C;
    let v1: u32 = 0x3D3E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_sub(2); cpu.a[5] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $C032BE: move.w d0, $b0b(a7)
pub fn i_c032be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC032C2;
    cpu.ir = 0x3F40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_add(0xB0B);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C032C2: movep.w $c0c(a3), d5
pub fn i_c032c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC032C6;
    cpu.ir = 0xB0B;
    let a = cpu.a[3].wrapping_add(0xC0C);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C032C6: movep.w $e0e(a5), d6
pub fn i_c032c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC032CA;
    cpu.ir = 0xD0D;
    let a = cpu.a[5].wrapping_add(0xE0E);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C032CA: btst.l d7, (a0)
pub fn i_c032ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC032CC;
    cpu.ir = 0xF10;
    let mk: u32 = 1u32 << ((cpu.d[7]) % 8);
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    12
}
/// $C032CC: move.b (a1), d0
pub fn i_c032cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC032CE;
    cpu.ir = 0x1011;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C032CE: move.b (a3), d1
pub fn i_c032ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC032D0;
    cpu.ir = 0x1213;
    let v2 = bus.read8(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C032D0: move.b (a4), -(a1)
pub fn i_c032d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC032D2;
    cpu.ir = 0x1314;
    let v2 = bus.read8(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[1].wrapping_sub(1); cpu.a[1] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C032D2: move.b (a6), -(a2)
pub fn i_c032d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC032D4;
    cpu.ir = 0x1516;
    let v2 = bus.read8(cpu.a[6]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[2].wrapping_sub(1); cpu.a[2] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C032D4: move.b (a0)+, -(a3)
pub fn i_c032d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC032D6;
    cpu.ir = 0x1718;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[3].wrapping_sub(1); cpu.a[3] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C032D6: move.b (a1)+, -(a4)
pub fn i_c032d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC032D8;
    cpu.ir = 0x1919;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C032D8: move.b (a3)+, d5
pub fn i_c032d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC032DA;
    cpu.ir = 0x1A1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C032DA: move.b (a5)+, d6
pub fn i_c032da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC032DC;
    cpu.ir = 0x1C1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C032DC: move.b (a7)+, d7
pub fn i_c032dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC032DE;
    cpu.ir = 0x1E1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C032DE: move.l -(a1), d0
pub fn i_c032de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC032E0;
    cpu.ir = 0x2021;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C032E0: move.l -(a3), d1
pub fn i_c032e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC032E2;
    cpu.ir = 0x2223;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C032E2: move.l -(a5), d2
pub fn i_c032e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC032E4;
    cpu.ir = 0x2425;
    let ad2 = cpu.a[5].wrapping_sub(4); cpu.a[5] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C032E4: move.l -(a7), d3
pub fn i_c032e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC032E6;
    cpu.ir = 0x2627;
    let ad2 = cpu.a[7].wrapping_sub(4); cpu.a[7] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C032E6: move.l $292a(a1), d4
pub fn i_c032e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC032EA;
    cpu.ir = 0x2829;
    let ad2 = cpu.a[1].wrapping_add(0x292A);
    let v3 = bus.read32(ad2);
    cpu.d[4] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C032EA: move.l $2d2e(a4), -(a5)
pub fn i_c032ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC032EE;
    cpu.ir = 0x2B2C;
    let ad2 = cpu.a[4].wrapping_add(0x2D2E);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[5];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[5] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C032EE: move.l ([$33343536, a0, d3.w], $3738), -(a7)
pub fn i_c032ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC032F2;
    cpu.ir = 0x2F30;
    let ad2 = cpu.a[0].wrapping_add(0x32).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[7];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[7] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $C032F2: move.w ([$3738393a, a4], d3.w * 4, $3b3c), -(a1)
pub fn i_c032f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC032F6;
    cpu.ir = 0x3334;
    let ad2 = cpu.a[4].wrapping_add(0x36).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1].wrapping_sub(2); cpu.a[1] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C032F6: move.w $393a.w, -(a3)
pub fn i_c032f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC032FA;
    cpu.ir = 0x3738;
    let v2 = bus.read16(0x393A) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[3].wrapping_sub(2); cpu.a[3] = ad3;
    bus.write16(ad3, v2 as u16);
    18
}
/// $C032FA: move.w #$3d3e, -(a5)
pub fn i_c032fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC032FE;
    cpu.ir = 0x3B3C;
    let v1: u32 = 0x3D3E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_sub(2); cpu.a[5] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $C032FE: move.w d0, $c0c(a7)
pub fn i_c032fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03302;
    cpu.ir = 0x3F40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_add(0xC0C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C03302: ?
pub fn i_c03302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03306;
    cpu.ir = 0xC0C;
    let v1 = cpu.a[4] & 0xFF;
    sub_with_flags(cpu, v1, 0xD, Size::Byte, false);
    8
}
/// $C03306: movep.w $e0f(a6), d6
pub fn i_c03306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0330A;
    cpu.ir = 0xD0E;
    let a = cpu.a[6].wrapping_add(0xE0F);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C0330A: move.b (a0), d0
pub fn i_c0330a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0330C;
    cpu.ir = 0x1010;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C0330C: move.b (a2), -(a0)
pub fn i_c0330c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0330E;
    cpu.ir = 0x1112;
    let v2 = bus.read8(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C0330E: move.b (a3), d1
pub fn i_c0330e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03310;
    cpu.ir = 0x1213;
    let v2 = bus.read8(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C03310: move.b (a5), d2
pub fn i_c03310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03312;
    cpu.ir = 0x1415;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C03312: move.b (a6), d3
pub fn i_c03312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03314;
    cpu.ir = 0x1616;
    let v2 = bus.read8(cpu.a[6]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C03314: move.b (a0)+, -(a3)
pub fn i_c03314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03316;
    cpu.ir = 0x1718;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[3].wrapping_sub(1); cpu.a[3] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C03316: move.b (a2)+, -(a4)
pub fn i_c03316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03318;
    cpu.ir = 0x191A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C03318: move.b (a4)+, -(a5)
pub fn i_c03318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0331A;
    cpu.ir = 0x1B1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C0331A: move.b (a6)+, -(a6)
pub fn i_c0331a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0331C;
    cpu.ir = 0x1D1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C0331C: move.b (a7)+, d7
pub fn i_c0331c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0331E;
    cpu.ir = 0x1E1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C0331E: move.l -(a1), d0
pub fn i_c0331e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03320;
    cpu.ir = 0x2021;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C03320: move.l -(a3), d1
pub fn i_c03320(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03322;
    cpu.ir = 0x2223;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C03322: move.l -(a5), d2
pub fn i_c03322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03324;
    cpu.ir = 0x2425;
    let ad2 = cpu.a[5].wrapping_sub(4); cpu.a[5] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C03324: move.l -(a7), d3
pub fn i_c03324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03326;
    cpu.ir = 0x2627;
    let ad2 = cpu.a[7].wrapping_sub(4); cpu.a[7] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C03326: move.l $2a2b(a1), d4
pub fn i_c03326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0332A;
    cpu.ir = 0x2829;
    let ad2 = cpu.a[1].wrapping_add(0x2A2B);
    let v3 = bus.read32(ad2);
    cpu.d[4] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C0332A: move.l $2e2f(a5), d6
pub fn i_c0332a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0332E;
    cpu.ir = 0x2C2D;
    let ad2 = cpu.a[5].wrapping_add(0x2E2F);
    let v3 = bus.read32(ad2);
    cpu.d[6] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C0332E: move.w ([$33343536, a1, d3.w], $3738), d0
pub fn i_c0332e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03332;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x32).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C03332: move.w ([$3738393a, a4], d3.w * 4, $3b3c), -(a1)
pub fn i_c03332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03336;
    cpu.ir = 0x3334;
    let ad2 = cpu.a[4].wrapping_add(0x36).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1].wrapping_sub(2); cpu.a[1] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C03336: move.w $393a.w, -(a3)
pub fn i_c03336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0333A;
    cpu.ir = 0x3738;
    let v2 = bus.read16(0x393A) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[3].wrapping_sub(2); cpu.a[3] = ad3;
    bus.write16(ad3, v2 as u16);
    18
}
/// $C0333A: move.w #$3d3e, -(a5)
pub fn i_c0333a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0333E;
    cpu.ir = 0x3B3C;
    let v1: u32 = 0x3D3E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_sub(2); cpu.a[5] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $C0333E: move.w d0, $d0d(a7)
pub fn i_c0333e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03342;
    cpu.ir = 0x3F40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_add(0xD0D);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C03342: movep.w $e0e(a5), d6
pub fn i_c03342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03346;
    cpu.ir = 0xD0D;
    let a = cpu.a[5].wrapping_add(0xE0E);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C03346: ?
pub fn i_c03346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0334A;
    cpu.ir = 0xE0F;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C0334A: move.b (a1), d0
pub fn i_c0334a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0334C;
    cpu.ir = 0x1011;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C0334C: move.b (a2), d1
pub fn i_c0334c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0334E;
    cpu.ir = 0x1212;
    let v2 = bus.read8(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C0334E: move.b (a4), -(a1)
pub fn i_c0334e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03350;
    cpu.ir = 0x1314;
    let v2 = bus.read8(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[1].wrapping_sub(1); cpu.a[1] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C03350: move.b (a5), -(a2)
pub fn i_c03350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03352;
    cpu.ir = 0x1515;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[2].wrapping_sub(1); cpu.a[2] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C03352: move.b (a7), d3
pub fn i_c03352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03354;
    cpu.ir = 0x1617;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C03354: move.b (a1)+, d4
pub fn i_c03354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03356;
    cpu.ir = 0x1819;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C03356: move.b (a2)+, d5
pub fn i_c03356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03358;
    cpu.ir = 0x1A1A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C03358: move.b (a4)+, -(a5)
pub fn i_c03358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0335A;
    cpu.ir = 0x1B1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C0335A: move.b (a6)+, -(a6)
pub fn i_c0335a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0335C;
    cpu.ir = 0x1D1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C0335C: move.b -(a0), -(a7)
pub fn i_c0335c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0335E;
    cpu.ir = 0x1F20;
    let ad2 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad2;
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    16
}
/// $C0335E: move.l -(a2), -(a0)
pub fn i_c0335e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03360;
    cpu.ir = 0x2122;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[0];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[0] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C03360: move.l -(a3), -(a1)
pub fn i_c03360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03362;
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
/// $C03362: move.l -(a5), d2
pub fn i_c03362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03364;
    cpu.ir = 0x2425;
    let ad2 = cpu.a[5].wrapping_sub(4); cpu.a[5] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C03364: move.l -(a7), d3
pub fn i_c03364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03366;
    cpu.ir = 0x2627;
    let ad2 = cpu.a[7].wrapping_sub(4); cpu.a[7] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C03366: move.l $2a2b(a1), d4
pub fn i_c03366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0336A;
    cpu.ir = 0x2829;
    let ad2 = cpu.a[1].wrapping_add(0x2A2B);
    let v3 = bus.read32(ad2);
    cpu.d[4] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C0336A: move.l $2e2f(a5), d6
pub fn i_c0336a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0336E;
    cpu.ir = 0x2C2D;
    let ad2 = cpu.a[5].wrapping_add(0x2E2F);
    let v3 = bus.read32(ad2);
    cpu.d[6] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C0336E: move.w $33(a1, d3.w), d0
pub fn i_c0336e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03372;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x33).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C03372: move.w $37(a5, d3.w), d2
pub fn i_c03372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03376;
    cpu.ir = 0x3435;
    let ad2 = cpu.a[5].wrapping_add(0x37).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C03376: move.w $393a3b3c.l, d4
pub fn i_c03376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0337C;
    cpu.ir = 0x3839;
    let v2 = bus.read16(0x393A3B3C) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $C0337C: ?
pub fn i_c0337c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0337E;
    cpu.ir = 0x3D3E;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $C0337E: move.w d0, $e0e(a7)
pub fn i_c0337e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03382;
    cpu.ir = 0x3F40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_add(0xE0E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C03382: ?
pub fn i_c03382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03386;
    cpu.ir = 0xE0E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C03386: btst.l d7, (a0)
pub fn i_c03386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03388;
    cpu.ir = 0xF10;
    let mk: u32 = 1u32 << ((cpu.d[7]) % 8);
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    12
}
/// $C03388: move.b (a1), d0
pub fn i_c03388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0338A;
    cpu.ir = 0x1011;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C0338A: move.b (a2), -(a0)
pub fn i_c0338a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0338C;
    cpu.ir = 0x1112;
    let v2 = bus.read8(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C0338C: move.b (a3), d1
pub fn i_c0338c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0338E;
    cpu.ir = 0x1213;
    let v2 = bus.read8(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C0338E: move.b (a5), d2
pub fn i_c0338e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03390;
    cpu.ir = 0x1415;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C03390: move.b (a6), -(a2)
pub fn i_c03390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03392;
    cpu.ir = 0x1516;
    let v2 = bus.read8(cpu.a[6]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[2].wrapping_sub(1); cpu.a[2] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C03392: move.b (a0)+, -(a3)
pub fn i_c03392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03394;
    cpu.ir = 0x1718;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[3].wrapping_sub(1); cpu.a[3] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C03394: move.b (a1)+, d4
pub fn i_c03394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03396;
    cpu.ir = 0x1819;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C03396: move.b (a3)+, d5
pub fn i_c03396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03398;
    cpu.ir = 0x1A1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C03398: move.b (a5)+, d6
pub fn i_c03398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0339A;
    cpu.ir = 0x1C1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C0339A: move.b (a6)+, d7
pub fn i_c0339a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0339C;
    cpu.ir = 0x1E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C0339C: move.b -(a0), -(a7)
pub fn i_c0339c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0339E;
    cpu.ir = 0x1F20;
    let ad2 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad2;
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    16
}
/// $C0339E: move.l -(a2), -(a0)
pub fn i_c0339e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC033A0;
    cpu.ir = 0x2122;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[0];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[0] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C033A0: move.l -(a4), -(a1)
pub fn i_c033a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC033A2;
    cpu.ir = 0x2324;
    let ad2 = cpu.a[4].wrapping_sub(4); cpu.a[4] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[1];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[1] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C033A2: move.l -(a6), -(a2)
pub fn i_c033a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC033A4;
    cpu.ir = 0x2526;
    let ad2 = cpu.a[6].wrapping_sub(4); cpu.a[6] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[2];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[2] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C033A4: move.l $2829(a0), -(a3)
pub fn i_c033a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC033A8;
    cpu.ir = 0x2728;
    let ad2 = cpu.a[0].wrapping_add(0x2829);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[3];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[3] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C033A8: move.l $2c2d(a3), d5
pub fn i_c033a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC033AC;
    cpu.ir = 0x2A2B;
    let ad2 = cpu.a[3].wrapping_add(0x2C2D);
    let v3 = bus.read32(ad2);
    cpu.d[5] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C033AC: move.l $3031(a7), d7
pub fn i_c033ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC033B0;
    cpu.ir = 0x2E2F;
    let ad2 = cpu.a[7].wrapping_add(0x3031);
    let v3 = bus.read32(ad2);
    cpu.d[7] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C033B0: move.w $35(a3, d3.w), d1
pub fn i_c033b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC033B4;
    cpu.ir = 0x3233;
    let ad2 = cpu.a[3].wrapping_add(0x35).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C033B4: move.w $39(a7, d3.l), d3
pub fn i_c033b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC033B8;
    cpu.ir = 0x3637;
    let ad2 = cpu.a[7].wrapping_add(0x39).wrapping_add(cpu.d[3]);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C033B8: move.w $c033f7(pc, d3.l), d5
pub fn i_c033b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC033BC;
    cpu.ir = 0x3A3B;
    let ad2 = 0xC033F7_u32.wrapping_add(cpu.d[3]);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C033BC: ?
pub fn i_c033bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC033BE;
    cpu.ir = 0x3E3F;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C033BE: negx.w d1
pub fn i_c033be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC033C0;
    cpu.ir = 0x4041;
    let v1 = cpu.d[1] & 0xFFFF;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFFFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x8000) != 0 && (r2 & 0x8000) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x8000 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C033C0: movep.w $f0f(a7), d7
pub fn i_c033c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC033C4;
    cpu.ir = 0xF0F;
    let a = cpu.a[7].wrapping_add(0xF0F);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C033C4: move.b (a0), d0
pub fn i_c033c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC033C6;
    cpu.ir = 0x1010;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C033C6: move.b (a1), d0
pub fn i_c033c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC033C8;
    cpu.ir = 0x1011;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C033C8: move.b (a1), -(a0)
pub fn i_c033c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC033CA;
    cpu.ir = 0x1111;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C033CA: move.b (a3), d1
pub fn i_c033ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC033CC;
    cpu.ir = 0x1213;
    let v2 = bus.read8(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C033CC: move.b (a4), -(a1)
pub fn i_c033cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC033CE;
    cpu.ir = 0x1314;
    let v2 = bus.read8(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[1].wrapping_sub(1); cpu.a[1] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C033CE: move.b (a5), -(a2)
pub fn i_c033ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC033D0;
    cpu.ir = 0x1515;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[2].wrapping_sub(1); cpu.a[2] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C033D0: move.b (a7), d3
pub fn i_c033d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC033D2;
    cpu.ir = 0x1617;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C033D2: move.b (a0)+, -(a3)
pub fn i_c033d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC033D4;
    cpu.ir = 0x1718;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[3].wrapping_sub(1); cpu.a[3] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C033D4: move.b (a2)+, -(a4)
pub fn i_c033d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC033D6;
    cpu.ir = 0x191A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C033D6: move.b (a3)+, -(a5)
pub fn i_c033d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC033D8;
    cpu.ir = 0x1B1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C033D8: move.b (a5)+, d6
pub fn i_c033d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC033DA;
    cpu.ir = 0x1C1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C033DA: move.b (a7)+, d7
pub fn i_c033da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC033DC;
    cpu.ir = 0x1E1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C033DC: move.l -(a1), d0
pub fn i_c033dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC033DE;
    cpu.ir = 0x2021;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C033DE: move.l -(a2), d1
pub fn i_c033de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC033E0;
    cpu.ir = 0x2222;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C033E0: move.l -(a4), -(a1)
pub fn i_c033e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC033E2;
    cpu.ir = 0x2324;
    let ad2 = cpu.a[4].wrapping_sub(4); cpu.a[4] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[1];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[1] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C033E2: move.l -(a6), -(a2)
pub fn i_c033e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC033E4;
    cpu.ir = 0x2526;
    let ad2 = cpu.a[6].wrapping_sub(4); cpu.a[6] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[2];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[2] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C033E4: move.l $292a(a0), -(a3)
pub fn i_c033e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC033E8;
    cpu.ir = 0x2728;
    let ad2 = cpu.a[0].wrapping_add(0x292A);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[3];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[3] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C033E8: move.l $2d2e(a4), -(a5)
pub fn i_c033e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC033EC;
    cpu.ir = 0x2B2C;
    let ad2 = cpu.a[4].wrapping_add(0x2D2E);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[5];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[5] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C033EC: move.l $3031(a7), d7
pub fn i_c033ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC033F0;
    cpu.ir = 0x2E2F;
    let ad2 = cpu.a[7].wrapping_add(0x3031);
    let v3 = bus.read32(ad2);
    cpu.d[7] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C033F0: move.w $35(a3, d3.w), d1
pub fn i_c033f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC033F4;
    cpu.ir = 0x3233;
    let ad2 = cpu.a[3].wrapping_add(0x35).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C033F4: move.w $39(a7, d3.l), d3
pub fn i_c033f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC033F8;
    cpu.ir = 0x3637;
    let ad2 = cpu.a[7].wrapping_add(0x39).wrapping_add(cpu.d[3]);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C033F8: move.w $c03437(pc, d3.l), d5
pub fn i_c033f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC033FC;
    cpu.ir = 0x3A3B;
    let ad2 = 0xC03437_u32.wrapping_add(cpu.d[3]);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C033FC: ?
pub fn i_c033fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC033FE;
    cpu.ir = 0x3E3F;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C033FE: negx.w d1
pub fn i_c033fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03400;
    cpu.ir = 0x4041;
    let v1 = cpu.d[1] & 0xFFFF;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFFFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x8000) != 0 && (r2 & 0x8000) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x8000 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C03400: move.b (a0), d0
pub fn i_c03400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03402;
    cpu.ir = 0x1010;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C03402: move.b (a0), d0
pub fn i_c03402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03404;
    cpu.ir = 0x1010;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C03404: move.b (a1), d0
pub fn i_c03404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03406;
    cpu.ir = 0x1011;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C03406: move.b (a1), -(a0)
pub fn i_c03406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03408;
    cpu.ir = 0x1111;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C03408: move.b (a2), d1
pub fn i_c03408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0340A;
    cpu.ir = 0x1212;
    let v2 = bus.read8(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C0340A: move.b (a3), -(a1)
pub fn i_c0340a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0340C;
    cpu.ir = 0x1313;
    let v2 = bus.read8(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[1].wrapping_sub(1); cpu.a[1] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C0340C: move.b (a5), d2
pub fn i_c0340c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0340E;
    cpu.ir = 0x1415;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C0340E: move.b (a6), -(a2)
pub fn i_c0340e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03410;
    cpu.ir = 0x1516;
    let v2 = bus.read8(cpu.a[6]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[2].wrapping_sub(1); cpu.a[2] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C03410: move.b (a7), -(a3)
pub fn i_c03410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03412;
    cpu.ir = 0x1717;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[3].wrapping_sub(1); cpu.a[3] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C03412: move.b (a1)+, d4
pub fn i_c03412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03414;
    cpu.ir = 0x1819;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C03414: move.b (a2)+, d5
pub fn i_c03414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03416;
    cpu.ir = 0x1A1A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C03416: move.b (a4)+, -(a5)
pub fn i_c03416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03418;
    cpu.ir = 0x1B1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C03418: move.b (a6)+, -(a6)
pub fn i_c03418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0341A;
    cpu.ir = 0x1D1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C0341A: move.b (a7)+, -(a7)
pub fn i_c0341a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0341C;
    cpu.ir = 0x1F1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C0341C: move.l -(a1), d0
pub fn i_c0341c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0341E;
    cpu.ir = 0x2021;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C0341E: move.l -(a3), d1
pub fn i_c0341e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03420;
    cpu.ir = 0x2223;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C03420: move.l -(a5), d2
pub fn i_c03420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03422;
    cpu.ir = 0x2425;
    let ad2 = cpu.a[5].wrapping_sub(4); cpu.a[5] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C03422: move.l -(a6), d3
pub fn i_c03422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03424;
    cpu.ir = 0x2626;
    let ad2 = cpu.a[6].wrapping_sub(4); cpu.a[6] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C03424: move.l $292a(a0), -(a3)
pub fn i_c03424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03428;
    cpu.ir = 0x2728;
    let ad2 = cpu.a[0].wrapping_add(0x292A);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[3];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[3] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C03428: move.l $2d2e(a4), -(a5)
pub fn i_c03428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0342C;
    cpu.ir = 0x2B2C;
    let ad2 = cpu.a[4].wrapping_add(0x2D2E);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[5];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[5] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C0342C: move.l ([$33343435, a0, d3.w], $3637), -(a7)
pub fn i_c0342c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03430;
    cpu.ir = 0x2F30;
    let ad2 = cpu.a[0].wrapping_add(0x32).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[7];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[7] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $C03430: move.w $35(a4, d3.w), -(a1)
pub fn i_c03430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03434;
    cpu.ir = 0x3334;
    let ad2 = cpu.a[4].wrapping_add(0x35).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1].wrapping_sub(2); cpu.a[1] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C03434: move.w $39(a7, d3.l), d3
pub fn i_c03434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03438;
    cpu.ir = 0x3637;
    let ad2 = cpu.a[7].wrapping_add(0x39).wrapping_add(cpu.d[3]);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C03438: move.w $c03477(pc, d3.l), d5
pub fn i_c03438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0343C;
    cpu.ir = 0x3A3B;
    let ad2 = 0xC03477_u32.wrapping_add(cpu.d[3]);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C0343C: ?
pub fn i_c0343c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0343E;
    cpu.ir = 0x3E3F;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C0343E: negx.w d1
pub fn i_c0343e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03440;
    cpu.ir = 0x4041;
    let v1 = cpu.d[1] & 0xFFFF;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFFFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x8000) != 0 && (r2 & 0x8000) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x8000 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C03440: move.b (a1), -(a0)
pub fn i_c03440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03442;
    cpu.ir = 0x1111;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C03442: move.b (a1), -(a0)
pub fn i_c03442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03444;
    cpu.ir = 0x1111;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C03444: move.b (a2), -(a0)
pub fn i_c03444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03446;
    cpu.ir = 0x1112;
    let v2 = bus.read8(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C03446: move.b (a2), d1
pub fn i_c03446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03448;
    cpu.ir = 0x1212;
    let v2 = bus.read8(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C03448: move.b (a3), -(a1)
pub fn i_c03448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0344A;
    cpu.ir = 0x1313;
    let v2 = bus.read8(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[1].wrapping_sub(1); cpu.a[1] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C0344A: move.b (a4), d2
pub fn i_c0344a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0344C;
    cpu.ir = 0x1414;
    let v2 = bus.read8(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C0344C: move.b (a5), -(a2)
pub fn i_c0344c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0344E;
    cpu.ir = 0x1515;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[2].wrapping_sub(1); cpu.a[2] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C0344E: move.b (a7), d3
pub fn i_c0344e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03450;
    cpu.ir = 0x1617;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C03450: move.b (a0)+, -(a3)
pub fn i_c03450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03452;
    cpu.ir = 0x1718;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[3].wrapping_sub(1); cpu.a[3] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C03452: move.b (a1)+, -(a4)
pub fn i_c03452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03454;
    cpu.ir = 0x1919;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C03454: move.b (a3)+, d5
pub fn i_c03454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03456;
    cpu.ir = 0x1A1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C03456: move.b (a5)+, d6
pub fn i_c03456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03458;
    cpu.ir = 0x1C1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C03458: move.b (a6)+, -(a6)
pub fn i_c03458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0345A;
    cpu.ir = 0x1D1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C0345A: move.b -(a0), -(a7)
pub fn i_c0345a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0345C;
    cpu.ir = 0x1F20;
    let ad2 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad2;
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    16
}
/// $C0345C: move.l -(a2), -(a0)
pub fn i_c0345c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0345E;
    cpu.ir = 0x2122;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[0];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[0] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C0345E: move.l -(a3), d1
pub fn i_c0345e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03460;
    cpu.ir = 0x2223;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C03460: move.l -(a5), d2
pub fn i_c03460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03462;
    cpu.ir = 0x2425;
    let ad2 = cpu.a[5].wrapping_sub(4); cpu.a[5] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C03462: move.l -(a7), d3
pub fn i_c03462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03464;
    cpu.ir = 0x2627;
    let ad2 = cpu.a[7].wrapping_sub(4); cpu.a[7] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C03464: move.l $2a2b(a1), d4
pub fn i_c03464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03468;
    cpu.ir = 0x2829;
    let ad2 = cpu.a[1].wrapping_add(0x2A2B);
    let v3 = bus.read32(ad2);
    cpu.d[4] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C03468: move.l $2d2e(a4), -(a5)
pub fn i_c03468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0346C;
    cpu.ir = 0x2B2C;
    let ad2 = cpu.a[4].wrapping_add(0x2D2E);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[5];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[5] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C0346C: move.l ([$33343536, a0, d3.w], $3738), -(a7)
pub fn i_c0346c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03470;
    cpu.ir = 0x2F30;
    let ad2 = cpu.a[0].wrapping_add(0x32).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[7];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[7] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $C03470: move.w ([$3738393a, a4], d3.w * 4, $3b3b), -(a1)
pub fn i_c03470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03474;
    cpu.ir = 0x3334;
    let ad2 = cpu.a[4].wrapping_add(0x36).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1].wrapping_sub(2); cpu.a[1] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C03474: move.w $393a.w, -(a3)
pub fn i_c03474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03478;
    cpu.ir = 0x3738;
    let v2 = bus.read16(0x393A) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[3].wrapping_sub(2); cpu.a[3] = ad3;
    bus.write16(ad3, v2 as u16);
    18
}
/// $C03478: move.w $c034b7(pc, d3.l), -(a5)
pub fn i_c03478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0347C;
    cpu.ir = 0x3B3B;
    let ad2 = 0xC034B7_u32.wrapping_add(cpu.d[3]);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(2); cpu.a[5] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C0347C: ?
pub fn i_c0347c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0347E;
    cpu.ir = 0x3E3F;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C0347E: negx.w d1
pub fn i_c0347e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03480;
    cpu.ir = 0x4041;
    let v1 = cpu.d[1] & 0xFFFF;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFFFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x8000) != 0 && (r2 & 0x8000) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x8000 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C03480: move.b (a2), d1
pub fn i_c03480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03482;
    cpu.ir = 0x1212;
    let v2 = bus.read8(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C03482: move.b (a2), d1
pub fn i_c03482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03484;
    cpu.ir = 0x1212;
    let v2 = bus.read8(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C03484: move.b (a3), d1
pub fn i_c03484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03486;
    cpu.ir = 0x1213;
    let v2 = bus.read8(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C03486: move.b (a3), -(a1)
pub fn i_c03486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03488;
    cpu.ir = 0x1313;
    let v2 = bus.read8(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[1].wrapping_sub(1); cpu.a[1] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C03488: move.b (a4), d2
pub fn i_c03488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0348A;
    cpu.ir = 0x1414;
    let v2 = bus.read8(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C0348A: move.b (a5), -(a2)
pub fn i_c0348a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0348C;
    cpu.ir = 0x1515;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[2].wrapping_sub(1); cpu.a[2] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C0348C: move.b (a6), d3
pub fn i_c0348c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0348E;
    cpu.ir = 0x1616;
    let v2 = bus.read8(cpu.a[6]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C0348E: move.b (a7), -(a3)
pub fn i_c0348e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03490;
    cpu.ir = 0x1717;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[3].wrapping_sub(1); cpu.a[3] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C03490: move.b (a1)+, d4
pub fn i_c03490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03492;
    cpu.ir = 0x1819;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C03492: move.b (a2)+, -(a4)
pub fn i_c03492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03494;
    cpu.ir = 0x191A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C03494: move.b (a4)+, -(a5)
pub fn i_c03494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03496;
    cpu.ir = 0x1B1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C03496: move.b (a5)+, d6
pub fn i_c03496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03498;
    cpu.ir = 0x1C1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C03498: move.b (a7)+, d7
pub fn i_c03498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0349A;
    cpu.ir = 0x1E1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C0349A: move.l -(a0), d0
pub fn i_c0349a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0349C;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C0349C: move.l -(a2), -(a0)
pub fn i_c0349c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0349E;
    cpu.ir = 0x2122;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[0];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[0] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C0349E: move.l -(a4), -(a1)
pub fn i_c0349e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC034A0;
    cpu.ir = 0x2324;
    let ad2 = cpu.a[4].wrapping_sub(4); cpu.a[4] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[1];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[1] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C034A0: move.l -(a6), -(a2)
pub fn i_c034a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC034A2;
    cpu.ir = 0x2526;
    let ad2 = cpu.a[6].wrapping_sub(4); cpu.a[6] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[2];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[2] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C034A2: move.l -(a7), d3
pub fn i_c034a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC034A4;
    cpu.ir = 0x2627;
    let ad2 = cpu.a[7].wrapping_sub(4); cpu.a[7] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C034A4: move.l $2a2b(a1), d4
pub fn i_c034a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC034A8;
    cpu.ir = 0x2829;
    let ad2 = cpu.a[1].wrapping_add(0x2A2B);
    let v3 = bus.read32(ad2);
    cpu.d[4] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C034A8: move.l $2e2f(a5), d6
pub fn i_c034a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC034AC;
    cpu.ir = 0x2C2D;
    let ad2 = cpu.a[5].wrapping_add(0x2E2F);
    let v3 = bus.read32(ad2);
    cpu.d[6] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C034AC: move.w ([$33343536, a0, d3.w], $3738), d0
pub fn i_c034ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC034B0;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x32).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C034B0: move.w ([$3738393a, a4], d3.w * 4, $3b3c), -(a1)
pub fn i_c034b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC034B4;
    cpu.ir = 0x3334;
    let ad2 = cpu.a[4].wrapping_add(0x36).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1].wrapping_sub(2); cpu.a[1] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C034B4: move.w $393a.w, -(a3)
pub fn i_c034b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC034B8;
    cpu.ir = 0x3738;
    let v2 = bus.read16(0x393A) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[3].wrapping_sub(2); cpu.a[3] = ad3;
    bus.write16(ad3, v2 as u16);
    18
}
/// $C034B8: move.w #$3d3e, -(a5)
pub fn i_c034b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC034BC;
    cpu.ir = 0x3B3C;
    let v1: u32 = 0x3D3E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_sub(2); cpu.a[5] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $C034BC: move.w d0, $4142(a7)
pub fn i_c034bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC034C0;
    cpu.ir = 0x3F40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_add(0x4142);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C034C0: move.b (a3), -(a1)
pub fn i_c034c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC034C2;
    cpu.ir = 0x1313;
    let v2 = bus.read8(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[1].wrapping_sub(1); cpu.a[1] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C034C2: move.b (a3), -(a1)
pub fn i_c034c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC034C4;
    cpu.ir = 0x1313;
    let v2 = bus.read8(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[1].wrapping_sub(1); cpu.a[1] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C034C4: move.b (a4), -(a1)
pub fn i_c034c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC034C6;
    cpu.ir = 0x1314;
    let v2 = bus.read8(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[1].wrapping_sub(1); cpu.a[1] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C034C6: move.b (a4), d2
pub fn i_c034c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC034C8;
    cpu.ir = 0x1414;
    let v2 = bus.read8(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C034C8: move.b (a5), -(a2)
pub fn i_c034c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC034CA;
    cpu.ir = 0x1515;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[2].wrapping_sub(1); cpu.a[2] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C034CA: move.b (a6), -(a2)
pub fn i_c034ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC034CC;
    cpu.ir = 0x1516;
    let v2 = bus.read8(cpu.a[6]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[2].wrapping_sub(1); cpu.a[2] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C034CC: move.b (a7), d3
pub fn i_c034cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC034CE;
    cpu.ir = 0x1617;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C034CE: move.b (a0)+, d4
pub fn i_c034ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC034D0;
    cpu.ir = 0x1818;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C034D0: move.b (a1)+, -(a4)
pub fn i_c034d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC034D2;
    cpu.ir = 0x1919;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C034D2: move.b (a3)+, d5
pub fn i_c034d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC034D4;
    cpu.ir = 0x1A1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C034D4: move.b (a4)+, d6
pub fn i_c034d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC034D6;
    cpu.ir = 0x1C1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C034D6: move.b (a6)+, -(a6)
pub fn i_c034d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC034D8;
    cpu.ir = 0x1D1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C034D8: move.b (a7)+, -(a7)
pub fn i_c034d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC034DA;
    cpu.ir = 0x1F1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C034DA: move.l -(a1), d0
pub fn i_c034da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC034DC;
    cpu.ir = 0x2021;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C034DC: move.l -(a3), d1
pub fn i_c034dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC034DE;
    cpu.ir = 0x2223;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C034DE: move.l -(a4), d2
pub fn i_c034de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC034E0;
    cpu.ir = 0x2424;
    let ad2 = cpu.a[4].wrapping_sub(4); cpu.a[4] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C034E0: move.l -(a6), -(a2)
pub fn i_c034e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC034E2;
    cpu.ir = 0x2526;
    let ad2 = cpu.a[6].wrapping_sub(4); cpu.a[6] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[2];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[2] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C034E2: move.l $292a(a0), -(a3)
pub fn i_c034e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC034E6;
    cpu.ir = 0x2728;
    let ad2 = cpu.a[0].wrapping_add(0x292A);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[3];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[3] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C034E6: move.l $2c2d(a3), d5
pub fn i_c034e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC034EA;
    cpu.ir = 0x2A2B;
    let ad2 = cpu.a[3].wrapping_add(0x2C2D);
    let v3 = bus.read32(ad2);
    cpu.d[5] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C034EA: move.l $3031(a7), d7
pub fn i_c034ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC034EE;
    cpu.ir = 0x2E2F;
    let ad2 = cpu.a[7].wrapping_add(0x3031);
    let v3 = bus.read32(ad2);
    cpu.d[7] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C034EE: move.w $35(a3, d3.w), d1
pub fn i_c034ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC034F2;
    cpu.ir = 0x3233;
    let ad2 = cpu.a[3].wrapping_add(0x35).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C034F2: move.w $393a3b3c(a6, d3.w * 8), -(a2)
pub fn i_c034f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC034F6;
    cpu.ir = 0x3536;
    let ad2 = cpu.a[6].wrapping_add(0x38).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2].wrapping_sub(2); cpu.a[2] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C034F6: move.w $c07034(pc), -(a4)
pub fn i_c034f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC034FA;
    cpu.ir = 0x393A;
    let v2 = bus.read16(0xC07034) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[4].wrapping_sub(2); cpu.a[4] = ad3;
    bus.write16(ad3, v2 as u16);
    18
}
/// $C034FA: ?
pub fn i_c034fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC034FC;
    cpu.ir = 0x3D3E;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $C034FC: move.w d0, $4142(a7)
pub fn i_c034fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03500;
    cpu.ir = 0x3F40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_add(0x4142);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C03500: move.b (a4), d2
pub fn i_c03500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03502;
    cpu.ir = 0x1414;
    let v2 = bus.read8(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C03502: move.b (a4), d2
pub fn i_c03502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03504;
    cpu.ir = 0x1414;
    let v2 = bus.read8(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C03504: move.b (a5), d2
pub fn i_c03504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03506;
    cpu.ir = 0x1415;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C03506: move.b (a5), -(a2)
pub fn i_c03506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03508;
    cpu.ir = 0x1515;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[2].wrapping_sub(1); cpu.a[2] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C03508: move.b (a6), d3
pub fn i_c03508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0350A;
    cpu.ir = 0x1616;
    let v2 = bus.read8(cpu.a[6]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C0350A: move.b (a7), d3
pub fn i_c0350a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0350C;
    cpu.ir = 0x1617;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C0350C: move.b (a0)+, -(a3)
pub fn i_c0350c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0350E;
    cpu.ir = 0x1718;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[3].wrapping_sub(1); cpu.a[3] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C0350E: move.b (a1)+, d4
pub fn i_c0350e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03510;
    cpu.ir = 0x1819;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C03510: move.b (a2)+, d5
pub fn i_c03510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03512;
    cpu.ir = 0x1A1A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C03512: move.b (a4)+, -(a5)
pub fn i_c03512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03514;
    cpu.ir = 0x1B1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C03514: move.b (a5)+, d6
pub fn i_c03514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03516;
    cpu.ir = 0x1C1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C03516: move.b (a6)+, d7
pub fn i_c03516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03518;
    cpu.ir = 0x1E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C03518: move.b -(a0), -(a7)
pub fn i_c03518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0351A;
    cpu.ir = 0x1F20;
    let ad2 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad2;
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    16
}
/// $C0351A: move.l -(a2), -(a0)
pub fn i_c0351a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0351C;
    cpu.ir = 0x2122;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[0];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[0] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C0351C: move.l -(a3), d1
pub fn i_c0351c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0351E;
    cpu.ir = 0x2223;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C0351E: move.l -(a5), d2
pub fn i_c0351e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03520;
    cpu.ir = 0x2425;
    let ad2 = cpu.a[5].wrapping_sub(4); cpu.a[5] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C03520: move.l -(a7), d3
pub fn i_c03520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03522;
    cpu.ir = 0x2627;
    let ad2 = cpu.a[7].wrapping_sub(4); cpu.a[7] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C03522: move.l $292a(a0), -(a3)
pub fn i_c03522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03526;
    cpu.ir = 0x2728;
    let ad2 = cpu.a[0].wrapping_add(0x292A);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[3];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[3] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C03526: move.l $2d2e(a4), -(a5)
pub fn i_c03526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0352A;
    cpu.ir = 0x2B2C;
    let ad2 = cpu.a[4].wrapping_add(0x2D2E);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[5];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[5] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C0352A: move.l $3031(a7), -(a7)
pub fn i_c0352a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0352E;
    cpu.ir = 0x2F2F;
    let ad2 = cpu.a[7].wrapping_add(0x3031);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[7];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[7] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C0352E: move.w $35(a3, d3.w), d1
pub fn i_c0352e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03532;
    cpu.ir = 0x3233;
    let ad2 = cpu.a[3].wrapping_add(0x35).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C03532: move.w $39(a7, d3.l), d3
pub fn i_c03532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03536;
    cpu.ir = 0x3637;
    let ad2 = cpu.a[7].wrapping_add(0x39).wrapping_add(cpu.d[3]);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C03536: move.w $3d3e3f40(a16, d3.l * 2), d5
pub fn i_c03536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0353A;
    cpu.ir = 0x3A3B;
    let ad2 = 0xC03574_u32.wrapping_add(cpu.d[3]);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C0353A: ?
pub fn i_c0353a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0353C;
    cpu.ir = 0x3D3E;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $C0353C: move.w d0, $4142(a7)
pub fn i_c0353c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03540;
    cpu.ir = 0x3F40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_add(0x4142);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C03540: move.b (a5), -(a2)
pub fn i_c03540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03542;
    cpu.ir = 0x1515;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[2].wrapping_sub(1); cpu.a[2] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C03542: move.b (a5), -(a2)
pub fn i_c03542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03544;
    cpu.ir = 0x1515;
    let v2 = bus.read8(cpu.a[5]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[2].wrapping_sub(1); cpu.a[2] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C03544: move.b (a6), -(a2)
pub fn i_c03544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03546;
    cpu.ir = 0x1516;
    let v2 = bus.read8(cpu.a[6]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[2].wrapping_sub(1); cpu.a[2] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C03546: move.b (a6), d3
pub fn i_c03546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03548;
    cpu.ir = 0x1616;
    let v2 = bus.read8(cpu.a[6]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C03548: move.b (a7), d3
pub fn i_c03548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0354A;
    cpu.ir = 0x1617;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C0354A: move.b (a0)+, -(a3)
pub fn i_c0354a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0354C;
    cpu.ir = 0x1718;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[3].wrapping_sub(1); cpu.a[3] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C0354C: move.b (a1)+, d4
pub fn i_c0354c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0354E;
    cpu.ir = 0x1819;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C0354E: move.b (a2)+, -(a4)
pub fn i_c0354e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03550;
    cpu.ir = 0x191A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C03550: move.b (a3)+, d5
pub fn i_c03550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03552;
    cpu.ir = 0x1A1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C03552: move.b (a4)+, d6
pub fn i_c03552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03554;
    cpu.ir = 0x1C1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C03554: move.b (a6)+, -(a6)
pub fn i_c03554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03556;
    cpu.ir = 0x1D1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C03556: move.b (a7)+, d7
pub fn i_c03556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03558;
    cpu.ir = 0x1E1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C03558: move.l -(a1), d0
pub fn i_c03558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0355A;
    cpu.ir = 0x2021;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C0355A: move.l -(a2), -(a0)
pub fn i_c0355a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0355C;
    cpu.ir = 0x2122;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[0];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[0] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C0355C: move.l -(a4), -(a1)
pub fn i_c0355c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0355E;
    cpu.ir = 0x2324;
    let ad2 = cpu.a[4].wrapping_sub(4); cpu.a[4] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[1];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[1] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C0355E: move.l -(a5), -(a2)
pub fn i_c0355e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03560;
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
/// $C03560: move.l -(a7), d3
pub fn i_c03560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03562;
    cpu.ir = 0x2627;
    let ad2 = cpu.a[7].wrapping_sub(4); cpu.a[7] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C03562: move.l $2a2b(a1), d4
pub fn i_c03562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03566;
    cpu.ir = 0x2829;
    let ad2 = cpu.a[1].wrapping_add(0x2A2B);
    let v3 = bus.read32(ad2);
    cpu.d[4] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C03566: move.l $2d2e(a4), -(a5)
pub fn i_c03566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0356A;
    cpu.ir = 0x2B2C;
    let ad2 = cpu.a[4].wrapping_add(0x2D2E);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[5];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[5] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C0356A: move.l ([$33333435, a0, d3.w], $3637), -(a7)
pub fn i_c0356a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0356E;
    cpu.ir = 0x2F30;
    let ad2 = cpu.a[0].wrapping_add(0x32).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[7];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[7] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $C0356E: move.w $35(a3, d3.w), -(a1)
pub fn i_c0356e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03572;
    cpu.ir = 0x3333;
    let ad2 = cpu.a[3].wrapping_add(0x35).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1].wrapping_sub(2); cpu.a[1] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C03572: move.w $39(a7, d3.l), d3
pub fn i_c03572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03576;
    cpu.ir = 0x3637;
    let ad2 = cpu.a[7].wrapping_add(0x39).wrapping_add(cpu.d[3]);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C03576: move.w $c035b5(pc, d3.l), d5
pub fn i_c03576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0357A;
    cpu.ir = 0x3A3B;
    let ad2 = 0xC035B5_u32.wrapping_add(cpu.d[3]);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C0357A: ?
pub fn i_c0357a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0357C;
    cpu.ir = 0x3E3F;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C0357C: negx.w d1
pub fn i_c0357c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0357E;
    cpu.ir = 0x4041;
    let v1 = cpu.d[1] & 0xFFFF;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFFFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x8000) != 0 && (r2 & 0x8000) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x8000 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C0357E: ?
pub fn i_c0357e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03580;
    cpu.ir = 0x4142;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C03580: move.b (a6), d3
pub fn i_c03580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03582;
    cpu.ir = 0x1616;
    let v2 = bus.read8(cpu.a[6]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C03582: move.b (a6), d3
pub fn i_c03582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03584;
    cpu.ir = 0x1616;
    let v2 = bus.read8(cpu.a[6]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C03584: move.b (a7), d3
pub fn i_c03584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03586;
    cpu.ir = 0x1617;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C03586: move.b (a7), -(a3)
pub fn i_c03586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03588;
    cpu.ir = 0x1717;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[3].wrapping_sub(1); cpu.a[3] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C03588: move.b (a0)+, -(a3)
pub fn i_c03588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0358A;
    cpu.ir = 0x1718;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[3].wrapping_sub(1); cpu.a[3] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C0358A: move.b (a1)+, d4
pub fn i_c0358a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0358C;
    cpu.ir = 0x1819;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C0358C: move.b (a2)+, -(a4)
pub fn i_c0358c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0358E;
    cpu.ir = 0x191A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C0358E: move.b (a3)+, d5
pub fn i_c0358e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03590;
    cpu.ir = 0x1A1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C03590: move.b (a4)+, -(a5)
pub fn i_c03590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03592;
    cpu.ir = 0x1B1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C03592: move.b (a5)+, d6
pub fn i_c03592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03594;
    cpu.ir = 0x1C1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C03594: move.b (a6)+, d7
pub fn i_c03594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03596;
    cpu.ir = 0x1E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C03596: move.b -(a0), -(a7)
pub fn i_c03596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03598;
    cpu.ir = 0x1F20;
    let ad2 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad2;
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    16
}
/// $C03598: move.l -(a1), -(a0)
pub fn i_c03598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0359A;
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
/// $C0359A: move.l -(a3), d1
pub fn i_c0359a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0359C;
    cpu.ir = 0x2223;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C0359C: move.l -(a4), d2
pub fn i_c0359c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0359E;
    cpu.ir = 0x2424;
    let ad2 = cpu.a[4].wrapping_sub(4); cpu.a[4] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C0359E: move.l -(a6), -(a2)
pub fn i_c0359e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC035A0;
    cpu.ir = 0x2526;
    let ad2 = cpu.a[6].wrapping_sub(4); cpu.a[6] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[2];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[2] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C035A0: move.l $2829(a0), -(a3)
pub fn i_c035a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC035A4;
    cpu.ir = 0x2728;
    let ad2 = cpu.a[0].wrapping_add(0x2829);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[3];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[3] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C035A4: move.l $2c2d(a3), d5
pub fn i_c035a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC035A8;
    cpu.ir = 0x2A2B;
    let ad2 = cpu.a[3].wrapping_add(0x2C2D);
    let v3 = bus.read32(ad2);
    cpu.d[5] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C035A8: move.l $2f30(a7), d7
pub fn i_c035a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC035AC;
    cpu.ir = 0x2E2F;
    let ad2 = cpu.a[7].wrapping_add(0x2F30);
    let v3 = bus.read32(ad2);
    cpu.d[7] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C035AC: move.w $35363738(a2, d3.w * 2), -(a0)
pub fn i_c035ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC035B0;
    cpu.ir = 0x3132;
    let ad2 = cpu.a[2].wrapping_add(0x34).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_sub(2); cpu.a[0] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C035B0: move.w $38393a3b(a6, d3.w * 8), -(a2)
pub fn i_c035b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC035B4;
    cpu.ir = 0x3536;
    let ad2 = cpu.a[6].wrapping_add(0x38).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2].wrapping_sub(2); cpu.a[2] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C035B4: move.w $3a3b3c3d.l, d4
pub fn i_c035b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC035BA;
    cpu.ir = 0x3839;
    let v2 = bus.read16(0x3A3B3C3D) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $C035BA: ?
pub fn i_c035ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC035BC;
    cpu.ir = 0x3E3F;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C035BC: negx.w d1
pub fn i_c035bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC035BE;
    cpu.ir = 0x4041;
    let v1 = cpu.d[1] & 0xFFFF;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFFFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x8000) != 0 && (r2 & 0x8000) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x8000 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C035BE: clr.w d3
pub fn i_c035be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC035C0;
    cpu.ir = 0x4243;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C035C0: move.b (a7), -(a3)
pub fn i_c035c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC035C2;
    cpu.ir = 0x1717;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[3].wrapping_sub(1); cpu.a[3] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C035C2: move.b (a7), -(a3)
pub fn i_c035c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC035C4;
    cpu.ir = 0x1717;
    let v2 = bus.read8(cpu.a[7]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[3].wrapping_sub(1); cpu.a[3] = ad3;
    bus.write8(ad3, v2 as u8);
    14
}
/// $C035C4: move.b (a0)+, -(a3)
pub fn i_c035c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC035C6;
    cpu.ir = 0x1718;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[3].wrapping_sub(1); cpu.a[3] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C035C6: move.b (a0)+, d4
pub fn i_c035c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC035C8;
    cpu.ir = 0x1818;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C035C8: move.b (a1)+, d4
pub fn i_c035c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC035CA;
    cpu.ir = 0x1819;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C035CA: move.b (a1)+, -(a4)
pub fn i_c035ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC035CC;
    cpu.ir = 0x1919;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C035CC: move.b (a2)+, d5
pub fn i_c035cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC035CE;
    cpu.ir = 0x1A1A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C035CE: move.b (a3)+, -(a5)
pub fn i_c035ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC035D0;
    cpu.ir = 0x1B1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C035D0: move.b (a5)+, d6
pub fn i_c035d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC035D2;
    cpu.ir = 0x1C1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C035D2: move.b (a6)+, -(a6)
pub fn i_c035d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC035D4;
    cpu.ir = 0x1D1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C035D4: move.b (a7)+, d7
pub fn i_c035d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC035D6;
    cpu.ir = 0x1E1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C035D6: move.l -(a1), d0
pub fn i_c035d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC035D8;
    cpu.ir = 0x2021;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C035D8: move.l -(a2), -(a0)
pub fn i_c035d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC035DA;
    cpu.ir = 0x2122;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[0];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[0] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C035DA: move.l -(a3), -(a1)
pub fn i_c035da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC035DC;
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
/// $C035DC: move.l -(a5), d2
pub fn i_c035dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC035DE;
    cpu.ir = 0x2425;
    let ad2 = cpu.a[5].wrapping_sub(4); cpu.a[5] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C035DE: move.l -(a7), d3
pub fn i_c035de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC035E0;
    cpu.ir = 0x2627;
    let ad2 = cpu.a[7].wrapping_sub(4); cpu.a[7] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C035E0: move.l $292a(a0), -(a3)
pub fn i_c035e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC035E4;
    cpu.ir = 0x2728;
    let ad2 = cpu.a[0].wrapping_add(0x292A);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[3];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[3] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C035E4: move.l $2c2d(a4), -(a5)
pub fn i_c035e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC035E8;
    cpu.ir = 0x2B2C;
    let ad2 = cpu.a[4].wrapping_add(0x2C2D);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[5];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[5] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C035E8: move.l $3031(a7), d7
pub fn i_c035e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC035EC;
    cpu.ir = 0x2E2F;
    let ad2 = cpu.a[7].wrapping_add(0x3031);
    let v3 = bus.read32(ad2);
    cpu.d[7] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C035EC: move.w $35363738(a3, d3.w * 2), d1
pub fn i_c035ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC035F0;
    cpu.ir = 0x3233;
    let ad2 = cpu.a[3].wrapping_add(0x34).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C035F0: move.w $393a3b3c(a6, d3.w * 8), -(a2)
pub fn i_c035f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC035F4;
    cpu.ir = 0x3536;
    let ad2 = cpu.a[6].wrapping_add(0x38).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2].wrapping_sub(2); cpu.a[2] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C035F4: move.w $c07132(pc), -(a4)
pub fn i_c035f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC035F8;
    cpu.ir = 0x393A;
    let v2 = bus.read16(0xC07132) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[4].wrapping_sub(2); cpu.a[4] = ad3;
    bus.write16(ad3, v2 as u16);
    18
}
/// $C035F8: ?
pub fn i_c035f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC035FA;
    cpu.ir = 0x3D3D;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $C035FA: ?
pub fn i_c035fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC035FC;
    cpu.ir = 0x3E3F;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C035FC: negx.w d1
pub fn i_c035fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC035FE;
    cpu.ir = 0x4041;
    let v1 = cpu.d[1] & 0xFFFF;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFFFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x8000) != 0 && (r2 & 0x8000) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x8000 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C035FE: clr.w d3
pub fn i_c035fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03600;
    cpu.ir = 0x4243;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C03600: move.b (a0)+, d4
pub fn i_c03600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03602;
    cpu.ir = 0x1818;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C03602: move.b (a0)+, d4
pub fn i_c03602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03604;
    cpu.ir = 0x1818;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C03604: move.b (a1)+, d4
pub fn i_c03604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03606;
    cpu.ir = 0x1819;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C03606: move.b (a1)+, -(a4)
pub fn i_c03606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03608;
    cpu.ir = 0x1919;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C03608: move.b (a2)+, -(a4)
pub fn i_c03608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0360A;
    cpu.ir = 0x191A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C0360A: move.b (a2)+, d5
pub fn i_c0360a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0360C;
    cpu.ir = 0x1A1A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C0360C: move.b (a3)+, -(a5)
pub fn i_c0360c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0360E;
    cpu.ir = 0x1B1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C0360E: move.b (a4)+, d6
pub fn i_c0360e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03610;
    cpu.ir = 0x1C1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C03610: move.b (a5)+, -(a6)
pub fn i_c03610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03612;
    cpu.ir = 0x1D1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C03612: move.b (a7)+, d7
pub fn i_c03612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03614;
    cpu.ir = 0x1E1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C03614: move.b -(a0), -(a7)
pub fn i_c03614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03616;
    cpu.ir = 0x1F20;
    let ad2 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad2;
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    16
}
/// $C03616: move.l -(a1), -(a0)
pub fn i_c03616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03618;
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
/// $C03618: move.l -(a3), d1
pub fn i_c03618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0361A;
    cpu.ir = 0x2223;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C0361A: move.l -(a4), -(a1)
pub fn i_c0361a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0361C;
    cpu.ir = 0x2324;
    let ad2 = cpu.a[4].wrapping_sub(4); cpu.a[4] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[1];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[1] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C0361C: move.l -(a6), -(a2)
pub fn i_c0361c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0361E;
    cpu.ir = 0x2526;
    let ad2 = cpu.a[6].wrapping_sub(4); cpu.a[6] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[2];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[2] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C0361E: move.l -(a7), d3
pub fn i_c0361e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03620;
    cpu.ir = 0x2627;
    let ad2 = cpu.a[7].wrapping_sub(4); cpu.a[7] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C03620: move.l $2a2a(a1), d4
pub fn i_c03620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03624;
    cpu.ir = 0x2829;
    let ad2 = cpu.a[1].wrapping_add(0x2A2A);
    let v3 = bus.read32(ad2);
    cpu.d[4] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C03624: move.l $2d2e(a4), -(a5)
pub fn i_c03624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03628;
    cpu.ir = 0x2B2C;
    let ad2 = cpu.a[4].wrapping_add(0x2D2E);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[5];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[5] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C03628: move.l $31(a0, d3.w), -(a7)
pub fn i_c03628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0362C;
    cpu.ir = 0x2F30;
    let ad2 = cpu.a[0].wrapping_add(0x31).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[7];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[7] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $C0362C: move.w $35(a3, d3.w), d1
pub fn i_c0362c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03630;
    cpu.ir = 0x3233;
    let ad2 = cpu.a[3].wrapping_add(0x35).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C03630: move.w $393a3b3c(a7, d3.w * 8), d3
pub fn i_c03630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03634;
    cpu.ir = 0x3637;
    let ad2 = cpu.a[7].wrapping_add(0x38).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C03634: move.w $c07172(pc), -(a4)
pub fn i_c03634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03638;
    cpu.ir = 0x393A;
    let v2 = bus.read16(0xC07172) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[4].wrapping_sub(2); cpu.a[4] = ad3;
    bus.write16(ad3, v2 as u16);
    18
}
/// $C03638: ?
pub fn i_c03638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0363A;
    cpu.ir = 0x3D3E;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $C0363A: move.w d0, $4142(a7)
pub fn i_c0363a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0363E;
    cpu.ir = 0x3F40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_add(0x4142);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C0363E: clr.w d3
pub fn i_c0363e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03640;
    cpu.ir = 0x4243;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C03640: move.b (a1)+, -(a4)
pub fn i_c03640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03642;
    cpu.ir = 0x1919;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C03642: move.b (a1)+, -(a4)
pub fn i_c03642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03644;
    cpu.ir = 0x1919;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C03644: move.b (a1)+, -(a4)
pub fn i_c03644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03646;
    cpu.ir = 0x1919;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C03646: move.b (a2)+, d5
pub fn i_c03646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03648;
    cpu.ir = 0x1A1A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C03648: move.b (a3)+, d5
pub fn i_c03648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0364A;
    cpu.ir = 0x1A1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C0364A: move.b (a3)+, -(a5)
pub fn i_c0364a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0364C;
    cpu.ir = 0x1B1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C0364C: move.b (a4)+, d6
pub fn i_c0364c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0364E;
    cpu.ir = 0x1C1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C0364E: move.b (a5)+, -(a6)
pub fn i_c0364e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03650;
    cpu.ir = 0x1D1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C03650: move.b (a6)+, d7
pub fn i_c03650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03652;
    cpu.ir = 0x1E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C03652: move.b (a7)+, -(a7)
pub fn i_c03652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03654;
    cpu.ir = 0x1F1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C03654: move.l -(a1), d0
pub fn i_c03654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03656;
    cpu.ir = 0x2021;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C03656: move.l -(a2), -(a0)
pub fn i_c03656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03658;
    cpu.ir = 0x2122;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[0];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[0] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C03658: move.l -(a3), -(a1)
pub fn i_c03658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0365A;
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
/// $C0365A: move.l -(a5), d2
pub fn i_c0365a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0365C;
    cpu.ir = 0x2425;
    let ad2 = cpu.a[5].wrapping_sub(4); cpu.a[5] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C0365C: move.l -(a6), d3
pub fn i_c0365c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0365E;
    cpu.ir = 0x2626;
    let ad2 = cpu.a[6].wrapping_sub(4); cpu.a[6] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C0365E: move.l $2929(a0), -(a3)
pub fn i_c0365e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03662;
    cpu.ir = 0x2728;
    let ad2 = cpu.a[0].wrapping_add(0x2929);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[3];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[3] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C03662: move.l $2c2d(a3), d5
pub fn i_c03662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03666;
    cpu.ir = 0x2A2B;
    let ad2 = cpu.a[3].wrapping_add(0x2C2D);
    let v3 = bus.read32(ad2);
    cpu.d[5] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C03666: move.l $2f30(a6), -(a6)
pub fn i_c03666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0366A;
    cpu.ir = 0x2D2E;
    let ad2 = cpu.a[6].wrapping_add(0x2F30);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[6];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[6] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C0366A: move.w ([$34353637, a2, d3.w * 2], $aaaaaaaa), -(a0)
pub fn i_c0366a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0366E;
    cpu.ir = 0x3132;
    let ad2 = cpu.a[2].wrapping_add(0x33).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_sub(2); cpu.a[0] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C0366E: move.w $37(a5, d3.w), d2
pub fn i_c0366e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03672;
    cpu.ir = 0x3435;
    let ad2 = cpu.a[5].wrapping_add(0x37).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C03672: move.w $3a3b3c3c.l, d4
pub fn i_c03672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03678;
    cpu.ir = 0x3839;
    let v2 = bus.read16(0x3A3B3C3C) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $C03678: ?
pub fn i_c03678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0367A;
    cpu.ir = 0x3D3E;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $C0367A: move.w d0, $4142(a7)
pub fn i_c0367a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0367E;
    cpu.ir = 0x3F40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_add(0x4142);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C0367E: ?
pub fn i_c0367e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03680;
    cpu.ir = 0x4344;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C03680: move.b (a2)+, d5
pub fn i_c03680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03682;
    cpu.ir = 0x1A1A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C03682: move.b (a2)+, d5
pub fn i_c03682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03684;
    cpu.ir = 0x1A1A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C03684: move.b (a2)+, d5
pub fn i_c03684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03686;
    cpu.ir = 0x1A1A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C03686: move.b (a3)+, -(a5)
pub fn i_c03686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03688;
    cpu.ir = 0x1B1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C03688: move.b (a4)+, -(a5)
pub fn i_c03688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0368A;
    cpu.ir = 0x1B1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C0368A: move.b (a4)+, d6
pub fn i_c0368a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0368C;
    cpu.ir = 0x1C1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C0368C: move.b (a5)+, -(a6)
pub fn i_c0368c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0368E;
    cpu.ir = 0x1D1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C0368E: move.b (a6)+, d7
pub fn i_c0368e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03690;
    cpu.ir = 0x1E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C03690: move.b (a7)+, -(a7)
pub fn i_c03690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03692;
    cpu.ir = 0x1F1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C03692: move.l -(a0), d0
pub fn i_c03692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03694;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C03694: move.l -(a1), -(a0)
pub fn i_c03694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03696;
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
/// $C03696: move.l -(a3), d1
pub fn i_c03696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03698;
    cpu.ir = 0x2223;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C03698: move.l -(a4), -(a1)
pub fn i_c03698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0369A;
    cpu.ir = 0x2324;
    let ad2 = cpu.a[4].wrapping_sub(4); cpu.a[4] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[1];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[1] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C0369A: move.l -(a5), -(a2)
pub fn i_c0369a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0369C;
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
/// $C0369C: move.l -(a7), d3
pub fn i_c0369c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0369E;
    cpu.ir = 0x2627;
    let ad2 = cpu.a[7].wrapping_sub(4); cpu.a[7] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C0369E: move.l $292a(a0), d4
pub fn i_c0369e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC036A2;
    cpu.ir = 0x2828;
    let ad2 = cpu.a[0].wrapping_add(0x292A);
    let v3 = bus.read32(ad2);
    cpu.d[4] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C036A2: move.l $2c2d(a4), -(a5)
pub fn i_c036a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC036A6;
    cpu.ir = 0x2B2C;
    let ad2 = cpu.a[4].wrapping_add(0x2C2D);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[5];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[5] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C036A6: move.l $3031(a7), d7
pub fn i_c036a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC036AA;
    cpu.ir = 0x2E2F;
    let ad2 = cpu.a[7].wrapping_add(0x3031);
    let v3 = bus.read32(ad2);
    cpu.d[7] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C036AA: move.w $35363737(a2, d3.w * 2), -(a0)
pub fn i_c036aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC036AE;
    cpu.ir = 0x3132;
    let ad2 = cpu.a[2].wrapping_add(0x34).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_sub(2); cpu.a[0] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C036AE: move.w ([$38393a3b, a6], d3.w * 8, $aaaaaaaa), -(a2)
pub fn i_c036ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC036B2;
    cpu.ir = 0x3536;
    let ad2 = cpu.a[6].wrapping_add(0x37).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2].wrapping_sub(2); cpu.a[2] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C036B2: move.w $3a3b3c3d.l, d4
pub fn i_c036b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC036B8;
    cpu.ir = 0x3839;
    let v2 = bus.read16(0x3A3B3C3D) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $C036B8: ?
pub fn i_c036b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC036BA;
    cpu.ir = 0x3E3F;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C036BA: negx.w d0
pub fn i_c036ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC036BC;
    cpu.ir = 0x4040;
    let v1 = cpu.d[0] & 0xFFFF;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFFFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x8000) != 0 && (r2 & 0x8000) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x8000 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C036BC: ?
pub fn i_c036bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC036BE;
    cpu.ir = 0x4142;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C036BE: ?
pub fn i_c036be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC036C0;
    cpu.ir = 0x4344;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C036C0: move.b (a3)+, -(a5)
pub fn i_c036c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC036C2;
    cpu.ir = 0x1B1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C036C2: move.b (a3)+, -(a5)
pub fn i_c036c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC036C4;
    cpu.ir = 0x1B1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C036C4: move.b (a3)+, -(a5)
pub fn i_c036c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC036C6;
    cpu.ir = 0x1B1B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(1); cpu.a[5] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C036C6: move.b (a4)+, d6
pub fn i_c036c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC036C8;
    cpu.ir = 0x1C1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C036C8: move.b (a4)+, d6
pub fn i_c036c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC036CA;
    cpu.ir = 0x1C1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C036CA: move.b (a5)+, -(a6)
pub fn i_c036ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC036CC;
    cpu.ir = 0x1D1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C036CC: move.b (a6)+, d7
pub fn i_c036cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC036CE;
    cpu.ir = 0x1E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C036CE: move.b (a7)+, d7
pub fn i_c036ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC036D0;
    cpu.ir = 0x1E1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C036D0: move.b -(a0), -(a7)
pub fn i_c036d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC036D2;
    cpu.ir = 0x1F20;
    let ad2 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad2;
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    16
}
/// $C036D2: move.l -(a1), d0
pub fn i_c036d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC036D4;
    cpu.ir = 0x2021;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C036D4: move.l -(a2), d1
pub fn i_c036d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC036D6;
    cpu.ir = 0x2222;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C036D6: move.l -(a3), -(a1)
pub fn i_c036d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC036D8;
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
/// $C036D8: move.l -(a5), d2
pub fn i_c036d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC036DA;
    cpu.ir = 0x2425;
    let ad2 = cpu.a[5].wrapping_sub(4); cpu.a[5] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C036DA: move.l -(a6), -(a2)
pub fn i_c036da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC036DC;
    cpu.ir = 0x2526;
    let ad2 = cpu.a[6].wrapping_sub(4); cpu.a[6] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[2];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[2] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C036DC: move.l $2829(a0), -(a3)
pub fn i_c036dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC036E0;
    cpu.ir = 0x2728;
    let ad2 = cpu.a[0].wrapping_add(0x2829);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[3];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[3] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C036E0: move.l $2b2c(a3), d5
pub fn i_c036e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC036E4;
    cpu.ir = 0x2A2B;
    let ad2 = cpu.a[3].wrapping_add(0x2B2C);
    let v3 = bus.read32(ad2);
    cpu.d[5] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C036E4: move.l $2f2f(a6), -(a6)
pub fn i_c036e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC036E8;
    cpu.ir = 0x2D2E;
    let ad2 = cpu.a[6].wrapping_add(0x2F2F);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[6];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[6] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C036E8: move.w $33(a1, d3.w), d0
pub fn i_c036e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC036EC;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x33).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C036EC: move.w ([$3738393a, a4], d3.w * 4, $3b3b), d2
pub fn i_c036ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC036F0;
    cpu.ir = 0x3434;
    let ad2 = cpu.a[4].wrapping_add(0x36).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C036F0: move.w $393a.w, -(a3)
pub fn i_c036f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC036F4;
    cpu.ir = 0x3738;
    let v2 = bus.read16(0x393A) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[3].wrapping_sub(2); cpu.a[3] = ad3;
    bus.write16(ad3, v2 as u16);
    18
}
/// $C036F4: move.w $c03733(pc, d3.l), -(a5)
pub fn i_c036f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC036F8;
    cpu.ir = 0x3B3B;
    let ad2 = 0xC03733_u32.wrapping_add(cpu.d[3]);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_sub(2); cpu.a[5] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C036F8: ?
pub fn i_c036f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC036FA;
    cpu.ir = 0x3E3F;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C036FA: negx.w d1
pub fn i_c036fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC036FC;
    cpu.ir = 0x4041;
    let v1 = cpu.d[1] & 0xFFFF;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFFFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x8000) != 0 && (r2 & 0x8000) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x8000 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C036FC: clr.w d3
pub fn i_c036fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC036FE;
    cpu.ir = 0x4243;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C036FE: neg.w d5
pub fn i_c036fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03700;
    cpu.ir = 0x4445;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C03700: move.b (a4)+, d6
pub fn i_c03700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03702;
    cpu.ir = 0x1C1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C03702: move.b (a4)+, d6
pub fn i_c03702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03704;
    cpu.ir = 0x1C1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C03704: move.b (a4)+, d6
pub fn i_c03704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03706;
    cpu.ir = 0x1C1C;
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C03706: move.b (a5)+, -(a6)
pub fn i_c03706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03708;
    cpu.ir = 0x1D1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C03708: move.b (a5)+, -(a6)
pub fn i_c03708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0370A;
    cpu.ir = 0x1D1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C0370A: move.b (a6)+, d7
pub fn i_c0370a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0370C;
    cpu.ir = 0x1E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C0370C: move.b (a7)+, d7
pub fn i_c0370c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0370E;
    cpu.ir = 0x1E1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C0370E: move.b -(a0), -(a7)
pub fn i_c0370e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03710;
    cpu.ir = 0x1F20;
    let ad2 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad2;
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    16
}
/// $C03710: move.l -(a1), d0
pub fn i_c03710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03712;
    cpu.ir = 0x2021;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C03712: move.l -(a2), -(a0)
pub fn i_c03712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03714;
    cpu.ir = 0x2122;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[0];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[0] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C03714: move.l -(a3), d1
pub fn i_c03714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03716;
    cpu.ir = 0x2223;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C03716: move.l -(a4), d2
pub fn i_c03716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03718;
    cpu.ir = 0x2424;
    let ad2 = cpu.a[4].wrapping_sub(4); cpu.a[4] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C03718: move.l -(a6), -(a2)
pub fn i_c03718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0371A;
    cpu.ir = 0x2526;
    let ad2 = cpu.a[6].wrapping_sub(4); cpu.a[6] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[2];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[2] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C0371A: move.l -(a7), d3
pub fn i_c0371a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0371C;
    cpu.ir = 0x2627;
    let ad2 = cpu.a[7].wrapping_sub(4); cpu.a[7] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C0371C: move.l $292a(a0), d4
pub fn i_c0371c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03720;
    cpu.ir = 0x2828;
    let ad2 = cpu.a[0].wrapping_add(0x292A);
    let v3 = bus.read32(ad2);
    cpu.d[4] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C03720: move.l $2c2d(a3), -(a5)
pub fn i_c03720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03724;
    cpu.ir = 0x2B2B;
    let ad2 = cpu.a[3].wrapping_add(0x2C2D);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[5];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[5] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C03724: move.l $2f30(a6), d7
pub fn i_c03724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03728;
    cpu.ir = 0x2E2E;
    let ad2 = cpu.a[6].wrapping_add(0x2F30);
    let v3 = bus.read32(ad2);
    cpu.d[7] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C03728: move.w $33(a2, d3.w), -(a0)
pub fn i_c03728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0372C;
    cpu.ir = 0x3132;
    let ad2 = cpu.a[2].wrapping_add(0x33).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_sub(2); cpu.a[0] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C0372C: move.w $37(a5, d3.w), d2
pub fn i_c0372c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03730;
    cpu.ir = 0x3435;
    let ad2 = cpu.a[5].wrapping_add(0x37).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C03730: move.w $393a.w, d4
pub fn i_c03730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03734;
    cpu.ir = 0x3838;
    let v2 = bus.read16(0x393A) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v2 & 0xFFFF);
    12
}
/// $C03734: move.w #$3d3e, -(a5)
pub fn i_c03734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03738;
    cpu.ir = 0x3B3C;
    let v1: u32 = 0x3D3E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_sub(2); cpu.a[5] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $C03738: move.w d0, $4041(a7)
pub fn i_c03738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0373C;
    cpu.ir = 0x3F40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_add(0x4041);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C0373C: clr.w d3
pub fn i_c0373c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0373E;
    cpu.ir = 0x4243;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C0373E: neg.w d5
pub fn i_c0373e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03740;
    cpu.ir = 0x4445;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C03740: move.b (a5)+, -(a6)
pub fn i_c03740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03742;
    cpu.ir = 0x1D1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C03742: move.b (a5)+, -(a6)
pub fn i_c03742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03744;
    cpu.ir = 0x1D1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C03744: move.b (a5)+, -(a6)
pub fn i_c03744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03746;
    cpu.ir = 0x1D1D;
    let ad2 = cpu.a[5]; cpu.a[5] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_sub(1); cpu.a[6] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C03746: move.b (a6)+, d7
pub fn i_c03746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03748;
    cpu.ir = 0x1E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C03748: move.b (a6)+, d7
pub fn i_c03748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0374A;
    cpu.ir = 0x1E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C0374A: move.b (a7)+, -(a7)
pub fn i_c0374a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0374C;
    cpu.ir = 0x1F1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C0374C: move.b -(a0), -(a7)
pub fn i_c0374c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0374E;
    cpu.ir = 0x1F20;
    let ad2 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad2;
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    16
}
/// $C0374E: move.l -(a1), d0
pub fn i_c0374e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03750;
    cpu.ir = 0x2021;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C03750: move.l -(a2), -(a0)
pub fn i_c03750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03752;
    cpu.ir = 0x2122;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[0];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[0] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C03752: move.l -(a3), d1
pub fn i_c03752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03754;
    cpu.ir = 0x2223;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C03754: move.l -(a4), -(a1)
pub fn i_c03754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03756;
    cpu.ir = 0x2324;
    let ad2 = cpu.a[4].wrapping_sub(4); cpu.a[4] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[1];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[1] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C03756: move.l -(a5), d2
pub fn i_c03756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03758;
    cpu.ir = 0x2425;
    let ad2 = cpu.a[5].wrapping_sub(4); cpu.a[5] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C03758: move.l -(a6), d3
pub fn i_c03758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0375A;
    cpu.ir = 0x2626;
    let ad2 = cpu.a[6].wrapping_sub(4); cpu.a[6] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C0375A: move.l $2829(a0), -(a3)
pub fn i_c0375a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0375E;
    cpu.ir = 0x2728;
    let ad2 = cpu.a[0].wrapping_add(0x2829);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[3];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[3] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C0375E: move.l $2b2c(a2), d5
pub fn i_c0375e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03762;
    cpu.ir = 0x2A2A;
    let ad2 = cpu.a[2].wrapping_add(0x2B2C);
    let v3 = bus.read32(ad2);
    cpu.d[5] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C03762: move.l $2e2f(a5), -(a6)
pub fn i_c03762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03766;
    cpu.ir = 0x2D2D;
    let ad2 = cpu.a[5].wrapping_add(0x2E2F);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[6];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[6] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C03766: move.w ([$33343536, a1, d3.w], $3637), d0
pub fn i_c03766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0376A;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x32).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C0376A: move.w ([$36373839, a4], d3.w * 4, $3a3b), -(a1)
pub fn i_c0376a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0376E;
    cpu.ir = 0x3334;
    let ad2 = cpu.a[4].wrapping_add(0x36).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1].wrapping_sub(2); cpu.a[1] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C0376E: move.w $39(a7, d3.l), d3
pub fn i_c0376e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03772;
    cpu.ir = 0x3637;
    let ad2 = cpu.a[7].wrapping_add(0x39).wrapping_add(cpu.d[3]);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C03772: move.w $c037b0(pc, d3.l), d5
pub fn i_c03772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03776;
    cpu.ir = 0x3A3B;
    let ad2 = 0xC037B0_u32.wrapping_add(cpu.d[3]);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C03776: ?
pub fn i_c03776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03778;
    cpu.ir = 0x3D3E;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $C03778: move.w d0, $4142(a7)
pub fn i_c03778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0377C;
    cpu.ir = 0x3F40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_add(0x4142);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C0377C: ?
pub fn i_c0377c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0377E;
    cpu.ir = 0x4344;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C0377E: neg.w d5
pub fn i_c0377e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03780;
    cpu.ir = 0x4445;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C03780: move.b (a6)+, d7
pub fn i_c03780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03782;
    cpu.ir = 0x1E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C03782: move.b (a6)+, d7
pub fn i_c03782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03784;
    cpu.ir = 0x1E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C03784: move.b (a6)+, d7
pub fn i_c03784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03786;
    cpu.ir = 0x1E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C03786: move.b (a7)+, -(a7)
pub fn i_c03786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03788;
    cpu.ir = 0x1F1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C03788: move.b (a7)+, -(a7)
pub fn i_c03788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0378A;
    cpu.ir = 0x1F1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C0378A: move.l -(a0), d0
pub fn i_c0378a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0378C;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C0378C: move.l -(a1), d0
pub fn i_c0378c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0378E;
    cpu.ir = 0x2021;
    let ad2 = cpu.a[1].wrapping_sub(4); cpu.a[1] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C0378E: move.l -(a2), -(a0)
pub fn i_c0378e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03790;
    cpu.ir = 0x2122;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[0];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[0] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C03790: move.l -(a2), d1
pub fn i_c03790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03792;
    cpu.ir = 0x2222;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C03792: move.l -(a4), -(a1)
pub fn i_c03792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03794;
    cpu.ir = 0x2324;
    let ad2 = cpu.a[4].wrapping_sub(4); cpu.a[4] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[1];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[1] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C03794: move.l -(a5), d2
pub fn i_c03794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03796;
    cpu.ir = 0x2425;
    let ad2 = cpu.a[5].wrapping_sub(4); cpu.a[5] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C03796: move.l -(a6), -(a2)
pub fn i_c03796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03798;
    cpu.ir = 0x2526;
    let ad2 = cpu.a[6].wrapping_sub(4); cpu.a[6] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[2];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[2] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C03798: move.l -(a7), d3
pub fn i_c03798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0379A;
    cpu.ir = 0x2627;
    let ad2 = cpu.a[7].wrapping_sub(4); cpu.a[7] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C0379A: move.l $292a(a0), d4
pub fn i_c0379a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0379E;
    cpu.ir = 0x2828;
    let ad2 = cpu.a[0].wrapping_add(0x292A);
    let v3 = bus.read32(ad2);
    cpu.d[4] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C0379E: move.l $2c2d(a3), d5
pub fn i_c0379e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC037A2;
    cpu.ir = 0x2A2B;
    let ad2 = cpu.a[3].wrapping_add(0x2C2D);
    let v3 = bus.read32(ad2);
    cpu.d[5] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C037A2: move.l $2f30(a6), -(a6)
pub fn i_c037a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC037A6;
    cpu.ir = 0x2D2E;
    let ad2 = cpu.a[6].wrapping_add(0x2F30);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[6];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[6] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C037A6: move.w $33(a1, d3.w), d0
pub fn i_c037a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC037AA;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x33).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C037AA: move.w ([$37383939, a4], d3.w * 4, $3a3b), d2
pub fn i_c037aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC037AE;
    cpu.ir = 0x3434;
    let ad2 = cpu.a[4].wrapping_add(0x36).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C037AE: move.w $3939.w, -(a3)
pub fn i_c037ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC037B2;
    cpu.ir = 0x3738;
    let v2 = bus.read16(0x3939) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[3].wrapping_sub(2); cpu.a[3] = ad3;
    bus.write16(ad3, v2 as u16);
    18
}
/// $C037B2: move.w $c037f1(pc, d3.l), d5
pub fn i_c037b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC037B6;
    cpu.ir = 0x3A3B;
    let ad2 = 0xC037F1_u32.wrapping_add(cpu.d[3]);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C037B6: ?
pub fn i_c037b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC037B8;
    cpu.ir = 0x3E3F;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C037B8: negx.w d0
pub fn i_c037b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC037BA;
    cpu.ir = 0x4040;
    let v1 = cpu.d[0] & 0xFFFF;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFFFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x8000) != 0 && (r2 & 0x8000) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x8000 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C037BA: ?
pub fn i_c037ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC037BC;
    cpu.ir = 0x4142;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C037BC: ?
pub fn i_c037bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC037BE;
    cpu.ir = 0x4344;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C037BE: ?
pub fn i_c037be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC037C0;
    cpu.ir = 0x4546;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C037C0: move.b (a7)+, -(a7)
pub fn i_c037c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC037C2;
    cpu.ir = 0x1F1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C037C2: move.b (a7)+, -(a7)
pub fn i_c037c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC037C4;
    cpu.ir = 0x1F1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C037C4: move.b (a7)+, -(a7)
pub fn i_c037c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC037C6;
    cpu.ir = 0x1F1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $C037C6: move.l -(a0), d0
pub fn i_c037c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC037C8;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C037C8: move.l -(a0), d0
pub fn i_c037c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC037CA;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C037CA: move.l -(a1), -(a0)
pub fn i_c037ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC037CC;
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
/// $C037CC: move.l -(a2), -(a0)
pub fn i_c037cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC037CE;
    cpu.ir = 0x2122;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[0];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[0] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C037CE: move.l -(a2), d1
pub fn i_c037ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC037D0;
    cpu.ir = 0x2222;
    let ad2 = cpu.a[2].wrapping_sub(4); cpu.a[2] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C037D0: move.l -(a3), -(a1)
pub fn i_c037d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC037D2;
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
/// $C037D2: move.l -(a4), d2
pub fn i_c037d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC037D4;
    cpu.ir = 0x2424;
    let ad2 = cpu.a[4].wrapping_sub(4); cpu.a[4] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C037D4: move.l -(a5), -(a2)
pub fn i_c037d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC037D6;
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
/// $C037D6: move.l -(a7), d3
pub fn i_c037d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC037D8;
    cpu.ir = 0x2627;
    let ad2 = cpu.a[7].wrapping_sub(4); cpu.a[7] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C037D8: move.l $2829(a0), -(a3)
pub fn i_c037d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC037DC;
    cpu.ir = 0x2728;
    let ad2 = cpu.a[0].wrapping_add(0x2829);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[3];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[3] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C037DC: move.l $2b2c(a2), d5
pub fn i_c037dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC037E0;
    cpu.ir = 0x2A2A;
    let ad2 = cpu.a[2].wrapping_add(0x2B2C);
    let v3 = bus.read32(ad2);
    cpu.d[5] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C037E0: move.l $2e2f(a5), -(a6)
pub fn i_c037e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC037E4;
    cpu.ir = 0x2D2D;
    let ad2 = cpu.a[5].wrapping_add(0x2E2F);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[6];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[6] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C037E4: move.w ([$33333435, a0, d3.w], $3637), d0
pub fn i_c037e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC037E8;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x32).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C037E8: move.w $35(a3, d3.w), -(a1)
pub fn i_c037e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC037EC;
    cpu.ir = 0x3333;
    let ad2 = cpu.a[3].wrapping_add(0x35).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1].wrapping_sub(2); cpu.a[1] = ad4;
    bus.write16(ad4, v3 as u16);
    20
}
/// $C037EC: move.w $393a3b3c(a7, d3.w * 8), d3
pub fn i_c037ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC037F0;
    cpu.ir = 0x3637;
    let ad2 = cpu.a[7].wrapping_add(0x38).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C037F0: move.w $c0732e(pc), -(a4)
pub fn i_c037f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC037F4;
    cpu.ir = 0x393A;
    let v2 = bus.read16(0xC0732E) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[4].wrapping_sub(2); cpu.a[4] = ad3;
    bus.write16(ad3, v2 as u16);
    18
}
/// $C037F4: ?
pub fn i_c037f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC037F6;
    cpu.ir = 0x3D3D;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $C037F6: ?
pub fn i_c037f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC037F8;
    cpu.ir = 0x3E3F;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C037F8: negx.w d1
pub fn i_c037f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC037FA;
    cpu.ir = 0x4041;
    let v1 = cpu.d[1] & 0xFFFF;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFFFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x8000) != 0 && (r2 & 0x8000) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x8000 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C037FA: clr.w d3
pub fn i_c037fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC037FC;
    cpu.ir = 0x4243;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C037FC: neg.w d4
pub fn i_c037fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC037FE;
    cpu.ir = 0x4444;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C037FE: ?
pub fn i_c037fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC03800;
    cpu.ir = 0x4546;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C045EC: dc.w $ffff
pub fn i_c045ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC045EE;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C045EE: dc.w $ffff
pub fn i_c045ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC045F0;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C045F0: dc.w $ffff
pub fn i_c045f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC045F2;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C045F2: dc.w $ffff
pub fn i_c045f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC045F4;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C045F4: dc.w $ffff
pub fn i_c045f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC045F6;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C045F6: dc.w $ffff
pub fn i_c045f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC045F8;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C045F8: dc.w $ffff
pub fn i_c045f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC045FA;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C045FA: dc.w $ffff
pub fn i_c045fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC045FC;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C045FC: dc.w $ffff
pub fn i_c045fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC045FE;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C04EBA: dc.w $ffff
pub fn i_c04eba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC04EBC;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C04EBC: dc.w $ffff
pub fn i_c04ebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC04EBE;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C04EBE: dc.w $ffff
pub fn i_c04ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC04EC0;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C04EC0: dc.w $ffff
pub fn i_c04ec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC04EC2;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C04EC2: dc.w $ffff
pub fn i_c04ec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC04EC4;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C04EC4: dc.w $ffff
pub fn i_c04ec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC04EC6;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C04EC6: dc.w $ffff
pub fn i_c04ec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC04EC8;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C04EC8: dc.w $ffff
pub fn i_c04ec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC04ECA;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C04ECA: dc.w $ffff
pub fn i_c04eca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC04ECC;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06000: dc.w $ffff
pub fn i_c06000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06002;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06002: dc.w $ffff
pub fn i_c06002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06004;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06004: dc.w $ffff
pub fn i_c06004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06006;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06006: dc.w $ffff
pub fn i_c06006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06008;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06008: dc.w $ffff
pub fn i_c06008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0600A;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0600A: dc.w $ffff
pub fn i_c0600a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0600C;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0600C: dc.w $ffff
pub fn i_c0600c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0600E;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0600E: dc.w $ffff
pub fn i_c0600e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06010;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06010: dc.w $ffff
pub fn i_c06010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06012;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06500: dc.w $ffff
pub fn i_c06500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06502;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06502: dc.w $ffff
pub fn i_c06502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06504;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06504: dc.w $ffff
pub fn i_c06504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06506;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06506: dc.w $ffff
pub fn i_c06506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06508;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06508: dc.w $ffff
pub fn i_c06508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0650A;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0650A: dc.w $ffff
pub fn i_c0650a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0650C;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0650C: dc.w $ffff
pub fn i_c0650c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0650E;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0650E: dc.w $ffff
pub fn i_c0650e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06510;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06510: dc.w $ffff
pub fn i_c06510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06512;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06600: dc.w $ffff
pub fn i_c06600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06602;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06602: dc.w $ffff
pub fn i_c06602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06604;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06604: dc.w $ffff
pub fn i_c06604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06606;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06606: dc.w $ffff
pub fn i_c06606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06608;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06608: dc.w $ffff
pub fn i_c06608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0660A;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0660A: dc.w $ffff
pub fn i_c0660a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0660C;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0660C: dc.w $ffff
pub fn i_c0660c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0660E;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0660E: dc.w $ffff
pub fn i_c0660e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06610;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06610: dc.w $ffff
pub fn i_c06610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06612;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06700: dc.w $ffff
pub fn i_c06700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06702;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06702: dc.w $ffff
pub fn i_c06702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06704;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06704: dc.w $ffff
pub fn i_c06704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06706;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06706: dc.w $ffff
pub fn i_c06706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06708;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06708: dc.w $ffff
pub fn i_c06708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0670A;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0670A: dc.w $ffff
pub fn i_c0670a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0670C;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0670C: dc.w $ffff
pub fn i_c0670c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0670E;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0670E: dc.w $ffff
pub fn i_c0670e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06710;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06710: dc.w $ffff
pub fn i_c06710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06712;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06C00: dc.w $ffff
pub fn i_c06c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06C02;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06C02: dc.w $ffff
pub fn i_c06c02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06C04;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06C04: dc.w $ffff
pub fn i_c06c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06C06;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06C06: dc.w $ffff
pub fn i_c06c06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06C08;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06C08: dc.w $ffff
pub fn i_c06c08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06C0A;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06C0A: dc.w $ffff
pub fn i_c06c0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06C0C;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06C0C: dc.w $ffff
pub fn i_c06c0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06C0E;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06C0E: dc.w $ffff
pub fn i_c06c0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06C10;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06C10: dc.w $ffff
pub fn i_c06c10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06C12;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06D00: dc.w $ffff
pub fn i_c06d00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06D02;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06D02: dc.w $ffff
pub fn i_c06d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06D04;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06D04: dc.w $ffff
pub fn i_c06d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06D06;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06D06: dc.w $ffff
pub fn i_c06d06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06D08;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06D08: dc.w $ffff
pub fn i_c06d08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06D0A;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06D0A: dc.w $ffff
pub fn i_c06d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06D0C;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06D0C: dc.w $ffff
pub fn i_c06d0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06D0E;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06D0E: dc.w $ffff
pub fn i_c06d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06D10;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06D10: dc.w $ffff
pub fn i_c06d10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06D12;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06E00: dc.w $ffff
pub fn i_c06e00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06E02;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06E02: dc.w $ffff
pub fn i_c06e02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06E04;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06E04: dc.w $ffff
pub fn i_c06e04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06E06;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06E06: dc.w $ffff
pub fn i_c06e06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06E08;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06E08: dc.w $ffff
pub fn i_c06e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06E0A;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06E0A: dc.w $ffff
pub fn i_c06e0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06E0C;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06E0C: dc.w $ffff
pub fn i_c06e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06E0E;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06E0E: dc.w $ffff
pub fn i_c06e0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06E10;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06E10: dc.w $ffff
pub fn i_c06e10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06E12;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06F00: dc.w $ffff
pub fn i_c06f00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06F02;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06F02: dc.w $ffff
pub fn i_c06f02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06F04;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06F04: dc.w $ffff
pub fn i_c06f04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06F06;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06F06: dc.w $ffff
pub fn i_c06f06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06F08;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06F08: dc.w $ffff
pub fn i_c06f08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06F0A;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06F0A: dc.w $ffff
pub fn i_c06f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06F0C;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06F0C: dc.w $ffff
pub fn i_c06f0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06F0E;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06F0E: dc.w $ffff
pub fn i_c06f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06F10;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C06F10: dc.w $ffff
pub fn i_c06f10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC06F12;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C07000: dc.w $ffff
pub fn i_c07000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC07002;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C07002: dc.w $ffff
pub fn i_c07002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC07004;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C07004: dc.w $ffff
pub fn i_c07004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC07006;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C07006: dc.w $ffff
pub fn i_c07006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC07008;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C07008: dc.w $ffff
pub fn i_c07008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0700A;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0700A: dc.w $ffff
pub fn i_c0700a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0700C;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0700C: dc.w $ffff
pub fn i_c0700c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0700E;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0700E: dc.w $ffff
pub fn i_c0700e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC07010;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C07010: dc.w $ffff
pub fn i_c07010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC07012;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C07204: dc.w $ffff
pub fn i_c07204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC07206;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C07206: dc.w $ffff
pub fn i_c07206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC07208;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C07208: dc.w $ffff
pub fn i_c07208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0720A;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0720A: dc.w $ffff
pub fn i_c0720a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0720C;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0720C: dc.w $ffff
pub fn i_c0720c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0720E;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0720E: dc.w $ffff
pub fn i_c0720e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC07210;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C07210: dc.w $ffff
pub fn i_c07210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC07212;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C07212: dc.w $ffff
pub fn i_c07212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC07214;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C07214: dc.w $ffff
pub fn i_c07214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC07216;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0906E: dc.w $ffff
pub fn i_c0906e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC09070;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C09070: dc.w $ffff
pub fn i_c09070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC09072;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C09072: dc.w $ffff
pub fn i_c09072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC09074;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C09074: dc.w $ffff
pub fn i_c09074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC09076;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C09076: dc.w $ffff
pub fn i_c09076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC09078;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C09078: dc.w $ffff
pub fn i_c09078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0907A;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0907A: dc.w $ffff
pub fn i_c0907a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0907C;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0907C: dc.w $ffff
pub fn i_c0907c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0907E;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0907E: dc.w $ffff
pub fn i_c0907e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC09080;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0B470: dc.w $ffff
pub fn i_c0b470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0B472;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0B472: dc.w $ffff
pub fn i_c0b472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0B474;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0B474: dc.w $ffff
pub fn i_c0b474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0B476;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0B476: dc.w $ffff
pub fn i_c0b476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0B478;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0B478: dc.w $ffff
pub fn i_c0b478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0B47A;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0B47A: dc.w $ffff
pub fn i_c0b47a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0B47C;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0B47C: dc.w $ffff
pub fn i_c0b47c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0B47E;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0B47E: dc.w $ffff
pub fn i_c0b47e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0B480;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0B480: dc.w $ffff
pub fn i_c0b480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0B482;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0FF80: dc.w $ffff
pub fn i_c0ff80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0FF82;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0FF82: dc.w $ffff
pub fn i_c0ff82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0FF84;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0FF84: dc.w $ffff
pub fn i_c0ff84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0FF86;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0FF86: dc.w $ffff
pub fn i_c0ff86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0FF88;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0FF88: dc.w $ffff
pub fn i_c0ff88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0FF8A;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0FF8A: dc.w $ffff
pub fn i_c0ff8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0FF8C;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0FF8C: dc.w $ffff
pub fn i_c0ff8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0FF8E;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0FF8E: dc.w $ffff
pub fn i_c0ff8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0FF90;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C0FF90: dc.w $ffff
pub fn i_c0ff90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC0FF92;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C10000: ori.b #$fc, d0
pub fn i_c10000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10004;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0xFC) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
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
/// $C10004: move.l d0, -(a3)
pub fn i_c10004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10006;
    cpu.ir = 0x2700;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let o2 = cpu.a[3];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[3] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
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
/// $C10012: ?
pub fn i_c10012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10016;
    cpu.ir = 0xC;
    let v1 = cpu.a[4] & 0xFF;
    let r2 = (v1 | 0xFC) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.a[4] = r2;
    8
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
/// $C10016: negx.b d0
pub fn i_c10016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10018;
    cpu.ir = 0x4000;
    let v1 = cpu.d[0] & 0xFF;
    let x = cpu.sr.get(StatusRegister::X) as u32; let r2 = 0u32.wrapping_sub(v1).wrapping_sub(x) & 0xFF;
    let bw = v1 != 0 || x != 0; let vf = (v1 & 0x80) != 0 && (r2 & 0x80) != 0;
    cpu.sr.set(StatusRegister::N, r2 & 0x80 != 0); if r2 != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C10018: ori.b #$6, ccr
pub fn i_c10018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1001C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x6);
    20
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
/// $C1007E: move.l d0, d0
pub fn i_c1007e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10080;
    cpu.ir = 0x2000;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
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
/// $C10168: ?
pub fn i_c10168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1016A;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1016A: bchg.b d0, -$3f(a4, d0.w)
pub fn i_c1016a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1016E;
    cpu.ir = 0x174;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 8);
    let ad1 = cpu.a[4].wrapping_add(0xFFFFFFC1).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 ^ mk;
    bus.write8(ad1, r3 as u8);
    18
}
/// $C1016E: bclr.b d0, -(a0)
pub fn i_c1016e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10170;
    cpu.ir = 0x1A0;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 8);
    let ad1 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad1;
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    14
}
/// $C10170: ?
pub fn i_c10170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10172;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C10172: bset.b d0, $1b7c(a2)
pub fn i_c10172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC10176;
    cpu.ir = 0x1EA;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 8);
    let ad1 = cpu.a[2].wrapping_add(0x1B7C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
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
/// $C10176: ori.b #$bf, d1
pub fn i_c10176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1017A;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0xBF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
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
/// $C101DC: ori.b #$2c, d3
pub fn i_c101dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC101E0;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x2C) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C101E0: movep.w $534c(a0), d0
pub fn i_c101e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC101E4;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x534C);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C101E4: ?
pub fn i_c101e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC101E6;
    cpu.ir = 0x4F54;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C101E6: ?
pub fn i_c101e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC101E8;
    cpu.ir = 0x20FF;
    let v1: u32 = 0x0;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C101E8: ori.b #$79, d0
pub fn i_c101e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC101EC;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x79) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0xC02B18, i_c02b18),
    (0xC02B1C, i_c02b1c),
    (0xC02B20, i_c02b20),
    (0xC02B22, i_c02b22),
    (0xC02B24, i_c02b24),
    (0xC02B26, i_c02b26),
    (0xC02B28, i_c02b28),
    (0xC02B2A, i_c02b2a),
    (0xC02B2C, i_c02b2c),
    (0xC02B2E, i_c02b2e),
    (0xC02B30, i_c02b30),
    (0xC02B32, i_c02b32),
    (0xC02B34, i_c02b34),
    (0xC02B36, i_c02b36),
    (0xC02B38, i_c02b38),
    (0xC02B3A, i_c02b3a),
    (0xC02B3C, i_c02b3c),
    (0xC02B3E, i_c02b3e),
    (0xC02B40, i_c02b40),
    (0xC02B42, i_c02b42),
    (0xC02B44, i_c02b44),
    (0xC02B48, i_c02b48),
    (0xC02B4C, i_c02b4c),
    (0xC02B50, i_c02b50),
    (0xC02B54, i_c02b54),
    (0xC02B58, i_c02b58),
    (0xC02B5C, i_c02b5c),
    (0xC02B60, i_c02b60),
    (0xC02B62, i_c02b62),
    (0xC02B64, i_c02b64),
    (0xC02B66, i_c02b66),
    (0xC02B68, i_c02b68),
    (0xC02B6A, i_c02b6a),
    (0xC02B6C, i_c02b6c),
    (0xC02B6E, i_c02b6e),
    (0xC02B70, i_c02b70),
    (0xC02B72, i_c02b72),
    (0xC02B74, i_c02b74),
    (0xC02B76, i_c02b76),
    (0xC02B78, i_c02b78),
    (0xC02B7A, i_c02b7a),
    (0xC02B7C, i_c02b7c),
    (0xC02B7E, i_c02b7e),
    (0xC02B80, i_c02b80),
    (0xC02B82, i_c02b82),
    (0xC02B84, i_c02b84),
    (0xC02B88, i_c02b88),
    (0xC02B8C, i_c02b8c),
    (0xC02B90, i_c02b90),
    (0xC02B94, i_c02b94),
    (0xC02B98, i_c02b98),
    (0xC02B9C, i_c02b9c),
    (0xC02BA0, i_c02ba0),
    (0xC02BA2, i_c02ba2),
    (0xC02BA4, i_c02ba4),
    (0xC02BA6, i_c02ba6),
    (0xC02BA8, i_c02ba8),
    (0xC02BAA, i_c02baa),
    (0xC02BAC, i_c02bac),
    (0xC02BAE, i_c02bae),
    (0xC02BB0, i_c02bb0),
    (0xC02BB2, i_c02bb2),
    (0xC02BB4, i_c02bb4),
    (0xC02BB6, i_c02bb6),
    (0xC02BB8, i_c02bb8),
    (0xC02BBA, i_c02bba),
    (0xC02BBC, i_c02bbc),
    (0xC02BBE, i_c02bbe),
    (0xC02BC0, i_c02bc0),
    (0xC02BC2, i_c02bc2),
    (0xC02BC4, i_c02bc4),
    (0xC02BC8, i_c02bc8),
    (0xC02BCC, i_c02bcc),
    (0xC02BD0, i_c02bd0),
    (0xC02BD4, i_c02bd4),
    (0xC02BD8, i_c02bd8),
    (0xC02BDC, i_c02bdc),
    (0xC02BE0, i_c02be0),
    (0xC02BE2, i_c02be2),
    (0xC02BE4, i_c02be4),
    (0xC02BE6, i_c02be6),
    (0xC02BE8, i_c02be8),
    (0xC02BEA, i_c02bea),
    (0xC02BEC, i_c02bec),
    (0xC02BEE, i_c02bee),
    (0xC02BF0, i_c02bf0),
    (0xC02BF2, i_c02bf2),
    (0xC02BF4, i_c02bf4),
    (0xC02BF6, i_c02bf6),
    (0xC02BF8, i_c02bf8),
    (0xC02BFA, i_c02bfa),
    (0xC02BFC, i_c02bfc),
    (0xC02BFE, i_c02bfe),
    (0xC02C00, i_c02c00),
    (0xC02C02, i_c02c02),
    (0xC02C04, i_c02c04),
    (0xC02C08, i_c02c08),
    (0xC02C0C, i_c02c0c),
    (0xC02C10, i_c02c10),
    (0xC02C14, i_c02c14),
    (0xC02C18, i_c02c18),
    (0xC02C1C, i_c02c1c),
    (0xC02C20, i_c02c20),
    (0xC02C22, i_c02c22),
    (0xC02C24, i_c02c24),
    (0xC02C26, i_c02c26),
    (0xC02C28, i_c02c28),
    (0xC02C2A, i_c02c2a),
    (0xC02C2C, i_c02c2c),
    (0xC02C2E, i_c02c2e),
    (0xC02C30, i_c02c30),
    (0xC02C32, i_c02c32),
    (0xC02C34, i_c02c34),
    (0xC02C36, i_c02c36),
    (0xC02C38, i_c02c38),
    (0xC02C3A, i_c02c3a),
    (0xC02C3C, i_c02c3c),
    (0xC02C3E, i_c02c3e),
    (0xC02C40, i_c02c40),
    (0xC02C42, i_c02c42),
    (0xC02C44, i_c02c44),
    (0xC02C48, i_c02c48),
    (0xC02C4E, i_c02c4e),
    (0xC02C52, i_c02c52),
    (0xC02C56, i_c02c56),
    (0xC02C5A, i_c02c5a),
    (0xC02C5E, i_c02c5e),
    (0xC02C62, i_c02c62),
    (0xC02C64, i_c02c64),
    (0xC02C66, i_c02c66),
    (0xC02C68, i_c02c68),
    (0xC02C6A, i_c02c6a),
    (0xC02C6C, i_c02c6c),
    (0xC02C6E, i_c02c6e),
    (0xC02C70, i_c02c70),
    (0xC02C72, i_c02c72),
    (0xC02C74, i_c02c74),
    (0xC02C76, i_c02c76),
    (0xC02C78, i_c02c78),
    (0xC02C7A, i_c02c7a),
    (0xC02C7C, i_c02c7c),
    (0xC02C7E, i_c02c7e),
    (0xC02C80, i_c02c80),
    (0xC02C82, i_c02c82),
    (0xC02C84, i_c02c84),
    (0xC02C88, i_c02c88),
    (0xC02C8E, i_c02c8e),
    (0xC02C92, i_c02c92),
    (0xC02C96, i_c02c96),
    (0xC02C9A, i_c02c9a),
    (0xC02C9E, i_c02c9e),
    (0xC02CA2, i_c02ca2),
    (0xC02CA4, i_c02ca4),
    (0xC02CA6, i_c02ca6),
    (0xC02CA8, i_c02ca8),
    (0xC02CAA, i_c02caa),
    (0xC02CAC, i_c02cac),
    (0xC02CAE, i_c02cae),
    (0xC02CB0, i_c02cb0),
    (0xC02CB2, i_c02cb2),
    (0xC02CB4, i_c02cb4),
    (0xC02CB6, i_c02cb6),
    (0xC02CB8, i_c02cb8),
    (0xC02CBA, i_c02cba),
    (0xC02CBC, i_c02cbc),
    (0xC02CBE, i_c02cbe),
    (0xC02CC0, i_c02cc0),
    (0xC02CC2, i_c02cc2),
    (0xC02CC4, i_c02cc4),
    (0xC02CC8, i_c02cc8),
    (0xC02CCE, i_c02cce),
    (0xC02CD2, i_c02cd2),
    (0xC02CD6, i_c02cd6),
    (0xC02CDA, i_c02cda),
    (0xC02CDE, i_c02cde),
    (0xC02CE2, i_c02ce2),
    (0xC02CE4, i_c02ce4),
    (0xC02CE6, i_c02ce6),
    (0xC02CE8, i_c02ce8),
    (0xC02CEA, i_c02cea),
    (0xC02CEC, i_c02cec),
    (0xC02CEE, i_c02cee),
    (0xC02CF0, i_c02cf0),
    (0xC02CF2, i_c02cf2),
    (0xC02CF4, i_c02cf4),
    (0xC02CF6, i_c02cf6),
    (0xC02CF8, i_c02cf8),
    (0xC02CFA, i_c02cfa),
    (0xC02CFC, i_c02cfc),
    (0xC02CFE, i_c02cfe),
    (0xC02D00, i_c02d00),
    (0xC02D02, i_c02d02),
    (0xC02D04, i_c02d04),
    (0xC02D08, i_c02d08),
    (0xC02D0E, i_c02d0e),
    (0xC02D12, i_c02d12),
    (0xC02D16, i_c02d16),
    (0xC02D1A, i_c02d1a),
    (0xC02D1E, i_c02d1e),
    (0xC02D22, i_c02d22),
    (0xC02D26, i_c02d26),
    (0xC02D28, i_c02d28),
    (0xC02D2A, i_c02d2a),
    (0xC02D2C, i_c02d2c),
    (0xC02D2E, i_c02d2e),
    (0xC02D30, i_c02d30),
    (0xC02D32, i_c02d32),
    (0xC02D34, i_c02d34),
    (0xC02D36, i_c02d36),
    (0xC02D38, i_c02d38),
    (0xC02D3A, i_c02d3a),
    (0xC02D3C, i_c02d3c),
    (0xC02D3E, i_c02d3e),
    (0xC02D40, i_c02d40),
    (0xC02D42, i_c02d42),
    (0xC02D44, i_c02d44),
    (0xC02D48, i_c02d48),
    (0xC02D4E, i_c02d4e),
    (0xC02D52, i_c02d52),
    (0xC02D56, i_c02d56),
    (0xC02D5A, i_c02d5a),
    (0xC02D5E, i_c02d5e),
    (0xC02D62, i_c02d62),
    (0xC02D66, i_c02d66),
    (0xC02D68, i_c02d68),
    (0xC02D6A, i_c02d6a),
    (0xC02D6C, i_c02d6c),
    (0xC02D6E, i_c02d6e),
    (0xC02D70, i_c02d70),
    (0xC02D72, i_c02d72),
    (0xC02D74, i_c02d74),
    (0xC02D76, i_c02d76),
    (0xC02D78, i_c02d78),
    (0xC02D7A, i_c02d7a),
    (0xC02D7C, i_c02d7c),
    (0xC02D7E, i_c02d7e),
    (0xC02D80, i_c02d80),
    (0xC02D82, i_c02d82),
    (0xC02D84, i_c02d84),
    (0xC02D88, i_c02d88),
    (0xC02D8E, i_c02d8e),
    (0xC02D92, i_c02d92),
    (0xC02D96, i_c02d96),
    (0xC02D9A, i_c02d9a),
    (0xC02D9E, i_c02d9e),
    (0xC02DA2, i_c02da2),
    (0xC02DA6, i_c02da6),
    (0xC02DA8, i_c02da8),
    (0xC02DAA, i_c02daa),
    (0xC02DAC, i_c02dac),
    (0xC02DAE, i_c02dae),
    (0xC02DB0, i_c02db0),
    (0xC02DB2, i_c02db2),
    (0xC02DB4, i_c02db4),
    (0xC02DB6, i_c02db6),
    (0xC02DB8, i_c02db8),
    (0xC02DBA, i_c02dba),
    (0xC02DBC, i_c02dbc),
    (0xC02DBE, i_c02dbe),
    (0xC02DC0, i_c02dc0),
    (0xC02DC2, i_c02dc2),
    (0xC02DC4, i_c02dc4),
    (0xC02DC8, i_c02dc8),
    (0xC02DCE, i_c02dce),
    (0xC02DD2, i_c02dd2),
    (0xC02DD6, i_c02dd6),
    (0xC02DDA, i_c02dda),
    (0xC02DDE, i_c02dde),
    (0xC02DE2, i_c02de2),
    (0xC02DE6, i_c02de6),
    (0xC02DE8, i_c02de8),
    (0xC02DEA, i_c02dea),
    (0xC02DEC, i_c02dec),
    (0xC02DEE, i_c02dee),
    (0xC02DF0, i_c02df0),
    (0xC02DF2, i_c02df2),
    (0xC02DF4, i_c02df4),
    (0xC02DF6, i_c02df6),
    (0xC02DF8, i_c02df8),
    (0xC02DFA, i_c02dfa),
    (0xC02DFC, i_c02dfc),
    (0xC02DFE, i_c02dfe),
    (0xC02E00, i_c02e00),
    (0xC02E02, i_c02e02),
    (0xC02E04, i_c02e04),
    (0xC02E08, i_c02e08),
    (0xC02E0C, i_c02e0c),
    (0xC02E10, i_c02e10),
    (0xC02E14, i_c02e14),
    (0xC02E18, i_c02e18),
    (0xC02E1C, i_c02e1c),
    (0xC02E20, i_c02e20),
    (0xC02E24, i_c02e24),
    (0xC02E28, i_c02e28),
    (0xC02E2A, i_c02e2a),
    (0xC02E2C, i_c02e2c),
    (0xC02E2E, i_c02e2e),
    (0xC02E30, i_c02e30),
    (0xC02E32, i_c02e32),
    (0xC02E34, i_c02e34),
    (0xC02E36, i_c02e36),
    (0xC02E38, i_c02e38),
    (0xC02E3A, i_c02e3a),
    (0xC02E3C, i_c02e3c),
    (0xC02E3E, i_c02e3e),
    (0xC02E40, i_c02e40),
    (0xC02E42, i_c02e42),
    (0xC02E44, i_c02e44),
    (0xC02E48, i_c02e48),
    (0xC02E4C, i_c02e4c),
    (0xC02E50, i_c02e50),
    (0xC02E54, i_c02e54),
    (0xC02E58, i_c02e58),
    (0xC02E5C, i_c02e5c),
    (0xC02E60, i_c02e60),
    (0xC02E64, i_c02e64),
    (0xC02E68, i_c02e68),
    (0xC02E6A, i_c02e6a),
    (0xC02E6C, i_c02e6c),
    (0xC02E6E, i_c02e6e),
    (0xC02E70, i_c02e70),
    (0xC02E72, i_c02e72),
    (0xC02E74, i_c02e74),
    (0xC02E76, i_c02e76),
    (0xC02E78, i_c02e78),
    (0xC02E7A, i_c02e7a),
    (0xC02E7C, i_c02e7c),
    (0xC02E7E, i_c02e7e),
    (0xC02E80, i_c02e80),
    (0xC02E82, i_c02e82),
    (0xC02E84, i_c02e84),
    (0xC02E88, i_c02e88),
    (0xC02E8C, i_c02e8c),
    (0xC02E90, i_c02e90),
    (0xC02E94, i_c02e94),
    (0xC02E98, i_c02e98),
    (0xC02E9C, i_c02e9c),
    (0xC02EA0, i_c02ea0),
    (0xC02EA4, i_c02ea4),
    (0xC02EA8, i_c02ea8),
    (0xC02EAA, i_c02eaa),
    (0xC02EAC, i_c02eac),
    (0xC02EAE, i_c02eae),
    (0xC02EB0, i_c02eb0),
    (0xC02EB2, i_c02eb2),
    (0xC02EB4, i_c02eb4),
    (0xC02EB6, i_c02eb6),
    (0xC02EB8, i_c02eb8),
    (0xC02EBA, i_c02eba),
    (0xC02EBC, i_c02ebc),
    (0xC02EBE, i_c02ebe),
    (0xC02EC0, i_c02ec0),
    (0xC02EC2, i_c02ec2),
    (0xC02EC4, i_c02ec4),
    (0xC02EC6, i_c02ec6),
    (0xC02ECA, i_c02eca),
    (0xC02ECE, i_c02ece),
    (0xC02ED2, i_c02ed2),
    (0xC02ED6, i_c02ed6),
    (0xC02EDA, i_c02eda),
    (0xC02EDE, i_c02ede),
    (0xC02EE2, i_c02ee2),
    (0xC02EE6, i_c02ee6),
    (0xC02EEA, i_c02eea),
    (0xC02EEC, i_c02eec),
    (0xC02EEE, i_c02eee),
    (0xC02EF0, i_c02ef0),
    (0xC02EF2, i_c02ef2),
    (0xC02EF4, i_c02ef4),
    (0xC02EF6, i_c02ef6),
    (0xC02EF8, i_c02ef8),
    (0xC02EFA, i_c02efa),
    (0xC02EFC, i_c02efc),
    (0xC02EFE, i_c02efe),
    (0xC02F00, i_c02f00),
    (0xC02F02, i_c02f02),
    (0xC02F04, i_c02f04),
    (0xC02F06, i_c02f06),
    (0xC02F0A, i_c02f0a),
    (0xC02F10, i_c02f10),
    (0xC02F14, i_c02f14),
    (0xC02F18, i_c02f18),
    (0xC02F1C, i_c02f1c),
    (0xC02F20, i_c02f20),
    (0xC02F24, i_c02f24),
    (0xC02F28, i_c02f28),
    (0xC02F2C, i_c02f2c),
    (0xC02F2E, i_c02f2e),
    (0xC02F30, i_c02f30),
    (0xC02F32, i_c02f32),
    (0xC02F34, i_c02f34),
    (0xC02F36, i_c02f36),
    (0xC02F38, i_c02f38),
    (0xC02F3A, i_c02f3a),
    (0xC02F3C, i_c02f3c),
    (0xC02F3E, i_c02f3e),
    (0xC02F40, i_c02f40),
    (0xC02F42, i_c02f42),
    (0xC02F44, i_c02f44),
    (0xC02F46, i_c02f46),
    (0xC02F4A, i_c02f4a),
    (0xC02F50, i_c02f50),
    (0xC02F54, i_c02f54),
    (0xC02F58, i_c02f58),
    (0xC02F5C, i_c02f5c),
    (0xC02F60, i_c02f60),
    (0xC02F64, i_c02f64),
    (0xC02F68, i_c02f68),
    (0xC02F6C, i_c02f6c),
    (0xC02F6E, i_c02f6e),
    (0xC02F70, i_c02f70),
    (0xC02F72, i_c02f72),
    (0xC02F74, i_c02f74),
    (0xC02F76, i_c02f76),
    (0xC02F78, i_c02f78),
    (0xC02F7A, i_c02f7a),
    (0xC02F7C, i_c02f7c),
    (0xC02F7E, i_c02f7e),
    (0xC02F80, i_c02f80),
    (0xC02F82, i_c02f82),
    (0xC02F84, i_c02f84),
    (0xC02F86, i_c02f86),
    (0xC02F8A, i_c02f8a),
    (0xC02F90, i_c02f90),
    (0xC02F94, i_c02f94),
    (0xC02F98, i_c02f98),
    (0xC02F9C, i_c02f9c),
    (0xC02FA0, i_c02fa0),
    (0xC02FA4, i_c02fa4),
    (0xC02FA8, i_c02fa8),
    (0xC02FAC, i_c02fac),
    (0xC02FAE, i_c02fae),
    (0xC02FB0, i_c02fb0),
    (0xC02FB2, i_c02fb2),
    (0xC02FB4, i_c02fb4),
    (0xC02FB6, i_c02fb6),
    (0xC02FB8, i_c02fb8),
    (0xC02FBA, i_c02fba),
    (0xC02FBC, i_c02fbc),
    (0xC02FBE, i_c02fbe),
    (0xC02FC0, i_c02fc0),
    (0xC02FC2, i_c02fc2),
    (0xC02FC4, i_c02fc4),
    (0xC02FC6, i_c02fc6),
    (0xC02FCA, i_c02fca),
    (0xC02FD0, i_c02fd0),
    (0xC02FD4, i_c02fd4),
    (0xC02FD8, i_c02fd8),
    (0xC02FDC, i_c02fdc),
    (0xC02FE0, i_c02fe0),
    (0xC02FE4, i_c02fe4),
    (0xC02FE8, i_c02fe8),
    (0xC02FEC, i_c02fec),
    (0xC02FEE, i_c02fee),
    (0xC02FF0, i_c02ff0),
    (0xC02FF2, i_c02ff2),
    (0xC02FF4, i_c02ff4),
    (0xC02FF6, i_c02ff6),
    (0xC02FF8, i_c02ff8),
    (0xC02FFA, i_c02ffa),
    (0xC02FFC, i_c02ffc),
    (0xC02FFE, i_c02ffe),
    (0xC03000, i_c03000),
    (0xC03004, i_c03004),
    (0xC03008, i_c03008),
    (0xC0300C, i_c0300c),
    (0xC03010, i_c03010),
    (0xC03012, i_c03012),
    (0xC03014, i_c03014),
    (0xC03016, i_c03016),
    (0xC03018, i_c03018),
    (0xC0301A, i_c0301a),
    (0xC0301C, i_c0301c),
    (0xC0301E, i_c0301e),
    (0xC03020, i_c03020),
    (0xC03022, i_c03022),
    (0xC03024, i_c03024),
    (0xC03026, i_c03026),
    (0xC03028, i_c03028),
    (0xC0302C, i_c0302c),
    (0xC03030, i_c03030),
    (0xC03034, i_c03034),
    (0xC03038, i_c03038),
    (0xC0303C, i_c0303c),
    (0xC0303E, i_c0303e),
    (0xC03040, i_c03040),
    (0xC03042, i_c03042),
    (0xC03046, i_c03046),
    (0xC0304A, i_c0304a),
    (0xC0304E, i_c0304e),
    (0xC03052, i_c03052),
    (0xC03054, i_c03054),
    (0xC03056, i_c03056),
    (0xC03058, i_c03058),
    (0xC0305A, i_c0305a),
    (0xC0305C, i_c0305c),
    (0xC0305E, i_c0305e),
    (0xC03060, i_c03060),
    (0xC03062, i_c03062),
    (0xC03064, i_c03064),
    (0xC03066, i_c03066),
    (0xC03068, i_c03068),
    (0xC0306C, i_c0306c),
    (0xC03070, i_c03070),
    (0xC03074, i_c03074),
    (0xC03078, i_c03078),
    (0xC0307E, i_c0307e),
    (0xC03080, i_c03080),
    (0xC03084, i_c03084),
    (0xC03088, i_c03088),
    (0xC0308C, i_c0308c),
    (0xC03090, i_c03090),
    (0xC03092, i_c03092),
    (0xC03094, i_c03094),
    (0xC03096, i_c03096),
    (0xC03098, i_c03098),
    (0xC0309A, i_c0309a),
    (0xC0309C, i_c0309c),
    (0xC0309E, i_c0309e),
    (0xC030A0, i_c030a0),
    (0xC030A2, i_c030a2),
    (0xC030A4, i_c030a4),
    (0xC030A6, i_c030a6),
    (0xC030A8, i_c030a8),
    (0xC030AC, i_c030ac),
    (0xC030B0, i_c030b0),
    (0xC030B4, i_c030b4),
    (0xC030B8, i_c030b8),
    (0xC030BE, i_c030be),
    (0xC030C0, i_c030c0),
    (0xC030C2, i_c030c2),
    (0xC030C6, i_c030c6),
    (0xC030CA, i_c030ca),
    (0xC030CE, i_c030ce),
    (0xC030D2, i_c030d2),
    (0xC030D4, i_c030d4),
    (0xC030D6, i_c030d6),
    (0xC030D8, i_c030d8),
    (0xC030DA, i_c030da),
    (0xC030DC, i_c030dc),
    (0xC030DE, i_c030de),
    (0xC030E0, i_c030e0),
    (0xC030E2, i_c030e2),
    (0xC030E4, i_c030e4),
    (0xC030E6, i_c030e6),
    (0xC030E8, i_c030e8),
    (0xC030EC, i_c030ec),
    (0xC030F0, i_c030f0),
    (0xC030F4, i_c030f4),
    (0xC030F8, i_c030f8),
    (0xC030FE, i_c030fe),
    (0xC03100, i_c03100),
    (0xC03104, i_c03104),
    (0xC03108, i_c03108),
    (0xC0310C, i_c0310c),
    (0xC03110, i_c03110),
    (0xC03112, i_c03112),
    (0xC03114, i_c03114),
    (0xC03116, i_c03116),
    (0xC03118, i_c03118),
    (0xC0311A, i_c0311a),
    (0xC0311C, i_c0311c),
    (0xC0311E, i_c0311e),
    (0xC03120, i_c03120),
    (0xC03122, i_c03122),
    (0xC03124, i_c03124),
    (0xC03126, i_c03126),
    (0xC03128, i_c03128),
    (0xC0312C, i_c0312c),
    (0xC03130, i_c03130),
    (0xC03134, i_c03134),
    (0xC03138, i_c03138),
    (0xC0313E, i_c0313e),
    (0xC03140, i_c03140),
    (0xC03142, i_c03142),
    (0xC03144, i_c03144),
    (0xC03148, i_c03148),
    (0xC0314C, i_c0314c),
    (0xC03150, i_c03150),
    (0xC03152, i_c03152),
    (0xC03154, i_c03154),
    (0xC03156, i_c03156),
    (0xC03158, i_c03158),
    (0xC0315A, i_c0315a),
    (0xC0315C, i_c0315c),
    (0xC0315E, i_c0315e),
    (0xC03160, i_c03160),
    (0xC03162, i_c03162),
    (0xC03164, i_c03164),
    (0xC03166, i_c03166),
    (0xC03168, i_c03168),
    (0xC0316C, i_c0316c),
    (0xC03170, i_c03170),
    (0xC03174, i_c03174),
    (0xC03178, i_c03178),
    (0xC0317E, i_c0317e),
    (0xC03180, i_c03180),
    (0xC03184, i_c03184),
    (0xC03188, i_c03188),
    (0xC0318C, i_c0318c),
    (0xC03190, i_c03190),
    (0xC03192, i_c03192),
    (0xC03194, i_c03194),
    (0xC03196, i_c03196),
    (0xC03198, i_c03198),
    (0xC0319A, i_c0319a),
    (0xC0319C, i_c0319c),
    (0xC0319E, i_c0319e),
    (0xC031A0, i_c031a0),
    (0xC031A2, i_c031a2),
    (0xC031A4, i_c031a4),
    (0xC031A6, i_c031a6),
    (0xC031A8, i_c031a8),
    (0xC031AC, i_c031ac),
    (0xC031B0, i_c031b0),
    (0xC031B4, i_c031b4),
    (0xC031B8, i_c031b8),
    (0xC031BE, i_c031be),
    (0xC031C0, i_c031c0),
    (0xC031C2, i_c031c2),
    (0xC031C6, i_c031c6),
    (0xC031CA, i_c031ca),
    (0xC031CE, i_c031ce),
    (0xC031D0, i_c031d0),
    (0xC031D2, i_c031d2),
    (0xC031D4, i_c031d4),
    (0xC031D6, i_c031d6),
    (0xC031D8, i_c031d8),
    (0xC031DA, i_c031da),
    (0xC031DC, i_c031dc),
    (0xC031DE, i_c031de),
    (0xC031E0, i_c031e0),
    (0xC031E2, i_c031e2),
    (0xC031E4, i_c031e4),
    (0xC031E6, i_c031e6),
    (0xC031EA, i_c031ea),
    (0xC031EE, i_c031ee),
    (0xC031F2, i_c031f2),
    (0xC031F6, i_c031f6),
    (0xC031FA, i_c031fa),
    (0xC031FE, i_c031fe),
    (0xC03200, i_c03200),
    (0xC03204, i_c03204),
    (0xC03208, i_c03208),
    (0xC0320C, i_c0320c),
    (0xC03210, i_c03210),
    (0xC03212, i_c03212),
    (0xC03214, i_c03214),
    (0xC03216, i_c03216),
    (0xC03218, i_c03218),
    (0xC0321A, i_c0321a),
    (0xC0321C, i_c0321c),
    (0xC0321E, i_c0321e),
    (0xC03220, i_c03220),
    (0xC03222, i_c03222),
    (0xC03224, i_c03224),
    (0xC03226, i_c03226),
    (0xC0322A, i_c0322a),
    (0xC0322E, i_c0322e),
    (0xC03232, i_c03232),
    (0xC03236, i_c03236),
    (0xC0323A, i_c0323a),
    (0xC0323C, i_c0323c),
    (0xC0323E, i_c0323e),
    (0xC03242, i_c03242),
    (0xC03246, i_c03246),
    (0xC0324A, i_c0324a),
    (0xC0324E, i_c0324e),
    (0xC03250, i_c03250),
    (0xC03252, i_c03252),
    (0xC03254, i_c03254),
    (0xC03256, i_c03256),
    (0xC03258, i_c03258),
    (0xC0325A, i_c0325a),
    (0xC0325C, i_c0325c),
    (0xC0325E, i_c0325e),
    (0xC03260, i_c03260),
    (0xC03262, i_c03262),
    (0xC03264, i_c03264),
    (0xC03266, i_c03266),
    (0xC0326A, i_c0326a),
    (0xC0326E, i_c0326e),
    (0xC03272, i_c03272),
    (0xC03276, i_c03276),
    (0xC0327A, i_c0327a),
    (0xC0327E, i_c0327e),
    (0xC03282, i_c03282),
    (0xC03286, i_c03286),
    (0xC0328A, i_c0328a),
    (0xC0328E, i_c0328e),
    (0xC03290, i_c03290),
    (0xC03292, i_c03292),
    (0xC03294, i_c03294),
    (0xC03296, i_c03296),
    (0xC03298, i_c03298),
    (0xC0329A, i_c0329a),
    (0xC0329C, i_c0329c),
    (0xC0329E, i_c0329e),
    (0xC032A0, i_c032a0),
    (0xC032A2, i_c032a2),
    (0xC032A4, i_c032a4),
    (0xC032A6, i_c032a6),
    (0xC032AA, i_c032aa),
    (0xC032AE, i_c032ae),
    (0xC032B2, i_c032b2),
    (0xC032B6, i_c032b6),
    (0xC032BA, i_c032ba),
    (0xC032BE, i_c032be),
    (0xC032C2, i_c032c2),
    (0xC032C6, i_c032c6),
    (0xC032CA, i_c032ca),
    (0xC032CC, i_c032cc),
    (0xC032CE, i_c032ce),
    (0xC032D0, i_c032d0),
    (0xC032D2, i_c032d2),
    (0xC032D4, i_c032d4),
    (0xC032D6, i_c032d6),
    (0xC032D8, i_c032d8),
    (0xC032DA, i_c032da),
    (0xC032DC, i_c032dc),
    (0xC032DE, i_c032de),
    (0xC032E0, i_c032e0),
    (0xC032E2, i_c032e2),
    (0xC032E4, i_c032e4),
    (0xC032E6, i_c032e6),
    (0xC032EA, i_c032ea),
    (0xC032EE, i_c032ee),
    (0xC032F2, i_c032f2),
    (0xC032F6, i_c032f6),
    (0xC032FA, i_c032fa),
    (0xC032FE, i_c032fe),
    (0xC03302, i_c03302),
    (0xC03306, i_c03306),
    (0xC0330A, i_c0330a),
    (0xC0330C, i_c0330c),
    (0xC0330E, i_c0330e),
    (0xC03310, i_c03310),
    (0xC03312, i_c03312),
    (0xC03314, i_c03314),
    (0xC03316, i_c03316),
    (0xC03318, i_c03318),
    (0xC0331A, i_c0331a),
    (0xC0331C, i_c0331c),
    (0xC0331E, i_c0331e),
    (0xC03320, i_c03320),
    (0xC03322, i_c03322),
    (0xC03324, i_c03324),
    (0xC03326, i_c03326),
    (0xC0332A, i_c0332a),
    (0xC0332E, i_c0332e),
    (0xC03332, i_c03332),
    (0xC03336, i_c03336),
    (0xC0333A, i_c0333a),
    (0xC0333E, i_c0333e),
    (0xC03342, i_c03342),
    (0xC03346, i_c03346),
    (0xC0334A, i_c0334a),
    (0xC0334C, i_c0334c),
    (0xC0334E, i_c0334e),
    (0xC03350, i_c03350),
    (0xC03352, i_c03352),
    (0xC03354, i_c03354),
    (0xC03356, i_c03356),
    (0xC03358, i_c03358),
    (0xC0335A, i_c0335a),
    (0xC0335C, i_c0335c),
    (0xC0335E, i_c0335e),
    (0xC03360, i_c03360),
    (0xC03362, i_c03362),
    (0xC03364, i_c03364),
    (0xC03366, i_c03366),
    (0xC0336A, i_c0336a),
    (0xC0336E, i_c0336e),
    (0xC03372, i_c03372),
    (0xC03376, i_c03376),
    (0xC0337C, i_c0337c),
    (0xC0337E, i_c0337e),
    (0xC03382, i_c03382),
    (0xC03386, i_c03386),
    (0xC03388, i_c03388),
    (0xC0338A, i_c0338a),
    (0xC0338C, i_c0338c),
    (0xC0338E, i_c0338e),
    (0xC03390, i_c03390),
    (0xC03392, i_c03392),
    (0xC03394, i_c03394),
    (0xC03396, i_c03396),
    (0xC03398, i_c03398),
    (0xC0339A, i_c0339a),
    (0xC0339C, i_c0339c),
    (0xC0339E, i_c0339e),
    (0xC033A0, i_c033a0),
    (0xC033A2, i_c033a2),
    (0xC033A4, i_c033a4),
    (0xC033A8, i_c033a8),
    (0xC033AC, i_c033ac),
    (0xC033B0, i_c033b0),
    (0xC033B4, i_c033b4),
    (0xC033B8, i_c033b8),
    (0xC033BC, i_c033bc),
    (0xC033BE, i_c033be),
    (0xC033C0, i_c033c0),
    (0xC033C4, i_c033c4),
    (0xC033C6, i_c033c6),
    (0xC033C8, i_c033c8),
    (0xC033CA, i_c033ca),
    (0xC033CC, i_c033cc),
    (0xC033CE, i_c033ce),
    (0xC033D0, i_c033d0),
    (0xC033D2, i_c033d2),
    (0xC033D4, i_c033d4),
    (0xC033D6, i_c033d6),
    (0xC033D8, i_c033d8),
    (0xC033DA, i_c033da),
    (0xC033DC, i_c033dc),
    (0xC033DE, i_c033de),
    (0xC033E0, i_c033e0),
    (0xC033E2, i_c033e2),
    (0xC033E4, i_c033e4),
    (0xC033E8, i_c033e8),
    (0xC033EC, i_c033ec),
    (0xC033F0, i_c033f0),
    (0xC033F4, i_c033f4),
    (0xC033F8, i_c033f8),
    (0xC033FC, i_c033fc),
    (0xC033FE, i_c033fe),
    (0xC03400, i_c03400),
    (0xC03402, i_c03402),
    (0xC03404, i_c03404),
    (0xC03406, i_c03406),
    (0xC03408, i_c03408),
    (0xC0340A, i_c0340a),
    (0xC0340C, i_c0340c),
    (0xC0340E, i_c0340e),
    (0xC03410, i_c03410),
    (0xC03412, i_c03412),
    (0xC03414, i_c03414),
    (0xC03416, i_c03416),
    (0xC03418, i_c03418),
    (0xC0341A, i_c0341a),
    (0xC0341C, i_c0341c),
    (0xC0341E, i_c0341e),
    (0xC03420, i_c03420),
    (0xC03422, i_c03422),
    (0xC03424, i_c03424),
    (0xC03428, i_c03428),
    (0xC0342C, i_c0342c),
    (0xC03430, i_c03430),
    (0xC03434, i_c03434),
    (0xC03438, i_c03438),
    (0xC0343C, i_c0343c),
    (0xC0343E, i_c0343e),
    (0xC03440, i_c03440),
    (0xC03442, i_c03442),
    (0xC03444, i_c03444),
    (0xC03446, i_c03446),
    (0xC03448, i_c03448),
    (0xC0344A, i_c0344a),
    (0xC0344C, i_c0344c),
    (0xC0344E, i_c0344e),
    (0xC03450, i_c03450),
    (0xC03452, i_c03452),
    (0xC03454, i_c03454),
    (0xC03456, i_c03456),
    (0xC03458, i_c03458),
    (0xC0345A, i_c0345a),
    (0xC0345C, i_c0345c),
    (0xC0345E, i_c0345e),
    (0xC03460, i_c03460),
    (0xC03462, i_c03462),
    (0xC03464, i_c03464),
    (0xC03468, i_c03468),
    (0xC0346C, i_c0346c),
    (0xC03470, i_c03470),
    (0xC03474, i_c03474),
    (0xC03478, i_c03478),
    (0xC0347C, i_c0347c),
    (0xC0347E, i_c0347e),
    (0xC03480, i_c03480),
    (0xC03482, i_c03482),
    (0xC03484, i_c03484),
    (0xC03486, i_c03486),
    (0xC03488, i_c03488),
    (0xC0348A, i_c0348a),
    (0xC0348C, i_c0348c),
    (0xC0348E, i_c0348e),
    (0xC03490, i_c03490),
    (0xC03492, i_c03492),
    (0xC03494, i_c03494),
    (0xC03496, i_c03496),
    (0xC03498, i_c03498),
    (0xC0349A, i_c0349a),
    (0xC0349C, i_c0349c),
    (0xC0349E, i_c0349e),
    (0xC034A0, i_c034a0),
    (0xC034A2, i_c034a2),
    (0xC034A4, i_c034a4),
    (0xC034A8, i_c034a8),
    (0xC034AC, i_c034ac),
    (0xC034B0, i_c034b0),
    (0xC034B4, i_c034b4),
    (0xC034B8, i_c034b8),
    (0xC034BC, i_c034bc),
    (0xC034C0, i_c034c0),
    (0xC034C2, i_c034c2),
    (0xC034C4, i_c034c4),
    (0xC034C6, i_c034c6),
    (0xC034C8, i_c034c8),
    (0xC034CA, i_c034ca),
    (0xC034CC, i_c034cc),
    (0xC034CE, i_c034ce),
    (0xC034D0, i_c034d0),
    (0xC034D2, i_c034d2),
    (0xC034D4, i_c034d4),
    (0xC034D6, i_c034d6),
    (0xC034D8, i_c034d8),
    (0xC034DA, i_c034da),
    (0xC034DC, i_c034dc),
    (0xC034DE, i_c034de),
    (0xC034E0, i_c034e0),
    (0xC034E2, i_c034e2),
    (0xC034E6, i_c034e6),
    (0xC034EA, i_c034ea),
    (0xC034EE, i_c034ee),
    (0xC034F2, i_c034f2),
    (0xC034F6, i_c034f6),
    (0xC034FA, i_c034fa),
    (0xC034FC, i_c034fc),
    (0xC03500, i_c03500),
    (0xC03502, i_c03502),
    (0xC03504, i_c03504),
    (0xC03506, i_c03506),
    (0xC03508, i_c03508),
    (0xC0350A, i_c0350a),
    (0xC0350C, i_c0350c),
    (0xC0350E, i_c0350e),
    (0xC03510, i_c03510),
    (0xC03512, i_c03512),
    (0xC03514, i_c03514),
    (0xC03516, i_c03516),
    (0xC03518, i_c03518),
    (0xC0351A, i_c0351a),
    (0xC0351C, i_c0351c),
    (0xC0351E, i_c0351e),
    (0xC03520, i_c03520),
    (0xC03522, i_c03522),
    (0xC03526, i_c03526),
    (0xC0352A, i_c0352a),
    (0xC0352E, i_c0352e),
    (0xC03532, i_c03532),
    (0xC03536, i_c03536),
    (0xC0353A, i_c0353a),
    (0xC0353C, i_c0353c),
    (0xC03540, i_c03540),
    (0xC03542, i_c03542),
    (0xC03544, i_c03544),
    (0xC03546, i_c03546),
    (0xC03548, i_c03548),
    (0xC0354A, i_c0354a),
    (0xC0354C, i_c0354c),
    (0xC0354E, i_c0354e),
    (0xC03550, i_c03550),
    (0xC03552, i_c03552),
    (0xC03554, i_c03554),
    (0xC03556, i_c03556),
    (0xC03558, i_c03558),
    (0xC0355A, i_c0355a),
    (0xC0355C, i_c0355c),
    (0xC0355E, i_c0355e),
    (0xC03560, i_c03560),
    (0xC03562, i_c03562),
    (0xC03566, i_c03566),
    (0xC0356A, i_c0356a),
    (0xC0356E, i_c0356e),
    (0xC03572, i_c03572),
    (0xC03576, i_c03576),
    (0xC0357A, i_c0357a),
    (0xC0357C, i_c0357c),
    (0xC0357E, i_c0357e),
    (0xC03580, i_c03580),
    (0xC03582, i_c03582),
    (0xC03584, i_c03584),
    (0xC03586, i_c03586),
    (0xC03588, i_c03588),
    (0xC0358A, i_c0358a),
    (0xC0358C, i_c0358c),
    (0xC0358E, i_c0358e),
    (0xC03590, i_c03590),
    (0xC03592, i_c03592),
    (0xC03594, i_c03594),
    (0xC03596, i_c03596),
    (0xC03598, i_c03598),
    (0xC0359A, i_c0359a),
    (0xC0359C, i_c0359c),
    (0xC0359E, i_c0359e),
    (0xC035A0, i_c035a0),
    (0xC035A4, i_c035a4),
    (0xC035A8, i_c035a8),
    (0xC035AC, i_c035ac),
    (0xC035B0, i_c035b0),
    (0xC035B4, i_c035b4),
    (0xC035BA, i_c035ba),
    (0xC035BC, i_c035bc),
    (0xC035BE, i_c035be),
    (0xC035C0, i_c035c0),
    (0xC035C2, i_c035c2),
    (0xC035C4, i_c035c4),
    (0xC035C6, i_c035c6),
    (0xC035C8, i_c035c8),
    (0xC035CA, i_c035ca),
    (0xC035CC, i_c035cc),
    (0xC035CE, i_c035ce),
    (0xC035D0, i_c035d0),
    (0xC035D2, i_c035d2),
    (0xC035D4, i_c035d4),
    (0xC035D6, i_c035d6),
    (0xC035D8, i_c035d8),
    (0xC035DA, i_c035da),
    (0xC035DC, i_c035dc),
    (0xC035DE, i_c035de),
    (0xC035E0, i_c035e0),
    (0xC035E4, i_c035e4),
    (0xC035E8, i_c035e8),
    (0xC035EC, i_c035ec),
    (0xC035F0, i_c035f0),
    (0xC035F4, i_c035f4),
    (0xC035F8, i_c035f8),
    (0xC035FA, i_c035fa),
    (0xC035FC, i_c035fc),
    (0xC035FE, i_c035fe),
    (0xC03600, i_c03600),
    (0xC03602, i_c03602),
    (0xC03604, i_c03604),
    (0xC03606, i_c03606),
    (0xC03608, i_c03608),
    (0xC0360A, i_c0360a),
    (0xC0360C, i_c0360c),
    (0xC0360E, i_c0360e),
    (0xC03610, i_c03610),
    (0xC03612, i_c03612),
    (0xC03614, i_c03614),
    (0xC03616, i_c03616),
    (0xC03618, i_c03618),
    (0xC0361A, i_c0361a),
    (0xC0361C, i_c0361c),
    (0xC0361E, i_c0361e),
    (0xC03620, i_c03620),
    (0xC03624, i_c03624),
    (0xC03628, i_c03628),
    (0xC0362C, i_c0362c),
    (0xC03630, i_c03630),
    (0xC03634, i_c03634),
    (0xC03638, i_c03638),
    (0xC0363A, i_c0363a),
    (0xC0363E, i_c0363e),
    (0xC03640, i_c03640),
    (0xC03642, i_c03642),
    (0xC03644, i_c03644),
    (0xC03646, i_c03646),
    (0xC03648, i_c03648),
    (0xC0364A, i_c0364a),
    (0xC0364C, i_c0364c),
    (0xC0364E, i_c0364e),
    (0xC03650, i_c03650),
    (0xC03652, i_c03652),
    (0xC03654, i_c03654),
    (0xC03656, i_c03656),
    (0xC03658, i_c03658),
    (0xC0365A, i_c0365a),
    (0xC0365C, i_c0365c),
    (0xC0365E, i_c0365e),
    (0xC03662, i_c03662),
    (0xC03666, i_c03666),
    (0xC0366A, i_c0366a),
    (0xC0366E, i_c0366e),
    (0xC03672, i_c03672),
    (0xC03678, i_c03678),
    (0xC0367A, i_c0367a),
    (0xC0367E, i_c0367e),
    (0xC03680, i_c03680),
    (0xC03682, i_c03682),
    (0xC03684, i_c03684),
    (0xC03686, i_c03686),
    (0xC03688, i_c03688),
    (0xC0368A, i_c0368a),
    (0xC0368C, i_c0368c),
    (0xC0368E, i_c0368e),
    (0xC03690, i_c03690),
    (0xC03692, i_c03692),
    (0xC03694, i_c03694),
    (0xC03696, i_c03696),
    (0xC03698, i_c03698),
    (0xC0369A, i_c0369a),
    (0xC0369C, i_c0369c),
    (0xC0369E, i_c0369e),
    (0xC036A2, i_c036a2),
    (0xC036A6, i_c036a6),
    (0xC036AA, i_c036aa),
    (0xC036AE, i_c036ae),
    (0xC036B2, i_c036b2),
    (0xC036B8, i_c036b8),
    (0xC036BA, i_c036ba),
    (0xC036BC, i_c036bc),
    (0xC036BE, i_c036be),
    (0xC036C0, i_c036c0),
    (0xC036C2, i_c036c2),
    (0xC036C4, i_c036c4),
    (0xC036C6, i_c036c6),
    (0xC036C8, i_c036c8),
    (0xC036CA, i_c036ca),
    (0xC036CC, i_c036cc),
    (0xC036CE, i_c036ce),
    (0xC036D0, i_c036d0),
    (0xC036D2, i_c036d2),
    (0xC036D4, i_c036d4),
    (0xC036D6, i_c036d6),
    (0xC036D8, i_c036d8),
    (0xC036DA, i_c036da),
    (0xC036DC, i_c036dc),
    (0xC036E0, i_c036e0),
    (0xC036E4, i_c036e4),
    (0xC036E8, i_c036e8),
    (0xC036EC, i_c036ec),
    (0xC036F0, i_c036f0),
    (0xC036F4, i_c036f4),
    (0xC036F8, i_c036f8),
    (0xC036FA, i_c036fa),
    (0xC036FC, i_c036fc),
    (0xC036FE, i_c036fe),
    (0xC03700, i_c03700),
    (0xC03702, i_c03702),
    (0xC03704, i_c03704),
    (0xC03706, i_c03706),
    (0xC03708, i_c03708),
    (0xC0370A, i_c0370a),
    (0xC0370C, i_c0370c),
    (0xC0370E, i_c0370e),
    (0xC03710, i_c03710),
    (0xC03712, i_c03712),
    (0xC03714, i_c03714),
    (0xC03716, i_c03716),
    (0xC03718, i_c03718),
    (0xC0371A, i_c0371a),
    (0xC0371C, i_c0371c),
    (0xC03720, i_c03720),
    (0xC03724, i_c03724),
    (0xC03728, i_c03728),
    (0xC0372C, i_c0372c),
    (0xC03730, i_c03730),
    (0xC03734, i_c03734),
    (0xC03738, i_c03738),
    (0xC0373C, i_c0373c),
    (0xC0373E, i_c0373e),
    (0xC03740, i_c03740),
    (0xC03742, i_c03742),
    (0xC03744, i_c03744),
    (0xC03746, i_c03746),
    (0xC03748, i_c03748),
    (0xC0374A, i_c0374a),
    (0xC0374C, i_c0374c),
    (0xC0374E, i_c0374e),
    (0xC03750, i_c03750),
    (0xC03752, i_c03752),
    (0xC03754, i_c03754),
    (0xC03756, i_c03756),
    (0xC03758, i_c03758),
    (0xC0375A, i_c0375a),
    (0xC0375E, i_c0375e),
    (0xC03762, i_c03762),
    (0xC03766, i_c03766),
    (0xC0376A, i_c0376a),
    (0xC0376E, i_c0376e),
    (0xC03772, i_c03772),
    (0xC03776, i_c03776),
    (0xC03778, i_c03778),
    (0xC0377C, i_c0377c),
    (0xC0377E, i_c0377e),
    (0xC03780, i_c03780),
    (0xC03782, i_c03782),
    (0xC03784, i_c03784),
    (0xC03786, i_c03786),
    (0xC03788, i_c03788),
    (0xC0378A, i_c0378a),
    (0xC0378C, i_c0378c),
    (0xC0378E, i_c0378e),
    (0xC03790, i_c03790),
    (0xC03792, i_c03792),
    (0xC03794, i_c03794),
    (0xC03796, i_c03796),
    (0xC03798, i_c03798),
    (0xC0379A, i_c0379a),
    (0xC0379E, i_c0379e),
    (0xC037A2, i_c037a2),
    (0xC037A6, i_c037a6),
    (0xC037AA, i_c037aa),
    (0xC037AE, i_c037ae),
    (0xC037B2, i_c037b2),
    (0xC037B6, i_c037b6),
    (0xC037B8, i_c037b8),
    (0xC037BA, i_c037ba),
    (0xC037BC, i_c037bc),
    (0xC037BE, i_c037be),
    (0xC037C0, i_c037c0),
    (0xC037C2, i_c037c2),
    (0xC037C4, i_c037c4),
    (0xC037C6, i_c037c6),
    (0xC037C8, i_c037c8),
    (0xC037CA, i_c037ca),
    (0xC037CC, i_c037cc),
    (0xC037CE, i_c037ce),
    (0xC037D0, i_c037d0),
    (0xC037D2, i_c037d2),
    (0xC037D4, i_c037d4),
    (0xC037D6, i_c037d6),
    (0xC037D8, i_c037d8),
    (0xC037DC, i_c037dc),
    (0xC037E0, i_c037e0),
    (0xC037E4, i_c037e4),
    (0xC037E8, i_c037e8),
    (0xC037EC, i_c037ec),
    (0xC037F0, i_c037f0),
    (0xC037F4, i_c037f4),
    (0xC037F6, i_c037f6),
    (0xC037F8, i_c037f8),
    (0xC037FA, i_c037fa),
    (0xC037FC, i_c037fc),
    (0xC037FE, i_c037fe),
    (0xC045EC, i_c045ec),
    (0xC045EE, i_c045ee),
    (0xC045F0, i_c045f0),
    (0xC045F2, i_c045f2),
    (0xC045F4, i_c045f4),
    (0xC045F6, i_c045f6),
    (0xC045F8, i_c045f8),
    (0xC045FA, i_c045fa),
    (0xC045FC, i_c045fc),
    (0xC04EBA, i_c04eba),
    (0xC04EBC, i_c04ebc),
    (0xC04EBE, i_c04ebe),
    (0xC04EC0, i_c04ec0),
    (0xC04EC2, i_c04ec2),
    (0xC04EC4, i_c04ec4),
    (0xC04EC6, i_c04ec6),
    (0xC04EC8, i_c04ec8),
    (0xC04ECA, i_c04eca),
    (0xC06000, i_c06000),
    (0xC06002, i_c06002),
    (0xC06004, i_c06004),
    (0xC06006, i_c06006),
    (0xC06008, i_c06008),
    (0xC0600A, i_c0600a),
    (0xC0600C, i_c0600c),
    (0xC0600E, i_c0600e),
    (0xC06010, i_c06010),
    (0xC06500, i_c06500),
    (0xC06502, i_c06502),
    (0xC06504, i_c06504),
    (0xC06506, i_c06506),
    (0xC06508, i_c06508),
    (0xC0650A, i_c0650a),
    (0xC0650C, i_c0650c),
    (0xC0650E, i_c0650e),
    (0xC06510, i_c06510),
    (0xC06600, i_c06600),
    (0xC06602, i_c06602),
    (0xC06604, i_c06604),
    (0xC06606, i_c06606),
    (0xC06608, i_c06608),
    (0xC0660A, i_c0660a),
    (0xC0660C, i_c0660c),
    (0xC0660E, i_c0660e),
    (0xC06610, i_c06610),
    (0xC06700, i_c06700),
    (0xC06702, i_c06702),
    (0xC06704, i_c06704),
    (0xC06706, i_c06706),
    (0xC06708, i_c06708),
    (0xC0670A, i_c0670a),
    (0xC0670C, i_c0670c),
    (0xC0670E, i_c0670e),
    (0xC06710, i_c06710),
    (0xC06C00, i_c06c00),
    (0xC06C02, i_c06c02),
    (0xC06C04, i_c06c04),
    (0xC06C06, i_c06c06),
    (0xC06C08, i_c06c08),
    (0xC06C0A, i_c06c0a),
    (0xC06C0C, i_c06c0c),
    (0xC06C0E, i_c06c0e),
    (0xC06C10, i_c06c10),
    (0xC06D00, i_c06d00),
    (0xC06D02, i_c06d02),
    (0xC06D04, i_c06d04),
    (0xC06D06, i_c06d06),
    (0xC06D08, i_c06d08),
    (0xC06D0A, i_c06d0a),
    (0xC06D0C, i_c06d0c),
    (0xC06D0E, i_c06d0e),
    (0xC06D10, i_c06d10),
    (0xC06E00, i_c06e00),
    (0xC06E02, i_c06e02),
    (0xC06E04, i_c06e04),
    (0xC06E06, i_c06e06),
    (0xC06E08, i_c06e08),
    (0xC06E0A, i_c06e0a),
    (0xC06E0C, i_c06e0c),
    (0xC06E0E, i_c06e0e),
    (0xC06E10, i_c06e10),
    (0xC06F00, i_c06f00),
    (0xC06F02, i_c06f02),
    (0xC06F04, i_c06f04),
    (0xC06F06, i_c06f06),
    (0xC06F08, i_c06f08),
    (0xC06F0A, i_c06f0a),
    (0xC06F0C, i_c06f0c),
    (0xC06F0E, i_c06f0e),
    (0xC06F10, i_c06f10),
    (0xC07000, i_c07000),
    (0xC07002, i_c07002),
    (0xC07004, i_c07004),
    (0xC07006, i_c07006),
    (0xC07008, i_c07008),
    (0xC0700A, i_c0700a),
    (0xC0700C, i_c0700c),
    (0xC0700E, i_c0700e),
    (0xC07010, i_c07010),
    (0xC07204, i_c07204),
    (0xC07206, i_c07206),
    (0xC07208, i_c07208),
    (0xC0720A, i_c0720a),
    (0xC0720C, i_c0720c),
    (0xC0720E, i_c0720e),
    (0xC07210, i_c07210),
    (0xC07212, i_c07212),
    (0xC07214, i_c07214),
    (0xC0906E, i_c0906e),
    (0xC09070, i_c09070),
    (0xC09072, i_c09072),
    (0xC09074, i_c09074),
    (0xC09076, i_c09076),
    (0xC09078, i_c09078),
    (0xC0907A, i_c0907a),
    (0xC0907C, i_c0907c),
    (0xC0907E, i_c0907e),
    (0xC0B470, i_c0b470),
    (0xC0B472, i_c0b472),
    (0xC0B474, i_c0b474),
    (0xC0B476, i_c0b476),
    (0xC0B478, i_c0b478),
    (0xC0B47A, i_c0b47a),
    (0xC0B47C, i_c0b47c),
    (0xC0B47E, i_c0b47e),
    (0xC0B480, i_c0b480),
    (0xC0FF80, i_c0ff80),
    (0xC0FF82, i_c0ff82),
    (0xC0FF84, i_c0ff84),
    (0xC0FF86, i_c0ff86),
    (0xC0FF88, i_c0ff88),
    (0xC0FF8A, i_c0ff8a),
    (0xC0FF8C, i_c0ff8c),
    (0xC0FF8E, i_c0ff8e),
    (0xC0FF90, i_c0ff90),
    (0xC10000, i_c10000),
    (0xC10002, i_c10002),
    (0xC10004, i_c10004),
    (0xC10006, i_c10006),
    (0xC1000C, i_c1000c),
    (0xC10012, i_c10012),
    (0xC10014, i_c10014),
    (0xC10016, i_c10016),
    (0xC10018, i_c10018),
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
    (0xC1007E, i_c1007e),
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
    (0xC10168, i_c10168),
    (0xC1016A, i_c1016a),
    (0xC1016E, i_c1016e),
    (0xC10170, i_c10170),
    (0xC10172, i_c10172),
    (0xC10174, i_c10174),
    (0xC10176, i_c10176),
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
    (0xC101DC, i_c101dc),
    (0xC101E0, i_c101e0),
    (0xC101E4, i_c101e4),
    (0xC101E6, i_c101e6),
    (0xC101E8, i_c101e8),
];
