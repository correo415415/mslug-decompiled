// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p08_4) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $086B90: lea.l $2edd48.l, a1
pub fn i_086b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B96;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EDD48;
    20
}
/// $086B96: jsr $43fac.l
pub fn i_086b96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86B9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86B9C);
    cpu.pc = 0x43FAC;
    16
}
/// $086B9C: jmp $518.l
pub fn i_086b9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86BA2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $086BA2: rts
pub fn i_086ba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86BA4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pillbox_Collapse_086ba4  ($086BA4, 190 B, enemies/bosses/fort_scene4_0865be.s)
/// $086BA4: lea.l $86c62(pc), a1
pub fn i_086ba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86BA8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86C62;
    16
}
/// $086BA8: jsr $4ae.l
pub fn i_086ba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86BAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86BAE);
    cpu.pc = 0x4AE;
    16
}
/// $086BAE: jsr $5dd22.l
pub fn i_086bae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86BB4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86BB4);
    cpu.pc = 0x5DD22;
    16
}
/// $086BB4: lea.l $86bba(pc), a1
pub fn i_086bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86BB8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86BBA;
    16
}
/// $086BB8: move.l a1, (a6)
pub fn i_086bb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86BBA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $086BBA: movea.l $c(a6), a0
pub fn i_086bba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86BBE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $086BBE: cmpi.b #$ff, $21(a0)
pub fn i_086bbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86BC4;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $086BC4: beq.w $86c4e
pub fn i_086bc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86BC8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x86C4E; return 10; }
    8
}
/// $086BC8: jsr $2783a.l
pub fn i_086bc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86BCE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86BCE);
    cpu.pc = 0x2783A;
    16
}
/// $086BCE: jsr $28d70.l
pub fn i_086bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86BD4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86BD4);
    cpu.pc = 0x28D70;
    16
}
/// $086BD4: move.b $21(a6), d0
pub fn i_086bd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86BD8;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $086BD8: andi.b #$f, d0
pub fn i_086bd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86BDC;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $086BDC: cmpi.b #$f, d0
pub fn i_086bdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86BE0;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xF, Size::Byte, false);
    8
}
/// $086BE0: bne.w $86c60
pub fn i_086be0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86BE4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x86C60; return 10; }
    8
}
/// $086BE4: move.l #$100, d0
pub fn i_086be4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86BEA;
    cpu.ir = 0x203C;
    let v1: u32 = 0x100;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $086BEA: jsr $51a28.l
pub fn i_086bea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86BF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86BF0);
    cpu.pc = 0x51A28;
    16
}
/// $086BF0: move.w #$102c, d0
pub fn i_086bf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86BF4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $086BF4: jsr $2352.l
pub fn i_086bf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86BFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86BFA);
    cpu.pc = 0x2352;
    16
}
/// $086BFA: lea.l $78066.l, a1
pub fn i_086bfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C00;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x78066;
    20
}
/// $086C00: jsr $4ae.l
pub fn i_086c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C06;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86C06);
    cpu.pc = 0x4AE;
    16
}
/// $086C06: jsr $5dd22.l
pub fn i_086c06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C0C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86C0C);
    cpu.pc = 0x5DD22;
    16
}
/// $086C0C: addi.w #$10, $24(a0)
pub fn i_086c0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C12;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $086C12: movea.l $74(a6), a2
pub fn i_086c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C16;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $086C16: jsr $5022a.l
pub fn i_086c16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86C1C);
    cpu.pc = 0x5022A;
    16
}
/// $086C1C: lea.l $2ed05c.l, a1
pub fn i_086c1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C22;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED05C;
    20
}
/// $086C22: jsr $77c7e.l
pub fn i_086c22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86C28);
    cpu.pc = 0x77C7E;
    16
}
/// $086C28: addi.w #$10, $24(a0)
pub fn i_086c28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C2E;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $086C2E: movea.l $7c(a6), a1
pub fn i_086c2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C32;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $086C32: move.l a1, d0
pub fn i_086c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C34;
    cpu.ir = 0x2009;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $086C34: cmpi.l #$ffffffff, d0
pub fn i_086c34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C3A;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0xFFFFFFFF, Size::Long, false);
    8
}
/// $086C3A: beq.w $86c4e
pub fn i_086c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C3E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x86C4E; return 10; }
    8
}
/// $086C3E: move.w #$82, d0
pub fn i_086c3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C42;
    cpu.ir = 0x303C;
    let v1: u32 = 0x82;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $086C42: move.b #$0, $82(a6)
pub fn i_086c42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C48;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086C48: jsr $4429e.l
pub fn i_086c48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86C4E);
    cpu.pc = 0x4429E;
    16
}
/// $086C4E: lea.l $2edd48.l, a1
pub fn i_086c4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C54;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EDD48;
    20
}
/// $086C54: jsr $43fac.l
pub fn i_086c54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C5A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86C5A);
    cpu.pc = 0x43FAC;
    16
}
/// $086C5A: jmp $518.l
pub fn i_086c5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C60;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $086C60: rts
pub fn i_086c60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C62;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pillbox_Fragment_086c62  ($086C62, 110 B, enemies/bosses/fort_scene4_0865be.s)
/// $086C62: move.w #$6c, d1
pub fn i_086c62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C66;
    cpu.ir = 0x323C;
    let v1: u32 = 0x6C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $086C66: jsr $236e.l
pub fn i_086c66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86C6C);
    cpu.pc = 0x236E;
    16
}
/// $086C6C: move.b #$ff, $32(a6)
pub fn i_086c6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C72;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086C72: move.b #$ff, $33(a6)
pub fn i_086c72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C78;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086C78: move.b #$0, $3a(a6)
pub fn i_086c78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C7E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086C7E: move.w #$0, $38(a6)
pub fn i_086c7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C84;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $086C84: addi.w #$8, $24(a6)
pub fn i_086c84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C8A;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $086C8A: lea.l $2eb2e4.l, a0
pub fn i_086c8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C90;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EB2E4;
    20
}
/// $086C90: jsr $28cd4.l
pub fn i_086c90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86C96);
    cpu.pc = 0x28CD4;
    16
}
/// $086C96: lea.l $86c9c(pc), a1
pub fn i_086c96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C9A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86C9C;
    16
}
/// $086C9A: move.l a1, (a6)
pub fn i_086c9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86C9C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $086C9C: movea.l $c(a6), a0
pub fn i_086c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86CA0;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $086CA0: movea.l $c(a0), a0
pub fn i_086ca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86CA4;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $086CA4: cmpi.b #$ff, $21(a0)
pub fn i_086ca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86CAA;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $086CAA: beq.w $86cc8
pub fn i_086caa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86CAE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x86CC8; return 10; }
    8
}
/// $086CAE: jsr $2783a.l
pub fn i_086cae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86CB4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86CB4);
    cpu.pc = 0x2783A;
    16
}
/// $086CB4: jsr $28d70.l
pub fn i_086cb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86CBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86CBA);
    cpu.pc = 0x28D70;
    16
}
/// $086CBA: bcc.w $86cce
pub fn i_086cba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86CBE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x86CCE; return 10; }
    8
}
/// $086CBE: movea.l $c(a6), a0
pub fn i_086cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86CC2;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $086CC2: ori.b #$f, $21(a0)
pub fn i_086cc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86CC8;
    cpu.ir = 0x28;
    let ad1 = cpu.a[0].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0xF) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $086CC8: jmp $518.l
pub fn i_086cc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86CCE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $086CCE: rts
pub fn i_086cce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86CD0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crate_Init_086cd0  ($086CD0, 222 B, enemies/bosses/fort_scene4_0865be.s)
/// $086CD0: move.w #$e7, d1
pub fn i_086cd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86CD4;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $086CD4: jsr $236e.l
pub fn i_086cd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86CDA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86CDA);
    cpu.pc = 0x236E;
    16
}
/// $086CDA: move.b #$0, $21(a6)
pub fn i_086cda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86CE0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086CE0: bra.w $86d34
pub fn i_086ce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86CE4;
    cpu.ir = 0x6000;
    cpu.pc = 0x86D34;
    10
}
/// $086CE4: move.w #$e8, d1
pub fn i_086ce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86CE8;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $086CE8: jsr $236e.l
pub fn i_086ce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86CEE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86CEE);
    cpu.pc = 0x236E;
    16
}
/// $086CEE: move.b #$1, $21(a6)
pub fn i_086cee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86CF4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086CF4: bra.w $86d34
pub fn i_086cf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86CF8;
    cpu.ir = 0x6000;
    cpu.pc = 0x86D34;
    10
}
/// $086CF8: move.w #$e9, d1
pub fn i_086cf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86CFC;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $086CFC: jsr $236e.l
pub fn i_086cfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86D02);
    cpu.pc = 0x236E;
    16
}
/// $086D02: move.b #$2, $21(a6)
pub fn i_086d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D08;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086D08: bra.w $86d34
pub fn i_086d08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D0C;
    cpu.ir = 0x6000;
    cpu.pc = 0x86D34;
    10
}
/// $086D0C: move.w #$ea, d1
pub fn i_086d0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D10;
    cpu.ir = 0x323C;
    let v1: u32 = 0xEA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $086D10: jsr $236e.l
pub fn i_086d10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D16;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86D16);
    cpu.pc = 0x236E;
    16
}
/// $086D16: move.b #$3, $21(a6)
pub fn i_086d16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D1C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086D1C: bra.w $86d34
pub fn i_086d1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D20;
    cpu.ir = 0x6000;
    cpu.pc = 0x86D34;
    10
}
/// $086D20: move.w #$eb, d1
pub fn i_086d20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D24;
    cpu.ir = 0x323C;
    let v1: u32 = 0xEB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $086D24: jsr $236e.l
pub fn i_086d24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86D2A);
    cpu.pc = 0x236E;
    16
}
/// $086D2A: move.b #$4, $21(a6)
pub fn i_086d2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D30;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086D30: bra.w $86d34
pub fn i_086d30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D34;
    cpu.ir = 0x6000;
    cpu.pc = 0x86D34;
    10
}
/// $086D34: move.b #$ff, $32(a6)
pub fn i_086d34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D3A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086D3A: move.b #$ff, $33(a6)
pub fn i_086d3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D40;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086D40: move.b #$0, $3a(a6)
pub fn i_086d40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D46;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086D46: move.w #$14, $66(a6)
pub fn i_086d46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D4C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $086D4C: move.w #$0, $38(a6)
pub fn i_086d4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D52;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $086D52: lea.l $86d58(pc), a1
pub fn i_086d52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D56;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86D58;
    16
}
/// $086D56: move.l a1, (a6)
pub fn i_086d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D58;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $086D58: movea.l $c(a6), a0
pub fn i_086d58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D5C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $086D5C: cmpi.b #$ff, $21(a0)
pub fn i_086d5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D62;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $086D62: beq.w $86da8
pub fn i_086d62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D66;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x86DA8; return 10; }
    8
}
/// $086D66: jsr $2783a.l
pub fn i_086d66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86D6C);
    cpu.pc = 0x2783A;
    16
}
/// $086D6C: jsr $2870a.l
pub fn i_086d6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86D72);
    cpu.pc = 0x2870A;
    16
}
/// $086D72: bcc.w $86d8c
pub fn i_086d72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D76;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x86D8C; return 10; }
    8
}
/// $086D76: lea.l $5e766.l, a0
pub fn i_086d76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D7C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $086D7C: jsr $5e770.l
pub fn i_086d7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86D82);
    cpu.pc = 0x5E770;
    16
}
/// $086D82: bclr.b #$3, $13(a6)
pub fn i_086d82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D88;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $086D88: jsr $8848c(pc)
pub fn i_086d88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D8C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x86D8C);
    cpu.pc = 0x8848C;
    16
}
/// $086D8C: jsr $28758.l
pub fn i_086d8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86D92);
    cpu.pc = 0x28758;
    16
}
/// $086D92: bcc.w $86dae
pub fn i_086d92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D96;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x86DAE; return 10; }
    8
}
/// $086D96: lea.l $ffff.w, a0
pub fn i_086d96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D9A;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $086D9A: move.l a0, $48(a6)
pub fn i_086d9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86D9E;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $086D9E: lea.l $86db4(pc), a1
pub fn i_086d9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86DA2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86DB4;
    16
}
/// $086DA2: move.l a1, (a6)
pub fn i_086da2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86DA4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $086DA4: bra.w $86dae
pub fn i_086da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86DA8;
    cpu.ir = 0x6000;
    cpu.pc = 0x86DAE;
    10
}
/// $086DA8: jmp $518.l
pub fn i_086da8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86DAE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== JsrPcThunk_086dae  ($086DAE, 6 B, thunks/jsr_pc_thunks.c)
/// $086DAE: jsr $88438(pc)
pub fn i_086dae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86DB2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x86DB2);
    cpu.pc = 0x88438;
    16
}
/// $086DB2: rts
pub fn i_086db2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86DB4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crate_Burst_086db4  ($086DB4, 150 B, enemies/bosses/fort_scene4_0865be.s)
/// $086DB4: move.l #$100, d0
pub fn i_086db4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86DBA;
    cpu.ir = 0x203C;
    let v1: u32 = 0x100;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $086DBA: jsr $51a28.l
pub fn i_086dba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86DC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86DC0);
    cpu.pc = 0x51A28;
    16
}
/// $086DC0: move.w #$1039, d0
pub fn i_086dc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86DC4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1039;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $086DC4: jsr $2352.l
pub fn i_086dc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86DCA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86DCA);
    cpu.pc = 0x2352;
    16
}
/// $086DCA: clr.l d0
pub fn i_086dca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86DCC;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $086DCC: move.b $21(a6), d0
pub fn i_086dcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86DD0;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $086DD0: movea.l #$2eb6ba, a0
pub fn i_086dd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86DD6;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2EB6BA;
    cpu.a[0] = v1;
    4
}
/// $086DD6: lsl.w #$2, d0
pub fn i_086dd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86DD8;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $086DD8: movea.l (a0, d0.w), a0
pub fn i_086dd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86DDC;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $086DDC: cmpa.l #$ffffffff, a0
pub fn i_086ddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86DE2;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $086DE2: beq.w $86dec
pub fn i_086de2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86DE6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x86DEC; return 10; }
    8
}
/// $086DE6: jsr $28cd4.l
pub fn i_086de6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86DEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86DEC);
    cpu.pc = 0x28CD4;
    16
}
/// $086DEC: lea.l $86df2(pc), a1
pub fn i_086dec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86DF0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86DF2;
    16
}
/// $086DF0: move.l a1, (a6)
pub fn i_086df0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86DF2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $086DF2: movea.l $c(a6), a0
pub fn i_086df2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86DF6;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $086DF6: cmpi.b #$ff, $21(a0)
pub fn i_086df6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86DFC;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $086DFC: beq.w $86e42
pub fn i_086dfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E00;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x86E42; return 10; }
    8
}
/// $086E00: jsr $2783a.l
pub fn i_086e00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E06;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86E06);
    cpu.pc = 0x2783A;
    16
}
/// $086E06: jsr $28d70.l
pub fn i_086e06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E0C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86E0C);
    cpu.pc = 0x28D70;
    16
}
/// $086E0C: bcc.w $86e48
pub fn i_086e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E10;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x86E48; return 10; }
    8
}
/// $086E10: move.w #$102f, d0
pub fn i_086e10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E14;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $086E14: jsr $2352.l
pub fn i_086e14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86E1A);
    cpu.pc = 0x2352;
    16
}
/// $086E1A: lea.l $7808a.l, a1
pub fn i_086e1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E20;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x7808A;
    20
}
/// $086E20: jsr $4ae.l
pub fn i_086e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86E26);
    cpu.pc = 0x4AE;
    16
}
/// $086E26: jsr $5dd22.l
pub fn i_086e26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E2C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86E2C);
    cpu.pc = 0x5DD22;
    16
}
/// $086E2C: movea.l $74(a6), a2
pub fn i_086e2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E30;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $086E30: jsr $5022a.l
pub fn i_086e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86E36);
    cpu.pc = 0x5022A;
    16
}
/// $086E36: lea.l $2ed05c.l, a1
pub fn i_086e36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E3C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED05C;
    20
}
/// $086E3C: jsr $77c7e.l
pub fn i_086e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E42;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86E42);
    cpu.pc = 0x77C7E;
    16
}
/// $086E42: jmp $518.l
pub fn i_086e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E48;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $086E48: rts
pub fn i_086e48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E4A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hatch_Init_086e4a  ($086E4A, 282 B, enemies/bosses/fort_scene4_0865be.s)
/// $086E4A: move.w #$30, $70(a6)
pub fn i_086e4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E50;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $086E50: move.b #$0, $20(a6)
pub fn i_086e50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E56;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086E56: move.b #$1, $21(a6)
pub fn i_086e56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E5C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086E5C: lea.l $ede9e.l, a1
pub fn i_086e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E62;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEDE9E;
    20
}
/// $086E62: move.l a1, $7c(a6)
pub fn i_086e62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E66;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $086E66: lea.l $edcb2.l, a1
pub fn i_086e66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E6C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEDCB2;
    20
}
/// $086E6C: move.l a1, $8c(a6)
pub fn i_086e6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E70;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $086E70: bra.w $86f64
pub fn i_086e70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E74;
    cpu.ir = 0x6000;
    cpu.pc = 0x86F64;
    10
}
/// $086E74: move.w #$30, $70(a6)
pub fn i_086e74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E7A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $086E7A: move.b #$1, $20(a6)
pub fn i_086e7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E80;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086E80: move.b #$1, $21(a6)
pub fn i_086e80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E86;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086E86: lea.l $ededa.l, a1
pub fn i_086e86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E8C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEDEDA;
    20
}
/// $086E8C: move.l a1, $7c(a6)
pub fn i_086e8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E90;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $086E90: lea.l $edd0a.l, a1
pub fn i_086e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E96;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEDD0A;
    20
}
/// $086E96: move.l a1, $8c(a6)
pub fn i_086e96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E9A;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $086E9A: bra.w $86f64
pub fn i_086e9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86E9E;
    cpu.ir = 0x6000;
    cpu.pc = 0x86F64;
    10
}
/// $086E9E: move.w #$30, $70(a6)
pub fn i_086e9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86EA4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $086EA4: move.b #$2, $20(a6)
pub fn i_086ea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86EAA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086EAA: move.b #$1, $21(a6)
pub fn i_086eaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86EB0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086EB0: lea.l $edefa.l, a1
pub fn i_086eb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86EB6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEDEFA;
    20
}
/// $086EB6: move.l a1, $7c(a6)
pub fn i_086eb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86EBA;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $086EBA: lea.l $edd86.l, a1
pub fn i_086eba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86EC0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEDD86;
    20
}
/// $086EC0: move.l a1, $8c(a6)
pub fn i_086ec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86EC4;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $086EC4: bra.w $86f64
pub fn i_086ec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86EC8;
    cpu.ir = 0x6000;
    cpu.pc = 0x86F64;
    10
}
/// $086EC8: move.w #$20, $70(a6)
pub fn i_086ec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86ECE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $086ECE: move.b #$3, $20(a6)
pub fn i_086ece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86ED4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086ED4: move.b #$1, $21(a6)
pub fn i_086ed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86EDA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086EDA: lea.l $edf1a.l, a1
pub fn i_086eda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86EE0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEDF1A;
    20
}
/// $086EE0: move.l a1, $7c(a6)
pub fn i_086ee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86EE4;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $086EE4: lea.l $ede02.l, a1
pub fn i_086ee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86EEA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEDE02;
    20
}
/// $086EEA: move.l a1, $8c(a6)
pub fn i_086eea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86EEE;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $086EEE: bra.w $86f64
pub fn i_086eee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86EF2;
    cpu.ir = 0x6000;
    cpu.pc = 0x86F64;
    10
}
/// $086EF2: move.w #$20, $70(a6)
pub fn i_086ef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86EF8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $086EF8: move.b #$4, $20(a6)
pub fn i_086ef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86EFE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086EFE: move.b #$1, $21(a6)
pub fn i_086efe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F04;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086F04: lea.l $edf52.l, a1
pub fn i_086f04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F0A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEDF52;
    20
}
/// $086F0A: move.l a1, $7c(a6)
pub fn i_086f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F0E;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $086F0E: lea.l $ede7e.l, a1
pub fn i_086f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F14;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEDE7E;
    20
}
/// $086F14: move.l a1, $8c(a6)
pub fn i_086f14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F18;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $086F18: bra.w $86f64
pub fn i_086f18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F1C;
    cpu.ir = 0x6000;
    cpu.pc = 0x86F64;
    10
}
/// $086F1C: movea.l $3c(a6), a1
pub fn i_086f1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F20;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $086F20: jsr $2942a.l
pub fn i_086f20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86F26);
    cpu.pc = 0x2942A;
    16
}
/// $086F26: move.w #$30, $70(a6)
pub fn i_086f26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F2C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $086F2C: move.b #$5, $20(a6)
pub fn i_086f2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F32;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086F32: lea.l $edf72.l, a1
pub fn i_086f32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F38;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEDF72;
    20
}
/// $086F38: move.l a1, $7c(a6)
pub fn i_086f38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F3C;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $086F3C: bra.w $86f64
pub fn i_086f3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F40;
    cpu.ir = 0x6000;
    cpu.pc = 0x86F64;
    10
}
/// $086F40: movea.l $3c(a6), a1
pub fn i_086f40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F44;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $086F44: jsr $2942a.l
pub fn i_086f44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86F4A);
    cpu.pc = 0x2942A;
    16
}
/// $086F4A: move.w #$30, $70(a6)
pub fn i_086f4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F50;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $086F50: move.b #$6, $20(a6)
pub fn i_086f50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F56;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086F56: lea.l $edfaa.l, a1
pub fn i_086f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F5C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEDFAA;
    20
}
/// $086F5C: move.l a1, $7c(a6)
pub fn i_086f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F60;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $086F60: bra.w $86f64
pub fn i_086f60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F64;
    cpu.ir = 0x6000;
    cpu.pc = 0x86F64;
    10
}
// ===== Hatch_WaitClosed_086f64  ($086F64, 80 B, enemies/bosses/fort_scene4_0865be.s)
/// $086F64: move.w #$5a, $72(a6)
pub fn i_086f64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F6A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x5A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $086F6A: lea.l $86f70(pc), a1
pub fn i_086f6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F6E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86F70;
    16
}
/// $086F6E: move.l a1, (a6)
pub fn i_086f6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F70;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $086F70: jsr $2783a.l
pub fn i_086f70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86F76);
    cpu.pc = 0x2783A;
    16
}
/// $086F76: subq.w #$1, $72(a6)
pub fn i_086f76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F7A;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $086F7A: bne.w $86f84
pub fn i_086f7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F7E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x86F84; return 10; }
    8
}
/// $086F7E: lea.l $86fb4(pc), a1
pub fn i_086f7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F82;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86FB4;
    16
}
/// $086F82: move.l a1, (a6)
pub fn i_086f82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F84;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $086F84: cmpi.b #$1, $21(a6)
pub fn i_086f84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F8A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $086F8A: bne.w $86fa2
pub fn i_086f8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F8E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x86FA2; return 10; }
    8
}
/// $086F8E: movea.l $c(a6), a0
pub fn i_086f8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F92;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $086F92: cmpi.b #$ff, $21(a0)
pub fn i_086f92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F98;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $086F98: bne.w $86fa2
pub fn i_086f98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86F9C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x86FA2; return 10; }
    8
}
/// $086F9C: lea.l $87108(pc), a1
pub fn i_086f9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86FA0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87108;
    16
}
/// $086FA0: move.l a1, (a6)
pub fn i_086fa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86FA2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $086FA2: jsr $4fa70.l
pub fn i_086fa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86FA8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86FA8);
    cpu.pc = 0x4FA70;
    16
}
/// $086FA8: bcc.w $86fb2
pub fn i_086fa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86FAC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x86FB2; return 10; }
    8
}
/// $086FAC: jmp $518.l
pub fn i_086fac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86FB2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $086FB2: rts
pub fn i_086fb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86FB4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hatch_Opening_086fb4  ($086FB4, 80 B, enemies/bosses/fort_scene4_0865be.s)
/// $086FB4: clr.l d0
pub fn i_086fb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86FB6;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $086FB6: move.b $20(a6), d0
pub fn i_086fb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86FBA;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $086FBA: movea.l #$2ebfae, a0
pub fn i_086fba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86FC0;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2EBFAE;
    cpu.a[0] = v1;
    4
}
/// $086FC0: lsl.w #$2, d0
pub fn i_086fc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86FC2;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $086FC2: movea.l (a0, d0.w), a0
pub fn i_086fc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86FC6;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $086FC6: cmpa.l #$ffffffff, a0
pub fn i_086fc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86FCC;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $086FCC: beq.w $86fd6
pub fn i_086fcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86FD0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x86FD6; return 10; }
    8
}
/// $086FD0: jsr $28cd4.l
pub fn i_086fd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86FD6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86FD6);
    cpu.pc = 0x28CD4;
    16
}
/// $086FD6: lea.l $86fdc(pc), a1
pub fn i_086fd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86FDA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86FDC;
    16
}
/// $086FDA: move.l a1, (a6)
pub fn i_086fda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86FDC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $086FDC: jsr $2783a.l
pub fn i_086fdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86FE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86FE2);
    cpu.pc = 0x2783A;
    16
}
/// $086FE2: jsr $28d70.l
pub fn i_086fe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86FE8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86FE8);
    cpu.pc = 0x28D70;
    16
}
/// $086FE8: bcc.w $86ff2
pub fn i_086fe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86FEC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x86FF2; return 10; }
    8
}
/// $086FEC: lea.l $87004(pc), a1
pub fn i_086fec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86FF0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87004;
    16
}
/// $086FF0: move.l a1, (a6)
pub fn i_086ff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86FF2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $086FF2: jsr $4fa70.l
pub fn i_086ff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86FF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86FF8);
    cpu.pc = 0x4FA70;
    16
}
/// $086FF8: bcc.w $87002
pub fn i_086ff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86FFC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x87002; return 10; }
    8
}
/// $086FFC: jmp $518.l
pub fn i_086ffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87002;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $087002: rts
pub fn i_087002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87004;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hatch_SpawnTroops_087004  ($087004, 100 B, enemies/bosses/fort_scene4_0865be.s)
/// $087004: move.w #$28, $72(a6)
pub fn i_087004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8700A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08700A: movea.l $7c(a6), a1
pub fn i_08700a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8700E;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08700E: move.w #$82, d0
pub fn i_08700e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87012;
    cpu.ir = 0x303C;
    let v1: u32 = 0x82;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $087012: move.b #$0, $82(a6)
pub fn i_087012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87018;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $087018: jsr $4429e.l
pub fn i_087018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8701E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8701E);
    cpu.pc = 0x4429E;
    16
}
/// $08701E: lea.l $87024(pc), a1
pub fn i_08701e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87022;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87024;
    16
}
/// $087022: move.l a1, (a6)
pub fn i_087022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87024;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087024: jsr $2783a.l
pub fn i_087024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8702A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8702A);
    cpu.pc = 0x2783A;
    16
}
/// $08702A: jsr $86552(pc)
pub fn i_08702a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8702E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8702E);
    cpu.pc = 0x86552;
    16
}
/// $08702E: bcs.w $87038
pub fn i_08702e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87032;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x87038; return 10; }
    8
}
/// $087032: lea.l $87068(pc), a1
pub fn i_087032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87036;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87068;
    16
}
/// $087036: move.l a1, (a6)
pub fn i_087036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87038;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087038: cmpi.b #$1, $21(a6)
pub fn i_087038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8703E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $08703E: bne.w $87056
pub fn i_08703e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87042;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x87056; return 10; }
    8
}
/// $087042: movea.l $c(a6), a0
pub fn i_087042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87046;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $087046: cmpi.b #$ff, $21(a0)
pub fn i_087046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8704C;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $08704C: bne.w $87056
pub fn i_08704c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87050;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x87056; return 10; }
    8
}
/// $087050: lea.l $87130(pc), a1
pub fn i_087050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87054;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87130;
    16
}
/// $087054: move.l a1, (a6)
pub fn i_087054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87056;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087056: jsr $4fa70.l
pub fn i_087056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8705C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8705C);
    cpu.pc = 0x4FA70;
    16
}
/// $08705C: bcc.w $87066
pub fn i_08705c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87060;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x87066; return 10; }
    8
}
/// $087060: jmp $518.l
pub fn i_087060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87066;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $087066: rts
pub fn i_087066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87068;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hatch_WaitOpen_087068  ($087068, 80 B, enemies/bosses/fort_scene4_0865be.s)
/// $087068: move.w #$50, $72(a6)
pub fn i_087068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8706E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x50;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08706E: lea.l $87074(pc), a1
pub fn i_08706e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87072;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87074;
    16
}
/// $087072: move.l a1, (a6)
pub fn i_087072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87074;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087074: jsr $2783a.l
pub fn i_087074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8707A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8707A);
    cpu.pc = 0x2783A;
    16
}
/// $08707A: subq.w #$1, $72(a6)
pub fn i_08707a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8707E;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08707E: bne.w $87088
pub fn i_08707e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87082;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x87088; return 10; }
    8
}
/// $087082: lea.l $870b8(pc), a1
pub fn i_087082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87086;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x870B8;
    16
}
/// $087086: move.l a1, (a6)
pub fn i_087086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87088;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087088: cmpi.b #$1, $21(a6)
pub fn i_087088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8708E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $08708E: bne.w $870a6
pub fn i_08708e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87092;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x870A6; return 10; }
    8
}
/// $087092: movea.l $c(a6), a0
pub fn i_087092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87096;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $087096: cmpi.b #$ff, $21(a0)
pub fn i_087096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8709C;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $08709C: bne.w $870a6
pub fn i_08709c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x870A0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x870A6; return 10; }
    8
}
/// $0870A0: lea.l $87130(pc), a1
pub fn i_0870a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x870A4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87130;
    16
}
/// $0870A4: move.l a1, (a6)
pub fn i_0870a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x870A6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0870A6: jsr $4fa70.l
pub fn i_0870a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x870AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x870AC);
    cpu.pc = 0x4FA70;
    16
}
/// $0870AC: bcc.w $870b6
pub fn i_0870ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x870B0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x870B6; return 10; }
    8
}
/// $0870B0: jmp $518.l
pub fn i_0870b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x870B6;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0870B6: rts
pub fn i_0870b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x870B8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hatch_Closing_0870b8  ($0870B8, 80 B, enemies/bosses/fort_scene4_0865be.s)
/// $0870B8: clr.l d0
pub fn i_0870b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x870BA;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $0870BA: move.b $20(a6), d0
pub fn i_0870ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x870BE;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0870BE: movea.l #$2ebfca, a0
pub fn i_0870be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x870C4;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2EBFCA;
    cpu.a[0] = v1;
    4
}
/// $0870C4: lsl.w #$2, d0
pub fn i_0870c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x870C6;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $0870C6: movea.l (a0, d0.w), a0
pub fn i_0870c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x870CA;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $0870CA: cmpa.l #$ffffffff, a0
pub fn i_0870ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x870D0;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $0870D0: beq.w $870da
pub fn i_0870d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x870D4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x870DA; return 10; }
    8
}
/// $0870D4: jsr $28cd4.l
pub fn i_0870d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x870DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x870DA);
    cpu.pc = 0x28CD4;
    16
}
/// $0870DA: lea.l $870e0(pc), a1
pub fn i_0870da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x870DE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x870E0;
    16
}
/// $0870DE: move.l a1, (a6)
pub fn i_0870de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x870E0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0870E0: jsr $2783a.l
pub fn i_0870e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x870E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x870E6);
    cpu.pc = 0x2783A;
    16
}
/// $0870E6: jsr $28d70.l
pub fn i_0870e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x870EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x870EC);
    cpu.pc = 0x28D70;
    16
}
/// $0870EC: bcc.w $870f6
pub fn i_0870ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x870F0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x870F6; return 10; }
    8
}
/// $0870F0: lea.l $86f64(pc), a1
pub fn i_0870f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x870F4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86F64;
    16
}
/// $0870F4: move.l a1, (a6)
pub fn i_0870f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x870F6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0870F6: jsr $4fa70.l
pub fn i_0870f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x870FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x870FC);
    cpu.pc = 0x4FA70;
    16
}
/// $0870FC: bcc.w $87106
pub fn i_0870fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87100;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x87106; return 10; }
    8
}
/// $087100: jmp $518.l
pub fn i_087100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87106;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $087106: rts
pub fn i_087106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87108;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hatch_FinalOpen_087108  ($087108, 40 B, enemies/bosses/fort_scene4_0865be.s)
/// $087108: clr.l d0
pub fn i_087108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8710A;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $08710A: move.b $20(a6), d0
pub fn i_08710a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8710E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08710E: movea.l #$2ebfae, a0
pub fn i_08710e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87114;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2EBFAE;
    cpu.a[0] = v1;
    4
}
/// $087114: lsl.w #$2, d0
pub fn i_087114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87116;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $087116: movea.l (a0, d0.w), a0
pub fn i_087116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8711A;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $08711A: cmpa.l #$ffffffff, a0
pub fn i_08711a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87120;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $087120: beq.w $8712a
pub fn i_087120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87124;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8712A; return 10; }
    8
}
/// $087124: jsr $28cd4.l
pub fn i_087124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8712A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8712A);
    cpu.pc = 0x28CD4;
    16
}
/// $08712A: lea.l $87130(pc), a1
pub fn i_08712a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8712E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87130;
    16
}
/// $08712E: move.l a1, (a6)
pub fn i_08712e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87130;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Hatch_FinalSpawn_087130  ($087130, 58 B, enemies/bosses/fort_scene4_0865be.s)
/// $087130: jsr $2783a.l
pub fn i_087130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87136;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87136);
    cpu.pc = 0x2783A;
    16
}
/// $087136: jsr $28d70.l
pub fn i_087136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8713C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8713C);
    cpu.pc = 0x28D70;
    16
}
/// $08713C: bcc.w $87158
pub fn i_08713c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87140;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x87158; return 10; }
    8
}
/// $087140: movea.l $8c(a6), a1
pub fn i_087140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87144;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $087144: move.w #$82, d0
pub fn i_087144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87148;
    cpu.ir = 0x303C;
    let v1: u32 = 0x82;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $087148: move.b #$0, $82(a6)
pub fn i_087148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8714E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08714E: jsr $4429e.l
pub fn i_08714e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87154;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87154);
    cpu.pc = 0x4429E;
    16
}
/// $087154: bra.w $87162
pub fn i_087154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87158;
    cpu.ir = 0x6000;
    cpu.pc = 0x87162;
    10
}
/// $087158: jsr $4fa70.l
pub fn i_087158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8715E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8715E);
    cpu.pc = 0x4FA70;
    16
}
/// $08715E: bcc.w $87168
pub fn i_08715e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87162;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x87168; return 10; }
    8
}
/// $087162: jmp $518.l
pub fn i_087162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87168;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $087168: rts
pub fn i_087168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8716A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Breakable_A_08716a  ($08716A, 144 B, enemies/bosses/fort_scene4_0865be.s)
/// $08716A: movea.l $3c(a6), a1
pub fn i_08716a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8716E;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08716E: jsr $2942a.l
pub fn i_08716e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87174;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87174);
    cpu.pc = 0x2942A;
    16
}
/// $087174: move.w #$20, $70(a6)
pub fn i_087174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8717A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08717A: lea.l $2ec726.l, a0
pub fn i_08717a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87180;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC726;
    20
}
/// $087180: move.l a0, $48(a6)
pub fn i_087180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87184;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $087184: lea.l $8718a(pc), a1
pub fn i_087184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87188;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8718A;
    16
}
/// $087188: move.l a1, (a6)
pub fn i_087188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8718A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08718A: jsr $2783a.l
pub fn i_08718a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87190;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87190);
    cpu.pc = 0x2783A;
    16
}
/// $087190: jsr $2870a.l
pub fn i_087190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87196;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87196);
    cpu.pc = 0x2870A;
    16
}
/// $087196: bcc.w $871ac
pub fn i_087196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8719A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x871AC; return 10; }
    8
}
/// $08719A: lea.l $5e766.l, a0
pub fn i_08719a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x871A0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $0871A0: jsr $5e770.l
pub fn i_0871a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x871A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x871A6);
    cpu.pc = 0x5E770;
    16
}
/// $0871A6: bclr.b #$3, $13(a6)
pub fn i_0871a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x871AC;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0871AC: jsr $28758.l
pub fn i_0871ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x871B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x871B2);
    cpu.pc = 0x28758;
    16
}
/// $0871B2: bcc.w $871e8
pub fn i_0871b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x871B6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x871E8; return 10; }
    8
}
/// $0871B6: move.w #$10a6, d0
pub fn i_0871b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x871BA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10A6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0871BA: jsr $2352.l
pub fn i_0871ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x871C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x871C0);
    cpu.pc = 0x2352;
    16
}
/// $0871C0: lea.l $2ed0ae.l, a1
pub fn i_0871c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x871C6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED0AE;
    20
}
/// $0871C6: jsr $77c7e.l
pub fn i_0871c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x871CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x871CC);
    cpu.pc = 0x77C7E;
    16
}
/// $0871CC: lea.l $2ed878.l, a2
pub fn i_0871cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x871D2;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED878;
    20
}
/// $0871D2: jsr $5022a.l
pub fn i_0871d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x871D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x871D8);
    cpu.pc = 0x5022A;
    16
}
/// $0871D8: lea.l $2ed864.l, a2
pub fn i_0871d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x871DE;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED864;
    20
}
/// $0871DE: jsr $5022a.l
pub fn i_0871de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x871E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x871E4);
    cpu.pc = 0x5022A;
    16
}
/// $0871E4: bra.w $871f2
pub fn i_0871e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x871E8;
    cpu.ir = 0x6000;
    cpu.pc = 0x871F2;
    10
}
/// $0871E8: jsr $4fa70.l
pub fn i_0871e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x871EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x871EE);
    cpu.pc = 0x4FA70;
    16
}
/// $0871EE: bcc.w $871f8
pub fn i_0871ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x871F2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x871F8; return 10; }
    8
}
/// $0871F2: jmp $518.l
pub fn i_0871f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x871F8;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0871F8: rts
pub fn i_0871f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x871FA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Breakable_B_0871fa  ($0871FA, 134 B, enemies/bosses/fort_scene4_0865be.s)
/// $0871FA: movea.l $3c(a6), a1
pub fn i_0871fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x871FE;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0871FE: jsr $2942a.l
pub fn i_0871fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87204;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87204);
    cpu.pc = 0x2942A;
    16
}
/// $087204: move.w #$20, $70(a6)
pub fn i_087204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8720A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08720A: lea.l $2ec726.l, a0
pub fn i_08720a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87210;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC726;
    20
}
/// $087210: move.l a0, $48(a6)
pub fn i_087210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87214;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $087214: lea.l $8721a(pc), a1
pub fn i_087214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87218;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8721A;
    16
}
/// $087218: move.l a1, (a6)
pub fn i_087218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8721A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08721A: jsr $2783a.l
pub fn i_08721a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87220;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87220);
    cpu.pc = 0x2783A;
    16
}
/// $087220: jsr $2870a.l
pub fn i_087220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87226;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87226);
    cpu.pc = 0x2870A;
    16
}
/// $087226: bcc.w $8723c
pub fn i_087226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8722A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8723C; return 10; }
    8
}
/// $08722A: lea.l $5e766.l, a0
pub fn i_08722a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87230;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $087230: jsr $5e770.l
pub fn i_087230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87236;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87236);
    cpu.pc = 0x5E770;
    16
}
/// $087236: bclr.b #$3, $13(a6)
pub fn i_087236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8723C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08723C: jsr $28758.l
pub fn i_08723c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87242;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87242);
    cpu.pc = 0x28758;
    16
}
/// $087242: bcc.w $8726e
pub fn i_087242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87246;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8726E; return 10; }
    8
}
/// $087246: lea.l $2ed0ae.l, a1
pub fn i_087246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8724C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED0AE;
    20
}
/// $08724C: jsr $77c7e.l
pub fn i_08724c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87252;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87252);
    cpu.pc = 0x77C7E;
    16
}
/// $087252: lea.l $2ed968.l, a2
pub fn i_087252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87258;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED968;
    20
}
/// $087258: jsr $5022a.l
pub fn i_087258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8725E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8725E);
    cpu.pc = 0x5022A;
    16
}
/// $08725E: lea.l $2ed97c.l, a2
pub fn i_08725e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87264;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED97C;
    20
}
/// $087264: jsr $5022a.l
pub fn i_087264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8726A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8726A);
    cpu.pc = 0x5022A;
    16
}
/// $08726A: bra.w $87278
pub fn i_08726a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8726E;
    cpu.ir = 0x6000;
    cpu.pc = 0x87278;
    10
}
/// $08726E: jsr $4fa70.l
pub fn i_08726e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87274;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87274);
    cpu.pc = 0x4FA70;
    16
}
/// $087274: bcc.w $8727e
pub fn i_087274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87278;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8727E; return 10; }
    8
}
/// $087278: jmp $518.l
pub fn i_087278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8727E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08727E: rts
pub fn i_08727e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87280;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Breakable_C_087280  ($087280, 122 B, enemies/bosses/fort_scene4_0865be.s)
/// $087280: movea.l $3c(a6), a1
pub fn i_087280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87284;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $087284: jsr $2942a.l
pub fn i_087284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8728A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8728A);
    cpu.pc = 0x2942A;
    16
}
/// $08728A: move.w #$20, $70(a6)
pub fn i_08728a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87290;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087290: lea.l $2ec726.l, a0
pub fn i_087290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87296;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC726;
    20
}
/// $087296: move.l a0, $48(a6)
pub fn i_087296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8729A;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08729A: lea.l $872a0(pc), a1
pub fn i_08729a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8729E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x872A0;
    16
}
/// $08729E: move.l a1, (a6)
pub fn i_08729e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x872A0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0872A0: jsr $2783a.l
pub fn i_0872a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x872A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x872A6);
    cpu.pc = 0x2783A;
    16
}
/// $0872A6: jsr $2870a.l
pub fn i_0872a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x872AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x872AC);
    cpu.pc = 0x2870A;
    16
}
/// $0872AC: bcc.w $872c2
pub fn i_0872ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x872B0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x872C2; return 10; }
    8
}
/// $0872B0: lea.l $5e766.l, a0
pub fn i_0872b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x872B6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $0872B6: jsr $5e770.l
pub fn i_0872b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x872BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x872BC);
    cpu.pc = 0x5E770;
    16
}
/// $0872BC: bclr.b #$3, $13(a6)
pub fn i_0872bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x872C2;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0872C2: jsr $28758.l
pub fn i_0872c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x872C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x872C8);
    cpu.pc = 0x28758;
    16
}
/// $0872C8: bcc.w $872e8
pub fn i_0872c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x872CC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x872E8; return 10; }
    8
}
/// $0872CC: lea.l $2ed0ae.l, a1
pub fn i_0872cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x872D2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED0AE;
    20
}
/// $0872D2: jsr $77c7e.l
pub fn i_0872d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x872D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x872D8);
    cpu.pc = 0x77C7E;
    16
}
/// $0872D8: lea.l $2eda6c.l, a2
pub fn i_0872d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x872DE;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EDA6C;
    20
}
/// $0872DE: jsr $5022a.l
pub fn i_0872de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x872E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x872E4);
    cpu.pc = 0x5022A;
    16
}
/// $0872E4: bra.w $872f2
pub fn i_0872e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x872E8;
    cpu.ir = 0x6000;
    cpu.pc = 0x872F2;
    10
}
/// $0872E8: jsr $4fa70.l
pub fn i_0872e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x872EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x872EE);
    cpu.pc = 0x4FA70;
    16
}
/// $0872EE: bcc.w $872f8
pub fn i_0872ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x872F2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x872F8; return 10; }
    8
}
/// $0872F2: jmp $518.l
pub fn i_0872f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x872F8;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0872F8: rts
pub fn i_0872f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x872FA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Signboard_0872fa  ($0872FA, 108 B, enemies/bosses/fort_scene4_0865be.s)
/// $0872FA: movea.l $3c(a6), a1
pub fn i_0872fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x872FE;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0872FE: jsr $2942a.l
pub fn i_0872fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87304;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87304);
    cpu.pc = 0x2942A;
    16
}
/// $087304: move.w #$20, $70(a6)
pub fn i_087304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8730A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08730A: lea.l $2ebfe6.l, a0
pub fn i_08730a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87310;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EBFE6;
    20
}
/// $087310: jsr $28cd4.l
pub fn i_087310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87316;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87316);
    cpu.pc = 0x28CD4;
    16
}
/// $087316: lea.l $8731c(pc), a1
pub fn i_087316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8731A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8731C;
    16
}
/// $08731A: move.l a1, (a6)
pub fn i_08731a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8731C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08731C: jsr $2783a.l
pub fn i_08731c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87322;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87322);
    cpu.pc = 0x2783A;
    16
}
/// $087322: jsr $28d70.l
pub fn i_087322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87328;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87328);
    cpu.pc = 0x28D70;
    16
}
/// $087328: jsr $2870a.l
pub fn i_087328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8732E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8732E);
    cpu.pc = 0x2870A;
    16
}
/// $08732E: bcc.w $8734e
pub fn i_08732e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87332;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8734E; return 10; }
    8
}
/// $087332: move.w #$10a9, d0
pub fn i_087332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87336;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10A9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $087336: jsr $2352.l
pub fn i_087336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8733C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8733C);
    cpu.pc = 0x2352;
    16
}
/// $08733C: lea.l $2ebffc.l, a0
pub fn i_08733c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87342;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EBFFC;
    20
}
/// $087342: jsr $28cd4.l
pub fn i_087342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87348;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87348);
    cpu.pc = 0x28CD4;
    16
}
/// $087348: bclr.b #$3, $13(a6)
pub fn i_087348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8734E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08734E: move.w #$7fff, $66(a6)
pub fn i_08734e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87354;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7FFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087354: jsr $4fa70.l
pub fn i_087354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8735A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8735A);
    cpu.pc = 0x4FA70;
    16
}
/// $08735A: bcc.w $87364
pub fn i_08735a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8735E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x87364; return 10; }
    8
}
/// $08735E: jmp $518.l
pub fn i_08735e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87364;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $087364: rts
pub fn i_087364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87366;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== CrateWall_Spawn13_087366  ($087366, 338 B, enemies/bosses/fort_scene4_0865be.s)
/// $087366: movea.l $3c(a6), a1
pub fn i_087366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8736A;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08736A: jsr $2942a.l
pub fn i_08736a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87370;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87370);
    cpu.pc = 0x2942A;
    16
}
/// $087370: lea.l $874ba(pc), a1
pub fn i_087370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87374;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x874BA;
    16
}
/// $087374: jsr $4ae.l
pub fn i_087374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8737A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8737A);
    cpu.pc = 0x4AE;
    16
}
/// $08737A: jsr $5dd22.l
pub fn i_08737a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87380;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87380);
    cpu.pc = 0x5DD22;
    16
}
/// $087380: subi.w #$10, $22(a0)
pub fn i_087380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87386;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $087386: addi.w #$40, $24(a0)
pub fn i_087386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8738C;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08738C: lea.l $874ea(pc), a1
pub fn i_08738c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87390;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x874EA;
    16
}
/// $087390: jsr $4ae.l
pub fn i_087390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87396;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87396);
    cpu.pc = 0x4AE;
    16
}
/// $087396: jsr $5dd22.l
pub fn i_087396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8739C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8739C);
    cpu.pc = 0x5DD22;
    16
}
/// $08739C: addi.w #$10, $22(a0)
pub fn i_08739c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x873A2;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0873A2: addi.w #$a0, $24(a0)
pub fn i_0873a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x873A8;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xA0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0873A8: lea.l $8751c(pc), a1
pub fn i_0873a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x873AC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8751C;
    16
}
/// $0873AC: jsr $4ae.l
pub fn i_0873ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x873B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x873B2);
    cpu.pc = 0x4AE;
    16
}
/// $0873B2: jsr $5dd22.l
pub fn i_0873b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x873B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x873B8);
    cpu.pc = 0x5DD22;
    16
}
/// $0873B8: addi.w #$50, $22(a0)
pub fn i_0873b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x873BE;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x50, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0873BE: addi.w #$a0, $24(a0)
pub fn i_0873be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x873C4;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xA0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0873C4: lea.l $8754e(pc), a1
pub fn i_0873c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x873C8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8754E;
    16
}
/// $0873C8: jsr $4ae.l
pub fn i_0873c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x873CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x873CE);
    cpu.pc = 0x4AE;
    16
}
/// $0873CE: jsr $5dd22.l
pub fn i_0873ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x873D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x873D4);
    cpu.pc = 0x5DD22;
    16
}
/// $0873D4: addi.w #$80, $22(a0)
pub fn i_0873d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x873DA;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x80, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0873DA: addi.w #$a0, $24(a0)
pub fn i_0873da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x873E0;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xA0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0873E0: lea.l $87580(pc), a1
pub fn i_0873e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x873E4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87580;
    16
}
/// $0873E4: jsr $4ae.l
pub fn i_0873e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x873EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x873EA);
    cpu.pc = 0x4AE;
    16
}
/// $0873EA: jsr $5dd22.l
pub fn i_0873ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x873F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x873F0);
    cpu.pc = 0x5DD22;
    16
}
/// $0873F0: addi.w #$10, $22(a0)
pub fn i_0873f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x873F6;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0873F6: addi.w #$80, $24(a0)
pub fn i_0873f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x873FC;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x80, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0873FC: lea.l $875b0(pc), a1
pub fn i_0873fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87400;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x875B0;
    16
}
/// $087400: jsr $4ae.l
pub fn i_087400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87406;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87406);
    cpu.pc = 0x4AE;
    16
}
/// $087406: jsr $5dd22.l
pub fn i_087406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8740C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8740C);
    cpu.pc = 0x5DD22;
    16
}
/// $08740C: addi.w #$60, $22(a0)
pub fn i_08740c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87412;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x60, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $087412: addi.w #$80, $24(a0)
pub fn i_087412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87418;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x80, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $087418: lea.l $875e0(pc), a1
pub fn i_087418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8741C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x875E0;
    16
}
/// $08741C: jsr $4ae.l
pub fn i_08741c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87422;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87422);
    cpu.pc = 0x4AE;
    16
}
/// $087422: jsr $5dd22.l
pub fn i_087422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87428;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87428);
    cpu.pc = 0x5DD22;
    16
}
/// $087428: addi.w #$50, $22(a0)
pub fn i_087428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8742E;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x50, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08742E: addi.w #$50, $24(a0)
pub fn i_08742e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87434;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x50, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $087434: lea.l $87610(pc), a1
pub fn i_087434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87438;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87610;
    16
}
/// $087438: jsr $4ae.l
pub fn i_087438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8743E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8743E);
    cpu.pc = 0x4AE;
    16
}
/// $08743E: jsr $5dd22.l
pub fn i_08743e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87444;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87444);
    cpu.pc = 0x5DD22;
    16
}
/// $087444: lea.l $87640(pc), a1
pub fn i_087444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87448;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87640;
    16
}
/// $087448: jsr $4ae.l
pub fn i_087448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8744E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8744E);
    cpu.pc = 0x4AE;
    16
}
/// $08744E: jsr $5dd22.l
pub fn i_08744e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87454;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87454);
    cpu.pc = 0x5DD22;
    16
}
/// $087454: addi.w #$20, $22(a0)
pub fn i_087454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8745A;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08745A: lea.l $87670(pc), a1
pub fn i_08745a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8745E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87670;
    16
}
/// $08745E: jsr $4ae.l
pub fn i_08745e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87464;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87464);
    cpu.pc = 0x4AE;
    16
}
/// $087464: jsr $5dd22.l
pub fn i_087464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8746A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8746A);
    cpu.pc = 0x5DD22;
    16
}
/// $08746A: addi.w #$40, $22(a0)
pub fn i_08746a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87470;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $087470: lea.l $876a0(pc), a1
pub fn i_087470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87474;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x876A0;
    16
}
/// $087474: jsr $4ae.l
pub fn i_087474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8747A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8747A);
    cpu.pc = 0x4AE;
    16
}
/// $08747A: jsr $5dd22.l
pub fn i_08747a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87480;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87480);
    cpu.pc = 0x5DD22;
    16
}
/// $087480: addi.w #$60, $22(a0)
pub fn i_087480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87486;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x60, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $087486: lea.l $876d0(pc), a1
pub fn i_087486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8748A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x876D0;
    16
}
/// $08748A: jsr $4ae.l
pub fn i_08748a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87490;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87490);
    cpu.pc = 0x4AE;
    16
}
/// $087490: jsr $5dd22.l
pub fn i_087490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87496;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87496);
    cpu.pc = 0x5DD22;
    16
}
/// $087496: addi.w #$80, $22(a0)
pub fn i_087496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8749C;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x80, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08749C: lea.l $87700(pc), a1
pub fn i_08749c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x874A0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87700;
    16
}
/// $0874A0: jsr $4ae.l
pub fn i_0874a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x874A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x874A6);
    cpu.pc = 0x4AE;
    16
}
/// $0874A6: jsr $5dd22.l
pub fn i_0874a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x874AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x874AC);
    cpu.pc = 0x5DD22;
    16
}
/// $0874AC: addi.w #$a0, $22(a0)
pub fn i_0874ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x874B2;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xA0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0874B2: jmp $518.l
pub fn i_0874b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x874B8;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Rts_0874b8  ($0874B8, 2 B, enemies/bosses/fort_scene4_0865be.s)
/// $0874B8: rts
pub fn i_0874b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x874BA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== CrateWall_Piece00_0874ba  ($0874BA, 48 B, enemies/bosses/fort_scene4_0865be.s)
/// $0874BA: lea.l $2ed990.l, a1
pub fn i_0874ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x874C0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED990;
    20
}
/// $0874C0: move.l a1, $74(a6)
pub fn i_0874c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x874C4;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0874C4: move.l #$ffffffff, $7c(a6)
pub fn i_0874c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x874CC;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0874CC: move.w #$0, $72(a6)
pub fn i_0874cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x874D2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0874D2: lea.l $2ec822.l, a0
pub fn i_0874d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x874D8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC822;
    20
}
/// $0874D8: move.l a0, $48(a6)
pub fn i_0874d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x874DC;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0874DC: lea.l $2ecfde.l, a1
pub fn i_0874dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x874E2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ECFDE;
    20
}
/// $0874E2: move.l a1, $88(a6)
pub fn i_0874e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x874E6;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0874E6: bra.w $87730
pub fn i_0874e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x874EA;
    cpu.ir = 0x6000;
    cpu.pc = 0x87730;
    10
}
// ===== CrateWall_Piece01_0874ea  ($0874EA, 50 B, enemies/bosses/fort_scene4_0865be.s)
/// $0874EA: lea.l $2ed9a4.l, a1
pub fn i_0874ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x874F0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED9A4;
    20
}
/// $0874F0: move.l a1, $74(a6)
pub fn i_0874f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x874F4;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0874F4: lea.l $edfe2.l, a1
pub fn i_0874f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x874FA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEDFE2;
    20
}
/// $0874FA: move.l a1, $7c(a6)
pub fn i_0874fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x874FE;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0874FE: move.w #$1, $72(a6)
pub fn i_0874fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87504;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087504: lea.l $2ec876.l, a0
pub fn i_087504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8750A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC876;
    20
}
/// $08750A: move.l a0, $48(a6)
pub fn i_08750a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8750E;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08750E: lea.l $2ecff0.l, a1
pub fn i_08750e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87514;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ECFF0;
    20
}
/// $087514: move.l a1, $88(a6)
pub fn i_087514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87518;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $087518: bra.w $87730
pub fn i_087518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8751C;
    cpu.ir = 0x6000;
    cpu.pc = 0x87730;
    10
}
// ===== CrateWall_Piece02_08751c  ($08751C, 50 B, enemies/bosses/fort_scene4_0865be.s)
/// $08751C: lea.l $2ed9b8.l, a1
pub fn i_08751c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87522;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED9B8;
    20
}
/// $087522: move.l a1, $74(a6)
pub fn i_087522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87526;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $087526: lea.l $ee002.l, a1
pub fn i_087526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8752C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEE002;
    20
}
/// $08752C: move.l a1, $7c(a6)
pub fn i_08752c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87530;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $087530: move.w #$2, $72(a6)
pub fn i_087530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87536;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087536: lea.l $2ec822.l, a0
pub fn i_087536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8753C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC822;
    20
}
/// $08753C: move.l a0, $48(a6)
pub fn i_08753c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87540;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $087540: lea.l $2ecfde.l, a1
pub fn i_087540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87546;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ECFDE;
    20
}
/// $087546: move.l a1, $88(a6)
pub fn i_087546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8754A;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08754A: bra.w $87730
pub fn i_08754a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8754E;
    cpu.ir = 0x6000;
    cpu.pc = 0x87730;
    10
}
// ===== CrateWall_Piece03_08754e  ($08754E, 50 B, enemies/bosses/fort_scene4_0865be.s)
/// $08754E: lea.l $2ed9cc.l, a1
pub fn i_08754e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87554;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED9CC;
    20
}
/// $087554: move.l a1, $74(a6)
pub fn i_087554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87558;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $087558: lea.l $ee022.l, a1
pub fn i_087558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8755E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEE022;
    20
}
/// $08755E: move.l a1, $7c(a6)
pub fn i_08755e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87562;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $087562: move.w #$3, $72(a6)
pub fn i_087562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87568;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087568: lea.l $2ec822.l, a0
pub fn i_087568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8756E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC822;
    20
}
/// $08756E: move.l a0, $48(a6)
pub fn i_08756e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87572;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $087572: lea.l $2ecfde.l, a1
pub fn i_087572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87578;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ECFDE;
    20
}
/// $087578: move.l a1, $88(a6)
pub fn i_087578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8757C;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08757C: bra.w $87730
pub fn i_08757c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87580;
    cpu.ir = 0x6000;
    cpu.pc = 0x87730;
    10
}
// ===== CrateWall_Piece04_087580  ($087580, 48 B, enemies/bosses/fort_scene4_0865be.s)
/// $087580: lea.l $2eda80.l, a1
pub fn i_087580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87586;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EDA80;
    20
}
/// $087586: move.l a1, $74(a6)
pub fn i_087586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8758A;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08758A: move.l #$ffffffff, $7c(a6)
pub fn i_08758a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87592;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $087592: move.w #$4, $72(a6)
pub fn i_087592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87598;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087598: lea.l $2ec8ca.l, a0
pub fn i_087598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8759E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC8CA;
    20
}
/// $08759E: move.l a0, $48(a6)
pub fn i_08759e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x875A2;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0875A2: lea.l $2ed002.l, a1
pub fn i_0875a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x875A8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED002;
    20
}
/// $0875A8: move.l a1, $88(a6)
pub fn i_0875a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x875AC;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0875AC: bra.w $87730
pub fn i_0875ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x875B0;
    cpu.ir = 0x6000;
    cpu.pc = 0x87730;
    10
}
// ===== CrateWall_Piece05_0875b0  ($0875B0, 48 B, enemies/bosses/fort_scene4_0865be.s)
/// $0875B0: lea.l $2eda94.l, a1
pub fn i_0875b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x875B6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EDA94;
    20
}
/// $0875B6: move.l a1, $74(a6)
pub fn i_0875b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x875BA;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0875BA: move.l #$ffffffff, $7c(a6)
pub fn i_0875ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x875C2;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0875C2: move.w #$5, $72(a6)
pub fn i_0875c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x875C8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0875C8: lea.l $2ec8ca.l, a0
pub fn i_0875c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x875CE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC8CA;
    20
}
/// $0875CE: move.l a0, $48(a6)
pub fn i_0875ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x875D2;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0875D2: lea.l $2ed002.l, a1
pub fn i_0875d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x875D8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED002;
    20
}
/// $0875D8: move.l a1, $88(a6)
pub fn i_0875d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x875DC;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0875DC: bra.w $87730
pub fn i_0875dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x875E0;
    cpu.ir = 0x6000;
    cpu.pc = 0x87730;
    10
}
// ===== CrateWall_Piece06_0875e0  ($0875E0, 48 B, enemies/bosses/fort_scene4_0865be.s)
/// $0875E0: lea.l $2ed9e0.l, a1
pub fn i_0875e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x875E6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED9E0;
    20
}
/// $0875E6: move.l a1, $74(a6)
pub fn i_0875e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x875EA;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0875EA: move.l #$ffffffff, $7c(a6)
pub fn i_0875ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x875F2;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0875F2: move.w #$6, $72(a6)
pub fn i_0875f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x875F8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0875F8: lea.l $2ec91e.l, a0
pub fn i_0875f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x875FE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC91E;
    20
}
/// $0875FE: move.l a0, $48(a6)
pub fn i_0875fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87602;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $087602: lea.l $2ed014.l, a1
pub fn i_087602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87608;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED014;
    20
}
/// $087608: move.l a1, $88(a6)
pub fn i_087608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8760C;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08760C: bra.w $87730
pub fn i_08760c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87610;
    cpu.ir = 0x6000;
    cpu.pc = 0x87730;
    10
}
// ===== CrateWall_Piece07_087610  ($087610, 48 B, enemies/bosses/fort_scene4_0865be.s)
/// $087610: lea.l $2ed9f4.l, a1
pub fn i_087610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87616;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED9F4;
    20
}
/// $087616: move.l a1, $74(a6)
pub fn i_087616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8761A;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08761A: move.l #$ffffffff, $7c(a6)
pub fn i_08761a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87622;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $087622: move.w #$7, $72(a6)
pub fn i_087622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87628;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087628: lea.l $2ec972.l, a0
pub fn i_087628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8762E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC972;
    20
}
/// $08762E: move.l a0, $48(a6)
pub fn i_08762e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87632;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $087632: lea.l $2ed026.l, a1
pub fn i_087632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87638;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED026;
    20
}
/// $087638: move.l a1, $88(a6)
pub fn i_087638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8763C;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08763C: bra.w $87730
pub fn i_08763c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87640;
    cpu.ir = 0x6000;
    cpu.pc = 0x87730;
    10
}
// ===== CrateWall_Piece08_087640  ($087640, 48 B, enemies/bosses/fort_scene4_0865be.s)
/// $087640: lea.l $2eda08.l, a1
pub fn i_087640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87646;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EDA08;
    20
}
/// $087646: move.l a1, $74(a6)
pub fn i_087646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8764A;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08764A: move.l #$ffffffff, $7c(a6)
pub fn i_08764a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87652;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $087652: move.w #$8, $72(a6)
pub fn i_087652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87658;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087658: lea.l $2ec9c6.l, a0
pub fn i_087658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8765E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC9C6;
    20
}
/// $08765E: move.l a0, $48(a6)
pub fn i_08765e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87662;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $087662: lea.l $2ed038.l, a1
pub fn i_087662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87668;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED038;
    20
}
/// $087668: move.l a1, $88(a6)
pub fn i_087668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8766C;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08766C: bra.w $87730
pub fn i_08766c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87670;
    cpu.ir = 0x6000;
    cpu.pc = 0x87730;
    10
}
// ===== CrateWall_Piece09_087670  ($087670, 48 B, enemies/bosses/fort_scene4_0865be.s)
/// $087670: lea.l $2eda1c.l, a1
pub fn i_087670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87676;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EDA1C;
    20
}
/// $087676: move.l a1, $74(a6)
pub fn i_087676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8767A;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08767A: move.l #$ffffffff, $7c(a6)
pub fn i_08767a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87682;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $087682: move.w #$9, $72(a6)
pub fn i_087682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87688;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087688: lea.l $2ec9c6.l, a0
pub fn i_087688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8768E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC9C6;
    20
}
/// $08768E: move.l a0, $48(a6)
pub fn i_08768e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87692;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $087692: lea.l $2ed038.l, a1
pub fn i_087692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87698;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED038;
    20
}
/// $087698: move.l a1, $88(a6)
pub fn i_087698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8769C;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08769C: bra.w $87730
pub fn i_08769c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x876A0;
    cpu.ir = 0x6000;
    cpu.pc = 0x87730;
    10
}
// ===== CrateWall_Piece10_0876a0  ($0876A0, 48 B, enemies/bosses/fort_scene4_0865be.s)
/// $0876A0: lea.l $2eda30.l, a1
pub fn i_0876a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x876A6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EDA30;
    20
}
/// $0876A6: move.l a1, $74(a6)
pub fn i_0876a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x876AA;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0876AA: move.l #$ffffffff, $7c(a6)
pub fn i_0876aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x876B2;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0876B2: move.w #$a, $72(a6)
pub fn i_0876b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x876B8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0876B8: lea.l $2ec9c6.l, a0
pub fn i_0876b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x876BE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC9C6;
    20
}
/// $0876BE: move.l a0, $48(a6)
pub fn i_0876be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x876C2;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0876C2: lea.l $2ed038.l, a1
pub fn i_0876c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x876C8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED038;
    20
}
/// $0876C8: move.l a1, $88(a6)
pub fn i_0876c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x876CC;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0876CC: bra.w $87730
pub fn i_0876cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x876D0;
    cpu.ir = 0x6000;
    cpu.pc = 0x87730;
    10
}
// ===== CrateWall_Piece11_0876d0  ($0876D0, 48 B, enemies/bosses/fort_scene4_0865be.s)
/// $0876D0: lea.l $2eda44.l, a1
pub fn i_0876d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x876D6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EDA44;
    20
}
/// $0876D6: move.l a1, $74(a6)
pub fn i_0876d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x876DA;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0876DA: move.l #$ffffffff, $7c(a6)
pub fn i_0876da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x876E2;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0876E2: move.w #$b, $72(a6)
pub fn i_0876e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x876E8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0876E8: lea.l $2ec9c6.l, a0
pub fn i_0876e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x876EE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC9C6;
    20
}
/// $0876EE: move.l a0, $48(a6)
pub fn i_0876ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x876F2;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0876F2: lea.l $2ed038.l, a1
pub fn i_0876f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x876F8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED038;
    20
}
/// $0876F8: move.l a1, $88(a6)
pub fn i_0876f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x876FC;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0876FC: bra.w $87730
pub fn i_0876fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87700;
    cpu.ir = 0x6000;
    cpu.pc = 0x87730;
    10
}
// ===== CrateWall_Piece12_087700  ($087700, 206 B, enemies/bosses/fort_scene4_0865be.s)
/// $087700: lea.l $2eda58.l, a1
pub fn i_087700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87706;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EDA58;
    20
}
/// $087706: move.l a1, $74(a6)
pub fn i_087706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8770A;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08770A: move.l #$ffffffff, $7c(a6)
pub fn i_08770a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87712;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $087712: move.w #$c, $72(a6)
pub fn i_087712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87718;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087718: lea.l $2ec9c6.l, a0
pub fn i_087718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8771E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC9C6;
    20
}
/// $08771E: move.l a0, $48(a6)
pub fn i_08771e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87722;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $087722: lea.l $2ed038.l, a1
pub fn i_087722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87728;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED038;
    20
}
/// $087728: move.l a1, $88(a6)
pub fn i_087728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8772C;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08772C: bra.w $87730
pub fn i_08772c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87730;
    cpu.ir = 0x6000;
    cpu.pc = 0x87730;
    10
}
/// $087730: move.w #$30, $70(a6)
pub fn i_087730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87736;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087736: move.w #$14, $66(a6)
pub fn i_087736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8773C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08773C: move.l $48(a6), $84(a6)
pub fn i_08773c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87742;
    cpu.ir = 0x2D6E;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x84);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $087742: lea.l $87748(pc), a1
pub fn i_087742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87746;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87748;
    16
}
/// $087746: move.l a1, (a6)
pub fn i_087746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87748;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087748: jsr $2783a.l
pub fn i_087748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8774E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8774E);
    cpu.pc = 0x2783A;
    16
}
/// $08774E: jsr $2870a.l
pub fn i_08774e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87754;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87754);
    cpu.pc = 0x2870A;
    16
}
/// $087754: bcc.w $8776a
pub fn i_087754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87758;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8776A; return 10; }
    8
}
/// $087758: lea.l $5e766.l, a0
pub fn i_087758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8775E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $08775E: jsr $5e770.l
pub fn i_08775e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87764;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87764);
    cpu.pc = 0x5E770;
    16
}
/// $087764: bclr.b #$3, $13(a6)
pub fn i_087764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8776A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08776A: jsr $28758.l
pub fn i_08776a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87770;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87770);
    cpu.pc = 0x28758;
    16
}
/// $087770: bcc.w $877be
pub fn i_087770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87774;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x877BE; return 10; }
    8
}
/// $087774: lea.l $ffff.w, a0
pub fn i_087774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87778;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $087778: move.l a0, $48(a6)
pub fn i_087778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8777C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08777C: move.w #$1027, d0
pub fn i_08777c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87780;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1027;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $087780: jsr $2352.l
pub fn i_087780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87786;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87786);
    cpu.pc = 0x2352;
    16
}
/// $087786: movea.l $74(a6), a2
pub fn i_087786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8778A;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $08778A: jsr $5022a.l
pub fn i_08778a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87790;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87790);
    cpu.pc = 0x5022A;
    16
}
/// $087790: movea.l $88(a6), a1
pub fn i_087790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87794;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $087794: jsr $77c7e.l
pub fn i_087794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8779A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8779A);
    cpu.pc = 0x77C7E;
    16
}
/// $08779A: cmpi.l #$ffffffff, $7c(a6)
pub fn i_08779a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x877A2;
    cpu.ir = 0xCAE;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read32(ad2);
    sub_with_flags(cpu, v3, 0xFFFFFFFF, Size::Long, false);
    20
}
/// $0877A2: beq.w $877c8
pub fn i_0877a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x877A6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x877C8; return 10; }
    8
}
/// $0877A6: movea.l $7c(a6), a1
pub fn i_0877a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x877AA;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0877AA: move.w #$82, d0
pub fn i_0877aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x877AE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x82;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0877AE: move.b #$0, $82(a6)
pub fn i_0877ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x877B4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0877B4: jsr $4429e.l
pub fn i_0877b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x877BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x877BA);
    cpu.pc = 0x4429E;
    16
}
/// $0877BA: bra.w $877c8
pub fn i_0877ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x877BE;
    cpu.ir = 0x6000;
    cpu.pc = 0x877C8;
    10
}
/// $0877BE: jsr $4fa70.l
pub fn i_0877be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x877C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x877C4);
    cpu.pc = 0x4FA70;
    16
}
/// $0877C4: bcc.w $877ce
pub fn i_0877c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x877C8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x877CE; return 10; }
    8
}
/// $0877C8: jmp $518.l
pub fn i_0877c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x877CE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== JsrPcThunk_0877ce  ($0877CE, 6 B, thunks/jsr_pc_thunks.c)
/// $0877CE: jsr $8846a(pc)
pub fn i_0877ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x877D2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x877D2);
    cpu.pc = 0x8846A;
    16
}
/// $0877D2: rts
pub fn i_0877d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x877D4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ArmoredCar_Init_0877d4  ($0877D4, 106 B, enemies/bosses/fort_scene4_0865be.s)
/// $0877D4: movea.l $3c(a6), a1
pub fn i_0877d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x877D8;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0877D8: jsr $2942a.l
pub fn i_0877d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x877DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x877DE);
    cpu.pc = 0x2942A;
    16
}
/// $0877DE: move.w #$e5, d1
pub fn i_0877de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x877E2;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0877E2: jsr $236e.l
pub fn i_0877e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x877E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x877E8);
    cpu.pc = 0x236E;
    16
}
/// $0877E8: move.b #$ff, $32(a6)
pub fn i_0877e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x877EE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0877EE: move.b #$ff, $33(a6)
pub fn i_0877ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x877F4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0877F4: move.b #$0, $3a(a6)
pub fn i_0877f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x877FA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0877FA: move.w #$f0, $70(a6)
pub fn i_0877fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87800;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087800: move.w #$0, $38(a6)
pub fn i_087800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87806;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087806: lea.l $2ec180.l, a0
pub fn i_087806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8780C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC180;
    20
}
/// $08780C: jsr $28cd4.l
pub fn i_08780c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87812;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87812);
    cpu.pc = 0x28CD4;
    16
}
/// $087812: lea.l $87aa0(pc), a1
pub fn i_087812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87816;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87AA0;
    16
}
/// $087816: jsr $4ae.l
pub fn i_087816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8781C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8781C);
    cpu.pc = 0x4AE;
    16
}
/// $08781C: jsr $5dd22.l
pub fn i_08781c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87822;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87822);
    cpu.pc = 0x5DD22;
    16
}
/// $087822: lea.l $87828(pc), a1
pub fn i_087822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87826;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87828;
    16
}
/// $087826: move.l a1, (a6)
pub fn i_087826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87828;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087828: cmpi.w #$100, $22(a6)
pub fn i_087828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8782E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x100, Size::Word, false);
    16
}
/// $08782E: blt.w $87846
pub fn i_08782e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87832;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x87846; return 10; }
    8
}
/// $087832: clr.b $10e39a.l
pub fn i_087832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87838;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $087838: jsr $2783a.l
pub fn i_087838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8783E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8783E);
    cpu.pc = 0x2783A;
    16
}
// ===== JsrAbsThunk_08783e  ($08783E, 8 B, thunks/jsr_abs_thunks.c)
/// $08783E: jsr $28d70.l
pub fn i_08783e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87844;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87844);
    cpu.pc = 0x28D70;
    16
}
/// $087844: rts
pub fn i_087844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87846;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ArmoredCar_Engage_087846  ($087846, 146 B, enemies/bosses/fort_scene4_0865be.s)
/// $087846: bclr.b #$3, $13(a6)
pub fn i_087846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8784C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08784C: bclr.b #$0, $13(a6)
pub fn i_08784c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87852;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $087852: lea.l $2c0628.l, a0
pub fn i_087852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87858;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C0628;
    20
}
/// $087858: jsr $799de.l
pub fn i_087858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8785E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8785E);
    cpu.pc = 0x799DE;
    16
}
/// $08785E: move.w d0, $66(a6)
pub fn i_08785e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87862;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087862: addi.w #$2e4, $66(a6)
pub fn i_087862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87868;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x66);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x2E4, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $087868: move.l #$2ede46, $60(a6)
pub fn i_087868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87870;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2EDE46;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $087870: lea.l $87876(pc), a1
pub fn i_087870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87874;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87876;
    16
}
/// $087874: move.l a1, (a6)
pub fn i_087874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87876;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087876: clr.b $10e39a.l
pub fn i_087876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8787C;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $08787C: jsr $2783a.l
pub fn i_08787c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87882;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87882);
    cpu.pc = 0x2783A;
    16
}
/// $087882: jsr $28998.l
pub fn i_087882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87888;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87888);
    cpu.pc = 0x28998;
    16
}
/// $087888: jsr $28d70.l
pub fn i_087888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8788E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8788E);
    cpu.pc = 0x28D70;
    16
}
/// $08788E: jsr $2870a.l
pub fn i_08788e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87894;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87894);
    cpu.pc = 0x2870A;
    16
}
/// $087894: bcc.w $878aa
pub fn i_087894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87898;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x878AA; return 10; }
    8
}
/// $087898: lea.l $5e766.l, a0
pub fn i_087898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8789E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $08789E: jsr $5e770.l
pub fn i_08789e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x878A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x878A4);
    cpu.pc = 0x5E770;
    16
}
/// $0878A4: bclr.b #$3, $13(a6)
pub fn i_0878a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x878AA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0878AA: cmpi.w #$29a, $66(a6)
pub fn i_0878aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x878B0;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x29A, Size::Word, false);
    16
}
/// $0878B0: bgt.w $878c6
pub fn i_0878b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x878B4;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x878C6; return 10; }
    8
}
/// $0878B4: lea.l $2ed0c0.l, a1
pub fn i_0878b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x878BA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED0C0;
    20
}
/// $0878BA: jsr $77c7e.l
pub fn i_0878ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x878C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x878C0);
    cpu.pc = 0x77C7E;
    16
}
/// $0878C0: lea.l $878d8(pc), a1
pub fn i_0878c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x878C4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x878D8;
    16
}
/// $0878C4: move.l a1, (a6)
pub fn i_0878c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x878C6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0878C6: jsr $4fa70.l
pub fn i_0878c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x878CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x878CC);
    cpu.pc = 0x4FA70;
    16
}
/// $0878CC: bcc.w $878d6
pub fn i_0878cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x878D0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x878D6; return 10; }
    8
}
/// $0878D0: jmp $518.l
pub fn i_0878d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x878D6;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0878D6: rts
pub fn i_0878d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x878D8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ArmoredCar_Damaged1_0878d8  ($0878D8, 126 B, enemies/bosses/fort_scene4_0865be.s)
/// $0878D8: move.w #$102b, d0
pub fn i_0878d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x878DC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0878DC: jsr $2352.l
pub fn i_0878dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x878E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x878E2);
    cpu.pc = 0x2352;
    16
}
/// $0878E2: lea.l $2ec19a.l, a0
pub fn i_0878e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x878E8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC19A;
    20
}
/// $0878E8: jsr $28cd4.l
pub fn i_0878e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x878EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x878EE);
    cpu.pc = 0x28CD4;
    16
}
/// $0878EE: lea.l $878f4(pc), a1
pub fn i_0878ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x878F2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x878F4;
    16
}
/// $0878F2: move.l a1, (a6)
pub fn i_0878f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x878F4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0878F4: clr.b $10e39a.l
pub fn i_0878f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x878FA;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $0878FA: jsr $2783a.l
pub fn i_0878fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87900;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87900);
    cpu.pc = 0x2783A;
    16
}
/// $087900: jsr $28998.l
pub fn i_087900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87906;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87906);
    cpu.pc = 0x28998;
    16
}
/// $087906: jsr $28d70.l
pub fn i_087906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8790C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8790C);
    cpu.pc = 0x28D70;
    16
}
/// $08790C: jsr $2870a.l
pub fn i_08790c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87912;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87912);
    cpu.pc = 0x2870A;
    16
}
/// $087912: bcc.w $87928
pub fn i_087912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87916;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x87928; return 10; }
    8
}
/// $087916: lea.l $5e766.l, a0
pub fn i_087916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8791C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $08791C: jsr $5e770.l
pub fn i_08791c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87922;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87922);
    cpu.pc = 0x5E770;
    16
}
/// $087922: bclr.b #$3, $13(a6)
pub fn i_087922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87928;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $087928: cmpi.w #$f6, $66(a6)
pub fn i_087928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8792E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xF6, Size::Word, false);
    16
}
/// $08792E: bgt.w $87944
pub fn i_08792e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87932;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x87944; return 10; }
    8
}
/// $087932: lea.l $2ed0d2.l, a1
pub fn i_087932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87938;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED0D2;
    20
}
/// $087938: jsr $77c7e.l
pub fn i_087938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8793E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8793E);
    cpu.pc = 0x77C7E;
    16
}
/// $08793E: lea.l $87956(pc), a1
pub fn i_08793e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87942;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87956;
    16
}
/// $087942: move.l a1, (a6)
pub fn i_087942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87944;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087944: jsr $4fa70.l
pub fn i_087944(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8794A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8794A);
    cpu.pc = 0x4FA70;
    16
}
/// $08794A: bcc.w $87954
pub fn i_08794a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8794E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x87954; return 10; }
    8
}
/// $08794E: jmp $518.l
pub fn i_08794e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87954;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $087954: rts
pub fn i_087954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87956;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ArmoredCar_Damaged2_087956  ($087956, 194 B, enemies/bosses/fort_scene4_0865be.s)
/// $087956: move.w #$102b, d0
pub fn i_087956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8795A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08795A: jsr $2352.l
pub fn i_08795a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87960;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87960);
    cpu.pc = 0x2352;
    16
}
/// $087960: lea.l $2ec1ae.l, a0
pub fn i_087960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87966;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC1AE;
    20
}
/// $087966: jsr $28cd4.l
pub fn i_087966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8796C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8796C);
    cpu.pc = 0x28CD4;
    16
}
/// $08796C: lea.l $87972(pc), a1
pub fn i_08796c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87970;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87972;
    16
}
/// $087970: move.l a1, (a6)
pub fn i_087970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87972;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087972: clr.b $10e39a.l
pub fn i_087972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87978;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $087978: jsr $2783a.l
pub fn i_087978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8797E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8797E);
    cpu.pc = 0x2783A;
    16
}
/// $08797E: jsr $28998.l
pub fn i_08797e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87984;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87984);
    cpu.pc = 0x28998;
    16
}
/// $087984: jsr $28d70.l
pub fn i_087984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8798A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8798A);
    cpu.pc = 0x28D70;
    16
}
/// $08798A: jsr $2870a.l
pub fn i_08798a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87990;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87990);
    cpu.pc = 0x2870A;
    16
}
/// $087990: bcc.w $879a6
pub fn i_087990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87994;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x879A6; return 10; }
    8
}
/// $087994: lea.l $5e766.l, a0
pub fn i_087994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8799A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $08799A: jsr $5e770.l
pub fn i_08799a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x879A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x879A0);
    cpu.pc = 0x5E770;
    16
}
/// $0879A0: bclr.b #$3, $13(a6)
pub fn i_0879a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x879A6;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0879A6: jsr $28758.l
pub fn i_0879a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x879AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x879AC);
    cpu.pc = 0x28758;
    16
}
/// $0879AC: bcc.w $87a06
pub fn i_0879ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x879B0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x87A06; return 10; }
    8
}
/// $0879B0: move.w #$1038, d0
pub fn i_0879b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x879B4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1038;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0879B4: jsr $2352.l
pub fn i_0879b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x879BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x879BA);
    cpu.pc = 0x2352;
    16
}
/// $0879BA: lea.l $2ed0e4.l, a1
pub fn i_0879ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x879C0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED0E4;
    20
}
/// $0879C0: jsr $77c7e.l
pub fn i_0879c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x879C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x879C6);
    cpu.pc = 0x77C7E;
    16
}
/// $0879C6: lea.l $2ecfcc.l, a1
pub fn i_0879c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x879CC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ECFCC;
    20
}
/// $0879CC: jsr $77c7e.l
pub fn i_0879cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x879D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x879D2);
    cpu.pc = 0x77C7E;
    16
}
/// $0879D2: lea.l $2edd56.l, a1
pub fn i_0879d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x879D8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EDD56;
    20
}
/// $0879D8: jsr $43fac.l
pub fn i_0879d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x879DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x879DE);
    cpu.pc = 0x43FAC;
    16
}
/// $0879DE: lea.l $2edd68.l, a1
pub fn i_0879de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x879E4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EDD68;
    20
}
/// $0879E4: jsr $43fac.l
pub fn i_0879e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x879EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x879EA);
    cpu.pc = 0x43FAC;
    16
}
/// $0879EA: jsr $434dc.l
pub fn i_0879ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x879F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x879F0);
    cpu.pc = 0x434DC;
    16
}
/// $0879F0: lea.l $87a62(pc), a1
pub fn i_0879f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x879F4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87A62;
    16
}
/// $0879F4: jsr $4ae.l
pub fn i_0879f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x879FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x879FA);
    cpu.pc = 0x4AE;
    16
}
/// $0879FA: jsr $5dd02.l
pub fn i_0879fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A00;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87A00);
    cpu.pc = 0x5DD02;
    16
}
/// $087A00: lea.l $87a18(pc), a1
pub fn i_087a00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A04;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87A18;
    16
}
/// $087A04: move.l a1, (a6)
pub fn i_087a04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A06;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087A06: jsr $4fa70.l
pub fn i_087a06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A0C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87A0C);
    cpu.pc = 0x4FA70;
    16
}
/// $087A0C: bcc.w $87a16
pub fn i_087a0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A10;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x87A16; return 10; }
    8
}
/// $087A10: jmp $518.l
pub fn i_087a10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A16;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $087A16: rts
pub fn i_087a16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A18;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ArmoredCar_Wreck_087a18  ($087A18, 74 B, enemies/bosses/fort_scene4_0865be.s)
/// $087A18: move.l #$5000, d0
pub fn i_087a18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A1E;
    cpu.ir = 0x203C;
    let v1: u32 = 0x5000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $087A1E: jsr $51a28.l
pub fn i_087a1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A24;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87A24);
    cpu.pc = 0x51A28;
    16
}
/// $087A24: lea.l $ffff.w, a0
pub fn i_087a24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A28;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $087A28: move.l a0, $48(a6)
pub fn i_087a28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A2C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $087A2C: lea.l $2ec1c2.l, a0
pub fn i_087a2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A32;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC1C2;
    20
}
/// $087A32: jsr $28cd4.l
pub fn i_087a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87A38);
    cpu.pc = 0x28CD4;
    16
}
/// $087A38: lea.l $87a3e(pc), a1
pub fn i_087a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A3C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87A3E;
    16
}
/// $087A3C: move.l a1, (a6)
pub fn i_087a3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A3E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087A3E: jsr $2783a.l
pub fn i_087a3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A44;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87A44);
    cpu.pc = 0x2783A;
    16
}
/// $087A44: jsr $28d70.l
pub fn i_087a44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87A4A);
    cpu.pc = 0x28D70;
    16
}
/// $087A4A: jsr $283d8.l
pub fn i_087a4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A50;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87A50);
    cpu.pc = 0x283D8;
    16
}
/// $087A50: jsr $4fa70.l
pub fn i_087a50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87A56);
    cpu.pc = 0x4FA70;
    16
}
/// $087A56: bcc.w $87a60
pub fn i_087a56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A5A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x87A60; return 10; }
    8
}
/// $087A5A: jmp $518.l
pub fn i_087a5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A60;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $087A60: rts
pub fn i_087a60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A62;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ArmoredCar_BlastBox_087a62  ($087A62, 62 B, enemies/bosses/fort_scene4_0865be.s)
/// $087A62: move.w #$10, $72(a6)
pub fn i_087a62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A68;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087A68: lea.l $87a6e(pc), a1
pub fn i_087a68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A6C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87A6E;
    16
}
/// $087A6C: move.l a1, (a6)
pub fn i_087a6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A6E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087A6E: jsr $2783a.l
pub fn i_087a6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A74;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87A74);
    cpu.pc = 0x2783A;
    16
}
/// $087A74: lea.l $2ecc34.l, a0
pub fn i_087a74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A7A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2ECC34;
    20
}
/// $087A7A: move.l a0, $4c(a6)
pub fn i_087a7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A7E;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $087A7E: jsr $283ca.l
pub fn i_087a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A84;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87A84);
    cpu.pc = 0x283CA;
    16
}
/// $087A84: jsr $283ca.l
pub fn i_087a84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87A8A);
    cpu.pc = 0x283CA;
    16
}
/// $087A8A: jsr $283d8.l
pub fn i_087a8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87A90);
    cpu.pc = 0x283D8;
    16
}
/// $087A90: subq.w #$1, $72(a6)
pub fn i_087a90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A94;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $087A94: bpl.w $87a9e
pub fn i_087a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A98;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x87A9E; return 10; }
    8
}
/// $087A98: jmp $518.l
pub fn i_087a98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87A9E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $087A9E: rts
pub fn i_087a9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87AA0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ArmoredCar_Turret_087aa0  ($087AA0, 124 B, enemies/bosses/fort_scene4_0865be.s)
/// $087AA0: move.w #$e5, d1
pub fn i_087aa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87AA4;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $087AA4: jsr $236e.l
pub fn i_087aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87AAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87AAA);
    cpu.pc = 0x236E;
    16
}
/// $087AAA: move.b #$ff, $32(a6)
pub fn i_087aaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87AB0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $087AB0: move.b #$ff, $33(a6)
pub fn i_087ab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87AB6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $087AB6: move.b #$0, $3a(a6)
pub fn i_087ab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87ABC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $087ABC: move.w #$8000, d0
pub fn i_087abc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87AC0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $087AC0: move.w #$80, d1
pub fn i_087ac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87AC4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $087AC4: jsr $2813c.l
pub fn i_087ac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87ACA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87ACA);
    cpu.pc = 0x2813C;
    16
}
/// $087ACA: lea.l $87ad0(pc), a1
pub fn i_087aca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87ACE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87AD0;
    16
}
/// $087ACE: move.l a1, (a6)
pub fn i_087ace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87AD0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087AD0: movea.l $c(a6), a0
pub fn i_087ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87AD4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $087AD4: move.w $22(a0), $22(a6)
pub fn i_087ad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87ADA;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $087ADA: move.w $24(a0), $24(a6)
pub fn i_087ada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87AE0;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $087AE0: clr.l d0
pub fn i_087ae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87AE2;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $087AE2: move.b $20(a0), d0
pub fn i_087ae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87AE6;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $087AE6: cmpi.b #$3, d0
pub fn i_087ae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87AEA;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $087AEA: beq.w $87b14
pub fn i_087aea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87AEE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x87B14; return 10; }
    8
}
/// $087AEE: movea.l #$2ec21a, a0
pub fn i_087aee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87AF4;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2EC21A;
    cpu.a[0] = v1;
    4
}
/// $087AF4: lsl.w #$2, d0
pub fn i_087af4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87AF6;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $087AF6: movea.l (a0, d0.w), a0
pub fn i_087af6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87AFA;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $087AFA: cmpa.l #$ffffffff, a0
pub fn i_087afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B00;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $087B00: beq.w $87b0a
pub fn i_087b00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B04;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x87B0A; return 10; }
    8
}
/// $087B04: jsr $28cd4.l
pub fn i_087b04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87B0A);
    cpu.pc = 0x28CD4;
    16
}
/// $087B0A: jsr $28d70.l
pub fn i_087b0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87B10);
    cpu.pc = 0x28D70;
    16
}
/// $087B10: bra.w $87b1a
pub fn i_087b10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B14;
    cpu.ir = 0x6000;
    cpu.pc = 0x87B1A;
    10
}
/// $087B14: jmp $518.l
pub fn i_087b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B1A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $087B1A: rts
pub fn i_087b1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B1C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Heli_Init_087b1c  ($087B1C, 10 B, enemies/bosses/fort_scene4_0865be.s)
/// $087B1C: movea.l $3c(a6), a1
pub fn i_087b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B20;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $087B20: jsr $2942a.l
pub fn i_087b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87B26);
    cpu.pc = 0x2942A;
    16
}
// ===== Heli_InitTmpl_087b26  ($087B26, 252 B, enemies/bosses/fort_scene4_0865be.s)
/// $087B26: jsr $2783a.l
pub fn i_087b26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B2C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87B2C);
    cpu.pc = 0x2783A;
    16
}
/// $087B2C: move.w #$86, d1
pub fn i_087b2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B30;
    cpu.ir = 0x323C;
    let v1: u32 = 0x86;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $087B30: jsr $236e.l
pub fn i_087b30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87B36);
    cpu.pc = 0x236E;
    16
}
/// $087B36: move.w #$87, d1
pub fn i_087b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B3A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x87;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $087B3A: jsr $236e.l
pub fn i_087b3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B40;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87B40);
    cpu.pc = 0x236E;
    16
}
/// $087B40: move.b #$ff, $32(a6)
pub fn i_087b40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B46;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $087B46: move.b #$ff, $33(a6)
pub fn i_087b46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B4C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $087B4C: move.b #$0, $3a(a6)
pub fn i_087b4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B52;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $087B52: move.w #$50, $70(a6)
pub fn i_087b52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B58;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x50;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087B58: lea.l $2c05a6.l, a0
pub fn i_087b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B5E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C05A6;
    20
}
/// $087B5E: jsr $799de.l
pub fn i_087b5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B64;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87B64);
    cpu.pc = 0x799DE;
    16
}
/// $087B64: move.w d0, $66(a6)
pub fn i_087b64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B68;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087B68: addi.w #$17c, $66(a6)
pub fn i_087b68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B6E;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x66);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x17C, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $087B6E: move.w #$0, $72(a6)
pub fn i_087b6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B74;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087B74: move.b #$0, $20(a6)
pub fn i_087b74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B7A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $087B7A: move.w #$2000, $38(a6)
pub fn i_087b7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B80;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087B80: lea.l $2ec226.l, a0
pub fn i_087b80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B86;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC226;
    20
}
/// $087B86: jsr $28cd4.l
pub fn i_087b86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B8C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87B8C);
    cpu.pc = 0x28CD4;
    16
}
/// $087B8C: lea.l $87eae(pc), a1
pub fn i_087b8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B90;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87EAE;
    16
}
/// $087B90: jsr $4ae.l
pub fn i_087b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87B96);
    cpu.pc = 0x4AE;
    16
}
/// $087B96: jsr $5dd22.l
pub fn i_087b96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87B9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87B9C);
    cpu.pc = 0x5DD22;
    16
}
/// $087B9C: lea.l $2edd7e.l, a1
pub fn i_087b9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87BA2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EDD7E;
    20
}
/// $087BA2: jsr $43fac.l
pub fn i_087ba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87BA8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87BA8);
    cpu.pc = 0x43FAC;
    16
}
/// $087BA8: move.l #$2eddfa, $60(a6)
pub fn i_087ba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87BB0;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2EDDFA;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $087BB0: lea.l $87bb6(pc), a1
pub fn i_087bb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87BB4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87BB6;
    16
}
/// $087BB4: move.l a1, (a6)
pub fn i_087bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87BB6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087BB6: jsr $28998.l
pub fn i_087bb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87BBC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87BBC);
    cpu.pc = 0x28998;
    16
}
/// $087BBC: jsr $2783a.l
pub fn i_087bbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87BC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87BC2);
    cpu.pc = 0x2783A;
    16
}
/// $087BC2: jsr $883ec(pc)
pub fn i_087bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87BC6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x87BC6);
    cpu.pc = 0x883EC;
    16
}
/// $087BC6: jsr $28d70.l
pub fn i_087bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87BCC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87BCC);
    cpu.pc = 0x28D70;
    16
}
/// $087BCC: jsr $2870a.l
pub fn i_087bcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87BD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87BD2);
    cpu.pc = 0x2870A;
    16
}
/// $087BD2: bcc.w $87bee
pub fn i_087bd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87BD6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x87BEE; return 10; }
    8
}
/// $087BD6: lea.l $5e766.l, a0
pub fn i_087bd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87BDC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $087BDC: jsr $5e770.l
pub fn i_087bdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87BE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87BE2);
    cpu.pc = 0x5E770;
    16
}
/// $087BE2: bclr.b #$3, $13(a6)
pub fn i_087be2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87BE8;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $087BE8: move.w #$f, $72(a6)
pub fn i_087be8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87BEE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087BEE: bclr.b #$0, $13(a6)
pub fn i_087bee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87BF4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $087BF4: move.w #$17c, $66(a6)
pub fn i_087bf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87BFA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x17C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087BFA: cmpi.w #$100, $22(a6)
pub fn i_087bfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C00;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x100, Size::Word, false);
    16
}
/// $087C00: bgt.w $87c0a
pub fn i_087c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C04;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x87C0A; return 10; }
    8
}
/// $087C04: lea.l $87c22(pc), a1
pub fn i_087c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C08;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87C22;
    16
}
/// $087C08: move.l a1, (a6)
pub fn i_087c08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C0A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087C0A: jsr $4fa70.l
pub fn i_087c0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87C10);
    cpu.pc = 0x4FA70;
    16
}
/// $087C10: bcc.w $87c20
pub fn i_087c10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C14;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x87C20; return 10; }
    8
}
/// $087C14: move.b #$ff, $20(a6)
pub fn i_087c14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C1A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $087C1A: jmp $518.l
pub fn i_087c1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C20;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $087C20: rts
pub fn i_087c20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C22;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Heli_Approach_087c22  ($087C22, 120 B, enemies/bosses/fort_scene4_0865be.s)
/// $087C22: lea.l $2ec23c.l, a0
pub fn i_087c22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C28;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC23C;
    20
}
/// $087C28: jsr $28cd4.l
pub fn i_087c28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C2E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87C2E);
    cpu.pc = 0x28CD4;
    16
}
/// $087C2E: lea.l $87c34(pc), a1
pub fn i_087c2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C32;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87C34;
    16
}
/// $087C32: move.l a1, (a6)
pub fn i_087c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C34;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087C34: jsr $28998.l
pub fn i_087c34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87C3A);
    cpu.pc = 0x28998;
    16
}
/// $087C3A: jsr $2783a.l
pub fn i_087c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C40;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87C40);
    cpu.pc = 0x2783A;
    16
}
/// $087C40: jsr $883ec(pc)
pub fn i_087c40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C44;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x87C44);
    cpu.pc = 0x883EC;
    16
}
/// $087C44: jsr $28d70.l
pub fn i_087c44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87C4A);
    cpu.pc = 0x28D70;
    16
}
/// $087C4A: bcc.w $87c54
pub fn i_087c4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C4E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x87C54; return 10; }
    8
}
/// $087C4E: lea.l $87c9a(pc), a1
pub fn i_087c4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C52;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87C9A;
    16
}
/// $087C52: move.l a1, (a6)
pub fn i_087c52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C54;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087C54: jsr $2870a.l
pub fn i_087c54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C5A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87C5A);
    cpu.pc = 0x2870A;
    16
}
/// $087C5A: bcc.w $87c76
pub fn i_087c5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C5E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x87C76; return 10; }
    8
}
/// $087C5E: lea.l $5e766.l, a0
pub fn i_087c5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C64;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $087C64: jsr $5e770.l
pub fn i_087c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87C6A);
    cpu.pc = 0x5E770;
    16
}
/// $087C6A: bclr.b #$3, $13(a6)
pub fn i_087c6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C70;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $087C70: move.w #$f, $72(a6)
pub fn i_087c70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C76;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087C76: bclr.b #$0, $13(a6)
pub fn i_087c76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C7C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $087C7C: move.w #$17c, $66(a6)
pub fn i_087c7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C82;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x17C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087C82: jsr $4fa70.l
pub fn i_087c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87C88);
    cpu.pc = 0x4FA70;
    16
}
/// $087C88: bcc.w $87c98
pub fn i_087c88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C8C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x87C98; return 10; }
    8
}
/// $087C8C: move.b #$ff, $20(a6)
pub fn i_087c8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C92;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $087C92: jmp $518.l
pub fn i_087c92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C98;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $087C98: rts
pub fn i_087c98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87C9A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Heli_Hover_087c9a  ($087C9A, 130 B, enemies/bosses/fort_scene4_0865be.s)
/// $087C9A: lea.l $ee042.l, a1
pub fn i_087c9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87CA0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEE042;
    20
}
/// $087CA0: move.w #$82, d0
pub fn i_087ca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87CA4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x82;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $087CA4: move.b #$0, $82(a6)
pub fn i_087ca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87CAA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $087CAA: jsr $4429e.l
pub fn i_087caa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87CB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87CB0);
    cpu.pc = 0x4429E;
    16
}
/// $087CB0: lea.l $87cb6(pc), a1
pub fn i_087cb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87CB4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87CB6;
    16
}
/// $087CB4: move.l a1, (a6)
pub fn i_087cb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87CB6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087CB6: jsr $28998.l
pub fn i_087cb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87CBC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87CBC);
    cpu.pc = 0x28998;
    16
}
/// $087CBC: jsr $2783a.l
pub fn i_087cbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87CC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87CC2);
    cpu.pc = 0x2783A;
    16
}
/// $087CC2: jsr $883ec(pc)
pub fn i_087cc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87CC6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x87CC6);
    cpu.pc = 0x883EC;
    16
}
/// $087CC6: jsr $28d70.l
pub fn i_087cc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87CCC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87CCC);
    cpu.pc = 0x28D70;
    16
}
/// $087CCC: jsr $2870a.l
pub fn i_087ccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87CD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87CD2);
    cpu.pc = 0x2870A;
    16
}
/// $087CD2: bcc.w $87cee
pub fn i_087cd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87CD6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x87CEE; return 10; }
    8
}
/// $087CD6: lea.l $5e766.l, a0
pub fn i_087cd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87CDC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $087CDC: jsr $5e770.l
pub fn i_087cdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87CE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87CE2);
    cpu.pc = 0x5E770;
    16
}
/// $087CE2: bclr.b #$3, $13(a6)
pub fn i_087ce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87CE8;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $087CE8: move.w #$f, $72(a6)
pub fn i_087ce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87CEE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087CEE: cmpi.w #$fc, $66(a6)
pub fn i_087cee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87CF4;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFC, Size::Word, false);
    16
}
/// $087CF4: bgt.w $87cfe
pub fn i_087cf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87CF8;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x87CFE; return 10; }
    8
}
/// $087CF8: lea.l $87d1c(pc), a1
pub fn i_087cf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87CFC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87D1C;
    16
}
/// $087CFC: move.l a1, (a6)
pub fn i_087cfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87CFE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087CFE: jsr $4fa70.l
pub fn i_087cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87D04);
    cpu.pc = 0x4FA70;
    16
}
/// $087D04: bcc.w $87d1a
pub fn i_087d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D08;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x87D1A; return 10; }
    8
}
/// $087D08: move.b #$ff, $82(a6)
pub fn i_087d08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D0E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $087D0E: move.b #$ff, $20(a6)
pub fn i_087d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D14;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $087D14: jmp $518.l
pub fn i_087d14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D1A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $087D1A: rts
pub fn i_087d1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D1C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Heli_Damaged1_087d1c  ($087D1C, 130 B, enemies/bosses/fort_scene4_0865be.s)
/// $087D1C: move.w #$1027, d0
pub fn i_087d1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D20;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1027;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $087D20: jsr $2352.l
pub fn i_087d20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87D26);
    cpu.pc = 0x2352;
    16
}
/// $087D26: lea.l $2ec2ce.l, a0
pub fn i_087d26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D2C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC2CE;
    20
}
/// $087D2C: jsr $28cd4.l
pub fn i_087d2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87D32);
    cpu.pc = 0x28CD4;
    16
}
/// $087D32: lea.l $87d38(pc), a1
pub fn i_087d32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D36;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87D38;
    16
}
/// $087D36: move.l a1, (a6)
pub fn i_087d36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D38;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087D38: jsr $28998.l
pub fn i_087d38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87D3E);
    cpu.pc = 0x28998;
    16
}
/// $087D3E: jsr $2783a.l
pub fn i_087d3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D44;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87D44);
    cpu.pc = 0x2783A;
    16
}
/// $087D44: jsr $883ec(pc)
pub fn i_087d44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D48;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x87D48);
    cpu.pc = 0x883EC;
    16
}
/// $087D48: jsr $28d70.l
pub fn i_087d48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87D4E);
    cpu.pc = 0x28D70;
    16
}
/// $087D4E: jsr $2870a.l
pub fn i_087d4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87D54);
    cpu.pc = 0x2870A;
    16
}
/// $087D54: bcc.w $87d70
pub fn i_087d54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D58;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x87D70; return 10; }
    8
}
/// $087D58: lea.l $5e766.l, a0
pub fn i_087d58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D5E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $087D5E: jsr $5e770.l
pub fn i_087d5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D64;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87D64);
    cpu.pc = 0x5E770;
    16
}
/// $087D64: bclr.b #$3, $13(a6)
pub fn i_087d64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D6A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $087D6A: move.w #$f, $72(a6)
pub fn i_087d6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D70;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087D70: cmpi.w #$7e, $66(a6)
pub fn i_087d70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D76;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x7E, Size::Word, false);
    16
}
/// $087D76: bgt.w $87d80
pub fn i_087d76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D7A;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x87D80; return 10; }
    8
}
/// $087D7A: lea.l $87d9e(pc), a1
pub fn i_087d7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D7E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87D9E;
    16
}
/// $087D7E: move.l a1, (a6)
pub fn i_087d7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D80;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087D80: jsr $4fa70.l
pub fn i_087d80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87D86);
    cpu.pc = 0x4FA70;
    16
}
/// $087D86: bcc.w $87d9c
pub fn i_087d86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D8A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x87D9C; return 10; }
    8
}
/// $087D8A: move.b #$ff, $82(a6)
pub fn i_087d8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D90;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $087D90: move.b #$ff, $20(a6)
pub fn i_087d90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D96;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $087D96: jmp $518.l
pub fn i_087d96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D9C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $087D9C: rts
pub fn i_087d9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87D9E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Heli_Damaged2_087d9e  ($087D9E, 130 B, enemies/bosses/fort_scene4_0865be.s)
/// $087D9E: move.w #$1027, d0
pub fn i_087d9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87DA2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1027;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $087DA2: jsr $2352.l
pub fn i_087da2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87DA8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87DA8);
    cpu.pc = 0x2352;
    16
}
/// $087DA8: lea.l $2ec2de.l, a0
pub fn i_087da8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87DAE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC2DE;
    20
}
/// $087DAE: jsr $28cd4.l
pub fn i_087dae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87DB4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87DB4);
    cpu.pc = 0x28CD4;
    16
}
/// $087DB4: lea.l $87dba(pc), a1
pub fn i_087db4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87DB8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87DBA;
    16
}
/// $087DB8: move.l a1, (a6)
pub fn i_087db8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87DBA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087DBA: jsr $28998.l
pub fn i_087dba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87DC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87DC0);
    cpu.pc = 0x28998;
    16
}
/// $087DC0: jsr $2783a.l
pub fn i_087dc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87DC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87DC6);
    cpu.pc = 0x2783A;
    16
}
/// $087DC6: jsr $883ec(pc)
pub fn i_087dc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87DCA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x87DCA);
    cpu.pc = 0x883EC;
    16
}
/// $087DCA: jsr $28d70.l
pub fn i_087dca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87DD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87DD0);
    cpu.pc = 0x28D70;
    16
}
/// $087DD0: jsr $2870a.l
pub fn i_087dd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87DD6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87DD6);
    cpu.pc = 0x2870A;
    16
}
/// $087DD6: bcc.w $87df2
pub fn i_087dd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87DDA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x87DF2; return 10; }
    8
}
/// $087DDA: lea.l $5e766.l, a0
pub fn i_087dda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87DE0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $087DE0: jsr $5e770.l
pub fn i_087de0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87DE6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87DE6);
    cpu.pc = 0x5E770;
    16
}
/// $087DE6: bclr.b #$3, $13(a6)
pub fn i_087de6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87DEC;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $087DEC: move.w #$f, $72(a6)
pub fn i_087dec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87DF2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087DF2: jsr $28758.l
pub fn i_087df2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87DF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87DF8);
    cpu.pc = 0x28758;
    16
}
/// $087DF8: bcc.w $87e02
pub fn i_087df8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87DFC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x87E02; return 10; }
    8
}
/// $087DFC: lea.l $87e20(pc), a1
pub fn i_087dfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E00;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87E20;
    16
}
/// $087E00: move.l a1, (a6)
pub fn i_087e00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E02;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087E02: jsr $4fa70.l
pub fn i_087e02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87E08);
    cpu.pc = 0x4FA70;
    16
}
/// $087E08: bcc.w $87e1e
pub fn i_087e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E0C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x87E1E; return 10; }
    8
}
/// $087E0C: move.b #$ff, $82(a6)
pub fn i_087e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E12;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $087E12: move.b #$ff, $20(a6)
pub fn i_087e12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E18;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $087E18: jmp $518.l
pub fn i_087e18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E1E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $087E1E: rts
pub fn i_087e1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E20;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Heli_Crash_087e20  ($087E20, 142 B, enemies/bosses/fort_scene4_0865be.s)
/// $087E20: move.l #$5000, d0
pub fn i_087e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E26;
    cpu.ir = 0x203C;
    let v1: u32 = 0x5000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $087E26: jsr $51a28.l
pub fn i_087e26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E2C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87E2C);
    cpu.pc = 0x51A28;
    16
}
/// $087E2C: move.w #$1023, d0
pub fn i_087e2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E30;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1023;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $087E30: jsr $2352.l
pub fn i_087e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87E36);
    cpu.pc = 0x2352;
    16
}
/// $087E36: jsr $2783a.l
pub fn i_087e36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87E3C);
    cpu.pc = 0x2783A;
    16
}
/// $087E3C: move.b #$ff, $82(a6)
pub fn i_087e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E42;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $087E42: move.b #$ff, $20(a6)
pub fn i_087e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E48;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $087E48: lea.l $77fd6.l, a1
pub fn i_087e48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E4E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $087E4E: jsr $4ae.l
pub fn i_087e4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87E54);
    cpu.pc = 0x4AE;
    16
}
/// $087E54: jsr $5dd22.l
pub fn i_087e54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E5A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87E5A);
    cpu.pc = 0x5DD22;
    16
}
/// $087E5A: lea.l $2edd92.l, a1
pub fn i_087e5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E60;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EDD92;
    20
}
/// $087E60: jsr $43fac.l
pub fn i_087e60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87E66);
    cpu.pc = 0x43FAC;
    16
}
/// $087E66: lea.l $2ed176.l, a1
pub fn i_087e66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E6C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED176;
    20
}
/// $087E6C: jsr $77c7e.l
pub fn i_087e6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87E72);
    cpu.pc = 0x77C7E;
    16
}
/// $087E72: lea.l $2ed188.l, a1
pub fn i_087e72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E78;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED188;
    20
}
/// $087E78: jsr $77c7e.l
pub fn i_087e78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87E7E);
    cpu.pc = 0x77C7E;
    16
}
/// $087E7E: lea.l $2ec2ee.l, a0
pub fn i_087e7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E84;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC2EE;
    20
}
/// $087E84: jsr $28cd4.l
pub fn i_087e84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87E8A);
    cpu.pc = 0x28CD4;
    16
}
/// $087E8A: lea.l $87e90(pc), a1
pub fn i_087e8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E8E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87E90;
    16
}
/// $087E8E: move.l a1, (a6)
pub fn i_087e8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E90;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087E90: jsr $2783a.l
pub fn i_087e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87E96);
    cpu.pc = 0x2783A;
    16
}
/// $087E96: jsr $28d70.l
pub fn i_087e96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87E9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87E9C);
    cpu.pc = 0x28D70;
    16
}
/// $087E9C: jsr $4fa70.l
pub fn i_087e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87EA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87EA2);
    cpu.pc = 0x4FA70;
    16
}
/// $087EA2: bcc.w $87eac
pub fn i_087ea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87EA6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x87EAC; return 10; }
    8
}
/// $087EA6: jmp $518.l
pub fn i_087ea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87EAC;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $087EAC: rts
pub fn i_087eac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87EAE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Heli_Dropper_087eae  ($087EAE, 158 B, enemies/bosses/fort_scene4_0865be.s)
/// $087EAE: move.w #$88, d1
pub fn i_087eae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87EB2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x88;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $087EB2: jsr $236e.l
pub fn i_087eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87EB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87EB8);
    cpu.pc = 0x236E;
    16
}
/// $087EB8: move.b #$ff, $32(a6)
pub fn i_087eb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87EBE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $087EBE: move.b #$ff, $33(a6)
pub fn i_087ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87EC4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $087EC4: move.b #$0, $3a(a6)
pub fn i_087ec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87ECA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $087ECA: move.w #$0, $38(a6)
pub fn i_087eca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87ED0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087ED0: lea.l $2ec2fe.l, a0
pub fn i_087ed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87ED6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC2FE;
    20
}
/// $087ED6: jsr $28cd4.l
pub fn i_087ed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87EDC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87EDC);
    cpu.pc = 0x28CD4;
    16
}
/// $087EDC: move.b #$0, $83(a6)
pub fn i_087edc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87EE2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x83);
    bus.write8(ad2, v1 as u8);
    12
}
/// $087EE2: lea.l $77228.l, a1
pub fn i_087ee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87EE8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77228;
    20
}
/// $087EE8: jsr $4ae.l
pub fn i_087ee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87EEE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87EEE);
    cpu.pc = 0x4AE;
    16
}
/// $087EEE: jsr $5dd22.l
pub fn i_087eee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87EF4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87EF4);
    cpu.pc = 0x5DD22;
    16
}
/// $087EF4: addi.w #$58, $22(a0)
pub fn i_087ef4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87EFA;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x58, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $087EFA: move.b #$30, $98(a0)
pub fn i_087efa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F00;
    cpu.ir = 0x117C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $087F00: move.b #$83, $99(a0)
pub fn i_087f00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F06;
    cpu.ir = 0x117C;
    let v1: u32 = 0x83;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $087F06: lea.l $87f0c(pc), a1
pub fn i_087f06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F0A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87F0C;
    16
}
/// $087F0A: move.l a1, (a6)
pub fn i_087f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F0C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087F0C: jsr $2783a.l
pub fn i_087f0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87F12);
    cpu.pc = 0x2783A;
    16
}
/// $087F12: jsr $28d70.l
pub fn i_087f12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87F18);
    cpu.pc = 0x28D70;
    16
}
/// $087F18: movea.l $c(a6), a0
pub fn i_087f18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F1C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $087F1C: cmpi.b #$ff, $20(a0)
pub fn i_087f1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F22;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $087F22: bne.w $87f4a
pub fn i_087f22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F26;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x87F4A; return 10; }
    8
}
/// $087F26: move.b #$ff, $83(a6)
pub fn i_087f26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F2C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x83);
    bus.write8(ad2, v1 as u8);
    12
}
/// $087F2C: lea.l $77f6a.l, a1
pub fn i_087f2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F32;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77F6A;
    20
}
/// $087F32: jsr $4ae.l
pub fn i_087f32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87F38);
    cpu.pc = 0x4AE;
    16
}
/// $087F38: jsr $5dd22.l
pub fn i_087f38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87F3E);
    cpu.pc = 0x5DD22;
    16
}
/// $087F3E: addi.w #$58, $22(a0)
pub fn i_087f3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F44;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x58, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $087F44: jmp $518.l
pub fn i_087f44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F4A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $087F4A: rts
pub fn i_087f4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F4C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Debris_Scatter_087f4c  ($087F4C, 112 B, enemies/bosses/fort_scene4_0865be.s)
/// $087F4C: move.w #$f3, d1
pub fn i_087f4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F50;
    cpu.ir = 0x323C;
    let v1: u32 = 0xF3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $087F50: jsr $236e.l
pub fn i_087f50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87F56);
    cpu.pc = 0x236E;
    16
}
/// $087F56: jsr $267e2.l
pub fn i_087f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87F5C);
    cpu.pc = 0x267E2;
    16
}
/// $087F5C: move.w #$7f, d0
pub fn i_087f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F60;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $087F60: jsr $5ea1c.l
pub fn i_087f60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87F66);
    cpu.pc = 0x5EA1C;
    16
}
/// $087F66: btst.b #$0, d0
pub fn i_087f66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F6A;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $087F6A: beq.w $87f70
pub fn i_087f6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F6E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x87F70; return 10; }
    8
}
/// $087F6E: neg.w d0
pub fn i_087f6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F70;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $087F70: move.w d0, $28(a6)
pub fn i_087f70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F74;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $087F74: andi.w #$7f, d0
pub fn i_087f74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F78;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $087F78: sub.w d0, $2a(a6)
pub fn i_087f78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F7C;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $087F7C: lea.l $2de4b0.l, a0
pub fn i_087f7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F82;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE4B0;
    20
}
/// $087F82: jsr $28cd4.l
pub fn i_087f82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87F88);
    cpu.pc = 0x28CD4;
    16
}
/// $087F88: lea.l $87f8e(pc), a1
pub fn i_087f88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F8C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x87F8E;
    16
}
/// $087F8C: move.l a1, (a6)
pub fn i_087f8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F8E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $087F8E: jsr $27cee.l
pub fn i_087f8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F94;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87F94);
    cpu.pc = 0x27CEE;
    16
}
/// $087F94: jsr $28d70.l
pub fn i_087f94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F9A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87F9A);
    cpu.pc = 0x28D70;
    16
}
/// $087F9A: bcs.w $87fb4
pub fn i_087f9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87F9E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x87FB4; return 10; }
    8
}
/// $087F9E: movea.l #$ffffffff, a0
pub fn i_087f9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87FA4;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $087FA4: lea.l $298736.l, a0
pub fn i_087fa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87FAA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x298736;
    20
}
/// $087FAA: jsr $5dd56.l
pub fn i_087faa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87FB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87FB0);
    cpu.pc = 0x5DD56;
    16
}
/// $087FB0: bcc.w $87fba
pub fn i_087fb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87FB4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x87FBA; return 10; }
    8
}
/// $087FB4: jmp $518.l
pub fn i_087fb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87FBA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $087FBA: rts
pub fn i_087fba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87FBC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Quad_Spawn_087fbc  ($087FBC, 80 B, enemies/bosses/fort_scene4_0865be.s)
/// $087FBC: movea.l $3c(a6), a1
pub fn i_087fbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87FC0;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $087FC0: jsr $2942a.l
pub fn i_087fc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87FC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87FC6);
    cpu.pc = 0x2942A;
    16
}
/// $087FC6: lea.l $8800e(pc), a1
pub fn i_087fc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87FCA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8800E;
    16
}
/// $087FCA: jsr $4ae.l
pub fn i_087fca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87FD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87FD0);
    cpu.pc = 0x4AE;
    16
}
/// $087FD0: jsr $5dd22.l
pub fn i_087fd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87FD6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87FD6);
    cpu.pc = 0x5DD22;
    16
}
/// $087FD6: lea.l $88028(pc), a1
pub fn i_087fd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87FDA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x88028;
    16
}
/// $087FDA: jsr $4ae.l
pub fn i_087fda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87FE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87FE0);
    cpu.pc = 0x4AE;
    16
}
/// $087FE0: jsr $5dd22.l
pub fn i_087fe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87FE6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87FE6);
    cpu.pc = 0x5DD22;
    16
}
/// $087FE6: lea.l $88042(pc), a1
pub fn i_087fe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87FEA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x88042;
    16
}
/// $087FEA: jsr $4ae.l
pub fn i_087fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87FF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87FF0);
    cpu.pc = 0x4AE;
    16
}
/// $087FF0: jsr $5dd22.l
pub fn i_087ff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87FF6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87FF6);
    cpu.pc = 0x5DD22;
    16
}
/// $087FF6: lea.l $8805c(pc), a1
pub fn i_087ff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87FFA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8805C;
    16
}
/// $087FFA: jsr $4ae.l
pub fn i_087ffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88000;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88000);
    cpu.pc = 0x4AE;
    16
}
/// $088000: jsr $5dd22.l
pub fn i_088000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88006;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88006);
    cpu.pc = 0x5DD22;
    16
}
/// $088006: jmp $518.l
pub fn i_088006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8800C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Rts_08800c  ($08800C, 2 B, enemies/bosses/fort_scene4_0865be.s)
/// $08800C: rts
pub fn i_08800c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8800E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Quad_A_08800e  ($08800E, 26 B, enemies/bosses/fort_scene4_0865be.s)
/// $08800E: lea.l $2ec30e.l, a0
pub fn i_08800e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88014;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC30E;
    20
}
/// $088014: jsr $28cd4.l
pub fn i_088014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8801A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8801A);
    cpu.pc = 0x28CD4;
    16
}
/// $08801A: lea.l $2ed19a.l, a1
pub fn i_08801a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88020;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED19A;
    20
}
/// $088020: move.l a1, $88(a6)
pub fn i_088020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88024;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $088024: bra.w $88076
pub fn i_088024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88028;
    cpu.ir = 0x6000;
    cpu.pc = 0x88076;
    10
}
// ===== Prop_Quad_B_088028  ($088028, 26 B, enemies/bosses/fort_scene4_0865be.s)
/// $088028: lea.l $2ec324.l, a0
pub fn i_088028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8802E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC324;
    20
}
/// $08802E: jsr $28cd4.l
pub fn i_08802e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88034;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88034);
    cpu.pc = 0x28CD4;
    16
}
/// $088034: lea.l $2ed1ac.l, a1
pub fn i_088034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8803A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED1AC;
    20
}
/// $08803A: move.l a1, $88(a6)
pub fn i_08803a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8803E;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08803E: bra.w $88076
pub fn i_08803e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88042;
    cpu.ir = 0x6000;
    cpu.pc = 0x88076;
    10
}
// ===== Prop_Quad_C_088042  ($088042, 26 B, enemies/bosses/fort_scene4_0865be.s)
/// $088042: lea.l $2ec33a.l, a0
pub fn i_088042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88048;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC33A;
    20
}
/// $088048: jsr $28cd4.l
pub fn i_088048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8804E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8804E);
    cpu.pc = 0x28CD4;
    16
}
/// $08804E: lea.l $2ed1be.l, a1
pub fn i_08804e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88054;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED1BE;
    20
}
/// $088054: move.l a1, $88(a6)
pub fn i_088054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88058;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $088058: bra.w $88076
pub fn i_088058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8805C;
    cpu.ir = 0x6000;
    cpu.pc = 0x88076;
    10
}
// ===== Prop_Quad_D_08805c  ($08805C, 184 B, enemies/bosses/fort_scene4_0865be.s)
/// $08805C: lea.l $2ec350.l, a0
pub fn i_08805c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88062;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC350;
    20
}
/// $088062: jsr $28cd4.l
pub fn i_088062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88068;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88068);
    cpu.pc = 0x28CD4;
    16
}
/// $088068: lea.l $2ed1d0.l, a1
pub fn i_088068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8806E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED1D0;
    20
}
/// $08806E: move.l a1, $88(a6)
pub fn i_08806e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88072;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $088072: bra.w $88076
pub fn i_088072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88076;
    cpu.ir = 0x6000;
    cpu.pc = 0x88076;
    10
}
/// $088076: move.w #$e4, d1
pub fn i_088076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8807A;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08807A: jsr $236e.l
pub fn i_08807a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88080;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88080);
    cpu.pc = 0x236E;
    16
}
/// $088080: move.b #$ff, $32(a6)
pub fn i_088080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88086;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $088086: move.b #$ff, $33(a6)
pub fn i_088086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8808C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08808C: move.w #$40, $70(a6)
pub fn i_08808c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88092;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $088092: move.w #$a, $66(a6)
pub fn i_088092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88098;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $088098: move.w #$8000, $38(a6)
pub fn i_088098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8809E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08809E: jsr $267e2.l
pub fn i_08809e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x880A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x880A4);
    cpu.pc = 0x267E2;
    16
}
/// $0880A4: jsr $27cee.l
pub fn i_0880a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x880AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x880AA);
    cpu.pc = 0x27CEE;
    16
}
/// $0880AA: lea.l $880b0(pc), a1
pub fn i_0880aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x880AE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x880B0;
    16
}
/// $0880AE: move.l a1, (a6)
pub fn i_0880ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x880B0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0880B0: jsr $2783a.l
pub fn i_0880b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x880B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x880B6);
    cpu.pc = 0x2783A;
    16
}
/// $0880B6: jsr $28d70.l
pub fn i_0880b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x880BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x880BC);
    cpu.pc = 0x28D70;
    16
}
/// $0880BC: jsr $2870a.l
pub fn i_0880bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x880C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x880C2);
    cpu.pc = 0x2870A;
    16
}
/// $0880C2: bcc.w $880d8
pub fn i_0880c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x880C6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x880D8; return 10; }
    8
}
/// $0880C6: lea.l $5e766.l, a0
pub fn i_0880c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x880CC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $0880CC: jsr $5e770.l
pub fn i_0880cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x880D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x880D2);
    cpu.pc = 0x5E770;
    16
}
/// $0880D2: bclr.b #$3, $13(a6)
pub fn i_0880d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x880D8;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0880D8: jsr $28758.l
pub fn i_0880d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x880DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x880DE);
    cpu.pc = 0x28758;
    16
}
/// $0880DE: bcc.w $88102
pub fn i_0880de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x880E2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x88102; return 10; }
    8
}
/// $0880E2: move.w #$102e, d0
pub fn i_0880e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x880E6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0880E6: jsr $2352.l
pub fn i_0880e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x880EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x880EC);
    cpu.pc = 0x2352;
    16
}
/// $0880EC: lea.l $ffff.w, a0
pub fn i_0880ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x880F0;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $0880F0: move.l a0, $48(a6)
pub fn i_0880f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x880F4;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0880F4: movea.l $88(a6), a1
pub fn i_0880f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x880F8;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0880F8: jsr $77c7e.l
pub fn i_0880f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x880FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x880FE);
    cpu.pc = 0x77C7E;
    16
}
/// $0880FE: bra.w $8810c
pub fn i_0880fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88102;
    cpu.ir = 0x6000;
    cpu.pc = 0x8810C;
    10
}
/// $088102: jsr $4fa70.l
pub fn i_088102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88108;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88108);
    cpu.pc = 0x4FA70;
    16
}
/// $088108: bcc.w $88112
pub fn i_088108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8810C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x88112; return 10; }
    8
}
/// $08810C: jmp $518.l
pub fn i_08810c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88112;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $088112: rts
pub fn i_088112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88114;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Barricade_Init_088114  ($088114, 148 B, enemies/bosses/fort_scene4_0865be.s)
/// $088114: movea.l $3c(a6), a1
pub fn i_088114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88118;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $088118: jsr $2942a.l
pub fn i_088118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8811E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8811E);
    cpu.pc = 0x2942A;
    16
}
/// $08811E: move.w #$e6, d1
pub fn i_08811e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88122;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $088122: jsr $236e.l
pub fn i_088122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88128;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88128);
    cpu.pc = 0x236E;
    16
}
/// $088128: move.b #$ff, $32(a6)
pub fn i_088128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8812E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08812E: move.b #$ff, $33(a6)
pub fn i_08812e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88134;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $088134: move.w #$40, $70(a6)
pub fn i_088134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8813A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08813A: move.w #$32, $66(a6)
pub fn i_08813a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88140;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x32;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $088140: move.w #$8000, $38(a6)
pub fn i_088140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88146;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $088146: jsr $267e2.l
pub fn i_088146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8814C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8814C);
    cpu.pc = 0x267E2;
    16
}
/// $08814C: jsr $27cee.l
pub fn i_08814c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88152;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88152);
    cpu.pc = 0x27CEE;
    16
}
/// $088152: lea.l $88158(pc), a1
pub fn i_088152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88156;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x88158;
    16
}
/// $088156: move.l a1, (a6)
pub fn i_088156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88158;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $088158: jsr $2783a.l
pub fn i_088158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8815E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8815E);
    cpu.pc = 0x2783A;
    16
}
/// $08815E: jsr $2870a.l
pub fn i_08815e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88164;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88164);
    cpu.pc = 0x2870A;
    16
}
/// $088164: bcc.w $8817a
pub fn i_088164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88168;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8817A; return 10; }
    8
}
/// $088168: lea.l $5e766.l, a0
pub fn i_088168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8816E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $08816E: jsr $5e770.l
pub fn i_08816e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88174;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88174);
    cpu.pc = 0x5E770;
    16
}
/// $088174: bclr.b #$3, $13(a6)
pub fn i_088174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8817A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08817A: move.w #$32, $66(a6)
pub fn i_08817a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88180;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x32;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $088180: bclr.b #$0, $13(a6)
pub fn i_088180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88186;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $088186: cmpi.w #$50, $22(a6)
pub fn i_088186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8818C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x50, Size::Word, false);
    16
}
/// $08818C: bgt.w $88196
pub fn i_08818c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88190;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x88196; return 10; }
    8
}
/// $088190: lea.l $881a8(pc), a1
pub fn i_088190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88194;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x881A8;
    16
}
/// $088194: move.l a1, (a6)
pub fn i_088194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88196;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $088196: jsr $4fa70.l
pub fn i_088196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8819C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8819C);
    cpu.pc = 0x4FA70;
    16
}
/// $08819C: bcc.w $881a6
pub fn i_08819c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x881A0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x881A6; return 10; }
    8
}
/// $0881A0: jmp $518.l
pub fn i_0881a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x881A6;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0881A6: rts
pub fn i_0881a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x881A8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Barricade_Arm_0881a8  ($0881A8, 98 B, enemies/bosses/fort_scene4_0865be.s)
/// $0881A8: lea.l $2ec366.l, a0
pub fn i_0881a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x881AE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC366;
    20
}
/// $0881AE: jsr $28cd4.l
pub fn i_0881ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x881B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x881B4);
    cpu.pc = 0x28CD4;
    16
}
/// $0881B4: lea.l $881ba(pc), a1
pub fn i_0881b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x881B8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x881BA;
    16
}
/// $0881B8: move.l a1, (a6)
pub fn i_0881b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x881BA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0881BA: jsr $2783a.l
pub fn i_0881ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x881C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x881C0);
    cpu.pc = 0x2783A;
    16
}
/// $0881C0: jsr $28d70.l
pub fn i_0881c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x881C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x881C6);
    cpu.pc = 0x28D70;
    16
}
/// $0881C6: bcc.w $881d0
pub fn i_0881c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x881CA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x881D0; return 10; }
    8
}
/// $0881CA: lea.l $8820a(pc), a1
pub fn i_0881ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x881CE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8820A;
    16
}
/// $0881CE: move.l a1, (a6)
pub fn i_0881ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x881D0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0881D0: jsr $2870a.l
pub fn i_0881d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x881D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x881D6);
    cpu.pc = 0x2870A;
    16
}
/// $0881D6: bcc.w $881ec
pub fn i_0881d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x881DA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x881EC; return 10; }
    8
}
/// $0881DA: lea.l $5e766.l, a0
pub fn i_0881da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x881E0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $0881E0: jsr $5e770.l
pub fn i_0881e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x881E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x881E6);
    cpu.pc = 0x5E770;
    16
}
/// $0881E6: bclr.b #$3, $13(a6)
pub fn i_0881e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x881EC;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0881EC: move.w #$32, $66(a6)
pub fn i_0881ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x881F2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x32;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0881F2: bclr.b #$0, $13(a6)
pub fn i_0881f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x881F8;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0881F8: jsr $4fa70.l
pub fn i_0881f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x881FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x881FE);
    cpu.pc = 0x4FA70;
    16
}
/// $0881FE: bcc.w $88208
pub fn i_0881fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88202;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x88208; return 10; }
    8
}
/// $088202: jmp $518.l
pub fn i_088202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88208;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $088208: rts
pub fn i_088208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8820A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Barricade_Stage1_08820a  ($08820A, 102 B, enemies/bosses/fort_scene4_0865be.s)
/// $08820A: lea.l $ee062.l, a1
pub fn i_08820a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88210;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEE062;
    20
}
/// $088210: move.w #$82, d0
pub fn i_088210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88214;
    cpu.ir = 0x303C;
    let v1: u32 = 0x82;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $088214: move.b #$0, $82(a6)
pub fn i_088214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8821A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08821A: jsr $4429e.l
pub fn i_08821a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88220;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88220);
    cpu.pc = 0x4429E;
    16
}
/// $088220: lea.l $88226(pc), a1
pub fn i_088220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88224;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x88226;
    16
}
/// $088224: move.l a1, (a6)
pub fn i_088224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88226;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $088226: jsr $2783a.l
pub fn i_088226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8822C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8822C);
    cpu.pc = 0x2783A;
    16
}
/// $08822C: jsr $28d70.l
pub fn i_08822c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88232;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88232);
    cpu.pc = 0x28D70;
    16
}
/// $088232: jsr $2870a.l
pub fn i_088232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88238;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88238);
    cpu.pc = 0x2870A;
    16
}
/// $088238: bcc.w $8824e
pub fn i_088238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8823C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8824E; return 10; }
    8
}
/// $08823C: lea.l $5e766.l, a0
pub fn i_08823c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88242;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $088242: jsr $5e770.l
pub fn i_088242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88248;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88248);
    cpu.pc = 0x5E770;
    16
}
/// $088248: bclr.b #$3, $13(a6)
pub fn i_088248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8824E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08824E: cmpi.w #$28, $66(a6)
pub fn i_08824e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88254;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x28, Size::Word, false);
    16
}
/// $088254: bgt.w $8825e
pub fn i_088254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88258;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8825E; return 10; }
    8
}
/// $088258: lea.l $88270(pc), a1
pub fn i_088258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8825C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x88270;
    16
}
/// $08825C: move.l a1, (a6)
pub fn i_08825c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8825E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08825E: jsr $4fa70.l
pub fn i_08825e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88264;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88264);
    cpu.pc = 0x4FA70;
    16
}
/// $088264: bcc.w $8826e
pub fn i_088264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88268;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8826E; return 10; }
    8
}
/// $088268: jmp $518.l
pub fn i_088268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8826E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08826E: rts
pub fn i_08826e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88270;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Barricade_Stage2_088270  ($088270, 92 B, enemies/bosses/fort_scene4_0865be.s)
/// $088270: lea.l $2ec3de.l, a0
pub fn i_088270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88276;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC3DE;
    20
}
/// $088276: jsr $28cd4.l
pub fn i_088276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8827C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8827C);
    cpu.pc = 0x28CD4;
    16
}
/// $08827C: lea.l $88282(pc), a1
pub fn i_08827c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88280;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x88282;
    16
}
/// $088280: move.l a1, (a6)
pub fn i_088280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88282;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $088282: jsr $2783a.l
pub fn i_088282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88288;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88288);
    cpu.pc = 0x2783A;
    16
}
/// $088288: jsr $28d70.l
pub fn i_088288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8828E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8828E);
    cpu.pc = 0x28D70;
    16
}
/// $08828E: jsr $2870a.l
pub fn i_08828e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88294;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88294);
    cpu.pc = 0x2870A;
    16
}
/// $088294: bcc.w $882aa
pub fn i_088294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88298;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x882AA; return 10; }
    8
}
/// $088298: lea.l $5e766.l, a0
pub fn i_088298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8829E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $08829E: jsr $5e770.l
pub fn i_08829e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x882A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x882A4);
    cpu.pc = 0x5E770;
    16
}
/// $0882A4: bclr.b #$3, $13(a6)
pub fn i_0882a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x882AA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0882AA: cmpi.w #$1e, $66(a6)
pub fn i_0882aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x882B0;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1E, Size::Word, false);
    16
}
/// $0882B0: bgt.w $882ba
pub fn i_0882b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x882B4;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x882BA; return 10; }
    8
}
/// $0882B4: lea.l $882cc(pc), a1
pub fn i_0882b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x882B8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x882CC;
    16
}
/// $0882B8: move.l a1, (a6)
pub fn i_0882b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x882BA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0882BA: jsr $4fa70.l
pub fn i_0882ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x882C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x882C0);
    cpu.pc = 0x4FA70;
    16
}
/// $0882C0: bcc.w $882ca
pub fn i_0882c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x882C4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x882CA; return 10; }
    8
}
/// $0882C4: jmp $518.l
pub fn i_0882c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x882CA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0882CA: rts
pub fn i_0882ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x882CC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Barricade_Stage3_0882cc  ($0882CC, 92 B, enemies/bosses/fort_scene4_0865be.s)
/// $0882CC: lea.l $2ec3ee.l, a0
pub fn i_0882cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x882D2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC3EE;
    20
}
/// $0882D2: jsr $28cd4.l
pub fn i_0882d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x882D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x882D8);
    cpu.pc = 0x28CD4;
    16
}
/// $0882D8: lea.l $882de(pc), a1
pub fn i_0882d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x882DC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x882DE;
    16
}
/// $0882DC: move.l a1, (a6)
pub fn i_0882dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x882DE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0882DE: jsr $2783a.l
pub fn i_0882de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x882E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x882E4);
    cpu.pc = 0x2783A;
    16
}
/// $0882E4: jsr $28d70.l
pub fn i_0882e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x882EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x882EA);
    cpu.pc = 0x28D70;
    16
}
/// $0882EA: jsr $2870a.l
pub fn i_0882ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x882F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x882F0);
    cpu.pc = 0x2870A;
    16
}
/// $0882F0: bcc.w $88306
pub fn i_0882f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x882F4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x88306; return 10; }
    8
}
/// $0882F4: lea.l $5e766.l, a0
pub fn i_0882f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x882FA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $0882FA: jsr $5e770.l
pub fn i_0882fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88300;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88300);
    cpu.pc = 0x5E770;
    16
}
/// $088300: bclr.b #$3, $13(a6)
pub fn i_088300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88306;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $088306: cmpi.w #$14, $66(a6)
pub fn i_088306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8830C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x14, Size::Word, false);
    16
}
/// $08830C: bgt.w $88316
pub fn i_08830c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88310;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x88316; return 10; }
    8
}
/// $088310: lea.l $88328(pc), a1
pub fn i_088310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88314;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x88328;
    16
}
/// $088314: move.l a1, (a6)
pub fn i_088314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88316;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $088316: jsr $4fa70.l
pub fn i_088316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8831C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8831C);
    cpu.pc = 0x4FA70;
    16
}
/// $08831C: bcc.w $88326
pub fn i_08831c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88320;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x88326; return 10; }
    8
}
/// $088320: jmp $518.l
pub fn i_088320(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88326;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $088326: rts
pub fn i_088326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88328;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Barricade_Stage4_088328  ($088328, 104 B, enemies/bosses/fort_scene4_0865be.s)
/// $088328: lea.l $2ec3fe.l, a0
pub fn i_088328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8832E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC3FE;
    20
}
/// $08832E: jsr $28cd4.l
pub fn i_08832e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88334;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88334);
    cpu.pc = 0x28CD4;
    16
}
/// $088334: lea.l $8833a(pc), a1
pub fn i_088334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88338;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8833A;
    16
}
/// $088338: move.l a1, (a6)
pub fn i_088338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8833A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08833A: jsr $2783a.l
pub fn i_08833a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88340;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88340);
    cpu.pc = 0x2783A;
    16
}
/// $088340: jsr $28d70.l
pub fn i_088340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88346;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88346);
    cpu.pc = 0x28D70;
    16
}
/// $088346: jsr $2870a.l
pub fn i_088346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8834C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8834C);
    cpu.pc = 0x2870A;
    16
}
/// $08834C: bcc.w $88362
pub fn i_08834c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88350;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x88362; return 10; }
    8
}
/// $088350: lea.l $5e766.l, a0
pub fn i_088350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88356;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $088356: jsr $5e770.l
pub fn i_088356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8835C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8835C);
    cpu.pc = 0x5E770;
    16
}
/// $08835C: bclr.b #$3, $13(a6)
pub fn i_08835c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88362;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $088362: cmpi.w #$a, $66(a6)
pub fn i_088362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88368;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xA, Size::Word, false);
    16
}
/// $088368: bgt.w $8837e
pub fn i_088368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8836C;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8837E; return 10; }
    8
}
/// $08836C: lea.l $2ed1e2.l, a1
pub fn i_08836c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88372;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED1E2;
    20
}
/// $088372: jsr $77c7e.l
pub fn i_088372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88378;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88378);
    cpu.pc = 0x77C7E;
    16
}
/// $088378: lea.l $88390(pc), a1
pub fn i_088378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8837C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x88390;
    16
}
/// $08837C: move.l a1, (a6)
pub fn i_08837c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8837E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08837E: jsr $4fa70.l
pub fn i_08837e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88384;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88384);
    cpu.pc = 0x4FA70;
    16
}
/// $088384: bcc.w $8838e
pub fn i_088384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88388;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8838E; return 10; }
    8
}
/// $088388: jmp $518.l
pub fn i_088388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8838E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08838E: rts
pub fn i_08838e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88390;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Barricade_Stage5_088390  ($088390, 92 B, enemies/bosses/fort_scene4_0865be.s)
/// $088390: lea.l $2ec40e.l, a0
pub fn i_088390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88396;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC40E;
    20
}
/// $088396: jsr $28cd4.l
pub fn i_088396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8839C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8839C);
    cpu.pc = 0x28CD4;
    16
}
/// $08839C: lea.l $883a2(pc), a1
pub fn i_08839c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x883A0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x883A2;
    16
}
/// $0883A0: move.l a1, (a6)
pub fn i_0883a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x883A2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0883A2: jsr $86552(pc)
pub fn i_0883a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x883A6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x883A6);
    cpu.pc = 0x86552;
    16
}
/// $0883A6: bcs.b $8837e
pub fn i_0883a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x883A8;
    cpu.ir = 0x65D6;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8837E; return 10; }
    8
}
/// $0883A8: jsr $2783a.l
pub fn i_0883a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x883AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x883AE);
    cpu.pc = 0x2783A;
    16
}
/// $0883AE: jsr $28d70.l
pub fn i_0883ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x883B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x883B4);
    cpu.pc = 0x28D70;
    16
}
/// $0883B4: jsr $2870a.l
pub fn i_0883b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x883BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x883BA);
    cpu.pc = 0x2870A;
    16
}
/// $0883BA: bcc.w $883d0
pub fn i_0883ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x883BE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x883D0; return 10; }
    8
}
/// $0883BE: lea.l $5e766.l, a0
pub fn i_0883be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x883C4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $0883C4: jsr $5e770.l
pub fn i_0883c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x883CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x883CA);
    cpu.pc = 0x5E770;
    16
}
/// $0883CA: bclr.b #$3, $13(a6)
pub fn i_0883ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x883D0;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0883D0: jsr $28758.l
pub fn i_0883d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x883D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x883D6);
    cpu.pc = 0x28758;
    16
}
/// $0883D6: bra.w $883e4
pub fn i_0883d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x883DA;
    cpu.ir = 0x6000;
    cpu.pc = 0x883E4;
    10
}
/// $0883DA: jsr $4fa70.l
pub fn i_0883da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x883E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x883E0);
    cpu.pc = 0x4FA70;
    16
}
/// $0883E0: bcc.w $883ea
pub fn i_0883e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x883E4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x883EA; return 10; }
    8
}
/// $0883E4: jmp $518.l
pub fn i_0883e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x883EA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0883EA: rts
pub fn i_0883ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x883EC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Heli_RotorAnim_0883ec  ($0883EC, 70 B, enemies/bosses/fort_scene4_0865be.s)
/// $0883EC: move.b $106f28.l, d0
pub fn i_0883ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x883F2;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $0883F2: cmpi.w #$0, $72(a6)
pub fn i_0883f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x883F8;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0883F8: ble.w $8841c
pub fn i_0883f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x883FC;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x8841C; return 10; }
    8
}
/// $0883FC: andi.b #$3, d0
pub fn i_0883fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88400;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $088400: bne.w $88436
pub fn i_088400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88404;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x88436; return 10; }
    8
}
/// $088404: move.w $18(a6), d0
pub fn i_088404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88408;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x18);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $088408: move.w $16(a6), $18(a6)
pub fn i_088408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8840E;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x18);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08840E: move.w d0, $16(a6)
pub fn i_08840e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88412;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    bus.write16(ad2, v1 as u16);
    12
}
/// $088412: move.w d0, $14(a6)
pub fn i_088412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88416;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad2, v1 as u16);
    12
}
/// $088416: subq.w #$1, $72(a6)
pub fn i_088416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8841A;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08841A: rts
pub fn i_08841a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8841C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $08841C: andi.b #$f, d0
pub fn i_08841c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88420;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $088420: bne.w $88436
pub fn i_088420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88424;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x88436; return 10; }
    8
}
/// $088424: move.w $18(a6), d0
pub fn i_088424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88428;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x18);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $088428: move.w $16(a6), $18(a6)
pub fn i_088428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8842E;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x18);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08842E: move.w d0, $16(a6)
pub fn i_08842e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88432;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskW_088432  ($088432, 6 B, kernel/task_setters_w.c)
/// $088432: move.w d0, $14(a6)
pub fn i_088432(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88436;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad2, v1 as u16);
    12
}
/// $088436: rts
pub fn i_088436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88438;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_HitboxPulseTable_088438  ($088438, 50 B, enemies/bosses/fort_scene4_0865be.s)
/// $088438: addq.w #$1, $72(a6)
pub fn i_088438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8843C;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08843C: move.w $72(a6), d0
pub fn i_08843c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88440;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $088440: andi.b #$3, d0
pub fn i_088440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88444;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $088444: bne.w $88460
pub fn i_088444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88448;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x88460; return 10; }
    8
}
/// $088448: clr.l d0
pub fn i_088448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8844A;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $08844A: move.b $21(a6), d0
pub fn i_08844a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8844E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08844E: asl.l #$2, d0
pub fn i_08844e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88450;
    cpu.ir = 0xE580;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Long, 0, true);
    cpu.d[0] = r;
    10
}
/// $088450: lea.l $2ec712.l, a0
pub fn i_088450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88456;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC712;
    20
}
/// $088456: movea.l (a0, d0.w), a0
pub fn i_088456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8845A;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $08845A: move.l a0, $48(a6)
pub fn i_08845a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8845E;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08845E: rts
pub fn i_08845e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88460;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $088460: lea.l $ffff.w, a0
pub fn i_088460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88464;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $088464: move.l a0, $48(a6)
pub fn i_088464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88468;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $088468: rts
pub fn i_088468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8846A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_HitboxPulseSaved_08846a  ($08846A, 34 B, enemies/bosses/fort_scene4_0865be.s)
/// $08846A: addq.w #$1, $72(a6)
pub fn i_08846a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8846E;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08846E: move.w $72(a6), d0
pub fn i_08846e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88472;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $088472: andi.b #$3, d0
pub fn i_088472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88476;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $088476: bne.w $88482
pub fn i_088476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8847A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x88482; return 10; }
    8
}
/// $08847A: move.l $84(a6), $48(a6)
pub fn i_08847a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88480;
    cpu.ir = 0x2D6E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $088480: rts
pub fn i_088480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88482;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $088482: lea.l $ffff.w, a0
pub fn i_088482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88486;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $088486: move.l a0, $48(a6)
pub fn i_088486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8848A;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08848A: rts
pub fn i_08848a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8848C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_PropagateDamageToParent_08848c  ($08848C, 26 B, enemies/bosses/fort_scene4_0865be.s)
/// $08848C: move.w $80(a6), d0
pub fn i_08848c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88490;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $088490: move.w $66(a6), d1
pub fn i_088490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88494;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $088494: sub.w d1, d0
pub fn i_088494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88496;
    cpu.ir = 0x9041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $088496: movea.l $c(a6), a0
pub fn i_088496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8849A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $08849A: sub.w d0, $66(a0)
pub fn i_08849a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8849E;
    cpu.ir = 0x9168;
    let ad1 = cpu.a[0].wrapping_add(0x66);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08849E: move.w $66(a6), $80(a6)
pub fn i_08849e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x884A4;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0884A4: rts
pub fn i_0884a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x884A6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Fort_SpawnChildren_V0_0884a6  ($0884A6, 270 B, enemies/bosses/fort_scene4_0865be.s)
/// $0884A6: lea.l $868a6(pc), a1
pub fn i_0884a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x884AA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x868A6;
    16
}
/// $0884AA: jsr $4ae.l
pub fn i_0884aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x884B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x884B0);
    cpu.pc = 0x4AE;
    16
}
/// $0884B0: jsr $5dd22.l
pub fn i_0884b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x884B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x884B6);
    cpu.pc = 0x5DD22;
    16
}
/// $0884B6: addi.w #$28, $22(a0)
pub fn i_0884b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x884BC;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x28, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0884BC: addi.w #$50, $24(a0)
pub fn i_0884bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x884C2;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x50, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0884C2: move.w #$60, $38(a0)
pub fn i_0884c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x884C8;
    cpu.ir = 0x317C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0884C8: lea.l $2ed738.l, a1
pub fn i_0884c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x884CE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED738;
    20
}
/// $0884CE: move.l a1, $74(a0)
pub fn i_0884ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x884D2;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0884D2: move.w #$0, $72(a0)
pub fn i_0884d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x884D8;
    cpu.ir = 0x317C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0884D8: lea.l $ee066.l, a1
pub fn i_0884d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x884DE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEE066;
    20
}
/// $0884DE: move.l a1, $7c(a0)
pub fn i_0884de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x884E2;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0884E2: lea.l $8686a(pc), a1
pub fn i_0884e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x884E6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8686A;
    16
}
/// $0884E6: jsr $4ae.l
pub fn i_0884e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x884EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x884EC);
    cpu.pc = 0x4AE;
    16
}
/// $0884EC: jsr $5dd22.l
pub fn i_0884ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x884F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x884F2);
    cpu.pc = 0x5DD22;
    16
}
/// $0884F2: addi.w #$68, $22(a0)
pub fn i_0884f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x884F8;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x68, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0884F8: addi.w #$50, $24(a0)
pub fn i_0884f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x884FE;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x50, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0884FE: move.w #$60, $38(a0)
pub fn i_0884fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88504;
    cpu.ir = 0x317C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $088504: lea.l $2ed74c.l, a1
pub fn i_088504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8850A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED74C;
    20
}
/// $08850A: move.l a1, $74(a0)
pub fn i_08850a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8850E;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08850E: move.w #$1, $72(a0)
pub fn i_08850e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88514;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $088514: lea.l $ee07c.l, a1
pub fn i_088514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8851A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEE07C;
    20
}
/// $08851A: move.l a1, $7c(a0)
pub fn i_08851a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8851E;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08851E: lea.l $868a6(pc), a1
pub fn i_08851e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88522;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x868A6;
    16
}
/// $088522: jsr $4ae.l
pub fn i_088522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88528;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88528);
    cpu.pc = 0x4AE;
    16
}
/// $088528: jsr $5dd22.l
pub fn i_088528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8852E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8852E);
    cpu.pc = 0x5DD22;
    16
}
/// $08852E: addi.w #$a8, $22(a0)
pub fn i_08852e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88534;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xA8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $088534: addi.w #$50, $24(a0)
pub fn i_088534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8853A;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x50, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08853A: move.w #$60, $38(a0)
pub fn i_08853a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88540;
    cpu.ir = 0x317C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $088540: lea.l $2ed760.l, a1
pub fn i_088540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88546;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED760;
    20
}
/// $088546: move.l a1, $74(a0)
pub fn i_088546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8854A;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08854A: move.w #$2, $72(a0)
pub fn i_08854a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88550;
    cpu.ir = 0x317C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $088550: lea.l $ee092.l, a1
pub fn i_088550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88556;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEE092;
    20
}
/// $088556: move.l a1, $7c(a0)
pub fn i_088556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8855A;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08855A: lea.l $86cd0(pc), a1
pub fn i_08855a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8855E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86CD0;
    16
}
/// $08855E: jsr $4ae.l
pub fn i_08855e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88564;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88564);
    cpu.pc = 0x4AE;
    16
}
/// $088564: jsr $5dd22.l
pub fn i_088564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8856A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8856A);
    cpu.pc = 0x5DD22;
    16
}
/// $08856A: addi.w #$30, $22(a0)
pub fn i_08856a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88570;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x30, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $088570: addi.w #$8, $24(a0)
pub fn i_088570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88576;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $088576: lea.l $2ed774.l, a1
pub fn i_088576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8857C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED774;
    20
}
/// $08857C: move.l a1, $74(a0)
pub fn i_08857c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88580;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $088580: move.w #$3, $72(a0)
pub fn i_088580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88586;
    cpu.ir = 0x317C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $088586: lea.l $86cd0(pc), a1
pub fn i_088586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8858A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86CD0;
    16
}
/// $08858A: jsr $4ae.l
pub fn i_08858a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88590;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88590);
    cpu.pc = 0x4AE;
    16
}
/// $088590: jsr $5dd22.l
pub fn i_088590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88596;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88596);
    cpu.pc = 0x5DD22;
    16
}
/// $088596: addi.w #$a8, $22(a0)
pub fn i_088596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8859C;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xA8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08859C: addi.w #$8, $24(a0)
pub fn i_08859c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x885A2;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0885A2: lea.l $2ed788.l, a1
pub fn i_0885a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x885A8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED788;
    20
}
/// $0885A8: move.l a1, $74(a0)
pub fn i_0885a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x885AC;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0885AC: move.w #$4, $72(a0)
pub fn i_0885ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x885B2;
    cpu.ir = 0x317C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0885B2: rts
pub fn i_0885b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x885B4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Fort_SpawnChildren_V1_0885b4  ($0885B4, 270 B, enemies/bosses/fort_scene4_0865be.s)
/// $0885B4: lea.l $868a6(pc), a1
pub fn i_0885b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x885B8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x868A6;
    16
}
/// $0885B8: jsr $4ae.l
pub fn i_0885b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x885BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x885BE);
    cpu.pc = 0x4AE;
    16
}
/// $0885BE: jsr $5dd22.l
pub fn i_0885be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x885C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x885C4);
    cpu.pc = 0x5DD22;
    16
}
/// $0885C4: addi.w #$28, $22(a0)
pub fn i_0885c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x885CA;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x28, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0885CA: addi.w #$40, $24(a0)
pub fn i_0885ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x885D0;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0885D0: move.w #$70, $38(a0)
pub fn i_0885d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x885D6;
    cpu.ir = 0x317C;
    let v1: u32 = 0x70;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0885D6: lea.l $2ed79c.l, a1
pub fn i_0885d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x885DC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED79C;
    20
}
/// $0885DC: move.l a1, $74(a0)
pub fn i_0885dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x885E0;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0885E0: move.w #$0, $72(a0)
pub fn i_0885e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x885E6;
    cpu.ir = 0x317C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0885E6: lea.l $ee0a8.l, a1
pub fn i_0885e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x885EC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEE0A8;
    20
}
/// $0885EC: move.l a1, $7c(a0)
pub fn i_0885ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x885F0;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0885F0: lea.l $8686a(pc), a1
pub fn i_0885f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x885F4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8686A;
    16
}
/// $0885F4: jsr $4ae.l
pub fn i_0885f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x885FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x885FA);
    cpu.pc = 0x4AE;
    16
}
/// $0885FA: jsr $5dd22.l
pub fn i_0885fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88600;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88600);
    cpu.pc = 0x5DD22;
    16
}
/// $088600: addi.w #$68, $22(a0)
pub fn i_088600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88606;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x68, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $088606: addi.w #$40, $24(a0)
pub fn i_088606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8860C;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08860C: move.w #$70, $38(a0)
pub fn i_08860c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88612;
    cpu.ir = 0x317C;
    let v1: u32 = 0x70;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $088612: lea.l $2ed7b0.l, a1
pub fn i_088612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88618;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED7B0;
    20
}
/// $088618: move.l a1, $74(a0)
pub fn i_088618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8861C;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08861C: move.w #$1, $72(a0)
pub fn i_08861c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88622;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $088622: lea.l $ee0be.l, a1
pub fn i_088622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88628;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEE0BE;
    20
}
/// $088628: move.l a1, $7c(a0)
pub fn i_088628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8862C;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08862C: lea.l $868a6(pc), a1
pub fn i_08862c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88630;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x868A6;
    16
}
/// $088630: jsr $4ae.l
pub fn i_088630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88636;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88636);
    cpu.pc = 0x4AE;
    16
}
/// $088636: jsr $5dd22.l
pub fn i_088636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8863C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8863C);
    cpu.pc = 0x5DD22;
    16
}
/// $08863C: addi.w #$a8, $22(a0)
pub fn i_08863c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88642;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xA8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $088642: addi.w #$40, $24(a0)
pub fn i_088642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88648;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $088648: move.w #$70, $38(a0)
pub fn i_088648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8864E;
    cpu.ir = 0x317C;
    let v1: u32 = 0x70;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08864E: lea.l $2ed7c4.l, a1
pub fn i_08864e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88654;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED7C4;
    20
}
/// $088654: move.l a1, $74(a0)
pub fn i_088654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88658;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $088658: move.w #$2, $72(a0)
pub fn i_088658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8865E;
    cpu.ir = 0x317C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08865E: lea.l $ee0d4.l, a1
pub fn i_08865e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88664;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEE0D4;
    20
}
/// $088664: move.l a1, $7c(a0)
pub fn i_088664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88668;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $088668: lea.l $86ce4(pc), a1
pub fn i_088668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8866C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86CE4;
    16
}
/// $08866C: jsr $4ae.l
pub fn i_08866c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88672;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88672);
    cpu.pc = 0x4AE;
    16
}
/// $088672: jsr $5dd22.l
pub fn i_088672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88678;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88678);
    cpu.pc = 0x5DD22;
    16
}
/// $088678: addi.w #$28, $22(a0)
pub fn i_088678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8867E;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x28, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08867E: addi.w #$0, $24(a0)
pub fn i_08867e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88684;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $088684: lea.l $2ed7d8.l, a1
pub fn i_088684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8868A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED7D8;
    20
}
/// $08868A: move.l a1, $74(a0)
pub fn i_08868a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8868E;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08868E: move.w #$3, $72(a0)
pub fn i_08868e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88694;
    cpu.ir = 0x317C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $088694: lea.l $86ce4(pc), a1
pub fn i_088694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88698;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86CE4;
    16
}
/// $088698: jsr $4ae.l
pub fn i_088698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8869E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8869E);
    cpu.pc = 0x4AE;
    16
}
/// $08869E: jsr $5dd22.l
pub fn i_08869e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x886A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x886A4);
    cpu.pc = 0x5DD22;
    16
}
/// $0886A4: addi.w #$a0, $22(a0)
pub fn i_0886a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x886AA;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xA0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0886AA: addi.w #$0, $24(a0)
pub fn i_0886aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x886B0;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0886B0: lea.l $2ed7ec.l, a1
pub fn i_0886b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x886B6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED7EC;
    20
}
/// $0886B6: move.l a1, $74(a0)
pub fn i_0886b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x886BA;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0886BA: move.w #$4, $72(a0)
pub fn i_0886ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x886C0;
    cpu.ir = 0x317C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0886C0: rts
pub fn i_0886c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x886C2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Fort_SpawnChildren_V2_0886c2  ($0886C2, 270 B, enemies/bosses/fort_scene4_0865be.s)
/// $0886C2: lea.l $868ba(pc), a1
pub fn i_0886c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x886C6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x868BA;
    16
}
/// $0886C6: jsr $4ae.l
pub fn i_0886c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x886CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x886CC);
    cpu.pc = 0x4AE;
    16
}
/// $0886CC: jsr $5dd22.l
pub fn i_0886cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x886D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x886D2);
    cpu.pc = 0x5DD22;
    16
}
/// $0886D2: addi.w #$28, $22(a0)
pub fn i_0886d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x886D8;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x28, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0886D8: addi.w #$50, $24(a0)
pub fn i_0886d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x886DE;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x50, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0886DE: move.w #$60, $38(a0)
pub fn i_0886de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x886E4;
    cpu.ir = 0x317C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0886E4: lea.l $2ed800.l, a1
pub fn i_0886e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x886EA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED800;
    20
}
/// $0886EA: move.l a1, $74(a0)
pub fn i_0886ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x886EE;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0886EE: move.w #$0, $72(a0)
pub fn i_0886ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x886F4;
    cpu.ir = 0x317C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0886F4: lea.l $ee0ea.l, a1
pub fn i_0886f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x886FA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEE0EA;
    20
}
/// $0886FA: move.l a1, $7c(a0)
pub fn i_0886fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x886FE;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0886FE: lea.l $868ba(pc), a1
pub fn i_0886fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88702;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x868BA;
    16
}
/// $088702: jsr $4ae.l
pub fn i_088702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88708;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88708);
    cpu.pc = 0x4AE;
    16
}
/// $088708: jsr $5dd22.l
pub fn i_088708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8870E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8870E);
    cpu.pc = 0x5DD22;
    16
}
/// $08870E: addi.w #$68, $22(a0)
pub fn i_08870e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88714;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x68, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $088714: addi.w #$50, $24(a0)
pub fn i_088714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8871A;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x50, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08871A: move.w #$60, $38(a0)
pub fn i_08871a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88720;
    cpu.ir = 0x317C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $088720: lea.l $2ed814.l, a1
pub fn i_088720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88726;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED814;
    20
}
/// $088726: move.l a1, $74(a0)
pub fn i_088726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8872A;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08872A: move.w #$1, $72(a0)
pub fn i_08872a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88730;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $088730: lea.l $ee100.l, a1
pub fn i_088730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88736;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEE100;
    20
}
/// $088736: move.l a1, $7c(a0)
pub fn i_088736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8873A;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08873A: lea.l $868ba(pc), a1
pub fn i_08873a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8873E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x868BA;
    16
}
/// $08873E: jsr $4ae.l
pub fn i_08873e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88744;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88744);
    cpu.pc = 0x4AE;
    16
}
/// $088744: jsr $5dd22.l
pub fn i_088744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8874A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8874A);
    cpu.pc = 0x5DD22;
    16
}
/// $08874A: addi.w #$a8, $22(a0)
pub fn i_08874a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88750;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xA8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $088750: addi.w #$50, $24(a0)
pub fn i_088750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88756;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x50, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $088756: move.w #$60, $38(a0)
pub fn i_088756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8875C;
    cpu.ir = 0x317C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08875C: lea.l $2ed828.l, a1
pub fn i_08875c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88762;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED828;
    20
}
/// $088762: move.l a1, $74(a0)
pub fn i_088762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88766;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $088766: move.w #$2, $72(a0)
pub fn i_088766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8876C;
    cpu.ir = 0x317C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08876C: lea.l $ee116.l, a1
pub fn i_08876c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88772;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEE116;
    20
}
/// $088772: move.l a1, $7c(a0)
pub fn i_088772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88776;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $088776: lea.l $86cf8(pc), a1
pub fn i_088776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8877A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86CF8;
    16
}
/// $08877A: jsr $4ae.l
pub fn i_08877a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88780;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88780);
    cpu.pc = 0x4AE;
    16
}
/// $088780: jsr $5dd22.l
pub fn i_088780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88786;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88786);
    cpu.pc = 0x5DD22;
    16
}
/// $088786: addi.w #$28, $22(a0)
pub fn i_088786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8878C;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x28, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08878C: addi.w #$8, $24(a0)
pub fn i_08878c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88792;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $088792: lea.l $2ed83c.l, a1
pub fn i_088792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88798;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED83C;
    20
}
/// $088798: move.l a1, $74(a0)
pub fn i_088798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8879C;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08879C: move.w #$3, $72(a0)
pub fn i_08879c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x887A2;
    cpu.ir = 0x317C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0887A2: lea.l $86cf8(pc), a1
pub fn i_0887a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x887A6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86CF8;
    16
}
/// $0887A6: jsr $4ae.l
pub fn i_0887a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x887AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x887AC);
    cpu.pc = 0x4AE;
    16
}
/// $0887AC: jsr $5dd22.l
pub fn i_0887ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x887B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x887B2);
    cpu.pc = 0x5DD22;
    16
}
/// $0887B2: addi.w #$a0, $22(a0)
pub fn i_0887b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x887B8;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xA0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0887B8: addi.w #$8, $24(a0)
pub fn i_0887b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x887BE;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0887BE: lea.l $2ed850.l, a1
pub fn i_0887be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x887C4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED850;
    20
}
/// $0887C4: move.l a1, $74(a0)
pub fn i_0887c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x887C8;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0887C8: move.w #$4, $72(a0)
pub fn i_0887c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x887CE;
    cpu.ir = 0x317C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0887CE: rts
pub fn i_0887ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x887D0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Fort_SpawnChildren_V3_0887d0  ($0887D0, 330 B, enemies/bosses/fort_scene4_0865be.s)
/// $0887D0: lea.l $86892(pc), a1
pub fn i_0887d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x887D4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86892;
    16
}
/// $0887D4: jsr $4ae.l
pub fn i_0887d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x887DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x887DA);
    cpu.pc = 0x4AE;
    16
}
/// $0887DA: jsr $5dd22.l
pub fn i_0887da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x887E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x887E0);
    cpu.pc = 0x5DD22;
    16
}
/// $0887E0: addi.w #$18, $22(a0)
pub fn i_0887e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x887E6;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0887E6: addi.w #$40, $24(a0)
pub fn i_0887e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x887EC;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0887EC: move.w #$70, $38(a0)
pub fn i_0887ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x887F2;
    cpu.ir = 0x317C;
    let v1: u32 = 0x70;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0887F2: lea.l $2ed88c.l, a1
pub fn i_0887f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x887F8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED88C;
    20
}
/// $0887F8: move.l a1, $74(a0)
pub fn i_0887f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x887FC;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0887FC: move.w #$0, $72(a0)
pub fn i_0887fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88802;
    cpu.ir = 0x317C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $088802: lea.l $ee12c.l, a1
pub fn i_088802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88808;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEE12C;
    20
}
/// $088808: move.l a1, $7c(a0)
pub fn i_088808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8880C;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08880C: lea.l $868a6(pc), a1
pub fn i_08880c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88810;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x868A6;
    16
}
/// $088810: jsr $4ae.l
pub fn i_088810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88816;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88816);
    cpu.pc = 0x4AE;
    16
}
/// $088816: jsr $5dd22.l
pub fn i_088816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8881C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8881C);
    cpu.pc = 0x5DD22;
    16
}
/// $08881C: addi.w #$48, $22(a0)
pub fn i_08881c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88822;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x48, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $088822: addi.w #$40, $24(a0)
pub fn i_088822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88828;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $088828: move.w #$70, $38(a0)
pub fn i_088828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8882E;
    cpu.ir = 0x317C;
    let v1: u32 = 0x70;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08882E: lea.l $2ed8a0.l, a1
pub fn i_08882e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88834;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED8A0;
    20
}
/// $088834: move.l a1, $74(a0)
pub fn i_088834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88838;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $088838: move.w #$1, $72(a0)
pub fn i_088838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8883E;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08883E: lea.l $ee130.l, a1
pub fn i_08883e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88844;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEE130;
    20
}
/// $088844: move.l a1, $7c(a0)
pub fn i_088844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88848;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $088848: lea.l $868a6(pc), a1
pub fn i_088848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8884C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x868A6;
    16
}
/// $08884C: jsr $4ae.l
pub fn i_08884c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88852;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88852);
    cpu.pc = 0x4AE;
    16
}
/// $088852: jsr $5dd22.l
pub fn i_088852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88858;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88858);
    cpu.pc = 0x5DD22;
    16
}
/// $088858: addi.w #$78, $22(a0)
pub fn i_088858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8885E;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x78, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08885E: addi.w #$40, $24(a0)
pub fn i_08885e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88864;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $088864: move.w #$70, $38(a0)
pub fn i_088864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8886A;
    cpu.ir = 0x317C;
    let v1: u32 = 0x70;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08886A: lea.l $2ed8b4.l, a1
pub fn i_08886a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88870;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED8B4;
    20
}
/// $088870: move.l a1, $74(a0)
pub fn i_088870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88874;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $088874: move.w #$2, $72(a0)
pub fn i_088874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8887A;
    cpu.ir = 0x317C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08887A: lea.l $ee146.l, a1
pub fn i_08887a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88880;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEE146;
    20
}
/// $088880: move.l a1, $7c(a0)
pub fn i_088880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88884;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $088884: lea.l $868a6(pc), a1
pub fn i_088884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88888;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x868A6;
    16
}
/// $088888: jsr $4ae.l
pub fn i_088888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8888E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8888E);
    cpu.pc = 0x4AE;
    16
}
/// $08888E: jsr $5dd22.l
pub fn i_08888e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88894;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88894);
    cpu.pc = 0x5DD22;
    16
}
/// $088894: addi.w #$a8, $22(a0)
pub fn i_088894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8889A;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xA8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08889A: addi.w #$40, $24(a0)
pub fn i_08889a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x888A0;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0888A0: move.w #$70, $38(a0)
pub fn i_0888a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x888A6;
    cpu.ir = 0x317C;
    let v1: u32 = 0x70;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0888A6: lea.l $2ed8c8.l, a1
pub fn i_0888a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x888AC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED8C8;
    20
}
/// $0888AC: move.l a1, $74(a0)
pub fn i_0888ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x888B0;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0888B0: move.w #$3, $72(a0)
pub fn i_0888b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x888B6;
    cpu.ir = 0x317C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0888B6: lea.l $ee15c.l, a1
pub fn i_0888b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x888BC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEE15C;
    20
}
/// $0888BC: move.l a1, $7c(a0)
pub fn i_0888bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x888C0;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0888C0: lea.l $86d0c(pc), a1
pub fn i_0888c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x888C4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86D0C;
    16
}
/// $0888C4: jsr $4ae.l
pub fn i_0888c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x888CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x888CA);
    cpu.pc = 0x4AE;
    16
}
/// $0888CA: jsr $5dd22.l
pub fn i_0888ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x888D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x888D0);
    cpu.pc = 0x5DD22;
    16
}
/// $0888D0: addi.w #$28, $22(a0)
pub fn i_0888d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x888D6;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x28, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0888D6: addi.w #$8, $24(a0)
pub fn i_0888d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x888DC;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0888DC: lea.l $2ed8dc.l, a1
pub fn i_0888dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x888E2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED8DC;
    20
}
/// $0888E2: move.l a1, $74(a0)
pub fn i_0888e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x888E6;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0888E6: move.w #$4, $72(a0)
pub fn i_0888e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x888EC;
    cpu.ir = 0x317C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0888EC: lea.l $86d0c(pc), a1
pub fn i_0888ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x888F0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86D0C;
    16
}
/// $0888F0: jsr $4ae.l
pub fn i_0888f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x888F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x888F6);
    cpu.pc = 0x4AE;
    16
}
/// $0888F6: jsr $5dd22.l
pub fn i_0888f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x888FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x888FC);
    cpu.pc = 0x5DD22;
    16
}
/// $0888FC: addi.w #$a0, $22(a0)
pub fn i_0888fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88902;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xA0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $088902: addi.w #$8, $24(a0)
pub fn i_088902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88908;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $088908: lea.l $2ed8f0.l, a1
pub fn i_088908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8890E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED8F0;
    20
}
/// $08890E: move.l a1, $74(a0)
pub fn i_08890e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88912;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x086B90, i_086b90),
    (0x086B96, i_086b96),
    (0x086B9C, i_086b9c),
    (0x086BA2, i_086ba2),
    (0x086BA4, i_086ba4),
    (0x086BA8, i_086ba8),
    (0x086BAE, i_086bae),
    (0x086BB4, i_086bb4),
    (0x086BB8, i_086bb8),
    (0x086BBA, i_086bba),
    (0x086BBE, i_086bbe),
    (0x086BC4, i_086bc4),
    (0x086BC8, i_086bc8),
    (0x086BCE, i_086bce),
    (0x086BD4, i_086bd4),
    (0x086BD8, i_086bd8),
    (0x086BDC, i_086bdc),
    (0x086BE0, i_086be0),
    (0x086BE4, i_086be4),
    (0x086BEA, i_086bea),
    (0x086BF0, i_086bf0),
    (0x086BF4, i_086bf4),
    (0x086BFA, i_086bfa),
    (0x086C00, i_086c00),
    (0x086C06, i_086c06),
    (0x086C0C, i_086c0c),
    (0x086C12, i_086c12),
    (0x086C16, i_086c16),
    (0x086C1C, i_086c1c),
    (0x086C22, i_086c22),
    (0x086C28, i_086c28),
    (0x086C2E, i_086c2e),
    (0x086C32, i_086c32),
    (0x086C34, i_086c34),
    (0x086C3A, i_086c3a),
    (0x086C3E, i_086c3e),
    (0x086C42, i_086c42),
    (0x086C48, i_086c48),
    (0x086C4E, i_086c4e),
    (0x086C54, i_086c54),
    (0x086C5A, i_086c5a),
    (0x086C60, i_086c60),
    (0x086C62, i_086c62),
    (0x086C66, i_086c66),
    (0x086C6C, i_086c6c),
    (0x086C72, i_086c72),
    (0x086C78, i_086c78),
    (0x086C7E, i_086c7e),
    (0x086C84, i_086c84),
    (0x086C8A, i_086c8a),
    (0x086C90, i_086c90),
    (0x086C96, i_086c96),
    (0x086C9A, i_086c9a),
    (0x086C9C, i_086c9c),
    (0x086CA0, i_086ca0),
    (0x086CA4, i_086ca4),
    (0x086CAA, i_086caa),
    (0x086CAE, i_086cae),
    (0x086CB4, i_086cb4),
    (0x086CBA, i_086cba),
    (0x086CBE, i_086cbe),
    (0x086CC2, i_086cc2),
    (0x086CC8, i_086cc8),
    (0x086CCE, i_086cce),
    (0x086CD0, i_086cd0),
    (0x086CD4, i_086cd4),
    (0x086CDA, i_086cda),
    (0x086CE0, i_086ce0),
    (0x086CE4, i_086ce4),
    (0x086CE8, i_086ce8),
    (0x086CEE, i_086cee),
    (0x086CF4, i_086cf4),
    (0x086CF8, i_086cf8),
    (0x086CFC, i_086cfc),
    (0x086D02, i_086d02),
    (0x086D08, i_086d08),
    (0x086D0C, i_086d0c),
    (0x086D10, i_086d10),
    (0x086D16, i_086d16),
    (0x086D1C, i_086d1c),
    (0x086D20, i_086d20),
    (0x086D24, i_086d24),
    (0x086D2A, i_086d2a),
    (0x086D30, i_086d30),
    (0x086D34, i_086d34),
    (0x086D3A, i_086d3a),
    (0x086D40, i_086d40),
    (0x086D46, i_086d46),
    (0x086D4C, i_086d4c),
    (0x086D52, i_086d52),
    (0x086D56, i_086d56),
    (0x086D58, i_086d58),
    (0x086D5C, i_086d5c),
    (0x086D62, i_086d62),
    (0x086D66, i_086d66),
    (0x086D6C, i_086d6c),
    (0x086D72, i_086d72),
    (0x086D76, i_086d76),
    (0x086D7C, i_086d7c),
    (0x086D82, i_086d82),
    (0x086D88, i_086d88),
    (0x086D8C, i_086d8c),
    (0x086D92, i_086d92),
    (0x086D96, i_086d96),
    (0x086D9A, i_086d9a),
    (0x086D9E, i_086d9e),
    (0x086DA2, i_086da2),
    (0x086DA4, i_086da4),
    (0x086DA8, i_086da8),
    (0x086DAE, i_086dae),
    (0x086DB2, i_086db2),
    (0x086DB4, i_086db4),
    (0x086DBA, i_086dba),
    (0x086DC0, i_086dc0),
    (0x086DC4, i_086dc4),
    (0x086DCA, i_086dca),
    (0x086DCC, i_086dcc),
    (0x086DD0, i_086dd0),
    (0x086DD6, i_086dd6),
    (0x086DD8, i_086dd8),
    (0x086DDC, i_086ddc),
    (0x086DE2, i_086de2),
    (0x086DE6, i_086de6),
    (0x086DEC, i_086dec),
    (0x086DF0, i_086df0),
    (0x086DF2, i_086df2),
    (0x086DF6, i_086df6),
    (0x086DFC, i_086dfc),
    (0x086E00, i_086e00),
    (0x086E06, i_086e06),
    (0x086E0C, i_086e0c),
    (0x086E10, i_086e10),
    (0x086E14, i_086e14),
    (0x086E1A, i_086e1a),
    (0x086E20, i_086e20),
    (0x086E26, i_086e26),
    (0x086E2C, i_086e2c),
    (0x086E30, i_086e30),
    (0x086E36, i_086e36),
    (0x086E3C, i_086e3c),
    (0x086E42, i_086e42),
    (0x086E48, i_086e48),
    (0x086E4A, i_086e4a),
    (0x086E50, i_086e50),
    (0x086E56, i_086e56),
    (0x086E5C, i_086e5c),
    (0x086E62, i_086e62),
    (0x086E66, i_086e66),
    (0x086E6C, i_086e6c),
    (0x086E70, i_086e70),
    (0x086E74, i_086e74),
    (0x086E7A, i_086e7a),
    (0x086E80, i_086e80),
    (0x086E86, i_086e86),
    (0x086E8C, i_086e8c),
    (0x086E90, i_086e90),
    (0x086E96, i_086e96),
    (0x086E9A, i_086e9a),
    (0x086E9E, i_086e9e),
    (0x086EA4, i_086ea4),
    (0x086EAA, i_086eaa),
    (0x086EB0, i_086eb0),
    (0x086EB6, i_086eb6),
    (0x086EBA, i_086eba),
    (0x086EC0, i_086ec0),
    (0x086EC4, i_086ec4),
    (0x086EC8, i_086ec8),
    (0x086ECE, i_086ece),
    (0x086ED4, i_086ed4),
    (0x086EDA, i_086eda),
    (0x086EE0, i_086ee0),
    (0x086EE4, i_086ee4),
    (0x086EEA, i_086eea),
    (0x086EEE, i_086eee),
    (0x086EF2, i_086ef2),
    (0x086EF8, i_086ef8),
    (0x086EFE, i_086efe),
    (0x086F04, i_086f04),
    (0x086F0A, i_086f0a),
    (0x086F0E, i_086f0e),
    (0x086F14, i_086f14),
    (0x086F18, i_086f18),
    (0x086F1C, i_086f1c),
    (0x086F20, i_086f20),
    (0x086F26, i_086f26),
    (0x086F2C, i_086f2c),
    (0x086F32, i_086f32),
    (0x086F38, i_086f38),
    (0x086F3C, i_086f3c),
    (0x086F40, i_086f40),
    (0x086F44, i_086f44),
    (0x086F4A, i_086f4a),
    (0x086F50, i_086f50),
    (0x086F56, i_086f56),
    (0x086F5C, i_086f5c),
    (0x086F60, i_086f60),
    (0x086F64, i_086f64),
    (0x086F6A, i_086f6a),
    (0x086F6E, i_086f6e),
    (0x086F70, i_086f70),
    (0x086F76, i_086f76),
    (0x086F7A, i_086f7a),
    (0x086F7E, i_086f7e),
    (0x086F82, i_086f82),
    (0x086F84, i_086f84),
    (0x086F8A, i_086f8a),
    (0x086F8E, i_086f8e),
    (0x086F92, i_086f92),
    (0x086F98, i_086f98),
    (0x086F9C, i_086f9c),
    (0x086FA0, i_086fa0),
    (0x086FA2, i_086fa2),
    (0x086FA8, i_086fa8),
    (0x086FAC, i_086fac),
    (0x086FB2, i_086fb2),
    (0x086FB4, i_086fb4),
    (0x086FB6, i_086fb6),
    (0x086FBA, i_086fba),
    (0x086FC0, i_086fc0),
    (0x086FC2, i_086fc2),
    (0x086FC6, i_086fc6),
    (0x086FCC, i_086fcc),
    (0x086FD0, i_086fd0),
    (0x086FD6, i_086fd6),
    (0x086FDA, i_086fda),
    (0x086FDC, i_086fdc),
    (0x086FE2, i_086fe2),
    (0x086FE8, i_086fe8),
    (0x086FEC, i_086fec),
    (0x086FF0, i_086ff0),
    (0x086FF2, i_086ff2),
    (0x086FF8, i_086ff8),
    (0x086FFC, i_086ffc),
    (0x087002, i_087002),
    (0x087004, i_087004),
    (0x08700A, i_08700a),
    (0x08700E, i_08700e),
    (0x087012, i_087012),
    (0x087018, i_087018),
    (0x08701E, i_08701e),
    (0x087022, i_087022),
    (0x087024, i_087024),
    (0x08702A, i_08702a),
    (0x08702E, i_08702e),
    (0x087032, i_087032),
    (0x087036, i_087036),
    (0x087038, i_087038),
    (0x08703E, i_08703e),
    (0x087042, i_087042),
    (0x087046, i_087046),
    (0x08704C, i_08704c),
    (0x087050, i_087050),
    (0x087054, i_087054),
    (0x087056, i_087056),
    (0x08705C, i_08705c),
    (0x087060, i_087060),
    (0x087066, i_087066),
    (0x087068, i_087068),
    (0x08706E, i_08706e),
    (0x087072, i_087072),
    (0x087074, i_087074),
    (0x08707A, i_08707a),
    (0x08707E, i_08707e),
    (0x087082, i_087082),
    (0x087086, i_087086),
    (0x087088, i_087088),
    (0x08708E, i_08708e),
    (0x087092, i_087092),
    (0x087096, i_087096),
    (0x08709C, i_08709c),
    (0x0870A0, i_0870a0),
    (0x0870A4, i_0870a4),
    (0x0870A6, i_0870a6),
    (0x0870AC, i_0870ac),
    (0x0870B0, i_0870b0),
    (0x0870B6, i_0870b6),
    (0x0870B8, i_0870b8),
    (0x0870BA, i_0870ba),
    (0x0870BE, i_0870be),
    (0x0870C4, i_0870c4),
    (0x0870C6, i_0870c6),
    (0x0870CA, i_0870ca),
    (0x0870D0, i_0870d0),
    (0x0870D4, i_0870d4),
    (0x0870DA, i_0870da),
    (0x0870DE, i_0870de),
    (0x0870E0, i_0870e0),
    (0x0870E6, i_0870e6),
    (0x0870EC, i_0870ec),
    (0x0870F0, i_0870f0),
    (0x0870F4, i_0870f4),
    (0x0870F6, i_0870f6),
    (0x0870FC, i_0870fc),
    (0x087100, i_087100),
    (0x087106, i_087106),
    (0x087108, i_087108),
    (0x08710A, i_08710a),
    (0x08710E, i_08710e),
    (0x087114, i_087114),
    (0x087116, i_087116),
    (0x08711A, i_08711a),
    (0x087120, i_087120),
    (0x087124, i_087124),
    (0x08712A, i_08712a),
    (0x08712E, i_08712e),
    (0x087130, i_087130),
    (0x087136, i_087136),
    (0x08713C, i_08713c),
    (0x087140, i_087140),
    (0x087144, i_087144),
    (0x087148, i_087148),
    (0x08714E, i_08714e),
    (0x087154, i_087154),
    (0x087158, i_087158),
    (0x08715E, i_08715e),
    (0x087162, i_087162),
    (0x087168, i_087168),
    (0x08716A, i_08716a),
    (0x08716E, i_08716e),
    (0x087174, i_087174),
    (0x08717A, i_08717a),
    (0x087180, i_087180),
    (0x087184, i_087184),
    (0x087188, i_087188),
    (0x08718A, i_08718a),
    (0x087190, i_087190),
    (0x087196, i_087196),
    (0x08719A, i_08719a),
    (0x0871A0, i_0871a0),
    (0x0871A6, i_0871a6),
    (0x0871AC, i_0871ac),
    (0x0871B2, i_0871b2),
    (0x0871B6, i_0871b6),
    (0x0871BA, i_0871ba),
    (0x0871C0, i_0871c0),
    (0x0871C6, i_0871c6),
    (0x0871CC, i_0871cc),
    (0x0871D2, i_0871d2),
    (0x0871D8, i_0871d8),
    (0x0871DE, i_0871de),
    (0x0871E4, i_0871e4),
    (0x0871E8, i_0871e8),
    (0x0871EE, i_0871ee),
    (0x0871F2, i_0871f2),
    (0x0871F8, i_0871f8),
    (0x0871FA, i_0871fa),
    (0x0871FE, i_0871fe),
    (0x087204, i_087204),
    (0x08720A, i_08720a),
    (0x087210, i_087210),
    (0x087214, i_087214),
    (0x087218, i_087218),
    (0x08721A, i_08721a),
    (0x087220, i_087220),
    (0x087226, i_087226),
    (0x08722A, i_08722a),
    (0x087230, i_087230),
    (0x087236, i_087236),
    (0x08723C, i_08723c),
    (0x087242, i_087242),
    (0x087246, i_087246),
    (0x08724C, i_08724c),
    (0x087252, i_087252),
    (0x087258, i_087258),
    (0x08725E, i_08725e),
    (0x087264, i_087264),
    (0x08726A, i_08726a),
    (0x08726E, i_08726e),
    (0x087274, i_087274),
    (0x087278, i_087278),
    (0x08727E, i_08727e),
    (0x087280, i_087280),
    (0x087284, i_087284),
    (0x08728A, i_08728a),
    (0x087290, i_087290),
    (0x087296, i_087296),
    (0x08729A, i_08729a),
    (0x08729E, i_08729e),
    (0x0872A0, i_0872a0),
    (0x0872A6, i_0872a6),
    (0x0872AC, i_0872ac),
    (0x0872B0, i_0872b0),
    (0x0872B6, i_0872b6),
    (0x0872BC, i_0872bc),
    (0x0872C2, i_0872c2),
    (0x0872C8, i_0872c8),
    (0x0872CC, i_0872cc),
    (0x0872D2, i_0872d2),
    (0x0872D8, i_0872d8),
    (0x0872DE, i_0872de),
    (0x0872E4, i_0872e4),
    (0x0872E8, i_0872e8),
    (0x0872EE, i_0872ee),
    (0x0872F2, i_0872f2),
    (0x0872F8, i_0872f8),
    (0x0872FA, i_0872fa),
    (0x0872FE, i_0872fe),
    (0x087304, i_087304),
    (0x08730A, i_08730a),
    (0x087310, i_087310),
    (0x087316, i_087316),
    (0x08731A, i_08731a),
    (0x08731C, i_08731c),
    (0x087322, i_087322),
    (0x087328, i_087328),
    (0x08732E, i_08732e),
    (0x087332, i_087332),
    (0x087336, i_087336),
    (0x08733C, i_08733c),
    (0x087342, i_087342),
    (0x087348, i_087348),
    (0x08734E, i_08734e),
    (0x087354, i_087354),
    (0x08735A, i_08735a),
    (0x08735E, i_08735e),
    (0x087364, i_087364),
    (0x087366, i_087366),
    (0x08736A, i_08736a),
    (0x087370, i_087370),
    (0x087374, i_087374),
    (0x08737A, i_08737a),
    (0x087380, i_087380),
    (0x087386, i_087386),
    (0x08738C, i_08738c),
    (0x087390, i_087390),
    (0x087396, i_087396),
    (0x08739C, i_08739c),
    (0x0873A2, i_0873a2),
    (0x0873A8, i_0873a8),
    (0x0873AC, i_0873ac),
    (0x0873B2, i_0873b2),
    (0x0873B8, i_0873b8),
    (0x0873BE, i_0873be),
    (0x0873C4, i_0873c4),
    (0x0873C8, i_0873c8),
    (0x0873CE, i_0873ce),
    (0x0873D4, i_0873d4),
    (0x0873DA, i_0873da),
    (0x0873E0, i_0873e0),
    (0x0873E4, i_0873e4),
    (0x0873EA, i_0873ea),
    (0x0873F0, i_0873f0),
    (0x0873F6, i_0873f6),
    (0x0873FC, i_0873fc),
    (0x087400, i_087400),
    (0x087406, i_087406),
    (0x08740C, i_08740c),
    (0x087412, i_087412),
    (0x087418, i_087418),
    (0x08741C, i_08741c),
    (0x087422, i_087422),
    (0x087428, i_087428),
    (0x08742E, i_08742e),
    (0x087434, i_087434),
    (0x087438, i_087438),
    (0x08743E, i_08743e),
    (0x087444, i_087444),
    (0x087448, i_087448),
    (0x08744E, i_08744e),
    (0x087454, i_087454),
    (0x08745A, i_08745a),
    (0x08745E, i_08745e),
    (0x087464, i_087464),
    (0x08746A, i_08746a),
    (0x087470, i_087470),
    (0x087474, i_087474),
    (0x08747A, i_08747a),
    (0x087480, i_087480),
    (0x087486, i_087486),
    (0x08748A, i_08748a),
    (0x087490, i_087490),
    (0x087496, i_087496),
    (0x08749C, i_08749c),
    (0x0874A0, i_0874a0),
    (0x0874A6, i_0874a6),
    (0x0874AC, i_0874ac),
    (0x0874B2, i_0874b2),
    (0x0874B8, i_0874b8),
    (0x0874BA, i_0874ba),
    (0x0874C0, i_0874c0),
    (0x0874C4, i_0874c4),
    (0x0874CC, i_0874cc),
    (0x0874D2, i_0874d2),
    (0x0874D8, i_0874d8),
    (0x0874DC, i_0874dc),
    (0x0874E2, i_0874e2),
    (0x0874E6, i_0874e6),
    (0x0874EA, i_0874ea),
    (0x0874F0, i_0874f0),
    (0x0874F4, i_0874f4),
    (0x0874FA, i_0874fa),
    (0x0874FE, i_0874fe),
    (0x087504, i_087504),
    (0x08750A, i_08750a),
    (0x08750E, i_08750e),
    (0x087514, i_087514),
    (0x087518, i_087518),
    (0x08751C, i_08751c),
    (0x087522, i_087522),
    (0x087526, i_087526),
    (0x08752C, i_08752c),
    (0x087530, i_087530),
    (0x087536, i_087536),
    (0x08753C, i_08753c),
    (0x087540, i_087540),
    (0x087546, i_087546),
    (0x08754A, i_08754a),
    (0x08754E, i_08754e),
    (0x087554, i_087554),
    (0x087558, i_087558),
    (0x08755E, i_08755e),
    (0x087562, i_087562),
    (0x087568, i_087568),
    (0x08756E, i_08756e),
    (0x087572, i_087572),
    (0x087578, i_087578),
    (0x08757C, i_08757c),
    (0x087580, i_087580),
    (0x087586, i_087586),
    (0x08758A, i_08758a),
    (0x087592, i_087592),
    (0x087598, i_087598),
    (0x08759E, i_08759e),
    (0x0875A2, i_0875a2),
    (0x0875A8, i_0875a8),
    (0x0875AC, i_0875ac),
    (0x0875B0, i_0875b0),
    (0x0875B6, i_0875b6),
    (0x0875BA, i_0875ba),
    (0x0875C2, i_0875c2),
    (0x0875C8, i_0875c8),
    (0x0875CE, i_0875ce),
    (0x0875D2, i_0875d2),
    (0x0875D8, i_0875d8),
    (0x0875DC, i_0875dc),
    (0x0875E0, i_0875e0),
    (0x0875E6, i_0875e6),
    (0x0875EA, i_0875ea),
    (0x0875F2, i_0875f2),
    (0x0875F8, i_0875f8),
    (0x0875FE, i_0875fe),
    (0x087602, i_087602),
    (0x087608, i_087608),
    (0x08760C, i_08760c),
    (0x087610, i_087610),
    (0x087616, i_087616),
    (0x08761A, i_08761a),
    (0x087622, i_087622),
    (0x087628, i_087628),
    (0x08762E, i_08762e),
    (0x087632, i_087632),
    (0x087638, i_087638),
    (0x08763C, i_08763c),
    (0x087640, i_087640),
    (0x087646, i_087646),
    (0x08764A, i_08764a),
    (0x087652, i_087652),
    (0x087658, i_087658),
    (0x08765E, i_08765e),
    (0x087662, i_087662),
    (0x087668, i_087668),
    (0x08766C, i_08766c),
    (0x087670, i_087670),
    (0x087676, i_087676),
    (0x08767A, i_08767a),
    (0x087682, i_087682),
    (0x087688, i_087688),
    (0x08768E, i_08768e),
    (0x087692, i_087692),
    (0x087698, i_087698),
    (0x08769C, i_08769c),
    (0x0876A0, i_0876a0),
    (0x0876A6, i_0876a6),
    (0x0876AA, i_0876aa),
    (0x0876B2, i_0876b2),
    (0x0876B8, i_0876b8),
    (0x0876BE, i_0876be),
    (0x0876C2, i_0876c2),
    (0x0876C8, i_0876c8),
    (0x0876CC, i_0876cc),
    (0x0876D0, i_0876d0),
    (0x0876D6, i_0876d6),
    (0x0876DA, i_0876da),
    (0x0876E2, i_0876e2),
    (0x0876E8, i_0876e8),
    (0x0876EE, i_0876ee),
    (0x0876F2, i_0876f2),
    (0x0876F8, i_0876f8),
    (0x0876FC, i_0876fc),
    (0x087700, i_087700),
    (0x087706, i_087706),
    (0x08770A, i_08770a),
    (0x087712, i_087712),
    (0x087718, i_087718),
    (0x08771E, i_08771e),
    (0x087722, i_087722),
    (0x087728, i_087728),
    (0x08772C, i_08772c),
    (0x087730, i_087730),
    (0x087736, i_087736),
    (0x08773C, i_08773c),
    (0x087742, i_087742),
    (0x087746, i_087746),
    (0x087748, i_087748),
    (0x08774E, i_08774e),
    (0x087754, i_087754),
    (0x087758, i_087758),
    (0x08775E, i_08775e),
    (0x087764, i_087764),
    (0x08776A, i_08776a),
    (0x087770, i_087770),
    (0x087774, i_087774),
    (0x087778, i_087778),
    (0x08777C, i_08777c),
    (0x087780, i_087780),
    (0x087786, i_087786),
    (0x08778A, i_08778a),
    (0x087790, i_087790),
    (0x087794, i_087794),
    (0x08779A, i_08779a),
    (0x0877A2, i_0877a2),
    (0x0877A6, i_0877a6),
    (0x0877AA, i_0877aa),
    (0x0877AE, i_0877ae),
    (0x0877B4, i_0877b4),
    (0x0877BA, i_0877ba),
    (0x0877BE, i_0877be),
    (0x0877C4, i_0877c4),
    (0x0877C8, i_0877c8),
    (0x0877CE, i_0877ce),
    (0x0877D2, i_0877d2),
    (0x0877D4, i_0877d4),
    (0x0877D8, i_0877d8),
    (0x0877DE, i_0877de),
    (0x0877E2, i_0877e2),
    (0x0877E8, i_0877e8),
    (0x0877EE, i_0877ee),
    (0x0877F4, i_0877f4),
    (0x0877FA, i_0877fa),
    (0x087800, i_087800),
    (0x087806, i_087806),
    (0x08780C, i_08780c),
    (0x087812, i_087812),
    (0x087816, i_087816),
    (0x08781C, i_08781c),
    (0x087822, i_087822),
    (0x087826, i_087826),
    (0x087828, i_087828),
    (0x08782E, i_08782e),
    (0x087832, i_087832),
    (0x087838, i_087838),
    (0x08783E, i_08783e),
    (0x087844, i_087844),
    (0x087846, i_087846),
    (0x08784C, i_08784c),
    (0x087852, i_087852),
    (0x087858, i_087858),
    (0x08785E, i_08785e),
    (0x087862, i_087862),
    (0x087868, i_087868),
    (0x087870, i_087870),
    (0x087874, i_087874),
    (0x087876, i_087876),
    (0x08787C, i_08787c),
    (0x087882, i_087882),
    (0x087888, i_087888),
    (0x08788E, i_08788e),
    (0x087894, i_087894),
    (0x087898, i_087898),
    (0x08789E, i_08789e),
    (0x0878A4, i_0878a4),
    (0x0878AA, i_0878aa),
    (0x0878B0, i_0878b0),
    (0x0878B4, i_0878b4),
    (0x0878BA, i_0878ba),
    (0x0878C0, i_0878c0),
    (0x0878C4, i_0878c4),
    (0x0878C6, i_0878c6),
    (0x0878CC, i_0878cc),
    (0x0878D0, i_0878d0),
    (0x0878D6, i_0878d6),
    (0x0878D8, i_0878d8),
    (0x0878DC, i_0878dc),
    (0x0878E2, i_0878e2),
    (0x0878E8, i_0878e8),
    (0x0878EE, i_0878ee),
    (0x0878F2, i_0878f2),
    (0x0878F4, i_0878f4),
    (0x0878FA, i_0878fa),
    (0x087900, i_087900),
    (0x087906, i_087906),
    (0x08790C, i_08790c),
    (0x087912, i_087912),
    (0x087916, i_087916),
    (0x08791C, i_08791c),
    (0x087922, i_087922),
    (0x087928, i_087928),
    (0x08792E, i_08792e),
    (0x087932, i_087932),
    (0x087938, i_087938),
    (0x08793E, i_08793e),
    (0x087942, i_087942),
    (0x087944, i_087944),
    (0x08794A, i_08794a),
    (0x08794E, i_08794e),
    (0x087954, i_087954),
    (0x087956, i_087956),
    (0x08795A, i_08795a),
    (0x087960, i_087960),
    (0x087966, i_087966),
    (0x08796C, i_08796c),
    (0x087970, i_087970),
    (0x087972, i_087972),
    (0x087978, i_087978),
    (0x08797E, i_08797e),
    (0x087984, i_087984),
    (0x08798A, i_08798a),
    (0x087990, i_087990),
    (0x087994, i_087994),
    (0x08799A, i_08799a),
    (0x0879A0, i_0879a0),
    (0x0879A6, i_0879a6),
    (0x0879AC, i_0879ac),
    (0x0879B0, i_0879b0),
    (0x0879B4, i_0879b4),
    (0x0879BA, i_0879ba),
    (0x0879C0, i_0879c0),
    (0x0879C6, i_0879c6),
    (0x0879CC, i_0879cc),
    (0x0879D2, i_0879d2),
    (0x0879D8, i_0879d8),
    (0x0879DE, i_0879de),
    (0x0879E4, i_0879e4),
    (0x0879EA, i_0879ea),
    (0x0879F0, i_0879f0),
    (0x0879F4, i_0879f4),
    (0x0879FA, i_0879fa),
    (0x087A00, i_087a00),
    (0x087A04, i_087a04),
    (0x087A06, i_087a06),
    (0x087A0C, i_087a0c),
    (0x087A10, i_087a10),
    (0x087A16, i_087a16),
    (0x087A18, i_087a18),
    (0x087A1E, i_087a1e),
    (0x087A24, i_087a24),
    (0x087A28, i_087a28),
    (0x087A2C, i_087a2c),
    (0x087A32, i_087a32),
    (0x087A38, i_087a38),
    (0x087A3C, i_087a3c),
    (0x087A3E, i_087a3e),
    (0x087A44, i_087a44),
    (0x087A4A, i_087a4a),
    (0x087A50, i_087a50),
    (0x087A56, i_087a56),
    (0x087A5A, i_087a5a),
    (0x087A60, i_087a60),
    (0x087A62, i_087a62),
    (0x087A68, i_087a68),
    (0x087A6C, i_087a6c),
    (0x087A6E, i_087a6e),
    (0x087A74, i_087a74),
    (0x087A7A, i_087a7a),
    (0x087A7E, i_087a7e),
    (0x087A84, i_087a84),
    (0x087A8A, i_087a8a),
    (0x087A90, i_087a90),
    (0x087A94, i_087a94),
    (0x087A98, i_087a98),
    (0x087A9E, i_087a9e),
    (0x087AA0, i_087aa0),
    (0x087AA4, i_087aa4),
    (0x087AAA, i_087aaa),
    (0x087AB0, i_087ab0),
    (0x087AB6, i_087ab6),
    (0x087ABC, i_087abc),
    (0x087AC0, i_087ac0),
    (0x087AC4, i_087ac4),
    (0x087ACA, i_087aca),
    (0x087ACE, i_087ace),
    (0x087AD0, i_087ad0),
    (0x087AD4, i_087ad4),
    (0x087ADA, i_087ada),
    (0x087AE0, i_087ae0),
    (0x087AE2, i_087ae2),
    (0x087AE6, i_087ae6),
    (0x087AEA, i_087aea),
    (0x087AEE, i_087aee),
    (0x087AF4, i_087af4),
    (0x087AF6, i_087af6),
    (0x087AFA, i_087afa),
    (0x087B00, i_087b00),
    (0x087B04, i_087b04),
    (0x087B0A, i_087b0a),
    (0x087B10, i_087b10),
    (0x087B14, i_087b14),
    (0x087B1A, i_087b1a),
    (0x087B1C, i_087b1c),
    (0x087B20, i_087b20),
    (0x087B26, i_087b26),
    (0x087B2C, i_087b2c),
    (0x087B30, i_087b30),
    (0x087B36, i_087b36),
    (0x087B3A, i_087b3a),
    (0x087B40, i_087b40),
    (0x087B46, i_087b46),
    (0x087B4C, i_087b4c),
    (0x087B52, i_087b52),
    (0x087B58, i_087b58),
    (0x087B5E, i_087b5e),
    (0x087B64, i_087b64),
    (0x087B68, i_087b68),
    (0x087B6E, i_087b6e),
    (0x087B74, i_087b74),
    (0x087B7A, i_087b7a),
    (0x087B80, i_087b80),
    (0x087B86, i_087b86),
    (0x087B8C, i_087b8c),
    (0x087B90, i_087b90),
    (0x087B96, i_087b96),
    (0x087B9C, i_087b9c),
    (0x087BA2, i_087ba2),
    (0x087BA8, i_087ba8),
    (0x087BB0, i_087bb0),
    (0x087BB4, i_087bb4),
    (0x087BB6, i_087bb6),
    (0x087BBC, i_087bbc),
    (0x087BC2, i_087bc2),
    (0x087BC6, i_087bc6),
    (0x087BCC, i_087bcc),
    (0x087BD2, i_087bd2),
    (0x087BD6, i_087bd6),
    (0x087BDC, i_087bdc),
    (0x087BE2, i_087be2),
    (0x087BE8, i_087be8),
    (0x087BEE, i_087bee),
    (0x087BF4, i_087bf4),
    (0x087BFA, i_087bfa),
    (0x087C00, i_087c00),
    (0x087C04, i_087c04),
    (0x087C08, i_087c08),
    (0x087C0A, i_087c0a),
    (0x087C10, i_087c10),
    (0x087C14, i_087c14),
    (0x087C1A, i_087c1a),
    (0x087C20, i_087c20),
    (0x087C22, i_087c22),
    (0x087C28, i_087c28),
    (0x087C2E, i_087c2e),
    (0x087C32, i_087c32),
    (0x087C34, i_087c34),
    (0x087C3A, i_087c3a),
    (0x087C40, i_087c40),
    (0x087C44, i_087c44),
    (0x087C4A, i_087c4a),
    (0x087C4E, i_087c4e),
    (0x087C52, i_087c52),
    (0x087C54, i_087c54),
    (0x087C5A, i_087c5a),
    (0x087C5E, i_087c5e),
    (0x087C64, i_087c64),
    (0x087C6A, i_087c6a),
    (0x087C70, i_087c70),
    (0x087C76, i_087c76),
    (0x087C7C, i_087c7c),
    (0x087C82, i_087c82),
    (0x087C88, i_087c88),
    (0x087C8C, i_087c8c),
    (0x087C92, i_087c92),
    (0x087C98, i_087c98),
    (0x087C9A, i_087c9a),
    (0x087CA0, i_087ca0),
    (0x087CA4, i_087ca4),
    (0x087CAA, i_087caa),
    (0x087CB0, i_087cb0),
    (0x087CB4, i_087cb4),
    (0x087CB6, i_087cb6),
    (0x087CBC, i_087cbc),
    (0x087CC2, i_087cc2),
    (0x087CC6, i_087cc6),
    (0x087CCC, i_087ccc),
    (0x087CD2, i_087cd2),
    (0x087CD6, i_087cd6),
    (0x087CDC, i_087cdc),
    (0x087CE2, i_087ce2),
    (0x087CE8, i_087ce8),
    (0x087CEE, i_087cee),
    (0x087CF4, i_087cf4),
    (0x087CF8, i_087cf8),
    (0x087CFC, i_087cfc),
    (0x087CFE, i_087cfe),
    (0x087D04, i_087d04),
    (0x087D08, i_087d08),
    (0x087D0E, i_087d0e),
    (0x087D14, i_087d14),
    (0x087D1A, i_087d1a),
    (0x087D1C, i_087d1c),
    (0x087D20, i_087d20),
    (0x087D26, i_087d26),
    (0x087D2C, i_087d2c),
    (0x087D32, i_087d32),
    (0x087D36, i_087d36),
    (0x087D38, i_087d38),
    (0x087D3E, i_087d3e),
    (0x087D44, i_087d44),
    (0x087D48, i_087d48),
    (0x087D4E, i_087d4e),
    (0x087D54, i_087d54),
    (0x087D58, i_087d58),
    (0x087D5E, i_087d5e),
    (0x087D64, i_087d64),
    (0x087D6A, i_087d6a),
    (0x087D70, i_087d70),
    (0x087D76, i_087d76),
    (0x087D7A, i_087d7a),
    (0x087D7E, i_087d7e),
    (0x087D80, i_087d80),
    (0x087D86, i_087d86),
    (0x087D8A, i_087d8a),
    (0x087D90, i_087d90),
    (0x087D96, i_087d96),
    (0x087D9C, i_087d9c),
    (0x087D9E, i_087d9e),
    (0x087DA2, i_087da2),
    (0x087DA8, i_087da8),
    (0x087DAE, i_087dae),
    (0x087DB4, i_087db4),
    (0x087DB8, i_087db8),
    (0x087DBA, i_087dba),
    (0x087DC0, i_087dc0),
    (0x087DC6, i_087dc6),
    (0x087DCA, i_087dca),
    (0x087DD0, i_087dd0),
    (0x087DD6, i_087dd6),
    (0x087DDA, i_087dda),
    (0x087DE0, i_087de0),
    (0x087DE6, i_087de6),
    (0x087DEC, i_087dec),
    (0x087DF2, i_087df2),
    (0x087DF8, i_087df8),
    (0x087DFC, i_087dfc),
    (0x087E00, i_087e00),
    (0x087E02, i_087e02),
    (0x087E08, i_087e08),
    (0x087E0C, i_087e0c),
    (0x087E12, i_087e12),
    (0x087E18, i_087e18),
    (0x087E1E, i_087e1e),
    (0x087E20, i_087e20),
    (0x087E26, i_087e26),
    (0x087E2C, i_087e2c),
    (0x087E30, i_087e30),
    (0x087E36, i_087e36),
    (0x087E3C, i_087e3c),
    (0x087E42, i_087e42),
    (0x087E48, i_087e48),
    (0x087E4E, i_087e4e),
    (0x087E54, i_087e54),
    (0x087E5A, i_087e5a),
    (0x087E60, i_087e60),
    (0x087E66, i_087e66),
    (0x087E6C, i_087e6c),
    (0x087E72, i_087e72),
    (0x087E78, i_087e78),
    (0x087E7E, i_087e7e),
    (0x087E84, i_087e84),
    (0x087E8A, i_087e8a),
    (0x087E8E, i_087e8e),
    (0x087E90, i_087e90),
    (0x087E96, i_087e96),
    (0x087E9C, i_087e9c),
    (0x087EA2, i_087ea2),
    (0x087EA6, i_087ea6),
    (0x087EAC, i_087eac),
    (0x087EAE, i_087eae),
    (0x087EB2, i_087eb2),
    (0x087EB8, i_087eb8),
    (0x087EBE, i_087ebe),
    (0x087EC4, i_087ec4),
    (0x087ECA, i_087eca),
    (0x087ED0, i_087ed0),
    (0x087ED6, i_087ed6),
    (0x087EDC, i_087edc),
    (0x087EE2, i_087ee2),
    (0x087EE8, i_087ee8),
    (0x087EEE, i_087eee),
    (0x087EF4, i_087ef4),
    (0x087EFA, i_087efa),
    (0x087F00, i_087f00),
    (0x087F06, i_087f06),
    (0x087F0A, i_087f0a),
    (0x087F0C, i_087f0c),
    (0x087F12, i_087f12),
    (0x087F18, i_087f18),
    (0x087F1C, i_087f1c),
    (0x087F22, i_087f22),
    (0x087F26, i_087f26),
    (0x087F2C, i_087f2c),
    (0x087F32, i_087f32),
    (0x087F38, i_087f38),
    (0x087F3E, i_087f3e),
    (0x087F44, i_087f44),
    (0x087F4A, i_087f4a),
    (0x087F4C, i_087f4c),
    (0x087F50, i_087f50),
    (0x087F56, i_087f56),
    (0x087F5C, i_087f5c),
    (0x087F60, i_087f60),
    (0x087F66, i_087f66),
    (0x087F6A, i_087f6a),
    (0x087F6E, i_087f6e),
    (0x087F70, i_087f70),
    (0x087F74, i_087f74),
    (0x087F78, i_087f78),
    (0x087F7C, i_087f7c),
    (0x087F82, i_087f82),
    (0x087F88, i_087f88),
    (0x087F8C, i_087f8c),
    (0x087F8E, i_087f8e),
    (0x087F94, i_087f94),
    (0x087F9A, i_087f9a),
    (0x087F9E, i_087f9e),
    (0x087FA4, i_087fa4),
    (0x087FAA, i_087faa),
    (0x087FB0, i_087fb0),
    (0x087FB4, i_087fb4),
    (0x087FBA, i_087fba),
    (0x087FBC, i_087fbc),
    (0x087FC0, i_087fc0),
    (0x087FC6, i_087fc6),
    (0x087FCA, i_087fca),
    (0x087FD0, i_087fd0),
    (0x087FD6, i_087fd6),
    (0x087FDA, i_087fda),
    (0x087FE0, i_087fe0),
    (0x087FE6, i_087fe6),
    (0x087FEA, i_087fea),
    (0x087FF0, i_087ff0),
    (0x087FF6, i_087ff6),
    (0x087FFA, i_087ffa),
    (0x088000, i_088000),
    (0x088006, i_088006),
    (0x08800C, i_08800c),
    (0x08800E, i_08800e),
    (0x088014, i_088014),
    (0x08801A, i_08801a),
    (0x088020, i_088020),
    (0x088024, i_088024),
    (0x088028, i_088028),
    (0x08802E, i_08802e),
    (0x088034, i_088034),
    (0x08803A, i_08803a),
    (0x08803E, i_08803e),
    (0x088042, i_088042),
    (0x088048, i_088048),
    (0x08804E, i_08804e),
    (0x088054, i_088054),
    (0x088058, i_088058),
    (0x08805C, i_08805c),
    (0x088062, i_088062),
    (0x088068, i_088068),
    (0x08806E, i_08806e),
    (0x088072, i_088072),
    (0x088076, i_088076),
    (0x08807A, i_08807a),
    (0x088080, i_088080),
    (0x088086, i_088086),
    (0x08808C, i_08808c),
    (0x088092, i_088092),
    (0x088098, i_088098),
    (0x08809E, i_08809e),
    (0x0880A4, i_0880a4),
    (0x0880AA, i_0880aa),
    (0x0880AE, i_0880ae),
    (0x0880B0, i_0880b0),
    (0x0880B6, i_0880b6),
    (0x0880BC, i_0880bc),
    (0x0880C2, i_0880c2),
    (0x0880C6, i_0880c6),
    (0x0880CC, i_0880cc),
    (0x0880D2, i_0880d2),
    (0x0880D8, i_0880d8),
    (0x0880DE, i_0880de),
    (0x0880E2, i_0880e2),
    (0x0880E6, i_0880e6),
    (0x0880EC, i_0880ec),
    (0x0880F0, i_0880f0),
    (0x0880F4, i_0880f4),
    (0x0880F8, i_0880f8),
    (0x0880FE, i_0880fe),
    (0x088102, i_088102),
    (0x088108, i_088108),
    (0x08810C, i_08810c),
    (0x088112, i_088112),
    (0x088114, i_088114),
    (0x088118, i_088118),
    (0x08811E, i_08811e),
    (0x088122, i_088122),
    (0x088128, i_088128),
    (0x08812E, i_08812e),
    (0x088134, i_088134),
    (0x08813A, i_08813a),
    (0x088140, i_088140),
    (0x088146, i_088146),
    (0x08814C, i_08814c),
    (0x088152, i_088152),
    (0x088156, i_088156),
    (0x088158, i_088158),
    (0x08815E, i_08815e),
    (0x088164, i_088164),
    (0x088168, i_088168),
    (0x08816E, i_08816e),
    (0x088174, i_088174),
    (0x08817A, i_08817a),
    (0x088180, i_088180),
    (0x088186, i_088186),
    (0x08818C, i_08818c),
    (0x088190, i_088190),
    (0x088194, i_088194),
    (0x088196, i_088196),
    (0x08819C, i_08819c),
    (0x0881A0, i_0881a0),
    (0x0881A6, i_0881a6),
    (0x0881A8, i_0881a8),
    (0x0881AE, i_0881ae),
    (0x0881B4, i_0881b4),
    (0x0881B8, i_0881b8),
    (0x0881BA, i_0881ba),
    (0x0881C0, i_0881c0),
    (0x0881C6, i_0881c6),
    (0x0881CA, i_0881ca),
    (0x0881CE, i_0881ce),
    (0x0881D0, i_0881d0),
    (0x0881D6, i_0881d6),
    (0x0881DA, i_0881da),
    (0x0881E0, i_0881e0),
    (0x0881E6, i_0881e6),
    (0x0881EC, i_0881ec),
    (0x0881F2, i_0881f2),
    (0x0881F8, i_0881f8),
    (0x0881FE, i_0881fe),
    (0x088202, i_088202),
    (0x088208, i_088208),
    (0x08820A, i_08820a),
    (0x088210, i_088210),
    (0x088214, i_088214),
    (0x08821A, i_08821a),
    (0x088220, i_088220),
    (0x088224, i_088224),
    (0x088226, i_088226),
    (0x08822C, i_08822c),
    (0x088232, i_088232),
    (0x088238, i_088238),
    (0x08823C, i_08823c),
    (0x088242, i_088242),
    (0x088248, i_088248),
    (0x08824E, i_08824e),
    (0x088254, i_088254),
    (0x088258, i_088258),
    (0x08825C, i_08825c),
    (0x08825E, i_08825e),
    (0x088264, i_088264),
    (0x088268, i_088268),
    (0x08826E, i_08826e),
    (0x088270, i_088270),
    (0x088276, i_088276),
    (0x08827C, i_08827c),
    (0x088280, i_088280),
    (0x088282, i_088282),
    (0x088288, i_088288),
    (0x08828E, i_08828e),
    (0x088294, i_088294),
    (0x088298, i_088298),
    (0x08829E, i_08829e),
    (0x0882A4, i_0882a4),
    (0x0882AA, i_0882aa),
    (0x0882B0, i_0882b0),
    (0x0882B4, i_0882b4),
    (0x0882B8, i_0882b8),
    (0x0882BA, i_0882ba),
    (0x0882C0, i_0882c0),
    (0x0882C4, i_0882c4),
    (0x0882CA, i_0882ca),
    (0x0882CC, i_0882cc),
    (0x0882D2, i_0882d2),
    (0x0882D8, i_0882d8),
    (0x0882DC, i_0882dc),
    (0x0882DE, i_0882de),
    (0x0882E4, i_0882e4),
    (0x0882EA, i_0882ea),
    (0x0882F0, i_0882f0),
    (0x0882F4, i_0882f4),
    (0x0882FA, i_0882fa),
    (0x088300, i_088300),
    (0x088306, i_088306),
    (0x08830C, i_08830c),
    (0x088310, i_088310),
    (0x088314, i_088314),
    (0x088316, i_088316),
    (0x08831C, i_08831c),
    (0x088320, i_088320),
    (0x088326, i_088326),
    (0x088328, i_088328),
    (0x08832E, i_08832e),
    (0x088334, i_088334),
    (0x088338, i_088338),
    (0x08833A, i_08833a),
    (0x088340, i_088340),
    (0x088346, i_088346),
    (0x08834C, i_08834c),
    (0x088350, i_088350),
    (0x088356, i_088356),
    (0x08835C, i_08835c),
    (0x088362, i_088362),
    (0x088368, i_088368),
    (0x08836C, i_08836c),
    (0x088372, i_088372),
    (0x088378, i_088378),
    (0x08837C, i_08837c),
    (0x08837E, i_08837e),
    (0x088384, i_088384),
    (0x088388, i_088388),
    (0x08838E, i_08838e),
    (0x088390, i_088390),
    (0x088396, i_088396),
    (0x08839C, i_08839c),
    (0x0883A0, i_0883a0),
    (0x0883A2, i_0883a2),
    (0x0883A6, i_0883a6),
    (0x0883A8, i_0883a8),
    (0x0883AE, i_0883ae),
    (0x0883B4, i_0883b4),
    (0x0883BA, i_0883ba),
    (0x0883BE, i_0883be),
    (0x0883C4, i_0883c4),
    (0x0883CA, i_0883ca),
    (0x0883D0, i_0883d0),
    (0x0883D6, i_0883d6),
    (0x0883DA, i_0883da),
    (0x0883E0, i_0883e0),
    (0x0883E4, i_0883e4),
    (0x0883EA, i_0883ea),
    (0x0883EC, i_0883ec),
    (0x0883F2, i_0883f2),
    (0x0883F8, i_0883f8),
    (0x0883FC, i_0883fc),
    (0x088400, i_088400),
    (0x088404, i_088404),
    (0x088408, i_088408),
    (0x08840E, i_08840e),
    (0x088412, i_088412),
    (0x088416, i_088416),
    (0x08841A, i_08841a),
    (0x08841C, i_08841c),
    (0x088420, i_088420),
    (0x088424, i_088424),
    (0x088428, i_088428),
    (0x08842E, i_08842e),
    (0x088432, i_088432),
    (0x088436, i_088436),
    (0x088438, i_088438),
    (0x08843C, i_08843c),
    (0x088440, i_088440),
    (0x088444, i_088444),
    (0x088448, i_088448),
    (0x08844A, i_08844a),
    (0x08844E, i_08844e),
    (0x088450, i_088450),
    (0x088456, i_088456),
    (0x08845A, i_08845a),
    (0x08845E, i_08845e),
    (0x088460, i_088460),
    (0x088464, i_088464),
    (0x088468, i_088468),
    (0x08846A, i_08846a),
    (0x08846E, i_08846e),
    (0x088472, i_088472),
    (0x088476, i_088476),
    (0x08847A, i_08847a),
    (0x088480, i_088480),
    (0x088482, i_088482),
    (0x088486, i_088486),
    (0x08848A, i_08848a),
    (0x08848C, i_08848c),
    (0x088490, i_088490),
    (0x088494, i_088494),
    (0x088496, i_088496),
    (0x08849A, i_08849a),
    (0x08849E, i_08849e),
    (0x0884A4, i_0884a4),
    (0x0884A6, i_0884a6),
    (0x0884AA, i_0884aa),
    (0x0884B0, i_0884b0),
    (0x0884B6, i_0884b6),
    (0x0884BC, i_0884bc),
    (0x0884C2, i_0884c2),
    (0x0884C8, i_0884c8),
    (0x0884CE, i_0884ce),
    (0x0884D2, i_0884d2),
    (0x0884D8, i_0884d8),
    (0x0884DE, i_0884de),
    (0x0884E2, i_0884e2),
    (0x0884E6, i_0884e6),
    (0x0884EC, i_0884ec),
    (0x0884F2, i_0884f2),
    (0x0884F8, i_0884f8),
    (0x0884FE, i_0884fe),
    (0x088504, i_088504),
    (0x08850A, i_08850a),
    (0x08850E, i_08850e),
    (0x088514, i_088514),
    (0x08851A, i_08851a),
    (0x08851E, i_08851e),
    (0x088522, i_088522),
    (0x088528, i_088528),
    (0x08852E, i_08852e),
    (0x088534, i_088534),
    (0x08853A, i_08853a),
    (0x088540, i_088540),
    (0x088546, i_088546),
    (0x08854A, i_08854a),
    (0x088550, i_088550),
    (0x088556, i_088556),
    (0x08855A, i_08855a),
    (0x08855E, i_08855e),
    (0x088564, i_088564),
    (0x08856A, i_08856a),
    (0x088570, i_088570),
    (0x088576, i_088576),
    (0x08857C, i_08857c),
    (0x088580, i_088580),
    (0x088586, i_088586),
    (0x08858A, i_08858a),
    (0x088590, i_088590),
    (0x088596, i_088596),
    (0x08859C, i_08859c),
    (0x0885A2, i_0885a2),
    (0x0885A8, i_0885a8),
    (0x0885AC, i_0885ac),
    (0x0885B2, i_0885b2),
    (0x0885B4, i_0885b4),
    (0x0885B8, i_0885b8),
    (0x0885BE, i_0885be),
    (0x0885C4, i_0885c4),
    (0x0885CA, i_0885ca),
    (0x0885D0, i_0885d0),
    (0x0885D6, i_0885d6),
    (0x0885DC, i_0885dc),
    (0x0885E0, i_0885e0),
    (0x0885E6, i_0885e6),
    (0x0885EC, i_0885ec),
    (0x0885F0, i_0885f0),
    (0x0885F4, i_0885f4),
    (0x0885FA, i_0885fa),
    (0x088600, i_088600),
    (0x088606, i_088606),
    (0x08860C, i_08860c),
    (0x088612, i_088612),
    (0x088618, i_088618),
    (0x08861C, i_08861c),
    (0x088622, i_088622),
    (0x088628, i_088628),
    (0x08862C, i_08862c),
    (0x088630, i_088630),
    (0x088636, i_088636),
    (0x08863C, i_08863c),
    (0x088642, i_088642),
    (0x088648, i_088648),
    (0x08864E, i_08864e),
    (0x088654, i_088654),
    (0x088658, i_088658),
    (0x08865E, i_08865e),
    (0x088664, i_088664),
    (0x088668, i_088668),
    (0x08866C, i_08866c),
    (0x088672, i_088672),
    (0x088678, i_088678),
    (0x08867E, i_08867e),
    (0x088684, i_088684),
    (0x08868A, i_08868a),
    (0x08868E, i_08868e),
    (0x088694, i_088694),
    (0x088698, i_088698),
    (0x08869E, i_08869e),
    (0x0886A4, i_0886a4),
    (0x0886AA, i_0886aa),
    (0x0886B0, i_0886b0),
    (0x0886B6, i_0886b6),
    (0x0886BA, i_0886ba),
    (0x0886C0, i_0886c0),
    (0x0886C2, i_0886c2),
    (0x0886C6, i_0886c6),
    (0x0886CC, i_0886cc),
    (0x0886D2, i_0886d2),
    (0x0886D8, i_0886d8),
    (0x0886DE, i_0886de),
    (0x0886E4, i_0886e4),
    (0x0886EA, i_0886ea),
    (0x0886EE, i_0886ee),
    (0x0886F4, i_0886f4),
    (0x0886FA, i_0886fa),
    (0x0886FE, i_0886fe),
    (0x088702, i_088702),
    (0x088708, i_088708),
    (0x08870E, i_08870e),
    (0x088714, i_088714),
    (0x08871A, i_08871a),
    (0x088720, i_088720),
    (0x088726, i_088726),
    (0x08872A, i_08872a),
    (0x088730, i_088730),
    (0x088736, i_088736),
    (0x08873A, i_08873a),
    (0x08873E, i_08873e),
    (0x088744, i_088744),
    (0x08874A, i_08874a),
    (0x088750, i_088750),
    (0x088756, i_088756),
    (0x08875C, i_08875c),
    (0x088762, i_088762),
    (0x088766, i_088766),
    (0x08876C, i_08876c),
    (0x088772, i_088772),
    (0x088776, i_088776),
    (0x08877A, i_08877a),
    (0x088780, i_088780),
    (0x088786, i_088786),
    (0x08878C, i_08878c),
    (0x088792, i_088792),
    (0x088798, i_088798),
    (0x08879C, i_08879c),
    (0x0887A2, i_0887a2),
    (0x0887A6, i_0887a6),
    (0x0887AC, i_0887ac),
    (0x0887B2, i_0887b2),
    (0x0887B8, i_0887b8),
    (0x0887BE, i_0887be),
    (0x0887C4, i_0887c4),
    (0x0887C8, i_0887c8),
    (0x0887CE, i_0887ce),
    (0x0887D0, i_0887d0),
    (0x0887D4, i_0887d4),
    (0x0887DA, i_0887da),
    (0x0887E0, i_0887e0),
    (0x0887E6, i_0887e6),
    (0x0887EC, i_0887ec),
    (0x0887F2, i_0887f2),
    (0x0887F8, i_0887f8),
    (0x0887FC, i_0887fc),
    (0x088802, i_088802),
    (0x088808, i_088808),
    (0x08880C, i_08880c),
    (0x088810, i_088810),
    (0x088816, i_088816),
    (0x08881C, i_08881c),
    (0x088822, i_088822),
    (0x088828, i_088828),
    (0x08882E, i_08882e),
    (0x088834, i_088834),
    (0x088838, i_088838),
    (0x08883E, i_08883e),
    (0x088844, i_088844),
    (0x088848, i_088848),
    (0x08884C, i_08884c),
    (0x088852, i_088852),
    (0x088858, i_088858),
    (0x08885E, i_08885e),
    (0x088864, i_088864),
    (0x08886A, i_08886a),
    (0x088870, i_088870),
    (0x088874, i_088874),
    (0x08887A, i_08887a),
    (0x088880, i_088880),
    (0x088884, i_088884),
    (0x088888, i_088888),
    (0x08888E, i_08888e),
    (0x088894, i_088894),
    (0x08889A, i_08889a),
    (0x0888A0, i_0888a0),
    (0x0888A6, i_0888a6),
    (0x0888AC, i_0888ac),
    (0x0888B0, i_0888b0),
    (0x0888B6, i_0888b6),
    (0x0888BC, i_0888bc),
    (0x0888C0, i_0888c0),
    (0x0888C4, i_0888c4),
    (0x0888CA, i_0888ca),
    (0x0888D0, i_0888d0),
    (0x0888D6, i_0888d6),
    (0x0888DC, i_0888dc),
    (0x0888E2, i_0888e2),
    (0x0888E6, i_0888e6),
    (0x0888EC, i_0888ec),
    (0x0888F0, i_0888f0),
    (0x0888F6, i_0888f6),
    (0x0888FC, i_0888fc),
    (0x088902, i_088902),
    (0x088908, i_088908),
    (0x08890E, i_08890e),
];
