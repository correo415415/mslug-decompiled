// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p08_1) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


// ===== SetTaskHandler_082b3e  ($082B3E, 8 B, kernel/task_handlers.c)
/// $082B3E: lea.l $82456(pc), a1
pub fn i_082b3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B42;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82456;
    16
}
/// $082B42: move.l a1, (a6)
pub fn i_082b42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B44;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082B44: rts
pub fn i_082b44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B46;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_082b46  ($082B46, 106 B, enemies/squad/para_squad_helpers_08283c.s)
/// $082B46: move.w #$76, d1
pub fn i_082b46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B4A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x76;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $082B4A: jsr $236e.l
pub fn i_082b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B50;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82B50);
    cpu.pc = 0x236E;
    16
}
/// $082B50: move.w #$d000, d0
pub fn i_082b50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B54;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $082B54: jsr $28134.l
pub fn i_082b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B5A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82B5A);
    cpu.pc = 0x28134;
    16
}
/// $082B5A: andi.w #$ffe3, $38(a6)
pub fn i_082b5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B60;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $082B60: ori.w #$1c, $38(a6)
pub fn i_082b60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B66;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $082B66: move.w #$10, $28(a6)
pub fn i_082b66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B6C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $082B6C: move.w #$c0, $2a(a6)
pub fn i_082b6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B72;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $082B72: lea.l $2e6aea.l, a0
pub fn i_082b72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B78;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6AEA;
    20
}
/// $082B78: jsr $28cd4.l
pub fn i_082b78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82B7E);
    cpu.pc = 0x28CD4;
    16
}
/// $082B7E: lea.l $82b84(pc), a1
pub fn i_082b7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B82;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82B84;
    16
}
/// $082B82: move.l a1, (a6)
pub fn i_082b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B84;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082B84: jsr $27cee.l
pub fn i_082b84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82B8A);
    cpu.pc = 0x27CEE;
    16
}
/// $082B8A: jsr $28d70.l
pub fn i_082b8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82B90);
    cpu.pc = 0x28D70;
    16
}
/// $082B90: bcc.w $82b9a
pub fn i_082b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B94;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x82B9A; return 10; }
    8
}
/// $082B94: lea.l $82456(pc), a1
pub fn i_082b94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B98;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82456;
    16
}
/// $082B98: move.l a1, (a6)
pub fn i_082b98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B9A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082B9A: movea.l #$ffffffff, a0
pub fn i_082b9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82BA0;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $082BA0: lea.l $2e5416.l, a0
pub fn i_082ba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82BA6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E5416;
    20
}
/// $082BA6: jsr $5dd56.l
pub fn i_082ba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82BAC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82BAC);
    cpu.pc = 0x5DD56;
    16
}
/// $082BAC: bcc.w $82bb6
pub fn i_082bac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82BB0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x82BB6; return 10; }
    8
}
// ===== SetTaskHandler_082bb0  ($082BB0, 8 B, kernel/task_handlers.c)
/// $082BB0: lea.l $82456(pc), a1
pub fn i_082bb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82BB4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82456;
    16
}
/// $082BB4: move.l a1, (a6)
pub fn i_082bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82BB6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082BB6: rts
pub fn i_082bb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82BB8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_082bb8  ($082BB8, 72 B, enemies/squad/para_squad_helpers_08283c.s)
/// $082BB8: move.w #$c000, d0
pub fn i_082bb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82BBC;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $082BBC: jsr $28134.l
pub fn i_082bbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82BC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82BC2);
    cpu.pc = 0x28134;
    16
}
/// $082BC2: andi.w #$ffe3, $38(a6)
pub fn i_082bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82BC8;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $082BC8: ori.w #$1c, $38(a6)
pub fn i_082bc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82BCE;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $082BCE: lea.l $2e6992.l, a0
pub fn i_082bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82BD4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6992;
    20
}
/// $082BD4: jsr $28cd4.l
pub fn i_082bd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82BDA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82BDA);
    cpu.pc = 0x28CD4;
    16
}
/// $082BDA: lea.l $82be0(pc), a1
pub fn i_082bda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82BDE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82BE0;
    16
}
/// $082BDE: move.l a1, (a6)
pub fn i_082bde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82BE0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082BE0: jsr $2783a.l
pub fn i_082be0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82BE6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82BE6);
    cpu.pc = 0x2783A;
    16
}
/// $082BE6: jsr $28d70.l
pub fn i_082be6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82BEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82BEC);
    cpu.pc = 0x28D70;
    16
}
/// $082BEC: bcc.w $82bf6
pub fn i_082bec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82BF0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x82BF6; return 10; }
    8
}
/// $082BF0: lea.l $82464(pc), a1
pub fn i_082bf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82BF4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82464;
    16
}
/// $082BF4: move.l a1, (a6)
pub fn i_082bf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82BF6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082BF6: jsr $5e45a.l
pub fn i_082bf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82BFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82BFC);
    cpu.pc = 0x5E45A;
    16
}
/// $082BFC: bcc.w $82c06
pub fn i_082bfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C00;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x82C06; return 10; }
    8
}
// ===== SetTaskHandler_082c00  ($082C00, 8 B, kernel/task_handlers.c)
/// $082C00: lea.l $82456(pc), a1
pub fn i_082c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C04;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82456;
    16
}
/// $082C04: move.l a1, (a6)
pub fn i_082c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C06;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082C06: rts
pub fn i_082c06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C08;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_082c08  ($082C08, 108 B, enemies/squad/para_squad_helpers_08283c.s)
/// $082C08: jsr $5e9b6.l
pub fn i_082c08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82C0E);
    cpu.pc = 0x5E9B6;
    16
}
/// $082C0E: andi.w #$7, d0
pub fn i_082c0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C12;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $082C12: move.w d0, $5c(a6)
pub fn i_082c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C16;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $082C16: add.w d0, d0
pub fn i_082c16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C18;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $082C18: lea.l $2e596a.l, a0
pub fn i_082c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C1E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E596A;
    20
}
/// $082C1E: move.w (a0, d0.w), d1
pub fn i_082c1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C22;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $082C22: jsr $236e.l
pub fn i_082c22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82C28);
    cpu.pc = 0x236E;
    16
}
/// $082C28: move.w $5c(a6), d0
pub fn i_082c28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C2C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $082C2C: movea.l #$2e594a, a0
pub fn i_082c2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C32;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2E594A;
    cpu.a[0] = v1;
    4
}
/// $082C32: lsl.w #$2, d0
pub fn i_082c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C34;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $082C34: movea.l (a0, d0.w), a0
pub fn i_082c34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C38;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $082C38: cmpa.l #$ffffffff, a0
pub fn i_082c38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C3E;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $082C3E: beq.w $82c48
pub fn i_082c3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C42;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x82C48; return 10; }
    8
}
/// $082C42: jsr $28cd4.l
pub fn i_082c42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C48;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82C48);
    cpu.pc = 0x28CD4;
    16
}
/// $082C48: move.w #$800, $36(a6)
pub fn i_082c48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C4E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x800;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $082C4E: jmp $6dce0.l
pub fn i_082c4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C54;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x6DCE0;
    8
}
// ===== ClrRamWord_082c74  ($082C74, 8 B, thunks/clr_ram_word.c)
/// $082C74: clr.b $10e39a.l
pub fn i_082c74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C7A;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $082C7A: rts
pub fn i_082c7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C7C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_00082C7C  ($082C7C, 42 B, enemies/squad/para_squad_helpers_08283c.s)
/// $082C7C: move.b $7e(a6), d0
pub fn i_082c7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C80;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $082C80: addq.b #$1, $7e(a6)
pub fn i_082c80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C84;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x7E);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $082C84: andi.w #$f, d0
pub fn i_082c84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C88;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $082C88: add.w d0, d0
pub fn i_082c88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C8A;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $082C8A: move.w $22(a6), $80(a6)
pub fn i_082c8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C90;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad4, v3 as u16);
    20
}
/// $082C90: move.w $24(a6), d1
pub fn i_082c90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C94;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $082C94: add.w (a0, d0.w), d1
pub fn i_082c94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C98;
    cpu.ir = 0xD270;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $082C98: move.w d1, $82(a6)
pub fn i_082c98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C9C;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $082C9C: andi.b #$f, $7e(a6)
pub fn i_082c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82CA2;
    cpu.ir = 0x22E;
    let ad1 = cpu.a[6].wrapping_add(0x7E);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0xF) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $082CA2: beq.w $82cac
pub fn i_082ca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82CA6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x82CAC; return 10; }
    8
}
// ===== ClearXN_082ca6  ($082CA6, 6 B, thunks/ccr_helpers.c)
/// $082CA6: andi.b #$ee, ccr
pub fn i_082ca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82CAA;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $082CAA: rts
pub fn i_082caa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82CAC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_082cac  ($082CAC, 6 B, thunks/ccr_helpers.c)
/// $082CAC: ori.b #$11, ccr
pub fn i_082cac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82CB0;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $082CB0: rts
pub fn i_082cb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82CB2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_00082CB2  ($082CB2, 18 B, enemies/squad/para_squad_helpers_08283c.s)
/// $082CB2: clr.b $7e(a6)
pub fn i_082cb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82CB6;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x7E);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $082CB6: move.w $22(a6), $80(a6)
pub fn i_082cb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82CBC;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad4, v3 as u16);
    20
}
/// $082CBC: move.w $24(a6), $82(a6)
pub fn i_082cbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82CC2;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad4, v3 as u16);
    20
}
/// $082CC2: rts
pub fn i_082cc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82CC4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_00082CC4  ($082CC4, 30 B, enemies/squad/para_squad_helpers_08283c.s)
/// $082CC4: movea.l $c(a6), a0
pub fn i_082cc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82CC8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $082CC8: move.w $38(a0), $38(a6)
pub fn i_082cc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82CCE;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad4, v3 as u16);
    20
}
/// $082CCE: move.w $80(a0), $22(a6)
pub fn i_082cce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82CD4;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $082CD4: move.w $82(a0), $24(a6)
pub fn i_082cd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82CDA;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $082CDA: move.b $73(a0), $73(a6)
pub fn i_082cda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82CE0;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x73);
    bus.write8(ad4, v3 as u8);
    20
}
/// $082CE0: rts
pub fn i_082ce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82CE2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_00082CE2  ($082CE2, 48 B, enemies/squad/para_squad_helpers_08283c.s)
/// $082CE2: movea.l $c(a6), a0
pub fn i_082ce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82CE6;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $082CE6: move.l $94(a0), $94(a6)
pub fn i_082ce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82CEC;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0x94);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x94);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $082CEC: move.w $38(a0), $38(a6)
pub fn i_082cec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82CF2;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad4, v3 as u16);
    20
}
/// $082CF2: move.w $80(a0), d0
pub fn i_082cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82CF6;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $082CF6: add.w $80(a6), d0
pub fn i_082cf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82CFA;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $082CFA: move.w d0, $22(a6)
pub fn i_082cfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82CFE;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $082CFE: move.w $82(a0), d0
pub fn i_082cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D02;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $082D02: add.w $82(a6), d0
pub fn i_082d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D06;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $082D06: move.w d0, $24(a6)
pub fn i_082d06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D0A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $082D0A: move.b $73(a0), $73(a6)
pub fn i_082d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D10;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x73);
    bus.write8(ad4, v3 as u8);
    20
}
/// $082D10: rts
pub fn i_082d10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D12;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_00082D12  ($082D12, 46 B, enemies/squad/para_squad_helpers_08283c.s)
/// $082D12: btst.b #$6, $72(a6)
pub fn i_082d12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D18;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $082D18: beq.w $82d22
pub fn i_082d18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D1C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x82D22; return 10; }
    8
}
/// $082D1C: bset.b #$0, $5a(a6)
pub fn i_082d1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D22;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $082D22: bclr.b #$6, $72(a6)
pub fn i_082d22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D28;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $082D28: bclr.b #$7, $72(a6)
pub fn i_082d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D2E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $082D2E: btst.b #$0, $5a(a6)
pub fn i_082d2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D34;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $082D34: beq.w $82d3e
pub fn i_082d34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D38;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x82D3E; return 10; }
    8
}
/// $082D38: bset.b #$7, $72(a6)
pub fn i_082d38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D3E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $082D3E: rts
pub fn i_082d3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D40;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_00082D40  ($082D40, 48 B, enemies/squad/para_squad_helpers_08283c.s)
/// $082D40: btst.b #$0, $5a(a6)
pub fn i_082d40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D46;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $082D46: beq.w $82d5a
pub fn i_082d46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D4A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x82D5A; return 10; }
    8
}
/// $082D4A: bclr.b #$0, $5a(a6)
pub fn i_082d4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D50;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $082D50: movea.l $c(a6), a0
pub fn i_082d50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D54;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $082D54: bset.b #$6, $72(a0)
pub fn i_082d54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D5A;
    cpu.ir = 0x8E8;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $082D5A: movea.l $c(a6), a0
pub fn i_082d5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D5E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $082D5E: btst.b #$7, $72(a0)
pub fn i_082d5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D64;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $082D64: beq.w $82d6e
pub fn i_082d64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D68;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x82D6E; return 10; }
    8
}
/// $082D68: bset.b #$0, $5a(a6)
pub fn i_082d68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D6E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $082D6E: rts
pub fn i_082d6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D70;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_00082D70  ($082D70, 62 B, enemies/squad/para_squad_helpers_08283c.s)
/// $082D70: tst.b $73(a6)
pub fn i_082d70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D74;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $082D74: bne.w $82dac
pub fn i_082d74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D78;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x82DAC; return 10; }
    8
}
/// $082D78: move.b $7d(a6), d0
pub fn i_082d78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D7C;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x7D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $082D7C: andi.b #$3, d0
pub fn i_082d7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D80;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $082D80: beq.w $82da6
pub fn i_082d80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D84;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x82DA6; return 10; }
    8
}
/// $082D84: lea.l $2be098.l, a0
pub fn i_082d84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D8A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BE098;
    20
}
/// $082D8A: cmpi.b #$3, d0
pub fn i_082d8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D8E;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $082D8E: beq.w $82d98
pub fn i_082d8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D92;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x82D98; return 10; }
    8
}
/// $082D92: lea.l $2be11a.l, a0
pub fn i_082d92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D98;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BE11A;
    20
}
/// $082D98: jsr $799de.l
pub fn i_082d98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D9E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82D9E);
    cpu.pc = 0x799DE;
    16
}
/// $082D9E: cmp.w $66(a6), d0
pub fn i_082d9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82DA2;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $082DA2: blt.w $82dac
pub fn i_082da2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82DA6;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x82DAC; return 10; }
    8
}
/// $082DA6: move.b #$ff, $73(a6)
pub fn i_082da6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82DAC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    bus.write8(ad2, v1 as u8);
    12
}
/// $082DAC: rts
pub fn i_082dac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82DAE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_00082DAE  ($082DAE, 58 B, enemies/squad/para_squad_helpers_08283c.s)
/// $082DAE: btst.b #$5, $72(a6)
pub fn i_082dae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82DB4;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $082DB4: bne.w $82de8
pub fn i_082db4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82DB8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x82DE8; return 10; }
    8
}
/// $082DB8: cmpi.w #$20, $22(a6)
pub fn i_082db8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82DBE;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x20, Size::Word, false);
    16
}
/// $082DBE: blt.w $82dee
pub fn i_082dbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82DC2;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x82DEE; return 10; }
    8
}
/// $082DC2: cmpi.w #$120, $22(a6)
pub fn i_082dc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82DC8;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x120, Size::Word, false);
    16
}
/// $082DC8: bgt.w $82df8
pub fn i_082dc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82DCC;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x82DF8; return 10; }
    8
}
/// $082DCC: tst.b $7b(a6)
pub fn i_082dcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82DD0;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $082DD0: bne.w $82de8
pub fn i_082dd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82DD4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x82DE8; return 10; }
    8
}
/// $082DD4: movea.l $94(a6), a0
pub fn i_082dd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82DD8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $082DD8: move.w $22(a0), d0
pub fn i_082dd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82DDC;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $082DDC: cmp.w $22(a6), d0
pub fn i_082ddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82DE0;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $082DE0: bgt.w $82dee
pub fn i_082de0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82DE4;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x82DEE; return 10; }
    8
}
/// $082DE4: bra.w $82df8
pub fn i_082de4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82DE8;
    cpu.ir = 0x6000;
    cpu.pc = 0x82DF8;
    10
}
// ===== ClearXN_082de8  ($082DE8, 6 B, thunks/ccr_helpers.c)
/// $082DE8: andi.b #$ee, ccr
pub fn i_082de8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82DEC;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $082DEC: rts
pub fn i_082dec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82DEE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClrFlag74_082dee  ($082DEE, 4 B, enemies/squad/para_squad_helpers_08283c.s)
/// $082DEE: clr.b $74(a6)
pub fn i_082dee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82DF2;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
// ===== SetXN_082df2  ($082DF2, 6 B, thunks/ccr_helpers.c)
/// $082DF2: ori.b #$11, ccr
pub fn i_082df2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82DF6;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $082DF6: rts
pub fn i_082df6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82DF8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetFlag74_082df8  ($082DF8, 6 B, enemies/squad/para_squad_helpers_08283c.s)
/// $082DF8: move.b #$1, $74(a6)
pub fn i_082df8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82DFE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== SetXN_082dfe  ($082DFE, 6 B, thunks/ccr_helpers.c)
/// $082DFE: ori.b #$11, ccr
pub fn i_082dfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E02;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $082E02: rts
pub fn i_082e02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E04;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_00082E04  ($082E04, 106 B, enemies/squad/para_squad_helpers_08283c.s)
/// $082E04: btst.b #$5, $72(a6)
pub fn i_082e04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E0A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $082E0A: bne.w $82e6e
pub fn i_082e0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E0E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x82E6E; return 10; }
    8
}
/// $082E0E: cmpi.w #$20, $22(a6)
pub fn i_082e0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E14;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x20, Size::Word, false);
    16
}
/// $082E14: blt.w $82e2a
pub fn i_082e14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E18;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x82E2A; return 10; }
    8
}
/// $082E18: cmpi.w #$120, $22(a6)
pub fn i_082e18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E1E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x120, Size::Word, false);
    16
}
/// $082E1E: bgt.w $82e2a
pub fn i_082e1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E22;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x82E2A; return 10; }
    8
}
/// $082E22: tst.b $7b(a6)
pub fn i_082e22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E26;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $082E26: bne.w $82e6e
pub fn i_082e26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E2A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x82E6E; return 10; }
    8
}
/// $082E2A: tst.b $74(a6)
pub fn i_082e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E2E;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $082E2E: beq.w $82e50
pub fn i_082e2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E32;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x82E50; return 10; }
    8
}
/// $082E32: cmpi.w #$30, $22(a6)
pub fn i_082e32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E38;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x30, Size::Word, false);
    16
}
/// $082E38: blt.w $82e6e
pub fn i_082e38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E3C;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x82E6E; return 10; }
    8
}
/// $082E3C: movea.l $94(a6), a0
pub fn i_082e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E40;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $082E40: move.w $22(a0), d0
pub fn i_082e40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E44;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $082E44: cmp.w $22(a6), d0
pub fn i_082e44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E48;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $082E48: bgt.w $82e6e
pub fn i_082e48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E4C;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x82E6E; return 10; }
    8
}
/// $082E4C: bra.w $82e74
pub fn i_082e4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E50;
    cpu.ir = 0x6000;
    cpu.pc = 0x82E74;
    10
}
/// $082E50: cmpi.w #$120, $22(a6)
pub fn i_082e50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E56;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x120, Size::Word, false);
    16
}
/// $082E56: bgt.w $82e6e
pub fn i_082e56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E5A;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x82E6E; return 10; }
    8
}
/// $082E5A: movea.l $94(a6), a0
pub fn i_082e5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E5E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $082E5E: move.w $22(a0), d0
pub fn i_082e5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E62;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $082E62: cmp.w $22(a6), d0
pub fn i_082e62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E66;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $082E66: blt.w $82e6e
pub fn i_082e66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E6A;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x82E6E; return 10; }
    8
}
/// $082E6A: bra.w $82e74
pub fn i_082e6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E6E;
    cpu.ir = 0x6000;
    cpu.pc = 0x82E74;
    10
}
// ===== SetXN_082e6e  ($082E6E, 6 B, thunks/ccr_helpers.c)
/// $082E6E: ori.b #$11, ccr
pub fn i_082e6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E72;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $082E72: rts
pub fn i_082e72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E74;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_082e74  ($082E74, 6 B, thunks/ccr_helpers.c)
/// $082E74: andi.b #$ee, ccr
pub fn i_082e74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E78;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $082E78: rts
pub fn i_082e78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E7A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_00082E7A  ($082E7A, 90 B, enemies/squad/para_squad_helpers_08283c.s)
/// $082E7A: move.b $72(a6), d0
pub fn i_082e7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E7E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $082E7E: andi.b #$7, d0
pub fn i_082e7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E82;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x7) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $082E82: bne.w $82eda
pub fn i_082e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E86;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x82EDA; return 10; }
    8
}
/// $082E86: btst.b #$4, $72(a6)
pub fn i_082e86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E8C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $082E8C: beq.w $82ea2
pub fn i_082e8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E90;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x82EA2; return 10; }
    8
}
/// $082E90: tst.b $7b(a6)
pub fn i_082e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E94;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $082E94: beq.w $82ea2
pub fn i_082e94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E98;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x82EA2; return 10; }
    8
}
/// $082E98: bset.b #$2, $72(a6)
pub fn i_082e98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E9E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $082E9E: bra.w $82ed4
pub fn i_082e9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82EA2;
    cpu.ir = 0x6000;
    cpu.pc = 0x82ED4;
    10
}
/// $082EA2: btst.b #$3, $72(a6)
pub fn i_082ea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82EA8;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $082EA8: beq.w $82eda
pub fn i_082ea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82EAC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x82EDA; return 10; }
    8
}
/// $082EAC: jsr $5e3fc.l
pub fn i_082eac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82EB2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82EB2);
    cpu.pc = 0x5E3FC;
    16
}
/// $082EB2: bcs.w $82ece
pub fn i_082eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82EB6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x82ECE; return 10; }
    8
}
/// $082EB6: jsr $5e9b6.l
pub fn i_082eb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82EBC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82EBC);
    cpu.pc = 0x5E9B6;
    16
}
/// $082EBC: andi.w #$7, d0
pub fn i_082ebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82EC0;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $082EC0: bne.w $82ece
pub fn i_082ec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82EC4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x82ECE; return 10; }
    8
}
/// $082EC4: bset.b #$1, $72(a6)
pub fn i_082ec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82ECA;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $082ECA: bra.w $82ed4
pub fn i_082eca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82ECE;
    cpu.ir = 0x6000;
    cpu.pc = 0x82ED4;
    10
}
/// $082ECE: bset.b #$0, $72(a6)
pub fn i_082ece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82ED4;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== SetXN_082ed4  ($082ED4, 6 B, thunks/ccr_helpers.c)
/// $082ED4: ori.b #$11, ccr
pub fn i_082ed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82ED8;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $082ED8: rts
pub fn i_082ed8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82EDA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_082eda  ($082EDA, 6 B, thunks/ccr_helpers.c)
/// $082EDA: andi.b #$ee, ccr
pub fn i_082eda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82EDE;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $082EDE: rts
pub fn i_082ede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82EE0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_00082EE0  ($082EE0, 12 B, enemies/squad/para_squad_helpers_08283c.s)
/// $082EE0: movea.l $c(a6), a0
pub fn i_082ee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82EE4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $082EE4: tst.b $20(a0)
pub fn i_082ee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82EE8;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $082EE8: bne.w $82ef2
pub fn i_082ee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82EEC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x82EF2; return 10; }
    8
}
// ===== ClearXN_082eec  ($082EEC, 6 B, thunks/ccr_helpers.c)
/// $082EEC: andi.b #$ee, ccr
pub fn i_082eec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82EF0;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $082EF0: rts
pub fn i_082ef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82EF2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_082ef2  ($082EF2, 6 B, thunks/ccr_helpers.c)
/// $082EF2: ori.b #$11, ccr
pub fn i_082ef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82EF6;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $082EF6: rts
pub fn i_082ef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82EF8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_00082EF8  ($082EF8, 44 B, enemies/squad/para_squad_helpers_08283c.s)
/// $082EF8: movea.l $c(a6), a0
pub fn i_082ef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82EFC;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $082EFC: btst.b #$0, $72(a0)
pub fn i_082efc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F02;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $082F02: bne.w $82f18
pub fn i_082f02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F06;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x82F18; return 10; }
    8
}
/// $082F06: movea.l $c(a6), a0
pub fn i_082f06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F0A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $082F0A: btst.b #$1, $72(a0)
pub fn i_082f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F10;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $082F10: bne.w $82f1e
pub fn i_082f10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F14;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x82F1E; return 10; }
    8
}
/// $082F14: clr.w d0
pub fn i_082f14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F16;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $082F16: rts
pub fn i_082f16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F18;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $082F18: move.w #$1, d0
pub fn i_082f18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F1C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $082F1C: rts
pub fn i_082f1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F1E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $082F1E: move.w #$2, d0
pub fn i_082f1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F22;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $082F22: rts
pub fn i_082f22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F24;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_00082F24  ($082F24, 84 B, enemies/squad/para_squad_helpers_08283c.s)
/// $082F24: movea.l $94(a6), a0
pub fn i_082f24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F28;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $082F28: addi.w #$38, $24(a6)
pub fn i_082f28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F2E;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x38, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $082F2E: jsr $5e070.l
pub fn i_082f2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F34;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82F34);
    cpu.pc = 0x5E070;
    16
}
/// $082F34: subi.w #$38, $24(a6)
pub fn i_082f34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F3A;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x38, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $082F3A: subi.w #$24, d0
pub fn i_082f3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F3E;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x24, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $082F3E: asr.w #$3, d0
pub fn i_082f3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F40;
    cpu.ir = 0xE640;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $082F40: cmp.w $5c(a6), d0
pub fn i_082f40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F44;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $082F44: beq.w $82f7e
pub fn i_082f44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F48;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x82F7E; return 10; }
    8
}
/// $082F48: move.w #$1, d1
pub fn i_082f48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F4C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $082F4C: cmp.w $5c(a6), d0
pub fn i_082f4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F50;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $082F50: bgt.w $82f56
pub fn i_082f50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F54;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x82F56; return 10; }
    8
}
/// $082F54: neg.w d1
pub fn i_082f54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F56;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $082F56: move.w d1, d0
pub fn i_082f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F58;
    cpu.ir = 0x3001;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $082F58: add.w $5c(a6), d1
pub fn i_082f58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F5C;
    cpu.ir = 0xD26E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $082F5C: cmpi.w #$0, d1
pub fn i_082f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F60;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $082F60: bge.w $82f66
pub fn i_082f60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F64;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x82F66; return 10; }
    8
}
/// $082F64: clr.w d0
pub fn i_082f64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F66;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $082F66: cmpi.w #$6, d1
pub fn i_082f66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F6A;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x6, Size::Word, false);
    8
}
/// $082F6A: ble.w $82f70
pub fn i_082f6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F6E;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x82F70; return 10; }
    8
}
/// $082F6E: clr.w d0
pub fn i_082f6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F70;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $082F70: movea.l $c(a6), a0
pub fn i_082f70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F74;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $082F74: clr.b $7b(a0)
pub fn i_082f74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F78;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x7B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
// ===== SetXN_082f78  ($082F78, 6 B, thunks/ccr_helpers.c)
/// $082F78: ori.b #$11, ccr
pub fn i_082f78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F7C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $082F7C: rts
pub fn i_082f7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F7E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetFlag7b_082f7e  ($082F7E, 12 B, enemies/squad/para_squad_helpers_08283c.s)
/// $082F7E: clr.w d0
pub fn i_082f7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F80;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $082F80: movea.l $c(a6), a0
pub fn i_082f80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F84;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $082F84: move.b #$1, $7b(a0)
pub fn i_082f84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F8A;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x7B);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== ClearXN_082f8a  ($082F8A, 6 B, thunks/ccr_helpers.c)
/// $082F8A: andi.b #$ee, ccr
pub fn i_082f8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F8E;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $082F8E: rts
pub fn i_082f8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F90;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_00082F90  ($082F90, 58 B, enemies/squad/para_squad_helpers_08283c.s)
/// $082F90: movea.l a6, a1
pub fn i_082f90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F92;
    cpu.ir = 0x224E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $082F92: cmpi.b #$ff, $7c(a6)
pub fn i_082f92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F98;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $082F98: beq.w $82fa0
pub fn i_082f98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F9C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x82FA0; return 10; }
    8
}
/// $082F9C: movea.l $c(a6), a1
pub fn i_082f9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82FA0;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $082FA0: movea.l $94(a1), a0
pub fn i_082fa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82FA4;
    cpu.ir = 0x2069;
    let ad2 = cpu.a[1].wrapping_add(0x94);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $082FA4: jsr $5e338.l
pub fn i_082fa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82FAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82FAA);
    cpu.pc = 0x5E338;
    16
}
/// $082FAA: bcc.w $82fc8
pub fn i_082faa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82FAE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x82FC8; return 10; }
    8
}
/// $082FAE: jsr $5e0d4.l
pub fn i_082fae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82FB4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82FB4);
    cpu.pc = 0x5E0D4;
    16
}
/// $082FB4: movea.l a6, a1
pub fn i_082fb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82FB6;
    cpu.ir = 0x224E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $082FB6: cmpi.b #$ff, $7c(a6)
pub fn i_082fb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82FBC;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $082FBC: beq.w $82fc4
pub fn i_082fbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82FC0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x82FC4; return 10; }
    8
}
/// $082FC0: movea.l $c(a6), a1
pub fn i_082fc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82FC4;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $082FC4: move.l a0, $94(a1)
pub fn i_082fc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82FC8;
    cpu.ir = 0x2348;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[1].wrapping_add(0x94);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $082FC8: rts
pub fn i_082fc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82FCA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_00082FCA  ($082FCA, 154 B, enemies/squad/para_squad_helpers_08283c.s)
/// $082FCA: lea.l $77e10.l, a1
pub fn i_082fca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82FD0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77E10;
    20
}
/// $082FD0: jsr $4ae.l
pub fn i_082fd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82FD6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82FD6);
    cpu.pc = 0x4AE;
    16
}
/// $082FD6: jsr $5dd02.l
pub fn i_082fd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82FDC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82FDC);
    cpu.pc = 0x5DD02;
    16
}
/// $082FDC: move.w $5c(a6), d0
pub fn i_082fdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82FE0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $082FE0: lsl.w #$2, d0
pub fn i_082fe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82FE2;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $082FE2: lea.l $2e5852.l, a1
pub fn i_082fe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82FE8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E5852;
    20
}
/// $082FE8: move.w (a1, d0.w), d1
pub fn i_082fe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82FEC;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $082FEC: move.w $2(a1, d0.w), d2
pub fn i_082fec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82FF0;
    cpu.ir = 0x3431;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $082FF0: add.w d2, $24(a0)
pub fn i_082ff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82FF4;
    cpu.ir = 0xD568;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $082FF4: btst.b #$0, $3a(a6)
pub fn i_082ff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82FFA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $082FFA: beq.w $83000
pub fn i_082ffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82FFE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x83000; return 10; }
    8
}
/// $082FFE: neg.w d1
pub fn i_082ffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83000;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $083000: add.w d1, $22(a0)
pub fn i_083000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83004;
    cpu.ir = 0xD368;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $083004: lea.l $82720(pc), a1
pub fn i_083004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83008;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82720;
    16
}
/// $083008: jsr $4ae.l
pub fn i_083008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8300E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8300E);
    cpu.pc = 0x4AE;
    16
}
/// $08300E: jsr $5dd02.l
pub fn i_08300e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83014;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83014);
    cpu.pc = 0x5DD02;
    16
}
/// $083014: move.w $5c(a6), d0
pub fn i_083014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83018;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $083018: move.w d0, d3
pub fn i_083018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8301A;
    cpu.ir = 0x3600;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08301A: lsl.w #$2, d0
pub fn i_08301a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8301C;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $08301C: lea.l $2e5852.l, a1
pub fn i_08301c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83022;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E5852;
    20
}
/// $083022: move.w (a1, d0.w), d1
pub fn i_083022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83026;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $083026: move.w $2(a1, d0.w), d2
pub fn i_083026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8302A;
    cpu.ir = 0x3431;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $08302A: add.w d2, $24(a0)
pub fn i_08302a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8302E;
    cpu.ir = 0xD568;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08302E: lea.l $2e586a.l, a2
pub fn i_08302e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83034;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2E586A;
    20
}
/// $083034: move.w d3, d4
pub fn i_083034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83036;
    cpu.ir = 0x3803;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $083036: add.w d4, d3
pub fn i_083036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83038;
    cpu.ir = 0xD644;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $083038: add.w d4, d3
pub fn i_083038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8303A;
    cpu.ir = 0xD644;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $08303A: add.w d3, d3
pub fn i_08303a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8303C;
    cpu.ir = 0xD643;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $08303C: move.w $2(a2, d3.w), $2e(a0)
pub fn i_08303c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83042;
    cpu.ir = 0x3172;
    let ad2 = cpu.a[2].wrapping_add(0x2).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x2E);
    bus.write16(ad4, v3 as u16);
    22
}
/// $083042: move.w $4(a2, d3.w), $2a(a0)
pub fn i_083042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83048;
    cpu.ir = 0x3172;
    let ad2 = cpu.a[2].wrapping_add(0x4).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x2A);
    bus.write16(ad4, v3 as u16);
    22
}
/// $083048: move.w (a2, d3.w), d4
pub fn i_083048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8304C;
    cpu.ir = 0x3832;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $08304C: btst.b #$0, $3a(a6)
pub fn i_08304c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83052;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $083052: beq.w $8305a
pub fn i_083052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83056;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8305A; return 10; }
    8
}
/// $083056: neg.w d1
pub fn i_083056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83058;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $083058: neg.w d4
pub fn i_083058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8305A;
    cpu.ir = 0x4444;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08305A: add.w d1, $22(a0)
pub fn i_08305a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8305E;
    cpu.ir = 0xD368;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08305E: move.w d4, $28(a0)
pub fn i_08305e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83062;
    cpu.ir = 0x3144;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083062: rts
pub fn i_083062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83064;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_00083064  ($083064, 190 B, enemies/squad/para_squad_helpers_08283c.s)
/// $083064: lea.l $8292c(pc), a1
pub fn i_083064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83068;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8292C;
    16
}
/// $083068: jsr $4ae.l
pub fn i_083068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8306E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8306E);
    cpu.pc = 0x4AE;
    16
}
/// $08306E: jsr $5dd02.l
pub fn i_08306e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83074;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83074);
    cpu.pc = 0x5DD02;
    16
}
/// $083074: move.w $5e(a6), d0
pub fn i_083074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83078;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x5E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $083078: move.w d0, d3
pub fn i_083078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8307A;
    cpu.ir = 0x3600;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08307A: lea.l $2e58ae.l, a1
pub fn i_08307a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83080;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E58AE;
    20
}
/// $083080: lsl.w #$2, d0
pub fn i_083080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83082;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $083082: move.w (a1, d0.w), d1
pub fn i_083082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83086;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $083086: move.w $2(a1, d0.w), d2
pub fn i_083086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8308A;
    cpu.ir = 0x3431;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $08308A: add.w d2, $24(a0)
pub fn i_08308a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8308E;
    cpu.ir = 0xD568;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08308E: lea.l $2e58c6.l, a2
pub fn i_08308e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83094;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2E58C6;
    20
}
/// $083094: move.w d3, d4
pub fn i_083094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83096;
    cpu.ir = 0x3803;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $083096: add.w d4, d3
pub fn i_083096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83098;
    cpu.ir = 0xD644;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $083098: add.w d4, d3
pub fn i_083098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8309A;
    cpu.ir = 0xD644;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $08309A: add.w d3, d3
pub fn i_08309a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8309C;
    cpu.ir = 0xD643;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $08309C: move.w $2(a2, d3.w), $2e(a0)
pub fn i_08309c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x830A2;
    cpu.ir = 0x3172;
    let ad2 = cpu.a[2].wrapping_add(0x2).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x2E);
    bus.write16(ad4, v3 as u16);
    22
}
/// $0830A2: move.w $4(a2, d3.w), $2a(a0)
pub fn i_0830a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x830A8;
    cpu.ir = 0x3172;
    let ad2 = cpu.a[2].wrapping_add(0x4).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x2A);
    bus.write16(ad4, v3 as u16);
    22
}
/// $0830A8: move.w (a2, d3.w), d3
pub fn i_0830a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x830AC;
    cpu.ir = 0x3632;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $0830AC: btst.b #$0, $3a(a6)
pub fn i_0830ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x830B2;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0830B2: beq.w $830ba
pub fn i_0830b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x830B6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x830BA; return 10; }
    8
}
/// $0830B6: neg.w d1
pub fn i_0830b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x830B8;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0830B8: neg.w d3
pub fn i_0830b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x830BA;
    cpu.ir = 0x4443;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0830BA: add.w d1, $22(a0)
pub fn i_0830ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x830BE;
    cpu.ir = 0xD368;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0830BE: add.w d3, $28(a0)
pub fn i_0830be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x830C2;
    cpu.ir = 0xD768;
    let ad1 = cpu.a[0].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[3] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0830C2: move.w #$10ff, d0
pub fn i_0830c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x830C6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0830C6: jsr $2352.l
pub fn i_0830c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x830CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x830CC);
    cpu.pc = 0x2352;
    16
}
/// $0830CC: lea.l $82a66(pc), a1
pub fn i_0830cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x830D0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82A66;
    16
}
/// $0830D0: jsr $4ae.l
pub fn i_0830d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x830D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x830D6);
    cpu.pc = 0x4AE;
    16
}
/// $0830D6: jsr $5dd02.l
pub fn i_0830d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x830DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x830DC);
    cpu.pc = 0x5DD02;
    16
}
/// $0830DC: move.w $5e(a6), d0
pub fn i_0830dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x830E0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x5E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0830E0: move.w d0, $5c(a0)
pub fn i_0830e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x830E4;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x5C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0830E4: move.w d0, d1
pub fn i_0830e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x830E6;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0830E6: lsl.w #$3, d0
pub fn i_0830e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x830E8;
    cpu.ir = 0xE748;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0830E8: addi.w #$20, d0
pub fn i_0830e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x830EC;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0830EC: cmpi.w #$80, d0
pub fn i_0830ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x830F0;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x80, Size::Word, false);
    8
}
/// $0830F0: bcs.w $830f8
pub fn i_0830f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x830F4;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x830F8; return 10; }
    8
}
/// $0830F4: move.w #$80, d0
pub fn i_0830f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x830F8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0830F8: move.w d0, $34(a0)
pub fn i_0830f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x830FC;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0830FC: lsl.w #$2, d1
pub fn i_0830fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x830FE;
    cpu.ir = 0xE549;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $0830FE: lea.l $2e589a.l, a1
pub fn i_0830fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83104;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E589A;
    20
}
/// $083104: move.w $2(a1, d1.w), d0
pub fn i_083104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83108;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $083108: add.w d0, $24(a0)
pub fn i_083108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8310C;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08310C: move.w (a1, d1.w), d0
pub fn i_08310c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83110;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $083110: btst.b #$0, $3a(a6)
pub fn i_083110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83116;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $083116: beq.w $8311c
pub fn i_083116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8311A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8311C; return 10; }
    8
}
/// $08311A: neg.w d0
pub fn i_08311a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8311C;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08311C: add.w d0, $22(a0)
pub fn i_08311c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83120;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $083120: rts
pub fn i_083120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83122;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_00083122  ($083122, 54 B, enemies/squad/para_squad_helpers_08283c.s)
/// $083122: move.b #$a, $7a(a6)
pub fn i_083122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83128;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083128: subq.b #$1, $7a(a6)
pub fn i_083128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8312C;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x7A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $08312C: lea.l $82480(pc), a1
pub fn i_08312c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83130;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82480;
    16
}
/// $083130: jsr $4ae.l
pub fn i_083130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83136;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83136);
    cpu.pc = 0x4AE;
    16
}
/// $083136: jsr $5dd02.l
pub fn i_083136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8313C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8313C);
    cpu.pc = 0x5DD02;
    16
}
/// $08313C: move.b $7a(a6), d0
pub fn i_08313c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83140;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $083140: andi.w #$f, d0
pub fn i_083140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83144;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $083144: move.w d0, $5e(a0)
pub fn i_083144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83148;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x5E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083148: move.w $5c(a6), $5c(a0)
pub fn i_083148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8314E;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x5C);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08314E: cmpi.b #$0, $7a(a6)
pub fn i_08314e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83154;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $083154: bgt.b $83128
pub fn i_083154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83156;
    cpu.ir = 0x6ED2;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x83128; return 10; }
    8
}
/// $083156: rts
pub fn i_083156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83158;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_00083158  ($083158, 96 B, enemies/squad/para_squad_helpers_08283c.s)
/// $083158: cmpi.b #$3, $7a(a6)
pub fn i_083158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8315E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $08315E: bne.w $83164
pub fn i_08315e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83162;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x83164; return 10; }
    8
}
/// $083162: rts
pub fn i_083162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83164;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $083164: lea.l $82562(pc), a1
pub fn i_083164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83168;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82562;
    16
}
/// $083168: jsr $4ae.l
pub fn i_083168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8316E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8316E);
    cpu.pc = 0x4AE;
    16
}
/// $08316E: jsr $5dd02.l
pub fn i_08316e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83174;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83174);
    cpu.pc = 0x5DD02;
    16
}
/// $083174: move.w $5c(a6), $5c(a0)
pub fn i_083174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8317A;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x5C);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08317A: clr.w $5e(a6)
pub fn i_08317a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8317E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x5E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08317E: lea.l $82562(pc), a1
pub fn i_08317e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83182;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82562;
    16
}
/// $083182: jsr $4ae.l
pub fn i_083182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83188;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83188);
    cpu.pc = 0x4AE;
    16
}
/// $083188: jsr $5dd02.l
pub fn i_083188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8318E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8318E);
    cpu.pc = 0x5DD02;
    16
}
/// $08318E: move.w $5c(a6), $5c(a0)
pub fn i_08318e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83194;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x5C);
    bus.write16(ad4, v3 as u16);
    20
}
/// $083194: move.w #$1, $5e(a6)
pub fn i_083194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8319A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08319A: lea.l $82562(pc), a1
pub fn i_08319a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8319E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82562;
    16
}
/// $08319E: jsr $4ae.l
pub fn i_08319e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x831A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x831A4);
    cpu.pc = 0x4AE;
    16
}
/// $0831A4: jsr $5dd02.l
pub fn i_0831a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x831AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x831AA);
    cpu.pc = 0x5DD02;
    16
}
/// $0831AA: move.w $5c(a6), $5c(a0)
pub fn i_0831aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x831B0;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x5C);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0831B0: move.w #$2, $5e(a6)
pub fn i_0831b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x831B6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0831B6: rts
pub fn i_0831b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x831B8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_000831B8  ($0831B8, 10 B, enemies/squad/para_squad_helpers_08283c.s)
/// $0831B8: move.b #$1, $10a2d1.l
pub fn i_0831b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x831C0;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2D1, v1 as u8);
    16
}
/// $0831C0: rts
pub fn i_0831c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x831C2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_000831C2  ($0831C2, 24 B, enemies/squad/para_squad_helpers_08283c.s)
/// $0831C2: lea.l $8267c(pc), a1
pub fn i_0831c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x831C6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8267C;
    16
}
/// $0831C6: jsr $4ae.l
pub fn i_0831c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x831CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x831CC);
    cpu.pc = 0x4AE;
    16
}
/// $0831CC: jsr $5dd02.l
pub fn i_0831cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x831D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x831D2);
    cpu.pc = 0x5DD02;
    16
}
/// $0831D2: move.w $5c(a6), $5c(a0)
pub fn i_0831d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x831D8;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x5C);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0831D8: rts
pub fn i_0831d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x831DA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_000831DA  ($0831DA, 66 B, enemies/squad/para_squad_helpers_08283c.s)
/// $0831DA: lea.l $2e5a22.l, a0
pub fn i_0831da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x831E0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E5A22;
    20
}
/// $0831E0: move.b $7c(a6), d0
pub fn i_0831e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x831E4;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0831E4: andi.w #$7, d0
pub fn i_0831e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x831E8;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0831E8: lsl.w #$2, d0
pub fn i_0831e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x831EA;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $0831EA: movea.l (a0, d0.w), a1
pub fn i_0831ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x831EE;
    cpu.ir = 0x2270;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    18
}
/// $0831EE: jsr $77c7e.l
pub fn i_0831ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x831F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x831F4);
    cpu.pc = 0x77C7E;
    16
}
/// $0831F4: move.w #$c000, $38(a0)
pub fn i_0831f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x831FA;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0831FA: lea.l $2e597a.l, a0
pub fn i_0831fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83200;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E597A;
    20
}
/// $083200: move.b $7c(a6), d0
pub fn i_083200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83204;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $083204: andi.w #$7, d0
pub fn i_083204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83208;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $083208: lsl.w #$2, d0
pub fn i_083208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8320A;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $08320A: movea.l (a0, d0.w), a1
pub fn i_08320a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8320E;
    cpu.ir = 0x2270;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    18
}
/// $08320E: jsr $77c7e.l
pub fn i_08320e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83214;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83214);
    cpu.pc = 0x77C7E;
    16
}
/// $083214: move.w #$c000, $38(a0)
pub fn i_083214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8321A;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08321A: rts
pub fn i_08321a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8321C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_0008321C  ($08321C, 56 B, enemies/squad/para_squad_helpers_08283c.s)
/// $08321C: lea.l $77fd6.l, a1
pub fn i_08321c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83222;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $083222: jsr $4ae.l
pub fn i_083222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83228;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83228);
    cpu.pc = 0x4AE;
    16
}
/// $083228: jsr $5dd02.l
pub fn i_083228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8322E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8322E);
    cpu.pc = 0x5DD02;
    16
}
/// $08322E: jsr $5e9b6.l
pub fn i_08322e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83234;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83234);
    cpu.pc = 0x5E9B6;
    16
}
/// $083234: andi.w #$1f, d0
pub fn i_083234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83238;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $083238: subi.w #$10, d0
pub fn i_083238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8323C;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08323C: add.w d0, $22(a0)
pub fn i_08323c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83240;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $083240: jsr $5e9b6.l
pub fn i_083240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83246;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83246);
    cpu.pc = 0x5E9B6;
    16
}
/// $083246: andi.w #$3f, d0
pub fn i_083246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8324A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08324A: subi.w #$10, d0
pub fn i_08324a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8324E;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08324E: add.w d0, $24(a0)
pub fn i_08324e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83252;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $083252: rts
pub fn i_083252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83254;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== LeaList_083254  ($083254, 6 B, enemies/squad/para_squad_helpers_08283c.s)
/// $083254: lea.l $2e5ab8.l, a1
pub fn i_083254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8325A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E5AB8;
    20
}
// ===== JsrAbsThunk_08325a  ($08325A, 8 B, thunks/jsr_abs_thunks.c)
/// $08325A: jsr $77c7e.l
pub fn i_08325a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83260;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83260);
    cpu.pc = 0x77C7E;
    16
}
/// $083260: rts
pub fn i_083260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83262;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== LeaList_083262  ($083262, 6 B, enemies/bosses/miniboss_module_083262.s)
/// $083262: lea.l $2e5aca.l, a1
pub fn i_083262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83268;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E5ACA;
    20
}
// ===== JsrAbsThunk_083268  ($083268, 8 B, thunks/jsr_abs_thunks.c)
/// $083268: jsr $77c7e.l
pub fn i_083268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8326E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8326E);
    cpu.pc = 0x77C7E;
    16
}
/// $08326E: rts
pub fn i_08326e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83270;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_00083270  ($083270, 28 B, enemies/bosses/miniboss_module_083262.s)
/// $083270: move.b #$4, $10a2d0.l
pub fn i_083270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83278;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2D0, v1 as u8);
    16
}
/// $083278: move.b #$0, $10a2d1.l
pub fn i_083278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83280;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2D1, v1 as u8);
    16
}
/// $083280: movea.l $c(a6), a0
pub fn i_083280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83284;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $083284: move.b #$77, $20(a0)
pub fn i_083284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8328A;
    cpu.ir = 0x117C;
    let v1: u32 = 0x77;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08328A: rts
pub fn i_08328a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8328C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_0008328C  ($08328C, 34 B, enemies/bosses/miniboss_module_083262.s)
/// $08328C: lea.l $2e5adc.l, a1
pub fn i_08328c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83292;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E5ADC;
    20
}
/// $083292: jsr $77c7e.l
pub fn i_083292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83298;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83298);
    cpu.pc = 0x77C7E;
    16
}
/// $083298: lea.l $2e5a10.l, a1
pub fn i_083298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8329E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E5A10;
    20
}
/// $08329E: jsr $77c7e.l
pub fn i_08329e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x832A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x832A4);
    cpu.pc = 0x77C7E;
    16
}
/// $0832A4: move.w #$102f, d0
pub fn i_0832a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x832A8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0832A8: jsr $2352.l
pub fn i_0832a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x832AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x832AE);
    cpu.pc = 0x2352;
    16
}
// ===== JsrAbsThunk_0832ae  ($0832AE, 8 B, thunks/jsr_abs_thunks.c)
/// $0832AE: jsr $434ce.l
pub fn i_0832ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x832B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x832B4);
    cpu.pc = 0x434CE;
    16
}
/// $0832B4: rts
pub fn i_0832b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x832B6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_000832B6  ($0832B6, 18 B, enemies/bosses/miniboss_module_083262.s)
/// $0832B6: lea.l $2e545e.l, a0
pub fn i_0832b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x832BC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E545E;
    20
}
/// $0832BC: move.w $5c(a6), d0
pub fn i_0832bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x832C0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0832C0: lsl.w #$2, d0
pub fn i_0832c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x832C2;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $0832C2: move.l (a0, d0.w), $4c(a6)
pub fn i_0832c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x832C8;
    cpu.ir = 0x2D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    30
}
// ===== JsrAbsThunk_0832c8  ($0832C8, 8 B, thunks/jsr_abs_thunks.c)
/// $0832C8: jsr $283ca.l
pub fn i_0832c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x832CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x832CE);
    cpu.pc = 0x283CA;
    16
}
/// $0832CE: rts
pub fn i_0832ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x832D0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_000832D0  ($0832D0, 16 B, enemies/bosses/miniboss_module_083262.s)
/// $0832D0: movea.l $8(a6), a1
pub fn i_0832d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x832D4;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0832D4: move.b $10(a6), d0
pub fn i_0832d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x832D8;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0832D8: cmp.b $10(a1), d0
pub fn i_0832d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x832DC;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $0832DC: bcs.w $832e6
pub fn i_0832dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x832E0;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x832E6; return 10; }
    8
}
// ===== ClearXN_0832e0  ($0832E0, 6 B, thunks/ccr_helpers.c)
/// $0832E0: andi.b #$ee, ccr
pub fn i_0832e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x832E4;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0832E4: rts
pub fn i_0832e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x832E6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_0832e6  ($0832E6, 6 B, thunks/ccr_helpers.c)
/// $0832E6: ori.b #$11, ccr
pub fn i_0832e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x832EA;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0832EA: rts
pub fn i_0832ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x832EC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_0832ec  ($0832EC, 128 B, enemies/bosses/miniboss_module_083262.s)
/// $0832EC: move.w #$a4, d1
pub fn i_0832ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x832F0;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0832F0: jsr $236e.l
pub fn i_0832f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x832F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x832F6);
    cpu.pc = 0x236E;
    16
}
/// $0832F6: move.b #$ff, $32(a6)
pub fn i_0832f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x832FC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0832FC: move.b #$ff, $33(a6)
pub fn i_0832fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83302;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083302: move.b #$0, $3a(a6)
pub fn i_083302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83308;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083308: move.w #$0, $38(a6)
pub fn i_083308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8330E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08330E: jsr $267e2.l
pub fn i_08330e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83314;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83314);
    cpu.pc = 0x267E2;
    16
}
/// $083314: move.w #$80, d0
pub fn i_083314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83318;
    cpu.ir = 0x303C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $083318: cmpi.w #$a0, $22(a6)
pub fn i_083318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8331E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xA0, Size::Word, false);
    16
}
/// $08331E: blt.w $8332a
pub fn i_08331e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83322;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x8332A; return 10; }
    8
}
/// $083322: neg.w d0
pub fn i_083322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83324;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $083324: bset.b #$0, $3a(a6)
pub fn i_083324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8332A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08332A: move.w d0, $28(a6)
pub fn i_08332a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8332E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08332E: lea.l $ffff.w, a0
pub fn i_08332e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83332;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $083332: move.l a0, $48(a6)
pub fn i_083332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83336;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $083336: lea.l $2e6cdc.l, a0
pub fn i_083336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8333C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6CDC;
    20
}
/// $08333C: jsr $28cd4.l
pub fn i_08333c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83342;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83342);
    cpu.pc = 0x28CD4;
    16
}
/// $083342: lea.l $83348(pc), a1
pub fn i_083342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83346;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83348;
    16
}
/// $083346: move.l a1, (a6)
pub fn i_083346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83348;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $083348: jsr $27cee.l
pub fn i_083348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8334E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8334E);
    cpu.pc = 0x27CEE;
    16
}
/// $08334E: jsr $28d70.l
pub fn i_08334e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83354;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83354);
    cpu.pc = 0x28D70;
    16
}
/// $083354: movea.l #$ffffffff, a0
pub fn i_083354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8335A;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $08335A: jsr $5dd5c.l
pub fn i_08335a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83360;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83360);
    cpu.pc = 0x5DD5C;
    16
}
/// $083360: bcc.w $8336a
pub fn i_083360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83364;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8336A; return 10; }
    8
}
/// $083364: jmp $518.l
pub fn i_083364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8336A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08336A: rts
pub fn i_08336a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8336C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_08336c  ($08336C, 186 B, enemies/bosses/miniboss_module_083262.s)
/// $08336C: move.w #$a4, d1
pub fn i_08336c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83370;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $083370: jsr $236e.l
pub fn i_083370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83376;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83376);
    cpu.pc = 0x236E;
    16
}
/// $083376: move.b #$ff, $32(a6)
pub fn i_083376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8337C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08337C: move.b #$ff, $33(a6)
pub fn i_08337c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83382;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083382: move.b #$0, $3a(a6)
pub fn i_083382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83388;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083388: move.w #$0, $38(a6)
pub fn i_083388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8338E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08338E: jsr $267e2.l
pub fn i_08338e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83394;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83394);
    cpu.pc = 0x267E2;
    16
}
/// $083394: move.w #$0, $72(a6)
pub fn i_083394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8339A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08339A: cmpi.w #$a0, $22(a6)
pub fn i_08339a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x833A0;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xA0, Size::Word, false);
    16
}
/// $0833A0: blt.w $833aa
pub fn i_0833a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x833A4;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x833AA; return 10; }
    8
}
/// $0833A4: bset.b #$0, $3a(a6)
pub fn i_0833a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x833AA;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0833AA: lea.l $2e6cdc.l, a0
pub fn i_0833aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x833B0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6CDC;
    20
}
/// $0833B0: jsr $28cd4.l
pub fn i_0833b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x833B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x833B6);
    cpu.pc = 0x28CD4;
    16
}
/// $0833B6: lea.l $ffff.w, a0
pub fn i_0833b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x833BA;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $0833BA: move.l a0, $48(a6)
pub fn i_0833ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x833BE;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0833BE: lea.l $833c4(pc), a1
pub fn i_0833be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x833C2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x833C4;
    16
}
/// $0833C2: move.l a1, (a6)
pub fn i_0833c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x833C4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0833C4: jsr $2783a.l
pub fn i_0833c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x833CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x833CA);
    cpu.pc = 0x2783A;
    16
}
/// $0833CA: jsr $28d70.l
pub fn i_0833ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x833D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x833D0);
    cpu.pc = 0x28D70;
    16
}
/// $0833D0: addi.w #$1, $72(a6)
pub fn i_0833d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x833D6;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0833D6: move.w $72(a6), d0
pub fn i_0833d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x833DA;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0833DA: btst.b #$1, d0
pub fn i_0833da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x833DE;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((1) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $0833DE: beq.w $8340e
pub fn i_0833de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x833E2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8340E; return 10; }
    8
}
/// $0833E2: jsr $5e9b6.l
pub fn i_0833e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x833E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x833E8);
    cpu.pc = 0x5E9B6;
    16
}
/// $0833E8: andi.w #$f, d0
pub fn i_0833e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x833EC;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0833EC: cmpi.w #$0, d0
pub fn i_0833ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x833F0;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $0833F0: bne.w $8340e
pub fn i_0833f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x833F4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8340E; return 10; }
    8
}
/// $0833F4: move.w #$50, d0
pub fn i_0833f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x833F8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x50;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0833F8: jsr $5e9e4.l
pub fn i_0833f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x833FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x833FE);
    cpu.pc = 0x5E9E4;
    16
}
/// $0833FE: btst.b #$0, $3a(a6)
pub fn i_0833fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83404;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $083404: beq.w $8340a
pub fn i_083404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83408;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8340A; return 10; }
    8
}
/// $083408: neg.w d0
pub fn i_083408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8340A;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08340A: add.w d0, $22(a6)
pub fn i_08340a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8340E;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08340E: movea.l #$ffffffff, a0
pub fn i_08340e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83414;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $083414: jsr $5dd5c.l
pub fn i_083414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8341A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8341A);
    cpu.pc = 0x5DD5C;
    16
}
/// $08341A: bcc.w $83424
pub fn i_08341a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8341E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83424; return 10; }
    8
}
/// $08341E: jmp $518.l
pub fn i_08341e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83424;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $083424: rts
pub fn i_083424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83426;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_083426  ($083426, 126 B, enemies/bosses/miniboss_module_083262.s)
/// $083426: move.w #$a4, d1
pub fn i_083426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8342A;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08342A: jsr $236e.l
pub fn i_08342a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83430;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83430);
    cpu.pc = 0x236E;
    16
}
/// $083430: move.b #$ff, $32(a6)
pub fn i_083430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83436;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083436: move.b #$ff, $33(a6)
pub fn i_083436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8343C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08343C: move.b #$0, $3a(a6)
pub fn i_08343c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83442;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083442: move.w #$0, $38(a6)
pub fn i_083442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83448;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083448: jsr $267e2.l
pub fn i_083448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8344E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8344E);
    cpu.pc = 0x267E2;
    16
}
/// $08344E: move.w #$80, d0
pub fn i_08344e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83452;
    cpu.ir = 0x303C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $083452: cmpi.w #$a0, $22(a6)
pub fn i_083452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83458;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xA0, Size::Word, false);
    16
}
/// $083458: blt.w $83464
pub fn i_083458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8345C;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x83464; return 10; }
    8
}
/// $08345C: neg.w d0
pub fn i_08345c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8345E;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08345E: bset.b #$0, $3a(a6)
pub fn i_08345e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83464;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $083464: move.w d0, $28(a6)
pub fn i_083464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83468;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083468: lea.l $2e6cdc.l, a0
pub fn i_083468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8346E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6CDC;
    20
}
/// $08346E: jsr $28cd4.l
pub fn i_08346e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83474;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83474);
    cpu.pc = 0x28CD4;
    16
}
/// $083474: lea.l $ffff.w, a0
pub fn i_083474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83478;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $083478: move.l a0, $48(a6)
pub fn i_083478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8347C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08347C: lea.l $83482(pc), a1
pub fn i_08347c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83480;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83482;
    16
}
/// $083480: move.l a1, (a6)
pub fn i_083480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83482;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $083482: jsr $86050(pc)
pub fn i_083482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83486;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x83486);
    cpu.pc = 0x86050;
    16
}
/// $083486: jsr $28d70.l
pub fn i_083486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8348C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8348C);
    cpu.pc = 0x28D70;
    16
}
/// $08348C: movea.l #$ffffffff, a0
pub fn i_08348c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83492;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $083492: jsr $5dd5c.l
pub fn i_083492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83498;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83498);
    cpu.pc = 0x5DD5C;
    16
}
/// $083498: bcc.w $834a2
pub fn i_083498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8349C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x834A2; return 10; }
    8
}
/// $08349C: jmp $518.l
pub fn i_08349c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x834A2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0834A2: rts
pub fn i_0834a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x834A4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_0834a4  ($0834A4, 170 B, enemies/bosses/miniboss_module_083262.s)
/// $0834A4: movea.l $3c(a6), a1
pub fn i_0834a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x834A8;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0834A8: jsr $2942a.l
pub fn i_0834a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x834AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x834AE);
    cpu.pc = 0x2942A;
    16
}
/// $0834AE: move.w #$a7, d1
pub fn i_0834ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x834B2;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0834B2: jsr $236e.l
pub fn i_0834b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x834B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x834B8);
    cpu.pc = 0x236E;
    16
}
/// $0834B8: move.w #$30, $70(a6)
pub fn i_0834b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x834BE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0834BE: move.b #$ff, $32(a6)
pub fn i_0834be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x834C4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0834C4: move.b #$ff, $33(a6)
pub fn i_0834c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x834CA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0834CA: move.b #$0, $3a(a6)
pub fn i_0834ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x834D0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0834D0: move.w #$1e, $66(a6)
pub fn i_0834d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x834D6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0834D6: lea.l $2e6c00.l, a0
pub fn i_0834d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x834DC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6C00;
    20
}
/// $0834DC: jsr $28cd4.l
pub fn i_0834dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x834E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x834E2);
    cpu.pc = 0x28CD4;
    16
}
/// $0834E2: lea.l $834e8(pc), a1
pub fn i_0834e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x834E6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x834E8;
    16
}
/// $0834E6: move.l a1, (a6)
pub fn i_0834e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x834E8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0834E8: clr.b $10e39a.l
pub fn i_0834e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x834EE;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $0834EE: jsr $2783a.l
pub fn i_0834ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x834F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x834F4);
    cpu.pc = 0x2783A;
    16
}
/// $0834F4: jsr $28d70.l
pub fn i_0834f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x834FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x834FA);
    cpu.pc = 0x28D70;
    16
}
/// $0834FA: jsr $2870a.l
pub fn i_0834fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83500;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83500);
    cpu.pc = 0x2870A;
    16
}
/// $083500: bcc.w $83516
pub fn i_083500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83504;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83516; return 10; }
    8
}
/// $083504: lea.l $5e766.l, a0
pub fn i_083504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8350A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $08350A: jsr $5e770.l
pub fn i_08350a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83510;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83510);
    cpu.pc = 0x5E770;
    16
}
/// $083510: bclr.b #$3, $13(a6)
pub fn i_083510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83516;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $083516: jsr $28758.l
pub fn i_083516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8351C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8351C);
    cpu.pc = 0x28758;
    16
}
/// $08351C: bcc.w $8353c
pub fn i_08351c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83520;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8353C; return 10; }
    8
}
/// $083520: lea.l $2eac8c.l, a1
pub fn i_083520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83526;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EAC8C;
    20
}
/// $083526: jsr $43fac.l
pub fn i_083526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8352C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8352C);
    cpu.pc = 0x43FAC;
    16
}
/// $08352C: move.w #$1037, d0
pub fn i_08352c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83530;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1037;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $083530: jsr $2352.l
pub fn i_083530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83536;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83536);
    cpu.pc = 0x2352;
    16
}
/// $083536: lea.l $8354e(pc), a1
pub fn i_083536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8353A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8354E;
    16
}
/// $08353A: move.l a1, (a6)
pub fn i_08353a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8353C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08353C: jsr $4fa70.l
pub fn i_08353c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83542;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83542);
    cpu.pc = 0x4FA70;
    16
}
/// $083542: bcc.w $8354c
pub fn i_083542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83546;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8354C; return 10; }
    8
}
/// $083546: jmp $518.l
pub fn i_083546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8354C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08354C: rts
pub fn i_08354c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8354E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_08354e  ($08354E, 72 B, enemies/bosses/miniboss_module_083262.s)
/// $08354E: move.l #$100, d0
pub fn i_08354e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83554;
    cpu.ir = 0x203C;
    let v1: u32 = 0x100;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $083554: jsr $51a28.l
pub fn i_083554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8355A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8355A);
    cpu.pc = 0x51A28;
    16
}
/// $08355A: lea.l $ffff.w, a0
pub fn i_08355a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8355E;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $08355E: move.l a0, $48(a6)
pub fn i_08355e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83562;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $083562: lea.l $2e6c16.l, a0
pub fn i_083562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83568;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6C16;
    20
}
/// $083568: jsr $28cd4.l
pub fn i_083568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8356E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8356E);
    cpu.pc = 0x28CD4;
    16
}
/// $08356E: lea.l $83574(pc), a1
pub fn i_08356e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83572;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83574;
    16
}
/// $083572: move.l a1, (a6)
pub fn i_083572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83574;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $083574: jsr $2783a.l
pub fn i_083574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8357A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8357A);
    cpu.pc = 0x2783A;
    16
}
/// $08357A: jsr $28d70.l
pub fn i_08357a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83580;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83580);
    cpu.pc = 0x28D70;
    16
}
/// $083580: bcs.w $8358e
pub fn i_083580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83584;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8358E; return 10; }
    8
}
/// $083584: jsr $4fa70.l
pub fn i_083584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8358A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8358A);
    cpu.pc = 0x4FA70;
    16
}
/// $08358A: bcc.w $83594
pub fn i_08358a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8358E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83594; return 10; }
    8
}
/// $08358E: jmp $518.l
pub fn i_08358e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83594;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $083594: rts
pub fn i_083594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83596;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_083596  ($083596, 92 B, enemies/bosses/miniboss_module_083262.s)
/// $083596: movea.l $3c(a6), a1
pub fn i_083596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8359A;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08359A: jsr $2942a.l
pub fn i_08359a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x835A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x835A0);
    cpu.pc = 0x2942A;
    16
}
/// $0835A0: move.w #$ab, d1
pub fn i_0835a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x835A4;
    cpu.ir = 0x323C;
    let v1: u32 = 0xAB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0835A4: jsr $236e.l
pub fn i_0835a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x835AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x835AA);
    cpu.pc = 0x236E;
    16
}
/// $0835AA: move.w #$30, $70(a6)
pub fn i_0835aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x835B0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0835B0: move.b #$ff, $32(a6)
pub fn i_0835b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x835B6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0835B6: move.b #$ff, $33(a6)
pub fn i_0835b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x835BC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0835BC: move.b #$0, $3a(a6)
pub fn i_0835bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x835C2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0835C2: lea.l $2e6d28.l, a0
pub fn i_0835c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x835C8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6D28;
    20
}
/// $0835C8: jsr $28cd4.l
pub fn i_0835c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x835CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x835CE);
    cpu.pc = 0x28CD4;
    16
}
/// $0835CE: lea.l $835d4(pc), a1
pub fn i_0835ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x835D2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x835D4;
    16
}
/// $0835D2: move.l a1, (a6)
pub fn i_0835d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x835D4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0835D4: jsr $2783a.l
pub fn i_0835d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x835DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x835DA);
    cpu.pc = 0x2783A;
    16
}
/// $0835DA: jsr $28d70.l
pub fn i_0835da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x835E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x835E0);
    cpu.pc = 0x28D70;
    16
}
/// $0835E0: jsr $4fa70.l
pub fn i_0835e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x835E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x835E6);
    cpu.pc = 0x4FA70;
    16
}
/// $0835E6: bcc.w $835f0
pub fn i_0835e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x835EA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x835F0; return 10; }
    8
}
/// $0835EA: jmp $518.l
pub fn i_0835ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x835F0;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0835F0: rts
pub fn i_0835f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x835F2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_0835f2  ($0835F2, 32 B, enemies/bosses/miniboss_module_083262.s)
/// $0835F2: movea.l $3c(a6), a1
pub fn i_0835f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x835F6;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0835F6: jsr $2942a.l
pub fn i_0835f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x835FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x835FC);
    cpu.pc = 0x2942A;
    16
}
/// $0835FC: move.b #$0, $21(a6)
pub fn i_0835fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83602;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083602: lea.l $2e6d38.l, a0
pub fn i_083602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83608;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6D38;
    20
}
/// $083608: jsr $28cd4.l
pub fn i_083608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8360E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8360E);
    cpu.pc = 0x28CD4;
    16
}
/// $08360E: bra.w $8364e
pub fn i_08360e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83612;
    cpu.ir = 0x6000;
    cpu.pc = 0x8364E;
    10
}
// ===== TaskHandler_083612  ($083612, 32 B, enemies/bosses/miniboss_module_083262.s)
/// $083612: movea.l $3c(a6), a1
pub fn i_083612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83616;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $083616: jsr $2942a.l
pub fn i_083616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8361C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8361C);
    cpu.pc = 0x2942A;
    16
}
/// $08361C: move.b #$1, $21(a6)
pub fn i_08361c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83622;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083622: lea.l $2e6da4.l, a0
pub fn i_083622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83628;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6DA4;
    20
}
/// $083628: jsr $28cd4.l
pub fn i_083628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8362E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8362E);
    cpu.pc = 0x28CD4;
    16
}
/// $08362E: bra.w $8364e
pub fn i_08362e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83632;
    cpu.ir = 0x6000;
    cpu.pc = 0x8364E;
    10
}
// ===== TaskHandler_083632  ($083632, 226 B, enemies/bosses/miniboss_module_083262.s)
/// $083632: movea.l $3c(a6), a1
pub fn i_083632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83636;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $083636: jsr $2942a.l
pub fn i_083636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8363C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8363C);
    cpu.pc = 0x2942A;
    16
}
/// $08363C: move.b #$2, $21(a6)
pub fn i_08363c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83642;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083642: lea.l $2e6da4.l, a0
pub fn i_083642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83648;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6DA4;
    20
}
/// $083648: jsr $28cd4.l
pub fn i_083648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8364E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8364E);
    cpu.pc = 0x28CD4;
    16
}
/// $08364E: move.w #$aa, d1
pub fn i_08364e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83652;
    cpu.ir = 0x323C;
    let v1: u32 = 0xAA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $083652: jsr $236e.l
pub fn i_083652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83658;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83658);
    cpu.pc = 0x236E;
    16
}
/// $083658: move.b #$0, $3a(a6)
pub fn i_083658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8365E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08365E: move.w $70(a6), d0
pub fn i_08365e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83662;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $083662: or.b d0, $3a(a6)
pub fn i_083662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83666;
    cpu.ir = 0x812E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | cpu.d[0]) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $083666: asr.w #$8, d0
pub fn i_083666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83668;
    cpu.ir = 0xE040;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $083668: move.b d0, $20(a6)
pub fn i_083668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8366C;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08366C: move.w #$30, $70(a6)
pub fn i_08366c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83672;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083672: move.b #$ff, $32(a6)
pub fn i_083672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83678;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083678: move.b #$ff, $33(a6)
pub fn i_083678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8367E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08367E: move.w #$3c, $66(a6)
pub fn i_08367e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83684;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083684: move.w #$8000, $38(a6)
pub fn i_083684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8368A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08368A: jsr $267e2.l
pub fn i_08368a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83690;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83690);
    cpu.pc = 0x267E2;
    16
}
/// $083690: jsr $27cee.l
pub fn i_083690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83696;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83696);
    cpu.pc = 0x27CEE;
    16
}
/// $083696: btst.b #$0, $3a(a6)
pub fn i_083696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8369C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $08369C: beq.w $836a6
pub fn i_08369c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x836A0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x836A6; return 10; }
    8
}
/// $0836A0: addi.w #$20, $22(a6)
pub fn i_0836a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x836A6;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0836A6: cmpi.b #$2, $21(a6)
pub fn i_0836a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x836AC;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $0836AC: bne.w $836c0
pub fn i_0836ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x836B0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x836C0; return 10; }
    8
}
/// $0836B0: lea.l $2e9698.l, a0
pub fn i_0836b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x836B6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E9698;
    20
}
/// $0836B6: move.l a0, $4c(a6)
pub fn i_0836b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x836BA;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0836BA: jsr $283ca.l
pub fn i_0836ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x836C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x836C0);
    cpu.pc = 0x283CA;
    16
}
/// $0836C0: lea.l $836c6(pc), a1
pub fn i_0836c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x836C4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x836C6;
    16
}
/// $0836C4: move.l a1, (a6)
pub fn i_0836c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x836C6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0836C6: jsr $2783a.l
pub fn i_0836c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x836CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x836CC);
    cpu.pc = 0x2783A;
    16
}
/// $0836CC: jsr $28d70.l
pub fn i_0836cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x836D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x836D2);
    cpu.pc = 0x28D70;
    16
}
/// $0836D2: jsr $863be(pc)
pub fn i_0836d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x836D6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x836D6);
    cpu.pc = 0x863BE;
    16
}
/// $0836D6: jsr $2870a.l
pub fn i_0836d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x836DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x836DC);
    cpu.pc = 0x2870A;
    16
}
/// $0836DC: bcc.w $836f2
pub fn i_0836dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x836E0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x836F2; return 10; }
    8
}
/// $0836E0: lea.l $5e766.l, a0
pub fn i_0836e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x836E6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $0836E6: jsr $5e770.l
pub fn i_0836e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x836EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x836EC);
    cpu.pc = 0x5E770;
    16
}
/// $0836EC: bclr.b #$3, $13(a6)
pub fn i_0836ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x836F2;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0836F2: cmpi.w #$28, $66(a6)
pub fn i_0836f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x836F8;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x28, Size::Word, false);
    16
}
/// $0836F8: bgt.w $83702
pub fn i_0836f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x836FC;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x83702; return 10; }
    8
}
/// $0836FC: lea.l $83714(pc), a1
pub fn i_0836fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83700;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83714;
    16
}
/// $083700: move.l a1, (a6)
pub fn i_083700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83702;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $083702: jsr $4fa70.l
pub fn i_083702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83708;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83708);
    cpu.pc = 0x4FA70;
    16
}
/// $083708: bcc.w $83712
pub fn i_083708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8370C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83712; return 10; }
    8
}
/// $08370C: jmp $518.l
pub fn i_08370c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83712;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $083712: rts
pub fn i_083712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83714;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_083714  ($083714, 118 B, enemies/bosses/miniboss_module_083262.s)
/// $083714: lea.l $2e6d4e.l, a0
pub fn i_083714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8371A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6D4E;
    20
}
/// $08371A: jsr $28cd4.l
pub fn i_08371a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83720;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83720);
    cpu.pc = 0x28CD4;
    16
}
/// $083720: cmpi.b #$0, $21(a6)
pub fn i_083720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83726;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $083726: beq.w $83736
pub fn i_083726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8372A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x83736; return 10; }
    8
}
/// $08372A: lea.l $2e6dba.l, a0
pub fn i_08372a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83730;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6DBA;
    20
}
/// $083730: jsr $28cd4.l
pub fn i_083730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83736;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83736);
    cpu.pc = 0x28CD4;
    16
}
/// $083736: lea.l $8373c(pc), a1
pub fn i_083736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8373A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8373C;
    16
}
/// $08373A: move.l a1, (a6)
pub fn i_08373a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8373C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08373C: jsr $2783a.l
pub fn i_08373c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83742;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83742);
    cpu.pc = 0x2783A;
    16
}
/// $083742: jsr $28d70.l
pub fn i_083742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83748;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83748);
    cpu.pc = 0x28D70;
    16
}
/// $083748: jsr $863be(pc)
pub fn i_083748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8374C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8374C);
    cpu.pc = 0x863BE;
    16
}
/// $08374C: jsr $2870a.l
pub fn i_08374c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83752;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83752);
    cpu.pc = 0x2870A;
    16
}
/// $083752: bcc.w $83768
pub fn i_083752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83756;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83768; return 10; }
    8
}
/// $083756: lea.l $5e766.l, a0
pub fn i_083756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8375C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $08375C: jsr $5e770.l
pub fn i_08375c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83762;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83762);
    cpu.pc = 0x5E770;
    16
}
/// $083762: bclr.b #$3, $13(a6)
pub fn i_083762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83768;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $083768: cmpi.w #$14, $66(a6)
pub fn i_083768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8376E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x14, Size::Word, false);
    16
}
/// $08376E: bgt.w $83778
pub fn i_08376e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83772;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x83778; return 10; }
    8
}
/// $083772: lea.l $8378a(pc), a1
pub fn i_083772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83776;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8378A;
    16
}
/// $083776: move.l a1, (a6)
pub fn i_083776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83778;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $083778: jsr $4fa70.l
pub fn i_083778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8377E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8377E);
    cpu.pc = 0x4FA70;
    16
}
/// $08377E: bcc.w $83788
pub fn i_08377e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83782;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83788; return 10; }
    8
}
/// $083782: jmp $518.l
pub fn i_083782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83788;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $083788: rts
pub fn i_083788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8378A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_08378a  ($08378A, 118 B, enemies/bosses/miniboss_module_083262.s)
/// $08378A: lea.l $2e6d6a.l, a0
pub fn i_08378a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83790;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6D6A;
    20
}
/// $083790: jsr $28cd4.l
pub fn i_083790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83796;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83796);
    cpu.pc = 0x28CD4;
    16
}
/// $083796: cmpi.b #$0, $21(a6)
pub fn i_083796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8379C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08379C: beq.w $837ac
pub fn i_08379c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x837A0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x837AC; return 10; }
    8
}
/// $0837A0: lea.l $2e6dd6.l, a0
pub fn i_0837a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x837A6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6DD6;
    20
}
/// $0837A6: jsr $28cd4.l
pub fn i_0837a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x837AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x837AC);
    cpu.pc = 0x28CD4;
    16
}
/// $0837AC: lea.l $837b2(pc), a1
pub fn i_0837ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x837B0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x837B2;
    16
}
/// $0837B0: move.l a1, (a6)
pub fn i_0837b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x837B2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0837B2: jsr $2783a.l
pub fn i_0837b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x837B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x837B8);
    cpu.pc = 0x2783A;
    16
}
/// $0837B8: jsr $28d70.l
pub fn i_0837b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x837BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x837BE);
    cpu.pc = 0x28D70;
    16
}
/// $0837BE: jsr $863be(pc)
pub fn i_0837be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x837C2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x837C2);
    cpu.pc = 0x863BE;
    16
}
/// $0837C2: jsr $2870a.l
pub fn i_0837c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x837C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x837C8);
    cpu.pc = 0x2870A;
    16
}
/// $0837C8: bcc.w $837de
pub fn i_0837c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x837CC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x837DE; return 10; }
    8
}
/// $0837CC: lea.l $5e766.l, a0
pub fn i_0837cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x837D2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $0837D2: jsr $5e770.l
pub fn i_0837d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x837D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x837D8);
    cpu.pc = 0x5E770;
    16
}
/// $0837D8: bclr.b #$3, $13(a6)
pub fn i_0837d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x837DE;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0837DE: jsr $28758.l
pub fn i_0837de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x837E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x837E4);
    cpu.pc = 0x28758;
    16
}
/// $0837E4: bcc.w $837ee
pub fn i_0837e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x837E8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x837EE; return 10; }
    8
}
/// $0837E8: lea.l $83800(pc), a1
pub fn i_0837e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x837EC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83800;
    16
}
/// $0837EC: move.l a1, (a6)
pub fn i_0837ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x837EE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0837EE: jsr $4fa70.l
pub fn i_0837ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x837F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x837F4);
    cpu.pc = 0x4FA70;
    16
}
/// $0837F4: bcc.w $837fe
pub fn i_0837f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x837F8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x837FE; return 10; }
    8
}
/// $0837F8: jmp $518.l
pub fn i_0837f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x837FE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0837FE: rts
pub fn i_0837fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83800;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_083800  ($083800, 294 B, enemies/bosses/miniboss_module_083262.s)
/// $083800: jsr $2783a.l
pub fn i_083800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83806;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83806);
    cpu.pc = 0x2783A;
    16
}
/// $083806: move.l #$2000, d0
pub fn i_083806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8380C;
    cpu.ir = 0x203C;
    let v1: u32 = 0x2000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08380C: jsr $51a28.l
pub fn i_08380c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83812;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83812);
    cpu.pc = 0x51A28;
    16
}
/// $083812: jsr $85fb0(pc)
pub fn i_083812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83816;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x83816);
    cpu.pc = 0x85FB0;
    16
}
/// $083816: lea.l $ffff.w, a0
pub fn i_083816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8381A;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $08381A: move.l a0, $4c(a6)
pub fn i_08381a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8381E;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08381E: jsr $283ca.l
pub fn i_08381e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83824;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83824);
    cpu.pc = 0x283CA;
    16
}
/// $083824: lea.l $ffff.w, a0
pub fn i_083824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83828;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $083828: move.l a0, $48(a6)
pub fn i_083828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8382C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08382C: cmpi.w #$170, $24(a6)
pub fn i_08382c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83832;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x170, Size::Word, false);
    16
}
/// $083832: bgt.w $83852
pub fn i_083832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83836;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x83852; return 10; }
    8
}
/// $083836: lea.l $2e9a1e.l, a1
pub fn i_083836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8383C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E9A1E;
    20
}
/// $08383C: cmpi.b #$0, $21(a6)
pub fn i_08383c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83842;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $083842: beq.w $8384c
pub fn i_083842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83846;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8384C; return 10; }
    8
}
/// $083846: lea.l $2e9a30.l, a1
pub fn i_083846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8384C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E9A30;
    20
}
/// $08384C: jsr $77c7e.l
pub fn i_08384c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83852;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83852);
    cpu.pc = 0x77C7E;
    16
}
/// $083852: lea.l $2e6d86.l, a0
pub fn i_083852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83858;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6D86;
    20
}
/// $083858: jsr $28cd4.l
pub fn i_083858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8385E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8385E);
    cpu.pc = 0x28CD4;
    16
}
/// $08385E: lea.l $2eaca0.l, a1
pub fn i_08385e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83864;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EACA0;
    20
}
/// $083864: cmpi.b #$0, $21(a6)
pub fn i_083864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8386A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08386A: beq.w $83880
pub fn i_08386a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8386E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x83880; return 10; }
    8
}
/// $08386E: lea.l $2e6df2.l, a0
pub fn i_08386e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83874;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6DF2;
    20
}
/// $083874: jsr $28cd4.l
pub fn i_083874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8387A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8387A);
    cpu.pc = 0x28CD4;
    16
}
/// $08387A: lea.l $2eacac.l, a1
pub fn i_08387a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83880;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EACAC;
    20
}
/// $083880: move.w $22(a6), d0
pub fn i_083880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83884;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $083884: movem.w d0, -(a7)
pub fn i_083884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83888;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $083888: btst.b #$0, $3a(a6)
pub fn i_083888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8388E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $08388E: beq.w $83898
pub fn i_08388e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83892;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x83898; return 10; }
    8
}
/// $083892: subi.w #$18, $22(a6)
pub fn i_083892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83898;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $083898: cmpi.b #$1, $20(a6)
pub fn i_083898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8389E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $08389E: beq.w $838a8
pub fn i_08389e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x838A2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x838A8; return 10; }
    8
}
/// $0838A2: jsr $43fac.l
pub fn i_0838a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x838A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x838A8);
    cpu.pc = 0x43FAC;
    16
}
/// $0838A8: movem.w (a7)+, d0
pub fn i_0838a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x838AC;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $0838AC: move.w d0, $22(a6)
pub fn i_0838ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x838B0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0838B0: jsr $267e2.l
pub fn i_0838b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x838B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x838B6);
    cpu.pc = 0x267E2;
    16
}
/// $0838B6: move.w #$fffd, $2e(a6)
pub fn i_0838b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x838BC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0838BC: move.w #$40, d0
pub fn i_0838bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x838C0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0838C0: btst.b #$0, $3a(a6)
pub fn i_0838c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x838C6;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0838C6: beq.w $838cc
pub fn i_0838c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x838CA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x838CC; return 10; }
    8
}
/// $0838CA: neg.w d0
pub fn i_0838ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x838CC;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0838CC: move.w d0, $28(a6)
pub fn i_0838cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x838D0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0838D0: move.w #$5a, $72(a6)
pub fn i_0838d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x838D6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x5A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0838D6: move.b #$f0, d0
pub fn i_0838d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x838DA;
    cpu.ir = 0x103C;
    let v1: u32 = 0xF0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0838DA: jsr $5e722.l
pub fn i_0838da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x838E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x838E0);
    cpu.pc = 0x5E722;
    16
}
/// $0838E0: lea.l $838e6(pc), a1
pub fn i_0838e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x838E4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x838E6;
    16
}
/// $0838E4: move.l a1, (a6)
pub fn i_0838e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x838E6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0838E6: cmpi.w #$50, $72(a6)
pub fn i_0838e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x838EC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x50, Size::Word, false);
    16
}
/// $0838EC: bne.w $838fa
pub fn i_0838ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x838F0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x838FA; return 10; }
    8
}
/// $0838F0: move.w #$1054, d0
pub fn i_0838f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x838F4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1054;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0838F4: jsr $2352.l
pub fn i_0838f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x838FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x838FA);
    cpu.pc = 0x2352;
    16
}
/// $0838FA: jsr $27cee.l
pub fn i_0838fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83900;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83900);
    cpu.pc = 0x27CEE;
    16
}
/// $083900: jsr $28d70.l
pub fn i_083900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83906;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83906);
    cpu.pc = 0x28D70;
    16
}
/// $083906: subq.w #$1, $72(a6)
pub fn i_083906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8390A;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08390A: bne.w $83914
pub fn i_08390a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8390E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x83914; return 10; }
    8
}
/// $08390E: lea.l $83926(pc), a1
pub fn i_08390e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83912;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83926;
    16
}
/// $083912: move.l a1, (a6)
pub fn i_083912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83914;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $083914: jsr $4fa70.l
pub fn i_083914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8391A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8391A);
    cpu.pc = 0x4FA70;
    16
}
/// $08391A: bcc.w $83924
pub fn i_08391a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8391E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83924; return 10; }
    8
}
/// $08391E: jmp $518.l
pub fn i_08391e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83924;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $083924: rts
pub fn i_083924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83926;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_083926  ($083926, 68 B, enemies/bosses/miniboss_module_083262.s)
/// $083926: lea.l $2e987e.l, a1
pub fn i_083926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8392C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E987E;
    20
}
/// $08392C: jsr $77c7e.l
pub fn i_08392c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83932;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83932);
    cpu.pc = 0x77C7E;
    16
}
/// $083932: move.w #$1e, $72(a6)
pub fn i_083932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83938;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083938: jsr $267e2.l
pub fn i_083938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8393E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8393E);
    cpu.pc = 0x267E2;
    16
}
/// $08393E: lea.l $83944(pc), a1
pub fn i_08393e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83942;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83944;
    16
}
/// $083942: move.l a1, (a6)
pub fn i_083942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83944;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $083944: jsr $27cee.l
pub fn i_083944(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8394A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8394A);
    cpu.pc = 0x27CEE;
    16
}
/// $08394A: jsr $28d70.l
pub fn i_08394a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83950;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83950);
    cpu.pc = 0x28D70;
    16
}
/// $083950: subq.w #$1, $72(a6)
pub fn i_083950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83954;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $083954: beq.w $83962
pub fn i_083954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83958;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x83962; return 10; }
    8
}
/// $083958: jsr $4fa70.l
pub fn i_083958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8395E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8395E);
    cpu.pc = 0x4FA70;
    16
}
/// $08395E: bcc.w $83968
pub fn i_08395e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83962;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83968; return 10; }
    8
}
/// $083962: jmp $518.l
pub fn i_083962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83968;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $083968: rts
pub fn i_083968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8396A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_08396a  ($08396A, 120 B, enemies/bosses/miniboss_module_083262.s)
/// $08396A: movea.l $3c(a6), a1
pub fn i_08396a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8396E;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08396E: jsr $2942a.l
pub fn i_08396e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83974;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83974);
    cpu.pc = 0x2942A;
    16
}
/// $083974: move.w #$a5, d1
pub fn i_083974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83978;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $083978: jsr $236e.l
pub fn i_083978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8397E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8397E);
    cpu.pc = 0x236E;
    16
}
/// $08397E: move.w #$10, $70(a6)
pub fn i_08397e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83984;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083984: move.b #$ff, $32(a6)
pub fn i_083984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8398A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08398A: move.b #$ff, $33(a6)
pub fn i_08398a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83990;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083990: move.b #$0, $3a(a6)
pub fn i_083990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83996;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083996: lea.l $2e6e10.l, a0
pub fn i_083996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8399C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6E10;
    20
}
/// $08399C: jsr $28cd4.l
pub fn i_08399c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x839A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x839A2);
    cpu.pc = 0x28CD4;
    16
}
/// $0839A2: bclr.b #$3, $13(a6)
pub fn i_0839a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x839A8;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0839A8: lea.l $839ae(pc), a1
pub fn i_0839a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x839AC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x839AE;
    16
}
/// $0839AC: move.l a1, (a6)
pub fn i_0839ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x839AE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0839AE: jsr $2783a.l
pub fn i_0839ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x839B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x839B4);
    cpu.pc = 0x2783A;
    16
}
/// $0839B4: move.b #$1, $44(a6)
pub fn i_0839b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x839BA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x44);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0839BA: jsr $28d70.l
pub fn i_0839ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x839C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x839C0);
    cpu.pc = 0x28D70;
    16
}
/// $0839C0: jsr $2870a.l
pub fn i_0839c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x839C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x839C6);
    cpu.pc = 0x2870A;
    16
}
/// $0839C6: bcc.w $839d0
pub fn i_0839c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x839CA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x839D0; return 10; }
    8
}
/// $0839CA: lea.l $839e2(pc), a1
pub fn i_0839ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x839CE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x839E2;
    16
}
/// $0839CE: move.l a1, (a6)
pub fn i_0839ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x839D0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0839D0: jsr $4fa70.l
pub fn i_0839d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x839D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x839D6);
    cpu.pc = 0x4FA70;
    16
}
/// $0839D6: bcc.w $839e0
pub fn i_0839d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x839DA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x839E0; return 10; }
    8
}
/// $0839DA: jmp $518.l
pub fn i_0839da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x839E0;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0839E0: rts
pub fn i_0839e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x839E2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_0839e2  ($0839E2, 84 B, enemies/bosses/miniboss_module_083262.s)
/// $0839E2: move.w #$10a9, d0
pub fn i_0839e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x839E6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10A9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0839E6: jsr $2352.l
pub fn i_0839e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x839EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x839EC);
    cpu.pc = 0x2352;
    16
}
/// $0839EC: move.b #$0, $44(a6)
pub fn i_0839ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x839F2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x44);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0839F2: move.w #$a, d0
pub fn i_0839f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x839F6;
    cpu.ir = 0x303C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0839F6: jsr $5e9e4.l
pub fn i_0839f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x839FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x839FC);
    cpu.pc = 0x5E9E4;
    16
}
/// $0839FC: addi.w #$23, d0
pub fn i_0839fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A00;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x23, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $083A00: move.w d0, $72(a6)
pub fn i_083a00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A04;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083A04: lea.l $83a0a(pc), a1
pub fn i_083a04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A08;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83A0A;
    16
}
/// $083A08: move.l a1, (a6)
pub fn i_083a08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A0A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $083A0A: jsr $2783a.l
pub fn i_083a0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83A10);
    cpu.pc = 0x2783A;
    16
}
/// $083A10: jsr $28d70.l
pub fn i_083a10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A16;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83A16);
    cpu.pc = 0x28D70;
    16
}
/// $083A16: subq.w #$1, $72(a6)
pub fn i_083a16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A1A;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $083A1A: bne.w $83a24
pub fn i_083a1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A1E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x83A24; return 10; }
    8
}
/// $083A1E: lea.l $839a2(pc), a1
pub fn i_083a1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A22;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x839A2;
    16
}
/// $083A22: move.l a1, (a6)
pub fn i_083a22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A24;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $083A24: jsr $4fa70.l
pub fn i_083a24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83A2A);
    cpu.pc = 0x4FA70;
    16
}
/// $083A2A: bcc.w $83a34
pub fn i_083a2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A2E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83A34; return 10; }
    8
}
/// $083A2E: jmp $518.l
pub fn i_083a2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A34;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $083A34: rts
pub fn i_083a34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A36;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_083a36  ($083A36, 110 B, enemies/bosses/miniboss_module_083262.s)
/// $083A36: movea.l $3c(a6), a1
pub fn i_083a36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A3A;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $083A3A: jsr $2942a.l
pub fn i_083a3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A40;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83A40);
    cpu.pc = 0x2942A;
    16
}
/// $083A40: move.w #$a5, d1
pub fn i_083a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A44;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $083A44: jsr $236e.l
pub fn i_083a44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83A4A);
    cpu.pc = 0x236E;
    16
}
/// $083A4A: move.w #$0, $70(a6)
pub fn i_083a4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A50;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083A50: move.b #$ff, $32(a6)
pub fn i_083a50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A56;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083A56: move.b #$ff, $33(a6)
pub fn i_083a56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A5C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083A5C: move.b #$0, $3a(a6)
pub fn i_083a5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A62;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083A62: move.w #$8000, $38(a6)
pub fn i_083a62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A68;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083A68: jsr $267e2.l
pub fn i_083a68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A6E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83A6E);
    cpu.pc = 0x267E2;
    16
}
/// $083A6E: jsr $27cee.l
pub fn i_083a6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A74;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83A74);
    cpu.pc = 0x27CEE;
    16
}
/// $083A74: lea.l $2e6ebc.l, a0
pub fn i_083a74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A7A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6EBC;
    20
}
/// $083A7A: jsr $28cd4.l
pub fn i_083a7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83A80);
    cpu.pc = 0x28CD4;
    16
}
/// $083A80: lea.l $83a86(pc), a1
pub fn i_083a80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A84;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83A86;
    16
}
/// $083A84: move.l a1, (a6)
pub fn i_083a84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A86;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $083A86: jsr $2783a.l
pub fn i_083a86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A8C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83A8C);
    cpu.pc = 0x2783A;
    16
}
/// $083A8C: jsr $28d70.l
pub fn i_083a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83A92);
    cpu.pc = 0x28D70;
    16
}
/// $083A92: jsr $4fa70.l
pub fn i_083a92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83A98);
    cpu.pc = 0x4FA70;
    16
}
/// $083A98: bcc.w $83aa2
pub fn i_083a98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A9C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83AA2; return 10; }
    8
}
/// $083A9C: jmp $518.l
pub fn i_083a9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83AA2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $083AA2: rts
pub fn i_083aa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83AA4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_083aa4  ($083AA4, 110 B, enemies/bosses/miniboss_module_083262.s)
/// $083AA4: movea.l $3c(a6), a1
pub fn i_083aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83AA8;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $083AA8: jsr $2942a.l
pub fn i_083aa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83AAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83AAE);
    cpu.pc = 0x2942A;
    16
}
/// $083AAE: move.w #$a7, d1
pub fn i_083aae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83AB2;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $083AB2: jsr $236e.l
pub fn i_083ab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83AB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83AB8);
    cpu.pc = 0x236E;
    16
}
/// $083AB8: move.w #$0, $70(a6)
pub fn i_083ab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83ABE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083ABE: move.b #$ff, $32(a6)
pub fn i_083abe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83AC4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083AC4: move.b #$ff, $33(a6)
pub fn i_083ac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83ACA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083ACA: move.b #$0, $3a(a6)
pub fn i_083aca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83AD0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083AD0: move.w #$8000, $38(a6)
pub fn i_083ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83AD6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083AD6: jsr $267e2.l
pub fn i_083ad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83ADC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83ADC);
    cpu.pc = 0x267E2;
    16
}
/// $083ADC: jsr $27cee.l
pub fn i_083adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83AE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83AE2);
    cpu.pc = 0x27CEE;
    16
}
/// $083AE2: lea.l $2e6ecc.l, a0
pub fn i_083ae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83AE8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6ECC;
    20
}
/// $083AE8: jsr $28cd4.l
pub fn i_083ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83AEE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83AEE);
    cpu.pc = 0x28CD4;
    16
}
/// $083AEE: lea.l $83af4(pc), a1
pub fn i_083aee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83AF2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83AF4;
    16
}
/// $083AF2: move.l a1, (a6)
pub fn i_083af2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83AF4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $083AF4: jsr $2783a.l
pub fn i_083af4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83AFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83AFA);
    cpu.pc = 0x2783A;
    16
}
/// $083AFA: jsr $28d70.l
pub fn i_083afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B00;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83B00);
    cpu.pc = 0x28D70;
    16
}
/// $083B00: jsr $4fa70.l
pub fn i_083b00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B06;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83B06);
    cpu.pc = 0x4FA70;
    16
}
/// $083B06: bcc.w $83b10
pub fn i_083b06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B0A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83B10; return 10; }
    8
}
/// $083B0A: jmp $518.l
pub fn i_083b0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B10;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $083B10: rts
pub fn i_083b10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B12;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_083b12  ($083B12, 114 B, enemies/bosses/miniboss_module_083262.s)
/// $083B12: movea.l $3c(a6), a1
pub fn i_083b12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B16;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $083B16: jsr $2942a.l
pub fn i_083b16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83B1C);
    cpu.pc = 0x2942A;
    16
}
/// $083B1C: move.w #$a7, d1
pub fn i_083b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B20;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $083B20: jsr $236e.l
pub fn i_083b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83B26);
    cpu.pc = 0x236E;
    16
}
/// $083B26: move.w #$100, $70(a6)
pub fn i_083b26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B2C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083B2C: move.b #$ff, $32(a6)
pub fn i_083b2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B32;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083B32: move.b #$ff, $33(a6)
pub fn i_083b32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B38;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083B38: move.b #$0, $3a(a6)
pub fn i_083b38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B3E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083B3E: lea.l $2e6edc.l, a0
pub fn i_083b3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B44;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6EDC;
    20
}
/// $083B44: jsr $28cd4.l
pub fn i_083b44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83B4A);
    cpu.pc = 0x28CD4;
    16
}
/// $083B4A: lea.l $83b92(pc), a1
pub fn i_083b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B4E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83B92;
    16
}
/// $083B4E: jsr $4ae.l
pub fn i_083b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83B54);
    cpu.pc = 0x4AE;
    16
}
/// $083B54: move.w $38(a6), $38(a0)
pub fn i_083b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B5A;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad4, v3 as u16);
    20
}
/// $083B5A: jsr $86076(pc)
pub fn i_083b5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B5E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x83B5E);
    cpu.pc = 0x86076;
    16
}
/// $083B5E: lea.l $83b64(pc), a1
pub fn i_083b5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B62;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83B64;
    16
}
/// $083B62: move.l a1, (a6)
pub fn i_083b62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B64;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $083B64: jsr $2783a.l
pub fn i_083b64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83B6A);
    cpu.pc = 0x2783A;
    16
}
/// $083B6A: cmpi.w #$120, $22(a6)
pub fn i_083b6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B70;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x120, Size::Word, false);
    16
}
/// $083B70: bgt.w $83b7a
pub fn i_083b70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B74;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x83B7A; return 10; }
    8
}
/// $083B74: jsr $28d70.l
pub fn i_083b74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B7A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83B7A);
    cpu.pc = 0x28D70;
    16
}
/// $083B7A: jsr $4fa70.l
pub fn i_083b7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83B80);
    cpu.pc = 0x4FA70;
    16
}
/// $083B80: bcc.w $83b90
pub fn i_083b80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B84;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83B90; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_083b84  ($083B84, 14 B, kernel/jsr5b6_jmp518.c)
/// $083B84: jsr $5b6.l
pub fn i_083b84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83B8A);
    cpu.pc = 0x5B6;
    16
}
/// $083B8A: jmp $518.l
pub fn i_083b8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B90;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $083B90: rts
pub fn i_083b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B92;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_083b92  ($083B92, 72 B, enemies/bosses/miniboss_module_083262.s)
/// $083B92: move.w #$a7, d1
pub fn i_083b92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B96;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $083B96: jsr $236e.l
pub fn i_083b96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83B9C);
    cpu.pc = 0x236E;
    16
}
/// $083B9C: move.b #$ff, $32(a6)
pub fn i_083b9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83BA2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083BA2: move.b #$ff, $33(a6)
pub fn i_083ba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83BA8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083BA8: move.b #$0, $3a(a6)
pub fn i_083ba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83BAE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083BAE: lea.l $2e6eec.l, a0
pub fn i_083bae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83BB4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6EEC;
    20
}
/// $083BB4: jsr $28cd4.l
pub fn i_083bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83BBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83BBA);
    cpu.pc = 0x28CD4;
    16
}
/// $083BBA: lea.l $83bc0(pc), a1
pub fn i_083bba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83BBE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83BC0;
    16
}
/// $083BBE: move.l a1, (a6)
pub fn i_083bbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83BC0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $083BC0: movea.l $c(a6), a0
pub fn i_083bc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83BC4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $083BC4: move.w $22(a0), $22(a6)
pub fn i_083bc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83BCA;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $083BCA: move.w $24(a0), $24(a6)
pub fn i_083bca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83BD0;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $083BD0: cmpi.w #$ff80, $22(a6)
pub fn i_083bd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83BD6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF80, Size::Word, false);
    16
}
/// $083BD6: blt.w $83be0
pub fn i_083bd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83BDA;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x83BE0; return 10; }
    8
}
// ===== JsrAbsThunk_083bda  ($083BDA, 8 B, thunks/jsr_abs_thunks.c)
/// $083BDA: jsr $28d70.l
pub fn i_083bda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83BE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83BE0);
    cpu.pc = 0x28D70;
    16
}
/// $083BE0: rts
pub fn i_083be0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83BE2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_083be2  ($083BE2, 176 B, enemies/bosses/miniboss_finale_083be2.s)
/// $083BE2: move.w #$166, d1
pub fn i_083be2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83BE6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x166;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $083BE6: jsr $236e.l
pub fn i_083be6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83BEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83BEC);
    cpu.pc = 0x236E;
    16
}
/// $083BEC: move.w #$c000, $38(a6)
pub fn i_083bec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83BF2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083BF2: move.b $98(a6), $21(a6)
pub fn i_083bf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83BF8;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad4, v3 as u8);
    20
}
/// $083BF8: move.b $99(a6), $3a(a6)
pub fn i_083bf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83BFE;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $083BFE: bra.w $83c0c
pub fn i_083bfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C02;
    cpu.ir = 0x6000;
    cpu.pc = 0x83C0C;
    10
}
/// $083C0C: move.b #$ff, $32(a6)
pub fn i_083c0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C12;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083C12: move.b #$ff, $33(a6)
pub fn i_083c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C18;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083C18: move.w #$20, $70(a6)
pub fn i_083c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C1E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083C1E: lea.l $2e6efc.l, a0
pub fn i_083c1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C24;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6EFC;
    20
}
/// $083C24: jsr $28cd4.l
pub fn i_083c24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83C2A);
    cpu.pc = 0x28CD4;
    16
}
/// $083C2A: btst.b #$0, $21(a6)
pub fn i_083c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C30;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $083C30: beq.w $83c40
pub fn i_083c30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C34;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x83C40; return 10; }
    8
}
/// $083C34: lea.l $2e6f12.l, a0
pub fn i_083c34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C3A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6F12;
    20
}
/// $083C3A: jsr $28cd4.l
pub fn i_083c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C40;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83C40);
    cpu.pc = 0x28CD4;
    16
}
/// $083C40: lea.l $83c46(pc), a1
pub fn i_083c40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C44;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83C46;
    16
}
/// $083C44: move.l a1, (a6)
pub fn i_083c44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C46;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $083C46: jsr $2783a.l
pub fn i_083c46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83C4C);
    cpu.pc = 0x2783A;
    16
}
/// $083C4C: cmpi.w #$140, $22(a6)
pub fn i_083c4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C52;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x140, Size::Word, false);
    16
}
/// $083C52: bgt.w $83c5c
pub fn i_083c52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C56;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x83C5C; return 10; }
    8
}
/// $083C56: jsr $28d70.l
pub fn i_083c56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83C5C);
    cpu.pc = 0x28D70;
    16
}
/// $083C5C: jsr $2870a.l
pub fn i_083c5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83C62);
    cpu.pc = 0x2870A;
    16
}
/// $083C62: bcc.w $83c80
pub fn i_083c62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C66;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83C80; return 10; }
    8
}
/// $083C66: move.w #$10a6, d0
pub fn i_083c66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C6A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10A6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $083C6A: jsr $2352.l
pub fn i_083c6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C70;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83C70);
    cpu.pc = 0x2352;
    16
}
/// $083C70: lea.l $2e9a0c.l, a1
pub fn i_083c70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C76;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E9A0C;
    20
}
/// $083C76: jsr $77c7e.l
pub fn i_083c76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C7C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83C7C);
    cpu.pc = 0x77C7E;
    16
}
/// $083C7C: bra.w $83c8a
pub fn i_083c7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C80;
    cpu.ir = 0x6000;
    cpu.pc = 0x83C8A;
    10
}
/// $083C80: jsr $4fa70.l
pub fn i_083c80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83C86);
    cpu.pc = 0x4FA70;
    16
}
/// $083C86: bcc.w $83c90
pub fn i_083c86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C8A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83C90; return 10; }
    8
}
/// $083C8A: jmp $518.l
pub fn i_083c8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C90;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $083C90: rts
pub fn i_083c90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C92;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_083c92  ($083C92, 152 B, enemies/bosses/miniboss_finale_083be2.s)
/// $083C92: movea.l $3c(a6), a1
pub fn i_083c92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C96;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $083C96: jsr $2942a.l
pub fn i_083c96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83C9C);
    cpu.pc = 0x2942A;
    16
}
/// $083C9C: move.w #$a6, d1
pub fn i_083c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83CA0;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $083CA0: jsr $236e.l
pub fn i_083ca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83CA6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83CA6);
    cpu.pc = 0x236E;
    16
}
/// $083CA6: move.w #$30, $70(a6)
pub fn i_083ca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83CAC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083CAC: move.b #$ff, $32(a6)
pub fn i_083cac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83CB2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083CB2: move.b #$ff, $33(a6)
pub fn i_083cb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83CB8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083CB8: move.b #$0, $3a(a6)
pub fn i_083cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83CBE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083CBE: move.w #$14, $66(a6)
pub fn i_083cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83CC4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083CC4: lea.l $2e6f28.l, a0
pub fn i_083cc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83CCA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6F28;
    20
}
/// $083CCA: jsr $28cd4.l
pub fn i_083cca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83CD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83CD0);
    cpu.pc = 0x28CD4;
    16
}
/// $083CD0: lea.l $83cd6(pc), a1
pub fn i_083cd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83CD4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83CD6;
    16
}
/// $083CD4: move.l a1, (a6)
pub fn i_083cd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83CD6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $083CD6: jsr $2783a.l
pub fn i_083cd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83CDC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83CDC);
    cpu.pc = 0x2783A;
    16
}
/// $083CDC: jsr $28d70.l
pub fn i_083cdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83CE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83CE2);
    cpu.pc = 0x28D70;
    16
}
/// $083CE2: jsr $2870a.l
pub fn i_083ce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83CE8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83CE8);
    cpu.pc = 0x2870A;
    16
}
/// $083CE8: bcc.w $83cfe
pub fn i_083ce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83CEC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83CFE; return 10; }
    8
}
/// $083CEC: lea.l $5e766.l, a0
pub fn i_083cec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83CF2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $083CF2: jsr $5e770.l
pub fn i_083cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83CF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83CF8);
    cpu.pc = 0x5E770;
    16
}
/// $083CF8: bclr.b #$3, $13(a6)
pub fn i_083cf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83CFE;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $083CFE: jsr $28758.l
pub fn i_083cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83D04);
    cpu.pc = 0x28758;
    16
}
/// $083D04: bcc.w $83d18
pub fn i_083d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D08;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83D18; return 10; }
    8
}
/// $083D08: move.w #$102e, d0
pub fn i_083d08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D0C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $083D0C: jsr $2352.l
pub fn i_083d0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83D12);
    cpu.pc = 0x2352;
    16
}
/// $083D12: lea.l $83d2a(pc), a1
pub fn i_083d12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D16;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83D2A;
    16
}
/// $083D16: move.l a1, (a6)
pub fn i_083d16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D18;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $083D18: jsr $4fa70.l
pub fn i_083d18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83D1E);
    cpu.pc = 0x4FA70;
    16
}
/// $083D1E: bcc.w $83d28
pub fn i_083d1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D22;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83D28; return 10; }
    8
}
/// $083D22: jmp $518.l
pub fn i_083d22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D28;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $083D28: rts
pub fn i_083d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D2A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_083d2a  ($083D2A, 68 B, enemies/bosses/miniboss_finale_083be2.s)
/// $083D2A: lea.l $ffff.w, a0
pub fn i_083d2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D2E;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $083D2E: move.l a0, $48(a6)
pub fn i_083d2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D32;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $083D32: lea.l $2e9a94.l, a1
pub fn i_083d32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D38;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E9A94;
    20
}
/// $083D38: jsr $77c7e.l
pub fn i_083d38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83D3E);
    cpu.pc = 0x77C7E;
    16
}
/// $083D3E: lea.l $2e6f3e.l, a0
pub fn i_083d3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D44;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6F3E;
    20
}
/// $083D44: jsr $28cd4.l
pub fn i_083d44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83D4A);
    cpu.pc = 0x28CD4;
    16
}
/// $083D4A: lea.l $83d50(pc), a1
pub fn i_083d4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D4E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83D50;
    16
}
/// $083D4E: move.l a1, (a6)
pub fn i_083d4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D50;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $083D50: jsr $2783a.l
pub fn i_083d50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83D56);
    cpu.pc = 0x2783A;
    16
}
/// $083D56: jsr $28d70.l
pub fn i_083d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83D5C);
    cpu.pc = 0x28D70;
    16
}
/// $083D5C: jsr $4fa70.l
pub fn i_083d5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83D62);
    cpu.pc = 0x4FA70;
    16
}
/// $083D62: bcc.w $83d6c
pub fn i_083d62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D66;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83D6C; return 10; }
    8
}
/// $083D66: jmp $518.l
pub fn i_083d66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D6C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $083D6C: rts
pub fn i_083d6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D6E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_083d6e  ($083D6E, 218 B, enemies/bosses/miniboss_finale_083be2.s)
/// $083D6E: movea.l $3c(a6), a1
pub fn i_083d6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D72;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $083D72: jsr $2942a.l
pub fn i_083d72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D78;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83D78);
    cpu.pc = 0x2942A;
    16
}
/// $083D78: lea.l $2e6f54.l, a0
pub fn i_083d78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D7E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6F54;
    20
}
/// $083D7E: jsr $28cd4.l
pub fn i_083d7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D84;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83D84);
    cpu.pc = 0x28CD4;
    16
}
/// $083D84: move.b #$0, $21(a6)
pub fn i_083d84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D8A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083D8A: bra.w $83daa
pub fn i_083d8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D8E;
    cpu.ir = 0x6000;
    cpu.pc = 0x83DAA;
    10
}
/// $083DAA: move.w #$30, $70(a6)
pub fn i_083daa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83DB0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083DB0: move.b #$0, $3a(a6)
pub fn i_083db0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83DB6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083DB6: move.w #$14, $66(a6)
pub fn i_083db6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83DBC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083DBC: lea.l $83dc2(pc), a1
pub fn i_083dbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83DC0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83DC2;
    16
}
/// $083DC0: move.l a1, (a6)
pub fn i_083dc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83DC2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $083DC2: jsr $2783a.l
pub fn i_083dc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83DC8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83DC8);
    cpu.pc = 0x2783A;
    16
}
/// $083DC8: jsr $28d70.l
pub fn i_083dc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83DCE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83DCE);
    cpu.pc = 0x28D70;
    16
}
/// $083DCE: jsr $2870a.l
pub fn i_083dce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83DD4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83DD4);
    cpu.pc = 0x2870A;
    16
}
/// $083DD4: bcc.w $83dea
pub fn i_083dd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83DD8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83DEA; return 10; }
    8
}
/// $083DD8: lea.l $5e766.l, a0
pub fn i_083dd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83DDE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $083DDE: jsr $5e770.l
pub fn i_083dde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83DE4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83DE4);
    cpu.pc = 0x5E770;
    16
}
/// $083DE4: bclr.b #$3, $13(a6)
pub fn i_083de4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83DEA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $083DEA: jsr $28758.l
pub fn i_083dea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83DF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83DF0);
    cpu.pc = 0x28758;
    16
}
/// $083DF0: bcc.w $83e36
pub fn i_083df0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83DF4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83E36; return 10; }
    8
}
/// $083DF4: move.w #$1027, d0
pub fn i_083df4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83DF8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1027;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $083DF8: jsr $2352.l
pub fn i_083df8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83DFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83DFE);
    cpu.pc = 0x2352;
    16
}
/// $083DFE: cmpi.b #$0, $21(a6)
pub fn i_083dfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E04;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $083E04: bne.w $83e1c
pub fn i_083e04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E08;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x83E1C; return 10; }
    8
}
/// $083E08: jsr $863e4(pc)
pub fn i_083e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E0C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x83E0C);
    cpu.pc = 0x863E4;
    16
}
/// $083E0C: lea.l $2e9aa6.l, a1
pub fn i_083e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E12;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E9AA6;
    20
}
/// $083E12: jsr $77c7e.l
pub fn i_083e12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83E18);
    cpu.pc = 0x77C7E;
    16
}
/// $083E18: bra.w $83e40
pub fn i_083e18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E1C;
    cpu.ir = 0x6000;
    cpu.pc = 0x83E40;
    10
}
/// $083E1C: jsr $863f2(pc)
pub fn i_083e1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E20;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x83E20);
    cpu.pc = 0x863F2;
    16
}
/// $083E20: subi.w #$10, $24(a6)
pub fn i_083e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E26;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $083E26: lea.l $2e9aa6.l, a1
pub fn i_083e26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E2C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E9AA6;
    20
}
/// $083E2C: jsr $77c7e.l
pub fn i_083e2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83E32);
    cpu.pc = 0x77C7E;
    16
}
/// $083E32: bra.w $83e40
pub fn i_083e32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E36;
    cpu.ir = 0x6000;
    cpu.pc = 0x83E40;
    10
}
/// $083E36: jsr $4fa70.l
pub fn i_083e36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83E3C);
    cpu.pc = 0x4FA70;
    16
}
/// $083E3C: bcc.w $83e46
pub fn i_083e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E40;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83E46; return 10; }
    8
}
/// $083E40: jmp $518.l
pub fn i_083e40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E46;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $083E46: rts
pub fn i_083e46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E48;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_083e48  ($083E48, 164 B, enemies/bosses/miniboss_finale_083be2.s)
/// $083E48: movea.l $3c(a6), a1
pub fn i_083e48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E4C;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $083E4C: jsr $2942a.l
pub fn i_083e4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83E52);
    cpu.pc = 0x2942A;
    16
}
/// $083E52: move.w #$89, d1
pub fn i_083e52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E56;
    cpu.ir = 0x323C;
    let v1: u32 = 0x89;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $083E56: jsr $236e.l
pub fn i_083e56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83E5C);
    cpu.pc = 0x236E;
    16
}
/// $083E5C: lea.l $2e6f80.l, a0
pub fn i_083e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E62;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6F80;
    20
}
/// $083E62: jsr $28cd4.l
pub fn i_083e62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E68;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83E68);
    cpu.pc = 0x28CD4;
    16
}
/// $083E68: move.w #$50, $70(a6)
pub fn i_083e68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E6E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x50;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083E6E: move.b #$0, $3a(a6)
pub fn i_083e6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E74;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083E74: move.w #$64, $66(a6)
pub fn i_083e74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E7A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x64;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083E7A: move.w #$8000, $38(a6)
pub fn i_083e7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E80;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083E80: jsr $267e2.l
pub fn i_083e80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83E86);
    cpu.pc = 0x267E2;
    16
}
/// $083E86: jsr $27afc.l
pub fn i_083e86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E8C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83E8C);
    cpu.pc = 0x27AFC;
    16
}
/// $083E8C: lea.l $83e92(pc), a1
pub fn i_083e8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E90;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83E92;
    16
}
/// $083E90: move.l a1, (a6)
pub fn i_083e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E92;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $083E92: jsr $2783a.l
pub fn i_083e92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83E98);
    cpu.pc = 0x2783A;
    16
}
/// $083E98: jsr $28d70.l
pub fn i_083e98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E9E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83E9E);
    cpu.pc = 0x28D70;
    16
}
/// $083E9E: jsr $2870a.l
pub fn i_083e9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83EA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83EA4);
    cpu.pc = 0x2870A;
    16
}
/// $083EA4: bcc.w $83ec0
pub fn i_083ea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83EA8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83EC0; return 10; }
    8
}
/// $083EA8: lea.l $5e766.l, a0
pub fn i_083ea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83EAE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $083EAE: jsr $5e770.l
pub fn i_083eae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83EB4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83EB4);
    cpu.pc = 0x5E770;
    16
}
/// $083EB4: bclr.b #$3, $13(a6)
pub fn i_083eb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83EBA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $083EBA: lea.l $83eec(pc), a1
pub fn i_083eba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83EBE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83EEC;
    16
}
/// $083EBE: move.l a1, (a6)
pub fn i_083ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83EC0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $083EC0: jsr $28758.l
pub fn i_083ec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83EC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83EC6);
    cpu.pc = 0x28758;
    16
}
/// $083EC6: bcc.w $83eda
pub fn i_083ec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83ECA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83EDA; return 10; }
    8
}
/// $083ECA: move.w #$1023, d0
pub fn i_083eca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83ECE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1023;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $083ECE: jsr $2352.l
pub fn i_083ece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83ED4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83ED4);
    cpu.pc = 0x2352;
    16
}
/// $083ED4: lea.l $83f26(pc), a1
pub fn i_083ed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83ED8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83F26;
    16
}
/// $083ED8: move.l a1, (a6)
pub fn i_083ed8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83EDA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $083EDA: jsr $4fa70.l
pub fn i_083eda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83EE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83EE0);
    cpu.pc = 0x4FA70;
    16
}
/// $083EE0: bcc.w $83eea
pub fn i_083ee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83EE4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83EEA; return 10; }
    8
}
/// $083EE4: jmp $518.l
pub fn i_083ee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83EEA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $083EEA: rts
pub fn i_083eea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83EEC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_083eec  ($083EEC, 58 B, enemies/bosses/miniboss_finale_083be2.s)
/// $083EEC: lea.l $2e6f96.l, a0
pub fn i_083eec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83EF2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6F96;
    20
}
/// $083EF2: jsr $28cd4.l
pub fn i_083ef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83EF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83EF8);
    cpu.pc = 0x28CD4;
    16
}
/// $083EF8: lea.l $83efe(pc), a1
pub fn i_083ef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83EFC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83EFE;
    16
}
/// $083EFC: move.l a1, (a6)
pub fn i_083efc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83EFE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $083EFE: jsr $2783a.l
pub fn i_083efe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83F04);
    cpu.pc = 0x2783A;
    16
}
/// $083F04: jsr $28d70.l
pub fn i_083f04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83F0A);
    cpu.pc = 0x28D70;
    16
}
/// $083F0A: bcc.w $83f14
pub fn i_083f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F0E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83F14; return 10; }
    8
}
/// $083F0E: lea.l $83e8c(pc), a1
pub fn i_083f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F12;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83E8C;
    16
}
/// $083F12: move.l a1, (a6)
pub fn i_083f12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F14;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $083F14: jsr $4fa70.l
pub fn i_083f14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83F1A);
    cpu.pc = 0x4FA70;
    16
}
/// $083F1A: bcc.w $83f24
pub fn i_083f1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F1E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83F24; return 10; }
    8
}
/// $083F1E: jmp $518.l
pub fn i_083f1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F24;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $083F24: rts
pub fn i_083f24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F26;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_083f26  ($083F26, 130 B, enemies/bosses/miniboss_finale_083be2.s)
/// $083F26: move.l #$2000, d0
pub fn i_083f26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F2C;
    cpu.ir = 0x203C;
    let v1: u32 = 0x2000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $083F2C: jsr $51a28.l
pub fn i_083f2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83F32);
    cpu.pc = 0x51A28;
    16
}
/// $083F32: lea.l $ffff.w, a0
pub fn i_083f32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F36;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $083F36: move.l a0, $48(a6)
pub fn i_083f36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F3A;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $083F3A: lea.l $2e9890.l, a1
pub fn i_083f3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F40;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E9890;
    20
}
/// $083F40: jsr $77c7e.l
pub fn i_083f40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F46;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83F46);
    cpu.pc = 0x77C7E;
    16
}
/// $083F46: lea.l $2e9a82.l, a1
pub fn i_083f46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F4C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E9A82;
    20
}
/// $083F4C: jsr $77c7e.l
pub fn i_083f4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83F52);
    cpu.pc = 0x77C7E;
    16
}
/// $083F52: lea.l $2e6fba.l, a0
pub fn i_083f52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F58;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6FBA;
    20
}
/// $083F58: jsr $28cd4.l
pub fn i_083f58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F5E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83F5E);
    cpu.pc = 0x28CD4;
    16
}
/// $083F5E: jsr $267e2.l
pub fn i_083f5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F64;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83F64);
    cpu.pc = 0x267E2;
    16
}
/// $083F64: move.w #$280, $2a(a6)
pub fn i_083f64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F6A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x280;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083F6A: move.w #$ffc0, $2e(a6)
pub fn i_083f6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F70;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFC0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083F70: lea.l $83f76(pc), a1
pub fn i_083f70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F74;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83F76;
    16
}
/// $083F74: move.l a1, (a6)
pub fn i_083f74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F76;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $083F76: jsr $27cee.l
pub fn i_083f76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F7C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83F7C);
    cpu.pc = 0x27CEE;
    16
}
/// $083F7C: bcc.w $83f90
pub fn i_083f7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F80;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83F90; return 10; }
    8
}
/// $083F80: cmpi.w #$0, $2a(a6)
pub fn i_083f80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F86;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $083F86: bgt.w $83f90
pub fn i_083f86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F8A;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x83F90; return 10; }
    8
}
/// $083F8A: jsr $267e2.l
pub fn i_083f8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83F90);
    cpu.pc = 0x267E2;
    16
}
/// $083F90: jsr $28d70.l
pub fn i_083f90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83F96);
    cpu.pc = 0x28D70;
    16
}
/// $083F96: jsr $4fa70.l
pub fn i_083f96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83F9C);
    cpu.pc = 0x4FA70;
    16
}
/// $083F9C: bcc.w $83fa6
pub fn i_083f9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83FA0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83FA6; return 10; }
    8
}
/// $083FA0: jmp $518.l
pub fn i_083fa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83FA6;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $083FA6: rts
pub fn i_083fa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83FA8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_083fa8  ($083FA8, 40 B, enemies/bosses/miniboss_finale_083be2.s)
/// $083FA8: movea.l $3c(a6), a1
pub fn i_083fa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83FAC;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $083FAC: jsr $2942a.l
pub fn i_083fac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83FB2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83FB2);
    cpu.pc = 0x2942A;
    16
}
/// $083FB2: move.w #$f0, $70(a6)
pub fn i_083fb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83FB8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083FB8: move.b #$0, $21(a6)
pub fn i_083fb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83FBE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083FBE: jsr $86328(pc)
pub fn i_083fbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83FC2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x83FC2);
    cpu.pc = 0x86328;
    16
}
/// $083FC2: lea.l $83fc8(pc), a1
pub fn i_083fc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83FC6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83FC8;
    16
}
/// $083FC6: move.l a1, (a6)
pub fn i_083fc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83FC8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $083FC8: jmp $518.l
pub fn i_083fc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83FCE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== TaskHandler_083fd0  ($083FD0, 18 B, enemies/bosses/miniboss_finale_083be2.s)
/// $083FD0: jsr $2783a.l
pub fn i_083fd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83FD6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83FD6);
    cpu.pc = 0x2783A;
    16
}
/// $083FD6: cmpi.w #$140, $22(a6)
pub fn i_083fd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83FDC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x140, Size::Word, false);
    16
}
/// $083FDC: ble.w $83fe2
pub fn i_083fdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83FE0;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x83FE2; return 10; }
    8
}
/// $083FE0: rts
pub fn i_083fe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83FE2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_083fe2  ($083FE2, 126 B, enemies/bosses/miniboss_finale_083be2.s)
/// $083FE2: move.w #$20, $70(a6)
pub fn i_083fe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83FE8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083FE8: move.w #$a, $66(a6)
pub fn i_083fe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83FEE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083FEE: lea.l $2e6fca.l, a0
pub fn i_083fee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83FF4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6FCA;
    20
}
/// $083FF4: jsr $28cd4.l
pub fn i_083ff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83FFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83FFA);
    cpu.pc = 0x28CD4;
    16
}
/// $083FFA: lea.l $84000(pc), a1
pub fn i_083ffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83FFE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84000;
    16
}
/// $083FFE: move.l a1, (a6)
pub fn i_083ffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84000;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084000: jsr $2783a.l
pub fn i_084000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84006;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84006);
    cpu.pc = 0x2783A;
    16
}
/// $084006: jsr $28d70.l
pub fn i_084006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8400C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8400C);
    cpu.pc = 0x28D70;
    16
}
/// $08400C: jsr $2870a.l
pub fn i_08400c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84012;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84012);
    cpu.pc = 0x2870A;
    16
}
/// $084012: bcc.w $84028
pub fn i_084012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84016;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x84028; return 10; }
    8
}
/// $084016: lea.l $5e766.l, a0
pub fn i_084016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8401C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $08401C: jsr $5e798.l
pub fn i_08401c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84022;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84022);
    cpu.pc = 0x5E798;
    16
}
/// $084022: bclr.b #$3, $13(a6)
pub fn i_084022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84028;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $084028: jsr $28758.l
pub fn i_084028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8402E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8402E);
    cpu.pc = 0x28758;
    16
}
/// $08402E: bcc.w $8404e
pub fn i_08402e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84032;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8404E; return 10; }
    8
}
/// $084032: lea.l $2eacb8.l, a1
pub fn i_084032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84038;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EACB8;
    20
}
/// $084038: jsr $43fac.l
pub fn i_084038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8403E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8403E);
    cpu.pc = 0x43FAC;
    16
}
/// $08403E: move.w #$1030, d0
pub fn i_08403e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84042;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1030;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $084042: jsr $2352.l
pub fn i_084042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84048;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84048);
    cpu.pc = 0x2352;
    16
}
/// $084048: lea.l $84060(pc), a1
pub fn i_084048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8404C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84060;
    16
}
/// $08404C: move.l a1, (a6)
pub fn i_08404c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8404E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08404E: jsr $4fa70.l
pub fn i_08404e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84054;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84054);
    cpu.pc = 0x4FA70;
    16
}
/// $084054: bcc.w $8405e
pub fn i_084054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84058;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8405E; return 10; }
    8
}
/// $084058: jmp $518.l
pub fn i_084058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8405E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08405E: rts
pub fn i_08405e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84060;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_084060  ($084060, 236 B, enemies/bosses/miniboss_finale_083be2.s)
/// $084060: move.l #$500, d0
pub fn i_084060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84066;
    cpu.ir = 0x203C;
    let v1: u32 = 0x500;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $084066: jsr $51a28.l
pub fn i_084066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8406C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8406C);
    cpu.pc = 0x51A28;
    16
}
/// $08406C: lea.l $ffff.w, a0
pub fn i_08406c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84070;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $084070: move.l a0, $48(a6)
pub fn i_084070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84074;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $084074: move.b $21(a6), d0
pub fn i_084074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84078;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $084078: movea.l $c(a6), a0
pub fn i_084078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8407C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $08407C: bset.b d0, $21(a0)
pub fn i_08407c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84080;
    cpu.ir = 0x1E8;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $084080: lea.l $2e9aca.l, a1
pub fn i_084080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84086;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E9ACA;
    20
}
/// $084086: jsr $77c7e.l
pub fn i_084086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8408C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8408C);
    cpu.pc = 0x77C7E;
    16
}
/// $08408C: lea.l $2eaa60.l, a0
pub fn i_08408c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84092;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EAA60;
    20
}
/// $084092: clr.l d0
pub fn i_084092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84094;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $084094: move.b $21(a6), d0
pub fn i_084094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84098;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $084098: asl.l #$4, d0
pub fn i_084098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8409A;
    cpu.ir = 0xE980;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Long, 0, true);
    cpu.d[0] = r;
    14
}
/// $08409A: movea.l (a0, d0.w), a2
pub fn i_08409a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8409E;
    cpu.ir = 0x2470;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    18
}
/// $08409E: movem.l a0, -(a7)
pub fn i_08409e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x840A2;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $0840A2: movem.l d0, -(a7)
pub fn i_0840a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x840A6;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $0840A6: jsr $5022a.l
pub fn i_0840a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x840AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x840AC);
    cpu.pc = 0x5022A;
    16
}
/// $0840AC: movem.l (a7)+, d0
pub fn i_0840ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x840B0;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $0840B0: movem.l (a7)+, a0
pub fn i_0840b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x840B4;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $0840B4: movea.l $8(a0, d0.w), a2
pub fn i_0840b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x840B8;
    cpu.ir = 0x2470;
    let ad2 = cpu.a[0].wrapping_add(0x8).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    18
}
/// $0840B8: jsr $5022a.l
pub fn i_0840b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x840BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x840BE);
    cpu.pc = 0x5022A;
    16
}
/// $0840BE: move.b #$0, $20(a6)
pub fn i_0840be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x840C4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0840C4: lea.l $840ca(pc), a1
pub fn i_0840c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x840C8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x840CA;
    16
}
/// $0840C8: move.l a1, (a6)
pub fn i_0840c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x840CA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0840CA: btst.b #$4, $20(a6)
pub fn i_0840ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x840D0;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0840D0: bne.w $84102
pub fn i_0840d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x840D4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x84102; return 10; }
    8
}
/// $0840D4: move.b $21(a6), d0
pub fn i_0840d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x840D8;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0840D8: subq.b #$1, d0
pub fn i_0840d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x840DA;
    cpu.ir = 0x5300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $0840DA: cmpi.b #$0, d0
pub fn i_0840da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x840DE;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $0840DE: bge.w $840ec
pub fn i_0840de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x840E2;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x840EC; return 10; }
    8
}
/// $0840E2: bset.b #$4, $20(a6)
pub fn i_0840e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x840E8;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0840E8: bra.w $84102
pub fn i_0840e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x840EC;
    cpu.ir = 0x6000;
    cpu.pc = 0x84102;
    10
}
/// $0840EC: movea.l $c(a6), a0
pub fn i_0840ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x840F0;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0840F0: btst.l d0, $21(a0)
pub fn i_0840f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x840F4;
    cpu.ir = 0x128;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0840F4: beq.w $84102
pub fn i_0840f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x840F8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x84102; return 10; }
    8
}
/// $0840F8: jsr $864b6(pc)
pub fn i_0840f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x840FC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x840FC);
    cpu.pc = 0x864B6;
    16
}
/// $0840FC: bset.b #$4, $20(a6)
pub fn i_0840fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84102;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $084102: btst.b #$0, $20(a6)
pub fn i_084102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84108;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $084108: bne.w $8413a
pub fn i_084108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8410C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8413A; return 10; }
    8
}
/// $08410C: move.b $21(a6), d0
pub fn i_08410c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84110;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $084110: addq.b #$1, d0
pub fn i_084110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84112;
    cpu.ir = 0x5200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $084112: cmpi.b #$7, d0
pub fn i_084112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84116;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x7, Size::Byte, false);
    8
}
/// $084116: ble.w $84124
pub fn i_084116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8411A;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x84124; return 10; }
    8
}
/// $08411A: bset.b #$0, $20(a6)
pub fn i_08411a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84120;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $084120: bra.w $8413a
pub fn i_084120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84124;
    cpu.ir = 0x6000;
    cpu.pc = 0x8413A;
    10
}
/// $084124: movea.l $c(a6), a0
pub fn i_084124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84128;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $084128: btst.l d0, $21(a0)
pub fn i_084128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8412C;
    cpu.ir = 0x128;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $08412C: beq.w $8413a
pub fn i_08412c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84130;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8413A; return 10; }
    8
}
/// $084130: jsr $864d0(pc)
pub fn i_084130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84134;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x84134);
    cpu.pc = 0x864D0;
    16
}
/// $084134: bset.b #$0, $20(a6)
pub fn i_084134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8413A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08413A: cmpi.b #$11, $21(a6)
pub fn i_08413a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84140;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x11, Size::Byte, false);
    16
}
/// $084140: bne.w $8414a
pub fn i_084140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84144;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8414A; return 10; }
    8
}
/// $084144: jmp $518.l
pub fn i_084144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8414A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08414A: rts
pub fn i_08414a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8414C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_08414c  ($08414C, 114 B, enemies/bosses/miniboss_finale_083be2.s)
/// $08414C: movea.l $3c(a6), a1
pub fn i_08414c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84150;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $084150: jsr $2942a.l
pub fn i_084150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84156;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84156);
    cpu.pc = 0x2942A;
    16
}
/// $084156: move.w #$a4, d1
pub fn i_084156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8415A;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08415A: jsr $236e.l
pub fn i_08415a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84160;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84160);
    cpu.pc = 0x236E;
    16
}
/// $084160: move.w #$60, $70(a6)
pub fn i_084160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84166;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $084166: move.b #$ff, $32(a6)
pub fn i_084166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8416C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08416C: move.b #$ff, $33(a6)
pub fn i_08416c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84172;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $084172: move.b #$0, $3a(a6)
pub fn i_084172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84178;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $084178: move.w #$1, $66(a6)
pub fn i_084178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8417E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08417E: lea.l $2e6fe0.l, a0
pub fn i_08417e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84184;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6FE0;
    20
}
/// $084184: jsr $28cd4.l
pub fn i_084184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8418A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8418A);
    cpu.pc = 0x28CD4;
    16
}
/// $08418A: lea.l $84190(pc), a1
pub fn i_08418a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8418E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84190;
    16
}
/// $08418E: move.l a1, (a6)
pub fn i_08418e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84190;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084190: jsr $2783a.l
pub fn i_084190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84196;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84196);
    cpu.pc = 0x2783A;
    16
}
/// $084196: jsr $28d70.l
pub fn i_084196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8419C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8419C);
    cpu.pc = 0x28D70;
    16
}
/// $08419C: jsr $2870a.l
pub fn i_08419c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x841A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x841A2);
    cpu.pc = 0x2870A;
    16
}
/// $0841A2: bcc.w $841ac
pub fn i_0841a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x841A6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x841AC; return 10; }
    8
}
/// $0841A6: lea.l $841be(pc), a1
pub fn i_0841a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x841AA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x841BE;
    16
}
/// $0841AA: move.l a1, (a6)
pub fn i_0841aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x841AC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0841AC: jsr $4fa70.l
pub fn i_0841ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x841B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x841B2);
    cpu.pc = 0x4FA70;
    16
}
/// $0841B2: bcc.w $841bc
pub fn i_0841b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x841B6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x841BC; return 10; }
    8
}
/// $0841B6: jmp $518.l
pub fn i_0841b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x841BC;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0841BC: rts
pub fn i_0841bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x841BE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_0841be  ($0841BE, 56 B, enemies/bosses/miniboss_finale_083be2.s)
/// $0841BE: lea.l $ffff.w, a0
pub fn i_0841be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x841C2;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $0841C2: move.l a0, $48(a6)
pub fn i_0841c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x841C6;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0841C6: lea.l $2e6ff6.l, a0
pub fn i_0841c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x841CC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6FF6;
    20
}
/// $0841CC: jsr $28cd4.l
pub fn i_0841cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x841D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x841D2);
    cpu.pc = 0x28CD4;
    16
}
/// $0841D2: lea.l $841d8(pc), a1
pub fn i_0841d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x841D6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x841D8;
    16
}
/// $0841D6: move.l a1, (a6)
pub fn i_0841d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x841D8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0841D8: jsr $2783a.l
pub fn i_0841d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x841DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x841DE);
    cpu.pc = 0x2783A;
    16
}
/// $0841DE: jsr $28d70.l
pub fn i_0841de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x841E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x841E4);
    cpu.pc = 0x28D70;
    16
}
/// $0841E4: jsr $4fa70.l
pub fn i_0841e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x841EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x841EA);
    cpu.pc = 0x4FA70;
    16
}
/// $0841EA: bcc.w $841f4
pub fn i_0841ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x841EE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x841F4; return 10; }
    8
}
/// $0841EE: jmp $518.l
pub fn i_0841ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x841F4;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0841F4: rts
pub fn i_0841f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x841F6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_0841f6  ($0841F6, 86 B, enemies/bosses/miniboss_finale_083be2.s)
/// $0841F6: movea.l $3c(a6), a1
pub fn i_0841f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x841FA;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0841FA: jsr $2942a.l
pub fn i_0841fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84200;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84200);
    cpu.pc = 0x2942A;
    16
}
/// $084200: move.w #$40, $70(a6)
pub fn i_084200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84206;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $084206: move.w #$1, $66(a6)
pub fn i_084206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8420C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08420C: lea.l $2e7006.l, a0
pub fn i_08420c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84212;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E7006;
    20
}
/// $084212: jsr $28cd4.l
pub fn i_084212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84218;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84218);
    cpu.pc = 0x28CD4;
    16
}
/// $084218: lea.l $8421e(pc), a1
pub fn i_084218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8421C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8421E;
    16
}
/// $08421C: move.l a1, (a6)
pub fn i_08421c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8421E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08421E: jsr $2783a.l
pub fn i_08421e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84224;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84224);
    cpu.pc = 0x2783A;
    16
}
/// $084224: jsr $28d70.l
pub fn i_084224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8422A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8422A);
    cpu.pc = 0x28D70;
    16
}
/// $08422A: jsr $2870a.l
pub fn i_08422a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84230;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84230);
    cpu.pc = 0x2870A;
    16
}
/// $084230: bcc.w $8423a
pub fn i_084230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84234;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8423A; return 10; }
    8
}
/// $084234: lea.l $8424c(pc), a1
pub fn i_084234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84238;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8424C;
    16
}
/// $084238: move.l a1, (a6)
pub fn i_084238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8423A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08423A: jsr $4fa70.l
pub fn i_08423a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84240;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84240);
    cpu.pc = 0x4FA70;
    16
}
/// $084240: bcc.w $8424a
pub fn i_084240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84244;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8424A; return 10; }
    8
}
/// $084244: jmp $518.l
pub fn i_084244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8424A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08424A: rts
pub fn i_08424a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8424C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_08424c  ($08424C, 26 B, enemies/bosses/miniboss_finale_083be2.s)
/// $08424C: lea.l $ffff.w, a0
pub fn i_08424c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84250;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $084250: move.l a0, $48(a6)
pub fn i_084250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84254;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $084254: jsr $86400(pc)
pub fn i_084254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84258;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x84258);
    cpu.pc = 0x86400;
    16
}
/// $084258: lea.l $8425e(pc), a1
pub fn i_084258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8425C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8425E;
    16
}
/// $08425C: move.l a1, (a6)
pub fn i_08425c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8425E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08425E: jmp $518.l
pub fn i_08425e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84264;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== TaskHandler_084266  ($084266, 28 B, enemies/bosses/miniboss_finale_083be2.s)
/// $084266: movea.l $3c(a6), a1
pub fn i_084266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8426A;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08426A: jsr $2942a.l
pub fn i_08426a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84270;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84270);
    cpu.pc = 0x2942A;
    16
}
/// $084270: jsr $86364(pc)
pub fn i_084270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84274;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x84274);
    cpu.pc = 0x86364;
    16
}
/// $084274: lea.l $8427a(pc), a1
pub fn i_084274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84278;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8427A;
    16
}
/// $084278: move.l a1, (a6)
pub fn i_084278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8427A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08427A: jmp $518.l
pub fn i_08427a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84280;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== TaskHandler_084282  ($084282, 78 B, enemies/bosses/miniboss_finale_083be2.s)
/// $084282: move.w #$0, $70(a6)
pub fn i_084282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84288;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $084288: move.w #$1, $66(a6)
pub fn i_084288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8428E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08428E: lea.l $2e701c.l, a0
pub fn i_08428e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84294;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E701C;
    20
}
/// $084294: jsr $28cd4.l
pub fn i_084294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8429A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8429A);
    cpu.pc = 0x28CD4;
    16
}
/// $08429A: lea.l $842a0(pc), a1
pub fn i_08429a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8429E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x842A0;
    16
}
/// $08429E: move.l a1, (a6)
pub fn i_08429e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x842A0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0842A0: jsr $2783a.l
pub fn i_0842a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x842A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x842A6);
    cpu.pc = 0x2783A;
    16
}
/// $0842A6: jsr $28d70.l
pub fn i_0842a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x842AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x842AC);
    cpu.pc = 0x28D70;
    16
}
/// $0842AC: jsr $2870a.l
pub fn i_0842ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x842B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x842B2);
    cpu.pc = 0x2870A;
    16
}
/// $0842B2: bcc.w $842be
pub fn i_0842b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x842B6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x842BE; return 10; }
    8
}
/// $0842B6: jsr $864ea(pc)
pub fn i_0842b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x842BA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x842BA);
    cpu.pc = 0x864EA;
    16
}
/// $0842BA: bra.w $842c8
pub fn i_0842ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x842BE;
    cpu.ir = 0x6000;
    cpu.pc = 0x842C8;
    10
}
/// $0842BE: jsr $4fa70.l
pub fn i_0842be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x842C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x842C4);
    cpu.pc = 0x4FA70;
    16
}
/// $0842C4: bcc.w $842ce
pub fn i_0842c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x842C8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x842CE; return 10; }
    8
}
/// $0842C8: jmp $518.l
pub fn i_0842c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x842CE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0842CE: rts
pub fn i_0842ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x842D0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_0842d0  ($0842D0, 198 B, enemies/bosses/miniboss_finale_083be2.s)
/// $0842D0: movea.l $3c(a6), a1
pub fn i_0842d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x842D4;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0842D4: jsr $2942a.l
pub fn i_0842d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x842DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x842DA);
    cpu.pc = 0x2942A;
    16
}
/// $0842DA: move.w #$30, $70(a6)
pub fn i_0842da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x842E0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0842E0: move.w #$1, $66(a6)
pub fn i_0842e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x842E6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0842E6: lea.l $2e7032.l, a0
pub fn i_0842e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x842EC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E7032;
    20
}
/// $0842EC: jsr $28cd4.l
pub fn i_0842ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x842F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x842F2);
    cpu.pc = 0x28CD4;
    16
}
/// $0842F2: lea.l $842f8(pc), a1
pub fn i_0842f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x842F6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x842F8;
    16
}
/// $0842F6: move.l a1, (a6)
pub fn i_0842f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x842F8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0842F8: jsr $2783a.l
pub fn i_0842f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x842FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x842FE);
    cpu.pc = 0x2783A;
    16
}
/// $0842FE: jsr $28d70.l
pub fn i_0842fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84304;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84304);
    cpu.pc = 0x28D70;
    16
}
/// $084304: jsr $2870a.l
pub fn i_084304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8430A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8430A);
    cpu.pc = 0x2870A;
    16
}
/// $08430A: bcc.w $84320
pub fn i_08430a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8430E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x84320; return 10; }
    8
}
/// $08430E: lea.l $5e766.l, a0
pub fn i_08430e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84314;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $084314: jsr $5e770.l
pub fn i_084314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8431A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8431A);
    cpu.pc = 0x5E770;
    16
}
/// $08431A: bclr.b #$3, $13(a6)
pub fn i_08431a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84320;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $084320: jsr $28758.l
pub fn i_084320(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84326;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84326);
    cpu.pc = 0x28758;
    16
}
/// $084326: bcc.w $84384
pub fn i_084326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8432A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x84384; return 10; }
    8
}
/// $08432A: move.l #$1000, d0
pub fn i_08432a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84330;
    cpu.ir = 0x203C;
    let v1: u32 = 0x1000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $084330: jsr $51a28.l
pub fn i_084330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84336;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84336);
    cpu.pc = 0x51A28;
    16
}
/// $084336: move.w #$1030, d0
pub fn i_084336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8433A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1030;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08433A: jsr $2352.l
pub fn i_08433a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84340;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84340);
    cpu.pc = 0x2352;
    16
}
/// $084340: lea.l $ffff.w, a0
pub fn i_084340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84344;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $084344: move.l a0, $48(a6)
pub fn i_084344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84348;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $084348: jsr $8640e(pc)
pub fn i_084348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8434C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8434C);
    cpu.pc = 0x8640E;
    16
}
/// $08434C: jsr $8641c(pc)
pub fn i_08434c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84350;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x84350);
    cpu.pc = 0x8641C;
    16
}
/// $084350: lea.l $2e9b00.l, a1
pub fn i_084350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84356;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E9B00;
    20
}
/// $084356: jsr $77c7e.l
pub fn i_084356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8435C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8435C);
    cpu.pc = 0x77C7E;
    16
}
/// $08435C: lea.l $2e9aca.l, a1
pub fn i_08435c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84362;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E9ACA;
    20
}
/// $084362: jsr $77c7e.l
pub fn i_084362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84368;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84368);
    cpu.pc = 0x77C7E;
    16
}
/// $084368: lea.l $2eacb8.l, a1
pub fn i_084368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8436E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EACB8;
    20
}
/// $08436E: jsr $43fac.l
pub fn i_08436e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84374;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84374);
    cpu.pc = 0x43FAC;
    16
}
/// $084374: lea.l $2eae76.l, a1
pub fn i_084374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8437A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EAE76;
    20
}
/// $08437A: jsr $43fac.l
pub fn i_08437a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84380;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84380);
    cpu.pc = 0x43FAC;
    16
}
/// $084380: bra.w $8438e
pub fn i_084380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84384;
    cpu.ir = 0x6000;
    cpu.pc = 0x8438E;
    10
}
/// $084384: jsr $4fa70.l
pub fn i_084384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8438A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8438A);
    cpu.pc = 0x4FA70;
    16
}
/// $08438A: bcc.w $84394
pub fn i_08438a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8438E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x84394; return 10; }
    8
}
/// $08438E: jmp $518.l
pub fn i_08438e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84394;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $084394: rts
pub fn i_084394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84396;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_084396  ($084396, 114 B, enemies/bosses/miniboss_finale_083be2.s)
/// $084396: movea.l $3c(a6), a1
pub fn i_084396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8439A;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08439A: jsr $2942a.l
pub fn i_08439a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x843A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x843A0);
    cpu.pc = 0x2942A;
    16
}
/// $0843A0: move.w #$a9, d1
pub fn i_0843a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x843A4;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0843A4: jsr $236e.l
pub fn i_0843a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x843AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x843AA);
    cpu.pc = 0x236E;
    16
}
/// $0843AA: move.w #$1, $38(a6)
pub fn i_0843aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x843B0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0843B0: jsr $860e4(pc)
pub fn i_0843b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x843B4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x843B4);
    cpu.pc = 0x860E4;
    16
}
/// $0843B4: jsr $86300(pc)
pub fn i_0843b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x843B8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x843B8);
    cpu.pc = 0x86300;
    16
}
/// $0843B8: lea.l $8495e(pc), a1
pub fn i_0843b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x843BC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8495E;
    16
}
/// $0843BC: jsr $4ae.l
pub fn i_0843bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x843C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x843C2);
    cpu.pc = 0x4AE;
    16
}
/// $0843C2: jsr $5dd22.l
pub fn i_0843c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x843C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x843C8);
    cpu.pc = 0x5DD22;
    16
}
/// $0843C8: move.b #$0, $21(a6)
pub fn i_0843c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x843CE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0843CE: lea.l $2e7048.l, a0
pub fn i_0843ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x843D4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E7048;
    20
}
/// $0843D4: jsr $28cd4.l
pub fn i_0843d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x843DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x843DA);
    cpu.pc = 0x28CD4;
    16
}
/// $0843DA: lea.l $843e0(pc), a1
pub fn i_0843da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x843DE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x843E0;
    16
}
/// $0843DE: move.l a1, (a6)
pub fn i_0843de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x843E0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0843E0: clr.b $10e39a.l
pub fn i_0843e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x843E6;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $0843E6: jsr $2783a.l
pub fn i_0843e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x843EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x843EC);
    cpu.pc = 0x2783A;
    16
}
/// $0843EC: jsr $28d70.l
pub fn i_0843ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x843F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x843F2);
    cpu.pc = 0x28D70;
    16
}
/// $0843F2: move.w #$7fff, $66(a6)
pub fn i_0843f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x843F8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7FFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0843F8: cmpi.b #$ff, $21(a6)
pub fn i_0843f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x843FE;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $0843FE: bne.w $8440e
pub fn i_0843fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84402;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8440E; return 10; }
    8
}
/// $084402: move.b #$ff, $74(a6)
pub fn i_084402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84408;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== SetTaskHandler_084408  ($084408, 8 B, kernel/task_handlers.c)
/// $084408: lea.l $84410(pc), a1
pub fn i_084408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8440C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84410;
    16
}
/// $08440C: move.l a1, (a6)
pub fn i_08440c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8440E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08440E: rts
pub fn i_08440e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84410;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_084410  ($084410, 168 B, enemies/bosses/miniboss_finale_083be2.s)
/// $084410: move.b #$3, $10e39c.l
pub fn i_084410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84418;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E39C, v1 as u8);
    16
}
/// $084418: lea.l $845b8(pc), a1
pub fn i_084418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8441C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x845B8;
    16
}
/// $08441C: jsr $4ae.l
pub fn i_08441c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84422;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84422);
    cpu.pc = 0x4AE;
    16
}
/// $084422: move.w #$a, $72(a0)
pub fn i_084422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84428;
    cpu.ir = 0x317C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $084428: lea.l $ffff.w, a0
pub fn i_084428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8442C;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $08442C: move.l a0, $48(a6)
pub fn i_08442c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84430;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $084430: move.w #$1033, d0
pub fn i_084430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84434;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1033;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $084434: jsr $2352.l
pub fn i_084434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8443A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8443A);
    cpu.pc = 0x2352;
    16
}
/// $08443A: move.w #$27, d1
pub fn i_08443a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8443E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x27;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08443E: move.w #$129, d2
pub fn i_08443e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84442;
    cpu.ir = 0x343C;
    let v1: u32 = 0x129;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $084442: move.w #$2, d3
pub fn i_084442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84446;
    cpu.ir = 0x363C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $084446: move.w #$1, d4
pub fn i_084446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8444A;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08444A: jsr $2c26.l
pub fn i_08444a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84450;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84450);
    cpu.pc = 0x2C26;
    16
}
/// $084450: lea.l $86586.l, a1
pub fn i_084450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84456;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x86586;
    20
}
/// $084456: jsr $4ae.l
pub fn i_084456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8445C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8445C);
    cpu.pc = 0x4AE;
    16
}
/// $08445C: lea.l $2ea790.l, a2
pub fn i_08445c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84462;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EA790;
    20
}
/// $084462: jsr $5022a.l
pub fn i_084462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84468;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84468);
    cpu.pc = 0x5022A;
    16
}
/// $084468: jsr $86196(pc)
pub fn i_084468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8446C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8446C);
    cpu.pc = 0x86196;
    16
}
/// $08446C: lea.l $2e98d8.l, a1
pub fn i_08446c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84472;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E98D8;
    20
}
/// $084472: jsr $77c7e.l
pub fn i_084472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84478;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84478);
    cpu.pc = 0x77C7E;
    16
}
/// $084478: lea.l $2e98ea.l, a1
pub fn i_084478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8447E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E98EA;
    20
}
/// $08447E: jsr $77c7e.l
pub fn i_08447e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84484;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84484);
    cpu.pc = 0x77C7E;
    16
}
/// $084484: move.w #$c000, $38(a0)
pub fn i_084484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8448A;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08448A: jsr $8610c(pc)
pub fn i_08448a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8448E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8448E);
    cpu.pc = 0x8610C;
    16
}
/// $08448E: move.b #$0, $21(a6)
pub fn i_08448e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84494;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $084494: lea.l $8449a(pc), a1
pub fn i_084494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84498;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8449A;
    16
}
/// $084498: move.l a1, (a6)
pub fn i_084498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8449A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08449A: clr.b $10e39a.l
pub fn i_08449a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x844A0;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $0844A0: jsr $2783a.l
pub fn i_0844a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x844A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x844A6);
    cpu.pc = 0x2783A;
    16
}
/// $0844A6: cmpi.b #$9, $21(a6)
pub fn i_0844a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x844AC;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x9, Size::Byte, false);
    16
}
/// $0844AC: blt.w $844be
pub fn i_0844ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x844B0;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x844BE; return 10; }
    8
}
/// $0844B0: move.b #$2, $10e39a.l
pub fn i_0844b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x844B8;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E39A, v1 as u8);
    16
}
// ===== SetTaskHandler_0844b8  ($0844B8, 8 B, kernel/task_handlers.c)
/// $0844B8: lea.l $844c0(pc), a1
pub fn i_0844b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x844BC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x844C0;
    16
}
/// $0844BC: move.l a1, (a6)
pub fn i_0844bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x844BE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0844BE: rts
pub fn i_0844be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x844C0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_0844c0  ($0844C0, 68 B, enemies/bosses/miniboss_finale_083be2.s)
/// $0844C0: jsr $2783a.l
pub fn i_0844c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x844C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x844C6);
    cpu.pc = 0x2783A;
    16
}
/// $0844C6: lea.l $2eacd4.l, a1
pub fn i_0844c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x844CC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EACD4;
    20
}
/// $0844CC: jsr $43fac.l
pub fn i_0844cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x844D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x844D2);
    cpu.pc = 0x43FAC;
    16
}
/// $0844D2: lea.l $2e7486.l, a0
pub fn i_0844d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x844D8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E7486;
    20
}
/// $0844D8: jsr $28cd4.l
pub fn i_0844d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x844DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x844DE);
    cpu.pc = 0x28CD4;
    16
}
/// $0844DE: lea.l $844e4(pc), a1
pub fn i_0844de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x844E2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x844E4;
    16
}
/// $0844E2: move.l a1, (a6)
pub fn i_0844e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x844E4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0844E4: clr.b $10e39a.l
pub fn i_0844e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x844EA;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $0844EA: jsr $2783a.l
pub fn i_0844ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x844F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x844F0);
    cpu.pc = 0x2783A;
    16
}
/// $0844F0: jsr $28d70.l
pub fn i_0844f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x844F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x844F6);
    cpu.pc = 0x28D70;
    16
}
/// $0844F6: bcc.w $8450a
pub fn i_0844f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x844FA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8450A; return 10; }
    8
}
/// $0844FA: jsr $6f0.l
pub fn i_0844fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84500;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84500);
    cpu.pc = 0x6F0;
    16
}
/// $084500: bcs.w $8450a
pub fn i_084500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84504;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8450A; return 10; }
    8
}
// ===== SetTaskHandler_084504  ($084504, 8 B, kernel/task_handlers.c)
/// $084504: lea.l $8450c(pc), a1
pub fn i_084504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84508;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8450C;
    16
}
/// $084508: move.l a1, (a6)
pub fn i_084508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8450A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08450A: rts
pub fn i_08450a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8450C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_08450c  ($08450C, 20 B, enemies/bosses/miniboss_finale_083be2.s)
/// $08450C: clr.b $10e39c.l
pub fn i_08450c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84512;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39C) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39C, r2 as u8);
    16
}
/// $084512: jsr $320d4.l
pub fn i_084512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84518;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84518);
    cpu.pc = 0x320D4;
    16
}
/// $084518: jmp $518.l
pub fn i_084518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8451E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== TaskHandler_084520  ($084520, 152 B, enemies/bosses/miniboss_finale_083be2.s)
/// $084520: move.w #$100, $70(a6)
pub fn i_084520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84526;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $084526: lea.l $2e9256.l, a0
pub fn i_084526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8452C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E9256;
    20
}
/// $08452C: move.l a0, $48(a6)
pub fn i_08452c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84530;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $084530: lea.l $84536(pc), a1
pub fn i_084530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84534;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84536;
    16
}
/// $084534: move.l a1, (a6)
pub fn i_084534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84536;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084536: jsr $2783a.l
pub fn i_084536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8453C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8453C);
    cpu.pc = 0x2783A;
    16
}
/// $08453C: move.w #$7fff, $66(a6)
pub fn i_08453c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84542;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7FFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $084542: jsr $2870a.l
pub fn i_084542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84548;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84548);
    cpu.pc = 0x2870A;
    16
}
/// $084548: bcc.w $84596
pub fn i_084548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8454C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x84596; return 10; }
    8
}
/// $08454C: lea.l $5e766.l, a0
pub fn i_08454c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84552;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $084552: jsr $5e770.l
pub fn i_084552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84558;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84558);
    cpu.pc = 0x5E770;
    16
}
/// $084558: bclr.b #$3, $13(a6)
pub fn i_084558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8455E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08455E: move.b $106f28.l, d0
pub fn i_08455e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84564;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $084564: cmpi.b #$1, $58(a6)
pub fn i_084564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8456A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $08456A: beq.w $8457c
pub fn i_08456a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8456E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8457C; return 10; }
    8
}
/// $08456E: cmpi.b #$2, $58(a6)
pub fn i_08456e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84574;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $084574: beq.w $8457c
pub fn i_084574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84578;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8457C; return 10; }
    8
}
/// $084578: bra.w $84596
pub fn i_084578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8457C;
    cpu.ir = 0x6000;
    cpu.pc = 0x84596;
    10
}
/// $08457C: btst.b #$0, d0
pub fn i_08457c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84580;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $084580: bne.w $84596
pub fn i_084580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84584;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x84596; return 10; }
    8
}
/// $084584: lea.l $53886.l, a1
pub fn i_084584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8458A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x53886;
    20
}
/// $08458A: jsr $4ae.l
pub fn i_08458a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84590;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84590);
    cpu.pc = 0x4AE;
    16
}
/// $084590: jsr $5dd22.l
pub fn i_084590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84596;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84596);
    cpu.pc = 0x5DD22;
    16
}
/// $084596: jsr $28758.l
pub fn i_084596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8459C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8459C);
    cpu.pc = 0x28758;
    16
}
/// $08459C: bcc.w $845a6
pub fn i_08459c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x845A0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x845A6; return 10; }
    8
}
/// $0845A0: bclr.b #$0, $13(a6)
pub fn i_0845a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x845A6;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0845A6: jsr $4fa70.l
pub fn i_0845a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x845AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x845AC);
    cpu.pc = 0x4FA70;
    16
}
/// $0845AC: bcc.w $845b6
pub fn i_0845ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x845B0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x845B6; return 10; }
    8
}
/// $0845B0: jmp $518.l
pub fn i_0845b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x845B6;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0845B6: rts
pub fn i_0845b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x845B8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_0845b8  ($0845B8, 44 B, enemies/bosses/miniboss_finale_083be2.s)
/// $0845B8: move.b #$1, $10a2d1.l
pub fn i_0845b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x845C0;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2D1, v1 as u8);
    16
}
/// $0845C0: lea.l $845c6(pc), a1
pub fn i_0845c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x845C4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x845C6;
    16
}
/// $0845C4: move.l a1, (a6)
pub fn i_0845c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x845C6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0845C6: subq.w #$1, $72(a6)
pub fn i_0845c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x845CA;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0845CA: beq.w $845e4
pub fn i_0845ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x845CE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x845E4; return 10; }
    8
}
/// $0845CE: move.w $72(a6), d0
pub fn i_0845ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x845D2;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0845D2: andi.w #$1, d0
pub fn i_0845d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x845D6;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0845D6: beq.w $845e2
pub fn i_0845d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x845DA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x845E2; return 10; }
    8
}
/// $0845DA: move.b #$1, $10a2d1.l
pub fn i_0845da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x845E2;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2D1, v1 as u8);
    16
}
/// $0845E2: rts
pub fn i_0845e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x845E4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_0845e4  ($0845E4, 8 B, kernel/jmp_to_scheduler.c)
/// $0845E4: jmp $518.l
pub fn i_0845e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x845EA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== TaskHandler_0845ec  ($0845EC, 76 B, enemies/bosses/miniboss_finale_083be2.s)
/// $0845EC: lea.l $845f2(pc), a1
pub fn i_0845ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x845F0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x845F2;
    16
}
/// $0845F0: move.l a1, (a6)
pub fn i_0845f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x845F2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0845F2: movea.l $c(a6), a0
pub fn i_0845f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x845F6;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0845F6: btst.b #$3, $13(a0)
pub fn i_0845f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x845FC;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0845FC: beq.w $84622
pub fn i_0845fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84600;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x84622; return 10; }
    8
}
/// $084600: move.b $20(a0), d0
pub fn i_084600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84604;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $084604: cmp.b $21(a6), d0
pub fn i_084604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84608;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $084608: bne.w $84622
pub fn i_084608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8460C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x84622; return 10; }
    8
}
/// $08460C: bclr.b #$3, $13(a0)
pub fn i_08460c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84612;
    cpu.ir = 0x8A8;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $084612: move.b $58(a0), $58(a6)
pub fn i_084612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84618;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x58);
    bus.write8(ad4, v3 as u8);
    20
}
/// $084618: jsr $8651e(pc)
pub fn i_084618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8461C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8461C);
    cpu.pc = 0x8651E;
    16
}
/// $08461C: lea.l $84638(pc), a1
pub fn i_08461c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84620;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84638;
    16
}
/// $084620: move.l a1, (a6)
pub fn i_084620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84622;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084622: movea.l $c(a6), a0
pub fn i_084622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84626;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $084626: cmpi.b #$ff, $21(a0)
pub fn i_084626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8462C;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $08462C: bne.w $84636
pub fn i_08462c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84630;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x84636; return 10; }
    8
}
/// $084630: jmp $518.l
pub fn i_084630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84636;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $084636: rts
pub fn i_084636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84638;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_084638  ($084638, 98 B, enemies/bosses/miniboss_finale_083be2.s)
/// $084638: move.l #$1000, d0
pub fn i_084638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8463E;
    cpu.ir = 0x203C;
    let v1: u32 = 0x1000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08463E: jsr $51a28.l
pub fn i_08463e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84644;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84644);
    cpu.pc = 0x51A28;
    16
}
/// $084644: move.w #$108d, d0
pub fn i_084644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84648;
    cpu.ir = 0x303C;
    let v1: u32 = 0x108D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $084648: jsr $2352.l
pub fn i_084648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8464E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8464E);
    cpu.pc = 0x2352;
    16
}
/// $08464E: lea.l $84654(pc), a1
pub fn i_08464e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84652;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84654;
    16
}
/// $084652: move.l a1, (a6)
pub fn i_084652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84654;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084654: movea.l $c(a6), a0
pub fn i_084654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84658;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $084658: btst.b #$3, $13(a0)
pub fn i_084658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8465E;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $08465E: beq.w $84684
pub fn i_08465e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84662;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x84684; return 10; }
    8
}
/// $084662: move.b $20(a0), d0
pub fn i_084662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84666;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $084666: cmp.b $21(a6), d0
pub fn i_084666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8466A;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $08466A: bne.w $84684
pub fn i_08466a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8466E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x84684; return 10; }
    8
}
/// $08466E: bclr.b #$3, $13(a0)
pub fn i_08466e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84674;
    cpu.ir = 0x8A8;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $084674: move.b $58(a0), $58(a6)
pub fn i_084674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8467A;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x58);
    bus.write8(ad4, v3 as u8);
    20
}
/// $08467A: jsr $86538(pc)
pub fn i_08467a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8467E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8467E);
    cpu.pc = 0x86538;
    16
}
/// $08467E: lea.l $8469a(pc), a1
pub fn i_08467e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84682;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8469A;
    16
}
/// $084682: move.l a1, (a6)
pub fn i_084682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84684;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084684: movea.l $c(a6), a0
pub fn i_084684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84688;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $084688: cmpi.b #$ff, $21(a0)
pub fn i_084688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8468E;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $08468E: bne.w $84698
pub fn i_08468e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84692;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x84698; return 10; }
    8
}
/// $084692: jmp $518.l
pub fn i_084692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84698;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $084698: rts
pub fn i_084698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8469A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_08469a  ($08469A, 70 B, enemies/bosses/miniboss_finale_083be2.s)
/// $08469A: move.w #$108d, d0
pub fn i_08469a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8469E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x108D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08469E: jsr $2352.l
pub fn i_08469e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x846A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x846A4);
    cpu.pc = 0x2352;
    16
}
/// $0846A4: lea.l $846aa(pc), a1
pub fn i_0846a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x846A8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x846AA;
    16
}
/// $0846A8: move.l a1, (a6)
pub fn i_0846a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x846AA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0846AA: movea.l $c(a6), a0
pub fn i_0846aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x846AE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0846AE: btst.b #$3, $13(a0)
pub fn i_0846ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x846B4;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0846B4: beq.w $846ca
pub fn i_0846b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x846B8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x846CA; return 10; }
    8
}
/// $0846B8: move.b $20(a0), d0
pub fn i_0846b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x846BC;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0846BC: cmp.b $21(a6), d0
pub fn i_0846bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x846C0;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $0846C0: bne.w $846ca
pub fn i_0846c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x846C4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x846CA; return 10; }
    8
}
/// $0846C4: bclr.b #$3, $13(a0)
pub fn i_0846c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x846CA;
    cpu.ir = 0x8A8;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0846CA: movea.l $c(a6), a0
pub fn i_0846ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x846CE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0846CE: cmpi.b #$ff, $21(a0)
pub fn i_0846ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x846D4;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $0846D4: bne.w $846de
pub fn i_0846d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x846D8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x846DE; return 10; }
    8
}
/// $0846D8: jmp $518.l
pub fn i_0846d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x846DE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0846DE: rts
pub fn i_0846de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x846E0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_0846e0  ($0846E0, 328 B, enemies/bosses/miniboss_finale_083be2.s)
/// $0846E0: move.w #$a9, d1
pub fn i_0846e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x846E4;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0846E4: jsr $236e.l
pub fn i_0846e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x846EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x846EA);
    cpu.pc = 0x236E;
    16
}
/// $0846EA: move.b #$ff, $32(a6)
pub fn i_0846ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x846F0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0846F0: move.b #$ff, $33(a6)
pub fn i_0846f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x846F6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0846F6: move.b #$0, $3a(a6)
pub fn i_0846f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x846FC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0846FC: addi.w #$80, $24(a6)
pub fn i_0846fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84702;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x80, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $084702: addi.w #$29, $22(a6)
pub fn i_084702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84708;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x29, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $084708: cmpi.b #$0, $21(a6)
pub fn i_084708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8470E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08470E: beq.w $84720
pub fn i_08470e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84712;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x84720; return 10; }
    8
}
/// $084712: move.b $21(a6), d0
pub fn i_084712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84716;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $084716: addi.w #$40, $22(a6)
pub fn i_084716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8471C;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08471C: subq.b #$1, d0
pub fn i_08471c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8471E;
    cpu.ir = 0x5300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $08471E: bne.b $84716
pub fn i_08471e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84720;
    cpu.ir = 0x66F6;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x84716; return 10; }
    8
}
/// $084720: move.w #$14, $66(a6)
pub fn i_084720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84726;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $084726: clr.l d0
pub fn i_084726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84728;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $084728: move.b $21(a6), d0
pub fn i_084728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8472C;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08472C: movea.l #$2e7556, a0
pub fn i_08472c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84732;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2E7556;
    cpu.a[0] = v1;
    4
}
/// $084732: lsl.w #$2, d0
pub fn i_084732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84734;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $084734: movea.l (a0, d0.w), a0
pub fn i_084734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84738;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $084738: cmpa.l #$ffffffff, a0
pub fn i_084738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8473E;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x082B3E, i_082b3e),
    (0x082B42, i_082b42),
    (0x082B44, i_082b44),
    (0x082B46, i_082b46),
    (0x082B4A, i_082b4a),
    (0x082B50, i_082b50),
    (0x082B54, i_082b54),
    (0x082B5A, i_082b5a),
    (0x082B60, i_082b60),
    (0x082B66, i_082b66),
    (0x082B6C, i_082b6c),
    (0x082B72, i_082b72),
    (0x082B78, i_082b78),
    (0x082B7E, i_082b7e),
    (0x082B82, i_082b82),
    (0x082B84, i_082b84),
    (0x082B8A, i_082b8a),
    (0x082B90, i_082b90),
    (0x082B94, i_082b94),
    (0x082B98, i_082b98),
    (0x082B9A, i_082b9a),
    (0x082BA0, i_082ba0),
    (0x082BA6, i_082ba6),
    (0x082BAC, i_082bac),
    (0x082BB0, i_082bb0),
    (0x082BB4, i_082bb4),
    (0x082BB6, i_082bb6),
    (0x082BB8, i_082bb8),
    (0x082BBC, i_082bbc),
    (0x082BC2, i_082bc2),
    (0x082BC8, i_082bc8),
    (0x082BCE, i_082bce),
    (0x082BD4, i_082bd4),
    (0x082BDA, i_082bda),
    (0x082BDE, i_082bde),
    (0x082BE0, i_082be0),
    (0x082BE6, i_082be6),
    (0x082BEC, i_082bec),
    (0x082BF0, i_082bf0),
    (0x082BF4, i_082bf4),
    (0x082BF6, i_082bf6),
    (0x082BFC, i_082bfc),
    (0x082C00, i_082c00),
    (0x082C04, i_082c04),
    (0x082C06, i_082c06),
    (0x082C08, i_082c08),
    (0x082C0E, i_082c0e),
    (0x082C12, i_082c12),
    (0x082C16, i_082c16),
    (0x082C18, i_082c18),
    (0x082C1E, i_082c1e),
    (0x082C22, i_082c22),
    (0x082C28, i_082c28),
    (0x082C2C, i_082c2c),
    (0x082C32, i_082c32),
    (0x082C34, i_082c34),
    (0x082C38, i_082c38),
    (0x082C3E, i_082c3e),
    (0x082C42, i_082c42),
    (0x082C48, i_082c48),
    (0x082C4E, i_082c4e),
    (0x082C74, i_082c74),
    (0x082C7A, i_082c7a),
    (0x082C7C, i_082c7c),
    (0x082C80, i_082c80),
    (0x082C84, i_082c84),
    (0x082C88, i_082c88),
    (0x082C8A, i_082c8a),
    (0x082C90, i_082c90),
    (0x082C94, i_082c94),
    (0x082C98, i_082c98),
    (0x082C9C, i_082c9c),
    (0x082CA2, i_082ca2),
    (0x082CA6, i_082ca6),
    (0x082CAA, i_082caa),
    (0x082CAC, i_082cac),
    (0x082CB0, i_082cb0),
    (0x082CB2, i_082cb2),
    (0x082CB6, i_082cb6),
    (0x082CBC, i_082cbc),
    (0x082CC2, i_082cc2),
    (0x082CC4, i_082cc4),
    (0x082CC8, i_082cc8),
    (0x082CCE, i_082cce),
    (0x082CD4, i_082cd4),
    (0x082CDA, i_082cda),
    (0x082CE0, i_082ce0),
    (0x082CE2, i_082ce2),
    (0x082CE6, i_082ce6),
    (0x082CEC, i_082cec),
    (0x082CF2, i_082cf2),
    (0x082CF6, i_082cf6),
    (0x082CFA, i_082cfa),
    (0x082CFE, i_082cfe),
    (0x082D02, i_082d02),
    (0x082D06, i_082d06),
    (0x082D0A, i_082d0a),
    (0x082D10, i_082d10),
    (0x082D12, i_082d12),
    (0x082D18, i_082d18),
    (0x082D1C, i_082d1c),
    (0x082D22, i_082d22),
    (0x082D28, i_082d28),
    (0x082D2E, i_082d2e),
    (0x082D34, i_082d34),
    (0x082D38, i_082d38),
    (0x082D3E, i_082d3e),
    (0x082D40, i_082d40),
    (0x082D46, i_082d46),
    (0x082D4A, i_082d4a),
    (0x082D50, i_082d50),
    (0x082D54, i_082d54),
    (0x082D5A, i_082d5a),
    (0x082D5E, i_082d5e),
    (0x082D64, i_082d64),
    (0x082D68, i_082d68),
    (0x082D6E, i_082d6e),
    (0x082D70, i_082d70),
    (0x082D74, i_082d74),
    (0x082D78, i_082d78),
    (0x082D7C, i_082d7c),
    (0x082D80, i_082d80),
    (0x082D84, i_082d84),
    (0x082D8A, i_082d8a),
    (0x082D8E, i_082d8e),
    (0x082D92, i_082d92),
    (0x082D98, i_082d98),
    (0x082D9E, i_082d9e),
    (0x082DA2, i_082da2),
    (0x082DA6, i_082da6),
    (0x082DAC, i_082dac),
    (0x082DAE, i_082dae),
    (0x082DB4, i_082db4),
    (0x082DB8, i_082db8),
    (0x082DBE, i_082dbe),
    (0x082DC2, i_082dc2),
    (0x082DC8, i_082dc8),
    (0x082DCC, i_082dcc),
    (0x082DD0, i_082dd0),
    (0x082DD4, i_082dd4),
    (0x082DD8, i_082dd8),
    (0x082DDC, i_082ddc),
    (0x082DE0, i_082de0),
    (0x082DE4, i_082de4),
    (0x082DE8, i_082de8),
    (0x082DEC, i_082dec),
    (0x082DEE, i_082dee),
    (0x082DF2, i_082df2),
    (0x082DF6, i_082df6),
    (0x082DF8, i_082df8),
    (0x082DFE, i_082dfe),
    (0x082E02, i_082e02),
    (0x082E04, i_082e04),
    (0x082E0A, i_082e0a),
    (0x082E0E, i_082e0e),
    (0x082E14, i_082e14),
    (0x082E18, i_082e18),
    (0x082E1E, i_082e1e),
    (0x082E22, i_082e22),
    (0x082E26, i_082e26),
    (0x082E2A, i_082e2a),
    (0x082E2E, i_082e2e),
    (0x082E32, i_082e32),
    (0x082E38, i_082e38),
    (0x082E3C, i_082e3c),
    (0x082E40, i_082e40),
    (0x082E44, i_082e44),
    (0x082E48, i_082e48),
    (0x082E4C, i_082e4c),
    (0x082E50, i_082e50),
    (0x082E56, i_082e56),
    (0x082E5A, i_082e5a),
    (0x082E5E, i_082e5e),
    (0x082E62, i_082e62),
    (0x082E66, i_082e66),
    (0x082E6A, i_082e6a),
    (0x082E6E, i_082e6e),
    (0x082E72, i_082e72),
    (0x082E74, i_082e74),
    (0x082E78, i_082e78),
    (0x082E7A, i_082e7a),
    (0x082E7E, i_082e7e),
    (0x082E82, i_082e82),
    (0x082E86, i_082e86),
    (0x082E8C, i_082e8c),
    (0x082E90, i_082e90),
    (0x082E94, i_082e94),
    (0x082E98, i_082e98),
    (0x082E9E, i_082e9e),
    (0x082EA2, i_082ea2),
    (0x082EA8, i_082ea8),
    (0x082EAC, i_082eac),
    (0x082EB2, i_082eb2),
    (0x082EB6, i_082eb6),
    (0x082EBC, i_082ebc),
    (0x082EC0, i_082ec0),
    (0x082EC4, i_082ec4),
    (0x082ECA, i_082eca),
    (0x082ECE, i_082ece),
    (0x082ED4, i_082ed4),
    (0x082ED8, i_082ed8),
    (0x082EDA, i_082eda),
    (0x082EDE, i_082ede),
    (0x082EE0, i_082ee0),
    (0x082EE4, i_082ee4),
    (0x082EE8, i_082ee8),
    (0x082EEC, i_082eec),
    (0x082EF0, i_082ef0),
    (0x082EF2, i_082ef2),
    (0x082EF6, i_082ef6),
    (0x082EF8, i_082ef8),
    (0x082EFC, i_082efc),
    (0x082F02, i_082f02),
    (0x082F06, i_082f06),
    (0x082F0A, i_082f0a),
    (0x082F10, i_082f10),
    (0x082F14, i_082f14),
    (0x082F16, i_082f16),
    (0x082F18, i_082f18),
    (0x082F1C, i_082f1c),
    (0x082F1E, i_082f1e),
    (0x082F22, i_082f22),
    (0x082F24, i_082f24),
    (0x082F28, i_082f28),
    (0x082F2E, i_082f2e),
    (0x082F34, i_082f34),
    (0x082F3A, i_082f3a),
    (0x082F3E, i_082f3e),
    (0x082F40, i_082f40),
    (0x082F44, i_082f44),
    (0x082F48, i_082f48),
    (0x082F4C, i_082f4c),
    (0x082F50, i_082f50),
    (0x082F54, i_082f54),
    (0x082F56, i_082f56),
    (0x082F58, i_082f58),
    (0x082F5C, i_082f5c),
    (0x082F60, i_082f60),
    (0x082F64, i_082f64),
    (0x082F66, i_082f66),
    (0x082F6A, i_082f6a),
    (0x082F6E, i_082f6e),
    (0x082F70, i_082f70),
    (0x082F74, i_082f74),
    (0x082F78, i_082f78),
    (0x082F7C, i_082f7c),
    (0x082F7E, i_082f7e),
    (0x082F80, i_082f80),
    (0x082F84, i_082f84),
    (0x082F8A, i_082f8a),
    (0x082F8E, i_082f8e),
    (0x082F90, i_082f90),
    (0x082F92, i_082f92),
    (0x082F98, i_082f98),
    (0x082F9C, i_082f9c),
    (0x082FA0, i_082fa0),
    (0x082FA4, i_082fa4),
    (0x082FAA, i_082faa),
    (0x082FAE, i_082fae),
    (0x082FB4, i_082fb4),
    (0x082FB6, i_082fb6),
    (0x082FBC, i_082fbc),
    (0x082FC0, i_082fc0),
    (0x082FC4, i_082fc4),
    (0x082FC8, i_082fc8),
    (0x082FCA, i_082fca),
    (0x082FD0, i_082fd0),
    (0x082FD6, i_082fd6),
    (0x082FDC, i_082fdc),
    (0x082FE0, i_082fe0),
    (0x082FE2, i_082fe2),
    (0x082FE8, i_082fe8),
    (0x082FEC, i_082fec),
    (0x082FF0, i_082ff0),
    (0x082FF4, i_082ff4),
    (0x082FFA, i_082ffa),
    (0x082FFE, i_082ffe),
    (0x083000, i_083000),
    (0x083004, i_083004),
    (0x083008, i_083008),
    (0x08300E, i_08300e),
    (0x083014, i_083014),
    (0x083018, i_083018),
    (0x08301A, i_08301a),
    (0x08301C, i_08301c),
    (0x083022, i_083022),
    (0x083026, i_083026),
    (0x08302A, i_08302a),
    (0x08302E, i_08302e),
    (0x083034, i_083034),
    (0x083036, i_083036),
    (0x083038, i_083038),
    (0x08303A, i_08303a),
    (0x08303C, i_08303c),
    (0x083042, i_083042),
    (0x083048, i_083048),
    (0x08304C, i_08304c),
    (0x083052, i_083052),
    (0x083056, i_083056),
    (0x083058, i_083058),
    (0x08305A, i_08305a),
    (0x08305E, i_08305e),
    (0x083062, i_083062),
    (0x083064, i_083064),
    (0x083068, i_083068),
    (0x08306E, i_08306e),
    (0x083074, i_083074),
    (0x083078, i_083078),
    (0x08307A, i_08307a),
    (0x083080, i_083080),
    (0x083082, i_083082),
    (0x083086, i_083086),
    (0x08308A, i_08308a),
    (0x08308E, i_08308e),
    (0x083094, i_083094),
    (0x083096, i_083096),
    (0x083098, i_083098),
    (0x08309A, i_08309a),
    (0x08309C, i_08309c),
    (0x0830A2, i_0830a2),
    (0x0830A8, i_0830a8),
    (0x0830AC, i_0830ac),
    (0x0830B2, i_0830b2),
    (0x0830B6, i_0830b6),
    (0x0830B8, i_0830b8),
    (0x0830BA, i_0830ba),
    (0x0830BE, i_0830be),
    (0x0830C2, i_0830c2),
    (0x0830C6, i_0830c6),
    (0x0830CC, i_0830cc),
    (0x0830D0, i_0830d0),
    (0x0830D6, i_0830d6),
    (0x0830DC, i_0830dc),
    (0x0830E0, i_0830e0),
    (0x0830E4, i_0830e4),
    (0x0830E6, i_0830e6),
    (0x0830E8, i_0830e8),
    (0x0830EC, i_0830ec),
    (0x0830F0, i_0830f0),
    (0x0830F4, i_0830f4),
    (0x0830F8, i_0830f8),
    (0x0830FC, i_0830fc),
    (0x0830FE, i_0830fe),
    (0x083104, i_083104),
    (0x083108, i_083108),
    (0x08310C, i_08310c),
    (0x083110, i_083110),
    (0x083116, i_083116),
    (0x08311A, i_08311a),
    (0x08311C, i_08311c),
    (0x083120, i_083120),
    (0x083122, i_083122),
    (0x083128, i_083128),
    (0x08312C, i_08312c),
    (0x083130, i_083130),
    (0x083136, i_083136),
    (0x08313C, i_08313c),
    (0x083140, i_083140),
    (0x083144, i_083144),
    (0x083148, i_083148),
    (0x08314E, i_08314e),
    (0x083154, i_083154),
    (0x083156, i_083156),
    (0x083158, i_083158),
    (0x08315E, i_08315e),
    (0x083162, i_083162),
    (0x083164, i_083164),
    (0x083168, i_083168),
    (0x08316E, i_08316e),
    (0x083174, i_083174),
    (0x08317A, i_08317a),
    (0x08317E, i_08317e),
    (0x083182, i_083182),
    (0x083188, i_083188),
    (0x08318E, i_08318e),
    (0x083194, i_083194),
    (0x08319A, i_08319a),
    (0x08319E, i_08319e),
    (0x0831A4, i_0831a4),
    (0x0831AA, i_0831aa),
    (0x0831B0, i_0831b0),
    (0x0831B6, i_0831b6),
    (0x0831B8, i_0831b8),
    (0x0831C0, i_0831c0),
    (0x0831C2, i_0831c2),
    (0x0831C6, i_0831c6),
    (0x0831CC, i_0831cc),
    (0x0831D2, i_0831d2),
    (0x0831D8, i_0831d8),
    (0x0831DA, i_0831da),
    (0x0831E0, i_0831e0),
    (0x0831E4, i_0831e4),
    (0x0831E8, i_0831e8),
    (0x0831EA, i_0831ea),
    (0x0831EE, i_0831ee),
    (0x0831F4, i_0831f4),
    (0x0831FA, i_0831fa),
    (0x083200, i_083200),
    (0x083204, i_083204),
    (0x083208, i_083208),
    (0x08320A, i_08320a),
    (0x08320E, i_08320e),
    (0x083214, i_083214),
    (0x08321A, i_08321a),
    (0x08321C, i_08321c),
    (0x083222, i_083222),
    (0x083228, i_083228),
    (0x08322E, i_08322e),
    (0x083234, i_083234),
    (0x083238, i_083238),
    (0x08323C, i_08323c),
    (0x083240, i_083240),
    (0x083246, i_083246),
    (0x08324A, i_08324a),
    (0x08324E, i_08324e),
    (0x083252, i_083252),
    (0x083254, i_083254),
    (0x08325A, i_08325a),
    (0x083260, i_083260),
    (0x083262, i_083262),
    (0x083268, i_083268),
    (0x08326E, i_08326e),
    (0x083270, i_083270),
    (0x083278, i_083278),
    (0x083280, i_083280),
    (0x083284, i_083284),
    (0x08328A, i_08328a),
    (0x08328C, i_08328c),
    (0x083292, i_083292),
    (0x083298, i_083298),
    (0x08329E, i_08329e),
    (0x0832A4, i_0832a4),
    (0x0832A8, i_0832a8),
    (0x0832AE, i_0832ae),
    (0x0832B4, i_0832b4),
    (0x0832B6, i_0832b6),
    (0x0832BC, i_0832bc),
    (0x0832C0, i_0832c0),
    (0x0832C2, i_0832c2),
    (0x0832C8, i_0832c8),
    (0x0832CE, i_0832ce),
    (0x0832D0, i_0832d0),
    (0x0832D4, i_0832d4),
    (0x0832D8, i_0832d8),
    (0x0832DC, i_0832dc),
    (0x0832E0, i_0832e0),
    (0x0832E4, i_0832e4),
    (0x0832E6, i_0832e6),
    (0x0832EA, i_0832ea),
    (0x0832EC, i_0832ec),
    (0x0832F0, i_0832f0),
    (0x0832F6, i_0832f6),
    (0x0832FC, i_0832fc),
    (0x083302, i_083302),
    (0x083308, i_083308),
    (0x08330E, i_08330e),
    (0x083314, i_083314),
    (0x083318, i_083318),
    (0x08331E, i_08331e),
    (0x083322, i_083322),
    (0x083324, i_083324),
    (0x08332A, i_08332a),
    (0x08332E, i_08332e),
    (0x083332, i_083332),
    (0x083336, i_083336),
    (0x08333C, i_08333c),
    (0x083342, i_083342),
    (0x083346, i_083346),
    (0x083348, i_083348),
    (0x08334E, i_08334e),
    (0x083354, i_083354),
    (0x08335A, i_08335a),
    (0x083360, i_083360),
    (0x083364, i_083364),
    (0x08336A, i_08336a),
    (0x08336C, i_08336c),
    (0x083370, i_083370),
    (0x083376, i_083376),
    (0x08337C, i_08337c),
    (0x083382, i_083382),
    (0x083388, i_083388),
    (0x08338E, i_08338e),
    (0x083394, i_083394),
    (0x08339A, i_08339a),
    (0x0833A0, i_0833a0),
    (0x0833A4, i_0833a4),
    (0x0833AA, i_0833aa),
    (0x0833B0, i_0833b0),
    (0x0833B6, i_0833b6),
    (0x0833BA, i_0833ba),
    (0x0833BE, i_0833be),
    (0x0833C2, i_0833c2),
    (0x0833C4, i_0833c4),
    (0x0833CA, i_0833ca),
    (0x0833D0, i_0833d0),
    (0x0833D6, i_0833d6),
    (0x0833DA, i_0833da),
    (0x0833DE, i_0833de),
    (0x0833E2, i_0833e2),
    (0x0833E8, i_0833e8),
    (0x0833EC, i_0833ec),
    (0x0833F0, i_0833f0),
    (0x0833F4, i_0833f4),
    (0x0833F8, i_0833f8),
    (0x0833FE, i_0833fe),
    (0x083404, i_083404),
    (0x083408, i_083408),
    (0x08340A, i_08340a),
    (0x08340E, i_08340e),
    (0x083414, i_083414),
    (0x08341A, i_08341a),
    (0x08341E, i_08341e),
    (0x083424, i_083424),
    (0x083426, i_083426),
    (0x08342A, i_08342a),
    (0x083430, i_083430),
    (0x083436, i_083436),
    (0x08343C, i_08343c),
    (0x083442, i_083442),
    (0x083448, i_083448),
    (0x08344E, i_08344e),
    (0x083452, i_083452),
    (0x083458, i_083458),
    (0x08345C, i_08345c),
    (0x08345E, i_08345e),
    (0x083464, i_083464),
    (0x083468, i_083468),
    (0x08346E, i_08346e),
    (0x083474, i_083474),
    (0x083478, i_083478),
    (0x08347C, i_08347c),
    (0x083480, i_083480),
    (0x083482, i_083482),
    (0x083486, i_083486),
    (0x08348C, i_08348c),
    (0x083492, i_083492),
    (0x083498, i_083498),
    (0x08349C, i_08349c),
    (0x0834A2, i_0834a2),
    (0x0834A4, i_0834a4),
    (0x0834A8, i_0834a8),
    (0x0834AE, i_0834ae),
    (0x0834B2, i_0834b2),
    (0x0834B8, i_0834b8),
    (0x0834BE, i_0834be),
    (0x0834C4, i_0834c4),
    (0x0834CA, i_0834ca),
    (0x0834D0, i_0834d0),
    (0x0834D6, i_0834d6),
    (0x0834DC, i_0834dc),
    (0x0834E2, i_0834e2),
    (0x0834E6, i_0834e6),
    (0x0834E8, i_0834e8),
    (0x0834EE, i_0834ee),
    (0x0834F4, i_0834f4),
    (0x0834FA, i_0834fa),
    (0x083500, i_083500),
    (0x083504, i_083504),
    (0x08350A, i_08350a),
    (0x083510, i_083510),
    (0x083516, i_083516),
    (0x08351C, i_08351c),
    (0x083520, i_083520),
    (0x083526, i_083526),
    (0x08352C, i_08352c),
    (0x083530, i_083530),
    (0x083536, i_083536),
    (0x08353A, i_08353a),
    (0x08353C, i_08353c),
    (0x083542, i_083542),
    (0x083546, i_083546),
    (0x08354C, i_08354c),
    (0x08354E, i_08354e),
    (0x083554, i_083554),
    (0x08355A, i_08355a),
    (0x08355E, i_08355e),
    (0x083562, i_083562),
    (0x083568, i_083568),
    (0x08356E, i_08356e),
    (0x083572, i_083572),
    (0x083574, i_083574),
    (0x08357A, i_08357a),
    (0x083580, i_083580),
    (0x083584, i_083584),
    (0x08358A, i_08358a),
    (0x08358E, i_08358e),
    (0x083594, i_083594),
    (0x083596, i_083596),
    (0x08359A, i_08359a),
    (0x0835A0, i_0835a0),
    (0x0835A4, i_0835a4),
    (0x0835AA, i_0835aa),
    (0x0835B0, i_0835b0),
    (0x0835B6, i_0835b6),
    (0x0835BC, i_0835bc),
    (0x0835C2, i_0835c2),
    (0x0835C8, i_0835c8),
    (0x0835CE, i_0835ce),
    (0x0835D2, i_0835d2),
    (0x0835D4, i_0835d4),
    (0x0835DA, i_0835da),
    (0x0835E0, i_0835e0),
    (0x0835E6, i_0835e6),
    (0x0835EA, i_0835ea),
    (0x0835F0, i_0835f0),
    (0x0835F2, i_0835f2),
    (0x0835F6, i_0835f6),
    (0x0835FC, i_0835fc),
    (0x083602, i_083602),
    (0x083608, i_083608),
    (0x08360E, i_08360e),
    (0x083612, i_083612),
    (0x083616, i_083616),
    (0x08361C, i_08361c),
    (0x083622, i_083622),
    (0x083628, i_083628),
    (0x08362E, i_08362e),
    (0x083632, i_083632),
    (0x083636, i_083636),
    (0x08363C, i_08363c),
    (0x083642, i_083642),
    (0x083648, i_083648),
    (0x08364E, i_08364e),
    (0x083652, i_083652),
    (0x083658, i_083658),
    (0x08365E, i_08365e),
    (0x083662, i_083662),
    (0x083666, i_083666),
    (0x083668, i_083668),
    (0x08366C, i_08366c),
    (0x083672, i_083672),
    (0x083678, i_083678),
    (0x08367E, i_08367e),
    (0x083684, i_083684),
    (0x08368A, i_08368a),
    (0x083690, i_083690),
    (0x083696, i_083696),
    (0x08369C, i_08369c),
    (0x0836A0, i_0836a0),
    (0x0836A6, i_0836a6),
    (0x0836AC, i_0836ac),
    (0x0836B0, i_0836b0),
    (0x0836B6, i_0836b6),
    (0x0836BA, i_0836ba),
    (0x0836C0, i_0836c0),
    (0x0836C4, i_0836c4),
    (0x0836C6, i_0836c6),
    (0x0836CC, i_0836cc),
    (0x0836D2, i_0836d2),
    (0x0836D6, i_0836d6),
    (0x0836DC, i_0836dc),
    (0x0836E0, i_0836e0),
    (0x0836E6, i_0836e6),
    (0x0836EC, i_0836ec),
    (0x0836F2, i_0836f2),
    (0x0836F8, i_0836f8),
    (0x0836FC, i_0836fc),
    (0x083700, i_083700),
    (0x083702, i_083702),
    (0x083708, i_083708),
    (0x08370C, i_08370c),
    (0x083712, i_083712),
    (0x083714, i_083714),
    (0x08371A, i_08371a),
    (0x083720, i_083720),
    (0x083726, i_083726),
    (0x08372A, i_08372a),
    (0x083730, i_083730),
    (0x083736, i_083736),
    (0x08373A, i_08373a),
    (0x08373C, i_08373c),
    (0x083742, i_083742),
    (0x083748, i_083748),
    (0x08374C, i_08374c),
    (0x083752, i_083752),
    (0x083756, i_083756),
    (0x08375C, i_08375c),
    (0x083762, i_083762),
    (0x083768, i_083768),
    (0x08376E, i_08376e),
    (0x083772, i_083772),
    (0x083776, i_083776),
    (0x083778, i_083778),
    (0x08377E, i_08377e),
    (0x083782, i_083782),
    (0x083788, i_083788),
    (0x08378A, i_08378a),
    (0x083790, i_083790),
    (0x083796, i_083796),
    (0x08379C, i_08379c),
    (0x0837A0, i_0837a0),
    (0x0837A6, i_0837a6),
    (0x0837AC, i_0837ac),
    (0x0837B0, i_0837b0),
    (0x0837B2, i_0837b2),
    (0x0837B8, i_0837b8),
    (0x0837BE, i_0837be),
    (0x0837C2, i_0837c2),
    (0x0837C8, i_0837c8),
    (0x0837CC, i_0837cc),
    (0x0837D2, i_0837d2),
    (0x0837D8, i_0837d8),
    (0x0837DE, i_0837de),
    (0x0837E4, i_0837e4),
    (0x0837E8, i_0837e8),
    (0x0837EC, i_0837ec),
    (0x0837EE, i_0837ee),
    (0x0837F4, i_0837f4),
    (0x0837F8, i_0837f8),
    (0x0837FE, i_0837fe),
    (0x083800, i_083800),
    (0x083806, i_083806),
    (0x08380C, i_08380c),
    (0x083812, i_083812),
    (0x083816, i_083816),
    (0x08381A, i_08381a),
    (0x08381E, i_08381e),
    (0x083824, i_083824),
    (0x083828, i_083828),
    (0x08382C, i_08382c),
    (0x083832, i_083832),
    (0x083836, i_083836),
    (0x08383C, i_08383c),
    (0x083842, i_083842),
    (0x083846, i_083846),
    (0x08384C, i_08384c),
    (0x083852, i_083852),
    (0x083858, i_083858),
    (0x08385E, i_08385e),
    (0x083864, i_083864),
    (0x08386A, i_08386a),
    (0x08386E, i_08386e),
    (0x083874, i_083874),
    (0x08387A, i_08387a),
    (0x083880, i_083880),
    (0x083884, i_083884),
    (0x083888, i_083888),
    (0x08388E, i_08388e),
    (0x083892, i_083892),
    (0x083898, i_083898),
    (0x08389E, i_08389e),
    (0x0838A2, i_0838a2),
    (0x0838A8, i_0838a8),
    (0x0838AC, i_0838ac),
    (0x0838B0, i_0838b0),
    (0x0838B6, i_0838b6),
    (0x0838BC, i_0838bc),
    (0x0838C0, i_0838c0),
    (0x0838C6, i_0838c6),
    (0x0838CA, i_0838ca),
    (0x0838CC, i_0838cc),
    (0x0838D0, i_0838d0),
    (0x0838D6, i_0838d6),
    (0x0838DA, i_0838da),
    (0x0838E0, i_0838e0),
    (0x0838E4, i_0838e4),
    (0x0838E6, i_0838e6),
    (0x0838EC, i_0838ec),
    (0x0838F0, i_0838f0),
    (0x0838F4, i_0838f4),
    (0x0838FA, i_0838fa),
    (0x083900, i_083900),
    (0x083906, i_083906),
    (0x08390A, i_08390a),
    (0x08390E, i_08390e),
    (0x083912, i_083912),
    (0x083914, i_083914),
    (0x08391A, i_08391a),
    (0x08391E, i_08391e),
    (0x083924, i_083924),
    (0x083926, i_083926),
    (0x08392C, i_08392c),
    (0x083932, i_083932),
    (0x083938, i_083938),
    (0x08393E, i_08393e),
    (0x083942, i_083942),
    (0x083944, i_083944),
    (0x08394A, i_08394a),
    (0x083950, i_083950),
    (0x083954, i_083954),
    (0x083958, i_083958),
    (0x08395E, i_08395e),
    (0x083962, i_083962),
    (0x083968, i_083968),
    (0x08396A, i_08396a),
    (0x08396E, i_08396e),
    (0x083974, i_083974),
    (0x083978, i_083978),
    (0x08397E, i_08397e),
    (0x083984, i_083984),
    (0x08398A, i_08398a),
    (0x083990, i_083990),
    (0x083996, i_083996),
    (0x08399C, i_08399c),
    (0x0839A2, i_0839a2),
    (0x0839A8, i_0839a8),
    (0x0839AC, i_0839ac),
    (0x0839AE, i_0839ae),
    (0x0839B4, i_0839b4),
    (0x0839BA, i_0839ba),
    (0x0839C0, i_0839c0),
    (0x0839C6, i_0839c6),
    (0x0839CA, i_0839ca),
    (0x0839CE, i_0839ce),
    (0x0839D0, i_0839d0),
    (0x0839D6, i_0839d6),
    (0x0839DA, i_0839da),
    (0x0839E0, i_0839e0),
    (0x0839E2, i_0839e2),
    (0x0839E6, i_0839e6),
    (0x0839EC, i_0839ec),
    (0x0839F2, i_0839f2),
    (0x0839F6, i_0839f6),
    (0x0839FC, i_0839fc),
    (0x083A00, i_083a00),
    (0x083A04, i_083a04),
    (0x083A08, i_083a08),
    (0x083A0A, i_083a0a),
    (0x083A10, i_083a10),
    (0x083A16, i_083a16),
    (0x083A1A, i_083a1a),
    (0x083A1E, i_083a1e),
    (0x083A22, i_083a22),
    (0x083A24, i_083a24),
    (0x083A2A, i_083a2a),
    (0x083A2E, i_083a2e),
    (0x083A34, i_083a34),
    (0x083A36, i_083a36),
    (0x083A3A, i_083a3a),
    (0x083A40, i_083a40),
    (0x083A44, i_083a44),
    (0x083A4A, i_083a4a),
    (0x083A50, i_083a50),
    (0x083A56, i_083a56),
    (0x083A5C, i_083a5c),
    (0x083A62, i_083a62),
    (0x083A68, i_083a68),
    (0x083A6E, i_083a6e),
    (0x083A74, i_083a74),
    (0x083A7A, i_083a7a),
    (0x083A80, i_083a80),
    (0x083A84, i_083a84),
    (0x083A86, i_083a86),
    (0x083A8C, i_083a8c),
    (0x083A92, i_083a92),
    (0x083A98, i_083a98),
    (0x083A9C, i_083a9c),
    (0x083AA2, i_083aa2),
    (0x083AA4, i_083aa4),
    (0x083AA8, i_083aa8),
    (0x083AAE, i_083aae),
    (0x083AB2, i_083ab2),
    (0x083AB8, i_083ab8),
    (0x083ABE, i_083abe),
    (0x083AC4, i_083ac4),
    (0x083ACA, i_083aca),
    (0x083AD0, i_083ad0),
    (0x083AD6, i_083ad6),
    (0x083ADC, i_083adc),
    (0x083AE2, i_083ae2),
    (0x083AE8, i_083ae8),
    (0x083AEE, i_083aee),
    (0x083AF2, i_083af2),
    (0x083AF4, i_083af4),
    (0x083AFA, i_083afa),
    (0x083B00, i_083b00),
    (0x083B06, i_083b06),
    (0x083B0A, i_083b0a),
    (0x083B10, i_083b10),
    (0x083B12, i_083b12),
    (0x083B16, i_083b16),
    (0x083B1C, i_083b1c),
    (0x083B20, i_083b20),
    (0x083B26, i_083b26),
    (0x083B2C, i_083b2c),
    (0x083B32, i_083b32),
    (0x083B38, i_083b38),
    (0x083B3E, i_083b3e),
    (0x083B44, i_083b44),
    (0x083B4A, i_083b4a),
    (0x083B4E, i_083b4e),
    (0x083B54, i_083b54),
    (0x083B5A, i_083b5a),
    (0x083B5E, i_083b5e),
    (0x083B62, i_083b62),
    (0x083B64, i_083b64),
    (0x083B6A, i_083b6a),
    (0x083B70, i_083b70),
    (0x083B74, i_083b74),
    (0x083B7A, i_083b7a),
    (0x083B80, i_083b80),
    (0x083B84, i_083b84),
    (0x083B8A, i_083b8a),
    (0x083B90, i_083b90),
    (0x083B92, i_083b92),
    (0x083B96, i_083b96),
    (0x083B9C, i_083b9c),
    (0x083BA2, i_083ba2),
    (0x083BA8, i_083ba8),
    (0x083BAE, i_083bae),
    (0x083BB4, i_083bb4),
    (0x083BBA, i_083bba),
    (0x083BBE, i_083bbe),
    (0x083BC0, i_083bc0),
    (0x083BC4, i_083bc4),
    (0x083BCA, i_083bca),
    (0x083BD0, i_083bd0),
    (0x083BD6, i_083bd6),
    (0x083BDA, i_083bda),
    (0x083BE0, i_083be0),
    (0x083BE2, i_083be2),
    (0x083BE6, i_083be6),
    (0x083BEC, i_083bec),
    (0x083BF2, i_083bf2),
    (0x083BF8, i_083bf8),
    (0x083BFE, i_083bfe),
    (0x083C0C, i_083c0c),
    (0x083C12, i_083c12),
    (0x083C18, i_083c18),
    (0x083C1E, i_083c1e),
    (0x083C24, i_083c24),
    (0x083C2A, i_083c2a),
    (0x083C30, i_083c30),
    (0x083C34, i_083c34),
    (0x083C3A, i_083c3a),
    (0x083C40, i_083c40),
    (0x083C44, i_083c44),
    (0x083C46, i_083c46),
    (0x083C4C, i_083c4c),
    (0x083C52, i_083c52),
    (0x083C56, i_083c56),
    (0x083C5C, i_083c5c),
    (0x083C62, i_083c62),
    (0x083C66, i_083c66),
    (0x083C6A, i_083c6a),
    (0x083C70, i_083c70),
    (0x083C76, i_083c76),
    (0x083C7C, i_083c7c),
    (0x083C80, i_083c80),
    (0x083C86, i_083c86),
    (0x083C8A, i_083c8a),
    (0x083C90, i_083c90),
    (0x083C92, i_083c92),
    (0x083C96, i_083c96),
    (0x083C9C, i_083c9c),
    (0x083CA0, i_083ca0),
    (0x083CA6, i_083ca6),
    (0x083CAC, i_083cac),
    (0x083CB2, i_083cb2),
    (0x083CB8, i_083cb8),
    (0x083CBE, i_083cbe),
    (0x083CC4, i_083cc4),
    (0x083CCA, i_083cca),
    (0x083CD0, i_083cd0),
    (0x083CD4, i_083cd4),
    (0x083CD6, i_083cd6),
    (0x083CDC, i_083cdc),
    (0x083CE2, i_083ce2),
    (0x083CE8, i_083ce8),
    (0x083CEC, i_083cec),
    (0x083CF2, i_083cf2),
    (0x083CF8, i_083cf8),
    (0x083CFE, i_083cfe),
    (0x083D04, i_083d04),
    (0x083D08, i_083d08),
    (0x083D0C, i_083d0c),
    (0x083D12, i_083d12),
    (0x083D16, i_083d16),
    (0x083D18, i_083d18),
    (0x083D1E, i_083d1e),
    (0x083D22, i_083d22),
    (0x083D28, i_083d28),
    (0x083D2A, i_083d2a),
    (0x083D2E, i_083d2e),
    (0x083D32, i_083d32),
    (0x083D38, i_083d38),
    (0x083D3E, i_083d3e),
    (0x083D44, i_083d44),
    (0x083D4A, i_083d4a),
    (0x083D4E, i_083d4e),
    (0x083D50, i_083d50),
    (0x083D56, i_083d56),
    (0x083D5C, i_083d5c),
    (0x083D62, i_083d62),
    (0x083D66, i_083d66),
    (0x083D6C, i_083d6c),
    (0x083D6E, i_083d6e),
    (0x083D72, i_083d72),
    (0x083D78, i_083d78),
    (0x083D7E, i_083d7e),
    (0x083D84, i_083d84),
    (0x083D8A, i_083d8a),
    (0x083DAA, i_083daa),
    (0x083DB0, i_083db0),
    (0x083DB6, i_083db6),
    (0x083DBC, i_083dbc),
    (0x083DC0, i_083dc0),
    (0x083DC2, i_083dc2),
    (0x083DC8, i_083dc8),
    (0x083DCE, i_083dce),
    (0x083DD4, i_083dd4),
    (0x083DD8, i_083dd8),
    (0x083DDE, i_083dde),
    (0x083DE4, i_083de4),
    (0x083DEA, i_083dea),
    (0x083DF0, i_083df0),
    (0x083DF4, i_083df4),
    (0x083DF8, i_083df8),
    (0x083DFE, i_083dfe),
    (0x083E04, i_083e04),
    (0x083E08, i_083e08),
    (0x083E0C, i_083e0c),
    (0x083E12, i_083e12),
    (0x083E18, i_083e18),
    (0x083E1C, i_083e1c),
    (0x083E20, i_083e20),
    (0x083E26, i_083e26),
    (0x083E2C, i_083e2c),
    (0x083E32, i_083e32),
    (0x083E36, i_083e36),
    (0x083E3C, i_083e3c),
    (0x083E40, i_083e40),
    (0x083E46, i_083e46),
    (0x083E48, i_083e48),
    (0x083E4C, i_083e4c),
    (0x083E52, i_083e52),
    (0x083E56, i_083e56),
    (0x083E5C, i_083e5c),
    (0x083E62, i_083e62),
    (0x083E68, i_083e68),
    (0x083E6E, i_083e6e),
    (0x083E74, i_083e74),
    (0x083E7A, i_083e7a),
    (0x083E80, i_083e80),
    (0x083E86, i_083e86),
    (0x083E8C, i_083e8c),
    (0x083E90, i_083e90),
    (0x083E92, i_083e92),
    (0x083E98, i_083e98),
    (0x083E9E, i_083e9e),
    (0x083EA4, i_083ea4),
    (0x083EA8, i_083ea8),
    (0x083EAE, i_083eae),
    (0x083EB4, i_083eb4),
    (0x083EBA, i_083eba),
    (0x083EBE, i_083ebe),
    (0x083EC0, i_083ec0),
    (0x083EC6, i_083ec6),
    (0x083ECA, i_083eca),
    (0x083ECE, i_083ece),
    (0x083ED4, i_083ed4),
    (0x083ED8, i_083ed8),
    (0x083EDA, i_083eda),
    (0x083EE0, i_083ee0),
    (0x083EE4, i_083ee4),
    (0x083EEA, i_083eea),
    (0x083EEC, i_083eec),
    (0x083EF2, i_083ef2),
    (0x083EF8, i_083ef8),
    (0x083EFC, i_083efc),
    (0x083EFE, i_083efe),
    (0x083F04, i_083f04),
    (0x083F0A, i_083f0a),
    (0x083F0E, i_083f0e),
    (0x083F12, i_083f12),
    (0x083F14, i_083f14),
    (0x083F1A, i_083f1a),
    (0x083F1E, i_083f1e),
    (0x083F24, i_083f24),
    (0x083F26, i_083f26),
    (0x083F2C, i_083f2c),
    (0x083F32, i_083f32),
    (0x083F36, i_083f36),
    (0x083F3A, i_083f3a),
    (0x083F40, i_083f40),
    (0x083F46, i_083f46),
    (0x083F4C, i_083f4c),
    (0x083F52, i_083f52),
    (0x083F58, i_083f58),
    (0x083F5E, i_083f5e),
    (0x083F64, i_083f64),
    (0x083F6A, i_083f6a),
    (0x083F70, i_083f70),
    (0x083F74, i_083f74),
    (0x083F76, i_083f76),
    (0x083F7C, i_083f7c),
    (0x083F80, i_083f80),
    (0x083F86, i_083f86),
    (0x083F8A, i_083f8a),
    (0x083F90, i_083f90),
    (0x083F96, i_083f96),
    (0x083F9C, i_083f9c),
    (0x083FA0, i_083fa0),
    (0x083FA6, i_083fa6),
    (0x083FA8, i_083fa8),
    (0x083FAC, i_083fac),
    (0x083FB2, i_083fb2),
    (0x083FB8, i_083fb8),
    (0x083FBE, i_083fbe),
    (0x083FC2, i_083fc2),
    (0x083FC6, i_083fc6),
    (0x083FC8, i_083fc8),
    (0x083FD0, i_083fd0),
    (0x083FD6, i_083fd6),
    (0x083FDC, i_083fdc),
    (0x083FE0, i_083fe0),
    (0x083FE2, i_083fe2),
    (0x083FE8, i_083fe8),
    (0x083FEE, i_083fee),
    (0x083FF4, i_083ff4),
    (0x083FFA, i_083ffa),
    (0x083FFE, i_083ffe),
    (0x084000, i_084000),
    (0x084006, i_084006),
    (0x08400C, i_08400c),
    (0x084012, i_084012),
    (0x084016, i_084016),
    (0x08401C, i_08401c),
    (0x084022, i_084022),
    (0x084028, i_084028),
    (0x08402E, i_08402e),
    (0x084032, i_084032),
    (0x084038, i_084038),
    (0x08403E, i_08403e),
    (0x084042, i_084042),
    (0x084048, i_084048),
    (0x08404C, i_08404c),
    (0x08404E, i_08404e),
    (0x084054, i_084054),
    (0x084058, i_084058),
    (0x08405E, i_08405e),
    (0x084060, i_084060),
    (0x084066, i_084066),
    (0x08406C, i_08406c),
    (0x084070, i_084070),
    (0x084074, i_084074),
    (0x084078, i_084078),
    (0x08407C, i_08407c),
    (0x084080, i_084080),
    (0x084086, i_084086),
    (0x08408C, i_08408c),
    (0x084092, i_084092),
    (0x084094, i_084094),
    (0x084098, i_084098),
    (0x08409A, i_08409a),
    (0x08409E, i_08409e),
    (0x0840A2, i_0840a2),
    (0x0840A6, i_0840a6),
    (0x0840AC, i_0840ac),
    (0x0840B0, i_0840b0),
    (0x0840B4, i_0840b4),
    (0x0840B8, i_0840b8),
    (0x0840BE, i_0840be),
    (0x0840C4, i_0840c4),
    (0x0840C8, i_0840c8),
    (0x0840CA, i_0840ca),
    (0x0840D0, i_0840d0),
    (0x0840D4, i_0840d4),
    (0x0840D8, i_0840d8),
    (0x0840DA, i_0840da),
    (0x0840DE, i_0840de),
    (0x0840E2, i_0840e2),
    (0x0840E8, i_0840e8),
    (0x0840EC, i_0840ec),
    (0x0840F0, i_0840f0),
    (0x0840F4, i_0840f4),
    (0x0840F8, i_0840f8),
    (0x0840FC, i_0840fc),
    (0x084102, i_084102),
    (0x084108, i_084108),
    (0x08410C, i_08410c),
    (0x084110, i_084110),
    (0x084112, i_084112),
    (0x084116, i_084116),
    (0x08411A, i_08411a),
    (0x084120, i_084120),
    (0x084124, i_084124),
    (0x084128, i_084128),
    (0x08412C, i_08412c),
    (0x084130, i_084130),
    (0x084134, i_084134),
    (0x08413A, i_08413a),
    (0x084140, i_084140),
    (0x084144, i_084144),
    (0x08414A, i_08414a),
    (0x08414C, i_08414c),
    (0x084150, i_084150),
    (0x084156, i_084156),
    (0x08415A, i_08415a),
    (0x084160, i_084160),
    (0x084166, i_084166),
    (0x08416C, i_08416c),
    (0x084172, i_084172),
    (0x084178, i_084178),
    (0x08417E, i_08417e),
    (0x084184, i_084184),
    (0x08418A, i_08418a),
    (0x08418E, i_08418e),
    (0x084190, i_084190),
    (0x084196, i_084196),
    (0x08419C, i_08419c),
    (0x0841A2, i_0841a2),
    (0x0841A6, i_0841a6),
    (0x0841AA, i_0841aa),
    (0x0841AC, i_0841ac),
    (0x0841B2, i_0841b2),
    (0x0841B6, i_0841b6),
    (0x0841BC, i_0841bc),
    (0x0841BE, i_0841be),
    (0x0841C2, i_0841c2),
    (0x0841C6, i_0841c6),
    (0x0841CC, i_0841cc),
    (0x0841D2, i_0841d2),
    (0x0841D6, i_0841d6),
    (0x0841D8, i_0841d8),
    (0x0841DE, i_0841de),
    (0x0841E4, i_0841e4),
    (0x0841EA, i_0841ea),
    (0x0841EE, i_0841ee),
    (0x0841F4, i_0841f4),
    (0x0841F6, i_0841f6),
    (0x0841FA, i_0841fa),
    (0x084200, i_084200),
    (0x084206, i_084206),
    (0x08420C, i_08420c),
    (0x084212, i_084212),
    (0x084218, i_084218),
    (0x08421C, i_08421c),
    (0x08421E, i_08421e),
    (0x084224, i_084224),
    (0x08422A, i_08422a),
    (0x084230, i_084230),
    (0x084234, i_084234),
    (0x084238, i_084238),
    (0x08423A, i_08423a),
    (0x084240, i_084240),
    (0x084244, i_084244),
    (0x08424A, i_08424a),
    (0x08424C, i_08424c),
    (0x084250, i_084250),
    (0x084254, i_084254),
    (0x084258, i_084258),
    (0x08425C, i_08425c),
    (0x08425E, i_08425e),
    (0x084266, i_084266),
    (0x08426A, i_08426a),
    (0x084270, i_084270),
    (0x084274, i_084274),
    (0x084278, i_084278),
    (0x08427A, i_08427a),
    (0x084282, i_084282),
    (0x084288, i_084288),
    (0x08428E, i_08428e),
    (0x084294, i_084294),
    (0x08429A, i_08429a),
    (0x08429E, i_08429e),
    (0x0842A0, i_0842a0),
    (0x0842A6, i_0842a6),
    (0x0842AC, i_0842ac),
    (0x0842B2, i_0842b2),
    (0x0842B6, i_0842b6),
    (0x0842BA, i_0842ba),
    (0x0842BE, i_0842be),
    (0x0842C4, i_0842c4),
    (0x0842C8, i_0842c8),
    (0x0842CE, i_0842ce),
    (0x0842D0, i_0842d0),
    (0x0842D4, i_0842d4),
    (0x0842DA, i_0842da),
    (0x0842E0, i_0842e0),
    (0x0842E6, i_0842e6),
    (0x0842EC, i_0842ec),
    (0x0842F2, i_0842f2),
    (0x0842F6, i_0842f6),
    (0x0842F8, i_0842f8),
    (0x0842FE, i_0842fe),
    (0x084304, i_084304),
    (0x08430A, i_08430a),
    (0x08430E, i_08430e),
    (0x084314, i_084314),
    (0x08431A, i_08431a),
    (0x084320, i_084320),
    (0x084326, i_084326),
    (0x08432A, i_08432a),
    (0x084330, i_084330),
    (0x084336, i_084336),
    (0x08433A, i_08433a),
    (0x084340, i_084340),
    (0x084344, i_084344),
    (0x084348, i_084348),
    (0x08434C, i_08434c),
    (0x084350, i_084350),
    (0x084356, i_084356),
    (0x08435C, i_08435c),
    (0x084362, i_084362),
    (0x084368, i_084368),
    (0x08436E, i_08436e),
    (0x084374, i_084374),
    (0x08437A, i_08437a),
    (0x084380, i_084380),
    (0x084384, i_084384),
    (0x08438A, i_08438a),
    (0x08438E, i_08438e),
    (0x084394, i_084394),
    (0x084396, i_084396),
    (0x08439A, i_08439a),
    (0x0843A0, i_0843a0),
    (0x0843A4, i_0843a4),
    (0x0843AA, i_0843aa),
    (0x0843B0, i_0843b0),
    (0x0843B4, i_0843b4),
    (0x0843B8, i_0843b8),
    (0x0843BC, i_0843bc),
    (0x0843C2, i_0843c2),
    (0x0843C8, i_0843c8),
    (0x0843CE, i_0843ce),
    (0x0843D4, i_0843d4),
    (0x0843DA, i_0843da),
    (0x0843DE, i_0843de),
    (0x0843E0, i_0843e0),
    (0x0843E6, i_0843e6),
    (0x0843EC, i_0843ec),
    (0x0843F2, i_0843f2),
    (0x0843F8, i_0843f8),
    (0x0843FE, i_0843fe),
    (0x084402, i_084402),
    (0x084408, i_084408),
    (0x08440C, i_08440c),
    (0x08440E, i_08440e),
    (0x084410, i_084410),
    (0x084418, i_084418),
    (0x08441C, i_08441c),
    (0x084422, i_084422),
    (0x084428, i_084428),
    (0x08442C, i_08442c),
    (0x084430, i_084430),
    (0x084434, i_084434),
    (0x08443A, i_08443a),
    (0x08443E, i_08443e),
    (0x084442, i_084442),
    (0x084446, i_084446),
    (0x08444A, i_08444a),
    (0x084450, i_084450),
    (0x084456, i_084456),
    (0x08445C, i_08445c),
    (0x084462, i_084462),
    (0x084468, i_084468),
    (0x08446C, i_08446c),
    (0x084472, i_084472),
    (0x084478, i_084478),
    (0x08447E, i_08447e),
    (0x084484, i_084484),
    (0x08448A, i_08448a),
    (0x08448E, i_08448e),
    (0x084494, i_084494),
    (0x084498, i_084498),
    (0x08449A, i_08449a),
    (0x0844A0, i_0844a0),
    (0x0844A6, i_0844a6),
    (0x0844AC, i_0844ac),
    (0x0844B0, i_0844b0),
    (0x0844B8, i_0844b8),
    (0x0844BC, i_0844bc),
    (0x0844BE, i_0844be),
    (0x0844C0, i_0844c0),
    (0x0844C6, i_0844c6),
    (0x0844CC, i_0844cc),
    (0x0844D2, i_0844d2),
    (0x0844D8, i_0844d8),
    (0x0844DE, i_0844de),
    (0x0844E2, i_0844e2),
    (0x0844E4, i_0844e4),
    (0x0844EA, i_0844ea),
    (0x0844F0, i_0844f0),
    (0x0844F6, i_0844f6),
    (0x0844FA, i_0844fa),
    (0x084500, i_084500),
    (0x084504, i_084504),
    (0x084508, i_084508),
    (0x08450A, i_08450a),
    (0x08450C, i_08450c),
    (0x084512, i_084512),
    (0x084518, i_084518),
    (0x084520, i_084520),
    (0x084526, i_084526),
    (0x08452C, i_08452c),
    (0x084530, i_084530),
    (0x084534, i_084534),
    (0x084536, i_084536),
    (0x08453C, i_08453c),
    (0x084542, i_084542),
    (0x084548, i_084548),
    (0x08454C, i_08454c),
    (0x084552, i_084552),
    (0x084558, i_084558),
    (0x08455E, i_08455e),
    (0x084564, i_084564),
    (0x08456A, i_08456a),
    (0x08456E, i_08456e),
    (0x084574, i_084574),
    (0x084578, i_084578),
    (0x08457C, i_08457c),
    (0x084580, i_084580),
    (0x084584, i_084584),
    (0x08458A, i_08458a),
    (0x084590, i_084590),
    (0x084596, i_084596),
    (0x08459C, i_08459c),
    (0x0845A0, i_0845a0),
    (0x0845A6, i_0845a6),
    (0x0845AC, i_0845ac),
    (0x0845B0, i_0845b0),
    (0x0845B6, i_0845b6),
    (0x0845B8, i_0845b8),
    (0x0845C0, i_0845c0),
    (0x0845C4, i_0845c4),
    (0x0845C6, i_0845c6),
    (0x0845CA, i_0845ca),
    (0x0845CE, i_0845ce),
    (0x0845D2, i_0845d2),
    (0x0845D6, i_0845d6),
    (0x0845DA, i_0845da),
    (0x0845E2, i_0845e2),
    (0x0845E4, i_0845e4),
    (0x0845EC, i_0845ec),
    (0x0845F0, i_0845f0),
    (0x0845F2, i_0845f2),
    (0x0845F6, i_0845f6),
    (0x0845FC, i_0845fc),
    (0x084600, i_084600),
    (0x084604, i_084604),
    (0x084608, i_084608),
    (0x08460C, i_08460c),
    (0x084612, i_084612),
    (0x084618, i_084618),
    (0x08461C, i_08461c),
    (0x084620, i_084620),
    (0x084622, i_084622),
    (0x084626, i_084626),
    (0x08462C, i_08462c),
    (0x084630, i_084630),
    (0x084636, i_084636),
    (0x084638, i_084638),
    (0x08463E, i_08463e),
    (0x084644, i_084644),
    (0x084648, i_084648),
    (0x08464E, i_08464e),
    (0x084652, i_084652),
    (0x084654, i_084654),
    (0x084658, i_084658),
    (0x08465E, i_08465e),
    (0x084662, i_084662),
    (0x084666, i_084666),
    (0x08466A, i_08466a),
    (0x08466E, i_08466e),
    (0x084674, i_084674),
    (0x08467A, i_08467a),
    (0x08467E, i_08467e),
    (0x084682, i_084682),
    (0x084684, i_084684),
    (0x084688, i_084688),
    (0x08468E, i_08468e),
    (0x084692, i_084692),
    (0x084698, i_084698),
    (0x08469A, i_08469a),
    (0x08469E, i_08469e),
    (0x0846A4, i_0846a4),
    (0x0846A8, i_0846a8),
    (0x0846AA, i_0846aa),
    (0x0846AE, i_0846ae),
    (0x0846B4, i_0846b4),
    (0x0846B8, i_0846b8),
    (0x0846BC, i_0846bc),
    (0x0846C0, i_0846c0),
    (0x0846C4, i_0846c4),
    (0x0846CA, i_0846ca),
    (0x0846CE, i_0846ce),
    (0x0846D4, i_0846d4),
    (0x0846D8, i_0846d8),
    (0x0846DE, i_0846de),
    (0x0846E0, i_0846e0),
    (0x0846E4, i_0846e4),
    (0x0846EA, i_0846ea),
    (0x0846F0, i_0846f0),
    (0x0846F6, i_0846f6),
    (0x0846FC, i_0846fc),
    (0x084702, i_084702),
    (0x084708, i_084708),
    (0x08470E, i_08470e),
    (0x084712, i_084712),
    (0x084716, i_084716),
    (0x08471C, i_08471c),
    (0x08471E, i_08471e),
    (0x084720, i_084720),
    (0x084726, i_084726),
    (0x084728, i_084728),
    (0x08472C, i_08472c),
    (0x084732, i_084732),
    (0x084734, i_084734),
    (0x084738, i_084738),
];
