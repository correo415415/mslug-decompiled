// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p05_2) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


// ===== Prop_TrapFlameBase_053b9e  ($053B9E, 96 B, props/helpers_0539f0.s)
/// $053B9E: move.w #$1e4, d1
pub fn i_053b9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53BA2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1E4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $053BA2: jsr $236e.l
pub fn i_053ba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53BA8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53BA8);
    cpu.pc = 0x236E;
    16
}
/// $053BA8: jsr $267e2.l
pub fn i_053ba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53BAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53BAE);
    cpu.pc = 0x267E2;
    16
}
/// $053BAE: move.w #$f000, $38(a6)
pub fn i_053bae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53BB4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $053BB4: jsr $27cee.l
pub fn i_053bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53BBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53BBA);
    cpu.pc = 0x27CEE;
    16
}
/// $053BBA: lea.l $297856.l, a0
pub fn i_053bba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53BC0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x297856;
    20
}
/// $053BC0: jsr $28cd4.l
pub fn i_053bc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53BC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53BC6);
    cpu.pc = 0x28CD4;
    16
}
/// $053BC6: move.w #$0, $72(a6)
pub fn i_053bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53BCC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $053BCC: lea.l $53bd2(pc), a1
pub fn i_053bcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53BD0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x53BD2;
    16
}
/// $053BD0: move.l a1, (a6)
pub fn i_053bd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53BD2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $053BD2: movea.l $c(a6), a0
pub fn i_053bd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53BD6;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $053BD6: move.w $22(a0), $22(a6)
pub fn i_053bd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53BDC;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $053BDC: move.w $24(a0), $24(a6)
pub fn i_053bdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53BE2;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $053BE2: jsr $28d70.l
pub fn i_053be2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53BE8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53BE8);
    cpu.pc = 0x28D70;
    16
}
/// $053BE8: movea.l $c(a6), a0
pub fn i_053be8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53BEC;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $053BEC: cmpi.b #$ff, $20(a0)
pub fn i_053bec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53BF2;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $053BF2: bne.w $53bfc
pub fn i_053bf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53BF6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x53BFC; return 10; }
    8
}
/// $053BF6: jmp $518.l
pub fn i_053bf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53BFC;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $053BFC: rts
pub fn i_053bfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53BFE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_TrapFlameHitFx_053bfe  ($053BFE, 70 B, props/helpers_0539f0.s)
/// $053BFE: move.w #$1e4, d1
pub fn i_053bfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C02;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1E4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $053C02: jsr $236e.l
pub fn i_053c02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53C08);
    cpu.pc = 0x236E;
    16
}
/// $053C08: jsr $267e2.l
pub fn i_053c08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53C0E);
    cpu.pc = 0x267E2;
    16
}
/// $053C0E: move.w #$8000, $38(a6)
pub fn i_053c0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C14;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $053C14: jsr $27cee.l
pub fn i_053c14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53C1A);
    cpu.pc = 0x27CEE;
    16
}
/// $053C1A: lea.l $2978cc.l, a0
pub fn i_053c1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C20;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2978CC;
    20
}
/// $053C20: jsr $28cd4.l
pub fn i_053c20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53C26);
    cpu.pc = 0x28CD4;
    16
}
/// $053C26: lea.l $53c2c(pc), a1
pub fn i_053c26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C2A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x53C2C;
    16
}
/// $053C2A: move.l a1, (a6)
pub fn i_053c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C2C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $053C2C: jsr $2783a.l
pub fn i_053c2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53C32);
    cpu.pc = 0x2783A;
    16
}
/// $053C32: jsr $28d70.l
pub fn i_053c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53C38);
    cpu.pc = 0x28D70;
    16
}
/// $053C38: bcc.w $53c42
pub fn i_053c38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C3C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x53C42; return 10; }
    8
}
/// $053C3C: jmp $518.l
pub fn i_053c3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C42;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $053C42: rts
pub fn i_053c42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C44;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_TrapFlameHitSpawn_053c44  ($053C44, 16 B, props/helpers_0539f0.s)
/// $053C44: lea.l $53bfe(pc), a1
pub fn i_053c44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C48;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x53BFE;
    16
}
/// $053C48: jsr $4ae.l
pub fn i_053c48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53C4E);
    cpu.pc = 0x4AE;
    16
}
/// $053C4E: jsr $5dd02.l
pub fn i_053c4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53C54);
    cpu.pc = 0x5DD02;
    16
}
// ===== SetTaskHandler_053c54  ($053C54, 8 B, kernel/task_handlers.c)
/// $053C54: lea.l $53c5c(pc), a1
pub fn i_053c54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C58;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x53C5C;
    16
}
/// $053C58: move.l a1, (a6)
pub fn i_053c58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C5A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $053C5A: rts
pub fn i_053c5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C5C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetTaskHandler_053c5c  ($053C5C, 8 B, kernel/task_handlers.c)
/// $053C5C: lea.l $53c64(pc), a1
pub fn i_053c5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C60;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x53C64;
    16
}
/// $053C60: move.l a1, (a6)
pub fn i_053c60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C62;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $053C62: rts
pub fn i_053c62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C64;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_BurnFollowVictim_053c64  ($053C64, 142 B, props/helpers_0539f0.s)
/// $053C64: movea.l $50(a6), a0
pub fn i_053c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C68;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x50);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $053C68: move.l a0, d0
pub fn i_053c68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C6A;
    cpu.ir = 0x2008;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $053C6A: cmpi.l #$ffffffff, d0
pub fn i_053c6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C70;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0xFFFFFFFF, Size::Long, false);
    8
}
/// $053C70: bne.w $53c7c
pub fn i_053c70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C74;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x53C7C; return 10; }
    8
}
/// $053C74: jmp $518.l
pub fn i_053c74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C7A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $053C7C: move.b $45(a0), d0
pub fn i_053c7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C80;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x45);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $053C80: cmpi.b #$0, d0
pub fn i_053c80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C84;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $053C84: bgt.w $53c9e
pub fn i_053c84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C88;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x53C9E; return 10; }
    8
}
/// $053C88: move.w $28(a0), d0
pub fn i_053c88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C8C;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $053C8C: asl.w #$1, d0
pub fn i_053c8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C8E;
    cpu.ir = 0xE340;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $053C8E: move.w d0, $28(a0)
pub fn i_053c8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C92;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $053C92: move.w #$1000, $2a(a0)
pub fn i_053c92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C98;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $053C98: move.w #$19, $72(a6)
pub fn i_053c98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C9E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x19;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $053C9E: lea.l $53ca4(pc), a1
pub fn i_053c9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53CA2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x53CA4;
    16
}
/// $053CA2: move.l a1, (a6)
pub fn i_053ca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53CA4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $053CA4: movea.l $50(a6), a0
pub fn i_053ca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53CA8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x50);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $053CA8: move.w $22(a0), $22(a6)
pub fn i_053ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53CAE;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $053CAE: move.w $24(a0), $24(a6)
pub fn i_053cae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53CB4;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $053CB4: move.w $38(a0), $38(a6)
pub fn i_053cb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53CBA;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad4, v3 as u16);
    20
}
/// $053CBA: move.w $72(a6), d0
pub fn i_053cba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53CBE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $053CBE: andi.w #$3, d0
pub fn i_053cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53CC2;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $053CC2: bne.w $53ce2
pub fn i_053cc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53CC6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x53CE2; return 10; }
    8
}
/// $053CC6: lea.l $53cf2(pc), a1
pub fn i_053cc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53CCA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x53CF2;
    16
}
/// $053CCA: jsr $4ae.l
pub fn i_053cca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53CD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53CD0);
    cpu.pc = 0x4AE;
    16
}
/// $053CD0: jsr $5dd22.l
pub fn i_053cd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53CD6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53CD6);
    cpu.pc = 0x5DD22;
    16
}
/// $053CD6: addi.w #$10, $24(a0)
pub fn i_053cd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53CDC;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $053CDC: move.w $38(a6), $38(a0)
pub fn i_053cdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53CE2;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad4, v3 as u16);
    20
}
/// $053CE2: subq.w #$1, $72(a6)
pub fn i_053ce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53CE6;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $053CE6: bpl.w $53cf0
pub fn i_053ce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53CEA;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x53CF0; return 10; }
    8
}
/// $053CEA: jmp $518.l
pub fn i_053cea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53CF0;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $053CF0: rts
pub fn i_053cf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53CF2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_BurnSmokePuff_053cf2  ($053CF2, 58 B, props/helpers_0539f0.s)
/// $053CF2: move.w #$2, d1
pub fn i_053cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53CF6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $053CF6: jsr $236e.l
pub fn i_053cf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53CFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53CFC);
    cpu.pc = 0x236E;
    16
}
/// $053CFC: jsr $267e2.l
pub fn i_053cfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53D02);
    cpu.pc = 0x267E2;
    16
}
/// $053D02: lea.l $297922.l, a0
pub fn i_053d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D08;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x297922;
    20
}
/// $053D08: jsr $28cd4.l
pub fn i_053d08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53D0E);
    cpu.pc = 0x28CD4;
    16
}
/// $053D0E: lea.l $53d14(pc), a1
pub fn i_053d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D12;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x53D14;
    16
}
/// $053D12: move.l a1, (a6)
pub fn i_053d12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D14;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $053D14: jsr $2783a.l
pub fn i_053d14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53D1A);
    cpu.pc = 0x2783A;
    16
}
/// $053D1A: jsr $28d70.l
pub fn i_053d1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D20;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53D20);
    cpu.pc = 0x28D70;
    16
}
/// $053D20: bcc.w $53d2a
pub fn i_053d20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D24;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x53D2A; return 10; }
    8
}
/// $053D24: jmp $518.l
pub fn i_053d24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D2A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $053D2A: rts
pub fn i_053d2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D2C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_SpawnBonusOnce_053d2c  ($053D2C, 36 B, props/helpers_0539f0.s)
/// $053D2C: cmpi.b #$0, $21(a6)
pub fn i_053d2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D32;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $053D32: bne.w $53d4e
pub fn i_053d32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D36;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x53D4E; return 10; }
    8
}
/// $053D36: move.b #$ff, $21(a6)
pub fn i_053d36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D3C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $053D3C: jsr $9ba56.l
pub fn i_053d3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D42;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53D42);
    cpu.pc = 0x9BA56;
    16
}
/// $053D42: subi.w #$18, $22(a0)
pub fn i_053d42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D48;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $053D48: addi.w #$30, $24(a0)
pub fn i_053d48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D4E;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x30, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $053D4E: rts
pub fn i_053d4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D50;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_PickSpriteByVictimDir_053d80  ($053D80, 66 B, props/helpers_0539f0.s)
/// $053D80: movea.l $50(a6), a0
pub fn i_053d80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D84;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x50);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $053D84: clr.l d0
pub fn i_053d84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D86;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $053D86: move.w $28(a0), d0
pub fn i_053d86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D8A;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $053D8A: cmpi.w #$0, d0
pub fn i_053d8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D8E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $053D8E: beq.w $53dc8
pub fn i_053d8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D92;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x53DC8; return 10; }
    8
}
/// $053D92: cmpi.b #$0, $20(a6)
pub fn i_053d92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D98;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $053D98: beq.w $53db4
pub fn i_053d98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D9C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x53DB4; return 10; }
    8
}
/// $053D9C: btst.b #$f, d0
pub fn i_053d9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53DA0;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((15) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $053DA0: beq.w $53dc8
pub fn i_053da0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53DA4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x53DC8; return 10; }
    8
}
/// $053DA4: lea.l $2976d2.l, a0
pub fn i_053da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53DAA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2976D2;
    20
}
/// $053DAA: jsr $28cd4.l
pub fn i_053daa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53DB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53DB0);
    cpu.pc = 0x28CD4;
    16
}
/// $053DB0: bra.w $53dc8
pub fn i_053db0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53DB4;
    cpu.ir = 0x6000;
    cpu.pc = 0x53DC8;
    10
}
/// $053DB4: btst.b #$f, d0
pub fn i_053db4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53DB8;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((15) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $053DB8: bne.w $53dc8
pub fn i_053db8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53DBC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x53DC8; return 10; }
    8
}
/// $053DBC: lea.l $297668.l, a0
pub fn i_053dbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53DC2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x297668;
    20
}
// ===== JsrAbsThunk_053dc2  ($053DC2, 8 B, thunks/jsr_abs_thunks.c)
/// $053DC2: jsr $28cd4.l
pub fn i_053dc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53DC8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53DC8);
    cpu.pc = 0x28CD4;
    16
}
/// $053DC8: rts
pub fn i_053dc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53DCA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_SyncSpriteWithParent_053dca  ($053DCA, 66 B, props/helpers_0539f0.s)
/// $053DCA: movea.l $c(a6), a0
pub fn i_053dca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53DCE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $053DCE: move.b $20(a0), d0
pub fn i_053dce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53DD2;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $053DD2: cmp.b $20(a6), d0
pub fn i_053dd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53DD6;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $053DD6: beq.w $53e00
pub fn i_053dd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53DDA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x53E00; return 10; }
    8
}
/// $053DDA: cmpi.b #$0, $20(a0)
pub fn i_053dda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53DE0;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $053DE0: bne.w $53df4
pub fn i_053de0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53DE4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x53DF4; return 10; }
    8
}
/// $053DE4: lea.l $297798.l, a0
pub fn i_053de4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53DEA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x297798;
    20
}
/// $053DEA: jsr $28cd4.l
pub fn i_053dea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53DF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53DF0);
    cpu.pc = 0x28CD4;
    16
}
/// $053DF0: bra.w $53e00
pub fn i_053df0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53DF4;
    cpu.ir = 0x6000;
    cpu.pc = 0x53E00;
    10
}
/// $053DF4: lea.l $297750.l, a0
pub fn i_053df4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53DFA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x297750;
    20
}
/// $053DFA: jsr $28cd4.l
pub fn i_053dfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E00;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53E00);
    cpu.pc = 0x28CD4;
    16
}
/// $053E00: movea.l $c(a6), a0
pub fn i_053e00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E04;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $053E04: move.b $20(a0), $20(a6)
pub fn i_053e04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E0A;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad4, v3 as u8);
    20
}
/// $053E0A: rts
pub fn i_053e0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E0C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_PlayBreakMusicByPhase_053e0c  ($053E0C, 100 B, props/helpers_0539f0.s)
/// $053E0C: cmpi.b #$0, $21(a6)
pub fn i_053e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E12;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $053E12: bne.w $53e24
pub fn i_053e12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E16;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x53E24; return 10; }
    8
}
/// $053E16: move.w #$102c, d0
pub fn i_053e16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E1A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $053E1A: jsr $2352.l
pub fn i_053e1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E20;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53E20);
    cpu.pc = 0x2352;
    16
}
/// $053E20: bra.w $53e76
pub fn i_053e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E24;
    cpu.ir = 0x6000;
    cpu.pc = 0x53E76;
    10
}
/// $053E24: cmpi.b #$1, $21(a6)
pub fn i_053e24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E2A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $053E2A: bne.w $53e3c
pub fn i_053e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E2E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x53E3C; return 10; }
    8
}
/// $053E2E: move.w #$102c, d0
pub fn i_053e2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E32;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $053E32: jsr $2352.l
pub fn i_053e32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53E38);
    cpu.pc = 0x2352;
    16
}
/// $053E38: bra.w $53e76
pub fn i_053e38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E3C;
    cpu.ir = 0x6000;
    cpu.pc = 0x53E76;
    10
}
/// $053E3C: cmpi.b #$2, $21(a6)
pub fn i_053e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E42;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $053E42: bne.w $53e54
pub fn i_053e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E46;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x53E54; return 10; }
    8
}
/// $053E46: move.w #$1023, d0
pub fn i_053e46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E4A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1023;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $053E4A: jsr $2352.l
pub fn i_053e4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E50;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53E50);
    cpu.pc = 0x2352;
    16
}
/// $053E50: bra.w $53e76
pub fn i_053e50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E54;
    cpu.ir = 0x6000;
    cpu.pc = 0x53E76;
    10
}
/// $053E54: cmpi.b #$3, $21(a6)
pub fn i_053e54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E5A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $053E5A: bne.w $53e6c
pub fn i_053e5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E5E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x53E6C; return 10; }
    8
}
/// $053E5E: move.w #$1035, d0
pub fn i_053e5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E62;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1035;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $053E62: jsr $2352.l
pub fn i_053e62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E68;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53E68);
    cpu.pc = 0x2352;
    16
}
/// $053E68: bra.w $53e76
pub fn i_053e68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E6C;
    cpu.ir = 0x6000;
    cpu.pc = 0x53E76;
    10
}
/// $053E6C: move.w #$1023, d0
pub fn i_053e6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E70;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1023;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_053e70  ($053E70, 8 B, thunks/jsr_abs_thunks.c)
/// $053E70: jsr $2352.l
pub fn i_053e70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53E76);
    cpu.pc = 0x2352;
    16
}
/// $053E76: rts
pub fn i_053e76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E78;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_RunDebrisScriptByPhase_053e78  ($053E78, 28 B, props/helpers_0539f0.s)
/// $053E78: lea.l $297f50.l, a0
pub fn i_053e78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E7E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x297F50;
    20
}
/// $053E7E: move.b $21(a6), d0
pub fn i_053e7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E82;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $053E82: andi.w #$ff, d0
pub fn i_053e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E86;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $053E86: asl.w #$2, d0
pub fn i_053e86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E88;
    cpu.ir = 0xE540;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $053E88: move.w $38(a6), d1
pub fn i_053e88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E8C;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $053E8C: asl.w #$4, d1
pub fn i_053e8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E8E;
    cpu.ir = 0xE941;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $053E8E: add.w d1, d0
pub fn i_053e8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E90;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $053E90: movea.l (a0, d0.w), a2
pub fn i_053e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E94;
    cpu.ir = 0x2470;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    18
}
// ===== JsrAbsThunk_053e94  ($053E94, 8 B, thunks/jsr_abs_thunks.c)
/// $053E94: jsr $5022a.l
pub fn i_053e94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E9A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53E9A);
    cpu.pc = 0x5022A;
    16
}
/// $053E9A: rts
pub fn i_053e9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E9C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_PickRandomItemPtr_053e9c  ($053E9C, 22 B, props/helpers_0539f0.s)
/// $053E9C: move.w #$3, d0
pub fn i_053e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53EA0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $053EA0: jsr $5ea1c.l
pub fn i_053ea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53EA6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53EA6);
    cpu.pc = 0x5EA1C;
    16
}
/// $053EA6: asl.w #$2, d0
pub fn i_053ea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53EA8;
    cpu.ir = 0xE540;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $053EA8: lea.l $2980fc.l, a0
pub fn i_053ea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53EAE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2980FC;
    20
}
/// $053EAE: movea.l (a0, d0.w), a0
pub fn i_053eae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53EB2;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
// ===== JsrAbsThunk_053eb2  ($053EB2, 8 B, thunks/jsr_abs_thunks.c)
/// $053EB2: jsr $28cd4.l
pub fn i_053eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53EB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53EB8);
    cpu.pc = 0x28CD4;
    16
}
/// $053EB8: rts
pub fn i_053eb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53EBA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_RunDebrisScriptByPrio_053eba  ($053EBA, 32 B, props/helpers_0539f0.s)
/// $053EBA: lea.l $298074.l, a1
pub fn i_053eba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53EC0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x298074;
    20
}
/// $053EC0: cmpi.w #$0, $38(a6)
pub fn i_053ec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53EC6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $053EC6: beq.w $53eda
pub fn i_053ec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53ECA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x53EDA; return 10; }
    8
}
/// $053ECA: cmpi.w #$6, $38(a6)
pub fn i_053eca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53ED0;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x6, Size::Word, false);
    16
}
/// $053ED0: beq.w $53eda
pub fn i_053ed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53ED4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x53EDA; return 10; }
    8
}
/// $053ED4: lea.l $298062.l, a1
pub fn i_053ed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53EDA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x298062;
    20
}
// ===== JsrAbsThunk_053eda  ($053EDA, 8 B, thunks/jsr_abs_thunks.c)
/// $053EDA: jsr $77c7e.l
pub fn i_053eda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53EE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53EE0);
    cpu.pc = 0x77C7E;
    16
}
/// $053EE0: rts
pub fn i_053ee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53EE2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_GateDebrisA_053ee2  ($053EE2, 24 B, props/helpers_0539f0.s)
/// $053EE2: lea.l $29860c.l, a2
pub fn i_053ee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53EE8;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29860C;
    20
}
/// $053EE8: jsr $5022a.l
pub fn i_053ee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53EEE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53EEE);
    cpu.pc = 0x5022A;
    16
}
/// $053EEE: lea.l $298620.l, a2
pub fn i_053eee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53EF4;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x298620;
    20
}
/// $053EF4: jsr $5022a.l
pub fn i_053ef4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53EFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53EFA);
    cpu.pc = 0x5022A;
    16
}
// ===== SDS_053efa  ($053EFA, 14 B, kernel/state_dispatch_stubs.c)
/// $053EFA: lea.l $298634.l, a2
pub fn i_053efa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F00;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x298634;
    20
}
/// $053F00: jsr $5022a.l
pub fn i_053f00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F06;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53F06);
    cpu.pc = 0x5022A;
    16
}
/// $053F06: rts
pub fn i_053f06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F08;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_GateDebrisB_053f08  ($053F08, 24 B, props/helpers_0539f0.s)
/// $053F08: lea.l $298648.l, a2
pub fn i_053f08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F0E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x298648;
    20
}
/// $053F0E: jsr $5022a.l
pub fn i_053f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53F14);
    cpu.pc = 0x5022A;
    16
}
/// $053F14: lea.l $29865c.l, a2
pub fn i_053f14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F1A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29865C;
    20
}
/// $053F1A: jsr $5022a.l
pub fn i_053f1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F20;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53F20);
    cpu.pc = 0x5022A;
    16
}
// ===== SDS_053f20  ($053F20, 14 B, kernel/state_dispatch_stubs.c)
/// $053F20: lea.l $298670.l, a2
pub fn i_053f20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F26;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x298670;
    20
}
/// $053F26: jsr $5022a.l
pub fn i_053f26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F2C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53F2C);
    cpu.pc = 0x5022A;
    16
}
/// $053F2C: rts
pub fn i_053f2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F2E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_GateDebrisC_053f2e  ($053F2E, 24 B, props/helpers_0539f0.s)
/// $053F2E: lea.l $298684.l, a2
pub fn i_053f2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F34;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x298684;
    20
}
/// $053F34: jsr $5022a.l
pub fn i_053f34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53F3A);
    cpu.pc = 0x5022A;
    16
}
/// $053F3A: lea.l $298698.l, a2
pub fn i_053f3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F40;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x298698;
    20
}
/// $053F40: jsr $5022a.l
pub fn i_053f40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F46;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53F46);
    cpu.pc = 0x5022A;
    16
}
// ===== SDS_053f46  ($053F46, 14 B, kernel/state_dispatch_stubs.c)
/// $053F46: lea.l $2986ac.l, a2
pub fn i_053f46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F4C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2986AC;
    20
}
/// $053F4C: jsr $5022a.l
pub fn i_053f4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53F52);
    cpu.pc = 0x5022A;
    16
}
/// $053F52: rts
pub fn i_053f52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F54;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_GateDebrisD_053f54  ($053F54, 24 B, props/helpers_0539f0.s)
/// $053F54: lea.l $2986c0.l, a2
pub fn i_053f54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F5A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2986C0;
    20
}
/// $053F5A: jsr $5022a.l
pub fn i_053f5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F60;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53F60);
    cpu.pc = 0x5022A;
    16
}
/// $053F60: lea.l $2986d4.l, a2
pub fn i_053f60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F66;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2986D4;
    20
}
/// $053F66: jsr $5022a.l
pub fn i_053f66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53F6C);
    cpu.pc = 0x5022A;
    16
}
// ===== SDS_053f6c  ($053F6C, 14 B, kernel/state_dispatch_stubs.c)
/// $053F6C: lea.l $2986e8.l, a2
pub fn i_053f6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F72;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2986E8;
    20
}
/// $053F72: jsr $5022a.l
pub fn i_053f72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F78;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53F78);
    cpu.pc = 0x5022A;
    16
}
/// $053F78: rts
pub fn i_053f78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F7A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CmpField10WithLink8_053f7a  ($053F7A, 16 B, props/helpers_0539f0.s)
/// $053F7A: movea.l $8(a6), a1
pub fn i_053f7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F7E;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $053F7E: move.b $10(a6), d0
pub fn i_053f7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F82;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $053F82: cmp.b $10(a1), d0
pub fn i_053f82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F86;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $053F86: bcs.w $53f90
pub fn i_053f86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F8A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x53F90; return 10; }
    8
}
// ===== ClearXN_053f8a  ($053F8A, 6 B, thunks/ccr_helpers.c)
/// $053F8A: andi.b #$ee, ccr
pub fn i_053f8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F8E;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $053F8E: rts
pub fn i_053f8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F90;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_053f90  ($053F90, 6 B, thunks/ccr_helpers.c)
/// $053F90: ori.b #$11, ccr
pub fn i_053f90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F94;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $053F94: rts
pub fn i_053f94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F96;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Building_053f96  ($053F96, 198 B, props/mission_053f96.s)
/// $053F96: movea.l $3c(a6), a1
pub fn i_053f96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F9A;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $053F9A: jsr $2942a.l
pub fn i_053f9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53FA0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53FA0);
    cpu.pc = 0x2942A;
    16
}
/// $053FA0: move.w #$8b, d1
pub fn i_053fa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53FA4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x8B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $053FA4: jsr $236e.l
pub fn i_053fa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53FAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53FAA);
    cpu.pc = 0x236E;
    16
}
/// $053FAA: move.w #$90, $70(a6)
pub fn i_053faa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53FB0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x90;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $053FB0: move.b #$ff, $32(a6)
pub fn i_053fb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53FB6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $053FB6: move.b #$ff, $33(a6)
pub fn i_053fb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53FBC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $053FBC: move.b #$0, $3a(a6)
pub fn i_053fbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53FC2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $053FC2: move.w #$78, $66(a6)
pub fn i_053fc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53FC8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x78;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $053FC8: lea.l $298936.l, a0
pub fn i_053fc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53FCE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x298936;
    20
}
/// $053FCE: jsr $28cd4.l
pub fn i_053fce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53FD4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53FD4);
    cpu.pc = 0x28CD4;
    16
}
/// $053FD4: lea.l $53fda(pc), a1
pub fn i_053fd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53FD8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x53FDA;
    16
}
/// $053FD8: move.l a1, (a6)
pub fn i_053fd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53FDA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $053FDA: clr.b $10e39a.l
pub fn i_053fda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53FE0;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $053FE0: jsr $2783a.l
pub fn i_053fe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53FE6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53FE6);
    cpu.pc = 0x2783A;
    16
}
/// $053FE6: jsr $28d70.l
pub fn i_053fe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53FEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53FEC);
    cpu.pc = 0x28D70;
    16
}
/// $053FEC: cmpi.b #$5, $58(a6)
pub fn i_053fec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53FF2;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x5, Size::Byte, false);
    16
}
/// $053FF2: beq.w $5401c
pub fn i_053ff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53FF6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5401C; return 10; }
    8
}
/// $053FF6: jsr $2870a.l
pub fn i_053ff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53FFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53FFC);
    cpu.pc = 0x2870A;
    16
}
/// $053FFC: bcc.w $54012
pub fn i_053ffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54000;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54012; return 10; }
    8
}
/// $054000: lea.l $5e766.l, a0
pub fn i_054000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54006;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $054006: jsr $5e770.l
pub fn i_054006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5400C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5400C);
    cpu.pc = 0x5E770;
    16
}
/// $05400C: bclr.b #$3, $13(a6)
pub fn i_05400c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54012;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $054012: cmpi.w #$3c, $66(a6)
pub fn i_054012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54018;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3C, Size::Word, false);
    16
}
/// $054018: bgt.w $5404a
pub fn i_054018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5401C;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x5404A; return 10; }
    8
}
/// $05401C: move.w #$1023, d0
pub fn i_05401c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54020;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1023;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $054020: jsr $2352.l
pub fn i_054020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54026;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54026);
    cpu.pc = 0x2352;
    16
}
/// $054026: bclr.b #$3, $13(a6)
pub fn i_054026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5402C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05402C: lea.l $29a278.l, a1
pub fn i_05402c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54032;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A278;
    20
}
/// $054032: jsr $77c7e.l
pub fn i_054032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54038;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54038);
    cpu.pc = 0x77C7E;
    16
}
/// $054038: lea.l $29a28a.l, a1
pub fn i_054038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5403E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A28A;
    20
}
/// $05403E: jsr $77c7e.l
pub fn i_05403e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54044;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54044);
    cpu.pc = 0x77C7E;
    16
}
/// $054044: lea.l $5405c(pc), a1
pub fn i_054044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54048;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5405C;
    16
}
/// $054048: move.l a1, (a6)
pub fn i_054048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5404A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05404A: jsr $4fa70.l
pub fn i_05404a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54050;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54050);
    cpu.pc = 0x4FA70;
    16
}
/// $054050: bcc.w $5405a
pub fn i_054050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54054;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5405A; return 10; }
    8
}
/// $054054: jmp $518.l
pub fn i_054054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5405A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $05405A: rts
pub fn i_05405a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5405C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_BuildingDamaged_05405c  ($05405C, 174 B, props/mission_053f96.s)
/// $05405C: jsr $2783a.l
pub fn i_05405c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54062;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54062);
    cpu.pc = 0x2783A;
    16
}
/// $054062: lea.l $29b29a.l, a1
pub fn i_054062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54068;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29B29A;
    20
}
/// $054068: jsr $43fac.l
pub fn i_054068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5406E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5406E);
    cpu.pc = 0x43FAC;
    16
}
/// $05406E: lea.l $298958.l, a0
pub fn i_05406e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54074;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x298958;
    20
}
/// $054074: jsr $28cd4.l
pub fn i_054074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5407A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5407A);
    cpu.pc = 0x28CD4;
    16
}
/// $05407A: lea.l $54080(pc), a1
pub fn i_05407a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5407E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54080;
    16
}
/// $05407E: move.l a1, (a6)
pub fn i_05407e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54080;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $054080: clr.b $10e39a.l
pub fn i_054080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54086;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $054086: jsr $2783a.l
pub fn i_054086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5408C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5408C);
    cpu.pc = 0x2783A;
    16
}
/// $05408C: jsr $28d70.l
pub fn i_05408c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54092;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54092);
    cpu.pc = 0x28D70;
    16
}
/// $054092: jsr $2870a.l
pub fn i_054092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54098;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54098);
    cpu.pc = 0x2870A;
    16
}
/// $054098: bcc.w $540ae
pub fn i_054098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5409C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x540AE; return 10; }
    8
}
/// $05409C: lea.l $5e766.l, a0
pub fn i_05409c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x540A2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $0540A2: jsr $5e770.l
pub fn i_0540a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x540A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x540A8);
    cpu.pc = 0x5E770;
    16
}
/// $0540A8: bclr.b #$3, $13(a6)
pub fn i_0540a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x540AE;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0540AE: jsr $28758.l
pub fn i_0540ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x540B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x540B4);
    cpu.pc = 0x28758;
    16
}
/// $0540B4: bcc.w $540f8
pub fn i_0540b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x540B8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x540F8; return 10; }
    8
}
/// $0540B8: bclr.b #$0, $13(a6)
pub fn i_0540b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x540BE;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0540BE: move.w #$1030, d0
pub fn i_0540be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x540C2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1030;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0540C2: jsr $2352.l
pub fn i_0540c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x540C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x540C8);
    cpu.pc = 0x2352;
    16
}
/// $0540C8: lea.l $29a278.l, a1
pub fn i_0540c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x540CE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A278;
    20
}
/// $0540CE: jsr $77c7e.l
pub fn i_0540ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x540D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x540D4);
    cpu.pc = 0x77C7E;
    16
}
/// $0540D4: lea.l $29a28a.l, a1
pub fn i_0540d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x540DA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A28A;
    20
}
/// $0540DA: jsr $77c7e.l
pub fn i_0540da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x540E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x540E0);
    cpu.pc = 0x77C7E;
    16
}
/// $0540E0: lea.l $29a3ce.l, a1
pub fn i_0540e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x540E6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A3CE;
    20
}
/// $0540E6: jsr $77c7e.l
pub fn i_0540e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x540EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x540EC);
    cpu.pc = 0x77C7E;
    16
}
/// $0540EC: move.w #$f000, $38(a0)
pub fn i_0540ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x540F2;
    cpu.ir = 0x317C;
    let v1: u32 = 0xF000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0540F2: lea.l $5410a(pc), a1
pub fn i_0540f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x540F6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5410A;
    16
}
/// $0540F6: move.l a1, (a6)
pub fn i_0540f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x540F8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0540F8: jsr $4fa70.l
pub fn i_0540f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x540FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x540FE);
    cpu.pc = 0x4FA70;
    16
}
/// $0540FE: bcc.w $54108
pub fn i_0540fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54102;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54108; return 10; }
    8
}
/// $054102: jmp $518.l
pub fn i_054102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54108;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $054108: rts
pub fn i_054108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5410A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_BuildingWreck_05410a  ($05410A, 86 B, props/mission_053f96.s)
/// $05410A: move.l #$2000, d0
pub fn i_05410a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54110;
    cpu.ir = 0x203C;
    let v1: u32 = 0x2000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $054110: jsr $51a28.l
pub fn i_054110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54116;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54116);
    cpu.pc = 0x51A28;
    16
}
/// $054116: jsr $2783a.l
pub fn i_054116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5411C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5411C);
    cpu.pc = 0x2783A;
    16
}
/// $05411C: lea.l $29b2b2.l, a1
pub fn i_05411c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54122;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29B2B2;
    20
}
/// $054122: jsr $43fac.l
pub fn i_054122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54128;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54128);
    cpu.pc = 0x43FAC;
    16
}
/// $054128: lea.l $ffff.w, a0
pub fn i_054128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5412C;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $05412C: move.l a0, $48(a6)
pub fn i_05412c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54130;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $054130: lea.l $29897a.l, a0
pub fn i_054130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54136;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29897A;
    20
}
/// $054136: jsr $28cd4.l
pub fn i_054136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5413C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5413C);
    cpu.pc = 0x28CD4;
    16
}
/// $05413C: lea.l $54142(pc), a1
pub fn i_05413c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54140;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54142;
    16
}
/// $054140: move.l a1, (a6)
pub fn i_054140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54142;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $054142: jsr $2783a.l
pub fn i_054142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54148;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54148);
    cpu.pc = 0x2783A;
    16
}
/// $054148: jsr $28d70.l
pub fn i_054148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5414E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5414E);
    cpu.pc = 0x28D70;
    16
}
/// $05414E: jsr $4fa70.l
pub fn i_05414e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54154;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54154);
    cpu.pc = 0x4FA70;
    16
}
/// $054154: bcc.w $5415e
pub fn i_054154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54158;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5415E; return 10; }
    8
}
/// $054158: jmp $518.l
pub fn i_054158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5415E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $05415E: rts
pub fn i_05415e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54160;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Column_054160  ($054160, 244 B, props/mission_053f96.s)
/// $054160: movea.l $3c(a6), a1
pub fn i_054160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54164;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $054164: jsr $2942a.l
pub fn i_054164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5416A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5416A);
    cpu.pc = 0x2942A;
    16
}
/// $05416A: move.w #$8c, d1
pub fn i_05416a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5416E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x8C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05416E: jsr $236e.l
pub fn i_05416e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54174;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54174);
    cpu.pc = 0x236E;
    16
}
/// $054174: move.w #$40, $70(a6)
pub fn i_054174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5417A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05417A: move.b #$ff, $32(a6)
pub fn i_05417a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54180;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $054180: move.b #$ff, $33(a6)
pub fn i_054180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54186;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $054186: move.b #$0, $3a(a6)
pub fn i_054186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5418C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05418C: move.w #$3c, $66(a6)
pub fn i_05418c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54192;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $054192: move.w #$8000, $38(a6)
pub fn i_054192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54198;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $054198: jsr $267e2.l
pub fn i_054198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5419E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5419E);
    cpu.pc = 0x267E2;
    16
}
/// $05419E: jsr $27cee.l
pub fn i_05419e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x541A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x541A4);
    cpu.pc = 0x27CEE;
    16
}
/// $0541A4: lea.l $29899e.l, a0
pub fn i_0541a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x541AA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29899E;
    20
}
/// $0541AA: jsr $28cd4.l
pub fn i_0541aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x541B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x541B0);
    cpu.pc = 0x28CD4;
    16
}
/// $0541B0: lea.l $541b6(pc), a1
pub fn i_0541b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x541B4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x541B6;
    16
}
/// $0541B4: move.l a1, (a6)
pub fn i_0541b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x541B6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0541B6: jsr $2783a.l
pub fn i_0541b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x541BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x541BC);
    cpu.pc = 0x2783A;
    16
}
/// $0541BC: jsr $28108.l
pub fn i_0541bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x541C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x541C2);
    cpu.pc = 0x28108;
    16
}
/// $0541C2: jsr $28d70.l
pub fn i_0541c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x541C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x541C8);
    cpu.pc = 0x28D70;
    16
}
/// $0541C8: jsr $2870a.l
pub fn i_0541c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x541CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x541CE);
    cpu.pc = 0x2870A;
    16
}
/// $0541CE: bcc.w $541e4
pub fn i_0541ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x541D2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x541E4; return 10; }
    8
}
/// $0541D2: lea.l $5e766.l, a0
pub fn i_0541d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x541D8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $0541D8: jsr $5e770.l
pub fn i_0541d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x541DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x541DE);
    cpu.pc = 0x5E770;
    16
}
/// $0541DE: bclr.b #$3, $13(a6)
pub fn i_0541de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x541E4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0541E4: jsr $28758.l
pub fn i_0541e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x541EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x541EA);
    cpu.pc = 0x28758;
    16
}
/// $0541EA: bcc.w $54242
pub fn i_0541ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x541EE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54242; return 10; }
    8
}
/// $0541EE: bclr.b #$0, $13(a6)
pub fn i_0541ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x541F4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0541F4: move.l #$3000, d0
pub fn i_0541f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x541FA;
    cpu.ir = 0x203C;
    let v1: u32 = 0x3000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0541FA: jsr $51a28.l
pub fn i_0541fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54200;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54200);
    cpu.pc = 0x51A28;
    16
}
/// $054200: move.w #$1023, d0
pub fn i_054200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54204;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1023;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $054204: jsr $2352.l
pub fn i_054204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5420A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5420A);
    cpu.pc = 0x2352;
    16
}
/// $05420A: lea.l $29b368.l, a1
pub fn i_05420a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54210;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29B368;
    20
}
/// $054210: jsr $43fac.l
pub fn i_054210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54216;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54216);
    cpu.pc = 0x43FAC;
    16
}
/// $054216: lea.l $29a29c.l, a1
pub fn i_054216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5421C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A29C;
    20
}
/// $05421C: jsr $77c7e.l
pub fn i_05421c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54222;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54222);
    cpu.pc = 0x77C7E;
    16
}
/// $054222: lea.l $29a2ae.l, a1
pub fn i_054222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54228;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A2AE;
    20
}
/// $054228: jsr $77c7e.l
pub fn i_054228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5422E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5422E);
    cpu.pc = 0x77C7E;
    16
}
/// $05422E: lea.l $29a3e0.l, a1
pub fn i_05422e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54234;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A3E0;
    20
}
/// $054234: jsr $77c7e.l
pub fn i_054234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5423A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5423A);
    cpu.pc = 0x77C7E;
    16
}
/// $05423A: jsr $55266(pc)
pub fn i_05423a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5423E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5423E);
    cpu.pc = 0x55266;
    16
}
/// $05423E: bra.w $5424c
pub fn i_05423e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54242;
    cpu.ir = 0x6000;
    cpu.pc = 0x5424C;
    10
}
/// $054242: jsr $4fa70.l
pub fn i_054242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54248;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54248);
    cpu.pc = 0x4FA70;
    16
}
/// $054248: bcc.w $54252
pub fn i_054248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5424C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54252; return 10; }
    8
}
/// $05424C: jmp $518.l
pub fn i_05424c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54252;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $054252: rts
pub fn i_054252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54254;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_CompoundWall_054254  ($054254, 228 B, props/mission_053f96.s)
/// $054254: move.b #$0, $20(a6)
pub fn i_054254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5425A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05425A: move.w #$118, $66(a6)
pub fn i_05425a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54260;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x118;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $054260: lea.l $2989b4.l, a0
pub fn i_054260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54266;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2989B4;
    20
}
/// $054266: jsr $28cd4.l
pub fn i_054266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5426C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5426C);
    cpu.pc = 0x28CD4;
    16
}
/// $05426C: jsr $2783a.l
pub fn i_05426c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54272;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54272);
    cpu.pc = 0x2783A;
    16
}
/// $054272: lea.l $29b2ca.l, a1
pub fn i_054272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54278;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29B2CA;
    20
}
/// $054278: jsr $43fac.l
pub fn i_054278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5427E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5427E);
    cpu.pc = 0x43FAC;
    16
}
/// $05427E: bra.w $542b6
pub fn i_05427e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54282;
    cpu.ir = 0x6000;
    cpu.pc = 0x542B6;
    10
}
/// $0542B6: move.w #$60, $70(a6)
pub fn i_0542b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x542BC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0542BC: lea.l $542c2(pc), a1
pub fn i_0542bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x542C0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x542C2;
    16
}
/// $0542C0: move.l a1, (a6)
pub fn i_0542c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x542C2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0542C2: clr.b $10e39a.l
pub fn i_0542c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x542C8;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $0542C8: jsr $2783a.l
pub fn i_0542c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x542CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x542CE);
    cpu.pc = 0x2783A;
    16
}
/// $0542CE: jsr $28d70.l
pub fn i_0542ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x542D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x542D4);
    cpu.pc = 0x28D70;
    16
}
/// $0542D4: jsr $2870a.l
pub fn i_0542d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x542DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x542DA);
    cpu.pc = 0x2870A;
    16
}
/// $0542DA: bcc.w $542f0
pub fn i_0542da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x542DE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x542F0; return 10; }
    8
}
/// $0542DE: lea.l $5e766.l, a0
pub fn i_0542de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x542E4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $0542E4: jsr $5e770.l
pub fn i_0542e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x542EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x542EA);
    cpu.pc = 0x5E770;
    16
}
/// $0542EA: bclr.b #$3, $13(a6)
pub fn i_0542ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x542F0;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0542F0: cmpi.w #$a0, $66(a6)
pub fn i_0542f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x542F6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xA0, Size::Word, false);
    16
}
/// $0542F6: bgt.w $54326
pub fn i_0542f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x542FA;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x54326; return 10; }
    8
}
/// $0542FA: move.w #$102a, d0
pub fn i_0542fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x542FE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0542FE: jsr $2352.l
pub fn i_0542fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54304;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54304);
    cpu.pc = 0x2352;
    16
}
/// $054304: lea.l $29a2d2.l, a1
pub fn i_054304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5430A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A2D2;
    20
}
/// $05430A: jsr $77c7e.l
pub fn i_05430a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54310;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54310);
    cpu.pc = 0x77C7E;
    16
}
/// $054310: cmpi.b #$0, $20(a6)
pub fn i_054310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54316;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $054316: beq.w $54320
pub fn i_054316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5431A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x54320; return 10; }
    8
}
/// $05431A: subi.w #$30, $22(a0)
pub fn i_05431a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54320;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x30, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $054320: lea.l $54338(pc), a1
pub fn i_054320(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54324;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54338;
    16
}
/// $054324: move.l a1, (a6)
pub fn i_054324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54326;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $054326: jsr $4fa70.l
pub fn i_054326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5432C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5432C);
    cpu.pc = 0x4FA70;
    16
}
/// $05432C: bcc.w $54336
pub fn i_05432c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54330;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54336; return 10; }
    8
}
/// $054330: jmp $518.l
pub fn i_054330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54336;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $054336: rts
pub fn i_054336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54338;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_CompoundWallDamaged_054338  ($054338, 192 B, props/mission_053f96.s)
/// $054338: cmpi.b #$0, $20(a6)
pub fn i_054338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5433E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $05433E: bne.w $54352
pub fn i_05433e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54342;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x54352; return 10; }
    8
}
/// $054342: lea.l $2989ca.l, a0
pub fn i_054342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54348;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2989CA;
    20
}
/// $054348: jsr $28cd4.l
pub fn i_054348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5434E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5434E);
    cpu.pc = 0x28CD4;
    16
}
/// $05434E: bra.w $5435e
pub fn i_05434e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54352;
    cpu.ir = 0x6000;
    cpu.pc = 0x5435E;
    10
}
/// $054352: lea.l $298a26.l, a0
pub fn i_054352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54358;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x298A26;
    20
}
/// $054358: jsr $28cd4.l
pub fn i_054358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5435E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5435E);
    cpu.pc = 0x28CD4;
    16
}
/// $05435E: lea.l $54364(pc), a1
pub fn i_05435e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54362;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54364;
    16
}
/// $054362: move.l a1, (a6)
pub fn i_054362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54364;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $054364: clr.b $10e39a.l
pub fn i_054364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5436A;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $05436A: jsr $2783a.l
pub fn i_05436a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54370;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54370);
    cpu.pc = 0x2783A;
    16
}
/// $054370: jsr $28d70.l
pub fn i_054370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54376;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54376);
    cpu.pc = 0x28D70;
    16
}
/// $054376: jsr $2870a.l
pub fn i_054376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5437C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5437C);
    cpu.pc = 0x2870A;
    16
}
/// $05437C: bcc.w $54392
pub fn i_05437c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54380;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54392; return 10; }
    8
}
/// $054380: lea.l $5e766.l, a0
pub fn i_054380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54386;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $054386: jsr $5e770.l
pub fn i_054386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5438C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5438C);
    cpu.pc = 0x5E770;
    16
}
/// $05438C: bclr.b #$3, $13(a6)
pub fn i_05438c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54392;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $054392: jsr $28758.l
pub fn i_054392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54398;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54398);
    cpu.pc = 0x28758;
    16
}
/// $054398: bcc.w $543e6
pub fn i_054398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5439C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x543E6; return 10; }
    8
}
/// $05439C: bclr.b #$0, $13(a6)
pub fn i_05439c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x543A2;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0543A2: move.w #$102b, d0
pub fn i_0543a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x543A6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0543A6: jsr $2352.l
pub fn i_0543a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x543AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x543AC);
    cpu.pc = 0x2352;
    16
}
/// $0543AC: cmpi.b #$0, $20(a6)
pub fn i_0543ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x543B2;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $0543B2: beq.w $543bc
pub fn i_0543b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x543B6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x543BC; return 10; }
    8
}
/// $0543B6: subi.w #$30, $22(a6)
pub fn i_0543b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x543BC;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x30, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0543BC: lea.l $29a2c0.l, a1
pub fn i_0543bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x543C2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A2C0;
    20
}
/// $0543C2: jsr $77c7e.l
pub fn i_0543c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x543C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x543C8);
    cpu.pc = 0x77C7E;
    16
}
/// $0543C8: move.w #$c000, $38(a0)
pub fn i_0543c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x543CE;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0543CE: lea.l $29a3f2.l, a1
pub fn i_0543ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x543D4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A3F2;
    20
}
/// $0543D4: jsr $77c7e.l
pub fn i_0543d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x543DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x543DA);
    cpu.pc = 0x77C7E;
    16
}
/// $0543DA: move.w #$c000, $38(a0)
pub fn i_0543da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x543E0;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0543E0: lea.l $543f8(pc), a1
pub fn i_0543e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x543E4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x543F8;
    16
}
/// $0543E4: move.l a1, (a6)
pub fn i_0543e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x543E6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0543E6: jsr $4fa70.l
pub fn i_0543e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x543EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x543EC);
    cpu.pc = 0x4FA70;
    16
}
/// $0543EC: bcc.w $543f6
pub fn i_0543ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x543F0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x543F6; return 10; }
    8
}
/// $0543F0: jmp $518.l
pub fn i_0543f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x543F6;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0543F6: rts
pub fn i_0543f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x543F8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_CompoundWallWreck_0543f8  ($0543F8, 132 B, props/mission_053f96.s)
/// $0543F8: jsr $2783a.l
pub fn i_0543f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x543FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x543FE);
    cpu.pc = 0x2783A;
    16
}
/// $0543FE: cmpi.b #$0, $20(a6)
pub fn i_0543fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54404;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $054404: bne.w $5443a
pub fn i_054404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54408;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5443A; return 10; }
    8
}
/// $054408: move.l #$3000, d0
pub fn i_054408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5440E;
    cpu.ir = 0x203C;
    let v1: u32 = 0x3000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05440E: jsr $51a28.l
pub fn i_05440e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54414;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54414);
    cpu.pc = 0x51A28;
    16
}
/// $054414: movea.l $c(a6), a0
pub fn i_054414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54418;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $054418: move.b #$ff, $20(a0)
pub fn i_054418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5441E;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05441E: lea.l $2989f4.l, a0
pub fn i_05441e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54424;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2989F4;
    20
}
/// $054424: jsr $28cd4.l
pub fn i_054424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5442A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5442A);
    cpu.pc = 0x28CD4;
    16
}
/// $05442A: lea.l $29b2de.l, a1
pub fn i_05442a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54430;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29B2DE;
    20
}
/// $054430: jsr $43fac.l
pub fn i_054430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54436;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54436);
    cpu.pc = 0x43FAC;
    16
}
/// $054436: bra.w $54464
pub fn i_054436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5443A;
    cpu.ir = 0x6000;
    cpu.pc = 0x54464;
    10
}
/// $05443A: move.l #$1000, d0
pub fn i_05443a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54440;
    cpu.ir = 0x203C;
    let v1: u32 = 0x1000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $054440: jsr $51a28.l
pub fn i_054440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54446;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54446);
    cpu.pc = 0x51A28;
    16
}
/// $054446: addi.w #$30, $22(a6)
pub fn i_054446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5444C;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x30, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $05444C: lea.l $298a3c.l, a0
pub fn i_05444c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54452;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x298A3C;
    20
}
/// $054452: jsr $28cd4.l
pub fn i_054452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54458;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54458);
    cpu.pc = 0x28CD4;
    16
}
/// $054458: lea.l $29b306.l, a1
pub fn i_054458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5445E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29B306;
    20
}
/// $05445E: jsr $43fac.l
pub fn i_05445e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54464;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54464);
    cpu.pc = 0x43FAC;
    16
}
/// $054464: lea.l $5446a(pc), a1
pub fn i_054464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54468;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5446A;
    16
}
/// $054468: move.l a1, (a6)
pub fn i_054468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5446A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05446A: jsr $2783a.l
pub fn i_05446a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54470;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54470);
    cpu.pc = 0x2783A;
    16
}
/// $054470: jsr $28d70.l
pub fn i_054470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54476;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54476);
    cpu.pc = 0x28D70;
    16
}
/// $054476: jmp $518.l
pub fn i_054476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5447C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Prop_Nop_05447c  ($05447C, 2 B, props/mission_053f96.s)
/// $05447C: rts
pub fn i_05447c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5447E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Compound_Spawn_05447e  ($05447E, 76 B, props/mission_053f96.s)
/// $05447E: movea.l $3c(a6), a1
pub fn i_05447e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54482;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $054482: jsr $2942a.l
pub fn i_054482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54488;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54488);
    cpu.pc = 0x2942A;
    16
}
/// $054488: move.w #$60, $70(a6)
pub fn i_054488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5448E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05448E: lea.l $54254(pc), a1
pub fn i_05448e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54492;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54254;
    16
}
/// $054492: jsr $4ae.l
pub fn i_054492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54498;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54498);
    cpu.pc = 0x4AE;
    16
}
/// $054498: jsr $5dd22.l
pub fn i_054498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5449E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5449E);
    cpu.pc = 0x5DD22;
    16
}
/// $05449E: lea.l $545c0(pc), a1
pub fn i_05449e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x544A2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x545C0;
    16
}
/// $0544A2: jsr $4ae.l
pub fn i_0544a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x544A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x544A8);
    cpu.pc = 0x4AE;
    16
}
/// $0544A8: jsr $5dd22.l
pub fn i_0544a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x544AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x544AE);
    cpu.pc = 0x5DD22;
    16
}
/// $0544AE: lea.l $54584(pc), a1
pub fn i_0544ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x544B2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54584;
    16
}
/// $0544B2: jsr $4ae.l
pub fn i_0544b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x544B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x544B8);
    cpu.pc = 0x4AE;
    16
}
/// $0544B8: jsr $5dd22.l
pub fn i_0544b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x544BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x544BE);
    cpu.pc = 0x5DD22;
    16
}
/// $0544BE: move.b #$0, $20(a6)
pub fn i_0544be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x544C4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0544C4: move.b #$0, $80(a6)
pub fn i_0544c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x544CA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== Prop_Compound_PhaseA_0544ca  ($0544CA, 78 B, props/mission_053f96.s)
/// $0544CA: move.b #$0, $21(a6)
pub fn i_0544ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x544D0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0544D0: lea.l $544d6(pc), a1
pub fn i_0544d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x544D4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x544D6;
    16
}
/// $0544D4: move.l a1, (a6)
pub fn i_0544d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x544D6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0544D6: jsr $2783a.l
pub fn i_0544d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x544DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x544DC);
    cpu.pc = 0x2783A;
    16
}
/// $0544DC: cmpi.b #$1, $21(a6)
pub fn i_0544dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x544E2;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $0544E2: bne.w $544ec
pub fn i_0544e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x544E6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x544EC; return 10; }
    8
}
/// $0544E6: lea.l $54518(pc), a1
pub fn i_0544e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x544EA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54518;
    16
}
/// $0544EA: move.l a1, (a6)
pub fn i_0544ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x544EC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0544EC: cmpi.b #$ff, $80(a6)
pub fn i_0544ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x544F2;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $0544F2: bne.w $544fc
pub fn i_0544f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x544F6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x544FC; return 10; }
    8
}
/// $0544F6: lea.l $54566(pc), a1
pub fn i_0544f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x544FA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54566;
    16
}
/// $0544FA: move.l a1, (a6)
pub fn i_0544fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x544FC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0544FC: jsr $6f0.l
pub fn i_0544fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54502;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54502);
    cpu.pc = 0x6F0;
    16
}
/// $054502: bcs.w $54516
pub fn i_054502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54506;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x54516; return 10; }
    8
}
/// $054506: jsr $4fa70.l
pub fn i_054506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5450C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5450C);
    cpu.pc = 0x4FA70;
    16
}
/// $05450C: bcc.w $54516
pub fn i_05450c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54510;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54516; return 10; }
    8
}
/// $054510: jmp $518.l
pub fn i_054510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54516;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $054516: rts
pub fn i_054516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54518;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Compound_PhaseB_054518  ($054518, 78 B, props/mission_053f96.s)
/// $054518: move.b #$ff, $21(a6)
pub fn i_054518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5451E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05451E: lea.l $54524(pc), a1
pub fn i_05451e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54522;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54524;
    16
}
/// $054522: move.l a1, (a6)
pub fn i_054522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54524;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $054524: jsr $2783a.l
pub fn i_054524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5452A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5452A);
    cpu.pc = 0x2783A;
    16
}
/// $05452A: cmpi.b #$2, $21(a6)
pub fn i_05452a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54530;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $054530: bne.w $5453a
pub fn i_054530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54534;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5453A; return 10; }
    8
}
/// $054534: lea.l $544ca(pc), a1
pub fn i_054534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54538;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x544CA;
    16
}
/// $054538: move.l a1, (a6)
pub fn i_054538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5453A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05453A: cmpi.b #$ff, $80(a6)
pub fn i_05453a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54540;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $054540: bne.w $5454a
pub fn i_054540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54544;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5454A; return 10; }
    8
}
/// $054544: lea.l $54566(pc), a1
pub fn i_054544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54548;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54566;
    16
}
/// $054548: move.l a1, (a6)
pub fn i_054548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5454A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05454A: jsr $6f0.l
pub fn i_05454a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54550;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54550);
    cpu.pc = 0x6F0;
    16
}
/// $054550: bcs.w $54564
pub fn i_054550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54554;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x54564; return 10; }
    8
}
/// $054554: jsr $4fa70.l
pub fn i_054554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5455A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5455A);
    cpu.pc = 0x4FA70;
    16
}
/// $05455A: bcc.w $54564
pub fn i_05455a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5455E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54564; return 10; }
    8
}
/// $05455E: jmp $518.l
pub fn i_05455e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54564;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $054564: rts
pub fn i_054564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54566;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Compound_Done_054566  ($054566, 30 B, props/mission_053f96.s)
/// $054566: move.b #$ff, $21(a6)
pub fn i_054566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5456C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05456C: lea.l $54572(pc), a1
pub fn i_05456c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54570;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54572;
    16
}
/// $054570: move.l a1, (a6)
pub fn i_054570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54572;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $054572: jsr $6f0.l
pub fn i_054572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54578;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54578);
    cpu.pc = 0x6F0;
    16
}
/// $054578: bcs.w $54582
pub fn i_054578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5457C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x54582; return 10; }
    8
}
/// $05457C: jmp $518.l
pub fn i_05457c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54582;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $054582: rts
pub fn i_054582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54584;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Compound_TriggerSpawn_054584  ($054584, 60 B, props/mission_053f96.s)
/// $054584: lea.l $5458a(pc), a1
pub fn i_054584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54588;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5458A;
    16
}
/// $054588: move.l a1, (a6)
pub fn i_054588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5458A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05458A: jsr $2783a.l
pub fn i_05458a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54590;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54590);
    cpu.pc = 0x2783A;
    16
}
/// $054590: cmpi.w #$e0, $22(a6)
pub fn i_054590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54596;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xE0, Size::Word, false);
    16
}
/// $054596: bgt.w $545be
pub fn i_054596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5459A;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x545BE; return 10; }
    8
}
/// $05459A: lea.l $79eb8.l, a1
pub fn i_05459a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x545A0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x79EB8;
    20
}
/// $0545A0: jsr $4ae.l
pub fn i_0545a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x545A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x545A6);
    cpu.pc = 0x4AE;
    16
}
/// $0545A6: jsr $5dd22.l
pub fn i_0545a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x545AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x545AC);
    cpu.pc = 0x5DD22;
    16
}
/// $0545AC: addi.w #$b0, $22(a0)
pub fn i_0545ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x545B2;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xB0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0545B2: addi.w #$40, $24(a0)
pub fn i_0545b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x545B8;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0545B8: jmp $518.l
pub fn i_0545b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x545BE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0545BE: rts
pub fn i_0545be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x545C0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_NeonSign_0545c0  ($0545C0, 250 B, props/mission_053f96.s)
/// $0545C0: jsr $2783a.l
pub fn i_0545c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x545C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x545C6);
    cpu.pc = 0x2783A;
    16
}
/// $0545C6: cmpi.w #$120, $22(a6)
pub fn i_0545c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x545CC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x120, Size::Word, false);
    16
}
/// $0545CC: ble.w $545d2
pub fn i_0545cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x545D0;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x545D2; return 10; }
    8
}
/// $0545D0: rts
pub fn i_0545d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x545D2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0545D2: move.w #$8d, d1
pub fn i_0545d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x545D6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x8D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0545D6: jsr $236e.l
pub fn i_0545d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x545DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x545DC);
    cpu.pc = 0x236E;
    16
}
/// $0545DC: move.w #$40, $70(a6)
pub fn i_0545dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x545E2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0545E2: addi.w #$a0, $22(a6)
pub fn i_0545e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x545E8;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xA0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0545E8: addi.w #$60, $24(a6)
pub fn i_0545e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x545EE;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x60, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0545EE: move.b #$ff, $32(a6)
pub fn i_0545ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x545F4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0545F4: move.b #$ff, $33(a6)
pub fn i_0545f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x545FA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0545FA: move.w #$8000, d0
pub fn i_0545fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x545FE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0545FE: move.w #$b0, d1
pub fn i_0545fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54602;
    cpu.ir = 0x323C;
    let v1: u32 = 0xB0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $054602: jsr $2813c.l
pub fn i_054602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54608;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54608);
    cpu.pc = 0x2813C;
    16
}
/// $054608: move.b #$0, $3a(a6)
pub fn i_054608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5460E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05460E: move.w #$3c, $66(a6)
pub fn i_05460e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54614;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $054614: lea.l $29908c.l, a0
pub fn i_054614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5461A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29908C;
    20
}
/// $05461A: jsr $28cd4.l
pub fn i_05461a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54620;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54620);
    cpu.pc = 0x28CD4;
    16
}
/// $054620: move.w #$0, $72(a6)
pub fn i_054620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54626;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $054626: move.b #$0, $74(a6)
pub fn i_054626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5462C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05462C: lea.l $54632(pc), a1
pub fn i_05462c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54630;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54632;
    16
}
/// $054630: move.l a1, (a6)
pub fn i_054630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54632;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $054632: jsr $2783a.l
pub fn i_054632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54638;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54638);
    cpu.pc = 0x2783A;
    16
}
/// $054638: jsr $28108.l
pub fn i_054638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5463E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5463E);
    cpu.pc = 0x28108;
    16
}
/// $05463E: jsr $5509c(pc)
pub fn i_05463e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54642;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x54642);
    cpu.pc = 0x5509C;
    16
}
/// $054642: jsr $28d70.l
pub fn i_054642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54648;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54648);
    cpu.pc = 0x28D70;
    16
}
/// $054648: jsr $2870a.l
pub fn i_054648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5464E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5464E);
    cpu.pc = 0x2870A;
    16
}
/// $05464E: bcc.w $54664
pub fn i_05464e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54652;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54664; return 10; }
    8
}
/// $054652: lea.l $5e766.l, a0
pub fn i_054652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54658;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $054658: jsr $5e770.l
pub fn i_054658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5465E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5465E);
    cpu.pc = 0x5E770;
    16
}
/// $05465E: bclr.b #$3, $13(a6)
pub fn i_05465e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54664;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $054664: jsr $28758.l
pub fn i_054664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5466A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5466A);
    cpu.pc = 0x28758;
    16
}
/// $05466A: bcc.w $546a8
pub fn i_05466a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5466E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x546A8; return 10; }
    8
}
/// $05466E: bclr.b #$0, $13(a6)
pub fn i_05466e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54674;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $054674: move.w #$102a, d0
pub fn i_054674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54678;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $054678: jsr $2352.l
pub fn i_054678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5467E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5467E);
    cpu.pc = 0x2352;
    16
}
/// $05467E: lea.l $29a2e4.l, a1
pub fn i_05467e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54684;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A2E4;
    20
}
/// $054684: jsr $77c7e.l
pub fn i_054684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5468A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5468A);
    cpu.pc = 0x77C7E;
    16
}
/// $05468A: lea.l $29a2f6.l, a1
pub fn i_05468a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54690;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A2F6;
    20
}
/// $054690: jsr $77c7e.l
pub fn i_054690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54696;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54696);
    cpu.pc = 0x77C7E;
    16
}
/// $054696: lea.l $29a404.l, a1
pub fn i_054696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5469C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A404;
    20
}
/// $05469C: jsr $77c7e.l
pub fn i_05469c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x546A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x546A2);
    cpu.pc = 0x77C7E;
    16
}
/// $0546A2: lea.l $546ba(pc), a1
pub fn i_0546a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x546A6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x546BA;
    16
}
/// $0546A6: move.l a1, (a6)
pub fn i_0546a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x546A8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0546A8: jsr $4fa70.l
pub fn i_0546a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x546AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x546AE);
    cpu.pc = 0x4FA70;
    16
}
/// $0546AE: bcc.w $546b8
pub fn i_0546ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x546B2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x546B8; return 10; }
    8
}
/// $0546B2: jmp $518.l
pub fn i_0546b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x546B8;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0546B8: rts
pub fn i_0546b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x546BA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_NeonSignWreck_0546ba  ($0546BA, 58 B, props/mission_053f96.s)
/// $0546BA: lea.l $2990a2.l, a0
pub fn i_0546ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x546C0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2990A2;
    20
}
/// $0546C0: jsr $28cd4.l
pub fn i_0546c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x546C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x546C6);
    cpu.pc = 0x28CD4;
    16
}
/// $0546C6: lea.l $546cc(pc), a1
pub fn i_0546c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x546CA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x546CC;
    16
}
/// $0546CA: move.l a1, (a6)
pub fn i_0546ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x546CC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0546CC: jsr $2783a.l
pub fn i_0546cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x546D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x546D2);
    cpu.pc = 0x2783A;
    16
}
/// $0546D2: jsr $28108.l
pub fn i_0546d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x546D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x546D8);
    cpu.pc = 0x28108;
    16
}
/// $0546D8: jsr $5509c(pc)
pub fn i_0546d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x546DC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x546DC);
    cpu.pc = 0x5509C;
    16
}
/// $0546DC: jsr $28d70.l
pub fn i_0546dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x546E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x546E2);
    cpu.pc = 0x28D70;
    16
}
/// $0546E2: jsr $4fa70.l
pub fn i_0546e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x546E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x546E8);
    cpu.pc = 0x4FA70;
    16
}
/// $0546E8: bcc.w $546f2
pub fn i_0546e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x546EC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x546F2; return 10; }
    8
}
/// $0546EC: jmp $518.l
pub fn i_0546ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x546F2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0546F2: rts
pub fn i_0546f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x546F4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Stall_0546f4  ($0546F4, 214 B, props/mission_053f96.s)
/// $0546F4: movea.l $3c(a6), a1
pub fn i_0546f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x546F8;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0546F8: jsr $2942a.l
pub fn i_0546f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x546FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x546FE);
    cpu.pc = 0x2942A;
    16
}
/// $0546FE: move.w #$91, d1
pub fn i_0546fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54702;
    cpu.ir = 0x323C;
    let v1: u32 = 0x91;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $054702: jsr $236e.l
pub fn i_054702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54708;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54708);
    cpu.pc = 0x236E;
    16
}
/// $054708: move.w #$a0, $70(a6)
pub fn i_054708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5470E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05470E: move.b #$ff, $32(a6)
pub fn i_05470e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54714;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $054714: move.b #$ff, $33(a6)
pub fn i_054714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5471A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05471A: move.b #$0, $3a(a6)
pub fn i_05471a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54720;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $054720: move.w #$3c, $66(a6)
pub fn i_054720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54726;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $054726: move.w #$8000, d0
pub fn i_054726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5472A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05472A: move.w #$a0, d1
pub fn i_05472a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5472E;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05472E: jsr $2813c.l
pub fn i_05472e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54734;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54734);
    cpu.pc = 0x2813C;
    16
}
/// $054734: lea.l $298aa8.l, a0
pub fn i_054734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5473A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x298AA8;
    20
}
/// $05473A: jsr $28cd4.l
pub fn i_05473a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54740;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54740);
    cpu.pc = 0x28CD4;
    16
}
/// $054740: lea.l $54746(pc), a1
pub fn i_054740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54744;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54746;
    16
}
/// $054744: move.l a1, (a6)
pub fn i_054744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54746;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $054746: jsr $2783a.l
pub fn i_054746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5474C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5474C);
    cpu.pc = 0x2783A;
    16
}
/// $05474C: jsr $28108.l
pub fn i_05474c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54752;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54752);
    cpu.pc = 0x28108;
    16
}
/// $054752: jsr $28d70.l
pub fn i_054752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54758;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54758);
    cpu.pc = 0x28D70;
    16
}
/// $054758: jsr $2870a.l
pub fn i_054758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5475E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5475E);
    cpu.pc = 0x2870A;
    16
}
/// $05475E: bcc.w $54774
pub fn i_05475e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54762;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54774; return 10; }
    8
}
/// $054762: lea.l $5e766.l, a0
pub fn i_054762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54768;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $054768: jsr $5e770.l
pub fn i_054768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5476E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5476E);
    cpu.pc = 0x5E770;
    16
}
/// $05476E: bclr.b #$3, $13(a6)
pub fn i_05476e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54774;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $054774: jsr $28758.l
pub fn i_054774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5477A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5477A);
    cpu.pc = 0x28758;
    16
}
/// $05477A: bcc.w $547b8
pub fn i_05477a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5477E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x547B8; return 10; }
    8
}
/// $05477E: bclr.b #$0, $13(a6)
pub fn i_05477e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54784;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $054784: move.w #$102c, d0
pub fn i_054784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54788;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $054788: jsr $2352.l
pub fn i_054788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5478E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5478E);
    cpu.pc = 0x2352;
    16
}
/// $05478E: lea.l $29a32c.l, a1
pub fn i_05478e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54794;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A32C;
    20
}
/// $054794: jsr $77c7e.l
pub fn i_054794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5479A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5479A);
    cpu.pc = 0x77C7E;
    16
}
/// $05479A: lea.l $29a33e.l, a1
pub fn i_05479a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x547A0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A33E;
    20
}
/// $0547A0: jsr $77c7e.l
pub fn i_0547a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x547A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x547A6);
    cpu.pc = 0x77C7E;
    16
}
/// $0547A6: lea.l $29a428.l, a1
pub fn i_0547a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x547AC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A428;
    20
}
/// $0547AC: jsr $77c7e.l
pub fn i_0547ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x547B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x547B2);
    cpu.pc = 0x77C7E;
    16
}
/// $0547B2: lea.l $547ca(pc), a1
pub fn i_0547b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x547B6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x547CA;
    16
}
/// $0547B6: move.l a1, (a6)
pub fn i_0547b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x547B8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0547B8: jsr $4fa70.l
pub fn i_0547b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x547BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x547BE);
    cpu.pc = 0x4FA70;
    16
}
/// $0547BE: bcc.w $547c8
pub fn i_0547be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x547C2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x547C8; return 10; }
    8
}
/// $0547C2: jmp $518.l
pub fn i_0547c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x547C8;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0547C8: rts
pub fn i_0547c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x547CA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_StallWreck_0547ca  ($0547CA, 80 B, props/mission_053f96.s)
/// $0547CA: jsr $2783a.l
pub fn i_0547ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x547D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x547D0);
    cpu.pc = 0x2783A;
    16
}
/// $0547D0: lea.l $298b24.l, a0
pub fn i_0547d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x547D6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x298B24;
    20
}
/// $0547D6: jsr $28cd4.l
pub fn i_0547d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x547DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x547DC);
    cpu.pc = 0x28CD4;
    16
}
/// $0547DC: move.l $106f50.l, d0
pub fn i_0547dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x547E2;
    cpu.ir = 0x2039;
    let v2 = bus.read32(0x106F50);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $0547E2: swap d0
pub fn i_0547e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x547E4;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0547E4: cmpi.w #$390, d0
pub fn i_0547e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x547E8;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x390, Size::Word, false);
    8
}
/// $0547E8: bge.w $547fc
pub fn i_0547e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x547EC;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x547FC; return 10; }
    8
}
/// $0547EC: jsr $552c8(pc)
pub fn i_0547ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x547F0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x547F0);
    cpu.pc = 0x552C8;
    16
}
/// $0547F0: lea.l $29b3b2.l, a1
pub fn i_0547f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x547F6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29B3B2;
    20
}
/// $0547F6: jsr $43fac.l
pub fn i_0547f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x547FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x547FC);
    cpu.pc = 0x43FAC;
    16
}
/// $0547FC: lea.l $54802(pc), a1
pub fn i_0547fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54800;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54802;
    16
}
/// $054800: move.l a1, (a6)
pub fn i_054800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54802;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $054802: jsr $2783a.l
pub fn i_054802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54808;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54808);
    cpu.pc = 0x2783A;
    16
}
/// $054808: jsr $28108.l
pub fn i_054808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5480E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5480E);
    cpu.pc = 0x28108;
    16
}
/// $05480E: jsr $28d70.l
pub fn i_05480e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54814;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54814);
    cpu.pc = 0x28D70;
    16
}
/// $054814: jmp $518.l
pub fn i_054814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5481A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Prop_Nop_05481a  ($05481A, 2 B, props/mission_053f96.s)
/// $05481A: rts
pub fn i_05481a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5481C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Fragile_05481c  ($05481C, 172 B, props/mission_053f96.s)
/// $05481C: movea.l $3c(a6), a1
pub fn i_05481c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54820;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $054820: jsr $2942a.l
pub fn i_054820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54826;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54826);
    cpu.pc = 0x2942A;
    16
}
/// $054826: move.w #$92, d1
pub fn i_054826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5482A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x92;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05482A: jsr $236e.l
pub fn i_05482a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54830;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54830);
    cpu.pc = 0x236E;
    16
}
/// $054830: move.w #$80, $70(a6)
pub fn i_054830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54836;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $054836: move.b #$ff, $32(a6)
pub fn i_054836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5483C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05483C: move.b #$ff, $33(a6)
pub fn i_05483c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54842;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $054842: move.b #$0, $3a(a6)
pub fn i_054842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54848;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $054848: move.w #$1, $66(a6)
pub fn i_054848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5484E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05484E: lea.l $298b34.l, a0
pub fn i_05484e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54854;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x298B34;
    20
}
/// $054854: jsr $28cd4.l
pub fn i_054854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5485A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5485A);
    cpu.pc = 0x28CD4;
    16
}
/// $05485A: lea.l $54860(pc), a1
pub fn i_05485a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5485E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54860;
    16
}
/// $05485E: move.l a1, (a6)
pub fn i_05485e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54860;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $054860: jsr $2783a.l
pub fn i_054860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54866;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54866);
    cpu.pc = 0x2783A;
    16
}
/// $054866: jsr $28d70.l
pub fn i_054866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5486C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5486C);
    cpu.pc = 0x28D70;
    16
}
/// $05486C: jsr $2870a.l
pub fn i_05486c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54872;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54872);
    cpu.pc = 0x2870A;
    16
}
/// $054872: bcc.w $54888
pub fn i_054872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54876;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54888; return 10; }
    8
}
/// $054876: lea.l $5e766.l, a0
pub fn i_054876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5487C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $05487C: jsr $5e770.l
pub fn i_05487c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54882;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54882);
    cpu.pc = 0x5E770;
    16
}
/// $054882: bclr.b #$3, $13(a6)
pub fn i_054882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54888;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $054888: jsr $28758.l
pub fn i_054888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5488E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5488E);
    cpu.pc = 0x28758;
    16
}
/// $05488E: bcc.w $548b6
pub fn i_05488e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54892;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x548B6; return 10; }
    8
}
/// $054892: bclr.b #$0, $13(a6)
pub fn i_054892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54898;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $054898: lea.l $29a350.l, a1
pub fn i_054898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5489E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A350;
    20
}
/// $05489E: jsr $77c7e.l
pub fn i_05489e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x548A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x548A4);
    cpu.pc = 0x77C7E;
    16
}
/// $0548A4: lea.l $29a43a.l, a1
pub fn i_0548a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x548AA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A43A;
    20
}
/// $0548AA: jsr $77c7e.l
pub fn i_0548aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x548B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x548B0);
    cpu.pc = 0x77C7E;
    16
}
/// $0548B0: lea.l $548c8(pc), a1
pub fn i_0548b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x548B4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x548C8;
    16
}
/// $0548B4: move.l a1, (a6)
pub fn i_0548b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x548B6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0548B6: jsr $4fa70.l
pub fn i_0548b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x548BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x548BC);
    cpu.pc = 0x4FA70;
    16
}
/// $0548BC: bcc.w $548c6
pub fn i_0548bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x548C0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x548C6; return 10; }
    8
}
/// $0548C0: jmp $518.l
pub fn i_0548c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x548C6;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0548C6: rts
pub fn i_0548c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x548C8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_FragileBreaking_0548c8  ($0548C8, 68 B, props/mission_053f96.s)
/// $0548C8: move.w #$1054, d0
pub fn i_0548c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x548CC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1054;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0548CC: jsr $2352.l
pub fn i_0548cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x548D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x548D2);
    cpu.pc = 0x2352;
    16
}
/// $0548D2: lea.l $298b4a.l, a0
pub fn i_0548d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x548D8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x298B4A;
    20
}
/// $0548D8: jsr $28cd4.l
pub fn i_0548d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x548DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x548DE);
    cpu.pc = 0x28CD4;
    16
}
/// $0548DE: lea.l $548e4(pc), a1
pub fn i_0548de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x548E2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x548E4;
    16
}
/// $0548E2: move.l a1, (a6)
pub fn i_0548e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x548E4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0548E4: jsr $2783a.l
pub fn i_0548e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x548EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x548EA);
    cpu.pc = 0x2783A;
    16
}
/// $0548EA: jsr $28d70.l
pub fn i_0548ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x548F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x548F0);
    cpu.pc = 0x28D70;
    16
}
/// $0548F0: bcc.w $548fa
pub fn i_0548f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x548F4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x548FA; return 10; }
    8
}
/// $0548F4: lea.l $5490c(pc), a1
pub fn i_0548f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x548F8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5490C;
    16
}
/// $0548F8: move.l a1, (a6)
pub fn i_0548f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x548FA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0548FA: jsr $4fa70.l
pub fn i_0548fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54900;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54900);
    cpu.pc = 0x4FA70;
    16
}
/// $054900: bcc.w $5490a
pub fn i_054900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54904;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5490A; return 10; }
    8
}
/// $054904: jmp $518.l
pub fn i_054904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5490A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $05490A: rts
pub fn i_05490a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5490C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_FragileWreck_05490c  ($05490C, 78 B, props/mission_053f96.s)
/// $05490C: jsr $2783a.l
pub fn i_05490c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54912;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54912);
    cpu.pc = 0x2783A;
    16
}
/// $054912: move.l #$1000, d0
pub fn i_054912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54918;
    cpu.ir = 0x203C;
    let v1: u32 = 0x1000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $054918: jsr $51a28.l
pub fn i_054918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5491E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5491E);
    cpu.pc = 0x51A28;
    16
}
/// $05491E: lea.l $29b380.l, a1
pub fn i_05491e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54924;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29B380;
    20
}
/// $054924: jsr $43fac.l
pub fn i_054924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5492A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5492A);
    cpu.pc = 0x43FAC;
    16
}
/// $05492A: lea.l $298b78.l, a0
pub fn i_05492a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54930;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x298B78;
    20
}
/// $054930: jsr $28cd4.l
pub fn i_054930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54936;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54936);
    cpu.pc = 0x28CD4;
    16
}
/// $054936: lea.l $5493c(pc), a1
pub fn i_054936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5493A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5493C;
    16
}
/// $05493A: move.l a1, (a6)
pub fn i_05493a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5493C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05493C: jsr $2783a.l
pub fn i_05493c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54942;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54942);
    cpu.pc = 0x2783A;
    16
}
/// $054942: jsr $28d70.l
pub fn i_054942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54948;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54948);
    cpu.pc = 0x28D70;
    16
}
/// $054948: jsr $4fa70.l
pub fn i_054948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5494E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5494E);
    cpu.pc = 0x4FA70;
    16
}
/// $05494E: bcc.w $54958
pub fn i_05494e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54952;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54958; return 10; }
    8
}
/// $054952: jmp $518.l
pub fn i_054952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54958;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $054958: rts
pub fn i_054958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5495A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Breakable_05495a  ($05495A, 192 B, props/mission_053f96.s)
/// $05495A: movea.l $3c(a6), a1
pub fn i_05495a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5495E;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $05495E: jsr $2942a.l
pub fn i_05495e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54964;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54964);
    cpu.pc = 0x2942A;
    16
}
/// $054964: move.w #$93, d1
pub fn i_054964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54968;
    cpu.ir = 0x323C;
    let v1: u32 = 0x93;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $054968: jsr $236e.l
pub fn i_054968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5496E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5496E);
    cpu.pc = 0x236E;
    16
}
/// $05496E: move.w #$50, $70(a6)
pub fn i_05496e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54974;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x50;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $054974: move.b #$ff, $32(a6)
pub fn i_054974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5497A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05497A: move.b #$ff, $33(a6)
pub fn i_05497a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54980;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $054980: move.b #$0, $3a(a6)
pub fn i_054980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54986;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $054986: move.w #$1, $66(a6)
pub fn i_054986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5498C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05498C: lea.l $298b88.l, a0
pub fn i_05498c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54992;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x298B88;
    20
}
/// $054992: jsr $28cd4.l
pub fn i_054992(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54998;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54998);
    cpu.pc = 0x28CD4;
    16
}
/// $054998: lea.l $5499e(pc), a1
pub fn i_054998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5499C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5499E;
    16
}
/// $05499C: move.l a1, (a6)
pub fn i_05499c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5499E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05499E: jsr $2783a.l
pub fn i_05499e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x549A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x549A4);
    cpu.pc = 0x2783A;
    16
}
/// $0549A4: jsr $28d70.l
pub fn i_0549a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x549AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x549AA);
    cpu.pc = 0x28D70;
    16
}
/// $0549AA: jsr $2870a.l
pub fn i_0549aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x549B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x549B0);
    cpu.pc = 0x2870A;
    16
}
/// $0549B0: bcc.w $549ba
pub fn i_0549b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x549B4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x549BA; return 10; }
    8
}
/// $0549B4: bclr.b #$3, $13(a6)
pub fn i_0549b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x549BA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0549BA: jsr $28758.l
pub fn i_0549ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x549C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x549C0);
    cpu.pc = 0x28758;
    16
}
/// $0549C0: bcc.w $54a08
pub fn i_0549c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x549C4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54A08; return 10; }
    8
}
/// $0549C4: bclr.b #$0, $13(a6)
pub fn i_0549c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x549CA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0549CA: move.w #$102e, d0
pub fn i_0549ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x549CE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0549CE: jsr $2352.l
pub fn i_0549ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x549D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x549D4);
    cpu.pc = 0x2352;
    16
}
/// $0549D4: lea.l $29b358.l, a1
pub fn i_0549d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x549DA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29B358;
    20
}
/// $0549DA: jsr $43fac.l
pub fn i_0549da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x549E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x549E0);
    cpu.pc = 0x43FAC;
    16
}
/// $0549E0: lea.l $29a362.l, a1
pub fn i_0549e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x549E6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A362;
    20
}
/// $0549E6: jsr $77c7e.l
pub fn i_0549e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x549EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x549EC);
    cpu.pc = 0x77C7E;
    16
}
/// $0549EC: lea.l $29a374.l, a1
pub fn i_0549ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x549F2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A374;
    20
}
/// $0549F2: jsr $77c7e.l
pub fn i_0549f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x549F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x549F8);
    cpu.pc = 0x77C7E;
    16
}
/// $0549F8: lea.l $29a44c.l, a1
pub fn i_0549f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x549FE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A44C;
    20
}
/// $0549FE: jsr $77c7e.l
pub fn i_0549fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54A04);
    cpu.pc = 0x77C7E;
    16
}
/// $054A04: bra.w $54a12
pub fn i_054a04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A08;
    cpu.ir = 0x6000;
    cpu.pc = 0x54A12;
    10
}
/// $054A08: jsr $4fa70.l
pub fn i_054a08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54A0E);
    cpu.pc = 0x4FA70;
    16
}
/// $054A0E: bcc.w $54a18
pub fn i_054a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A12;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54A18; return 10; }
    8
}
/// $054A12: jmp $518.l
pub fn i_054a12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A18;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $054A18: rts
pub fn i_054a18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A1A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_HitStages_054a1a  ($054A1A, 258 B, props/mission_053f96.s)
/// $054A1A: movea.l $3c(a6), a1
pub fn i_054a1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A1E;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $054A1E: jsr $2942a.l
pub fn i_054a1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A24;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54A24);
    cpu.pc = 0x2942A;
    16
}
/// $054A24: move.w #$7a, d1
pub fn i_054a24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A28;
    cpu.ir = 0x323C;
    let v1: u32 = 0x7A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $054A28: jsr $236e.l
pub fn i_054a28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A2E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54A2E);
    cpu.pc = 0x236E;
    16
}
/// $054A2E: move.w #$40, $70(a6)
pub fn i_054a2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A34;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $054A34: move.b #$ff, $32(a6)
pub fn i_054a34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A3A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $054A3A: move.b #$ff, $33(a6)
pub fn i_054a3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A40;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $054A40: move.b #$0, $3a(a6)
pub fn i_054a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A46;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $054A46: move.w #$46, $66(a6)
pub fn i_054a46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A4C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x46;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $054A4C: move.b #$6, $44(a6)
pub fn i_054a4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A52;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x44);
    bus.write8(ad2, v1 as u8);
    12
}
/// $054A52: lea.l $298b9e.l, a0
pub fn i_054a52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A58;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x298B9E;
    20
}
/// $054A58: jsr $28cd4.l
pub fn i_054a58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A5E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54A5E);
    cpu.pc = 0x28CD4;
    16
}
/// $054A5E: lea.l $54a64(pc), a1
pub fn i_054a5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A62;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54A64;
    16
}
/// $054A62: move.l a1, (a6)
pub fn i_054a62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A64;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $054A64: jsr $2783a.l
pub fn i_054a64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54A6A);
    cpu.pc = 0x2783A;
    16
}
/// $054A6A: jsr $28d70.l
pub fn i_054a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A70;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54A70);
    cpu.pc = 0x28D70;
    16
}
/// $054A70: bcc.w $54a80
pub fn i_054a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A74;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54A80; return 10; }
    8
}
/// $054A74: lea.l $298b9e.l, a0
pub fn i_054a74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A7A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x298B9E;
    20
}
/// $054A7A: jsr $28cd4.l
pub fn i_054a7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54A80);
    cpu.pc = 0x28CD4;
    16
}
/// $054A80: jsr $2870a.l
pub fn i_054a80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54A86);
    cpu.pc = 0x2870A;
    16
}
/// $054A86: bcc.w $54abc
pub fn i_054a86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A8A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54ABC; return 10; }
    8
}
/// $054A8A: lea.l $5e766.l, a0
pub fn i_054a8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A90;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $054A90: jsr $5e770.l
pub fn i_054a90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54A96);
    cpu.pc = 0x5E770;
    16
}
/// $054A96: bclr.b #$3, $13(a6)
pub fn i_054a96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A9C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $054A9C: move.w $72(a6), d0
pub fn i_054a9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54AA0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $054AA0: movea.l #$298c46, a0
pub fn i_054aa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54AA6;
    cpu.ir = 0x207C;
    let v1: u32 = 0x298C46;
    cpu.a[0] = v1;
    4
}
/// $054AA6: lsl.w #$2, d0
pub fn i_054aa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54AA8;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $054AA8: movea.l (a0, d0.w), a0
pub fn i_054aa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54AAC;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $054AAC: cmpa.l #$ffffffff, a0
pub fn i_054aac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54AB2;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $054AB2: beq.w $54abc
pub fn i_054ab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54AB6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x54ABC; return 10; }
    8
}
/// $054AB6: jsr $28cd4.l
pub fn i_054ab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54ABC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54ABC);
    cpu.pc = 0x28CD4;
    16
}
/// $054ABC: jsr $28758.l
pub fn i_054abc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54AC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54AC2);
    cpu.pc = 0x28758;
    16
}
/// $054AC2: bcc.w $54b0a
pub fn i_054ac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54AC6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54B0A; return 10; }
    8
}
/// $054AC6: move.l #$300, d0
pub fn i_054ac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54ACC;
    cpu.ir = 0x203C;
    let v1: u32 = 0x300;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $054ACC: jsr $51a28.l
pub fn i_054acc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54AD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54AD2);
    cpu.pc = 0x51A28;
    16
}
/// $054AD2: bclr.b #$0, $13(a6)
pub fn i_054ad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54AD8;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $054AD8: move.w #$1023, d0
pub fn i_054ad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54ADC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1023;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $054ADC: jsr $2352.l
pub fn i_054adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54AE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54AE2);
    cpu.pc = 0x2352;
    16
}
/// $054AE2: lea.l $29a386.l, a1
pub fn i_054ae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54AE8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A386;
    20
}
/// $054AE8: jsr $77c7e.l
pub fn i_054ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54AEE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54AEE);
    cpu.pc = 0x77C7E;
    16
}
/// $054AEE: lea.l $29a398.l, a1
pub fn i_054aee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54AF4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A398;
    20
}
/// $054AF4: jsr $77c7e.l
pub fn i_054af4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54AFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54AFA);
    cpu.pc = 0x77C7E;
    16
}
/// $054AFA: lea.l $29a45e.l, a1
pub fn i_054afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B00;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A45E;
    20
}
/// $054B00: jsr $77c7e.l
pub fn i_054b00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B06;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54B06);
    cpu.pc = 0x77C7E;
    16
}
/// $054B06: bra.w $54b14
pub fn i_054b06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B0A;
    cpu.ir = 0x6000;
    cpu.pc = 0x54B14;
    10
}
/// $054B0A: jsr $4fa70.l
pub fn i_054b0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54B10);
    cpu.pc = 0x4FA70;
    16
}
/// $054B10: bcc.w $54b1a
pub fn i_054b10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B14;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54B1A; return 10; }
    8
}
/// $054B14: jmp $518.l
pub fn i_054b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B1A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $054B1A: rts
pub fn i_054b1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B1C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Small_054b1c  ($054B1C, 194 B, props/mission_053f96.s)
/// $054B1C: movea.l $3c(a6), a1
pub fn i_054b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B20;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $054B20: jsr $2942a.l
pub fn i_054b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54B26);
    cpu.pc = 0x2942A;
    16
}
/// $054B26: move.w #$94, d1
pub fn i_054b26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B2A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x94;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $054B2A: jsr $236e.l
pub fn i_054b2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54B30);
    cpu.pc = 0x236E;
    16
}
/// $054B30: move.w #$50, $70(a6)
pub fn i_054b30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B36;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x50;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $054B36: move.b #$ff, $32(a6)
pub fn i_054b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B3C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $054B3C: move.b #$ff, $33(a6)
pub fn i_054b3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B42;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $054B42: move.b #$0, $3a(a6)
pub fn i_054b42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B48;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $054B48: move.w #$28, $66(a6)
pub fn i_054b48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B4E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $054B4E: lea.l $299050.l, a0
pub fn i_054b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B54;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x299050;
    20
}
/// $054B54: jsr $28cd4.l
pub fn i_054b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B5A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54B5A);
    cpu.pc = 0x28CD4;
    16
}
/// $054B5A: lea.l $54b60(pc), a1
pub fn i_054b5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B5E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54B60;
    16
}
/// $054B5E: move.l a1, (a6)
pub fn i_054b5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B60;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $054B60: jsr $2783a.l
pub fn i_054b60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54B66);
    cpu.pc = 0x2783A;
    16
}
/// $054B66: jsr $28d70.l
pub fn i_054b66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54B6C);
    cpu.pc = 0x28D70;
    16
}
/// $054B6C: jsr $2870a.l
pub fn i_054b6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54B72);
    cpu.pc = 0x2870A;
    16
}
/// $054B72: bcc.w $54b88
pub fn i_054b72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B76;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54B88; return 10; }
    8
}
/// $054B76: lea.l $5e766.l, a0
pub fn i_054b76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B7C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $054B7C: jsr $5e770.l
pub fn i_054b7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54B82);
    cpu.pc = 0x5E770;
    16
}
/// $054B82: bclr.b #$3, $13(a6)
pub fn i_054b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B88;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $054B88: jsr $28758.l
pub fn i_054b88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B8E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54B8E);
    cpu.pc = 0x28758;
    16
}
/// $054B8E: bcc.w $54bcc
pub fn i_054b8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B92;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54BCC; return 10; }
    8
}
/// $054B92: bclr.b #$0, $13(a6)
pub fn i_054b92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B98;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $054B98: move.w #$102e, d0
pub fn i_054b98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B9C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $054B9C: jsr $2352.l
pub fn i_054b9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54BA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54BA2);
    cpu.pc = 0x2352;
    16
}
/// $054BA2: lea.l $29a3aa.l, a1
pub fn i_054ba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54BA8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A3AA;
    20
}
/// $054BA8: jsr $77c7e.l
pub fn i_054ba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54BAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54BAE);
    cpu.pc = 0x77C7E;
    16
}
/// $054BAE: lea.l $29a3bc.l, a1
pub fn i_054bae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54BB4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A3BC;
    20
}
/// $054BB4: jsr $77c7e.l
pub fn i_054bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54BBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54BBA);
    cpu.pc = 0x77C7E;
    16
}
/// $054BBA: lea.l $29a470.l, a1
pub fn i_054bba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54BC0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A470;
    20
}
/// $054BC0: jsr $77c7e.l
pub fn i_054bc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54BC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54BC6);
    cpu.pc = 0x77C7E;
    16
}
/// $054BC6: lea.l $54bde(pc), a1
pub fn i_054bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54BCA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54BDE;
    16
}
/// $054BCA: move.l a1, (a6)
pub fn i_054bca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54BCC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $054BCC: jsr $4fa70.l
pub fn i_054bcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54BD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54BD2);
    cpu.pc = 0x4FA70;
    16
}
/// $054BD2: bcc.w $54bdc
pub fn i_054bd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54BD6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54BDC; return 10; }
    8
}
/// $054BD6: jmp $518.l
pub fn i_054bd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54BDC;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $054BDC: rts
pub fn i_054bdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54BDE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_SmallWreck_054bde  ($054BDE, 48 B, props/mission_053f96.s)
/// $054BDE: lea.l $299066.l, a0
pub fn i_054bde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54BE4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x299066;
    20
}
/// $054BE4: jsr $28cd4.l
pub fn i_054be4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54BEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54BEA);
    cpu.pc = 0x28CD4;
    16
}
/// $054BEA: lea.l $54bf0(pc), a1
pub fn i_054bea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54BEE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54BF0;
    16
}
/// $054BEE: move.l a1, (a6)
pub fn i_054bee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54BF0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $054BF0: jsr $2783a.l
pub fn i_054bf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54BF6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54BF6);
    cpu.pc = 0x2783A;
    16
}
/// $054BF6: jsr $28d70.l
pub fn i_054bf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54BFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54BFC);
    cpu.pc = 0x28D70;
    16
}
/// $054BFC: jsr $4fa70.l
pub fn i_054bfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54C02);
    cpu.pc = 0x4FA70;
    16
}
/// $054C02: bcc.w $54c0c
pub fn i_054c02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C06;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54C0C; return 10; }
    8
}
/// $054C06: jmp $518.l
pub fn i_054c06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C0C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $054C0C: rts
pub fn i_054c0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C0E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_ScaledHP_054c0e  ($054C0E, 228 B, props/mission_053f96.s)
/// $054C0E: movea.l $3c(a6), a1
pub fn i_054c0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C12;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $054C12: jsr $2942a.l
pub fn i_054c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54C18);
    cpu.pc = 0x2942A;
    16
}
/// $054C18: jsr $2783a.l
pub fn i_054c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54C1E);
    cpu.pc = 0x2783A;
    16
}
/// $054C1E: lea.l $29b31a.l, a1
pub fn i_054c1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C24;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29B31A;
    20
}
/// $054C24: jsr $43fac.l
pub fn i_054c24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54C2A);
    cpu.pc = 0x43FAC;
    16
}
/// $054C2A: move.w #$60, $70(a6)
pub fn i_054c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C30;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $054C30: lea.l $2c0196.l, a0
pub fn i_054c30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C36;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C0196;
    20
}
/// $054C36: jsr $799de.l
pub fn i_054c36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54C3C);
    cpu.pc = 0x799DE;
    16
}
/// $054C3C: move.w d0, $66(a6)
pub fn i_054c3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C40;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $054C40: lea.l $298a52.l, a0
pub fn i_054c40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C46;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x298A52;
    20
}
/// $054C46: jsr $28cd4.l
pub fn i_054c46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54C4C);
    cpu.pc = 0x28CD4;
    16
}
/// $054C4C: lea.l $54c52(pc), a1
pub fn i_054c4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C50;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54C52;
    16
}
/// $054C50: move.l a1, (a6)
pub fn i_054c50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C52;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $054C52: clr.b $10e39a.l
pub fn i_054c52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C58;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $054C58: jsr $2783a.l
pub fn i_054c58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C5E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54C5E);
    cpu.pc = 0x2783A;
    16
}
/// $054C5E: jsr $28d70.l
pub fn i_054c5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C64;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54C64);
    cpu.pc = 0x28D70;
    16
}
/// $054C64: jsr $2870a.l
pub fn i_054c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54C6A);
    cpu.pc = 0x2870A;
    16
}
/// $054C6A: bcc.w $54c80
pub fn i_054c6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C6E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54C80; return 10; }
    8
}
/// $054C6E: lea.l $5e766.l, a0
pub fn i_054c6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C74;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $054C74: jsr $5e770.l
pub fn i_054c74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C7A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54C7A);
    cpu.pc = 0x5E770;
    16
}
/// $054C7A: bclr.b #$3, $13(a6)
pub fn i_054c7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C80;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $054C80: jsr $28758.l
pub fn i_054c80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54C86);
    cpu.pc = 0x28758;
    16
}
/// $054C86: bcc.w $54ce0
pub fn i_054c86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C8A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54CE0; return 10; }
    8
}
/// $054C8A: move.l #$1000, d0
pub fn i_054c8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C90;
    cpu.ir = 0x203C;
    let v1: u32 = 0x1000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $054C90: jsr $51a28.l
pub fn i_054c90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54C96);
    cpu.pc = 0x51A28;
    16
}
/// $054C96: move.w #$102b, d0
pub fn i_054c96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C9A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $054C9A: jsr $2352.l
pub fn i_054c9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54CA0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54CA0);
    cpu.pc = 0x2352;
    16
}
/// $054CA0: lea.l $29b32e.l, a1
pub fn i_054ca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54CA6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29B32E;
    20
}
/// $054CA6: jsr $43fac.l
pub fn i_054ca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54CAC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54CAC);
    cpu.pc = 0x43FAC;
    16
}
/// $054CAC: lea.l $29a606.l, a2
pub fn i_054cac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54CB2;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A606;
    20
}
/// $054CB2: jsr $5022a.l
pub fn i_054cb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54CB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54CB8);
    cpu.pc = 0x5022A;
    16
}
/// $054CB8: lea.l $29a308.l, a1
pub fn i_054cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54CBE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A308;
    20
}
/// $054CBE: jsr $77c7e.l
pub fn i_054cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54CC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54CC4);
    cpu.pc = 0x77C7E;
    16
}
/// $054CC4: lea.l $29a308.l, a1
pub fn i_054cc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54CCA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A308;
    20
}
/// $054CCA: jsr $77c7e.l
pub fn i_054cca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54CD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54CD0);
    cpu.pc = 0x77C7E;
    16
}
/// $054CD0: lea.l $29a416.l, a1
pub fn i_054cd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54CD6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A416;
    20
}
/// $054CD6: jsr $77c7e.l
pub fn i_054cd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54CDC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54CDC);
    cpu.pc = 0x77C7E;
    16
}
/// $054CDC: bra.w $54cea
pub fn i_054cdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54CE0;
    cpu.ir = 0x6000;
    cpu.pc = 0x54CEA;
    10
}
/// $054CE0: jsr $4fa70.l
pub fn i_054ce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54CE6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54CE6);
    cpu.pc = 0x4FA70;
    16
}
/// $054CE6: bcc.w $54cf0
pub fn i_054ce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54CEA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54CF0; return 10; }
    8
}
/// $054CEA: jmp $518.l
pub fn i_054cea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54CF0;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $054CF0: rts
pub fn i_054cf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54CF2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_MultiStage_054cf2  ($054CF2, 382 B, props/mission_053f96.s)
/// $054CF2: movea.l $3c(a6), a1
pub fn i_054cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54CF6;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $054CF6: jsr $2942a.l
pub fn i_054cf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54CFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54CFC);
    cpu.pc = 0x2942A;
    16
}
/// $054CFC: move.w #$0, $72(a6)
pub fn i_054cfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54D02;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $054D02: lea.l $2990b8.l, a0
pub fn i_054d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54D08;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2990B8;
    20
}
/// $054D08: jsr $28cd4.l
pub fn i_054d08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54D0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54D0E);
    cpu.pc = 0x28CD4;
    16
}
/// $054D0E: bra.w $54d8e
pub fn i_054d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54D12;
    cpu.ir = 0x6000;
    cpu.pc = 0x54D8E;
    10
}
/// $054D8E: move.w #$95, d1
pub fn i_054d8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54D92;
    cpu.ir = 0x323C;
    let v1: u32 = 0x95;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $054D92: jsr $236e.l
pub fn i_054d92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54D98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54D98);
    cpu.pc = 0x236E;
    16
}
/// $054D98: cmpi.w #$4, $72(a6)
pub fn i_054d98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54D9E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Word, false);
    16
}
/// $054D9E: beq.w $54dc2
pub fn i_054d9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54DA2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x54DC2; return 10; }
    8
}
/// $054DA2: lea.l $6293e.l, a1
pub fn i_054da2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54DA8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x6293E;
    20
}
/// $054DA8: jsr $4ae.l
pub fn i_054da8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54DAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54DAE);
    cpu.pc = 0x4AE;
    16
}
/// $054DAE: jsr $5dd22.l
pub fn i_054dae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54DB4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54DB4);
    cpu.pc = 0x5DD22;
    16
}
/// $054DB4: move.w $72(a6), d0
pub fn i_054db4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54DB8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $054DB8: move.b d0, $62(a0)
pub fn i_054db8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54DBC;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x62);
    bus.write8(ad2, v1 as u8);
    12
}
/// $054DBC: addi.w #$40, $22(a0)
pub fn i_054dbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54DC2;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $054DC2: move.w #$80, $70(a6)
pub fn i_054dc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54DC8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $054DC8: move.b #$ff, $32(a6)
pub fn i_054dc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54DCE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $054DCE: move.b #$ff, $33(a6)
pub fn i_054dce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54DD4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $054DD4: move.b #$0, $3a(a6)
pub fn i_054dd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54DDA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $054DDA: lea.l $2c0114.l, a0
pub fn i_054dda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54DE0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C0114;
    20
}
/// $054DE0: jsr $799de.l
pub fn i_054de0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54DE6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54DE6);
    cpu.pc = 0x799DE;
    16
}
/// $054DE6: move.w d0, $66(a6)
pub fn i_054de6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54DEA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $054DEA: addi.w #$140, $66(a6)
pub fn i_054dea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54DF0;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x66);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x140, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $054DF0: move.b #$0, $20(a6)
pub fn i_054df0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54DF6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $054DF6: move.w $72(a6), d0
pub fn i_054df6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54DFA;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $054DFA: cmpi.w #$4, $72(a6)
pub fn i_054dfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E00;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Word, false);
    16
}
/// $054E00: bne.w $54e0c
pub fn i_054e00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E04;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x54E0C; return 10; }
    8
}
/// $054E04: lea.l $ffff.w, a0
pub fn i_054e04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E08;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $054E08: move.l a0, $48(a6)
pub fn i_054e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E0C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $054E0C: lea.l $54e12(pc), a1
pub fn i_054e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E10;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54E12;
    16
}
/// $054E10: move.l a1, (a6)
pub fn i_054e10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E12;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $054E12: jsr $2783a.l
pub fn i_054e12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54E18);
    cpu.pc = 0x2783A;
    16
}
/// $054E18: jsr $28d70.l
pub fn i_054e18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54E1E);
    cpu.pc = 0x28D70;
    16
}
/// $054E1E: cmpi.w #$4, $72(a6)
pub fn i_054e1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E24;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Word, false);
    16
}
/// $054E24: beq.w $54e5e
pub fn i_054e24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E28;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x54E5E; return 10; }
    8
}
/// $054E28: jsr $2870a.l
pub fn i_054e28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E2E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54E2E);
    cpu.pc = 0x2870A;
    16
}
/// $054E2E: bcc.w $54e44
pub fn i_054e2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E32;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54E44; return 10; }
    8
}
/// $054E32: lea.l $5e766.l, a0
pub fn i_054e32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E38;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $054E38: jsr $5e770.l
pub fn i_054e38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54E3E);
    cpu.pc = 0x5E770;
    16
}
/// $054E3E: bclr.b #$3, $13(a6)
pub fn i_054e3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E44;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $054E44: cmpi.w #$d4, $66(a6)
pub fn i_054e44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E4A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xD4, Size::Word, false);
    16
}
/// $054E4A: bgt.w $54e5e
pub fn i_054e4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E4E;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x54E5E; return 10; }
    8
}
/// $054E4E: move.w #$1027, d0
pub fn i_054e4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E52;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1027;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $054E52: jsr $2352.l
pub fn i_054e52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E58;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54E58);
    cpu.pc = 0x2352;
    16
}
/// $054E58: lea.l $54e70(pc), a1
pub fn i_054e58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E5C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54E70;
    16
}
/// $054E5C: move.l a1, (a6)
pub fn i_054e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E5E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $054E5E: jsr $4fa70.l
pub fn i_054e5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E64;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54E64);
    cpu.pc = 0x4FA70;
    16
}
/// $054E64: bcc.w $54e6e
pub fn i_054e64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E68;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54E6E; return 10; }
    8
}
/// $054E68: jmp $518.l
pub fn i_054e68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E6E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $054E6E: rts
pub fn i_054e6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E70;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_MultiStageDamaged_054e70  ($054E70, 122 B, props/mission_053f96.s)
/// $054E70: move.w $72(a6), d0
pub fn i_054e70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E74;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $054E74: movea.l #$2990ce, a0
pub fn i_054e74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E7A;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2990CE;
    cpu.a[0] = v1;
    4
}
/// $054E7A: lsl.w #$2, d0
pub fn i_054e7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E7C;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $054E7C: movea.l (a0, d0.w), a0
pub fn i_054e7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E80;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $054E80: cmpa.l #$ffffffff, a0
pub fn i_054e80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E86;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $054E86: beq.w $54e90
pub fn i_054e86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E8A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x54E90; return 10; }
    8
}
/// $054E8A: jsr $28cd4.l
pub fn i_054e8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54E90);
    cpu.pc = 0x28CD4;
    16
}
/// $054E90: lea.l $54e96(pc), a1
pub fn i_054e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E94;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54E96;
    16
}
/// $054E94: move.l a1, (a6)
pub fn i_054e94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E96;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $054E96: jsr $2783a.l
pub fn i_054e96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54E9C);
    cpu.pc = 0x2783A;
    16
}
/// $054E9C: jsr $28d70.l
pub fn i_054e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54EA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54EA2);
    cpu.pc = 0x28D70;
    16
}
/// $054EA2: jsr $2870a.l
pub fn i_054ea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54EA8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54EA8);
    cpu.pc = 0x2870A;
    16
}
/// $054EA8: bcc.w $54ebe
pub fn i_054ea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54EAC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54EBE; return 10; }
    8
}
/// $054EAC: lea.l $5e766.l, a0
pub fn i_054eac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54EB2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $054EB2: jsr $5e770.l
pub fn i_054eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54EB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54EB8);
    cpu.pc = 0x5E770;
    16
}
/// $054EB8: bclr.b #$3, $13(a6)
pub fn i_054eb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54EBE;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $054EBE: cmpi.w #$6a, $66(a6)
pub fn i_054ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54EC4;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x6A, Size::Word, false);
    16
}
/// $054EC4: bgt.w $54ed8
pub fn i_054ec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54EC8;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x54ED8; return 10; }
    8
}
/// $054EC8: move.w #$1027, d0
pub fn i_054ec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54ECC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1027;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $054ECC: jsr $2352.l
pub fn i_054ecc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54ED2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54ED2);
    cpu.pc = 0x2352;
    16
}
/// $054ED2: lea.l $54eea(pc), a1
pub fn i_054ed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54ED6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54EEA;
    16
}
/// $054ED6: move.l a1, (a6)
pub fn i_054ed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54ED8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $054ED8: jsr $4fa70.l
pub fn i_054ed8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54EDE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54EDE);
    cpu.pc = 0x4FA70;
    16
}
/// $054EDE: bcc.w $54ee8
pub fn i_054ede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54EE2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54EE8; return 10; }
    8
}
/// $054EE2: jmp $518.l
pub fn i_054ee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54EE8;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $054EE8: rts
pub fn i_054ee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54EEA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_MultiStageCritical_054eea  ($054EEA, 128 B, props/mission_053f96.s)
/// $054EEA: move.w $72(a6), d0
pub fn i_054eea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54EEE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $054EEE: movea.l #$2990de, a0
pub fn i_054eee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54EF4;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2990DE;
    cpu.a[0] = v1;
    4
}
/// $054EF4: lsl.w #$2, d0
pub fn i_054ef4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54EF6;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $054EF6: movea.l (a0, d0.w), a0
pub fn i_054ef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54EFA;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $054EFA: cmpa.l #$ffffffff, a0
pub fn i_054efa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F00;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $054F00: beq.w $54f0a
pub fn i_054f00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F04;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x54F0A; return 10; }
    8
}
/// $054F04: jsr $28cd4.l
pub fn i_054f04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54F0A);
    cpu.pc = 0x28CD4;
    16
}
/// $054F0A: lea.l $54f10(pc), a1
pub fn i_054f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F0E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54F10;
    16
}
/// $054F0E: move.l a1, (a6)
pub fn i_054f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F10;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $054F10: jsr $2783a.l
pub fn i_054f10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F16;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54F16);
    cpu.pc = 0x2783A;
    16
}
/// $054F16: jsr $28d70.l
pub fn i_054f16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54F1C);
    cpu.pc = 0x28D70;
    16
}
/// $054F1C: jsr $2870a.l
pub fn i_054f1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F22;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54F22);
    cpu.pc = 0x2870A;
    16
}
/// $054F22: bcc.w $54f38
pub fn i_054f22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F26;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54F38; return 10; }
    8
}
/// $054F26: lea.l $5e766.l, a0
pub fn i_054f26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F2C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $054F2C: jsr $5e770.l
pub fn i_054f2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54F32);
    cpu.pc = 0x5E770;
    16
}
/// $054F32: bclr.b #$3, $13(a6)
pub fn i_054f32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F38;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $054F38: jsr $28758.l
pub fn i_054f38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54F3E);
    cpu.pc = 0x28758;
    16
}
/// $054F3E: bcc.w $54f58
pub fn i_054f3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F42;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54F58; return 10; }
    8
}
/// $054F42: bclr.b #$0, $13(a6)
pub fn i_054f42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F48;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $054F48: move.w #$1035, d0
pub fn i_054f48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F4C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1035;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $054F4C: jsr $2352.l
pub fn i_054f4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54F52);
    cpu.pc = 0x2352;
    16
}
/// $054F52: lea.l $54f6a(pc), a1
pub fn i_054f52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F56;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54F6A;
    16
}
/// $054F56: move.l a1, (a6)
pub fn i_054f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F58;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $054F58: jsr $4fa70.l
pub fn i_054f58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F5E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54F5E);
    cpu.pc = 0x4FA70;
    16
}
/// $054F5E: bcc.w $54f68
pub fn i_054f5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F62;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54F68; return 10; }
    8
}
/// $054F62: jmp $518.l
pub fn i_054f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F68;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $054F68: rts
pub fn i_054f68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F6A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_MultiStageWreck_054f6a  ($054F6A, 80 B, props/mission_053f96.s)
/// $054F6A: move.l #$4000, d0
pub fn i_054f6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F70;
    cpu.ir = 0x203C;
    let v1: u32 = 0x4000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $054F70: jsr $51a28.l
pub fn i_054f70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54F76);
    cpu.pc = 0x51A28;
    16
}
/// $054F76: move.w $72(a6), d0
pub fn i_054f76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F7A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $054F7A: movea.l #$2990ee, a0
pub fn i_054f7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F80;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2990EE;
    cpu.a[0] = v1;
    4
}
/// $054F80: lsl.w #$2, d0
pub fn i_054f80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F82;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $054F82: movea.l (a0, d0.w), a0
pub fn i_054f82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F86;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $054F86: cmpa.l #$ffffffff, a0
pub fn i_054f86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F8C;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $054F8C: beq.w $54f96
pub fn i_054f8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F90;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x54F96; return 10; }
    8
}
/// $054F90: jsr $28cd4.l
pub fn i_054f90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54F96);
    cpu.pc = 0x28CD4;
    16
}
/// $054F96: lea.l $54f9c(pc), a1
pub fn i_054f96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F9A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54F9C;
    16
}
/// $054F9A: move.l a1, (a6)
pub fn i_054f9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F9C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $054F9C: jsr $2783a.l
pub fn i_054f9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54FA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54FA2);
    cpu.pc = 0x2783A;
    16
}
/// $054FA2: jsr $28d70.l
pub fn i_054fa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54FA8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54FA8);
    cpu.pc = 0x28D70;
    16
}
/// $054FA8: jsr $4fa70.l
pub fn i_054fa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54FAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54FAE);
    cpu.pc = 0x4FA70;
    16
}
/// $054FAE: bcc.w $54fb8
pub fn i_054fae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54FB2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54FB8; return 10; }
    8
}
/// $054FB2: jmp $518.l
pub fn i_054fb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54FB8;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $054FB8: rts
pub fn i_054fb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54FBA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FixBlink2_PhaseA_054fba  ($054FBA, 72 B, props/mission_053f96.s)
/// $054FBA: move.w #$48, d1
pub fn i_054fba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54FBE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x48;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $054FBE: move.w #$79, d2
pub fn i_054fbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54FC2;
    cpu.ir = 0x343C;
    let v1: u32 = 0x79;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $054FC2: move.w #$2, d3
pub fn i_054fc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54FC6;
    cpu.ir = 0x363C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $054FC6: move.w #$1, d4
pub fn i_054fc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54FCA;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $054FCA: jsr $2c26.l
pub fn i_054fca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54FD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54FD0);
    cpu.pc = 0x2C26;
    16
}
/// $054FD0: move.w #$40, $72(a6)
pub fn i_054fd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54FD6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $054FD6: lea.l $54fdc(pc), a1
pub fn i_054fd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54FDA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54FDC;
    16
}
/// $054FDA: move.l a1, (a6)
pub fn i_054fda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54FDC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $054FDC: subq.w #$1, $72(a6)
pub fn i_054fdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54FE0;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $054FE0: bne.w $54fea
pub fn i_054fe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54FE4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x54FEA; return 10; }
    8
}
/// $054FE4: lea.l $55002(pc), a1
pub fn i_054fe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54FE8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x55002;
    16
}
/// $054FE8: move.l a1, (a6)
pub fn i_054fe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54FEA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $054FEA: move.l $106f5c.l, d0
pub fn i_054fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54FF0;
    cpu.ir = 0x2039;
    let v2 = bus.read32(0x106F5C);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $054FF0: swap d0
pub fn i_054ff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54FF2;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $054FF2: cmpi.w #$300, d0
pub fn i_054ff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54FF6;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x300, Size::Word, false);
    8
}
/// $054FF6: blt.w $55000
pub fn i_054ff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54FFA;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x55000; return 10; }
    8
}
/// $054FFA: jmp $518.l
pub fn i_054ffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55000;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $055000: rts
pub fn i_055000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55002;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FixBlink2_PhaseB_055002  ($055002, 72 B, props/mission_053f96.s)
/// $055002: move.w #$48, d1
pub fn i_055002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55006;
    cpu.ir = 0x323C;
    let v1: u32 = 0x48;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $055006: move.w #$78, d2
pub fn i_055006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5500A;
    cpu.ir = 0x343C;
    let v1: u32 = 0x78;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05500A: move.w #$1, d3
pub fn i_05500a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5500E;
    cpu.ir = 0x363C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05500E: move.w #$1, d4
pub fn i_05500e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55012;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $055012: jsr $2c26.l
pub fn i_055012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55018;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55018);
    cpu.pc = 0x2C26;
    16
}
/// $055018: move.w #$20, $72(a6)
pub fn i_055018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5501E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05501E: lea.l $55024(pc), a1
pub fn i_05501e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55022;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x55024;
    16
}
/// $055022: move.l a1, (a6)
pub fn i_055022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55024;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $055024: subq.w #$1, $72(a6)
pub fn i_055024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55028;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $055028: bne.w $55032
pub fn i_055028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5502C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x55032; return 10; }
    8
}
/// $05502C: lea.l $54fba(pc), a1
pub fn i_05502c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55030;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54FBA;
    16
}
/// $055030: move.l a1, (a6)
pub fn i_055030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55032;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $055032: move.l $106f5c.l, d0
pub fn i_055032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55038;
    cpu.ir = 0x2039;
    let v2 = bus.read32(0x106F5C);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $055038: swap d0
pub fn i_055038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5503A;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05503A: cmpi.w #$300, d0
pub fn i_05503a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5503E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x300, Size::Word, false);
    8
}
/// $05503E: blt.w $55048
pub fn i_05503e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55042;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x55048; return 10; }
    8
}
/// $055042: jmp $518.l
pub fn i_055042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55048;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $055048: rts
pub fn i_055048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5504A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Blocker_05504a  ($05504A, 82 B, props/mission_053f96.s)
/// $05504A: movea.l $3c(a6), a1
pub fn i_05504a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5504E;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $05504E: jsr $2942a.l
pub fn i_05504e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55054;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55054);
    cpu.pc = 0x2942A;
    16
}
/// $055054: move.w #$e, d1
pub fn i_055054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55058;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $055058: jsr $236e.l
pub fn i_055058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5505E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5505E);
    cpu.pc = 0x236E;
    16
}
/// $05505E: bset.b #$6, $12(a6)
pub fn i_05505e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55064;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $055064: move.w #$f000, $38(a6)
pub fn i_055064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5506A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05506A: move.w #$60, $70(a6)
pub fn i_05506a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55070;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $055070: move.l #$29b3c0, $60(a6)
pub fn i_055070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55078;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x29B3C0;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $055078: lea.l $5507e(pc), a1
pub fn i_055078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5507C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5507E;
    16
}
/// $05507C: move.l a1, (a6)
pub fn i_05507c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5507E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05507E: jsr $28998.l
pub fn i_05507e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55084;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55084);
    cpu.pc = 0x28998;
    16
}
/// $055084: jsr $2783a.l
pub fn i_055084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5508A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5508A);
    cpu.pc = 0x2783A;
    16
}
/// $05508A: jsr $4fa70.l
pub fn i_05508a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55090;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55090);
    cpu.pc = 0x4FA70;
    16
}
/// $055090: bcc.w $5509a
pub fn i_055090(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55094;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5509A; return 10; }
    8
}
/// $055094: jmp $518.l
pub fn i_055094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5509A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $05509A: rts
pub fn i_05509a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5509C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== NeonSign_FixUpdate_05509c  ($05509C, 34 B, props/mission_053f96.s)
/// $05509C: cmpi.b #$0, $74(a6)
pub fn i_05509c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x550A2;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $0550A2: bne.w $55146
pub fn i_0550a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x550A6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x55146; return 10; }
    8
}
/// $0550A6: move.b $1081b1.l, d0
pub fn i_0550a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x550AC;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x1081B1) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $0550AC: cmpi.b #$0, d0
pub fn i_0550ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x550B0;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $0550B0: beq.w $550c4
pub fn i_0550b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x550B4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x550C4; return 10; }
    8
}
/// $0550B4: jsr $551d0(pc)
pub fn i_0550b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x550B8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x550B8);
    cpu.pc = 0x551D0;
    16
}
/// $0550B8: move.b $1081b1.l, d0
pub fn i_0550b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x550BE;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x1081B1) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
// ===== SetTaskB_0550be  ($0550BE, 6 B, kernel/task_setters_b.c)
/// $0550BE: move.b d0, $74(a6)
pub fn i_0550be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x550C2;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0550C2: rts
pub fn i_0550c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x550C4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== NeonSign_FixByParentPhase_0550c4  ($0550C4, 68 B, props/mission_053f96.s)
/// $0550C4: movea.l $c(a6), a0
pub fn i_0550c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x550C8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0550C8: cmpi.b #$7f, $80(a0)
pub fn i_0550c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x550CE;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x7F, Size::Byte, false);
    16
}
/// $0550CE: beq.w $550e6
pub fn i_0550ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x550D2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x550E6; return 10; }
    8
}
/// $0550D2: cmpi.b #$ff, $80(a0)
pub fn i_0550d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x550D8;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $0550D8: bne.w $550e8
pub fn i_0550d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x550DC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x550E8; return 10; }
    8
}
/// $0550DC: jsr $551d0(pc)
pub fn i_0550dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x550E0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x550E0);
    cpu.pc = 0x551D0;
    16
}
/// $0550E0: move.b #$7f, $80(a0)
pub fn i_0550e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x550E6;
    cpu.ir = 0x117C;
    let v1: u32 = 0x7F;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0550E6: rts
pub fn i_0550e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x550E8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0550E8: movea.l $c(a6), a0
pub fn i_0550e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x550EC;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0550EC: cmpi.b #$ff, $21(a0)
pub fn i_0550ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x550F2;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $0550F2: beq.w $55146
pub fn i_0550f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x550F6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x55146; return 10; }
    8
}
/// $0550F6: addq.w #$1, $72(a6)
pub fn i_0550f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x550FA;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0550FA: movea.l $c(a6), a0
pub fn i_0550fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x550FE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0550FE: cmpi.b #$1, $21(a0)
pub fn i_0550fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55104;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $055104: bne.w $5510e
pub fn i_055104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55108;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5510E; return 10; }
    8
}
// ===== JsrPcThunk_055108  ($055108, 6 B, thunks/jsr_pc_thunks.c)
/// $055108: jsr $55214(pc)
pub fn i_055108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5510C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5510C);
    cpu.pc = 0x55214;
    16
}
/// $05510C: rts
pub fn i_05510c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5510E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== NeonSign_FixPhase2_05510e  ($05510E, 14 B, props/mission_053f96.s)
/// $05510E: movea.l $c(a6), a0
pub fn i_05510e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55112;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $055112: cmpi.b #$2, $21(a0)
pub fn i_055112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55118;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $055118: bne.w $55122
pub fn i_055118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5511C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x55122; return 10; }
    8
}
// ===== JsrPcThunk_05511c  ($05511C, 6 B, thunks/jsr_pc_thunks.c)
/// $05511C: jsr $55148(pc)
pub fn i_05511c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55120;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x55120);
    cpu.pc = 0x55148;
    16
}
/// $055120: rts
pub fn i_055120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55122;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== NeonSign_FixBlink_055122  ($055122, 32 B, props/mission_053f96.s)
/// $055122: move.w $72(a6), d0
pub fn i_055122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55126;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $055126: andi.w #$3, d0
pub fn i_055126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5512A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05512A: bne.w $55146
pub fn i_05512a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5512E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x55146; return 10; }
    8
}
/// $05512E: move.w $72(a6), d0
pub fn i_05512e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55132;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $055132: btst.b #$2, d0
pub fn i_055132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55136;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((2) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $055136: bne.w $55142
pub fn i_055136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5513A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x55142; return 10; }
    8
}
/// $05513A: jsr $5518c(pc)
pub fn i_05513a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5513E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5513E);
    cpu.pc = 0x5518C;
    16
}
/// $05513E: bra.w $55146
pub fn i_05513e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55142;
    cpu.ir = 0x6000;
    cpu.pc = 0x55146;
    10
}
// ===== JsrPcThunk_055142  ($055142, 6 B, thunks/jsr_pc_thunks.c)
/// $055142: jsr $55148(pc)
pub fn i_055142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55146;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x55146);
    cpu.pc = 0x55148;
    16
}
/// $055146: rts
pub fn i_055146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55148;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== NeonSign_TilesOffA_055148  ($055148, 60 B, props/mission_053f96.s)
/// $055148: move.w #$1a, d1
pub fn i_055148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5514C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05514C: move.w #$4b, d2
pub fn i_05514c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55150;
    cpu.ir = 0x343C;
    let v1: u32 = 0x4B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $055150: move.w #$ffff, d3
pub fn i_055150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55154;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $055154: move.w #$1, d4
pub fn i_055154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55158;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $055158: jsr $2c26.l
pub fn i_055158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5515E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5515E);
    cpu.pc = 0x2C26;
    16
}
/// $05515E: move.w #$1d, d1
pub fn i_05515e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55162;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $055162: move.w #$4e, d2
pub fn i_055162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55166;
    cpu.ir = 0x343C;
    let v1: u32 = 0x4E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $055166: move.w #$ffff, d3
pub fn i_055166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5516A;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05516A: move.w #$1, d4
pub fn i_05516a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5516E;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05516E: jsr $2c26.l
pub fn i_05516e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55174;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55174);
    cpu.pc = 0x2C26;
    16
}
/// $055174: move.w $14(a6), d1
pub fn i_055174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55178;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $055178: move.w #$8e, d2
pub fn i_055178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5517C;
    cpu.ir = 0x343C;
    let v1: u32 = 0x8E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05517C: move.w #$ffff, d3
pub fn i_05517c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55180;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $055180: move.w #$1, d4
pub fn i_055180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55184;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_055184  ($055184, 8 B, thunks/jsr_abs_thunks.c)
/// $055184: jsr $2c30.l
pub fn i_055184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5518A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5518A);
    cpu.pc = 0x2C30;
    16
}
/// $05518A: rts
pub fn i_05518a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5518C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== NeonSign_TilesOnA_05518c  ($05518C, 60 B, props/mission_053f96.s)
/// $05518C: move.w #$1a, d1
pub fn i_05518c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55190;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $055190: move.w #$4a, d2
pub fn i_055190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55194;
    cpu.ir = 0x343C;
    let v1: u32 = 0x4A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $055194: move.w #$ffff, d3
pub fn i_055194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55198;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $055198: move.w #$1, d4
pub fn i_055198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5519C;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05519C: jsr $2c26.l
pub fn i_05519c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x551A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x551A2);
    cpu.pc = 0x2C26;
    16
}
/// $0551A2: move.w #$1d, d1
pub fn i_0551a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x551A6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0551A6: move.w #$4d, d2
pub fn i_0551a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x551AA;
    cpu.ir = 0x343C;
    let v1: u32 = 0x4D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0551AA: move.w #$ffff, d3
pub fn i_0551aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x551AE;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0551AE: move.w #$1, d4
pub fn i_0551ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x551B2;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0551B2: jsr $2c26.l
pub fn i_0551b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x551B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x551B8);
    cpu.pc = 0x2C26;
    16
}
/// $0551B8: move.w $14(a6), d1
pub fn i_0551b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x551BC;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0551BC: move.w #$8d, d2
pub fn i_0551bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x551C0;
    cpu.ir = 0x343C;
    let v1: u32 = 0x8D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0551C0: move.w #$ffff, d3
pub fn i_0551c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x551C4;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0551C4: move.w #$1, d4
pub fn i_0551c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x551C8;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_0551c8  ($0551C8, 8 B, thunks/jsr_abs_thunks.c)
/// $0551C8: jsr $2c30.l
pub fn i_0551c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x551CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x551CE);
    cpu.pc = 0x2C30;
    16
}
/// $0551CE: rts
pub fn i_0551ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x551D0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== NeonSign_TilesDark_0551d0  ($0551D0, 60 B, props/mission_053f96.s)
/// $0551D0: move.w #$1a, d1
pub fn i_0551d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x551D4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0551D4: move.w #$bd, d2
pub fn i_0551d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x551D8;
    cpu.ir = 0x343C;
    let v1: u32 = 0xBD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0551D8: move.w #$1, d3
pub fn i_0551d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x551DC;
    cpu.ir = 0x363C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0551DC: move.w #$2, d4
pub fn i_0551dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x551E0;
    cpu.ir = 0x383C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0551E0: jsr $2c26.l
pub fn i_0551e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x551E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x551E6);
    cpu.pc = 0x2C26;
    16
}
/// $0551E6: move.w #$1d, d1
pub fn i_0551e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x551EA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0551EA: move.w #$be, d2
pub fn i_0551ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x551EE;
    cpu.ir = 0x343C;
    let v1: u32 = 0xBE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0551EE: move.w #$1, d3
pub fn i_0551ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x551F2;
    cpu.ir = 0x363C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0551F2: move.w #$2, d4
pub fn i_0551f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x551F6;
    cpu.ir = 0x383C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0551F6: jsr $2c26.l
pub fn i_0551f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x551FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x551FC);
    cpu.pc = 0x2C26;
    16
}
/// $0551FC: move.w $14(a6), d1
pub fn i_0551fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55200;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $055200: move.w #$123, d2
pub fn i_055200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55204;
    cpu.ir = 0x343C;
    let v1: u32 = 0x123;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $055204: move.w #$1, d3
pub fn i_055204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55208;
    cpu.ir = 0x363C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $055208: move.w #$2, d4
pub fn i_055208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5520C;
    cpu.ir = 0x383C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_05520c  ($05520C, 8 B, thunks/jsr_abs_thunks.c)
/// $05520C: jsr $2c30.l
pub fn i_05520c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55212;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55212);
    cpu.pc = 0x2C30;
    16
}
/// $055212: rts
pub fn i_055212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55214;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== NeonSign_TilesOnB_055214  ($055214, 60 B, props/mission_053f96.s)
/// $055214: move.w #$1a, d1
pub fn i_055214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55218;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $055218: move.w #$4c, d2
pub fn i_055218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5521C;
    cpu.ir = 0x343C;
    let v1: u32 = 0x4C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05521C: move.w #$1, d3
pub fn i_05521c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55220;
    cpu.ir = 0x363C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $055220: move.w #$1, d4
pub fn i_055220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55224;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $055224: jsr $2c26.l
pub fn i_055224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5522A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5522A);
    cpu.pc = 0x2C26;
    16
}
/// $05522A: move.w #$1d, d1
pub fn i_05522a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5522E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05522E: move.w #$4f, d2
pub fn i_05522e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55232;
    cpu.ir = 0x343C;
    let v1: u32 = 0x4F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $055232: move.w #$1, d3
pub fn i_055232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55236;
    cpu.ir = 0x363C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $055236: move.w #$1, d4
pub fn i_055236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5523A;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05523A: jsr $2c26.l
pub fn i_05523a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55240;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55240);
    cpu.pc = 0x2C26;
    16
}
/// $055240: move.w $14(a6), d1
pub fn i_055240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55244;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $055244: move.w #$8f, d2
pub fn i_055244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55248;
    cpu.ir = 0x343C;
    let v1: u32 = 0x8F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $055248: move.w #$1, d3
pub fn i_055248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5524C;
    cpu.ir = 0x363C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05524C: move.w #$1, d4
pub fn i_05524c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55250;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_055250  ($055250, 8 B, thunks/jsr_abs_thunks.c)
/// $055250: jsr $2c30.l
pub fn i_055250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55256;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55256);
    cpu.pc = 0x2C30;
    16
}
/// $055256: rts
pub fn i_055256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55258;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055258  ($055258, 14 B, kernel/state_dispatch_stubs.c)
/// $055258: lea.l $29a552.l, a2
pub fn i_055258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5525E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A552;
    20
}
/// $05525E: jsr $5022a.l
pub fn i_05525e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55264;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55264);
    cpu.pc = 0x5022A;
    16
}
/// $055264: rts
pub fn i_055264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55266;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055266  ($055266, 14 B, kernel/state_dispatch_stubs.c)
/// $055266: lea.l $29a566.l, a2
pub fn i_055266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5526C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A566;
    20
}
/// $05526C: jsr $5022a.l
pub fn i_05526c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55272;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55272);
    cpu.pc = 0x5022A;
    16
}
/// $055272: rts
pub fn i_055272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55274;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055274  ($055274, 14 B, kernel/state_dispatch_stubs.c)
/// $055274: lea.l $29a57a.l, a2
pub fn i_055274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5527A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A57A;
    20
}
/// $05527A: jsr $5022a.l
pub fn i_05527a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55280;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55280);
    cpu.pc = 0x5022A;
    16
}
/// $055280: rts
pub fn i_055280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55282;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055282  ($055282, 14 B, kernel/state_dispatch_stubs.c)
/// $055282: lea.l $29a58e.l, a2
pub fn i_055282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55288;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A58E;
    20
}
/// $055288: jsr $5022a.l
pub fn i_055288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5528E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5528E);
    cpu.pc = 0x5022A;
    16
}
/// $05528E: rts
pub fn i_05528e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55290;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055290  ($055290, 14 B, kernel/state_dispatch_stubs.c)
/// $055290: lea.l $29a5a2.l, a2
pub fn i_055290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55296;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A5A2;
    20
}
/// $055296: jsr $5022a.l
pub fn i_055296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5529C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5529C);
    cpu.pc = 0x5022A;
    16
}
/// $05529C: rts
pub fn i_05529c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5529E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05529e  ($05529E, 14 B, kernel/state_dispatch_stubs.c)
/// $05529E: lea.l $29a5b6.l, a2
pub fn i_05529e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x552A4;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A5B6;
    20
}
/// $0552A4: jsr $5022a.l
pub fn i_0552a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x552AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x552AA);
    cpu.pc = 0x5022A;
    16
}
/// $0552AA: rts
pub fn i_0552aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x552AC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0552ac  ($0552AC, 14 B, kernel/state_dispatch_stubs.c)
/// $0552AC: lea.l $29a5ca.l, a2
pub fn i_0552ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x552B2;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A5CA;
    20
}
/// $0552B2: jsr $5022a.l
pub fn i_0552b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x552B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x552B8);
    cpu.pc = 0x5022A;
    16
}
/// $0552B8: rts
pub fn i_0552b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x552BA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0552ba  ($0552BA, 14 B, kernel/state_dispatch_stubs.c)
/// $0552BA: lea.l $29a5de.l, a2
pub fn i_0552ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x552C0;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A5DE;
    20
}
/// $0552C0: jsr $5022a.l
pub fn i_0552c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x552C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x552C6);
    cpu.pc = 0x5022A;
    16
}
/// $0552C6: rts
pub fn i_0552c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x552C8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0552c8  ($0552C8, 14 B, kernel/state_dispatch_stubs.c)
/// $0552C8: lea.l $29a5f2.l, a2
pub fn i_0552c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x552CE;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A5F2;
    20
}
/// $0552CE: jsr $5022a.l
pub fn i_0552ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x552D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x552D4);
    cpu.pc = 0x5022A;
    16
}
/// $0552D4: rts
pub fn i_0552d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x552D6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0552d6  ($0552D6, 14 B, kernel/state_dispatch_stubs.c)
/// $0552D6: lea.l $29a61a.l, a2
pub fn i_0552d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x552DC;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A61A;
    20
}
/// $0552DC: jsr $5022a.l
pub fn i_0552dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x552E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x552E2);
    cpu.pc = 0x5022A;
    16
}
/// $0552E2: rts
pub fn i_0552e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x552E4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0552e4  ($0552E4, 14 B, kernel/state_dispatch_stubs.c)
/// $0552E4: lea.l $29a62e.l, a2
pub fn i_0552e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x552EA;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A62E;
    20
}
/// $0552EA: jsr $5022a.l
pub fn i_0552ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x552F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x552F0);
    cpu.pc = 0x5022A;
    16
}
/// $0552F0: rts
pub fn i_0552f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x552F2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0552f2  ($0552F2, 14 B, kernel/state_dispatch_stubs.c)
/// $0552F2: lea.l $29a642.l, a2
pub fn i_0552f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x552F8;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A642;
    20
}
/// $0552F8: jsr $5022a.l
pub fn i_0552f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x552FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x552FE);
    cpu.pc = 0x5022A;
    16
}
/// $0552FE: rts
pub fn i_0552fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55300;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055300  ($055300, 14 B, kernel/state_dispatch_stubs.c)
/// $055300: lea.l $29a656.l, a2
pub fn i_055300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55306;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A656;
    20
}
/// $055306: jsr $5022a.l
pub fn i_055306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5530C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5530C);
    cpu.pc = 0x5022A;
    16
}
/// $05530C: rts
pub fn i_05530c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5530E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05530e  ($05530E, 14 B, kernel/state_dispatch_stubs.c)
/// $05530E: lea.l $29a66a.l, a2
pub fn i_05530e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55314;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A66A;
    20
}
/// $055314: jsr $5022a.l
pub fn i_055314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5531A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5531A);
    cpu.pc = 0x5022A;
    16
}
/// $05531A: rts
pub fn i_05531a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5531C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05531c  ($05531C, 14 B, kernel/state_dispatch_stubs.c)
/// $05531C: lea.l $29a67e.l, a2
pub fn i_05531c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55322;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A67E;
    20
}
/// $055322: jsr $5022a.l
pub fn i_055322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55328;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55328);
    cpu.pc = 0x5022A;
    16
}
/// $055328: rts
pub fn i_055328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5532A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05532a  ($05532A, 14 B, kernel/state_dispatch_stubs.c)
/// $05532A: lea.l $29a692.l, a2
pub fn i_05532a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55330;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A692;
    20
}
/// $055330: jsr $5022a.l
pub fn i_055330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55336;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55336);
    cpu.pc = 0x5022A;
    16
}
/// $055336: rts
pub fn i_055336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55338;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055338  ($055338, 14 B, kernel/state_dispatch_stubs.c)
/// $055338: lea.l $29a6a6.l, a2
pub fn i_055338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5533E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A6A6;
    20
}
/// $05533E: jsr $5022a.l
pub fn i_05533e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55344;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55344);
    cpu.pc = 0x5022A;
    16
}
/// $055344: rts
pub fn i_055344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55346;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055346  ($055346, 14 B, kernel/state_dispatch_stubs.c)
/// $055346: lea.l $29a6ba.l, a2
pub fn i_055346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5534C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A6BA;
    20
}
/// $05534C: jsr $5022a.l
pub fn i_05534c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55352;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55352);
    cpu.pc = 0x5022A;
    16
}
/// $055352: rts
pub fn i_055352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55354;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055354  ($055354, 14 B, kernel/state_dispatch_stubs.c)
/// $055354: lea.l $29a6ce.l, a2
pub fn i_055354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5535A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A6CE;
    20
}
/// $05535A: jsr $5022a.l
pub fn i_05535a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55360;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55360);
    cpu.pc = 0x5022A;
    16
}
/// $055360: rts
pub fn i_055360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55362;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055362  ($055362, 14 B, kernel/state_dispatch_stubs.c)
/// $055362: lea.l $29a6e2.l, a2
pub fn i_055362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55368;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A6E2;
    20
}
/// $055368: jsr $5022a.l
pub fn i_055368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5536E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5536E);
    cpu.pc = 0x5022A;
    16
}
/// $05536E: rts
pub fn i_05536e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55370;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055370  ($055370, 14 B, kernel/state_dispatch_stubs.c)
/// $055370: lea.l $29a6f6.l, a2
pub fn i_055370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55376;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A6F6;
    20
}
/// $055376: jsr $5022a.l
pub fn i_055376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5537C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5537C);
    cpu.pc = 0x5022A;
    16
}
/// $05537C: rts
pub fn i_05537c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5537E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05537e  ($05537E, 14 B, kernel/state_dispatch_stubs.c)
/// $05537E: lea.l $29a70a.l, a2
pub fn i_05537e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55384;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A70A;
    20
}
/// $055384: jsr $5022a.l
pub fn i_055384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5538A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5538A);
    cpu.pc = 0x5022A;
    16
}
/// $05538A: rts
pub fn i_05538a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5538C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05538c  ($05538C, 14 B, kernel/state_dispatch_stubs.c)
/// $05538C: lea.l $29a71e.l, a2
pub fn i_05538c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55392;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A71E;
    20
}
/// $055392: jsr $5022a.l
pub fn i_055392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55398;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55398);
    cpu.pc = 0x5022A;
    16
}
/// $055398: rts
pub fn i_055398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5539A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05539a  ($05539A, 14 B, kernel/state_dispatch_stubs.c)
/// $05539A: lea.l $29a732.l, a2
pub fn i_05539a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x553A0;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A732;
    20
}
/// $0553A0: jsr $5022a.l
pub fn i_0553a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x553A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x553A6);
    cpu.pc = 0x5022A;
    16
}
/// $0553A6: rts
pub fn i_0553a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x553A8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0553a8  ($0553A8, 14 B, kernel/state_dispatch_stubs.c)
/// $0553A8: lea.l $29a746.l, a2
pub fn i_0553a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x553AE;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A746;
    20
}
/// $0553AE: jsr $5022a.l
pub fn i_0553ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x553B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x553B4);
    cpu.pc = 0x5022A;
    16
}
/// $0553B4: rts
pub fn i_0553b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x553B6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0553b6  ($0553B6, 14 B, kernel/state_dispatch_stubs.c)
/// $0553B6: lea.l $29a75a.l, a2
pub fn i_0553b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x553BC;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A75A;
    20
}
/// $0553BC: jsr $5022a.l
pub fn i_0553bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x553C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x553C2);
    cpu.pc = 0x5022A;
    16
}
/// $0553C2: rts
pub fn i_0553c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x553C4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0553c4  ($0553C4, 14 B, kernel/state_dispatch_stubs.c)
/// $0553C4: lea.l $29a76e.l, a2
pub fn i_0553c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x553CA;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A76E;
    20
}
/// $0553CA: jsr $5022a.l
pub fn i_0553ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x553D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x553D0);
    cpu.pc = 0x5022A;
    16
}
/// $0553D0: rts
pub fn i_0553d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x553D2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0553d2  ($0553D2, 14 B, kernel/state_dispatch_stubs.c)
/// $0553D2: lea.l $29a782.l, a2
pub fn i_0553d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x553D8;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A782;
    20
}
/// $0553D8: jsr $5022a.l
pub fn i_0553d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x553DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x553DE);
    cpu.pc = 0x5022A;
    16
}
/// $0553DE: rts
pub fn i_0553de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x553E0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0553e0  ($0553E0, 14 B, kernel/state_dispatch_stubs.c)
/// $0553E0: lea.l $29a796.l, a2
pub fn i_0553e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x553E6;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A796;
    20
}
/// $0553E6: jsr $5022a.l
pub fn i_0553e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x553EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x553EC);
    cpu.pc = 0x5022A;
    16
}
/// $0553EC: rts
pub fn i_0553ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x553EE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0553ee  ($0553EE, 14 B, kernel/state_dispatch_stubs.c)
/// $0553EE: lea.l $29a7aa.l, a2
pub fn i_0553ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x553F4;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A7AA;
    20
}
/// $0553F4: jsr $5022a.l
pub fn i_0553f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x553FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x553FA);
    cpu.pc = 0x5022A;
    16
}
/// $0553FA: rts
pub fn i_0553fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x553FC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0553fc  ($0553FC, 14 B, kernel/state_dispatch_stubs.c)
/// $0553FC: lea.l $29a7be.l, a2
pub fn i_0553fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55402;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A7BE;
    20
}
/// $055402: jsr $5022a.l
pub fn i_055402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55408;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55408);
    cpu.pc = 0x5022A;
    16
}
/// $055408: rts
pub fn i_055408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5540A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05540a  ($05540A, 14 B, kernel/state_dispatch_stubs.c)
/// $05540A: lea.l $29a7d2.l, a2
pub fn i_05540a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55410;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A7D2;
    20
}
/// $055410: jsr $5022a.l
pub fn i_055410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55416;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55416);
    cpu.pc = 0x5022A;
    16
}
/// $055416: rts
pub fn i_055416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55418;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055418  ($055418, 14 B, kernel/state_dispatch_stubs.c)
/// $055418: lea.l $29a7e6.l, a2
pub fn i_055418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5541E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A7E6;
    20
}
/// $05541E: jsr $5022a.l
pub fn i_05541e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55424;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55424);
    cpu.pc = 0x5022A;
    16
}
/// $055424: rts
pub fn i_055424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55426;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055426  ($055426, 14 B, kernel/state_dispatch_stubs.c)
/// $055426: lea.l $29a7fa.l, a2
pub fn i_055426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5542C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A7FA;
    20
}
/// $05542C: jsr $5022a.l
pub fn i_05542c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55432;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55432);
    cpu.pc = 0x5022A;
    16
}
/// $055432: rts
pub fn i_055432(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55434;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055434  ($055434, 14 B, kernel/state_dispatch_stubs.c)
/// $055434: lea.l $29a80e.l, a2
pub fn i_055434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5543A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A80E;
    20
}
/// $05543A: jsr $5022a.l
pub fn i_05543a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55440;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55440);
    cpu.pc = 0x5022A;
    16
}
/// $055440: rts
pub fn i_055440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55442;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055442  ($055442, 14 B, kernel/state_dispatch_stubs.c)
/// $055442: lea.l $29a822.l, a2
pub fn i_055442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55448;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A822;
    20
}
/// $055448: jsr $5022a.l
pub fn i_055448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5544E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5544E);
    cpu.pc = 0x5022A;
    16
}
/// $05544E: rts
pub fn i_05544e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55450;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055450  ($055450, 14 B, kernel/state_dispatch_stubs.c)
/// $055450: lea.l $29a836.l, a2
pub fn i_055450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55456;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A836;
    20
}
/// $055456: jsr $5022a.l
pub fn i_055456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5545C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5545C);
    cpu.pc = 0x5022A;
    16
}
/// $05545C: rts
pub fn i_05545c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5545E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05545e  ($05545E, 14 B, kernel/state_dispatch_stubs.c)
/// $05545E: lea.l $29a84a.l, a2
pub fn i_05545e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55464;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A84A;
    20
}
/// $055464: jsr $5022a.l
pub fn i_055464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5546A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5546A);
    cpu.pc = 0x5022A;
    16
}
/// $05546A: rts
pub fn i_05546a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5546C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05546c  ($05546C, 14 B, kernel/state_dispatch_stubs.c)
/// $05546C: lea.l $29a85e.l, a2
pub fn i_05546c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55472;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A85E;
    20
}
/// $055472: jsr $5022a.l
pub fn i_055472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55478;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55478);
    cpu.pc = 0x5022A;
    16
}
/// $055478: rts
pub fn i_055478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5547A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05547a  ($05547A, 14 B, kernel/state_dispatch_stubs.c)
/// $05547A: lea.l $29a872.l, a2
pub fn i_05547a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55480;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A872;
    20
}
/// $055480: jsr $5022a.l
pub fn i_055480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55486;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55486);
    cpu.pc = 0x5022A;
    16
}
/// $055486: rts
pub fn i_055486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55488;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055488  ($055488, 14 B, kernel/state_dispatch_stubs.c)
/// $055488: lea.l $29a886.l, a2
pub fn i_055488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5548E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A886;
    20
}
/// $05548E: jsr $5022a.l
pub fn i_05548e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55494;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55494);
    cpu.pc = 0x5022A;
    16
}
/// $055494: rts
pub fn i_055494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55496;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055496  ($055496, 14 B, kernel/state_dispatch_stubs.c)
/// $055496: lea.l $29a89a.l, a2
pub fn i_055496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5549C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A89A;
    20
}
/// $05549C: jsr $5022a.l
pub fn i_05549c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x554A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x554A2);
    cpu.pc = 0x5022A;
    16
}
/// $0554A2: rts
pub fn i_0554a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x554A4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0554a4  ($0554A4, 14 B, kernel/state_dispatch_stubs.c)
/// $0554A4: lea.l $29a8ae.l, a2
pub fn i_0554a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x554AA;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A8AE;
    20
}
/// $0554AA: jsr $5022a.l
pub fn i_0554aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x554B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x554B0);
    cpu.pc = 0x5022A;
    16
}
/// $0554B0: rts
pub fn i_0554b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x554B2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0554b2  ($0554B2, 14 B, kernel/state_dispatch_stubs.c)
/// $0554B2: lea.l $29a8c2.l, a2
pub fn i_0554b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x554B8;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A8C2;
    20
}
/// $0554B8: jsr $5022a.l
pub fn i_0554b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x554BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x554BE);
    cpu.pc = 0x5022A;
    16
}
/// $0554BE: rts
pub fn i_0554be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x554C0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0554c0  ($0554C0, 14 B, kernel/state_dispatch_stubs.c)
/// $0554C0: lea.l $29a8d6.l, a2
pub fn i_0554c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x554C6;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A8D6;
    20
}
/// $0554C6: jsr $5022a.l
pub fn i_0554c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x554CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x554CC);
    cpu.pc = 0x5022A;
    16
}
/// $0554CC: rts
pub fn i_0554cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x554CE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0554ce  ($0554CE, 14 B, kernel/state_dispatch_stubs.c)
/// $0554CE: lea.l $29a8ea.l, a2
pub fn i_0554ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x554D4;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A8EA;
    20
}
/// $0554D4: jsr $5022a.l
pub fn i_0554d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x554DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x554DA);
    cpu.pc = 0x5022A;
    16
}
/// $0554DA: rts
pub fn i_0554da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x554DC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0554dc  ($0554DC, 14 B, kernel/state_dispatch_stubs.c)
/// $0554DC: lea.l $29a8fe.l, a2
pub fn i_0554dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x554E2;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A8FE;
    20
}
/// $0554E2: jsr $5022a.l
pub fn i_0554e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x554E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x554E8);
    cpu.pc = 0x5022A;
    16
}
/// $0554E8: rts
pub fn i_0554e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x554EA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0554ea  ($0554EA, 14 B, kernel/state_dispatch_stubs.c)
/// $0554EA: lea.l $29a912.l, a2
pub fn i_0554ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x554F0;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A912;
    20
}
/// $0554F0: jsr $5022a.l
pub fn i_0554f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x554F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x554F6);
    cpu.pc = 0x5022A;
    16
}
/// $0554F6: rts
pub fn i_0554f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x554F8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0554f8  ($0554F8, 14 B, kernel/state_dispatch_stubs.c)
/// $0554F8: lea.l $29a926.l, a2
pub fn i_0554f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x554FE;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A926;
    20
}
/// $0554FE: jsr $5022a.l
pub fn i_0554fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55504;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55504);
    cpu.pc = 0x5022A;
    16
}
/// $055504: rts
pub fn i_055504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55506;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055506  ($055506, 14 B, kernel/state_dispatch_stubs.c)
/// $055506: lea.l $29a93a.l, a2
pub fn i_055506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5550C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A93A;
    20
}
/// $05550C: jsr $5022a.l
pub fn i_05550c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55512;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55512);
    cpu.pc = 0x5022A;
    16
}
/// $055512: rts
pub fn i_055512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55514;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055514  ($055514, 14 B, kernel/state_dispatch_stubs.c)
/// $055514: lea.l $29a94e.l, a2
pub fn i_055514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5551A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A94E;
    20
}
/// $05551A: jsr $5022a.l
pub fn i_05551a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55520;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55520);
    cpu.pc = 0x5022A;
    16
}
/// $055520: rts
pub fn i_055520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55522;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055522  ($055522, 14 B, kernel/state_dispatch_stubs.c)
/// $055522: lea.l $29a962.l, a2
pub fn i_055522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55528;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A962;
    20
}
/// $055528: jsr $5022a.l
pub fn i_055528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5552E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5552E);
    cpu.pc = 0x5022A;
    16
}
/// $05552E: rts
pub fn i_05552e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55530;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055530  ($055530, 14 B, kernel/state_dispatch_stubs.c)
/// $055530: lea.l $29a976.l, a2
pub fn i_055530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55536;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A976;
    20
}
/// $055536: jsr $5022a.l
pub fn i_055536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5553C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5553C);
    cpu.pc = 0x5022A;
    16
}
/// $05553C: rts
pub fn i_05553c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5553E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05553e  ($05553E, 14 B, kernel/state_dispatch_stubs.c)
/// $05553E: lea.l $29a98a.l, a2
pub fn i_05553e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55544;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A98A;
    20
}
/// $055544: jsr $5022a.l
pub fn i_055544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5554A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5554A);
    cpu.pc = 0x5022A;
    16
}
/// $05554A: rts
pub fn i_05554a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5554C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05554c  ($05554C, 14 B, kernel/state_dispatch_stubs.c)
/// $05554C: lea.l $29a99e.l, a2
pub fn i_05554c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55552;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A99E;
    20
}
/// $055552: jsr $5022a.l
pub fn i_055552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55558;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55558);
    cpu.pc = 0x5022A;
    16
}
/// $055558: rts
pub fn i_055558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5555A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05555a  ($05555A, 14 B, kernel/state_dispatch_stubs.c)
/// $05555A: lea.l $29a9b2.l, a2
pub fn i_05555a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55560;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A9B2;
    20
}
/// $055560: jsr $5022a.l
pub fn i_055560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55566;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55566);
    cpu.pc = 0x5022A;
    16
}
/// $055566: rts
pub fn i_055566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55568;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055568  ($055568, 14 B, kernel/state_dispatch_stubs.c)
/// $055568: lea.l $29a9c6.l, a2
pub fn i_055568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5556E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A9C6;
    20
}
/// $05556E: jsr $5022a.l
pub fn i_05556e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55574;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55574);
    cpu.pc = 0x5022A;
    16
}
/// $055574: rts
pub fn i_055574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55576;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055576  ($055576, 14 B, kernel/state_dispatch_stubs.c)
/// $055576: lea.l $29a9da.l, a2
pub fn i_055576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5557C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A9DA;
    20
}
/// $05557C: jsr $5022a.l
pub fn i_05557c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55582;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55582);
    cpu.pc = 0x5022A;
    16
}
/// $055582: rts
pub fn i_055582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55584;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055584  ($055584, 14 B, kernel/state_dispatch_stubs.c)
/// $055584: lea.l $29a9ee.l, a2
pub fn i_055584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5558A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A9EE;
    20
}
/// $05558A: jsr $5022a.l
pub fn i_05558a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55590;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55590);
    cpu.pc = 0x5022A;
    16
}
/// $055590: rts
pub fn i_055590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55592;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055592  ($055592, 14 B, kernel/state_dispatch_stubs.c)
/// $055592: lea.l $29aa02.l, a2
pub fn i_055592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55598;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AA02;
    20
}
/// $055598: jsr $5022a.l
pub fn i_055598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5559E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5559E);
    cpu.pc = 0x5022A;
    16
}
/// $05559E: rts
pub fn i_05559e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x555A0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0555a0  ($0555A0, 14 B, kernel/state_dispatch_stubs.c)
/// $0555A0: lea.l $29aa16.l, a2
pub fn i_0555a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x555A6;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AA16;
    20
}
/// $0555A6: jsr $5022a.l
pub fn i_0555a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x555AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x555AC);
    cpu.pc = 0x5022A;
    16
}
/// $0555AC: rts
pub fn i_0555ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x555AE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0555ae  ($0555AE, 14 B, kernel/state_dispatch_stubs.c)
/// $0555AE: lea.l $29aa2a.l, a2
pub fn i_0555ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x555B4;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AA2A;
    20
}
/// $0555B4: jsr $5022a.l
pub fn i_0555b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x555BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x555BA);
    cpu.pc = 0x5022A;
    16
}
/// $0555BA: rts
pub fn i_0555ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x555BC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0555bc  ($0555BC, 14 B, kernel/state_dispatch_stubs.c)
/// $0555BC: lea.l $29aa3e.l, a2
pub fn i_0555bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x555C2;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AA3E;
    20
}
/// $0555C2: jsr $5022a.l
pub fn i_0555c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x555C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x555C8);
    cpu.pc = 0x5022A;
    16
}
/// $0555C8: rts
pub fn i_0555c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x555CA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0555ca  ($0555CA, 14 B, kernel/state_dispatch_stubs.c)
/// $0555CA: lea.l $29aa52.l, a2
pub fn i_0555ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x555D0;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AA52;
    20
}
/// $0555D0: jsr $5022a.l
pub fn i_0555d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x555D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x555D6);
    cpu.pc = 0x5022A;
    16
}
/// $0555D6: rts
pub fn i_0555d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x555D8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0555d8  ($0555D8, 14 B, kernel/state_dispatch_stubs.c)
/// $0555D8: lea.l $29aa66.l, a2
pub fn i_0555d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x555DE;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AA66;
    20
}
/// $0555DE: jsr $5022a.l
pub fn i_0555de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x555E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x555E4);
    cpu.pc = 0x5022A;
    16
}
/// $0555E4: rts
pub fn i_0555e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x555E6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0555e6  ($0555E6, 14 B, kernel/state_dispatch_stubs.c)
/// $0555E6: lea.l $29aa7a.l, a2
pub fn i_0555e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x555EC;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AA7A;
    20
}
/// $0555EC: jsr $5022a.l
pub fn i_0555ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x555F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x555F2);
    cpu.pc = 0x5022A;
    16
}
/// $0555F2: rts
pub fn i_0555f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x555F4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0555f4  ($0555F4, 14 B, kernel/state_dispatch_stubs.c)
/// $0555F4: lea.l $29aa8e.l, a2
pub fn i_0555f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x555FA;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AA8E;
    20
}
/// $0555FA: jsr $5022a.l
pub fn i_0555fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55600;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55600);
    cpu.pc = 0x5022A;
    16
}
/// $055600: rts
pub fn i_055600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55602;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055602  ($055602, 14 B, kernel/state_dispatch_stubs.c)
/// $055602: lea.l $29aaa2.l, a2
pub fn i_055602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55608;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AAA2;
    20
}
/// $055608: jsr $5022a.l
pub fn i_055608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5560E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5560E);
    cpu.pc = 0x5022A;
    16
}
/// $05560E: rts
pub fn i_05560e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55610;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055610  ($055610, 14 B, kernel/state_dispatch_stubs.c)
/// $055610: lea.l $29aab6.l, a2
pub fn i_055610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55616;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AAB6;
    20
}
/// $055616: jsr $5022a.l
pub fn i_055616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5561C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5561C);
    cpu.pc = 0x5022A;
    16
}
/// $05561C: rts
pub fn i_05561c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5561E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05561e  ($05561E, 14 B, kernel/state_dispatch_stubs.c)
/// $05561E: lea.l $29aaca.l, a2
pub fn i_05561e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55624;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AACA;
    20
}
/// $055624: jsr $5022a.l
pub fn i_055624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5562A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5562A);
    cpu.pc = 0x5022A;
    16
}
/// $05562A: rts
pub fn i_05562a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5562C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05562c  ($05562C, 14 B, kernel/state_dispatch_stubs.c)
/// $05562C: lea.l $29aade.l, a2
pub fn i_05562c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55632;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AADE;
    20
}
/// $055632: jsr $5022a.l
pub fn i_055632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55638;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55638);
    cpu.pc = 0x5022A;
    16
}
/// $055638: rts
pub fn i_055638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5563A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05563a  ($05563A, 14 B, kernel/state_dispatch_stubs.c)
/// $05563A: lea.l $29aaf2.l, a2
pub fn i_05563a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55640;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AAF2;
    20
}
/// $055640: jsr $5022a.l
pub fn i_055640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55646;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55646);
    cpu.pc = 0x5022A;
    16
}
/// $055646: rts
pub fn i_055646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55648;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055648  ($055648, 14 B, kernel/state_dispatch_stubs.c)
/// $055648: lea.l $29ab06.l, a2
pub fn i_055648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5564E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AB06;
    20
}
/// $05564E: jsr $5022a.l
pub fn i_05564e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55654;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55654);
    cpu.pc = 0x5022A;
    16
}
/// $055654: rts
pub fn i_055654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55656;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055656  ($055656, 14 B, kernel/state_dispatch_stubs.c)
/// $055656: lea.l $29ab1a.l, a2
pub fn i_055656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5565C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AB1A;
    20
}
/// $05565C: jsr $5022a.l
pub fn i_05565c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55662;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55662);
    cpu.pc = 0x5022A;
    16
}
/// $055662: rts
pub fn i_055662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55664;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055664  ($055664, 14 B, kernel/state_dispatch_stubs.c)
/// $055664: lea.l $29ab2e.l, a2
pub fn i_055664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5566A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AB2E;
    20
}
/// $05566A: jsr $5022a.l
pub fn i_05566a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55670;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55670);
    cpu.pc = 0x5022A;
    16
}
/// $055670: rts
pub fn i_055670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55672;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055672  ($055672, 14 B, kernel/state_dispatch_stubs.c)
/// $055672: lea.l $29ab42.l, a2
pub fn i_055672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55678;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AB42;
    20
}
/// $055678: jsr $5022a.l
pub fn i_055678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5567E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5567E);
    cpu.pc = 0x5022A;
    16
}
/// $05567E: rts
pub fn i_05567e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55680;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055680  ($055680, 14 B, kernel/state_dispatch_stubs.c)
/// $055680: lea.l $29ab56.l, a2
pub fn i_055680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55686;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AB56;
    20
}
/// $055686: jsr $5022a.l
pub fn i_055686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5568C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5568C);
    cpu.pc = 0x5022A;
    16
}
/// $05568C: rts
pub fn i_05568c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5568E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05568e  ($05568E, 14 B, kernel/state_dispatch_stubs.c)
/// $05568E: lea.l $29ab6a.l, a2
pub fn i_05568e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55694;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AB6A;
    20
}
/// $055694: jsr $5022a.l
pub fn i_055694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5569A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5569A);
    cpu.pc = 0x5022A;
    16
}
/// $05569A: rts
pub fn i_05569a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5569C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05569c  ($05569C, 14 B, kernel/state_dispatch_stubs.c)
/// $05569C: lea.l $29ab7e.l, a2
pub fn i_05569c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x556A2;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AB7E;
    20
}
/// $0556A2: jsr $5022a.l
pub fn i_0556a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x556A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x556A8);
    cpu.pc = 0x5022A;
    16
}
/// $0556A8: rts
pub fn i_0556a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x556AA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0556aa  ($0556AA, 14 B, kernel/state_dispatch_stubs.c)
/// $0556AA: lea.l $29ab92.l, a2
pub fn i_0556aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x556B0;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AB92;
    20
}
/// $0556B0: jsr $5022a.l
pub fn i_0556b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x556B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x556B6);
    cpu.pc = 0x5022A;
    16
}
/// $0556B6: rts
pub fn i_0556b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x556B8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0556b8  ($0556B8, 14 B, kernel/state_dispatch_stubs.c)
/// $0556B8: lea.l $29aba6.l, a2
pub fn i_0556b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x556BE;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29ABA6;
    20
}
/// $0556BE: jsr $5022a.l
pub fn i_0556be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x556C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x556C4);
    cpu.pc = 0x5022A;
    16
}
/// $0556C4: rts
pub fn i_0556c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x556C6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0556c6  ($0556C6, 14 B, kernel/state_dispatch_stubs.c)
/// $0556C6: lea.l $29abba.l, a2
pub fn i_0556c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x556CC;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29ABBA;
    20
}
/// $0556CC: jsr $5022a.l
pub fn i_0556cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x556D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x556D2);
    cpu.pc = 0x5022A;
    16
}
/// $0556D2: rts
pub fn i_0556d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x556D4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0556d4  ($0556D4, 14 B, kernel/state_dispatch_stubs.c)
/// $0556D4: lea.l $29abce.l, a2
pub fn i_0556d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x556DA;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29ABCE;
    20
}
/// $0556DA: jsr $5022a.l
pub fn i_0556da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x556E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x556E0);
    cpu.pc = 0x5022A;
    16
}
/// $0556E0: rts
pub fn i_0556e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x556E2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0556e2  ($0556E2, 14 B, kernel/state_dispatch_stubs.c)
/// $0556E2: lea.l $29abe2.l, a2
pub fn i_0556e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x556E8;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29ABE2;
    20
}
/// $0556E8: jsr $5022a.l
pub fn i_0556e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x556EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x556EE);
    cpu.pc = 0x5022A;
    16
}
/// $0556EE: rts
pub fn i_0556ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x556F0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0556f0  ($0556F0, 14 B, kernel/state_dispatch_stubs.c)
/// $0556F0: lea.l $29abf6.l, a2
pub fn i_0556f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x556F6;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29ABF6;
    20
}
/// $0556F6: jsr $5022a.l
pub fn i_0556f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x556FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x556FC);
    cpu.pc = 0x5022A;
    16
}
/// $0556FC: rts
pub fn i_0556fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x556FE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0556fe  ($0556FE, 14 B, kernel/state_dispatch_stubs.c)
/// $0556FE: lea.l $29ac0a.l, a2
pub fn i_0556fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55704;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AC0A;
    20
}
/// $055704: jsr $5022a.l
pub fn i_055704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5570A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5570A);
    cpu.pc = 0x5022A;
    16
}
/// $05570A: rts
pub fn i_05570a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5570C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05570c  ($05570C, 14 B, kernel/state_dispatch_stubs.c)
/// $05570C: lea.l $29ac1e.l, a2
pub fn i_05570c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55712;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AC1E;
    20
}
/// $055712: jsr $5022a.l
pub fn i_055712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55718;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55718);
    cpu.pc = 0x5022A;
    16
}
/// $055718: rts
pub fn i_055718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5571A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05571a  ($05571A, 14 B, kernel/state_dispatch_stubs.c)
/// $05571A: lea.l $29ac32.l, a2
pub fn i_05571a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55720;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AC32;
    20
}
/// $055720: jsr $5022a.l
pub fn i_055720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55726;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55726);
    cpu.pc = 0x5022A;
    16
}
/// $055726: rts
pub fn i_055726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55728;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055728  ($055728, 14 B, kernel/state_dispatch_stubs.c)
/// $055728: lea.l $29ac46.l, a2
pub fn i_055728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5572E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AC46;
    20
}
/// $05572E: jsr $5022a.l
pub fn i_05572e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55734;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55734);
    cpu.pc = 0x5022A;
    16
}
/// $055734: rts
pub fn i_055734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55736;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055736  ($055736, 14 B, kernel/state_dispatch_stubs.c)
/// $055736: lea.l $29ac5a.l, a2
pub fn i_055736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5573C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AC5A;
    20
}
/// $05573C: jsr $5022a.l
pub fn i_05573c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55742;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55742);
    cpu.pc = 0x5022A;
    16
}
/// $055742: rts
pub fn i_055742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55744;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055744  ($055744, 14 B, kernel/state_dispatch_stubs.c)
/// $055744: lea.l $29ac6e.l, a2
pub fn i_055744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5574A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AC6E;
    20
}
/// $05574A: jsr $5022a.l
pub fn i_05574a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55750;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55750);
    cpu.pc = 0x5022A;
    16
}
/// $055750: rts
pub fn i_055750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55752;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055752  ($055752, 14 B, kernel/state_dispatch_stubs.c)
/// $055752: lea.l $29ac82.l, a2
pub fn i_055752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55758;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AC82;
    20
}
/// $055758: jsr $5022a.l
pub fn i_055758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5575E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5575E);
    cpu.pc = 0x5022A;
    16
}
/// $05575E: rts
pub fn i_05575e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55760;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055760  ($055760, 14 B, kernel/state_dispatch_stubs.c)
/// $055760: lea.l $29ac96.l, a2
pub fn i_055760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55766;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AC96;
    20
}
/// $055766: jsr $5022a.l
pub fn i_055766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5576C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5576C);
    cpu.pc = 0x5022A;
    16
}
/// $05576C: rts
pub fn i_05576c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5576E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05576e  ($05576E, 14 B, kernel/state_dispatch_stubs.c)
/// $05576E: lea.l $29acaa.l, a2
pub fn i_05576e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55774;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29ACAA;
    20
}
/// $055774: jsr $5022a.l
pub fn i_055774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5577A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5577A);
    cpu.pc = 0x5022A;
    16
}
/// $05577A: rts
pub fn i_05577a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5577C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05577c  ($05577C, 14 B, kernel/state_dispatch_stubs.c)
/// $05577C: lea.l $29acbe.l, a2
pub fn i_05577c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55782;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29ACBE;
    20
}
/// $055782: jsr $5022a.l
pub fn i_055782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55788;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55788);
    cpu.pc = 0x5022A;
    16
}
/// $055788: rts
pub fn i_055788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5578A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05578a  ($05578A, 14 B, kernel/state_dispatch_stubs.c)
/// $05578A: lea.l $29acd2.l, a2
pub fn i_05578a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55790;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29ACD2;
    20
}
/// $055790: jsr $5022a.l
pub fn i_055790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55796;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55796);
    cpu.pc = 0x5022A;
    16
}
/// $055796: rts
pub fn i_055796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55798;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055798  ($055798, 14 B, kernel/state_dispatch_stubs.c)
/// $055798: lea.l $29ace6.l, a2
pub fn i_055798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5579E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29ACE6;
    20
}
/// $05579E: jsr $5022a.l
pub fn i_05579e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x557A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x557A4);
    cpu.pc = 0x5022A;
    16
}
/// $0557A4: rts
pub fn i_0557a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x557A6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0557a6  ($0557A6, 14 B, kernel/state_dispatch_stubs.c)
/// $0557A6: lea.l $29acfa.l, a2
pub fn i_0557a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x557AC;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29ACFA;
    20
}
/// $0557AC: jsr $5022a.l
pub fn i_0557ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x557B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x557B2);
    cpu.pc = 0x5022A;
    16
}
/// $0557B2: rts
pub fn i_0557b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x557B4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0557b4  ($0557B4, 14 B, kernel/state_dispatch_stubs.c)
/// $0557B4: lea.l $29ad0e.l, a2
pub fn i_0557b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x557BA;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AD0E;
    20
}
/// $0557BA: jsr $5022a.l
pub fn i_0557ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x557C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x557C0);
    cpu.pc = 0x5022A;
    16
}
/// $0557C0: rts
pub fn i_0557c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x557C2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0557c2  ($0557C2, 14 B, kernel/state_dispatch_stubs.c)
/// $0557C2: lea.l $29ad22.l, a2
pub fn i_0557c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x557C8;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AD22;
    20
}
/// $0557C8: jsr $5022a.l
pub fn i_0557c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x557CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x557CE);
    cpu.pc = 0x5022A;
    16
}
/// $0557CE: rts
pub fn i_0557ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x557D0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0557d0  ($0557D0, 14 B, kernel/state_dispatch_stubs.c)
/// $0557D0: lea.l $29ad36.l, a2
pub fn i_0557d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x557D6;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AD36;
    20
}
/// $0557D6: jsr $5022a.l
pub fn i_0557d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x557DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x557DC);
    cpu.pc = 0x5022A;
    16
}
/// $0557DC: rts
pub fn i_0557dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x557DE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0557de  ($0557DE, 14 B, kernel/state_dispatch_stubs.c)
/// $0557DE: lea.l $29ad4a.l, a2
pub fn i_0557de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x557E4;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AD4A;
    20
}
/// $0557E4: jsr $5022a.l
pub fn i_0557e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x557EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x557EA);
    cpu.pc = 0x5022A;
    16
}
/// $0557EA: rts
pub fn i_0557ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x557EC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0557ec  ($0557EC, 14 B, kernel/state_dispatch_stubs.c)
/// $0557EC: lea.l $29ad5e.l, a2
pub fn i_0557ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x557F2;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AD5E;
    20
}
/// $0557F2: jsr $5022a.l
pub fn i_0557f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x557F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x557F8);
    cpu.pc = 0x5022A;
    16
}
/// $0557F8: rts
pub fn i_0557f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x557FA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0557fa  ($0557FA, 14 B, kernel/state_dispatch_stubs.c)
/// $0557FA: lea.l $29ad72.l, a2
pub fn i_0557fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55800;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AD72;
    20
}
/// $055800: jsr $5022a.l
pub fn i_055800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55806;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55806);
    cpu.pc = 0x5022A;
    16
}
/// $055806: rts
pub fn i_055806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55808;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055808  ($055808, 14 B, kernel/state_dispatch_stubs.c)
/// $055808: lea.l $29ad86.l, a2
pub fn i_055808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5580E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AD86;
    20
}
/// $05580E: jsr $5022a.l
pub fn i_05580e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55814;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55814);
    cpu.pc = 0x5022A;
    16
}
/// $055814: rts
pub fn i_055814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55816;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055816  ($055816, 14 B, kernel/state_dispatch_stubs.c)
/// $055816: lea.l $29ad9a.l, a2
pub fn i_055816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5581C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AD9A;
    20
}
/// $05581C: jsr $5022a.l
pub fn i_05581c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55822;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55822);
    cpu.pc = 0x5022A;
    16
}
/// $055822: rts
pub fn i_055822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55824;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055824  ($055824, 14 B, kernel/state_dispatch_stubs.c)
/// $055824: lea.l $29adae.l, a2
pub fn i_055824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5582A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29ADAE;
    20
}
/// $05582A: jsr $5022a.l
pub fn i_05582a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55830;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55830);
    cpu.pc = 0x5022A;
    16
}
/// $055830: rts
pub fn i_055830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55832;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055832  ($055832, 14 B, kernel/state_dispatch_stubs.c)
/// $055832: lea.l $29adc2.l, a2
pub fn i_055832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55838;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29ADC2;
    20
}
/// $055838: jsr $5022a.l
pub fn i_055838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5583E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5583E);
    cpu.pc = 0x5022A;
    16
}
/// $05583E: rts
pub fn i_05583e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55840;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055840  ($055840, 14 B, kernel/state_dispatch_stubs.c)
/// $055840: lea.l $29add6.l, a2
pub fn i_055840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55846;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29ADD6;
    20
}
/// $055846: jsr $5022a.l
pub fn i_055846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5584C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5584C);
    cpu.pc = 0x5022A;
    16
}
/// $05584C: rts
pub fn i_05584c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5584E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05584e  ($05584E, 14 B, kernel/state_dispatch_stubs.c)
/// $05584E: lea.l $29adea.l, a2
pub fn i_05584e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55854;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29ADEA;
    20
}
/// $055854: jsr $5022a.l
pub fn i_055854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5585A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5585A);
    cpu.pc = 0x5022A;
    16
}
/// $05585A: rts
pub fn i_05585a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5585C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05585c  ($05585C, 14 B, kernel/state_dispatch_stubs.c)
/// $05585C: lea.l $29adfe.l, a2
pub fn i_05585c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55862;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29ADFE;
    20
}
/// $055862: jsr $5022a.l
pub fn i_055862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55868;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55868);
    cpu.pc = 0x5022A;
    16
}
/// $055868: rts
pub fn i_055868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5586A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05586a  ($05586A, 14 B, kernel/state_dispatch_stubs.c)
/// $05586A: lea.l $29ae12.l, a2
pub fn i_05586a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55870;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AE12;
    20
}
/// $055870: jsr $5022a.l
pub fn i_055870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55876;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55876);
    cpu.pc = 0x5022A;
    16
}
/// $055876: rts
pub fn i_055876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55878;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055878  ($055878, 14 B, kernel/state_dispatch_stubs.c)
/// $055878: lea.l $29ae26.l, a2
pub fn i_055878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5587E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AE26;
    20
}
/// $05587E: jsr $5022a.l
pub fn i_05587e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55884;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55884);
    cpu.pc = 0x5022A;
    16
}
/// $055884: rts
pub fn i_055884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55886;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055886  ($055886, 14 B, kernel/state_dispatch_stubs.c)
/// $055886: lea.l $29ae3a.l, a2
pub fn i_055886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5588C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AE3A;
    20
}
/// $05588C: jsr $5022a.l
pub fn i_05588c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55892;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55892);
    cpu.pc = 0x5022A;
    16
}
/// $055892: rts
pub fn i_055892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55894;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055894  ($055894, 14 B, kernel/state_dispatch_stubs.c)
/// $055894: lea.l $29ae4e.l, a2
pub fn i_055894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5589A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AE4E;
    20
}
/// $05589A: jsr $5022a.l
pub fn i_05589a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x558A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x558A0);
    cpu.pc = 0x5022A;
    16
}
/// $0558A0: rts
pub fn i_0558a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x558A2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0558a2  ($0558A2, 14 B, kernel/state_dispatch_stubs.c)
/// $0558A2: lea.l $29ae62.l, a2
pub fn i_0558a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x558A8;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AE62;
    20
}
/// $0558A8: jsr $5022a.l
pub fn i_0558a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x558AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x558AE);
    cpu.pc = 0x5022A;
    16
}
/// $0558AE: rts
pub fn i_0558ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x558B0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0558b0  ($0558B0, 14 B, kernel/state_dispatch_stubs.c)
/// $0558B0: lea.l $29ae76.l, a2
pub fn i_0558b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x558B6;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AE76;
    20
}
/// $0558B6: jsr $5022a.l
pub fn i_0558b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x558BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x558BC);
    cpu.pc = 0x5022A;
    16
}
/// $0558BC: rts
pub fn i_0558bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x558BE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0558be  ($0558BE, 14 B, kernel/state_dispatch_stubs.c)
/// $0558BE: lea.l $29ae8a.l, a2
pub fn i_0558be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x558C4;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AE8A;
    20
}
/// $0558C4: jsr $5022a.l
pub fn i_0558c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x558CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x558CA);
    cpu.pc = 0x5022A;
    16
}
/// $0558CA: rts
pub fn i_0558ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x558CC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0558cc  ($0558CC, 14 B, kernel/state_dispatch_stubs.c)
/// $0558CC: lea.l $29ae9e.l, a2
pub fn i_0558cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x558D2;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AE9E;
    20
}
/// $0558D2: jsr $5022a.l
pub fn i_0558d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x558D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x558D8);
    cpu.pc = 0x5022A;
    16
}
/// $0558D8: rts
pub fn i_0558d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x558DA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0558da  ($0558DA, 14 B, kernel/state_dispatch_stubs.c)
/// $0558DA: lea.l $29aeb2.l, a2
pub fn i_0558da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x558E0;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AEB2;
    20
}
/// $0558E0: jsr $5022a.l
pub fn i_0558e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x558E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x558E6);
    cpu.pc = 0x5022A;
    16
}
/// $0558E6: rts
pub fn i_0558e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x558E8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0558e8  ($0558E8, 14 B, kernel/state_dispatch_stubs.c)
/// $0558E8: lea.l $29aec6.l, a2
pub fn i_0558e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x558EE;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29AEC6;
    20
}
/// $0558EE: jsr $5022a.l
pub fn i_0558ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x558F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x558F4);
    cpu.pc = 0x5022A;
    16
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x053B9E, i_053b9e),
    (0x053BA2, i_053ba2),
    (0x053BA8, i_053ba8),
    (0x053BAE, i_053bae),
    (0x053BB4, i_053bb4),
    (0x053BBA, i_053bba),
    (0x053BC0, i_053bc0),
    (0x053BC6, i_053bc6),
    (0x053BCC, i_053bcc),
    (0x053BD0, i_053bd0),
    (0x053BD2, i_053bd2),
    (0x053BD6, i_053bd6),
    (0x053BDC, i_053bdc),
    (0x053BE2, i_053be2),
    (0x053BE8, i_053be8),
    (0x053BEC, i_053bec),
    (0x053BF2, i_053bf2),
    (0x053BF6, i_053bf6),
    (0x053BFC, i_053bfc),
    (0x053BFE, i_053bfe),
    (0x053C02, i_053c02),
    (0x053C08, i_053c08),
    (0x053C0E, i_053c0e),
    (0x053C14, i_053c14),
    (0x053C1A, i_053c1a),
    (0x053C20, i_053c20),
    (0x053C26, i_053c26),
    (0x053C2A, i_053c2a),
    (0x053C2C, i_053c2c),
    (0x053C32, i_053c32),
    (0x053C38, i_053c38),
    (0x053C3C, i_053c3c),
    (0x053C42, i_053c42),
    (0x053C44, i_053c44),
    (0x053C48, i_053c48),
    (0x053C4E, i_053c4e),
    (0x053C54, i_053c54),
    (0x053C58, i_053c58),
    (0x053C5A, i_053c5a),
    (0x053C5C, i_053c5c),
    (0x053C60, i_053c60),
    (0x053C62, i_053c62),
    (0x053C64, i_053c64),
    (0x053C68, i_053c68),
    (0x053C6A, i_053c6a),
    (0x053C70, i_053c70),
    (0x053C74, i_053c74),
    (0x053C7C, i_053c7c),
    (0x053C80, i_053c80),
    (0x053C84, i_053c84),
    (0x053C88, i_053c88),
    (0x053C8C, i_053c8c),
    (0x053C8E, i_053c8e),
    (0x053C92, i_053c92),
    (0x053C98, i_053c98),
    (0x053C9E, i_053c9e),
    (0x053CA2, i_053ca2),
    (0x053CA4, i_053ca4),
    (0x053CA8, i_053ca8),
    (0x053CAE, i_053cae),
    (0x053CB4, i_053cb4),
    (0x053CBA, i_053cba),
    (0x053CBE, i_053cbe),
    (0x053CC2, i_053cc2),
    (0x053CC6, i_053cc6),
    (0x053CCA, i_053cca),
    (0x053CD0, i_053cd0),
    (0x053CD6, i_053cd6),
    (0x053CDC, i_053cdc),
    (0x053CE2, i_053ce2),
    (0x053CE6, i_053ce6),
    (0x053CEA, i_053cea),
    (0x053CF0, i_053cf0),
    (0x053CF2, i_053cf2),
    (0x053CF6, i_053cf6),
    (0x053CFC, i_053cfc),
    (0x053D02, i_053d02),
    (0x053D08, i_053d08),
    (0x053D0E, i_053d0e),
    (0x053D12, i_053d12),
    (0x053D14, i_053d14),
    (0x053D1A, i_053d1a),
    (0x053D20, i_053d20),
    (0x053D24, i_053d24),
    (0x053D2A, i_053d2a),
    (0x053D2C, i_053d2c),
    (0x053D32, i_053d32),
    (0x053D36, i_053d36),
    (0x053D3C, i_053d3c),
    (0x053D42, i_053d42),
    (0x053D48, i_053d48),
    (0x053D4E, i_053d4e),
    (0x053D80, i_053d80),
    (0x053D84, i_053d84),
    (0x053D86, i_053d86),
    (0x053D8A, i_053d8a),
    (0x053D8E, i_053d8e),
    (0x053D92, i_053d92),
    (0x053D98, i_053d98),
    (0x053D9C, i_053d9c),
    (0x053DA0, i_053da0),
    (0x053DA4, i_053da4),
    (0x053DAA, i_053daa),
    (0x053DB0, i_053db0),
    (0x053DB4, i_053db4),
    (0x053DB8, i_053db8),
    (0x053DBC, i_053dbc),
    (0x053DC2, i_053dc2),
    (0x053DC8, i_053dc8),
    (0x053DCA, i_053dca),
    (0x053DCE, i_053dce),
    (0x053DD2, i_053dd2),
    (0x053DD6, i_053dd6),
    (0x053DDA, i_053dda),
    (0x053DE0, i_053de0),
    (0x053DE4, i_053de4),
    (0x053DEA, i_053dea),
    (0x053DF0, i_053df0),
    (0x053DF4, i_053df4),
    (0x053DFA, i_053dfa),
    (0x053E00, i_053e00),
    (0x053E04, i_053e04),
    (0x053E0A, i_053e0a),
    (0x053E0C, i_053e0c),
    (0x053E12, i_053e12),
    (0x053E16, i_053e16),
    (0x053E1A, i_053e1a),
    (0x053E20, i_053e20),
    (0x053E24, i_053e24),
    (0x053E2A, i_053e2a),
    (0x053E2E, i_053e2e),
    (0x053E32, i_053e32),
    (0x053E38, i_053e38),
    (0x053E3C, i_053e3c),
    (0x053E42, i_053e42),
    (0x053E46, i_053e46),
    (0x053E4A, i_053e4a),
    (0x053E50, i_053e50),
    (0x053E54, i_053e54),
    (0x053E5A, i_053e5a),
    (0x053E5E, i_053e5e),
    (0x053E62, i_053e62),
    (0x053E68, i_053e68),
    (0x053E6C, i_053e6c),
    (0x053E70, i_053e70),
    (0x053E76, i_053e76),
    (0x053E78, i_053e78),
    (0x053E7E, i_053e7e),
    (0x053E82, i_053e82),
    (0x053E86, i_053e86),
    (0x053E88, i_053e88),
    (0x053E8C, i_053e8c),
    (0x053E8E, i_053e8e),
    (0x053E90, i_053e90),
    (0x053E94, i_053e94),
    (0x053E9A, i_053e9a),
    (0x053E9C, i_053e9c),
    (0x053EA0, i_053ea0),
    (0x053EA6, i_053ea6),
    (0x053EA8, i_053ea8),
    (0x053EAE, i_053eae),
    (0x053EB2, i_053eb2),
    (0x053EB8, i_053eb8),
    (0x053EBA, i_053eba),
    (0x053EC0, i_053ec0),
    (0x053EC6, i_053ec6),
    (0x053ECA, i_053eca),
    (0x053ED0, i_053ed0),
    (0x053ED4, i_053ed4),
    (0x053EDA, i_053eda),
    (0x053EE0, i_053ee0),
    (0x053EE2, i_053ee2),
    (0x053EE8, i_053ee8),
    (0x053EEE, i_053eee),
    (0x053EF4, i_053ef4),
    (0x053EFA, i_053efa),
    (0x053F00, i_053f00),
    (0x053F06, i_053f06),
    (0x053F08, i_053f08),
    (0x053F0E, i_053f0e),
    (0x053F14, i_053f14),
    (0x053F1A, i_053f1a),
    (0x053F20, i_053f20),
    (0x053F26, i_053f26),
    (0x053F2C, i_053f2c),
    (0x053F2E, i_053f2e),
    (0x053F34, i_053f34),
    (0x053F3A, i_053f3a),
    (0x053F40, i_053f40),
    (0x053F46, i_053f46),
    (0x053F4C, i_053f4c),
    (0x053F52, i_053f52),
    (0x053F54, i_053f54),
    (0x053F5A, i_053f5a),
    (0x053F60, i_053f60),
    (0x053F66, i_053f66),
    (0x053F6C, i_053f6c),
    (0x053F72, i_053f72),
    (0x053F78, i_053f78),
    (0x053F7A, i_053f7a),
    (0x053F7E, i_053f7e),
    (0x053F82, i_053f82),
    (0x053F86, i_053f86),
    (0x053F8A, i_053f8a),
    (0x053F8E, i_053f8e),
    (0x053F90, i_053f90),
    (0x053F94, i_053f94),
    (0x053F96, i_053f96),
    (0x053F9A, i_053f9a),
    (0x053FA0, i_053fa0),
    (0x053FA4, i_053fa4),
    (0x053FAA, i_053faa),
    (0x053FB0, i_053fb0),
    (0x053FB6, i_053fb6),
    (0x053FBC, i_053fbc),
    (0x053FC2, i_053fc2),
    (0x053FC8, i_053fc8),
    (0x053FCE, i_053fce),
    (0x053FD4, i_053fd4),
    (0x053FD8, i_053fd8),
    (0x053FDA, i_053fda),
    (0x053FE0, i_053fe0),
    (0x053FE6, i_053fe6),
    (0x053FEC, i_053fec),
    (0x053FF2, i_053ff2),
    (0x053FF6, i_053ff6),
    (0x053FFC, i_053ffc),
    (0x054000, i_054000),
    (0x054006, i_054006),
    (0x05400C, i_05400c),
    (0x054012, i_054012),
    (0x054018, i_054018),
    (0x05401C, i_05401c),
    (0x054020, i_054020),
    (0x054026, i_054026),
    (0x05402C, i_05402c),
    (0x054032, i_054032),
    (0x054038, i_054038),
    (0x05403E, i_05403e),
    (0x054044, i_054044),
    (0x054048, i_054048),
    (0x05404A, i_05404a),
    (0x054050, i_054050),
    (0x054054, i_054054),
    (0x05405A, i_05405a),
    (0x05405C, i_05405c),
    (0x054062, i_054062),
    (0x054068, i_054068),
    (0x05406E, i_05406e),
    (0x054074, i_054074),
    (0x05407A, i_05407a),
    (0x05407E, i_05407e),
    (0x054080, i_054080),
    (0x054086, i_054086),
    (0x05408C, i_05408c),
    (0x054092, i_054092),
    (0x054098, i_054098),
    (0x05409C, i_05409c),
    (0x0540A2, i_0540a2),
    (0x0540A8, i_0540a8),
    (0x0540AE, i_0540ae),
    (0x0540B4, i_0540b4),
    (0x0540B8, i_0540b8),
    (0x0540BE, i_0540be),
    (0x0540C2, i_0540c2),
    (0x0540C8, i_0540c8),
    (0x0540CE, i_0540ce),
    (0x0540D4, i_0540d4),
    (0x0540DA, i_0540da),
    (0x0540E0, i_0540e0),
    (0x0540E6, i_0540e6),
    (0x0540EC, i_0540ec),
    (0x0540F2, i_0540f2),
    (0x0540F6, i_0540f6),
    (0x0540F8, i_0540f8),
    (0x0540FE, i_0540fe),
    (0x054102, i_054102),
    (0x054108, i_054108),
    (0x05410A, i_05410a),
    (0x054110, i_054110),
    (0x054116, i_054116),
    (0x05411C, i_05411c),
    (0x054122, i_054122),
    (0x054128, i_054128),
    (0x05412C, i_05412c),
    (0x054130, i_054130),
    (0x054136, i_054136),
    (0x05413C, i_05413c),
    (0x054140, i_054140),
    (0x054142, i_054142),
    (0x054148, i_054148),
    (0x05414E, i_05414e),
    (0x054154, i_054154),
    (0x054158, i_054158),
    (0x05415E, i_05415e),
    (0x054160, i_054160),
    (0x054164, i_054164),
    (0x05416A, i_05416a),
    (0x05416E, i_05416e),
    (0x054174, i_054174),
    (0x05417A, i_05417a),
    (0x054180, i_054180),
    (0x054186, i_054186),
    (0x05418C, i_05418c),
    (0x054192, i_054192),
    (0x054198, i_054198),
    (0x05419E, i_05419e),
    (0x0541A4, i_0541a4),
    (0x0541AA, i_0541aa),
    (0x0541B0, i_0541b0),
    (0x0541B4, i_0541b4),
    (0x0541B6, i_0541b6),
    (0x0541BC, i_0541bc),
    (0x0541C2, i_0541c2),
    (0x0541C8, i_0541c8),
    (0x0541CE, i_0541ce),
    (0x0541D2, i_0541d2),
    (0x0541D8, i_0541d8),
    (0x0541DE, i_0541de),
    (0x0541E4, i_0541e4),
    (0x0541EA, i_0541ea),
    (0x0541EE, i_0541ee),
    (0x0541F4, i_0541f4),
    (0x0541FA, i_0541fa),
    (0x054200, i_054200),
    (0x054204, i_054204),
    (0x05420A, i_05420a),
    (0x054210, i_054210),
    (0x054216, i_054216),
    (0x05421C, i_05421c),
    (0x054222, i_054222),
    (0x054228, i_054228),
    (0x05422E, i_05422e),
    (0x054234, i_054234),
    (0x05423A, i_05423a),
    (0x05423E, i_05423e),
    (0x054242, i_054242),
    (0x054248, i_054248),
    (0x05424C, i_05424c),
    (0x054252, i_054252),
    (0x054254, i_054254),
    (0x05425A, i_05425a),
    (0x054260, i_054260),
    (0x054266, i_054266),
    (0x05426C, i_05426c),
    (0x054272, i_054272),
    (0x054278, i_054278),
    (0x05427E, i_05427e),
    (0x0542B6, i_0542b6),
    (0x0542BC, i_0542bc),
    (0x0542C0, i_0542c0),
    (0x0542C2, i_0542c2),
    (0x0542C8, i_0542c8),
    (0x0542CE, i_0542ce),
    (0x0542D4, i_0542d4),
    (0x0542DA, i_0542da),
    (0x0542DE, i_0542de),
    (0x0542E4, i_0542e4),
    (0x0542EA, i_0542ea),
    (0x0542F0, i_0542f0),
    (0x0542F6, i_0542f6),
    (0x0542FA, i_0542fa),
    (0x0542FE, i_0542fe),
    (0x054304, i_054304),
    (0x05430A, i_05430a),
    (0x054310, i_054310),
    (0x054316, i_054316),
    (0x05431A, i_05431a),
    (0x054320, i_054320),
    (0x054324, i_054324),
    (0x054326, i_054326),
    (0x05432C, i_05432c),
    (0x054330, i_054330),
    (0x054336, i_054336),
    (0x054338, i_054338),
    (0x05433E, i_05433e),
    (0x054342, i_054342),
    (0x054348, i_054348),
    (0x05434E, i_05434e),
    (0x054352, i_054352),
    (0x054358, i_054358),
    (0x05435E, i_05435e),
    (0x054362, i_054362),
    (0x054364, i_054364),
    (0x05436A, i_05436a),
    (0x054370, i_054370),
    (0x054376, i_054376),
    (0x05437C, i_05437c),
    (0x054380, i_054380),
    (0x054386, i_054386),
    (0x05438C, i_05438c),
    (0x054392, i_054392),
    (0x054398, i_054398),
    (0x05439C, i_05439c),
    (0x0543A2, i_0543a2),
    (0x0543A6, i_0543a6),
    (0x0543AC, i_0543ac),
    (0x0543B2, i_0543b2),
    (0x0543B6, i_0543b6),
    (0x0543BC, i_0543bc),
    (0x0543C2, i_0543c2),
    (0x0543C8, i_0543c8),
    (0x0543CE, i_0543ce),
    (0x0543D4, i_0543d4),
    (0x0543DA, i_0543da),
    (0x0543E0, i_0543e0),
    (0x0543E4, i_0543e4),
    (0x0543E6, i_0543e6),
    (0x0543EC, i_0543ec),
    (0x0543F0, i_0543f0),
    (0x0543F6, i_0543f6),
    (0x0543F8, i_0543f8),
    (0x0543FE, i_0543fe),
    (0x054404, i_054404),
    (0x054408, i_054408),
    (0x05440E, i_05440e),
    (0x054414, i_054414),
    (0x054418, i_054418),
    (0x05441E, i_05441e),
    (0x054424, i_054424),
    (0x05442A, i_05442a),
    (0x054430, i_054430),
    (0x054436, i_054436),
    (0x05443A, i_05443a),
    (0x054440, i_054440),
    (0x054446, i_054446),
    (0x05444C, i_05444c),
    (0x054452, i_054452),
    (0x054458, i_054458),
    (0x05445E, i_05445e),
    (0x054464, i_054464),
    (0x054468, i_054468),
    (0x05446A, i_05446a),
    (0x054470, i_054470),
    (0x054476, i_054476),
    (0x05447C, i_05447c),
    (0x05447E, i_05447e),
    (0x054482, i_054482),
    (0x054488, i_054488),
    (0x05448E, i_05448e),
    (0x054492, i_054492),
    (0x054498, i_054498),
    (0x05449E, i_05449e),
    (0x0544A2, i_0544a2),
    (0x0544A8, i_0544a8),
    (0x0544AE, i_0544ae),
    (0x0544B2, i_0544b2),
    (0x0544B8, i_0544b8),
    (0x0544BE, i_0544be),
    (0x0544C4, i_0544c4),
    (0x0544CA, i_0544ca),
    (0x0544D0, i_0544d0),
    (0x0544D4, i_0544d4),
    (0x0544D6, i_0544d6),
    (0x0544DC, i_0544dc),
    (0x0544E2, i_0544e2),
    (0x0544E6, i_0544e6),
    (0x0544EA, i_0544ea),
    (0x0544EC, i_0544ec),
    (0x0544F2, i_0544f2),
    (0x0544F6, i_0544f6),
    (0x0544FA, i_0544fa),
    (0x0544FC, i_0544fc),
    (0x054502, i_054502),
    (0x054506, i_054506),
    (0x05450C, i_05450c),
    (0x054510, i_054510),
    (0x054516, i_054516),
    (0x054518, i_054518),
    (0x05451E, i_05451e),
    (0x054522, i_054522),
    (0x054524, i_054524),
    (0x05452A, i_05452a),
    (0x054530, i_054530),
    (0x054534, i_054534),
    (0x054538, i_054538),
    (0x05453A, i_05453a),
    (0x054540, i_054540),
    (0x054544, i_054544),
    (0x054548, i_054548),
    (0x05454A, i_05454a),
    (0x054550, i_054550),
    (0x054554, i_054554),
    (0x05455A, i_05455a),
    (0x05455E, i_05455e),
    (0x054564, i_054564),
    (0x054566, i_054566),
    (0x05456C, i_05456c),
    (0x054570, i_054570),
    (0x054572, i_054572),
    (0x054578, i_054578),
    (0x05457C, i_05457c),
    (0x054582, i_054582),
    (0x054584, i_054584),
    (0x054588, i_054588),
    (0x05458A, i_05458a),
    (0x054590, i_054590),
    (0x054596, i_054596),
    (0x05459A, i_05459a),
    (0x0545A0, i_0545a0),
    (0x0545A6, i_0545a6),
    (0x0545AC, i_0545ac),
    (0x0545B2, i_0545b2),
    (0x0545B8, i_0545b8),
    (0x0545BE, i_0545be),
    (0x0545C0, i_0545c0),
    (0x0545C6, i_0545c6),
    (0x0545CC, i_0545cc),
    (0x0545D0, i_0545d0),
    (0x0545D2, i_0545d2),
    (0x0545D6, i_0545d6),
    (0x0545DC, i_0545dc),
    (0x0545E2, i_0545e2),
    (0x0545E8, i_0545e8),
    (0x0545EE, i_0545ee),
    (0x0545F4, i_0545f4),
    (0x0545FA, i_0545fa),
    (0x0545FE, i_0545fe),
    (0x054602, i_054602),
    (0x054608, i_054608),
    (0x05460E, i_05460e),
    (0x054614, i_054614),
    (0x05461A, i_05461a),
    (0x054620, i_054620),
    (0x054626, i_054626),
    (0x05462C, i_05462c),
    (0x054630, i_054630),
    (0x054632, i_054632),
    (0x054638, i_054638),
    (0x05463E, i_05463e),
    (0x054642, i_054642),
    (0x054648, i_054648),
    (0x05464E, i_05464e),
    (0x054652, i_054652),
    (0x054658, i_054658),
    (0x05465E, i_05465e),
    (0x054664, i_054664),
    (0x05466A, i_05466a),
    (0x05466E, i_05466e),
    (0x054674, i_054674),
    (0x054678, i_054678),
    (0x05467E, i_05467e),
    (0x054684, i_054684),
    (0x05468A, i_05468a),
    (0x054690, i_054690),
    (0x054696, i_054696),
    (0x05469C, i_05469c),
    (0x0546A2, i_0546a2),
    (0x0546A6, i_0546a6),
    (0x0546A8, i_0546a8),
    (0x0546AE, i_0546ae),
    (0x0546B2, i_0546b2),
    (0x0546B8, i_0546b8),
    (0x0546BA, i_0546ba),
    (0x0546C0, i_0546c0),
    (0x0546C6, i_0546c6),
    (0x0546CA, i_0546ca),
    (0x0546CC, i_0546cc),
    (0x0546D2, i_0546d2),
    (0x0546D8, i_0546d8),
    (0x0546DC, i_0546dc),
    (0x0546E2, i_0546e2),
    (0x0546E8, i_0546e8),
    (0x0546EC, i_0546ec),
    (0x0546F2, i_0546f2),
    (0x0546F4, i_0546f4),
    (0x0546F8, i_0546f8),
    (0x0546FE, i_0546fe),
    (0x054702, i_054702),
    (0x054708, i_054708),
    (0x05470E, i_05470e),
    (0x054714, i_054714),
    (0x05471A, i_05471a),
    (0x054720, i_054720),
    (0x054726, i_054726),
    (0x05472A, i_05472a),
    (0x05472E, i_05472e),
    (0x054734, i_054734),
    (0x05473A, i_05473a),
    (0x054740, i_054740),
    (0x054744, i_054744),
    (0x054746, i_054746),
    (0x05474C, i_05474c),
    (0x054752, i_054752),
    (0x054758, i_054758),
    (0x05475E, i_05475e),
    (0x054762, i_054762),
    (0x054768, i_054768),
    (0x05476E, i_05476e),
    (0x054774, i_054774),
    (0x05477A, i_05477a),
    (0x05477E, i_05477e),
    (0x054784, i_054784),
    (0x054788, i_054788),
    (0x05478E, i_05478e),
    (0x054794, i_054794),
    (0x05479A, i_05479a),
    (0x0547A0, i_0547a0),
    (0x0547A6, i_0547a6),
    (0x0547AC, i_0547ac),
    (0x0547B2, i_0547b2),
    (0x0547B6, i_0547b6),
    (0x0547B8, i_0547b8),
    (0x0547BE, i_0547be),
    (0x0547C2, i_0547c2),
    (0x0547C8, i_0547c8),
    (0x0547CA, i_0547ca),
    (0x0547D0, i_0547d0),
    (0x0547D6, i_0547d6),
    (0x0547DC, i_0547dc),
    (0x0547E2, i_0547e2),
    (0x0547E4, i_0547e4),
    (0x0547E8, i_0547e8),
    (0x0547EC, i_0547ec),
    (0x0547F0, i_0547f0),
    (0x0547F6, i_0547f6),
    (0x0547FC, i_0547fc),
    (0x054800, i_054800),
    (0x054802, i_054802),
    (0x054808, i_054808),
    (0x05480E, i_05480e),
    (0x054814, i_054814),
    (0x05481A, i_05481a),
    (0x05481C, i_05481c),
    (0x054820, i_054820),
    (0x054826, i_054826),
    (0x05482A, i_05482a),
    (0x054830, i_054830),
    (0x054836, i_054836),
    (0x05483C, i_05483c),
    (0x054842, i_054842),
    (0x054848, i_054848),
    (0x05484E, i_05484e),
    (0x054854, i_054854),
    (0x05485A, i_05485a),
    (0x05485E, i_05485e),
    (0x054860, i_054860),
    (0x054866, i_054866),
    (0x05486C, i_05486c),
    (0x054872, i_054872),
    (0x054876, i_054876),
    (0x05487C, i_05487c),
    (0x054882, i_054882),
    (0x054888, i_054888),
    (0x05488E, i_05488e),
    (0x054892, i_054892),
    (0x054898, i_054898),
    (0x05489E, i_05489e),
    (0x0548A4, i_0548a4),
    (0x0548AA, i_0548aa),
    (0x0548B0, i_0548b0),
    (0x0548B4, i_0548b4),
    (0x0548B6, i_0548b6),
    (0x0548BC, i_0548bc),
    (0x0548C0, i_0548c0),
    (0x0548C6, i_0548c6),
    (0x0548C8, i_0548c8),
    (0x0548CC, i_0548cc),
    (0x0548D2, i_0548d2),
    (0x0548D8, i_0548d8),
    (0x0548DE, i_0548de),
    (0x0548E2, i_0548e2),
    (0x0548E4, i_0548e4),
    (0x0548EA, i_0548ea),
    (0x0548F0, i_0548f0),
    (0x0548F4, i_0548f4),
    (0x0548F8, i_0548f8),
    (0x0548FA, i_0548fa),
    (0x054900, i_054900),
    (0x054904, i_054904),
    (0x05490A, i_05490a),
    (0x05490C, i_05490c),
    (0x054912, i_054912),
    (0x054918, i_054918),
    (0x05491E, i_05491e),
    (0x054924, i_054924),
    (0x05492A, i_05492a),
    (0x054930, i_054930),
    (0x054936, i_054936),
    (0x05493A, i_05493a),
    (0x05493C, i_05493c),
    (0x054942, i_054942),
    (0x054948, i_054948),
    (0x05494E, i_05494e),
    (0x054952, i_054952),
    (0x054958, i_054958),
    (0x05495A, i_05495a),
    (0x05495E, i_05495e),
    (0x054964, i_054964),
    (0x054968, i_054968),
    (0x05496E, i_05496e),
    (0x054974, i_054974),
    (0x05497A, i_05497a),
    (0x054980, i_054980),
    (0x054986, i_054986),
    (0x05498C, i_05498c),
    (0x054992, i_054992),
    (0x054998, i_054998),
    (0x05499C, i_05499c),
    (0x05499E, i_05499e),
    (0x0549A4, i_0549a4),
    (0x0549AA, i_0549aa),
    (0x0549B0, i_0549b0),
    (0x0549B4, i_0549b4),
    (0x0549BA, i_0549ba),
    (0x0549C0, i_0549c0),
    (0x0549C4, i_0549c4),
    (0x0549CA, i_0549ca),
    (0x0549CE, i_0549ce),
    (0x0549D4, i_0549d4),
    (0x0549DA, i_0549da),
    (0x0549E0, i_0549e0),
    (0x0549E6, i_0549e6),
    (0x0549EC, i_0549ec),
    (0x0549F2, i_0549f2),
    (0x0549F8, i_0549f8),
    (0x0549FE, i_0549fe),
    (0x054A04, i_054a04),
    (0x054A08, i_054a08),
    (0x054A0E, i_054a0e),
    (0x054A12, i_054a12),
    (0x054A18, i_054a18),
    (0x054A1A, i_054a1a),
    (0x054A1E, i_054a1e),
    (0x054A24, i_054a24),
    (0x054A28, i_054a28),
    (0x054A2E, i_054a2e),
    (0x054A34, i_054a34),
    (0x054A3A, i_054a3a),
    (0x054A40, i_054a40),
    (0x054A46, i_054a46),
    (0x054A4C, i_054a4c),
    (0x054A52, i_054a52),
    (0x054A58, i_054a58),
    (0x054A5E, i_054a5e),
    (0x054A62, i_054a62),
    (0x054A64, i_054a64),
    (0x054A6A, i_054a6a),
    (0x054A70, i_054a70),
    (0x054A74, i_054a74),
    (0x054A7A, i_054a7a),
    (0x054A80, i_054a80),
    (0x054A86, i_054a86),
    (0x054A8A, i_054a8a),
    (0x054A90, i_054a90),
    (0x054A96, i_054a96),
    (0x054A9C, i_054a9c),
    (0x054AA0, i_054aa0),
    (0x054AA6, i_054aa6),
    (0x054AA8, i_054aa8),
    (0x054AAC, i_054aac),
    (0x054AB2, i_054ab2),
    (0x054AB6, i_054ab6),
    (0x054ABC, i_054abc),
    (0x054AC2, i_054ac2),
    (0x054AC6, i_054ac6),
    (0x054ACC, i_054acc),
    (0x054AD2, i_054ad2),
    (0x054AD8, i_054ad8),
    (0x054ADC, i_054adc),
    (0x054AE2, i_054ae2),
    (0x054AE8, i_054ae8),
    (0x054AEE, i_054aee),
    (0x054AF4, i_054af4),
    (0x054AFA, i_054afa),
    (0x054B00, i_054b00),
    (0x054B06, i_054b06),
    (0x054B0A, i_054b0a),
    (0x054B10, i_054b10),
    (0x054B14, i_054b14),
    (0x054B1A, i_054b1a),
    (0x054B1C, i_054b1c),
    (0x054B20, i_054b20),
    (0x054B26, i_054b26),
    (0x054B2A, i_054b2a),
    (0x054B30, i_054b30),
    (0x054B36, i_054b36),
    (0x054B3C, i_054b3c),
    (0x054B42, i_054b42),
    (0x054B48, i_054b48),
    (0x054B4E, i_054b4e),
    (0x054B54, i_054b54),
    (0x054B5A, i_054b5a),
    (0x054B5E, i_054b5e),
    (0x054B60, i_054b60),
    (0x054B66, i_054b66),
    (0x054B6C, i_054b6c),
    (0x054B72, i_054b72),
    (0x054B76, i_054b76),
    (0x054B7C, i_054b7c),
    (0x054B82, i_054b82),
    (0x054B88, i_054b88),
    (0x054B8E, i_054b8e),
    (0x054B92, i_054b92),
    (0x054B98, i_054b98),
    (0x054B9C, i_054b9c),
    (0x054BA2, i_054ba2),
    (0x054BA8, i_054ba8),
    (0x054BAE, i_054bae),
    (0x054BB4, i_054bb4),
    (0x054BBA, i_054bba),
    (0x054BC0, i_054bc0),
    (0x054BC6, i_054bc6),
    (0x054BCA, i_054bca),
    (0x054BCC, i_054bcc),
    (0x054BD2, i_054bd2),
    (0x054BD6, i_054bd6),
    (0x054BDC, i_054bdc),
    (0x054BDE, i_054bde),
    (0x054BE4, i_054be4),
    (0x054BEA, i_054bea),
    (0x054BEE, i_054bee),
    (0x054BF0, i_054bf0),
    (0x054BF6, i_054bf6),
    (0x054BFC, i_054bfc),
    (0x054C02, i_054c02),
    (0x054C06, i_054c06),
    (0x054C0C, i_054c0c),
    (0x054C0E, i_054c0e),
    (0x054C12, i_054c12),
    (0x054C18, i_054c18),
    (0x054C1E, i_054c1e),
    (0x054C24, i_054c24),
    (0x054C2A, i_054c2a),
    (0x054C30, i_054c30),
    (0x054C36, i_054c36),
    (0x054C3C, i_054c3c),
    (0x054C40, i_054c40),
    (0x054C46, i_054c46),
    (0x054C4C, i_054c4c),
    (0x054C50, i_054c50),
    (0x054C52, i_054c52),
    (0x054C58, i_054c58),
    (0x054C5E, i_054c5e),
    (0x054C64, i_054c64),
    (0x054C6A, i_054c6a),
    (0x054C6E, i_054c6e),
    (0x054C74, i_054c74),
    (0x054C7A, i_054c7a),
    (0x054C80, i_054c80),
    (0x054C86, i_054c86),
    (0x054C8A, i_054c8a),
    (0x054C90, i_054c90),
    (0x054C96, i_054c96),
    (0x054C9A, i_054c9a),
    (0x054CA0, i_054ca0),
    (0x054CA6, i_054ca6),
    (0x054CAC, i_054cac),
    (0x054CB2, i_054cb2),
    (0x054CB8, i_054cb8),
    (0x054CBE, i_054cbe),
    (0x054CC4, i_054cc4),
    (0x054CCA, i_054cca),
    (0x054CD0, i_054cd0),
    (0x054CD6, i_054cd6),
    (0x054CDC, i_054cdc),
    (0x054CE0, i_054ce0),
    (0x054CE6, i_054ce6),
    (0x054CEA, i_054cea),
    (0x054CF0, i_054cf0),
    (0x054CF2, i_054cf2),
    (0x054CF6, i_054cf6),
    (0x054CFC, i_054cfc),
    (0x054D02, i_054d02),
    (0x054D08, i_054d08),
    (0x054D0E, i_054d0e),
    (0x054D8E, i_054d8e),
    (0x054D92, i_054d92),
    (0x054D98, i_054d98),
    (0x054D9E, i_054d9e),
    (0x054DA2, i_054da2),
    (0x054DA8, i_054da8),
    (0x054DAE, i_054dae),
    (0x054DB4, i_054db4),
    (0x054DB8, i_054db8),
    (0x054DBC, i_054dbc),
    (0x054DC2, i_054dc2),
    (0x054DC8, i_054dc8),
    (0x054DCE, i_054dce),
    (0x054DD4, i_054dd4),
    (0x054DDA, i_054dda),
    (0x054DE0, i_054de0),
    (0x054DE6, i_054de6),
    (0x054DEA, i_054dea),
    (0x054DF0, i_054df0),
    (0x054DF6, i_054df6),
    (0x054DFA, i_054dfa),
    (0x054E00, i_054e00),
    (0x054E04, i_054e04),
    (0x054E08, i_054e08),
    (0x054E0C, i_054e0c),
    (0x054E10, i_054e10),
    (0x054E12, i_054e12),
    (0x054E18, i_054e18),
    (0x054E1E, i_054e1e),
    (0x054E24, i_054e24),
    (0x054E28, i_054e28),
    (0x054E2E, i_054e2e),
    (0x054E32, i_054e32),
    (0x054E38, i_054e38),
    (0x054E3E, i_054e3e),
    (0x054E44, i_054e44),
    (0x054E4A, i_054e4a),
    (0x054E4E, i_054e4e),
    (0x054E52, i_054e52),
    (0x054E58, i_054e58),
    (0x054E5C, i_054e5c),
    (0x054E5E, i_054e5e),
    (0x054E64, i_054e64),
    (0x054E68, i_054e68),
    (0x054E6E, i_054e6e),
    (0x054E70, i_054e70),
    (0x054E74, i_054e74),
    (0x054E7A, i_054e7a),
    (0x054E7C, i_054e7c),
    (0x054E80, i_054e80),
    (0x054E86, i_054e86),
    (0x054E8A, i_054e8a),
    (0x054E90, i_054e90),
    (0x054E94, i_054e94),
    (0x054E96, i_054e96),
    (0x054E9C, i_054e9c),
    (0x054EA2, i_054ea2),
    (0x054EA8, i_054ea8),
    (0x054EAC, i_054eac),
    (0x054EB2, i_054eb2),
    (0x054EB8, i_054eb8),
    (0x054EBE, i_054ebe),
    (0x054EC4, i_054ec4),
    (0x054EC8, i_054ec8),
    (0x054ECC, i_054ecc),
    (0x054ED2, i_054ed2),
    (0x054ED6, i_054ed6),
    (0x054ED8, i_054ed8),
    (0x054EDE, i_054ede),
    (0x054EE2, i_054ee2),
    (0x054EE8, i_054ee8),
    (0x054EEA, i_054eea),
    (0x054EEE, i_054eee),
    (0x054EF4, i_054ef4),
    (0x054EF6, i_054ef6),
    (0x054EFA, i_054efa),
    (0x054F00, i_054f00),
    (0x054F04, i_054f04),
    (0x054F0A, i_054f0a),
    (0x054F0E, i_054f0e),
    (0x054F10, i_054f10),
    (0x054F16, i_054f16),
    (0x054F1C, i_054f1c),
    (0x054F22, i_054f22),
    (0x054F26, i_054f26),
    (0x054F2C, i_054f2c),
    (0x054F32, i_054f32),
    (0x054F38, i_054f38),
    (0x054F3E, i_054f3e),
    (0x054F42, i_054f42),
    (0x054F48, i_054f48),
    (0x054F4C, i_054f4c),
    (0x054F52, i_054f52),
    (0x054F56, i_054f56),
    (0x054F58, i_054f58),
    (0x054F5E, i_054f5e),
    (0x054F62, i_054f62),
    (0x054F68, i_054f68),
    (0x054F6A, i_054f6a),
    (0x054F70, i_054f70),
    (0x054F76, i_054f76),
    (0x054F7A, i_054f7a),
    (0x054F80, i_054f80),
    (0x054F82, i_054f82),
    (0x054F86, i_054f86),
    (0x054F8C, i_054f8c),
    (0x054F90, i_054f90),
    (0x054F96, i_054f96),
    (0x054F9A, i_054f9a),
    (0x054F9C, i_054f9c),
    (0x054FA2, i_054fa2),
    (0x054FA8, i_054fa8),
    (0x054FAE, i_054fae),
    (0x054FB2, i_054fb2),
    (0x054FB8, i_054fb8),
    (0x054FBA, i_054fba),
    (0x054FBE, i_054fbe),
    (0x054FC2, i_054fc2),
    (0x054FC6, i_054fc6),
    (0x054FCA, i_054fca),
    (0x054FD0, i_054fd0),
    (0x054FD6, i_054fd6),
    (0x054FDA, i_054fda),
    (0x054FDC, i_054fdc),
    (0x054FE0, i_054fe0),
    (0x054FE4, i_054fe4),
    (0x054FE8, i_054fe8),
    (0x054FEA, i_054fea),
    (0x054FF0, i_054ff0),
    (0x054FF2, i_054ff2),
    (0x054FF6, i_054ff6),
    (0x054FFA, i_054ffa),
    (0x055000, i_055000),
    (0x055002, i_055002),
    (0x055006, i_055006),
    (0x05500A, i_05500a),
    (0x05500E, i_05500e),
    (0x055012, i_055012),
    (0x055018, i_055018),
    (0x05501E, i_05501e),
    (0x055022, i_055022),
    (0x055024, i_055024),
    (0x055028, i_055028),
    (0x05502C, i_05502c),
    (0x055030, i_055030),
    (0x055032, i_055032),
    (0x055038, i_055038),
    (0x05503A, i_05503a),
    (0x05503E, i_05503e),
    (0x055042, i_055042),
    (0x055048, i_055048),
    (0x05504A, i_05504a),
    (0x05504E, i_05504e),
    (0x055054, i_055054),
    (0x055058, i_055058),
    (0x05505E, i_05505e),
    (0x055064, i_055064),
    (0x05506A, i_05506a),
    (0x055070, i_055070),
    (0x055078, i_055078),
    (0x05507C, i_05507c),
    (0x05507E, i_05507e),
    (0x055084, i_055084),
    (0x05508A, i_05508a),
    (0x055090, i_055090),
    (0x055094, i_055094),
    (0x05509A, i_05509a),
    (0x05509C, i_05509c),
    (0x0550A2, i_0550a2),
    (0x0550A6, i_0550a6),
    (0x0550AC, i_0550ac),
    (0x0550B0, i_0550b0),
    (0x0550B4, i_0550b4),
    (0x0550B8, i_0550b8),
    (0x0550BE, i_0550be),
    (0x0550C2, i_0550c2),
    (0x0550C4, i_0550c4),
    (0x0550C8, i_0550c8),
    (0x0550CE, i_0550ce),
    (0x0550D2, i_0550d2),
    (0x0550D8, i_0550d8),
    (0x0550DC, i_0550dc),
    (0x0550E0, i_0550e0),
    (0x0550E6, i_0550e6),
    (0x0550E8, i_0550e8),
    (0x0550EC, i_0550ec),
    (0x0550F2, i_0550f2),
    (0x0550F6, i_0550f6),
    (0x0550FA, i_0550fa),
    (0x0550FE, i_0550fe),
    (0x055104, i_055104),
    (0x055108, i_055108),
    (0x05510C, i_05510c),
    (0x05510E, i_05510e),
    (0x055112, i_055112),
    (0x055118, i_055118),
    (0x05511C, i_05511c),
    (0x055120, i_055120),
    (0x055122, i_055122),
    (0x055126, i_055126),
    (0x05512A, i_05512a),
    (0x05512E, i_05512e),
    (0x055132, i_055132),
    (0x055136, i_055136),
    (0x05513A, i_05513a),
    (0x05513E, i_05513e),
    (0x055142, i_055142),
    (0x055146, i_055146),
    (0x055148, i_055148),
    (0x05514C, i_05514c),
    (0x055150, i_055150),
    (0x055154, i_055154),
    (0x055158, i_055158),
    (0x05515E, i_05515e),
    (0x055162, i_055162),
    (0x055166, i_055166),
    (0x05516A, i_05516a),
    (0x05516E, i_05516e),
    (0x055174, i_055174),
    (0x055178, i_055178),
    (0x05517C, i_05517c),
    (0x055180, i_055180),
    (0x055184, i_055184),
    (0x05518A, i_05518a),
    (0x05518C, i_05518c),
    (0x055190, i_055190),
    (0x055194, i_055194),
    (0x055198, i_055198),
    (0x05519C, i_05519c),
    (0x0551A2, i_0551a2),
    (0x0551A6, i_0551a6),
    (0x0551AA, i_0551aa),
    (0x0551AE, i_0551ae),
    (0x0551B2, i_0551b2),
    (0x0551B8, i_0551b8),
    (0x0551BC, i_0551bc),
    (0x0551C0, i_0551c0),
    (0x0551C4, i_0551c4),
    (0x0551C8, i_0551c8),
    (0x0551CE, i_0551ce),
    (0x0551D0, i_0551d0),
    (0x0551D4, i_0551d4),
    (0x0551D8, i_0551d8),
    (0x0551DC, i_0551dc),
    (0x0551E0, i_0551e0),
    (0x0551E6, i_0551e6),
    (0x0551EA, i_0551ea),
    (0x0551EE, i_0551ee),
    (0x0551F2, i_0551f2),
    (0x0551F6, i_0551f6),
    (0x0551FC, i_0551fc),
    (0x055200, i_055200),
    (0x055204, i_055204),
    (0x055208, i_055208),
    (0x05520C, i_05520c),
    (0x055212, i_055212),
    (0x055214, i_055214),
    (0x055218, i_055218),
    (0x05521C, i_05521c),
    (0x055220, i_055220),
    (0x055224, i_055224),
    (0x05522A, i_05522a),
    (0x05522E, i_05522e),
    (0x055232, i_055232),
    (0x055236, i_055236),
    (0x05523A, i_05523a),
    (0x055240, i_055240),
    (0x055244, i_055244),
    (0x055248, i_055248),
    (0x05524C, i_05524c),
    (0x055250, i_055250),
    (0x055256, i_055256),
    (0x055258, i_055258),
    (0x05525E, i_05525e),
    (0x055264, i_055264),
    (0x055266, i_055266),
    (0x05526C, i_05526c),
    (0x055272, i_055272),
    (0x055274, i_055274),
    (0x05527A, i_05527a),
    (0x055280, i_055280),
    (0x055282, i_055282),
    (0x055288, i_055288),
    (0x05528E, i_05528e),
    (0x055290, i_055290),
    (0x055296, i_055296),
    (0x05529C, i_05529c),
    (0x05529E, i_05529e),
    (0x0552A4, i_0552a4),
    (0x0552AA, i_0552aa),
    (0x0552AC, i_0552ac),
    (0x0552B2, i_0552b2),
    (0x0552B8, i_0552b8),
    (0x0552BA, i_0552ba),
    (0x0552C0, i_0552c0),
    (0x0552C6, i_0552c6),
    (0x0552C8, i_0552c8),
    (0x0552CE, i_0552ce),
    (0x0552D4, i_0552d4),
    (0x0552D6, i_0552d6),
    (0x0552DC, i_0552dc),
    (0x0552E2, i_0552e2),
    (0x0552E4, i_0552e4),
    (0x0552EA, i_0552ea),
    (0x0552F0, i_0552f0),
    (0x0552F2, i_0552f2),
    (0x0552F8, i_0552f8),
    (0x0552FE, i_0552fe),
    (0x055300, i_055300),
    (0x055306, i_055306),
    (0x05530C, i_05530c),
    (0x05530E, i_05530e),
    (0x055314, i_055314),
    (0x05531A, i_05531a),
    (0x05531C, i_05531c),
    (0x055322, i_055322),
    (0x055328, i_055328),
    (0x05532A, i_05532a),
    (0x055330, i_055330),
    (0x055336, i_055336),
    (0x055338, i_055338),
    (0x05533E, i_05533e),
    (0x055344, i_055344),
    (0x055346, i_055346),
    (0x05534C, i_05534c),
    (0x055352, i_055352),
    (0x055354, i_055354),
    (0x05535A, i_05535a),
    (0x055360, i_055360),
    (0x055362, i_055362),
    (0x055368, i_055368),
    (0x05536E, i_05536e),
    (0x055370, i_055370),
    (0x055376, i_055376),
    (0x05537C, i_05537c),
    (0x05537E, i_05537e),
    (0x055384, i_055384),
    (0x05538A, i_05538a),
    (0x05538C, i_05538c),
    (0x055392, i_055392),
    (0x055398, i_055398),
    (0x05539A, i_05539a),
    (0x0553A0, i_0553a0),
    (0x0553A6, i_0553a6),
    (0x0553A8, i_0553a8),
    (0x0553AE, i_0553ae),
    (0x0553B4, i_0553b4),
    (0x0553B6, i_0553b6),
    (0x0553BC, i_0553bc),
    (0x0553C2, i_0553c2),
    (0x0553C4, i_0553c4),
    (0x0553CA, i_0553ca),
    (0x0553D0, i_0553d0),
    (0x0553D2, i_0553d2),
    (0x0553D8, i_0553d8),
    (0x0553DE, i_0553de),
    (0x0553E0, i_0553e0),
    (0x0553E6, i_0553e6),
    (0x0553EC, i_0553ec),
    (0x0553EE, i_0553ee),
    (0x0553F4, i_0553f4),
    (0x0553FA, i_0553fa),
    (0x0553FC, i_0553fc),
    (0x055402, i_055402),
    (0x055408, i_055408),
    (0x05540A, i_05540a),
    (0x055410, i_055410),
    (0x055416, i_055416),
    (0x055418, i_055418),
    (0x05541E, i_05541e),
    (0x055424, i_055424),
    (0x055426, i_055426),
    (0x05542C, i_05542c),
    (0x055432, i_055432),
    (0x055434, i_055434),
    (0x05543A, i_05543a),
    (0x055440, i_055440),
    (0x055442, i_055442),
    (0x055448, i_055448),
    (0x05544E, i_05544e),
    (0x055450, i_055450),
    (0x055456, i_055456),
    (0x05545C, i_05545c),
    (0x05545E, i_05545e),
    (0x055464, i_055464),
    (0x05546A, i_05546a),
    (0x05546C, i_05546c),
    (0x055472, i_055472),
    (0x055478, i_055478),
    (0x05547A, i_05547a),
    (0x055480, i_055480),
    (0x055486, i_055486),
    (0x055488, i_055488),
    (0x05548E, i_05548e),
    (0x055494, i_055494),
    (0x055496, i_055496),
    (0x05549C, i_05549c),
    (0x0554A2, i_0554a2),
    (0x0554A4, i_0554a4),
    (0x0554AA, i_0554aa),
    (0x0554B0, i_0554b0),
    (0x0554B2, i_0554b2),
    (0x0554B8, i_0554b8),
    (0x0554BE, i_0554be),
    (0x0554C0, i_0554c0),
    (0x0554C6, i_0554c6),
    (0x0554CC, i_0554cc),
    (0x0554CE, i_0554ce),
    (0x0554D4, i_0554d4),
    (0x0554DA, i_0554da),
    (0x0554DC, i_0554dc),
    (0x0554E2, i_0554e2),
    (0x0554E8, i_0554e8),
    (0x0554EA, i_0554ea),
    (0x0554F0, i_0554f0),
    (0x0554F6, i_0554f6),
    (0x0554F8, i_0554f8),
    (0x0554FE, i_0554fe),
    (0x055504, i_055504),
    (0x055506, i_055506),
    (0x05550C, i_05550c),
    (0x055512, i_055512),
    (0x055514, i_055514),
    (0x05551A, i_05551a),
    (0x055520, i_055520),
    (0x055522, i_055522),
    (0x055528, i_055528),
    (0x05552E, i_05552e),
    (0x055530, i_055530),
    (0x055536, i_055536),
    (0x05553C, i_05553c),
    (0x05553E, i_05553e),
    (0x055544, i_055544),
    (0x05554A, i_05554a),
    (0x05554C, i_05554c),
    (0x055552, i_055552),
    (0x055558, i_055558),
    (0x05555A, i_05555a),
    (0x055560, i_055560),
    (0x055566, i_055566),
    (0x055568, i_055568),
    (0x05556E, i_05556e),
    (0x055574, i_055574),
    (0x055576, i_055576),
    (0x05557C, i_05557c),
    (0x055582, i_055582),
    (0x055584, i_055584),
    (0x05558A, i_05558a),
    (0x055590, i_055590),
    (0x055592, i_055592),
    (0x055598, i_055598),
    (0x05559E, i_05559e),
    (0x0555A0, i_0555a0),
    (0x0555A6, i_0555a6),
    (0x0555AC, i_0555ac),
    (0x0555AE, i_0555ae),
    (0x0555B4, i_0555b4),
    (0x0555BA, i_0555ba),
    (0x0555BC, i_0555bc),
    (0x0555C2, i_0555c2),
    (0x0555C8, i_0555c8),
    (0x0555CA, i_0555ca),
    (0x0555D0, i_0555d0),
    (0x0555D6, i_0555d6),
    (0x0555D8, i_0555d8),
    (0x0555DE, i_0555de),
    (0x0555E4, i_0555e4),
    (0x0555E6, i_0555e6),
    (0x0555EC, i_0555ec),
    (0x0555F2, i_0555f2),
    (0x0555F4, i_0555f4),
    (0x0555FA, i_0555fa),
    (0x055600, i_055600),
    (0x055602, i_055602),
    (0x055608, i_055608),
    (0x05560E, i_05560e),
    (0x055610, i_055610),
    (0x055616, i_055616),
    (0x05561C, i_05561c),
    (0x05561E, i_05561e),
    (0x055624, i_055624),
    (0x05562A, i_05562a),
    (0x05562C, i_05562c),
    (0x055632, i_055632),
    (0x055638, i_055638),
    (0x05563A, i_05563a),
    (0x055640, i_055640),
    (0x055646, i_055646),
    (0x055648, i_055648),
    (0x05564E, i_05564e),
    (0x055654, i_055654),
    (0x055656, i_055656),
    (0x05565C, i_05565c),
    (0x055662, i_055662),
    (0x055664, i_055664),
    (0x05566A, i_05566a),
    (0x055670, i_055670),
    (0x055672, i_055672),
    (0x055678, i_055678),
    (0x05567E, i_05567e),
    (0x055680, i_055680),
    (0x055686, i_055686),
    (0x05568C, i_05568c),
    (0x05568E, i_05568e),
    (0x055694, i_055694),
    (0x05569A, i_05569a),
    (0x05569C, i_05569c),
    (0x0556A2, i_0556a2),
    (0x0556A8, i_0556a8),
    (0x0556AA, i_0556aa),
    (0x0556B0, i_0556b0),
    (0x0556B6, i_0556b6),
    (0x0556B8, i_0556b8),
    (0x0556BE, i_0556be),
    (0x0556C4, i_0556c4),
    (0x0556C6, i_0556c6),
    (0x0556CC, i_0556cc),
    (0x0556D2, i_0556d2),
    (0x0556D4, i_0556d4),
    (0x0556DA, i_0556da),
    (0x0556E0, i_0556e0),
    (0x0556E2, i_0556e2),
    (0x0556E8, i_0556e8),
    (0x0556EE, i_0556ee),
    (0x0556F0, i_0556f0),
    (0x0556F6, i_0556f6),
    (0x0556FC, i_0556fc),
    (0x0556FE, i_0556fe),
    (0x055704, i_055704),
    (0x05570A, i_05570a),
    (0x05570C, i_05570c),
    (0x055712, i_055712),
    (0x055718, i_055718),
    (0x05571A, i_05571a),
    (0x055720, i_055720),
    (0x055726, i_055726),
    (0x055728, i_055728),
    (0x05572E, i_05572e),
    (0x055734, i_055734),
    (0x055736, i_055736),
    (0x05573C, i_05573c),
    (0x055742, i_055742),
    (0x055744, i_055744),
    (0x05574A, i_05574a),
    (0x055750, i_055750),
    (0x055752, i_055752),
    (0x055758, i_055758),
    (0x05575E, i_05575e),
    (0x055760, i_055760),
    (0x055766, i_055766),
    (0x05576C, i_05576c),
    (0x05576E, i_05576e),
    (0x055774, i_055774),
    (0x05577A, i_05577a),
    (0x05577C, i_05577c),
    (0x055782, i_055782),
    (0x055788, i_055788),
    (0x05578A, i_05578a),
    (0x055790, i_055790),
    (0x055796, i_055796),
    (0x055798, i_055798),
    (0x05579E, i_05579e),
    (0x0557A4, i_0557a4),
    (0x0557A6, i_0557a6),
    (0x0557AC, i_0557ac),
    (0x0557B2, i_0557b2),
    (0x0557B4, i_0557b4),
    (0x0557BA, i_0557ba),
    (0x0557C0, i_0557c0),
    (0x0557C2, i_0557c2),
    (0x0557C8, i_0557c8),
    (0x0557CE, i_0557ce),
    (0x0557D0, i_0557d0),
    (0x0557D6, i_0557d6),
    (0x0557DC, i_0557dc),
    (0x0557DE, i_0557de),
    (0x0557E4, i_0557e4),
    (0x0557EA, i_0557ea),
    (0x0557EC, i_0557ec),
    (0x0557F2, i_0557f2),
    (0x0557F8, i_0557f8),
    (0x0557FA, i_0557fa),
    (0x055800, i_055800),
    (0x055806, i_055806),
    (0x055808, i_055808),
    (0x05580E, i_05580e),
    (0x055814, i_055814),
    (0x055816, i_055816),
    (0x05581C, i_05581c),
    (0x055822, i_055822),
    (0x055824, i_055824),
    (0x05582A, i_05582a),
    (0x055830, i_055830),
    (0x055832, i_055832),
    (0x055838, i_055838),
    (0x05583E, i_05583e),
    (0x055840, i_055840),
    (0x055846, i_055846),
    (0x05584C, i_05584c),
    (0x05584E, i_05584e),
    (0x055854, i_055854),
    (0x05585A, i_05585a),
    (0x05585C, i_05585c),
    (0x055862, i_055862),
    (0x055868, i_055868),
    (0x05586A, i_05586a),
    (0x055870, i_055870),
    (0x055876, i_055876),
    (0x055878, i_055878),
    (0x05587E, i_05587e),
    (0x055884, i_055884),
    (0x055886, i_055886),
    (0x05588C, i_05588c),
    (0x055892, i_055892),
    (0x055894, i_055894),
    (0x05589A, i_05589a),
    (0x0558A0, i_0558a0),
    (0x0558A2, i_0558a2),
    (0x0558A8, i_0558a8),
    (0x0558AE, i_0558ae),
    (0x0558B0, i_0558b0),
    (0x0558B6, i_0558b6),
    (0x0558BC, i_0558bc),
    (0x0558BE, i_0558be),
    (0x0558C4, i_0558c4),
    (0x0558CA, i_0558ca),
    (0x0558CC, i_0558cc),
    (0x0558D2, i_0558d2),
    (0x0558D8, i_0558d8),
    (0x0558DA, i_0558da),
    (0x0558E0, i_0558e0),
    (0x0558E6, i_0558e6),
    (0x0558E8, i_0558e8),
    (0x0558EE, i_0558ee),
];
