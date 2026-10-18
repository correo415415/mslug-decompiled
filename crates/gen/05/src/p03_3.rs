// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p03_3) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $035E58: jsr $32d32(pc)
pub fn i_035e58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35E5C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x35E5C);
    cpu.pc = 0x32D32;
    16
}
/// $035E5C: jsr $27a92.l
pub fn i_035e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35E62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x35E62);
    cpu.pc = 0x27A92;
    16
}
/// $035E62: jsr $32aa8(pc)
pub fn i_035e62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35E66;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x35E66);
    cpu.pc = 0x32AA8;
    16
}
/// $035E66: bcc.w $35ea0
pub fn i_035e66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35E6A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x35EA0; return 10; }
    8
}
/// $035E6A: lea.l $34704(pc), a1
pub fn i_035e6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35E6E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x34704;
    16
}
/// $035E6E: move.l a1, (a6)
pub fn i_035e6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35E70;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $035E70: move.b #$1, $78(a6)
pub fn i_035e70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35E76;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035E76: lsl.b #$4, d0
pub fn i_035e76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35E78;
    cpu.ir = 0xE908;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    14
}
/// $035E78: or.b $78(a6), d0
pub fn i_035e78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35E7C;
    cpu.ir = 0x802E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    let r = (v3 | cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $035E7C: move.b d0, $79(a6)
pub fn i_035e7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35E80;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x79);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035E80: btst.b #$0, $88(a6)
pub fn i_035e80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35E86;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $035E86: beq.w $35e90
pub fn i_035e86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35E8A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x35E90; return 10; }
    8
}
/// $035E8A: lea.l $33a5e(pc), a1
pub fn i_035e8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35E8E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33A5E;
    16
}
/// $035E8E: move.l a1, (a6)
pub fn i_035e8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35E90;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $035E90: cmpi.w #$0, $82(a6)
pub fn i_035e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35E96;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $035E96: bne.w $35ea0
pub fn i_035e96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35E9A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x35EA0; return 10; }
    8
}
/// $035E9A: lea.l $33d64(pc), a1
pub fn i_035e9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35E9E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33D64;
    16
}
/// $035E9E: move.l a1, (a6)
pub fn i_035e9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35EA0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $035EA0: jsr $35cd8(pc)
pub fn i_035ea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35EA4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x35EA4);
    cpu.pc = 0x35CD8;
    16
}
/// $035EA4: jmp $34ada(pc)
pub fn i_035ea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35EA8;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x34ADA;
    8
}
// ===== Player_MeleeAlt_035ea8  ($035EA8, 200 B, player/states_0342c4.s)
/// $035EA8: move.w #$0, $7c(a6)
pub fn i_035ea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35EAE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $035EAE: move.w #$0, $7e(a6)
pub fn i_035eae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35EB4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $035EB4: bclr.b #$2, $8c(a6)
pub fn i_035eb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35EBA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $035EBA: bclr.b #$1, $8c(a6)
pub fn i_035eba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35EC0;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $035EC0: bclr.b #$3, $8c(a6)
pub fn i_035ec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35EC6;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $035EC6: lea.l $32638(pc), a0
pub fn i_035ec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35ECA;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x32638;
    16
}
/// $035ECA: move.l a0, $4c(a6)
pub fn i_035eca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35ECE;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $035ECE: jsr $283ca.l
pub fn i_035ece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35ED4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x35ED4);
    cpu.pc = 0x283CA;
    16
}
/// $035ED4: move.l #$32500, $60(a6)
pub fn i_035ed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35EDC;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x32500;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $035EDC: jsr $2abcc.l
pub fn i_035edc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35EE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x35EE2);
    cpu.pc = 0x2ABCC;
    16
}
/// $035EE2: bcs.w $35f0e
pub fn i_035ee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35EE6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x35F0E; return 10; }
    8
}
/// $035EE6: move.b #$33, $70(a6)
pub fn i_035ee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35EEC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x33;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035EEC: lea.l $279efe.l, a0
pub fn i_035eec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35EF2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279EFE;
    20
}
/// $035EF2: move.l -$4(a0), $74(a6)
pub fn i_035ef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35EF8;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $035EF8: move.b #$ff, $21(a6)
pub fn i_035ef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35EFE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035EFE: lea.l $279efe.l, a0
pub fn i_035efe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F04;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279EFE;
    20
}
/// $035F04: jsr $28cd4.l
pub fn i_035f04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x35F0A);
    cpu.pc = 0x28CD4;
    16
}
/// $035F0A: bra.w $35f32
pub fn i_035f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F0E;
    cpu.ir = 0x6000;
    cpu.pc = 0x35F32;
    10
}
/// $035F0E: move.b #$33, $70(a6)
pub fn i_035f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F14;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x33;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035F14: lea.l $279efe.l, a0
pub fn i_035f14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F1A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279EFE;
    20
}
/// $035F1A: move.l -$4(a0), $74(a6)
pub fn i_035f1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F20;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $035F20: move.b #$ff, $21(a6)
pub fn i_035f20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F26;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035F26: lea.l $279efe.l, a0
pub fn i_035f26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F2C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279EFE;
    20
}
/// $035F2C: jsr $28cd4.l
pub fn i_035f2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x35F32);
    cpu.pc = 0x28CD4;
    16
}
/// $035F32: lea.l $32788(pc), a0
pub fn i_035f32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F36;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x32788;
    16
}
/// $035F36: move.l a0, $48(a6)
pub fn i_035f36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F3A;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $035F3A: lea.l $35f40(pc), a1
pub fn i_035f3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F3E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x35F40;
    16
}
/// $035F3E: move.l a1, (a6)
pub fn i_035f3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F40;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $035F40: jsr $32ff2(pc)
pub fn i_035f40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F44;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x35F44);
    cpu.pc = 0x32FF2;
    16
}
/// $035F44: jsr $32d32(pc)
pub fn i_035f44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F48;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x35F48);
    cpu.pc = 0x32D32;
    16
}
/// $035F48: jsr $27a92.l
pub fn i_035f48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x35F4E);
    cpu.pc = 0x27A92;
    16
}
/// $035F4E: jsr $32aa8(pc)
pub fn i_035f4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F52;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x35F52);
    cpu.pc = 0x32AA8;
    16
}
/// $035F52: bcc.w $35f6c
pub fn i_035f52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F56;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x35F6C; return 10; }
    8
}
/// $035F56: lea.l $34704(pc), a1
pub fn i_035f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F5A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x34704;
    16
}
/// $035F5A: move.l a1, (a6)
pub fn i_035f5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F5C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $035F5C: move.b #$1, $78(a6)
pub fn i_035f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F62;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035F62: lsl.b #$4, d0
pub fn i_035f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F64;
    cpu.ir = 0xE908;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    14
}
/// $035F64: or.b $78(a6), d0
pub fn i_035f64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F68;
    cpu.ir = 0x802E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    let r = (v3 | cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $035F68: move.b d0, $79(a6)
pub fn i_035f68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F6C;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x79);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035F6C: jmp $34ccc(pc)
pub fn i_035f6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F70;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x34CCC;
    8
}
// ===== Player_MeleeAlt_Walk_035f70  ($035F70, 332 B, player/states_0342c4.s)
/// $035F70: move.w #$0, $7c(a6)
pub fn i_035f70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F76;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $035F76: move.w #$ffff, $7e(a6)
pub fn i_035f76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F7C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $035F7C: bclr.b #$2, $8c(a6)
pub fn i_035f7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F82;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $035F82: bclr.b #$1, $8c(a6)
pub fn i_035f82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F88;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $035F88: bclr.b #$3, $8c(a6)
pub fn i_035f88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F8E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $035F8E: lea.l $32638(pc), a0
pub fn i_035f8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F92;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x32638;
    16
}
/// $035F92: move.l a0, $4c(a6)
pub fn i_035f92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F96;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $035F96: jsr $283ca.l
pub fn i_035f96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x35F9C);
    cpu.pc = 0x283CA;
    16
}
/// $035F9C: move.l #$32500, $60(a6)
pub fn i_035f9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35FA4;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x32500;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $035FA4: jsr $2abcc.l
pub fn i_035fa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35FAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x35FAA);
    cpu.pc = 0x2ABCC;
    16
}
/// $035FAA: bcs.w $35fd6
pub fn i_035faa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35FAE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x35FD6; return 10; }
    8
}
/// $035FAE: move.b #$33, $70(a6)
pub fn i_035fae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35FB4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x33;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035FB4: lea.l $279efe.l, a0
pub fn i_035fb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35FBA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279EFE;
    20
}
/// $035FBA: move.l -$4(a0), $74(a6)
pub fn i_035fba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35FC0;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $035FC0: move.b #$ff, $21(a6)
pub fn i_035fc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35FC6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035FC6: lea.l $279efe.l, a0
pub fn i_035fc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35FCC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279EFE;
    20
}
/// $035FCC: jsr $28cd4.l
pub fn i_035fcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35FD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x35FD2);
    cpu.pc = 0x28CD4;
    16
}
/// $035FD2: bra.w $35ffa
pub fn i_035fd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35FD6;
    cpu.ir = 0x6000;
    cpu.pc = 0x35FFA;
    10
}
/// $035FD6: move.b #$33, $70(a6)
pub fn i_035fd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35FDC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x33;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035FDC: lea.l $279efe.l, a0
pub fn i_035fdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35FE2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279EFE;
    20
}
/// $035FE2: move.l -$4(a0), $74(a6)
pub fn i_035fe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35FE8;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $035FE8: move.b #$ff, $21(a6)
pub fn i_035fe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35FEE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035FEE: lea.l $279efe.l, a0
pub fn i_035fee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35FF4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279EFE;
    20
}
/// $035FF4: jsr $28cd4.l
pub fn i_035ff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35FFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x35FFA);
    cpu.pc = 0x28CD4;
    16
}
/// $035FFA: lea.l $32788(pc), a0
pub fn i_035ffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35FFE;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x32788;
    16
}
/// $035FFE: move.l a0, $48(a6)
pub fn i_035ffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36002;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $036002: lea.l $36008(pc), a1
pub fn i_036002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36006;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36008;
    16
}
/// $036006: move.l a1, (a6)
pub fn i_036006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36008;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036008: jsr $32ff2(pc)
pub fn i_036008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3600C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3600C);
    cpu.pc = 0x32FF2;
    16
}
/// $03600C: jsr $32d32(pc)
pub fn i_03600c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36010;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x36010);
    cpu.pc = 0x32D32;
    16
}
/// $036010: clr.w d7
pub fn i_036010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36012;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $036012: jsr $32e3c(pc)
pub fn i_036012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36016;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x36016);
    cpu.pc = 0x32E3C;
    16
}
/// $036016: bcc.w $36024
pub fn i_036016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3601A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x36024; return 10; }
    8
}
/// $03601A: bclr.b #$0, $3a(a6)
pub fn i_03601a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36020;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036020: move.w #$300, d7
pub fn i_036020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36024;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $036024: bcc.w $36036
pub fn i_036024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36028;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x36036; return 10; }
    8
}
/// $036028: jsr $32e42(pc)
pub fn i_036028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3602C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3602C);
    cpu.pc = 0x32E42;
    16
}
/// $03602C: bset.b #$0, $3a(a6)
pub fn i_03602c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36032;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036032: move.w #$fd00, d7
pub fn i_036032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36036;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0xFD00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $036036: move.w d7, $28(a6)
pub fn i_036036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3603A;
    cpu.ir = 0x3D47;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03603A: jsr $27a92.l
pub fn i_03603a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36040;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36040);
    cpu.pc = 0x27A92;
    16
}
/// $036040: jsr $32aa8(pc)
pub fn i_036040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36044;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x36044);
    cpu.pc = 0x32AA8;
    16
}
/// $036044: bcc.w $3605e
pub fn i_036044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36048;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3605E; return 10; }
    8
}
/// $036048: lea.l $34704(pc), a1
pub fn i_036048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3604C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x34704;
    16
}
/// $03604C: move.l a1, (a6)
pub fn i_03604c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3604E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03604E: move.b #$1, $78(a6)
pub fn i_03604e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36054;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $036054: lsl.b #$4, d0
pub fn i_036054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36056;
    cpu.ir = 0xE908;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    14
}
/// $036056: or.b $78(a6), d0
pub fn i_036056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3605A;
    cpu.ir = 0x802E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    let r = (v3 | cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $03605A: move.b d0, $79(a6)
pub fn i_03605a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3605E;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x79);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03605E: jsr $330d0(pc)
pub fn i_03605e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36062;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x36062);
    cpu.pc = 0x330D0;
    16
}
/// $036062: bcc.w $360b8
pub fn i_036062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36066;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x360B8; return 10; }
    8
}
/// $036066: cmpi.b #$ff, d1
pub fn i_036066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3606A;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $03606A: bne.w $36078
pub fn i_03606a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3606E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x36078; return 10; }
    8
}
/// $03606E: lea.l $35d34(pc), a1
pub fn i_03606e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36072;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x35D34;
    16
}
/// $036072: move.l a1, (a6)
pub fn i_036072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36074;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036074: bra.w $360b8
pub fn i_036074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36078;
    cpu.ir = 0x6000;
    cpu.pc = 0x360B8;
    10
}
/// $036078: cmpi.b #$3, d1
pub fn i_036078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3607C;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $03607C: bne.w $3608a
pub fn i_03607c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36080;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3608A; return 10; }
    8
}
/// $036080: lea.l $360bc(pc), a1
pub fn i_036080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36084;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x360BC;
    16
}
/// $036084: move.l a1, (a6)
pub fn i_036084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36086;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036086: bra.w $360b8
pub fn i_036086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3608A;
    cpu.ir = 0x6000;
    cpu.pc = 0x360B8;
    10
}
/// $03608A: cmpi.b #$4, d1
pub fn i_03608a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3608E;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x4, Size::Byte, false);
    8
}
/// $03608E: bne.w $3609c
pub fn i_03608e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36092;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3609C; return 10; }
    8
}
/// $036092: lea.l $3873c(pc), a1
pub fn i_036092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36096;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3873C;
    16
}
/// $036096: move.l a1, (a6)
pub fn i_036096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36098;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036098: bra.w $360b8
pub fn i_036098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3609C;
    cpu.ir = 0x6000;
    cpu.pc = 0x360B8;
    10
}
/// $03609C: cmpi.b #$1, d1
pub fn i_03609c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x360A0;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $0360A0: bne.w $360ae
pub fn i_0360a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x360A4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x360AE; return 10; }
    8
}
/// $0360A4: lea.l $357be(pc), a1
pub fn i_0360a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x360A8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x357BE;
    16
}
/// $0360A8: move.l a1, (a6)
pub fn i_0360a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x360AA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0360AA: bra.w $360b8
pub fn i_0360aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x360AE;
    cpu.ir = 0x6000;
    cpu.pc = 0x360B8;
    10
}
/// $0360AE: lea.l $357aa(pc), a1
pub fn i_0360ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x360B2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x357AA;
    16
}
/// $0360B2: move.l a1, (a6)
pub fn i_0360b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x360B4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0360B4: bra.w $360b8
pub fn i_0360b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x360B8;
    cpu.ir = 0x6000;
    cpu.pc = 0x360B8;
    10
}
/// $0360B8: jmp $3548c(pc)
pub fn i_0360b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x360BC;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x3548C;
    8
}
// ===== Player_ThrowGrenade_Stand_0360bc  ($0360BC, 342 B, player/states_0342c4.s)
/// $0360BC: move.w #$0, $7c(a6)
pub fn i_0360bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x360C2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0360C2: move.w #$0, $7e(a6)
pub fn i_0360c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x360C8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0360C8: bclr.b #$2, $8c(a6)
pub fn i_0360c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x360CE;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0360CE: bclr.b #$1, $8c(a6)
pub fn i_0360ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x360D4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0360D4: bclr.b #$3, $8c(a6)
pub fn i_0360d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x360DA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0360DA: clr.w $28(a6)
pub fn i_0360da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x360DE;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0360DE: clr.w $2c(a6)
pub fn i_0360de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x360E2;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0360E2: clr.w $2a(a6)
pub fn i_0360e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x360E6;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0360E6: clr.w $2e(a6)
pub fn i_0360e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x360EA;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0360EA: move.l #$32500, $60(a6)
pub fn i_0360ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x360F2;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x32500;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0360F2: jsr $2abcc.l
pub fn i_0360f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x360F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x360F8);
    cpu.pc = 0x2ABCC;
    16
}
/// $0360F8: bcs.w $36124
pub fn i_0360f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x360FC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x36124; return 10; }
    8
}
/// $0360FC: move.b #$33, $70(a6)
pub fn i_0360fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36102;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x33;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $036102: lea.l $279d2e.l, a0
pub fn i_036102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36108;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D2E;
    20
}
/// $036108: move.l -$4(a0), $74(a6)
pub fn i_036108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3610E;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $03610E: move.b #$ff, $21(a6)
pub fn i_03610e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36114;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $036114: lea.l $279d2e.l, a0
pub fn i_036114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3611A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D2E;
    20
}
/// $03611A: jsr $28cd4.l
pub fn i_03611a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36120;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36120);
    cpu.pc = 0x28CD4;
    16
}
/// $036120: bra.w $36148
pub fn i_036120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36124;
    cpu.ir = 0x6000;
    cpu.pc = 0x36148;
    10
}
/// $036124: move.b #$33, $70(a6)
pub fn i_036124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3612A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x33;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03612A: lea.l $279d2e.l, a0
pub fn i_03612a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36130;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D2E;
    20
}
/// $036130: move.l -$4(a0), $74(a6)
pub fn i_036130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36136;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $036136: move.b #$ff, $21(a6)
pub fn i_036136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3613C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03613C: lea.l $279d2e.l, a0
pub fn i_03613c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36142;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D2E;
    20
}
/// $036142: jsr $28cd4.l
pub fn i_036142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36148;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36148);
    cpu.pc = 0x28CD4;
    16
}
/// $036148: clr.b $3b(a6)
pub fn i_036148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3614C;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x3B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03614C: lea.l $36152(pc), a1
pub fn i_03614c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36150;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36152;
    16
}
/// $036150: move.l a1, (a6)
pub fn i_036150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36152;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036152: jsr $32ff2(pc)
pub fn i_036152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36156;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x36156);
    cpu.pc = 0x32FF2;
    16
}
/// $036156: jsr $32d32(pc)
pub fn i_036156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3615A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3615A);
    cpu.pc = 0x32D32;
    16
}
/// $03615A: jsr $27a92.l
pub fn i_03615a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36160;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36160);
    cpu.pc = 0x27A92;
    16
}
/// $036160: jsr $32aa8(pc)
pub fn i_036160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36164;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x36164);
    cpu.pc = 0x32AA8;
    16
}
/// $036164: bcc.w $3619e
pub fn i_036164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36168;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3619E; return 10; }
    8
}
/// $036168: lea.l $34704(pc), a1
pub fn i_036168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3616C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x34704;
    16
}
/// $03616C: move.l a1, (a6)
pub fn i_03616c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3616E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03616E: move.b #$1, $78(a6)
pub fn i_03616e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36174;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $036174: lsl.b #$4, d0
pub fn i_036174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36176;
    cpu.ir = 0xE908;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    14
}
/// $036176: or.b $78(a6), d0
pub fn i_036176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3617A;
    cpu.ir = 0x802E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    let r = (v3 | cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $03617A: move.b d0, $79(a6)
pub fn i_03617a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3617E;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x79);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03617E: btst.b #$0, $88(a6)
pub fn i_03617e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36184;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $036184: beq.w $3618e
pub fn i_036184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36188;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3618E; return 10; }
    8
}
/// $036188: lea.l $33a5e(pc), a1
pub fn i_036188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3618C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33A5E;
    16
}
/// $03618C: move.l a1, (a6)
pub fn i_03618c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3618E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03618E: cmpi.w #$0, $82(a6)
pub fn i_03618e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36194;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $036194: bne.w $3619e
pub fn i_036194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36198;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3619E; return 10; }
    8
}
/// $036198: lea.l $33d64(pc), a1
pub fn i_036198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3619C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33D64;
    16
}
/// $03619C: move.l a1, (a6)
pub fn i_03619c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3619E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03619E: btst.b #$2, $8c(a6)
pub fn i_03619e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x361A4;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0361A4: bne.w $3620a
pub fn i_0361a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x361A8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3620A; return 10; }
    8
}
/// $0361A8: btst.b #$0, $3a(a6)
pub fn i_0361a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x361AE;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0361AE: bne.w $361d2
pub fn i_0361ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x361B2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x361D2; return 10; }
    8
}
/// $0361B2: jsr $32e3c(pc)
pub fn i_0361b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x361B6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x361B6);
    cpu.pc = 0x32E3C;
    16
}
/// $0361B6: bcc.w $361c0
pub fn i_0361b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x361BA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x361C0; return 10; }
    8
}
/// $0361BA: lea.l $34b38(pc), a1
pub fn i_0361ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x361BE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x34B38;
    16
}
/// $0361BE: move.l a1, (a6)
pub fn i_0361be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x361C0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0361C0: jsr $32e42(pc)
pub fn i_0361c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x361C4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x361C4);
    cpu.pc = 0x32E42;
    16
}
/// $0361C4: bcc.w $361ce
pub fn i_0361c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x361C8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x361CE; return 10; }
    8
}
/// $0361C8: lea.l $35bf8(pc), a1
pub fn i_0361c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x361CC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x35BF8;
    16
}
/// $0361CC: move.l a1, (a6)
pub fn i_0361cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x361CE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0361CE: bra.w $361ee
pub fn i_0361ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x361D2;
    cpu.ir = 0x6000;
    cpu.pc = 0x361EE;
    10
}
/// $0361D2: jsr $32e3c(pc)
pub fn i_0361d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x361D6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x361D6);
    cpu.pc = 0x32E3C;
    16
}
/// $0361D6: bcc.w $361e0
pub fn i_0361d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x361DA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x361E0; return 10; }
    8
}
/// $0361DA: lea.l $35aba(pc), a1
pub fn i_0361da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x361DE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x35ABA;
    16
}
/// $0361DE: move.l a1, (a6)
pub fn i_0361de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x361E0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0361E0: jsr $32e42(pc)
pub fn i_0361e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x361E4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x361E4);
    cpu.pc = 0x32E42;
    16
}
/// $0361E4: bcc.w $361ee
pub fn i_0361e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x361E8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x361EE; return 10; }
    8
}
/// $0361E8: lea.l $34d32(pc), a1
pub fn i_0361e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x361EC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x34D32;
    16
}
/// $0361EC: move.l a1, (a6)
pub fn i_0361ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x361EE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0361EE: jsr $32e90(pc)
pub fn i_0361ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x361F2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x361F2);
    cpu.pc = 0x32E90;
    16
}
/// $0361F2: bcc.w $361fc
pub fn i_0361f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x361F6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x361FC; return 10; }
    8
}
/// $0361F6: lea.l $37c74(pc), a1
pub fn i_0361f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x361FA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37C74;
    16
}
/// $0361FA: move.l a1, (a6)
pub fn i_0361fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x361FC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0361FC: jsr $33034(pc)
pub fn i_0361fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36200;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x36200);
    cpu.pc = 0x33034;
    16
}
/// $036200: bcc.w $3620a
pub fn i_036200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36204;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3620A; return 10; }
    8
}
/// $036204: lea.l $36914(pc), a1
pub fn i_036204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36208;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36914;
    16
}
/// $036208: move.l a1, (a6)
pub fn i_036208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3620A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03620A: jsr $35cd8(pc)
pub fn i_03620a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3620E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3620E);
    cpu.pc = 0x35CD8;
    16
}
/// $03620E: jmp $34ada(pc)
pub fn i_03620e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36212;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x34ADA;
    8
}
// ===== Player_ThrowGrenade_Walk_036212  ($036212, 262 B, player/states_0342c4.s)
/// $036212: move.w #$0, $7c(a6)
pub fn i_036212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36218;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $036218: move.w #$0, $7e(a6)
pub fn i_036218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3621E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03621E: bclr.b #$2, $8c(a6)
pub fn i_03621e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36224;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036224: bclr.b #$1, $8c(a6)
pub fn i_036224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3622A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03622A: bclr.b #$3, $8c(a6)
pub fn i_03622a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36230;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036230: move.l #$32500, $60(a6)
pub fn i_036230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36238;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x32500;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $036238: jsr $2abcc.l
pub fn i_036238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3623E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3623E);
    cpu.pc = 0x2ABCC;
    16
}
/// $03623E: bcs.w $3626a
pub fn i_03623e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36242;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x3626A; return 10; }
    8
}
/// $036242: move.b #$33, $70(a6)
pub fn i_036242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36248;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x33;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $036248: lea.l $279d38.l, a0
pub fn i_036248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3624E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D38;
    20
}
/// $03624E: move.l -$4(a0), $74(a6)
pub fn i_03624e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36254;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $036254: move.b #$ff, $21(a6)
pub fn i_036254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3625A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03625A: lea.l $279d38.l, a0
pub fn i_03625a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36260;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D38;
    20
}
/// $036260: jsr $28cd4.l
pub fn i_036260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36266;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36266);
    cpu.pc = 0x28CD4;
    16
}
/// $036266: bra.w $3628e
pub fn i_036266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3626A;
    cpu.ir = 0x6000;
    cpu.pc = 0x3628E;
    10
}
/// $03626A: move.b #$33, $70(a6)
pub fn i_03626a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36270;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x33;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $036270: lea.l $279d38.l, a0
pub fn i_036270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36276;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D38;
    20
}
/// $036276: move.l -$4(a0), $74(a6)
pub fn i_036276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3627C;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $03627C: move.b #$ff, $21(a6)
pub fn i_03627c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36282;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $036282: lea.l $279d38.l, a0
pub fn i_036282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36288;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D38;
    20
}
/// $036288: jsr $28cd4.l
pub fn i_036288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3628E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3628E);
    cpu.pc = 0x28CD4;
    16
}
/// $03628E: clr.b $3b(a6)
pub fn i_03628e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36292;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x3B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $036292: lea.l $36298(pc), a1
pub fn i_036292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36296;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36298;
    16
}
/// $036296: move.l a1, (a6)
pub fn i_036296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36298;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036298: jsr $32ff2(pc)
pub fn i_036298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3629C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3629C);
    cpu.pc = 0x32FF2;
    16
}
/// $03629C: jsr $32d32(pc)
pub fn i_03629c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x362A0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x362A0);
    cpu.pc = 0x32D32;
    16
}
/// $0362A0: jsr $27a92.l
pub fn i_0362a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x362A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x362A6);
    cpu.pc = 0x27A92;
    16
}
/// $0362A6: jsr $32aa8(pc)
pub fn i_0362a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x362AA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x362AA);
    cpu.pc = 0x32AA8;
    16
}
/// $0362AA: bcc.w $362c4
pub fn i_0362aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x362AE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x362C4; return 10; }
    8
}
/// $0362AE: lea.l $34704(pc), a1
pub fn i_0362ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x362B2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x34704;
    16
}
/// $0362B2: move.l a1, (a6)
pub fn i_0362b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x362B4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0362B4: move.b #$1, $78(a6)
pub fn i_0362b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x362BA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0362BA: lsl.b #$4, d0
pub fn i_0362ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x362BC;
    cpu.ir = 0xE908;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    14
}
/// $0362BC: or.b $78(a6), d0
pub fn i_0362bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x362C0;
    cpu.ir = 0x802E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    let r = (v3 | cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0362C0: move.b d0, $79(a6)
pub fn i_0362c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x362C4;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x79);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0362C4: btst.b #$2, $8c(a6)
pub fn i_0362c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x362CA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0362CA: bne.w $36314
pub fn i_0362ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x362CE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x36314; return 10; }
    8
}
/// $0362CE: btst.b #$0, $3a(a6)
pub fn i_0362ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x362D4;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0362D4: bne.w $362ea
pub fn i_0362d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x362D8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x362EA; return 10; }
    8
}
/// $0362D8: jsr $32e42(pc)
pub fn i_0362d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x362DC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x362DC);
    cpu.pc = 0x32E42;
    16
}
/// $0362DC: bcc.w $362e6
pub fn i_0362dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x362E0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x362E6; return 10; }
    8
}
/// $0362E0: lea.l $35bf8(pc), a1
pub fn i_0362e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x362E4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x35BF8;
    16
}
/// $0362E4: move.l a1, (a6)
pub fn i_0362e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x362E6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0362E6: bra.w $362f8
pub fn i_0362e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x362EA;
    cpu.ir = 0x6000;
    cpu.pc = 0x362F8;
    10
}
/// $0362EA: jsr $32e3c(pc)
pub fn i_0362ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x362EE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x362EE);
    cpu.pc = 0x32E3C;
    16
}
/// $0362EE: bcc.w $362f8
pub fn i_0362ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x362F2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x362F8; return 10; }
    8
}
/// $0362F2: lea.l $35aba(pc), a1
pub fn i_0362f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x362F6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x35ABA;
    16
}
/// $0362F6: move.l a1, (a6)
pub fn i_0362f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x362F8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0362F8: jsr $32e90(pc)
pub fn i_0362f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x362FC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x362FC);
    cpu.pc = 0x32E90;
    16
}
/// $0362FC: bcc.w $36306
pub fn i_0362fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36300;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x36306; return 10; }
    8
}
/// $036300: lea.l $37c74(pc), a1
pub fn i_036300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36304;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37C74;
    16
}
/// $036304: move.l a1, (a6)
pub fn i_036304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36306;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036306: jsr $33034(pc)
pub fn i_036306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3630A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3630A);
    cpu.pc = 0x33034;
    16
}
/// $03630A: bcc.w $36314
pub fn i_03630a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3630E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x36314; return 10; }
    8
}
/// $03630E: lea.l $36914(pc), a1
pub fn i_03630e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36312;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36914;
    16
}
/// $036312: move.l a1, (a6)
pub fn i_036312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36314;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036314: jmp $34ccc(pc)
pub fn i_036314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36318;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x34CCC;
    8
}
// ===== Player_ThrowGrenade_WalkLoop_036318  ($036318, 394 B, player/states_0342c4.s)
/// $036318: move.w #$0, $7c(a6)
pub fn i_036318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3631E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03631E: move.w #$ffff, $7e(a6)
pub fn i_03631e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36324;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $036324: bclr.b #$2, $8c(a6)
pub fn i_036324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3632A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03632A: bclr.b #$1, $8c(a6)
pub fn i_03632a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36330;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036330: bclr.b #$3, $8c(a6)
pub fn i_036330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36336;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036336: move.l #$32500, $60(a6)
pub fn i_036336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3633E;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x32500;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $03633E: jsr $2abcc.l
pub fn i_03633e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36344;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36344);
    cpu.pc = 0x2ABCC;
    16
}
/// $036344: bcs.w $36370
pub fn i_036344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36348;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x36370; return 10; }
    8
}
/// $036348: move.b #$33, $70(a6)
pub fn i_036348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3634E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x33;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03634E: lea.l $279d38.l, a0
pub fn i_03634e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36354;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D38;
    20
}
/// $036354: move.l -$4(a0), $74(a6)
pub fn i_036354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3635A;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $03635A: move.b #$ff, $21(a6)
pub fn i_03635a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36360;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $036360: lea.l $279d38.l, a0
pub fn i_036360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36366;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D38;
    20
}
/// $036366: jsr $28cd4.l
pub fn i_036366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3636C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3636C);
    cpu.pc = 0x28CD4;
    16
}
/// $03636C: bra.w $36394
pub fn i_03636c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36370;
    cpu.ir = 0x6000;
    cpu.pc = 0x36394;
    10
}
/// $036370: move.b #$33, $70(a6)
pub fn i_036370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36376;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x33;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $036376: lea.l $279d38.l, a0
pub fn i_036376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3637C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D38;
    20
}
/// $03637C: move.l -$4(a0), $74(a6)
pub fn i_03637c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36382;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $036382: move.b #$ff, $21(a6)
pub fn i_036382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36388;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $036388: lea.l $279d38.l, a0
pub fn i_036388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3638E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D38;
    20
}
/// $03638E: jsr $28cd4.l
pub fn i_03638e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36394;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36394);
    cpu.pc = 0x28CD4;
    16
}
/// $036394: clr.b $3b(a6)
pub fn i_036394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36398;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x3B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $036398: lea.l $3639e(pc), a1
pub fn i_036398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3639C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3639E;
    16
}
/// $03639C: move.l a1, (a6)
pub fn i_03639c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3639E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03639E: jsr $32ff2(pc)
pub fn i_03639e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x363A2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x363A2);
    cpu.pc = 0x32FF2;
    16
}
/// $0363A2: jsr $32d32(pc)
pub fn i_0363a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x363A6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x363A6);
    cpu.pc = 0x32D32;
    16
}
/// $0363A6: clr.w d7
pub fn i_0363a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x363A8;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0363A8: jsr $32e3c(pc)
pub fn i_0363a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x363AC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x363AC);
    cpu.pc = 0x32E3C;
    16
}
/// $0363AC: bcc.w $363ba
pub fn i_0363ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x363B0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x363BA; return 10; }
    8
}
/// $0363B0: bclr.b #$0, $3a(a6)
pub fn i_0363b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x363B6;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0363B6: move.w #$300, d7
pub fn i_0363b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x363BA;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0363BA: bcc.w $363cc
pub fn i_0363ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x363BE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x363CC; return 10; }
    8
}
/// $0363BE: jsr $32e42(pc)
pub fn i_0363be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x363C2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x363C2);
    cpu.pc = 0x32E42;
    16
}
/// $0363C2: bset.b #$0, $3a(a6)
pub fn i_0363c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x363C8;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0363C8: move.w #$fd00, d7
pub fn i_0363c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x363CC;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0xFD00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0363CC: move.w d7, $28(a6)
pub fn i_0363cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x363D0;
    cpu.ir = 0x3D47;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0363D0: jsr $27a92.l
pub fn i_0363d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x363D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x363D6);
    cpu.pc = 0x27A92;
    16
}
/// $0363D6: jsr $32aa8(pc)
pub fn i_0363d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x363DA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x363DA);
    cpu.pc = 0x32AA8;
    16
}
/// $0363DA: bcc.w $363f4
pub fn i_0363da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x363DE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x363F4; return 10; }
    8
}
/// $0363DE: lea.l $34704(pc), a1
pub fn i_0363de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x363E2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x34704;
    16
}
/// $0363E2: move.l a1, (a6)
pub fn i_0363e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x363E4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0363E4: move.b #$1, $78(a6)
pub fn i_0363e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x363EA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0363EA: lsl.b #$4, d0
pub fn i_0363ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x363EC;
    cpu.ir = 0xE908;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    14
}
/// $0363EC: or.b $78(a6), d0
pub fn i_0363ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x363F0;
    cpu.ir = 0x802E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    let r = (v3 | cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0363F0: move.b d0, $79(a6)
pub fn i_0363f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x363F4;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x79);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0363F4: btst.b #$2, $8c(a6)
pub fn i_0363f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x363FA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0363FA: bne.w $36444
pub fn i_0363fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x363FE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x36444; return 10; }
    8
}
/// $0363FE: btst.b #$0, $3a(a6)
pub fn i_0363fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36404;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $036404: bne.w $3641a
pub fn i_036404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36408;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3641A; return 10; }
    8
}
/// $036408: jsr $32e42(pc)
pub fn i_036408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3640C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3640C);
    cpu.pc = 0x32E42;
    16
}
/// $03640C: bcc.w $36416
pub fn i_03640c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36410;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x36416; return 10; }
    8
}
/// $036410: lea.l $35bf8(pc), a1
pub fn i_036410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36414;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x35BF8;
    16
}
/// $036414: move.l a1, (a6)
pub fn i_036414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36416;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036416: bra.w $36428
pub fn i_036416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3641A;
    cpu.ir = 0x6000;
    cpu.pc = 0x36428;
    10
}
/// $03641A: jsr $32e3c(pc)
pub fn i_03641a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3641E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3641E);
    cpu.pc = 0x32E3C;
    16
}
/// $03641E: bcc.w $36428
pub fn i_03641e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36422;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x36428; return 10; }
    8
}
/// $036422: lea.l $35aba(pc), a1
pub fn i_036422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36426;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x35ABA;
    16
}
/// $036426: move.l a1, (a6)
pub fn i_036426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36428;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036428: jsr $32e90(pc)
pub fn i_036428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3642C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3642C);
    cpu.pc = 0x32E90;
    16
}
/// $03642C: bcc.w $36436
pub fn i_03642c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36430;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x36436; return 10; }
    8
}
/// $036430: lea.l $37c74(pc), a1
pub fn i_036430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36434;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37C74;
    16
}
/// $036434: move.l a1, (a6)
pub fn i_036434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36436;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036436: jsr $33034(pc)
pub fn i_036436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3643A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3643A);
    cpu.pc = 0x33034;
    16
}
/// $03643A: bcc.w $36444
pub fn i_03643a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3643E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x36444; return 10; }
    8
}
/// $03643E: lea.l $36914(pc), a1
pub fn i_03643e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36442;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36914;
    16
}
/// $036442: move.l a1, (a6)
pub fn i_036442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36444;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036444: jsr $330d0(pc)
pub fn i_036444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36448;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x36448);
    cpu.pc = 0x330D0;
    16
}
/// $036448: bcc.w $3649e
pub fn i_036448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3644C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3649E; return 10; }
    8
}
/// $03644C: cmpi.b #$ff, d1
pub fn i_03644c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36450;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $036450: bne.w $3645e
pub fn i_036450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36454;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3645E; return 10; }
    8
}
/// $036454: lea.l $35d34(pc), a1
pub fn i_036454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36458;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x35D34;
    16
}
/// $036458: move.l a1, (a6)
pub fn i_036458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3645A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03645A: bra.w $3649e
pub fn i_03645a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3645E;
    cpu.ir = 0x6000;
    cpu.pc = 0x3649E;
    10
}
/// $03645E: cmpi.b #$3, d1
pub fn i_03645e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36462;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $036462: bne.w $36470
pub fn i_036462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36466;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x36470; return 10; }
    8
}
/// $036466: lea.l $360bc(pc), a1
pub fn i_036466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3646A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x360BC;
    16
}
/// $03646A: move.l a1, (a6)
pub fn i_03646a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3646C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03646C: bra.w $3649e
pub fn i_03646c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36470;
    cpu.ir = 0x6000;
    cpu.pc = 0x3649E;
    10
}
/// $036470: cmpi.b #$4, d1
pub fn i_036470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36474;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x4, Size::Byte, false);
    8
}
/// $036474: bne.w $36482
pub fn i_036474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36478;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x36482; return 10; }
    8
}
/// $036478: lea.l $3873c(pc), a1
pub fn i_036478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3647C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3873C;
    16
}
/// $03647C: move.l a1, (a6)
pub fn i_03647c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3647E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03647E: bra.w $3649e
pub fn i_03647e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36482;
    cpu.ir = 0x6000;
    cpu.pc = 0x3649E;
    10
}
/// $036482: cmpi.b #$1, d1
pub fn i_036482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36486;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $036486: bne.w $36494
pub fn i_036486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3648A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x36494; return 10; }
    8
}
/// $03648A: lea.l $357be(pc), a1
pub fn i_03648a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3648E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x357BE;
    16
}
/// $03648E: move.l a1, (a6)
pub fn i_03648e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36490;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036490: bra.w $3649e
pub fn i_036490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36494;
    cpu.ir = 0x6000;
    cpu.pc = 0x3649E;
    10
}
/// $036494: lea.l $357aa(pc), a1
pub fn i_036494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36498;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x357AA;
    16
}
/// $036498: move.l a1, (a6)
pub fn i_036498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3649A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03649A: bra.w $3649e
pub fn i_03649a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3649E;
    cpu.ir = 0x6000;
    cpu.pc = 0x3649E;
    10
}
/// $03649E: jmp $3548c(pc)
pub fn i_03649e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x364A2;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x3548C;
    8
}
// ===== Player_RideSlug_0364a2  ($0364A2, 400 B, player/states_0342c4.s)
/// $0364A2: bclr.b #$2, $8c(a6)
pub fn i_0364a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x364A8;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0364A8: bclr.b #$1, $8c(a6)
pub fn i_0364a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x364AE;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0364AE: bclr.b #$3, $8c(a6)
pub fn i_0364ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x364B4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0364B4: bclr.b #$0, $3a(a6)
pub fn i_0364b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x364BA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0364BA: lea.l $328d8(pc), a0
pub fn i_0364ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x364BE;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x328D8;
    16
}
/// $0364BE: move.l a0, $48(a6)
pub fn i_0364be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x364C2;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0364C2: move.w #$0, $7c(a6)
pub fn i_0364c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x364C8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0364C8: move.w #$10, $7e(a6)
pub fn i_0364c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x364CE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0364CE: move.b #$2, $70(a6)
pub fn i_0364ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x364D4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0364D4: lea.l $279f08.l, a0
pub fn i_0364d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x364DA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279F08;
    20
}
/// $0364DA: move.l -$4(a0), $74(a6)
pub fn i_0364da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x364E0;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0364E0: move.b #$ff, $21(a6)
pub fn i_0364e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x364E6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0364E6: lea.l $279f08.l, a0
pub fn i_0364e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x364EC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279F08;
    20
}
/// $0364EC: jsr $28cd4.l
pub fn i_0364ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x364F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x364F2);
    cpu.pc = 0x28CD4;
    16
}
/// $0364F2: clr.w $28(a6)
pub fn i_0364f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x364F6;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0364F6: clr.w $2c(a6)
pub fn i_0364f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x364FA;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0364FA: clr.w $2a(a6)
pub fn i_0364fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x364FE;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0364FE: clr.w $2e(a6)
pub fn i_0364fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36502;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $036502: lea.l $36508(pc), a1
pub fn i_036502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36506;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36508;
    16
}
/// $036506: move.l a1, (a6)
pub fn i_036506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36508;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036508: jsr $33016(pc)
pub fn i_036508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3650C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3650C);
    cpu.pc = 0x33016;
    16
}
/// $03650C: jsr $2783a.l
pub fn i_03650c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36512;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36512);
    cpu.pc = 0x2783A;
    16
}
/// $036512: lea.l $100580.l, a0
pub fn i_036512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36518;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100580;
    20
}
/// $036518: move.w $22(a0), d0
pub fn i_036518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3651C;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03651C: move.w $24(a0), d1
pub fn i_03651c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36520;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $036520: addq.w #$1, d1
pub fn i_036520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36522;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $036522: move.w d0, $22(a6)
pub fn i_036522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36526;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $036526: move.w d1, $24(a6)
pub fn i_036526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3652A;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03652A: jsr $32aa8(pc)
pub fn i_03652a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3652E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3652E);
    cpu.pc = 0x32AA8;
    16
}
/// $03652E: jsr $2aca2.l
pub fn i_03652e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36534;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36534);
    cpu.pc = 0x2ACA2;
    16
}
/// $036534: lea.l $100580.l, a0
pub fn i_036534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3653A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100580;
    20
}
/// $03653A: move.b $68(a6), d2
pub fn i_03653a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3653E;
    cpu.ir = 0x142E;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $03653E: move.b d2, $68(a0)
pub fn i_03653e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36542;
    cpu.ir = 0x1142;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x68);
    bus.write8(ad2, v1 as u8);
    12
}
/// $036542: jsr $2abf0.l
pub fn i_036542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36548;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36548);
    cpu.pc = 0x2ABF0;
    16
}
/// $036548: bcs.w $365bc
pub fn i_036548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3654C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x365BC; return 10; }
    8
}
/// $03654C: jsr $5cef8.l
pub fn i_03654c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36552;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36552);
    cpu.pc = 0x5CEF8;
    16
}
/// $036552: bcs.w $3656a
pub fn i_036552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36556;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x3656A; return 10; }
    8
}
/// $036556: lea.l $100580.l, a0
pub fn i_036556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3655C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100580;
    20
}
/// $03655C: btst.b #$0, $13(a0)
pub fn i_03655c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36562;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $036562: bne.w $3656a
pub fn i_036562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36566;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3656A; return 10; }
    8
}
/// $036566: bra.w $365bc
pub fn i_036566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3656A;
    cpu.ir = 0x6000;
    cpu.pc = 0x365BC;
    10
}
/// $03656A: btst.b #$2, $8c(a6)
pub fn i_03656a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36570;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $036570: bne.w $36598
pub fn i_036570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36574;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x36598; return 10; }
    8
}
/// $036574: jsr $5cdc0.l
pub fn i_036574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3657A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3657A);
    cpu.pc = 0x5CDC0;
    16
}
/// $03657A: bcc.w $36598
pub fn i_03657a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3657E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x36598; return 10; }
    8
}
/// $03657E: jsr $2abd2.l
pub fn i_03657e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36584;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36584);
    cpu.pc = 0x2ABD2;
    16
}
/// $036584: bcc.w $36598
pub fn i_036584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36588;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x36598; return 10; }
    8
}
/// $036588: btst.b #$0, $13(a0)
pub fn i_036588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3658E;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03658E: bne.w $36598
pub fn i_03658e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36592;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x36598; return 10; }
    8
}
/// $036592: lea.l $366fe(pc), a1
pub fn i_036592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36596;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x366FE;
    16
}
/// $036596: move.l a1, (a6)
pub fn i_036596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36598;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036598: jsr $5cdb4.l
pub fn i_036598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3659E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3659E);
    cpu.pc = 0x5CDB4;
    16
}
/// $03659E: bcc.w $365bc
pub fn i_03659e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x365A2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x365BC; return 10; }
    8
}
/// $0365A2: jsr $2abd2.l
pub fn i_0365a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x365A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x365A8);
    cpu.pc = 0x2ABD2;
    16
}
/// $0365A8: bcc.w $365bc
pub fn i_0365a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x365AC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x365BC; return 10; }
    8
}
/// $0365AC: lea.l $36796(pc), a1
pub fn i_0365ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x365B0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36796;
    16
}
/// $0365B0: move.l a1, (a6)
pub fn i_0365b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x365B2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0365B2: addi.w #$20, $24(a6)
pub fn i_0365b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x365B8;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0365B8: bra.w $365f8
pub fn i_0365b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x365BC;
    cpu.ir = 0x6000;
    cpu.pc = 0x365F8;
    10
}
/// $0365BC: jsr $2abf0.l
pub fn i_0365bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x365C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x365C2);
    cpu.pc = 0x2ABF0;
    16
}
/// $0365C2: bcs.w $365e8
pub fn i_0365c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x365C6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x365E8; return 10; }
    8
}
/// $0365C6: jsr $5d00e.l
pub fn i_0365c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x365CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x365CC);
    cpu.pc = 0x5D00E;
    16
}
/// $0365CC: bcc.w $365e8
pub fn i_0365cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x365D0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x365E8; return 10; }
    8
}
/// $0365D0: lea.l $36796(pc), a1
pub fn i_0365d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x365D4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36796;
    16
}
/// $0365D4: move.l a1, (a6)
pub fn i_0365d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x365D6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0365D6: addi.w #$20, $24(a6)
pub fn i_0365d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x365DC;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0365DC: move.b #$3c, $45(a6)
pub fn i_0365dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x365E2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0365E2: move.b #$3c, $59(a6)
pub fn i_0365e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x365E8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x59);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0365E8: jsr $2a25c.l
pub fn i_0365e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x365EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x365EE);
    cpu.pc = 0x2A25C;
    16
}
/// $0365EE: bcc.w $365f8
pub fn i_0365ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x365F2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x365F8; return 10; }
    8
}
/// $0365F2: lea.l $36796(pc), a1
pub fn i_0365f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x365F6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36796;
    16
}
/// $0365F6: move.l a1, (a6)
pub fn i_0365f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x365F8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0365F8: cmpi.b #$3, $106ece.l
pub fn i_0365f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36600;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    20
}
/// $036600: beq.w $36618
pub fn i_036600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36604;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x36618; return 10; }
    8
}
/// $036604: movea.l #$ffffffff, a0
pub fn i_036604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3660A;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $03660A: lea.l $324c6(pc), a0
pub fn i_03660a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3660E;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x324C6;
    16
}
/// $03660E: jsr $5dd56.l
pub fn i_03660e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36614;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36614);
    cpu.pc = 0x5DD56;
    16
}
/// $036614: bra.w $36628
pub fn i_036614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36618;
    cpu.ir = 0x6000;
    cpu.pc = 0x36628;
    10
}
/// $036618: movea.l #$ffffffff, a0
pub fn i_036618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3661E;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $03661E: lea.l $324bc(pc), a0
pub fn i_03661e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36622;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x324BC;
    16
}
/// $036622: jsr $5dd56.l
pub fn i_036622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36628;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36628);
    cpu.pc = 0x5DD56;
    16
}
/// $036628: bcc.w $36632
pub fn i_036628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3662C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x36632; return 10; }
    8
}
/// $03662C: lea.l $37b8e(pc), a1
pub fn i_03662c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36630;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37B8E;
    16
}
/// $036630: move.l a1, (a6)
pub fn i_036630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36632;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Player_RideSlug_Tail_036632  ($036632, 6 B, player/air_death_crouch_036632.s)
/// $036632: jsr $33522(pc)
pub fn i_036632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36636;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x36636);
    cpu.pc = 0x33522;
    16
}
/// $036636: rts
pub fn i_036636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36638;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_HangRing_036638  ($036638, 198 B, player/air_death_crouch_036632.s)
/// $036638: bclr.b #$2, $8c(a6)
pub fn i_036638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3663E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03663E: bclr.b #$1, $8c(a6)
pub fn i_03663e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36644;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036644: bclr.b #$3, $8c(a6)
pub fn i_036644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3664A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03664A: bclr.b #$0, $3a(a6)
pub fn i_03664a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36650;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036650: lea.l $3292c(pc), a0
pub fn i_036650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36654;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3292C;
    16
}
/// $036654: move.l a0, $48(a6)
pub fn i_036654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36658;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $036658: move.w #$0, $7c(a6)
pub fn i_036658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3665E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03665E: move.w #$10, $7e(a6)
pub fn i_03665e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36664;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $036664: move.b #$3, $70(a6)
pub fn i_036664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3666A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03666A: lea.l $279f08.l, a0
pub fn i_03666a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36670;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279F08;
    20
}
/// $036670: move.l -$4(a0), $74(a6)
pub fn i_036670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36676;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $036676: move.b #$ff, $21(a6)
pub fn i_036676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3667C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03667C: lea.l $279f08.l, a0
pub fn i_03667c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36682;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279F08;
    20
}
/// $036682: jsr $28cd4.l
pub fn i_036682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36688;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36688);
    cpu.pc = 0x28CD4;
    16
}
/// $036688: clr.w $28(a6)
pub fn i_036688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3668C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $03668C: clr.w $2c(a6)
pub fn i_03668c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36690;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $036690: clr.w $2a(a6)
pub fn i_036690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36694;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $036694: clr.w $2e(a6)
pub fn i_036694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36698;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $036698: lea.l $3669e(pc), a1
pub fn i_036698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3669C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3669E;
    16
}
/// $03669C: move.l a1, (a6)
pub fn i_03669c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3669E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03669E: jsr $33016(pc)
pub fn i_03669e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x366A2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x366A2);
    cpu.pc = 0x33016;
    16
}
/// $0366A2: jsr $2783a.l
pub fn i_0366a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x366A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x366A8);
    cpu.pc = 0x2783A;
    16
}
/// $0366A8: move.b $86(a6), d0
pub fn i_0366a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x366AC;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0366AC: jsr $8f5dc.l
pub fn i_0366ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x366B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x366B2);
    cpu.pc = 0x8F5DC;
    16
}
/// $0366B2: bcc.w $366c2
pub fn i_0366b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x366B6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x366C2; return 10; }
    8
}
/// $0366B6: move.w d0, $22(a6)
pub fn i_0366b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x366BA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0366BA: move.w d1, $24(a6)
pub fn i_0366ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x366BE;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0366BE: bra.w $366d4
pub fn i_0366be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x366C2;
    cpu.ir = 0x6000;
    cpu.pc = 0x366D4;
    10
}
/// $0366C2: lea.l $36914(pc), a1
pub fn i_0366c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x366C6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36914;
    16
}
/// $0366C6: move.l a1, (a6)
pub fn i_0366c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x366C8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0366C8: addi.w #$20, $24(a6)
pub fn i_0366c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x366CE;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0366CE: move.b #$14, $45(a6)
pub fn i_0366ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x366D4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0366D4: jsr $32aa8(pc)
pub fn i_0366d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x366D8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x366D8);
    cpu.pc = 0x32AA8;
    16
}
/// $0366D8: jsr $5cdb4.l
pub fn i_0366d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x366DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x366DE);
    cpu.pc = 0x5CDB4;
    16
}
/// $0366DE: bcc.w $366f8
pub fn i_0366de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x366E2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x366F8; return 10; }
    8
}
/// $0366E2: lea.l $36796(pc), a1
pub fn i_0366e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x366E6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36796;
    16
}
/// $0366E6: move.l a1, (a6)
pub fn i_0366e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x366E8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0366E8: addi.w #$20, $24(a6)
pub fn i_0366e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x366EE;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0366EE: move.b #$14, $45(a6)
pub fn i_0366ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x366F4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0366F4: bra.w $366f8
pub fn i_0366f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x366F8;
    cpu.ir = 0x6000;
    cpu.pc = 0x366F8;
    10
}
/// $0366F8: jsr $33522(pc)
pub fn i_0366f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x366FC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x366FC);
    cpu.pc = 0x33522;
    16
}
/// $0366FC: rts
pub fn i_0366fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x366FE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_RideSlug_Pose2_0366fe  ($0366FE, 152 B, player/air_death_crouch_036632.s)
/// $0366FE: move.w #$0, $7c(a6)
pub fn i_0366fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36704;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $036704: move.w #$0, $7e(a6)
pub fn i_036704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3670A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03670A: bclr.b #$2, $8c(a6)
pub fn i_03670a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36710;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036710: bclr.b #$1, $8c(a6)
pub fn i_036710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36716;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036716: bclr.b #$3, $8c(a6)
pub fn i_036716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3671C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03671C: jsr $5d5b6.l
pub fn i_03671c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36722;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36722);
    cpu.pc = 0x5D5B6;
    16
}
/// $036722: asl.w #$1, d0
pub fn i_036722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36724;
    cpu.ir = 0xE340;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $036724: lea.l $324d0(pc), a0
pub fn i_036724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36728;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x324D0;
    16
}
/// $036728: move.w (a0, d0.w), $2c(a6)
pub fn i_036728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3672E;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad4, v3 as u16);
    22
}
/// $03672E: lea.l $328d8(pc), a0
pub fn i_03672e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36732;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x328D8;
    16
}
/// $036732: move.l a0, $48(a6)
pub fn i_036732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36736;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $036736: move.b #$2, $70(a6)
pub fn i_036736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3673C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03673C: lea.l $279f1c.l, a0
pub fn i_03673c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36742;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279F1C;
    20
}
/// $036742: move.l -$4(a0), $74(a6)
pub fn i_036742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36748;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $036748: move.b #$ff, $21(a6)
pub fn i_036748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3674E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03674E: lea.l $279f1c.l, a0
pub fn i_03674e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36754;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279F1C;
    20
}
/// $036754: jsr $28cd4.l
pub fn i_036754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3675A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3675A);
    cpu.pc = 0x28CD4;
    16
}
/// $03675A: clr.b $3b(a6)
pub fn i_03675a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3675E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x3B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03675E: lea.l $36764(pc), a1
pub fn i_03675e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36762;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36764;
    16
}
/// $036762: move.l a1, (a6)
pub fn i_036762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36764;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036764: jsr $33016(pc)
pub fn i_036764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36768;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x36768);
    cpu.pc = 0x33016;
    16
}
/// $036768: jsr $2783a.l
pub fn i_036768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3676E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3676E);
    cpu.pc = 0x2783A;
    16
}
/// $03676E: lea.l $100580.l, a0
pub fn i_03676e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36774;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100580;
    20
}
/// $036774: move.w $22(a0), d0
pub fn i_036774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36778;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $036778: move.w $24(a0), d1
pub fn i_036778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3677C;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03677C: move.w d0, $22(a6)
pub fn i_03677c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36780;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $036780: move.w d1, $24(a6)
pub fn i_036780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36784;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $036784: jsr $32aa8(pc)
pub fn i_036784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36788;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x36788);
    cpu.pc = 0x32AA8;
    16
}
/// $036788: bcc.w $36792
pub fn i_036788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3678C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x36792; return 10; }
    8
}
/// $03678C: lea.l $364a2(pc), a1
pub fn i_03678c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36790;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x364A2;
    16
}
/// $036790: move.l a1, (a6)
pub fn i_036790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36792;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036792: jmp $3652e(pc)
pub fn i_036792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36796;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x3652E;
    8
}
// ===== Player_SlugJumpOff_036796  ($036796, 330 B, player/air_death_crouch_036632.s)
/// $036796: bclr.b #$0, $8c(a6)
pub fn i_036796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3679C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03679C: move.w #$0, $7c(a6)
pub fn i_03679c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x367A2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0367A2: move.w #$0, $7e(a6)
pub fn i_0367a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x367A8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0367A8: bclr.b #$2, $8c(a6)
pub fn i_0367a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x367AE;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0367AE: bclr.b #$1, $8c(a6)
pub fn i_0367ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x367B4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0367B4: bclr.b #$3, $8c(a6)
pub fn i_0367b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x367BA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0367BA: move.l #$32500, $60(a6)
pub fn i_0367ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x367C2;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x32500;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0367C2: move.b #$26, $70(a6)
pub fn i_0367c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x367C8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x26;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0367C8: lea.l $279690.l, a0
pub fn i_0367c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x367CE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279690;
    20
}
/// $0367CE: move.l -$4(a0), $74(a6)
pub fn i_0367ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x367D4;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0367D4: move.b #$ff, $21(a6)
pub fn i_0367d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x367DA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0367DA: lea.l $279690.l, a0
pub fn i_0367da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x367E0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279690;
    20
}
/// $0367E0: jsr $28cd4.l
pub fn i_0367e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x367E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x367E6);
    cpu.pc = 0x28CD4;
    16
}
/// $0367E6: move.w #$87f, $2a(a6)
pub fn i_0367e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x367EC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x87F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0367EC: move.w #$ff6f, $2e(a6)
pub fn i_0367ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x367F2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF6F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0367F2: lea.l $32884(pc), a0
pub fn i_0367f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x367F6;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x32884;
    16
}
/// $0367F6: move.l a0, $48(a6)
pub fn i_0367f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x367FA;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0367FA: jsr $5d5b6.l
pub fn i_0367fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36800;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36800);
    cpu.pc = 0x5D5B6;
    16
}
/// $036800: lea.l $324e8(pc), a0
pub fn i_036800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36804;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x324E8;
    16
}
/// $036804: asl.w #$1, d0
pub fn i_036804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36806;
    cpu.ir = 0xE340;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $036806: move.w (a0, d0.w), d1
pub fn i_036806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3680A;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $03680A: move.w d1, $28(a6)
pub fn i_03680a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3680E;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03680E: lea.l $36814(pc), a1
pub fn i_03680e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36812;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36814;
    16
}
/// $036812: move.l a1, (a6)
pub fn i_036812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36814;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036814: jsr $5d5b6.l
pub fn i_036814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3681A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3681A);
    cpu.pc = 0x5D5B6;
    16
}
/// $03681A: asl.w #$1, d0
pub fn i_03681a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3681C;
    cpu.ir = 0xE340;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $03681C: lea.l $324d8(pc), a0
pub fn i_03681c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36820;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x324D8;
    16
}
/// $036820: move.w (a0, d0.w), $2c(a6)
pub fn i_036820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36826;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad4, v3 as u16);
    22
}
/// $036826: move.w $28(a6), d0
pub fn i_036826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3682A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03682A: move.w #$200, d1
pub fn i_03682a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3682E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x200;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03682E: jsr $267f4.l
pub fn i_03682e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36834;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36834);
    cpu.pc = 0x267F4;
    16
}
/// $036834: move.w d0, $28(a6)
pub fn i_036834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36838;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $036838: move.w $2a(a6), d0
pub fn i_036838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3683C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03683C: bgt.w $3684e
pub fn i_03683c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36840;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x3684E; return 10; }
    8
}
/// $036840: move.w #$780, d1
pub fn i_036840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36844;
    cpu.ir = 0x323C;
    let v1: u32 = 0x780;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $036844: jsr $267f4.l
pub fn i_036844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3684A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3684A);
    cpu.pc = 0x267F4;
    16
}
/// $03684A: move.w d0, $2a(a6)
pub fn i_03684a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3684E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03684E: jsr $33016(pc)
pub fn i_03684e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36852;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x36852);
    cpu.pc = 0x33016;
    16
}
/// $036852: bset.b #$1, $8d(a6)
pub fn i_036852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36858;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036858: jsr $27b66.l
pub fn i_036858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3685E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3685E);
    cpu.pc = 0x27B66;
    16
}
/// $03685E: bcc.w $3686e
pub fn i_03685e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36862;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3686E; return 10; }
    8
}
/// $036862: lea.l $33ec2(pc), a1
pub fn i_036862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36866;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33EC2;
    16
}
/// $036866: move.l a1, (a6)
pub fn i_036866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36868;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036868: bclr.b #$1, $8d(a6)
pub fn i_036868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3686E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03686E: move.w $2a(a6), d0
pub fn i_03686e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36872;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $036872: bgt.w $36892
pub fn i_036872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36876;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x36892; return 10; }
    8
}
/// $036876: move.b #$3, d0
pub fn i_036876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3687A;
    cpu.ir = 0x103C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03687A: and.b $69(a6), d0
pub fn i_03687a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3687E;
    cpu.ir = 0xC02E;
    let ad2 = cpu.a[6].wrapping_add(0x69);
    let v3 = bus.read8(ad2) as u32;
    let r = (v3 & cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $03687E: beq.w $36892
pub fn i_03687e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36882;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x36892; return 10; }
    8
}
/// $036882: btst.b #$2, $69(a6)
pub fn i_036882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36888;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x69);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $036888: beq.w $36892
pub fn i_036888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3688C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x36892; return 10; }
    8
}
/// $03688C: lea.l $368e0(pc), a1
pub fn i_03688c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36890;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x368E0;
    16
}
/// $036890: move.l a1, (a6)
pub fn i_036890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36892;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036892: jsr $32aa8(pc)
pub fn i_036892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36896;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x36896);
    cpu.pc = 0x32AA8;
    16
}
/// $036896: btst.b #$1, $8d(a6)
pub fn i_036896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3689C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03689C: beq.w $368da
pub fn i_03689c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x368A0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x368DA; return 10; }
    8
}
/// $0368A0: cmpi.b #$3, $106ece.l
pub fn i_0368a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x368A8;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    20
}
/// $0368A8: beq.w $368c0
pub fn i_0368a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x368AC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x368C0; return 10; }
    8
}
/// $0368AC: movea.l #$ffffffff, a0
pub fn i_0368ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x368B2;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0368B2: lea.l $324c6(pc), a0
pub fn i_0368b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x368B6;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x324C6;
    16
}
/// $0368B6: jsr $5dd56.l
pub fn i_0368b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x368BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x368BC);
    cpu.pc = 0x5DD56;
    16
}
/// $0368BC: bra.w $368d0
pub fn i_0368bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x368C0;
    cpu.ir = 0x6000;
    cpu.pc = 0x368D0;
    10
}
/// $0368C0: movea.l #$ffffffff, a0
pub fn i_0368c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x368C6;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0368C6: lea.l $324bc(pc), a0
pub fn i_0368c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x368CA;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x324BC;
    16
}
/// $0368CA: jsr $5dd56.l
pub fn i_0368ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x368D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x368D0);
    cpu.pc = 0x5DD56;
    16
}
/// $0368D0: bcc.w $368da
pub fn i_0368d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x368D4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x368DA; return 10; }
    8
}
/// $0368D4: lea.l $37b8e(pc), a1
pub fn i_0368d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x368D8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37B8E;
    16
}
/// $0368D8: move.l a1, (a6)
pub fn i_0368d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x368DA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0368DA: jsr $33522(pc)
pub fn i_0368da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x368DE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x368DE);
    cpu.pc = 0x33522;
    16
}
/// $0368DE: rts
pub fn i_0368de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x368E0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_JumpDropEmptyWeapon_0368e0  ($0368E0, 52 B, player/air_death_crouch_036632.s)
/// $0368E0: cmpi.w #$0, $82(a6)
pub fn i_0368e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x368E6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0368E6: bne.w $36910
pub fn i_0368e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x368EA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x36910; return 10; }
    8
}
/// $0368EA: cmpi.b #$0, $71(a6)
pub fn i_0368ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x368F0;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $0368F0: beq.w $36910
pub fn i_0368f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x368F4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x36910; return 10; }
    8
}
/// $0368F4: lea.l $38be4(pc), a1
pub fn i_0368f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x368F8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38BE4;
    16
}
/// $0368F8: jsr $4ae.l
pub fn i_0368f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x368FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x368FE);
    cpu.pc = 0x4AE;
    16
}
/// $0368FE: jsr $5dd02.l
pub fn i_0368fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36904;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36904);
    cpu.pc = 0x5DD02;
    16
}
/// $036904: move.b #$0, $71(a6)
pub fn i_036904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3690A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03690A: move.w #$a, $82(a6)
pub fn i_03690a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36910;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $036910: bra.w $36914
pub fn i_036910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36914;
    cpu.ir = 0x6000;
    cpu.pc = 0x36914;
    10
}
// ===== Player_JumpStart_036914  ($036914, 348 B, player/air_death_crouch_036632.s)
/// $036914: move.w #$0, $7c(a6)
pub fn i_036914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3691A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03691A: move.w #$0, $7e(a6)
pub fn i_03691a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36920;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $036920: bclr.b #$2, $8c(a6)
pub fn i_036920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36926;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036926: bclr.b #$1, $8c(a6)
pub fn i_036926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3692C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03692C: bclr.b #$3, $8c(a6)
pub fn i_03692c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36932;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036932: move.l #$32500, $60(a6)
pub fn i_036932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3693A;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x32500;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $03693A: tst.w $28(a6)
pub fn i_03693a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3693E;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $03693E: beq.w $3696a
pub fn i_03693e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36942;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3696A; return 10; }
    8
}
/// $036942: move.b #$25, $70(a6)
pub fn i_036942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36948;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x25;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $036948: lea.l $279c78.l, a0
pub fn i_036948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3694E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279C78;
    20
}
/// $03694E: move.l -$4(a0), $74(a6)
pub fn i_03694e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36954;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $036954: move.b #$ff, $21(a6)
pub fn i_036954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3695A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03695A: lea.l $279c78.l, a0
pub fn i_03695a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36960;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279C78;
    20
}
/// $036960: jsr $28cd4.l
pub fn i_036960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36966;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36966);
    cpu.pc = 0x28CD4;
    16
}
/// $036966: bra.w $3698e
pub fn i_036966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3696A;
    cpu.ir = 0x6000;
    cpu.pc = 0x3698E;
    10
}
/// $03696A: move.b #$26, $70(a6)
pub fn i_03696a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36970;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x26;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $036970: lea.l $279b2c.l, a0
pub fn i_036970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36976;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279B2C;
    20
}
/// $036976: move.l -$4(a0), $74(a6)
pub fn i_036976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3697C;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $03697C: move.b #$ff, $21(a6)
pub fn i_03697c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36982;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $036982: lea.l $279b2c.l, a0
pub fn i_036982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36988;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279B2C;
    20
}
/// $036988: jsr $28cd4.l
pub fn i_036988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3698E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3698E);
    cpu.pc = 0x28CD4;
    16
}
/// $03698E: jsr $32f3c(pc)
pub fn i_03698e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36992;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x36992);
    cpu.pc = 0x32F3C;
    16
}
/// $036992: cmpi.b #$8, $78(a6)
pub fn i_036992(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36998;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x8, Size::Byte, false);
    16
}
/// $036998: bne.w $36a16
pub fn i_036998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3699C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x36A16; return 10; }
    8
}
/// $03699C: andi.w #$fffe, $38(a6)
pub fn i_03699c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x369A2;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFFE) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0369A2: move.w #$3, $72(a6)
pub fn i_0369a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x369A8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0369A8: cmpi.b #$26, $70(a6)
pub fn i_0369a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x369AE;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x26, Size::Byte, false);
    16
}
/// $0369AE: beq.w $369e6
pub fn i_0369ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x369B2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x369E6; return 10; }
    8
}
/// $0369B2: move.w #$2, $7c(a6)
pub fn i_0369b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x369B8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0369B8: move.w #$2, $7e(a6)
pub fn i_0369b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x369BE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0369BE: move.b #$25, $70(a6)
pub fn i_0369be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x369C4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x25;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0369C4: lea.l $279b0e.l, a0
pub fn i_0369c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x369CA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279B0E;
    20
}
/// $0369CA: move.l -$4(a0), $74(a6)
pub fn i_0369ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x369D0;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0369D0: move.b #$ff, $21(a6)
pub fn i_0369d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x369D6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0369D6: lea.l $279b0e.l, a0
pub fn i_0369d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x369DC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279B0E;
    20
}
/// $0369DC: jsr $28cd4.l
pub fn i_0369dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x369E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x369E2);
    cpu.pc = 0x28CD4;
    16
}
/// $0369E2: bra.w $36a16
pub fn i_0369e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x369E6;
    cpu.ir = 0x6000;
    cpu.pc = 0x36A16;
    10
}
/// $0369E6: move.w #$0, $7c(a6)
pub fn i_0369e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x369EC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0369EC: move.w #$7, $7e(a6)
pub fn i_0369ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x369F2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0369F2: move.b #$26, $70(a6)
pub fn i_0369f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x369F8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x26;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0369F8: lea.l $279b04.l, a0
pub fn i_0369f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x369FE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279B04;
    20
}
/// $0369FE: move.l -$4(a0), $74(a6)
pub fn i_0369fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36A04;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $036A04: move.b #$ff, $21(a6)
pub fn i_036a04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36A0A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $036A0A: lea.l $279b04.l, a0
pub fn i_036a0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36A10;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279B04;
    20
}
/// $036A10: jsr $28cd4.l
pub fn i_036a10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36A16;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36A16);
    cpu.pc = 0x28CD4;
    16
}
/// $036A16: move.w #$9cd, $2a(a6)
pub fn i_036a16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36A1C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x9CD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $036A1C: move.w #$ff3f, $2e(a6)
pub fn i_036a1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36A22;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF3F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $036A22: move.b #$5, $90(a6)
pub fn i_036a22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36A28;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write8(ad2, v1 as u8);
    12
}
/// $036A28: lea.l $32830(pc), a0
pub fn i_036a28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36A2C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x32830;
    16
}
/// $036A2C: move.l a0, $48(a6)
pub fn i_036a2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36A30;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $036A30: lea.l $36a36(pc), a1
pub fn i_036a30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36A34;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36A36;
    16
}
/// $036A34: move.l a1, (a6)
pub fn i_036a34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36A36;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036A36: subi.b #$1, $90(a6)
pub fn i_036a36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36A3C;
    cpu.ir = 0x42E;
    let ad1 = cpu.a[6].wrapping_add(0x90);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $036A3C: cmpi.b #$4, $90(a6)
pub fn i_036a3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36A42;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Byte, false);
    16
}
/// $036A42: bne.w $36a56
pub fn i_036a42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36A46;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x36A56; return 10; }
    8
}
/// $036A46: jsr $5cd6c.l
pub fn i_036a46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36A4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36A4C);
    cpu.pc = 0x5CD6C;
    16
}
/// $036A4C: bcc.w $36a56
pub fn i_036a4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36A50;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x36A56; return 10; }
    8
}
/// $036A50: lea.l $36a70(pc), a1
pub fn i_036a50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36A54;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36A70;
    16
}
/// $036A54: move.l a1, (a6)
pub fn i_036a54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36A56;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036A56: cmpi.b #$0, $90(a6)
pub fn i_036a56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36A5C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $036A5C: bne.w $36a70
pub fn i_036a5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36A60;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x36A70; return 10; }
    8
}
/// $036A60: move.w $2a(a6), d0
pub fn i_036a60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36A64;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $036A64: asr.w #$2, d0
pub fn i_036a64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36A66;
    cpu.ir = 0xE440;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $036A66: move.w d0, $2a(a6)
pub fn i_036a66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36A6A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $036A6A: lea.l $36a70(pc), a1
pub fn i_036a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36A6E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36A70;
    16
}
/// $036A6E: move.l a1, (a6)
pub fn i_036a6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36A70;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Player_JumpAir_036a70  ($036A70, 540 B, player/air_death_crouch_036632.s)
/// $036A70: jsr $5cde4.l
pub fn i_036a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36A76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36A76);
    cpu.pc = 0x5CDE4;
    16
}
/// $036A76: bcc.w $36aa8
pub fn i_036a76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36A7A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x36AA8; return 10; }
    8
}
/// $036A7A: jsr $5d5b6.l
pub fn i_036a7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36A80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36A80);
    cpu.pc = 0x5D5B6;
    16
}
/// $036A80: asl.w #$1, d0
pub fn i_036a80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36A82;
    cpu.ir = 0xE340;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $036A82: lea.l $324d0(pc), a0
pub fn i_036a82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36A86;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x324D0;
    16
}
/// $036A86: move.w (a0, d0.w), $2c(a6)
pub fn i_036a86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36A8C;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad4, v3 as u16);
    22
}
/// $036A8C: beq.w $36aa4
pub fn i_036a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36A90;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x36AA4; return 10; }
    8
}
/// $036A90: ble.w $36a9e
pub fn i_036a90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36A94;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x36A9E; return 10; }
    8
}
/// $036A94: bclr.b #$0, $3a(a6)
pub fn i_036a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36A9A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036A9A: bra.w $36aa4
pub fn i_036a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36A9E;
    cpu.ir = 0x6000;
    cpu.pc = 0x36AA4;
    10
}
/// $036A9E: bset.b #$0, $3a(a6)
pub fn i_036a9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36AA4;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036AA4: bra.w $36aba
pub fn i_036aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36AA8;
    cpu.ir = 0x6000;
    cpu.pc = 0x36ABA;
    10
}
/// $036AA8: jsr $5d5b6.l
pub fn i_036aa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36AAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36AAE);
    cpu.pc = 0x5D5B6;
    16
}
/// $036AAE: asl.w #$1, d0
pub fn i_036aae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36AB0;
    cpu.ir = 0xE340;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $036AB0: lea.l $324d0(pc), a0
pub fn i_036ab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36AB4;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x324D0;
    16
}
/// $036AB4: move.w (a0, d0.w), $2c(a6)
pub fn i_036ab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36ABA;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad4, v3 as u16);
    22
}
/// $036ABA: cmpi.w #$0, $2c(a6)
pub fn i_036aba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36AC0;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $036AC0: bne.w $36ae8
pub fn i_036ac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36AC4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x36AE8; return 10; }
    8
}
/// $036AC4: cmpi.w #$0, $28(a6)
pub fn i_036ac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36ACA;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $036ACA: beq.w $36ae8
pub fn i_036aca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36ACE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x36AE8; return 10; }
    8
}
/// $036ACE: cmpi.w #$0, $28(a6)
pub fn i_036ace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36AD4;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $036AD4: bmi.w $36ae2
pub fn i_036ad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36AD8;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x36AE2; return 10; }
    8
}
/// $036AD8: move.w #$ffe8, $2c(a6)
pub fn i_036ad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36ADE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFE8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $036ADE: bra.w $36ae8
pub fn i_036ade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36AE2;
    cpu.ir = 0x6000;
    cpu.pc = 0x36AE8;
    10
}
/// $036AE2: move.w #$18, $2c(a6)
pub fn i_036ae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36AE8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x18;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $036AE8: move.w $28(a6), d0
pub fn i_036ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36AEC;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $036AEC: move.w #$400, d1
pub fn i_036aec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36AF0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $036AF0: move.w #$300, d1
pub fn i_036af0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36AF4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $036AF4: jsr $267f4.l
pub fn i_036af4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36AFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36AFA);
    cpu.pc = 0x267F4;
    16
}
/// $036AFA: move.w d0, $28(a6)
pub fn i_036afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36AFE;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $036AFE: move.w $2a(a6), d0
pub fn i_036afe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B02;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $036B02: bgt.w $36b14
pub fn i_036b02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B06;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x36B14; return 10; }
    8
}
/// $036B06: move.w #$780, d1
pub fn i_036b06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B0A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x780;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $036B0A: jsr $267f4.l
pub fn i_036b0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36B10);
    cpu.pc = 0x267F4;
    16
}
/// $036B10: move.w d0, $2a(a6)
pub fn i_036b10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B14;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $036B14: jsr $33016(pc)
pub fn i_036b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B18;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x36B18);
    cpu.pc = 0x33016;
    16
}
/// $036B18: bset.b #$1, $8d(a6)
pub fn i_036b18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B1E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036B1E: bclr.b #$5, $8d(a6)
pub fn i_036b1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B24;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036B24: btst.b #$5, $100001.l
pub fn i_036b24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B2C;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((5) % 8);
    let v2 = bus.read8(0x100001) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $036B2C: beq.w $36b54
pub fn i_036b2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B30;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x36B54; return 10; }
    8
}
/// $036B30: move.w $2e(a6), d0
pub fn i_036b30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B34;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $036B34: movem.w d0, -(a7)
pub fn i_036b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B38;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $036B38: move.w $28(a6), d0
pub fn i_036b38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B3C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $036B3C: movem.w d0, -(a7)
pub fn i_036b3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B40;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $036B40: move.w $2a(a6), d0
pub fn i_036b40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B44;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $036B44: movem.w d0, -(a7)
pub fn i_036b44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B48;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $036B48: clr.w $28(a6)
pub fn i_036b48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B4C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $036B4C: clr.w $2a(a6)
pub fn i_036b4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B50;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $036B50: clr.w $2e(a6)
pub fn i_036b50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B54;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $036B54: jsr $27b66.l
pub fn i_036b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B5A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36B5A);
    cpu.pc = 0x27B66;
    16
}
/// $036B5A: bcc.w $36b70
pub fn i_036b5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B5E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x36B70; return 10; }
    8
}
/// $036B5E: lea.l $33ede(pc), a1
pub fn i_036b5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B62;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33EDE;
    16
}
/// $036B62: move.l a1, (a6)
pub fn i_036b62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B64;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036B64: bclr.b #$1, $8d(a6)
pub fn i_036b64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B6A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036B6A: bclr.b #$5, $8d(a6)
pub fn i_036b6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B70;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036B70: jsr $32aa8(pc)
pub fn i_036b70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B74;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x36B74);
    cpu.pc = 0x32AA8;
    16
}
/// $036B74: bcc.w $36b7e
pub fn i_036b74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B78;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x36B7E; return 10; }
    8
}
/// $036B78: bclr.b #$1, $8c(a6)
pub fn i_036b78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B7E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036B7E: jsr $3705a(pc)
pub fn i_036b7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B82;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x36B82);
    cpu.pc = 0x3705A;
    16
}
/// $036B82: btst.b #$5, $100001.l
pub fn i_036b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B8A;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((5) % 8);
    let v2 = bus.read8(0x100001) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $036B8A: beq.w $36ba6
pub fn i_036b8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B8E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x36BA6; return 10; }
    8
}
/// $036B8E: movem.w (a7)+, d0
pub fn i_036b8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B92;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $036B92: move.w d0, $2a(a6)
pub fn i_036b92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B96;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $036B96: movem.w (a7)+, d0
pub fn i_036b96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B9A;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $036B9A: move.w d0, $28(a6)
pub fn i_036b9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36B9E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $036B9E: movem.w (a7)+, d0
pub fn i_036b9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36BA2;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $036BA2: move.w d0, $2e(a6)
pub fn i_036ba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36BA6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $036BA6: move.w $2a(a6), d0
pub fn i_036ba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36BAA;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $036BAA: bgt.w $36c36
pub fn i_036baa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36BAE;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x36C36; return 10; }
    8
}
/// $036BAE: bset.b #$5, $8d(a6)
pub fn i_036bae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36BB4;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036BB4: btst.b #$0, $8c(a6)
pub fn i_036bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36BBA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $036BBA: beq.w $36bc8
pub fn i_036bba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36BBE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x36BC8; return 10; }
    8
}
/// $036BBE: lea.l $364a2(pc), a1
pub fn i_036bbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36BC2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x364A2;
    16
}
/// $036BC2: move.l a1, (a6)
pub fn i_036bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36BC4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036BC4: jmp $36c3a(pc)
pub fn i_036bc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36BC8;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x36C3A;
    8
}
/// $036BC8: btst.b #$1, $8d(a6)
pub fn i_036bc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36BCE;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $036BCE: beq.w $36bfe
pub fn i_036bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36BD2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x36BFE; return 10; }
    8
}
/// $036BD2: jsr $8f470.l
pub fn i_036bd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36BD8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36BD8);
    cpu.pc = 0x8F470;
    16
}
/// $036BD8: bcc.w $36bfe
pub fn i_036bd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36BDC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x36BFE; return 10; }
    8
}
/// $036BDC: cmpi.w #$0, d2
pub fn i_036bdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36BE0;
    cpu.ir = 0xC42;
    let v1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $036BE0: bmi.w $36bfe
pub fn i_036be0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36BE4;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x36BFE; return 10; }
    8
}
/// $036BE4: move.b d2, $86(a6)
pub fn i_036be4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36BE8;
    cpu.ir = 0x1D42;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write8(ad2, v1 as u8);
    12
}
/// $036BE8: lea.l $36638(pc), a1
pub fn i_036be8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36BEC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36638;
    16
}
/// $036BEC: move.l a1, (a6)
pub fn i_036bec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36BEE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036BEE: jsr $8f520.l
pub fn i_036bee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36BF4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36BF4);
    cpu.pc = 0x8F520;
    16
}
/// $036BF4: bclr.b #$5, $8d(a6)
pub fn i_036bf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36BFA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036BFA: jmp $36c3a(pc)
pub fn i_036bfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36BFE;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x36C3A;
    8
}
/// $036BFE: move.b #$3, d0
pub fn i_036bfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C02;
    cpu.ir = 0x103C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $036C02: and.b $69(a6), d0
pub fn i_036c02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C06;
    cpu.ir = 0xC02E;
    let ad2 = cpu.a[6].wrapping_add(0x69);
    let v3 = bus.read8(ad2) as u32;
    let r = (v3 & cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $036C06: beq.w $36c36
pub fn i_036c06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C0A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x36C36; return 10; }
    8
}
/// $036C0A: btst.b #$2, $69(a6)
pub fn i_036c0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C10;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x69);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $036C10: beq.w $36c36
pub fn i_036c10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C14;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x36C36; return 10; }
    8
}
/// $036C14: lea.l $368e0(pc), a1
pub fn i_036c14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C18;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x368E0;
    16
}
/// $036C18: move.l a1, (a6)
pub fn i_036c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C1A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036C1A: move.w #$400, $28(a6)
pub fn i_036c1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C20;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $036C20: clr.w $2c(a6)
pub fn i_036c20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C24;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $036C24: btst.b #$1, $69(a6)
pub fn i_036c24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C2A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x69);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $036C2A: bne.w $36c32
pub fn i_036c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C2E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x36C32; return 10; }
    8
}
/// $036C2E: neg.w $28(a6)
pub fn i_036c2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C32;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $036C32: jmp $36c3a(pc)
pub fn i_036c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C36;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x36C3A;
    8
}
/// $036C36: jsr $37168(pc)
pub fn i_036c36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C3A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x36C3A);
    cpu.pc = 0x37168;
    16
}
/// $036C3A: btst.b #$1, $8d(a6)
pub fn i_036c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C40;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $036C40: beq.w $36c86
pub fn i_036c40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C44;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x36C86; return 10; }
    8
}
/// $036C44: cmpi.b #$3, $106ece.l
pub fn i_036c44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C4C;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    20
}
/// $036C4C: beq.w $36c64
pub fn i_036c4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C50;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x36C64; return 10; }
    8
}
/// $036C50: movea.l #$ffffffff, a0
pub fn i_036c50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C56;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $036C56: lea.l $324c6(pc), a0
pub fn i_036c56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C5A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x324C6;
    16
}
/// $036C5A: jsr $5dd56.l
pub fn i_036c5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C60;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36C60);
    cpu.pc = 0x5DD56;
    16
}
/// $036C60: bra.w $36c74
pub fn i_036c60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C64;
    cpu.ir = 0x6000;
    cpu.pc = 0x36C74;
    10
}
/// $036C64: movea.l #$ffffffff, a0
pub fn i_036c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C6A;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $036C6A: lea.l $324bc(pc), a0
pub fn i_036c6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C6E;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x324BC;
    16
}
/// $036C6E: jsr $5dd56.l
pub fn i_036c6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C74;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36C74);
    cpu.pc = 0x5DD56;
    16
}
/// $036C74: bcc.w $36c7e
pub fn i_036c74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C78;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x36C7E; return 10; }
    8
}
/// $036C78: lea.l $37b8e(pc), a1
pub fn i_036c78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C7C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37B8E;
    16
}
/// $036C7C: move.l a1, (a6)
pub fn i_036c7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C7E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036C7E: jsr $33522(pc)
pub fn i_036c7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C82;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x36C82);
    cpu.pc = 0x33522;
    16
}
/// $036C82: bra.w $36c8a
pub fn i_036c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C86;
    cpu.ir = 0x6000;
    cpu.pc = 0x36C8A;
    10
}
/// $036C86: jsr $33522(pc)
pub fn i_036c86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C8A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x36C8A);
    cpu.pc = 0x33522;
    16
}
/// $036C8A: rts
pub fn i_036c8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C8C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_SpawnFreeFall_036c8c  ($036C8C, 216 B, player/air_death_crouch_036632.s)
/// $036C8C: cmpa.l #$100440, a6
pub fn i_036c8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C92;
    cpu.ir = 0xBDFC;
    let v1: u32 = 0x100440;
    let h1 = cpu.a[6];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $036C92: bne.w $36ca6
pub fn i_036c92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C96;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x36CA6; return 10; }
    8
}
/// $036C96: lea.l $32112.l, a1
pub fn i_036c96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36C9C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x32112;
    20
}
/// $036C9C: jsr $4ae.l
pub fn i_036c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36CA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36CA2);
    cpu.pc = 0x4AE;
    16
}
/// $036CA2: bra.w $36cb2
pub fn i_036ca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36CA6;
    cpu.ir = 0x6000;
    cpu.pc = 0x36CB2;
    10
}
/// $036CA6: lea.l $32142.l, a1
pub fn i_036ca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36CAC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x32142;
    20
}
/// $036CAC: jsr $4ae.l
pub fn i_036cac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36CB2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36CB2);
    cpu.pc = 0x4AE;
    16
}
/// $036CB2: clr.w $28(a6)
pub fn i_036cb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36CB6;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $036CB6: clr.w $2c(a6)
pub fn i_036cb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36CBA;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $036CBA: move.w #$0, $7c(a6)
pub fn i_036cba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36CC0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $036CC0: move.w #$0, $7e(a6)
pub fn i_036cc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36CC6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $036CC6: bclr.b #$2, $8c(a6)
pub fn i_036cc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36CCC;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036CCC: bclr.b #$1, $8c(a6)
pub fn i_036ccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36CD2;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036CD2: bclr.b #$3, $8c(a6)
pub fn i_036cd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36CD8;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036CD8: move.l #$32500, $60(a6)
pub fn i_036cd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36CE0;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x32500;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $036CE0: move.b #$26, $70(a6)
pub fn i_036ce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36CE6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x26;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $036CE6: lea.l $279b2c.l, a0
pub fn i_036ce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36CEC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279B2C;
    20
}
/// $036CEC: move.l -$4(a0), $74(a6)
pub fn i_036cec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36CF2;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $036CF2: move.b #$ff, $21(a6)
pub fn i_036cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36CF8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $036CF8: lea.l $279b2c.l, a0
pub fn i_036cf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36CFE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279B2C;
    20
}
/// $036CFE: jsr $28cd4.l
pub fn i_036cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36D04);
    cpu.pc = 0x28CD4;
    16
}
/// $036D04: move.w #$ff80, $2e(a6)
pub fn i_036d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D0A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $036D0A: lea.l $ffff.w, a0
pub fn i_036d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D0E;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $036D0E: move.l a0, $48(a6)
pub fn i_036d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D12;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $036D12: clr.b $59(a6)
pub fn i_036d12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D16;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x59);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $036D16: clr.b $45(a6)
pub fn i_036d16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D1A;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x45);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $036D1A: lea.l $36d20(pc), a1
pub fn i_036d1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D1E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36D20;
    16
}
/// $036D1E: move.l a1, (a6)
pub fn i_036d1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D20;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036D20: ori.b #$3c, $59(a6)
pub fn i_036d20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D26;
    cpu.ir = 0x2E;
    let ad1 = cpu.a[6].wrapping_add(0x59);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x3C) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036D26: ori.b #$3c, $45(a6)
pub fn i_036d26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D2C;
    cpu.ir = 0x2E;
    let ad1 = cpu.a[6].wrapping_add(0x45);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x3C) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036D2C: bset.b #$6, $13(a6)
pub fn i_036d2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D32;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036D32: jsr $36e42(pc)
pub fn i_036d32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D36;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x36D36);
    cpu.pc = 0x36E42;
    16
}
/// $036D36: lea.l $33ede(pc), a0
pub fn i_036d36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D3A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x33EDE;
    16
}
/// $036D3A: cmpa.l (a6), a0
pub fn i_036d3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D3C;
    cpu.ir = 0xB1D6;
    let v2 = bus.read32(cpu.a[6]);
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v2, Size::Long, false);
    14
}
/// $036D3C: bne.w $36d4c
pub fn i_036d3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D40;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x36D4C; return 10; }
    8
}
/// $036D40: move.b #$50, $45(a6)
pub fn i_036d40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D46;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x50;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $036D46: move.b #$50, $59(a6)
pub fn i_036d46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D4C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x50;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x59);
    bus.write8(ad2, v1 as u8);
    12
}
/// $036D4C: lea.l $368e0(pc), a0
pub fn i_036d4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D50;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x368E0;
    16
}
/// $036D50: cmpa.l (a6), a0
pub fn i_036d50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D52;
    cpu.ir = 0xB1D6;
    let v2 = bus.read32(cpu.a[6]);
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v2, Size::Long, false);
    14
}
/// $036D52: bne.w $36d62
pub fn i_036d52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D56;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x36D62; return 10; }
    8
}
/// $036D56: move.b #$50, $45(a6)
pub fn i_036d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D5C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x50;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $036D5C: move.b #$50, $59(a6)
pub fn i_036d5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D62;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x50;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x59);
    bus.write8(ad2, v1 as u8);
    12
}
/// $036D62: rts
pub fn i_036d62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D64;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_Knockback_036d64  ($036D64, 102 B, player/air_death_crouch_036632.s)
/// $036D64: clr.w $28(a6)
pub fn i_036d64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D68;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $036D68: clr.w $2c(a6)
pub fn i_036d68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D6C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $036D6C: move.w #$ff80, $2e(a6)
pub fn i_036d6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D72;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $036D72: move.w #$0, $7c(a6)
pub fn i_036d72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D78;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $036D78: move.w #$0, $7e(a6)
pub fn i_036d78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D7E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $036D7E: bclr.b #$2, $8c(a6)
pub fn i_036d7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D84;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036D84: bclr.b #$1, $8c(a6)
pub fn i_036d84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D8A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036D8A: bclr.b #$3, $8c(a6)
pub fn i_036d8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D90;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036D90: move.l #$32500, $60(a6)
pub fn i_036d90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D98;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x32500;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $036D98: move.b #$26, $70(a6)
pub fn i_036d98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36D9E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x26;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $036D9E: lea.l $279b2c.l, a0
pub fn i_036d9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36DA4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279B2C;
    20
}
/// $036DA4: move.l -$4(a0), $74(a6)
pub fn i_036da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36DAA;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $036DAA: move.b #$ff, $21(a6)
pub fn i_036daa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36DB0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $036DB0: lea.l $279b2c.l, a0
pub fn i_036db0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36DB6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279B2C;
    20
}
/// $036DB6: jsr $28cd4.l
pub fn i_036db6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36DBC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36DBC);
    cpu.pc = 0x28CD4;
    16
}
/// $036DBC: lea.l $32830(pc), a0
pub fn i_036dbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36DC0;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x32830;
    16
}
/// $036DC0: move.l a0, $48(a6)
pub fn i_036dc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36DC4;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $036DC4: lea.l $36dca(pc), a1
pub fn i_036dc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36DC8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36DCA;
    16
}
/// $036DC8: move.l a1, (a6)
pub fn i_036dc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36DCA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Player_Knockback_AirCtrl_036dca  ($036DCA, 120 B, player/air_death_crouch_036632.s)
/// $036DCA: jsr $5cde4.l
pub fn i_036dca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36DD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36DD0);
    cpu.pc = 0x5CDE4;
    16
}
/// $036DD0: bcc.w $36e02
pub fn i_036dd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36DD4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x36E02; return 10; }
    8
}
/// $036DD4: jsr $5d5b6.l
pub fn i_036dd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36DDA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36DDA);
    cpu.pc = 0x5D5B6;
    16
}
/// $036DDA: asl.w #$1, d0
pub fn i_036dda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36DDC;
    cpu.ir = 0xE340;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $036DDC: lea.l $324d0(pc), a0
pub fn i_036ddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36DE0;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x324D0;
    16
}
/// $036DE0: move.w (a0, d0.w), $2c(a6)
pub fn i_036de0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36DE6;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad4, v3 as u16);
    22
}
/// $036DE6: beq.w $36dfe
pub fn i_036de6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36DEA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x36DFE; return 10; }
    8
}
/// $036DEA: ble.w $36df8
pub fn i_036dea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36DEE;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x36DF8; return 10; }
    8
}
/// $036DEE: bclr.b #$0, $3a(a6)
pub fn i_036dee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36DF4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036DF4: bra.w $36dfe
pub fn i_036df4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36DF8;
    cpu.ir = 0x6000;
    cpu.pc = 0x36DFE;
    10
}
/// $036DF8: bset.b #$0, $3a(a6)
pub fn i_036df8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36DFE;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036DFE: bra.w $36e14
pub fn i_036dfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E02;
    cpu.ir = 0x6000;
    cpu.pc = 0x36E14;
    10
}
/// $036E02: jsr $5d5b6.l
pub fn i_036e02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36E08);
    cpu.pc = 0x5D5B6;
    16
}
/// $036E08: asl.w #$1, d0
pub fn i_036e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E0A;
    cpu.ir = 0xE340;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $036E0A: lea.l $324d0(pc), a0
pub fn i_036e0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E0E;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x324D0;
    16
}
/// $036E0E: move.w (a0, d0.w), $2c(a6)
pub fn i_036e0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E14;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad4, v3 as u16);
    22
}
/// $036E14: cmpi.w #$0, $2c(a6)
pub fn i_036e14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E1A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $036E1A: bne.w $36e42
pub fn i_036e1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E1E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x36E42; return 10; }
    8
}
/// $036E1E: cmpi.w #$0, $28(a6)
pub fn i_036e1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E24;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $036E24: beq.w $36e42
pub fn i_036e24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E28;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x36E42; return 10; }
    8
}
/// $036E28: cmpi.w #$0, $28(a6)
pub fn i_036e28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E2E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $036E2E: bmi.w $36e3c
pub fn i_036e2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E32;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x36E3C; return 10; }
    8
}
/// $036E32: move.w #$ffe8, $2c(a6)
pub fn i_036e32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E38;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFE8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $036E38: bra.w $36e42
pub fn i_036e38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E3C;
    cpu.ir = 0x6000;
    cpu.pc = 0x36E42;
    10
}
/// $036E3C: move.w #$18, $2c(a6)
pub fn i_036e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E42;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x18;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== Player_Fall_Physics_036e42  ($036E42, 384 B, player/air_death_crouch_036632.s)
/// $036E42: move.w $28(a6), d0
pub fn i_036e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E46;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $036E46: move.w #$400, d1
pub fn i_036e46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E4A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $036E4A: move.w #$300, d1
pub fn i_036e4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E4E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $036E4E: jsr $267f4.l
pub fn i_036e4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36E54);
    cpu.pc = 0x267F4;
    16
}
/// $036E54: move.w d0, $28(a6)
pub fn i_036e54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E58;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $036E58: move.w $2a(a6), d0
pub fn i_036e58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E5C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $036E5C: bgt.w $36e72
pub fn i_036e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E60;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x36E72; return 10; }
    8
}
/// $036E60: move.w $2a(a6), d0
pub fn i_036e60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E64;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $036E64: move.w #$400, d1
pub fn i_036e64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E68;
    cpu.ir = 0x323C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $036E68: jsr $267f4.l
pub fn i_036e68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E6E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36E6E);
    cpu.pc = 0x267F4;
    16
}
/// $036E6E: move.w d0, $2a(a6)
pub fn i_036e6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E72;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $036E72: bset.b #$1, $8d(a6)
pub fn i_036e72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E78;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036E78: bclr.b #$5, $8d(a6)
pub fn i_036e78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E7E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036E7E: jsr $33016(pc)
pub fn i_036e7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E82;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x36E82);
    cpu.pc = 0x33016;
    16
}
/// $036E82: jsr $27bc8.l
pub fn i_036e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36E88);
    cpu.pc = 0x27BC8;
    16
}
/// $036E88: bcc.w $36e9e
pub fn i_036e88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E8C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x36E9E; return 10; }
    8
}
/// $036E8C: lea.l $33ede(pc), a1
pub fn i_036e8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E90;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33EDE;
    16
}
/// $036E90: move.l a1, (a6)
pub fn i_036e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E92;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036E92: bclr.b #$1, $8d(a6)
pub fn i_036e92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E98;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036E98: bclr.b #$5, $8d(a6)
pub fn i_036e98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36E9E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036E9E: jsr $32aa8(pc)
pub fn i_036e9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36EA2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x36EA2);
    cpu.pc = 0x32AA8;
    16
}
/// $036EA2: bcc.w $36eac
pub fn i_036ea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36EA6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x36EAC; return 10; }
    8
}
/// $036EA6: bclr.b #$1, $8c(a6)
pub fn i_036ea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36EAC;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036EAC: cmpi.l #$36ff6, (a6)
pub fn i_036eac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36EB2;
    cpu.ir = 0xC96;
    let v2 = bus.read32(cpu.a[6]);
    sub_with_flags(cpu, v2, 0x36FF6, Size::Long, false);
    16
}
/// $036EB2: beq.b $36ebe
pub fn i_036eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36EB4;
    cpu.ir = 0x670A;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x36EBE; return 10; }
    8
}
/// $036EB4: cmpi.l #$37046, (a6)
pub fn i_036eb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36EBA;
    cpu.ir = 0xC96;
    let v2 = bus.read32(cpu.a[6]);
    sub_with_flags(cpu, v2, 0x37046, Size::Long, false);
    16
}
/// $036EBA: bne.w $36ec2
pub fn i_036eba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36EBE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x36EC2; return 10; }
    8
}
/// $036EBE: bra.w $36ec6
pub fn i_036ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36EC2;
    cpu.ir = 0x6000;
    cpu.pc = 0x36EC6;
    10
}
/// $036EC2: jsr $3705a(pc)
pub fn i_036ec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36EC6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x36EC6);
    cpu.pc = 0x3705A;
    16
}
/// $036EC6: move.w $2a(a6), d0
pub fn i_036ec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36ECA;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $036ECA: bgt.w $36f56
pub fn i_036eca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36ECE;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x36F56; return 10; }
    8
}
/// $036ECE: bset.b #$5, $8d(a6)
pub fn i_036ece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36ED4;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036ED4: btst.b #$0, $8c(a6)
pub fn i_036ed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36EDA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $036EDA: beq.w $36ee8
pub fn i_036eda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36EDE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x36EE8; return 10; }
    8
}
/// $036EDE: lea.l $364a2(pc), a1
pub fn i_036ede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36EE2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x364A2;
    16
}
/// $036EE2: move.l a1, (a6)
pub fn i_036ee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36EE4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036EE4: jmp $36c3a(pc)
pub fn i_036ee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36EE8;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x36C3A;
    8
}
/// $036EE8: btst.b #$1, $8d(a6)
pub fn i_036ee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36EEE;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $036EEE: beq.w $36f1e
pub fn i_036eee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36EF2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x36F1E; return 10; }
    8
}
/// $036EF2: jsr $8f470.l
pub fn i_036ef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36EF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36EF8);
    cpu.pc = 0x8F470;
    16
}
/// $036EF8: bcc.w $36f1e
pub fn i_036ef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36EFC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x36F1E; return 10; }
    8
}
/// $036EFC: cmpi.w #$0, d2
pub fn i_036efc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F00;
    cpu.ir = 0xC42;
    let v1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $036F00: bmi.w $36f1e
pub fn i_036f00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F04;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x36F1E; return 10; }
    8
}
/// $036F04: move.b d2, $86(a6)
pub fn i_036f04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F08;
    cpu.ir = 0x1D42;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write8(ad2, v1 as u8);
    12
}
/// $036F08: lea.l $36638(pc), a1
pub fn i_036f08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F0C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36638;
    16
}
/// $036F0C: move.l a1, (a6)
pub fn i_036f0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F0E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036F0E: jsr $8f520.l
pub fn i_036f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36F14);
    cpu.pc = 0x8F520;
    16
}
/// $036F14: bclr.b #$5, $8d(a6)
pub fn i_036f14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F1A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036F1A: jmp $36f70(pc)
pub fn i_036f1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F1E;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x36F70;
    8
}
/// $036F1E: move.b #$3, d0
pub fn i_036f1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F22;
    cpu.ir = 0x103C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $036F22: and.b $69(a6), d0
pub fn i_036f22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F26;
    cpu.ir = 0xC02E;
    let ad2 = cpu.a[6].wrapping_add(0x69);
    let v3 = bus.read8(ad2) as u32;
    let r = (v3 & cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $036F26: beq.w $36f56
pub fn i_036f26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F2A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x36F56; return 10; }
    8
}
/// $036F2A: btst.b #$2, $69(a6)
pub fn i_036f2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F30;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x69);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $036F30: beq.w $36f56
pub fn i_036f30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F34;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x36F56; return 10; }
    8
}
/// $036F34: lea.l $368e0(pc), a1
pub fn i_036f34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F38;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x368E0;
    16
}
/// $036F38: move.l a1, (a6)
pub fn i_036f38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F3A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036F3A: move.w #$400, $28(a6)
pub fn i_036f3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F40;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $036F40: clr.w $2c(a6)
pub fn i_036f40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F44;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $036F44: btst.b #$1, $69(a6)
pub fn i_036f44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F4A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x69);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $036F4A: bne.w $36f52
pub fn i_036f4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F4E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x36F52; return 10; }
    8
}
/// $036F4E: neg.w $28(a6)
pub fn i_036f4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F52;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $036F52: jmp $36f70(pc)
pub fn i_036f52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F56;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x36F70;
    8
}
/// $036F56: cmpi.l #$36ff6, (a6)
pub fn i_036f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F5C;
    cpu.ir = 0xC96;
    let v2 = bus.read32(cpu.a[6]);
    sub_with_flags(cpu, v2, 0x36FF6, Size::Long, false);
    16
}
/// $036F5C: beq.b $36f68
pub fn i_036f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F5E;
    cpu.ir = 0x670A;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x36F68; return 10; }
    8
}
/// $036F5E: cmpi.l #$37046, (a6)
pub fn i_036f5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F64;
    cpu.ir = 0xC96;
    let v2 = bus.read32(cpu.a[6]);
    sub_with_flags(cpu, v2, 0x37046, Size::Long, false);
    16
}
/// $036F64: bne.w $36f6c
pub fn i_036f64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F68;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x36F6C; return 10; }
    8
}
/// $036F68: bra.w $36f70
pub fn i_036f68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F6C;
    cpu.ir = 0x6000;
    cpu.pc = 0x36F70;
    10
}
/// $036F6C: jsr $37168(pc)
pub fn i_036f6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F70;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x36F70);
    cpu.pc = 0x37168;
    16
}
/// $036F70: btst.b #$1, $8d(a6)
pub fn i_036f70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F76;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $036F76: beq.w $36fbc
pub fn i_036f76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F7A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x36FBC; return 10; }
    8
}
/// $036F7A: cmpi.b #$3, $106ece.l
pub fn i_036f7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F82;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    20
}
/// $036F82: beq.w $36f9a
pub fn i_036f82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F86;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x36F9A; return 10; }
    8
}
/// $036F86: movea.l #$ffffffff, a0
pub fn i_036f86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F8C;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $036F8C: lea.l $324c6(pc), a0
pub fn i_036f8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F90;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x324C6;
    16
}
/// $036F90: jsr $5dd56.l
pub fn i_036f90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36F96);
    cpu.pc = 0x5DD56;
    16
}
/// $036F96: bra.w $36faa
pub fn i_036f96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36F9A;
    cpu.ir = 0x6000;
    cpu.pc = 0x36FAA;
    10
}
/// $036F9A: movea.l #$ffffffff, a0
pub fn i_036f9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36FA0;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $036FA0: lea.l $324bc(pc), a0
pub fn i_036fa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36FA4;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x324BC;
    16
}
/// $036FA4: jsr $5dd56.l
pub fn i_036fa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36FAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36FAA);
    cpu.pc = 0x5DD56;
    16
}
/// $036FAA: bcc.w $36fb4
pub fn i_036faa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36FAE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x36FB4; return 10; }
    8
}
/// $036FAE: lea.l $37b8e(pc), a1
pub fn i_036fae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36FB2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37B8E;
    16
}
/// $036FB2: move.l a1, (a6)
pub fn i_036fb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36FB4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036FB4: jsr $33522(pc)
pub fn i_036fb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36FB8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x36FB8);
    cpu.pc = 0x33522;
    16
}
/// $036FB8: bra.w $36fc0
pub fn i_036fb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36FBC;
    cpu.ir = 0x6000;
    cpu.pc = 0x36FC0;
    10
}
/// $036FBC: jsr $33522(pc)
pub fn i_036fbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36FC0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x36FC0);
    cpu.pc = 0x33522;
    16
}
/// $036FC0: rts
pub fn i_036fc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36FC2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_KnockbackDelay_036fc2  ($036FC2, 86 B, player/air_death_crouch_036632.s)
/// $036FC2: clr.w $28(a6)
pub fn i_036fc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36FC6;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $036FC6: clr.w $2c(a6)
pub fn i_036fc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36FCA;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $036FCA: bclr.b #$2, $8c(a6)
pub fn i_036fca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36FD0;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036FD0: bclr.b #$1, $8c(a6)
pub fn i_036fd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36FD6;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036FD6: bclr.b #$3, $8c(a6)
pub fn i_036fd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36FDC;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036FDC: move.l #$32500, $60(a6)
pub fn i_036fdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36FE4;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x32500;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $036FE4: move.w #$ff80, $2e(a6)
pub fn i_036fe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36FEA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $036FEA: move.b #$c, $46(a6)
pub fn i_036fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36FF0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x46);
    bus.write8(ad2, v1 as u8);
    12
}
/// $036FF0: lea.l $36ff6(pc), a1
pub fn i_036ff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36FF4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36FF6;
    16
}
/// $036FF4: move.l a1, (a6)
pub fn i_036ff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36FF6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036FF6: cmpi.b #$0, $46(a6)
pub fn i_036ff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36FFC;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x46);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $036FFC: bgt.w $37004
pub fn i_036ffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37000;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x37004; return 10; }
    8
}
/// $037000: jmp $36d72(pc)
pub fn i_037000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37004;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x36D72;
    8
}
/// $037004: jsr $5cde4.l
pub fn i_037004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3700A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3700A);
    cpu.pc = 0x5CDE4;
    16
}
/// $03700A: bcc.w $37012
pub fn i_03700a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3700E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x37012; return 10; }
    8
}
/// $03700E: jmp $36d72(pc)
pub fn i_03700e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37012;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x36D72;
    8
}
/// $037012: jsr $36dca(pc)
pub fn i_037012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37016;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37016);
    cpu.pc = 0x36DCA;
    16
}
/// $037016: rts
pub fn i_037016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37018;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_KnockbackHold_037018  ($037018, 66 B, player/air_death_crouch_036632.s)
/// $037018: clr.w $28(a6)
pub fn i_037018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3701C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $03701C: clr.w $2c(a6)
pub fn i_03701c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37020;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $037020: bclr.b #$2, $8c(a6)
pub fn i_037020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37026;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $037026: bclr.b #$1, $8c(a6)
pub fn i_037026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3702C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03702C: bclr.b #$3, $8c(a6)
pub fn i_03702c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37032;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $037032: move.l #$32500, $60(a6)
pub fn i_037032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3703A;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x32500;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $03703A: move.w #$ff80, $2e(a6)
pub fn i_03703a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37040;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037040: lea.l $37046(pc), a1
pub fn i_037040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37044;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37046;
    16
}
/// $037044: move.l a1, (a6)
pub fn i_037044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37046;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037046: jsr $5cde4.l
pub fn i_037046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3704C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3704C);
    cpu.pc = 0x5CDE4;
    16
}
/// $03704C: bcc.w $37054
pub fn i_03704c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37050;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x37054; return 10; }
    8
}
/// $037050: jmp $36d72(pc)
pub fn i_037050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37054;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x36D72;
    8
}
/// $037054: jsr $36dca(pc)
pub fn i_037054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37058;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37058);
    cpu.pc = 0x36DCA;
    16
}
/// $037058: rts
pub fn i_037058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3705A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerAnimState_03705A  ($03705A, 270 B, player/helpers_033638.s)
/// $03705A: btst.b #$2, $8c(a6)
pub fn i_03705a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37060;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $037060: bne.w $37166
pub fn i_037060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37064;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37166; return 10; }
    8
}
/// $037064: btst.b #$1, $8c(a6)
pub fn i_037064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3706A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03706A: bne.w $37166
pub fn i_03706a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3706E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37166; return 10; }
    8
}
/// $03706E: cmpi.b #$8, $78(a6)
pub fn i_03706e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37074;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x8, Size::Byte, false);
    16
}
/// $037074: bne.w $370ec
pub fn i_037074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37078;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x370EC; return 10; }
    8
}
/// $037078: cmpi.w #$3, $72(a6)
pub fn i_037078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3707E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Word, false);
    16
}
/// $03707E: beq.w $370e8
pub fn i_03707e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37082;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x370E8; return 10; }
    8
}
/// $037082: andi.w #$fffe, $38(a6)
pub fn i_037082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37088;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFFE) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $037088: move.w #$3, $72(a6)
pub fn i_037088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3708E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03708E: cmpi.b #$26, $70(a6)
pub fn i_03708e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37094;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x26, Size::Byte, false);
    16
}
/// $037094: beq.w $370ba
pub fn i_037094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37098;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x370BA; return 10; }
    8
}
/// $037098: move.w #$2, $7c(a6)
pub fn i_037098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3709E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03709E: move.w #$2, $7e(a6)
pub fn i_03709e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x370A4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0370A4: move.b #$25, $70(a6)
pub fn i_0370a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x370AA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x25;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0370AA: lea.l $279b0e.l, a0
pub fn i_0370aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x370B0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279B0E;
    20
}
/// $0370B0: move.l -$4(a0), $74(a6)
pub fn i_0370b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x370B6;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0370B6: bra.w $370d8
pub fn i_0370b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x370BA;
    cpu.ir = 0x6000;
    cpu.pc = 0x370D8;
    10
}
/// $0370BA: move.w #$0, $7c(a6)
pub fn i_0370ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x370C0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0370C0: move.w #$7, $7e(a6)
pub fn i_0370c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x370C6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0370C6: move.b #$26, $70(a6)
pub fn i_0370c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x370CC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x26;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0370CC: lea.l $279b04.l, a0
pub fn i_0370cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x370D2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279B04;
    20
}
/// $0370D2: move.l -$4(a0), $74(a6)
pub fn i_0370d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x370D8;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0370D8: cmpi.b #$18, $79(a6)
pub fn i_0370d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x370DE;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x79);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x18, Size::Byte, false);
    16
}
/// $0370DE: bne.w $370e8
pub fn i_0370de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x370E2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x370E8; return 10; }
    8
}
/// $0370E2: bset.b #$5, $69(a6)
pub fn i_0370e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x370E8;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x69);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0370E8: bra.w $37166
pub fn i_0370e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x370EC;
    cpu.ir = 0x6000;
    cpu.pc = 0x37166;
    10
}
/// $0370EC: cmpi.w #$0, $72(a6)
pub fn i_0370ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x370F2;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0370F2: beq.w $37166
pub fn i_0370f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x370F6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x37166; return 10; }
    8
}
/// $0370F6: move.w #$0, $72(a6)
pub fn i_0370f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x370FC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0370FC: cmpi.b #$26, $70(a6)
pub fn i_0370fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37102;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x26, Size::Byte, false);
    16
}
/// $037102: beq.w $37138
pub fn i_037102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37106;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x37138; return 10; }
    8
}
/// $037106: move.w #$0, $7c(a6)
pub fn i_037106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3710C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03710C: move.w #$2, $7e(a6)
pub fn i_03710c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37112;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037112: move.b #$25, $70(a6)
pub fn i_037112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37118;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x25;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $037118: lea.l $279b22.l, a0
pub fn i_037118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3711E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279B22;
    20
}
/// $03711E: move.l -$4(a0), $74(a6)
pub fn i_03711e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37124;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $037124: cmpi.b #$0, $71(a6)
pub fn i_037124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3712A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $03712A: bne.w $37134
pub fn i_03712a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3712E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37134; return 10; }
    8
}
/// $03712E: ori.w #$1, $38(a6)
pub fn i_03712e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37134;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $037134: bra.w $37156
pub fn i_037134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37138;
    cpu.ir = 0x6000;
    cpu.pc = 0x37156;
    10
}
/// $037138: move.w #$0, $7c(a6)
pub fn i_037138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3713E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03713E: move.w #$7, $7e(a6)
pub fn i_03713e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37144;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037144: move.b #$26, $70(a6)
pub fn i_037144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3714A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x26;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03714A: lea.l $279b18.l, a0
pub fn i_03714a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37150;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279B18;
    20
}
/// $037150: move.l -$4(a0), $74(a6)
pub fn i_037150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37156;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $037156: cmpi.b #$81, $79(a6)
pub fn i_037156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3715C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x79);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x81, Size::Byte, false);
    16
}
/// $03715C: bne.w $37166
pub fn i_03715c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37160;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37166; return 10; }
    8
}
/// $037160: bset.b #$5, $69(a6)
pub fn i_037160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37166;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x69);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $037166: rts
pub fn i_037166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37168;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_AirActionSelect_037168  ($037168, 1130 B, player/air_death_crouch_036632.s)
/// $037168: jsr $330d0(pc)
pub fn i_037168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3716C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3716C);
    cpu.pc = 0x330D0;
    16
}
/// $03716C: bcc.w $375d0
pub fn i_03716c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37170;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x375D0; return 10; }
    8
}
/// $037170: bset.b #$2, $8c(a6)
pub fn i_037170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37176;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $037176: cmpi.b #$ff, d1
pub fn i_037176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3717A;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $03717A: bne.w $371d8
pub fn i_03717a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3717E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x371D8; return 10; }
    8
}
/// $03717E: bset.b #$5, $69(a6)
pub fn i_03717e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37184;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x69);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $037184: cmpi.b #$26, $70(a6)
pub fn i_037184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3718A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x26, Size::Byte, false);
    16
}
/// $03718A: beq.w $371b0
pub fn i_03718a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3718E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x371B0; return 10; }
    8
}
/// $03718E: move.w #$0, $7c(a6)
pub fn i_03718e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37194;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037194: move.w #$2, $7e(a6)
pub fn i_037194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3719A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03719A: move.b #$25, $70(a6)
pub fn i_03719a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x371A0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x25;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0371A0: lea.l $279d56.l, a0
pub fn i_0371a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x371A6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D56;
    20
}
/// $0371A6: move.l -$4(a0), $74(a6)
pub fn i_0371a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x371AC;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0371AC: bra.w $371ce
pub fn i_0371ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x371B0;
    cpu.ir = 0x6000;
    cpu.pc = 0x371CE;
    10
}
/// $0371B0: move.w #$0, $7c(a6)
pub fn i_0371b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x371B6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0371B6: move.w #$7, $7e(a6)
pub fn i_0371b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x371BC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0371BC: move.b #$26, $70(a6)
pub fn i_0371bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x371C2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x26;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0371C2: lea.l $279d56.l, a0
pub fn i_0371c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x371C8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D56;
    20
}
/// $0371C8: move.l -$4(a0), $74(a6)
pub fn i_0371c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x371CE;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0371CE: move.w #$0, $72(a6)
pub fn i_0371ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x371D4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0371D4: bra.w $375d0
pub fn i_0371d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x371D8;
    cpu.ir = 0x6000;
    cpu.pc = 0x375D0;
    10
}
/// $0371D8: bset.b #$5, $69(a6)
pub fn i_0371d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x371DE;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x69);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0371DE: cmpi.b #$3, d1
pub fn i_0371de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x371E2;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $0371E2: bne.w $37328
pub fn i_0371e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x371E6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37328; return 10; }
    8
}
/// $0371E6: cmpi.b #$26, $70(a6)
pub fn i_0371e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x371EC;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x26, Size::Byte, false);
    16
}
/// $0371EC: beq.w $3729e
pub fn i_0371ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x371F0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3729E; return 10; }
    8
}
/// $0371F0: move.w #$0, $7c(a6)
pub fn i_0371f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x371F6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0371F6: move.w #$0, $7e(a6)
pub fn i_0371f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x371FC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0371FC: jsr $5d5b6.l
pub fn i_0371fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37202;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37202);
    cpu.pc = 0x5D5B6;
    16
}
/// $037202: btst.b #$0, $3a(a6)
pub fn i_037202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37208;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $037208: bne.w $37214
pub fn i_037208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3720C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37214; return 10; }
    8
}
/// $03720C: move.b #$1, d2
pub fn i_03720c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37210;
    cpu.ir = 0x143C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $037210: bra.w $37218
pub fn i_037210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37214;
    cpu.ir = 0x6000;
    cpu.pc = 0x37218;
    10
}
/// $037214: move.b #$2, d2
pub fn i_037214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37218;
    cpu.ir = 0x143C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $037218: cmpi.b #$0, d0
pub fn i_037218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3721C;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $03721C: bne.w $3723c
pub fn i_03721c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37220;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3723C; return 10; }
    8
}
/// $037220: cmpi.b #$1, $78(a6)
pub fn i_037220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37226;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $037226: bne.w $3722e
pub fn i_037226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3722A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3722E; return 10; }
    8
}
/// $03722A: move.b #$ff, d2
pub fn i_03722a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3722E;
    cpu.ir = 0x143C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03722E: cmpi.b #$2, $78(a6)
pub fn i_03722e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37234;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $037234: bne.w $3723c
pub fn i_037234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37238;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3723C; return 10; }
    8
}
/// $037238: move.b #$0, d2
pub fn i_037238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3723C;
    cpu.ir = 0x143C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03723C: btst.b #$2, $100001.l
pub fn i_03723c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37244;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((2) % 8);
    let v2 = bus.read8(0x100001) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $037244: bne.w $37248
pub fn i_037244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37248;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37248; return 10; }
    8
}
/// $037248: cmp.b d0, d2
pub fn i_037248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3724A;
    cpu.ir = 0xB400;
    let v1 = cpu.d[0] & 0xFF;
    let h1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $03724A: bne.w $37276
pub fn i_03724a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3724E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37276; return 10; }
    8
}
/// $03724E: move.w #$3, $7c(a6)
pub fn i_03724e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37254;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037254: move.w #$fffa, $7e(a6)
pub fn i_037254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3725A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03725A: move.b #$25, $70(a6)
pub fn i_03725a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37260;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x25;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $037260: lea.l $279d4c.l, a0
pub fn i_037260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37266;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D4C;
    20
}
/// $037266: move.l -$4(a0), $74(a6)
pub fn i_037266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3726C;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $03726C: move.w #$2, $72(a6)
pub fn i_03726c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37272;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037272: bra.w $3729a
pub fn i_037272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37276;
    cpu.ir = 0x6000;
    cpu.pc = 0x3729A;
    10
}
/// $037276: move.w #$0, $7c(a6)
pub fn i_037276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3727C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03727C: move.w #$2, $7e(a6)
pub fn i_03727c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37282;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037282: move.b #$25, $70(a6)
pub fn i_037282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37288;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x25;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $037288: lea.l $279d42.l, a0
pub fn i_037288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3728E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D42;
    20
}
/// $03728E: move.l -$4(a0), $74(a6)
pub fn i_03728e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37294;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $037294: move.w #$0, $72(a6)
pub fn i_037294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3729A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03729A: bra.w $37324
pub fn i_03729a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3729E;
    cpu.ir = 0x6000;
    cpu.pc = 0x37324;
    10
}
/// $03729E: move.w #$6, $7c(a6)
pub fn i_03729e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x372A4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0372A4: move.w #$3, $7e(a6)
pub fn i_0372a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x372AA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0372AA: jsr $5d5b6.l
pub fn i_0372aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x372B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x372B0);
    cpu.pc = 0x5D5B6;
    16
}
/// $0372B0: btst.b #$0, $3a(a6)
pub fn i_0372b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x372B6;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0372B6: bne.w $372c2
pub fn i_0372b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x372BA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x372C2; return 10; }
    8
}
/// $0372BA: move.b #$1, d2
pub fn i_0372ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x372BE;
    cpu.ir = 0x143C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0372BE: bra.w $372c6
pub fn i_0372be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x372C2;
    cpu.ir = 0x6000;
    cpu.pc = 0x372C6;
    10
}
/// $0372C2: move.b #$2, d2
pub fn i_0372c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x372C6;
    cpu.ir = 0x143C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0372C6: btst.b #$2, $100001.l
pub fn i_0372c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x372CE;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((2) % 8);
    let v2 = bus.read8(0x100001) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $0372CE: bne.w $372d2
pub fn i_0372ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x372D2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x372D2; return 10; }
    8
}
/// $0372D2: cmp.b d0, d2
pub fn i_0372d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x372D4;
    cpu.ir = 0xB400;
    let v1 = cpu.d[0] & 0xFF;
    let h1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $0372D4: bne.w $37300
pub fn i_0372d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x372D8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37300; return 10; }
    8
}
/// $0372D8: move.w #$0, $7c(a6)
pub fn i_0372d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x372DE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0372DE: move.w #$0, $7e(a6)
pub fn i_0372de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x372E4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0372E4: move.b #$26, $70(a6)
pub fn i_0372e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x372EA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x26;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0372EA: lea.l $279d4c.l, a0
pub fn i_0372ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x372F0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D4C;
    20
}
/// $0372F0: move.l -$4(a0), $74(a6)
pub fn i_0372f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x372F6;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0372F6: move.w #$2, $72(a6)
pub fn i_0372f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x372FC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0372FC: bra.w $37324
pub fn i_0372fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37300;
    cpu.ir = 0x6000;
    cpu.pc = 0x37324;
    10
}
/// $037300: move.w #$0, $7c(a6)
pub fn i_037300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37306;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037306: move.w #$7, $7e(a6)
pub fn i_037306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3730C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03730C: move.b #$26, $70(a6)
pub fn i_03730c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37312;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x26;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $037312: lea.l $279d42.l, a0
pub fn i_037312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37318;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D42;
    20
}
/// $037318: move.l -$4(a0), $74(a6)
pub fn i_037318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3731E;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $03731E: move.w #$0, $72(a6)
pub fn i_03731e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37324;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037324: bra.w $375d0
pub fn i_037324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37328;
    cpu.ir = 0x6000;
    cpu.pc = 0x375D0;
    10
}
/// $037328: cmpi.b #$4, d1
pub fn i_037328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3732C;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x4, Size::Byte, false);
    8
}
/// $03732C: bne.w $37390
pub fn i_03732c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37330;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37390; return 10; }
    8
}
/// $037330: andi.w #$fffe, $38(a6)
pub fn i_037330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37336;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFFE) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $037336: cmpi.b #$26, $70(a6)
pub fn i_037336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3733C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x26, Size::Byte, false);
    16
}
/// $03733C: beq.w $37362
pub fn i_03733c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37340;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x37362; return 10; }
    8
}
/// $037340: move.w #$2, $7c(a6)
pub fn i_037340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37346;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037346: move.w #$2, $7e(a6)
pub fn i_037346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3734C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03734C: move.b #$25, $70(a6)
pub fn i_03734c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37352;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x25;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $037352: lea.l $279c6e.l, a0
pub fn i_037352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37358;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279C6E;
    20
}
/// $037358: move.l -$4(a0), $74(a6)
pub fn i_037358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3735E;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $03735E: bra.w $37380
pub fn i_03735e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37362;
    cpu.ir = 0x6000;
    cpu.pc = 0x37380;
    10
}
/// $037362: move.w #$0, $7c(a6)
pub fn i_037362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37368;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037368: move.w #$7, $7e(a6)
pub fn i_037368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3736E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03736E: move.b #$26, $70(a6)
pub fn i_03736e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37374;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x26;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $037374: lea.l $279afa.l, a0
pub fn i_037374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3737A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279AFA;
    20
}
/// $03737A: move.l -$4(a0), $74(a6)
pub fn i_03737a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37380;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $037380: move.w #$3, $72(a6)
pub fn i_037380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37386;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037386: bset.b #$1, $8c(a6)
pub fn i_037386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3738C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03738C: bra.w $375d0
pub fn i_03738c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37390;
    cpu.ir = 0x6000;
    cpu.pc = 0x375D0;
    10
}
/// $037390: cmpi.b #$1, d1
pub fn i_037390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37394;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $037394: bne.w $373f2
pub fn i_037394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37398;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x373F2; return 10; }
    8
}
/// $037398: cmpi.b #$26, $70(a6)
pub fn i_037398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3739E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x26, Size::Byte, false);
    16
}
/// $03739E: beq.w $373c4
pub fn i_03739e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x373A2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x373C4; return 10; }
    8
}
/// $0373A2: move.w #$0, $7c(a6)
pub fn i_0373a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x373A8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0373A8: move.w #$2, $7e(a6)
pub fn i_0373a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x373AE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0373AE: move.b #$25, $70(a6)
pub fn i_0373ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x373B4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x25;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0373B4: lea.l $279c3c.l, a0
pub fn i_0373b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x373BA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279C3C;
    20
}
/// $0373BA: move.l -$4(a0), $74(a6)
pub fn i_0373ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x373C0;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0373C0: bra.w $373e2
pub fn i_0373c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x373C4;
    cpu.ir = 0x6000;
    cpu.pc = 0x373E2;
    10
}
/// $0373C4: move.w #$0, $7c(a6)
pub fn i_0373c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x373CA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0373CA: move.w #$6, $7e(a6)
pub fn i_0373ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x373D0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0373D0: move.b #$26, $70(a6)
pub fn i_0373d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x373D6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x26;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0373D6: lea.l $279ad2.l, a0
pub fn i_0373d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x373DC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279AD2;
    20
}
/// $0373DC: move.l -$4(a0), $74(a6)
pub fn i_0373dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x373E2;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0373E2: move.w #$1, $72(a6)
pub fn i_0373e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x373E8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0373E8: bset.b #$1, $8c(a6)
pub fn i_0373e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x373EE;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0373EE: bra.w $375d0
pub fn i_0373ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x373F2;
    cpu.ir = 0x6000;
    cpu.pc = 0x375D0;
    10
}
/// $0373F2: btst.b #$2, $100001.l
pub fn i_0373f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x373FA;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((2) % 8);
    let v2 = bus.read8(0x100001) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $0373FA: bne.w $373fe
pub fn i_0373fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x373FE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x373FE; return 10; }
    8
}
/// $0373FE: cmpi.b #$2, d1
pub fn i_0373fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37402;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $037402: bne.w $3746c
pub fn i_037402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37406;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3746C; return 10; }
    8
}
/// $037406: cmpi.b #$26, $70(a6)
pub fn i_037406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3740C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x26, Size::Byte, false);
    16
}
/// $03740C: beq.w $37438
pub fn i_03740c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37410;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x37438; return 10; }
    8
}
/// $037410: move.w #$6, $7c(a6)
pub fn i_037410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37416;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037416: move.w #$fffd, $7e(a6)
pub fn i_037416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3741C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03741C: move.b #$25, $70(a6)
pub fn i_03741c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37422;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x25;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $037422: lea.l $279c32.l, a0
pub fn i_037422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37428;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279C32;
    20
}
/// $037428: move.l -$4(a0), $74(a6)
pub fn i_037428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3742E;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $03742E: move.w #$2, $72(a6)
pub fn i_03742e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37434;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037434: bra.w $3745c
pub fn i_037434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37438;
    cpu.ir = 0x6000;
    cpu.pc = 0x3745C;
    10
}
/// $037438: move.w #$0, $7c(a6)
pub fn i_037438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3743E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03743E: move.w #$0, $7e(a6)
pub fn i_03743e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37444;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037444: move.b #$26, $70(a6)
pub fn i_037444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3744A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x26;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03744A: lea.l $279ac8.l, a0
pub fn i_03744a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37450;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279AC8;
    20
}
/// $037450: move.l -$4(a0), $74(a6)
pub fn i_037450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37456;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $037456: move.w #$2, $72(a6)
pub fn i_037456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3745C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03745C: move.w #$2, $72(a6)
pub fn i_03745c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37462;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037462: bset.b #$1, $8c(a6)
pub fn i_037462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37468;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $037468: bra.w $375d0
pub fn i_037468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3746C;
    cpu.ir = 0x6000;
    cpu.pc = 0x375D0;
    10
}
/// $03746C: cmpi.b #$26, $70(a6)
pub fn i_03746c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37472;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x26, Size::Byte, false);
    16
}
/// $037472: beq.w $37528
pub fn i_037472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37476;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x37528; return 10; }
    8
}
/// $037476: jsr $5d5b6.l
pub fn i_037476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3747C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3747C);
    cpu.pc = 0x5D5B6;
    16
}
/// $03747C: btst.b #$0, $3a(a6)
pub fn i_03747c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37482;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $037482: bne.w $3748e
pub fn i_037482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37486;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3748E; return 10; }
    8
}
/// $037486: move.b #$1, d2
pub fn i_037486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3748A;
    cpu.ir = 0x143C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03748A: bra.w $37492
pub fn i_03748a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3748E;
    cpu.ir = 0x6000;
    cpu.pc = 0x37492;
    10
}
/// $03748E: move.b #$2, d2
pub fn i_03748e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37492;
    cpu.ir = 0x143C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $037492: cmpi.b #$0, d0
pub fn i_037492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37496;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $037496: bne.w $374b6
pub fn i_037496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3749A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x374B6; return 10; }
    8
}
/// $03749A: cmpi.b #$1, $78(a6)
pub fn i_03749a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x374A0;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $0374A0: bne.w $374a8
pub fn i_0374a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x374A4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x374A8; return 10; }
    8
}
/// $0374A4: move.b #$ff, d2
pub fn i_0374a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x374A8;
    cpu.ir = 0x143C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0374A8: cmpi.b #$2, $78(a6)
pub fn i_0374a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x374AE;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $0374AE: bne.w $374b6
pub fn i_0374ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x374B2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x374B6; return 10; }
    8
}
/// $0374B2: move.b #$0, d2
pub fn i_0374b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x374B6;
    cpu.ir = 0x143C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0374B6: btst.b #$2, $100001.l
pub fn i_0374b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x374BE;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((2) % 8);
    let v2 = bus.read8(0x100001) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $0374BE: bne.w $374c2
pub fn i_0374be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x374C2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x374C2; return 10; }
    8
}
/// $0374C2: cmp.b d0, d2
pub fn i_0374c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x374C4;
    cpu.ir = 0xB400;
    let v1 = cpu.d[0] & 0xFF;
    let h1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $0374C4: bne.w $374f0
pub fn i_0374c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x374C8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x374F0; return 10; }
    8
}
/// $0374C8: move.w #$6, $7c(a6)
pub fn i_0374c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x374CE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0374CE: move.w #$fffd, $7e(a6)
pub fn i_0374ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x374D4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0374D4: move.b #$25, $70(a6)
pub fn i_0374d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x374DA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x25;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0374DA: lea.l $279c32.l, a0
pub fn i_0374da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x374E0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279C32;
    20
}
/// $0374E0: move.l -$4(a0), $74(a6)
pub fn i_0374e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x374E6;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0374E6: move.w #$2, $72(a6)
pub fn i_0374e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x374EC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0374EC: bra.w $37524
pub fn i_0374ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x374F0;
    cpu.ir = 0x6000;
    cpu.pc = 0x37524;
    10
}
/// $0374F0: move.w #$0, $7c(a6)
pub fn i_0374f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x374F6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0374F6: move.w #$2, $7e(a6)
pub fn i_0374f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x374FC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0374FC: move.b #$25, $70(a6)
pub fn i_0374fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37502;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x25;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $037502: lea.l $279c28.l, a0
pub fn i_037502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37508;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279C28;
    20
}
/// $037508: move.l -$4(a0), $74(a6)
pub fn i_037508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3750E;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $03750E: cmpi.b #$0, $71(a6)
pub fn i_03750e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37514;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $037514: bne.w $3751e
pub fn i_037514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37518;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3751E; return 10; }
    8
}
/// $037518: ori.w #$1, $38(a6)
pub fn i_037518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3751E;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $03751E: move.w #$0, $72(a6)
pub fn i_03751e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37524;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037524: bra.w $375c6
pub fn i_037524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37528;
    cpu.ir = 0x6000;
    cpu.pc = 0x375C6;
    10
}
/// $037528: jsr $5d5b6.l
pub fn i_037528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3752E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3752E);
    cpu.pc = 0x5D5B6;
    16
}
/// $03752E: btst.b #$0, $3a(a6)
pub fn i_03752e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37534;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $037534: bne.w $37540
pub fn i_037534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37538;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37540; return 10; }
    8
}
/// $037538: move.b #$1, d2
pub fn i_037538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3753C;
    cpu.ir = 0x143C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03753C: bra.w $37544
pub fn i_03753c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37540;
    cpu.ir = 0x6000;
    cpu.pc = 0x37544;
    10
}
/// $037540: move.b #$2, d2
pub fn i_037540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37544;
    cpu.ir = 0x143C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $037544: cmpi.b #$0, d0
pub fn i_037544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37548;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $037548: bne.w $37568
pub fn i_037548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3754C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37568; return 10; }
    8
}
/// $03754C: cmpi.b #$1, $78(a6)
pub fn i_03754c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37552;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $037552: bne.w $3755a
pub fn i_037552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37556;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3755A; return 10; }
    8
}
/// $037556: move.b #$ff, d2
pub fn i_037556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3755A;
    cpu.ir = 0x143C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03755A: cmpi.b #$2, $78(a6)
pub fn i_03755a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37560;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $037560: bne.w $37568
pub fn i_037560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37564;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37568; return 10; }
    8
}
/// $037564: move.b #$0, d2
pub fn i_037564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37568;
    cpu.ir = 0x143C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $037568: btst.b #$2, $100001.l
pub fn i_037568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37570;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((2) % 8);
    let v2 = bus.read8(0x100001) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $037570: bne.w $37574
pub fn i_037570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37574;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37574; return 10; }
    8
}
/// $037574: cmp.b d0, d2
pub fn i_037574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37576;
    cpu.ir = 0xB400;
    let v1 = cpu.d[0] & 0xFF;
    let h1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $037576: bne.w $375a2
pub fn i_037576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3757A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x375A2; return 10; }
    8
}
/// $03757A: move.w #$0, $7c(a6)
pub fn i_03757a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37580;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037580: move.w #$0, $7e(a6)
pub fn i_037580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37586;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037586: move.b #$26, $70(a6)
pub fn i_037586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3758C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x26;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03758C: lea.l $279ac8.l, a0
pub fn i_03758c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37592;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279AC8;
    20
}
/// $037592: move.l -$4(a0), $74(a6)
pub fn i_037592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37598;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $037598: move.w #$2, $72(a6)
pub fn i_037598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3759E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03759E: bra.w $375c6
pub fn i_03759e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x375A2;
    cpu.ir = 0x6000;
    cpu.pc = 0x375C6;
    10
}
/// $0375A2: move.w #$0, $7c(a6)
pub fn i_0375a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x375A8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0375A8: move.w #$7, $7e(a6)
pub fn i_0375a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x375AE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0375AE: move.b #$26, $70(a6)
pub fn i_0375ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x375B4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x26;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0375B4: lea.l $279abe.l, a0
pub fn i_0375b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x375BA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279ABE;
    20
}
/// $0375BA: move.l -$4(a0), $74(a6)
pub fn i_0375ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x375C0;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0375C0: move.w #$0, $72(a6)
pub fn i_0375c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x375C6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0375C6: bset.b #$1, $8c(a6)
pub fn i_0375c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x375CC;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0375CC: bra.w $375d0
pub fn i_0375cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x375D0;
    cpu.ir = 0x6000;
    cpu.pc = 0x375D0;
    10
}
/// $0375D0: rts
pub fn i_0375d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x375D2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_Death_0375d2  ($0375D2, 136 B, player/air_death_crouch_036632.s)
/// $0375D2: move.b #$ff, $6c(a6)
pub fn i_0375d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x375D8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x6C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0375D8: lea.l $78840.l, a1
pub fn i_0375d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x375DE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x78840;
    20
}
/// $0375DE: jsr $4ae.l
pub fn i_0375de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x375E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x375E4);
    cpu.pc = 0x4AE;
    16
}
/// $0375E4: jsr $5dd02.l
pub fn i_0375e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x375EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x375EA);
    cpu.pc = 0x5DD02;
    16
}
/// $0375EA: move.w #$1053, d0
pub fn i_0375ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x375EE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1053;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0375EE: jsr $2352.l
pub fn i_0375ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x375F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x375F4);
    cpu.pc = 0x2352;
    16
}
/// $0375F4: move.b #$0, $70(a6)
pub fn i_0375f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x375FA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0375FA: lea.l $279f76.l, a0
pub fn i_0375fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37600;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279F76;
    20
}
/// $037600: move.l -$4(a0), $74(a6)
pub fn i_037600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37606;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $037606: move.b #$ff, $21(a6)
pub fn i_037606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3760C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03760C: lea.l $279f76.l, a0
pub fn i_03760c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37612;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279F76;
    20
}
/// $037612: jsr $28cd4.l
pub fn i_037612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37618;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37618);
    cpu.pc = 0x28CD4;
    16
}
/// $037618: lea.l $3765a(pc), a1
pub fn i_037618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3761C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3765A;
    16
}
/// $03761C: move.l a1, (a6)
pub fn i_03761c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3761E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03761E: cmpi.b #$3, $106ece.l
pub fn i_03761e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37626;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    20
}
/// $037626: beq.w $3763e
pub fn i_037626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3762A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3763E; return 10; }
    8
}
/// $03762A: movea.l #$ffffffff, a0
pub fn i_03762a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37630;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $037630: lea.l $324c6(pc), a0
pub fn i_037630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37634;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x324C6;
    16
}
/// $037634: jsr $5dd56.l
pub fn i_037634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3763A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3763A);
    cpu.pc = 0x5DD56;
    16
}
/// $03763A: bra.w $3764e
pub fn i_03763a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3763E;
    cpu.ir = 0x6000;
    cpu.pc = 0x3764E;
    10
}
/// $03763E: movea.l #$ffffffff, a0
pub fn i_03763e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37644;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $037644: lea.l $324bc(pc), a0
pub fn i_037644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37648;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x324BC;
    16
}
/// $037648: jsr $5dd56.l
pub fn i_037648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3764E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3764E);
    cpu.pc = 0x5DD56;
    16
}
/// $03764E: bcc.w $37658
pub fn i_03764e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37652;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x37658; return 10; }
    8
}
/// $037652: lea.l $37b8e(pc), a1
pub fn i_037652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37656;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37B8E;
    16
}
/// $037656: move.l a1, (a6)
pub fn i_037656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37658;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037658: rts
pub fn i_037658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3765A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_Death_PlayMusic_03765a  ($03765A, 42 B, player/air_death_crouch_036632.s)
/// $03765A: lea.l $37c1a(pc), a1
pub fn i_03765a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3765E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37C1A;
    16
}
/// $03765E: move.l a1, (a6)
pub fn i_03765e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37660;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037660: cmpa.l #$100440, a6
pub fn i_037660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37666;
    cpu.ir = 0xBDFC;
    let v1: u32 = 0x100440;
    let h1 = cpu.a[6];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $037666: bne.w $37678
pub fn i_037666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3766A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37678; return 10; }
    8
}
/// $03766A: move.w #$1123, d0
pub fn i_03766a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3766E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1123;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03766E: jsr $2352.l
pub fn i_03766e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37674;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37674);
    cpu.pc = 0x2352;
    16
}
/// $037674: bra.w $37682
pub fn i_037674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37678;
    cpu.ir = 0x6000;
    cpu.pc = 0x37682;
    10
}
/// $037678: move.w #$1087, d0
pub fn i_037678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3767C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1087;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03767C: jsr $2352.l
pub fn i_03767c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37682;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37682);
    cpu.pc = 0x2352;
    16
}
/// $037682: bra.b $3761e
pub fn i_037682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37684;
    cpu.ir = 0x609A;
    cpu.pc = 0x3761E;
    10
}
// ===== Player_Death_Alt_037684  ($037684, 244 B, player/air_death_crouch_036632.s)
/// $037684: jsr $5e9b6.l
pub fn i_037684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3768A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3768A);
    cpu.pc = 0x5E9B6;
    16
}
/// $03768A: andi.b #$b, d0
pub fn i_03768a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3768E;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xB) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $03768E: bne.w $375d2
pub fn i_03768e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37692;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x375D2; return 10; }
    8
}
/// $037692: move.b #$ff, $6c(a6)
pub fn i_037692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37698;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x6C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $037698: lea.l $39214(pc), a1
pub fn i_037698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3769C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x39214;
    16
}
/// $03769C: jsr $4ae.l
pub fn i_03769c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x376A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x376A2);
    cpu.pc = 0x4AE;
    16
}
/// $0376A2: jsr $5dd02.l
pub fn i_0376a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x376A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x376A8);
    cpu.pc = 0x5DD02;
    16
}
/// $0376A8: lea.l $78840.l, a1
pub fn i_0376a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x376AE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x78840;
    20
}
/// $0376AE: jsr $4ae.l
pub fn i_0376ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x376B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x376B4);
    cpu.pc = 0x4AE;
    16
}
/// $0376B4: jsr $5dd02.l
pub fn i_0376b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x376BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x376BA);
    cpu.pc = 0x5DD02;
    16
}
/// $0376BA: move.w #$d000, $38(a0)
pub fn i_0376ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x376C0;
    cpu.ir = 0x317C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0376C0: addi.w #$10, $24(a0)
pub fn i_0376c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x376C6;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0376C6: move.w #$1053, d0
pub fn i_0376c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x376CA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1053;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0376CA: jsr $2352.l
pub fn i_0376ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x376D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x376D0);
    cpu.pc = 0x2352;
    16
}
/// $0376D0: move.w #$0, $7c(a6)
pub fn i_0376d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x376D6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0376D6: move.w #$0, $7e(a6)
pub fn i_0376d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x376DC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0376DC: bclr.b #$2, $8c(a6)
pub fn i_0376dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x376E2;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0376E2: bclr.b #$1, $8c(a6)
pub fn i_0376e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x376E8;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0376E8: bclr.b #$3, $8c(a6)
pub fn i_0376e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x376EE;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0376EE: bset.b #$0, $13(a6)
pub fn i_0376ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x376F4;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0376F4: move.b #$0, $70(a6)
pub fn i_0376f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x376FA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0376FA: lea.l $279f80.l, a0
pub fn i_0376fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37700;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279F80;
    20
}
/// $037700: move.l -$4(a0), $74(a6)
pub fn i_037700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37706;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $037706: move.b #$ff, $21(a6)
pub fn i_037706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3770C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03770C: lea.l $279f80.l, a0
pub fn i_03770c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37712;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279F80;
    20
}
/// $037712: jsr $28cd4.l
pub fn i_037712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37718;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37718);
    cpu.pc = 0x28CD4;
    16
}
/// $037718: jsr $267e2.l
pub fn i_037718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3771E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3771E);
    cpu.pc = 0x267E2;
    16
}
/// $03771E: lea.l $37724(pc), a1
pub fn i_03771e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37722;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37724;
    16
}
/// $037722: move.l a1, (a6)
pub fn i_037722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37724;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037724: jsr $32ff2(pc)
pub fn i_037724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37728;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37728);
    cpu.pc = 0x32FF2;
    16
}
/// $037728: jsr $2783a.l
pub fn i_037728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3772E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3772E);
    cpu.pc = 0x2783A;
    16
}
/// $03772E: jsr $32aa8(pc)
pub fn i_03772e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37732;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37732);
    cpu.pc = 0x32AA8;
    16
}
/// $037732: bcc.w $3773c
pub fn i_037732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37736;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3773C; return 10; }
    8
}
/// $037736: lea.l $37c1a(pc), a1
pub fn i_037736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3773A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37C1A;
    16
}
/// $03773A: move.l a1, (a6)
pub fn i_03773a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3773C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03773C: cmpi.b #$3, $106ece.l
pub fn i_03773c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37744;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    20
}
/// $037744: beq.w $3775c
pub fn i_037744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37748;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3775C; return 10; }
    8
}
/// $037748: movea.l #$ffffffff, a0
pub fn i_037748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3774E;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $03774E: lea.l $324c6(pc), a0
pub fn i_03774e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37752;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x324C6;
    16
}
/// $037752: jsr $5dd56.l
pub fn i_037752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37758;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37758);
    cpu.pc = 0x5DD56;
    16
}
/// $037758: bra.w $3776c
pub fn i_037758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3775C;
    cpu.ir = 0x6000;
    cpu.pc = 0x3776C;
    10
}
/// $03775C: movea.l #$ffffffff, a0
pub fn i_03775c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37762;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $037762: lea.l $324bc(pc), a0
pub fn i_037762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37766;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x324BC;
    16
}
/// $037766: jsr $5dd56.l
pub fn i_037766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3776C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3776C);
    cpu.pc = 0x5DD56;
    16
}
/// $03776C: bcc.w $37776
pub fn i_03776c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37770;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x37776; return 10; }
    8
}
/// $037770: lea.l $37b8e(pc), a1
pub fn i_037770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37774;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37B8E;
    16
}
/// $037774: move.l a1, (a6)
pub fn i_037774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37776;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037776: rts
pub fn i_037776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37778;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_Death_Fall_037778  ($037778, 252 B, player/air_death_crouch_036632.s)
/// $037778: move.w #$0, $7c(a6)
pub fn i_037778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3777E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03777E: move.w #$0, $7e(a6)
pub fn i_03777e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37784;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037784: bclr.b #$2, $8c(a6)
pub fn i_037784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3778A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03778A: bclr.b #$1, $8c(a6)
pub fn i_03778a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37790;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $037790: bclr.b #$3, $8c(a6)
pub fn i_037790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37796;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $037796: bset.b #$0, $13(a6)
pub fn i_037796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3779C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03779C: move.b #$0, $70(a6)
pub fn i_03779c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x377A2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0377A2: lea.l $279f44.l, a0
pub fn i_0377a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x377A8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279F44;
    20
}
/// $0377A8: move.l -$4(a0), $74(a6)
pub fn i_0377a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x377AE;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0377AE: move.b #$ff, $21(a6)
pub fn i_0377ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x377B4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0377B4: lea.l $279f44.l, a0
pub fn i_0377b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x377BA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279F44;
    20
}
/// $0377BA: jsr $28cd4.l
pub fn i_0377ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x377C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x377C0);
    cpu.pc = 0x28CD4;
    16
}
/// $0377C0: jsr $267e2.l
pub fn i_0377c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x377C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x377C6);
    cpu.pc = 0x267E2;
    16
}
/// $0377C6: move.w #$fee9, d0
pub fn i_0377c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x377CA;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFEE9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0377CA: jsr $5dca4.l
pub fn i_0377ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x377D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x377D0);
    cpu.pc = 0x5DCA4;
    16
}
/// $0377D0: move.w d0, $28(a6)
pub fn i_0377d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x377D4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0377D4: move.w #$457, $2a(a6)
pub fn i_0377d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x377DA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x457;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0377DA: move.w #$ff9b, $2e(a6)
pub fn i_0377da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x377E0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF9B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0377E0: move.w #$0, $2c(a6)
pub fn i_0377e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x377E6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0377E6: lea.l $377ec(pc), a1
pub fn i_0377e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x377EA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x377EC;
    16
}
/// $0377EA: move.l a1, (a6)
pub fn i_0377ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x377EC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0377EC: bset.b #$1, $8d(a6)
pub fn i_0377ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x377F2;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0377F2: jsr $32ff2(pc)
pub fn i_0377f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x377F6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x377F6);
    cpu.pc = 0x32FF2;
    16
}
/// $0377F6: jsr $27bc8.l
pub fn i_0377f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x377FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x377FC);
    cpu.pc = 0x27BC8;
    16
}
/// $0377FC: bcc.w $37806
pub fn i_0377fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37800;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x37806; return 10; }
    8
}
/// $037800: lea.l $37874(pc), a1
pub fn i_037800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37804;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37874;
    16
}
/// $037804: move.l a1, (a6)
pub fn i_037804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37806;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037806: cmpi.b #$1, $106ece.l
pub fn i_037806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3780E;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $03780E: bne.w $37834
pub fn i_03780e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37812;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37834; return 10; }
    8
}
/// $037812: move.w $22(a6), d1
pub fn i_037812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37816;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $037816: move.w $24(a6), d2
pub fn i_037816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3781A;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03781A: cmpi.w #$10c, d2
pub fn i_03781a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3781E;
    cpu.ir = 0xC42;
    let v1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x10C, Size::Word, false);
    8
}
/// $03781E: bge.w $37834
pub fn i_03781e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37822;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x37834; return 10; }
    8
}
/// $037822: jsr $27db2.l
pub fn i_037822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37828;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37828);
    cpu.pc = 0x27DB2;
    16
}
/// $037828: cmpi.b #$40, d7
pub fn i_037828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3782C;
    cpu.ir = 0xC07;
    let v1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, v1, 0x40, Size::Byte, false);
    8
}
/// $03782C: bne.w $37834
pub fn i_03782c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37830;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37834; return 10; }
    8
}
/// $037830: jsr $33522(pc)
pub fn i_037830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37834;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37834);
    cpu.pc = 0x33522;
    16
}
/// $037834: jsr $32aa8(pc)
pub fn i_037834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37838;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37838);
    cpu.pc = 0x32AA8;
    16
}
/// $037838: cmpi.b #$3, $106ece.l
pub fn i_037838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37840;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    20
}
/// $037840: beq.w $37858
pub fn i_037840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37844;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x37858; return 10; }
    8
}
/// $037844: movea.l #$ffffffff, a0
pub fn i_037844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3784A;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $03784A: lea.l $324c6(pc), a0
pub fn i_03784a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3784E;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x324C6;
    16
}
/// $03784E: jsr $5dd56.l
pub fn i_03784e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37854;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37854);
    cpu.pc = 0x5DD56;
    16
}
/// $037854: bra.w $37868
pub fn i_037854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37858;
    cpu.ir = 0x6000;
    cpu.pc = 0x37868;
    10
}
/// $037858: movea.l #$ffffffff, a0
pub fn i_037858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3785E;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $03785E: lea.l $324bc(pc), a0
pub fn i_03785e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37862;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x324BC;
    16
}
/// $037862: jsr $5dd56.l
pub fn i_037862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37868;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37868);
    cpu.pc = 0x5DD56;
    16
}
/// $037868: bcc.w $37872
pub fn i_037868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3786C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x37872; return 10; }
    8
}
/// $03786C: lea.l $37b8e(pc), a1
pub fn i_03786c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37870;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37B8E;
    16
}
/// $037870: move.l a1, (a6)
pub fn i_037870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37872;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037872: rts
pub fn i_037872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37874;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_Death_FallLanded_037874  ($037874, 82 B, player/air_death_crouch_036632.s)
/// $037874: clr.w $28(a6)
pub fn i_037874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37878;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $037878: clr.w $2a(a6)
pub fn i_037878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3787C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $03787C: move.b #$0, $70(a6)
pub fn i_03787c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37882;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $037882: lea.l $279f4e.l, a0
pub fn i_037882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37888;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279F4E;
    20
}
/// $037888: move.l -$4(a0), $74(a6)
pub fn i_037888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3788E;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $03788E: move.b #$ff, $21(a6)
pub fn i_03788e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37894;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $037894: lea.l $279f4e.l, a0
pub fn i_037894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3789A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279F4E;
    20
}
/// $03789A: jsr $28cd4.l
pub fn i_03789a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x378A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x378A0);
    cpu.pc = 0x28CD4;
    16
}
/// $0378A0: jsr $267e2.l
pub fn i_0378a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x378A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x378A6);
    cpu.pc = 0x267E2;
    16
}
/// $0378A6: lea.l $378ac(pc), a1
pub fn i_0378a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x378AA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x378AC;
    16
}
/// $0378AA: move.l a1, (a6)
pub fn i_0378aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x378AC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0378AC: jsr $32ff2(pc)
pub fn i_0378ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x378B0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x378B0);
    cpu.pc = 0x32FF2;
    16
}
/// $0378B0: jsr $27a92.l
pub fn i_0378b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x378B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x378B6);
    cpu.pc = 0x27A92;
    16
}
/// $0378B6: jsr $32aa8(pc)
pub fn i_0378b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x378BA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x378BA);
    cpu.pc = 0x32AA8;
    16
}
/// $0378BA: bcc.w $378c4
pub fn i_0378ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x378BE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x378C4; return 10; }
    8
}
/// $0378BE: lea.l $37c1a(pc), a1
pub fn i_0378be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x378C2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37C1A;
    16
}
/// $0378C2: move.l a1, (a6)
pub fn i_0378c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x378C4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0378C4: rts
pub fn i_0378c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x378C6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_Death_FallSpawnFx_0378c6  ($0378C6, 284 B, player/air_death_crouch_036632.s)
/// $0378C6: lea.l $391ee(pc), a1
pub fn i_0378c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x378CA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x391EE;
    16
}
/// $0378CA: jsr $4ae.l
pub fn i_0378ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x378D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x378D0);
    cpu.pc = 0x4AE;
    16
}
/// $0378D0: jsr $5dd02.l
pub fn i_0378d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x378D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x378D6);
    cpu.pc = 0x5DD02;
    16
}
/// $0378D6: lea.l $39148(pc), a1
pub fn i_0378d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x378DA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x39148;
    16
}
/// $0378DA: jsr $4ae.l
pub fn i_0378da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x378E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x378E0);
    cpu.pc = 0x4AE;
    16
}
/// $0378E0: jsr $5dd02.l
pub fn i_0378e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x378E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x378E6);
    cpu.pc = 0x5DD02;
    16
}
/// $0378E6: move.w #$0, $7c(a6)
pub fn i_0378e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x378EC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0378EC: move.w #$0, $7e(a6)
pub fn i_0378ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x378F2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0378F2: bclr.b #$2, $8c(a6)
pub fn i_0378f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x378F8;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0378F8: bclr.b #$1, $8c(a6)
pub fn i_0378f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x378FE;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0378FE: bclr.b #$3, $8c(a6)
pub fn i_0378fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37904;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $037904: bset.b #$0, $13(a6)
pub fn i_037904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3790A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03790A: move.b #$0, $70(a6)
pub fn i_03790a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37910;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $037910: lea.l $279f58.l, a0
pub fn i_037910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37916;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279F58;
    20
}
/// $037916: move.l -$4(a0), $74(a6)
pub fn i_037916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3791C;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $03791C: move.b #$ff, $21(a6)
pub fn i_03791c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37922;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $037922: lea.l $279f58.l, a0
pub fn i_037922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37928;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279F58;
    20
}
/// $037928: jsr $28cd4.l
pub fn i_037928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3792E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3792E);
    cpu.pc = 0x28CD4;
    16
}
/// $03792E: jsr $267e2.l
pub fn i_03792e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37934;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37934);
    cpu.pc = 0x267E2;
    16
}
/// $037934: move.w #$fee9, d0
pub fn i_037934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37938;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFEE9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $037938: jsr $5dca4.l
pub fn i_037938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3793E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3793E);
    cpu.pc = 0x5DCA4;
    16
}
/// $03793E: move.w d0, $28(a6)
pub fn i_03793e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37942;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037942: move.w #$457, $2a(a6)
pub fn i_037942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37948;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x457;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037948: move.w #$ff9b, $2e(a6)
pub fn i_037948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3794E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF9B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03794E: move.w #$0, $2c(a6)
pub fn i_03794e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37954;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037954: lea.l $3795a(pc), a1
pub fn i_037954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37958;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3795A;
    16
}
/// $037958: move.l a1, (a6)
pub fn i_037958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3795A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03795A: bset.b #$1, $8d(a6)
pub fn i_03795a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37960;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $037960: jsr $32ff2(pc)
pub fn i_037960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37964;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37964);
    cpu.pc = 0x32FF2;
    16
}
/// $037964: jsr $27bc8.l
pub fn i_037964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3796A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3796A);
    cpu.pc = 0x27BC8;
    16
}
/// $03796A: bcc.w $37974
pub fn i_03796a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3796E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x37974; return 10; }
    8
}
/// $03796E: lea.l $379e2(pc), a1
pub fn i_03796e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37972;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x379E2;
    16
}
/// $037972: move.l a1, (a6)
pub fn i_037972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37974;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037974: cmpi.b #$1, $106ece.l
pub fn i_037974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3797C;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $03797C: bne.w $379a2
pub fn i_03797c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37980;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x379A2; return 10; }
    8
}
/// $037980: move.w $22(a6), d1
pub fn i_037980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37984;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $037984: move.w $24(a6), d2
pub fn i_037984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37988;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $037988: cmpi.w #$10c, d2
pub fn i_037988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3798C;
    cpu.ir = 0xC42;
    let v1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x10C, Size::Word, false);
    8
}
/// $03798C: bge.w $379a2
pub fn i_03798c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37990;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x379A2; return 10; }
    8
}
/// $037990: jsr $27db2.l
pub fn i_037990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37996;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37996);
    cpu.pc = 0x27DB2;
    16
}
/// $037996: cmpi.b #$40, d7
pub fn i_037996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3799A;
    cpu.ir = 0xC07;
    let v1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, v1, 0x40, Size::Byte, false);
    8
}
/// $03799A: bne.w $379a2
pub fn i_03799a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3799E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x379A2; return 10; }
    8
}
/// $03799E: jsr $33522(pc)
pub fn i_03799e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x379A2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x379A2);
    cpu.pc = 0x33522;
    16
}
/// $0379A2: jsr $32aa8(pc)
pub fn i_0379a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x379A6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x379A6);
    cpu.pc = 0x32AA8;
    16
}
/// $0379A6: cmpi.b #$3, $106ece.l
pub fn i_0379a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x379AE;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    20
}
/// $0379AE: beq.w $379c6
pub fn i_0379ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x379B2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x379C6; return 10; }
    8
}
/// $0379B2: movea.l #$ffffffff, a0
pub fn i_0379b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x379B8;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0379B8: lea.l $324c6(pc), a0
pub fn i_0379b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x379BC;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x324C6;
    16
}
/// $0379BC: jsr $5dd56.l
pub fn i_0379bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x379C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x379C2);
    cpu.pc = 0x5DD56;
    16
}
/// $0379C2: bra.w $379d6
pub fn i_0379c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x379C6;
    cpu.ir = 0x6000;
    cpu.pc = 0x379D6;
    10
}
/// $0379C6: movea.l #$ffffffff, a0
pub fn i_0379c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x379CC;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0379CC: lea.l $324bc(pc), a0
pub fn i_0379cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x379D0;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x324BC;
    16
}
/// $0379D0: jsr $5dd56.l
pub fn i_0379d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x379D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x379D6);
    cpu.pc = 0x5DD56;
    16
}
/// $0379D6: bcc.w $379e0
pub fn i_0379d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x379DA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x379E0; return 10; }
    8
}
/// $0379DA: lea.l $37b8e(pc), a1
pub fn i_0379da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x379DE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37B8E;
    16
}
/// $0379DE: move.l a1, (a6)
pub fn i_0379de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x379E0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0379E0: rts
pub fn i_0379e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x379E2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_Death_FallLandedB_0379e2  ($0379E2, 82 B, player/air_death_crouch_036632.s)
/// $0379E2: clr.w $28(a6)
pub fn i_0379e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x379E6;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0379E6: clr.w $2a(a6)
pub fn i_0379e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x379EA;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0379EA: move.b #$0, $70(a6)
pub fn i_0379ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x379F0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0379F0: lea.l $279f62.l, a0
pub fn i_0379f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x379F6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279F62;
    20
}
/// $0379F6: move.l -$4(a0), $74(a6)
pub fn i_0379f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x379FC;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0379FC: move.b #$ff, $21(a6)
pub fn i_0379fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37A02;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $037A02: lea.l $279f62.l, a0
pub fn i_037a02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37A08;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279F62;
    20
}
/// $037A08: jsr $28cd4.l
pub fn i_037a08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37A0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37A0E);
    cpu.pc = 0x28CD4;
    16
}
/// $037A0E: jsr $267e2.l
pub fn i_037a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37A14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37A14);
    cpu.pc = 0x267E2;
    16
}
/// $037A14: lea.l $37a1a(pc), a1
pub fn i_037a14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37A18;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37A1A;
    16
}
/// $037A18: move.l a1, (a6)
pub fn i_037a18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37A1A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037A1A: jsr $32ff2(pc)
pub fn i_037a1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37A1E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37A1E);
    cpu.pc = 0x32FF2;
    16
}
/// $037A1E: jsr $27a92.l
pub fn i_037a1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37A24;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37A24);
    cpu.pc = 0x27A92;
    16
}
/// $037A24: jsr $32aa8(pc)
pub fn i_037a24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37A28;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37A28);
    cpu.pc = 0x32AA8;
    16
}
/// $037A28: bcc.w $37a32
pub fn i_037a28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37A2C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x37A32; return 10; }
    8
}
/// $037A2C: lea.l $37c1a(pc), a1
pub fn i_037a2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37A30;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37C1A;
    16
}
/// $037A30: move.l a1, (a6)
pub fn i_037a30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37A32;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037A32: rts
pub fn i_037a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37A34;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_Death_PrioE000_037a34  ($037A34, 10 B, player/air_death_crouch_036632.s)
/// $037A34: move.w #$e000, $38(a6)
pub fn i_037a34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37A3A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xE000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037A3A: bra.w $37a3e
pub fn i_037a3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37A3E;
    cpu.ir = 0x6000;
    cpu.pc = 0x37A3E;
    10
}
// ===== Player_Death_Generic_037a3e  ($037A3E, 180 B, player/air_death_crouch_036632.s)
/// $037A3E: move.w #$0, $7c(a6)
pub fn i_037a3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37A44;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037A44: move.w #$0, $7e(a6)
pub fn i_037a44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37A4A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037A4A: bclr.b #$2, $8c(a6)
pub fn i_037a4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37A50;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $037A50: bclr.b #$1, $8c(a6)
pub fn i_037a50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37A56;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $037A56: bclr.b #$3, $8c(a6)
pub fn i_037a56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37A5C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $037A5C: bset.b #$0, $13(a6)
pub fn i_037a5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37A62;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $037A62: move.b #$0, $70(a6)
pub fn i_037a62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37A68;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $037A68: lea.l $279f30.l, a0
pub fn i_037a68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37A6E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279F30;
    20
}
/// $037A6E: move.l -$4(a0), $74(a6)
pub fn i_037a6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37A74;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $037A74: move.b #$ff, $21(a6)
pub fn i_037a74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37A7A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $037A7A: lea.l $279f30.l, a0
pub fn i_037a7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37A80;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279F30;
    20
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x035E58, i_035e58),
    (0x035E5C, i_035e5c),
    (0x035E62, i_035e62),
    (0x035E66, i_035e66),
    (0x035E6A, i_035e6a),
    (0x035E6E, i_035e6e),
    (0x035E70, i_035e70),
    (0x035E76, i_035e76),
    (0x035E78, i_035e78),
    (0x035E7C, i_035e7c),
    (0x035E80, i_035e80),
    (0x035E86, i_035e86),
    (0x035E8A, i_035e8a),
    (0x035E8E, i_035e8e),
    (0x035E90, i_035e90),
    (0x035E96, i_035e96),
    (0x035E9A, i_035e9a),
    (0x035E9E, i_035e9e),
    (0x035EA0, i_035ea0),
    (0x035EA4, i_035ea4),
    (0x035EA8, i_035ea8),
    (0x035EAE, i_035eae),
    (0x035EB4, i_035eb4),
    (0x035EBA, i_035eba),
    (0x035EC0, i_035ec0),
    (0x035EC6, i_035ec6),
    (0x035ECA, i_035eca),
    (0x035ECE, i_035ece),
    (0x035ED4, i_035ed4),
    (0x035EDC, i_035edc),
    (0x035EE2, i_035ee2),
    (0x035EE6, i_035ee6),
    (0x035EEC, i_035eec),
    (0x035EF2, i_035ef2),
    (0x035EF8, i_035ef8),
    (0x035EFE, i_035efe),
    (0x035F04, i_035f04),
    (0x035F0A, i_035f0a),
    (0x035F0E, i_035f0e),
    (0x035F14, i_035f14),
    (0x035F1A, i_035f1a),
    (0x035F20, i_035f20),
    (0x035F26, i_035f26),
    (0x035F2C, i_035f2c),
    (0x035F32, i_035f32),
    (0x035F36, i_035f36),
    (0x035F3A, i_035f3a),
    (0x035F3E, i_035f3e),
    (0x035F40, i_035f40),
    (0x035F44, i_035f44),
    (0x035F48, i_035f48),
    (0x035F4E, i_035f4e),
    (0x035F52, i_035f52),
    (0x035F56, i_035f56),
    (0x035F5A, i_035f5a),
    (0x035F5C, i_035f5c),
    (0x035F62, i_035f62),
    (0x035F64, i_035f64),
    (0x035F68, i_035f68),
    (0x035F6C, i_035f6c),
    (0x035F70, i_035f70),
    (0x035F76, i_035f76),
    (0x035F7C, i_035f7c),
    (0x035F82, i_035f82),
    (0x035F88, i_035f88),
    (0x035F8E, i_035f8e),
    (0x035F92, i_035f92),
    (0x035F96, i_035f96),
    (0x035F9C, i_035f9c),
    (0x035FA4, i_035fa4),
    (0x035FAA, i_035faa),
    (0x035FAE, i_035fae),
    (0x035FB4, i_035fb4),
    (0x035FBA, i_035fba),
    (0x035FC0, i_035fc0),
    (0x035FC6, i_035fc6),
    (0x035FCC, i_035fcc),
    (0x035FD2, i_035fd2),
    (0x035FD6, i_035fd6),
    (0x035FDC, i_035fdc),
    (0x035FE2, i_035fe2),
    (0x035FE8, i_035fe8),
    (0x035FEE, i_035fee),
    (0x035FF4, i_035ff4),
    (0x035FFA, i_035ffa),
    (0x035FFE, i_035ffe),
    (0x036002, i_036002),
    (0x036006, i_036006),
    (0x036008, i_036008),
    (0x03600C, i_03600c),
    (0x036010, i_036010),
    (0x036012, i_036012),
    (0x036016, i_036016),
    (0x03601A, i_03601a),
    (0x036020, i_036020),
    (0x036024, i_036024),
    (0x036028, i_036028),
    (0x03602C, i_03602c),
    (0x036032, i_036032),
    (0x036036, i_036036),
    (0x03603A, i_03603a),
    (0x036040, i_036040),
    (0x036044, i_036044),
    (0x036048, i_036048),
    (0x03604C, i_03604c),
    (0x03604E, i_03604e),
    (0x036054, i_036054),
    (0x036056, i_036056),
    (0x03605A, i_03605a),
    (0x03605E, i_03605e),
    (0x036062, i_036062),
    (0x036066, i_036066),
    (0x03606A, i_03606a),
    (0x03606E, i_03606e),
    (0x036072, i_036072),
    (0x036074, i_036074),
    (0x036078, i_036078),
    (0x03607C, i_03607c),
    (0x036080, i_036080),
    (0x036084, i_036084),
    (0x036086, i_036086),
    (0x03608A, i_03608a),
    (0x03608E, i_03608e),
    (0x036092, i_036092),
    (0x036096, i_036096),
    (0x036098, i_036098),
    (0x03609C, i_03609c),
    (0x0360A0, i_0360a0),
    (0x0360A4, i_0360a4),
    (0x0360A8, i_0360a8),
    (0x0360AA, i_0360aa),
    (0x0360AE, i_0360ae),
    (0x0360B2, i_0360b2),
    (0x0360B4, i_0360b4),
    (0x0360B8, i_0360b8),
    (0x0360BC, i_0360bc),
    (0x0360C2, i_0360c2),
    (0x0360C8, i_0360c8),
    (0x0360CE, i_0360ce),
    (0x0360D4, i_0360d4),
    (0x0360DA, i_0360da),
    (0x0360DE, i_0360de),
    (0x0360E2, i_0360e2),
    (0x0360E6, i_0360e6),
    (0x0360EA, i_0360ea),
    (0x0360F2, i_0360f2),
    (0x0360F8, i_0360f8),
    (0x0360FC, i_0360fc),
    (0x036102, i_036102),
    (0x036108, i_036108),
    (0x03610E, i_03610e),
    (0x036114, i_036114),
    (0x03611A, i_03611a),
    (0x036120, i_036120),
    (0x036124, i_036124),
    (0x03612A, i_03612a),
    (0x036130, i_036130),
    (0x036136, i_036136),
    (0x03613C, i_03613c),
    (0x036142, i_036142),
    (0x036148, i_036148),
    (0x03614C, i_03614c),
    (0x036150, i_036150),
    (0x036152, i_036152),
    (0x036156, i_036156),
    (0x03615A, i_03615a),
    (0x036160, i_036160),
    (0x036164, i_036164),
    (0x036168, i_036168),
    (0x03616C, i_03616c),
    (0x03616E, i_03616e),
    (0x036174, i_036174),
    (0x036176, i_036176),
    (0x03617A, i_03617a),
    (0x03617E, i_03617e),
    (0x036184, i_036184),
    (0x036188, i_036188),
    (0x03618C, i_03618c),
    (0x03618E, i_03618e),
    (0x036194, i_036194),
    (0x036198, i_036198),
    (0x03619C, i_03619c),
    (0x03619E, i_03619e),
    (0x0361A4, i_0361a4),
    (0x0361A8, i_0361a8),
    (0x0361AE, i_0361ae),
    (0x0361B2, i_0361b2),
    (0x0361B6, i_0361b6),
    (0x0361BA, i_0361ba),
    (0x0361BE, i_0361be),
    (0x0361C0, i_0361c0),
    (0x0361C4, i_0361c4),
    (0x0361C8, i_0361c8),
    (0x0361CC, i_0361cc),
    (0x0361CE, i_0361ce),
    (0x0361D2, i_0361d2),
    (0x0361D6, i_0361d6),
    (0x0361DA, i_0361da),
    (0x0361DE, i_0361de),
    (0x0361E0, i_0361e0),
    (0x0361E4, i_0361e4),
    (0x0361E8, i_0361e8),
    (0x0361EC, i_0361ec),
    (0x0361EE, i_0361ee),
    (0x0361F2, i_0361f2),
    (0x0361F6, i_0361f6),
    (0x0361FA, i_0361fa),
    (0x0361FC, i_0361fc),
    (0x036200, i_036200),
    (0x036204, i_036204),
    (0x036208, i_036208),
    (0x03620A, i_03620a),
    (0x03620E, i_03620e),
    (0x036212, i_036212),
    (0x036218, i_036218),
    (0x03621E, i_03621e),
    (0x036224, i_036224),
    (0x03622A, i_03622a),
    (0x036230, i_036230),
    (0x036238, i_036238),
    (0x03623E, i_03623e),
    (0x036242, i_036242),
    (0x036248, i_036248),
    (0x03624E, i_03624e),
    (0x036254, i_036254),
    (0x03625A, i_03625a),
    (0x036260, i_036260),
    (0x036266, i_036266),
    (0x03626A, i_03626a),
    (0x036270, i_036270),
    (0x036276, i_036276),
    (0x03627C, i_03627c),
    (0x036282, i_036282),
    (0x036288, i_036288),
    (0x03628E, i_03628e),
    (0x036292, i_036292),
    (0x036296, i_036296),
    (0x036298, i_036298),
    (0x03629C, i_03629c),
    (0x0362A0, i_0362a0),
    (0x0362A6, i_0362a6),
    (0x0362AA, i_0362aa),
    (0x0362AE, i_0362ae),
    (0x0362B2, i_0362b2),
    (0x0362B4, i_0362b4),
    (0x0362BA, i_0362ba),
    (0x0362BC, i_0362bc),
    (0x0362C0, i_0362c0),
    (0x0362C4, i_0362c4),
    (0x0362CA, i_0362ca),
    (0x0362CE, i_0362ce),
    (0x0362D4, i_0362d4),
    (0x0362D8, i_0362d8),
    (0x0362DC, i_0362dc),
    (0x0362E0, i_0362e0),
    (0x0362E4, i_0362e4),
    (0x0362E6, i_0362e6),
    (0x0362EA, i_0362ea),
    (0x0362EE, i_0362ee),
    (0x0362F2, i_0362f2),
    (0x0362F6, i_0362f6),
    (0x0362F8, i_0362f8),
    (0x0362FC, i_0362fc),
    (0x036300, i_036300),
    (0x036304, i_036304),
    (0x036306, i_036306),
    (0x03630A, i_03630a),
    (0x03630E, i_03630e),
    (0x036312, i_036312),
    (0x036314, i_036314),
    (0x036318, i_036318),
    (0x03631E, i_03631e),
    (0x036324, i_036324),
    (0x03632A, i_03632a),
    (0x036330, i_036330),
    (0x036336, i_036336),
    (0x03633E, i_03633e),
    (0x036344, i_036344),
    (0x036348, i_036348),
    (0x03634E, i_03634e),
    (0x036354, i_036354),
    (0x03635A, i_03635a),
    (0x036360, i_036360),
    (0x036366, i_036366),
    (0x03636C, i_03636c),
    (0x036370, i_036370),
    (0x036376, i_036376),
    (0x03637C, i_03637c),
    (0x036382, i_036382),
    (0x036388, i_036388),
    (0x03638E, i_03638e),
    (0x036394, i_036394),
    (0x036398, i_036398),
    (0x03639C, i_03639c),
    (0x03639E, i_03639e),
    (0x0363A2, i_0363a2),
    (0x0363A6, i_0363a6),
    (0x0363A8, i_0363a8),
    (0x0363AC, i_0363ac),
    (0x0363B0, i_0363b0),
    (0x0363B6, i_0363b6),
    (0x0363BA, i_0363ba),
    (0x0363BE, i_0363be),
    (0x0363C2, i_0363c2),
    (0x0363C8, i_0363c8),
    (0x0363CC, i_0363cc),
    (0x0363D0, i_0363d0),
    (0x0363D6, i_0363d6),
    (0x0363DA, i_0363da),
    (0x0363DE, i_0363de),
    (0x0363E2, i_0363e2),
    (0x0363E4, i_0363e4),
    (0x0363EA, i_0363ea),
    (0x0363EC, i_0363ec),
    (0x0363F0, i_0363f0),
    (0x0363F4, i_0363f4),
    (0x0363FA, i_0363fa),
    (0x0363FE, i_0363fe),
    (0x036404, i_036404),
    (0x036408, i_036408),
    (0x03640C, i_03640c),
    (0x036410, i_036410),
    (0x036414, i_036414),
    (0x036416, i_036416),
    (0x03641A, i_03641a),
    (0x03641E, i_03641e),
    (0x036422, i_036422),
    (0x036426, i_036426),
    (0x036428, i_036428),
    (0x03642C, i_03642c),
    (0x036430, i_036430),
    (0x036434, i_036434),
    (0x036436, i_036436),
    (0x03643A, i_03643a),
    (0x03643E, i_03643e),
    (0x036442, i_036442),
    (0x036444, i_036444),
    (0x036448, i_036448),
    (0x03644C, i_03644c),
    (0x036450, i_036450),
    (0x036454, i_036454),
    (0x036458, i_036458),
    (0x03645A, i_03645a),
    (0x03645E, i_03645e),
    (0x036462, i_036462),
    (0x036466, i_036466),
    (0x03646A, i_03646a),
    (0x03646C, i_03646c),
    (0x036470, i_036470),
    (0x036474, i_036474),
    (0x036478, i_036478),
    (0x03647C, i_03647c),
    (0x03647E, i_03647e),
    (0x036482, i_036482),
    (0x036486, i_036486),
    (0x03648A, i_03648a),
    (0x03648E, i_03648e),
    (0x036490, i_036490),
    (0x036494, i_036494),
    (0x036498, i_036498),
    (0x03649A, i_03649a),
    (0x03649E, i_03649e),
    (0x0364A2, i_0364a2),
    (0x0364A8, i_0364a8),
    (0x0364AE, i_0364ae),
    (0x0364B4, i_0364b4),
    (0x0364BA, i_0364ba),
    (0x0364BE, i_0364be),
    (0x0364C2, i_0364c2),
    (0x0364C8, i_0364c8),
    (0x0364CE, i_0364ce),
    (0x0364D4, i_0364d4),
    (0x0364DA, i_0364da),
    (0x0364E0, i_0364e0),
    (0x0364E6, i_0364e6),
    (0x0364EC, i_0364ec),
    (0x0364F2, i_0364f2),
    (0x0364F6, i_0364f6),
    (0x0364FA, i_0364fa),
    (0x0364FE, i_0364fe),
    (0x036502, i_036502),
    (0x036506, i_036506),
    (0x036508, i_036508),
    (0x03650C, i_03650c),
    (0x036512, i_036512),
    (0x036518, i_036518),
    (0x03651C, i_03651c),
    (0x036520, i_036520),
    (0x036522, i_036522),
    (0x036526, i_036526),
    (0x03652A, i_03652a),
    (0x03652E, i_03652e),
    (0x036534, i_036534),
    (0x03653A, i_03653a),
    (0x03653E, i_03653e),
    (0x036542, i_036542),
    (0x036548, i_036548),
    (0x03654C, i_03654c),
    (0x036552, i_036552),
    (0x036556, i_036556),
    (0x03655C, i_03655c),
    (0x036562, i_036562),
    (0x036566, i_036566),
    (0x03656A, i_03656a),
    (0x036570, i_036570),
    (0x036574, i_036574),
    (0x03657A, i_03657a),
    (0x03657E, i_03657e),
    (0x036584, i_036584),
    (0x036588, i_036588),
    (0x03658E, i_03658e),
    (0x036592, i_036592),
    (0x036596, i_036596),
    (0x036598, i_036598),
    (0x03659E, i_03659e),
    (0x0365A2, i_0365a2),
    (0x0365A8, i_0365a8),
    (0x0365AC, i_0365ac),
    (0x0365B0, i_0365b0),
    (0x0365B2, i_0365b2),
    (0x0365B8, i_0365b8),
    (0x0365BC, i_0365bc),
    (0x0365C2, i_0365c2),
    (0x0365C6, i_0365c6),
    (0x0365CC, i_0365cc),
    (0x0365D0, i_0365d0),
    (0x0365D4, i_0365d4),
    (0x0365D6, i_0365d6),
    (0x0365DC, i_0365dc),
    (0x0365E2, i_0365e2),
    (0x0365E8, i_0365e8),
    (0x0365EE, i_0365ee),
    (0x0365F2, i_0365f2),
    (0x0365F6, i_0365f6),
    (0x0365F8, i_0365f8),
    (0x036600, i_036600),
    (0x036604, i_036604),
    (0x03660A, i_03660a),
    (0x03660E, i_03660e),
    (0x036614, i_036614),
    (0x036618, i_036618),
    (0x03661E, i_03661e),
    (0x036622, i_036622),
    (0x036628, i_036628),
    (0x03662C, i_03662c),
    (0x036630, i_036630),
    (0x036632, i_036632),
    (0x036636, i_036636),
    (0x036638, i_036638),
    (0x03663E, i_03663e),
    (0x036644, i_036644),
    (0x03664A, i_03664a),
    (0x036650, i_036650),
    (0x036654, i_036654),
    (0x036658, i_036658),
    (0x03665E, i_03665e),
    (0x036664, i_036664),
    (0x03666A, i_03666a),
    (0x036670, i_036670),
    (0x036676, i_036676),
    (0x03667C, i_03667c),
    (0x036682, i_036682),
    (0x036688, i_036688),
    (0x03668C, i_03668c),
    (0x036690, i_036690),
    (0x036694, i_036694),
    (0x036698, i_036698),
    (0x03669C, i_03669c),
    (0x03669E, i_03669e),
    (0x0366A2, i_0366a2),
    (0x0366A8, i_0366a8),
    (0x0366AC, i_0366ac),
    (0x0366B2, i_0366b2),
    (0x0366B6, i_0366b6),
    (0x0366BA, i_0366ba),
    (0x0366BE, i_0366be),
    (0x0366C2, i_0366c2),
    (0x0366C6, i_0366c6),
    (0x0366C8, i_0366c8),
    (0x0366CE, i_0366ce),
    (0x0366D4, i_0366d4),
    (0x0366D8, i_0366d8),
    (0x0366DE, i_0366de),
    (0x0366E2, i_0366e2),
    (0x0366E6, i_0366e6),
    (0x0366E8, i_0366e8),
    (0x0366EE, i_0366ee),
    (0x0366F4, i_0366f4),
    (0x0366F8, i_0366f8),
    (0x0366FC, i_0366fc),
    (0x0366FE, i_0366fe),
    (0x036704, i_036704),
    (0x03670A, i_03670a),
    (0x036710, i_036710),
    (0x036716, i_036716),
    (0x03671C, i_03671c),
    (0x036722, i_036722),
    (0x036724, i_036724),
    (0x036728, i_036728),
    (0x03672E, i_03672e),
    (0x036732, i_036732),
    (0x036736, i_036736),
    (0x03673C, i_03673c),
    (0x036742, i_036742),
    (0x036748, i_036748),
    (0x03674E, i_03674e),
    (0x036754, i_036754),
    (0x03675A, i_03675a),
    (0x03675E, i_03675e),
    (0x036762, i_036762),
    (0x036764, i_036764),
    (0x036768, i_036768),
    (0x03676E, i_03676e),
    (0x036774, i_036774),
    (0x036778, i_036778),
    (0x03677C, i_03677c),
    (0x036780, i_036780),
    (0x036784, i_036784),
    (0x036788, i_036788),
    (0x03678C, i_03678c),
    (0x036790, i_036790),
    (0x036792, i_036792),
    (0x036796, i_036796),
    (0x03679C, i_03679c),
    (0x0367A2, i_0367a2),
    (0x0367A8, i_0367a8),
    (0x0367AE, i_0367ae),
    (0x0367B4, i_0367b4),
    (0x0367BA, i_0367ba),
    (0x0367C2, i_0367c2),
    (0x0367C8, i_0367c8),
    (0x0367CE, i_0367ce),
    (0x0367D4, i_0367d4),
    (0x0367DA, i_0367da),
    (0x0367E0, i_0367e0),
    (0x0367E6, i_0367e6),
    (0x0367EC, i_0367ec),
    (0x0367F2, i_0367f2),
    (0x0367F6, i_0367f6),
    (0x0367FA, i_0367fa),
    (0x036800, i_036800),
    (0x036804, i_036804),
    (0x036806, i_036806),
    (0x03680A, i_03680a),
    (0x03680E, i_03680e),
    (0x036812, i_036812),
    (0x036814, i_036814),
    (0x03681A, i_03681a),
    (0x03681C, i_03681c),
    (0x036820, i_036820),
    (0x036826, i_036826),
    (0x03682A, i_03682a),
    (0x03682E, i_03682e),
    (0x036834, i_036834),
    (0x036838, i_036838),
    (0x03683C, i_03683c),
    (0x036840, i_036840),
    (0x036844, i_036844),
    (0x03684A, i_03684a),
    (0x03684E, i_03684e),
    (0x036852, i_036852),
    (0x036858, i_036858),
    (0x03685E, i_03685e),
    (0x036862, i_036862),
    (0x036866, i_036866),
    (0x036868, i_036868),
    (0x03686E, i_03686e),
    (0x036872, i_036872),
    (0x036876, i_036876),
    (0x03687A, i_03687a),
    (0x03687E, i_03687e),
    (0x036882, i_036882),
    (0x036888, i_036888),
    (0x03688C, i_03688c),
    (0x036890, i_036890),
    (0x036892, i_036892),
    (0x036896, i_036896),
    (0x03689C, i_03689c),
    (0x0368A0, i_0368a0),
    (0x0368A8, i_0368a8),
    (0x0368AC, i_0368ac),
    (0x0368B2, i_0368b2),
    (0x0368B6, i_0368b6),
    (0x0368BC, i_0368bc),
    (0x0368C0, i_0368c0),
    (0x0368C6, i_0368c6),
    (0x0368CA, i_0368ca),
    (0x0368D0, i_0368d0),
    (0x0368D4, i_0368d4),
    (0x0368D8, i_0368d8),
    (0x0368DA, i_0368da),
    (0x0368DE, i_0368de),
    (0x0368E0, i_0368e0),
    (0x0368E6, i_0368e6),
    (0x0368EA, i_0368ea),
    (0x0368F0, i_0368f0),
    (0x0368F4, i_0368f4),
    (0x0368F8, i_0368f8),
    (0x0368FE, i_0368fe),
    (0x036904, i_036904),
    (0x03690A, i_03690a),
    (0x036910, i_036910),
    (0x036914, i_036914),
    (0x03691A, i_03691a),
    (0x036920, i_036920),
    (0x036926, i_036926),
    (0x03692C, i_03692c),
    (0x036932, i_036932),
    (0x03693A, i_03693a),
    (0x03693E, i_03693e),
    (0x036942, i_036942),
    (0x036948, i_036948),
    (0x03694E, i_03694e),
    (0x036954, i_036954),
    (0x03695A, i_03695a),
    (0x036960, i_036960),
    (0x036966, i_036966),
    (0x03696A, i_03696a),
    (0x036970, i_036970),
    (0x036976, i_036976),
    (0x03697C, i_03697c),
    (0x036982, i_036982),
    (0x036988, i_036988),
    (0x03698E, i_03698e),
    (0x036992, i_036992),
    (0x036998, i_036998),
    (0x03699C, i_03699c),
    (0x0369A2, i_0369a2),
    (0x0369A8, i_0369a8),
    (0x0369AE, i_0369ae),
    (0x0369B2, i_0369b2),
    (0x0369B8, i_0369b8),
    (0x0369BE, i_0369be),
    (0x0369C4, i_0369c4),
    (0x0369CA, i_0369ca),
    (0x0369D0, i_0369d0),
    (0x0369D6, i_0369d6),
    (0x0369DC, i_0369dc),
    (0x0369E2, i_0369e2),
    (0x0369E6, i_0369e6),
    (0x0369EC, i_0369ec),
    (0x0369F2, i_0369f2),
    (0x0369F8, i_0369f8),
    (0x0369FE, i_0369fe),
    (0x036A04, i_036a04),
    (0x036A0A, i_036a0a),
    (0x036A10, i_036a10),
    (0x036A16, i_036a16),
    (0x036A1C, i_036a1c),
    (0x036A22, i_036a22),
    (0x036A28, i_036a28),
    (0x036A2C, i_036a2c),
    (0x036A30, i_036a30),
    (0x036A34, i_036a34),
    (0x036A36, i_036a36),
    (0x036A3C, i_036a3c),
    (0x036A42, i_036a42),
    (0x036A46, i_036a46),
    (0x036A4C, i_036a4c),
    (0x036A50, i_036a50),
    (0x036A54, i_036a54),
    (0x036A56, i_036a56),
    (0x036A5C, i_036a5c),
    (0x036A60, i_036a60),
    (0x036A64, i_036a64),
    (0x036A66, i_036a66),
    (0x036A6A, i_036a6a),
    (0x036A6E, i_036a6e),
    (0x036A70, i_036a70),
    (0x036A76, i_036a76),
    (0x036A7A, i_036a7a),
    (0x036A80, i_036a80),
    (0x036A82, i_036a82),
    (0x036A86, i_036a86),
    (0x036A8C, i_036a8c),
    (0x036A90, i_036a90),
    (0x036A94, i_036a94),
    (0x036A9A, i_036a9a),
    (0x036A9E, i_036a9e),
    (0x036AA4, i_036aa4),
    (0x036AA8, i_036aa8),
    (0x036AAE, i_036aae),
    (0x036AB0, i_036ab0),
    (0x036AB4, i_036ab4),
    (0x036ABA, i_036aba),
    (0x036AC0, i_036ac0),
    (0x036AC4, i_036ac4),
    (0x036ACA, i_036aca),
    (0x036ACE, i_036ace),
    (0x036AD4, i_036ad4),
    (0x036AD8, i_036ad8),
    (0x036ADE, i_036ade),
    (0x036AE2, i_036ae2),
    (0x036AE8, i_036ae8),
    (0x036AEC, i_036aec),
    (0x036AF0, i_036af0),
    (0x036AF4, i_036af4),
    (0x036AFA, i_036afa),
    (0x036AFE, i_036afe),
    (0x036B02, i_036b02),
    (0x036B06, i_036b06),
    (0x036B0A, i_036b0a),
    (0x036B10, i_036b10),
    (0x036B14, i_036b14),
    (0x036B18, i_036b18),
    (0x036B1E, i_036b1e),
    (0x036B24, i_036b24),
    (0x036B2C, i_036b2c),
    (0x036B30, i_036b30),
    (0x036B34, i_036b34),
    (0x036B38, i_036b38),
    (0x036B3C, i_036b3c),
    (0x036B40, i_036b40),
    (0x036B44, i_036b44),
    (0x036B48, i_036b48),
    (0x036B4C, i_036b4c),
    (0x036B50, i_036b50),
    (0x036B54, i_036b54),
    (0x036B5A, i_036b5a),
    (0x036B5E, i_036b5e),
    (0x036B62, i_036b62),
    (0x036B64, i_036b64),
    (0x036B6A, i_036b6a),
    (0x036B70, i_036b70),
    (0x036B74, i_036b74),
    (0x036B78, i_036b78),
    (0x036B7E, i_036b7e),
    (0x036B82, i_036b82),
    (0x036B8A, i_036b8a),
    (0x036B8E, i_036b8e),
    (0x036B92, i_036b92),
    (0x036B96, i_036b96),
    (0x036B9A, i_036b9a),
    (0x036B9E, i_036b9e),
    (0x036BA2, i_036ba2),
    (0x036BA6, i_036ba6),
    (0x036BAA, i_036baa),
    (0x036BAE, i_036bae),
    (0x036BB4, i_036bb4),
    (0x036BBA, i_036bba),
    (0x036BBE, i_036bbe),
    (0x036BC2, i_036bc2),
    (0x036BC4, i_036bc4),
    (0x036BC8, i_036bc8),
    (0x036BCE, i_036bce),
    (0x036BD2, i_036bd2),
    (0x036BD8, i_036bd8),
    (0x036BDC, i_036bdc),
    (0x036BE0, i_036be0),
    (0x036BE4, i_036be4),
    (0x036BE8, i_036be8),
    (0x036BEC, i_036bec),
    (0x036BEE, i_036bee),
    (0x036BF4, i_036bf4),
    (0x036BFA, i_036bfa),
    (0x036BFE, i_036bfe),
    (0x036C02, i_036c02),
    (0x036C06, i_036c06),
    (0x036C0A, i_036c0a),
    (0x036C10, i_036c10),
    (0x036C14, i_036c14),
    (0x036C18, i_036c18),
    (0x036C1A, i_036c1a),
    (0x036C20, i_036c20),
    (0x036C24, i_036c24),
    (0x036C2A, i_036c2a),
    (0x036C2E, i_036c2e),
    (0x036C32, i_036c32),
    (0x036C36, i_036c36),
    (0x036C3A, i_036c3a),
    (0x036C40, i_036c40),
    (0x036C44, i_036c44),
    (0x036C4C, i_036c4c),
    (0x036C50, i_036c50),
    (0x036C56, i_036c56),
    (0x036C5A, i_036c5a),
    (0x036C60, i_036c60),
    (0x036C64, i_036c64),
    (0x036C6A, i_036c6a),
    (0x036C6E, i_036c6e),
    (0x036C74, i_036c74),
    (0x036C78, i_036c78),
    (0x036C7C, i_036c7c),
    (0x036C7E, i_036c7e),
    (0x036C82, i_036c82),
    (0x036C86, i_036c86),
    (0x036C8A, i_036c8a),
    (0x036C8C, i_036c8c),
    (0x036C92, i_036c92),
    (0x036C96, i_036c96),
    (0x036C9C, i_036c9c),
    (0x036CA2, i_036ca2),
    (0x036CA6, i_036ca6),
    (0x036CAC, i_036cac),
    (0x036CB2, i_036cb2),
    (0x036CB6, i_036cb6),
    (0x036CBA, i_036cba),
    (0x036CC0, i_036cc0),
    (0x036CC6, i_036cc6),
    (0x036CCC, i_036ccc),
    (0x036CD2, i_036cd2),
    (0x036CD8, i_036cd8),
    (0x036CE0, i_036ce0),
    (0x036CE6, i_036ce6),
    (0x036CEC, i_036cec),
    (0x036CF2, i_036cf2),
    (0x036CF8, i_036cf8),
    (0x036CFE, i_036cfe),
    (0x036D04, i_036d04),
    (0x036D0A, i_036d0a),
    (0x036D0E, i_036d0e),
    (0x036D12, i_036d12),
    (0x036D16, i_036d16),
    (0x036D1A, i_036d1a),
    (0x036D1E, i_036d1e),
    (0x036D20, i_036d20),
    (0x036D26, i_036d26),
    (0x036D2C, i_036d2c),
    (0x036D32, i_036d32),
    (0x036D36, i_036d36),
    (0x036D3A, i_036d3a),
    (0x036D3C, i_036d3c),
    (0x036D40, i_036d40),
    (0x036D46, i_036d46),
    (0x036D4C, i_036d4c),
    (0x036D50, i_036d50),
    (0x036D52, i_036d52),
    (0x036D56, i_036d56),
    (0x036D5C, i_036d5c),
    (0x036D62, i_036d62),
    (0x036D64, i_036d64),
    (0x036D68, i_036d68),
    (0x036D6C, i_036d6c),
    (0x036D72, i_036d72),
    (0x036D78, i_036d78),
    (0x036D7E, i_036d7e),
    (0x036D84, i_036d84),
    (0x036D8A, i_036d8a),
    (0x036D90, i_036d90),
    (0x036D98, i_036d98),
    (0x036D9E, i_036d9e),
    (0x036DA4, i_036da4),
    (0x036DAA, i_036daa),
    (0x036DB0, i_036db0),
    (0x036DB6, i_036db6),
    (0x036DBC, i_036dbc),
    (0x036DC0, i_036dc0),
    (0x036DC4, i_036dc4),
    (0x036DC8, i_036dc8),
    (0x036DCA, i_036dca),
    (0x036DD0, i_036dd0),
    (0x036DD4, i_036dd4),
    (0x036DDA, i_036dda),
    (0x036DDC, i_036ddc),
    (0x036DE0, i_036de0),
    (0x036DE6, i_036de6),
    (0x036DEA, i_036dea),
    (0x036DEE, i_036dee),
    (0x036DF4, i_036df4),
    (0x036DF8, i_036df8),
    (0x036DFE, i_036dfe),
    (0x036E02, i_036e02),
    (0x036E08, i_036e08),
    (0x036E0A, i_036e0a),
    (0x036E0E, i_036e0e),
    (0x036E14, i_036e14),
    (0x036E1A, i_036e1a),
    (0x036E1E, i_036e1e),
    (0x036E24, i_036e24),
    (0x036E28, i_036e28),
    (0x036E2E, i_036e2e),
    (0x036E32, i_036e32),
    (0x036E38, i_036e38),
    (0x036E3C, i_036e3c),
    (0x036E42, i_036e42),
    (0x036E46, i_036e46),
    (0x036E4A, i_036e4a),
    (0x036E4E, i_036e4e),
    (0x036E54, i_036e54),
    (0x036E58, i_036e58),
    (0x036E5C, i_036e5c),
    (0x036E60, i_036e60),
    (0x036E64, i_036e64),
    (0x036E68, i_036e68),
    (0x036E6E, i_036e6e),
    (0x036E72, i_036e72),
    (0x036E78, i_036e78),
    (0x036E7E, i_036e7e),
    (0x036E82, i_036e82),
    (0x036E88, i_036e88),
    (0x036E8C, i_036e8c),
    (0x036E90, i_036e90),
    (0x036E92, i_036e92),
    (0x036E98, i_036e98),
    (0x036E9E, i_036e9e),
    (0x036EA2, i_036ea2),
    (0x036EA6, i_036ea6),
    (0x036EAC, i_036eac),
    (0x036EB2, i_036eb2),
    (0x036EB4, i_036eb4),
    (0x036EBA, i_036eba),
    (0x036EBE, i_036ebe),
    (0x036EC2, i_036ec2),
    (0x036EC6, i_036ec6),
    (0x036ECA, i_036eca),
    (0x036ECE, i_036ece),
    (0x036ED4, i_036ed4),
    (0x036EDA, i_036eda),
    (0x036EDE, i_036ede),
    (0x036EE2, i_036ee2),
    (0x036EE4, i_036ee4),
    (0x036EE8, i_036ee8),
    (0x036EEE, i_036eee),
    (0x036EF2, i_036ef2),
    (0x036EF8, i_036ef8),
    (0x036EFC, i_036efc),
    (0x036F00, i_036f00),
    (0x036F04, i_036f04),
    (0x036F08, i_036f08),
    (0x036F0C, i_036f0c),
    (0x036F0E, i_036f0e),
    (0x036F14, i_036f14),
    (0x036F1A, i_036f1a),
    (0x036F1E, i_036f1e),
    (0x036F22, i_036f22),
    (0x036F26, i_036f26),
    (0x036F2A, i_036f2a),
    (0x036F30, i_036f30),
    (0x036F34, i_036f34),
    (0x036F38, i_036f38),
    (0x036F3A, i_036f3a),
    (0x036F40, i_036f40),
    (0x036F44, i_036f44),
    (0x036F4A, i_036f4a),
    (0x036F4E, i_036f4e),
    (0x036F52, i_036f52),
    (0x036F56, i_036f56),
    (0x036F5C, i_036f5c),
    (0x036F5E, i_036f5e),
    (0x036F64, i_036f64),
    (0x036F68, i_036f68),
    (0x036F6C, i_036f6c),
    (0x036F70, i_036f70),
    (0x036F76, i_036f76),
    (0x036F7A, i_036f7a),
    (0x036F82, i_036f82),
    (0x036F86, i_036f86),
    (0x036F8C, i_036f8c),
    (0x036F90, i_036f90),
    (0x036F96, i_036f96),
    (0x036F9A, i_036f9a),
    (0x036FA0, i_036fa0),
    (0x036FA4, i_036fa4),
    (0x036FAA, i_036faa),
    (0x036FAE, i_036fae),
    (0x036FB2, i_036fb2),
    (0x036FB4, i_036fb4),
    (0x036FB8, i_036fb8),
    (0x036FBC, i_036fbc),
    (0x036FC0, i_036fc0),
    (0x036FC2, i_036fc2),
    (0x036FC6, i_036fc6),
    (0x036FCA, i_036fca),
    (0x036FD0, i_036fd0),
    (0x036FD6, i_036fd6),
    (0x036FDC, i_036fdc),
    (0x036FE4, i_036fe4),
    (0x036FEA, i_036fea),
    (0x036FF0, i_036ff0),
    (0x036FF4, i_036ff4),
    (0x036FF6, i_036ff6),
    (0x036FFC, i_036ffc),
    (0x037000, i_037000),
    (0x037004, i_037004),
    (0x03700A, i_03700a),
    (0x03700E, i_03700e),
    (0x037012, i_037012),
    (0x037016, i_037016),
    (0x037018, i_037018),
    (0x03701C, i_03701c),
    (0x037020, i_037020),
    (0x037026, i_037026),
    (0x03702C, i_03702c),
    (0x037032, i_037032),
    (0x03703A, i_03703a),
    (0x037040, i_037040),
    (0x037044, i_037044),
    (0x037046, i_037046),
    (0x03704C, i_03704c),
    (0x037050, i_037050),
    (0x037054, i_037054),
    (0x037058, i_037058),
    (0x03705A, i_03705a),
    (0x037060, i_037060),
    (0x037064, i_037064),
    (0x03706A, i_03706a),
    (0x03706E, i_03706e),
    (0x037074, i_037074),
    (0x037078, i_037078),
    (0x03707E, i_03707e),
    (0x037082, i_037082),
    (0x037088, i_037088),
    (0x03708E, i_03708e),
    (0x037094, i_037094),
    (0x037098, i_037098),
    (0x03709E, i_03709e),
    (0x0370A4, i_0370a4),
    (0x0370AA, i_0370aa),
    (0x0370B0, i_0370b0),
    (0x0370B6, i_0370b6),
    (0x0370BA, i_0370ba),
    (0x0370C0, i_0370c0),
    (0x0370C6, i_0370c6),
    (0x0370CC, i_0370cc),
    (0x0370D2, i_0370d2),
    (0x0370D8, i_0370d8),
    (0x0370DE, i_0370de),
    (0x0370E2, i_0370e2),
    (0x0370E8, i_0370e8),
    (0x0370EC, i_0370ec),
    (0x0370F2, i_0370f2),
    (0x0370F6, i_0370f6),
    (0x0370FC, i_0370fc),
    (0x037102, i_037102),
    (0x037106, i_037106),
    (0x03710C, i_03710c),
    (0x037112, i_037112),
    (0x037118, i_037118),
    (0x03711E, i_03711e),
    (0x037124, i_037124),
    (0x03712A, i_03712a),
    (0x03712E, i_03712e),
    (0x037134, i_037134),
    (0x037138, i_037138),
    (0x03713E, i_03713e),
    (0x037144, i_037144),
    (0x03714A, i_03714a),
    (0x037150, i_037150),
    (0x037156, i_037156),
    (0x03715C, i_03715c),
    (0x037160, i_037160),
    (0x037166, i_037166),
    (0x037168, i_037168),
    (0x03716C, i_03716c),
    (0x037170, i_037170),
    (0x037176, i_037176),
    (0x03717A, i_03717a),
    (0x03717E, i_03717e),
    (0x037184, i_037184),
    (0x03718A, i_03718a),
    (0x03718E, i_03718e),
    (0x037194, i_037194),
    (0x03719A, i_03719a),
    (0x0371A0, i_0371a0),
    (0x0371A6, i_0371a6),
    (0x0371AC, i_0371ac),
    (0x0371B0, i_0371b0),
    (0x0371B6, i_0371b6),
    (0x0371BC, i_0371bc),
    (0x0371C2, i_0371c2),
    (0x0371C8, i_0371c8),
    (0x0371CE, i_0371ce),
    (0x0371D4, i_0371d4),
    (0x0371D8, i_0371d8),
    (0x0371DE, i_0371de),
    (0x0371E2, i_0371e2),
    (0x0371E6, i_0371e6),
    (0x0371EC, i_0371ec),
    (0x0371F0, i_0371f0),
    (0x0371F6, i_0371f6),
    (0x0371FC, i_0371fc),
    (0x037202, i_037202),
    (0x037208, i_037208),
    (0x03720C, i_03720c),
    (0x037210, i_037210),
    (0x037214, i_037214),
    (0x037218, i_037218),
    (0x03721C, i_03721c),
    (0x037220, i_037220),
    (0x037226, i_037226),
    (0x03722A, i_03722a),
    (0x03722E, i_03722e),
    (0x037234, i_037234),
    (0x037238, i_037238),
    (0x03723C, i_03723c),
    (0x037244, i_037244),
    (0x037248, i_037248),
    (0x03724A, i_03724a),
    (0x03724E, i_03724e),
    (0x037254, i_037254),
    (0x03725A, i_03725a),
    (0x037260, i_037260),
    (0x037266, i_037266),
    (0x03726C, i_03726c),
    (0x037272, i_037272),
    (0x037276, i_037276),
    (0x03727C, i_03727c),
    (0x037282, i_037282),
    (0x037288, i_037288),
    (0x03728E, i_03728e),
    (0x037294, i_037294),
    (0x03729A, i_03729a),
    (0x03729E, i_03729e),
    (0x0372A4, i_0372a4),
    (0x0372AA, i_0372aa),
    (0x0372B0, i_0372b0),
    (0x0372B6, i_0372b6),
    (0x0372BA, i_0372ba),
    (0x0372BE, i_0372be),
    (0x0372C2, i_0372c2),
    (0x0372C6, i_0372c6),
    (0x0372CE, i_0372ce),
    (0x0372D2, i_0372d2),
    (0x0372D4, i_0372d4),
    (0x0372D8, i_0372d8),
    (0x0372DE, i_0372de),
    (0x0372E4, i_0372e4),
    (0x0372EA, i_0372ea),
    (0x0372F0, i_0372f0),
    (0x0372F6, i_0372f6),
    (0x0372FC, i_0372fc),
    (0x037300, i_037300),
    (0x037306, i_037306),
    (0x03730C, i_03730c),
    (0x037312, i_037312),
    (0x037318, i_037318),
    (0x03731E, i_03731e),
    (0x037324, i_037324),
    (0x037328, i_037328),
    (0x03732C, i_03732c),
    (0x037330, i_037330),
    (0x037336, i_037336),
    (0x03733C, i_03733c),
    (0x037340, i_037340),
    (0x037346, i_037346),
    (0x03734C, i_03734c),
    (0x037352, i_037352),
    (0x037358, i_037358),
    (0x03735E, i_03735e),
    (0x037362, i_037362),
    (0x037368, i_037368),
    (0x03736E, i_03736e),
    (0x037374, i_037374),
    (0x03737A, i_03737a),
    (0x037380, i_037380),
    (0x037386, i_037386),
    (0x03738C, i_03738c),
    (0x037390, i_037390),
    (0x037394, i_037394),
    (0x037398, i_037398),
    (0x03739E, i_03739e),
    (0x0373A2, i_0373a2),
    (0x0373A8, i_0373a8),
    (0x0373AE, i_0373ae),
    (0x0373B4, i_0373b4),
    (0x0373BA, i_0373ba),
    (0x0373C0, i_0373c0),
    (0x0373C4, i_0373c4),
    (0x0373CA, i_0373ca),
    (0x0373D0, i_0373d0),
    (0x0373D6, i_0373d6),
    (0x0373DC, i_0373dc),
    (0x0373E2, i_0373e2),
    (0x0373E8, i_0373e8),
    (0x0373EE, i_0373ee),
    (0x0373F2, i_0373f2),
    (0x0373FA, i_0373fa),
    (0x0373FE, i_0373fe),
    (0x037402, i_037402),
    (0x037406, i_037406),
    (0x03740C, i_03740c),
    (0x037410, i_037410),
    (0x037416, i_037416),
    (0x03741C, i_03741c),
    (0x037422, i_037422),
    (0x037428, i_037428),
    (0x03742E, i_03742e),
    (0x037434, i_037434),
    (0x037438, i_037438),
    (0x03743E, i_03743e),
    (0x037444, i_037444),
    (0x03744A, i_03744a),
    (0x037450, i_037450),
    (0x037456, i_037456),
    (0x03745C, i_03745c),
    (0x037462, i_037462),
    (0x037468, i_037468),
    (0x03746C, i_03746c),
    (0x037472, i_037472),
    (0x037476, i_037476),
    (0x03747C, i_03747c),
    (0x037482, i_037482),
    (0x037486, i_037486),
    (0x03748A, i_03748a),
    (0x03748E, i_03748e),
    (0x037492, i_037492),
    (0x037496, i_037496),
    (0x03749A, i_03749a),
    (0x0374A0, i_0374a0),
    (0x0374A4, i_0374a4),
    (0x0374A8, i_0374a8),
    (0x0374AE, i_0374ae),
    (0x0374B2, i_0374b2),
    (0x0374B6, i_0374b6),
    (0x0374BE, i_0374be),
    (0x0374C2, i_0374c2),
    (0x0374C4, i_0374c4),
    (0x0374C8, i_0374c8),
    (0x0374CE, i_0374ce),
    (0x0374D4, i_0374d4),
    (0x0374DA, i_0374da),
    (0x0374E0, i_0374e0),
    (0x0374E6, i_0374e6),
    (0x0374EC, i_0374ec),
    (0x0374F0, i_0374f0),
    (0x0374F6, i_0374f6),
    (0x0374FC, i_0374fc),
    (0x037502, i_037502),
    (0x037508, i_037508),
    (0x03750E, i_03750e),
    (0x037514, i_037514),
    (0x037518, i_037518),
    (0x03751E, i_03751e),
    (0x037524, i_037524),
    (0x037528, i_037528),
    (0x03752E, i_03752e),
    (0x037534, i_037534),
    (0x037538, i_037538),
    (0x03753C, i_03753c),
    (0x037540, i_037540),
    (0x037544, i_037544),
    (0x037548, i_037548),
    (0x03754C, i_03754c),
    (0x037552, i_037552),
    (0x037556, i_037556),
    (0x03755A, i_03755a),
    (0x037560, i_037560),
    (0x037564, i_037564),
    (0x037568, i_037568),
    (0x037570, i_037570),
    (0x037574, i_037574),
    (0x037576, i_037576),
    (0x03757A, i_03757a),
    (0x037580, i_037580),
    (0x037586, i_037586),
    (0x03758C, i_03758c),
    (0x037592, i_037592),
    (0x037598, i_037598),
    (0x03759E, i_03759e),
    (0x0375A2, i_0375a2),
    (0x0375A8, i_0375a8),
    (0x0375AE, i_0375ae),
    (0x0375B4, i_0375b4),
    (0x0375BA, i_0375ba),
    (0x0375C0, i_0375c0),
    (0x0375C6, i_0375c6),
    (0x0375CC, i_0375cc),
    (0x0375D0, i_0375d0),
    (0x0375D2, i_0375d2),
    (0x0375D8, i_0375d8),
    (0x0375DE, i_0375de),
    (0x0375E4, i_0375e4),
    (0x0375EA, i_0375ea),
    (0x0375EE, i_0375ee),
    (0x0375F4, i_0375f4),
    (0x0375FA, i_0375fa),
    (0x037600, i_037600),
    (0x037606, i_037606),
    (0x03760C, i_03760c),
    (0x037612, i_037612),
    (0x037618, i_037618),
    (0x03761C, i_03761c),
    (0x03761E, i_03761e),
    (0x037626, i_037626),
    (0x03762A, i_03762a),
    (0x037630, i_037630),
    (0x037634, i_037634),
    (0x03763A, i_03763a),
    (0x03763E, i_03763e),
    (0x037644, i_037644),
    (0x037648, i_037648),
    (0x03764E, i_03764e),
    (0x037652, i_037652),
    (0x037656, i_037656),
    (0x037658, i_037658),
    (0x03765A, i_03765a),
    (0x03765E, i_03765e),
    (0x037660, i_037660),
    (0x037666, i_037666),
    (0x03766A, i_03766a),
    (0x03766E, i_03766e),
    (0x037674, i_037674),
    (0x037678, i_037678),
    (0x03767C, i_03767c),
    (0x037682, i_037682),
    (0x037684, i_037684),
    (0x03768A, i_03768a),
    (0x03768E, i_03768e),
    (0x037692, i_037692),
    (0x037698, i_037698),
    (0x03769C, i_03769c),
    (0x0376A2, i_0376a2),
    (0x0376A8, i_0376a8),
    (0x0376AE, i_0376ae),
    (0x0376B4, i_0376b4),
    (0x0376BA, i_0376ba),
    (0x0376C0, i_0376c0),
    (0x0376C6, i_0376c6),
    (0x0376CA, i_0376ca),
    (0x0376D0, i_0376d0),
    (0x0376D6, i_0376d6),
    (0x0376DC, i_0376dc),
    (0x0376E2, i_0376e2),
    (0x0376E8, i_0376e8),
    (0x0376EE, i_0376ee),
    (0x0376F4, i_0376f4),
    (0x0376FA, i_0376fa),
    (0x037700, i_037700),
    (0x037706, i_037706),
    (0x03770C, i_03770c),
    (0x037712, i_037712),
    (0x037718, i_037718),
    (0x03771E, i_03771e),
    (0x037722, i_037722),
    (0x037724, i_037724),
    (0x037728, i_037728),
    (0x03772E, i_03772e),
    (0x037732, i_037732),
    (0x037736, i_037736),
    (0x03773A, i_03773a),
    (0x03773C, i_03773c),
    (0x037744, i_037744),
    (0x037748, i_037748),
    (0x03774E, i_03774e),
    (0x037752, i_037752),
    (0x037758, i_037758),
    (0x03775C, i_03775c),
    (0x037762, i_037762),
    (0x037766, i_037766),
    (0x03776C, i_03776c),
    (0x037770, i_037770),
    (0x037774, i_037774),
    (0x037776, i_037776),
    (0x037778, i_037778),
    (0x03777E, i_03777e),
    (0x037784, i_037784),
    (0x03778A, i_03778a),
    (0x037790, i_037790),
    (0x037796, i_037796),
    (0x03779C, i_03779c),
    (0x0377A2, i_0377a2),
    (0x0377A8, i_0377a8),
    (0x0377AE, i_0377ae),
    (0x0377B4, i_0377b4),
    (0x0377BA, i_0377ba),
    (0x0377C0, i_0377c0),
    (0x0377C6, i_0377c6),
    (0x0377CA, i_0377ca),
    (0x0377D0, i_0377d0),
    (0x0377D4, i_0377d4),
    (0x0377DA, i_0377da),
    (0x0377E0, i_0377e0),
    (0x0377E6, i_0377e6),
    (0x0377EA, i_0377ea),
    (0x0377EC, i_0377ec),
    (0x0377F2, i_0377f2),
    (0x0377F6, i_0377f6),
    (0x0377FC, i_0377fc),
    (0x037800, i_037800),
    (0x037804, i_037804),
    (0x037806, i_037806),
    (0x03780E, i_03780e),
    (0x037812, i_037812),
    (0x037816, i_037816),
    (0x03781A, i_03781a),
    (0x03781E, i_03781e),
    (0x037822, i_037822),
    (0x037828, i_037828),
    (0x03782C, i_03782c),
    (0x037830, i_037830),
    (0x037834, i_037834),
    (0x037838, i_037838),
    (0x037840, i_037840),
    (0x037844, i_037844),
    (0x03784A, i_03784a),
    (0x03784E, i_03784e),
    (0x037854, i_037854),
    (0x037858, i_037858),
    (0x03785E, i_03785e),
    (0x037862, i_037862),
    (0x037868, i_037868),
    (0x03786C, i_03786c),
    (0x037870, i_037870),
    (0x037872, i_037872),
    (0x037874, i_037874),
    (0x037878, i_037878),
    (0x03787C, i_03787c),
    (0x037882, i_037882),
    (0x037888, i_037888),
    (0x03788E, i_03788e),
    (0x037894, i_037894),
    (0x03789A, i_03789a),
    (0x0378A0, i_0378a0),
    (0x0378A6, i_0378a6),
    (0x0378AA, i_0378aa),
    (0x0378AC, i_0378ac),
    (0x0378B0, i_0378b0),
    (0x0378B6, i_0378b6),
    (0x0378BA, i_0378ba),
    (0x0378BE, i_0378be),
    (0x0378C2, i_0378c2),
    (0x0378C4, i_0378c4),
    (0x0378C6, i_0378c6),
    (0x0378CA, i_0378ca),
    (0x0378D0, i_0378d0),
    (0x0378D6, i_0378d6),
    (0x0378DA, i_0378da),
    (0x0378E0, i_0378e0),
    (0x0378E6, i_0378e6),
    (0x0378EC, i_0378ec),
    (0x0378F2, i_0378f2),
    (0x0378F8, i_0378f8),
    (0x0378FE, i_0378fe),
    (0x037904, i_037904),
    (0x03790A, i_03790a),
    (0x037910, i_037910),
    (0x037916, i_037916),
    (0x03791C, i_03791c),
    (0x037922, i_037922),
    (0x037928, i_037928),
    (0x03792E, i_03792e),
    (0x037934, i_037934),
    (0x037938, i_037938),
    (0x03793E, i_03793e),
    (0x037942, i_037942),
    (0x037948, i_037948),
    (0x03794E, i_03794e),
    (0x037954, i_037954),
    (0x037958, i_037958),
    (0x03795A, i_03795a),
    (0x037960, i_037960),
    (0x037964, i_037964),
    (0x03796A, i_03796a),
    (0x03796E, i_03796e),
    (0x037972, i_037972),
    (0x037974, i_037974),
    (0x03797C, i_03797c),
    (0x037980, i_037980),
    (0x037984, i_037984),
    (0x037988, i_037988),
    (0x03798C, i_03798c),
    (0x037990, i_037990),
    (0x037996, i_037996),
    (0x03799A, i_03799a),
    (0x03799E, i_03799e),
    (0x0379A2, i_0379a2),
    (0x0379A6, i_0379a6),
    (0x0379AE, i_0379ae),
    (0x0379B2, i_0379b2),
    (0x0379B8, i_0379b8),
    (0x0379BC, i_0379bc),
    (0x0379C2, i_0379c2),
    (0x0379C6, i_0379c6),
    (0x0379CC, i_0379cc),
    (0x0379D0, i_0379d0),
    (0x0379D6, i_0379d6),
    (0x0379DA, i_0379da),
    (0x0379DE, i_0379de),
    (0x0379E0, i_0379e0),
    (0x0379E2, i_0379e2),
    (0x0379E6, i_0379e6),
    (0x0379EA, i_0379ea),
    (0x0379F0, i_0379f0),
    (0x0379F6, i_0379f6),
    (0x0379FC, i_0379fc),
    (0x037A02, i_037a02),
    (0x037A08, i_037a08),
    (0x037A0E, i_037a0e),
    (0x037A14, i_037a14),
    (0x037A18, i_037a18),
    (0x037A1A, i_037a1a),
    (0x037A1E, i_037a1e),
    (0x037A24, i_037a24),
    (0x037A28, i_037a28),
    (0x037A2C, i_037a2c),
    (0x037A30, i_037a30),
    (0x037A32, i_037a32),
    (0x037A34, i_037a34),
    (0x037A3A, i_037a3a),
    (0x037A3E, i_037a3e),
    (0x037A44, i_037a44),
    (0x037A4A, i_037a4a),
    (0x037A50, i_037a50),
    (0x037A56, i_037a56),
    (0x037A5C, i_037a5c),
    (0x037A62, i_037a62),
    (0x037A68, i_037a68),
    (0x037A6E, i_037a6e),
    (0x037A74, i_037a74),
    (0x037A7A, i_037a7a),
];
